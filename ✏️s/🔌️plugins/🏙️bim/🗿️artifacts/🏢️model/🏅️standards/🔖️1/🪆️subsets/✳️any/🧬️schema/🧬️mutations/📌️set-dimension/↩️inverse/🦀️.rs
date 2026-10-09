//! ↩️ Inverse of `SetDimension`: an absolute `SetDimension` restoring the base value of exactly the fields the forward really changes, none when the dimension is absent or nothing changes.

use super::SetDimension;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetDimension, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.dimensions.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetDimension(SetDimension::from_patch(payload.id.clone(), restore))]
}
