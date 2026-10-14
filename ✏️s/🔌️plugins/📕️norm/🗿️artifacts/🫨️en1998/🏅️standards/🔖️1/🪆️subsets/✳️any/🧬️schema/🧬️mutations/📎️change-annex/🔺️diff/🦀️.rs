//! 📎️ `change-annex` diff — sets the national annex; an annex the document already holds is a `mutation.no-op`.
use crate::change_annex::ChangeAnnex;
use semio_s_artifact_norm_en1998_diff::En1998Diff;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeAnnex, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "annex unchanged");
    }
    protocol::MutationOutcome::new(En1998Diff { annex: Some(payload.new_annex.clone()), ..Default::default() })
}
