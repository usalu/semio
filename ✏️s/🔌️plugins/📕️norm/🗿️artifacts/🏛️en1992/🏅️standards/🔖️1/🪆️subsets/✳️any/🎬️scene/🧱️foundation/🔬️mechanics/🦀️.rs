//! 🔬️ Mechanics: the pure engineering geometry behind the pictures (SI units, `y` across the width, `z` up, origin at the section centroid, compression and sagging positive).
//! Every function here reads the one general section [`Sec`] of the check engine; none of it replaces the verification there, it only draws.

use super::checks::{self, Sec};
use super::layout::{class_of, Class};
use super::model::{En1992Snapshot, RcMember, SupportCondition};
use super::norm::ActionFactors;
use super::reinforcement::is_top;
use std::f64::consts::PI;

const MM: f64 = 1.0e-3;

pub const X_LADDER: [f64; 18] = [0.01, 0.03, 0.06, 0.1, 0.15, 0.2, 0.27, 0.35, 0.45, 0.55, 0.7, 0.85, 1.0, 1.2, 1.5, 2.0, 4.0, 1000.0];

/// 🧮️ Stress resultant about the centroid: normal force (compression positive) and moments about `y` (`my = Σ F z`) and `z` (`mz = Σ F y`).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Resultant {
    pub n: f64,
    pub my: f64,
    pub mz: f64,
}

/// 🎯️ Design effects of the displayed combination: `n` compression positive, `my` sagging positive.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Effects {
    pub n: f64,
    pub my: f64,
    pub mz: f64,
}

/// 🪨️ Crack-control state of a bending member (EN 1992-1-1 7.3.4).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CrackState {
    pub x: f64,
    pub hc_eff: f64,
    pub rho_p_eff: f64,
    pub sr_max: f64,
}

fn gamma(kind: semio_s_artifact_norm_en1992_model::ActionKind, factors: &ActionFactors) -> f64 {
    if kind == semio_s_artifact_norm_en1992_model::ActionKind::Permanent { factors.gamma_g_sup() } else { factors.gamma_q_building() }
}

fn sagging_coefficient(support: SupportCondition) -> f64 {
    match support {
        SupportCondition::SimplySupported => 1.0 / 8.0,
        SupportCondition::Fixed => 1.0 / 24.0,
        SupportCondition::Continuous => 9.0 / 128.0,
        SupportCondition::Cantilever => -0.5,
    }
}

/// 🎯️ The ULS point of the report: the largest flexure station of the member (`|M_Ed|` and the axial force of the governing combination, compression positive, no `Mz` in the snapshot); `None` without flexure stations.
pub fn reported_effects(report: &super::norm::CheckReport, member: &RcMember) -> Option<Effects> {
    let id = format!("en1992.6.1.flexure.{}", member.id);
    let station = report.checks.iter().find(|c| c.id == id)?.stations.iter().max_by(|a, b| a.utilization.total_cmp(&b.utilization))?;
    let (m, n) = (station.values.first()?.value, station.values.get(2)?.value);
    Some(Effects { n: -n, my: m, mz: 0.0 })
}

/// 🎯️ The illustrative ULS point when the report has no station: permanent actions γ_G,sup, the leading variable action γ_Q and the others γ_Q ψ0 of their category (registry rows of the annex, K_FI of the reliability class); accidental actions are not combined. Illustrative fallback only, the verdict is the report.
pub fn design_effects(member: &RcMember, factors: &ActionFactors) -> Effects {
    let live: Vec<&super::model::LoadCaseActions> = member.actions.iter().filter(|a| a.kind != semio_s_artifact_norm_en1992_model::ActionKind::Accidental).collect();
    let g: f64 = live.iter().filter(|a| a.kind == semio_s_artifact_norm_en1992_model::ActionKind::Permanent).map(|a| a.g_k_line).sum::<f64>() + member.udl;
    let q: Vec<(f64, f64)> = live.iter().map(|a| (a.q_k_line, checks::psi_factors(factors, a.category).0)).collect();
    let gamma_q = factors.gamma_q_building();
    let (lead, q_max) = q.iter().enumerate().fold((usize::MAX, 0.0_f64), |best, (i, p)| if p.0 > best.1 { (i, p.0) } else { best });
    let accompanying: f64 = q.iter().enumerate().filter(|(i, _)| *i != lead).map(|(_, p)| gamma_q * p.1 * p.0).sum();
    let w_d = factors.gamma_g_sup() * g + gamma_q * q_max + accompanying;
    let n = live.iter().map(|a| gamma(a.kind, factors) * a.n_k).sum();
    let m_loads: f64 = live.iter().map(|a| gamma(a.kind, factors) * a.m_k).sum();
    let span_moment = if class_of(member.kind) == Class::Upright { 0.0 } else { sagging_coefficient(member.support) * w_d * member.span.powi(2) };
    Effects { n, my: m_loads + span_moment, mz: 0.0 }
}

/// 📊️ Characteristic line load of the displayed deflection (N/m).
pub fn characteristic_line_load(member: &RcMember) -> f64 {
    member.actions.iter().filter(|a| a.kind != semio_s_artifact_norm_en1992_model::ActionKind::Accidental).map(|a| a.g_k_line + a.q_k_line).sum::<f64>() + member.udl
}

/// 📉️ Relative moment `|M| / |M|max` of a uniformly loaded span at `xi = x / L` (cantilever fixed at `xi = 0`, continuous propped at `xi = 1`).
pub fn moment_shape(support: SupportCondition, xi: f64) -> f64 {
    match support {
        SupportCondition::SimplySupported => 4.0 * xi * (1.0 - xi),
        SupportCondition::Cantilever => (1.0 - xi).powi(2),
        SupportCondition::Fixed => (6.0 * xi - 6.0 * xi * xi - 1.0).abs(),
        SupportCondition::Continuous => (3.0 * xi - 4.0 * xi * xi).abs(),
    }
}

/// 🌊️ Elastic deflection `w / (q L^4 / EI)` of a uniformly loaded span at `xi = x / L`.
pub fn deflection_coefficient(support: SupportCondition, xi: f64) -> f64 {
    match support {
        SupportCondition::SimplySupported => xi * (1.0 - 2.0 * xi * xi + xi.powi(3)) / 24.0,
        SupportCondition::Cantilever => xi * xi * (6.0 - 4.0 * xi + xi * xi) / 24.0,
        SupportCondition::Fixed => (xi * (1.0 - xi)).powi(2) / 24.0,
        SupportCondition::Continuous => xi * (1.0 - 3.0 * xi * xi + 2.0 * xi.powi(3)) / 48.0,
    }
}

/// 🏗️ The general section of `member` with its bars, tendon and design strengths of the document, the one section every check and every picture reads; `None` without its concrete or steel grade.
pub fn member_sec(document: &En1992Snapshot, member: &RcMember) -> Option<Sec> {
    let (c, st) = (document.concrete(&member.concrete_grade_id)?, document.reinforcement(&member.reinforcement_grade_id)?);
    Some(checks::member_section_in(document, member, c.f_ck, st.f_yk, st.e_s, document.annex, false))
}

fn resultant(state: (f64, f64, f64)) -> Resultant {
    Resultant { n: state.0, my: state.1 * MM, mz: -state.2 * MM }
}

/// ⛓️ Centric tension: every bar and tendon at its design strength (`my = Σ F z`, `mz = Σ F y` in N·m).
pub fn tension(sec: &Sec) -> Resultant {
    let bars = sec.bars.iter().map(|b| (-sec.fyd * b.a, b.y, b.z));
    let tendons = sec.tendons.iter().map(|t| (-sec.fp_d * t.a, t.y, t.z));
    bars.chain(tendons).fold(Resultant::default(), |r, (f, y, z)| Resultant { n: r.n + f, my: r.my + f * z * MM, mz: r.mz + f * y * MM })
}

/// 🧱️ Centric compression resistance `N0` of the general section (N).
pub fn n0(sec: &Sec) -> f64 {
    sec.n_compression()
}

/// 📏️ The larger overall dimension of the section (m).
pub fn size(sec: &Sec) -> f64 {
    let g = sec.gross_props();
    (g.y_max - g.y_min).max(g.z_top - g.z_bot) * MM
}

fn depth_along(sec: &Sec, theta: f64) -> f64 {
    let (lo, hi) = sec.extent((theta.cos(), theta.sin()));
    hi - lo
}

/// 🕸️ The N-My-Mz capacity surface by strain compatibility on the polygon: one row per neutral-axis depth of [`X_LADDER`] (the first row is centric tension), `angles + 1` columns around the section (the last repeats the first); `mz = Σ F y`.
pub fn surface(sec: &Sec, angles: usize) -> Vec<Vec<Resultant>> {
    let mut rows = vec![vec![tension(sec); angles + 1]];
    for rel in X_LADDER {
        rows.push((0..=angles).map(|k| {
            let theta = 2.0 * PI * k as f64 / angles as f64;
            resultant(sec.state(theta, rel * depth_along(sec, theta)))
        }).collect());
    }
    rows
}

/// ⭕️ The `(My, Mz)` capacity ring at normal force `n` (N, compression positive) around the section, convex-hull ready.
pub fn slice(sec: &Sec, n: f64, angles: usize) -> Vec<[f64; 2]> {
    (0..angles)
        .filter_map(|k| {
            let theta = 2.0 * PI * k as f64 / angles as f64;
            let r = resultant(sec.state(theta, sec.solve_x(theta, n)?));
            Some([r.my, r.mz])
        })
        .collect()
}

/// 🌊️ Short-term deflection of the uncracked section at `xi` (m, downwards positive).
pub fn deflection(sec: &Sec, member: &RcMember, xi: f64) -> f64 {
    let ei = sec.c.ecm * 1.0e6 * sec.gross_props().i_y * 1.0e-12;
    characteristic_line_load(member) * member.span.powi(4) / ei * deflection_coefficient(member.support, xi)
}

/// 🪨️ Crack spacing of EN 1992-1-1 7.3.4 for the elastic cracked state of the general section under the quasi-permanent moment and normal force of the member: the neutral axis from [`Sec::elastic`], A_c,eff from the outline, k1 = 0,8, k2 = 0,5 and k3, k4 from the registry of the annex.
pub fn crack_state(sec: &Sec, member: &RcMember, factors: &ActionFactors) -> Option<CrackState> {
    let annex = factors.annex();
    let effects = checks::combine_member_actions(member, factors);
    let qp = checks::governing(&effects, "sls_qp")?;
    let state = sec.elastic(qp.m_ed.abs() * 1.0e3, -qp.n_ed, false)?;
    let phi = member.longitudinal.iter().filter(|l| !is_top(l.position)).map(|l| l.diameter).fold(0.0, f64::max);
    let (hc, ac) = sec.crack_zone(state.x);
    let rho_p_eff = sec.tension_area() / ac.max(1e-9);
    (phi > 0.0 && rho_p_eff > 0.0).then(|| CrackState { x: state.x * MM, hc_eff: hc * MM, rho_p_eff, sr_max: checks::ndp(annex, "ndp.en1992.k3-crack") * member.cover + 0.8 * 0.5 * checks::ndp(annex, "ndp.en1992.k4-crack") * phi / rho_p_eff })
}

