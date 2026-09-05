//! 🔺️ `change-exchange-process` — sparse diff construction.

use super::mutation::ChangeExchangeProcess;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeExchangeProcess, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.exchange_process == payload.new_exchange_process {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Exchange process already has this value.");
    }
    protocol::MutationOutcome::new(Iso16757Diff { exchange_process: Some(payload.new_exchange_process.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
