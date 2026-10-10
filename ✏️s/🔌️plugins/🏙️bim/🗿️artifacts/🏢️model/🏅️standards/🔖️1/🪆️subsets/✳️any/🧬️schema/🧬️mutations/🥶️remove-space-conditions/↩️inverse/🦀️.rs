//! ↩️ Inverse of `RemoveSpaceConditions`: one absolute `SetSpaceConditions` that states every field of the removed record (an unstated field as an assigned null), none when the space has no conditions.

use super::RemoveSpaceConditions;
use crate::{ModelMutation, ModelSnapshot};
use super::super::set_space_conditions::SetSpaceConditions;

pub fn inverse(payload: &RemoveSpaceConditions, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.space_conditions.get(&payload.id) else {
        return Vec::new();
    };
    vec![ModelMutation::SetSpaceConditions(SetSpaceConditions::stating(&payload.id, record))]
}
