//! ↩️ Inverse of `CreateRailing`: the concrete `DeleteRailing` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_railing::DeleteRailing;
use crate::standards::v1::subsets::any::schema::mutations::create_railing::CreateRailing;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateRailing, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_railing::DeleteRailing>,
{
    if base.railings.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteRailing { id: payload.id.clone() })]
}
