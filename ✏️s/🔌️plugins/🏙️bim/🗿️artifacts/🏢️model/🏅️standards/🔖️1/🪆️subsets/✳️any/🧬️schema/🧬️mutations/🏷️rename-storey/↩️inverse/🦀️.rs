//! ↩️ Inverse of `RenameStorey`: an absolute `RenameStorey` back to the base name, none when the storey is absent.

use crate::standards::v1::subsets::any::schema::mutations::rename_storey::RenameStorey;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &RenameStorey, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rename_storey::RenameStorey>,
{
    match base.storeys.get(&payload.id) {
        Some(storey) => vec![Op::from(RenameStorey { id: payload.id.clone(), name: storey.name.clone() })],
        None => Vec::new(),
    }
}
