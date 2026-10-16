//! 🔗️ EN 1993 special element checks: joints, fatigue, fire, cold-formed members, plated panels, shells, tension components, bridges, towers, piles and crane runways.

use super::*;

//#region 🔖️ComplianceHelpers
/// 🔩 The joint, fatigue, fire, cold-formed, stainless, plated, shell, through-thickness, tension, high-strength, bridge, tower, pile and crane checks of `check_full_steel_structure`, appended to the same report.
pub(super) fn check_special_elements(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, report: &mut CheckReport) {
    // Joints 1-8
    for joint in &document.joints {
        let shear_forces: Vec<(String, f64)> = joint.actions.iter().map(|a| (a.load_case_id.clone(), a.shear)).collect();
        let tension_forces: Vec<(String, f64)> = joint.actions.iter().map(|a| (a.load_case_id.clone(), a.tension)).collect();
        let shear_combos = combine_scalar_forces(document, &shear_forces);
        let tension_combos = combine_scalar_forces(document, &tension_forces);
        let shear_ed = governing_scalar(&shear_combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let tension_ed = governing_scalar(&tension_combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let shear_gov = governing_scalar(&shear_combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");
        let tension_gov = governing_scalar(&tension_combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");

        if joint.kind == "bolted" {
            let n_bolts = joint.bolt_rows * joint.bolts_per_row;
            let a_s = part_1_8::bolt_as(joint.bolt_diameter);
            let f_ub = part_1_8::bolt_fub(&joint.bolt_class);
            let d0 = joint.bolt_diameter + 0.002;
            let p1 = joint.pitch;
            let p2 = if joint.gauge > 0.0 { joint.gauge } else { joint.pitch };
            let fv = part_1_8::bolt_shear_resistance_n(n_bolts, a_s, f_ub, joint.shear_planes.max(1), params.gamma_m2);
            let alpha_b = part_1_8::bearing_alpha_b(joint.end_distance, p1, d0, f_ub, joint.plate_fu);
            let k1 = part_1_8::bearing_k1(joint.edge_distance, p2, d0);
            let fb = part_1_8::bolt_bearing_resistance_n(k1, alpha_b, joint.plate_fu, joint.bolt_diameter, joint.plate_thickness, n_bolts, params.gamma_m2);
            let ft = part_1_8::bolt_tension_resistance_n(n_bolts, a_s, f_ub, params.gamma_m2);

            let mut b = CheckResult::assess(
                format!("en1993.1-8.3.6.shear.{}", joint.id),
                "DIN EN 1993-1-8",
                ClauseId::new("EN 1993-1-8", "3", "3.6.1"),
                subject_joint(joint, &format!("joints[id={}].actions", joint.id)),
                loc("Bolt shear", "Schraubenschub"),
            )
            .annex(annex)
            .utilization(force_n(shear_ed), force_n(fv))
            .explanation(loc(
                &format!("F_v,Ed={:.1} kN (gov. {}), F_v,Rd={:.1} kN ({}×M{:.0} {}).", shear_ed / 1000.0, shear_gov, fv / 1000.0, n_bolts, joint.bolt_diameter * 1000.0, joint.bolt_class),
                &format!("Schraubenschub F_v,Ed={:.1} kN (maßgebend {}), F_v,Rd={:.1} kN ({}×M{:.0} {}).", shear_ed / 1000.0, shear_gov, fv / 1000.0, n_bolts, joint.bolt_diameter * 1000.0, joint.bolt_class),
            ));
            if shear_ed > fv {
                let n_req = ((shear_ed * params.gamma_m2 / (joint.shear_planes.max(1) as f64 * 0.6 * a_s * f_ub)).ceil() as u32).max(1);
                b = b.remedy(Remedy::at_least(
                    subject_joint(joint, &format!("joints[id={}].boltRows", joint.id)),
                    dimensionless(joint.bolt_rows as f64),
                    dimensionless((n_req / joint.bolts_per_row.max(1)).max(1) as f64),
                    loc(
                        &format!("Increase to at least {} bolts (e.g. {} rows × {}).", n_req, (n_req + joint.bolts_per_row - 1) / joint.bolts_per_row.max(1), joint.bolts_per_row),
                        &format!("Mindestens {} Schrauben vorsehen (z. B. {} Reihen × {}).", n_req, (n_req + joint.bolts_per_row - 1) / joint.bolts_per_row.max(1), joint.bolts_per_row),
                    ),
                ));
                let d_opts = ["0.022", "0.024", "0.027", "0.030"].iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
                b = b.remedy(Remedy::one_of(
                    subject_joint(joint, &format!("joints[id={}].boltDiameter", joint.id)),
                    d_opts.clone(),
                    loc(
                        &format!("Or increase bolt diameter (m): {}.", d_opts.join(", ")),
                        &format!("Oder Schraubendurchmesser erhöhen (m): {}.", d_opts.join(", ")),
                    ),
                ));
            }
            report.push(b.build());

            let mut b = CheckResult::assess(
                format!("en1993.1-8.3.6.bearing.{}", joint.id),
                "DIN EN 1993-1-8",
                ClauseId::new("EN 1993-1-8", "3", "3.6.1"),
                subject_joint(joint, &format!("joints[id={}].pitch", joint.id)),
                loc("Bolt bearing", "Lochleibung"),
            )
            .annex(annex)
            .utilization(force_n(shear_ed), force_n(fb))
            .explanation(loc(
                &format!("F_b,Ed={:.1} kN (gov. {}), F_b,Rd={:.1} kN (α_b={:.2} from e1/p1, k1={:.2} from e2/p2).", shear_ed / 1000.0, shear_gov, fb / 1000.0, alpha_b, k1),
                &format!("Lochleibung F_b,Ed={:.1} kN (maßgebend {}), F_b,Rd={:.1} kN (α_b={:.2} aus e1/p1, k1={:.2} aus e2/p2).", shear_ed / 1000.0, shear_gov, fb / 1000.0, alpha_b, k1),
            ));
            if shear_ed > fb {
                let need_alpha = shear_ed * params.gamma_m2 / (k1.max(0.1) * joint.plate_fu * joint.bolt_diameter * joint.plate_thickness * n_bolts as f64);
                let e1_req = (need_alpha.min(1.0) * 3.0 * d0).max(joint.end_distance);
                b = b.remedy(Remedy::at_least(
                    subject_joint(joint, &format!("joints[id={}].endDistance", joint.id)),
                    length_m(joint.end_distance),
                    length_m(e1_req),
                    loc(
                        &format!("Increase end distance e1 from {:.0} mm to at least {:.0} mm.", joint.end_distance * 1000.0, e1_req * 1000.0),
                        &format!("Randabstand e1 von {:.0} mm auf mindestens {:.0} mm erhöhen.", joint.end_distance * 1000.0, e1_req * 1000.0),
                    ),
                ));
                let p1_req = ((need_alpha.min(1.0) + 0.25) * 3.0 * d0).max(joint.pitch);
                b = b.remedy(Remedy::at_least(
                    subject_joint(joint, &format!("joints[id={}].pitch", joint.id)),
                    length_m(joint.pitch),
                    length_m(p1_req),
                    loc(
                        &format!("Or increase pitch p1 to at least {:.0} mm.", p1_req * 1000.0),
                        &format!("Oder Lochabstand p1 auf mindestens {:.0} mm erhöhen.", p1_req * 1000.0),
                    ),
                ));
            }
            report.push(b.build());

            if tension_ed > 0.0 {
                let mut b = CheckResult::assess(
                    format!("en1993.1-8.3.6.tension.{}", joint.id),
                    "DIN EN 1993-1-8",
                    ClauseId::new("EN 1993-1-8", "3", "3.6.1"),
                    subject_joint(joint, &format!("joints[id={}].actions", joint.id)),
                    loc("Bolt tension", "Schrauben Zug"),
                )
                .annex(annex)
                .utilization(force_n(tension_ed), force_n(ft))
                .explanation(loc(
                    &format!("F_t,Ed={:.1} kN (gov. {}), F_t,Rd={:.1} kN.", tension_ed / 1000.0, tension_gov, ft / 1000.0),
                    &format!("Schrauben Zug F_t,Ed={:.1} kN (maßgebend {}), F_t,Rd={:.1} kN.", tension_ed / 1000.0, tension_gov, ft / 1000.0),
                ));
                if tension_ed > ft {
                    let n_req = ((tension_ed / (0.9 * a_s * f_ub / params.gamma_m2)).ceil() as u32).max(1);
                    b = b.remedy(Remedy::at_least(
                        subject_joint(joint, &format!("joints[id={}].boltRows", joint.id)),
                        dimensionless(joint.bolt_rows as f64),
                        dimensionless((n_req / joint.bolts_per_row.max(1)).max(1) as f64),
                        loc(
                            &format!("Increase bolt count to at least {} for tension.", n_req),
                            &format!("Schraubenanzahl für Zug auf mindestens {} erhöhen.", n_req),
                        ),
                    ));
                }
                report.push(b.build());
            }

            let cat = joint.category.to_ascii_uppercase();
            if cat == "B" || cat == "C" {
                let f_p_c = part_1_8::preload_force_n(a_s, f_ub, joint.preload_force);
                let fs = part_1_8::slip_resistance_n(
                    joint.slip_factor_ks.max(0.1),
                    joint.friction_surfaces.max(1),
                    joint.friction_mu.max(0.1),
                    f_p_c,
                    n_bolts.max(1),
                    params.gamma_m3,
                );
                let mut b = CheckResult::assess(
                    format!("en1993.1-8.3.9.slip.{}", joint.id),
                    "DIN EN 1993-1-8",
                    ClauseId::new("EN 1993-1-8", "3", "3.9"),
                    subject_joint(joint, &format!("joints[id={}].frictionMu", joint.id)),
                    loc("Slip resistance", "Gleitfestigkeit"),
                )
                .annex(annex)
                .utilization(force_n(shear_ed), force_n(fs))
                .explanation(loc(
                    &format!("F_s,Ed={:.1} kN (gov. {}), F_s,Rd={:.1} kN (cat. {}, k_s={:.2}, n={}, μ={:.2}, F_p,C={:.1} kN, γ_M3={}).", shear_ed / 1000.0, shear_gov, fs / 1000.0, cat, joint.slip_factor_ks, joint.friction_surfaces, joint.friction_mu, f_p_c / 1000.0, params.gamma_m3),
                    &format!("Gleitfestigkeit F_s,Ed={:.1} kN (maßgebend {}), F_s,Rd={:.1} kN (Kat. {}, k_s={:.2}, n={}, μ={:.2}, F_p,C={:.1} kN, γ_M3={}).", shear_ed / 1000.0, shear_gov, fs / 1000.0, cat, joint.slip_factor_ks, joint.friction_surfaces, joint.friction_mu, f_p_c / 1000.0, params.gamma_m3),
                ));
                if shear_ed > fs {
                    let mu_req = shear_ed * params.gamma_m3 / (joint.slip_factor_ks.max(0.1) * joint.friction_surfaces.max(1) as f64 * f_p_c * n_bolts.max(1) as f64);
                    b = b.remedy(Remedy::at_least(
                        subject_joint(joint, &format!("joints[id={}].frictionMu", joint.id)),
                        dimensionless(joint.friction_mu),
                        dimensionless(mu_req.min(0.5)),
                        loc(
                            &format!("Increase friction coefficient μ to at least {:.2}.", mu_req.min(0.5)),
                            &format!("Reibungszahl μ auf mindestens {:.2} erhöhen.", mu_req.min(0.5)),
                        ),
                    ));
                    let fp_req = shear_ed * params.gamma_m3 / (joint.slip_factor_ks.max(0.1) * joint.friction_surfaces.max(1) as f64 * joint.friction_mu.max(0.1) * n_bolts.max(1) as f64);
                    b = b.remedy(Remedy::at_least(
                        subject_joint(joint, &format!("joints[id={}].preloadForce", joint.id)),
                        force_n(joint.preload_force),
                        force_n(fp_req),
                        loc(
                            &format!("Increase preload F_p,C to at least {:.1} kN.", fp_req / 1000.0),
                            &format!("Vorspannkraft F_p,C auf mindestens {:.1} kN erhöhen.", fp_req / 1000.0),
                        ),
                    ));
                }
                report.push(b.build());
            }
        } else if joint.kind == "welded" {
            let beta = part_1_8::beta_w(&joint.weld_grade);
            let fw_s = part_1_8::fillet_weld_simplified_n(joint.weld_throat, joint.weld_length, joint.weld_fu, beta, params.gamma_m2);
            let (f_ed_dir, fw_d) = part_1_8::fillet_weld_directional_n(joint.weld_throat, joint.weld_length, joint.weld_fu, beta, params.gamma_m2, shear_ed);
            let mut b = CheckResult::assess(
                format!("en1993.1-8.4.5.simplified.{}", joint.id),
                "DIN EN 1993-1-8",
                ClauseId::new("EN 1993-1-8", "4", "4.5.3.3"),
                subject_joint(joint, &format!("joints[id={}].weldThroat", joint.id)),
                loc("Fillet weld simplified method", "Kehlnaht vereinfachtes Verfahren"),
            )
            .annex(annex)
            .utilization(force_n(shear_ed), force_n(fw_s))
            .explanation(loc(
                &format!("F_w,Ed={:.1} kN (gov. {}), F_w,Rd={:.1} kN (a={:.1} mm, ℓ={:.0} mm).", shear_ed / 1000.0, shear_gov, fw_s / 1000.0, joint.weld_throat * 1000.0, joint.weld_length * 1000.0),
                &format!("Kehlnaht (vereinfacht) F_w,Ed={:.1} kN (maßgebend {}), F_w,Rd={:.1} kN (a={:.1} mm, ℓ={:.0} mm).", shear_ed / 1000.0, shear_gov, fw_s / 1000.0, joint.weld_throat * 1000.0, joint.weld_length * 1000.0),
            ));
            if shear_ed > fw_s {
                let mut a_req = shear_ed * beta * 3.0_f64.sqrt() * params.gamma_m2 / (joint.weld_fu * joint.weld_length.max(1e-6));
                while part_1_8::fillet_weld_simplified_n(a_req, joint.weld_length, joint.weld_fu, beta, params.gamma_m2) < shear_ed {
                    a_req = a_req.next_up();
                }
                b = b.remedy(Remedy::at_least(
                    subject_joint(joint, &format!("joints[id={}].weldThroat", joint.id)),
                    length_m(joint.weld_throat),
                    length_m(a_req),
                    loc(
                        &format!("Increase weld throat a from {:.1} mm to at least {:.1} mm.", joint.weld_throat * 1000.0, a_req * 1000.0),
                        &format!("Nahtdicke a von {:.1} mm auf mindestens {:.1} mm erhöhen.", joint.weld_throat * 1000.0, a_req * 1000.0),
                    ),
                ));
            }
            report.push(b.build());

            let mut b = CheckResult::assess(
                format!("en1993.1-8.4.5.directional.{}", joint.id),
                "DIN EN 1993-1-8",
                ClauseId::new("EN 1993-1-8", "4", "4.5.3.2"),
                subject_joint(joint, &format!("joints[id={}].weldThroat", joint.id)),
                loc("Fillet weld directional method", "Kehlnaht gerichtetes Verfahren"),
            )
            .annex(annex)
            .utilization(force_n(f_ed_dir), force_n(fw_d))
            .explanation(loc(
                &format!("Directional σ_eq·A={:.1} kN (gov. {}), f_w,d·A={:.1} kN.", f_ed_dir / 1000.0, shear_gov, fw_d / 1000.0),
                &format!("Gerichtetes Verfahren σ_eq·A={:.1} kN (maßgebend {}), f_w,d·A={:.1} kN.", f_ed_dir / 1000.0, shear_gov, fw_d / 1000.0),
            ));
            if f_ed_dir > fw_d {
                let mut a_req = joint.weld_throat * (f_ed_dir / fw_d.max(1e-9));
                loop {
                    let (ed, rd) = part_1_8::fillet_weld_directional_n(a_req, joint.weld_length, joint.weld_fu, beta, params.gamma_m2, shear_ed);
                    if ed <= rd {
                        break;
                    }
                    a_req = a_req.next_up();
                }
                b = b.remedy(Remedy::at_least(
                    subject_joint(joint, &format!("joints[id={}].weldThroat", joint.id)),
                    length_m(joint.weld_throat),
                    length_m(a_req),
                    loc(
                        &format!("Increase weld throat a to at least {:.1} mm.", a_req * 1000.0),
                        &format!("Nahtdicke a auf mindestens {:.1} mm erhöhen.", a_req * 1000.0),
                    ),
                ));
            }
            report.push(b.build());
        }
    }

    // Fatigue 1-9 — Palmgren-Miner
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

    // Fire 1-2
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
        // χ_fi uses member fy if available else S355
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

    // Cold-formed 1-3
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

    // Stainless materials checked via members with kind=stainless
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

    // Plated 1-5
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

    // Shell / silo 1-6 + 4-1
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

    // 1-10 through-thickness
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

    // 1-11 tension components
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

    // 1-12 HSS materials
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

    // Bridge 2
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

    // Tower 3-1
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

    // Piles 5
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

    // Crane 6
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
//#endregion 🔖️ComplianceHelpers
