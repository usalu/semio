//! ↩️ Inverse of `ArrayElements`: one `DeleteElements` of the minted ids of every copied element in every copy; the cascade removes their
//! openings and their property and classification entries, which is exactly what the array created. Empty when the array is refused.

use crate::standards::v1::subsets::any::schema::mutations::delete_elements::DeleteElements;
use crate::standards::v1::subsets::any::schema::mutations::modify;
use crate::standards::v1::subsets::any::schema::mutations::array_elements::diff::maps_of;
use crate::standards::v1::subsets::any::schema::mutations::array_elements::ArrayElements;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &ArrayElements, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_elements::DeleteElements>,
{
    let Some(maps) = maps_of(payload) else {
        return Vec::new();
    };
    match modify::duplicate(base, &payload.ids, &payload.prefix, &maps) {
        Ok(built) if !built.roots.is_empty() => vec![Op::from(DeleteElements { ids: built.roots })],
        _ => Vec::new(),
    }
}
