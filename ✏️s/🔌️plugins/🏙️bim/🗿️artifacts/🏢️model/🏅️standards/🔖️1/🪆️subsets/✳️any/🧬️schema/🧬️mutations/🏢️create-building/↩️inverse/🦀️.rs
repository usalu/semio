//! ↩️ Inverse of `CreateBuilding`: the concrete `DeleteBuilding` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_building::DeleteBuilding;
use crate::standards::v1::subsets::any::schema::mutations::create_building::CreateBuilding;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateBuilding, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_building::DeleteBuilding>,
{
    if base.buildings.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteBuilding { id: payload.id.clone() })]
}
