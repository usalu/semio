//! ↩️ Inverse of `RemoveSpaceConditions`: one absolute `SetSpaceConditions` that states every field of the removed record (an unstated field as an assigned null), none when the space has no conditions.

use crate::standards::v1::subsets::any::schema::mutations::remove_space_conditions::RemoveSpaceConditions;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::set_space_conditions::SetSpaceConditions;

pub fn inverse<Op>(payload: &RemoveSpaceConditions, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_space_conditions::SetSpaceConditions>,
{
    let Some(record) = base.space_conditions.get(&payload.id) else {
        return Vec::new();
    };
    vec![Op::from(SetSpaceConditions::stating(&payload.id, record))]
}
