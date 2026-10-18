//! ↩️ Inverse of `CreateClashSet`: the concrete `DeleteClashSet` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_clash_set::DeleteClashSet;
use crate::standards::v1::subsets::any::schema::mutations::create_clash_set::CreateClashSet;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateClashSet, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_clash_set::DeleteClashSet>,
{
    if base.clash_sets.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteClashSet { id: payload.id.clone() })]
}
