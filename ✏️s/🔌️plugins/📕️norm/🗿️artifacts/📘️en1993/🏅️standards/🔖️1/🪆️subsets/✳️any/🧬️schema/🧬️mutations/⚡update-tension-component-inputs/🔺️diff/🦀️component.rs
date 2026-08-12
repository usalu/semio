//! 🔺️ `update-tension-component-inputs` — sparse diff construction.

use super::mutation::UpdateTensionComponentInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateTensionComponentInputs, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff {
        tension_component_f_uk_kn: Some(payload.new_tension_component_f_uk_kn),
        tension_component_f_k_kn: Some(payload.new_tension_component_f_k_kn),
        tension_component_n_ed_kn: Some(payload.new_tension_component_n_ed_kn),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
