//! ↩️ Inverse of `DeleteBeamType`: the concrete `CreateBeamType` carrying the full removed record, none when the beam type was absent.

use super::super::create_beam_type::CreateBeamType;
use super::DeleteBeamType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteBeamType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.beam_types.get(&payload.id) {
        Some(record) => vec![ModelMutation::CreateBeamType(CreateBeamType { id: payload.id.clone(), beam_type: record.clone() })],
        None => Vec::new(),
    }
}
