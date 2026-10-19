//! ↩️ Inverse of `CreateTag`: the concrete `DeleteTag` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_96::standards::v1::subsets::any::schema::mutations::delete_tag::DeleteTag;
use crate::standards::v1::subsets::any::schema::mutations::create_tag::CreateTag;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateTag, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_96::standards::v1::subsets::any::schema::mutations::delete_tag::DeleteTag>,
{
    if base.tags.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteTag { id: payload.id.clone() })]
}
