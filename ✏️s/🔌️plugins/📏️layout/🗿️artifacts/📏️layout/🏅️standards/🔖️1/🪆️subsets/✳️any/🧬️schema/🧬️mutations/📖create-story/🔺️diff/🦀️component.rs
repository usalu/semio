//! 🔺 Diff constructor for `create-story`.

use super::mutation::CreateStory;
use crate::artifacts::layout::schema::diff::LayoutStoriesDelta;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 📖CreateStory
pub fn diff_create_story(payload: &CreateStory, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff { stories: Some(LayoutStoriesDelta { added: vec![payload.story.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 📖CreateStory
