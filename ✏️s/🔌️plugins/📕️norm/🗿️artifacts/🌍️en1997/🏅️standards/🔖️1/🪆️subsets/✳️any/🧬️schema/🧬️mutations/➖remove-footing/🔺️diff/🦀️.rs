use super::RemoveFooting;
use crate::diff::{En1997Diff, En1997FootingList};
use crate::En1997Snapshot;
pub fn diff(payload: &RemoveFooting, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.footings.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("footing #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    let mut footings = base.footings.clone();
    footings.remove(payload.index);
    protocol::MutationOutcome::new(En1997Diff { footings: Some(En1997FootingList { values: footings }), ..Default::default() })
}
