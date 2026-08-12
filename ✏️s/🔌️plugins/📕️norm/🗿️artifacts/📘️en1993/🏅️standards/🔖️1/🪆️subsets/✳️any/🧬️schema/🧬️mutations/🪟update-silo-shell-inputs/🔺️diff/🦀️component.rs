//! 🔺️ `update-silo-shell-inputs` — sparse diff construction.

use super::mutation::UpdateSiloShellInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateSiloShellInputs, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff {
        silo_t_mm: Some(payload.new_silo_t_mm),
        silo_r_mm: Some(payload.new_silo_r_mm),
        shell_sigma_x_ed_mpa: Some(payload.new_shell_sigma_x_ed_mpa),
        silo_k: Some(payload.new_silo_k),
        silo_gamma_kn_m3: Some(payload.new_silo_gamma_kn_m3),
        silo_depth_m: Some(payload.new_silo_depth_m),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
