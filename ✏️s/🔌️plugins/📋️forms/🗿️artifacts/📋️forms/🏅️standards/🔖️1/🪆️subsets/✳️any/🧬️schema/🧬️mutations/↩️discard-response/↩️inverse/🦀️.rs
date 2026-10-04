//! ↩️ Restoring a response retains its original values, labels and order.
use super::mutation::DiscardResponse;
use crate::{FormMutation, FormsSnapshot};

pub fn inverse(payload: &DiscardResponse, base: &FormsSnapshot) -> Result<Vec<FormMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.responses.iter().enumerate().find(|(_, response)| response.id == payload.id).map(|(index, response)| vec![FormMutation::CommitResponse(crate::schema::mutations::commit_response::mutation::CommitResponse { response: response.clone(), index: Some(index) })]).unwrap_or_default()

    })())
}
