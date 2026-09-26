use super::ChangeAnnex;
use crate::diff::En1997Diff;
use crate::En1997Snapshot;
pub fn diff(payload: &ChangeAnnex, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "annex unchanged");
    }
    protocol::MutationOutcome::new(En1997Diff { annex: Some(payload.new_annex), ..Default::default() })
}
