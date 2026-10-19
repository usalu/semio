//! ↩️ Inverse of `DeleteElements`: one concrete create per removed element (sites first in creation order, openings last) and one
//! property or classification setter per removed data entry, in storage order so that the store, which replays the vector reversed,
//! restores the base exactly. Empty when the selection is refused.

use crate::standards::v1::subsets::any::schema::mutations::delete_elements::DeleteElements;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteElements, base: &ModelSnapshot) -> Vec<Op>
where
    Op: semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::inverse(base, &payload.ids)
}
