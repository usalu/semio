//! ↩️ `change-irradiance-w-m2` — undo restores BASE's `irradiance_w_m2`.

use super::mutation::ChangeIrradianceWM2;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeIrradianceWM2, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeIrradianceWM2(ChangeIrradianceWM2 { new_irradiance_w_m2: base.irradiance_w_m2 })]
}
//#endregion 🔖️Inverse
