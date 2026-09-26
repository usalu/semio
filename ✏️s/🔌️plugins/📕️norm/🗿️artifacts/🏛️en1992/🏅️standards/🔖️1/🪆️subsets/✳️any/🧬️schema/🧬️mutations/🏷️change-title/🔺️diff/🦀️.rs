use crate::diff::En1992Diff;
use crate::mutations::change_title::ChangeTitle;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeTitle, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.title == payload.new_title {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff { title: Some(payload.new_title.clone()), ..Default::default() })
}
