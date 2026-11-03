//! 🧱️ Member catalogue-gap checks: bow imperfection, shear area, torsional buckling, rolled and simplified LTB and floor frequency of EN 1993-1-1. See [`super`].

use super::*;

/// 🌀 Torsional buckling of one member under its governing compression (EN 1993-1-1 6.3.1.4): N_cr,T, slenderness, reduction and resistance.
#[derive(Clone, Copy, Debug)]
pub struct TorsionalSummary {
    pub n_ed: f64,
    pub n_cr_t: f64,
    pub n_cr_z: f64,
    pub lambda_t: f64,
    pub lambda_z: f64,
    pub alpha: f64,
    pub chi_t: f64,
    pub n_b_rd_t: f64,
}

fn resolved<'a>(document: &'a En1993Snapshot, member: &SteelMember) -> Option<(&'a SteelSection, &'a SteelMaterial)> {
    Some((find_section(document, &member.section_id)?, find_material(document, &member.material_id)?))
}

fn class_and_area(section: &SteelSection, material: &SteelMaterial) -> (u8, f64) {
    let class = part_1_1::section_class_rolled_i(section, material.fy);
    (class, if class == 4 { part_1_1::effective_area_class4(section, material.fy) } else { section.area })
}

fn peak(rows: &[&CombinedEffects], value: impl Fn(&CombinedEffects) -> f64) -> f64 {
    rows.iter().map(|r| value(r)).fold(0.0, f64::max)
}

fn path(member: &SteelMember, leaf: &str) -> String {
    format!("members[id={}].{leaf}", member.id)
}

/// 🌀 The torsional buckling summary of the member under the largest compression of its ultimate rows; `None` without compression or torsional stiffness.
pub fn torsional_summary(document: &En1993Snapshot, member_id: &str) -> Option<TorsionalSummary> {
    let member = document.members.iter().find(|m| m.id == member_id)?;
    let (section, material) = resolved(document, member)?;
    let params = AnnexParams::for_choice(document.annex);
    let combos = combine_member_actions(document, &member.id);
    let n_ed = peak(&uls_rows(&combos), |r| r.action.n);
    if n_ed <= 0.0 || section.it <= 0.0 || section.area <= 0.0 {
        return None;
    }
    let (_, a_res) = class_and_area(section, material);
    let l_t = member.buckling_length_z.max(1e-9);
    let i0_sq = (section.iy + section.iz) / section.area;
    let n_cr_t = clauses::stability::n_cr_torsional(material.g_modulus, section.it, material.e_modulus, section.iw, l_t, i0_sq);
    let n_cr_z = clauses::stability::n_cr(material.e_modulus * section.iz, l_t);
    let (lambda_t, lambda_z) = ((a_res * material.fy / n_cr_t).sqrt(), (a_res * material.fy / n_cr_z).sqrt());
    let curve = part_1_1::buckling_curve_rolled_i(section, 'z', material.fy);
    let chi_t = part_1_1::chi(lambda_t, curve);
    Some(TorsionalSummary { n_ed, n_cr_t, n_cr_z, lambda_t, lambda_z, alpha: curve.alpha(), chi_t, n_b_rd_t: part_1_1::buckling_resistance_n(a_res, material.fy, chi_t, params) })
}

fn check_torsional(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, member: &SteelMember, section: &SteelSection, material: &SteelMaterial, report: &mut CheckReport) {
    let Some(s) = torsional_summary(document, &member.id) else { return };
    let mut b = CheckResult::assess(
        format!("en1993.6.3.1.tf.{}", member.id),
        "DIN EN 1993-1-1",
        ClauseId::new("EN 1993-1-1", "6", "6.3.1.4"),
        subject_member(member, &path(member, "bucklingLengthZ")),
        loc("Torsional and torsional-flexural buckling", "Drill- und Biegedrillknicken"),
    )
    .annex(annex)
    .ndp("ndp.en1993.v-101.1")
    .ndps(&[AnnexParams::GAMMA_M1])
    .utilization(force_n(s.n_ed), force_n(s.n_b_rd_t))
    .explanation(loc(
        &format!("N_Ed={:.1} kN, N_cr,T={:.0} kN (G·I_t + π²E·I_w/l_T² over i_0², l_T = minor-axis buckling length), N_cr,z={:.0} kN; λ̄_T={:.3} (λ̄_z={:.3}), χ_T={:.3} on the z-z curve (α={:.2}), N_b,Rd,T={:.1} kN. Doubly symmetric section: y_0 = 0, so N_cr,TF = min(N_cr,y, N_cr,T) and the flexural check governs unless λ̄_T > λ̄_z.", s.n_ed / 1e3, s.n_cr_t / 1e3, s.n_cr_z / 1e3, s.lambda_t, s.lambda_z, s.chi_t, s.alpha, s.n_b_rd_t / 1e3),
        &format!("N_Ed={:.1} kN, N_cr,T={:.0} kN (G·I_t + π²E·I_w/l_T² über i_0², l_T = Knicklänge um die schwache Achse), N_cr,z={:.0} kN; λ̄_T={:.3} (λ̄_z={:.3}), χ_T={:.3} auf der z-z-Linie (α={:.2}), N_b,Rd,T={:.1} kN. Doppeltsymmetrischer Querschnitt: y_0 = 0, daher N_cr,TF = min(N_cr,y, N_cr,T); maßgebend ist das Biegeknicken, solange λ̄_T ≤ λ̄_z.", s.n_ed / 1e3, s.n_cr_t / 1e3, s.n_cr_z / 1e3, s.lambda_t, s.lambda_z, s.chi_t, s.alpha, s.n_b_rd_t / 1e3),
    ));
    if s.n_ed > s.n_b_rd_t {
        let options = next_section_options(section, s.n_ed * params.gamma_m1 / (s.chi_t.max(1e-3) * material.fy), 0.0);
        if !options.is_empty() {
            b = b.remedy(Remedy::one_of(subject_member(member, &path(member, "sectionId")), options.clone(), loc(&format!("Upsize section: {}.", options.join(", ")), &format!("Querschnitt vergrößern: {}.", options.join(", ")))));
        }
        let half = 0.5 * member.buckling_length_z;
        b = b.remedy(Remedy::at_most(subject_member(member, &path(member, "bucklingLengthZ")), length_m(member.buckling_length_z), length_m(half), loc(&format!("Or add torsional restraints so that l_T is at most {:.2} m (first estimate, half the length).", half), &format!("Oder Torsionshalterungen anordnen, sodass l_T höchstens {:.2} m beträgt (erste Schätzung, halbe Länge).", half))));
    }
    report.push(b.build());
}

fn check_shear_area(annex: AnnexChoice, member: &SteelMember, section: &SteelSection, material: &SteelMaterial, report: &mut CheckReport) {
    let hw = section.h - 2.0 * section.tf;
    let eta = clauses::resistance::eta_shear(material.fy / 1.0e6);
    let a_v = match section.kind {
        semio_s_artifact_norm_en1993_model::SectionKind::RolledI => clauses::resistance::av_rolled_i(section.area, section.b, section.tf, section.tw, section.r, hw, eta),
        semio_s_artifact_norm_en1993_model::SectionKind::WeldedI => clauses::resistance::av_welded_web(hw, section.tw, eta),
        semio_s_artifact_norm_en1993_model::SectionKind::Chs => clauses::resistance::av_chs(section.area),
        semio_s_artifact_norm_en1993_model::SectionKind::Hss | semio_s_artifact_norm_en1993_model::SectionKind::Angle => return,
    };
    let excess = ((section.shear_area_z - a_v) / a_v).max(0.0);
    let mut b = CheckResult::assess(
        format!("en1993.6.2.6.av.{}", member.id),
        "DIN EN 1993-1-1",
        ClauseId::new("EN 1993-1-1", "6", "6.2.6(3)"),
        subject_member(member, &path(member, "sectionId")),
        loc("Shear area A_v of the section", "Schubfläche A_v des Querschnitts"),
    )
    .annex(annex)
    .ndp("ndp.en1993.v-050.1")
    .utilization(dimensionless(excess), dimensionless(0.03))
    .explanation(loc(
        &format!("Stored A_v,z={:.2} cm², A_v,z of 6.2.6(3) = {:.2} cm² (η={eta}, h_w={:.0} mm); the stored value may be conservative but must not exceed the formula by more than 3 %.", section.shear_area_z * 1e4, a_v * 1e4, hw * 1e3),
        &format!("Hinterlegte A_v,z={:.2} cm², A_v,z nach 6.2.6(3) = {:.2} cm² (η={eta}, h_w={:.0} mm); der Wert darf konservativ sein, die Formel aber um höchstens 3 % überschreiten.", section.shear_area_z * 1e4, a_v * 1e4, hw * 1e3),
    ));
    if excess > 0.03 {
        b = b.remedy(Remedy::at_most(
            SubjectRef::new(&section.id, format!("sections[id={}].shearAreaZ", section.id), loc("Shear area A_v,z", "Schubfläche A_v,z")),
            area_m2(section.shear_area_z),
            area_m2(a_v),
            loc(&format!("Reduce A_v,z to at most {:.2} cm².", a_v * 1e4), &format!("A_v,z auf höchstens {:.2} cm² verringern.", a_v * 1e4)),
        ));
    }
    report.push(b.build());
}

fn curve_index(curve: part_1_1::BucklingCurve) -> usize {
    match curve {
        part_1_1::BucklingCurve::A0 => 0,
        part_1_1::BucklingCurve::A => 1,
        part_1_1::BucklingCurve::B => 2,
        part_1_1::BucklingCurve::C => 3,
        part_1_1::BucklingCurve::D => 4,
    }
}

fn check_bow(annex: AnnexChoice, member: &SteelMember, section: &SteelSection, material: &SteelMaterial, a_res: f64, n_ed: f64, report: &mut CheckReport) {
    let plastic = member.analysis == semio_s_artifact_norm_en1993_model::AnalysisKind::Plastic;
    let (iy, iz) = (part_1_1::radius_of_gyration(section.iy, section.area), part_1_1::radius_of_gyration(section.iz, section.area));
    let lambda_y = part_1_1::lambda_bar(member.buckling_length_y, iy, material.e_modulus, material.fy);
    let lambda_z = part_1_1::lambda_bar(member.buckling_length_z, iz, material.e_modulus, material.fy);
    let (lambda, axis) = if lambda_z >= lambda_y { (lambda_z, 'z') } else { (lambda_y, 'y') };
    let index = curve_index(part_1_1::buckling_curve_rolled_i(section, axis, material.fy));
    let ratio = clauses::global::bow_ratio(index, plastic);
    let e0 = ratio * member.length;
    let required = clauses::global::bow_must_be_included(lambda, a_res, material.fy, n_ed);
    let e0_lt = 0.5 * e0;
    let table_row = format!("ndp.en1993.1-1.t51.{}.{}", ["a0", "a", "b", "c", "d"][index], if plastic { "plastic" } else { "elastic" });
    report.push(
        CheckResult::assess(
            format!("en1993.5.3.2.bow.{}", member.id),
            "DIN EN 1993-1-1",
            ClauseId::new("EN 1993-1-1", "5", "5.3.2(3)b"),
            subject_member(member, &path(member, "bucklingLengthZ")),
            loc("Bow imperfection e_0 of the member", "Vorkrümmung e_0 des Bauteils"),
        )
        .annex(annex)
        .ndps(&[table_row.as_str(), "ndp.en1993.v-024.1", "ndp.en1993.v-026.1"])
        .informative(length_m(e0))
        .explanation(loc(
            &format!("e_0=L/{:.0}={:.1} mm ({} analysis, buckling axis {axis}); the bow must {}be included in the global analysis (λ̄={lambda:.3} {} 0.5·√(A·f_y/N_Ed)={:.3}); for lateral-torsional buckling with a second-order torsional analysis k·e_0 = 0.5·e_0 = {:.1} mm (5.3.4).", 1.0 / ratio, e0 * 1e3, if plastic { "plastic" } else { "elastic" }, if required { "" } else { "not " }, if required { ">" } else { "≤" }, 0.5 * (a_res * material.fy / n_ed).sqrt(), e0_lt * 1e3),
            &format!("e_0=L/{:.0}={:.1} mm ({} Berechnung, Knickachse {axis}); die Vorkrümmung ist in der globalen Berechnung {}zu berücksichtigen (λ̄={lambda:.3} {} 0,5·√(A·f_y/N_Ed)={:.3}); beim Biegedrillknicken mit Torsion nach Theorie II. Ordnung k·e_0 = 0,5·e_0 = {:.1} mm (5.3.4).", 1.0 / ratio, e0 * 1e3, if plastic { "plastisch" } else { "elastisch" }, if required { "" } else { "nicht " }, if required { ">" } else { "≤" }, 0.5 * (a_res * material.fy / n_ed).sqrt(), e0_lt * 1e3),
        ))
        .build(),
    );
}

fn moment_kc(member: &SteelMember) -> f64 {
    match member.moment_diagram {
        semio_s_artifact_norm_en1993_model::MomentDiagram::Uniform => clauses::stability::kc_psi(1.0),
        semio_s_artifact_norm_en1993_model::MomentDiagram::Parabolic => clauses::stability::kc_udl_fork(),
        _ => clauses::stability::kc_psi(member.end_moment_ratio_psi),
    }
}

fn check_rolled_ltb(annex: AnnexChoice, params: AnnexParams, member: &SteelMember, section: &SteelSection, material: &SteelMaterial, class: u8, my_ed: f64, report: &mut CheckReport) {
    let rolled = section.kind != semio_s_artifact_norm_en1993_model::SectionKind::WeldedI;
    let w_y = part_1_1::bending_modulus_y(section, class, material.fy);
    let curve = clauses::stability::ltb_curve_table_6_5(rolled, section.h / section.b.max(1e-9));
    let m_cr = part_1_1::m_cr_rolled_nm(section, material, member.ltb_length.max(member.ltb_restraint_spacing), member.end_moment_ratio_psi, member.load_application.as_str(), member.moment_diagram.as_str());
    let lambda_lt = part_1_1::lambda_lt(w_y, material.fy, m_cr);
    let chi = clauses::stability::chi_lt_rolled(lambda_lt, curve.alpha(), 0.4, 0.75, my_ed, m_cr);
    let (kc, f) = (moment_kc(member), clauses::stability::ltb_f(moment_kc(member), lambda_lt));
    let chi_mod = clauses::stability::chi_lt_mod(chi, f);
    let m_b_rd = part_1_1::ltb_resistance_nm(w_y, material.fy, chi_mod, params);
    let mut b = CheckResult::assess(
        format!("en1993.6.3.2.3.rolled.{}", member.id),
        "DIN EN 1993-1-1",
        ClauseId::new("EN 1993-1-1", "6", "6.3.2.3"),
        subject_member(member, &path(member, "ltbLength")),
        loc("Lateral-torsional buckling, rolled and equivalent welded sections", "Biegedrillknicken, Walzprofile und gleichartige geschweißte Querschnitte"),
    )
    .annex(annex)
    .ndps(&["ndp.en1993.v-111.1", "ndp.en1993.v-111.2", AnnexParams::GAMMA_M1])
    .utilization(moment_nm(my_ed), moment_nm(m_b_rd))
    .explanation(loc(
        &format!("M_y,Ed={:.1} kNm, M_cr={:.0} kNm, λ̄_LT={lambda_lt:.3} (λ̄_LT,0=0.4, β=0.75, Table 6.5 α={:.2}), χ_LT={chi:.3}, k_c={kc:.3}, f={f:.3}, χ_LT,mod={chi_mod:.3}, M_b,Rd={:.1} kNm.", my_ed / 1e3, m_cr / 1e3, curve.alpha(), m_b_rd / 1e3),
        &format!("M_y,Ed={:.1} kNm, M_cr={:.0} kNm, λ̄_LT={lambda_lt:.3} (λ̄_LT,0=0,4, β=0,75, Tabelle 6.5 α={:.2}), χ_LT={chi:.3}, k_c={kc:.3}, f={f:.3}, χ_LT,mod={chi_mod:.3}, M_b,Rd={:.1} kNm.", my_ed / 1e3, m_cr / 1e3, curve.alpha(), m_b_rd / 1e3),
    ));
    if my_ed > m_b_rd {
        let options = next_section_options(section, section.area, my_ed * params.gamma_m1 / (chi_mod.max(1e-3) * material.fy));
        if !options.is_empty() {
            b = b.remedy(Remedy::one_of(subject_member(member, &path(member, "sectionId")), options.clone(), loc(&format!("Upsize section: {}.", options.join(", ")), &format!("Querschnitt vergrößern: {}.", options.join(", ")))));
        }
        let spacing = if member.ltb_restraint_spacing > 0.0 { member.ltb_restraint_spacing } else { member.ltb_length };
        b = b.remedy(Remedy::at_most(subject_member(member, &path(member, "ltbRestraintSpacing")), length_m(spacing), length_m(0.5 * spacing), loc(&format!("Or halve the distance between lateral restraints to {:.2} m.", 0.5 * spacing), &format!("Oder den Abstand der seitlichen Halterungen auf {:.2} m halbieren.", 0.5 * spacing))));
    }
    report.push(b.build());
}

fn check_simplified_ltb(annex: AnnexChoice, params: AnnexParams, member: &SteelMember, section: &SteelSection, material: &SteelMaterial, w_y: f64, my_ed: f64, report: &mut CheckReport) {
    let (b_f, t_f, t_w, h_w) = (section.b * 1e3, section.tf * 1e3, section.tw * 1e3, (section.h - 2.0 * section.tf) * 1e3);
    let a_f = b_f * t_f + h_w * t_w / 6.0;
    let i_f = t_f * b_f.powi(3) / 12.0 + h_w / 6.0 * t_w.powi(3) / 12.0;
    let i_fz = (i_f / a_f).sqrt();
    let l_c = (if member.ltb_restraint_spacing > 0.0 { member.ltb_restraint_spacing } else { member.ltb_length }) * 1e3;
    let kc = moment_kc(member);
    let lambda_c0 = 0.4 + 0.1;
    let mc_rd = part_1_1::bending_resistance_nm(w_y, material.fy, 1, params);
    let utilisation = clauses::stability::simplified_compression_flange_utilisation(kc, l_c, i_fz, material.fy / 1.0e6, lambda_c0, mc_rd, my_ed);
    let lambda_f = kc * l_c / (i_fz * clauses::stability::lambda_1(material.fy / 1.0e6));
    let mut b = CheckResult::assess(
        format!("en1993.6.3.2.4.cf.{}", member.id),
        "DIN EN 1993-1-1",
        ClauseId::new("EN 1993-1-1", "6", "6.3.2.4"),
        subject_member(member, &path(member, "ltbRestraintSpacing")),
        loc("Simplified LTB check, compression flange as column", "Vereinfachter Biegedrillknicknachweis, Druckflansch als Druckstab"),
    )
    .annex(annex)
    .ndps(&["ndp.en1993.v-112.1", AnnexParams::GAMMA_M0])
    .utilization(dimensionless(utilisation), dimensionless(1.0))
    .explanation(loc(
        &format!("λ̄_f=k_c·L_c/(i_f,z·λ_1)={lambda_f:.3} (k_c={kc:.3}, L_c={l_c:.0} mm, i_f,z={i_fz:.1} mm of the flange plus 1/6 of the web) against λ̄_c0·M_c,Rd/M_y,Ed={:.3} (λ̄_c0={lambda_c0}, M_c,Rd={:.1} kNm, M_y,Ed={:.1} kNm).", lambda_c0 * mc_rd / my_ed, mc_rd / 1e3, my_ed / 1e3),
        &format!("λ̄_f=k_c·L_c/(i_f,z·λ_1)={lambda_f:.3} (k_c={kc:.3}, L_c={l_c:.0} mm, i_f,z={i_fz:.1} mm für Flansch plus 1/6 Steg) gegenüber λ̄_c0·M_c,Rd/M_y,Ed={:.3} (λ̄_c0={lambda_c0}, M_c,Rd={:.1} kNm, M_y,Ed={:.1} kNm).", lambda_c0 * mc_rd / my_ed, mc_rd / 1e3, my_ed / 1e3),
    ));
    if utilisation > 1.0 {
        b = b.remedy(Remedy::at_most(
            subject_member(member, &path(member, "ltbRestraintSpacing")),
            length_m(l_c / 1e3),
            length_m(l_c / 1e3 / utilisation),
            loc(&format!("Reduce the distance between lateral restraints to at most {:.0} mm.", l_c / utilisation), &format!("Abstand der seitlichen Halterungen auf höchstens {:.0} mm verringern.", l_c / utilisation)),
        ));
    }
    report.push(b.build());
}

fn check_frequency(document: &En1993Snapshot, annex: AnnexChoice, member: &SteelMember, section: &SteelSection, material: &SteelMaterial, report: &mut CheckReport) {
    let permanent: f64 = document
        .member_actions
        .iter()
        .filter(|a| a.member_id == member.id && document.load_cases.iter().any(|c| c.id == a.load_case_id && c.kind == semio_s_artifact_norm_en1993_model::ActionKind::Permanent))
        .map(|a| a.action.my.abs())
        .sum();
    if member.moment_diagram != semio_s_artifact_norm_en1993_model::MomentDiagram::Parabolic || permanent <= 0.0 || member.length <= 0.0 {
        return;
    }
    let mass = 8.0 * permanent / (member.length * member.length) / 9.81;
    let f1 = clauses::misc::floor_frequency(member.length, material.e_modulus * section.iy, mass);
    let mut b = CheckResult::assess(
        format!("en1993.7.2.3.freq.{}", member.id),
        "DIN EN 1993-1-1",
        ClauseId::new("EN 1993-1-1", "7", "7.2.3"),
        subject_member(member, &path(member, "length")),
        loc("Natural frequency of the floor beam", "Eigenfrequenz des Deckenträgers"),
    )
    .annex(annex)
    .ndp("ndp.en1993.v-091.1")
    .minimum(dimensionless(f1), dimensionless(3.0))
    .explanation(loc(
        &format!("f_1=π/(2L²)·√(EI/m)={f1:.2} Hz (EI={:.0} kNm², m={mass:.0} kg/m from the permanent moment {:.1} kNm of a simply supported beam under uniform load) against the recommended minimum 3 Hz.", material.e_modulus * section.iy / 1e3, permanent / 1e3),
        &format!("f_1=π/(2L²)·√(EI/m)={f1:.2} Hz (EI={:.0} kNm², m={mass:.0} kg/m aus dem ständigen Moment {:.1} kNm eines Einfeldträgers unter Gleichlast) gegenüber dem empfohlenen Mindestwert 3 Hz.", material.e_modulus * section.iy / 1e3, permanent / 1e3),
    ));
    if f1 < 3.0 {
        let options = next_section_options(section, section.area, section.w_pl_y * (3.0 / f1.max(1e-3)).powi(2));
        if !options.is_empty() {
            b = b.remedy(Remedy::one_of(subject_member(member, &path(member, "sectionId")), options.clone(), loc(&format!("Stiffen the section: {}.", options.join(", ")), &format!("Querschnitt steifer wählen: {}.", options.join(", ")))));
        }
        let l_max = member.length * (f1 / 3.0).max(0.0).sqrt();
        b = b.remedy(Remedy::at_most(subject_member(member, &path(member, "length")), length_m(member.length), length_m(l_max), loc(&format!("Or shorten the span to at most {:.2} m (f_1 is proportional to 1/L²).", l_max), &format!("Oder die Spannweite auf höchstens {:.2} m verkürzen (f_1 ist proportional zu 1/L²).", l_max))));
    }
    report.push(b.build());
}

/// 🧱️ Appends the member catalogue-gap checks of every admitted member.
pub(super) fn check_members(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, scope: Scope<'_>, report: &mut CheckReport) {
    for member in document.members.iter().filter(|m| scope.admits("vidMembers", &m.id)) {
        let Some((section, material)) = resolved(document, member) else { continue };
        let kind = member_kind(member.member_type.as_str());
        let (class, a_res) = class_and_area(section, material);
        let combos = combine_member_actions(document, &member.id);
        let rows = uls_rows(&combos);
        let (n_ed, my_ed) = (peak(&rows, |r| r.action.n), peak(&rows, |r| r.action.my.abs()));
        check_shear_area(annex, member, section, material, report);
        if kind_allows_flexural_buckling(&kind) {
            check_torsional(document, annex, params, member, section, material, report);
            if n_ed > 0.0 {
                check_bow(annex, member, section, material, a_res, n_ed, report);
            }
        }
        if kind_allows_ltb(&kind) && my_ed > 0.0 && part_1_1::is_i_section(section.kind) {
            check_rolled_ltb(annex, params, member, section, material, class, my_ed, report);
            if class <= 3 {
                check_simplified_ltb(annex, params, member, section, material, part_1_1::bending_modulus_y(section, class, material.fy), my_ed, report);
            }
        }
        if kind_allows_bending(&kind) {
            check_frequency(document, annex, member, section, material, report);
        }
    }
}
