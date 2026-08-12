//! 🔺️ `change-envelope-area-m2` — sparse diff construction.

use super::mutation::ChangeEnvelopeAreaM2;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeEnvelopeAreaM2, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { envelope_area_m2: Some(payload.new_envelope_area_m2), ..Default::default() }
}
//#endregion 🔖️Diff
