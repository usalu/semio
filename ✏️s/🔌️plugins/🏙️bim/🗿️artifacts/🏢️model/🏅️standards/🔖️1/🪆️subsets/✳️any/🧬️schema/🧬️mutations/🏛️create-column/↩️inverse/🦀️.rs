//! ↩️ Inverse of `CreateColumn`: the concrete `DeleteColumn` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_column::DeleteColumn;
use crate::standards::v1::subsets::any::schema::mutations::create_column::CreateColumn;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateColumn, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_column::DeleteColumn>,
{
    if base.columns.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteColumn { id: payload.id.clone() })]
}
