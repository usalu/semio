//! ↩️ Inverse of `CreateSlab`: the concrete `DeleteSlab` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_slab::DeleteSlab;
use crate::standards::v1::subsets::any::schema::mutations::create_slab::CreateSlab;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateSlab, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_slab::DeleteSlab>,
{
    if base.slabs.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteSlab { id: payload.id.clone() })]
}
