//! ↩️ Inverse for `ChangeSeed` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;

pub fn inverse(payload: &super::ChangeSeed, base: &Grid3dSnapshot) -> Result<Vec<Grid3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let _ = payload;
    vec![crate::mutations::change_seed(base.seed)]

    })())
}
