//! ↩️ Inverse for `ResizeGrid` — restores the base extent first, then every pinned/masked cell the
//! shrink cascaded away, each landing back at its canonical row-major position.

use crate::standards::v1::subsets::any::schema::mutations::mask_cell::mask_cell;
use crate::standards::v1::subsets::any::schema::mutations::pin_cell::pin_cell;
use crate::standards::v1::subsets::any::schema::mutations::resize_grid::resize_grid;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse<Op>(payload: &super::ResizeGrid, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::mask_cell::MaskCell> + From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell> + From<crate::standards::v1::subsets::any::schema::mutations::resize_grid::ResizeGrid>,
{
    Ok((|| {
    if base.width == payload.width && base.height == payload.height {
        return Vec::new();
    }
    let outside = |x: u32, y: u32| x >= payload.width || y >= payload.height;
    let mut restore = vec![resize_grid(base.width, base.height)];
    restore.extend(base.masked.iter().filter(|cell| outside(cell.x, cell.y)).map(|cell| mask_cell(cell.x, cell.y)));
    restore.extend(base.pinned.iter().filter(|cell| outside(cell.x, cell.y)).map(|cell| pin_cell(cell.x, cell.y, cell.tile_id.clone())));
    restore.reverse();
    restore

    })())
}
