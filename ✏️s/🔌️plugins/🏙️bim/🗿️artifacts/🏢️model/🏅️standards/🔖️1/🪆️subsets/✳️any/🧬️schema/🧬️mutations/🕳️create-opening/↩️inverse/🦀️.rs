//! ↩️ Inverse of `CreateOpening`: the concrete `DeleteOpening` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_opening::DeleteOpening;
use crate::standards::v1::subsets::any::schema::mutations::create_opening::CreateOpening;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateOpening, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_opening::DeleteOpening>,
{
    if base.openings.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteOpening { id: payload.id.clone() })]
}
