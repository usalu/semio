//! ↩️ Inverse for `ChangeCellSize` — restores the base extent of one cell.

use crate::mutations::{change_cell_size, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse(_payload: &super::ChangeCellSize, base: &Grid2dSnapshot) -> Result<Vec<Grid2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![change_cell_size(base.cell_width, base.cell_height)]

    })())
}
