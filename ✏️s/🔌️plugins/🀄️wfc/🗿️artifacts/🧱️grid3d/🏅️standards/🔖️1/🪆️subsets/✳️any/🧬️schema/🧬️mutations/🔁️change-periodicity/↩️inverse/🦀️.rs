//! ↩️ Inverse for `ChangePeriodicity` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::standards::v1::subsets::any::schema::mutations::change_periodicity::change_periodicity;
use crate::schema::snapshot::*;

pub fn inverse<Op>(payload: &super::ChangePeriodicity, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_periodicity::ChangePeriodicity>,
{
    Ok((|| {
    let _ = payload;
    vec![change_periodicity(base.periodic_x, base.periodic_y, base.periodic_z)]

    })())
}
