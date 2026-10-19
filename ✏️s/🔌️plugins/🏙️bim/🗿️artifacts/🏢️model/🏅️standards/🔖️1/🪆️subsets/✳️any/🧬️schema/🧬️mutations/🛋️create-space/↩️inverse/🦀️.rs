//! ↩️ Inverse of `CreateSpace`: the concrete `DeleteSpace` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_64::standards::v1::subsets::any::schema::mutations::delete_space::DeleteSpace;
use crate::standards::v1::subsets::any::schema::mutations::create_space::CreateSpace;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateSpace, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_64::standards::v1::subsets::any::schema::mutations::delete_space::DeleteSpace>,
{
    if base.spaces.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteSpace { id: payload.id.clone() })]
}
