//! 🔺️ `change-consequence-class` — sparse diff construction; writes only
//! `En1990Diff.consequence_class`.

use super::ChangeConsequenceClass;
use crate::artifacts::en1990::{En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeConsequenceClass, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !(1..=3).contains(&payload.new_consequence_class) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Consequence class must be 1 (CC1), 2 (CC2), or 3 (CC3); got {}.", payload.new_consequence_class), Vec::<String>::new());
    }
    if base.consequence_class == payload.new_consequence_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Consequence class already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { consequence_class: Some(payload.new_consequence_class), ..Default::default() })
}
//#endregion 🔖️Diff
