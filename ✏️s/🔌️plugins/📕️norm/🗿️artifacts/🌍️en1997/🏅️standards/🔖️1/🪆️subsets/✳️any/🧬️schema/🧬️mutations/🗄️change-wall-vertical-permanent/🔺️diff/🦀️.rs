//! 🔺️ `change-wall-vertical-permanent` diff: patches `vertical_permanent` of the addressed retaining wall row.

use crate::change_wall_vertical_permanent::ChangeWallVerticalPermanent;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997RetainingWallsPatch, En1997RetainingWallsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeWallVerticalPermanent, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_vertical_permanent.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.retaining_walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.vertical_permanent == payload.new_vertical_permanent {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The retaining wall already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { retaining_walls: Some(En1997RetainingWallsRows::modification(&payload.id, En1997RetainingWallsPatch { vertical_permanent: Some(payload.new_vertical_permanent), ..Default::default() })), ..Default::default() })
}
