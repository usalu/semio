//! ↩️ Inverse for `ResizeGrid` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;

pub fn inverse(payload: &super::ResizeGrid, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
    let _ = payload;
    vec![
        crate::mutations::resize_grid(base.width, base.height, base.depth),
        crate::mutations::change_cell_sizes(Grid3dAxis::X, base.cell_sizes_x.clone()),
        crate::mutations::change_cell_sizes(Grid3dAxis::Y, base.cell_sizes_y.clone()),
        crate::mutations::change_cell_sizes(Grid3dAxis::Z, base.cell_sizes_z.clone()),
    ]
}
