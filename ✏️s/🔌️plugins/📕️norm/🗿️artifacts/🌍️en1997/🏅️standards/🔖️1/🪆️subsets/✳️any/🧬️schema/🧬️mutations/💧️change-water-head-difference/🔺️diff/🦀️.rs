//! 🔺️ `change-water-head-difference` diff: patches `head_difference` of the addressed layer water row.

use crate::change_water_head_difference::ChangeWaterHeadDifference;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LayerWatersPatch, En1997LayerWatersRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeWaterHeadDifference, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_head_difference.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.layer_waters.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("water {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.head_difference == payload.new_head_difference {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The layer water already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { layer_waters: Some(En1997LayerWatersRows::modification(&payload.id, En1997LayerWatersPatch { head_difference: Some(payload.new_head_difference), ..Default::default() })), ..Default::default() })
}
