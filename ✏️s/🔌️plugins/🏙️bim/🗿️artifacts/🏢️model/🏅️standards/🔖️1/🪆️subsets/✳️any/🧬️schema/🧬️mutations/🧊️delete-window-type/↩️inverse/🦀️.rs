//! ↩️ Inverse of `DeleteWindowType`: the concrete `CreateWindowType` carrying the full removed record, none when the window type was absent.

use super::super::create_window_type::CreateWindowType;
use super::DeleteWindowType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteWindowType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.window_types.get(&payload.id) {
        Some(record) => vec![ModelMutation::CreateWindowType(CreateWindowType { id: payload.id.clone(), window_type: record.clone() })],
        None => Vec::new(),
    }
}
