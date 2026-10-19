//! ↩️ Inverse of `DeleteTextNote`: the concrete `CreateTextNote` carrying the full removed record, in storage order through the shared cascade.

use crate::standards::v1::subsets::any::schema::mutations::delete_text_note::DeleteTextNote;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteTextNote, base: &ModelSnapshot) -> Vec<Op>
where
    Op: semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::inverse(base, std::slice::from_ref(&payload.id))
}
