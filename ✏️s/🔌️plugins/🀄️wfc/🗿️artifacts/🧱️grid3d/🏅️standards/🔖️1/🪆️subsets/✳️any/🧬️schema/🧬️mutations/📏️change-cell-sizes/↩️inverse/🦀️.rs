//! ↩️ Inverse for `ChangeCellSizes` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::standards::v1::subsets::any::schema::mutations::change_cell_sizes::change_cell_sizes;
use crate::schema::snapshot::*;

pub fn inverse<Op>(payload: &super::ChangeCellSizes, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_cell_sizes::ChangeCellSizes>,
{
    Ok((|| {
    vec![change_cell_sizes(payload.axis, payload.axis.sizes(base).clone())]

    })())
}
