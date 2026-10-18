//! ↩️ Inverse of `CreateZone`: the concrete `DeleteZone` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_zone::DeleteZone;
use crate::standards::v1::subsets::any::schema::mutations::elements;
use crate::standards::v1::subsets::any::schema::mutations::create_zone::CreateZone;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateZone, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_zone::DeleteZone>,
{
    if elements::taken(base, &payload.id).is_some() {
        return Vec::new();
    }
    vec![Op::from(DeleteZone { id: payload.id.clone() })]
}
