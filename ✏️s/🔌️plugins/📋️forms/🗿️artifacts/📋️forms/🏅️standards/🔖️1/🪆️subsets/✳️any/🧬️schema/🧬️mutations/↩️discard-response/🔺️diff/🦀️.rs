//! ↩️ Retraction removes one response while retaining the document definition.
use super::mutation::DiscardResponse;
use crate::{FormsDiff, FormsSnapshot};

pub fn diff(payload: &DiscardResponse, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    if !base.responses.iter().any(|response| response.id == payload.id) { return protocol::MutationOutcome::error("forms.missing-response", "The response does not exist.", [payload.id.clone()]); }
    let responses: Vec<_> = base.responses.iter().filter(|response| response.id != payload.id).cloned().collect();
    let results = crate::forms_results_child(&responses);
    protocol::MutationOutcome::new(FormsDiff { responses: Some(responses), results: Some(results), ..Default::default() })
}
