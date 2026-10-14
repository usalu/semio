//! ↩️ Inverse of `SetWindowType`: an absolute `SetWindowType` restoring the base value of exactly the fields the forward really changes, none when the window type is absent or nothing changes.

use super::SetWindowType;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot, diff::Patch};

pub fn inverse(payload: &SetWindowType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.window_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetWindowType(SetWindowType::from_patch(payload.id.clone(), restore))]
}
