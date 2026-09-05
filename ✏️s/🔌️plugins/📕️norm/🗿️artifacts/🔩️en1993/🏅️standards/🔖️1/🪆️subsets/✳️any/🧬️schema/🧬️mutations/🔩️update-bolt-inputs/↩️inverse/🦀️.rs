//! ↩️ `update-bolt-inputs` — undo restores BASE's bolt inputs.

use super::UpdateBoltInputs;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateBoltInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdateBoltInputs(UpdateBoltInputs {
        new_bolt_f_ed_kn: base.bolt_f_ed_kn,
        new_bolt_n_bolts: base.bolt_n_bolts,
        new_bolt_a_s_mm2: base.bolt_a_s_mm2,
        new_bolt_e1_mm: base.bolt_e1_mm,
        new_bolt_e2_mm: base.bolt_e2_mm,
        new_bolt_d0_mm: base.bolt_d0_mm,
        new_bolt_d_mm: base.bolt_d_mm,
        new_bolt_t_mm: base.bolt_t_mm,
        new_bolt_f_u_mpa: base.bolt_f_u_mpa,
        new_bolt_f_ub_mpa: base.bolt_f_ub_mpa,
    })]
}
//#endregion 🔖️Inverse
