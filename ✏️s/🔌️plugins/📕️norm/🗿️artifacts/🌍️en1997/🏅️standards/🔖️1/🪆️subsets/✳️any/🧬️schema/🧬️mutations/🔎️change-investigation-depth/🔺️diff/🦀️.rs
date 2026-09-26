use super::ChangeInvestigationDepth;
use crate::diff::En1997Diff;
use crate::En1997Snapshot;
pub fn diff(payload: &ChangeInvestigationDepth, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_investigation_depth.is_finite() || payload.new_investigation_depth <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "investigation depth must be positive", Vec::<String>::new());
    }
    if base.investigation_depth == payload.new_investigation_depth {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "depth unchanged");
    }
    protocol::MutationOutcome::new(En1997Diff { investigation_depth: Some(payload.new_investigation_depth), ..Default::default() })
}
