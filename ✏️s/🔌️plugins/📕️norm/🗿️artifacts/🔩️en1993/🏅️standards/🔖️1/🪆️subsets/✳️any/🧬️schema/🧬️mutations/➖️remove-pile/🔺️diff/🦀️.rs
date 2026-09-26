use super::RemovePile;
use crate::diff::En1993PileList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemovePile, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.piles.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("pile index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.piles.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { piles: Some(En1993PileList { values }), ..Default::default() })
}
