//! ↩️ Inverse of `SetDimension`: an absolute `SetDimension` restoring the base value of exactly the fields the forward really changes, none when the dimension is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_dimension::SetDimension;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Patch;

pub fn inverse<Op>(payload: &SetDimension, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_dimension::SetDimension>,
{
    let Some(record) = base.dimensions.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetDimension::from_patch(payload.id.clone(), restore))]
}
