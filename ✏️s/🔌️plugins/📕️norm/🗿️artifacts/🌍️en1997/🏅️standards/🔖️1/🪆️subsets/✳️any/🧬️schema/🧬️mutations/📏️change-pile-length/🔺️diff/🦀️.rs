use super::ChangePileLength;
use crate::diff::{En1997Diff, En1997PileList};
use crate::En1997Snapshot;
pub fn diff(payload: &ChangePileLength, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_length.is_finite() || payload.new_length <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "pile length must be positive", vec![payload.id.clone()]);
    }
    let Some(idx) = base.piles.iter().position(|f| f.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile {} missing", payload.id), vec![payload.id.clone()]);
    };
    let mut piles = base.piles.clone();
    piles[idx].length = payload.new_length;
    protocol::MutationOutcome::new(En1997Diff { piles: Some(En1997PileList { values: piles }), ..Default::default() })
}
