//! 🔥️ Fire resistance at the rated time: bending with axial force on the reduced sections of EN 1992-1-2 Annex B.1 (500 °C isotherm) and Annex B.2 (zone method), both read from the single fire field. See [`super`].

use super::*;

/// 🔥 Checks M_Ed,fi ≤ M_Rd,fi(N_Ed,fi) of the isotherm section and of the zone section for the frequent combination (EN 1990 6.5.3) at the rated duration; the bars keep `k_s(θ_s) A_s`, the concrete of the reduced section keeps f_ck.
pub(super) fn check_fire_resistance(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, sec: &Sec, f_yk: f64, effects: Option<&DesignEffects>) {
    let (Some(fire), Some(e)) = (member.fire.as_ref(), effects) else { return };
    if sec.bars.is_empty() {
        return;
    }
    let (outline, faces) = (member.outline(), fire_field::exposed_faces(member.kind));
    let field = fire_field::solve_outline(&outline, faces, fire_field::rating_minutes(fire.rating), fire_field::MOISTURE_DEFAULT);
    let to_mm = |ring: &[(f64, f64)]| -> Vec<(f64, f64)> { ring.iter().map(|p| (p.0 * 1000.0, p.1 * 1000.0)).collect() };
    let zone = fire_field::zone_section(&outline, faces, &field, fire_field::zones());
    let (m_ed, n_comp) = (e.m_ed.abs(), -e.n_ed);
    let methods = [
        ("nm", "B.1", field.isotherm_ring(fire_field::LEVEL_500, 72), l("500 °C isotherm section, N-M at fire time", "500-°C-Isothermenquerschnitt, N-M zur Branddauer"), l("500 °C isotherm", "500-°C-Isotherme")),
        ("zone", "B.2", zone.contour.clone(), l("Zone method section, N-M at fire time", "Zonenmethode-Querschnitt, N-M zur Branddauer"), l("zone method", "Zonenmethode")),
    ];
    for (key, annex_ref, ring, title, name) in methods {
        let m_rd = if ring.len() >= 3 { fire_field::reduced_section(sec, to_mm(&ring), &field, f_yk / 1.0e6).m_rd_y(n_comp).map_or(0.0, |r| r.0 / 1000.0) } else { 0.0 };
        let check = CheckResult::assess(format!("en1992.1-2.{key}.{}", member.id), part_12(), ClauseId::new("EN 1992-1-2", &format!("§{annex_ref}"), annex_ref), member_ref(doc, member, &member_path(&member.id, "fire")), title).remedies_attached_by("attach_minimal_remedies")
            .utilization(moment_nm(m_ed), moment_nm(m_rd.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("{} {:?}: M_Ed,fi={:.1} kNm ≤ M_Rd,fi={:.1} kNm at N_Ed,fi={:.0} kN (frequent combination {}; θ_max={:.0} °C, a_z,max={:.0} mm)", name.en, fire.rating, m_ed / 1e3, m_rd / 1e3, -n_comp / 1e3, e.combination_id, field.peak(), zone.damage.iter().copied().fold(0.0, f64::max) * 1e3),
                &format!("{} {:?}: M_Ed,fi={:.1} kNm ≤ M_Rd,fi={:.1} kNm bei N_Ed,fi={:.0} kN (häufige Kombination {}; θ_max={:.0} °C, a_z,max={:.0} mm)", name.de, fire.rating, m_ed / 1e3, m_rd / 1e3, -n_comp / 1e3, e.combination_id, field.peak(), zone.damage.iter().copied().fold(0.0, f64::max) * 1e3),
            ));
        out.push(ndps(check, &["ndp.en1992.fire-ks-kc"]).build());
    }
}
