//! ↩️ Inverse of `SetSheet`: an absolute `SetSheet` restoring the base value of exactly the fields the forward really changes, none when the sheet is absent or nothing changes.

use super::SetSheet;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetSheet, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.sheets.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).negate(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetSheet(SetSheet::from_patch(payload.id.clone(), restore))]
}
