//! 🧷️ Restraint checks: M_cr of the actual supports, springs and load height by finite elements and the resulting M_b,Rd (6.3.2), the general method (6.3.4) and the stable length next to a plastic hinge (6.3.5.3). See [EN 1993-1-1](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::mcr_fe::{mcr_fe, BeamProps, Load, McrResult, Restraints, Support};
use crate::clauses::stability as st;
use semio_s_artifact_norm_en1993_model::Restraint;

const ELEMENTS: usize = 40;

struct Context<'a> {
    restraint: &'a Restraint,
    member: &'a SteelMember,
    section: &'a SteelSection,
    material: &'a SteelMaterial,
}

fn subject(r: &Restraint, field: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(&r.id, format!("restraints[id={}].{field}", r.id), loc(en, de))
}

fn supports(r: &Restraint, l: f64) -> Vec<Support> {
    let end = |kind: &str, x: f64| if kind == "fixed" { Support::fixed(x) } else { Support::fork(x) };
    let mut out = vec![end(r.start_support.as_str(), 0.0), end(r.end_support.as_str(), l)];
    let x = r.intermediate_position * 1e3;
    if x > 0.0 && x < l {
        out.push(Support::fork(x));
    }
    out
}

fn mcr_run(c: &Context<'_>, torsional_spring: f64) -> Option<(f64, McrResult)> {
    let (r, m, s, mat) = (c.restraint, c.member, c.section, c.material);
    let l = m.length * 1e3;
    let props = BeamProps { l, e: mat.e_modulus / 1e6, g: mat.g_modulus / 1e6, iz: s.iz * 1e12, it: s.it * 1e12, iw: s.iw * 1e18 };
    let lateral = if r.lateral_spring_stiffness > 0.0 && r.lateral_spring_position > 0.0 && r.lateral_spring_position < m.length { vec![(r.lateral_spring_position * 1e3, r.lateral_spring_stiffness / 1e3)] } else { Vec::new() };
    let restraints = Restraints { supports: supports(r, l), torsional_spring_per_length: torsional_spring, lateral_springs: lateral };
    let psi = m.end_moment_ratio_psi;
    let (peak, result) = match r.load_distribution {
        semio_s_artifact_norm_en1993_model::RestraintDistribution::Udl => {
            let parabola = move |x: f64| x * (l - x) / 2.0;
            let q = |_x: f64| 1.0;
            (l * l / 8.0, mcr_fe(&props, &Load { moment: &parabola, line_load: &q, zg: r.load_height * 1e3 }, &restraints, ELEMENTS))
        }
        semio_s_artifact_norm_en1993_model::RestraintDistribution::Linear => {
            let linear = move |x: f64| 1.0 + (psi - 1.0) * x / l;
            let q = |_x: f64| 0.0;
            (1.0, mcr_fe(&props, &Load { moment: &linear, line_load: &q, zg: r.load_height * 1e3 }, &restraints, ELEMENTS))
        }
        _ => {
            let uniform = |_x: f64| 1.0;
            let q = |_x: f64| 0.0;
            (1.0, mcr_fe(&props, &Load { moment: &uniform, line_load: &q, zg: r.load_height * 1e3 }, &restraints, ELEMENTS))
        }
    };
    result.map(|res| (peak, res))
}

fn mcr_nm(c: &Context<'_>, torsional_spring: f64) -> Option<f64> {
    mcr_run(c, torsional_spring).map(|(peak, res)| res.lambda * peak / 1e3)
}

fn alpha_lt(s: &SteelSection) -> f64 {
    st::ltb_alpha_table_6_4(s.kind != semio_s_artifact_norm_en1993_model::SectionKind::WeldedI, s.kind == semio_s_artifact_norm_en1993_model::SectionKind::WeldedI, s.h / s.b.max(1e-9))
}

fn resisting_modulus(c: &Context<'_>) -> f64 {
    let class = part_1_1::section_class_rolled_i(c.section, c.material.fy);
    part_1_1::bending_modulus_y(c.section, class, c.material.fy)
}

fn m_b_rd(c: &Context<'_>, mcr: f64, m_ed: f64, gm1: f64) -> (f64, f64, f64) {
    let (s, mat) = (c.section, c.material);
    let w = resisting_modulus(c);
    let lambda = st::lambda_lt_from_mcr(w * 1e9, mat.fy / 1e6, mcr * 1e3);
    let chi = st::chi_lt_general(lambda, alpha_lt(s), m_ed * 1e3, mcr * 1e3);
    (chi * w * mat.fy / gm1, chi, lambda)
}

fn check_mcr(annex: AnnexChoice, params: AnnexParams, c: &Context<'_>, report: &mut CheckReport) -> Option<f64> {
    let r = c.restraint;
    if r.design_moment <= 0.0 || c.member.length <= 0.0 {
        return None;
    }
    let mcr = mcr_nm(c, r.torsional_spring)?;
    let (rd, chi, lambda) = m_b_rd(c, mcr, r.design_moment, params.gamma_m1);
    let closed = st::mcr_closed_form(c.material.e_modulus / 1e6, c.material.g_modulus / 1e6, c.section.iz * 1e12, c.section.it * 1e12, c.section.iw * 1e18, c.member.length * 1e3, 1.0, 1.0, st::c1_end_moments(c.member.end_moment_ratio_psi), 0.0, 0.0) / 1e3;
    let explanation = loc(
        &format!("M_cr by Hermite finite elements ({ELEMENTS} elements)={:.1} kNm for the supports {}/{}, intermediate x={:.2} m, spring c_θ={:.0} N, load height {:.0} mm; closed form with C_1 of the end moments {:.1} kNm; λ̄_LT={lambda:.3}, χ_LT={chi:.3}, M_b,Rd={:.1} kNm (W_el), M_Ed={:.1} kNm.", mcr / 1e3, r.start_support, r.end_support, r.intermediate_position, r.torsional_spring, r.load_height * 1e3, closed / 1e3, rd / 1e3, r.design_moment / 1e3),
        &format!("M_cr mit Hermite-Finiten-Elementen ({ELEMENTS} Elemente)={:.1} kNm für die Lagerung {}/{}, Zwischenlager x={:.2} m, Drehbettung c_θ={:.0} N, Lastangriff {:.0} mm; Näherung mit C_1 der Randmomente {:.1} kNm; λ̄_LT={lambda:.3}, χ_LT={chi:.3}, M_b,Rd={:.1} kNm (W_el), M_Ed={:.1} kNm.", mcr / 1e3, r.start_support, r.end_support, r.intermediate_position, r.torsional_spring, r.load_height * 1e3, closed / 1e3, rd / 1e3, r.design_moment / 1e3),
    );
    let mut b = CheckResult::assess(format!("en1993.1-1.6.3.2.mcrfe.{}", r.id), "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "6", "6.3.2.2"), subject(r, "startSupport", "Support", "Lagerung"), loc("Lateral-torsional buckling with the actual supports and restraints", "Biegedrillknicken mit den tatsächlichen Lagerungen und Halterungen"))
        .annex(annex)
        .ndp(AnnexParams::GAMMA_M1)
        .ndp("ndp.en1993.v-113b.1")
        .utilization(moment_nm(r.design_moment), moment_nm(rd))
        .explanation(explanation);
    if r.design_moment > rd {
        let mut spring = r.torsional_spring.max(1.0);
        for _ in 0..24 {
            if mcr_nm(c, spring).is_some_and(|m| r.design_moment <= m_b_rd(c, m, r.design_moment, params.gamma_m1).0) {
                break;
            }
            spring *= 1.6;
        }
        b = b
            .remedy(Remedy::at_least(subject(r, "torsionalSpring", "Torsional spring", "Drehbettung"), force_n(r.torsional_spring), force_n(spring), loc(&format!("Provide a continuous torsional restraint of at least {spring:.0} N (N·m/m)."), &format!("Kontinuierliche Drehbettung von mindestens {spring:.0} N (N·m/m) vorsehen."))))
            .remedy(Remedy::at_most(subject(r, "designMoment", "Design moment", "Bemessungsmoment"), moment_nm(r.design_moment), moment_nm(rd), loc(&format!("Or limit M_y,Ed to at most {:.1} kNm.", rd / 1e3), &format!("Oder M_y,Ed auf höchstens {:.1} kNm begrenzen.", rd / 1e3))));
    }
    report.push(b.build());
    Some(mcr)
}

fn check_general(annex: AnnexChoice, params: AnnexParams, c: &Context<'_>, mcr: f64, report: &mut CheckReport) {
    let (r, s, mat) = (c.restraint, c.section, c.material);
    if r.general_method != 1 || r.design_moment <= 0.0 {
        return;
    }
    let fy = mat.fy / 1e6;
    let class = part_1_1::section_class_rolled_i(s, mat.fy);
    let a_res = if class == 4 { part_1_1::effective_area_class4(s, mat.fy) } else { s.area };
    let (n_rk, m_rk) = (a_res * 1e6 * fy, resisting_modulus(c) * 1e9 * fy);
    let alpha_ult = st::alpha_ult_k_linear(r.design_axial, n_rk, r.design_moment * 1e3, m_rk, 0.0, 1.0);
    let ncr_z = st::n_cr(mat.e_modulus / 1e6 * s.iz * 1e12, c.member.buckling_length_z * 1e3);
    let alpha_cr_op = 1.0 / (r.design_axial.abs() / ncr_z + r.design_moment / mcr);
    let curve_alpha = st::curve_table_6_2(st::BucklingSectionKind::RolledI, false, s.h * 1e3, s.b * 1e3, s.tf * 1e3, fy).alpha();
    let alpha = alpha_lt(s).max(if r.design_axial.abs() > 0.0 { curve_alpha } else { 0.0 });
    let g = st::general_method_6_3_4(alpha_ult, alpha_cr_op, alpha, params.gamma_m1);
    let explanation = loc(
        &format!("α_ult,k={alpha_ult:.3} (N_Rk={:.0} kN, M_Rk={:.1} kNm), α_cr,op={alpha_cr_op:.3} (1/α_cr,op = N/N_cr,z + M/M_cr), λ̄_op={:.3}, χ_op={:.3}; utilisation γ_M1/(χ_op α_ult,k)={:.3}.", n_rk / 1e3, m_rk / 1e6, g.lambda_op, g.chi_op, g.utilisation),
        &format!("α_ult,k={alpha_ult:.3} (N_Rk={:.0} kN, M_Rk={:.1} kNm), α_cr,op={alpha_cr_op:.3} (1/α_cr,op = N/N_cr,z + M/M_cr), λ̄_op={:.3}, χ_op={:.3}; Ausnutzung γ_M1/(χ_op α_ult,k)={:.3}.", n_rk / 1e3, m_rk / 1e6, g.lambda_op, g.chi_op, g.utilisation),
    );
    let mut b = CheckResult::assess(format!("en1993.1-1.6.3.4.general.{}", r.id), "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "6", "6.3.4"), subject(r, "generalMethod", "General method", "Allgemeines Verfahren"), loc("General method for lateral and lateral-torsional buckling", "Allgemeines Verfahren für Knicken und Biegedrillknicken"))
        .annex(annex)
        .ndp(AnnexParams::GAMMA_M1)
        .ndp("ndp.en1993.v-123.1")
        .utilization(dimensionless(g.utilisation), dimensionless(1.0))
        .explanation(explanation);
    if g.utilisation > 1.0 {
        let allowed = r.design_moment / g.utilisation;
        b = b.remedy(Remedy::at_most(subject(r, "designMoment", "Design moment", "Bemessungsmoment"), moment_nm(r.design_moment), moment_nm(allowed), loc(&format!("Limit M_y,Ed to at most {:.1} kNm.", allowed / 1e3), &format!("M_y,Ed auf höchstens {:.1} kNm begrenzen.", allowed / 1e3))));
    }
    report.push(b.build());
}

fn stable_length(c: &Context<'_>) -> (f64, f64) {
    let (r, s, mat) = (c.restraint, c.section, c.material);
    let c1 = st::c1_end_moments(c.member.end_moment_ratio_psi);
    (st::plastic_hinge_stable_length((s.iz / s.area).sqrt() * 1e3, r.design_axial.abs(), s.area * 1e6, c1, s.w_pl_y * 1e9, s.it * 1e12, mat.fy / 1e6) / 1e3, c1)
}

/// 🧷️ Conditions of the plastic global analysis (EN 1993-1-1 5.6, V-029): class 1 at the hinges (5.6(2)), a restraint at the hinge location and the stable length L_m between hinge and next restraint (5.6(3), 6.3.5.3, Eq. 6.67); the verdict is the largest of the section class and the hinge-to-restraint distance over L_m, so 1 is the limit; without a restraint row that gives the hinge distance the gate is not applicable and names the missing input.
fn check_plastic_analysis(document: &En1993Snapshot, annex: AnnexChoice, scope: Scope<'_>, report: &mut CheckReport) {
    for member in document.members.iter().filter(|m| m.analysis == semio_s_artifact_norm_en1993_model::AnalysisKind::Plastic && scope.admits("members", &m.id)) {
        let (Some(section), Some(material)) = (document.sections.iter().find(|s| s.id == member.section_id), document.materials.iter().find(|m| m.id == member.material_id)) else { continue };
        let class = part_1_1::section_class_rolled_i(section, material.fy);
        let subject = || subject_member(member, &format!("members[id={}].analysis", member.id));
        let builder = CheckResult::assess(format!("en1993.5.6.plastic.{}", member.id), "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "5", "5.6"), subject(), loc("Plastic global analysis conditions", "Voraussetzungen der plastischen Schnittgrößenermittlung")).annex(annex);
        let hinge: Vec<(&Restraint, f64)> = document.restraints.iter().filter(|r| r.member_id == member.id && r.hinge_to_restraint > 0.0).map(|r| (r, stable_length(&Context { restraint: r, member, section, material }).0)).collect();
        let Some((worst, l_m)) = hinge.iter().copied().max_by(|a, b| (a.0.hinge_to_restraint / a.1).total_cmp(&(b.0.hinge_to_restraint / b.1))) else {
            report.push(builder.not_applicable(loc("No restraint row gives the distance from the plastic hinge to the next restraint (hingeToRestraint); the restraint at the hinge and the stable length L_m are not verified.", "Keine Halterungszeile gibt den Abstand vom Fließgelenk zur nächsten Halterung an (hingeToRestraint); Halterung am Gelenk und stabile Länge L_m sind nicht nachgewiesen.")).build());
            continue;
        };
        let ratio = worst.hinge_to_restraint / l_m;
        let mut b = builder.utilization(dimensionless((class as f64).max(ratio)), dimensionless(1.0)).explanation(loc(
            &format!("Class {class} at the hinge (class 1 required), hinge to restraint {:.2} m against L_m={l_m:.2} m (restraint {}).", worst.hinge_to_restraint, worst.id),
            &format!("Klasse {class} am Gelenk (Klasse 1 gefordert), Fließgelenk bis Halterung {:.2} m gegen L_m={l_m:.2} m (Halterung {}).", worst.hinge_to_restraint, worst.id),
        ));
        if ratio > 1.0 {
            b = b.remedy(Remedy::at_most(SubjectRef::new(&worst.id, format!("restraints[id={}].hingeToRestraint", worst.id), loc("Hinge to restraint", "Fließgelenk bis Halterung")), length_m(worst.hinge_to_restraint), length_m(l_m), loc(&format!("Place the next restraint within {l_m:.2} m of the hinge."), &format!("Nächste Halterung innerhalb von {l_m:.2} m vom Fließgelenk anordnen."))));
        }
        report.push(b.build());
    }
}

fn check_stable_length(annex: AnnexChoice, c: &Context<'_>, report: &mut CheckReport) {
    let r = c.restraint;
    if r.hinge_to_restraint <= 0.0 {
        return;
    }
    let (l_m, c1) = stable_length(c);
    let explanation = loc(
        &format!("L_m=38 i_z/√(N_Ed/(57.4 A)+W_pl,y²/(756 C_1² A I_T)(f_y/235)²)={l_m:.2} m (C_1={c1:.3}); distance from the plastic hinge to the next restraint {:.2} m.", r.hinge_to_restraint),
        &format!("L_m=38 i_z/√(N_Ed/(57,4 A)+W_pl,y²/(756 C_1² A I_T)(f_y/235)²)={l_m:.2} m (C_1={c1:.3}); Abstand vom Fließgelenk zur nächsten Halterung {:.2} m.", r.hinge_to_restraint),
    );
    let mut b = CheckResult::assess(format!("en1993.1-1.6.3.5.lm.{}", r.id), "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "6", "6.3.5.3"), subject(r, "hingeToRestraint", "Hinge to restraint", "Fließgelenk bis Halterung"), loc("Stable length next to a plastic hinge", "Stabile Länge neben einem Fließgelenk"))
        .annex(annex)
        .utilization(length_m(r.hinge_to_restraint), length_m(l_m))
        .explanation(explanation);
    if r.hinge_to_restraint > l_m {
        b = b.remedy(Remedy::at_most(subject(r, "hingeToRestraint", "Hinge to restraint", "Fließgelenk bis Halterung"), length_m(r.hinge_to_restraint), length_m(l_m), loc(&format!("Place the next restraint within {l_m:.2} m of the hinge."), &format!("Nächste Halterung innerhalb von {l_m:.2} m vom Fließgelenk anordnen."))));
    }
    report.push(b.build());
}

/// 🧷️ Appends the restraint checks of every admitted restraint row.
pub fn check_restraints(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, scope: Scope<'_>, report: &mut CheckReport) {
    for restraint in document.restraints.iter().filter(|r| scope.admits("restraints", &r.id)) {
        let Some(member) = document.members.iter().find(|m| m.id == restraint.member_id) else { continue };
        let (Some(section), Some(material)) = (document.sections.iter().find(|s| s.id == member.section_id), document.materials.iter().find(|m| m.id == member.material_id)) else { continue };
        let context = Context { restraint, member, section, material };
        let mcr = check_mcr(annex, params, &context, report);
        if let Some(mcr) = mcr {
            check_general(annex, params, &context, mcr, report);
        }
        check_stable_length(annex, &context, report);
    }
    check_plastic_analysis(document, annex, scope, report);
}

/// 🧷️ M_cr by finite elements, χ_LT and the mode shape of one restraint row for the scene diagrams (N·m, m, rad).
#[derive(Clone, Debug)]
pub struct RestraintSummary {
    pub restraint_id: String,
    pub mcr: f64,
    pub lambda_lt: f64,
    pub chi_lt: f64,
    pub alpha_lt: f64,
    pub m_ed: f64,
    pub m_b_rd: f64,
    pub stations: Vec<f64>,
    pub mode_v: Vec<f64>,
    pub mode_phi: Vec<f64>,
}

/// 🧷️ The M_cr, χ_LT and mode shape summary of a restraint, `None` without member data or design moment.
pub fn restraint_summary(document: &En1993Snapshot, restraint_id: &str) -> Option<RestraintSummary> {
    let restraint = document.restraints.iter().find(|r| r.id == restraint_id)?;
    let member = document.members.iter().find(|m| m.id == restraint.member_id)?;
    let section = document.sections.iter().find(|s| s.id == member.section_id)?;
    let material = document.materials.iter().find(|m| m.id == member.material_id)?;
    if restraint.design_moment <= 0.0 || member.length <= 0.0 {
        return None;
    }
    let params = AnnexParams::for_choice(document.annex);
    let context = Context { restraint, member, section, material };
    let (peak, run) = mcr_run(&context, restraint.torsional_spring)?;
    let mcr = run.lambda * peak / 1e3;
    let (m_b_rd, chi_lt, lambda_lt) = m_b_rd(&context, mcr, restraint.design_moment, params.gamma_m1);
    Some(RestraintSummary {
        restraint_id: restraint.id.clone(),
        mcr,
        lambda_lt,
        chi_lt,
        alpha_lt: alpha_lt(section),
        m_ed: restraint.design_moment,
        m_b_rd,
        stations: run.stations.iter().map(|x| x / 1e3).collect(),
        mode_v: run.mode_v.clone(),
        mode_phi: run.mode_phi.clone(),
    })
}
