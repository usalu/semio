//! 🔺️ `update-plated-inputs` — sparse diff construction.

use super::mutation::UpdatePlatedInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdatePlatedInputs, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff {
        plated_lambda_p: Some(payload.new_plated_lambda_p),
        plated_sigma_ed_mpa: Some(payload.new_plated_sigma_ed_mpa),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
