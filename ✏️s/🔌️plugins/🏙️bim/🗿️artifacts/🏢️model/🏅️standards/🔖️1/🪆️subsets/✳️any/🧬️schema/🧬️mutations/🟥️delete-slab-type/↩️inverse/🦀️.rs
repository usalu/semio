//! ↩️ Inverse of `DeleteSlabType`: the setters of the properties and classifications of the type, then the concrete `CreateSlabType` carrying the full removed record, none when the slab type was absent.

use crate::standards::v1::subsets::any::schema::mutations::create_slab_type::CreateSlabType;
use crate::standards::v1::subsets::any::schema::mutations::delete_slab_type::DeleteSlabType;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteSlabType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse + From<crate::standards::v1::subsets::any::schema::mutations::create_slab_type::CreateSlabType>,
{
    match base.slab_types.get(&payload.id) {
        Some(slab_type) => semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::data_rows(base, &payload.id).into_iter().chain(std::iter::once(Op::from(CreateSlabType { id: payload.id.clone(), slab_type: slab_type.clone() }))).collect(),
        None => Vec::new(),
    }
}
