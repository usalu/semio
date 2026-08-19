//! ↩️ `change-bridge-span-m` — undo restores BASE's bridge span.

use super::mutation::ChangeBridgeSpanM;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeBridgeSpanM, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeBridgeSpanM(ChangeBridgeSpanM { new_bridge_span_m: base.bridge_span_m.clone() })]
}
//#endregion 🔖️Inverse
