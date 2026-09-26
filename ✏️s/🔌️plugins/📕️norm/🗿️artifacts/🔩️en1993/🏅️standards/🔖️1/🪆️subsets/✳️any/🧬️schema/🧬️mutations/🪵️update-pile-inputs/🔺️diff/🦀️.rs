//! 🔺️ `upsert-pile` — sparse diff construction.

use super::UpdatePileInputs;
use crate::diff::En1993PileList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdatePileInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.piles.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.pile.id) {
        if values[idx] == payload.pile {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.pile.clone();
    } else {
        values.push(payload.pile.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { piles: Some(En1993PileList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
