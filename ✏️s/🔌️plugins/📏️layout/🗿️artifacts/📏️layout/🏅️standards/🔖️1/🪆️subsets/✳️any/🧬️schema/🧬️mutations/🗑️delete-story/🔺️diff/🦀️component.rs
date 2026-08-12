//! 🔺 Diff constructor for `delete-story`.

use super::mutation::DeleteStory;
use crate::artifacts::layout::schema::diff::LayoutStoriesDelta;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🗑️DeleteStory
pub fn diff_delete_story(payload: &DeleteStory, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff { stories: Some(LayoutStoriesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🗑️DeleteStory
