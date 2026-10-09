//! ↩️ Inverse of `CreateSheet`: the concrete `DeleteSheet` of the id it created, none when the id was already taken.

use super::super::delete_sheet::DeleteSheet;
use super::CreateSheet;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateSheet, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.sheets.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteSheet(DeleteSheet { id: payload.id.clone() })]
}
