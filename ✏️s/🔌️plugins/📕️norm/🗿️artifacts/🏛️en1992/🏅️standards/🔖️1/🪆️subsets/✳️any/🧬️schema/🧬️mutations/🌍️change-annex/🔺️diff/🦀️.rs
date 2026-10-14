use semio_s_artifact_norm_en1992_diff::En1992Diff;
use crate::change_annex::ChangeAnnex;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeAnnex, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Annex already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { annex: Some(payload.new_annex), ..Default::default() })
}
