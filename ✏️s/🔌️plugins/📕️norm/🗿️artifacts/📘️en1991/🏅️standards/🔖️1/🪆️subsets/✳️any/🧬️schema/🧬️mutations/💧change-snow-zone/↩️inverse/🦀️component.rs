//! ↩️ `change-snow-zone` — undo restores BASE's snow zone.

use super::mutation::ChangeSnowZone;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeSnowZone, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeSnowZone(ChangeSnowZone { new_snow_zone: base.snow_zone.clone() })]
}
//#endregion 🔖️Inverse
