//! 🔺 Diff constructor for `edit-story`.

use super::mutation::EditStory;
use crate::artifacts::layout::schema::diff::{LayoutStoriesDelta, LayoutStoryPatchEntry};
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, TextStoryPatch};

//#region 📝EditStory
pub fn diff_edit_story(payload: &EditStory, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(story) = base.stories.iter().find(|story| story.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Story \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if story.content == payload.new_content {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Story \"{}\" content is unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        stories: Some(LayoutStoriesDelta {
            patched: vec![LayoutStoryPatchEntry { id: payload.id.clone(), patch: TextStoryPatch { content: Some(payload.new_content.clone()) } }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 📝EditStory
