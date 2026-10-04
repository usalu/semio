//! ↩️ `change-exchange-process` — undo restores BASE's exchange process.

use super::mutation::ChangeExchangeProcess;
use crate::{Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeExchangeProcess, base: &Iso16757Snapshot) -> Result<Vec<Iso16757Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Iso16757Mutation::ChangeExchangeProcess(ChangeExchangeProcess { new_exchange_process: base.exchange_process })]

    })())
}
//#endregion 🔖️Inverse
