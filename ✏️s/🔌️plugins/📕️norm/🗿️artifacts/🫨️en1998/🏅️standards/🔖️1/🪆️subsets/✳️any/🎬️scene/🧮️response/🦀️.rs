//! 🧮️ Response: the seismic quantities the projection draws (masses, T1, base shear, storey forces, modes), derived with the same `part_1` functions as the full seismic check so picture and report cannot disagree.

use super::model::{En1998Building, En1998Snapshot};
use super::schema::{cracked_stiffness, na_de, part_1, part_3, wall_plan, AnnexParams};

pub use super::structure_views::Site;

/// 🧭️ Seismic forces of one horizontal resisting system.
#[derive(Clone, Debug, PartialEq)]
pub struct SystemResponse {
    pub id: String,
    pub along_y: bool,
    pub q: f64,
    pub s_d: f64,
    pub f_b: f64,
    pub delta: f64,
    pub forces: Vec<f64>,
}

/// 🌊️ One eigenmode of the shear building in one horizontal direction: normalised shape, period, participation factor, effective mass share and the elastic modal displacement per unit of the normalised shape in metres (u_i = Γ·S_De·φ_i).
#[derive(Clone, Debug, PartialEq)]
pub struct Mode {
    pub along_y: bool,
    pub index: u32,
    pub period: f64,
    pub shape: Vec<f64>,
    pub gamma: f64,
    pub m_eff_ratio: f64,
    pub amplitude_m: f64,
}

/// 🏢️ Seismic response of one building.
#[derive(Clone, Debug, PartialEq)]
pub struct Response {
    pub modes: Vec<Mode>,
    pub masses: Vec<f64>,
    pub heights: Vec<f64>,
    pub elevations: Vec<f64>,
    pub t1: f64,
    pub t1_gross: f64,
    pub lambda: f64,
    pub systems: Vec<SystemResponse>,
    pub mode: Vec<f64>,
    pub mode_period: f64,
}

/// 🌍️ Resolves the annex of the document exactly as the full seismic check does.
pub fn annex_params(document: &En1998Snapshot) -> AnnexParams {
    if document.annex == semio_s_plugin_norm_compliance::AnnexChoice::En {
        let ground = part_1::spectrum_ground_letter_of(document);
        AnnexParams::En { a_gr: document.site.a_gr, ground, spectrum_type1: document.site.en_spectrum_type == semio_s_artifact_norm_en1998_model::EnSpectrumType::Type1 }
    } else {
        AnnexParams::De { s_ap_r: document.site.s_ap_r, combo: na_de::GroundCombo::from(document.site.de_ground_combo) }
    }
}

pub fn site(document: &En1998Snapshot) -> Site {
    let (a_gr, s, tb, tc, td) = annex_params(document).ground_params();
    Site { a_gr, s, tb, tc, td, gamma_i: part_1::gamma_i(document.site.importance_class), section: part_3::AnnexA::from_registry(annex_params(document).choice()) }
}

pub fn respond(site: &Site, building: &En1998Building) -> Response {
    let n = building.storeys.len();
    let masses: Vec<f64> = building.storeys.iter().enumerate().map(|(i, s)| part_1::storey_seismic_mass_kg(s, i + 1 == n)).collect();
    let heights: Vec<f64> = building.storeys.iter().map(|s| s.height_m).collect();
    let elevations: Vec<f64> = heights.iter().scan(0.0, |z, h| { *z += h; Some(*z) }).collect();
    let total_height = elevations.last().copied().unwrap_or(0.0);
    let total_mass: f64 = masses.iter().sum();
    let (stiffness_x, stiffness_y) = cracked_stiffness::storeys_xy(building);
    let (t1, _) = part_1::fundamental_period_s(building.t1_method, building.ct, building.t1_given_s, total_height, &masses, &stiffness_x, &stiffness_y);
    let (gross_x, gross_y) = cracked_stiffness::gross_storeys_xy(building);
    let (t1_gross, _) = part_1::fundamental_period_s(building.t1_method, building.ct, building.t1_given_s, total_height, &masses, &gross_x, &gross_y);
    let lambda = part_1::lambda_factor(t1, site.tc, n);
    let systems = building.systems.iter().map(|system| system_response(site, building, system, &masses, &heights, total_mass, t1, lambda)).collect();
    let modes = modes_of(site, building, &masses);
    let (mode, mode_period) = modes.iter().find(|m| !m.along_y && m.index == 1).map_or((Vec::new(), 0.0), |m| (m.shape.clone(), m.period));
    Response { modes, mode, mode_period, masses, heights, elevations, t1, t1_gross, lambda, systems }
}

fn system_response(site: &Site, building: &En1998Building, system: &super::model::En1998System, masses: &[f64], heights: &[f64], total_mass: f64, t1: f64, lambda: f64) -> SystemResponse {
    let along_y = system.direction == semio_s_artifact_norm_en1998_model::AnalysisDirection::Y;
    let q = part_1::behaviour_factor(system.q0, system.alpha_u_over_alpha_1, system.k_w).min(part_1::q_max_for_system(system.system_type, system.material, system.ductility_class));
    let s_e = part_1::elastic_response_spectrum(site.a_gr, site.s, site.tb, site.tc, site.td, t1);
    let s_d = part_1::design_spectrum_at(site.a_gr, site.gamma_i, site.s, site.tb, site.tc, site.td, q, t1);
    let f_b = part_1::base_shear_n(s_d, total_mass, lambda);
    let plan = if along_y { building.plan_width_m } else { building.plan_length_m };
    let delta = if building.plan_width_m > 0.0 && building.plan_length_m > 0.0 {
        let e0 = building.storeys.iter().map(|s| {
            let (cs_x, cs_y, _) = wall_plan::stiffness_centre_of(&building.members, s);
            if along_y { (s.centre_of_mass_x_m - cs_x).abs() } else { (s.centre_of_mass_y_m - cs_y).abs() }
        }).fold(0.0_f64, f64::max);
        part_1::torsion_amplification(0.5 * plan.max(1e-9) + e0, plan.max(1e-9))
    } else {
        1.0
    };
    let forces = part_1::storey_forces_n(f_b, masses, heights).into_iter().map(|f| f * delta).collect();
    SystemResponse { id: system.id.clone(), along_y, q, s_d, f_b, delta, forces }
}

fn modes_of(site: &Site, building: &En1998Building, masses: &[f64]) -> Vec<Mode> {
    let total: f64 = masses.iter().sum();
    let mut out = Vec::new();
    for along_y in [false, true] {
        let k: Vec<f64> = building.storeys.iter().map(|s| if along_y { cracked_stiffness::storey_y(building, s) } else { cracked_stiffness::storey_x(building, s) }).collect();
        if masses.is_empty() || masses.iter().any(|m| *m <= 0.0) || k.iter().any(|v| *v <= 0.0) {
            continue;
        }
        let (periods, shapes) = part_1::shear_building_modes(masses, &k);
        let m_eff = part_1::effective_modal_masses(masses, &shapes);
        for (n, (period, shape)) in periods.iter().zip(&shapes).enumerate() {
            let peak = shape.iter().copied().fold(0.0_f64, |a, v| if v.abs() > a.abs() { v } else { a });
            if peak.abs() < 1e-12 || *period <= 0.0 {
                continue;
            }
            let shape: Vec<f64> = shape.iter().map(|v| v / peak).collect();
            let l: f64 = shape.iter().zip(masses).map(|(p, m)| p * m).sum();
            let m_n: f64 = shape.iter().zip(masses).map(|(p, m)| p * p * m).sum();
            let gamma = if m_n > 0.0 { l / m_n } else { 0.0 };
            let s_e = part_1::elastic_response_spectrum(site.a_gr, site.s, site.tb, site.tc, site.td, *period);
            let s_de = site.gamma_i * s_e * (period / (2.0 * std::f64::consts::PI)).powi(2);
            out.push(Mode { along_y, index: n as u32 + 1, period: *period, shape, gamma, m_eff_ratio: if total > 0.0 { m_eff[n] / total } else { 0.0 }, amplitude_m: gamma * s_de });
        }
    }
    out
}

/// 🧱️ Required clear gap between two neighbouring units, d ≥ √(d₁² + d₂²) (EN 1998-1 4.4.2.7).
pub fn joint_demand(a: &En1998Building, b: &En1998Building) -> f64 {
    let top = |x: &En1998Building| x.storeys.iter().map(|s| s.drift_x_m).sum::<f64>();
    part_1::seismic_joint_gap_m(top(a), top(b))
}
