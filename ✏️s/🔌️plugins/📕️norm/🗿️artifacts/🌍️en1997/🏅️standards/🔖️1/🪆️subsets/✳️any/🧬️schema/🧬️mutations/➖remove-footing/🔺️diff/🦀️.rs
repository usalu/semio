use super::RemoveFooting;
use crate::En1997Snapshot;
use crate::diff::{En1997Diff, En1997FootingsRows};

pub fn diff(payload: &RemoveFooting, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.footings.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("footing #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { footings: Some(En1997FootingsRows { removed: vec![base.footings[payload.index].id.clone()], ..Default::default() }), ..Default::default() })
}
