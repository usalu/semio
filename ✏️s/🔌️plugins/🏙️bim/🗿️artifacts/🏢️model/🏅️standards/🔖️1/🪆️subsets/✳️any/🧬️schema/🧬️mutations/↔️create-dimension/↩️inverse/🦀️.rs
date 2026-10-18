//! ↩️ Inverse of `CreateDimension`: the concrete `DeleteDimension` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_dimension::DeleteDimension;
use crate::standards::v1::subsets::any::schema::mutations::create_dimension::CreateDimension;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateDimension, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_dimension::DeleteDimension>,
{
    if base.dimensions.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteDimension { id: payload.id.clone() })]
}
