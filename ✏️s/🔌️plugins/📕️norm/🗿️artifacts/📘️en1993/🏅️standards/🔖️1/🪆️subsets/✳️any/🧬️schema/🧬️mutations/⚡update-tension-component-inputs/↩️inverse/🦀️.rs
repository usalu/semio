//! ↩️ `update-tension-component-inputs` — undo restores BASE's tension component inputs.

use super::UpdateTensionComponentInputs;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateTensionComponentInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdateTensionComponentInputs(UpdateTensionComponentInputs {
        new_tension_component_f_uk_kn: base.tension_component_f_uk_kn,
        new_tension_component_f_k_kn: base.tension_component_f_k_kn,
        new_tension_component_n_ed_kn: base.tension_component_n_ed_kn,
    })]
}
//#endregion 🔖️Inverse
