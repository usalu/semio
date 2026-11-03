use crate::change_uls_equation::ChangeUlsEquation;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn diff(payload: &ChangeUlsEquation, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.uls_equation == payload.new_value {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "ULS equation already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { uls_equation: Some(payload.new_value), ..Default::default() })
}
