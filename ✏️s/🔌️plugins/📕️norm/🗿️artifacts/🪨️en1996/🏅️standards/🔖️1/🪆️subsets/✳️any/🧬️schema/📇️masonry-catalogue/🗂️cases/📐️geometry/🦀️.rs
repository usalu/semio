//! 📐️ Geometry declarations of the EN1996 design cases: per catalogue entry the form it draws in 3D, the inputs that size it and drive its load bar and the auxiliary roles of its form; generated from the hand-maintained `./🔣️.json` by `🧰️scripts/catalogue-bridge-v5/geometry_gen.py`. See [`super`].

use semio_s_plugin_norm_compliance::design_case::{self, AuxRow, CaseForm, CaseGeometry, GeometryRow};

const fn row(entry: &'static str, form: CaseForm, width: &'static str, length: &'static str, height: &'static str, load: &'static str, aux: &'static [AuxRow]) -> GeometryRow {
    GeometryRow { entry, geometry: CaseGeometry { form, width, length, height, load, aux } }
}

/// 📐️ The declaration of every catalogue entry, in registry order.
pub static GEOMETRY: &[GeometryRow] = &[
    row("M-10", CaseForm::Wall, "t_m", "l_m", "h_m", "", &[("dim_x", "bearing_m"), ("dim_x2", "stiff_len_m"), ("dim_z", "stiff_t_m")]),
    row("M-11", CaseForm::Wall, "t_m", "", "hef_m", "", &[("dim_z", "t1_m"), ("dim_z2", "t2_m")]),
    row("M-12", CaseForm::Wall, "t_m", "", "hef_m", "n_top_n", &[("dim_y", "ehe_m")]),
    row("M-13", CaseForm::Wall, "t_m", "", "hef_m", "n_mid_n", &[]),
    row("M-14", CaseForm::Wall, "t_m", "lefm_m", "hc_m", "ned_c_n", &[("dim_x", "a1_m")]),
    row("M-15", CaseForm::Wall, "t_m", "pier_l_m", "", "n_pier_n", &[("dim_y", "opening_w_m")]),
    row("M-16", CaseForm::Wall, "t_m", "", "h_m", "w_ed_pa", &[("dim_z", "hef_m")]),
    row("M-20", CaseForm::Wall, "t_m", "l_m", "", "v_ed_n", &[("dim_z", "flange_t_m"), ("dim_x", "web_spacing_m"), ("dim_z2", "height_total_m")]),
    row("M-21", CaseForm::Wall, "", "b_m", "h_m", "v_ed_n", &[("dim_y", "e0_m")]),
    row("M-30", CaseForm::Wall, "t_m", "", "h_m", "w_ed_pa", &[]),
    row("M-31", CaseForm::Wall, "t_m", "l_m", "h_m", "w_ed_pa", &[]),
    row("M-32", CaseForm::Wall, "t_m", "la_m", "", "w_ed_pa", &[]),
    row("M-33", CaseForm::Wall, "", "", "he_m", "qp_pa", &[]),
    row("M-34", CaseForm::Wall, "t_m", "", "h_m", "qk_pa", &[("dim_z", "he_m")]),
    row("M-01", CaseForm::Block, "unit_w_m", "", "unit_h_m", "", &[]),
    row("M-02", CaseForm::Gauge, "", "", "", "fm_pa", &[]),
    row("M-03", CaseForm::Gauge, "", "", "", "sigma_pa", &[]),
    row("M-04", CaseForm::Gauge, "", "", "", "sigma_pa", &[]),
    row("M-05", CaseForm::Cylinder, "diameter_m", "", "lb_avail_m", "", &[]),
    row("M-06", CaseForm::Gauge, "", "", "", "fk_pa", &[]),
    row("M-07", CaseForm::Gauge, "", "", "", "class", &[]),
    row("M-40", CaseForm::Wall, "d_m", "b_m", "", "m_ed_nm", &[]),
    row("M-41", CaseForm::Wall, "d_m", "b_m", "", "v_ed_n", &[("dim_x", "s_m"), ("dim_x2", "av_m")]),
    row("M-42", CaseForm::Wall, "", "l_m", "h_m", "n_ed_n", &[("dim_y", "b_m")]),
    row("M-43", CaseForm::Wall, "t_m", "l_m", "", "v_ed_n", &[("dim_x", "col_min_m"), ("dim_x2", "col_spacing_m")]),
    row("M-44", CaseForm::Wall, "", "", "", "p0_n", &[]),
    row("M-50", CaseForm::Wall, "cavity_m", "", "h_wall_m", "w_ed_pa", &[("dim_z", "t1_m"), ("dim_z2", "t2_m")]),
    row("M-51", CaseForm::Wall, "", "s_anchor_m", "h_trib_m", "w_ed_pa", &[]),
    row("M-45", CaseForm::Cylinder, "diameter_m", "", "provided_m", "", &[]),
    row("M-52", CaseForm::Wall, "", "spacing_m", "", "", &[]),
    row("M-60", CaseForm::Plate, "", "", "cover_m", "", &[]),
    row("M-61", CaseForm::Member, "", "span_m", "d_m", "", &[]),
    row("M-70", CaseForm::Wall, "t_m", "", "", "", &[("dim_z", "bed_joint_m"), ("dim_x", "overlap_m"), ("dim_z2", "unit_h_m"), ("dim_z3", "chase_depth_m"), ("dim_x2", "diameter_m"), ("dim_y", "chase_width_m"), ("dim_x3", "chase_length_m")]),
    row("M-80", CaseForm::Block, "l_m", "span_m", "height_total_m", "qk_pa", &[("dim_z", "h_m"), ("dim_z2", "t_m")]),
    row("M-81", CaseForm::Wall, "t_m", "span_m", "hef_m", "n_ed_n", &[]),
    row("M-82", CaseForm::Wall, "t_m", "lc_m", "", "v_ed_n", &[]),
    row("M-83", CaseForm::Wall, "t_m", "", "h_m", "w_ed_pa", &[("dim_z", "he_m")]),
    row("M-90", CaseForm::Wall, "t_m", "", "", "rei", &[("dim_z", "plaster_m")]),
    row("M-91", CaseForm::Wall, "t_m", "l_m", "", "n_ed_fi_n", &[("dim_z", "a_theta_m")]),
    row("M-95", CaseForm::Gauge, "", "", "", "q", &[]),
];

/// 📐️ The geometry declared for the catalogue entry `entry_id`.
pub fn geometry_of(entry_id: &str) -> Option<CaseGeometry> {
    design_case::geometry_in(GEOMETRY, entry_id)
}
