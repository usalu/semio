//! ↩️ Inverse of `SetStoreyLevel`: an absolute `SetStoreyLevel` back to the base level, none when the storey is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_storey_level::SetStoreyLevel;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetStoreyLevel, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_storey_level::SetStoreyLevel>,
{
    match base.storeys.get(&payload.id) {
        Some(storey) => vec![Op::from(SetStoreyLevel { id: payload.id.clone(), level: storey.level })],
        None => Vec::new(),
    }
}
