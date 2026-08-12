//! 🔺️ `change-irradiance-w-m2` — sparse diff construction.

use super::mutation::ChangeIrradianceWM2;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeIrradianceWM2, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { irradiance_w_m2: Some(payload.new_irradiance_w_m2), ..Default::default() }
}
//#endregion 🔖️Diff
