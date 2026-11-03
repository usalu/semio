//! 🔩️ Joint row component model: the component method of an end plate joint from explicit bolt rows (`JointRow`) or the bolt grid, M_j,Rd, S_j,ini, the classification limits, the moment-rotation characteristic and the secant stiffness fed into the frame analysis. See [EN 1993-1-8 §6](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::joint_components::{self as jc, ColumnSide, PlateSide, RowGeom, RowComponent, RowKind, RowsResult};
use crate::clauses::joints;
use semio_s_artifact_norm_en1993_model::JointRow;

fn e_mpa() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-1.e-modulus")
}
fn psi_end_plate() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-8.psi-end-plate")
}
const CURVE_SAMPLES: usize = 40;

/// 🔩️ Column side of the joint in mm, N and MPa.
pub fn column_side(j: &SteelJoint) -> ColumnSide {
    ColumnSide {
        tfc: j.column_flange_thickness * 1e3,
        twc: j.column_web_thickness * 1e3,
        dc: j.column_web_depth * 1e3,
        avc: j.column_web_shear_area * 1e6,
        rc: j.column_root_radius * 1e3,
        fy: j.column_fy / 1e6,
        w: j.gauge * 1e3,
        e: j.edge_distance * 1e3,
        sigma_com: j.column_web_stress / 1e6,
        beta: j.transformation_parameter.max(1e-6),
        tfb: j.beam_flange_thickness * 1e3,
        ab: j.weld_throat * 1e3,
    }
}

fn kind_of(row_type: &str) -> RowKind {
    match row_type {
        "extended" => RowKind::Extended,
        "inner" => RowKind::Inner,
        _ => RowKind::FirstBelow,
    }
}

fn bolt_tension_rd(class: &str, a_s_mm2: f64, params: AnnexParams) -> f64 {
    0.9 * joints::bolt_fub(class) * a_s_mm2 / params.gamma_m2
}

/// 🔩️ Rows derived from the joint's single bolt grid: the first row extended, the others directly below, all with the grid pitch.
pub fn synthesized_rows(j: &SteelJoint, params: AnnexParams) -> Vec<RowGeom> {
    let a_s = j.bolt_stress_area * 1e6;
    let lever_1 = j.compression_lever * 1e3;
    (0..j.bolt_rows.max(1) as usize)
        .map(|r| RowGeom {
            kind: if r == 0 { RowKind::Extended } else { RowKind::FirstBelow },
            lever: lever_1 - r as f64 * j.pitch * 1e3,
            m: j.bolt_to_web * 1e3,
            e: j.edge_distance * 1e3,
            mx: j.extension_to_flange * 1e3,
            ex: j.extension_edge * 1e3,
            alpha: j.alpha_factor,
            pitch: j.pitch * 1e3,
            bolts: f64::from(j.bolts_per_row),
            d: j.bolt_diameter * 1e3,
            a_s,
            lb: j.bolt_grip_length * 1e3,
            ft_rd_bolt: bolt_tension_rd(&j.bolt_class, a_s, params),
            group: false,
        })
        .collect()
}

pub fn geometry_of(row: &JointRow, params: AnnexParams) -> RowGeom {
    let a_s = row.bolt_stress_area * 1e6;
    RowGeom {
        kind: kind_of(row.row_type.as_str()),
        lever: row.lever * 1e3,
        m: row.bolt_to_web * 1e3,
        e: row.edge_distance * 1e3,
        mx: row.extension_to_flange * 1e3,
        ex: row.extension_edge * 1e3,
        alpha: row.alpha_factor,
        pitch: row.pitch_to_previous * 1e3,
        bolts: f64::from(row.bolts),
        d: row.bolt_diameter * 1e3,
        a_s,
        lb: row.grip_length * 1e3,
        ft_rd_bolt: bolt_tension_rd(&row.bolt_class, a_s, params),
        group: row.group_with_previous == 1,
    }
}

pub fn explicit<'a>(document: &'a En1993Snapshot, j: &SteelJoint) -> Vec<&'a JointRow> {
    let mut rows: Vec<&JointRow> = document.joint_rows.iter().filter(|r| r.joint_id == j.id && r.bolts > 0 && r.lever > 0.0).collect();
    rows.sort_by(|a, b| b.lever.total_cmp(&a.lever));
    rows
}

/// 🔩️ The explicit `JointRow` geometry of the joint when it has rows, else the rows derived from its bolt grid.
pub fn rows_of(document: &En1993Snapshot, j: &SteelJoint, params: AnnexParams) -> Vec<RowGeom> {
    let rows = explicit(document, j);
    if rows.is_empty() { synthesized_rows(j, params) } else { rows.into_iter().map(|r| geometry_of(r, params)).collect() }
}

/// 🔩️ Component method result of the joint with end plate thickness `tp` (mm).
pub fn component_result(j: &SteelJoint, rows: &[RowGeom], tp: f64, params: AnnexParams) -> RowsResult {
    let plate = PlateSide { tp, bp: j.plate_width * 1e3, fy: j.plate_fy / 1e6, twb: j.beam_web_thickness * 1e3, fy_wb: j.beam_web_fy / 1e6 };
    let f_beam = jc::fc_fb_rd(j.beam_moment_resistance * 1e3, j.compression_lever * 1e3);
    jc::component_rows(&column_side(j), &plate, rows, f_beam, e_mpa(), params.gamma_m0, params.gamma_m1)
}

/// 🔩️ Everything the scene and the diagrams show of one end plate joint: components per row, M_j,Rd, S_j,ini, the classification limits and the φ(M) characteristic (N, N·m, rad).
#[derive(Clone, Debug)]
pub struct JointRowSummary {
    pub joint_id: String,
    pub row_ids: Vec<String>,
    pub levers: Vec<f64>,
    pub component_forces: Vec<[f64; 4]>,
    pub row_forces: Vec<f64>,
    pub governing: Vec<RowComponent>,
    pub m_j_rd: f64,
    pub m_j_ed: f64,
    pub s_ini: f64,
    pub z: f64,
    pub stiffness_class: &'static str,
    pub strength_class: &'static str,
    pub rigid_limit: f64,
    pub pinned_limit: f64,
    pub curve: Vec<(f64, f64)>,
    pub compression_cap: f64,
    pub column_web_compression: f64,
    pub panel_shear: f64,
    pub beam_compression: f64,
}

/// 🔩️ The component summary of an end plate joint, `None` for other joint kinds or incomplete geometry.
pub fn joint_row_summary(document: &En1993Snapshot, joint_id: &str) -> Option<JointRowSummary> {
    let j = document.joints.iter().find(|j| j.id == joint_id)?;
    if j.kind != semio_s_artifact_norm_en1993_model::JointKind::EndPlate || j.plate_thickness <= 0.0 || j.column_flange_thickness <= 0.0 || j.compression_lever <= 0.0 {
        return None;
    }
    let params = AnnexParams::for_choice(document.annex);
    let explicit_rows = explicit(document, j);
    let rows = rows_of(document, j, params);
    let result = component_result(j, &rows, j.plate_thickness * 1e3, params);
    let m_rd = result.m_j_rd / 1e3;
    let braced = j.braced_frame == 1;
    let (ib, lb) = ((j.beam_inertia * 1e12).max(1.0), (j.beam_span * 1e3).max(1.0));
    let s_ini = result.s_ini / 1e3;
    let stiffness_class = match jc::classify_stiffness_frame(result.s_ini, e_mpa(), ib, lb, braced, j.stiffness_ratio) {
        "nominally-pinned" => "pinned",
        "rigid" => "rigid",
        _ => "semiRigid",
    };
    let curve = jc::m_phi_curve(s_ini, m_rd, psi_end_plate(), CURVE_SAMPLES);
    Some(JointRowSummary {
        joint_id: j.id.clone(),
        row_ids: explicit_rows.iter().map(|r| r.id.clone()).collect(),
        levers: rows.iter().map(|r| r.lever / 1e3).collect(),
        component_forces: result.rows.iter().map(|r| [r.f_column_flange, r.f_end_plate, r.f_web_tension, r.f_beam_web]).collect(),
        row_forces: result.rows.iter().map(|r| r.f_rd).collect(),
        governing: result.rows.iter().map(|r| r.governing).collect(),
        m_j_rd: m_rd,
        m_j_ed: j.moment_demand,
        s_ini,
        z: result.z / 1e3,
        stiffness_class,
        strength_class: jc::classify_strength_full(m_rd, j.beam_moment_resistance, j.column_moment_resistance, false),
        rigid_limit: (if braced { 8.0 } else { 25.0 }) * e_mpa() * ib / lb / 1e3,
        pinned_limit: 0.5 * e_mpa() * ib / lb / 1e3,
        curve,
        compression_cap: result.cap,
        column_web_compression: result.f_c_wc_rd,
        panel_shear: result.v_wp_rd,
        beam_compression: result.f_beam,
    })
}

/// 🔗️ The secant rotational stiffness S_j = S_j,ini/μ of the joint at its design moment M_j,Ed [N·m/rad] (EN 1993-1-8 5.1.2, 6.3.1), `None` for joint kinds or geometry without a component result.
pub fn joint_secant(document: &En1993Snapshot, joint: &SteelJoint) -> Option<f64> {
    let s = joint_row_summary(document, &joint.id)?;
    let secant = jc::s_j(s.s_ini, jc::mu_stiffness(s.m_j_ed, s.m_j_rd, psi_end_plate()));
    (secant.is_finite() && secant > 0.0).then_some(secant)
}
