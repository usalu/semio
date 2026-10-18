//! ↩️ Inverse of `DeleteDoorType`: the setters of the properties and classifications of the type, then the concrete `CreateDoorType` carrying the full removed record, none when the door type was absent.

use crate::standards::v1::subsets::any::schema::mutations::create_door_type::CreateDoorType;
use crate::standards::v1::subsets::any::schema::mutations::cascade;
use crate::standards::v1::subsets::any::schema::mutations::delete_door_type::DeleteDoorType;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot};

pub fn inverse<Op>(payload: &DeleteDoorType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: crate::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse + From<crate::standards::v1::subsets::any::schema::mutations::create_door_type::CreateDoorType>,
{
    match base.door_types.get(&payload.id) {
        Some(record) => cascade::data_rows(base, &payload.id).into_iter().chain(std::iter::once(Op::from(CreateDoorType { id: payload.id.clone(), door_type: record.clone() }))).collect(),
        None => Vec::new(),
    }
}
