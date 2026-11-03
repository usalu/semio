//! ⏳️ Fatigue, fire and thin walled checks: the fatigue, fire, cold-formed, stainless and plated panel checks. See [`super`].

use super::*;

/// ⏳ Checks the Palmgren-Miner fatigue details of EN 1993-1-9 and the fire resistance of EN 1993-1-2.
pub(super) fn check_fatigue_and_fire(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, scope: Scope<'_>, report: &mut CheckReport) {
    for fat in document.fatigue_details.iter().filter(|e| scope.admits("fatigueDetails", &e.id)) {
        let method = part_1_9::parse_method(fat.method.as_str());
        let gmf = part_1_9::gamma_mf(method);
        let delta_c = part_1_9::detail_category_pa(fat.category);
        let history: Vec<f64> = fat.stress_history.split_whitespace().filter_map(|v| v.parse::<f64>().ok()).collect();
        let counted: Vec<(f64, f64)> = if fat.spectrum.is_empty() && history.len() >= 2 { crate::clauses::fatigue_fire::rainflow(&history).into_iter().map(|(range, n)| (range * 1e6, n)).collect() } else { Vec::new() };
        let spectrum: Vec<(f64, f64)> = if counted.is_empty() { fat.spectrum.iter().map(|b| (b.delta_sigma, b.cycles)).collect() } else { counted.clone() };
        let origin_en = if counted.is_empty() { format!("{} bands", fat.spectrum.len()) } else { format!("{} rainflow blocks of {} history points (ASTM three-point counting, half cycles 0.5)", counted.len(), history.len()) };
        let origin_de = if counted.is_empty() { format!("{} Stufen", fat.spectrum.len()) } else { format!("{} Rainflow-Blöcke aus {} Punkten der Spannungsgeschichte (ASTM-Dreipunktzählung, Halbzyklen 0,5)", counted.len(), history.len()) };
        let damage = part_1_9::miner_damage(&spectrum, delta_c, gmf);
        let mut b = CheckResult::assess(
            format!("en1993.1-9.fat.{}", fat.id),
            "DIN EN 1993-1-9",
            ClauseId::new("EN 1993-1-9", "8", "8"),
            SubjectRef::new(&fat.id, &format!("fatigueDetails[id={}].{}", fat.id, if counted.is_empty() { "spectrum" } else { "stressHistory" }), loc(&format!("Fatigue {}", fat.id), &format!("Ermüdung {}", fat.id))),
            loc("Fatigue Palmgren-Miner damage", "Ermüdungsschädigung Palmgren-Miner"),
        )
        .annex(annex)
        .ndp(AnnexParams::GAMMA_MF)
        .utilization(dimensionless(damage), dimensionless(1.0))
        .explanation(loc(
            &format!("D={:.3} (Σ n_i/N_i, cat. {}, γ_Mf={}, {origin_en}).", damage, fat.category, gmf),
            &format!("Schädigung D={:.3} (Σ n_i/N_i, Kerbfall {}, γ_Mf={}, {origin_de}).", damage, fat.category, gmf),
        ));
        if damage > 1.0 {
            let cats = [80u8, 90, 100, 112, 125, 140, 160];
            let options: Vec<String> = cats.iter().filter(|c| {
                part_1_9::miner_damage(&spectrum, part_1_9::detail_category_pa(**c), gmf) <= 1.0
            }).map(|c| c.to_string()).collect();
            if !options.is_empty() {
                b = b.remedy(Remedy::one_of(
                    SubjectRef::new(&fat.id, &format!("fatigueDetails[id={}].category", fat.id), loc("Detail category", "Kerbfall")),
                    options.clone(),
                    loc(
                        &format!("Improve detail category to one of: {}.", options.join(", ")),
                        &format!("Kerbfall verbessern auf eine von: {}.", options.join(", ")),
                    ),
                ));
            }
            if let Some(peak) = fat.spectrum.iter().max_by(|a, b| a.delta_sigma.partial_cmp(&b.delta_sigma).unwrap()) {
                let ds_req = peak.delta_sigma / damage.max(1.0).powf(1.0 / 3.0);
                b = b.remedy(Remedy::at_most(
                    SubjectRef::new(&fat.id, &format!("fatigueDetails[id={}].spectrum[id={}].deltaSigma", fat.id, peak.id), loc("Stress range", "Spannungsschwingbreite")),
                    stress_pa(peak.delta_sigma),
                    stress_pa(ds_req),
                    loc(
                        &format!("Reduce peak Δσ to at most {:.1} MPa.", ds_req / 1e6),
                        &format!("Spitzen-Δσ auf höchstens {:.1} MPa reduzieren.", ds_req / 1e6),
                    ),
                ));
            }
        }
        report.push(b.build());
    }

    for fire in document.fire_exposures.iter().filter(|e| scope.admits("fireExposures", &e.id)) {
        let rating = part_1_2::parse_rating(fire.rating.as_str());
        let theta_heating = part_1_2::steel_temperature_c(
            rating,
            fire.section_factor,
            fire.protection_thickness,
            fire.protection_conductivity,
            fire.protection_density,
            fire.protection_specific_heat,
        );
        let theta_a = fire.design_temperature.max(theta_heating);
        let class4 = document
            .members
            .iter()
            .find(|m| m.id == fire.member_id)
            .and_then(|m| find_section(document, &m.section_id).zip(find_material(document, &m.material_id)))
            .is_some_and(|(sec, mat)| part_1_1::section_class_rolled_i(sec, mat.fy) == 4);
        let theta_cr = if class4 { 350.0 } else { part_1_2::critical_temperature_c(fire.mu0) };
        let t_req = part_1_2::required_protection_thickness(
            rating,
            fire.section_factor,
            fire.mu0,
            fire.protection_conductivity,
            fire.protection_density,
            fire.protection_specific_heat,
        );
        let ky = part_1_2::k_y_theta(theta_a);
        let ke = part_1_2::k_e_theta(theta_a);
        let fire_member = document.members.iter().find(|m| m.id == fire.member_id);
        let fire_section = fire_member.and_then(|m| find_section(document, &m.section_id));
        let fy_fire = fire_member.and_then(|m| find_material(document, &m.material_id)).map(|m| m.fy).unwrap_or(355.0e6);
        let lambda_member = match (fire_member, fire_section) {
            (Some(m), Some(sec)) => part_1_1::lambda_bar(m.buckling_length_z, part_1_1::radius_of_gyration(sec.iz, sec.area), 210.0e9, fy_fire),
            _ => 0.0,
        };
        let chi_fi = part_1_2::chi_fi(lambda_member, theta_a, fy_fire);

        let mut b = CheckResult::assess(
            format!("en1993.1-2.prot.{}", fire.id),
            "DIN EN 1993-1-2",
            ClauseId::new("EN 1993-1-2", "4", "4.2.5"),
            SubjectRef::new(&fire.id, &format!("fireExposures[id={}].protectionThickness", fire.id), loc(&format!("Fire {}", fire.id), &format!("Brandschutz {}", fire.id))),
            loc("Fire protection thickness", "Brandschutzdicke"),
        )
        .annex(annex)
        .minimum(length_m(fire.protection_thickness), length_m(t_req))
        .explanation(loc(
            &format!("t_p={:.0} mm, required={:.0} mm (§4.2.5 incremental, Am/V={:.0} m⁻¹, θ_a={:.0} °C).", fire.protection_thickness * 1000.0, t_req * 1000.0, fire.section_factor, theta_a),
            &format!("t_p={:.0} mm, erforderlich={:.0} mm (§4.2.5 inkrementell, Am/V={:.0} m⁻¹, θ_a={:.0} °C).", fire.protection_thickness * 1000.0, t_req * 1000.0, fire.section_factor, theta_a),
        ));
        if fire.protection_thickness + 1e-9 < t_req {
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&fire.id, &format!("fireExposures[id={}].protectionThickness", fire.id), loc("Protection thickness", "Brandschutzdicke")),
                length_m(fire.protection_thickness),
                length_m(t_req),
                loc(
                    &format!("Increase protection from {:.0} mm to at least {:.0} mm.", fire.protection_thickness * 1000.0, t_req * 1000.0),
                    &format!("Brandschutz von {:.0} mm auf mindestens {:.0} mm erhöhen.", fire.protection_thickness * 1000.0, t_req * 1000.0),
                ),
            ));
        }
        report.push(b.build());

        let mut b = CheckResult::assess(
            format!("en1993.1-2.theta.{}", fire.id),
            "DIN EN 1993-1-2",
            ClauseId::new("EN 1993-1-2", "4", "4.2.4"),
            SubjectRef::new(&fire.id, &format!("fireExposures[id={}].mu0", fire.id), loc(&format!("Fire {}", fire.id), &format!("Brand {}", fire.id))),
            loc("Critical steel temperature", "Kritische Stahltemperatur"),
        )
        .annex(annex)
        .minimum(temperature_c(theta_cr), temperature_c(theta_a))
        .explanation(loc(
            &format!("θ_a,cr={:.0} °C ≥ θ_a={:.0} °C (μ₀={:.2}, k_y,θ={:.2}, k_E,θ={:.2}, χ_fi={:.2}).", theta_cr, theta_a, fire.mu0, ky, ke, chi_fi),
            &format!("Kritische Stahltemperatur θ_a,cr={:.0} °C ≥ berechnete θ_a={:.0} °C (Ausnutzung μ₀={:.2}, Abminderung k_y,θ={:.2}, k_E,θ={:.2}, χ_fi={:.2}).", theta_cr, theta_a, fire.mu0, ky, ke, chi_fi),
        ));
        if theta_cr + 1e-6 < theta_a {
            let mut mu = fire.mu0;
            for _ in 0..40 {
                if part_1_2::critical_temperature_c(mu) >= theta_a {
                    break;
                }
                mu *= 0.95;
            }
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&fire.id, &format!("fireExposures[id={}].mu0", fire.id), loc("Load level μ0", "Ausnutzung μ0")),
                dimensionless(fire.mu0),
                dimensionless(mu),
                loc(
                    &format!("Reduce cold utilization μ₀ from {:.2} to at most {:.2}.", fire.mu0, mu),
                    &format!("Kaltausnutzung μ₀ von {:.2} auf höchstens {:.2} senken.", fire.mu0, mu),
                ),
            ));
            if t_req > fire.protection_thickness {
                b = b.remedy(Remedy::at_least(
                    SubjectRef::new(&fire.id, &format!("fireExposures[id={}].protectionThickness", fire.id), loc("Protection thickness", "Brandschutzdicke")),
                    length_m(fire.protection_thickness),
                    length_m(t_req),
                    loc(
                        &format!("Or increase protection to at least {:.0} mm.", t_req * 1000.0),
                        &format!("Oder Brandschutz auf mindestens {:.0} mm erhöhen.", t_req * 1000.0),
                    ),
                ));
            }
        }
        report.push(b.build());
        super::special_parts_checks::check_stainless_fire(document, annex, fire, report);
    }
}

/// 🧊 Checks the cold-formed members, the stainless members and the plated panels.
pub(super) fn check_cold_formed_stainless_and_plated(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, scope: Scope<'_>, report: &mut CheckReport) {
    for cf in document.cold_formed_members.iter().filter(|e| scope.admits("coldFormedMembers", &e.id)) {
        check_cold_formed_section(cf, annex, report);
        let forces: Vec<(String, f64)> = cf.actions.iter().map(|a| (a.load_case_id.clone(), a.force)).collect();
        let combos = combine_scalar_forces(document, &forces);
        let n_ed = governing_scalar(&combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let gov = governing_scalar(&combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");
        let lp = part_1_3::lambda_p(cf.b_bar, cf.thickness, cf.fy, cf.k_sigma);
        let rho = part_1_3::reduction_factor(lp, cf.psi);
        let n_eff = rho * cf.gross_resistance;
        let mut b = CheckResult::assess(
            format!("en1993.1-3.cf.{}", cf.id),
            "DIN EN 1993-1-3",
            ClauseId::new("EN 1993-1-3", "5", "5.5.2"),
            SubjectRef::new(&cf.id, &format!("coldFormedMembers[id={}].thickness", cf.id), loc(&format!("CF {}", cf.id), &format!("Kaltprofil {}", cf.id))),
            loc("Cold-formed effective section", "Kaltprofil wirksamer Querschnitt"),
        )
        .annex(annex)
        .utilization(force_n(n_ed), force_n(n_eff))
        .explanation(loc(
            &format!("N_Ed={:.1} kN (gov. {}), N_eff,Rd={:.1} kN (ρ={:.3}, λ_p={:.3}).", n_ed / 1000.0, gov, n_eff / 1000.0, rho, lp),
            &format!("Kaltprofil N_Ed={:.1} kN (maßgebend {}), N_eff,Rd={:.1} kN (ρ={:.3}, λ_p={:.3}).", n_ed / 1000.0, gov, n_eff / 1000.0, rho, lp),
        ));
        if n_ed > n_eff {
            let t_req = cf.thickness * (n_ed / n_eff.max(1e-9)).sqrt();
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&cf.id, &format!("coldFormedMembers[id={}].thickness", cf.id), loc("Thickness", "Dicke")),
                length_m(cf.thickness),
                length_m(t_req),
                loc(
                    &format!("Increase thickness from {:.2} mm to at least {:.2} mm.", cf.thickness * 1000.0, t_req * 1000.0),
                    &format!("Dicke von {:.2} mm auf mindestens {:.2} mm erhöhen.", cf.thickness * 1000.0, t_req * 1000.0),
                ),
            ));
            let gr_req = n_ed / rho.max(1e-9);
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&cf.id, &format!("coldFormedMembers[id={}].grossResistance", cf.id), loc("Gross resistance", "Bruttotragfähigkeit")),
                force_n(cf.gross_resistance),
                force_n(gr_req),
                loc(
                    &format!("Or raise gross resistance to ≥ {:.1} kN.", gr_req / 1000.0),
                    &format!("Oder Bruttotragfähigkeit auf ≥ {:.1} kN anheben.", gr_req / 1000.0),
                ),
            ));
        }
        report.push(b.build());
        super::special_parts_checks::check_cold_formed(annex, params, cf, n_ed, n_eff, report);
    }

    for material in document.materials.iter().filter(|m| m.kind == semio_s_artifact_norm_en1993_model::MaterialKind::Stainless && scope.admits("stainlessMaterials", &m.id)) {
        for member in document.members.iter().filter(|m| m.material_id == material.id) {
            let Some(section) = find_section(document, &member.section_id) else { continue };
            let m_rd = part_1_4::bending_resistance_nm(section.w_pl_y, material.fy);
            let combinations = combine_member_actions(document, &member.id);
            if let Some(gov) = governing_effects(&combinations, "uls") {
                let mut b = CheckResult::assess(
                    format!("en1993.1-4.ss.{}", member.id),
                    "DIN EN 1993-1-4",
                    ClauseId::new("EN 1993-1-4", "6", "6"),
                    subject_member(member, &format!("members[id={}].id", member.id)),
                    loc("Stainless bending", "Nichtrostender Stahl Biegung"),
                )
                .annex(annex)
                .utilization(moment_nm(gov.action.my.abs()), moment_nm(m_rd))
                .explanation(loc(
                    &format!("M_Ed={:.1} kNm (gov. {}), M_Rd={:.1} kNm (γ_M=1.1).", gov.action.my.abs() / 1000.0, gov.combination_id, m_rd / 1000.0),
                    &format!("Edelstahl Biegung M_Ed={:.1} kNm (maßgebend {}), M_Rd={:.1} kNm (γ_M=1.1).", gov.action.my.abs() / 1000.0, gov.combination_id, m_rd / 1000.0),
                ));
                if gov.action.my.abs() > m_rd {
                    let grades = ["1.4404", "1.4462"].iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
                    b = b.remedy(Remedy::one_of(
                        subject_member(member, &format!("materials[id={}].grade", material.id)),
                        grades.clone(),
                        loc(
                            &format!("Increase stainless grade; options: {}.", grades.join(", ")),
                            &format!("Edelstahlgüte erhöhen; Optionen: {}.", grades.join(", ")),
                        ),
                    ));
                }
                report.push(b.build());
            }
        }
    }

    for panel in document.plated_panels.iter().filter(|e| scope.admits("platedPanels", &e.id)) {
        let forces: Vec<(String, f64)> = panel.actions.iter().map(|a| (a.load_case_id.clone(), a.force)).collect();
        let combos = combine_scalar_forces(document, &forces);
        let sigma_ed = governing_scalar(&combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let gov = governing_scalar(&combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");
        let lp = part_1_5::lambda_p(panel.b, panel.thickness, panel.fy, panel.k_sigma);
        let rd = part_1_5::local_buckling_stress_rd(panel.fy, lp, params);
        let mut b = CheckResult::assess(
            format!("en1993.1-5.plate.{}", panel.id),
            "DIN EN 1993-1-5",
            ClauseId::new("EN 1993-1-5", "4", "4.1"),
            SubjectRef::new(&panel.id, &format!("platedPanels[id={}].thickness", panel.id), loc(&format!("Plate {}", panel.id), &format!("Beulblech {}", panel.id))),
            loc("Plate buckling", "Plattenbeulen"),
        )
        .annex(annex)
        .ndp(AnnexParams::GAMMA_M0)
        .utilization(stress_pa(sigma_ed), stress_pa(rd))
        .explanation(loc(
            &format!("σ_Ed={:.1} MPa (gov. {}), σ_c,Rd={:.1} MPa (λ_p={:.3}).", sigma_ed / 1e6, gov, rd / 1e6, lp),
            &format!("Plattenbeulen σ_Ed={:.1} MPa (maßgebend {}), σ_c,Rd={:.1} MPa (λ_p={:.3}).", sigma_ed / 1e6, gov, rd / 1e6, lp),
        ));
        if sigma_ed > rd {
            let t_req = panel.thickness * (sigma_ed / rd.max(1e-9)).sqrt();
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&panel.id, &format!("platedPanels[id={}].thickness", panel.id), loc("Thickness", "Dicke")),
                length_m(panel.thickness),
                length_m(t_req),
                loc(
                    &format!("Increase plate thickness to at least {:.1} mm.", t_req * 1000.0),
                    &format!("Blechdicke auf mindestens {:.1} mm erhöhen.", t_req * 1000.0),
                ),
            ));
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&panel.id, &format!("platedPanels[id={}].b", panel.id), loc("Panel width", "Blechbreite")),
                length_m(panel.b),
                length_m(panel.b * (rd / sigma_ed.max(1e-9)).sqrt()),
                loc(
                    &format!("Or reduce panel width b."),
                    &format!("Oder Blechbreite b verringern."),
                ),
            ));
        }
        report.push(b.build());

        if let (Some(member), Some(section)) = (fire_member, fire_section) {
            let rows = combine_member_actions(document, &member.id);
            let n_ed = uls_rows(&rows).iter().map(|r| r.action.n).fold(0.0_f64, f64::max);
            if n_ed > 0.0 {
                let n_fi_ed = 0.65 * n_ed;
                let n_b_fi = crate::clauses::fatigue_fire::n_b_fi_t_rd(chi_fi, section.area, ky, fy_fire, 1.0);
                let mut nfi =
                    CheckResult::assess(
                        format!("en1993.1-2.nfi.{}", fire.id),
                        "DIN EN 1993-1-2",
                        ClauseId::new("EN 1993-1-2", "4", "4.2.3.2"),
                        SubjectRef::new(&fire.id, &format!("fireExposures[id={}].designTemperature", fire.id), loc(&format!("Fire {}", fire.id), &format!("Brand {}", fire.id))),
                        loc("Buckling resistance in fire", "Knickwiderstand im Brandfall"),
                    )
                    .annex(annex)
                    .utilization(force_n(n_fi_ed), force_n(n_b_fi))
                    .explanation(loc(
                        &format!("N_fi,Ed=0.65·N_Ed={:.1} kN, N_b,fi,t,Rd={:.1} kN (χ_fi={:.3}, k_y,θ={:.3}, λ̄={:.2}).", n_fi_ed / 1000.0, n_b_fi / 1000.0, chi_fi, ky, lambda_member),
                        &format!("N_fi,Ed=0,65·N_Ed={:.1} kN, N_b,fi,t,Rd={:.1} kN (χ_fi={:.3}, k_y,θ={:.3}, λ̄={:.2}).", n_fi_ed / 1000.0, n_b_fi / 1000.0, chi_fi, ky, lambda_member),
                    ));
                if !(n_fi_ed <= n_b_fi) {
                    let k = if (n_fi_ed / n_b_fi).is_finite() { n_fi_ed / n_b_fi } else { 2.0 };
                    nfi = nfi.remedy(Remedy::at_least(
                        SubjectRef::new(&member.id, &format!("sections[id={}].area", section.id), loc("Section area", "Querschnittsfläche")),
                        area_m2(section.area),
                        area_m2(section.area * k),
                        loc(&format!("Increase the section area to about {:.1} cm² (first estimate) or protect the member against fire.", section.area * k * 1e4), &format!("Querschnittsfläche auf etwa {:.1} cm² erhöhen (erste Schätzung) oder das Bauteil brandschutztechnisch bekleiden.", section.area * k * 1e4)),
                    ));
                }
                report.push(nfi.build());
            }
        }
        super::special_parts_checks::check_plated(annex, params, panel, report);
    }
}
