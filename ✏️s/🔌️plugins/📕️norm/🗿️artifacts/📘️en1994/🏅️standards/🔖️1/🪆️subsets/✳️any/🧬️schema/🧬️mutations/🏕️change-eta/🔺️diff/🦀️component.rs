//! 🔺️ `change-eta` — sparse diff construction.

use super::mutation::ChangeEta;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeEta, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_eta.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Eta must be a finite number.", Vec::<String>::new());
    }
    if base.eta == payload.new_eta {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Eta already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { eta: Some(payload.new_eta.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
