//! 🏠 `change-roof-has-parapet` diff: patches `has_parapet` of the addressed roof row.

use crate::change_roof_has_parapet::ChangeRoofHasParapet;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RoofDelta, En1991RoofPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRoofHasParapet, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.roofs.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Roof {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.has_parapet == payload.new_has_parapet {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The roof already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { roofs: En1991RoofDelta::modification(&payload.id, En1991RoofPatch { has_parapet: Some(payload.new_has_parapet), ..Default::default() }), ..Default::default() })
}
