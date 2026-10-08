//! ↩️ Inverse of `CreateColumn`: the concrete `DeleteColumn` of the id it created, none when the id was already taken.

use super::super::delete_column::DeleteColumn;
use super::CreateColumn;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateColumn, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.columns.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteColumn(DeleteColumn { id: payload.id.clone() })]
}
