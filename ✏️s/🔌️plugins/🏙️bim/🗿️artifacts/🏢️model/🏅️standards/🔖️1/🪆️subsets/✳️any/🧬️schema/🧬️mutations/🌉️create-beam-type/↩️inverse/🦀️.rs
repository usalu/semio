//! ↩️ Inverse of `CreateBeamType`: the concrete `DeleteBeamType` of the id it created, none when the id was already taken.

use super::super::delete_beam_type::DeleteBeamType;
use super::CreateBeamType;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &CreateBeamType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.beam_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteBeamType(DeleteBeamType { id: payload.id.clone() })]
}
