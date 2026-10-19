//! ↩️ Inverse of `CreateSlabType`: the concrete `DeleteSlabType` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_slab_type::DeleteSlabType;
use crate::standards::v1::subsets::any::schema::mutations::create_slab_type::CreateSlabType;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateSlabType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_slab_type::DeleteSlabType>,
{
    if base.slab_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteSlabType { id: payload.id.clone() })]
}
