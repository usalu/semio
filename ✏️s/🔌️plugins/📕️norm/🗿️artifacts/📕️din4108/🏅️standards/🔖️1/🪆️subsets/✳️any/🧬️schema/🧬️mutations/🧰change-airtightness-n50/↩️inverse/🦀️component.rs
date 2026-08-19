//! ↩️ `change-airtightness-n50` — undo restores BASE's `airtightness_n50`.

use super::mutation::ChangeAirtightnessN50;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeAirtightnessN50, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeAirtightnessN50(ChangeAirtightnessN50 { new_airtightness_n50: base.airtightness_n50 })]
}
//#endregion 🔖️Inverse
