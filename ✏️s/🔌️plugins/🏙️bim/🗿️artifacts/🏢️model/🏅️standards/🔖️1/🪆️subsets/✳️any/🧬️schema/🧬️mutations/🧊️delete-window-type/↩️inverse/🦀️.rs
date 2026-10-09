//! ↩️ Inverse of `DeleteWindowType`: the setters of the properties and classifications of the type, then the concrete `CreateWindowType` carrying the full removed record, none when the window type was absent.

use super::super::create_window_type::CreateWindowType;
use super::super::cascade;
use super::DeleteWindowType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteWindowType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.window_types.get(&payload.id) {
        Some(record) => cascade::data_rows(base, &payload.id).into_iter().chain(std::iter::once(ModelMutation::CreateWindowType(CreateWindowType { id: payload.id.clone(), window_type: record.clone() }))).collect(),
        None => Vec::new(),
    }
}
