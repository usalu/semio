use semio_s_artifact_norm_en1992_diff::En1992Diff;
use crate::change_delta_c_dev::ChangeDeltaCDev;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeDeltaCDev, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if (base.delta_c_dev - payload.new_delta_c_dev).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff { delta_c_dev: Some(payload.new_delta_c_dev), ..Default::default() })
}
