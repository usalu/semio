//! ↩️ Inverse of `CreateViewport`: the concrete `DeleteViewport` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_105::standards::v1::subsets::any::schema::mutations::delete_viewport::DeleteViewport;
use crate::standards::v1::subsets::any::schema::mutations::create_viewport::CreateViewport;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateViewport, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_105::standards::v1::subsets::any::schema::mutations::delete_viewport::DeleteViewport>,
{
    if base.viewports.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteViewport { id: payload.id.clone() })]
}
