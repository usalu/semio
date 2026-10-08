//! 📨️ A repeated identical response is a no-op; conflicting identities are rejected.
use super::mutation::CommitResponse;
use crate::schema::diff::{FormsResponsesDelta};
use crate::{FormsDiff, FormsSnapshot};

pub fn diff(payload: &CommitResponse, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    if let Err(message) = payload.response.validate() { return protocol::MutationOutcome::fatal("mutation.invariant", message, [payload.response.id.clone()]); }
    if let Some(existing) = base.responses.iter().find(|response| response.id == payload.response.id) {
        if existing == &payload.response { return protocol::MutationOutcome::empty(); }
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A different response has this id.", [payload.response.id.clone()]);
    }
    let at = payload.index.map_or(base.responses.len(), |index| index.min(base.responses.len()));
    protocol::MutationOutcome::new(FormsDiff { responses: Some(FormsResponsesDelta::insertion(at, payload.response.clone())), ..Default::default() })
}
