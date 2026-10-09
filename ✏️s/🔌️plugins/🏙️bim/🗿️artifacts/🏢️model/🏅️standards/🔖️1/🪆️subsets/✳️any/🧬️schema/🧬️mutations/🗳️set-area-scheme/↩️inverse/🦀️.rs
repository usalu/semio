//! ↩️ Inverse of `SetAreaScheme`: an absolute `SetAreaScheme` restoring the base value of exactly the fields the forward really changes, none when the scheme is absent or nothing changes.

use super::SetAreaScheme;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetAreaScheme, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.area_schemes.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetAreaScheme(SetAreaScheme::from_patch(payload.id.clone(), restore))]
}
