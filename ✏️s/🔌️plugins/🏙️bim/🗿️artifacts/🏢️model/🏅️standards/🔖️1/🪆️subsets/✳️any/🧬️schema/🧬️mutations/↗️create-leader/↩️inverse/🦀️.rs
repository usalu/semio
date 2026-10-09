//! ↩️ Inverse of `CreateLeader`: the concrete `DeleteLeader` of the id it created, none when the id was already taken.

use super::super::delete_leader::DeleteLeader;
use super::CreateLeader;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateLeader, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.leaders.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteLeader(DeleteLeader { id: payload.id.clone() })]
}
