//! ↩️ Inverse of `SetSheet`: an absolute `SetSheet` restoring the base value of exactly the fields the forward really changes, none when the sheet is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_sheet::SetSheet;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetSheet, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_sheet::SetSheet>,
{
    let Some(record) = base.sheets.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetSheet::from_patch(payload.id.clone(), restore))]
}
