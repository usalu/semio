use super::RemovePile;
use crate::diff::{En1997Diff, En1997PileList};
use crate::En1997Snapshot;
pub fn diff(payload: &RemovePile, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.piles.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    let mut piles = base.piles.clone();
    piles.remove(payload.index);
    protocol::MutationOutcome::new(En1997Diff { piles: Some(En1997PileList { values: piles }), ..Default::default() })
}
