//! 🔺️ `upsert-section` — sparse diff construction.

use super::UpdateThroughThicknessInputs;
use crate::diff::En1993SectionList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateThroughThicknessInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.sections.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.section.id) {
        if values[idx] == payload.section {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.section.clone();
    } else {
        values.push(payload.section.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { sections: Some(En1993SectionList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
