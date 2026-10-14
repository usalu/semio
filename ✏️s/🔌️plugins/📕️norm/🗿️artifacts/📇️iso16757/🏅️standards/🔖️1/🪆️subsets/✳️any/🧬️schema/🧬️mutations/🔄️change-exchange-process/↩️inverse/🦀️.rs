//! ↩️ `change-exchange-process` — undo restores BASE's exchange process.

use crate::change_exchange_process::mutation::ChangeExchangeProcess;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeExchangeProcess, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_exchange_process::mutation::ChangeExchangeProcess> {
    Ok((|| {
    vec![Op::from(ChangeExchangeProcess { new_exchange_process: base.exchange_process })]

    })())
}
//#endregion 🔖️Inverse
