//! 📨️ A repeated identical response is a no-op; conflicting identities are rejected.
use super::mutation::CommitResponse;
use crate::schema::diff::{forms_diff_from_responses_delta, FormsResponsesDelta};
use crate::{FormsDiff, FormsSnapshot};

pub fn diff(payload: &CommitResponse, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    if let Err(message) = payload.response.validate() { return protocol::MutationOutcome::fatal("mutation.invariant", message, [payload.response.id.clone()]); }
    if let Some(existing) = base.responses.iter().find(|response| response.id == payload.response.id) {
        if existing == &payload.response { return protocol::MutationOutcome::empty(); }
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A different response has this id.", [payload.response.id.clone()]);
    }
    let reordered = payload.index.filter(|index| *index < base.responses.len()).map(|index| {
        let mut order: Vec<String> = base.responses.iter().map(|response| response.id.clone()).collect();
        order.insert(index, payload.response.id.clone());
        order
    });
    protocol::MutationOutcome::new(forms_diff_from_responses_delta(&FormsResponsesDelta { added: vec![payload.response.clone()], reordered, ..Default::default() }, base))
}
