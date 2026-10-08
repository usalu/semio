//! ↩️ Inverse of `DeleteColumnType`: the concrete `CreateColumnType` carrying the full removed record, none when the column type was absent.

use super::super::create_column_type::CreateColumnType;
use super::DeleteColumnType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteColumnType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.column_types.get(&payload.id) {
        Some(record) => vec![ModelMutation::CreateColumnType(CreateColumnType { id: payload.id.clone(), column_type: record.clone() })],
        None => Vec::new(),
    }
}
