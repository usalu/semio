//! ↩️ Inverse of `DeleteBeamType`: the setters of the properties and classifications of the type, then the concrete `CreateBeamType` carrying the full removed record, none when the beam type was absent.

use super::super::create_beam_type::CreateBeamType;
use super::super::cascade;
use super::DeleteBeamType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteBeamType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.beam_types.get(&payload.id) {
        Some(record) => cascade::data_rows(base, &payload.id).into_iter().chain(std::iter::once(ModelMutation::CreateBeamType(CreateBeamType { id: payload.id.clone(), beam_type: record.clone() }))).collect(),
        None => Vec::new(),
    }
}
