//! ↩️ Inverse of `CreateBeam`: the concrete `DeleteBeam` of the id it created, none when the id was already taken.

use super::super::delete_beam::DeleteBeam;
use super::CreateBeam;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateBeam, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.beams.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteBeam(DeleteBeam { id: payload.id.clone() })]
}
