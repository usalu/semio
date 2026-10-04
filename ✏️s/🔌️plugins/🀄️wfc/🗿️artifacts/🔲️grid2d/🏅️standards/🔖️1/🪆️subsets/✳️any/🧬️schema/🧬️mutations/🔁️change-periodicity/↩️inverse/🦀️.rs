//! ↩️ Inverse for `ChangePeriodicity` — restores both axes' base wrap flags.

use crate::mutations::{change_periodicity, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse(_payload: &super::ChangePeriodicity, base: &Grid2dSnapshot) -> Result<Vec<Grid2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![change_periodicity(base.periodic_x, base.periodic_y)]

    })())
}
