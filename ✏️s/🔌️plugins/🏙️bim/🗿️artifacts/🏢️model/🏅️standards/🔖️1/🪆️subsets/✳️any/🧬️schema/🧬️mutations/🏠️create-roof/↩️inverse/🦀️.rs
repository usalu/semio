//! ↩️ Inverse of `CreateRoof`: the concrete `DeleteRoof` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_roof::DeleteRoof;
use crate::standards::v1::subsets::any::schema::mutations::create_roof::CreateRoof;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateRoof, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_roof::DeleteRoof>,
{
    if base.roofs.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteRoof { id: payload.id.clone() })]
}
