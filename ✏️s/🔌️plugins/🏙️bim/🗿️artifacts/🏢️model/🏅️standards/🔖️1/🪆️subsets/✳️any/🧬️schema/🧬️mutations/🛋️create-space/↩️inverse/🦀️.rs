//! ↩️ Inverse of `CreateSpace`: the concrete `DeleteSpace` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_space::DeleteSpace;
use crate::standards::v1::subsets::any::schema::mutations::create_space::CreateSpace;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateSpace, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_space::DeleteSpace>,
{
    if base.spaces.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteSpace { id: payload.id.clone() })]
}
