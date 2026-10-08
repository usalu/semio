//! ↩️ Inverse of `SetGridLine`: an absolute `SetGridLine` restoring the base value of exactly the fields the forward really changes, none when the grid line is absent or nothing changes.

use super::SetGridLine;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetGridLine, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.grids.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).negate(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetGridLine(SetGridLine::from_patch(payload.id.clone(), restore))]
}
