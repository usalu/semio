//! 🔭️ Special-part checks: the pure clause modules of `🧮️clauses` (cold-formed, stainless, plated, shells, joints, tension, bridges, towers, containment, sheet piles, crane runways, brittle fracture) applied per entity. See [EN 1993](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::{bridge_fatigue, cold_formed, cold_stainless, containment, crane_runway, effective_width, joint_components as jc, joints, misc, plated_ext, plates_shells, resistance, sheet_piling, shell_stability, stainless, tension_components, toughness, towers_masts};
use jc::{Mode1Method, RowGeom};
use semio_s_artifact_norm_en1993_model::{BridgeFatigue, ColdFormedMember, CraneRunway, FireExposure, PlatedPanel, SiloShell, SteelPile, TensionComponent, TowerLeg};
use std::f64::consts::PI;

fn nu() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-1.poisson")
}
fn alpha_t() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-1.alpha-t")
}
fn e_mpa() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-1.e-modulus")
}

pub(super) struct Case<'a> {
    pub(super) id: String,
    pub(super) part: &'a str,
    pub(super) family: &'a str,
    pub(super) chapter: &'a str,
    pub(super) section: &'a str,
    pub(super) subject: SubjectRef,
    pub(super) title: LocalizedCopy,
    pub(super) demand: Quantity,
    pub(super) capacity: Quantity,
    pub(super) explanation: LocalizedCopy,
    pub(super) ndps: &'a [&'a str],
    pub(super) gammas: &'a [&'a str],
}

/// 🔭️ Builds one utilisation check; the remedies are computed only when the check fails.
pub(super) fn emit(report: &mut CheckReport, annex: AnnexChoice, case: Case<'_>, remedies: impl FnOnce() -> Vec<Remedy>) {
    let fails = !(case.demand.value <= case.capacity.value);
    let mut b = CheckResult::assess(case.id, case.part, ClauseId::new(case.family, case.chapter, case.section), case.subject.clone(), case.title)
        .annex(annex)
        .utilization(case.demand, case.capacity)
        .explanation(case.explanation);
    for id in case.ndps.iter().chain(case.gammas) {
        b = b.ndp(id);
    }
    if fails {
        let mut list = remedies();
        if list.is_empty() {
            list.push(Remedy::advisory(case.subject, loc("No value of a single field within its limits solves this check: revise several design inputs together.", "Kein Wert eines einzelnen Feldes innerhalb seiner Grenzen löst diese Prüfung: mehrere Bemessungseingaben gemeinsam überarbeiten.")));
        }
        for remedy in list {
            b = b.remedy(remedy);
        }
    }
    report.push(b.build());
}

pub(super) fn subj(id: &str, collection: &str, field: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(id, format!("{collection}[id={id}].{field}"), loc(en, de))
}

const SOLVER_REACH: f64 = 100.0;
const SOLVER_MARGIN: f64 = 1.0e-6;

/// 🎯 The smallest `x` of `[start, limit]` with `ok(x)`, solved by bisection on the check's own utilisation (`ok` must be monotone); `limit` is the largest value the field admits (its hard range, a physical cap) and defaults to `SOLVER_REACH · start`; `None` when `ok` fails even at the limit, so the caller offers no value instead of an unreachable one.
pub(super) fn grow(start: f64, limit: Option<f64>, ok: impl Fn(f64) -> bool) -> Option<f64> {
    let low = start.max(1e-9);
    if ok(low) {
        return Some(low);
    }
    let high = limit.unwrap_or(low * SOLVER_REACH);
    if !(high > low) || !ok(high) {
        return None;
    }
    let (mut lo, mut hi) = (low, high);
    for _ in 0..64 {
        let middle = 0.5 * (lo + hi);
        if ok(middle) { hi = middle } else { lo = middle }
    }
    let padded = (hi * (1.0 + SOLVER_MARGIN)).min(high);
    Some(if ok(padded) { padded } else { hi })
}

/// 🎯 The largest `x` of `[limit, start]` with `ok(x)`, solved by bisection on the check's own utilisation (`ok` must be monotone); `limit` is the smallest value the field admits (its hard range, a physical cap) and defaults to `start / SOLVER_REACH`; `None` when `ok` fails even at the limit.
pub(super) fn shrink(start: f64, limit: Option<f64>, ok: impl Fn(f64) -> bool) -> Option<f64> {
    if ok(start) {
        return Some(start);
    }
    let low = limit.unwrap_or(start / SOLVER_REACH);
    if !(low < start) || !ok(low) {
        return None;
    }
    let (mut lo, mut hi) = (low, start);
    for _ in 0..64 {
        let middle = 0.5 * (lo + hi);
        if ok(middle) { lo = middle } else { hi = middle }
    }
    let padded = (lo * (1.0 - SOLVER_MARGIN)).max(low);
    Some(if ok(padded) { padded } else { lo })
}

/// 🧩 The remedies that could be solved: unreachable ones (`None`) are dropped, never replaced by an invented value.
pub(super) fn solved(list: impl IntoIterator<Item = Option<Remedy>>) -> Vec<Remedy> {
    list.into_iter().flatten().collect()
}

pub(super) fn lift(subject: SubjectRef, current: Quantity, required: Quantity, en: &str, de: &str) -> Remedy {
    Remedy::at_least(subject, current, required, loc(en, de))
}

pub(super) fn cap(subject: SubjectRef, current: Quantity, required: Quantity, en: &str, de: &str) -> Remedy {
    Remedy::at_most(subject, current, required, loc(en, de))
}

pub(super) fn bilingual(en: String, de: String) -> LocalizedCopy {
    loc(&en, &de)
}

fn governing_force(document: &En1993Snapshot, actions: &[semio_s_artifact_norm_en1993_model::ForceAction]) -> (f64, String) {
    let forces: Vec<(String, f64)> = actions.iter().map(|a| (a.load_case_id.clone(), a.force)).collect();
    let combos = combine_scalar_forces(document, &forces);
    let gov = governing_scalar(&combos, "uls");
    (gov.map(|c| c.value.abs()).unwrap_or(0.0), gov.map(|c| c.combination_id.clone()).unwrap_or_else(|| "—".into()))
}

/// 🥫 EN 1993-1-3: distortional buckling of the edge stiffener, web shear, web crippling, bending with crippling and sheeting restraint.
pub(super) fn check_cold_formed(annex: AnnexChoice, params: AnnexParams, c: &ColdFormedMember, n_ed: f64, n_c_rd: f64, report: &mut CheckReport) {
    let (t, fyb, hw, phi) = (c.thickness * 1e3, c.fy / 1e6, c.web_height * 1e3, c.web_slope);
    let (gm0, gm1) = (params.gamma_m0, params.gamma_m1);
    let title = |en: &str, de: &str| loc(en, de);
    let subject = |field: &str, en: &str, de: &str| subj(&c.id, "coldFormedMembers", field, en, de);
    let dist = |cp: f64, th: f64| cold_formed::distortional_edge_stiffener(&cold_formed::EdgeStiffened { bp: c.flange_width * 1e3, cp, t: th, fyb, hw, b2: c.opposite_flange_width * 1e3, kf: c.spring_factor, e: e_mpa() }, gm0);
    if c.lip_length > 0.0 && c.flange_width > 0.0 && c.flange_stress > 0.0 {
        let d = dist(c.lip_length * 1e3, t);
        let (sigma_ed, sigma_rd) = (c.flange_stress / 1e6, d.chi_d * fyb / gm0);
        let case = Case {
            id: format!("en1993.1-3.5.5.dist.{}", c.id),
            part: "DIN EN 1993-1-3",
            family: "EN 1993-1-3",
            chapter: "5",
            section: "5.5.3.2",
            subject: subject("lipLength", "Edge fold", "Randlippe"),
            title: title("Distortional buckling of the edge stiffener", "Verdrehbeulen der Randsteife"),
            demand: stress_pa(sigma_ed * 1e6),
            capacity: stress_pa(sigma_rd * 1e6),
            explanation: bilingual(
                format!("σ_com,Ed={sigma_ed:.1} MPa, χ_d={:.3} (λ̄_d={:.3}, σ_cr,s={:.1} MPa, A_s={:.1} mm², {} iterations), χ_d f_yb/γM0={sigma_rd:.1} MPa.", d.chi_d, d.lambda_d, d.sigma_cr_s, d.a_s, d.iterations),
                format!("σ_com,Ed={sigma_ed:.1} MPa, χ_d={:.3} (λ̄_d={:.3}, σ_cr,s={:.1} MPa, A_s={:.1} mm², {} Iterationen), χ_d f_yb/γM0={sigma_rd:.1} MPa.", d.chi_d, d.lambda_d, d.sigma_cr_s, d.a_s, d.iterations),
            ),
            ndps: &["ndp.en1993.v-250.1"],
            gammas: &[AnnexParams::GAMMA_M0],
        };
        emit(report, annex, case, || {
            let lip = grow(c.lip_length * 1e3, Some(0.6 * c.flange_width * 1e3), |cp| dist(cp, t).chi_d * fyb / gm0 >= sigma_ed);
            let th = grow(t, None, |x| dist(c.lip_length * 1e3, x).chi_d * fyb / gm0 >= sigma_ed);
            solved([
                lip.map(|lip| lift(subject("lipLength", "Edge fold", "Randlippe"), length_m(c.lip_length), length_m(lip / 1e3), &format!("Lengthen the edge fold to at least {lip:.1} mm."), &format!("Randlippe auf mindestens {lip:.1} mm verlängern."))),
                th.map(|th| lift(subject("thickness", "Thickness", "Dicke"), length_m(c.thickness), length_m(th / 1e3), &format!("Or increase the thickness to at least {th:.2} mm."), &format!("Oder die Dicke auf mindestens {th:.2} mm erhöhen."))),
            ])
        });
    }
    if c.web_height > 0.0 && c.shear_force > 0.0 {
        let v_rd = |th: f64| {
            let sw = hw / phi.to_radians().sin();
            let lw = cold_formed::lambda_w_cold(sw, th, fyb, e_mpa());
            cold_formed::v_b_rd_cold(hw, phi, th, cold_formed::f_bv(lw, fyb, c.stiffened_support == 1), gm0)
        };
        let rd = v_rd(t);
        let case = Case {
            id: format!("en1993.1-3.6.1.5.{}", c.id),
            part: "DIN EN 1993-1-3",
            family: "EN 1993-1-3",
            chapter: "6",
            section: "6.1.5",
            subject: subject("webHeight", "Web", "Steg"),
            title: title("Web shear resistance", "Stegschubtragfähigkeit"),
            demand: force_n(c.shear_force),
            capacity: force_n(rd),
            explanation: bilingual(format!("V_Ed={:.2} kN, V_b,Rd={:.2} kN (f_bv from Table 6.1).", c.shear_force / 1e3, rd / 1e3), format!("V_Ed={:.2} kN, V_b,Rd={:.2} kN (f_bv nach Tabelle 6.1).", c.shear_force / 1e3, rd / 1e3)),
            ndps: &["ndp.en1993.v-255.1"],
            gammas: &[AnnexParams::GAMMA_M0],
        };
        emit(report, annex, case, || {
            solved([grow(t, None, |x| v_rd(x) >= c.shear_force).map(|th| lift(subject("thickness", "Thickness", "Dicke"), length_m(c.thickness), length_m(th / 1e3), &format!("Increase the thickness to at least {th:.2} mm."), &format!("Dicke auf mindestens {th:.2} mm erhöhen.")))])
        });
    }
    if c.web_height > 0.0 && c.support_reaction > 0.0 && c.support_length > 0.0 {
        let r_rd = |th: f64, ss: f64| {
            let (alpha, la) = cold_formed::web_crippling_alpha(c.crippling_category as u8, ss);
            cold_formed::web_crippling_rd(alpha, th, fyb, e_mpa(), c.corner_radius * 1e3, la, phi, gm1)
        };
        let rd = r_rd(t, c.support_length * 1e3);
        let case = Case {
            id: format!("en1993.1-3.6.1.7.{}", c.id),
            part: "DIN EN 1993-1-3",
            family: "EN 1993-1-3",
            chapter: "6",
            section: "6.1.7",
            subject: subject("supportReaction", "Support reaction", "Auflagerkraft"),
            title: title("Web crippling", "Stegkrüppeln"),
            demand: force_n(c.support_reaction),
            capacity: force_n(rd),
            explanation: bilingual(format!("F_Ed={:.2} kN, R_w,Rd={:.2} kN (category {}).", c.support_reaction / 1e3, rd / 1e3, c.crippling_category), format!("F_Ed={:.2} kN, R_w,Rd={:.2} kN (Kategorie {}).", c.support_reaction / 1e3, rd / 1e3, c.crippling_category)),
            ndps: &["ndp.en1993.v-256.1"],
            gammas: &[AnnexParams::GAMMA_M1],
        };
        emit(report, annex, case, || {
            solved([grow(t, None, |x| r_rd(x, c.support_length * 1e3) >= c.support_reaction).map(|th| lift(subject("thickness", "Thickness", "Dicke"), length_m(c.thickness), length_m(th / 1e3), &format!("Increase the thickness to at least {th:.2} mm."), &format!("Dicke auf mindestens {th:.2} mm erhöhen.")))])
        });
        if c.moment_resistance > 0.0 {
            let u = cold_formed::bending_crippling_utilisation(c.bending_moment, c.moment_resistance, c.support_reaction, rd);
            let case = Case {
                id: format!("en1993.1-3.6.1.11.{}", c.id),
                part: "DIN EN 1993-1-3",
                family: "EN 1993-1-3",
                chapter: "6",
                section: "6.1.11",
                subject: subject("bendingMoment", "Bending moment", "Biegemoment"),
                title: title("Bending with web crippling", "Biegung mit Stegkrüppeln"),
                demand: dimensionless(u),
                capacity: dimensionless(1.0),
                explanation: bilingual(format!("M_Ed/M_c,Rd + F_Ed/R_w,Rd = {:.3} ≤ 1.25.", u * 1.25), format!("M_Ed/M_c,Rd + F_Ed/R_w,Rd = {:.3} ≤ 1,25.", u * 1.25)),
                ndps: &["ndp.en1993.v-257.1"],
                gammas: &[AnnexParams::GAMMA_M1],
            };
            emit(report, annex, case, || {
                solved([grow(t, None, |x| cold_formed::bending_crippling_utilisation(c.bending_moment, c.moment_resistance, c.support_reaction, r_rd(x, c.support_length * 1e3)) <= 1.0).map(|th| lift(subject("thickness", "Thickness", "Dicke"), length_m(c.thickness), length_m(th / 1e3), &format!("Increase the thickness to at least {th:.2} mm."), &format!("Dicke auf mindestens {th:.2} mm erhöhen.")))])
            });
        }
    }
    if n_ed.abs() > 0.0 && n_c_rd > 0.0 && c.bending_moment > 0.0 && c.moment_resistance > 0.0 {
        let u = cold_formed::axial_bending_utilisation(n_ed.abs(), n_c_rd, c.bending_moment, c.moment_resistance);
        let case = Case {
            id: format!("en1993.1-3.6.1.9.{}", c.id),
            part: "DIN EN 1993-1-3",
            family: "EN 1993-1-3",
            chapter: "6",
            section: "6.1.9",
            subject: subject("bendingMoment", "Bending moment", "Biegemoment"),
            title: title("Axial compression with bending", "Druck mit Biegung"),
            demand: dimensionless(u),
            capacity: dimensionless(1.0),
            explanation: bilingual(format!("N_Ed/N_c,Rd + M_Ed/M_c,Rd = {:.2}/{:.2} kN + {:.3}/{:.3} kNm = {u:.3} (centroid shift ΔM not carried).", n_ed.abs() / 1e3, n_c_rd / 1e3, c.bending_moment / 1e3, c.moment_resistance / 1e3), format!("N_Ed/N_c,Rd + M_Ed/M_c,Rd = {:.2}/{:.2} kN + {:.3}/{:.3} kNm = {u:.3} (Schwerpunktverschiebung ΔM nicht geführt).", n_ed.abs() / 1e3, n_c_rd / 1e3, c.bending_moment / 1e3, c.moment_resistance / 1e3)),
            ndps: &[],
            gammas: &[AnnexParams::GAMMA_M0],
        };
        emit(report, annex, case, || {
            solved([grow(t, None, |x| cold_formed::axial_bending_utilisation(n_ed.abs(), n_c_rd * x / t, c.bending_moment, c.moment_resistance * x / t) <= 1.0).map(|th| lift(subject("thickness", "Thickness", "Dicke"), length_m(c.thickness), length_m(th / 1e3), &format!("Increase the thickness to at least {th:.2} mm."), &format!("Dicke auf mindestens {th:.2} mm erhöhen.")))])
        });
    }
    if c.sheeting_stiffness > 0.0 && c.required_restraint_stiffness > 0.0 && c.web_height > 0.0 {
        let c_d = |th: f64| cold_formed::c_d_series(c.sheeting_stiffness, cold_formed::c_d_web_bending(e_mpa(), th, hw));
        let provided = c_d(t);
        let case = Case {
            id: format!("en1993.1-3.10.1.5.{}", c.id),
            part: "DIN EN 1993-1-3",
            family: "EN 1993-1-3",
            chapter: "10",
            section: "10.1.5",
            subject: subject("sheetingStiffness", "Sheeting restraint", "Drehbettung"),
            title: title("Rotational restraint of the sheeting", "Drehbettung durch das Trapezblech"),
            demand: force_n(c.required_restraint_stiffness),
            capacity: force_n(provided),
            explanation: bilingual(format!("C_D={provided:.1} N·m/m (C_D,C={:.1}, web bending {:.1}) against C_D,req={:.1}.", c.sheeting_stiffness, cold_formed::c_d_web_bending(e_mpa(), t, hw), c.required_restraint_stiffness), format!("C_D={provided:.1} N·m/m (C_D,C={:.1}, Stegbiegung {:.1}) gegen C_D,req={:.1}.", c.sheeting_stiffness, cold_formed::c_d_web_bending(e_mpa(), t, hw), c.required_restraint_stiffness)),
            ndps: &["ndp.en1993.v-113c.1"],
            gammas: &[],
        };
        emit(report, annex, case, || {
            let th = grow(t, None, |x| c_d(x) >= c.required_restraint_stiffness);
            let web = cold_formed::c_d_web_bending(e_mpa(), t, hw);
            let sheeting = (web > c.required_restraint_stiffness).then(|| (1.0 / (1.0 / c.required_restraint_stiffness - 1.0 / web)) * (1.0 + 1.0e-6));
            solved([
                th.map(|th| lift(subject("thickness", "Thickness", "Dicke"), length_m(c.thickness), length_m(th / 1e3), &format!("Increase the thickness to at least {th:.2} mm."), &format!("Dicke auf mindestens {th:.2} mm erhöhen."))),
                sheeting.map(|sheeting| lift(subject("sheetingStiffness", "Sheeting restraint", "Drehbettung"), force_n(c.sheeting_stiffness), force_n(sheeting), "Or use sheeting with a larger rotational stiffness C_D,C.", "Oder Trapezblech mit größerer Drehbettung C_D,C wählen.")),
            ])
        });
    }
}

/// 🔬 EN 1993-1-4 member interaction and fire reduction of stainless members, EN 1993-1-10 brittle fracture of every member.
pub(super) fn check_member(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, member: &SteelMember, report: &mut CheckReport) {
    let Some(section) = find_section(document, &member.section_id) else { return };
    let Some(material) = find_material(document, &member.material_id) else { return };
    brittle_fracture(annex, report, member, section, material);
    let kind = match member.stainless_forming {
        semio_s_artifact_norm_en1993_model::StainlessForming::ColdFormedOpen => stainless::StainlessMember::ColdFormedOpen,
        semio_s_artifact_norm_en1993_model::StainlessForming::WeldedOpen => stainless::StainlessMember::WeldedOpen,
        semio_s_artifact_norm_en1993_model::StainlessForming::Hollow => stainless::StainlessMember::Hollow,
        _ => return,
    };
    if material.kind != semio_s_artifact_norm_en1993_model::MaterialKind::Stainless {
        return;
    }
    let welded = kind == stainless::StainlessMember::WeldedOpen;
    let (fy, e_mpa, gm1) = (material.fy / 1e6, material.e_modulus / 1e6, params.gamma_m1);
    let (h, b, tw, tf, r) = (section.h * 1e3, section.b * 1e3, section.tw * 1e3, section.tf * 1e3, section.r * 1e3);
    let eps = cold_stainless::stainless_eps(fy, e_mpa);
    let c_out = (b - tw - 2.0 * r) / 2.0;
    let h_in = h - 2.0 * tf - 2.0 * r;
    let rho_o = stainless::rho_outstand_stainless(effective_width::lambda_p(c_out, tf, eps, 0.43), welded);
    let rho_i = stainless::rho_internal_stainless(effective_width::lambda_p(h_in, tw, eps, 4.0), welded);
    let area = section.area * 1e6;
    let a_eff = area - 4.0 * c_out * tf * (1.0 - rho_o) - h_in * tw * (1.0 - rho_i);
    let (n_rk, my_rk, mz_rk) = (a_eff * fy, section.w_el_y * 1e9 * fy, section.w_el_z * 1e9 * fy);
    let (iy, iz) = (part_1_1::radius_of_gyration(section.iy * 1e12, area), part_1_1::radius_of_gyration(section.iz * 1e12, area));
    let (lam_y, lam_z) = (part_1_1::lambda_bar(member.buckling_length_y * 1e3, iy, e_mpa, fy), part_1_1::lambda_bar(member.buckling_length_z * 1e3, iz, e_mpa, fy));
    let (chi_y, chi_z) = (stainless::chi_flexural_stainless(lam_y, kind), stainless::chi_flexural_stainless(lam_z, kind));
    let m_cr = part_1_1::m_cr_rolled_nm(section, material, member.ltb_length.max(member.ltb_restraint_spacing), member.end_moment_ratio_psi, member.load_application.as_str(), member.moment_diagram.as_str());
    let lam_lt = part_1_1::lambda_lt(section.w_el_y, material.fy, m_cr);
    let chi_lt = stainless::chi_lt_stainless(lam_lt, kind);
    let combinations = combine_member_actions(document, &member.id);
    if let Some(gov) = governing_effects(&combinations, "uls") {
        let (n_ed, my_ed, mz_ed) = (gov.action.n.abs(), gov.action.my.abs() * 1e3, gov.action.mz.abs() * 1e3);
        let chi_n = chi_y.min(chi_z);
        let n_y = n_ed / (chi_y * n_rk / gm1);
        let n_z = n_ed / (chi_z * n_rk / gm1);
        let cm_lt = (0.6 + 0.4 * member.end_moment_ratio_psi).max(0.4);
        let ky = if chi_lt < 1.0 { stainless::k_lt_interaction(lam_z, n_z, cm_lt) } else { stainless::k_interaction(lam_y, n_y) };
        let kz = stainless::k_interaction(lam_z, n_z);
        let u = stainless::interaction_stainless(n_ed, chi_n, n_rk, ky, my_ed, chi_lt, my_rk, kz, mz_ed, mz_rk, gm1);
        let sigma = (my_ed / (section.w_el_y * 1e9)).max(1.0);
        let n_ro = if material.ramberg_osgood_n > 0.0 { material.ramberg_osgood_n } else { cold_stainless::stainless_grade(&material.grade).map_or(7.0, |g| g.n) };
        let secant = stainless::deflection_secant_factor(sigma, e_mpa, fy, n_ro);
        let case = Case {
            id: format!("en1993.1-4.5.5.{}", member.id),
            part: "DIN EN 1993-1-4",
            family: "EN 1993-1-4",
            chapter: "5",
            section: "5.5.4",
            subject: subject_member(member, &format!("members[id={}].sectionId", member.id)),
            title: loc("Stainless member interaction", "Nichtrostender Stab Interaktion"),
            demand: dimensionless(u),
            capacity: dimensionless(1.0),
            explanation: bilingual(
                format!("N_Ed={:.1} kN, M_y,Ed={:.1} kNm (gov. {}), χ_y={chi_y:.3}, χ_z={chi_z:.3}, χ_LT={chi_lt:.3}, k_y={ky:.3}, ρ_outstand={rho_o:.3}, ρ_internal={rho_i:.3}, SLS deflection factor E/E_s={secant:.3} (Ramberg-Osgood n={n_ro:.1}).", n_ed / 1e3, my_ed / 1e6, gov.combination_id),
                format!("N_Ed={:.1} kN, M_y,Ed={:.1} kNm (maßgebend {}), χ_y={chi_y:.3}, χ_z={chi_z:.3}, χ_LT={chi_lt:.3}, k_y={ky:.3}, ρ_Outstand={rho_o:.3}, ρ_innen={rho_i:.3}, Durchbiegungsfaktor E/E_s={secant:.3} (Ramberg-Osgood n={n_ro:.1}).", n_ed / 1e3, my_ed / 1e6, gov.combination_id),
            ),
            ndps: if material.ramberg_osgood_n > 0.0 { &["ndp.en1993.v-300.1"] } else { &["ndp.en1993.v-300.1", "ndp.en1993.v-300.2"] },
            gammas: &[AnnexParams::GAMMA_M1],
        };
        emit(report, annex, case, || {
            let options = next_section_options(section, section.area * u.max(1.0), section.w_pl_y * u.max(1.0));
            vec![Remedy::one_of(subject_member(member, &format!("members[id={}].sectionId", member.id)), options.clone(), loc(&format!("Upsize the stainless section: {}.", options.join(", ")), &format!("Edelstahlquerschnitt vergrößern: {}.", options.join(", "))))]
        });
    }

}

/// 🔥 EN 1993-1-2 Table C.1: load ratio of a stainless member against k_0.2p,θ at the fire design temperature.
pub(super) fn check_stainless_fire(document: &En1993Snapshot, annex: AnnexChoice, fire: &FireExposure, report: &mut CheckReport) {
    let Some(member) = document.members.iter().find(|m| m.id == fire.member_id) else { return };
    let Some(material) = find_material(document, &member.material_id) else { return };
    if material.kind != semio_s_artifact_norm_en1993_model::MaterialKind::Stainless {
        return;
    }
    let Some((k02, ke, ku)) = stainless::stainless_fire_factors(&material.grade, fire.design_temperature) else { return };
    let subject = || subj(&fire.id, "fireExposures", "mu0", "Fire load ratio", "Brandlastgrad");
    let case = Case {
        id: format!("en1993.1-4.fire.{}", fire.id),
        part: "DIN EN 1993-1-2",
        family: "EN 1993-1-2",
        chapter: "C",
        section: "C.2",
        subject: subject(),
        title: loc("Stainless steel fire resistance", "Nichtrostender Stahl Brandwiderstand"),
        demand: dimensionless(fire.mu0),
        capacity: dimensionless(k02),
        explanation: bilingual(format!("μ0={:.2} against k_0.2p,θ={k02:.3} at θ={:.0} °C (k_E,θ={ke:.2}, k_u,θ={ku:.2}).", fire.mu0, fire.design_temperature), format!("μ0={:.2} gegen k_0,2p,θ={k02:.3} bei θ={:.0} °C (k_E,θ={ke:.2}, k_u,θ={ku:.2}).", fire.mu0, fire.design_temperature)),
        ndps: &["ndp.en1993.v-305.1"],
        gammas: &[],
    };
    emit(report, annex, case, || vec![cap(subject(), dimensionless(fire.mu0), dimensionless(k02), &format!("Reduce the fire load ratio μ0 to at most {k02:.2}."), &format!("Brandlastgrad μ0 auf höchstens {k02:.2} senken."))]);
}

/// 🌡 Why Table 2.1 of EN 1993-1-10 gives no thickness limit for `grade` and `sub_grade` at `ratio` and `t_ed`, in English and German; the plate and the member check share it.
fn gap_words(gap: toughness::Gap, grade: &str, sub_grade: &str, ratio: f64, t_ed: f64) -> (String, String) {
    match gap {
        toughness::Gap::UnknownColumn => (format!("{grade} {sub_grade} is not listed in Table 2.1; verify the thickness limit manually."), format!("{grade} {sub_grade} steht nicht in Tabelle 2.1; Dickengrenze manuell nachweisen.")),
        toughness::Gap::StressAboveTable => (format!("σ_Ed/f_y={ratio:.2} exceeds the highest stress column (0.75) of Table 2.1; verify the thickness limit manually."), format!("σ_Ed/f_y={ratio:.2} übersteigt die höchste Spannungsspalte (0,75) der Tabelle 2.1; Dickengrenze manuell nachweisen.")),
        toughness::Gap::TemperatureBelowTable => (format!("T_Ed={t_ed:.1} °C is below the range of Table 2.1 (−50 °C); verify the thickness limit manually."), format!("T_Ed={t_ed:.1} °C liegt unter dem Bereich der Tabelle 2.1 (−50 °C); Dickengrenze manuell nachweisen.")),
        toughness::Gap::CellNotTranscribed(cell) => (format!("Table 2.1 cell {cell} of {grade} {sub_grade} at σ_Ed/f_y={ratio:.2} is not transcribed (printed EN 1993-1-10 Table 2.1 needed); verify the thickness limit manually."), format!("Zelle {cell} der Tabelle 2.1 für {grade} {sub_grade} bei σ_Ed/f_y={ratio:.2} ist nicht übertragen (gedruckte EN 1993-1-10 Tabelle 2.1 nötig); Dickengrenze manuell nachweisen.")),
    }
}

/// 🌡 EN 1993-1-10 Table 2.1: permissible thickness at the reference temperature T_Ed of the member, the stress column selected (and interpolated) by σ_Ed/f_y(t).
fn brittle_fracture(annex: AnnexChoice, report: &mut CheckReport, member: &SteelMember, section: &SteelSection, material: &SteelMaterial) {
    let (t, t_ed) = toughness_point(member, section);
    let ratio = member.toughness_stress_ratio;
    let subject = || subject_member(member, &format!("materials[id={}].subgrade", material.id));
    let id = format!("en1993.1-10.t.{}", member.id);
    let explicit = member.design_temperature != 0.0 || member.radiation_shift != 0.0 || member.stress_shift != 0.0 || member.margin_shift != 0.0 || member.strain_rate_shift != 0.0 || member.cold_forming_radius > 0.0 || ratio != 0.75;
    let title = loc("Brittle fracture avoidance", "Vermeidung von Sprödbruch");
    let limit = match toughness::limit(&material.grade, &material.subgrade, ratio, t_ed) {
        Ok(limit) => limit,
        Err(gap) => {
            let (en, de) = gap_words(gap, &material.grade, &material.subgrade, ratio, t_ed);
            report.push(CheckResult::assess(id, "DIN EN 1993-1-10", ClauseId::new("EN 1993-1-10", "2", "2.1"), subject(), title).annex(annex).not_applicable(loc(&en, &de)).build());
            return;
        }
    };
    let basis = limit.levels.iter().map(|l| format!("{l:.2}")).collect::<Vec<_>>().join("/");
    let (note_en, note_de) = if limit.conservative { (", conservative next-higher stress column", ", konservativ nächsthöhere Spannungsspalte") } else { ("", "") };
    let case = Case {
        id,
        part: "DIN EN 1993-1-10",
        family: "EN 1993-1-10",
        chapter: "2",
        section: "2.1",
        subject: subject(),
        title,
        demand: length_m(t / 1e3),
        capacity: length_m(limit.t_max / 1e3),
        explanation: bilingual(
            format!("t={t:.1} mm, t_max={:.1} mm at T_Ed={t_ed:.1} °C and σ_Ed/f_y={ratio:.2} ({} {}, Table 2.1 column {basis} f_y(t){note_en}).", limit.t_max, material.grade, material.subgrade),
            format!("t={t:.1} mm, t_max={:.1} mm bei T_Ed={t_ed:.1} °C und σ_Ed/f_y={ratio:.2} ({} {}, Tabelle 2.1 Spalte {basis} f_y(t){note_de}).", limit.t_max, material.grade, material.subgrade),
        ),
        ndps: if explicit { &["ndp.en1993.l402.1"] } else { &[] },
        gammas: &[],
    };
    emit(report, annex, case, || {
        let options: Vec<String> = toughness::sub_grades(&material.grade).iter().filter(|s| **s != material.subgrade && toughness::max_thickness(&material.grade, s, ratio, t_ed).is_some_and(|m| m >= t)).map(|s| (*s).to_string()).collect();
        if options.is_empty() {
            return vec![Remedy::advisory(subject(), loc("No sub-grade of this grade reaches the required thickness: choose a higher steel grade or reduce the thickness.", "Keine Gütegruppe dieser Stahlsorte erreicht die geforderte Dicke: höhere Stahlsorte wählen oder die Dicke verringern."))];
        }
        vec![Remedy::one_of(subject(), options.clone(), loc(&format!("Choose a tougher sub-grade: {}.", options.join(", ")), &format!("Zähere Gütegruppe wählen: {}.", options.join(", "))))]
    });
}

/// 🧩 EN 1993-1-5: stiffened plate buckling, shear lag, patch loading, M-V interaction, stiffener rules and the reduced stress method.
pub(super) fn check_plated(annex: AnnexChoice, params: AnnexParams, p: &PlatedPanel, report: &mut CheckReport) {
    let (a, b, t, fy, gm1) = (p.a * 1e3, p.b * 1e3, p.thickness * 1e3, p.fy / 1e6, params.gamma_m1);
    let eps = (235.0 / fy).sqrt();
    let subject = |field: &str, en: &str, de: &str| subj(&p.id, "platedPanels", field, en, de);
    let raise_t = |ok: &dyn Fn(f64) -> bool| {
        grow(t, None, |x| ok(x)).map(|th| lift(subject("thickness", "Thickness", "Dicke"), length_m(p.thickness), length_m(th / 1e3), &format!("Increase the plate thickness to at least {th:.1} mm."), &format!("Blechdicke auf mindestens {th:.1} mm erhöhen.")))
    };
    if p.stiffener_inertia > 0.0 && p.stiffener_area > 0.0 && p.compression_stress > 0.0 {
        let res = |th: f64, i_sl: f64| {
            let sigma_cr_p = p.k_sigma * plated_ext::sigma_e(e_mpa(), th, b);
            let sigma_cr_c = PI * PI * e_mpa() * i_sl / (p.stiffener_area * 1e6 * a * a);
            let lam_c = (fy / sigma_cr_c).sqrt();
            let ae = plated_ext::alpha_e(p.stiffener_alpha, (i_sl / (p.stiffener_area * 1e6)).sqrt(), p.stiffener_eccentricity * 1e3);
            let chi_c = plated_ext::chi_column_stiffened(lam_c, ae);
            let rho = effective_width::rho_internal(effective_width::lambda_p(b, th, eps, p.k_sigma), 1.0);
            (plated_ext::rho_c_interpolated(rho, chi_c, sigma_cr_p, sigma_cr_c) * fy / gm1, sigma_cr_p, sigma_cr_c, chi_c)
        };
        let (rd, scp, scc, chi_c) = res(t, p.stiffener_inertia * 1e12);
        let case = Case {
            id: format!("en1993.1-5.4.5.{}", p.id),
            part: "DIN EN 1993-1-5",
            family: "EN 1993-1-5",
            chapter: "4",
            section: "4.5.4",
            subject: subject("stiffenerInertia", "Stiffener", "Steife"),
            title: loc("Stiffened plate buckling", "Beulen des ausgesteiften Blechs"),
            demand: stress_pa(p.compression_stress),
            capacity: stress_pa(rd * 1e6),
            explanation: bilingual(format!("σ_Ed={:.1} MPa, ρ_c f_y/γM1={rd:.1} MPa (σ_cr,p={scp:.1}, σ_cr,c={scc:.1} MPa, χ_c={chi_c:.3}).", p.compression_stress / 1e6), format!("σ_Ed={:.1} MPa, ρ_c f_y/γM1={rd:.1} MPa (σ_cr,p={scp:.1}, σ_cr,c={scc:.1} MPa, χ_c={chi_c:.3}).", p.compression_stress / 1e6)),
            ndps: &["ndp.en1993.v-362.1"],
            gammas: &[AnnexParams::GAMMA_M1],
        };
        emit(report, annex, case, || {
            let i_req = grow(p.stiffener_inertia * 1e12, None, |i| res(t, i).0 >= p.compression_stress / 1e6);
            solved([
                raise_t(&|x| res(x, p.stiffener_inertia * 1e12).0 >= p.compression_stress / 1e6),
                i_req.map(|i_req| lift(subject("stiffenerInertia", "Stiffener", "Steife"), Quantity::new(QuantityKind::Dimensionless, p.stiffener_inertia), Quantity::new(QuantityKind::Dimensionless, i_req / 1e12), &format!("Or increase the stiffener second moment to at least {:.3e} m⁴.", i_req / 1e12), &format!("Oder das Steifenflächenmoment auf mindestens {:.3e} m⁴ erhöhen.", i_req / 1e12))),
            ])
        });
    }
    if p.half_width > 0.0 && p.equivalent_span > 0.0 && p.compression_stress > 0.0 {
        let amp = |b0: f64| {
            let kappa = plated_ext::kappa_shear_lag(b0, p.equivalent_span * 1e3, p.stiffener_area * 1e6, t);
            let beta = if p.hogging == 1 { plated_ext::shear_lag_beta_hogging(kappa) } else { plates_shells::shear_lag_beta(kappa) };
            (plated_ext::a_eff_shear_lag(1.0, beta, kappa), kappa, beta)
        };
        let (beta_ult, kappa, beta) = amp(p.half_width * 1e3);
        let case = Case {
            id: format!("en1993.1-5.3.sl.{}", p.id),
            part: "DIN EN 1993-1-5",
            family: "EN 1993-1-5",
            chapter: "3",
            section: "3.3",
            subject: subject("halfWidth", "Flange half width", "Gurthalbbreite"),
            title: loc("Shear lag at the ultimate limit state", "Schubverzerrung im Grenzzustand der Tragfähigkeit"),
            demand: stress_pa(p.compression_stress / beta_ult),
            capacity: stress_pa(p.fy / gm1),
            explanation: bilingual(format!("κ={kappa:.3}, β={beta:.3}, β_ult={beta_ult:.3}: σ_Ed/β_ult={:.1} MPa against f_y/γM1={:.1} MPa.", p.compression_stress / beta_ult / 1e6, fy / gm1), format!("κ={kappa:.3}, β={beta:.3}, β_ult={beta_ult:.3}: σ_Ed/β_ult={:.1} MPa gegen f_y/γM1={:.1} MPa.", p.compression_stress / beta_ult / 1e6, fy / gm1)),
            ndps: &["ndp.en1993.v-360.1"],
            gammas: &[AnnexParams::GAMMA_M1],
        };
        emit(report, annex, case, || {
            solved([shrink(p.half_width * 1e3, None, |x| p.compression_stress / amp(x).0 <= p.fy / gm1).map(|b0| cap(subject("halfWidth", "Flange half width", "Gurthalbbreite"), length_m(p.half_width), length_m(b0 / 1e3), &format!("Reduce the flange half width b_0 to at most {b0:.0} mm."), &format!("Gurthalbbreite b_0 auf höchstens {b0:.0} mm verringern.")))])
        });
    }
    let kind = match p.patch_type {
        semio_s_artifact_norm_en1993_model::PatchLoadType::Through => Some((plated_ext::PatchType::Through, "a")),
        semio_s_artifact_norm_en1993_model::PatchLoadType::OneFlangeNearEnd => Some((plated_ext::PatchType::OneFlangeNearEnd, "b")),
        semio_s_artifact_norm_en1993_model::PatchLoadType::EndFlange => Some((plated_ext::PatchType::EndFlange, "c")),
        _ => None,
    };
    if let (Some((kind, tag)), true) = (kind, p.patch_force > 0.0 && a > 0.0 && p.flange_thickness > 0.0) {
        let f_rd = |th: f64| {
            let base = plated_ext::patch_resistance(kind, b, th, a, p.flange_thickness * 1e3, p.flange_width * 1e3, fy, p.flange_fy / 1e6, p.patch_length * 1e3, p.patch_offset * 1e3, gm1);
            if p.longitudinal_inertia > 0.0 && kind == plated_ext::PatchType::Through {
                let kf = plated_ext::k_f_longitudinal(b, a, th, p.longitudinal_offset * 1e3, p.longitudinal_inertia * 1e12);
                let f_cr = 0.9 * kf * e_mpa() * th.powi(3) / b;
                let chi = (0.5 / (base.l_y * th * fy / f_cr).sqrt()).min(1.0);
                fy * chi * base.l_y * th / gm1
            } else {
                base.f_rd
            }
        };
        let rd = f_rd(t);
        let case = Case {
            id: format!("en1993.1-5.6.{tag}.{}", p.id),
            part: "DIN EN 1993-1-5",
            family: "EN 1993-1-5",
            chapter: "6",
            section: "6.2",
            subject: subject("patchForce", "Transverse force", "Querkraft"),
            title: loc(&format!("Resistance to transverse forces, type {tag}"), &format!("Beanspruchbarkeit bei Querlasten, Typ {tag}")),
            demand: force_n(p.patch_force),
            capacity: force_n(rd),
            explanation: bilingual(format!("F_Ed={:.1} kN, F_Rd={:.1} kN (type {tag}).", p.patch_force / 1e3, rd / 1e3), format!("F_Ed={:.1} kN, F_Rd={:.1} kN (Typ {tag}).", p.patch_force / 1e3, rd / 1e3)),
            ndps: if tag == "a" { &[] } else { &["ndp.en1993.v-371.1"] },
            gammas: &[AnnexParams::GAMMA_M1],
        };
        emit(report, annex, case, || solved([raise_t(&|x| f_rd(x) >= p.patch_force), Some(lift(subject("patchLength", "Stiff bearing length", "Lasteinleitungslänge"), length_m(p.patch_length), length_m(p.patch_length * 2.0), "Or enlarge the stiff bearing length s_s.", "Oder die Lasteinleitungslänge s_s vergrößern."))]));
        if p.patch_moment_resistance > 0.0 {
            let u = plated_ext::patch_bending_utilisation(p.patch_force, rd, p.patch_moment, p.patch_moment_resistance);
            let case = Case {
                id: format!("en1993.1-5.7.2.{}", p.id),
                part: "DIN EN 1993-1-5",
                family: "EN 1993-1-5",
                chapter: "7",
                section: "7.2",
                subject: subject("patchMoment", "Moment with patch load", "Moment bei Lasteinleitung"),
                title: loc("Transverse force with bending", "Querlast mit Biegung"),
                demand: dimensionless(u),
                capacity: dimensionless(1.0),
                explanation: bilingual(format!("η2 + 0.8 η1 = {:.3} ≤ 1.4.", u * 1.4), format!("η2 + 0,8 η1 = {:.3} ≤ 1,4.", u * 1.4)),
                ndps: &[],
                gammas: &[AnnexParams::GAMMA_M1],
            };
            emit(report, annex, case, || solved([raise_t(&|x| plated_ext::patch_bending_utilisation(p.patch_force, f_rd(x), p.patch_moment, p.patch_moment_resistance) <= 1.0)]));
        }
    }
    if p.shear_force != 0.0 && a > 0.0 && b > 0.0 && t > 0.0 && fy > 0.0 {
        let flange = (p.flange_width > 0.0 && p.flange_thickness > 0.0 && p.flange_fy > 0.0 && p.flange_moment > 0.0).then(|| (p.flange_width * 1e3, p.flange_thickness * 1e3, p.flange_fy / 1e6, p.flange_moment * 1e3, p.bending_moment * 1e3));
        let shear = |th: f64| plated_ext::panel_shear(a, b, th, fy, p.stiffener_inertia * 1e12, flange, gm1);
        let (v_ed, now) = (p.shear_force.abs(), shear(t));
        let case = Case {
            id: format!("en1993.1-5.5.panel.{}", p.id),
            part: "DIN EN 1993-1-5",
            family: "EN 1993-1-5",
            chapter: "5",
            section: "5.2",
            subject: subject("shearForce", "Shear force", "Querkraft"),
            title: loc("Shear buckling resistance of the web panel", "Schubbeulwiderstand des Stegfeldes"),
            demand: force_n(v_ed),
            capacity: force_n(now.v_b_rd),
            explanation: bilingual(
                format!("V_Ed={:.1} kN, V_b,Rd={:.1} kN = V_bw,Rd {:.1} kN + V_bf,Rd {:.1} kN (cap {:.1} kN; a/h_w={:.2}, k_τ={:.2}, λ̄_w={:.3}, χ_w={:.3}, non-rigid end post).", v_ed / 1e3, now.v_b_rd / 1e3, now.v_bw_rd / 1e3, now.v_bf_rd / 1e3, now.v_cap / 1e3, a / b, now.k_tau, now.lambda_w, now.chi_w),
                format!("V_Ed={:.1} kN, V_b,Rd={:.1} kN = V_bw,Rd {:.1} kN + V_bf,Rd {:.1} kN (Grenze {:.1} kN; a/h_w={:.2}, k_τ={:.2}, λ̄_w={:.3}, χ_w={:.3}, nachgiebiges Endauflager).", v_ed / 1e3, now.v_b_rd / 1e3, now.v_bw_rd / 1e3, now.v_bf_rd / 1e3, now.v_cap / 1e3, a / b, now.k_tau, now.lambda_w, now.chi_w),
            ),
            ndps: &[],
            gammas: &[AnnexParams::GAMMA_M1],
        };
        emit(report, annex, case, || {
            let spacing = shrink(a, None, |x| plated_ext::panel_shear(x, b, t, fy, p.stiffener_inertia * 1e12, flange, gm1).v_b_rd >= v_ed);
            let width = shrink(b, None, |x| plated_ext::panel_shear(a, x, t, fy, p.stiffener_inertia * 1e12, flange, gm1).v_b_rd >= v_ed);
            solved([
                raise_t(&|x| shear(x).v_b_rd >= v_ed),
                spacing.map(|a_req| cap(subject("a", "Panel length a (transverse stiffener spacing)", "Feldlänge a (Quersteifenabstand)"), length_m(p.a), length_m(a_req / 1e3), &format!("Or space the transverse stiffeners at most {a_req:.0} mm apart (a/h_w = {:.2}).", a_req / b), &format!("Oder die Quersteifen höchstens {a_req:.0} mm auseinander anordnen (a/h_w = {:.2}).", a_req / b))),
                width.map(|b_req| cap(subject("b", "Panel width b (web height h_w)", "Feldbreite b (Steghöhe h_w)"), length_m(p.b), length_m(b_req / 1e3), &format!("Or reduce the web panel height h_w to at most {b_req:.0} mm (a/h_w = {:.2}).", a / b_req), &format!("Oder die Stegfeldhöhe h_w auf höchstens {b_req:.0} mm verringern (a/h_w = {:.2}).", a / b_req))),
            ])
        });
    }
    if p.plastic_moment > 0.0 && p.shear_force > 0.0 && p.bending_moment > 0.0 {
        let v_bw = |th: f64| {
            let k_tau = plated_ext::k_tau_stiffened(a, b, th, p.stiffener_inertia * 1e12);
            let lam = resistance::lambda_w(b, th, eps, k_tau);
            resistance::v_bw_rd(resistance::chi_w(lam, resistance::eta_shear(fy), false), fy, b, th, gm1)
        };
        let u = plated_ext::mv_interaction(p.bending_moment, p.plastic_moment, p.flange_moment, p.shear_force, v_bw(t));
        let case = Case {
            id: format!("en1993.1-5.7.1.{}", p.id),
            part: "DIN EN 1993-1-5",
            family: "EN 1993-1-5",
            chapter: "7",
            section: "7.1",
            subject: subject("shearForce", "Shear force", "Querkraft"),
            title: loc("Bending and shear interaction", "Interaktion Biegung und Querkraft"),
            demand: dimensionless(u),
            capacity: dimensionless(1.0),
            explanation: bilingual(format!("η1={:.3}, η3={:.3}, interaction {u:.3} (V_bw,Rd={:.1} kN).", p.bending_moment / p.plastic_moment, p.shear_force / v_bw(t), v_bw(t) / 1e3), format!("η1={:.3}, η3={:.3}, Interaktion {u:.3} (V_bw,Rd={:.1} kN).", p.bending_moment / p.plastic_moment, p.shear_force / v_bw(t), v_bw(t) / 1e3)),
            ndps: &["ndp.en1993.v-367.1"],
            gammas: &[AnnexParams::GAMMA_M1],
        };
        emit(report, annex, case, || solved([raise_t(&|x| plated_ext::mv_interaction(p.bending_moment, p.plastic_moment, p.flange_moment, p.shear_force, v_bw(x)) <= 1.0)]));
    }
    if p.transverse_stiffener_inertia > 0.0 && a > 0.0 {
        let ist_min = plated_ext::i_st_min(a, b, t);
        let case = Case {
            id: format!("en1993.1-5.9.3.{}", p.id),
            part: "DIN EN 1993-1-5",
            family: "EN 1993-1-5",
            chapter: "9",
            section: "9.3.3",
            subject: subject("transverseStiffenerInertia", "Transverse stiffener", "Quersteife"),
            title: loc("Minimum transverse stiffener rigidity", "Mindeststeifigkeit der Quersteife"),
            demand: dimensionless(ist_min / 1e12),
            capacity: dimensionless(p.transverse_stiffener_inertia),
            explanation: bilingual(format!("I_st={:.3e} m⁴ against I_st,min={:.3e} m⁴.", p.transverse_stiffener_inertia, ist_min / 1e12), format!("I_st={:.3e} m⁴ gegen I_st,min={:.3e} m⁴.", p.transverse_stiffener_inertia, ist_min / 1e12)),
            ndps: &["ndp.en1993.v-369.1"],
            gammas: &[],
        };
        emit(report, annex, case, || vec![lift(subject("transverseStiffenerInertia", "Transverse stiffener", "Quersteife"), dimensionless(p.transverse_stiffener_inertia), dimensionless(ist_min / 1e12), &format!("Increase I_st to at least {:.3e} m⁴.", ist_min / 1e12), &format!("I_st auf mindestens {:.3e} m⁴ erhöhen.", ist_min / 1e12))]);
        if let Some((k, ip)) = thin_walled_section_checks::stiffener_constants(p) {
            let (it_mm, ip_mm) = (k.it * 1e12, ip * 1e12);
            let u = plated_ext::stiffener_torsion_utilisation(it_mm, ip_mm, fy, e_mpa());
            let case = Case {
                id: format!("en1993.1-5.9.2.{}", p.id),
                part: "DIN EN 1993-1-5",
                family: "EN 1993-1-5",
                chapter: "9",
                section: "9.2.1",
                subject: subject("stiffenerShape", "Stiffener outline", "Steifenkontur"),
                title: loc("Torsional stiffener criterion", "Torsionskriterium der Steife"),
                demand: dimensionless(u),
                capacity: dimensionless(1.0),
                explanation: bilingual(
                    format!("5.3 f_y/E ÷ (I_T/I_p) = {u:.3} with I_T={it_mm:.3e} mm⁴, I_p={ip_mm:.3e} mm⁴ and I_w={:.3e} mm⁶ of the {} outline by the finite-element kernel analyse_section on the solid outline (I_p about the attachment point).", k.iw * 1e18, format!("{:?}", p.stiffener_shape)),
                    format!("5,3 f_y/E ÷ (I_T/I_p) = {u:.3} mit I_T={it_mm:.3e} mm⁴, I_p={ip_mm:.3e} mm⁴ und I_w={:.3e} mm⁶ der Kontur {} nach dem Finite-Elemente-Kern analyse_section an der Vollkontur (I_p um den Anschlusspunkt).", k.iw * 1e18, format!("{:?}", p.stiffener_shape)),
                ),
                ndps: &[],
                gammas: &[],
            };
            emit(report, annex, case, || vec![lift(subject("stiffenerThickness", "Stiffener thickness", "Steifendicke"), length_m(p.stiffener_thickness), length_m(p.stiffener_thickness * 1.5), "Increase the stiffener thickness (I_T grows with t³) or reduce its height.", "Steifendicke erhöhen (I_T wächst mit t³) oder die Steifenhöhe verringern.")]);
        }
    }
    if p.lateral_pressure > 0.0 && p.lateral_pressure_coefficient > 0.0 && t > 0.0 {
        let gm0 = params.gamma_m0;
        let stress = |th: f64| misc::plate_lateral_pressure_stress(p.lateral_pressure_coefficient, p.lateral_pressure / 1e6, b, th);
        let (sigma, limit) = (stress(t), fy / gm0);
        let case = Case {
            id: format!("en1993.1-7.lateral.{}", p.id),
            part: "DIN EN 1993-1-7",
            family: "EN 1993-1-7",
            chapter: "6",
            section: "6.2",
            subject: subject("lateralPressure", "Lateral pressure", "Querdruck"),
            title: loc("Plate under lateral pressure", "Blech unter Querdruck"),
            demand: stress_pa(sigma * 1e6),
            capacity: stress_pa(limit * 1e6),
            explanation: bilingual(
                format!("σ = k p b²/t² = {:.3}·{:.4} MPa·({b:.0} mm)²/({t:.1} mm)² = {sigma:.1} MPa against f_y/γM0 = {limit:.1} MPa (linear-elastic plate bending; k is entered from the Timoshenko or Roark table, the table values are not verified).", p.lateral_pressure_coefficient, p.lateral_pressure / 1e6),
                format!("σ = k p b²/t² = {:.3}·{:.4} MPa·({b:.0} mm)²/({t:.1} mm)² = {sigma:.1} MPa gegen f_y/γM0 = {limit:.1} MPa (linear-elastische Plattenbiegung; k wird aus der Tabelle nach Timoshenko oder Roark eingegeben, die Tabellenwerte sind ungeprüft).", p.lateral_pressure_coefficient, p.lateral_pressure / 1e6),
            ),
            ndps: &[],
            gammas: &[AnnexParams::GAMMA_M0],
        };
        emit(report, annex, case, || solved([raise_t(&|x| stress(x) <= limit), Some(cap(subject("lateralPressure", "Lateral pressure", "Querdruck"), stress_pa(p.lateral_pressure), stress_pa(p.lateral_pressure * limit / sigma), "Reduce the lateral pressure or stiffen the panel (smaller b).", "Querdruck verringern oder das Feld aussteifen (kleineres b)."))]));
    }
    if p.flange_induced_k > 0.0 && p.flange_area > 0.0 && p.flange_fy > 0.0 {
        let aw = b * t;
        let afc = p.flange_area * 1e6;
        let u = plated_ext::flange_induced_buckling_utilisation(b, t, p.flange_induced_k, e_mpa(), p.flange_fy / 1e6, aw, afc);
        let case = Case {
            id: format!("en1993.1-5.8.{}", p.id),
            part: "DIN EN 1993-1-5",
            family: "EN 1993-1-5",
            chapter: "8",
            section: "8",
            subject: subject("flangeArea", "Compression flange", "Druckgurt"),
            title: loc("Flange-induced web buckling", "Gurtinduziertes Stegbeulen"),
            demand: dimensionless(u),
            capacity: dimensionless(1.0),
            explanation: bilingual(format!("h_w/t_w={:.0} against the limit k E/f_yf √(A_w/A_fc) (k={:.2}).", b / t, p.flange_induced_k), format!("h_w/t_w={:.0} gegen die Grenze k E/f_yf √(A_w/A_fc) (k={:.2}).", b / t, p.flange_induced_k)),
            ndps: &[],
            gammas: &[],
        };
        emit(report, annex, case, || {
            let req = afc * u * u;
            vec![lift(subject("flangeArea", "Compression flange", "Druckgurt"), area_m2(p.flange_area), area_m2(req / 1e6), &format!("Increase the compression flange area to at least {:.0} mm².", req), &format!("Druckgurtfläche auf mindestens {:.0} mm² erhöhen.", req))]
        });
    }
    if p.alpha_cr > 0.0 && (p.stress_x > 0.0 || p.stress_z > 0.0 || p.shear_stress > 0.0) {
        let ult = plated_ext::alpha_ult_k(p.stress_x / 1e6, p.stress_z / 1e6, p.shear_stress / 1e6, fy);
        let (lam, rho) = plated_ext::reduced_stress_rho(ult, p.alpha_cr);
        let u = plated_ext::reduced_stress_simple(ult, rho, gm1);
        let case = Case {
            id: format!("en1993.1-5.10.{}", p.id),
            part: "DIN EN 1993-1-5",
            family: "EN 1993-1-5",
            chapter: "10",
            section: "10",
            subject: subject("alphaCr", "Critical load amplifier", "Kritischer Laststeigerungsfaktor"),
            title: loc("Reduced stress method", "Methode der reduzierten Spannungen"),
            demand: dimensionless(u),
            capacity: dimensionless(1.0),
            explanation: bilingual(format!("α_ult,k={ult:.3}, α_cr={:.3}, λ̄_p={lam:.3}, ρ={rho:.3}: γM1/(ρ α_ult,k)={u:.3}.", p.alpha_cr), format!("α_ult,k={ult:.3}, α_cr={:.3}, λ̄_p={lam:.3}, ρ={rho:.3}: γM1/(ρ α_ult,k)={u:.3}.", p.alpha_cr)),
            ndps: &["ndp.en1993.v-372.1"],
            gammas: &[AnnexParams::GAMMA_M1],
        };
        emit(report, annex, case, || vec![lift(subject("alphaCr", "Critical load amplifier", "Kritischer Laststeigerungsfaktor"), dimensionless(p.alpha_cr), dimensionless(p.alpha_cr * u * u), "Increase the critical load amplifier α_cr by thicker plates or closer stiffeners.", "Kritischen Laststeigerungsfaktor α_cr durch dickere Bleche oder engere Steifen erhöhen.")]);
    }
}

/// 🫙 EN 1993-1-6 LS3/LS1 stresses of shells, EN 1993-4-1 Janssen loads, EN 1993-4-2 tank courses and EN 1993-4-3 pipelines.
pub(super) fn check_shell(annex: AnnexChoice, params: AnnexParams, s: &SiloShell, report: &mut CheckReport) {
    let (r, t, fy, gm0, gm1) = (s.radius * 1e3, s.thickness * 1e3, s.fy / 1e6, params.gamma_m0, params.gamma_m1);
    let subject = |field: &str, en: &str, de: &str| subj(&s.id, "siloShells", field, en, de);
    let raise_t = |ok: &dyn Fn(f64) -> bool| {
        grow(t, None, |x| ok(x)).map(|th| lift(subject("thickness", "Wall thickness", "Wanddicke"), length_m(s.thickness), length_m(th / 1e3), &format!("Increase the wall thickness to at least {th:.1} mm."), &format!("Wanddicke auf mindestens {th:.1} mm erhöhen.")))
    };
    let class = s.fabrication_class.as_str().chars().next();
    if let (Some(class), true) = (class, s.length > 0.0) {
        let l = s.length * 1e3;
        let slenderness = shell_stability::thin_shell_ratio(t, r);
        let case = Case {
            id: format!("en1993.1-6.8.thin.{}", s.id),
            part: "DIN EN 1993-1-6",
            family: "EN 1993-1-6",
            chapter: "8",
            section: "8.1",
            subject: subject("thickness", "Wall thickness", "Wanddicke"),
            title: loc("Thin-shell gate t/r < 1/20", "Dünnschalen-Schranke t/r < 1/20"),
            demand: dimensionless(slenderness * shell_stability::THIN_SHELL_LIMIT_INVERSE),
            capacity: dimensionless(1.0),
            explanation: bilingual(format!("t/r = {t:.1}/{r:.0} = {slenderness:.4} against 1/20; the thin-shell checks LS1 to LS4 apply below the limit."), format!("t/r = {t:.1}/{r:.0} = {slenderness:.4} gegen 1/20; die Dünnschalennachweise LS1 bis LS4 gelten unterhalb der Grenze.")),
            ndps: &[],
            gammas: &[],
        };
        emit(report, annex, case, || vec![cap(subject("thickness", "Wall thickness", "Wanddicke"), length_m(s.thickness), length_m(r / shell_stability::THIN_SHELL_LIMIT_INVERSE / 1e3 * 0.999), "Reduce the wall thickness or enlarge the radius until t/r < 1/20, or verify as a thick shell.", "Wanddicke verringern oder Radius vergrößern, bis t/r < 1/20, oder als dicke Schale nachweisen.")]);
        let l = s.length * 1e3;
        let ax = |th: f64| shell_stability::axial_design(l, r, th, fy, class, s.boundary_factor_axial, gm1);
        let hoop = |th: f64| shell_stability::hoop_design(l, r, th, fy, class, s.boundary_factor_hoop, gm1);
        let shear = |th: f64| shell_stability::shear_design(l, r, th, fy, s.boundary_factor_shear, gm1);
        let sx = s.meridional_stress / 1e6;
        if sx > 0.0 {
            let d = ax(t);
            let case = Case {
                id: format!("en1993.1-6.8.5.x.{}", s.id),
                part: "DIN EN 1993-1-6",
                family: "EN 1993-1-6",
                chapter: "8",
                section: "8.5.2",
                subject: subject("meridionalStress", "Meridional stress", "Meridianspannung"),
                title: loc("Meridional buckling (LS3)", "Meridionales Beulen (LS3)"),
                demand: stress_pa(sx * 1e6),
                capacity: stress_pa(d.sigma_rd * 1e6),
                explanation: bilingual(format!("σ_x,Ed={sx:.1} MPa, σ_x,Rd={:.1} MPa (σ_x,Rcr={:.1} MPa, λ̄={:.3}, χ={:.3}, class {class}).", d.sigma_rd, d.sigma_cr, d.lambda, d.chi), format!("σ_x,Ed={sx:.1} MPa, σ_x,Rd={:.1} MPa (σ_x,Rcr={:.1} MPa, λ̄={:.3}, χ={:.3}, Klasse {class}).", d.sigma_rd, d.sigma_cr, d.lambda, d.chi)),
                ndps: &["ndp.en1993.v-800.1"],
                gammas: &[AnnexParams::GAMMA_M1],
            };
            emit(report, annex, case, || solved([raise_t(&|x| ax(x).sigma_rd >= sx)]));
        }
        let st = s.hoop_stress / 1e6;
        if st > 0.0 && s.boundary_factor_hoop > 0.0 {
            let d = hoop(t);
            let case = Case {
                id: format!("en1993.1-6.8.5.theta.{}", s.id),
                part: "DIN EN 1993-1-6",
                family: "EN 1993-1-6",
                chapter: "8",
                section: "8.5.3",
                subject: subject("hoopStress", "Hoop stress", "Ringspannung"),
                title: loc("Circumferential buckling (LS3)", "Umfangsbeulen (LS3)"),
                demand: stress_pa(st * 1e6),
                capacity: stress_pa(d.sigma_rd * 1e6),
                explanation: bilingual(format!("σ_θ,Ed={st:.1} MPa, σ_θ,Rd={:.1} MPa (σ_θ,Rcr={:.1} MPa, χ={:.3}).", d.sigma_rd, d.sigma_cr, d.chi), format!("σ_θ,Ed={st:.1} MPa, σ_θ,Rd={:.1} MPa (σ_θ,Rcr={:.1} MPa, χ={:.3}).", d.sigma_rd, d.sigma_cr, d.chi)),
                ndps: &["ndp.en1993.v-801.1"],
                gammas: &[AnnexParams::GAMMA_M1],
            };
            emit(report, annex, case, || solved([raise_t(&|x| hoop(x).sigma_rd >= st)]));
        }
        let tau = s.shell_shear_stress / 1e6;
        if tau > 0.0 && s.boundary_factor_shear > 0.0 {
            let d = shear(t);
            let case = Case {
                id: format!("en1993.1-6.8.5.tau.{}", s.id),
                part: "DIN EN 1993-1-6",
                family: "EN 1993-1-6",
                chapter: "8",
                section: "8.5.4",
                subject: subject("shellShearStress", "Shear stress", "Schubspannung"),
                title: loc("Shear buckling (LS3)", "Schubbeulen (LS3)"),
                demand: stress_pa(tau * 1e6),
                capacity: stress_pa(d.sigma_rd * 1e6),
                explanation: bilingual(format!("τ_Ed={tau:.1} MPa, τ_Rd={:.1} MPa (τ_Rcr={:.1} MPa).", d.sigma_rd, d.sigma_cr), format!("τ_Ed={tau:.1} MPa, τ_Rd={:.1} MPa (τ_Rcr={:.1} MPa).", d.sigma_rd, d.sigma_cr)),
                ndps: &["ndp.en1993.v-802.1"],
                gammas: &[AnnexParams::GAMMA_M1],
            };
            emit(report, annex, case, || solved([raise_t(&|x| shear(x).sigma_rd >= tau)]));
        }
        if sx > 0.0 && st > 0.0 && s.boundary_factor_hoop > 0.0 {
            let (dx, dt) = (ax(t), hoop(t));
            let (tau_rd, chi_tau, tau_use) = if tau > 0.0 && s.boundary_factor_shear > 0.0 { let d = shear(t); (d.sigma_rd, d.chi, tau) } else { (1.0, 1.0, 0.0) };
            let u = shell_stability::interaction_ls3(sx, dx.sigma_rd, dx.chi, st, dt.sigma_rd, dt.chi, tau_use, tau_rd, chi_tau);
            let case = Case {
                id: format!("en1993.1-6.8.5.int.{}", s.id),
                part: "DIN EN 1993-1-6",
                family: "EN 1993-1-6",
                chapter: "8",
                section: "8.5.5",
                subject: subject("thickness", "Wall thickness", "Wanddicke"),
                title: loc("Buckling interaction (LS3)", "Beulinteraktion (LS3)"),
                demand: dimensionless(u),
                capacity: dimensionless(1.0),
                explanation: bilingual(format!("Interaction of σ_x, σ_θ and τ: {u:.3}."), format!("Interaktion von σ_x, σ_θ und τ: {u:.3}.")),
                ndps: &["ndp.en1993.v-801.2"],
                gammas: &[AnnexParams::GAMMA_M1],
            };
            emit(report, annex, case, || solved([raise_t(&|x| {
                let (a, b) = (ax(x), hoop(x));
                let (tr, ct) = if tau > 0.0 && s.boundary_factor_shear > 0.0 { let d = shear(x); (d.sigma_rd, d.chi) } else { (1.0, 1.0) };
                shell_stability::interaction_ls3(sx, a.sigma_rd, a.chi, st, b.sigma_rd, b.chi, tau_use, tr, ct) <= 1.0
            })]));
        }
        if sx > 0.0 || st > 0.0 || tau > 0.0 {
            let u = shell_stability::ls1_von_mises(sx, st, tau, fy, gm0);
            let case = Case {
                id: format!("en1993.1-6.8.2.vm.{}", s.id),
                part: "DIN EN 1993-1-6",
                family: "EN 1993-1-6",
                chapter: "8",
                section: "8.2",
                subject: subject("thickness", "Wall thickness", "Wanddicke"),
                title: loc("Plastic limit state (LS1) von Mises", "Plastischer Grenzzustand (LS1) von Mises"),
                demand: dimensionless(u),
                capacity: dimensionless(1.0),
                explanation: bilingual(format!("Von Mises utilisation {u:.3} of f_y/γM0={:.1} MPa.", fy / gm0), format!("Von-Mises-Ausnutzung {u:.3} von f_y/γM0={:.1} MPa.", fy / gm0)),
                ndps: &[],
                gammas: &[AnnexParams::GAMMA_M0],
            };
            emit(report, annex, case, || solved([raise_t(&|x| shell_stability::ls1_von_mises(sx * t / x, st * t / x, tau * t / x, fy, gm0) <= 1.0)]));
        }
    }
    if s.wall_friction > 0.0 && s.plan_area > 0.0 && s.plan_perimeter > 0.0 {
        let (gamma, z, area, perimeter) = (s.gamma * 1e-9, s.depth * 1e3, s.plan_area * 1e6, s.plan_perimeter * 1e3);
        let ph = containment::janssen_ph(gamma, s.k, s.wall_friction, area, perimeter, z);
        let pw = containment::janssen_pw(s.wall_friction, ph);
        let pv = containment::janssen_pv(gamma, s.k, s.wall_friction, area, perimeter, z);
        let nx = containment::janssen_nx(gamma, s.k, s.wall_friction, area, perimeter, z);
        let hoop = |th: f64| containment::hoop_membrane(ph, r, th);
        let case = Case {
            id: format!("en1993.4-1.5.2.ph.{}", s.id),
            part: "DIN EN 1993-4-1",
            family: "EN 1993-4-1",
            chapter: "5",
            section: "5.3",
            subject: subject("thickness", "Wall thickness", "Wanddicke"),
            title: loc("Silo wall hoop tension (Janssen)", "Silowand Ringzug (Janssen)"),
            demand: stress_pa(hoop(t) * 1e6),
            capacity: stress_pa(fy / gm0 * 1e6),
            explanation: bilingual(format!("p_h={:.1} kPa, p_w={:.1} kPa, p_v={:.1} kPa, n_x={nx:.2} N/mm at z={:.1} m; σ_θ={:.1} MPa.", ph * 1e3, pw * 1e3, pv * 1e3, s.depth, hoop(t)), format!("p_h={:.1} kPa, p_w={:.1} kPa, p_v={:.1} kPa, n_x={nx:.2} N/mm bei z={:.1} m; σ_θ={:.1} MPa.", ph * 1e3, pw * 1e3, pv * 1e3, s.depth, hoop(t))),
            ndps: &["ndp.en1993.v-803.1"],
            gammas: &[AnnexParams::GAMMA_M0],
        };
        emit(report, annex, case, || solved([raise_t(&|x| hoop(x) <= fy / gm0)]));
        let axial_rd = |th: f64| class.map_or(fy / gm0, |c| shell_stability::axial_design(if s.length > 0.0 { s.length * 1e3 } else { s.depth * 1e3 }, r, th, fy, c, s.boundary_factor_axial, gm1).sigma_rd);
        let case = Case {
            id: format!("en1993.4-1.5.2.nx.{}", s.id),
            part: "DIN EN 1993-4-1",
            family: "EN 1993-4-1",
            chapter: "5",
            section: "5.3",
            subject: subject("thickness", "Wall thickness", "Wanddicke"),
            title: loc("Silo wall meridional compression (Janssen)", "Silowand Meridiandruck (Janssen)"),
            demand: stress_pa(nx / t * 1e6),
            capacity: stress_pa(axial_rd(t) * 1e6),
            explanation: bilingual(format!("n_x/t={:.1} MPa against {:.1} MPa.", nx / t, axial_rd(t)), format!("n_x/t={:.1} MPa gegen {:.1} MPa.", nx / t, axial_rd(t))),
            ndps: &["ndp.en1993.v-803.2"],
            gammas: &[AnnexParams::GAMMA_M0, AnnexParams::GAMMA_M1],
        };
        emit(report, annex, case, || solved([raise_t(&|x| nx / x <= axial_rd(x))]));
    }
    if s.liquid_density > 0.0 && s.liquid_height > 0.0 {
        let t_req = containment::tank_course_thickness(s.liquid_density, s.liquid_height * 1e3, s.course_elevation * 1e3, r, fy, gm0);
        let case = Case {
            id: format!("en1993.4-2.6.{}", s.id),
            part: "DIN EN 1993-4-2",
            family: "EN 1993-4-2",
            chapter: "6",
            section: "6.3",
            subject: subject("thickness", "Course thickness", "Schussdicke"),
            title: loc("Tank course thickness", "Tankschussdicke"),
            demand: length_m(t_req / 1e3),
            capacity: length_m(s.thickness),
            explanation: bilingual(format!("t_req={t_req:.2} mm for ρ={:.0} kg/m³, H={:.1} m at h={:.1} m (hydrostatic membrane).", s.liquid_density, s.liquid_height, s.course_elevation), format!("t_erf={t_req:.2} mm für ρ={:.0} kg/m³, H={:.1} m bei h={:.1} m (hydrostatische Membran).", s.liquid_density, s.liquid_height, s.course_elevation)),
            ndps: &["ndp.en1993.v-804.1"],
            gammas: &[AnnexParams::GAMMA_M0],
        };
        emit(report, annex, case, || vec![lift(subject("thickness", "Course thickness", "Schussdicke"), length_m(s.thickness), length_m(t_req / 1e3), &format!("Increase the course thickness to at least {t_req:.2} mm."), &format!("Schussdicke auf mindestens {t_req:.2} mm erhöhen."))]);
    }
    if s.internal_pressure > 0.0 || s.external_pressure > 0.0 {
        let d = 2.0 * r;
        if s.internal_pressure > 0.0 {
            let p = s.internal_pressure / 1e6;
            let t_req = containment::pipe_wall_thickness(p, d, fy, gm0);
            let case = Case {
                id: format!("en1993.4-3.5.t.{}", s.id),
                part: "DIN EN 1993-4-3",
                family: "EN 1993-4-3",
                chapter: "5",
                section: "5.2",
                subject: subject("thickness", "Wall thickness", "Wanddicke"),
                title: loc("Pipe wall thickness", "Rohrwanddicke"),
                demand: length_m(t_req / 1e3),
                capacity: length_m(s.thickness),
                explanation: bilingual(format!("t_req={t_req:.2} mm for p={p:.2} MPa and d={d:.0} mm."), format!("t_erf={t_req:.2} mm für p={p:.2} MPa und d={d:.0} mm.")),
                ndps: &["ndp.en1993.v-905.1"],
                gammas: &[AnnexParams::GAMMA_M0],
            };
            emit(report, annex, case, || vec![lift(subject("thickness", "Wall thickness", "Wanddicke"), length_m(s.thickness), length_m(t_req / 1e3), &format!("Increase the wall thickness to at least {t_req:.2} mm."), &format!("Wanddicke auf mindestens {t_req:.2} mm erhöhen."))]);
            let vm = |th: f64| {
                let st = containment::hoop_membrane(p, r, th);
                let sx = containment::restrained_axial_stress(nu(), st, e_mpa(), alpha_t(), s.temperature_change);
                containment::pipe_von_mises(sx, st, fy, gm0)
            };
            let case = Case {
                id: format!("en1993.4-3.5.vm.{}", s.id),
                part: "DIN EN 1993-4-3",
                family: "EN 1993-4-3",
                chapter: "5",
                section: "5.3",
                subject: subject("thickness", "Wall thickness", "Wanddicke"),
                title: loc("Restrained pipe biaxial stress", "Eingespanntes Rohr zweiachsige Spannung"),
                demand: dimensionless(vm(t)),
                capacity: dimensionless(1.0),
                explanation: bilingual(format!("Von Mises utilisation {:.3} (ΔT={:.0} K, ν=0.3, α=12·10⁻⁶ 1/K).", vm(t), s.temperature_change), format!("Von-Mises-Ausnutzung {:.3} (ΔT={:.0} K, ν=0,3, α=12·10⁻⁶ 1/K).", vm(t), s.temperature_change)),
                ndps: &["ndp.en1993.v-905.2"],
                gammas: &[AnnexParams::GAMMA_M0],
            };
            emit(report, annex, case, || solved([raise_t(&|x| vm(x) <= 1.0)]));
        }
        if s.external_pressure > 0.0 {
            let p_cr = containment::pipe_p_cr(e_mpa(), nu(), t, d);
            let case = Case {
                id: format!("en1993.4-3.5.pcr.{}", s.id),
                part: "DIN EN 1993-4-3",
                family: "EN 1993-4-3",
                chapter: "5",
                section: "5.4",
                subject: subject("externalPressure", "External pressure", "Außendruck"),
                title: loc("Pipe collapse under external pressure", "Rohrbeulen unter Außendruck"),
                demand: stress_pa(s.external_pressure),
                capacity: stress_pa(p_cr * 1e6),
                explanation: bilingual(format!("p_e={:.3} MPa against p_cr={p_cr:.3} MPa.", s.external_pressure / 1e6), format!("p_e={:.3} MPa gegen p_cr={p_cr:.3} MPa.", s.external_pressure / 1e6)),
                ndps: &["ndp.en1993.v-905.3"],
                gammas: &[],
            };
            emit(report, annex, case, || solved([raise_t(&|x| containment::pipe_p_cr(e_mpa(), nu(), x, d) >= s.external_pressure / 1e6)]));
        }
    }
}

struct EndPlate {
    m_j_rd: f64,
    s_ini: f64,
    z: f64,
    forces: Vec<f64>,
    mode: u8,
}

/// 🔗 EN 1993-1-8 §6 component method of the end plate joint: T-stubs, column web components, row distribution, M_j,Rd and S_j,ini (mm, N, MPa) for the given rows.
fn end_plate(j: &SteelJoint, rows: &[RowGeom], tp: f64, params: AnnexParams) -> EndPlate {
    let result = super::joint_row_checks::component_result(j, rows, tp, params);
    let mode = result.rows.iter().map(|r| r.mode_column.max(r.mode_plate)).max().unwrap_or(1);
    EndPlate { m_j_rd: result.m_j_rd, s_ini: result.s_ini, z: result.z, forces: result.rows.iter().map(|r| r.f_rd).collect(), mode }
}

/// 🔩 Base plate moment resistance by plastic equilibrium of the compression and tension resistances of both sides (mm, N, MPa) with the flange T-stub footprint `(t_fc + 2c) × (b_fc + 2c)` limited by the plate (EN 1993-1-8 6.2.5(3)); `None` outside the resistances.
fn base_plate(j: &SteelJoint, bfc: f64, tp: f64, params: AnnexParams) -> (Option<f64>, f64, f64, f64) {
    let (gm0, gm2) = (params.gamma_m0, params.gamma_m2);
    let (fyp, bp, l_p, tfc) = (j.plate_fy / 1e6, j.plate_width * 1e3, j.base_plate_length * 1e3, j.column_flange_thickness * 1e3);
    let (a0, a1, fcd) = (j.concrete_area_a0 * 1e6, j.concrete_area_a1 * 1e6, j.concrete_fcd / 1e6);
    let jc::CStrip { b_eff, l_eff, f_jd: fjd, fc_rd: fc, .. } = jc::base_plate_c_strip(tfc, bfc, tp, fyp, gm0, l_p, bp, j.joint_coefficient, fcd, &|b, l| ((b * l).min(a0), a1));
    let (m, e) = (j.bolt_to_web * 1e3, j.edge_distance * 1e3);
    let as_ = j.bolt_stress_area * 1e6;
    let leff = jc::l_eff_column_flange_single(m, e);
    let sum_ft = f64::from(j.bolts_per_row) * 0.9 * joints::bolt_fub(&j.bolt_class) * as_ / gm2;
    let stub = jc::t_stub_modes(leff.l1(), leff.l2(), tp, fyp, m, jc::n_prying(e, m), 0.0, sum_ft, as_, f64::from(j.bolts_per_row), j.anchor_length * 1e3, Mode1Method::Two, gm0);
    let ft = stub.ft_rd.min(sum_ft);
    let m_rd = jc::base_plate_m_rd(j.base_normal_force, ft, fc, ft, fc, j.base_lever_left * 1e3, j.base_lever_right * 1e3);
    let k13 = jc::k13_concrete(j.concrete_modulus / 1e6, b_eff, l_eff, e_mpa());
    let k15 = jc::k15_base_plate(leff.l1(), tp, m);
    let k16 = jc::k16_anchor(j.anchor_area * 1e6, j.anchor_length * 1e3);
    let z = (j.base_lever_left + j.base_lever_right) * 1e3;
    (m_rd, jc::s_j_ini_base(e_mpa(), z, 1.0, &[k13, k15, k16]), fjd, fc)
}

/// 🔗 EN 1993-1-8 end plate joints (M_j,Rd, stiffness class) and base plates (M_Rd).
pub(super) fn check_joint(annex: AnnexChoice, params: AnnexParams, j: &SteelJoint, flange_width: Option<f64>, rows: &[RowGeom], report: &mut CheckReport) {
    let subject = |field: &str, en: &str, de: &str| subj(&j.id, "joints", field, en, de);
    if j.kind == semio_s_artifact_norm_en1993_model::JointKind::EndPlate && j.plate_thickness > 0.0 && j.column_flange_thickness > 0.0 && j.compression_lever > 0.0 {
        let tp = j.plate_thickness * 1e3;
        let ep = end_plate(j, rows, tp, params);
        let m_ed = j.moment_demand;
        let m_rd = ep.m_j_rd / 1e3;
        let strength = jc::classify_strength_full(m_rd, j.beam_moment_resistance, j.column_moment_resistance, false);
        let mu = jc::mu_stiffness(m_ed, m_rd, 2.7);
        let case = Case {
            id: format!("en1993.1-8.6.3.mj.{}", j.id),
            part: "DIN EN 1993-1-8",
            family: "EN 1993-1-8",
            chapter: "6",
            section: "6.2.7",
            subject: subject("plateThickness", "End plate thickness", "Stirnplattendicke"),
            title: loc("End plate joint moment resistance", "Momententragfähigkeit des Stirnplattenanschlusses"),
            demand: moment_nm(m_ed),
            capacity: moment_nm(m_rd),
            explanation: bilingual(
                format!("M_j,Ed={:.1} kNm, M_j,Rd={:.1} kNm (row forces {} kN, governing mode {}), {strength} strength, S_j,ini={:.1} MNm/rad, μ={mu:.2}.", m_ed / 1e3, m_rd / 1e3, ep.forces.iter().map(|f| format!("{:.0}", f / 1e3)).collect::<Vec<_>>().join("/"), ep.mode, ep.s_ini / 1e9),
                format!("M_j,Ed={:.1} kNm, M_j,Rd={:.1} kNm (Zeilenkräfte {} kN, maßgebender Modus {}), {strength} Tragfähigkeit, S_j,ini={:.1} MNm/rad, μ={mu:.2}.", m_ed / 1e3, m_rd / 1e3, ep.forces.iter().map(|f| format!("{:.0}", f / 1e3)).collect::<Vec<_>>().join("/"), ep.mode, ep.s_ini / 1e9),
            ),
            ndps: &["ndp.en1993.v-420.1"],
            gammas: &[AnnexParams::GAMMA_M0, AnnexParams::GAMMA_M1, AnnexParams::GAMMA_M2],
        };
        emit(report, annex, case, || {
            if let Some(t_req) = grow(tp, Some(60.0), |x| end_plate(j, rows, x, params).m_j_rd / 1e3 >= m_ed) {
                vec![lift(subject("plateThickness", "End plate thickness", "Stirnplattendicke"), length_m(j.plate_thickness), length_m(t_req / 1e3), &format!("Increase the end plate thickness to at least {t_req:.0} mm."), &format!("Stirnplattendicke auf mindestens {t_req:.0} mm erhöhen."))]
            } else {
                vec![cap(subject("momentDemand", "Joint moment", "Anschlussmoment"), moment_nm(m_ed), moment_nm(m_rd), &format!("Reduce the joint moment to at most {:.1} kNm or stiffen the column web.", m_rd / 1e3), &format!("Anschlussmoment auf höchstens {:.1} kNm senken oder Stützensteg aussteifen.", m_rd / 1e3))]
            }
        });
        if j.assumed_class != semio_s_artifact_norm_en1993_model::StiffnessClass::Unset && j.beam_inertia > 0.0 && j.beam_span > 0.0 {
            let (ib, lb) = (j.beam_inertia * 1e12, j.beam_span * 1e3);
            let kb = if j.braced_frame == 1 { 8.0 } else { 25.0 };
            let (upper, lower) = (kb * e_mpa() * ib / lb, 0.5 * e_mpa() * ib / lb);
            let class = jc::classify_stiffness_frame(ep.s_ini, e_mpa(), ib, lb, j.braced_frame == 1, j.stiffness_ratio);
            let u = match j.assumed_class {
                semio_s_artifact_norm_en1993_model::StiffnessClass::Rigid => upper / ep.s_ini,
                semio_s_artifact_norm_en1993_model::StiffnessClass::Pinned => ep.s_ini / lower,
                _ => (lower / ep.s_ini).max(ep.s_ini / upper),
            };
            let (name, token) = match class { "rigid" => ("rigid", "Rigid"), "nominally-pinned" => ("pinned", "Pinned"), _ => ("semi-rigid", "SemiRigid") };
            let case = Case {
                id: format!("en1993.1-8.5.2.sj.{}", j.id),
                part: "DIN EN 1993-1-8",
                family: "EN 1993-1-8",
                chapter: "5",
                section: "5.2.2",
                subject: subject("assumedClass", "Assumed stiffness class", "Angenommene Steifigkeitsklasse"),
                title: loc("Joint stiffness classification", "Steifigkeitsklassifizierung des Anschlusses"),
                demand: dimensionless(u),
                capacity: dimensionless(1.0),
                explanation: bilingual(format!("S_j,ini={:.1} MNm/rad is {class} (rigid ≥ {:.1}, pinned ≤ {:.1} MNm/rad); z={:.0} mm.", ep.s_ini / 1e9, upper / 1e9, lower / 1e9, ep.z), format!("S_j,ini={:.1} MNm/rad ist {class} (starr ≥ {:.1}, gelenkig ≤ {:.1} MNm/rad); z={:.0} mm.", ep.s_ini / 1e9, upper / 1e9, lower / 1e9, ep.z)),
                ndps: &["ndp.en1993.v-431.2"],
                gammas: &[],
            };
            emit(report, annex, case, || {
                let mut out = vec![Remedy::one_of(subject("assumedClass", "Assumed stiffness class", "Angenommene Steifigkeitsklasse"), vec![token.to_string()], loc(&format!("Assume the {name} behaviour in the global analysis."), &format!("Im Gesamttragwerk das Verhalten {name} annehmen.")))];
                if j.assumed_class == semio_s_artifact_norm_en1993_model::StiffnessClass::Rigid {
                    if let Some(t_req) = grow(tp, Some(60.0), |x| end_plate(j, rows, x, params).s_ini >= upper) {
                        out.push(lift(subject("plateThickness", "End plate thickness", "Stirnplattendicke"), length_m(j.plate_thickness), length_m(t_req / 1e3), &format!("Or increase the end plate thickness to {t_req:.0} mm."), &format!("Oder Stirnplattendicke auf {t_req:.0} mm erhöhen.")));
                    }
                }
                out
            });
        }
    }
    if let Some(bfc) = flange_width.map(|b| b * 1e3).filter(|_| j.kind == semio_s_artifact_norm_en1993_model::JointKind::BasePlate && j.plate_thickness > 0.0 && j.base_plate_length > 0.0 && j.concrete_area_a0 > 0.0) {
        let tp = j.plate_thickness * 1e3;
        let (m_rd, s_ini, fjd, fc) = base_plate(j, bfc, tp, params);
        let capacity = m_rd.map_or(0.0, |m| m / 1e3);
        let case = Case {
            id: format!("en1993.1-8.6.2.base.{}", j.id),
            part: "DIN EN 1993-1-8",
            family: "EN 1993-1-8",
            chapter: "6",
            section: "6.2.8",
            subject: subject("plateThickness", "Base plate thickness", "Fußplattendicke"),
            title: loc("Column base moment resistance", "Momententragfähigkeit des Stützenfußes"),
            demand: moment_nm(j.base_moment.abs()),
            capacity: moment_nm(capacity),
            explanation: bilingual(
                m_rd.map_or_else(|| format!("N_Ed={:.1} kN lies outside the base plate resistances.", j.base_normal_force / 1e3), |_| format!("M_Ed={:.1} kNm, M_Rd={:.1} kNm (f_jd={fjd:.1} MPa, F_C,Rd={:.0} kN, S_j,ini={:.1} MNm/rad).", j.base_moment / 1e3, capacity / 1e3, fc / 1e3, s_ini / 1e9)),
                m_rd.map_or_else(|| format!("N_Ed={:.1} kN liegt außerhalb der Fußplattenwiderstände.", j.base_normal_force / 1e3), |_| format!("M_Ed={:.1} kNm, M_Rd={:.1} kNm (f_jd={fjd:.1} MPa, F_C,Rd={:.0} kN, S_j,ini={:.1} MNm/rad).", j.base_moment / 1e3, capacity / 1e3, fc / 1e3, s_ini / 1e9)),
            ),
            ndps: &["ndp.en1993.v-431.1"],
            gammas: &[AnnexParams::GAMMA_M0, AnnexParams::GAMMA_M2],
        };
        emit(report, annex, case, || {
            solved([grow(tp, Some(80.0), |x| base_plate(j, bfc, x, params).0.is_some_and(|m| m / 1e3 >= j.base_moment.abs())).map(|t_req| lift(subject("plateThickness", "Base plate thickness", "Fußplattendicke"), length_m(j.plate_thickness), length_m(t_req / 1e3), &format!("Increase the base plate thickness to at least {t_req:.0} mm."), &format!("Fußplattendicke auf mindestens {t_req:.0} mm erhöhen.")))])
        });
    }
}

/// 🪢 EN 1993-1-11 cable, threaded rod and EN 1993-1-8 pin checks.
pub(super) fn check_tension(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, c: &TensionComponent, report: &mut CheckReport) {
    let (n_ed, gov) = governing_force(document, &c.actions);
    let subject = |field: &str, en: &str, de: &str| subj(&c.id, "tensionComponents", field, en, de);
    if c.partial_factor > 0.0 {
        let u = tension_components::cable_utilisation(n_ed, c.f_uk, c.partial_factor);
        let sag = if c.cable_horizontal_force > 0.0 { tension_components::cable_sag(c.cable_weight / 1e3, c.cable_length * 1e3, c.cable_horizontal_force) } else { 0.0 };
        let e_eff = if c.cable_stress > 0.0 { tension_components::ernst_modulus(c.cable_modulus / 1e6, c.cable_unit_weight * 1e-9, c.cable_length * 1e3, c.cable_stress / 1e6) } else { c.cable_modulus / 1e6 };
        let case = Case {
            id: format!("en1993.1-11.6.2.cable.{}", c.id),
            part: "DIN EN 1993-1-11",
            family: "EN 1993-1-11",
            chapter: "6",
            section: "6.2",
            subject: subject("fUk", "Breaking strength", "Bruchkraft"),
            title: loc("Cable resistance", "Seiltragfähigkeit"),
            demand: dimensionless(u),
            capacity: dimensionless(1.0),
            explanation: bilingual(format!("F_Ed={:.1} kN (gov. {gov}), F_Rd=F_uk/γ_R={:.1} kN; sag {:.1} mm, E_eff={:.0} GPa (Ernst).", n_ed / 1e3, c.f_uk / c.partial_factor / 1e3, sag, e_eff / 1e3), format!("F_Ed={:.1} kN (maßgebend {gov}), F_Rd=F_uk/γ_R={:.1} kN; Durchhang {:.1} mm, E_eff={:.0} GPa (Ernst).", n_ed / 1e3, c.f_uk / c.partial_factor / 1e3, sag, e_eff / 1e3)),
            ndps: &["ndp.en1993.v-860.1"],
            gammas: &[],
        };
        emit(report, annex, case, || vec![lift(subject("fUk", "Breaking strength", "Bruchkraft"), force_n(c.f_uk), force_n(n_ed * c.partial_factor), &format!("Increase the cable breaking force F_uk to at least {:.1} kN.", n_ed * c.partial_factor / 1e3), &format!("Seilbruchkraft F_uk auf mindestens {:.1} kN erhöhen.", n_ed * c.partial_factor / 1e3))]);
    }
    if c.thread_factor > 0.0 && c.rod_stress_area > 0.0 {
        let rd = tension_components::rod_tension_rd(c.thread_factor, c.rod_stress_area * 1e6, c.rod_fu / 1e6, params.gamma_m2);
        let case = Case {
            id: format!("en1993.1-8.3.6.rod.{}", c.id),
            part: "DIN EN 1993-1-8",
            family: "EN 1993-1-8",
            chapter: "3",
            section: "3.6.1",
            subject: subject("rodStressArea", "Rod stress area", "Stangenspannungsquerschnitt"),
            title: loc("Threaded rod tension", "Gewindestange Zug"),
            demand: force_n(n_ed),
            capacity: force_n(rd),
            explanation: bilingual(format!("F_Ed={:.1} kN, N_t,Rd=k_t A_s f_u/γM2={:.1} kN.", n_ed / 1e3, rd / 1e3), format!("F_Ed={:.1} kN, N_t,Rd=k_t A_s f_u/γM2={:.1} kN.", n_ed / 1e3, rd / 1e3)),
            ndps: &[],
            gammas: &[AnnexParams::GAMMA_M2],
        };
        emit(report, annex, case, || {
            let a_req = c.rod_stress_area * n_ed / rd;
            vec![lift(subject("rodStressArea", "Rod stress area", "Stangenspannungsquerschnitt"), area_m2(c.rod_stress_area), area_m2(a_req), &format!("Increase the stress area to at least {:.0} mm².", a_req * 1e6), &format!("Spannungsquerschnitt auf mindestens {:.0} mm² erhöhen.", a_req * 1e6))]
        });
    }
    if c.pin_diameter > 0.0 {
        let (d, t, fy) = (c.pin_diameter * 1e3, c.pin_plate_thickness * 1e3, c.pin_plate_fy / 1e6);
        let (gm0, gm2) = (params.gamma_m0, params.gamma_m2);
        let a = PI * d * d / 4.0;
        let fv_rd = tension_components::pin_shear_rd(a, c.pin_fup / 1e6, gm2);
        let m_ed = tension_components::pin_moment_of_plates(n_ed, c.pin_outer_thickness * 1e3, c.pin_inner_thickness * 1e3, c.pin_gap * 1e3);
        let m_rd = tension_components::pin_moment_rd(PI * d.powi(3) / 32.0, c.pin_fyp / 1e6, gm0);
        let u = tension_components::pin_combined(m_ed, m_rd, n_ed / 2.0, fv_rd);
        let b_rd = tension_components::pin_bearing_rd(t, d, fy, gm0);
        let ser = tension_components::pin_bearing_ser(t, d, fy, 1.0);
        let case = Case {
            id: format!("en1993.1-8.3.13.pin.{}", c.id),
            part: "DIN EN 1993-1-8",
            family: "EN 1993-1-8",
            chapter: "3",
            section: "3.13",
            subject: subject("pinDiameter", "Pin diameter", "Bolzendurchmesser"),
            title: loc("Pin shear and bending", "Bolzen Schub und Biegung"),
            demand: dimensionless(u),
            capacity: dimensionless(1.0),
            explanation: bilingual(format!("M_Ed={:.2} kNm, M_Rd={:.2} kNm, F_v,Ed={:.1} kN, F_v,Rd={:.1} kN; serviceability bearing limit {:.1} kN.", m_ed / 1e6, m_rd / 1e6, n_ed / 2e3, fv_rd / 1e3, ser / 1e3), format!("M_Ed={:.2} kNm, M_Rd={:.2} kNm, F_v,Ed={:.1} kN, F_v,Rd={:.1} kN; Gebrauchstauglichkeits-Lochleibung {:.1} kN.", m_ed / 1e6, m_rd / 1e6, n_ed / 2e3, fv_rd / 1e3, ser / 1e3)),
            ndps: &[],
            gammas: &[AnnexParams::GAMMA_M0, AnnexParams::GAMMA_M2],
        };
        emit(report, annex, case, || {
            let d_req = grow(d, None, |x| {
                let ar = PI * x * x / 4.0;
                tension_components::pin_combined(m_ed, tension_components::pin_moment_rd(PI * x.powi(3) / 32.0, c.pin_fyp / 1e6, gm0), n_ed / 2.0, tension_components::pin_shear_rd(ar, c.pin_fup / 1e6, gm2)) <= 1.0
            });
            solved([d_req.map(|d_req| lift(subject("pinDiameter", "Pin diameter", "Bolzendurchmesser"), length_m(c.pin_diameter), length_m(d_req / 1e3), &format!("Increase the pin diameter to at least {d_req:.0} mm."), &format!("Bolzendurchmesser auf mindestens {d_req:.0} mm erhöhen.")))])
        });
        let case = Case {
            id: format!("en1993.1-8.3.13.pinb.{}", c.id),
            part: "DIN EN 1993-1-8",
            family: "EN 1993-1-8",
            chapter: "3",
            section: "3.13",
            subject: subject("pinPlateThickness", "Pin plate thickness", "Bolzenblechdicke"),
            title: loc("Pin bearing", "Bolzen Lochleibung"),
            demand: force_n(n_ed),
            capacity: force_n(b_rd),
            explanation: bilingual(format!("F_Ed={:.1} kN, F_b,Rd=1.5 t d f_y/γM0={:.1} kN.", n_ed / 1e3, b_rd / 1e3), format!("F_Ed={:.1} kN, F_b,Rd=1,5 t d f_y/γM0={:.1} kN.", n_ed / 1e3, b_rd / 1e3)),
            ndps: &[],
            gammas: &[AnnexParams::GAMMA_M0],
        };
        emit(report, annex, case, || {
            let t_req = t * n_ed / b_rd;
            vec![lift(subject("pinPlateThickness", "Pin plate thickness", "Bolzenblechdicke"), length_m(c.pin_plate_thickness), length_m(t_req / 1e3), &format!("Increase the plate thickness to at least {t_req:.1} mm."), &format!("Blechdicke auf mindestens {t_req:.1} mm erhöhen."))]
        });
    }
}

/// 🌉 EN 1993-2 §9.5 fatigue with λ factors from the traffic data and the orthotropic deck detail ladder.
pub(super) fn check_bridge(annex: AnnexChoice, params: AnnexParams, bf: &BridgeFatigue, report: &mut CheckReport) {
    if !(bf.heavy_vehicle_weight > 0.0 && bf.reference_weight > 0.0) {
        return;
    }
    let subject = |field: &str, en: &str, de: &str| subj(&bf.id, "bridgeFatigue", field, en, de);
    let l2 = bridge_fatigue::lambda_2(bf.heavy_vehicle_weight, bf.reference_weight, bf.observed_traffic, 0.5e6);
    let l3 = bridge_fatigue::lambda_3(bf.design_life);
    let l4 = bridge_fatigue::lambda_4(&[(bf.second_lane_traffic, bf.second_lane_weight)], bf.observed_traffic, bf.heavy_vehicle_weight);
    let raw = bf.lambda_1 * l2 * l3 * l4;
    let lam = if bf.lambda_max > 0.0 { raw.min(bf.lambda_max) } else { raw };
    let delta_e2 = bridge_fatigue::delta_sigma_e2_bridge(bf.lambda_1, l2, l3, l4, if bf.lambda_max > 0.0 { bf.lambda_max } else { f64::INFINITY }, bf.phi2, bf.delta_sigma_p / 1e6);
    let ks = bridge_fatigue::size_effect(bf.plate_thickness * 1e3);
    let detail = bridge_fatigue::orthotropic_detail(&bf.detail_id);
    let delta_c = detail.map_or(f64::from(bf.category), |d| d.min(f64::from(bf.category))) * ks;
    let u = bridge_fatigue::fatigue_utilisation(delta_e2, delta_c, 1.0, params.gamma_mf);
    let case = Case {
        id: format!("en1993.2.9.5.{}", bf.id),
        part: "DIN EN 1993-2",
        family: "EN 1993-2",
        chapter: "9",
        section: "9.5.2",
        subject: subject("category", "Detail category", "Kerbfall"),
        title: loc("Bridge fatigue damage equivalence", "Brückenermüdung Schadensäquivalenz"),
        demand: dimensionless(u),
        capacity: dimensionless(1.0),
        explanation: bilingual(format!("λ1={:.2}, λ2={l2:.3}, λ3={l3:.3}, λ4={l4:.3} → λ={lam:.3}; Δσ_E,2={delta_e2:.1} MPa against Δσ_C/γMf={:.1} MPa (k_s={ks:.2}).", bf.lambda_1, delta_c / params.gamma_mf), format!("λ1={:.2}, λ2={l2:.3}, λ3={l3:.3}, λ4={l4:.3} → λ={lam:.3}; Δσ_E,2={delta_e2:.1} MPa gegen Δσ_C/γMf={:.1} MPa (k_s={ks:.2}).", bf.lambda_1, delta_c / params.gamma_mf)),
        ndps: if detail.is_some() { &["ndp.en1993.v-880.1", "ndp.en1993.v-880.2"] } else { &["ndp.en1993.v-880.1"] },
        gammas: &[AnnexParams::GAMMA_MF],
    };
    emit(report, annex, case, || {
        let needed = delta_c * u / ks;
        let next = bridge_fatigue::category_ladder().into_iter().rev().find(|c| *c >= needed).unwrap_or(160.0);
        vec![lift(subject("category", "Detail category", "Kerbfall"), dimensionless(f64::from(bf.category)), dimensionless(next), &format!("Choose a detail of category {next:.0} or higher."), &format!("Kerbfall {next:.0} oder höher wählen."))]
    });
}

/// 🗼 EN 1993-3-1 lattice slenderness and EN 1993-3-2 vortex shedding and ovalling of masts and chimneys.
pub(super) fn check_tower(annex: AnnexChoice, _params: AnnexParams, tw: &TowerLeg, report: &mut CheckReport) {
    let subject = |field: &str, en: &str, de: &str| subj(&tw.id, "towerLegs", field, en, de);
    let kind = match tw.lattice_kind {
        semio_s_artifact_norm_en1993_model::LatticeKind::Leg => Some((towers_masts::LatticeMember::Leg, 120.0)),
        semio_s_artifact_norm_en1993_model::LatticeKind::Diagonal => Some((towers_masts::LatticeMember::Diagonal, 180.0)),
        semio_s_artifact_norm_en1993_model::LatticeKind::Redundant => Some((towers_masts::LatticeMember::Redundant, 250.0)),
        _ => None,
    };
    if let (Some((kind, limit)), true) = (kind, tw.slenderness > 0.0) {
        let eff = towers_masts::lambda_eff_lattice(kind, tw.slenderness);
        let guy = if tw.guy_area > 0.0 && tw.guy_length > 0.0 { towers_masts::guy_stiffness(160.0e3, tw.guy_area * 1e6, tw.guy_angle.to_radians(), tw.guy_length * 1e3) } else { 0.0 };
        let case = Case {
            id: format!("en1993.3-1.5.lambda.{}", tw.id),
            part: "DIN EN 1993-3-1",
            family: "EN 1993-3-1",
            chapter: "5",
            section: "5.4",
            subject: subject("slenderness", "Slenderness", "Schlankheit"),
            title: loc("Lattice member effective slenderness", "Wirksame Schlankheit des Gitterstabs"),
            demand: dimensionless(eff),
            capacity: dimensionless(limit),
            explanation: bilingual(format!("λ_eff={eff:.1} against {limit:.0} ({}); guy stiffness {:.1} N/mm.", tw.lattice_kind, guy), format!("λ_eff={eff:.1} gegen {limit:.0} ({}); Abspannsteifigkeit {:.1} N/mm.", tw.lattice_kind, guy)),
            ndps: &["ndp.en1993.v-850.1"],
            gammas: &[],
        };
        emit(report, annex, case, || {
            let max = match kind { towers_masts::LatticeMember::Leg => limit / 0.9, towers_masts::LatticeMember::Diagonal => (limit - 0.35) / 0.7, towers_masts::LatticeMember::Redundant => limit / 0.7 };
            vec![cap(subject("slenderness", "Slenderness", "Schlankheit"), dimensionless(tw.slenderness), dimensionless(max), &format!("Reduce the member slenderness to at most {max:.0}."), &format!("Stabschlankheit auf höchstens {max:.0} verringern."))]
        });
    }
    if tw.diameter > 0.0 && tw.strouhal > 0.0 && tw.height > 0.0 && tw.permitted_amplitude_ratio > 0.0 {
        let n1 = towers_masts::cantilever_frequency(tw.height, tw.bending_stiffness, tw.mass_per_length);
        let v_crit = towers_masts::v_crit(tw.diameter, n1, tw.strouhal);
        let sc = towers_masts::scruton(tw.log_decrement, tw.mass_per_length, tw.air_density, tw.diameter);
        let y = towers_masts::y_max_over_b(tw.strouhal, sc, tw.k_factor, tw.kw_factor, tw.lateral_force_coefficient);
        let cycles = towers_masts::vortex_cycles(tw.design_life * 3.15576e7, tw.crossing_rate, v_crit, tw.mean_wind_speed);
        let case = Case {
            id: format!("en1993.3-2.vortex.{}", tw.id),
            part: "DIN EN 1993-3-2",
            family: "EN 1993-3-2",
            chapter: "5",
            section: "5.2",
            subject: subject("logDecrement", "Structural damping", "Strukturdämpfung"),
            title: loc("Vortex-induced across-wind amplitude", "Wirbelerregte Querschwingungsamplitude"),
            demand: dimensionless(y),
            capacity: dimensionless(tw.permitted_amplitude_ratio),
            explanation: bilingual(format!("n1={n1:.2} Hz, v_crit={v_crit:.1} m/s, Sc={sc:.2}, y_max/b={y:.3}, N={cycles:.2e} cycles in {:.0} years.", tw.design_life), format!("n1={n1:.2} Hz, v_crit={v_crit:.1} m/s, Sc={sc:.2}, y_max/b={y:.3}, N={cycles:.2e} Lastwechsel in {:.0} Jahren.", tw.design_life)),
            ndps: &["ndp.en1993.v-851.1"],
            gammas: &[],
        };
        emit(report, annex, case, || {
            let delta = tw.log_decrement * y / tw.permitted_amplitude_ratio;
            vec![lift(subject("logDecrement", "Structural damping", "Strukturdämpfung"), dimensionless(tw.log_decrement), dimensionless(delta), &format!("Raise the structural damping δ_s to at least {delta:.3} (damper)."), &format!("Strukturdämpfung δ_s auf mindestens {delta:.3} erhöhen (Schwingungsdämpfer)."))]
        });
        if tw.shell_thickness > 0.0 && tw.ring_mode >= 2.0 {
            let n_i = towers_masts::ovalling_frequency(tw.ring_mode, tw.diameter / 2.0, tw.shell_thickness, 210.0e9, 7850.0);
            let v_i = towers_masts::ovalling_v_crit(tw.diameter, n_i, tw.strouhal);
            let u = 1.25 * tw.mean_wind_speed / v_i;
            let case = Case {
                id: format!("en1993.3-2.ovalling.{}", tw.id),
                part: "DIN EN 1993-3-2",
                family: "EN 1993-3-2",
                chapter: "5",
                section: "5.3",
                subject: subject("shellThickness", "Shell thickness", "Schalendicke"),
                title: loc("Ovalling of the shell", "Ovalisierung der Schale"),
                demand: dimensionless(u),
                capacity: dimensionless(1.0),
                explanation: bilingual(format!("n_{:.0}={n_i:.2} Hz, v_crit,{:.0}={v_i:.1} m/s against 1.25 v_m={:.1} m/s.", tw.ring_mode, tw.ring_mode, 1.25 * tw.mean_wind_speed), format!("n_{:.0}={n_i:.2} Hz, v_crit,{:.0}={v_i:.1} m/s gegen 1,25 v_m={:.1} m/s.", tw.ring_mode, tw.ring_mode, 1.25 * tw.mean_wind_speed)),
                ndps: &["ndp.en1993.v-851.2"],
                gammas: &[],
            };
            emit(report, annex, case, || {
                let t_req = grow(tw.shell_thickness, None, |x| 1.25 * tw.mean_wind_speed <= towers_masts::ovalling_v_crit(tw.diameter, towers_masts::ovalling_frequency(tw.ring_mode, tw.diameter / 2.0, x, 210.0e9, 7850.0), tw.strouhal));
                solved([t_req.map(|t_req| lift(subject("shellThickness", "Shell thickness", "Schalendicke"), length_m(tw.shell_thickness), length_m(t_req), &format!("Increase the shell thickness to at least {:.1} mm or add ring stiffeners.", t_req * 1e3), &format!("Schalendicke auf mindestens {:.1} mm erhöhen oder Ringsteifen anordnen.", t_req * 1e3)))])
            });
        }
    }
}

/// 🧷 EN 1993-5 sheet piles: corrosion allowance, bending with high shear, shear and interlock shear.
pub(super) fn check_pile(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, pile: &SteelPile, report: &mut CheckReport) {
    let env = match pile.environment {
        semio_s_artifact_norm_en1993_model::SheetPileEnvironment::UndisturbedSoil => sheet_piling::Environment::UndisturbedSoil,
        semio_s_artifact_norm_en1993_model::SheetPileEnvironment::PollutedSoil => sheet_piling::Environment::PollutedSoil,
        semio_s_artifact_norm_en1993_model::SheetPileEnvironment::CompactedFill => sheet_piling::Environment::CompactedFill,
        semio_s_artifact_norm_en1993_model::SheetPileEnvironment::NonCompactedFill => sheet_piling::Environment::NonCompactedFill,
        _ => return,
    };
    let Some(material) = find_material(document, &pile.material_id) else { return };
    if !(pile.section_modulus > 0.0 && pile.wall_thickness > 0.0) {
        return;
    }
    let (fy, gm0) = (material.fy / 1e6, params.gamma_m0);
    let subject = |field: &str, en: &str, de: &str| subj(&pile.id, "piles", field, en, de);
    let resist = |t_gross: f64| {
        let loss = sheet_piling::corrosion_loss(env, pile.design_life);
        let ratio = sheet_piling::net_thickness(t_gross, loss, loss) / t_gross;
        let v_pl = sheet_piling::v_pl_rd_pile(pile.shear_area * 1e6 * ratio, fy, gm0);
        let fy_red = sheet_piling::fy_reduced_by_shear(pile.design_shear, v_pl, fy);
        (sheet_piling::m_c_rd_pile(pile.interlock_bending_factor, pile.section_modulus * 1e9 * ratio, fy_red, gm0), v_pl, loss, sheet_piling::net_thickness(t_gross, loss, loss))
    };
    let t = pile.wall_thickness * 1e3;
    let (m_rd, v_pl, loss, t_net) = resist(t);
    let raise_t = |ok: &dyn Fn(f64) -> bool| {
        grow(t, None, |x| ok(x)).map(|th| lift(subject("wallThickness", "Sheet pile thickness", "Spundwandstärke"), length_m(pile.wall_thickness), length_m(th / 1e3), &format!("Increase the sheet pile thickness to at least {th:.1} mm."), &format!("Spundwandstärke auf mindestens {th:.1} mm erhöhen.")))
    };
    let case = Case {
        id: format!("en1993.5.5.2.m.{}", pile.id),
        part: "DIN EN 1993-5",
        family: "EN 1993-5",
        chapter: "5",
        section: "5.2.2",
        subject: subject("designMoment", "Design moment", "Bemessungsmoment"),
        title: loc("Sheet pile bending resistance", "Spundwand Biegetragfähigkeit"),
        demand: moment_nm(pile.design_moment),
        capacity: moment_nm(m_rd / 1e3),
        explanation: bilingual(format!("M_Ed={:.1} kNm, M_c,Rd={:.1} kNm after corrosion {loss:.2} mm per face (t_net={t_net:.2} mm, β_B={:.2}).", pile.design_moment / 1e3, m_rd / 1e6, pile.interlock_bending_factor), format!("M_Ed={:.1} kNm, M_c,Rd={:.1} kNm nach Korrosion {loss:.2} mm je Seite (t_net={t_net:.2} mm, β_B={:.2}).", pile.design_moment / 1e3, m_rd / 1e6, pile.interlock_bending_factor)),
        ndps: &["ndp.en1993.v-870.1"],
        gammas: &[AnnexParams::GAMMA_M0],
    };
    emit(report, annex, case, || solved([raise_t(&|x| resist(x).0 / 1e3 >= pile.design_moment)]));
    let case = Case {
        id: format!("en1993.5.5.2.v.{}", pile.id),
        part: "DIN EN 1993-5",
        family: "EN 1993-5",
        chapter: "5",
        section: "5.2.2",
        subject: subject("designShear", "Design shear", "Bemessungsquerkraft"),
        title: loc("Sheet pile shear resistance", "Spundwand Schubtragfähigkeit"),
        demand: force_n(pile.design_shear),
        capacity: force_n(v_pl),
        explanation: bilingual(format!("V_Ed={:.1} kN, V_pl,Rd={:.1} kN.", pile.design_shear / 1e3, v_pl / 1e3), format!("V_Ed={:.1} kN, V_pl,Rd={:.1} kN.", pile.design_shear / 1e3, v_pl / 1e3)),
        ndps: &["ndp.en1993.v-870.2"],
        gammas: &[AnnexParams::GAMMA_M0],
    };
    emit(report, annex, case, || solved([raise_t(&|x| resist(x).1 >= pile.design_shear)]));
    let il = |th: f64| sheet_piling::interlock_shear_utilisation(pile.interlock_shear / 1e3, pile.interlock_stiffness_factor, fy, resist(th).3, gm0);
    let case = Case {
        id: format!("en1993.5.5.2.il.{}", pile.id),
        part: "DIN EN 1993-5",
        family: "EN 1993-5",
        chapter: "5",
        section: "5.2.2",
        subject: subject("interlockShear", "Interlock shear", "Schlossschub"),
        title: loc("Sheet pile interlock shear", "Spundwand Schlossschub"),
        demand: dimensionless(il(t)),
        capacity: dimensionless(1.0),
        explanation: bilingual(format!("V_L,Ed={:.0} kN/m, utilisation {:.3}.", pile.interlock_shear / 1e3, il(t)), format!("V_L,Ed={:.0} kN/m, Ausnutzung {:.3}.", pile.interlock_shear / 1e3, il(t))),
        ndps: &["ndp.en1993.v-870.3"],
        gammas: &[AnnexParams::GAMMA_M0],
    };
    emit(report, annex, case, || solved([raise_t(&|x| il(x) <= 1.0)]));
}

/// 🚜 EN 1993-6 crane runway girders: wheel load introduction, torsion, von Mises web check and fatigue.
pub(super) fn check_crane(annex: AnnexChoice, params: AnnexParams, c: &CraneRunway, report: &mut CheckReport) {
    if !(c.max_wheel_load > 0.0 && c.web_thickness > 0.0) {
        return;
    }
    let (tw, fy, gm0) = (c.web_thickness * 1e3, c.fy / 1e6, params.gamma_m0);
    let subject = |field: &str, en: &str, de: &str| subj(&c.id, "craneRunways", field, en, de);
    let fz = crane_runway::wheel_load_design(c.max_wheel_load, c.phi, c.load_factor);
    let stresses = |th: f64| {
        let i_rf = crane_runway::i_rf(c.rail_inertia * 1e12, c.flange_effective_width * 1e3, c.flange_thickness * 1e3, c.rail_fixed == 1);
        let l_eff = crane_runway::l_eff(i_rf, th);
        let soz = crane_runway::sigma_oz(fz, l_eff, th);
        let tau = crane_runway::tau_oxz(soz) + crane_runway::tau_st_venant(crane_runway::wheel_torsion(fz, c.wheel_eccentricity * 1e3), th, c.torsion_constant * 1e12);
        (soz, tau, l_eff)
    };
    let vm = |th: f64| {
        let (soz, tau, _) = stresses(th);
        crane_runway::web_von_mises(c.longitudinal_stress / 1e6, soz, tau, fy, gm0)
    };
    let (soz, tau, l_eff) = stresses(tw);
    let hertz = crane_runway::hertz_line_contact(fz, e_mpa(), c.rail_contact_width * 1e3, c.rail_radius * 1e3);
    let case = Case {
        id: format!("en1993.6.5.7.vm.{}", c.id),
        part: "DIN EN 1993-6",
        family: "EN 1993-6",
        chapter: "5",
        section: "5.7",
        subject: subject("webThickness", "Web thickness", "Stegdicke"),
        title: loc("Web under wheel loads (von Mises)", "Steg unter Radlasten (von Mises)"),
        demand: dimensionless(vm(tw)),
        capacity: dimensionless(1.0),
        explanation: bilingual(format!("F_z,Ed={:.1} kN, l_eff={l_eff:.0} mm, σ_oz={soz:.1} MPa, τ={tau:.1} MPa, σ_ox={:.1} MPa; Hertz stress {hertz:.0} MPa.", fz / 1e3, c.longitudinal_stress / 1e6), format!("F_z,Ed={:.1} kN, l_eff={l_eff:.0} mm, σ_oz={soz:.1} MPa, τ={tau:.1} MPa, σ_ox={:.1} MPa; Hertzsche Pressung {hertz:.0} MPa.", fz / 1e3, c.longitudinal_stress / 1e6)),
        ndps: &["ndp.en1993.v-890.1"],
        gammas: &[AnnexParams::GAMMA_M0],
    };
    emit(report, annex, case, || {
        solved([grow(tw, None, |x| vm(x) <= 1.0).map(|th| lift(subject("webThickness", "Web thickness", "Stegdicke"), length_m(c.web_thickness), length_m(th / 1e3), &format!("Increase the web thickness to at least {th:.1} mm."), &format!("Stegdicke auf mindestens {th:.1} mm erhöhen.")))])
    });
    if c.crane_nu > 0.0 && c.stress_range > 0.0 && c.detail_category_stress > 0.0 {
        let lam = crane_runway::lambda_crane(c.crane_nu, c.crane_kq, c.damage_exponent);
        let u = crane_runway::crane_fatigue_utilisation(lam, c.fatigue_phi, c.stress_range / 1e6, c.detail_category_stress / 1e6, 1.0, params.gamma_mf);
        let case = Case {
            id: format!("en1993.6.9.fat.{}", c.id),
            part: "DIN EN 1993-6",
            family: "EN 1993-6",
            chapter: "9",
            section: "9.4",
            subject: subject("stressRange", "Stress range", "Spannungsschwingbreite"),
            title: loc("Crane runway fatigue", "Kranbahn Ermüdung"),
            demand: dimensionless(u),
            capacity: dimensionless(1.0),
            explanation: bilingual(format!("λ={lam:.3} (ν={:.2}, k_Q={:.2}, m={:.0}), Δσ={:.1} MPa against Δσ_C={:.1} MPa.", c.crane_nu, c.crane_kq, c.damage_exponent, c.stress_range / 1e6, c.detail_category_stress / 1e6), format!("λ={lam:.3} (ν={:.2}, k_Q={:.2}, m={:.0}), Δσ={:.1} MPa gegen Δσ_C={:.1} MPa.", c.crane_nu, c.crane_kq, c.damage_exponent, c.stress_range / 1e6, c.detail_category_stress / 1e6)),
            ndps: &["ndp.en1993.v-890.2"],
            gammas: &[AnnexParams::GAMMA_MF],
        };
        emit(report, annex, case, || {
            let needed = c.detail_category_stress / 1e6 * u;
            let next = bridge_fatigue::category_ladder().into_iter().rev().find(|k| *k >= needed).unwrap_or(160.0);
            vec![lift(subject("detailCategoryStress", "Detail category", "Kerbfall"), stress_pa(c.detail_category_stress), stress_pa(next * 1e6), &format!("Choose a detail of category {next:.0} or higher."), &format!("Kerbfall {next:.0} oder höher wählen."))]
        });
    }
}

#[cfg(test)]
mod solver_tests {
    use super::*;

    #[test]
    fn grow_solves_the_smallest_value_and_gives_up_at_the_limit() {
        let x = grow(1.0, None, |x| x * x >= 50.0).expect("reachable");
        assert!(x * x >= 50.0 && (x - 50f64.sqrt()).abs() < 1e-3, "{x}");
        assert_eq!(grow(2.0, None, |x| x >= 1.0), Some(2.0));
        assert!(grow(1.0, None, |x| x >= 1.0e6).is_none(), "beyond the solver reach");
        assert!(grow(1.0, Some(5.0), |x| x >= 6.0).is_none(), "beyond the field limit");
        let capped = grow(1.0, Some(60.0), |x| x >= 59.0).expect("inside the field limit");
        assert!((59.0..=60.0).contains(&capped), "{capped}");
    }

    #[test]
    fn shrink_solves_the_largest_value_and_gives_up_at_the_limit() {
        let x = shrink(10.0, None, |x| x * x <= 50.0).expect("reachable");
        assert!(x * x <= 50.0 && (x - 50f64.sqrt()).abs() < 1e-3, "{x}");
        assert_eq!(shrink(1.0, None, |x| x <= 2.0), Some(1.0));
        assert!(shrink(10.0, None, |x| x <= 1.0e-6).is_none(), "beyond the solver reach");
        assert!(shrink(10.0, Some(4.0), |x| x <= 3.0).is_none(), "beyond the field limit");
    }

    #[test]
    fn solved_keeps_only_the_remedies_that_could_be_solved() {
        let subject = subj("p1", "platedPanels", "thickness", "Thickness", "Dicke");
        let remedy = lift(subject, length_m(0.01), length_m(0.02), "Increase.", "Erhöhen.");
        assert_eq!(solved([None, Some(remedy.clone()), None]), vec![remedy]);
        assert!(solved([None, None]).is_empty());
    }
}
