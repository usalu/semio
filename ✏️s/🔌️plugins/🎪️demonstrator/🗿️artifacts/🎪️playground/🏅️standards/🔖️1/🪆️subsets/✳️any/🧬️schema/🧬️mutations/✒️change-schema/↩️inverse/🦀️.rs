//! ↩️ `change-schema` — undo reconstructed from BASE state.

use super::ChangeSchema;
use crate::standards::v1::subsets::any::schema::{mutations::PlaygroundMutation, snapshot::PlaygroundSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSchema, base: &PlaygroundSnapshot) -> Result<Vec<PlaygroundMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![PlaygroundMutation::ChangeSchema(ChangeSchema { new_schema: base.schema.clone() })]

    })())
}
//#endregion 🔖️Inverse
