//! ↩️ Inverse for `ChangeCellSize` — restores the base extent of one cell.

use crate::standards::v1::subsets::any::schema::mutations::change_cell_size::change_cell_size;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse<Op>(_payload: &super::ChangeCellSize, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_cell_size::ChangeCellSize>,
{
    Ok((|| {
    vec![change_cell_size(base.cell_width, base.cell_height)]

    })())
}
