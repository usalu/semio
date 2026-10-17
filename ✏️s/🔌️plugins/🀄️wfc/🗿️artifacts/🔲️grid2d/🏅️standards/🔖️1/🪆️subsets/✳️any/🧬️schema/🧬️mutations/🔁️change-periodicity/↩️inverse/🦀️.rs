//! ↩️ Inverse for `ChangePeriodicity` — restores both axes' base wrap flags.

use crate::standards::v1::subsets::any::schema::mutations::change_periodicity::change_periodicity;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse<Op>(_payload: &super::ChangePeriodicity, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_periodicity::ChangePeriodicity>,
{
    Ok((|| {
    vec![change_periodicity(base.periodic_x, base.periodic_y)]

    })())
}
