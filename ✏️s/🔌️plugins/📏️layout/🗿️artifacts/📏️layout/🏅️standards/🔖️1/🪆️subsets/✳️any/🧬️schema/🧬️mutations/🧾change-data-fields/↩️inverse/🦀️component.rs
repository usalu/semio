//! ↩ Inverse constructor for `change-data-fields` — reconstructed from captured BASE state.

use super::mutation::ChangeDataFields;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;

//#region 🧾ChangeDataFields
pub async fn inverse_change_data_fields(_payload: &ChangeDataFields, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::ChangeDataFields(ChangeDataFields { new_json: base.data_fields_json.clone() })]
}
//#endregion 🧾ChangeDataFields
