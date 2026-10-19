//! 📋️ EN 1992 member evaluation against EN 1992-1-1 and 1-2, with optional bridge and liquid-retaining checks.

use super::*;

#[path = "📐️cover-and-constitutive-checks/🦀️.rs"]
mod cover_and_constitutive_checks;
use cover_and_constitutive_checks::{check_cover, check_constitutive};

#[path = "🏗️resistance-and-serviceability-checks/🦀️.rs"]
mod resistance_and_serviceability_checks;
use resistance_and_serviceability_checks::{check_ultimate_resistances, check_crack_width, check_reinforcement_limits, check_deflection, check_sls_stress_limits};

#[path = "🧱️slenderness-prestress-and-ductility-checks/🦀️.rs"]
mod slenderness_prestress_and_ductility_checks;
use slenderness_prestress_and_ductility_checks::{check_column_slenderness, check_prestress, check_ductility};

#[path = "🔥️fire-anchorage-bridge-and-liquid-checks/🦀️.rs"]
mod fire_anchorage_bridge_and_liquid_checks;
use fire_anchorage_bridge_and_liquid_checks::{check_fire, check_anchorage_and_laps, check_bridge, check_liquid_retaining};

/// 📋️ Evaluate one member against EN 1992-1-1/1-2 (+ optional bridge/liquid).
pub fn evaluate_member(doc: &En1992Snapshot, member: &RcMember) -> Vec<CheckResult> {
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
    let b = member.width;
    let d = member.effective_depth;
    let h = member.height;
    let a_s = member.a_s_tension();
    let rho = member.rho_l();
    let params = na_de::AnnexParams::for_choice(annex);
    check_cover(doc, member, &mut out, annex, f_ck);
    check_constitutive(doc, member, &mut out, annex, concrete, f_ck, eps_cu2, n_parabola, f_ck_cube);
    let effects = combine_member_actions(member);
    let uls = governing(&effects, "uls");
    let acc = governing(&effects, "accidental");
    let sls_qp = governing(&effects, "sls_qp");
    let sls_char = governing(&effects, "sls_char");
    check_ultimate_resistances(doc, member, &mut out, annex, f_ck, f_yk, e_s, eps_cu2, n_parabola, b, d, h, a_s, rho, uls, acc);
    check_crack_width(doc, member, &mut out, annex, f_ck, e_s, b, d, h, a_s, rho, sls_qp);
    check_reinforcement_limits(doc, member, &mut out, annex, f_ck, f_yk, b, d, h, a_s);
    check_deflection(doc, member, &mut out, annex, d, rho);
    check_column_slenderness(doc, member, &mut out, annex, f_ck, f_yk, e_s, eps_cu2, n_parabola, b, d, h, a_s, &params, uls);
    check_prestress(doc, member, &mut out, annex, f_ck, b, h);
    check_ductility(doc, member, &mut out, annex, reinf);
    check_fire(doc, member, &mut out, annex, b);
    check_sls_stress_limits(doc, member, &mut out, annex, concrete, f_ck, f_yk, e_s, b, d, a_s, sls_qp, sls_char);
    check_anchorage_and_laps(doc, member, &mut out, annex, concrete, f_yk, &params);
    check_bridge(doc, member, &mut out, annex, f_ck);
    check_liquid_retaining(doc, member, &mut out, annex, e_s);
    out
}
