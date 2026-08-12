//! 🔺️ `change-t-int-c` — sparse diff construction.

use super::mutation::ChangeTIntC;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeTIntC, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { t_int_c: Some(payload.new_t_int_c), ..Default::default() }
}
//#endregion 🔖️Diff
