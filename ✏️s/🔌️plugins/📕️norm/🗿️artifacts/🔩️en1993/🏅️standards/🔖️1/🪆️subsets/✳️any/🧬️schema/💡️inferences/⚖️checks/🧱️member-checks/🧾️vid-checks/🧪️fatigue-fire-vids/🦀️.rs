//! 🧪️ Material, fatigue, fire and cold-formed catalogue-gap checks: grade, ductility and constants (EN 1993-1-1 3.2), constant-amplitude fatigue (EN 1993-1-9), LTB and bending with axial force in fire (EN 1993-1-2 4.2.3) and flange curling (EN 1993-1-3 5.4). See [`super`].

use super::*;
use semio_s_artifact_norm_en1993_model::{ColdFormedMember, FatigueDetail, FireExposure};

const NOMINAL: [(&str, f64, f64); 4] = [("S235", 235.0, 360.0), ("S275", 275.0, 430.0), ("S355", 355.0, 510.0), ("S450", 440.0, 550.0)];

fn material_subject(m: &SteelMaterial, leaf: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(&m.id, format!("materials[id={}].{leaf}", m.id), loc(en, de))
}

fn check_grade(annex: AnnexChoice, m: &SteelMaterial, report: &mut CheckReport) {
    let grade = m.grade.to_ascii_uppercase();
    let Some((name, fy_n, fu_n)) = NOMINAL.iter().find(|(g, _, _)| grade.starts_with(g)).copied() else { return };
    let (fy_n, fu_n) = (fy_n * 1e6, fu_n * 1e6);
    let ratio = ((m.fy / fy_n).max(m.fu / fu_n) * 1e9).round() / 1e9;
    let mut b = CheckResult::assess(
        format!("en1993.3.2.1.grade.{}", m.id),
        "DIN EN 1993-1-1",
        ClauseId::new("EN 1993-1-1", "3", "3.2.1/Table 3.1"),
        material_subject(m, "fy", "Yield strength", "Streckgrenze"),
        loc("Strength against Table 3.1", "Festigkeit gegenüber Tabelle 3.1"),
    )
    .annex(annex)
    .ndp("ndp.en1993.v-010.1")
    .utilization(dimensionless(ratio), dimensionless(1.0))
    .explanation(loc(
        &format!("{name} (t ≤ 40 mm): f_y={:.0} MPa against {:.0} MPa, f_u={:.0} MPa against {:.0} MPa; the declared strengths must not exceed Table 3.1.", m.fy / 1e6, fy_n / 1e6, m.fu / 1e6, fu_n / 1e6),
        &format!("{name} (t ≤ 40 mm): f_y={:.0} MPa gegenüber {:.0} MPa, f_u={:.0} MPa gegenüber {:.0} MPa; die angesetzten Festigkeiten dürfen Tabelle 3.1 nicht überschreiten.", m.fy / 1e6, fy_n / 1e6, m.fu / 1e6, fu_n / 1e6),
    ));
    if m.fy > fy_n {
        b = b.remedy(Remedy::at_most(material_subject(m, "fy", "Yield strength", "Streckgrenze"), stress_pa(m.fy), stress_pa(fy_n), loc(&format!("Reduce f_y to at most {:.0} MPa.", fy_n / 1e6), &format!("f_y auf höchstens {:.0} MPa verringern.", fy_n / 1e6))));
    }
    if m.fu > fu_n {
        b = b.remedy(Remedy::at_most(material_subject(m, "fu", "Ultimate strength", "Zugfestigkeit"), stress_pa(m.fu), stress_pa(fu_n), loc(&format!("Reduce f_u to at most {:.0} MPa.", fu_n / 1e6), &format!("f_u auf höchstens {:.0} MPa verringern.", fu_n / 1e6))));
    }
    report.push(b.build());
}

fn check_ductility(document: &En1993Snapshot, annex: AnnexChoice, m: &SteelMaterial, report: &mut CheckReport) {
    let ratio = m.fu / m.fy.max(1e-9);
    let limit = semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-1.ductility.fu-fy");
    let certificate = document.material_ductility.iter().any(|d| d.material_id == m.id);
    let mut b = CheckResult::assess(
        format!("en1993.3.2.2.ductility.{}", m.id),
        "DIN EN 1993-1-1",
        ClauseId::new("EN 1993-1-1", "3", "3.2.2"),
        material_subject(m, "fu", "Ultimate strength", "Zugfestigkeit"),
        loc("Ductility f_u/f_y", "Duktilität f_u/f_y"),
    )
    .annex(annex)
    .ndp("ndp.en1993.v-011.1")
    .minimum(dimensionless(ratio), dimensionless(limit))
    .explanation(if certificate {
        loc(&format!("f_u/f_y={ratio:.3} against {limit:.2}; the elongation at failure and ε_u ≥ 15 ε_y follow from the ductility row of the material."), &format!("f_u/f_y={ratio:.3} gegenüber {limit:.2}; Bruchdehnung und ε_u ≥ 15 ε_y folgen aus der Duktilitätszeile des Werkstoffs."))
    } else {
        loc(&format!("f_u/f_y={ratio:.3} against {limit:.2}; the elongation at failure (15 %) and ε_u ≥ 15 ε_y cannot be checked, no ductility row (materialDuctility) names this material."), &format!("f_u/f_y={ratio:.3} gegenüber {limit:.2}; die Bruchdehnung (15 %) und ε_u ≥ 15 ε_y sind nicht prüfbar, keine Duktilitätszeile (materialDuctility) nennt diesen Werkstoff."))
    });
    if ratio < limit {
        b = b.remedy(Remedy::at_least(material_subject(m, "fu", "Ultimate strength", "Zugfestigkeit"), stress_pa(m.fu), stress_pa(limit * m.fy), loc(&format!("Choose a grade with f_u ≥ {:.0} MPa.", limit * m.fy / 1e6), &format!("Güte mit f_u ≥ {:.0} MPa wählen.", limit * m.fy / 1e6))));
    }
    report.push(b.build());
}

fn check_certificate(document: &En1993Snapshot, annex: AnnexChoice, m: &SteelMaterial, report: &mut CheckReport) {
    let Some(row) = document.material_ductility.iter().find(|d| d.material_id == m.id) else { return };
    let rule = semio_s_plugin_norm_compliance::ndp_common;
    let at = |leaf: &str, en: &str, de: &str| SubjectRef::new(&row.id, format!("materialDuctility[id={}].{leaf}", row.id), loc(en, de));
    if row.elongation > 0.0 {
        let limit = rule("ndp.en1993.1-1.ductility.elongation");
        let mut b = CheckResult::assess(format!("en1993.3.2.2.elongation.{}", row.id), "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "3", "3.2.2"), at("elongation", "Elongation at failure", "Bruchdehnung"), loc("Elongation at failure", "Bruchdehnung"))
            .annex(annex)
            .ndp("ndp.en1993.v-011.1")
            .minimum(dimensionless(row.elongation), dimensionless(limit))
            .explanation(loc(&format!("Elongation at failure {:.1} % against {:.0} %.", row.elongation * 100.0, limit * 100.0), &format!("Bruchdehnung {:.1} % gegenüber {:.0} %.", row.elongation * 100.0, limit * 100.0)));
        if row.elongation < limit {
            b = b.remedy(Remedy::at_least(at("elongation", "Elongation at failure", "Bruchdehnung"), dimensionless(row.elongation), dimensionless(limit), loc(&format!("Choose a material with an elongation of at least {:.0} %.", limit * 100.0), &format!("Einen Werkstoff mit mindestens {:.0} % Bruchdehnung wählen.", limit * 100.0))));
        }
        report.push(b.build());
    }
    if row.ultimate_strain > 0.0 && m.e_modulus > 0.0 {
        let required = rule("ndp.en1993.1-1.ductility.eps-u-factor") * m.fy / m.e_modulus;
        let mut b = CheckResult::assess(format!("en1993.3.2.2.strain.{}", row.id), "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "3", "3.2.2"), at("ultimateStrain", "Ultimate strain", "Gleichmaßdehnung"), loc("Ultimate strain ε_u", "Gleichmaßdehnung ε_u"))
            .annex(annex)
            .ndp("ndp.en1993.v-011.1")
            .minimum(dimensionless(row.ultimate_strain), dimensionless(required))
            .explanation(loc(&format!("ε_u={:.4} against 15 ε_y={:.4} (ε_y=f_y/E={:.5}).", row.ultimate_strain, required, m.fy / m.e_modulus), &format!("ε_u={:.4} gegenüber 15 ε_y={:.4} (ε_y=f_y/E={:.5}).", row.ultimate_strain, required, m.fy / m.e_modulus)));
        if row.ultimate_strain < required {
            b = b.remedy(Remedy::at_least(at("ultimateStrain", "Ultimate strain", "Gleichmaßdehnung"), dimensionless(row.ultimate_strain), dimensionless(required), loc(&format!("Choose a material with ε_u ≥ {required:.4}."), &format!("Einen Werkstoff mit ε_u ≥ {required:.4} wählen."))));
        }
        report.push(b.build());
    }
}

fn check_constants(annex: AnnexChoice, m: &SteelMaterial, report: &mut CheckReport) {
    let g_nominal = m.e_modulus / (2.0 * (1.0 + 0.3));
    let deviation = ((m.e_modulus - 210.0e9).abs() / 210.0e9).max((m.g_modulus - g_nominal).abs() / g_nominal);
    let mut b = CheckResult::assess(
        format!("en1993.3.2.6.constants.{}", m.id),
        "DIN EN 1993-1-1",
        ClauseId::new("EN 1993-1-1", "3", "3.2.6"),
        material_subject(m, "eModulus", "Modulus of elasticity", "Elastizitätsmodul"),
        loc("Design values of the material constants", "Rechenwerte der Werkstoffkonstanten"),
    )
    .annex(annex)
    .ndp("ndp.en1993.v-013.1")
    .utilization(dimensionless(deviation), dimensionless(0.01))
    .explanation(loc(
        &format!("E={:.0} MPa (nominal 210000 MPa), G={:.0} MPa (E/(2(1+ν)) = {:.0} MPa with ν=0.3); largest deviation {:.2} % against the 1 % tolerance.", m.e_modulus / 1e6, m.g_modulus / 1e6, g_nominal / 1e6, deviation * 100.0),
        &format!("E={:.0} MPa (nominal 210000 MPa), G={:.0} MPa (E/(2(1+ν)) = {:.0} MPa mit ν=0,3); größte Abweichung {:.2} % gegenüber der Toleranz 1 %.", m.e_modulus / 1e6, m.g_modulus / 1e6, g_nominal / 1e6, deviation * 100.0),
    ));
    if deviation > 0.01 {
        b = b.remedy(Remedy::one_of(material_subject(m, "eModulus", "Modulus of elasticity", "Elastizitätsmodul"), vec!["210000000000".into()], loc("Set E to 210000 MPa.", "E auf 210000 MPa setzen.")));
        b = b.remedy(Remedy::one_of(material_subject(m, "gModulus", "Shear modulus", "Schubmodul"), vec![format!("{:.0}", m.e_modulus / 2.6)], loc("Set G to E/2.6.", "G auf E/2,6 setzen.")));
    }
    report.push(b.build());
}

/// 🧪️ Appends the material grade, ductility and constants checks of every admitted carbon steel material.
pub(super) fn check_materials(document: &En1993Snapshot, annex: AnnexChoice, scope: Scope<'_>, report: &mut CheckReport) {
    for m in document.materials.iter().filter(|m| m.kind != semio_s_artifact_norm_en1993_model::MaterialKind::Stainless && scope.admits("vidMaterials", &m.id)) {
        check_grade(annex, m, report);
        check_ductility(document, annex, m, report);
        check_certificate(document, annex, m, report);
        check_constants(annex, m, report);
    }
}

/// 🔄️ Appends the constant-amplitude fatigue limit check (variable amplitude: informative ratio) of every admitted detail.
pub(super) fn check_fatigue(document: &En1993Snapshot, annex: AnnexChoice, scope: Scope<'_>, report: &mut CheckReport) {
    for fat in document.fatigue_details.iter().filter(|f| scope.admits("vidFatigue", &f.id)) {
        fatigue_limit(annex, fat, report);
    }
}

fn fatigue_limit(annex: AnnexChoice, fat: &FatigueDetail, report: &mut CheckReport) {
    let gmf = part_1_9::gamma_mf(part_1_9::parse_method(fat.method.as_str()));
    let delta_c = part_1_9::detail_category_pa(fat.category);
    let (delta_d, delta_l) = (clauses::fatigue_fire::delta_sigma_d(delta_c), clauses::fatigue_fire::delta_sigma_l(delta_c));
    let peak = fat.spectrum.iter().map(|b| b.delta_sigma).fold(0.0, f64::max);
    let below = fat.spectrum.iter().filter(|b| b.delta_sigma * gmf <= delta_l).count();
    let subject = |leaf: &str| SubjectRef::new(&fat.id, format!("fatigueDetails[id={}].{leaf}", fat.id), loc(&format!("Fatigue {}", fat.id), &format!("Ermüdung {}", fat.id)));
    let builder = CheckResult::assess(
        format!("en1993.1-9.6.cafl.{}", fat.id),
        "DIN EN 1993-1-9",
        ClauseId::new("EN 1993-1-9", "6", "6.3"),
        subject("category"),
        loc("Fatigue limit Δσ_D and cut-off Δσ_L of the S-N curve", "Dauerfestigkeit Δσ_D und Schwellenwert Δσ_L der Wöhlerlinie"),
    )
    .annex(annex)
    .ndps(&["ndp.en1993.v-502.1", AnnexParams::GAMMA_MF]);
    let (count, cycles_en) = (fat.spectrum.len(), "cycles");
    if count == 1 {
        let mut b = builder.utilization(stress_pa(peak * gmf), stress_pa(delta_d)).explanation(loc(
            &format!("Constant amplitude, γ_Ff·Δσ ≤ Δσ_D/γ_Mf: category Δσ_C={:.0} MPa, Δσ_D={:.1} MPa (5·10⁶ {cycles_en}), Δσ_L={:.1} MPa (10⁸ {cycles_en}), γ_Mf={gmf}, γ_Ff=1.0, range {:.1} MPa.", delta_c / 1e6, delta_d / 1e6, delta_l / 1e6, peak / 1e6),
            &format!("Konstante Amplitude, γ_Ff·Δσ ≤ Δσ_D/γ_Mf: Kerbfall Δσ_C={:.0} MPa, Δσ_D={:.1} MPa (5·10⁶ Lastspiele), Δσ_L={:.1} MPa (10⁸ Lastspiele), γ_Mf={gmf}, γ_Ff=1,0, Schwingbreite {:.1} MPa.", delta_c / 1e6, delta_d / 1e6, delta_l / 1e6, peak / 1e6),
        ));
        if peak * gmf > delta_d {
            let categories: Vec<String> = [80u8, 90, 100, 112, 125, 140, 160].iter().filter(|c| clauses::fatigue_fire::delta_sigma_d(part_1_9::detail_category_pa(**c)) >= peak * gmf).map(|c| c.to_string()).collect();
            if !categories.is_empty() {
                b = b.remedy(Remedy::one_of(subject("category"), categories.clone(), loc(&format!("Improve the detail category to one of: {}.", categories.join(", ")), &format!("Kerbfall verbessern auf eine von: {}.", categories.join(", ")))));
            }
            b = b.remedy(Remedy::at_most(subject("spectrum"), stress_pa(peak), stress_pa(delta_d / gmf), loc(&format!("Or reduce the stress range to at most {:.1} MPa.", delta_d / gmf / 1e6), &format!("Oder die Schwingbreite auf höchstens {:.1} MPa verringern.", delta_d / gmf / 1e6))));
        }
        report.push(b.build());
    } else {
        report.push(
            builder
                .informative(dimensionless(peak * gmf / delta_d))
                .explanation(loc(
                    &format!("Variable amplitude: the limit Δσ_D does not decide, the damage sum of en1993.1-9.fat does. Ratio γ_Mf·Δσ_max/Δσ_D={:.3}; category Δσ_C={:.0} MPa, Δσ_D={:.1} MPa, Δσ_L={:.1} MPa, γ_Mf={gmf}; {below} of {count} bands at or below the cut-off.", peak * gmf / delta_d, delta_c / 1e6, delta_d / 1e6, delta_l / 1e6),
                    &format!("Veränderliche Amplitude: nicht Δσ_D, sondern die Schädigungssumme von en1993.1-9.fat entscheidet. Verhältnis γ_Mf·Δσ_max/Δσ_D={:.3}; Kerbfall Δσ_C={:.0} MPa, Δσ_D={:.1} MPa, Δσ_L={:.1} MPa, γ_Mf={gmf}; {below} von {count} Stufen auf oder unter dem Schwellenwert.", peak * gmf / delta_d, delta_c / 1e6, delta_d / 1e6, delta_l / 1e6),
                ))
                .build(),
        );
    }
}

/// 🔥 LTB resistance in fire (EN 1993-1-2 4.2.3.3) and bending with axial force (4.2.3.5) of one fire exposure, with the M_b,fi,θ,Rd(θ) curve.
#[derive(Clone, Debug)]
pub struct FireLtbSummary {
    pub theta_a: f64,
    pub k_y: f64,
    pub k_e: f64,
    pub lambda_lt: f64,
    pub chi_lt_fi: f64,
    pub m_fi_ed: f64,
    pub m_b_fi_rd: f64,
    pub n_fi_ed: f64,
    pub interaction: Option<(f64, f64)>,
    pub curve: Vec<(f64, f64)>,
}

/// 🔥 The fire LTB summary of `fire_id` for class 1 to 3 members with a bending moment; `None` otherwise.
pub fn fire_ltb_summary(document: &En1993Snapshot, fire_id: &str) -> Option<FireLtbSummary> {
    let fire: &FireExposure = document.fire_exposures.iter().find(|f| f.id == fire_id)?;
    let member = document.members.iter().find(|m| m.id == fire.member_id)?;
    let section = find_section(document, &member.section_id)?;
    let material = find_material(document, &member.material_id)?;
    let class = part_1_1::section_class_rolled_i(section, material.fy);
    let combos = combine_member_actions(document, &member.id);
    let rows = uls_rows(&combos);
    let my_ed = rows.iter().map(|r| r.action.my.abs()).fold(0.0, f64::max);
    if class > 3 || my_ed <= 0.0 {
        return None;
    }
    let heating = part_1_2::steel_temperature_c(part_1_2::parse_rating(fire.rating.as_str()), fire.section_factor, fire.protection_thickness, fire.protection_conductivity, fire.protection_density, fire.protection_specific_heat);
    let theta_a = fire.design_temperature.max(heating);
    let w_y = part_1_1::bending_modulus_y(section, class, material.fy);
    let m_cr = part_1_1::m_cr_rolled_nm(section, material, member.ltb_length.max(member.ltb_restraint_spacing), member.end_moment_ratio_psi, member.load_application.as_str(), member.moment_diagram.as_str());
    let lambda_lt = part_1_1::lambda_lt(w_y, material.fy, m_cr);
    let m_b = |theta: f64| clauses::fatigue_fire::m_b_fi_t_rd(part_1_2::chi_fi(lambda_lt, theta, material.fy), w_y, part_1_2::k_y_theta(theta), material.fy, 1.0);
    let (k_y, k_e) = (part_1_2::k_y_theta(theta_a), part_1_2::k_e_theta(theta_a));
    let (m_fi_ed, n_fi_ed) = (0.65 * my_ed, 0.65 * rows.iter().map(|r| r.action.n).fold(0.0, f64::max));
    let interaction = (n_fi_ed > 0.0).then(|| {
        let (iy, iz) = (part_1_1::radius_of_gyration(section.iy, section.area), part_1_1::radius_of_gyration(section.iz, section.area));
        let (lambda_y, lambda_z) = (part_1_1::lambda_bar(member.buckling_length_y, iy, material.e_modulus, material.fy), part_1_1::lambda_bar(member.buckling_length_z, iz, material.e_modulus, material.fy));
        let (chi_y, chi_z) = (part_1_2::chi_fi(lambda_y, theta_a, material.fy), part_1_2::chi_fi(lambda_z, theta_a, material.fy));
        let shift = (k_y / k_e.max(1e-9)).sqrt();
        let (area, fy) = (section.area, material.fy);
        let k_y_int = clauses::fatigue_fire::k_fire(clauses::fatigue_fire::mu_y_fire(1.3, lambda_y * shift), n_fi_ed, chi_y, area, k_y, fy, 1.0, 3.0);
        let k_lt_int = clauses::fatigue_fire::k_fire(clauses::fatigue_fire::mu_lt_fire(lambda_z * shift, 1.3), n_fi_ed, chi_z, area, k_y, fy, 1.0, 1.0);
        let m_rd_lt = m_b(theta_a);
        clauses::fatigue_fire::bending_axial_fire(n_fi_ed, chi_y.min(chi_z) * area * k_y * fy, chi_z * area * k_y * fy, m_fi_ed, m_rd_lt, k_y_int, k_lt_int)
    });
    Some(FireLtbSummary { theta_a, k_y, k_e, lambda_lt, chi_lt_fi: part_1_2::chi_fi(lambda_lt, theta_a, material.fy), m_fi_ed, m_b_fi_rd: m_b(theta_a), n_fi_ed, interaction, curve: (1..=50).map(|i| (20.0 * i as f64, m_b(20.0 * i as f64))).collect() })
}

/// 🔥 Appends the LTB and the bending with axial force checks in fire of every admitted fire exposure.
pub(super) fn check_fire(document: &En1993Snapshot, annex: AnnexChoice, scope: Scope<'_>, report: &mut CheckReport) {
    for fire in document.fire_exposures.iter().filter(|f| scope.admits("vidFire", &f.id)) {
        let Some(s) = fire_ltb_summary(document, &fire.id) else { continue };
        let subject = || SubjectRef::new(&fire.id, format!("fireExposures[id={}].designTemperature", fire.id), loc(&format!("Fire {}", fire.id), &format!("Brand {}", fire.id)));
        let mut b = CheckResult::assess(
            format!("en1993.1-2.4.2.3.ltb.{}", fire.id),
            "DIN EN 1993-1-2",
            ClauseId::new("EN 1993-1-2", "4", "4.2.3.3"),
            subject(),
            loc("Lateral-torsional buckling in fire", "Biegedrillknicken im Brandfall"),
        )
        .annex(annex)
        .ndp("ndp.en1993.v-607.1")
        .utilization(moment_nm(s.m_fi_ed), moment_nm(s.m_b_fi_rd))
        .explanation(loc(
            &format!("M_fi,Ed=0.65·M_Ed={:.1} kNm, M_b,fi,t,Rd=χ_LT,fi·W_y·k_y,θ·f_y={:.1} kNm (θ_a={:.0} °C, k_y,θ={:.3}, k_E,θ={:.3}, λ̄_LT={:.3}, χ_LT,fi={:.3}).", s.m_fi_ed / 1e3, s.m_b_fi_rd / 1e3, s.theta_a, s.k_y, s.k_e, s.lambda_lt, s.chi_lt_fi),
            &format!("M_fi,Ed=0,65·M_Ed={:.1} kNm, M_b,fi,t,Rd=χ_LT,fi·W_y·k_y,θ·f_y={:.1} kNm (θ_a={:.0} °C, k_y,θ={:.3}, k_E,θ={:.3}, λ̄_LT={:.3}, χ_LT,fi={:.3}).", s.m_fi_ed / 1e3, s.m_b_fi_rd / 1e3, s.theta_a, s.k_y, s.k_e, s.lambda_lt, s.chi_lt_fi),
        ));
        if s.m_fi_ed > s.m_b_fi_rd {
            let allowed = s.curve.iter().filter(|(_, m)| *m >= s.m_fi_ed).map(|(t, _)| *t).fold(20.0, f64::max);
            b = b.remedy(Remedy::at_most(subject(), temperature_c(s.theta_a), temperature_c(allowed), loc(&format!("Limit the steel temperature to {:.0} °C by thicker fire protection.", allowed), &format!("Stahltemperatur durch dickeren Brandschutz auf {:.0} °C begrenzen.", allowed))));
        }
        report.push(b.build());
        let Some((eq_a, eq_b)) = s.interaction else { continue };
        let utilisation = eq_a.max(eq_b);
        let mut b = CheckResult::assess(
            format!("en1993.1-2.4.2.3.int.{}", fire.id),
            "DIN EN 1993-1-2",
            ClauseId::new("EN 1993-1-2", "4", "4.2.3.5"),
            subject(),
            loc("Bending and axial compression in fire", "Biegung und Druckkraft im Brandfall"),
        )
        .annex(annex)
        .ndp("ndp.en1993.v-608.1")
        .utilization(dimensionless(utilisation), dimensionless(1.0))
        .explanation(loc(
            &format!("Eq. 4.21a={eq_a:.3}, Eq. 4.21b={eq_b:.3} for N_fi,Ed={:.1} kN and M_fi,Ed={:.1} kNm at θ_a={:.0} °C (β_M=1.3, strong-axis bending).", s.n_fi_ed / 1e3, s.m_fi_ed / 1e3, s.theta_a),
            &format!("Gl. 4.21a={eq_a:.3}, Gl. 4.21b={eq_b:.3} für N_fi,Ed={:.1} kN und M_fi,Ed={:.1} kNm bei θ_a={:.0} °C (β_M=1,3, Biegung um die starke Achse).", s.n_fi_ed / 1e3, s.m_fi_ed / 1e3, s.theta_a),
        ));
        if utilisation > 1.0 {
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&fire.id, format!("fireExposures[id={}].mu0", fire.id), loc("Load level μ0", "Ausnutzung μ0")),
                dimensionless(fire.mu0),
                dimensionless(fire.mu0 / utilisation),
                loc(&format!("Reduce the fire load level μ₀ from {:.2} to at most {:.2}.", fire.mu0, fire.mu0 / utilisation), &format!("Brandlastniveau μ₀ von {:.2} auf höchstens {:.2} senken.", fire.mu0, fire.mu0 / utilisation)),
            ));
        }
        report.push(b.build());
    }
}

fn curling(annex: AnnexChoice, cf: &ColdFormedMember, report: &mut CheckReport) {
    let e = clauses::stability::e_steel() * 1e6;
    let z = cf.web_height / 2.0;
    let u = clauses::cold_formed::flange_curling(cf.flange_stress, cf.flange_width, e, cf.thickness, z);
    let limit = 0.05 * cf.web_height;
    let subject = || SubjectRef::new(&cf.id, format!("coldFormedMembers[id={}].flangeWidth", cf.id), loc("Flange width", "Flanschbreite"));
    let mut b = CheckResult::assess(
        format!("en1993.1-3.5.4.curl.{}", cf.id),
        "DIN EN 1993-1-3",
        ClauseId::new("EN 1993-1-3", "5", "5.4"),
        subject(),
        loc("Flange curling", "Gurtkrümmung"),
    )
    .annex(annex)
    .ndp("ndp.en1993.v-252.1")
    .utilization(length_m(u), length_m(limit))
    .explanation(loc(
        &format!("u=2σ_a²·b_s⁴/(E²·t²·z)={:.3} mm (σ_a={:.0} MPa, b_s={:.0} mm, t={:.2} mm, z={:.0} mm) against 5 % of the depth = {:.2} mm.", u * 1e3, cf.flange_stress / 1e6, cf.flange_width * 1e3, cf.thickness * 1e3, z * 1e3, limit * 1e3),
        &format!("u=2σ_a²·b_s⁴/(E²·t²·z)={:.3} mm (σ_a={:.0} MPa, b_s={:.0} mm, t={:.2} mm, z={:.0} mm) gegenüber 5 % der Querschnittshöhe = {:.2} mm.", u * 1e3, cf.flange_stress / 1e6, cf.flange_width * 1e3, cf.thickness * 1e3, z * 1e3, limit * 1e3),
    ));
    if u > limit {
        let b_s = cf.flange_width * (limit / u).powf(0.25);
        b = b.remedy(Remedy::at_most(subject(), length_m(cf.flange_width), length_m(b_s), loc(&format!("Reduce the flange width to at most {:.0} mm or add an edge stiffener.", b_s * 1e3), &format!("Flanschbreite auf höchstens {:.0} mm verringern oder einen Randsteifen anordnen.", b_s * 1e3))));
    }
    report.push(b.build());
}

/// 🥫️ Appends the flange curling check of every admitted cold-formed member that carries a flange stress.
pub(super) fn check_cold_formed(document: &En1993Snapshot, annex: AnnexChoice, scope: Scope<'_>, report: &mut CheckReport) {
    for cf in document.cold_formed_members.iter().filter(|c| scope.admits("vidColdFormed", &c.id)) {
        if cf.flange_stress > 0.0 && cf.flange_width > 0.0 && cf.web_height > 0.0 && cf.thickness > 0.0 {
            curling(annex, cf, report);
        }
    }
}
