use super::ChangeDesignSituation;
use crate::diff::En1997Diff;
use crate::En1997Snapshot;
pub fn diff(payload: &ChangeDesignSituation, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.design_situation == payload.new_design_situation {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "situation unchanged");
    }
    protocol::MutationOutcome::new(En1997Diff { design_situation: Some(payload.new_design_situation.clone()), ..Default::default() })
}
