//! ↩️ Inverse of `CreateSlab`: the concrete `DeleteSlab` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_68::standards::v1::subsets::any::schema::mutations::delete_slab::DeleteSlab;
use crate::standards::v1::subsets::any::schema::mutations::create_slab::CreateSlab;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateSlab, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_68::standards::v1::subsets::any::schema::mutations::delete_slab::DeleteSlab>,
{
    if base.slabs.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteSlab { id: payload.id.clone() })]
}
