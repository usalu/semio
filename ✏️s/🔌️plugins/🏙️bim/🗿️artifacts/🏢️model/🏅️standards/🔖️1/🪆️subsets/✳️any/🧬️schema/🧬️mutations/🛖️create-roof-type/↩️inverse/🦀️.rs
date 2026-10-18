//! ↩️ Inverse of `CreateRoofType`: the concrete `DeleteRoofType` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_roof_type::DeleteRoofType;
use crate::standards::v1::subsets::any::schema::mutations::create_roof_type::CreateRoofType;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateRoofType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_roof_type::DeleteRoofType>,
{
    if base.roof_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteRoofType { id: payload.id.clone() })]
}
