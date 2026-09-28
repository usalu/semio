//! 📨️ A repeated identical response is a no-op; conflicting identities are rejected.
use super::mutation::CommitResponse;
use crate::{FormsDiff, FormsSnapshot};

pub fn diff(payload: &CommitResponse, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    if let Err(message) = payload.response.validate() { return protocol::MutationOutcome::error("forms.invalid-response", message, [payload.response.id.clone()]); }
    if let Some(existing) = base.responses.iter().find(|response| response.id == payload.response.id) {
        if existing == &payload.response { return protocol::MutationOutcome::empty(); }
        return protocol::MutationOutcome::error("forms.duplicate-response", "A different response has this id.", [payload.response.id.clone()]);
    }
    let mut responses = base.responses.clone();
    responses.insert(payload.index.unwrap_or(responses.len()).min(responses.len()), payload.response.clone());
    let results = crate::forms_results_child(&responses);
    protocol::MutationOutcome::new(FormsDiff { responses: Some(responses), results: Some(results), ..Default::default() })
}
