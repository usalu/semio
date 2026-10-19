//! 🏛️ Shell and structure checks: the shell, through-thickness, tension, high-strength, bridge, tower, pile and crane checks. See [`super`].

use super::*;

/// 🥫 Checks the shells and silos, the through-thickness properties, the tension components and the high-strength steels.
pub(super) fn check_shell_and_member_properties(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, report: &mut CheckReport) {
    for silo in &document.silo_shells {
        let p_h = part_4::janssen_pressure(silo.k, silo.gamma, silo.depth);
        let sigma_ed = part_4::membrane_hoop_stress(p_h, silo.radius, silo.thickness);
        let sigma_rcr = part_1_6::sigma_x_rcr(silo.thickness, silo.radius, 210.0e9);
        let lam = part_1_6::lambda_bar(silo.fy, sigma_rcr);
        let alpha = part_1_6::alpha_imperfection(silo.radius, silo.thickness);
        let chi = part_1_6::chi(lam, alpha);
        let sigma_rd = part_1_6::design_resistance(silo.fy, chi, params);
        let mut b = CheckResult::assess(
            format!("en1993.1-6.shell.{}", silo.id),
            "DIN EN 1993-1-6",
            ClauseId::new("EN 1993-1-6", "8", "8.5.2"),
            SubjectRef::new(&silo.id, &format!("siloShells[id={}].thickness", silo.id), loc(&format!("Shell {}", silo.id), &format!("Schale {}", silo.id))),
            loc("Shell buckling", "Schalenbeulen"),
        )
        .annex(annex)
        .utilization(stress_pa(sigma_ed), stress_pa(sigma_rd))
        .explanation(loc(
            &format!("σ_Ed={:.1} MPa (Janssen hoop), σ_x,Rd={:.1} MPa (χ={:.3}).", sigma_ed / 1e6, sigma_rd / 1e6, chi),
            &format!("Schalenbeulen σ_Ed={:.1} MPa (Janssen Ringspannung), σ_x,Rd={:.1} MPa (χ={:.3}).", sigma_ed / 1e6, sigma_rd / 1e6, chi),
        ));
        if sigma_ed > sigma_rd {
            let t_req = silo.thickness * (sigma_ed / sigma_rd);
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&silo.id, &format!("siloShells[id={}].thickness", silo.id), loc("Wall thickness", "Wanddicke")),
                length_m(silo.thickness),
                length_m(t_req),
                loc(
                    &format!("Increase silo wall thickness to at least {:.1} mm.", t_req * 1000.0),
                    &format!("Silowanddicke auf mindestens {:.1} mm erhöhen.", t_req * 1000.0),
                ),
            ));
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&silo.id, &format!("siloShells[id={}].depth", silo.id), loc("Fill depth", "Füllhöhe")),
                length_m(silo.depth),
                length_m(silo.depth * sigma_rd / sigma_ed.max(1e-9)),
                loc(
                    &format!("Or reduce fill depth."),
                    &format!("Oder Füllhöhe verringern."),
                ),
            ));
        }
        report.push(b.build());

        let mut b = CheckResult::assess(
            format!("en1993.4-1.silo.{}", silo.id),
            "DIN EN 1993-4-1",
            ClauseId::new("EN 1993-4-1", "5", "5.3"),
            SubjectRef::new(&silo.id, &format!("siloShells[id={}].thickness", silo.id), loc(&format!("Silo {}", silo.id),
                    &format!("Nachweis: Silo {}", silo.id))),
            loc("Silo wall membrane stress", "Silowand Membranspannung"),
        )
        .annex(annex)
        .utilization(stress_pa(sigma_ed), stress_pa(sigma_rd))
        .explanation(loc(
            &format!("Janssen hoop membrane σ_θ,Ed={:.1} MPa versus shell design resistance.", sigma_ed / 1e6),
            &format!("Janssen-Ringspannung σ_θ,Ed={:.1} MPa gegenüber Schalenbemessungswiderstand.", sigma_ed / 1e6),
        ));
        if sigma_ed > sigma_rd {
            let t_req = silo.thickness * (sigma_ed / sigma_rd);
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&silo.id, &format!("siloShells[id={}].thickness", silo.id), loc("Wall thickness", "Wanddicke")),
                length_m(silo.thickness),
                length_m(t_req),
                loc(
                    &format!("Increase thickness to ≥ {:.1} mm.", t_req * 1000.0),
                    &format!("Dicke auf ≥ {:.1} mm erhöhen.", t_req * 1000.0),
                ),
            ));
        }
        report.push(b.build());
    }

    for member in &document.members {
        let Some(section) = find_section(document, &member.section_id) else { continue };
        let Some(material) = find_material(document, &member.material_id) else { continue };
        let t = section.tf.max(section.tw);
        let t_max = part_1_10::max_permissible_thickness(&material.subgrade, 0.0);
        let mut b = CheckResult::assess(
            format!("en1993.1-10.t.{}", member.id),
            "DIN EN 1993-1-10",
            ClauseId::new("EN 1993-1-10", "2", "2.1"),
            subject_member(member, &format!("materials[id={}].subgrade", material.id)),
            loc("Through-thickness toughness", "Zähigkeitsanforderung"),
        )
        .annex(annex)
        .utilization(length_m(t), length_m(t_max))
        .explanation(loc(
            &format!("t={:.0} mm, t_max={:.0} mm for subgrade {}.", t * 1000.0, t_max * 1000.0, material.subgrade),
            &format!("t={:.0} mm, t_max={:.0} mm für Güte {}.", t * 1000.0, t_max * 1000.0, material.subgrade),
        ));
        if t > t_max {
            let options = ["J2", "K2", "M", "N"].iter().filter(|s| part_1_10::max_permissible_thickness(s, 0.0) >= t).map(|s| (*s).to_string()).collect::<Vec<_>>();
            b = b.remedy(Remedy::one_of(
                subject_member(member, &format!("materials[id={}].subgrade", material.id)),
                options.clone(),
                loc(
                    &format!("Upgrade steel subgrade to one of: {}.", options.join(", ")),
                    &format!("Stahlgütegruppe anheben auf: {}.", options.join(", ")),
                ),
            ));
        }
        report.push(b.build());
    }

    for tc in &document.tension_components {
        let forces: Vec<(String, f64)> = tc.actions.iter().map(|a| (a.load_case_id.clone(), a.force)).collect();
        let combos = combine_scalar_forces(document, &forces);
        let n_ed = governing_scalar(&combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let gov = governing_scalar(&combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");
        let rd = part_1_11::tension_component_resistance_n(tc.f_uk, tc.f_k);
        let mut b = CheckResult::assess(
            format!("en1993.1-11.tc.{}", tc.id),
            "DIN EN 1993-1-11",
            ClauseId::new("EN 1993-1-11", "6", "6.2"),
            SubjectRef::new(&tc.id, &format!("tensionComponents[id={}].actions", tc.id), loc(&format!("Cable {}", tc.id), &format!("Zugglied {}", tc.id))),
            loc("Tension component", "Zugglied"),
        )
        .annex(annex)
        .utilization(force_n(n_ed), force_n(rd))
        .explanation(loc(
            &format!("N_Ed={:.1} kN (gov. {}), F_Rd={:.1} kN.", n_ed / 1000.0, gov, rd / 1000.0),
            &format!("Zugglied N_Ed={:.1} kN (maßgebend {}), F_Rd={:.1} kN.", n_ed / 1000.0, gov, rd / 1000.0),
        ));
        if n_ed > rd {
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&tc.id, &format!("tensionComponents[id={}].fK", tc.id), loc("Characteristic resistance", "Charakteristische Tragfähigkeit")),
                force_n(tc.f_k),
                force_n(n_ed),
                loc(
                    &format!("Increase F_k to at least {:.1} kN.", n_ed / 1000.0),
                    &format!("F_k auf mindestens {:.1} kN erhöhen.", n_ed / 1000.0),
                ),
            ));
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&tc.id, &format!("tensionComponents[id={}].fUk", tc.id), loc("Ultimate strength", "Bruchfestigkeit")),
                force_n(tc.f_uk),
                force_n(n_ed * 1.5),
                loc(
                    &format!("Or raise F_uk to ≥ {:.1} kN.", n_ed * 1.5 / 1000.0),
                    &format!("Oder F_uk auf ≥ {:.1} kN anheben.", n_ed * 1.5 / 1000.0),
                ),
            ));
        }
        report.push(b.build());
    }

    for material in document.materials.iter().filter(|m| m.kind == "hss" || m.fy >= 460.0e6) {
        for member in document.members.iter().filter(|m| m.material_id == material.id) {
            let Some(section) = find_section(document, &member.section_id) else { continue };
            let class = part_1_1::section_class_rolled_i(section, material.fy);
            let m_rd = part_1_12::elastic_bending_resistance_nm(section.w_el_y, material.fy, class, params);
            let combinations = combine_member_actions(document, &member.id);
            if let Some(gov) = governing_effects(&combinations, "uls") {
                let mut b = CheckResult::assess(
                    format!("en1993.1-12.hss.{}", member.id),
                    "DIN EN 1993-1-12",
                    ClauseId::new("EN 1993-1-12", "4", "4.1"),
                    subject_member(member, &format!("members[id={}].id", member.id)),
                    loc("HSS elastic bending", "HSS elastische Biegung"),
                )
                .annex(annex)
                .utilization(
                    if m_rd > 0.0 {
                        moment_nm(gov.action.my.abs())
                    } else if gov.action.my.abs() > 0.0 {
                        dimensionless(class as f64)
                    } else {
                        moment_nm(0.0)
                    },
                    if m_rd > 0.0 {
                        moment_nm(m_rd)
                    } else if gov.action.my.abs() > 0.0 {
                        dimensionless(3.0)
                    } else {
                        moment_nm(1.0)
                    },
                )
                .explanation(loc(
                    &format!("M_Ed={:.1} kNm (gov. {}), M_el,Rd={:.1} kNm (class {}).", gov.action.my.abs() / 1000.0, gov.combination_id, m_rd / 1000.0, class),
                    &format!("HSS Biegung M_Ed={:.1} kNm (maßgebend {}), M_el,Rd={:.1} kNm (Klasse {}).", gov.action.my.abs() / 1000.0, gov.combination_id, m_rd / 1000.0, class),
                ));
                if gov.action.my.abs() > m_rd {
                    let options = next_section_options(section, section.area, section.w_pl_y);
                    if !options.is_empty() {
                        b = b.remedy(Remedy::one_of(
                            subject_member(member, &format!("members[id={}].sectionId", member.id)),
                            options.clone(),
                            loc(
                                &format!("Select stockier section: {}.", options.join(", ")),
                                &format!("Gedrungeneren Querschnitt wählen: {}.", options.join(", ")),
                            ),
                        ));
                    } else if m_rd > 0.0 {
                        let w_req = gov.action.my.abs() * params.gamma_m0 / material.fy * 1.01;
                        b = b.remedy(Remedy::at_least(
                            subject_member(member, &format!("sections[id={}].wElY", section.id)),
                            moment_nm(section.w_el_y),
                            moment_nm(w_req.max(section.w_el_y)),
                            loc("Increase W_el,y so M_el,Rd covers M_Ed.", "W_el,y erhöhen, damit M_el,Rd die M_Ed deckt."),
                        ));
                    }
                }
                report.push(b.build());
            }
        }
    }
}

/// 🌉 Checks the bridge, tower, pile and crane runway rules.
pub(super) fn check_bridge_tower_pile_and_crane(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, report: &mut CheckReport) {
    for bf in &document.bridge_fatigue {
        let method = part_1_9::parse_method(&bf.method);
        let gmf = part_1_9::gamma_mf(method);
        let delta_e2 = part_2::damage_equivalent_stress(bf.lambda, bf.phi2, bf.delta_sigma_p);
        let limit = part_1_9::detail_category_pa(bf.category) / gmf;
        let mut b = CheckResult::assess(
            format!("en1993.2.fat.{}", bf.id),
            "DIN EN 1993-2",
            ClauseId::new("EN 1993-2", "9", "9.3"),
            SubjectRef::new(&bf.id, &format!("bridgeFatigue[id={}].deltaSigmaP", bf.id), loc(&format!("Bridge {}", bf.id), &format!("Brücke {}", bf.id))),
            loc("Bridge damage-equivalent fatigue", "Schädigungsäquivalente Ermüdung Brücke"),
        )
        .annex(annex)
        .utilization(stress_pa(delta_e2), stress_pa(limit))
        .explanation(loc(
            &format!("Δσ_E2={:.1} MPa (λ={:.2}, φ2={:.2}), limit={:.1} MPa.", delta_e2 / 1e6, bf.lambda, bf.phi2, limit / 1e6),
            &format!("Schädigungsäquivalente Δσ_E2={:.1} MPa (λ={:.2}, φ2={:.2}), Grenzwert={:.1} MPa.", delta_e2 / 1e6, bf.lambda, bf.phi2, limit / 1e6),
        ));
        if delta_e2 > limit {
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&bf.id, &format!("bridgeFatigue[id={}].deltaSigmaP", bf.id), loc("Stress range", "Spannungsschwingbreite")),
                stress_pa(bf.delta_sigma_p),
                stress_pa(limit / (bf.lambda * bf.phi2).max(1e-9)),
                loc(
                    &format!("Reduce Δσ_p to at most {:.1} MPa.", limit / (bf.lambda * bf.phi2).max(1e-9) / 1e6),
                    &format!("Δσ_p auf höchstens {:.1} MPa reduzieren.", limit / (bf.lambda * bf.phi2).max(1e-9) / 1e6),
                ),
            ));
            b = b.remedy(Remedy::one_of(
                SubjectRef::new(&bf.id, &format!("bridgeFatigue[id={}].category", bf.id), loc("Detail category", "Kerbfall")),
                vec!["100".into(), "112".into(), "125".into()],
                loc(
                    &format!("Or improve bridge detail category."),
                    &format!("Oder Brücken-Kerbfall verbessern."),
                ),
            ));
        }
        report.push(b.build());

        if let Some(member) = document.members.iter().find(|m| m.id == bf.member_id) {
            if let (Some(section), Some(material)) = (find_section(document, &member.section_id), find_material(document, &member.material_id)) {
                let combinations = combine_member_actions(document, &member.id);
                if let Some(gov) = governing_effects(&combinations, "uls") {
                    let n_rd = part_1_1::axial_resistance_n(section.area, material.fy, params);
                    let m_rd = part_1_1::bending_resistance_nm(section.w_pl_y, material.fy, 2, params);
                    let eta = part_2::bridge_interaction_eta(gov.action.n, n_rd, gov.action.my, m_rd);
                    let mut b = CheckResult::assess(
                        format!("en1993.2.int.{}", bf.id),
                        "DIN EN 1993-2",
                        ClauseId::new("EN 1993-2", "6", "6"),
                        subject_member(member, &format!("members[id={}].id", member.id)),
                        loc("Bridge member interaction", "Brückenbauteil Interaktion"),
                    )
                    .annex(annex)
                    .utilization(dimensionless(eta), dimensionless(1.0))
                    .explanation(loc(
                        &format!("Bridge interaction η={:.3} (gov. {}).", eta, gov.combination_id),
                        &format!("Brücken-Interaktion η={:.3} (maßgebende Kombination {}).", eta, gov.combination_id),
                    ));
                    if eta > 1.0 {
                        let options = next_section_options(section, section.area * eta, section.w_pl_y * eta);
                        b = b.remedy(Remedy::one_of(
                            subject_member(member, &format!("members[id={}].sectionId", member.id)),
                            options.clone(),
                            loc(&format!("Upsize: {}.", options.join(", ")), &format!("Querschnitt vergrößern: {}.", options.join(", "))),
                        ));
                    }
                    report.push(b.build());
                }
            }
        }
    }

    for tower in &document.tower_legs {
        let Some(member) = document.members.iter().find(|m| m.id == tower.member_id) else { continue };
        let Some(section) = find_section(document, &member.section_id) else { continue };
        let Some(material) = find_material(document, &member.material_id) else { continue };
        let forces: Vec<(String, f64)> = tower.actions.iter().map(|a| (a.load_case_id.clone(), a.force)).collect();
        let combos = combine_scalar_forces(document, &forces);
        let n_ed = governing_scalar(&combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let gov = governing_scalar(&combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");
        let iy = part_1_1::radius_of_gyration(section.iy, section.area);
        let curve = part_1_1::buckling_curve_rolled_i(section, 'y', material.fy);
        let lam = part_1_1::lambda_bar(member.buckling_length_y, iy, material.e_modulus, material.fy);
        let chi = part_1_1::chi(lam, curve);
        let n_rd = part_3::tower_buckling_n(section.area, material.fy, chi, tower.force_coefficient, tower.dynamic_factor, params);
        let mut b = CheckResult::assess(
            format!("en1993.3-1.tower.{}", tower.id),
            "DIN EN 1993-3-1",
            ClauseId::new("EN 1993-3-1", "5", "5"),
            SubjectRef::new(&tower.id, &format!("towerLegs[id={}].actions", tower.id), loc(&format!("Tower {}", tower.id), &format!("Turm {}", tower.id))),
            loc("Tower leg buckling", "Turmstiel Knicken"),
        )
        .annex(annex)
        .utilization(force_n(n_ed), force_n(n_rd))
        .explanation(loc(
            &format!("N_Ed={:.1} kN (gov. {}), N_b,Rd={:.1} kN (χ={:.3}, c_f={:.2}, c_d={:.2} Annex B).", n_ed / 1000.0, gov, n_rd / 1000.0, chi, tower.force_coefficient, tower.dynamic_factor),
            &format!("Turmstiel N_Ed={:.1} kN (maßgebend {}), N_b,Rd={:.1} kN (χ={:.3}, c_f={:.2}, c_d={:.2} Anhang B).", n_ed / 1000.0, gov, n_rd / 1000.0, chi, tower.force_coefficient, tower.dynamic_factor),
        ));
        if n_ed > n_rd {
            let options = next_section_options(section, section.area * n_ed / n_rd.max(1e-9), section.w_pl_y);
            b = b.remedy(Remedy::one_of(
                subject_member(member, &format!("members[id={}].sectionId", member.id)),
                options.clone(),
                loc(&format!("Upsize tower section: {}.", options.join(", ")), &format!("Turmquerschnitt vergrößern: {}.", options.join(", "))),
            ));
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&tower.id, &format!("towerLegs[id={}].forceCoefficient", tower.id), loc("Force coefficient", "Kraftbeiwert")),
                dimensionless(tower.force_coefficient),
                dimensionless((tower.force_coefficient * n_rd / n_ed.max(1e-9)).max(0.5)),
                loc(
                    &format!("Or reduce force coefficient c_f."),
                    &format!("Oder Kraftbeiwert c_f verringern."),
                ),
            ));
        }
        report.push(b.build());
    }

    for pile in &document.piles {
        let Some(section) = find_section(document, &pile.section_id) else { continue };
        let Some(material) = find_material(document, &pile.material_id) else { continue };
        let limit = part_5::pile_driving_stress_limit(material.fy);
        let mut b = CheckResult::assess(
            format!("en1993.5.drive.{}", pile.id),
            "DIN EN 1993-5",
            ClauseId::new("EN 1993-5", "12", "12.1"),
            SubjectRef::new(&pile.id, &format!("piles[id={}].drivingStress", pile.id), loc(&format!("Pile {}", pile.id), &format!("Pfahl {}", pile.id))),
            loc("Pile driving stress", "Rammspannung"),
        )
        .annex(annex)
        .utilization(stress_pa(pile.driving_stress), stress_pa(limit))
        .explanation(loc(
            &format!("σ={:.0} MPa, limit 0.9·f_y={:.0} MPa.", pile.driving_stress / 1e6, limit / 1e6),
            &format!("Rammspannung σ={:.0} MPa, Grenze 0.9·f_y={:.0} MPa.", pile.driving_stress / 1e6, limit / 1e6),
        ));
        if pile.driving_stress > limit {
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&pile.id, &format!("piles[id={}].drivingStress", pile.id), loc("Driving stress", "Rammspannung")),
                stress_pa(pile.driving_stress),
                stress_pa(limit),
                loc(
                    &format!("Reduce driving stress to ≤ {:.0} MPa.", limit / 1e6),
                    &format!("Rammspannung auf ≤ {:.0} MPa reduzieren.", limit / 1e6),
                ),
            ));
        }
        report.push(b.build());

        let k_red = part_5::pile_soil_reduction(pile.driving_stress, material.fy, pile.embedded_length, pile.shaft_perimeter);
        let n_rd = part_5::pile_compression_n(section.area, material.fy, k_red, params);
        let forces: Vec<(String, f64)> = pile.actions.iter().map(|a| (a.load_case_id.clone(), a.force)).collect();
        let combos = combine_scalar_forces(document, &forces);
        let n_ed = governing_scalar(&combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let gov = governing_scalar(&combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");
        let mut b = CheckResult::assess(
            format!("en1993.5.comp.{}", pile.id),
            "DIN EN 1993-5",
            ClauseId::new("EN 1993-5", "6", "6.1"),
            SubjectRef::new(&pile.id, &format!("piles[id={}].actions", pile.id), loc(&format!("Pile {}", pile.id), &format!("Pfahl {}", pile.id))),
            loc("Pile compression", "Pfahl Druck"),
        )
        .annex(annex)
        .utilization(force_n(n_ed), force_n(n_rd))
        .explanation(loc(
            &format!("N_Ed={:.1} kN (gov. {}), N_c,Rd={:.1} kN (k_red={:.2} derived from driving/geometry).", n_ed / 1000.0, gov, n_rd / 1000.0, k_red),
            &format!("Pfahl Druck N_Ed={:.1} kN (maßgebend {}), N_c,Rd={:.1} kN (k_red={:.2} aus Rammspannung/Geometrie abgeleitet).", n_ed / 1000.0, gov, n_rd / 1000.0, k_red),
        ));
        if n_ed > n_rd {
            let options = next_section_options(section, section.area * n_ed / n_rd.max(1e-9), section.w_pl_y);
            b = b.remedy(Remedy::one_of(
                SubjectRef::new(&pile.id, &format!("piles[id={}].sectionId", pile.id), loc("Pile section", "Pfahlquerschnitt")),
                options.clone(),
                loc(&format!("Upsize pile section: {}.", options.join(", ")), &format!("Pfahlquerschnitt vergrößern: {}.", options.join(", "))),
            ));
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&pile.id, &format!("piles[id={}].drivingStress", pile.id), loc("Driving stress", "Rammspannung")),
                stress_pa(pile.driving_stress),
                stress_pa(0.6 * material.fy),
                loc(
                    &format!("Or reduce driving stress to raise k_red."),
                    &format!("Oder Rammspannung senken, um k_red anzuheben."),
                ),
            ));
        }
        report.push(b.build());
    }

    for crane in &document.crane_runways {
        let forces: Vec<(String, f64)> = crane.actions.iter().map(|a| (a.load_case_id.clone(), a.force * crane.phi.max(1.0))).collect();
        let combos = combine_scalar_forces(document, &forces);
        let f_ed = governing_scalar(&combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let gov = governing_scalar(&combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");
        let l_eff = part_6::effective_length(crane.wheel_contact_length, crane.dispersion);
        let sigma = part_6::wheel_load_web_stress(f_ed, l_eff, crane.web_thickness);
        let limit = crane.fy / params.gamma_m0;
        let mut b = CheckResult::assess(
            format!("en1993.6.crane.{}", crane.id),
            "DIN EN 1993-6",
            ClauseId::new("EN 1993-6", "5", "5.7.1"),
            SubjectRef::new(&crane.id, &format!("craneRunways[id={}].webThickness", crane.id), loc(&format!("Crane {}", crane.id), &format!("Kranbahn {}", crane.id))),
            loc("Crane runway web stress", "Kranbahn Stegspannung"),
        )
        .annex(annex)
        .utilization(stress_pa(sigma), stress_pa(limit))
        .explanation(loc(
            &format!("σ_oz={:.1} MPa (gov. {}, φ={:.2}), f_y/γ_M0={:.1} MPa.", sigma / 1e6, gov, crane.phi, limit / 1e6),
            &format!("Kranbahn-Stegspannung σ_oz={:.1} MPa (maßgebend {}, φ={:.2}), f_y/γ_M0={:.1} MPa.", sigma / 1e6, gov, crane.phi, limit / 1e6),
        ));
        if sigma > limit {
            let t_req = f_ed / (l_eff * limit);
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&crane.id, &format!("craneRunways[id={}].webThickness", crane.id), loc("Web thickness", "Stegdicke")),
                length_m(crane.web_thickness),
                length_m(t_req),
                loc(
                    &format!("Increase web thickness to at least {:.1} mm.", t_req * 1000.0),
                    &format!("Stegdicke auf mindestens {:.1} mm erhöhen.", t_req * 1000.0),
                ),
            ));
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&crane.id, &format!("craneRunways[id={}].phi", crane.id), loc("Dynamic factor", "Dynamikfaktor")),
                dimensionless(crane.phi),
                dimensionless((crane.phi * limit / sigma.max(1e-9)).max(1.0)),
                loc(
                    &format!("Or reduce dynamic factor φ."),
                    &format!("Oder Dynamikfaktor φ verringern."),
                ),
            ));
        }
        report.push(b.build());
    }
}
