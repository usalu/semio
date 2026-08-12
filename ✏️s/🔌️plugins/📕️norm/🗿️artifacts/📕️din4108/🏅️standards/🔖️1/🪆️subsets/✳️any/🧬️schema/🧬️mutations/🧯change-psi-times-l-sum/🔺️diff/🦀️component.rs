//! 🔺️ `change-psi-times-l-sum` — sparse diff construction.

use super::mutation::ChangePsiTimesLSum;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangePsiTimesLSum, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { psi_times_l_sum: Some(payload.new_psi_times_l_sum), ..Default::default() }
}
//#endregion 🔖️Diff
