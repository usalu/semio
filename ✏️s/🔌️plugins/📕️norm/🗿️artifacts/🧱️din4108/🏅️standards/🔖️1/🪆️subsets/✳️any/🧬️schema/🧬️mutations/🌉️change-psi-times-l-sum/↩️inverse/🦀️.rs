//! ↩️ `change-psi-times-l-sum` — undo restores BASE's `psi_times_l_sum`.

use super::ChangePsiTimesLSum;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangePsiTimesLSum, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangePsiTimesLSum(ChangePsiTimesLSum { new_psi_times_l_sum: base.psi_times_l_sum })]
}
//#endregion 🔖️Inverse
