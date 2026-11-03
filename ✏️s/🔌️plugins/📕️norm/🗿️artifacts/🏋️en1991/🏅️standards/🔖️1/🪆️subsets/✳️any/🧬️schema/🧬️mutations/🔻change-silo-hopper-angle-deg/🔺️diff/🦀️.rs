//! 🔺️ Diff for `change-silo-hopper-angle-deg`.
use crate::change_silo_hopper_angle_deg::ChangeSiloHopperAngleDeg;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeSiloHopperAngleDeg, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_hopper_angle_deg == payload.new_silo_hopper_angle_deg {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_hopper_angle_deg: Some(payload.new_silo_hopper_angle_deg), ..Default::default() })
}
