//! ↩️ Inverse of `DeleteColumnType`: the setters of the properties and classifications of the type, then the concrete `CreateColumnType` carrying the full removed record, none when the column type was absent.

use super::super::create_column_type::CreateColumnType;
use super::super::cascade;
use super::DeleteColumnType;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot};

pub fn inverse<Op>(payload: &DeleteColumnType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: crate::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse + From<crate::standards::v1::subsets::any::schema::mutations::create_column_type::CreateColumnType>,
{
    match base.column_types.get(&payload.id) {
        Some(record) => cascade::data_rows(base, &payload.id).into_iter().chain(std::iter::once(Op::from(CreateColumnType { id: payload.id.clone(), column_type: record.clone() }))).collect(),
        None => Vec::new(),
    }
}
