//! 🔺 Diff constructor for `create-story`.

use super::mutation::CreateStory;
use crate::artifacts::layout::schema::diff::LayoutStoriesDelta;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 📖CreateStory
pub fn diff_create_story(payload: &CreateStory, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.stories.iter().any(|story| story.id == payload.story.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A story with id \"{}\" already exists.", payload.story.id), [payload.story.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { stories: Some(LayoutStoriesDelta { added: vec![payload.story.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 📖CreateStory
