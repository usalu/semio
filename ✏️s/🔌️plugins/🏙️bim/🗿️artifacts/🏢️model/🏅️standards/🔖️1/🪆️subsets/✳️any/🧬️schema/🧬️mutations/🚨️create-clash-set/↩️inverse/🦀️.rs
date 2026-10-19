//! ↩️ Inverse of `CreateClashSet`: the concrete `DeleteClashSet` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_118::standards::v1::subsets::any::schema::mutations::delete_clash_set::DeleteClashSet;
use crate::standards::v1::subsets::any::schema::mutations::create_clash_set::CreateClashSet;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateClashSet, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_118::standards::v1::subsets::any::schema::mutations::delete_clash_set::DeleteClashSet>,
{
    if base.clash_sets.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteClashSet { id: payload.id.clone() })]
}
