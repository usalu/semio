//! 📋️ EN 1992 member evaluation against EN 1992-1-1 and 1-2, with optional bridge and liquid-retaining checks.

use super::*;

#[path = "📉️effect-profile/🦀️.rs"]
mod effect_profile;
use effect_profile::Profile;

#[path = "🔩️stud-layout/🦀️.rs"]
mod stud_layout;
use stud_layout::stud_layout;

#[path = "🕳️control-perimeter/🦀️.rs"]
mod control_perimeter;
use control_perimeter::{control_perimeter, local_factor};

#[path = "📐️cover-and-constitutive-checks/🦀️.rs"]
mod cover_and_constitutive_checks;
use cover_and_constitutive_checks::{check_bar_cover, check_cover, check_constitutive};

#[path = "🏗️resistance-and-serviceability-checks/🦀️.rs"]
mod resistance_and_serviceability_checks;
pub use resistance_and_serviceability_checks::{crack_case, deep_beam_truss, torsion_case, torsion_chords, torsion_cover_mm, torsion_t_min_mm, DeepTruss, DEEP_BEAM_RATIO};
use resistance_and_serviceability_checks::{check_ultimate_resistances, check_crack_width, check_reinforcement_limits, check_strut_and_tie, check_deflection, check_deflection_profile, check_sls_stress_limits};

#[path = "🧱️slenderness-prestress-and-ductility-checks/🦀️.rs"]
mod slenderness_prestress_and_ductility_checks;
use slenderness_prestress_and_ductility_checks::{check_column_slenderness, check_prestress, check_ductility};

#[path = "🧷️entity-cases/🦀️.rs"]
pub mod entity_cases;
use entity_cases::check_entity_cases;
pub use entity_cases::evaluate_material_cases;

#[path = "🔥️fire-resistance/🦀️.rs"]
mod fire_resistance;
use fire_resistance::check_fire_resistance;

#[path = "🔥️fire-anchorage-bridge-and-liquid-checks/🦀️.rs"]
mod fire_anchorage_bridge_and_liquid_checks;
use fire_anchorage_bridge_and_liquid_checks::{check_fire, check_anchorage_and_laps, check_bridge, check_liquid_retaining};

#[path = "🩹️minimal-remedies/🦀️.rs"]
pub(super) mod minimal_remedies;
use minimal_remedies::attach_minimal_remedies;

/// 📋️ Evaluate one member against EN 1992-1-1/1-2 (+ optional bridge/liquid); every failing check carries the minimal one-click remedies of [`minimal_remedies`].
pub fn evaluate_member(doc: &En1992Snapshot, member: &RcMember) -> Vec<CheckResult> {
    let mut results = evaluate_member_unsolved(doc, member);
    attach_minimal_remedies(doc, member, &mut results);
    results
}

/// ⚡️ The report with the checks of the one dragged member `member_id` re-evaluated against `document` (the preview document): no remedy search, no fire thermal solve (the fire rows stay as they were), every other row untouched and in place, so crack lines and utilisation colours follow the drag within the live budget.
pub fn preview_member_report(document: &En1992Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport, member_id: &str) -> semio_s_plugin_norm_compliance::CheckReport {
    let Some(member) = document.members.iter().find(|m| m.id == member_id) else { return report.clone() };
    let mut candidate = member.clone();
    candidate.fire = None;
    let mut fresh: Vec<CheckResult> = evaluate_member_unsolved(document, &candidate).into_iter().filter(|r| !r.id.starts_with("en1992.1-2.")).collect();
    let mut out = semio_s_plugin_norm_compliance::CheckReport::default();
    let mut placed = false;
    for check in &report.checks {
        if check.subject.entity_id == member_id && !check.id.starts_with("en1992.1-2.") {
            if !placed {
                out.extend(std::mem::take(&mut fresh));
                placed = true;
            }
        } else {
            out.push(check.clone());
        }
    }
    out.extend(fresh);
    out
}

/// 📋️ The member checks without the solved remedies; the remedy search re-evaluates through this.
pub(super) fn evaluate_member_unsolved(doc: &En1992Snapshot, member: &RcMember) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let annex = doc.annex;
    let Some(concrete) = doc.concrete(&member.concrete_grade_id) else {
        let options: Vec<String> = doc.concrete_grades.iter().map(|g| g.id.clone()).collect();
        let mut missing = CheckResult::assess(format!("en1992.materials.concrete.{}", member.id), part_11(), clause_11("§3.1"), member_ref(doc, member, &member_path(&member.id, "concreteGradeId")), l("Concrete grade", "Betonfestigkeitsklasse"))
            .annex(annex)
            .explanation(l(
                &format!("Concrete grade id '{}' is missing.", member.concrete_grade_id),
                &format!("Betonfestigkeitsklasse-Id '{}' fehlt.", member.concrete_grade_id),
            ))
            .status(CheckStatus::Fail);
        missing = missing.remedy(Remedy::one_of(
            member_ref(doc, member, &member_path(&member.id, "concreteGradeId")),
            if options.is_empty() { vec!["c30".into()] } else { options },
            l("Set concreteGradeId to a declared concrete grade.", "concreteGradeId auf eine deklarierte Betonfestigkeitsklasse setzen."),
        ));
        out.push(missing.build());
        return out;
    };
    let Some(reinf) = doc.reinforcement(&member.reinforcement_grade_id) else {
        let options: Vec<String> = doc.reinforcement_grades.iter().map(|g| g.id.clone()).collect();
        let mut missing = CheckResult::assess(format!("en1992.materials.steel.{}", member.id), part_11(), clause_11("§3.2"), member_ref(doc, member, &member_path(&member.id, "reinforcementGradeId")), l("Reinforcement grade", "Betonstahl"))
            .annex(annex)
            .explanation(l(
                &format!("Reinforcement grade id '{}' is missing.", member.reinforcement_grade_id),
                &format!("Betonstahl-Id '{}' fehlt.", member.reinforcement_grade_id),
            ))
            .status(CheckStatus::Fail);
        missing = missing.remedy(Remedy::one_of(
            member_ref(doc, member, &member_path(&member.id, "reinforcementGradeId")),
            if options.is_empty() { vec!["b500".into()] } else { options },
            l("Set reinforcementGradeId to a declared steel grade.", "reinforcementGradeId auf einen deklarierten Betonstahl setzen."),
        ));
        out.push(missing.build());
        return out;
    };
    let f_ck = concrete.f_ck;
    let f_yk = reinf.f_yk;
    let e_s = reinf.e_s;
    let eps_cu2 = concrete.eps_cu2();
    let n_parabola = concrete.n_parabola();
    let f_ck_cube = concrete.f_ck_cube();
    let sec = member_section_in(doc, member, f_ck, f_yk, e_s, annex, false);
    let geo = Geo::of(&sec, member);
    let params = na_de::AnnexParams::for_choice(annex);
    check_cover(doc, member, &mut out, annex, f_ck);
    check_bar_cover(doc, member, &mut out, annex, f_ck);
    check_constitutive(doc, member, &mut out, annex, concrete, f_ck, eps_cu2, n_parabola, f_ck_cube);
    let effects = combine_member_actions(member, &doc.action_factors());
    let uls = governing(&effects, "uls");
    let acc = governing(&effects, "accidental");
    let sls_qp = governing(&effects, "sls_qp");
    let sls_char = governing(&effects, "sls_char");
    let uls_v = governing_by(&effects, "uls", |e| e.v_ed.abs());
    let uls_t = governing_by(&effects, "uls", |e| e.t_ed.abs());
    let uls_p = governing_by(&effects, "uls", |e| e.v_ed_punch.abs());
    check_ultimate_resistances(doc, member, &mut out, annex, f_ck, f_yk, e_s, &sec, &geo, uls, acc, uls_v, uls_t, uls_p);
    check_crack_width(doc, member, &mut out, annex, f_ck, e_s, &sec, &geo, sls_qp);
    check_reinforcement_limits(doc, member, &mut out, annex, f_ck, f_yk, &geo);
    check_strut_and_tie(doc, member, &mut out, annex, f_ck, &geo);
    check_deflection(doc, member, &mut out, annex, f_ck, &geo);
    check_deflection_profile(doc, member, &mut out, annex, concrete, e_s, &sec, &geo, sls_qp);
    check_column_slenderness(doc, member, &mut out, annex, f_ck, f_yk, e_s, &sec, &geo, &params, uls);
    check_prestress(doc, member, &mut out, annex, f_ck, &geo, sls_qp.map(|e| e.m_ed).unwrap_or(0.0));
    check_ductility(doc, member, &mut out, annex, reinf);
    check_fire(doc, member, &mut out, annex, geo.fire_dimension(member.kind));
    let fire_lead = if ndp(annex, "ndp.en1992.psi-fire-lead") >= 1.5 { ["sls_qp", "sls_freq"] } else { ["sls_freq", "sls_qp"] };
    check_fire_resistance(doc, member, &mut out, annex, &sec, f_yk, governing(&effects, fire_lead[0]).or_else(|| governing(&effects, fire_lead[1])));
    check_sls_stress_limits(doc, member, &mut out, annex, f_ck, f_yk, &sec, &geo, sls_qp, sls_char);
    check_anchorage_and_laps(doc, member, &mut out, annex, concrete, f_yk, &params);
    check_bridge(doc, member, &mut out, annex, f_ck);
    check_liquid_retaining(doc, member, &mut out, annex, e_s);
    check_entity_cases(doc, member, &mut out, annex, f_ck, f_yk);
    out
}
