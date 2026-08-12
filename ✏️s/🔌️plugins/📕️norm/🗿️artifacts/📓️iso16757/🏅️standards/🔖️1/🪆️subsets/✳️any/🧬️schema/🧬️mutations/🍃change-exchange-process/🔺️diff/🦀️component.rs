//! 🔺️ `change-exchange-process` — sparse diff construction.

use super::mutation::ChangeExchangeProcess;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeExchangeProcess, _base: &Iso16757Snapshot) -> Iso16757Diff {
    Iso16757Diff { exchange_process: Some(payload.new_exchange_process), ..Default::default() }
}
//#endregion 🔖️Diff
