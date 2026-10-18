//! ↩️ Inverse of `SetGridLine`: an absolute `SetGridLine` restoring the base value of exactly the fields the forward really changes, none when the grid line is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_grid_line::SetGridLine;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetGridLine, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_grid_line::SetGridLine>,
{
    let Some(record) = base.grids.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetGridLine::from_patch(payload.id.clone(), restore))]
}
