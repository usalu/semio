//! ↩️ Inverse of `CreateDimension`: the concrete `DeleteDimension` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_95::standards::v1::subsets::any::schema::mutations::delete_dimension::DeleteDimension;
use crate::standards::v1::subsets::any::schema::mutations::create_dimension::CreateDimension;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateDimension, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_95::standards::v1::subsets::any::schema::mutations::delete_dimension::DeleteDimension>,
{
    if base.dimensions.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteDimension { id: payload.id.clone() })]
}
