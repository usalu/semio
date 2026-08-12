//! ↩️ `change-schema` — undo reconstructed from BASE state.

use super::mutation::ChangeSchema;
use crate::artifacts::playground::{PlaygroundMutation, PlaygroundSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSchema, base: &PlaygroundSnapshot) -> Vec<PlaygroundMutation> {
    vec![PlaygroundMutation::ChangeSchema(ChangeSchema { new_schema: base.schema.clone() })]
}
//#endregion 🔖️Inverse
