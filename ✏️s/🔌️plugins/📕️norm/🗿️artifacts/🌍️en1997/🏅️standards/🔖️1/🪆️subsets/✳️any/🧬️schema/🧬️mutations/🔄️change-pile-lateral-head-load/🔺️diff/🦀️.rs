//! 🔺️ `change-pile-lateral-head-load` diff: patches `lateral_head_load` of the addressed pile row.

use crate::change_pile_lateral_head_load::ChangePileLateralHeadLoad;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PilesPatch, En1997PilesRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangePileLateralHeadLoad, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_lateral_head_load.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.piles.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.lateral_head_load == payload.new_lateral_head_load {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pile already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { piles: Some(En1997PilesRows::modification(&payload.id, En1997PilesPatch { lateral_head_load: Some(payload.new_lateral_head_load), ..Default::default() })), ..Default::default() })
}
