//! ↩️ Inverse of `SetSpaceConditions`: `RemoveSpaceConditions` when the forward creates the record, else an absolute `SetSpaceConditions` restoring the base value of exactly the fields the forward really changes.

use crate::standards::v1::subsets::any::schema::mutations::set_space_conditions::SetSpaceConditions;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Patch;
use crate::standards::v1::subsets::any::schema::mutations::remove_space_conditions::RemoveSpaceConditions;

pub fn inverse<Op>(payload: &SetSpaceConditions, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::remove_space_conditions::RemoveSpaceConditions> + From<crate::standards::v1::subsets::any::schema::mutations::set_space_conditions::SetSpaceConditions>,
{
    if !base.spaces.contains_key(&payload.id) {
        return Vec::new();
    }
    let Some(record) = base.space_conditions.get(&payload.id) else {
        return vec![Op::from(RemoveSpaceConditions { id: payload.id.clone() })];
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetSpaceConditions::from_patch(payload.id.clone(), restore))]
}
