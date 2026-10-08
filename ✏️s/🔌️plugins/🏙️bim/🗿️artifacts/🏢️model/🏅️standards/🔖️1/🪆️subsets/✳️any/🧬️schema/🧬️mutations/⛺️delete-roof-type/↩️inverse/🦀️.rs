//! ↩️ Inverse of `DeleteRoofType`: the concrete `CreateRoofType` carrying the full removed record, none when the roof type was absent.

use super::super::create_roof_type::CreateRoofType;
use super::DeleteRoofType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteRoofType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.roof_types.get(&payload.id) {
        Some(roof_type) => vec![ModelMutation::CreateRoofType(CreateRoofType { id: payload.id.clone(), roof_type: roof_type.clone() })],
        None => Vec::new(),
    }
}
