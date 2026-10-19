//! ↩️ Inverse of `DeleteComponent`: the concrete `CreateComponent` of the removed component, one `SetComponentOverride` per removed override and one setter per removed property or classification, in storage
//! order (dependants first, the component last), so the store, which replays the vector reversed, recreates the component before anything that belongs to it.

use crate::standards::v1::subsets::any::schema::mutations::delete_component::DeleteComponent;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteComponent, base: &ModelSnapshot) -> Vec<Op>
where
    Op: semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::inverse(base, std::slice::from_ref(&payload.id))
}
