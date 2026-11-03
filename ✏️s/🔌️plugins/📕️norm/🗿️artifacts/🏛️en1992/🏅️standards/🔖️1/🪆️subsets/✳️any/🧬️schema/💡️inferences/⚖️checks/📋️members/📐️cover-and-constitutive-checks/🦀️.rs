//! 📐️ Cover and constitutive checks: the durability cover and the constitutive relation checks of the member evaluation. See [`super`].

use super::*;

/// 📐 Checks the concrete cover against the durability minimum c_nom.
pub(super) fn check_cover(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, f_ck: f64) {
    let phi0 = member.longitudinal.first().map(|l| l.diameter).unwrap_or(0.012);
    let agg0 = member.longitudinal.first().map(|l| l.aggregate_size).unwrap_or(0.016);
    let bundle0 = 1u32;
    let c_min_b = part_1_1::c_min_b_m(annex, phi0, agg0, bundle0);
    let c_min_dur = part_1_1::c_min_dur_adjusted_m(annex, member.exposure, doc.design_working_life_years, f_ck, member.kind);
    let (c_min, c_nom) = part_1_1::c_nom_m(c_min_b, c_min_dur, doc.delta_c_dev);
    let cover_path = member_path(&member.id, "cover");
    let mut cover_check = CheckResult::assess(format!("en1992.4.4.cover.{}", member.id), part_11(), clause_11("§4.4"), member_ref(doc, member, &cover_path), l("Concrete cover for durability", "Betondeckung für Dauerhaftigkeit"))
        .minimum(length_m(member.cover), length_m(c_nom))
        .annex(annex)
        .explanation(l(
            &format!("c={:.0} mm ≥ c_nom={:.0} mm (c_min={:.0}=max(c_min,b={:.0}, c_min,dur={:.0}), life={:.0} a, cement={:?}, Δc_dev={:.0})", member.cover * 1000.0, c_nom * 1000.0, c_min * 1000.0, c_min_b * 1000.0, c_min_dur * 1000.0, doc.design_working_life_years, doc.cement_type, doc.delta_c_dev * 1000.0),
            &format!("c={:.0} mm ≥ c_nom={:.0} mm (c_min={:.0}=max(c_min,b={:.0}, c_min,dur={:.0}), Nutzungsdauer={:.0} a, Zement={:?}, Δc_dev={:.0})", member.cover * 1000.0, c_nom * 1000.0, c_min * 1000.0, c_min_b * 1000.0, c_min_dur * 1000.0, doc.design_working_life_years, doc.cement_type, doc.delta_c_dev * 1000.0),
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
    out.push(ndps(cover_check, &["ndp.en1992.d-03.1", "ndp.en1992.d-03.2", "ndp.en1992.d-04.1"]).build());
}

/// 📏 Distance (m) of a point inside the closed polygon to its nearest edge; negative when the point lies outside.
fn edge_distance(outline: &[(f64, f64)], p: (f64, f64)) -> f64 {
    let n = outline.len();
    let nearest = (0..n)
        .map(|i| {
            let (a, b) = (outline[i], outline[(i + 1) % n]);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let t = if dx * dx + dy * dy < 1e-18 { 0.0 } else { (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0) };
            (p.0 - a.0 - t * dx).hypot(p.1 - a.1 - t * dy)
        })
        .fold(f64::INFINITY, f64::min);
    let inside = (0..n).filter(|&i| {
        let (a, b) = (outline[i], outline[(i + 1) % n]);
        (a.1 > p.1) != (b.1 > p.1) && p.0 < a.0 + (p.1 - a.1) * (b.0 - a.0) / (b.1 - a.1)
    }).count() % 2 == 1;
    if inside { nearest } else { -nearest }
}

/// 📐 Checks the persisted bar layout against the durability cover: the surface of every explicit bar (`bars[]`) keeps at least c_nom from the nearest concrete surface (EN 1992-1-1 4.4.1; links outside the bars only add to it, so this is a necessary condition); emitted only for members with explicit bars.
pub(super) fn check_bar_cover(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, f_ck: f64) {
    if member.bars.is_empty() {
        return;
    }
    let phi0 = member.longitudinal.first().map(|l| l.diameter).unwrap_or(0.012);
    let agg0 = member.longitudinal.first().map(|l| l.aggregate_size).unwrap_or(0.016);
    let c_min_b = part_1_1::c_min_b_m(annex, phi0, agg0, 1);
    let c_min_dur = part_1_1::c_min_dur_adjusted_m(annex, member.exposure, doc.design_working_life_years, f_ck, member.kind);
    let (_, c_nom) = part_1_1::c_nom_m(c_min_b, c_min_dur, doc.delta_c_dev);
    let outline = member.outline();
    let (worst, cover) = member
        .bars
        .iter()
        .map(|bar| (bar, edge_distance(&outline, member.bar_position(bar)) - bar.diameter / 2.0))
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(bar, cover)| (bar.id.clone(), cover))
        .unwrap_or_default();
    let path = member_path(&member.id, &format!("bars[id={worst}].z"));
    let check = CheckResult::assess(format!("en1992.4.4.bar-cover.{}", member.id), part_11(), clause_11("§4.4"), member_ref(doc, member, &path), l("Concrete cover of the placed bars", "Betondeckung der platzierten Stäbe")).remedies_attached_by("attach_minimal_remedies")
        .minimum(length_m(cover), length_m(c_nom))
        .annex(annex)
        .explanation(l(
            &format!("Bar {worst}: surface distance {:.0} mm ≥ c_nom={:.0} mm (persisted bar positions, links not counted).", cover * 1000.0, c_nom * 1000.0),
            &format!("Stab {worst}: Randabstand {:.0} mm ≥ c_nom={:.0} mm (gespeicherte Stabpositionen, Bügel nicht gezählt).", cover * 1000.0, c_nom * 1000.0),
        ));
    out.push(ndps(check, &["ndp.en1992.d-03.1", "ndp.en1992.d-03.2", "ndp.en1992.d-04.1"]).build());
}

/// 🧪 Checks the concrete strength class against the minimum class of the exposure (Annex E.1DE) and the scope of Table 3.1 (f_ck ≤ 90 MPa).
pub(super) fn check_constitutive(
    doc: &En1992Snapshot,
    member: &RcMember,
    out: &mut Vec<CheckResult>,
    annex: AnnexChoice,
    _concrete: &semio_s_artifact_norm_en1992_model::ConcreteGrade,
    f_ck: f64,
    _eps_cu2: f64,
    _n_parabola: f64,
    _f_ck_cube: f64,
) {
    let name = format!("{:?}", member.exposure).to_uppercase();
    let required = min_fck(&name, false).unwrap_or(12.0) * 1.0e6;
    let target = member_ref(doc, member, &format!("concreteGrades[id={}].fCk", member.concrete_grade_id));
    let mut mat = CheckResult::assess(format!("en1992.4.2.min-class.{}", member.id), part_11(), clause_11("§4.2"), target.clone(), l("Minimum concrete class for the exposure", "Mindestbetonfestigkeitsklasse für die Expositionsklasse"))
        .minimum(stress_pa(f_ck), stress_pa(required))
        .annex(annex)
        .explanation(l(
            &format!("Annex E.1DE: f_ck={:.0} MPa ≥ f_ck,min={:.0} MPa for exposure {}.", f_ck / 1e6, required / 1e6, name),
            &format!("Anhang E.1DE: f_ck={:.0} MPa ≥ f_ck,min={:.0} MPa für Expositionsklasse {}.", f_ck / 1e6, required / 1e6, name),
        ));
    if f_ck < required {
        mat = mat.remedy(Remedy::at_least(target, stress_pa(f_ck), stress_pa(required), l("Raise the concrete class to the minimum class of the exposure.", "Betonfestigkeitsklasse auf die Mindestklasse der Expositionsklasse anheben.")));
    }
    out.push(ndps(mat, &["ndp.en1992.d-02.1"]).build());
    if f_ck > 90.0e6 {
        out.push(CheckResult::assess(format!("en1992.3.1.scope.{}", member.id), part_11(), clause_11("§3.1"), member_ref(doc, member, &format!("concreteGrades[id={}].fCk", member.concrete_grade_id)), l("Concrete class outside EN 1992-1-1", "Betonklasse außerhalb EN 1992-1-1")).annex(annex).not_applicable(l("f_ck above 90 MPa is outside the scope of EN 1992-1-1.", "f_ck über 90 MPa liegt außerhalb des Anwendungsbereichs von EN 1992-1-1.")).build());
    }
}
