//! 📐️ Geometry declarations of the EN 1994 design cases: per catalogue entry the form it draws in 3D, the inputs that size it and drive its load bar. See [`super`].

use semio_s_plugin_norm_compliance::design_case::{self, AuxRow, CaseForm, CaseGeometry, GeometryRow};

const fn row(entry: &'static str, form: CaseForm, width: &'static str, length: &'static str, height: &'static str, load: &'static str, aux: &'static [AuxRow]) -> GeometryRow {
    GeometryRow { entry, geometry: CaseGeometry { form, width, length, height, load, aux } }
}

/// 📐️ The declaration of every catalogue entry, in catalogue order.
pub static GEOMETRY: &[GeometryRow] = &[
    row("C94-01", CaseForm::Gauge, "", "", "", "p_ed", &[]),
    row("C94-02", CaseForm::Member, "b_f", "a_panel", "h_w", "f_ed", &[]),
    row("C94-03", CaseForm::Column, "d_min", "d_min", "l_col", "n_intro", &[]),
    row("C94-04", CaseForm::Member, "", "l_span", "", "q_line", &[]),
    row("C94-05", CaseForm::Member, "", "l_span", "", "q_line", &[]),
    row("C94-06", CaseForm::Gauge, "", "", "", "sigma_ed", &[]),
    row("C94-07", CaseForm::Member, "sec_b", "", "sec_h", "m_ed", &[]),
    row("C94-08", CaseForm::Member, "sec_b", "", "sec_h", "m_ed", &[]),
    row("C94-09", CaseForm::Member, "", "l_span", "", "q_line", &[]),
    row("C94-10", CaseForm::Member, "sec_b", "l_ltb", "sec_h", "m_ed", &[]),
    row("C94-11", CaseForm::Member, "b_eff", "l_span", "sec_h", "q_line", &[]),
    row("C94-12", CaseForm::Gauge, "", "", "", "t_fire", &[]),
];

/// 🔎 The geometry declared for `entry_id`.
pub fn geometry_of(entry_id: &str) -> Option<CaseGeometry> {
    design_case::geometry_in(GEOMETRY, entry_id)
}
