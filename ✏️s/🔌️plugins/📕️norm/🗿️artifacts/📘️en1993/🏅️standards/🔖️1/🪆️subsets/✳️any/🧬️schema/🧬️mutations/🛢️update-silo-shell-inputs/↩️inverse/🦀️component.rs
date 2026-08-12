//! ↩️ `update-silo-shell-inputs` — undo restores BASE's silo shell inputs.

use super::mutation::UpdateSiloShellInputs;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateSiloShellInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdateSiloShellInputs(UpdateSiloShellInputs {
        new_silo_t_mm: base.silo_t_mm,
        new_silo_r_mm: base.silo_r_mm,
        new_shell_sigma_x_ed_mpa: base.shell_sigma_x_ed_mpa,
        new_silo_k: base.silo_k,
        new_silo_gamma_kn_m3: base.silo_gamma_kn_m3,
        new_silo_depth_m: base.silo_depth_m,
    })]
}
//#endregion 🔖️Inverse
