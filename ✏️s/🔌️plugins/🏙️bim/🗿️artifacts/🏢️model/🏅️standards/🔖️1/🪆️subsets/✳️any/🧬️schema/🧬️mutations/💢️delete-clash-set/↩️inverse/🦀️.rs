//! ↩️ Inverse of `DeleteClashSet`: one concrete create per removed record and one setter per removed property or classification, in storage order (dependants first, the target last), so the store, which replays the
//! vector reversed, recreates the target before anything on it.

use crate::standards::v1::subsets::any::schema::mutations::delete_clash_set::DeleteClashSet;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteClashSet, base: &ModelSnapshot) -> Vec<Op>
where
    Op: semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::inverse(base, std::slice::from_ref(&payload.id))
}
