use mongodb::bson;
use serde::Serialize;
use thiserror::Error;
use std::convert::Infallible;
use warp::{http::StatusCode, reply, Rejection, Reply};


#[derive(Error, Debug)]
pub enum Error{
    #[error("mongodb error:{0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("could not access field in document: {0}")]

    MongoDataError(#[from] bson::document::ValueAccessError),
    #[error("invalid id used: {0}")]

    InvalidIDError(String),
}


#[derive(Serialize)]
struct ErrorResponse{
    message: String,
}

impl warp::reject::Reject for Error{}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<Box<dyn Reply>, Infallible>{
    let code: StatusCode;
    let message: &str;

    if err.is_not_found(){
        code = StatusCode::NOT_FOUND;
        message 
    }
}

