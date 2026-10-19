//! ↩️ Inverse of `DeleteMepElement`: the concrete `CreateMepElement` of the removed element and one setter per removed property or classification, in storage order (the element last), so the store,
//! which replays the vector reversed, recreates the element before its data.

use crate::standards::v1::subsets::any::schema::mutations::delete_mep_element::DeleteMepElement;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteMepElement, base: &ModelSnapshot) -> Vec<Op>
where
    Op: semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::inverse(base, std::slice::from_ref(&payload.id))
}
