//! 🔺️ Diff for `change-assumed-silo-wall-friction`.
use crate::change_assumed_silo_wall_friction::ChangeAssumedSiloWallFriction;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedSiloWallFriction, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_silo_wall_friction == payload.new_assumed_silo_wall_friction {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_silo_wall_friction: Some(payload.new_assumed_silo_wall_friction), ..Default::default() })
}
