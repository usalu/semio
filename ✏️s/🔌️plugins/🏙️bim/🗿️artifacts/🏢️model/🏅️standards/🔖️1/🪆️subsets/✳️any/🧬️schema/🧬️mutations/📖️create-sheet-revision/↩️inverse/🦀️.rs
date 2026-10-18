//! ↩️ Inverse of `CreateSheetRevision`: the concrete `DeleteSheetRevision` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_sheet_revision::DeleteSheetRevision;
use crate::standards::v1::subsets::any::schema::mutations::create_sheet_revision::CreateSheetRevision;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateSheetRevision, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_sheet_revision::DeleteSheetRevision>,
{
    if base.sheet_revisions.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteSheetRevision { id: payload.id.clone() })]
}
