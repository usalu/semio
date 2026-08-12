//! 🔺 Diff constructor for `edit-story`.

use super::mutation::EditStory;
use crate::artifacts::layout::schema::diff::{LayoutStoriesDelta, LayoutStoryPatchEntry};
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, TextStoryPatch};

//#region 📝EditStory
pub fn diff_edit_story(payload: &EditStory, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff {
        stories: Some(LayoutStoriesDelta {
            patched: vec![LayoutStoryPatchEntry { id: payload.id.clone(), patch: TextStoryPatch { content: Some(payload.new_content.clone()) } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 📝EditStory
