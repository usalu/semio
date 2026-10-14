//! 🔺️ `change-exchange-process` — sparse diff construction.

use crate::change_exchange_process::mutation::ChangeExchangeProcess;
use semio_s_artifact_norm_iso16757_diff::Iso16757Diff;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeExchangeProcess, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.exchange_process == payload.new_exchange_process {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Exchange process already has this value.");
    }
    protocol::MutationOutcome::new(Iso16757Diff { exchange_process: Some(payload.new_exchange_process), ..Default::default() })
}
//#endregion 🔖️Diff
