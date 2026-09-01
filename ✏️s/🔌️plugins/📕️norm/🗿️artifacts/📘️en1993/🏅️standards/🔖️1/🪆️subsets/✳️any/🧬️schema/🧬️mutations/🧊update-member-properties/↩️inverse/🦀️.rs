//! ↩️ `update-member-properties` — undo restores BASE's member properties.

use super::UpdateMemberProperties;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateMemberProperties, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdateMemberProperties(UpdateMemberProperties {
        new_n_ed_kn: base.n_ed_kn,
        new_m_ed_knm: base.m_ed_knm,
        new_v_ed_kn: base.v_ed_kn,
        new_a_mm2: base.a_mm2,
        new_a_v_mm2: base.a_v_mm2,
        new_w_pl_mm3: base.w_pl_mm3,
        new_f_y_mpa: base.f_y_mpa,
        new_f_u_mpa: base.f_u_mpa,
        new_chi: base.chi,
        new_a_net_mm2: base.a_net_mm2,
        new_tension_n_ed_kn: base.tension_n_ed_kn,
    })]
}
//#endregion 🔖️Inverse
