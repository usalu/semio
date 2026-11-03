//! 🔩️ Joint checks: the bolted and welded joint checks of EN 1993-1-8. See [`super`].

use super::*;

/// 🔩 Checks every joint: bolted shear, bearing and tension and welded joint resistances.
pub(super) fn check_joints(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, scope: Scope<'_>, report: &mut CheckReport) {
    for joint in document.joints.iter().filter(|e| scope.admits("joints", &e.id)) {
        let shear_forces: Vec<(String, f64)> = joint.actions.iter().map(|a| (a.load_case_id.clone(), a.shear)).collect();
        let tension_forces: Vec<(String, f64)> = joint.actions.iter().map(|a| (a.load_case_id.clone(), a.tension)).collect();
        let shear_combos = combine_scalar_forces(document, &shear_forces);
        let tension_combos = combine_scalar_forces(document, &tension_forces);
        let shear_ed = governing_scalar(&shear_combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let tension_ed = governing_scalar(&tension_combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let shear_gov = governing_scalar(&shear_combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");
        let tension_gov = governing_scalar(&tension_combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");

        if joint.kind == semio_s_artifact_norm_en1993_model::JointKind::Bolted {
            let n_bolts = joint.bolt_rows * joint.bolts_per_row;
            let a_s = part_1_8::bolt_as(joint.bolt_diameter);
            let f_ub = part_1_8::bolt_fub(&joint.bolt_class);
            let d0 = crate::clauses::joints::hole_diameter(joint.bolt_diameter * 1000.0) / 1000.0;
            let alpha_v = crate::clauses::joints::alpha_v(&joint.bolt_class, true);
            let p1 = joint.pitch;
            let p2 = if joint.gauge > 0.0 { joint.gauge } else { joint.pitch };
            let fv = part_1_8::bolt_shear_resistance_n(n_bolts, a_s, f_ub, joint.shear_planes.max(1), alpha_v, params.gamma_m2);
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
            .ndp(AnnexParams::GAMMA_M2)
            .utilization(force_n(shear_ed), force_n(fv))
            .explanation(loc(
                &format!("F_v,Ed={:.1} kN (gov. {}), F_v,Rd={:.1} kN ({}×M{:.0} {}).", shear_ed / 1000.0, shear_gov, fv / 1000.0, n_bolts, joint.bolt_diameter * 1000.0, joint.bolt_class),
                &format!("Schraubenschub F_v,Ed={:.1} kN (maßgebend {}), F_v,Rd={:.1} kN ({}×M{:.0} {}).", shear_ed / 1000.0, shear_gov, fv / 1000.0, n_bolts, joint.bolt_diameter * 1000.0, joint.bolt_class),
            ));
            if shear_ed > fv {
                let n_req = ((shear_ed * params.gamma_m2 / (joint.shear_planes.max(1) as f64 * alpha_v * a_s * f_ub)).ceil() as u32).max(1);
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
            .ndp(AnnexParams::GAMMA_M2)
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
                .ndp(AnnexParams::GAMMA_M2)
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

            if tension_ed > 0.0 && shear_ed > 0.0 {
                let eta = crate::clauses::joints::shear_tension_utilisation(shear_ed, fv, tension_ed, ft);
                let mut interaction =
                    CheckResult::assess(
                        format!("en1993.1-8.3.6.interaction.{}", joint.id),
                        "DIN EN 1993-1-8",
                        ClauseId::new("EN 1993-1-8", "3", "3.6.1/Table 3.4"),
                        subject_joint(joint, &format!("joints[id={}].actions", joint.id)),
                        loc("Bolt shear and tension interaction", "Schrauben Abscheren und Zug"),
                    )
                    .annex(annex)
                    .ndp(AnnexParams::GAMMA_M2)
                    .utilization(dimensionless(eta), dimensionless(1.0))
                    .explanation(loc(
                        &format!("F_v,Ed/F_v,Rd + F_t,Ed/(1.4 F_t,Rd) = {:.3}.", eta),
                        &format!("F_v,Ed/F_v,Rd + F_t,Ed/(1,4 F_t,Rd) = {:.3}.", eta),
                    ));
                if !(eta <= 1.0) {
                    let k = if eta.is_finite() { eta } else { 2.0 };
                    let n_req = ((joint.bolt_rows as f64) * k).ceil();
                    interaction = interaction.remedy(Remedy::at_least(
                        subject_joint(joint, &format!("joints[id={}].boltRows", joint.id)),
                        dimensionless(joint.bolt_rows as f64),
                        dimensionless(n_req),
                        loc(&format!("Increase the bolt rows to at least {n_req:.0} for the combined shear and tension."), &format!("Schraubenreihen auf mindestens {n_req:.0} für Abscheren und Zug erhöhen.")),
                    ));
                }
                report.push(interaction.build());
            }

            for sp in crate::clauses::joints::spacing_checks(joint.end_distance * 1000.0, joint.edge_distance * 1000.0, p1 * 1000.0, p2 * 1000.0, d0 * 1000.0, joint.plate_thickness * 1000.0) {
                let (value, limit) = (sp.value, sp.limit);
                if value <= 0.0 {
                    continue;
                }
                let builder = CheckResult::assess(
                    format!("en1993.1-8.3.5.{}.{}", sp.name, joint.id),
                    "DIN EN 1993-1-8",
                    ClauseId::new("EN 1993-1-8", "3", "3.5/Table 3.3"),
                    subject_joint(joint, &format!("joints[id={}].pitch", joint.id)),
                    loc(&format!("Bolt spacing {}", sp.name), &format!("Lochabstand {}", sp.name)),
                )
                .annex(annex)
                .explanation(loc(
                    &format!("{}: {:.1} mm against {:.1} mm.", sp.name, value, limit),
                    &format!("{}: {:.1} mm gegenüber {:.1} mm.", sp.name, value, limit),
                ));
                let mut spacing = if sp.is_minimum { builder.minimum(length_m(value / 1000.0), length_m(limit / 1000.0)) } else { builder.utilization(length_m(value / 1000.0), length_m(limit / 1000.0)) };
                if !sp.ok {
                    let (leaf, label_en, label_de) = match sp.name.split(',').next().unwrap_or("") {
                        "e1" => ("endDistance", "end distance e1", "Endabstand e1"),
                        "e2" => ("edgeDistance", "edge distance e2", "Randabstand e2"),
                        "p1" => ("pitch", "pitch p1", "Lochabstand p1"),
                        _ if joint.gauge > 0.0 => ("gauge", "gauge p2", "Wurzelmaß p2"),
                        _ => ("pitch", "pitch p2", "Lochabstand p2"),
                    };
                    let target = subject_joint(joint, &format!("joints[id={}].{leaf}", joint.id));
                    let (current, required) = (length_m(value / 1000.0), length_m(limit / 1000.0));
                    spacing = spacing.remedy(if sp.is_minimum {
                        Remedy::at_least(target, current, required, loc(&format!("Increase the {label_en} from {value:.1} mm to at least {limit:.1} mm."), &format!("{label_de} von {value:.1} mm auf mindestens {limit:.1} mm erhöhen.")))
                    } else {
                        Remedy::at_most(target, current, required, loc(&format!("Reduce the {label_en} from {value:.1} mm to at most {limit:.1} mm."), &format!("{label_de} von {value:.1} mm auf höchstens {limit:.1} mm verringern.")))
                    });
                }
                report.push(spacing.build());
            }

            let cat = joint.category.as_str().to_ascii_uppercase();
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
                .ndp(AnnexParams::GAMMA_M3)
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
        } else if joint.kind == semio_s_artifact_norm_en1993_model::JointKind::Welded {
            let beta = part_1_8::beta_w(&joint.weld_grade);
            let weld_par = if joint.weld_force_longitudinal != 0.0 { joint.weld_force_longitudinal.abs() } else { shear_ed };
            let weld_perp = joint.weld_force_transverse;
            let fw_s = part_1_8::fillet_weld_simplified_n(joint.weld_throat, joint.weld_length, joint.weld_fu, beta, params.gamma_m2);
            let weld_ed = weld_par.hypot(weld_perp);
            let (f_ed_dir, fw_d) = part_1_8::fillet_weld_directional_n(joint.weld_throat, joint.weld_length, joint.weld_fu, beta, params.gamma_m2, weld_par, weld_perp);
            let (sigma_perp, tau_perp, tau_par) = crate::clauses::joints::weld_throat_stresses(weld_par, weld_perp, std::f64::consts::FRAC_PI_4, joint.weld_throat, joint.weld_length);
            let (crit_equiv, crit_normal) = crate::clauses::joints::weld_directional_criteria(sigma_perp, tau_perp, tau_par, joint.weld_fu, beta, params.gamma_m2);
            let mut b = CheckResult::assess(
                format!("en1993.1-8.4.5.simplified.{}", joint.id),
                "DIN EN 1993-1-8",
                ClauseId::new("EN 1993-1-8", "4", "4.5.3.3"),
                subject_joint(joint, &format!("joints[id={}].weldThroat", joint.id)),
                loc("Fillet weld simplified method", "Kehlnaht vereinfachtes Verfahren"),
            )
            .annex(annex)
            .ndp(AnnexParams::GAMMA_M2)
            .utilization(force_n(weld_ed), force_n(fw_s))
            .explanation(loc(
                &format!("F_w,Ed={:.1} kN (resultant of shear {:.1} kN and transverse {:.1} kN, gov. {}), F_w,Rd={:.1} kN (a={:.1} mm, ℓ={:.0} mm).", weld_ed / 1000.0, weld_par / 1000.0, weld_perp / 1000.0, shear_gov, fw_s / 1000.0, joint.weld_throat * 1000.0, joint.weld_length * 1000.0),
                &format!("Kehlnaht (vereinfacht) F_w,Ed={:.1} kN (Resultierende aus Längskraft {:.1} kN und Querkraft {:.1} kN, maßgebend {}), F_w,Rd={:.1} kN (a={:.1} mm, ℓ={:.0} mm).", weld_ed / 1000.0, weld_par / 1000.0, weld_perp / 1000.0, shear_gov, fw_s / 1000.0, joint.weld_throat * 1000.0, joint.weld_length * 1000.0),
            ));
            if weld_ed > fw_s {
                let mut a_req = weld_ed * beta * 3.0_f64.sqrt() * params.gamma_m2 / (joint.weld_fu * joint.weld_length.max(1e-6));
                while part_1_8::fillet_weld_simplified_n(a_req, joint.weld_length, joint.weld_fu, beta, params.gamma_m2) < weld_ed {
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
            .ndp(AnnexParams::GAMMA_M2)
            .utilization(force_n(f_ed_dir), force_n(fw_d))
            .explanation(loc(
                &format!("Directional method: σ⊥={:.1}, τ⊥={:.1}, τ∥={:.1} N/mm² from the longitudinal {:.1} kN and transverse {:.1} kN (gov. {}); [σ⊥²+3(τ⊥²+τ∥²)]^0.5 ≤ f_u/(β_w γ_M2) at {:.3}, σ⊥ ≤ 0.9 f_u/γ_M2 at {:.3}; σ_eq·A={:.1} kN, f_w,d·A={:.1} kN.", sigma_perp / 1e6, tau_perp / 1e6, tau_par / 1e6, weld_par / 1000.0, weld_perp / 1000.0, shear_gov, crit_equiv, crit_normal, f_ed_dir / 1000.0, fw_d / 1000.0),
                &format!("Gerichtetes Verfahren: σ⊥={:.1}, τ⊥={:.1}, τ∥={:.1} N/mm² aus Längskraft {:.1} kN und Querkraft {:.1} kN (maßgebend {}); [σ⊥²+3(τ⊥²+τ∥²)]^0,5 ≤ f_u/(β_w γ_M2) mit {:.3}, σ⊥ ≤ 0,9 f_u/γ_M2 mit {:.3}; σ_eq·A={:.1} kN, f_w,d·A={:.1} kN.", sigma_perp / 1e6, tau_perp / 1e6, tau_par / 1e6, weld_par / 1000.0, weld_perp / 1000.0, shear_gov, crit_equiv, crit_normal, f_ed_dir / 1000.0, fw_d / 1000.0),
            ));
            if f_ed_dir > fw_d {
                let mut a_req = joint.weld_throat * (f_ed_dir / fw_d.max(1e-9));
                loop {
                    let (ed, rd) = part_1_8::fillet_weld_directional_n(a_req, joint.weld_length, joint.weld_fu, beta, params.gamma_m2, weld_par, weld_perp);
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
        super::special_parts_checks::check_joint(annex, params, joint, super::plate_checks::column_flange_width(document, joint), &super::joint_row_checks::rows_of(document, joint, params), report);
    }
}
