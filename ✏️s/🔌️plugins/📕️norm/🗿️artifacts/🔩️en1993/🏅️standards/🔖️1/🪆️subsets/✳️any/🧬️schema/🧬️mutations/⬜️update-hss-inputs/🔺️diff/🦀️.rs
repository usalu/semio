//! 🔺️ `upsert-load-case` — sparse diff construction.

use super::UpdateHssInputs;
use crate::diff::En1993LoadCaseList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateHssInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.load_cases.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.load_case.id) {
        if values[idx] == payload.load_case {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.load_case.clone();
    } else {
        values.push(payload.load_case.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { load_cases: Some(En1993LoadCaseList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
