//! ⏳️ Fatigue, fire and thin walled checks: the fatigue, fire, cold-formed, stainless and plated panel checks. See [`super`].

use super::*;

/// ⏳ Checks the Palmgren-Miner fatigue details of EN 1993-1-9 and the fire resistance of EN 1993-1-2.
pub(super) fn check_fatigue_and_fire(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, report: &mut CheckReport) {
    for fat in &document.fatigue_details {
        let method = part_1_9::parse_method(&fat.method);
        let gmf = part_1_9::gamma_mf(method);
        params.gamma_mf = gmf;
        let delta_c = part_1_9::detail_category_pa(fat.category);
        let spectrum: Vec<(f64, f64)> = fat.spectrum.iter().map(|b| (b.delta_sigma, b.cycles)).collect();
        let damage = part_1_9::miner_damage(&spectrum, delta_c, gmf);
        let mut b = CheckResult::assess(
            format!("en1993.1-9.fat.{}", fat.id),
            "DIN EN 1993-1-9",
            ClauseId::new("EN 1993-1-9", "8", "8"),
            SubjectRef::new(&fat.id, &format!("fatigueDetails[id={}].spectrum", fat.id), loc(&format!("Fatigue {}", fat.id), &format!("Ermüdung {}", fat.id))),
            loc("Fatigue Palmgren-Miner damage", "Ermüdungsschädigung Palmgren-Miner"),
        )
        .annex(annex)
        .utilization(dimensionless(damage), dimensionless(1.0))
        .explanation(loc(
            &format!("D={:.3} (Σ n_i/N_i, cat. {}, γ_Mf={}, {} bands).", damage, fat.category, gmf, fat.spectrum.len()),
            &format!("Schädigung D={:.3} (Σ n_i/N_i, Kerbfall {}, γ_Mf={}, {} Stufen).", damage, fat.category, gmf, fat.spectrum.len()),
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

    for fire in &document.fire_exposures {
        let rating = part_1_2::parse_rating(&fire.rating);
        let theta_heating = part_1_2::steel_temperature_c(
            rating,
            fire.section_factor,
            fire.protection_thickness,
            fire.protection_conductivity,
            fire.protection_density,
            fire.protection_specific_heat,
        );
        let theta_a = fire.design_temperature.max(theta_heating);
        let theta_cr = part_1_2::critical_temperature_c(fire.mu0);
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
        let lambda_theta = if ke > 1e-9 { (ky / ke).sqrt() } else { 10.0 };
        let fy_fire = document
            .members
            .iter()
            .find(|m| m.id == fire.member_id)
            .and_then(|m| find_material(document, &m.material_id))
            .map(|m| m.fy)
            .unwrap_or(355.0e6);
        let chi_fi = part_1_2::chi_fi(lambda_theta, fy_fire);

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
    }
}

/// 🧊 Checks the cold-formed members, the stainless members and the plated panels.
pub(super) fn check_cold_formed_stainless_and_plated(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, report: &mut CheckReport) {
    for cf in &document.cold_formed_members {
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
    }

    for material in document.materials.iter().filter(|m| m.kind == "stainless") {
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
                    let grades = ["S355", "S460"].iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
                    b = b.remedy(Remedy::one_of(
                        subject_member(member, &format!("materials[id={}].grade", material.id)),
                        grades.clone(),
                        loc(
                            &format!("Increase steel grade; options: {}.", grades.join(", ")),
                            &format!("Stahlgüte erhöhen; Optionen: {}.", grades.join(", ")),
                        ),
                    ));
                }
                report.push(b.build());
            }
        }
    }

    for panel in &document.plated_panels {
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
    }
}
