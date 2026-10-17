//! ↩️ Inverse of `DeleteWallType`: the setters of the properties and classifications of the type, then the concrete `CreateWallType` carrying the full removed record, none when the wall type was absent.

use super::super::create_wall_type::CreateWallType;
use super::super::cascade;
use super::DeleteWallType;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot};

pub fn inverse<Op>(payload: &DeleteWallType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: crate::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse + From<crate::standards::v1::subsets::any::schema::mutations::create_wall_type::CreateWallType>,
{
    match base.wall_types.get(&payload.id) {
        Some(wall_type) => cascade::data_rows(base, &payload.id).into_iter().chain(std::iter::once(Op::from(CreateWallType { id: payload.id.clone(), wall_type: wall_type.clone() }))).collect(),
        None => Vec::new(),
    }
}
