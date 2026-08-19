//! ↩ Inverse constructor for `delete-story` — captures the removed story's full payload and position.

use super::mutation::DeleteStory;
use crate::artifacts::layout::mutations::{create_story, LayoutMutation};
use crate::artifacts::layout::LayoutSnapshot;

//#region 🗑️DeleteStory
pub async fn inverse_delete_story(payload: &DeleteStory, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.stories.iter().position(|story| story.id == payload.id) {
        Some(index) => vec![LayoutMutation::CreateStory(create_story::mutation::CreateStory { story: base.stories[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
//#endregion 🗑️DeleteStory
