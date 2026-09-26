use super::RemoveLoadCase;
use crate::diff::En1993LoadCaseList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemoveLoadCase, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.load_cases.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("load-case index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.load_cases.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { load_cases: Some(En1993LoadCaseList { values }), ..Default::default() })
}
