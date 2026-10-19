//! ↩️ Inverse of `CreateRule`: the concrete `DeleteRule` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_119::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule;
use crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateRule, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_119::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule>,
{
    if base.rules.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteRule { id: payload.id.clone() })]
}
