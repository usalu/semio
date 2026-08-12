//! 🔺️ `change-silo-q-nominal` sparse diff construction — writes only `En1998Diff.silo_q_nominal` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_silo_q_nominal::mutation::ChangeSiloQNominal;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSiloQNominal, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { silo_q_nominal: Some(payload.new_silo_q_nominal.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
