//! ↩️ Inverse of `CreateSheet`: the concrete `DeleteSheet` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_104::standards::v1::subsets::any::schema::mutations::delete_sheet::DeleteSheet;
use crate::standards::v1::subsets::any::schema::mutations::create_sheet::CreateSheet;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateSheet, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_104::standards::v1::subsets::any::schema::mutations::delete_sheet::DeleteSheet>,
{
    if base.sheets.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteSheet { id: payload.id.clone() })]
}
