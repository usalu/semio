use crate::change_annex::ChangeAnnex;
use semio_s_artifact_norm_en1997_diff::En1997Diff;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn diff(payload: &ChangeAnnex, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "annex unchanged");
    }
    protocol::MutationOutcome::new(En1997Diff { annex: Some(payload.new_annex), ..Default::default() })
}
