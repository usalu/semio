//! 📐️ Cover and constitutive checks: the durability cover and the constitutive relation checks of the member evaluation. See [`super`].

use super::*;

/// 📐 Checks the concrete cover against the durability minimum c_nom.
pub(super) fn check_cover(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, f_ck: f64) {
    let phi0 = member.longitudinal.first().map(|l| l.diameter).unwrap_or(0.012);
    let agg0 = member.longitudinal.first().map(|l| l.aggregate_size).unwrap_or(0.016);
    let bundle0 = 1u32;
    let c_min_b = part_1_1::c_min_b_m(phi0, agg0, bundle0);
    let c_min_dur = part_1_1::c_min_dur_adjusted_m(member.exposure, doc.design_working_life_years, &doc.cement_type, f_ck, member.kind);
    let (c_min, c_nom) = part_1_1::c_nom_m(c_min_b, c_min_dur, doc.delta_c_dev);
    let cover_path = member_path(&member.id, "cover");
    let mut cover_check = CheckResult::assess(format!("en1992.4.4.cover.{}", member.id), part_11(), clause_11("§4.4"), member_ref(doc, member, &cover_path), l("Concrete cover for durability", "Betondeckung für Dauerhaftigkeit"))
        .minimum(length_m(member.cover), length_m(c_nom))
        .annex(annex)
        .explanation(l(
            &format!("c={:.0} mm ≥ c_nom={:.0} mm (c_min={:.0}=max(c_min,b={:.0}, c_min,dur={:.0}), life={:.0} a, cement={}, Δc_dev={:.0})", member.cover * 1000.0, c_nom * 1000.0, c_min * 1000.0, c_min_b * 1000.0, c_min_dur * 1000.0, doc.design_working_life_years, doc.cement_type, doc.delta_c_dev * 1000.0),
            &format!("c={:.0} mm ≥ c_nom={:.0} mm (c_min={:.0}=max(c_min,b={:.0}, c_min,dur={:.0}), Nutzungsdauer={:.0} a, Zement={}, Δc_dev={:.0})", member.cover * 1000.0, c_nom * 1000.0, c_min * 1000.0, c_min_b * 1000.0, c_min_dur * 1000.0, doc.design_working_life_years, doc.cement_type, doc.delta_c_dev * 1000.0),
        ));
    if member.cover < c_nom {
        cover_check = cover_check.remedy(Remedy::at_least(
            member_ref(doc, member, &cover_path),
            length_m(member.cover),
            length_m(c_nom),
            l(
                &format!("Increase cover of {} from {:.0} mm to at least {:.0} mm (c_nom).", member.id, member.cover * 1000.0, c_nom * 1000.0),
                &format!("Betondeckung von {} von {:.0} mm auf mindestens {:.0} mm (c_nom) erhöhen.", member.id, member.cover * 1000.0, c_nom * 1000.0),
            ),
        ));
    }
    out.push(cover_check.build());
}

/// 🧪 Checks the constitutive relation of the concrete grade against Table 3.1.
pub(super) fn check_constitutive(
    doc: &En1992Snapshot,
    member: &RcMember,
    out: &mut Vec<CheckResult>,
    annex: AnnexChoice,
    concrete: &semio_s_artifact_norm_en1992_model::ConcreteGrade,
    f_ck: f64,
    eps_cu2: f64,
    n_parabola: f64,
    f_ck_cube: f64,
) {
    {
        let computed = f_ck * concrete.eps_c2() * n_parabola;
        let limit = f_ck_cube * eps_cu2 * 2.0;
        let mut mat = CheckResult::assess(format!("en1992.3.1.constitutive.{}", member.id), part_11(), clause_11("§3.1.7"), member_ref(doc, member, &format!("concreteGrades[id={}].fCk", member.concrete_grade_id)), l("Concrete Table 3.1 constitutive", "Beton Tabelle 3.1 Stoffgesetz"))
            .utilization(stress_pa(computed), stress_pa(limit.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("Table 3.1: f_ck={:.0} MPa / f_ck,cube={:.0} MPa; ε_c2={:.2}‰ / ε_cu2={:.2}‰; n={:.2}", f_ck / 1e6, f_ck_cube / 1e6, concrete.eps_c2() * 1e3, eps_cu2 * 1e3, n_parabola),
                &format!("Tabelle 3.1: f_ck={:.0} MPa / f_ck,cube={:.0} MPa; ε_c2={:.2}‰ / ε_cu2={:.2}‰; n={:.2}", f_ck / 1e6, f_ck_cube / 1e6, concrete.eps_c2() * 1e3, eps_cu2 * 1e3, n_parabola),
            ));
        if computed > limit {
            mat = mat.remedy(Remedy::at_most(
                member_ref(doc, member, &format!("concreteGrades[id={}].fCk", member.concrete_grade_id)),
                stress_pa(f_ck),
                stress_pa(f_ck * limit / computed.max(1.0)),
                l("Reduce f_ck toward a Table 3.1 class so constitutive ratios stay consistent.", "f_ck auf eine Klasse nach Tabelle 3.1 absenken, damit die Stoffgesetz-Verhältnisse konsistent bleiben."),
            ));
        }
        out.push(mat.build());
    }
}
