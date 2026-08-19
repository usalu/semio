//! 🔺️ `change-fire-curve` — sparse diff construction.

use super::mutation::ChangeFireCurve;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeFireCurve, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_curve == payload.new_fire_curve {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Fire curve already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_curve: Some(payload.new_fire_curve.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
