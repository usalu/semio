//! ↩️ Inverse of `CreateAreaScheme`: the concrete `DeleteAreaScheme` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_area_scheme::DeleteAreaScheme;
use crate::standards::v1::subsets::any::schema::mutations::elements;
use crate::standards::v1::subsets::any::schema::mutations::create_area_scheme::CreateAreaScheme;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateAreaScheme, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_area_scheme::DeleteAreaScheme>,
{
    if elements::taken(base, &payload.id).is_some() {
        return Vec::new();
    }
    vec![Op::from(DeleteAreaScheme { id: payload.id.clone() })]
}
