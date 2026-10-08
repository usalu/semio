//! ↩️ Retraction removes one response while retaining the document definition.
use super::mutation::DiscardResponse;
use crate::schema::diff::{forms_diff_from_responses_delta, FormsResponsesDelta};
use crate::{FormsDiff, FormsSnapshot};

pub fn diff(payload: &DiscardResponse, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    if !base.responses.iter().any(|response| response.id == payload.id) { return protocol::MutationOutcome::error("mutation.target-missing", "The response does not exist.", [payload.id.clone()]); }
    protocol::MutationOutcome::new(forms_diff_from_responses_delta(&FormsResponsesDelta { removed: vec![payload.id.clone()], ..Default::default() }, base))
}
