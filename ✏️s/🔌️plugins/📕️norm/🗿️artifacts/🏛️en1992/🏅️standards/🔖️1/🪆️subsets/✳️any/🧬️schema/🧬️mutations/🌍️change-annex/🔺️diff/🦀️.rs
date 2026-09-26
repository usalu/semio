use crate::diff::En1992Diff;
use crate::mutations::change_annex::ChangeAnnex;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeAnnex, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Annex already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { annex: Some(payload.new_annex), ..Default::default() })
}
