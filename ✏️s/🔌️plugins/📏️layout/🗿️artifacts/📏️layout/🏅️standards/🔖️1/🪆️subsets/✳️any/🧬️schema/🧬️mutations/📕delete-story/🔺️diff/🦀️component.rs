//! 🔺 Diff constructor for `delete-story`.

use super::mutation::DeleteStory;
use crate::artifacts::layout::schema::diff::LayoutStoriesDelta;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🗑️DeleteStory
pub fn diff_delete_story(payload: &DeleteStory, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if !base.stories.iter().any(|story| story.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Story \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { stories: Some(LayoutStoriesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🗑️DeleteStory
