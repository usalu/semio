//! 🔺️ `change-pile-material-id` diff: patches `material_id` of the addressed pile row.

use crate::change_pile_material_id::ChangePileMaterialId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PileDelta, En1993PilePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangePileMaterialId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.piles.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "pile", Vec::<String>::new());
    };
    if row.material_id == payload.new_material_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pile already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { piles: En1993PileDelta::modification(payload.id.clone(), En1993PilePatch { material_id: Some(payload.new_material_id.clone()), ..Default::default() }), ..Default::default() })
}
