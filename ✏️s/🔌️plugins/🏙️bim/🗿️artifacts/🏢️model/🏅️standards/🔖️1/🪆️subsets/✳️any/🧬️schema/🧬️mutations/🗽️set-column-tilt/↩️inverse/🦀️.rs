//! ↩️ Inverse of `SetColumnTilt`: an absolute `SetColumnTilt` back to the base tilt (plumb when none), none when the column is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_column_tilt::SetColumnTilt;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetColumnTilt, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_column_tilt::SetColumnTilt>,
{
    match base.columns.get(&payload.id) {
        Some(column) => vec![Op::from(SetColumnTilt { id: payload.id.clone(), tilt: column.tilt })],
        None => Vec::new(),
    }
}
