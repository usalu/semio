//! 🔺 Diff constructor for `change-data-fields`.

use super::mutation::ChangeDataFields;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🧾ChangeDataFields
pub async fn diff_change_data_fields(payload: &ChangeDataFields, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.data_fields_json == payload.new_json {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Data fields are already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff { data_fields_json: Some(payload.new_json.clone()), ..Default::default() })
}
//#endregion 🧾ChangeDataFields
