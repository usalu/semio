//! ↩️ Inverse of `DeleteCeilingType`: the setters of the properties and classifications of the type, then the concrete `CreateCeilingType` carrying the full removed record, none when the ceiling type was absent.

use super::super::create_ceiling_type::CreateCeilingType;
use super::super::cascade;
use super::DeleteCeilingType;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot};

pub fn inverse<Op>(payload: &DeleteCeilingType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: crate::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse + From<crate::standards::v1::subsets::any::schema::mutations::create_ceiling_type::CreateCeilingType>,
{
    match base.ceiling_types.get(&payload.id) {
        Some(ceiling_type) => cascade::data_rows(base, &payload.id).into_iter().chain(std::iter::once(Op::from(CreateCeilingType { id: payload.id.clone(), ceiling_type: ceiling_type.clone() }))).collect(),
        None => Vec::new(),
    }
}
