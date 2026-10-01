use super::InsertLoadCase;
use crate::diff::En1993LoadCaseList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertLoadCase, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.load_cases.iter().any(|existing| existing.id == payload.load_case.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Load case id {} already exists.", payload.load_case.id), [payload.load_case.id.clone()]);
    }
    let mut values = base.load_cases.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.load_case.clone());
    protocol::MutationOutcome::new(En1993Diff { load_cases: Some(En1993LoadCaseList { values }), ..Default::default() })
}
