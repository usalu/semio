//! ↩️ Inverse of `SetColumn`: an absolute `SetColumn` restoring the base value of exactly the fields the forward really changes,
//! none when the column is absent or nothing would change.

use super::SetColumn;
use crate::{ModelMutation, ModelSnapshot};

fn restored<T: Clone + PartialEq>(value: &Option<T>, current: &T) -> Option<T> {
    value.as_ref().filter(|next| *next != current).map(|_| current.clone())
}

pub fn inverse(payload: &SetColumn, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(column) = base.columns.get(&payload.id) else {
        return Vec::new();
    };
    let restore = SetColumn {
        id: payload.id.clone(),
        column_type: restored(&payload.column_type, &column.column_type),
        position: restored(&payload.position, &column.position),
        rotation: restored(&payload.rotation, &column.rotation),
        base_offset: restored(&payload.base_offset, &column.base_offset),
        top: restored(&payload.top, &column.top),
        name: restored(&payload.name, &column.name),
    };
    let nothing = SetColumn { id: payload.id.clone(), column_type: None, position: None, rotation: None, base_offset: None, top: None, name: None };
    if restore == nothing {
        return Vec::new();
    }
    vec![ModelMutation::SetColumn(restore)]
}
