//! 🧮️ Profile: the vertical state of one wall under one load case (design actions at top, middle and bottom, eccentricities, reduction factors Φ and utilisation per station),
//! computed through the same public `masonry` functions and the same EN 1990 combinations as the compression checks, so zones and diagrams agree with the report.

extern crate semio_s_artifact_norm_en1996_model as model;
extern crate semio_s_artifact_norm_en1996_schema as schema;
extern crate semio_s_plugin_norm_compliance as norm;

use model::{ConcentratedLoad, En1996Snapshot, MasonryWall, WallLoadCase};
use norm::{AnnexChoice, DesignSituation};
use schema::checks::masonry as m;

/// ⚖️ One EN 1990 combination: vertical design force `n` at top, middle and bottom, design shear `v` and design wind pressure `w`.
#[derive(Clone, Debug, PartialEq)]
pub struct Effects {
    pub n: [f64; 3],
    pub v: f64,
    pub w: f64,
}

fn exceptional(lc: &WallLoadCase, default: DesignSituation) -> bool {
    matches!(DesignSituation::from(lc.design_situation), DesignSituation::Accidental | DesignSituation::Seismic) || matches!(default, DesignSituation::Accidental)
}

/// ⚖️ The ULS combinations of a load case for the annex: the design effects of the shared EN 1990 combination (`m::design_effects`) at top, middle and bottom.
pub fn effects(annex: AnnexChoice, wall: &MasonryWall, lc: &WallLoadCase, default: DesignSituation, reliability_class: norm::ReliabilityClass) -> Vec<Effects> {
    m::design_effects(annex, wall, lc, default, reliability_class).into_iter().map(|d| Effects { n: [d.n_top, d.n_mid, d.n_bot], v: d.v, w: d.w_pa }).collect()
}

/// 🧱️ The vertical state of a wall under one load case; station order is top, middle, bottom.
#[derive(Clone, Debug, PartialEq)]
pub struct Profile {
    pub load_case: String,
    pub gamma_m: f64,
    pub f_d: f64,
    pub area: f64,
    pub n: [f64; 3],
    pub e_top: f64,
    pub e_bottom: f64,
    pub e_init: f64,
    pub e_hm: f64,
    pub e_k: f64,
    pub e_mk: f64,
    pub e_slab: f64,
    pub e_theta: f64,
    pub phi: [f64; 3],
    pub phi2: f64,
    pub utilisation: [f64; 3],
    pub lambda: f64,
    pub wind: f64,
    pub shear_sigma: f64,
    pub n_min: f64,
}

impl Profile {
    pub fn governing_station(&self) -> usize {
        (0..3).max_by(|a, b| self.utilisation[*a].total_cmp(&self.utilisation[*b])).unwrap_or(0)
    }

    pub fn resistance(&self, station: usize) -> f64 {
        self.phi[station] * self.f_d * self.area
    }
}

/// 🧮️ The profile of `wall` under `lc` for the document annex, masonry class and design situation.
pub fn profile(document: &En1996Snapshot, wall: &MasonryWall, lc: &WallLoadCase) -> Profile {
    let sit = DesignSituation::from(lc.design_situation);
    let gamma_m = m::AnnexParams::for_document(document.annex, document.masonry_class, if matches!(sit, DesignSituation::Persistent) { document.design_situation } else { sit }).gamma_m();
    let f_d = m::f_k_pa(document.annex, wall) / gamma_m;
    let area = m::area_m2(wall);
    let all = effects(document.annex, wall, lc, document.design_situation, document.reliability_class);
    let ge = all.iter().max_by(|a, b| a.n[2].total_cmp(&b.n[2])).unwrap_or(&all[0]).clone();
    let gs = all.iter().max_by(|a, b| (a.v / a.n[1].max(1.0)).total_cmp(&(b.v / b.n[1].max(1.0)))).unwrap_or(&all[0]).clone();
    let t = wall.thickness_m;
    let e_slab = m::eccentricity_from_slab_bearing_m(wall);
    let e_theta = m::eccentricity_from_slab_rotation_m(document.annex, wall, lc, ge.n[0].max(ge.n[1]));
    let phi2 = m::phi_2_slab_span(document.annex, lc.slab_span_m);
    let e_top = (wall.eccentricity_top_m + e_slab + e_theta).abs();
    let e_bottom = wall.eccentricity_bottom_m.abs();
    let h_ef = m::h_ef_m(wall);
    let e_init = h_ef / 450.0;
    let e_hm = ge.w * wall.height_m.powi(2) / 8.0 * m::effective_length_m(wall) / ge.n[1].max(1.0);
    let e_m = 0.5 * (e_top + e_bottom) + e_init + e_hm;
    let e_k = m::creep_eccentricity_m(wall, e_m);
    let e_mk = (e_m + e_k).abs().max(0.05 * t);
    let phi = [m::phi_i(e_top + e_init, t) * phi2, m::phi_m(m::slenderness(wall), e_mk, t, m::k_e(document.annex)) * phi2, m::phi_i(e_bottom + e_init, t) * phi2];
    let utilisation = [0, 1, 2].map(|i| ge.n[i] / (phi[i] * f_d * area).max(1e-9));
    Profile {
        load_case: lc.id.clone(),
        gamma_m,
        f_d,
        area,
        n: ge.n,
        e_top,
        e_bottom,
        e_init,
        e_hm,
        e_k,
        e_mk,
        e_slab,
        e_theta,
        phi,
        phi2,
        utilisation,
        lambda: m::slenderness(wall),
        wind: ge.w,
        shear_sigma: gs.n[1] / area.max(1e-9),
        n_min: all.iter().map(|e| e.n[1]).fold(f64::INFINITY, f64::min),
    }
}

/// 🧮️ The profile of the load case with the largest station utilisation; `None` for a wall without load cases.
pub fn governing(document: &En1996Snapshot, wall: &MasonryWall) -> Option<Profile> {
    wall.load_cases
        .iter()
        .map(|lc| profile(document, wall, lc))
        .max_by(|a, b| a.utilisation[a.governing_station()].total_cmp(&b.utilisation[b.governing_station()]))
}

/// 🧱️ The lateral resistance of a wall panel: design moments of resistance per metre in the two directions, the edge conditions of the held sides and the design wind pressure of the governing load case.
#[derive(Clone, Debug, PartialEq)]
pub struct Lateral {
    pub load_case: String,
    pub m1: f64,
    pub m2: f64,
    pub edges: m::numerical_models::Edges,
    pub l_ef: f64,
    pub w_ed: f64,
}

impl Lateral {
    pub fn pressure(&self, l: f64, h: f64) -> f64 {
        m::numerical_models::yield_line_pressure(self.edges, l, h, self.m1, self.m2)
    }
}

/// 🧱️ The lateral resistance of `wall` under the load case with the largest wind pressure; `None` when no load case carries wind.
pub fn lateral(document: &En1996Snapshot, wall: &MasonryWall) -> Option<Lateral> {
    let lc = wall.load_cases.iter().filter(|lc| lc.q_p_wind_pa * lc.c_pe.abs() > 0.0).max_by(|a, b| (a.q_p_wind_pa * a.c_pe.abs()).total_cmp(&(b.q_p_wind_pa * b.c_pe.abs())))?;
    let gamma_m = m::AnnexParams::for_document(document.annex, document.masonry_class, document.design_situation).gamma_m();
    let z = wall.thickness_m * wall.thickness_m / 6.0;
    let sigma = governing(document, wall).map_or(0.0, |p| (p.n_min / m::area_m2(wall).max(1e-9)).clamp(0.0, 0.2 * p.f_d));
    let code = match wall.support_sides {
        2 => "SSFF",
        3 => "SSSF",
        _ => "SSSS",
    };
    let yq = m::gamma_q(document.annex, document.reliability_class, exceptional(lc, document.design_situation));
    Some(Lateral {
        load_case: lc.id.clone(),
        m1: (m::f_xk_pa(1, wall) / gamma_m + sigma) * z,
        m2: m::f_xk_pa(2, wall) / gamma_m * z,
        edges: m::numerical_models::Edges::parse(code)?,
        l_ef: m::effective_length_m(wall),
        w_ed: yq * lc.q_p_wind_pa * lc.c_pe.abs(),
    })
}

/// 📐️ The bearing area A_b, the effective area A_ef = l_efm·t (60° spread to mid-height, clipped to the effective wall length) and the enhancement β of Eq. 6.10 of a concentrated load, the same terms the check evaluates.
pub fn bearing_terms(wall: &MasonryWall, c: &ConcentratedLoad) -> (f64, f64, f64) {
    let ab = if c.bearing_area_m2 > 0.0 { c.bearing_area_m2 } else { c.bearing_length_m * wall.thickness_m };
    let a_ef = (c.bearing_length_m + wall.height_m / 3f64.sqrt()).min(m::effective_length_m(wall)) * wall.thickness_m;
    (ab, a_ef, m::concentrated_beta(c.distance_a1_m.max(0.0), wall.height_m, ab, a_ef))
}
