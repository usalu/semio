//! 🔺️ `update-silo-shell-inputs` — sparse diff construction.

use super::mutation::UpdateSiloShellInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &UpdateSiloShellInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_silo_t_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Silo t mm must be a finite number, got {}.", payload.new_silo_t_mm), Vec::<String>::new());
    }
    if !payload.new_silo_r_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Silo r mm must be a finite number, got {}.", payload.new_silo_r_mm), Vec::<String>::new());
    }
    if !payload.new_shell_sigma_x_ed_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Shell sigma x ed mpa must be a finite number, got {}.", payload.new_shell_sigma_x_ed_mpa), Vec::<String>::new());
    }
    if !payload.new_silo_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Silo k must be a finite number, got {}.", payload.new_silo_k), Vec::<String>::new());
    }
    if !payload.new_silo_gamma_kn_m3.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Silo gamma kn m3 must be a finite number, got {}.", payload.new_silo_gamma_kn_m3), Vec::<String>::new());
    }
    if !payload.new_silo_depth_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Silo depth m must be a finite number, got {}.", payload.new_silo_depth_m), Vec::<String>::new());
    }
    if base.silo_t_mm == payload.new_silo_t_mm && base.silo_r_mm == payload.new_silo_r_mm && base.shell_sigma_x_ed_mpa == payload.new_shell_sigma_x_ed_mpa && base.silo_k == payload.new_silo_k && base.silo_gamma_kn_m3 == payload.new_silo_gamma_kn_m3 && base.silo_depth_m == payload.new_silo_depth_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff { silo_t_mm: Some(payload.new_silo_t_mm), silo_r_mm: Some(payload.new_silo_r_mm), shell_sigma_x_ed_mpa: Some(payload.new_shell_sigma_x_ed_mpa), silo_k: Some(payload.new_silo_k), silo_gamma_kn_m3: Some(payload.new_silo_gamma_kn_m3), silo_depth_m: Some(payload.new_silo_depth_m), ..Default::default() })
}
//#endregion 🔖️Diff
