//! 🔺️ `change-bridge-v-rd-kn` sparse diff construction — writes only `En1998Diff.bridge_v_rd_kn` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_bridge_v_rd_kn::mutation::ChangeBridgeVRdKn;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeBridgeVRdKn, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_bridge_v_rd_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bridge design shear resistance [kN] must be a finite number, got {}.", payload.new_bridge_v_rd_kn), Vec::<String>::new());
    }
    if base.bridge_v_rd_kn == payload.new_bridge_v_rd_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Bridge design shear resistance [kN] is already {}.", payload.new_bridge_v_rd_kn));
    }
    protocol::MutationOutcome::new(En1998Diff { bridge_v_rd_kn: Some(payload.new_bridge_v_rd_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
