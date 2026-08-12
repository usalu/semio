//! ↩ Inverse constructor for `create-story` — always undoes to `delete-story`.

use super::mutation::CreateStory;
use crate::artifacts::layout::mutations::{delete_story, LayoutMutation};
use crate::artifacts::layout::LayoutSnapshot;

//#region 📖CreateStory
pub fn inverse_create_story(payload: &CreateStory, _base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::DeleteStory(delete_story::mutation::DeleteStory { id: payload.story.id.clone() })]
}
//#endregion 📖CreateStory
