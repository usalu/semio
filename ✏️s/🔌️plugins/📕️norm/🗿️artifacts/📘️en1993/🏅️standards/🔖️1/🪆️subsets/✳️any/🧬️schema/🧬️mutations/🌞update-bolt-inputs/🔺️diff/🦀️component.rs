//! 🔺️ `update-bolt-inputs` — sparse diff construction.

use super::mutation::UpdateBoltInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateBoltInputs, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff {
        bolt_f_ed_kn: Some(payload.new_bolt_f_ed_kn),
        bolt_n_bolts: Some(payload.new_bolt_n_bolts),
        bolt_a_s_mm2: Some(payload.new_bolt_a_s_mm2),
        bolt_e1_mm: Some(payload.new_bolt_e1_mm),
        bolt_e2_mm: Some(payload.new_bolt_e2_mm),
        bolt_d0_mm: Some(payload.new_bolt_d0_mm),
        bolt_d_mm: Some(payload.new_bolt_d_mm),
        bolt_t_mm: Some(payload.new_bolt_t_mm),
        bolt_f_u_mpa: Some(payload.new_bolt_f_u_mpa),
        bolt_f_ub_mpa: Some(payload.new_bolt_f_ub_mpa),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
