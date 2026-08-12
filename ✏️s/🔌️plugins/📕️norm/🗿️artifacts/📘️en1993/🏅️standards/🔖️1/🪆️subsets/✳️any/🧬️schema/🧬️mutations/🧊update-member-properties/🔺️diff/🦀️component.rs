//! 🔺️ `update-member-properties` — sparse diff construction.

use super::mutation::UpdateMemberProperties;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateMemberProperties, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff {
        n_ed_kn: Some(payload.new_n_ed_kn),
        m_ed_knm: Some(payload.new_m_ed_knm),
        v_ed_kn: Some(payload.new_v_ed_kn),
        a_mm2: Some(payload.new_a_mm2),
        a_v_mm2: Some(payload.new_a_v_mm2),
        w_pl_mm3: Some(payload.new_w_pl_mm3),
        f_y_mpa: Some(payload.new_f_y_mpa),
        f_u_mpa: Some(payload.new_f_u_mpa),
        chi: Some(payload.new_chi),
        a_net_mm2: Some(payload.new_a_net_mm2),
        tension_n_ed_kn: Some(payload.new_tension_n_ed_kn),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
