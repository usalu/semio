use super::ChangeDesignApproach;
use crate::diff::En1997Diff;
use crate::En1997Snapshot;
pub fn diff(payload: &ChangeDesignApproach, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.design_approach == payload.new_design_approach {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "approach unchanged");
    }
    protocol::MutationOutcome::new(En1997Diff { design_approach: Some(payload.new_design_approach.clone()), ..Default::default() })
}
