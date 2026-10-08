use super::ChangePileCount;
use crate::En1997Snapshot;
use crate::diff::{En1997Diff, En1997PilesRows, En1997PilesPatch};

pub fn diff(payload: &ChangePileCount, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.new_count == 0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "pile count must be >= 1", vec![payload.id.clone()]);
    }
    let Some(idx) = base.piles.iter().position(|f| f.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile {} missing", payload.id), vec![payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1997Diff {
        piles: Some(En1997PilesRows::modification(&payload.id, En1997PilesPatch { count: Some(payload.new_count), ..Default::default() })),
        ..Default::default()
    })
}
