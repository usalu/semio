//! ↩️ Inverse for `ChangeSeed` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::standards::v1::subsets::any::schema::mutations::change_seed::change_seed;
use crate::schema::snapshot::*;

pub fn inverse<Op>(payload: &super::ChangeSeed, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_seed::ChangeSeed>,
{
    Ok((|| {
    let _ = payload;
    vec![change_seed(base.seed)]

    })())
}
