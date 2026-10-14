use crate::change_design_approach::ChangeDesignApproach;
use semio_s_artifact_norm_en1997_diff::En1997Diff;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn diff(payload: &ChangeDesignApproach, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.design_approach == payload.new_design_approach {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "approach unchanged");
    }
    protocol::MutationOutcome::new(En1997Diff { design_approach: Some(payload.new_design_approach.clone()), ..Default::default() })
}
