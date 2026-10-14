use crate::change_design_situation::ChangeDesignSituation;
use semio_s_artifact_norm_en1997_diff::En1997Diff;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn diff(payload: &ChangeDesignSituation, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.design_situation == payload.new_design_situation {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "situation unchanged");
    }
    protocol::MutationOutcome::new(En1997Diff { design_situation: Some(payload.new_design_situation.clone()), ..Default::default() })
}
