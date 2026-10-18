//! ↩️ Inverse of `CreateFamily`: the concrete `DeleteFamily` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_family::DeleteFamily;
use crate::standards::v1::subsets::any::schema::mutations::create_family::CreateFamily;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateFamily, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_family::DeleteFamily>,
{
    if base.families.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteFamily { id: payload.id.clone() })]
}
