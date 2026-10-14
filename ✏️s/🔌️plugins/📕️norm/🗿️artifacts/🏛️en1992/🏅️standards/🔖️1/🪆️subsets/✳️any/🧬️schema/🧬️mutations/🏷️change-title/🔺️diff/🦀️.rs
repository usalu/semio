use semio_s_artifact_norm_en1992_diff::En1992Diff;
use crate::change_title::ChangeTitle;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeTitle, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.title == payload.new_title {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff { title: Some(payload.new_title.clone()), ..Default::default() })
}
