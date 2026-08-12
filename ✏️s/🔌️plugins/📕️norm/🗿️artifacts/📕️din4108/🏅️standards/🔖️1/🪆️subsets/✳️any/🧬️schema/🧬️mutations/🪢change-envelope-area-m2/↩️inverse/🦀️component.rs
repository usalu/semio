//! ↩️ `change-envelope-area-m2` — undo restores BASE's `envelope_area_m2`.

use super::mutation::ChangeEnvelopeAreaM2;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeEnvelopeAreaM2, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeEnvelopeAreaM2(ChangeEnvelopeAreaM2 { new_envelope_area_m2: base.envelope_area_m2 })]
}
//#endregion 🔖️Inverse
