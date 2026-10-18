//! ↩️ Inverse of `CreateGridLine`: the concrete `DeleteGridLine` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_grid_line::DeleteGridLine;
use crate::standards::v1::subsets::any::schema::mutations::create_grid_line::CreateGridLine;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateGridLine, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_grid_line::DeleteGridLine>,
{
    if base.grids.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteGridLine { id: payload.id.clone() })]
}
