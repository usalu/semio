//! ↩️ Inverse of `CreateFamilySolid`: the concrete `DeleteFamilySolid` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_family_solid::DeleteFamilySolid;
use crate::standards::v1::subsets::any::schema::mutations::create_family_solid::CreateFamilySolid;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateFamilySolid, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_family_solid::DeleteFamilySolid>,
{
    if base.family_solids.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteFamilySolid { id: payload.id.clone() })]
}
