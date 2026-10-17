//! ↩️ Inverse for `ResizeGrid` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::standards::v1::subsets::any::schema::mutations::change_cell_sizes::change_cell_sizes;
use crate::standards::v1::subsets::any::schema::mutations::resize_grid::resize_grid;
use crate::schema::snapshot::*;

pub fn inverse<Op>(payload: &super::ResizeGrid, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_cell_sizes::ChangeCellSizes> + From<crate::standards::v1::subsets::any::schema::mutations::resize_grid::ResizeGrid>,
{
    Ok((|| {
    let _ = payload;
    vec![
        resize_grid(base.width, base.height, base.depth),
        change_cell_sizes(Grid3dAxis::X, base.cell_sizes_x.clone()),
        change_cell_sizes(Grid3dAxis::Y, base.cell_sizes_y.clone()),
        change_cell_sizes(Grid3dAxis::Z, base.cell_sizes_z.clone()),
    ]

    })())
}
