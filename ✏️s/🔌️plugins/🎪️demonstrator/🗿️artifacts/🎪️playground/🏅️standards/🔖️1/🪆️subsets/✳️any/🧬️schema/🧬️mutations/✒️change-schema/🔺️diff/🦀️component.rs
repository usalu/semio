//! 🔺️ `change-schema` — sparse diff construction.

use super::mutation::ChangeSchema;
use crate::artifacts::playground::{PlaygroundDiff, PlaygroundSnapshot};

//#region 🔖️Diff
/// 🔺️ The `schema` slot is the only sparse field this payload ever touches.
pub fn diff(payload: &ChangeSchema, _base: &PlaygroundSnapshot) -> PlaygroundDiff {
    PlaygroundDiff { schema: Some(payload.new_schema.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
