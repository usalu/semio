//! ↩️ `change-airtightness-class` — undo restores BASE's `airtightness_class`.

use super::mutation::ChangeAirtightnessClass;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeAirtightnessClass, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeAirtightnessClass(ChangeAirtightnessClass { new_airtightness_class: base.airtightness_class.clone() })]
}
//#endregion 🔖️Inverse
