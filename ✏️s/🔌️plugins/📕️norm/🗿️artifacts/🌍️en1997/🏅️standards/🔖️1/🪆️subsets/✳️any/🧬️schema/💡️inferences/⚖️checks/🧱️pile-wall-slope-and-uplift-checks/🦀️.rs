//! 🧱️ EN 1997 pile, retaining wall, overall stability and uplift checks.

use super::*;

/// 🧱️ Pile, retaining wall, overall stability and uplift checks of [`check_project`]: the second half of its design loop, moved out unchanged.
#[allow(clippy::too_many_arguments, reason = "the scope joins the locals of the original check_project phase")]
pub fn check_piles_walls_slopes_and_uplift(doc: &En1997Snapshot, report: &mut CheckReport, approach: DesignApproach, annex: AnnexChoice, gwl: f64, part1: &str, part1054: &str, design_situations: &[String], scope: Scope<'_>, step: &mut dyn FnMut(f32) -> bool) {
    check_pile_resistances(doc, report, approach, annex, gwl, part1, design_situations, scope);
    check_retaining_walls(doc, report, approach, annex, gwl, part1, part1054, design_situations, scope);
    check_slopes(doc, report, annex, gwl, part1, scope, step);
    check_uplift(doc, report, annex, gwl, part1, design_situations, scope);
}

/// 🧗️ Design compressive resistance R_c,d of the pile group and the terms it rests on, from the test profiles or the model resistances: characteristic value min{mean/ξ3; min/ξ4; model} per component, partial factors of the pile type.
pub struct PileDesignResistance {
    pub r_c_d: f64,
    pub r_s_k: f64,
    pub gamma_b: f64,
    pub gamma_s: f64,
    pub xi3: f64,
    pub xi4: f64,
}

pub fn pile_design_resistance(pile: &semio_s_artifact_norm_en1997_model::Pile, approach: DesignApproach, p: &AnnexParams) -> PileDesignResistance {
    let a_base = std::f64::consts::PI * (pile.diameter * 0.5).powi(2);
    let r_s_model = part_1::shaft_resistance_n(pile.alpha_s, pile.diameter, pile.unit_shaft_resistance, pile.length);
    let r_b_model = part_1::base_resistance_n(pile.unit_base_resistance, a_base);
    let (r_s_mean, r_s_min, r_b_mean, r_b_min, n) = if pile.test_profiles.is_empty() {
        (r_s_model, r_s_model, r_b_model, r_b_model, 1u32)
    } else {
        let shafts: Vec<f64> = pile.test_profiles.iter().map(|t| t.shaft_resistance).collect();
        let bases: Vec<f64> = pile.test_profiles.iter().map(|t| t.base_resistance).collect();
        let n = shafts.len() as u32;
        (shafts.iter().sum::<f64>() / n as f64, shafts.iter().cloned().fold(f64::INFINITY, f64::min), bases.iter().sum::<f64>() / n as f64, bases.iter().cloned().fold(f64::INFINITY, f64::min), n)
    };
    let r_s_model_k = part_1::pile_characteristic_resistance_n(r_s_model, r_s_model, 1);
    let r_b_model_k = part_1::pile_characteristic_resistance_n(r_b_model, r_b_model, 1);
    let (gamma_b, gamma_s, xi3, xi4) = part_1::pile_type_factors(pile.pile_type.as_str(), n, p, approach);
    let r_b_k = (r_b_mean / xi3).min(r_b_min / xi4).min(r_b_model_k);
    let r_s_k = (r_s_mean / xi3).min(r_s_min / xi4).min(r_s_model_k);
    let r_c_d = (part_1::pile_design_resistance_typed(r_b_k, r_s_k, gamma_b, gamma_s) * pile.count as f64).max(1.0);
    PileDesignResistance { r_c_d, r_s_k, gamma_b, gamma_s, xi3, xi4 }
}

/// 🪵️ Checks the axial resistance of every pile.
fn check_pile_resistances(
    doc: &En1997Snapshot,
    report: &mut CheckReport,
    approach: DesignApproach,
    annex: AnnexChoice,
    gwl: f64,
    part1: &str,
    design_situations: &[String],
    scope: Scope<'_>,
) {
    for (_, pile) in doc.piles.iter().enumerate().filter(|(index, _)| scope.admits("piles", *index)) {
        let label = loc(&format!("Pile {}", pile.id), &format!("Pfahl {}", pile.id));
        entity_catalogue_cases::pile_settlement_catalogue(doc, pile, report);
        entity_catalogue_cases::pile_ground_catalogue(doc, pile, report);
        let path_l = id_path("piles", &pile.id, "length");
        let mut best_comp: Option<(f64, String, CheckResult)> = None;
        let mut best_ten: Option<(f64, CheckResult)> = None;
        for sit in design_situations.iter() {
            let p = resolve_params(approach, annex, sit);
            let PileDesignResistance { r_c_d, r_s_k: r_s_k_t, gamma_b: gamma_b_t, gamma_s: gamma_s_t, xi3: xi3_t, xi4: xi4_t } = pile_design_resistance(pile, approach, &p);
            let n_ed = design_actions(&p, pile.compression_permanent, pile.compression_variable);
            let u = if r_c_d > 1e-9 { n_ed / r_c_d } else { 0.0 };
            let sit_lbl = situation_short(sit);
            let mut comp = CheckResult::assess(
                format!("en1997.7.6.2.compression.{}", pile.id),
                part1,
                ClauseId::new("EN 1997-1", "7", "7.6.2"),
                subject_entity(&pile.id, &path_l, &label.en, &label.de),
                loc("Pile axial compression", "Pfahldruckwiderstand"),
            )
            .utilization(Quantity::new(QuantityKind::Force, n_ed), Quantity::new(QuantityKind::Force, r_c_d))
            .annex(annex)
            .explanation(loc(
                &format!("Governing sit={sit_lbl}; N_ed={:.0} kN vs R_c,d={:.0} kN (type={}, γ_b={:.2}, γ_s={:.2}, ξ3={:.2}, ξ4={:.2}).", n_ed / 1000.0, r_c_d / 1000.0, pile.pile_type, gamma_b_t, gamma_s_t, xi3_t, xi4_t),
                &format!("Maßgebend sit={sit_lbl}; N_ed={:.0} kN gegen R_c,d={:.0} kN (Typ={}, γ_b={:.2}, γ_s={:.2}, ξ3={:.2}, ξ4={:.2}).", n_ed / 1000.0, r_c_d / 1000.0, pile.pile_type, gamma_b_t, gamma_s_t, xi3_t, xi4_t),
            ));
            if n_ed > r_c_d {
                let n_req = ((n_ed / (r_c_d / pile.count.max(1) as f64)).ceil() as u32).max(pile.count);
                comp = comp.remedy(Remedy::at_least(
                    subject_entity(&pile.id, &id_path("piles", &pile.id, "count"), &label.en, &label.de),
                    Quantity::new(QuantityKind::Dimensionless, pile.count as f64),
                    Quantity::new(QuantityKind::Dimensionless, n_req as f64),
                    loc("Increase pile count or length for compression.", "Pfahlan zahl oder -länge für Druck erhöhen."),
                )).remedy(Remedy::at_least(
                    subject_entity(&pile.id, &path_l, &label.en, &label.de),
                    Quantity::length_m(pile.length),
                    Quantity::length_m(pile.length * n_ed / r_c_d.max(1.0)),
                    loc("Increase pile length to raise compression resistance.", "Pfahllänge erhöhen, um den Druckwiderstand zu steigern."),
                ));
            }
            comp = comp.field("pile-shaft", station_fields::pile_shaft(&doc.layers, gwl, pile.length, r_s_k_t / gamma_s_t * pile.count as f64, n_ed, u));
            let built = comp.ndps(&factor_ndp_ids(annex)).build();
            if best_comp.as_ref().map(|(bu, _, _)| u > *bu).unwrap_or(true) {
                best_comp = Some((u, sit.clone(), built));
            }
            let t_ed = design_actions(&p, pile.tension_permanent, pile.tension_variable);
            let r_t_d = (r_s_k_t / p.gamma_st * pile.count as f64).max(1.0);
            if t_ed > 1.0 || pile.tension_permanent > 0.0 || pile.tension_variable > 0.0 {
                let u_t = if r_t_d > 1e-9 { t_ed / r_t_d } else { 0.0 };
                let mut ten = CheckResult::assess(
                    format!("en1997.7.6.3.tension.{}", pile.id),
                    part1,
                    ClauseId::new("EN 1997-1", "7", "7.6.3"),
                    subject_entity(&pile.id, &path_l, &label.en, &label.de),
                    loc("Pile tension resistance", "Pfahlzugwiderstand"),
                )
                .utilization(Quantity::new(QuantityKind::Force, t_ed), Quantity::new(QuantityKind::Force, r_t_d))
                .annex(annex)
                .explanation(loc(
                    &format!("Sit={sit_lbl}; T_ed={:.0} kN vs R_t,d={:.0} kN.", t_ed / 1000.0, r_t_d / 1000.0),
                    &format!("Sit={sit_lbl}; T_ed={:.0} kN gegen R_t,d={:.0} kN.", t_ed / 1000.0, r_t_d / 1000.0),
                ));
                if t_ed > r_t_d {
                    let n_req = ((t_ed / (r_t_d / pile.count.max(1) as f64)).ceil() as u32).max(pile.count);
                    let path_tv = id_path("piles", &pile.id, "tensionVariable");
                    let t_var_cap = if pile.tension_variable > 1.0 {
                        let p_only = design_actions(&p, pile.tension_permanent, 0.0);
                        if p_only <= r_t_d {
                            (pile.tension_variable * (r_t_d - p_only).max(0.0) / (t_ed - p_only).max(1.0)).max(0.0)
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    };
                    ten = ten.remedy(Remedy::at_least(
                        subject_entity(&pile.id, &id_path("piles", &pile.id, "count"), &label.en, &label.de),
                        Quantity::new(QuantityKind::Dimensionless, pile.count as f64),
                        Quantity::new(QuantityKind::Dimensionless, n_req as f64),
                        loc("Increase pile count to raise tension resistance.", "Pfahlanzahl erhöhen, um den Zugwiderstand zu steigern."),
                    ));
                    if pile.tension_variable > t_var_cap + 1.0 {
                        ten = ten.remedy(Remedy::at_most(
                            subject_entity(&pile.id, &path_tv, &label.en, &label.de),
                            Quantity::new(QuantityKind::Force, pile.tension_variable),
                            Quantity::new(QuantityKind::Force, t_var_cap),
                            loc("Reduce variable tension action.", "Veränderliche Zugeinwirkung reduzieren."),
                        ));
                    }
                    if n_req <= pile.count {
                        let path_tp = id_path("piles", &pile.id, "tensionPermanent");
                        let t_perm_req = pile.tension_permanent * r_t_d / t_ed.max(1.0);
                        ten = ten.remedy(Remedy::at_most(
                            subject_entity(&pile.id, &path_tp, &label.en, &label.de),
                            Quantity::new(QuantityKind::Force, pile.tension_permanent),
                            Quantity::new(QuantityKind::Force, t_perm_req),
                            loc("Reduce permanent tension action.", "Ständige Zugeinwirkung reduzieren."),
                        ));
                    }
                }
                let tb = ten.ndps(&factor_ndp_ids(annex)).build();
                if best_ten.as_ref().map(|(bu, _)| u_t > *bu).unwrap_or(true) {
                    best_ten = Some((u_t, tb));
                }
            }
        }
        if let Some((u, sit, c)) = best_comp {
            let chk = c.id.clone();
            let chk_remedies = c.remedies.clone();
            report.push(c);
            push_governing_situation_summary(
                report,
                format!("en1997.governing.situation.pile.{}", pile.id),
                part1,
                annex,
                subject_entity(&pile.id, &path_l, &label.en, &label.de),
                &sit,
                doc.design_approach.as_str(),
                u,
                &chk,
                &chk_remedies,
            );
        }
        if let Some((_, c)) = best_ten { report.push(c); }
        lateral_and_seismic::check_pile(doc, pile, approach, annex, report);
    }

        if doc.retaining_walls.is_empty() && scope.part("wall-na") {
        report.push(
            CheckResult::assess("en1997.wall.na", part1, ClauseId::new("EN 1997-1", "9", "9.7"), SubjectRef::whole(loc("Project", "Projekt")), loc("Retaining wall checks", "Stützkonstruktionsnachweise"))
                .not_applicable(loc("No retaining walls in the subject.", "Keine Stützbauwerke im Gegenstand."))
                .annex(annex)
                .build(),
        );
    }
}

/// 🔩️ Checks the earth pressure, sliding and overturning of every retaining wall.
fn check_retaining_walls(
    doc: &En1997Snapshot,
    report: &mut CheckReport,
    approach: DesignApproach,
    annex: AnnexChoice,
    gwl: f64,
    part1: &str,
    part1054: &str,
    design_situations: &[String],
    scope: Scope<'_>,
) {
    for (_, wall) in doc.retaining_walls.iter().enumerate().filter(|(index, _)| scope.admits("retainingWalls", *index)) {
        let label = loc(&format!("Wall {}", wall.id), &format!("Wand {}", wall.id));
        let phi_w = wall.backfill_phi_deg.clamp(0.0, 50.0);
        let delta_w = wall.wall_friction_deg.clamp(0.0, phi_w);
        let geometry_fail = !(0.0..=50.0).contains(&wall.backfill_phi_deg) || !(0.0..=wall.backfill_phi_deg.clamp(0.0, 50.0)).contains(&wall.wall_friction_deg);
        let earth = |p: &AnnexParams| {
            let phi_d = (phi_w.to_radians().tan() / p.gamma_phi).atan().to_degrees();
            let delta_d = delta_w.min(phi_d);
            let (k, tag) = part_1::earth_pressure_coefficient(phi_d, wall.earth_pressure_mode.as_str(), wall.ocr, delta_d);
            let e_a = 0.5 * k * wall.backfill_gamma * wall.height.powi(2) + k * wall.surcharge * wall.height;
            let dr = if tag == "K0" { 0.0 } else { delta_d.to_radians() };
            (k, tag, e_a * dr.cos(), e_a * dr.sin(), phi_d, delta_d, dr)
        };
        let mut wall_sit = doc.design_situation.as_str().to_string();
        let mut wall_u = -1.0f64;
        for sit in design_situations.iter() {
            let p = resolve_params(approach, annex, sit);
            let (_, _, e_ah_try, e_av_try, phi_try, delta_try, _) = earth(&p);
            let h_try = design_actions(&p, wall.horizontal_permanent + e_ah_try, 0.0);
            let gamma_w_try = 9_810.0;
            let gamma_c_try = wall.concrete_gamma.max(1_000.0);
            let stem_vol = wall.stem_thickness * (wall.height + wall.base_width);
            let self_w = stem_vol * gamma_c_try;
            let v_try = wall.vertical_permanent + self_w + e_av_try;
            let a_try = wall.base_width.max(0.1);
            let gamma_p = if gwl < wall.embedment { (wall.backfill_gamma - gamma_w_try).max(1_000.0) } else { wall.backfill_gamma };
            let e_p_k = part_1::passive_earth_force_n(phi_try, gamma_p, wall.embedment, 1.0, delta_try);
            let r_slide = part_1::sliding_resistance_with_params(phi_w, 0.0, v_try, a_try, &p, 0.0, 0.0, wall.backfill_gamma, false) + 0.5 * e_p_k / p.gamma_r_e;
            let u = if r_slide > 1e-9 { h_try / r_slide } else { 0.0 };
            if u > wall_u {
                wall_u = u;
                wall_sit = sit.clone();
            }
        }
        let p_project = resolve_params(approach, annex, &wall_sit);
        let (k_ep, k_label, e_ah, e_av, phi_d, delta_d, delta_rad) = earth(&p_project);
        let h_d = design_actions(&p_project, wall.horizontal_permanent + e_ah, 0.0);
        let gamma_w = 9_810.0;
        let stem_below_gwl = (wall.embedment - gwl).max(0.0).min(wall.height + wall.embedment);
        let gamma_c = wall.concrete_gamma.max(1_000.0);
        let gamma_c_prime = (gamma_c - gamma_w).max(1_000.0);
        let stem_vol = wall.stem_thickness * (wall.height + wall.base_width);
        let self_weight = if gwl >= wall.height + wall.embedment {
            stem_vol * gamma_c
        } else if gwl <= 0.0 {
            stem_vol * gamma_c_prime
        } else {
            let frac_sub = (stem_below_gwl / (wall.height + wall.embedment).max(0.1)).clamp(0.0, 1.0);
            stem_vol * (gamma_c * (1.0 - frac_sub) + gamma_c_prime * frac_sub)
        };
        let v_k = wall.vertical_permanent + self_weight + e_av;
        let a = wall.base_width * 1.0;
        let gamma_p = if gwl < wall.embedment { (wall.backfill_gamma - gamma_w).max(1_000.0) } else { wall.backfill_gamma };
        let e_p_k = part_1::passive_earth_force_n(phi_d, gamma_p, wall.embedment, 1.0, delta_d);
        let r_passive = e_p_k / p_project.gamma_r_e;
        let r_base = part_1::sliding_resistance_with_params(phi_w, 0.0, v_k, a, &p_project, 0.0, 0.0, wall.backfill_gamma, false);
        let r_slide = r_base + 0.5 * r_passive;
        let path_b = id_path("retainingWalls", &wall.id, "baseWidth");
        let mut slide = CheckResult::assess(
            format!("en1997.9.sliding.{}", wall.id),
            part1,
            ClauseId::new("EN 1997-1", "9", "9.7.3"),
            subject_entity(&wall.id, &path_b, &label.en, &label.de),
            loc("Retaining wall sliding", "Gleiten der Stützwand"),
        )
        .utilization(Quantity::new(QuantityKind::Force, h_d), Quantity::new(QuantityKind::Force, r_slide))
        .annex(annex)
        .explanation(loc(
            &format!("Sit={}; H_d={:.0} kN, R_slide={:.0} kN, {k_label}={:.3}, embed={:.2} m, γ_R,e={:.2}.", situation_short(&wall_sit), h_d/1000.0, r_slide/1000.0, k_ep, wall.embedment, p_project.gamma_r_e),
            &format!("Sit={}; H_d={:.0} kN, R_Gleiten={:.0} kN, {k_label}={:.3}, Einbindung={:.2} m, γ_R,e={:.2}.", situation_short(&wall_sit), h_d/1000.0, r_slide/1000.0, k_ep, wall.embedment, p_project.gamma_r_e),
        ));
        if h_d > r_slide {
            let path_h = id_path("retainingWalls", &wall.id, "height");
            let path_e = id_path("retainingWalls", &wall.id, "embedment");
            let half_passive = 0.5 * r_passive;
            let width_req = if r_base > 1e-9 {
                wall.base_width * ((h_d - half_passive).max(0.0) / r_base).max(1.0) * 1.05
            } else {
                wall.base_width * h_d / r_slide.max(1.0) * 1.05
            };
            let e_p_need = 2.0 * (h_d - r_base).max(0.0) * p_project.gamma_r_e;
            let embed_req = if e_p_k > 1e-9 {
                (wall.embedment * (e_p_need / e_p_k).sqrt() * 1.05).max(wall.embedment)
            } else {
                (wall.embedment * 2.0).max(2.0)
            };
            let h_scale = (r_slide / h_d.max(1e-9)).sqrt().clamp(0.05, 1.0);
            let height_req = (wall.height * h_scale * 0.95).max(0.5);
            slide = slide.remedy(Remedy::at_least(
                subject_entity(&wall.id, &path_b, &label.en, &label.de),
                Quantity::length_m(wall.base_width),
                Quantity::length_m(width_req),
                loc("Widen wall base to raise sliding resistance.", "Sohlbreite erhöhen, um den Gleitwiderstand zu steigern."),
            )).remedy(Remedy::at_least(
                subject_entity(&wall.id, &path_e, &label.en, &label.de),
                Quantity::length_m(wall.embedment),
                Quantity::length_m(embed_req),
                loc("Increase wall embedment to raise passive resistance.", "Wand-Einbindung erhöhen, um den Passivwiderstand zu steigern."),
            )).remedy(Remedy::at_most(
                subject_entity(&wall.id, &path_h, &label.en, &label.de),
                Quantity::length_m(wall.height),
                Quantity::length_m(height_req),
                loc("Reduce retained height to lower active earth pressure.", "Stützhöhe reduzieren, um den aktiven Erddruck zu verringern."),
            ));
        }
        slide = slide.field("earth-pressure", station_fields::earth_pressure_profile(k_ep, wall.backfill_gamma, wall.surcharge, wall.height, delta_rad.cos(), if r_slide > 1e-9 { h_d / r_slide } else { 0.0 }));
        report.push(slide.ndps(&factor_ndp_ids(annex)).build());

        let path_mode = id_path("retainingWalls", &wall.id, "earthPressureMode");
        let path_move = id_path("retainingWalls", &wall.id, "wallMovement");
        let ka_r = part_1::ka_rankine(phi_w);
        let k0 = part_1::k0_jaky(phi_w, wall.ocr);
        let mode_key = wall.earth_pressure_mode.as_str().to_ascii_lowercase().replace(['-','_',' '], "");
        let move_key = wall.wall_movement.as_str().to_ascii_lowercase().replace(['-','_',' '], "");
        let mode_ok = matches!(mode_key.as_str(), "active" | "atrest" | "k0" | "increasedactive" | "ruhe");
        let move_ok = matches!(move_key.as_str(), "free" | "rigid" | "propped" | "gestuetzt" | "gestützt" | "steif" | "");
        let consistent = match move_key.as_str() {
            "free" | "" => true,
            "rigid" | "steif" | "propped" | "gestuetzt" | "gestützt" => matches!(mode_key.as_str(), "atrest" | "k0" | "increasedactive" | "ruhe"),
            _ => false,
        };
        let regime_fail = !mode_ok || !move_ok || !consistent;
        let regime_mode = if matches!(move_key.as_str(), "free" | "") { "active" } else { "increasedActive" };
        let (k_required, _) = part_1::earth_pressure_coefficient(phi_d, regime_mode, wall.ocr, delta_d);
        let mut ep = CheckResult::assess(
            format!("en1997.9.earthPressure.{}", wall.id),
            part1054,
            ClauseId::new("DIN 4085", "6", "6.1"),
            subject_entity(&wall.id, &path_mode, &label.en, &label.de),
            loc("Earth-pressure regime vs wall movement (DIN 4085)", "Erddruckansatz vs Wandbewegung (DIN 4085)"),
        )
        .utilization(Quantity::new(QuantityKind::Dimensionless, k_required), Quantity::new(QuantityKind::Dimensionless, k_ep))
        .annex(annex)
        .explanation(loc(
            &format!("Mode={}, movement={}, {k_label}={:.3} against the {regime_mode} minimum {k_required:.3} of the movement class (Ka,R={:.3}, K0={:.3}); utilisation K_required/K_used; consistent={}.", wall.earth_pressure_mode, wall.wall_movement, k_ep, ka_r, k0, consistent && mode_ok && move_ok),
            &format!("Ansatz={}, Bewegung={}, {k_label}={:.3} gegen das Minimum {k_required:.3} ({regime_mode}) der Bewegungsklasse (Ka,R={:.3}, K0={:.3}); Ausnutzung K_erf/K_ansatz; konsistent={}.", wall.earth_pressure_mode, wall.wall_movement, k_ep, ka_r, k0, consistent && mode_ok && move_ok),
        ));
        if geometry_fail {
            let path_phi = id_path("retainingWalls", &wall.id, "backfillPhiDeg");
            let path_delta = id_path("retainingWalls", &wall.id, "wallFrictionDeg");
            ep = ep.status(CheckStatus::Fail).remedy(Remedy::at_most(
                subject_entity(&wall.id, &path_delta, &label.en, &label.de),
                Quantity::new(QuantityKind::Dimensionless, wall.wall_friction_deg),
                Quantity::new(QuantityKind::Dimensionless, phi_w),
                loc("Limit the wall friction to 0 ≤ δ ≤ φ′ (EN 1997-1 9.5.1).", "Wandreibung auf 0 ≤ δ ≤ φ′ begrenzen (EN 1997-1 9.5.1)."),
            )).remedy(Remedy::at_most(
                subject_entity(&wall.id, &path_phi, &label.en, &label.de),
                Quantity::new(QuantityKind::Dimensionless, wall.backfill_phi_deg),
                Quantity::new(QuantityKind::Dimensionless, 50.0),
                loc("Keep the backfill friction angle within 0°…50°.", "Reibungswinkel der Hinterfüllung im Bereich 0°…50° halten."),
            ));
        } else if !mode_ok || !move_ok {
            ep = ep.status(CheckStatus::Fail).remedy(Remedy::one_of(
                subject_entity(&wall.id, &path_mode, &label.en, &label.de),
                vec!["active".into(), "atRest".into(), "increasedActive".into()],
                loc("Select active, atRest, or increasedActive per DIN 4085.", "active, atRest oder increasedActive nach DIN 4085 wählen."),
            )).remedy(Remedy::one_of(
                subject_entity(&wall.id, &path_move, &label.en, &label.de),
                vec!["free".into(), "rigid".into(), "propped".into()],
                loc("Select free, rigid, or propped wall movement class.", "Wandbewegungsklasse free, rigid oder propped wählen."),
            ));
        } else if !consistent {
            ep = ep.status(CheckStatus::Fail).remedy(Remedy::one_of(
                subject_entity(&wall.id, &path_mode, &label.en, &label.de),
                if matches!(move_key.as_str(), "free" | "") {
                    vec!["active".into()]
                } else {
                    vec!["atRest".into(), "increasedActive".into()]
                },
                loc(
                    "Align earth-pressure mode with wall movement (DIN 4085 Table): free→active; rigid/propped→atRest or increasedActive.",
                    "Erddruckansatz an Wandbewegung anpassen (DIN 4085 Tabelle): frei→active; starr/gestützt→atRest oder increasedActive.",
                ),
            )).remedy(Remedy::one_of(
                subject_entity(&wall.id, &path_move, &label.en, &label.de),
                vec!["free".into(), "rigid".into(), "propped".into()],
                loc("Or change wall movement class to match the selected earth-pressure mode.", "Oder Wandbewegungsklasse dem gewählten Erddruckansatz anpassen."),
            ));
        }
        report.push(ep.ndps(&factor_ndp_ids(annex)).build());

        let equ = resolve_upl_params(annex, &wall_sit);
        let delta_cos = delta_rad.cos();
        let e_soil = 0.5 * k_ep * wall.backfill_gamma * wall.height.powi(2) * delta_cos;
        let e_surcharge = k_ep * wall.surcharge * wall.height * delta_cos;
        let m_dst = equ.gamma_equ_dst * (e_soil * wall.height / 3.0 + wall.horizontal_permanent * wall.height / 2.0) + equ.gamma_equ_q * e_surcharge * wall.height / 2.0;
        let m_stb = equ.gamma_equ_stb * ((wall.vertical_permanent + self_weight) * wall.base_width / 2.0 + e_av * wall.base_width);
        let path_e = id_path("retainingWalls", &wall.id, "embedment");
        let mut ot = CheckResult::assess(
            format!("en1997.9.overturning.{}", wall.id),
            part1,
            ClauseId::new("EN 1997-1", "2", "2.4.7.2"),
            subject_entity(&wall.id, &path_e, &label.en, &label.de),
            loc("Retaining wall overturning (EQU)", "Kippen der Stützwand (EQU)"),
        )
        .minimum(Quantity::new(QuantityKind::Moment, m_stb), Quantity::new(QuantityKind::Moment, m_dst))
        .annex(annex)
        .explanation(loc(&format!("M_stb,d={:.0} kNm vs M_dst,d={:.0} kNm (γ_stb={:.2}, γ_dst={:.2}, γ_Q={:.2}).", m_stb/1000.0, m_dst/1000.0, equ.gamma_equ_stb, equ.gamma_equ_dst, equ.gamma_equ_q), &format!("M_stb,d={:.0} kNm gegen M_dst,d={:.0} kNm (γ_stb={:.2}, γ_dst={:.2}, γ_Q={:.2}).", m_stb/1000.0, m_dst/1000.0, equ.gamma_equ_stb, equ.gamma_equ_dst, equ.gamma_equ_q)));
        if m_stb < m_dst {
            ot = ot.remedy(Remedy::at_least(
                subject_entity(&wall.id, &path_b, &label.en, &label.de),
                Quantity::length_m(wall.base_width),
                Quantity::length_m(wall.base_width * m_dst / m_stb.max(1.0)),
                loc("Widen base to increase restoring moment.", "Sohlbreite erhöhen, um das rückstellende Moment zu steigern."),
            ));
        }
        report.push(ot.ndps(&factor_ndp_ids(annex)).build());
        entity_catalogue_cases::check_wall_catalogue(doc, wall, report);
        lateral_and_seismic::check_wall(doc, wall, annex, report);
        if wall_u >= 0.0 {
            let governing_remedies = report.checks.iter().find(|c| c.id == format!("en1997.9.sliding.{}", wall.id)).map(|c| c.remedies.clone()).unwrap_or_default();
            push_governing_situation_summary(
                report,
                format!("en1997.governing.situation.wall.{}", wall.id),
                part1,
                annex,
                subject_entity(&wall.id, &id_path("retainingWalls", &wall.id, "baseWidth"), &label.en, &label.de),
                &wall_sit,
                doc.design_approach.as_str(),
                wall_u,
                &format!("en1997.9.sliding.{}", wall.id),
                &governing_remedies,
            );
        }
    }

    if doc.slopes.is_empty() && scope.part("slope-na") {
        report.push(
            CheckResult::assess("en1997.slope.na", part1, ClauseId::new("EN 1997-1", "11", "11.5"), SubjectRef::whole(loc("Project", "Projekt")), loc("Slope stability", "Böschungsbruch"))
                .not_applicable(loc("No slopes in the subject.", "Keine Böschungen im Gegenstand."))
                .annex(annex)
                .build(),
        );
    }
}

/// ⛰️ Extruded 3D slip body of a slope with a stated width (SLOPE-05): the governing layer as homogeneous ground, the user circle or the critical circle of a grid search, cohesive end planes; `None` when no sliding mass develops.
fn slope_extruded(doc: &En1997Snapshot, slope: &semio_s_artifact_norm_en1997_model::Slope, annex: AnnexChoice, gwl: f64, part1: &str, geo3: &AnnexParams, label: &LocalizedCopy) -> Option<CheckResult> {
    use crate::catalogue::slopes::bishop_extruded;
    let layer = doc.layers.iter().find(|l| l.id == slope.governing_layer_id)?;
    let (tan_phi, c) = (layer.phi_prime_deg.to_radians().tan() / geo3.gamma_phi, layer.cohesion_effective / geo3.gamma_c);
    let circle = entity_catalogue_cases::slope_circle(doc, slope, gwl, tan_phi, c)?;
    let (f2, f3, gain, volume) = bishop_extruded(&circle, tan_phi, c, slope.width, 60).filter(|r| r.1.is_finite() && r.1 > 0.0)?;
    let subject = subject_entity(&slope.id, &id_path("slopes", &slope.id, "width"), &label.en, &label.de);
    let mut check = CheckResult::assess(
        format!("en1997.11.bishop3d.{}", slope.id),
        part1,
        ClauseId::new("EN 1997-1", "11", "11.5"),
        subject.clone(),
        loc("Overall stability, extruded 3D slip body (SLOPE-05)", "Gesamtstandsicherheit, extrudierter 3D-Gleitkörper (SLOPE-05)"),
    )
    .minimum(Quantity::new(QuantityKind::Dimensionless, f3), Quantity::new(QuantityKind::Dimensionless, 1.0))
    .annex(annex)
    .explanation(loc(
        &format!("F_3D = {f3:.2} = F_2D {f2:.2} + {gain:.3} from cohesive end planes; body width {:.1} m, volume {volume:.0} m3, governing layer {} as homogeneous ground, GEO-3 factors.", slope.width, slope.governing_layer_id),
        &format!("F_3D = {f3:.2} = F_2D {f2:.2} + {gain:.3} aus kohäsiven Stirnflächen; Körperbreite {:.1} m, Volumen {volume:.0} m3, maßgebende Schicht {} als homogener Baugrund, GEO-3-Beiwerte.", slope.width, slope.governing_layer_id),
    ));
    if f3 < 1.0 {
        check = check.remedy(Remedy::at_most(
            subject_entity(&slope.id, &id_path("slopes", &slope.id, "angleDeg"), &label.en, &label.de),
            Quantity::new(QuantityKind::Dimensionless, slope.angle_deg),
            Quantity::new(QuantityKind::Dimensionless, (slope.angle_deg * 0.85).max(10.0)),
            loc("Flatten the slope angle to raise the extruded slip body factor.", "Böschungswinkel abflachen, um die Sicherheit des extrudierten Gleitkörpers zu erhöhen."),
        ));
    }
    Some(check.ndps(&factor_ndp_ids(annex)).build())
}

/// ⚙️ Checks the overall stability of every slope.
fn check_slopes(doc: &En1997Snapshot, report: &mut CheckReport, annex: AnnexChoice, gwl: f64, part1: &str, scope: Scope<'_>, step: &mut dyn FnMut(f32) -> bool) {
    for (_, slope) in doc.slopes.iter().enumerate().filter(|(index, _)| scope.admits("slopes", *index)) {
        let geo3 = DesignApproach::Geo3.annex_params(annex);
        let label = loc(&format!("Slope {}", slope.id), &format!("Böschung {}", slope.id));
        let path = id_path("slopes", &slope.id, "angleDeg");
        let path_gov = id_path("slopes", &slope.id, "governingLayerId");
        let custom = slope.circle_radius > 1e-6;
        let fixed = custom.then(|| part_1::bishop_fos_circle(&doc.layers, &slope.governing_layer_id, slope.height, slope.angle_deg, slope.length, gwl, geo3.gamma_phi, geo3.gamma_c, slope.circle_centre_x, slope.circle_centre_z, slope.circle_radius)).flatten();
        let fos_opt = if custom {
            fixed.as_ref().map(|(fos, _)| *fos)
        } else {
            part_1::bishop_fos_slices_with(&doc.layers, &slope.governing_layer_id, slope.height, slope.angle_deg, slope.length, gwl, geo3.gamma_phi, geo3.gamma_c, step)
        };
        if let Some(fos) = fos_opt.filter(|f| f.is_nan()) {
            let _ = fos;
            report.push(
                CheckResult::assess(
                    format!("en1997.11.bishop.{}", slope.id),
                    part1,
                    ClauseId::new("EN 1997-1", "11", "11.5"),
                    subject_entity(&slope.id, &path, &label.en, &label.de),
                    loc("Overall stability (Bishop slices / GEO-3)", "Gesamtstandsicherheit (Bishop-Lamellen / GEO-3)"),
                )
                .annex(annex)
                .not_applicable(loc("Not evaluated: the slip-circle search was cancelled or found no admissible circle (or the chosen circle cuts no sliding mass).", "Nicht ausgewertet: Gleitkreissuche abgebrochen oder kein zulässiger Gleitkreis gefunden."))
                .build(),
            );
        } else if let Some(fos) = fos_opt {
            let mut st = CheckResult::assess(
                format!("en1997.11.bishop.{}", slope.id),
                part1,
                ClauseId::new("EN 1997-1", "11", "11.5"),
                subject_entity(&slope.id, &path, &label.en, &label.de),
                loc("Overall stability (Bishop slices / GEO-3)", "Gesamtstandsicherheit (Bishop-Lamellen / GEO-3)"),
            )
            .minimum(Quantity::new(QuantityKind::Dimensionless, fos), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .annex(annex)
            .explanation(loc(
                &format!("FoS={:.2} (GEO-3 per EN 1997-1 §2.4.7.3.4 / DIN 1054 NA; slices by depth, governing layer {}, L={:.1} m, GWL={:.1} m, γ_φ={:.2}).", fos, slope.governing_layer_id, slope.length, gwl, geo3.gamma_phi),
                &format!("η={:.2} (GEO-3 nach EN 1997-1 §2.4.7.3.4 / DIN 1054 NA; Lamellen nach Tiefe, maßgebende Schicht {}, L={:.1} m, GW={:.1} m, γ_φ={:.2}).", fos, slope.governing_layer_id, slope.length, gwl, geo3.gamma_phi),
            ));
            if fos < 1.0 {
                let path_h = id_path("slopes", &slope.id, "height");
                st = st.remedy(Remedy::at_most(
                    subject_entity(&slope.id, &path, &label.en, &label.de),
                    Quantity::new(QuantityKind::Dimensionless, slope.angle_deg),
                    Quantity::new(QuantityKind::Dimensionless, (slope.angle_deg * 0.85).max(10.0)),
                    loc("Flatten the slope angle to raise Bishop FoS.", "Böschungswinkel abflachen, um die Bishop-Sicherheit zu erhöhen."),
                )).remedy(Remedy::at_most(
                    subject_entity(&slope.id, &path_h, &label.en, &label.de),
                    Quantity::length_m(slope.height),
                    Quantity::length_m((slope.height * 0.9).max(1.0)),
                    loc("Reduce slope height to raise Bishop FoS.", "Böschungshöhe verringern, um die Bishop-Sicherheit zu erhöhen."),
                ));
            }
            let mobilisation = match &fixed {
                Some((_, slices)) => Some(slices.clone()).filter(|slices| !slices.is_empty()),
                None => part_1::bishop_critical_slices(&doc.layers, &slope.governing_layer_id, slope.height, slope.angle_deg, slope.length, gwl, geo3.gamma_phi, geo3.gamma_c),
            };
            if let Some(mobilisation) = mobilisation {
                st = st.field("slip-slice", station_fields::slip_slices(&mobilisation, if fos > 1e-9 { 1.0 / fos } else { 0.0 }));
            }
            report.push(st.ndps(&factor_ndp_ids(annex)).build());
            if slope.width > 1e-6 {
                if let Some(extruded) = slope_extruded(doc, slope, annex, gwl, part1, &geo3, &label) {
                    report.push(extruded);
                }
            }
        } else {
            let choices: Vec<String> = doc.layers.iter().map(|l| l.id.clone()).collect();
            let st = CheckResult::assess(
                format!("en1997.11.bishop.{}", slope.id),
                part1,
                ClauseId::new("EN 1997-1", "11", "11.5"),
                subject_entity(&slope.id, &path_gov, &label.en, &label.de),
                loc("Overall stability (Bishop slices / GEO-3)", "Gesamtstandsicherheit (Bishop-Lamellen / GEO-3)"),
            )
            .utilization(Quantity::new(QuantityKind::Dimensionless, 0.0), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .status(CheckStatus::Fail)
            .annex(annex)
            .explanation(loc(
                &format!("Governing layer id '{}' is missing or unknown — Bishop slices cannot run.", slope.governing_layer_id),
                &format!("Maßgebende Schicht-Id '{}' fehlt oder unbekannt — Bishop-Lamellen nicht möglich.", slope.governing_layer_id),
            ))
            .remedy(if choices.is_empty() {
                Remedy::at_least(
                    subject_entity(&slope.id, &path_gov, &label.en, &label.de),
                    Quantity::new(QuantityKind::Dimensionless, 0.0),
                    Quantity::new(QuantityKind::Dimensionless, 1.0),
                    loc("Add a soil layer and set slopes[].governingLayerId to its id.", "Bodenschicht anlegen und slopes[].governingLayerId setzen."),
                )
            } else {
                Remedy::one_of(
                    subject_entity(&slope.id, &path_gov, &label.en, &label.de),
                    choices,
                    loc("Set governingLayerId to an existing layer id.", "governingLayerId auf eine bestehende Schicht-Id setzen."),
                )
            });
            report.push(st.ndps(&factor_ndp_ids(annex)).build());
        }
        entity_catalogue_cases::check_slope_catalogue(doc, slope, annex, gwl, report);
    }

    if doc.uplift_cases.is_empty() && scope.part("upl-na") {
        report.push(
            CheckResult::assess("en1997.upl.na", part1, ClauseId::new("EN 1997-1", "10", "10.2"), SubjectRef::whole(loc("Project", "Projekt")), loc("Uplift / heave", "Aufschwimmen / hydraulischer Grundbruch"))
                .not_applicable(loc("No uplift/heave cases in the subject.", "Keine Aufschwimm-/HYD-Fälle im Gegenstand."))
                .annex(annex)
                .build(),
        );
    }
}

/// 🧲️ Checks the buoyancy uplift of every uplift case.
fn check_uplift(doc: &En1997Snapshot, report: &mut CheckReport, annex: AnnexChoice, gwl: f64, part1: &str, design_situations: &[String], scope: Scope<'_>) {
    for (_, u) in doc.uplift_cases.iter().enumerate().filter(|(index, _)| scope.admits("upliftCases", *index)) {
        let label = loc(&format!("Uplift {}", u.id), &format!("Aufschwimmen {}", u.id));
        let pore_given = u.pore_pressure > 1.0;
        let pore = u.pore_pressure.max(0.0);
        let mut upl_sit = doc.design_situation.as_str().to_string();
        let mut upl_u = -1.0f64;
        let mut upl_p = resolve_upl_params(annex, doc.design_situation.as_str());
        for sit in design_situations.iter() {
            let p = resolve_upl_params(annex, sit);
            let stab_try = u.permanent_stabilizing * p.gamma_g_stb;
            let dest_try = p.gamma_g_dst * u.permanent_destabilizing + p.gamma_q_dst * u.variable_destabilizing;
            let u_try = if stab_try > 1e-9 { dest_try / stab_try } else { 0.0 };
            if u_try > upl_u {
                upl_u = u_try;
                upl_sit = sit.clone();
                upl_p = p;
            }
        }
        let stab = u.permanent_stabilizing * upl_p.gamma_g_stb;
        let dest = upl_p.gamma_g_dst * u.permanent_destabilizing + upl_p.gamma_q_dst * u.variable_destabilizing;
        let path_s = id_path("upliftCases", &u.id, "permanentStabilizing");
        let mut upl = CheckResult::assess(
            format!("en1997.10.upl.{}", u.id),
            part1,
            ClauseId::new("EN 1997-1", "10", "10.2"),
            subject_entity(&u.id, &path_s, &label.en, &label.de),
            loc("Uplift equilibrium (UPL)", "Aufschwimmen (UPL)"),
        )
        .minimum(Quantity::new(QuantityKind::Force, stab), Quantity::new(QuantityKind::Force, dest))
        .annex(annex)
        .explanation(loc(
            &format!("G_stb,d={:.0} kN vs V_dst,d={:.0} kN (γ_stb={:.2}, γ_dst={:.2}/γ_Q={:.2}).", stab/1000.0, dest/1000.0, upl_p.gamma_g_stb, upl_p.gamma_g_dst, upl_p.gamma_q_dst),
            &format!("G_stb,d={:.0} kN gegen V_dst,d={:.0} kN (γ_stb={:.2}, γ_dst={:.2}/γ_Q={:.2}).", stab/1000.0, dest/1000.0, upl_p.gamma_g_stb, upl_p.gamma_g_dst, upl_p.gamma_q_dst),
        ));
        if stab < dest {
            let g_req = dest / upl_p.gamma_g_stb.max(0.5);
            upl = upl.remedy(Remedy::at_least(
                subject_entity(&u.id, &path_s, &label.en, &label.de),
                Quantity::new(QuantityKind::Force, u.permanent_stabilizing),
                Quantity::new(QuantityKind::Force, g_req),
                loc(&format!("Increase permanent stabilizing force to at least {:.0} kN.", g_req/1000.0), &format!("Ständige stabilisierende Einwirkung auf mindestens {:.0} kN erhöhen.", g_req/1000.0)),
            ));
        }
        report.push(upl.ndps(&factor_ndp_ids(annex)).build());

        let sigma_d = u.total_stress * upl_p.gamma_hyd_stb;
        let u_d = pore * upl_p.gamma_hyd;
        let path_t = id_path("upliftCases", &u.id, "totalStress");
        let mut hyd = CheckResult::assess(
            format!("en1997.10.hyd.{}", u.id),
            part1,
            ClauseId::new("EN 1997-1", "10", "10.3"),
            subject_entity(&u.id, &path_t, &label.en, &label.de),
            loc("Hydraulic heave (HYD)", "Hydraulischer Grundbruch (HYD)"),
        )
        .minimum(Quantity::new(QuantityKind::Pressure, sigma_d), Quantity::new(QuantityKind::Pressure, u_d))
        .annex(annex)
        .explanation(loc(
            &format!("σ={:.0} kPa vs u_d={:.0} kPa (γ_HYD={:.2}, GWL={:.1} m).", sigma_d/1000.0, u_d/1000.0, upl_p.gamma_hyd, gwl),
            &format!("σ={:.0} kPa gegen u_d={:.0} kPa (γ_HYD={:.2}, GW={:.1} m).", sigma_d/1000.0, u_d/1000.0, upl_p.gamma_hyd, gwl),
        ));
        if sigma_d < u_d {
            hyd = hyd.remedy(Remedy::at_least(
                subject_entity(&u.id, &path_t, &label.en, &label.de),
                Quantity::new(QuantityKind::Pressure, u.total_stress),
                Quantity::new(QuantityKind::Pressure, u_d / upl_p.gamma_hyd_stb.max(0.5)),
                loc("Increase total stress or lower groundwater / pore pressure.", "Totalspannung erhöhen oder Grundwasser / Porenwasserdruck absenken."),
            )).remedy(Remedy::at_most(
                subject_entity("", "groundwaterLevel", "Groundwater level", "Grundwasserstand"),
                Quantity::length_m(gwl),
                Quantity::length_m((gwl * 0.7).max(0.0)),
                loc("Lower groundwater level to reduce pore pressure.", "Grundwasserstand absenken, um den Porenwasserdruck zu reduzieren."),
            ));
        }
        if !pore_given {
            hyd = hyd.not_applicable(loc("No pore pressure given: the hydraulic-heave check needs the pore pressure at the base.", "Kein Porenwasserdruck angegeben: der Nachweis gegen hydraulischen Grundbruch braucht den Porenwasserdruck an der Sohle."));
        }
        report.push(hyd.ndps(&factor_ndp_ids(annex)).build());
        entity_catalogue_cases::check_uplift_catalogue(doc, u, report);
        if upl_u >= 0.0 {
            let governing_remedies = report.checks.iter().find(|c| c.id == format!("en1997.10.upl.{}", u.id)).map(|c| c.remedies.clone()).unwrap_or_default();
            push_governing_situation_summary(
                report,
                format!("en1997.governing.situation.uplift.{}", u.id),
                part1,
                annex,
                subject_entity(&u.id, &path_s, &label.en, &label.de),
                &upl_sit,
                doc.design_approach.as_str(),
                upl_u,
                &format!("en1997.10.upl.{}", u.id),
                &governing_remedies,
            );
        }
    }
}
