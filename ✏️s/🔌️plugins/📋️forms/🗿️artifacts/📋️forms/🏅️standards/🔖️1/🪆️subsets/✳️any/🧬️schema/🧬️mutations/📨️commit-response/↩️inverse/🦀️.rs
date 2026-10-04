//! ↩️ Only a newly admitted submission has a retraction inverse.
use super::mutation::CommitResponse;
use crate::{FormMutation, FormsSnapshot};

pub fn inverse(payload: &CommitResponse, base: &FormsSnapshot) -> Result<Vec<FormMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.responses.iter().any(|response| response.id == payload.response.id) { return Vec::new(); }
    vec![FormMutation::DiscardResponse(crate::schema::mutations::discard_response::mutation::DiscardResponse { id: payload.response.id.clone() })]

    })())
}
