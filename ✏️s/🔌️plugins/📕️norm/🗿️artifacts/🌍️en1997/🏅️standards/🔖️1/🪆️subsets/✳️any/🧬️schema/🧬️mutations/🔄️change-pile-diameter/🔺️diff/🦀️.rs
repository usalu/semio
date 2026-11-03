//! 🔺️ `change-pile-diameter` diff: patches `diameter` of the addressed pile row.

use crate::change_pile_diameter::ChangePileDiameter;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PilesPatch, En1997PilesRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangePileDiameter, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_diameter.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.piles.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.diameter == payload.new_diameter {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pile already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { piles: Some(En1997PilesRows::modification(&payload.id, En1997PilesPatch { diameter: Some(payload.new_diameter), ..Default::default() })), ..Default::default() })
}
