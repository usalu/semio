//! 🔺️ `change-bridge-span-m` — sparse diff construction.

use super::mutation::ChangeBridgeSpanM;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeBridgeSpanM, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { bridge_span_m: Some(payload.new_bridge_span_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
