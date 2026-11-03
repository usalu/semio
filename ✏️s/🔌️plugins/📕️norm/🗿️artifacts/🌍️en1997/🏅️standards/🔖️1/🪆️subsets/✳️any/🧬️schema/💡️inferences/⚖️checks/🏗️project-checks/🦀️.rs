//! 🏗️ EN 1997 project checks: footing and layer checks of a project snapshot and the governing situation.

use super::*;

/// 📚️ The demo document with a design case for every catalogue entry that no typed entity realises: the compliant package plus [`catalogue_demo_cases`] without the entries whose spec-trace row is a `Check` row (their entities already carry the checks, so a case would duplicate them).
pub fn catalogue_demo_document() -> En1997Snapshot {
    let realised = |entry: &str| spec_trace::TABLE.items.iter().any(|item| item.id == entry && matches!(item.realisation, semio_s_plugin_norm_compliance::spec_trace::Realisation::Check(_)));
    En1997Snapshot { cases: catalogue::catalogue_demo_cases().into_iter().filter(|case| !realised(&case.entry_id)).collect(), ..semio_s_artifact_norm_en1997_model::compliant_demo() }
}

pub fn check_project(doc: &En1997Snapshot) -> CheckReport {
    check_project_scoped(doc, Scope::All, &mut |_| true)
}

/// 🌐️ The governing-situation summary of the whole project, derived from the merged results of every other check (the post-fold aggregate of the units).
pub fn governing_summary(doc: &En1997Snapshot, report: &CheckReport) -> Vec<CheckResult> {
    let approach_lbl = doc.design_approach.as_str().to_string();
    let (part1, annex) = ("DIN EN 1997-1", doc.annex);
    let design_situations = project_design_situations(doc);
    let mut out = CheckReport::default();
    let governing_info = report
        .checks
        .iter()
        .filter(|c| c.status != CheckStatus::NotApplicable && !c.id.starts_with("en1997.governing.situation"))
        .max_by(|a, b| a.utilization.partial_cmp(&b.utilization).unwrap_or(std::cmp::Ordering::Equal))
        .map(|c| (c.utilization, c.id.clone(), c.explanation.en.clone(), c.remedies.clone()));
    if let Some((gov_u, gov_id, gov_expl, gov_remedies)) = governing_info {
        let sit_guess = design_situations
            .iter()
            .find(|s| gov_expl.contains(situation_short(s)) || gov_expl.contains(s.as_str()))
            .cloned()
            .unwrap_or_else(|| doc.design_situation.as_str().to_string());
        push_governing_situation_summary(&mut out, "en1997.governing.situation".into(), part1, annex, SubjectRef::whole(loc("Project", "Projekt")), &sit_guess, &approach_lbl, gov_u, &gov_id, &gov_remedies);
    }
    out.checks
}

fn project_design_situations(doc: &En1997Snapshot) -> Vec<String> {
    let mut design_situations: Vec<String> = vec![doc.design_situation.as_str().to_string()];
    for footing in doc.footings.iter() {
        for lc in footing.load_cases.iter() {
            let sit = situation_for_load_case(doc, lc);
            if !design_situations.iter().any(|s| s == &sit) {
                design_situations.push(sit);
            }
        }
    }
    design_situations
}

/// 🎯️ The project check restricted to `scope` (`step(fraction)` is polled by the slip-circle search and returns `false` to cancel): the parts `integrity`, `footing-na`, `pile-na`, `wall-na`, `slope-na`, `upl-na` or one row of `layers`, `footings`, `piles`, `retainingWalls`, `slopes`, `upliftCases`; the ordered concatenation of the units in [`En1997Units`] plus [`governing_summary`] is [`check_project`].
pub fn check_project_scoped(declared: &En1997Snapshot, scope: Scope<'_>, step: &mut dyn FnMut(f32) -> bool) -> CheckReport {
    let resolved = ground_derivation::resolved_document(declared);
    let doc: &En1997Snapshot = &resolved;
    let mut report = CheckReport::default();
    if scope.part("integrity") {
        push_all_duplicate_ids(&mut report, doc);
        ground_derivation::push_integrity(&mut report, doc);
    }
    let approach = parse_design_approach(doc.design_approach.as_str());
    let annex = doc.annex;
    let part1 = "DIN EN 1997-1";
    let part2 = "DIN EN 1997-2";
    let part1054 = "DIN 1054";
    let gwl = doc.groundwater_level;
    let gk = doc.geotechnical_category;
    let design_situations = project_design_situations(doc);
    if scope.part("integrity") {
        push_design_selection_check(doc, &mut report, part1054);
        let gk_ok = (1..=3).contains(&gk);
        let mut gk_check = CheckResult::assess(
            "en1997.gk.category",
            part1054,
            ClauseId::new("DIN 1054", "A", "A.2"),
            subject_entity("", "geotechnicalCategory", "Geotechnical category", "Geotechnische Kategorie"),
            loc("Geotechnical category GK", "Geotechnische Kategorie GK"),
        )
        .annex(annex)
        .explanation(loc(
            &format!("GK={gk} (1=simple, 2=standard, 3=complex)."),
            &format!("GK={gk} (1=einfach, 2=üblich, 3=schwierig)."),
        ));
        if !gk_ok {
            gk_check = gk_check.status(CheckStatus::Fail).remedy(Remedy::one_of(
                subject_entity("", "geotechnicalCategory", "Geotechnical category", "Geotechnische Kategorie"),
                vec!["1".into(), "2".into(), "3".into()],
                loc("Set geotechnical category to 1, 2, or 3.", "Geotechnische Kategorie auf 1, 2 oder 3 setzen."),
            ));
        } else {
            gk_check = gk_check.status(CheckStatus::Pass);
        }
        report.push(gk_check.ndps(&factor_ndp_ids(annex)).build());
    }
    if scope.part("investigation-extent") {
        investigation_extent::push_investigation_extent(&mut report, doc);
    }
    check_soil_layers(declared, doc, &mut report, annex, part1, part2, scope);
    if doc.footings.is_empty() && scope.part("footing-na") {
        report.push(
            CheckResult::assess("en1997.footing.na", part1, ClauseId::new("EN 1997-1", "6", "6.5"), SubjectRef::whole(loc("Project", "Projekt")), loc("Spread foundation checks", "Flachgründungsnachweise"))
                .not_applicable(loc("No spread foundations in the subject.", "Keine Flachgründungen im Gegenstand."))
                .annex(annex)
                .build(),
        );
    }
    check_footings(doc, &mut report, approach, annex, part1, part2, part1054, gwl, gk, scope);
    sls_pairs::check_sls_pairs(doc, &mut report, annex, gwl, scope);
    if doc.piles.is_empty() && scope.part("pile-na") {
        report.push(
            CheckResult::assess("en1997.pile.na", part1, ClauseId::new("EN 1997-1", "7", "7.6"), SubjectRef::whole(loc("Project", "Projekt")), loc("Pile checks", "Pfahlnachweise"))
                .not_applicable(loc("No piles in the subject.", "Keine Pfähle im Gegenstand."))
                .annex(annex)
                .build(),
        );
    }
    check_piles_walls_slopes_and_uplift(doc, &mut report, approach, annex, gwl, part1, part1054, &design_situations, scope, step);
    geo_entities::check_embedded_walls(doc, &mut report, scope);
    geo_entities::check_anchors(doc, &mut report, scope);
    geo_entities::check_pile_groups(doc, &mut report, scope);
    geo_entities::check_lab_tests(doc, &mut report, scope);
    geo_entities::check_layer_waters(doc, &mut report, scope);
    for (_, case) in doc.cases.iter().enumerate().filter(|(index, _)| scope.admits("cases", *index)) {
        report.extend(crate::catalogue::evaluate_design_case(&semio_s_plugin_norm_compliance::design_case::DesignCase::from(case), annex));
    }
    if matches!(scope, Scope::All) {
        let summary = governing_summary(doc, &report);
        report.extend(summary);
    }
    report
}

/// 🧱️ Checks the strength parameters and the id uniqueness of every soil layer.
fn check_soil_layers(declared: &En1997Snapshot, doc: &En1997Snapshot, report: &mut CheckReport, annex: AnnexChoice, part1: &str, part2: &str, scope: Scope<'_>) {
    for (_, layer) in doc.layers.iter().enumerate().filter(|(index, _)| scope.admits("layers", *index)) {
        let overburden = layer.gamma * ((layer.depth_top + layer.depth_bottom) * 0.5).max(0.5);
        report.push(layer_model_check(doc, layer, annex, part1));
        for check in ground_derivation::layer_checks(declared, layer) {
            report.push(check);
        }
        report.extend(investigation_extent::cone_factor_check(layer, annex));
        if layer.cpt_qc > 0.0 || layer.spt_n > 0.0 {
            let overburden = layer.gamma * ((layer.depth_top + layer.depth_bottom) * 0.5).max(0.5);
            let path_phi = format!("layers[id={}].phiPrimeDeg", layer.id);
            let path_cpt = format!("layers[id={}].cptQc", layer.id);
            let path_spt = format!("layers[id={}].sptN", layer.id);
            let label_l = loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id));
            if layer.cpt_qc > 0.0 {
                let phi_cpt = part_2::phi_from_cpt_deg(layer.cpt_qc, overburden.max(50_000.0));
                let mut cpt_chk = CheckResult::assess(
                    format!("en1997.2.phi.cpt.{}", layer.id),
                    part2,
                    ClauseId::new("EN 1997-2", "2", "2.4.3"),
                    subject_entity(&layer.id, &path_cpt, &label_l.en, &label_l.de),
                    loc("φ′ vs CPT correlation (EN 1997-2)", "φ′ vs CPT-Korrelation (EN 1997-2)"),
                )
                .utilization(
                    Quantity::new(QuantityKind::Dimensionless, layer.phi_prime_deg),
                    Quantity::new(QuantityKind::Dimensionless, phi_cpt.max(1e-6)),
                )
                .annex(annex)
                .explanation(loc(
                    &format!("φ′_stated={:.1}° vs φ′_CPT={:.1}° (qc={:.2} MPa, σ′v0={:.0} kPa).", layer.phi_prime_deg, phi_cpt, layer.cpt_qc/1e6, overburden/1000.0),
                    &format!("φ′_angegeben={:.1}° gegen φ′_CPT={:.1}° (qc={:.2} MPa, σ′v0={:.0} kPa).", layer.phi_prime_deg, phi_cpt, layer.cpt_qc/1e6, overburden/1000.0),
                ));
                if layer.phi_prime_deg > phi_cpt + 1e-9 {
                    cpt_chk = cpt_chk.status(CheckStatus::Fail).remedy(Remedy::at_most(
                        subject_entity(&layer.id, &path_phi, &label_l.en, &label_l.de),
                        Quantity::new(QuantityKind::Dimensionless, layer.phi_prime_deg),
                        Quantity::new(QuantityKind::Dimensionless, phi_cpt),
                        loc(
                            &format!("Reduce stated φ′ to ≤ {:.1}° (CPT correlation, EN 1997-1 §2.4.5.2).", phi_cpt),
                            &format!("Angegebenen φ′ auf ≤ {:.1}° absenken (CPT-Korrelation, EN 1997-1 §2.4.5.2).", phi_cpt),
                        ),
                    ));
                }
                report.push(cpt_chk.build());
            }
            if layer.spt_n > 0.0 {
                let phi_spt = part_2::phi_from_spt_deg(layer.spt_n, overburden.max(50_000.0));
                let mut spt_chk = CheckResult::assess(
                    format!("en1997.2.phi.spt.{}", layer.id),
                    part2,
                    ClauseId::new("EN 1997-2", "2", "2.4.3"),
                    subject_entity(&layer.id, &path_spt, &label_l.en, &label_l.de),
                    loc("φ′ vs SPT correlation (EN 1997-2)", "φ′ vs SPT-Korrelation (EN 1997-2)"),
                )
                .utilization(
                    Quantity::new(QuantityKind::Dimensionless, layer.phi_prime_deg),
                    Quantity::new(QuantityKind::Dimensionless, phi_spt.max(1e-6)),
                )
                .annex(annex)
                .explanation(loc(
                    &format!("φ′_stated={:.1}° vs φ′_SPT={:.1}° (N={:.0}).", layer.phi_prime_deg, phi_spt, layer.spt_n),
                    &format!("φ′_angegeben={:.1}° gegen φ′_SPT={:.1}° (N={:.0}).", layer.phi_prime_deg, phi_spt, layer.spt_n),
                ));
                if layer.phi_prime_deg > phi_spt + 1e-9 {
                    spt_chk = spt_chk.status(CheckStatus::Fail).remedy(Remedy::at_most(
                        subject_entity(&layer.id, &path_phi, &label_l.en, &label_l.de),
                        Quantity::new(QuantityKind::Dimensionless, layer.phi_prime_deg),
                        Quantity::new(QuantityKind::Dimensionless, phi_spt),
                        loc(
                            &format!("Reduce stated φ′ to ≤ {:.1}° (SPT correlation, EN 1997-1 §2.4.5.2).", phi_spt),
                            &format!("Angegebenen φ′ auf ≤ {:.1}° absenken (SPT-Korrelation, EN 1997-1 §2.4.5.2).", phi_spt),
                        ),
                    ));
                }
                report.push(spt_chk.build());
            }
            let (phi_char, phi_src) = part_2::investigation_phi_deg(layer, overburden)
                .unwrap_or((layer.phi_prime_deg, "stated"));
            let mut phi_chk = CheckResult::assess(
                format!("en1997.2.phi.derived.{}", layer.id),
                part2,
                ClauseId::new("EN 1997-2", "2", "2.4.3"),
                subject_entity(&layer.id, &path_phi, &label_l.en, &label_l.de),
                loc("Characteristic φ′ vs investigation (EN 1997-1 §2.4.5.2)", "Charakteristischer φ′ vs Erkundung (EN 1997-1 §2.4.5.2)"),
            )
            .utilization(
                Quantity::new(QuantityKind::Dimensionless, layer.phi_prime_deg),
                Quantity::new(QuantityKind::Dimensionless, phi_char.max(1e-6)),
            )
            .annex(annex)
            .explanation(loc(
                &format!("φ′_stated={:.1}° vs φ′_char={:.1}° from {phi_src} (EN 1997-2 CPT/SPT; qc={:.0} MPa, N={:.0}).", layer.phi_prime_deg, phi_char, layer.cpt_qc/1e6, layer.spt_n),
                &format!("φ′_angegeben={:.1}° gegen φ′_char={:.1}° aus {phi_src} (EN 1997-2 CPT/SPT; qc={:.0} MPa, N={:.0}).", layer.phi_prime_deg, phi_char, layer.cpt_qc/1e6, layer.spt_n),
            ));
            if layer.phi_prime_deg > phi_char + 1e-9 {
                phi_chk = phi_chk.status(CheckStatus::Fail).remedy(Remedy::at_most(
                    subject_entity(&layer.id, &path_phi, &label_l.en, &label_l.de),
                    Quantity::new(QuantityKind::Dimensionless, layer.phi_prime_deg),
                    Quantity::new(QuantityKind::Dimensionless, phi_char),
                    loc(
                        &format!("Reduce stated φ′ to ≤ {:.1}° (cautious estimate from {phi_src}, EN 1997-1 §2.4.5.2).", phi_char),
                        &format!("Angegebenen φ′ auf ≤ {:.1}° absenken (vorsichtiger Schätzwert aus {phi_src}, EN 1997-1 §2.4.5.2).", phi_char),
                    ),
                ));
            }
            report.push(phi_chk.build());
        }
    }
}

/// 🪵️ Checks the bearing, sliding, eccentricity and settlement of every spread foundation.
fn check_footings(
    doc: &En1997Snapshot,
    report: &mut CheckReport,
    approach: DesignApproach,
    annex: AnnexChoice,
    part1: &str,
    part2: &str,
    part1054: &str,
    gwl: f64,
    gk: u8,
    scope: Scope<'_>,
) {
    for (_, footing) in doc.footings.iter().enumerate().filter(|(index, _)| scope.admits("footings", *index)) {
        let layer = layer_at(&doc.layers, footing.embedment).cloned().unwrap_or_else(|| SoilLayer {
            id: "default".into(), soil_type: semio_s_artifact_norm_en1997_model::SoilType::Sand, depth_top: 0.0, depth_bottom: 10.0,
            gamma: 18_000.0, gamma_prime: 10_000.0, phi_prime_deg: 30.0, cohesion_effective: 0.0, cohesion_undrained: 0.0,
            oedometric_modulus: 30e6, poisson_ratio: 0.3, cpt_qc: 0.0, spt_n: 0.0,
        });
        let overburden = layer.gamma * footing.embedment.max(0.5);
        let (phi_char, _phi_src) = part_2::characteristic_phi_deg(&layer, overburden);
        let path_w = id_path("footings", &footing.id, "width");
        let label = loc(&format!("Footing {}", footing.id), &format!("Fundament {}", footing.id));
        if footing.load_cases.is_empty() {
            report.push(
                CheckResult::assess(format!("en1997.footing.{}.loads.na", footing.id), part1, ClauseId::new("EN 1997-1", "6", "6.5"), subject_entity(&footing.id, &path_w, &label.en, &label.de), loc("Foundation actions", "Einwirkungen auf das Fundament"))
                    .not_applicable(loc("No load cases on this footing.", "Keine Lastfälle für dieses Fundament."))
                    .annex(annex)
                    .build(),
            );
        }
        let gamma_below = part_1::effective_gamma_below_pa_m(layer.gamma, layer.gamma_prime, footing.embedment, gwl, footing.width);
        let phi_use = phi_char;
        let mut foot_gov_u = -1.0f64;
        let mut foot_gov_sit = doc.design_situation.as_str().to_string();
        let mut foot_gov_check = String::new();
        check_footing_load_cases(doc, report, approach, annex, part1, part1054, gwl, gk, footing, &layer, &path_w, &label, gamma_below, phi_use, &mut foot_gov_u, &mut foot_gov_sit, &mut foot_gov_check);
        check_footing_uplift_and_bounds(doc, report, annex, part1, gwl, footing, &layer, &label);
        check_footing_governing_situation(doc, report, annex, part1, part2, gk, footing, &path_w, &label, foot_gov_u, &foot_gov_sit, &foot_gov_check);
    }
}

/// 🔩️ Checks the bearing, sliding, eccentricity and settlement of every load case of one spread foundation and tracks the governing one.
#[allow(clippy::too_many_arguments, reason = "the extracted phase keeps the locals of its original function as explicit parameters")]
fn check_footing_load_cases(
    doc: &En1997Snapshot,
    report: &mut CheckReport,
    approach: DesignApproach,
    annex: AnnexChoice,
    part1: &str,
    part1054: &str,
    gwl: f64,
    gk: u8,
    footing: &SpreadFoundation,
    layer: &SoilLayer,
    path_w: &str,
    label: &LocalizedCopy,
    gamma_below: f64,
    phi_use: f64,
    foot_gov_u: &mut f64,
    foot_gov_sit: &mut String,
    foot_gov_check: &mut String,
) {
    for lc in footing.load_cases.iter() {
        let sit = situation_for_load_case(doc, lc);
        let p = resolve_params(approach, annex, &sit);
        let v_d = design_actions(&p, lc.vertical_permanent, lc.vertical_variable);
        let h_d = design_actions(&p, lc.horizontal_permanent, lc.horizontal_variable);
        let m_d = design_actions(&p, lc.moment_permanent, lc.moment_variable);
        let e_b = m_d / v_d.max(1.0);
        let a = footing.width * footing.length;

        let sit_kind = parse_design_situation(&sit);
        let cohesive = part_1::is_cohesive_layer(layer);
        let short_term = matches!(sit_kind, DesignSituation::Transient | DesignSituation::Accidental);
        let use_undrained = cohesive && short_term && layer.cohesion_undrained > 0.0;
        let r_v = if gk == 1 {
            let q_all = part_1::presumptive_bearing_pa(layer.soil_type.as_str(), footing.embedment);
            q_all * a / p.gamma_r_v
        } else if use_undrained {
            part_1::undrained_bearing_resistance_n(
                layer.cohesion_undrained, gamma_below, footing.width, footing.length, footing.embedment, e_b, h_d, v_d, &p,
            )
        } else {
            part_1::design_bearing_resistance_with_params(
                phi_use, layer.cohesion_effective, layer.gamma, gamma_below, footing.width, footing.length, footing.embedment,
                e_b, 0.0, h_d, v_d, footing.base_inclination_deg, &p,
            )
        };
        let r_v = r_v.max(1.0);
        let b_req = find_required_width(footing, layer, phi_use, lc, approach, annex, &sit, gwl, footing.width);
        let mut bearing = CheckResult::assess(
            format!("en1997.6.5.bearing.{}.{}", footing.id, lc.id),
            if gk == 1 { part1054 } else { part1 },
            ClauseId::new(if gk == 1 { "DIN 1054" } else { "EN 1997-1" }, "6", if gk == 1 { "A.6.10" } else { "6.5.2" }),
            subject_entity(&footing.id, path_w, &label.en, &label.de),
            loc("Bearing resistance", "Tragfähigkeit"),
        )
        .utilization(Quantity::new(QuantityKind::Force, v_d), Quantity::new(QuantityKind::Force, r_v))
        .annex(annex)
        .explanation(loc(
            &format!("V_d={:.0} kN vs R_d={:.0} kN (GK{gk}, α={:.1}°, γ′_eff={:.0}, GWL={:.1} m{}).", v_d / 1000.0, r_v / 1000.0, footing.base_inclination_deg, gamma_below, gwl, if use_undrained { ", undrained c_u Annex D.3" } else { "" }),
            &format!("V_d={:.0} kN gegen R_d={:.0} kN (GK{gk}, α={:.1}°, γ′_eff={:.0}, GW={:.1} m{}).", v_d / 1000.0, r_v / 1000.0, footing.base_inclination_deg, gamma_below, gwl, if use_undrained { ", undräniert c_u Anhang D.3" } else { "" }),
        ));
        if v_d > r_v {
            bearing = bearing.remedy(Remedy::at_least(
                subject_entity(&footing.id, path_w, &label.en, &label.de),
                Quantity::length_m(footing.width),
                Quantity::length_m(b_req),
                loc(&format!("Increase footing width from {:.2} m to at least {:.2} m.", footing.width, b_req), &format!("Fundamentbreite von {:.2} m auf mindestens {:.2} m erhöhen.", footing.width, b_req)),
            ));
        }
        let u_bearing = if r_v > 1e-9 { v_d / r_v } else { 0.0 };
        if u_bearing > *foot_gov_u {
            *foot_gov_u = u_bearing;
            *foot_gov_sit = sit.clone();
            *foot_gov_check = format!("en1997.6.5.bearing.{}.{}", footing.id, lc.id);
        }
        if gk != 1 {
            bearing = bearing.field("soil-pressure", station_fields::contact_pressure(footing.width, footing.length, e_b, v_d, u_bearing));
        }
        if gk == 1 {
            for key in [part_1::presumptive_soil_key(layer.soil_type.as_str()), "embedment-gain", "embedment-cap"] {
                bearing = bearing.ndp(&format!("ndp.en1997.gk1.presumptive.{key}"));
            }
        }
        report.push(bearing.build());

        let gamma_passive = part_1::effective_gamma_below_pa_m(layer.gamma, layer.gamma_prime, 0.0, gwl, footing.embedment.max(0.1));
        let r_h = if use_undrained {
            part_1::undrained_sliding_resistance_n(layer.cohesion_undrained, a, &p)
        } else {
            part_1::sliding_resistance_with_params(
                phi_use, layer.cohesion_effective, lc.vertical_permanent, a, &p,
                footing.embedment, footing.length, gamma_passive, footing.embedment > 0.0,
            )
        };
        let r_h = r_h.max(1.0);
        let path_hv = format!("footings[id={}].loadCases[id={}].horizontalVariable", footing.id, lc.id);
        let mut sliding = CheckResult::assess(
            format!("en1997.6.5.3.sliding.{}.{}", footing.id, lc.id),
            part1,
            ClauseId::new("EN 1997-1", "6", "6.5.3"),
            subject_entity(&footing.id, &path_hv, &label.en, &label.de),
            loc("Sliding resistance", "Gleitwiderstand"),
        )
        .utilization(Quantity::new(QuantityKind::Force, h_d), Quantity::new(QuantityKind::Force, r_h))
        .annex(annex)
        .explanation(loc(
            &format!("H_d={:.0} kN vs R_h,d={:.0} kN (passive ≤0.5·E_p,k, γ_eff={:.0}).", h_d / 1000.0, r_h / 1000.0, gamma_passive),
            &format!("H_d={:.0} kN gegen R_h,d={:.0} kN (Passiv ≤0.5·E_p,k, γ_eff={:.0}).", h_d / 1000.0, r_h / 1000.0, gamma_passive),
        ));
        if h_d > r_h {
            sliding = sliding.remedy(Remedy::at_most(
                subject_entity(&footing.id, &path_hv, &label.en, &label.de),
                Quantity::new(QuantityKind::Force, lc.horizontal_variable),
                Quantity::new(QuantityKind::Force, (r_h / p.gamma_q.max(1.0)).max(0.0)),
                loc("Reduce horizontal variable action or increase embedment/width.", "Horizontale veränderliche Einwirkung reduzieren oder Einbindung/Breite erhöhen."),
            )).remedy(Remedy::at_least(
                subject_entity(&footing.id, path_w, &label.en, &label.de),
                Quantity::length_m(footing.width),
                Quantity::length_m(b_req.max(footing.width * 1.2)),
                loc("Increase footing width to raise sliding resistance.", "Fundamentbreite erhöhen, um den Gleitwiderstand zu steigern."),
            ));
        }
        let u_sliding = if r_h > 1e-9 { h_d / r_h } else { 0.0 };
        if u_sliding > *foot_gov_u {
            *foot_gov_u = u_sliding;
            *foot_gov_sit = sit.clone();
            *foot_gov_check = format!("en1997.6.5.3.sliding.{}.{}", footing.id, lc.id);
        }
        report.push(sliding.build());
        entity_catalogue_cases::footing_sliding_catalogue(doc, footing, lc, &sit, phi_use, use_undrained.then_some(layer.cohesion_undrained), report);
        if gk != 1 && !use_undrained && layer.phi_prime_deg > 0.5 {
            entity_catalogue_cases::footing_din4017_catalogue(doc, footing, lc, &sit, layer, gamma_below, report);
        }
        if layer.cohesion_undrained > 0.0 || cohesive {
            let path_cu = format!("layers[id={}].cohesionUndrained", layer.id);
            let r_vu = part_1::undrained_bearing_resistance_n(
                layer.cohesion_undrained.max(1.0), gamma_below, footing.width, footing.length, footing.embedment, e_b, h_d, v_d, &p,
            );
            let r_vu = r_vu.max(1.0);
            let mut ub = CheckResult::assess(
                format!("en1997.6.5.bearing.undrained.{}.{}", footing.id, lc.id),
                part1,
                ClauseId::new("EN 1997-1", "6", "D.3"),
                subject_entity(&footing.id, &path_cu, &label.en, &label.de),
                loc("Undrained bearing resistance (Annex D.3)", "Undränierte Tragfähigkeit (Anhang D.3)"),
            )
            .utilization(Quantity::new(QuantityKind::Force, v_d), Quantity::new(QuantityKind::Force, r_vu))
            .annex(annex)
            .explanation(loc(
                &format!("Total-stress R/A′=(π+2)·c_u·b_c·s_c·i_c+q; c_u={:.0} kPa, R_d={:.0} kN (BS-T short-term).", layer.cohesion_undrained/1000.0, r_vu/1000.0),
                &format!("Totalspannung R/A′=(π+2)·c_u·b_c·s_c·i_c+q; c_u={:.0} kPa, R_d={:.0} kN (BS-T kurzzeitig).", layer.cohesion_undrained/1000.0, r_vu/1000.0),
            ));
            if v_d > r_vu {
                ub = ub.remedy(Remedy::at_least(
                    subject_entity(&layer.id, &path_cu, &format!("Layer {}", layer.id), &format!("Schicht {}", layer.id)),
                    Quantity::new(QuantityKind::Pressure, layer.cohesion_undrained),
                    Quantity::new(QuantityKind::Pressure, layer.cohesion_undrained * v_d / r_vu.max(1.0)),
                    loc("Increase undrained strength or footing size for short-term bearing.", "Undränierte Festigkeit oder Fundamentgröße für Kurzzeit erhöhen."),
                )).remedy(Remedy::at_least(
                    subject_entity(&footing.id, path_w, &label.en, &label.de),
                    Quantity::length_m(footing.width),
                    Quantity::length_m(footing.width * (v_d / r_vu.max(1.0)).sqrt()),
                    loc("Widen footing for undrained bearing.", "Fundament für undränierte Tragfähigkeit verbreitern."),
                ));
            }
            report.push(ub.build());
            let r_hu = part_1::undrained_sliding_resistance_n(layer.cohesion_undrained.max(1.0), a, &p);
            let r_hu = r_hu.max(1.0);
            let mut us = CheckResult::assess(
                format!("en1997.6.5.3.sliding.undrained.{}.{}", footing.id, lc.id),
                part1,
                ClauseId::new("EN 1997-1", "6", "6.5.3"),
                subject_entity(&footing.id, &path_cu, &label.en, &label.de),
                loc("Undrained sliding resistance (short-term)", "Undränierter Gleitwiderstand (kurzzeitig)"),
            )
            .utilization(Quantity::new(QuantityKind::Force, h_d), Quantity::new(QuantityKind::Force, r_hu))
            .annex(annex)
            .explanation(loc(
                &format!("R_d=A·c_u/γ_R,h; c_u={:.0} kPa, R_h,d={:.0} kN.", layer.cohesion_undrained/1000.0, r_hu/1000.0),
                &format!("R_d=A·c_u/γ_R,h; c_u={:.0} kPa, R_h,d={:.0} kN. (DE-NA)", layer.cohesion_undrained/1000.0, r_hu/1000.0),
            ));
            if h_d > r_hu {
                us = us.remedy(Remedy::at_least(
                    subject_entity(&layer.id, &path_cu, &format!("Layer {}", layer.id), &format!("Schicht {}", layer.id)),
                    Quantity::new(QuantityKind::Pressure, layer.cohesion_undrained),
                    Quantity::new(QuantityKind::Pressure, layer.cohesion_undrained * h_d / r_hu.max(1.0)),
                    loc("Increase c_u or reduce horizontal action for short-term sliding.", "c_u erhöhen oder Horizontallast für Kurzzeitgleiten reduzieren."),
                ));
            }
            report.push(us.build());
        }

        let e_perm = lc.moment_permanent / lc.vertical_permanent.max(1.0);
        let e_tot = (lc.moment_permanent + lc.moment_variable) / (lc.vertical_permanent + lc.vertical_variable).max(1.0);
        let limit_perm = footing.width / 6.0;
        let limit_tot = footing.width / 3.0;
        let ecc_ok = e_perm <= limit_perm + 1e-9 && e_tot <= limit_tot + 1e-9;
        let ecc_u = (e_perm / limit_perm.max(1e-9)).max(e_tot / limit_tot.max(1e-9));
        let mut ecc = CheckResult::assess(
            format!("en1997.6.5.4.eccentricity.{}.{}", footing.id, lc.id),
            part1054,
            ClauseId::new("DIN 1054", "7", "7.5"),
            subject_entity(&footing.id, path_w, &label.en, &label.de),
            loc("Eccentricity limits", "Ausmittenbegrenzung"),
        )
        .utilization(Quantity::length_m(ecc_u), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .annex(annex)
        .explanation(loc(
            &format!("e_perm={:.3} m (≤ B/6={:.3}), e_tot={:.3} m (≤ B/3={:.3}).", e_perm, limit_perm, e_tot, limit_tot),
            &format!("e_perm={:.3} m (≤ B/6={:.3}), e_tot={:.3} m (≤ B/3={:.3}). (DE-NA)", e_perm, limit_perm, e_tot, limit_tot),
        ));
        if !ecc_ok {
            let b_need = (6.0 * e_perm).max(3.0 * e_tot);
            ecc = ecc.remedy(Remedy::at_least(
                subject_entity(&footing.id, path_w, &label.en, &label.de),
                Quantity::length_m(footing.width),
                Quantity::length_m(b_need),
                loc(&format!("Widen footing to at least {:.2} m so eccentricity stays within B/6 and B/3.", b_need), &format!("Fundament auf mindestens {:.2} m verbreitern.", b_need)),
            ));
        }
        report.push(ecc.build());
    }
}

/// ⚙️ Checks the uplift and the dimension bounds of one spread foundation.
#[allow(clippy::too_many_arguments, reason = "the extracted phase keeps the locals of its original function as explicit parameters")]
fn check_footing_uplift_and_bounds(
    doc: &En1997Snapshot,
    report: &mut CheckReport,
    annex: AnnexChoice,
    part1: &str,
    gwl: f64,
    footing: &SpreadFoundation,
    layer: &SoilLayer,
    label: &LocalizedCopy,
) {
    {
        let mut q_sls_max: f64 = 0.0;
        let area = (footing.width * footing.length).max(1e-9_f64);
        for lc in &footing.load_cases {
            q_sls_max = q_sls_max.max((lc.vertical_permanent + lc.vertical_variable) / area);
        }
        let (s, gov_id, s_gov) = part_1::settlement_oedometric_with_gwl(
            &doc.layers, footing.width, footing.length, footing.embedment, q_sls_max, gwl,
        );
        let gov_layer = doc.layers.iter().find(|l| l.id == gov_id).unwrap_or(layer);
        let gov_path = format!("layers[id={}].oedometricModulus", gov_layer.id);
        let path_limit = id_path("footings", &footing.id, "settlementLimit");
        let mut sett = CheckResult::assess(
            format!("en1997.6.6.settlement.{}", footing.id),
            part1,
            ClauseId::new("EN 1997-1", "6", "6.6"),
            subject_entity(&footing.id, &path_limit, &label.en, &label.de),
            loc("Serviceability settlement", "Setzung (GZ 2)"),
        )
        .utilization(Quantity::length_m(s), Quantity::length_m(footing.settlement_limit))
        .annex(annex)
        .explanation(loc(
            &format!("s={:.1} mm vs limit {:.1} mm (oedometric, GWL={:.1} m, layer {}).", s * 1000.0, footing.settlement_limit * 1000.0, gwl, gov_layer.id),
            &format!("s={:.1} mm gegen Grenze {:.1} mm (ödometrisch, GW={:.1} m, Schicht {}).", s * 1000.0, footing.settlement_limit * 1000.0, gwl, gov_layer.id),
        ));
        if s > footing.settlement_limit {
            let s_other = (s - s_gov).max(0.0);
            let room = (footing.settlement_limit - s_other).max(footing.settlement_limit * 1e-6);
            let e_req = gov_layer.oedometric_modulus * (s_gov / room).max(1.0) * 1.02;
            sett = sett.remedy(Remedy::at_least(
                subject_entity(&gov_layer.id, &gov_path, &format!("Layer {}", gov_layer.id), &format!("Schicht {}", gov_layer.id)),
                Quantity::new(QuantityKind::Stress, gov_layer.oedometric_modulus),
                Quantity::new(QuantityKind::Stress, e_req),
                loc(&format!("Increase oedometric modulus of layer {} to at least {:.0} MPa.", gov_layer.id, e_req / 1e6), &format!("Steifemodul der Schicht {} auf mindestens {:.0} MPa erhöhen.", gov_layer.id, e_req / 1e6)),
            )).remedy(Remedy::at_least(
                subject_entity(&footing.id, &path_limit, &label.en, &label.de),
                Quantity::length_m(footing.settlement_limit),
                Quantity::length_m(s),
                loc("Raise the settlement limit to at least the computed settlement.", "Setzungsgrenze mindestens auf die berechnete Setzung anheben."),
            ));
        }
        report.push(sett.build());
        let (s_el, el_gov_id) = part_1::elastic_settlement_layered(&doc.layers, footing.width, footing.length, footing.embedment, q_sls_max);
        let el_gov = doc.layers.iter().find(|l| l.id == el_gov_id).unwrap_or(gov_layer);
        let nu = el_gov.poisson_ratio.clamp(0.0, 0.49);
        let e_mod = el_gov.oedometric_modulus.max(1.0);
        let path_nu = format!("layers[id={}].poissonRatio", el_gov.id);
        let mut el = CheckResult::assess(
            format!("en1997.6.6.elastic.{}", footing.id),
            part1,
            ClauseId::new("EN 1997-1", "6", "F.2"),
            subject_entity(&footing.id, &path_nu, &label.en, &label.de),
            loc("Immediate elastic settlement (Annex F.2)", "Sofortsetzung elastisch (Anhang F.2)"),
        )
        .utilization(Quantity::length_m(s_el), Quantity::length_m(footing.settlement_limit))
        .annex(annex)
        .explanation(loc(
            &format!("s_el=p·B·(1−ν²)·I/E = {:.1} mm (ν={:.2}, E_oed={:.0} MPa, layer {}).", s_el*1000.0, nu, e_mod/1e6, el_gov.id),
            &format!("s_el=p·B·(1−ν²)·I/E = {:.1} mm (ν={:.2}, E_oed={:.0} MPa, Schicht {}).", s_el*1000.0, nu, e_mod/1e6, el_gov.id),
        ));
        if s_el > footing.settlement_limit {
            el = el.remedy(Remedy::at_most(
                subject_entity(&el_gov.id, &path_nu, &format!("Layer {}", el_gov.id), &format!("Schicht {}", el_gov.id)),
                Quantity::new(QuantityKind::Dimensionless, nu),
                Quantity::new(QuantityKind::Dimensionless, (nu * 0.85).max(0.1)),
                loc("Reduce Poisson ratio or raise stiffness / widen footing for elastic settlement.", "Poissonzahl senken oder Steifigkeit erhöhen / Fundament verbreitern."),
            )).remedy(Remedy::at_least(
                subject_entity(&el_gov.id, &format!("layers[id={}].oedometricModulus", el_gov.id), &format!("Layer {}", el_gov.id), &format!("Schicht {}", el_gov.id)),
                Quantity::new(QuantityKind::Pressure, e_mod),
                Quantity::new(QuantityKind::Pressure, e_mod * s_el / footing.settlement_limit.max(1e-6)),
                loc("Increase oedometric modulus to reduce elastic settlement.", "Ödometermodul erhöhen, um Sofortsetzung zu reduzieren."),
            ));
        }
        report.push(el.build());
    }
}

/// 🧲️ Reports the governing situation of one spread foundation.
#[allow(clippy::too_many_arguments, reason = "the extracted phase keeps the locals of its original function as explicit parameters")]
fn check_footing_governing_situation(
    doc: &En1997Snapshot,
    report: &mut CheckReport,
    annex: AnnexChoice,
    part1: &str,
    part2: &str,
    gk: u8,
    footing: &SpreadFoundation,
    path_w: &str,
    label: &LocalizedCopy,
    foot_gov_u: f64,
    foot_gov_sit: &str,
    foot_gov_check: &str,
) {
    let reached = ground_derivation::reached_depth(doc);
    let deepest = doc.investigations.iter().max_by(|a, b| a.depth.partial_cmp(&b.depth).unwrap_or(std::cmp::Ordering::Equal));
    let (subject_id, subject_path) = deepest.map_or((String::new(), "investigationDepth".to_string()), |i| (i.id.clone(), format!("investigations[id={}].depth", i.id)));
    let z_min = investigation_extent::footing_required_depth_m(footing, gk);
    let z_a = part_2::min_investigation_depth_for_gk(footing.width.max(footing.length), gk);
    let mut inv = CheckResult::assess(
        format!("en1997.2.4.2.investigation.{}", footing.id),
        part2,
        ClauseId::new("EN 1997-2", "2", "2.4.2"),
        subject_entity(&subject_id, &subject_path, "Ground investigation", "Baugrunderkundung"),
        loc("Minimum investigation depth", "Mindestaufschlusstiefe"),
    )
    .minimum(Quantity::length_m(reached), Quantity::length_m(z_min))
    .annex(annex)
    .explanation(loc(
        &format!("z={:.2} m ({}); GK{gk} requires embedment {:.2} m + z_a = max(6 m, 2 B) = {:.2} m, so ≥ {:.2} m.", reached, if deepest.is_some() { "deepest investigation point" } else { "declared depth" }, footing.embedment, z_a, z_min),
        &format!("z={:.2} m ({}); GK{gk} fordert Einbindetiefe {:.2} m + z_a = max(6 m, 2 B) = {:.2} m, also ≥ {:.2} m.", reached, if deepest.is_some() { "tiefster Erkundungspunkt" } else { "angegebene Tiefe" }, footing.embedment, z_a, z_min),
    ));
    if reached < z_min {
        inv = inv.remedy(Remedy::at_least(
            subject_entity(&subject_id, &subject_path, "Ground investigation", "Baugrunderkundung"),
            Quantity::length_m(reached),
            Quantity::length_m(z_min),
            loc(&format!("Deepen investigation from {:.2} m to at least {:.2} m.", reached, z_min), &format!("Aufschlusstiefe von {:.2} m auf mindestens {:.2} m erhöhen.", reached, z_min)),
        ));
    }
    report.push(inv.build());
    if foot_gov_u >= 0.0 {
        let governing_remedies = report.checks.iter().find(|c| c.id == foot_gov_check).map(|c| c.remedies.clone()).unwrap_or_default();
        push_governing_situation_summary(
            report,
            format!("en1997.governing.situation.footing.{}", footing.id),
            part1,
            annex,
            subject_entity(&footing.id, path_w, &label.en, &label.de),
            foot_gov_sit,
            doc.design_approach.as_str(),
            foot_gov_u,
            foot_gov_check,
            &governing_remedies,
        );
    }
}

/// 🧭️ Rejects design-approach and design-situation spellings the factor tables do not know instead of silently falling back.
fn push_design_selection_check(doc: &En1997Snapshot, report: &mut CheckReport, part: &str) {
    let approach_key = doc.design_approach.as_str().to_ascii_lowercase().replace(['-', '_', ' ', '*'], "");
    let situation_key = doc.design_situation.as_str().to_ascii_lowercase().replace(['-', '_'], "");
    let approach_ok = matches!(approach_key.as_str(), "da1str" | "da1c1" | "da1geo" | "da1c2" | "da2" | "da2star" | "da2s" | "da3" | "geo3");
    let situation_ok = matches!(situation_key.as_str(), "bsp" | "bst" | "bsa" | "persistent" | "transient" | "accidental");
    let subject = subject_entity("", "designApproach", "Design approach", "Nachweisverfahren");
    let mut check = CheckResult::assess(
        "en1997.design.selection",
        part,
        ClauseId::new("EN 1997-1", "2", "2.4.7"),
        subject.clone(),
        loc("Design approach and situation", "Nachweisverfahren und Bemessungssituation"),
    )
    .annex(doc.annex)
    .explanation(loc(
        &format!("Approach '{}' and situation '{}' resolve to {} and {}.", doc.design_approach, doc.design_situation, parse_design_approach(doc.design_approach.as_str()).label(doc.annex), situation_short(doc.design_situation.as_str())),
        &format!("Nachweisverfahren '{}' und Situation '{}' werden zu {} und {} aufgelöst.", doc.design_approach, doc.design_situation, parse_design_approach(doc.design_approach.as_str()).label(doc.annex), situation_short(doc.design_situation.as_str())),
    ));
    if approach_ok && situation_ok {
        check = check.status(CheckStatus::Pass);
    } else {
        check = check.status(CheckStatus::Fail);
        if !approach_ok {
            check = check.remedy(Remedy::one_of(subject, vec!["da1str".into(), "da1geo".into(), "da2".into(), "da2star".into(), "da3".into(), "geo3".into()], loc("Select a known design approach.", "Ein bekanntes Nachweisverfahren wählen.")));
        }
        if !situation_ok {
            check = check.remedy(Remedy::one_of(subject_entity("", "designSituation", "Design situation", "Bemessungssituation"), vec!["bsP".into(), "bsT".into(), "bsA".into()], loc("Select BS-P, BS-T or BS-A.", "BS-P, BS-T oder BS-A wählen.")));
        }
    }
    report.push(check.build());
}

/// 🧱️ Ground-model consistency of one layer (EN 1997-1 §2.4.3, DIN 4020): positive thickness, no gap or overlap with the next layer, γ′ < γ, φ′ ∈ [0°, 50°], c′, c_u ≥ 0, E_oed > 0.
fn layer_model_check(doc: &En1997Snapshot, layer: &SoilLayer, annex: AnnexChoice, part: &str) -> CheckResult {
    let label = loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id));
    let target = |field: &str| subject_entity(&layer.id, &id_path("layers", &layer.id, field), &label.en, &label.de);
    let mut check = CheckResult::assess(
        format!("en1997.layer.model.{}", layer.id),
        part,
        ClauseId::new("EN 1997-1", "2", "2.4.3"),
        target("depthTop"),
        loc("Ground model consistency", "Baugrundmodell Konsistenz"),
    )
    .annex(annex);
    let mut problems_en: Vec<String> = Vec::new();
    let mut problems_de: Vec<String> = Vec::new();
    let mut failed = false;
    let mut note = |en: &str, de: &str| {
        problems_en.push(en.into());
        problems_de.push(de.into());
    };
    if layer.depth_bottom <= layer.depth_top {
        failed = true;
        note("thickness not positive", "Schichtdicke nicht positiv");
        check = check.remedy(Remedy::at_least(target("depthBottom"), Quantity::length_m(layer.depth_bottom), Quantity::length_m(layer.depth_top + 0.1), loc("Make the layer at least 0.1 m thick.", "Schicht mindestens 0,1 m dick machen.")));
    }
    let mut sorted: Vec<&SoilLayer> = doc.layers.iter().collect();
    sorted.sort_by(|a, b| a.depth_top.partial_cmp(&b.depth_top).unwrap_or(std::cmp::Ordering::Equal));
    if let Some(pos) = sorted.iter().position(|l| std::ptr::eq(*l, layer)) {
        if let Some(next) = sorted.get(pos + 1) {
            if (next.depth_top - layer.depth_bottom).abs() > 1e-6 {
                failed = true;
                note("gap or overlap with the next layer", "Lücke oder Überlappung zur nächsten Schicht");
                check = check.remedy(Remedy::exactly(
                    subject_entity(&next.id, &id_path("layers", &next.id, "depthTop"), &format!("Layer {}", next.id), &format!("Schicht {}", next.id)),
                    Quantity::length_m(next.depth_top),
                    Quantity::length_m(layer.depth_bottom),
                    loc("Start the next layer where this layer ends.", "Nächste Schicht dort beginnen lassen, wo diese endet."),
                ));
            }
        }
    }
    if layer.gamma_prime >= layer.gamma {
        failed = true;
        note("γ′ not below γ", "γ′ nicht kleiner als γ");
        check = check.remedy(Remedy::at_most(target("gammaPrime"), Quantity::new(QuantityKind::Dimensionless, layer.gamma_prime), Quantity::new(QuantityKind::Dimensionless, (layer.gamma - 9_810.0).max(1_000.0)), loc("Set γ′ = γ − γw.", "γ′ = γ − γw setzen.")));
    }
    if !(0.0..=50.0).contains(&layer.phi_prime_deg) {
        failed = true;
        note("φ′ outside 0°…50°", "φ′ außerhalb 0°…50°");
        check = check.remedy(Remedy::one_of(target("phiPrimeDeg"), vec!["0".into(), "30".into(), "50".into()], loc("Set φ′ within 0°…50°.", "φ′ im Bereich 0°…50° wählen.")));
    }
    if layer.cohesion_effective < 0.0 || layer.cohesion_undrained < 0.0 {
        failed = true;
        note("negative cohesion", "negative Kohäsion");
        check = check.remedy(Remedy::at_least(target("cohesionEffective"), Quantity::new(QuantityKind::Pressure, layer.cohesion_effective), Quantity::new(QuantityKind::Pressure, 0.0), loc("Cohesion must not be negative.", "Kohäsion darf nicht negativ sein.")));
    }
    if layer.oedometric_modulus <= 0.0 {
        failed = true;
        note("E_oed not positive", "E_oed nicht positiv");
        check = check.remedy(Remedy::at_least(target("oedometricModulus"), Quantity::new(QuantityKind::Stress, layer.oedometric_modulus), Quantity::new(QuantityKind::Stress, 1.0e6), loc("Set a positive oedometric modulus.", "Positiven Steifemodul setzen.")));
    }
    let verdict_en = if failed { problems_en.join("; ") } else { "layer consistent".into() };
    let verdict_de = if failed { problems_de.join("; ") } else { "Schicht konsistent".into() };
    check
        .status(if failed { CheckStatus::Fail } else { CheckStatus::Pass })
        .explanation(loc(
            &format!("Layer {} ({}) {:.1}…{:.1} m, φ′={:.1}°, c_u={:.0} kPa: {verdict_en}.", layer.id, layer.soil_type, layer.depth_top, layer.depth_bottom, layer.phi_prime_deg, layer.cohesion_undrained / 1000.0),
            &format!("Schicht {} ({}) {:.1}…{:.1} m, φ′={:.1}°, c_u={:.0} kPa: {verdict_de}.", layer.id, layer.soil_type, layer.depth_top, layer.depth_bottom, layer.phi_prime_deg, layer.cohesion_undrained / 1000.0),
        ))
        .build()
}
