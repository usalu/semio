//! ↩️ Retraction removes one response while retaining the document definition.
use super::mutation::DiscardResponse;
use crate::schema::diff::{FormsResponsesDelta};
use crate::{FormsDiff, FormsSnapshot};

pub fn diff(payload: &DiscardResponse, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let Some(index) = base.responses.iter().position(|response| response.id == payload.id) else { return protocol::MutationOutcome::error("mutation.target-missing", "The response does not exist.", [payload.id.clone()]); };
    protocol::MutationOutcome::new(FormsDiff { responses: Some(FormsResponsesDelta::removal(&base.responses, index)), ..Default::default() })
}
