//! 🔺 Diff constructor for `change-data-fields`.

use super::mutation::ChangeDataFields;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🧾ChangeDataFields
pub fn diff_change_data_fields(payload: &ChangeDataFields, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff { data_fields_json: Some(payload.new_json.clone()), ..Default::default() }
}
//#endregion 🧾ChangeDataFields
