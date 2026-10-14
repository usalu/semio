//! 🔺️ Diff for `change-fire-load-density-qf`.
use crate::change_fire_load_density_qf::ChangeFireLoadDensityQf;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeFireLoadDensityQf, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_load_density_qf == payload.new_fire_load_density_qf {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_load_density_qf: Some(payload.new_fire_load_density_qf), ..Default::default() })
}
