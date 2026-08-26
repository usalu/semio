//! 🔺️ `update-member-properties` — sparse diff construction.

use super::mutation::UpdateMemberProperties;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateMemberProperties, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let fields: [(&str, f64); 11] = [
        ("N Ed kn", payload.new_n_ed_kn),
        ("M Ed knm", payload.new_m_ed_knm),
        ("V Ed kn", payload.new_v_ed_kn),
        ("A mm2", payload.new_a_mm2),
        ("A v mm2", payload.new_a_v_mm2),
        ("W pl mm3", payload.new_w_pl_mm3),
        ("F y mpa", payload.new_f_y_mpa),
        ("F u mpa", payload.new_f_u_mpa),
        ("Chi", payload.new_chi),
        ("A net mm2", payload.new_a_net_mm2),
        ("Tension N Ed kn", payload.new_tension_n_ed_kn),
    ];
    for (label, value) in fields {
        if !value.is_finite() {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("{label} must be a finite number, got {value}."), Vec::<String>::new());
        }
    }
    if base.n_ed_kn == payload.new_n_ed_kn
        && base.m_ed_knm == payload.new_m_ed_knm
        && base.v_ed_kn == payload.new_v_ed_kn
        && base.a_mm2 == payload.new_a_mm2
        && base.a_v_mm2 == payload.new_a_v_mm2
        && base.w_pl_mm3 == payload.new_w_pl_mm3
        && base.f_y_mpa == payload.new_f_y_mpa
        && base.f_u_mpa == payload.new_f_u_mpa
        && base.chi == payload.new_chi
        && base.a_net_mm2 == payload.new_a_net_mm2
        && base.tension_n_ed_kn == payload.new_tension_n_ed_kn
    {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff {
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
    })
}
//#endregion 🔖️Diff
