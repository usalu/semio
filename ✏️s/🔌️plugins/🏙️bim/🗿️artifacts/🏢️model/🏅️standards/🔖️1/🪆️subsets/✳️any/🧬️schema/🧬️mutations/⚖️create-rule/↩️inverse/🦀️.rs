//! ↩️ Inverse of `CreateRule`: the concrete `DeleteRule` of the id it created, none when the id was already taken.

use super::super::delete_rule::DeleteRule;
use super::CreateRule;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateRule, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.rules.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteRule(DeleteRule { id: payload.id.clone() })]
}
