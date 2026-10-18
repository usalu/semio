//! ↩️ Inverse of `CreateSheet`: the concrete `DeleteSheet` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_sheet::DeleteSheet;
use crate::standards::v1::subsets::any::schema::mutations::create_sheet::CreateSheet;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateSheet, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_sheet::DeleteSheet>,
{
    if base.sheets.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteSheet { id: payload.id.clone() })]
}
