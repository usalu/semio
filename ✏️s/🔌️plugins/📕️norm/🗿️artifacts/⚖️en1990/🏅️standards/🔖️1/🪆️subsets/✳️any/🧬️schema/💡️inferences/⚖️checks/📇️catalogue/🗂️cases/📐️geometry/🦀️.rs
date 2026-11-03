//! 📐️ Geometry declarations of the EN1990 design cases: per catalogue entry the form it draws in 3D, the inputs that size it and drive its load bar and the auxiliary roles of its form; generated from the hand-maintained `./🔣️.json` by `🧰️scripts/catalogue-bridge-v5/geometry_gen.py`. See [`super`].

use semio_s_plugin_norm_compliance::design_case::{self, AuxRow, CaseForm, CaseGeometry, GeometryRow};

const fn row(entry: &'static str, form: CaseForm, width: &'static str, length: &'static str, height: &'static str, load: &'static str, aux: &'static [AuxRow]) -> GeometryRow {
    GeometryRow { entry, geometry: CaseGeometry { form, width, length, height, load, aux } }
}

/// 📐️ The declaration of every catalogue entry, in registry order.
pub static GEOMETRY: &[GeometryRow] = &[
    row("E90.01", CaseForm::Symbol, "", "", "", "", &[]),
    row("E90.02", CaseForm::Gauge, "", "", "", "gamma_f", &[]),
    row("E90.03", CaseForm::Gauge, "", "", "", "years", &[]),
    row("E90.04", CaseForm::Gauge, "", "", "", "years", &[]),
    row("E90.05", CaseForm::Gauge, "", "", "", "rc", &[]),
    row("E90.06", CaseForm::Symbol, "", "", "", "", &[]),
    row("E90.07", CaseForm::Symbol, "", "", "", "", &[]),
    row("E90.08", CaseForm::Load, "", "", "", "g_mean", &[]),
    row("E90.09", CaseForm::Symbol, "", "", "", "", &[("dim_z", "altitude")]),
    row("E90.10", CaseForm::Gauge, "", "", "", "x_k", &[]),
    row("E90.11", CaseForm::Column, "a_nom", "a_nom", "", "", &[("dim_x", "delta_a")]),
    row("E90.12", CaseForm::Gauge, "", "", "", "alpha_cr", &[]),
    row("E90.13", CaseForm::Gauge, "", "", "", "mean", &[]),
    row("E90.14", CaseForm::Symbol, "", "", "", "", &[]),
    row("E90.20", CaseForm::Gauge, "", "", "", "gamma_f", &[]),
    row("E90.21", CaseForm::Gauge, "", "", "", "delta_sigma_c", &[]),
    row("E90.15", CaseForm::Load, "", "", "", "t_d", &[("dim_z", "altitude")]),
    row("E90.16", CaseForm::Gauge, "", "", "", "r_d", &[("dim_z", "altitude")]),
    row("E90.17", CaseForm::Gauge, "", "", "", "r_d", &[("dim_z", "altitude")]),
    row("E90.18", CaseForm::Gauge, "", "", "", "r_d", &[("dim_z", "altitude")]),
    row("E90.19", CaseForm::Gauge, "", "", "", "cd_char", &[("dim_z", "altitude"), ("dim_x", "cd_freq"), ("dim_x2", "cd_qp")]),
    row("E90.23", CaseForm::Symbol, "", "", "", "", &[]),
    row("E90.22", CaseForm::Symbol, "", "", "", "", &[("dim_z", "altitude")]),
    row("E90.24", CaseForm::Block, "span", "", "height", "w_c", &[("dim_z", "h_storey")]),
    row("E90.25", CaseForm::Symbol, "", "", "", "", &[]),
    row("E90.26", CaseForm::Gauge, "", "", "", "r_d", &[]),
    row("E90.27", CaseForm::Member, "", "span", "", "acc_v", &[]),
    row("E90.28", CaseForm::Gauge, "", "", "", "beta", &[]),
    row("E90.29", CaseForm::Gauge, "", "", "", "beta_t", &[]),
    row("E90.30", CaseForm::Gauge, "", "", "", "mean", &[]),
];

/// 📐️ The geometry declared for the catalogue entry `entry_id`.
pub fn geometry_of(entry_id: &str) -> Option<CaseGeometry> {
    design_case::geometry_in(GEOMETRY, entry_id)
}
