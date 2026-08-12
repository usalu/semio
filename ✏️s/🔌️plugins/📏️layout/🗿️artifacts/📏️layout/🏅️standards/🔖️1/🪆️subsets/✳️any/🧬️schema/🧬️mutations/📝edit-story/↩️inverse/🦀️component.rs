//! ↩ Inverse constructor for `edit-story` — reconstructed from captured BASE state.

use super::mutation::EditStory;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;

//#region 📝EditStory
pub fn inverse_edit_story(payload: &EditStory, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.stories.iter().find(|story| story.id == payload.id) {
        Some(story) => vec![LayoutMutation::EditStory(EditStory { id: payload.id.clone(), new_content: story.content.clone() })],
        None => Vec::new(),
    }
}
//#endregion 📝EditStory
