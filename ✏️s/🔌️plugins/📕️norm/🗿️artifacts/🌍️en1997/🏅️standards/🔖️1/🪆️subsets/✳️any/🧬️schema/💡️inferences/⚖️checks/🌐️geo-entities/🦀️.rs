//! 🌐️ Checks of the snapshot entities that drive the catalogue: ground anchors (ANC-01…04), embedded walls (RW-07, and RW-12 with the groundwater table), pile groups (PILE-06), laboratory series (GI-10…12 and the characteristic lower bound GI-17), the water regime of a layer (HYD-01) and the piping at the foot of an embedded wall (HYD-02, HYD-03 with the finite-difference flow net). Each entity becomes one catalogue case; its results are retargeted to the entity row so the editor selects, highlights and re-evaluates it like any other row.

use super::*;
use crate::catalogue::{evaluate_outcomes, outcomes_to_results, Case, Outcome, R};

pub fn case(doc: &En1997Snapshot, id: &str) -> Case {
    Case::new(id, doc.annex).with_s("approach", doc.design_approach.as_str()).with_s("situation", doc.design_situation.as_str())
}

pub fn retarget(mut results: Vec<CheckResult>, collection: &str, id: &str, en: &str, de: &str) -> Vec<CheckResult> {
    let subject = SubjectRef::new(id, format!("{collection}[id={id}]"), loc(en, de));
    for result in &mut results {
        result.subject = subject.clone();
        for remedy in &mut result.remedies {
            remedy.target = subject.clone();
        }
    }
    results
}

fn evaluate(entry: &str, case: &Case, extend: impl FnOnce(&mut Vec<Outcome>)) -> Vec<CheckResult> {
    let mut outcomes: R = evaluate_outcomes(entry, case);
    if let Ok(list) = outcomes.as_mut() {
        extend(list);
    }
    outcomes_to_results(entry, case, outcomes)
}

fn link_check(doc: &En1997Snapshot, collection: &str, id: &str, en: &str, de: &str, target: &str, field: &str, options: Vec<String>) -> CheckResult {
    let subject = SubjectRef::new(id, format!("{collection}[id={id}].{field}"), loc(en, de));
    let ok = options.iter().any(|option| option == target);
    let mut check = CheckResult::assess(format!("en1997.link.{collection}.{field}.{id}"), "DIN EN 1997-1", ClauseId::new("EN 1997-1", "2", "2.4.3"), subject.clone(), loc(&format!("{en}: reference {field}"), &format!("{de}: Verweis {field}")))
        .annex(doc.annex)
        .explanation(loc(&format!("{field} = '{target}' must name an existing row."), &format!("{field} = '{target}' muss eine vorhandene Zeile benennen.")));
    check = if ok {
        check.status(CheckStatus::Pass)
    } else {
        check.status(CheckStatus::Fail).remedy(Remedy::one_of(subject, options, loc(&format!("Point {field} at an existing row."), &format!("{field} auf eine vorhandene Zeile setzen."))))
    };
    check.build()
}

pub fn check_anchors(doc: &En1997Snapshot, report: &mut CheckReport, scope: Scope<'_>) {
    for (_, anchor) in doc.anchors.iter().enumerate().filter(|(index, _)| scope.admits("anchors", *index)) {
        let (en, de) = (format!("Anchor {}", anchor.id), format!("Anker {}", anchor.id));
        report.push(link_check(doc, "anchors", &anchor.id, &en, &de, &anchor.wall_id, "wallId", doc.embedded_walls.iter().map(|w| w.id.clone()).collect()));
        let mut suitability = case(doc, &anchor.id).with_n("xi_a", anchor.test_factor).with_n("a_d", anchor.design_force);
        for (k, test) in anchor.tests.iter().enumerate() {
            suitability = suitability.with_n(&format!("ra{}", k + 1), test.pull_out_resistance);
        }
        report.extend(retarget(evaluate("ANC-01", &suitability, |_| {}), "anchors", &anchor.id, &en, &de));
        let empirical = case(doc, &anchor.id).with_n("d", anchor.fixed_diameter).with_n("l_f", anchor.fixed_length).with_n("tau_f", anchor.bond_stress).with_n("a_d", anchor.design_force);
        report.extend(retarget(evaluate("ANC-02", &empirical, |_| {}), "anchors", &anchor.id, &en, &de));
        let theta = anchor.inclination_deg;
        let force = case(doc, &anchor.id).with_n("e_h_d", anchor.design_force * theta.to_radians().cos()).with_n("theta_deg", theta);
        report.extend(retarget(evaluate("ANC-03", &force, |_| {}), "anchors", &anchor.id, &en, &de));
        let tendon = case(doc, &anchor.id).with_n("a_d", anchor.design_force).with_n("p_t_k", anchor.tendon_yield_force).with_n("l_min", anchor.min_free_length).with_n("l_free", anchor.free_length).with_s("tendon", anchor.tendon.as_str());
        report.extend(retarget(evaluate("ANC-04", &tendon, |_| {}), "anchors", &anchor.id, &en, &de));
    }
}

pub fn check_embedded_walls(doc: &En1997Snapshot, report: &mut CheckReport, scope: Scope<'_>) {
    for (_, wall) in doc.embedded_walls.iter().enumerate().filter(|(index, _)| scope.admits("embeddedWalls", *index)) {
        let (en, de) = (format!("Embedded wall {}", wall.id), format!("Eingespannte Wand {}", wall.id));
        let mut c = case(doc, &wall.id)
            .with_n("h", wall.retained_height)
            .with_n("gamma", wall.backfill_gamma)
            .with_n("phi_deg", wall.phi_deg)
            .with_n("delta_a_deg", wall.delta_active_deg)
            .with_n("delta_p_deg", wall.delta_passive_deg)
            .with_n("q", wall.surcharge)
            .with_s("kind", if wall.kind == semio_s_artifact_norm_en1997_model::EmbeddedWallKind::Cantilever { "cantilever" } else { "free_earth" });
        if wall.kind != semio_s_artifact_norm_en1997_model::EmbeddedWallKind::Cantilever {
            c = c.with_n("prop_depth", wall.prop_depth);
        }
        if wall.undrained_strength > 0.0 {
            c = c.with_n("cu_k", wall.undrained_strength).with_n("nc", 5.14);
        }
        if wall.embedment > 1e-6 && wall.phi_deg > 0.0 {
            entity_catalogue_cases::embedded_passive_catalogue(doc, wall, report);
        }
        let provided = wall.embedment;
        report.extend(retarget(
            evaluate("RW-07", &c, |outcomes| {
                if let Some(required) = outcomes.iter().find(|o| o.key == "t_design").map(|o| o.capacity) {
                    outcomes.insert(0, Outcome::length("embedment", "Required embedment t vs provided", "Erforderliche Einbindetiefe t gegen vorhandene", required, provided));
                }
            }),
            "embeddedWalls",
            &wall.id,
            &en,
            &de,
        ));
        if wall.embedment > 1e-6 && doc.groundwater_level < wall.retained_height + wall.embedment {
            if let Some(layer) = layer_at(&doc.layers, wall.retained_height + 0.5 * wall.embedment) {
                let wet = c.clone().with_n("gamma_sat", layer.gamma_prime + c.gamma_w()).with_n("z_w", doc.groundwater_level.max(0.0)).with_n("z_wf", wall.retained_height.max(doc.groundwater_level)).with_n("t_provided", provided);
                report.extend(retarget(evaluate("RW-12", &wet, |_| {}), "embeddedWalls", &wall.id, &en, &de));
            }
        }
        let head = wall.retained_height - doc.groundwater_level;
        if head > 1e-6 && wall.embedment > 1e-6 {
            if let Some(layer) = layer_at(&doc.layers, wall.retained_height + 0.5 * wall.embedment) {
                let piping = case(doc, &wall.id).with_n("delta_h", head).with_n("gamma_prime", layer.gamma_prime).with_n("d_embed", wall.embedment);
                report.extend(retarget(evaluate("HYD-02", &piping, |_| {}), "embeddedWalls", &wall.id, &en, &de));
                let flow = piping.clone().with_n("t_layer", (doc.investigation_depth - wall.retained_height).max(2.0 * wall.embedment)).with_n("cells", 8.0);
                report.extend(retarget(evaluate("HYD-03", &flow, |_| {}), "embeddedWalls", &wall.id, &en, &de));
                entity_catalogue_cases::drawdown_catalogue(doc, wall, head, layer, report);
            }
        }
    }
}

pub fn check_pile_groups(doc: &En1997Snapshot, report: &mut CheckReport, scope: Scope<'_>) {
    for (_, group) in doc.pile_groups.iter().enumerate().filter(|(index, _)| scope.admits("pileGroups", *index)) {
        let (en, de) = (format!("Pile group {}", group.id), format!("Pfahlgruppe {}", group.id));
        report.push(link_check(doc, "pileGroups", &group.id, &en, &de, &group.pile_id, "pileId", doc.piles.iter().map(|p| p.id.clone()).collect()));
        let mut c = case(doc, &group.id)
            .with_n("m", f64::from(group.rows))
            .with_n("n", f64::from(group.columns))
            .with_n("s", group.spacing)
            .with_n("v", group.cap_vertical)
            .with_n("mx", group.cap_moment_x)
            .with_n("my", group.cap_moment_y)
            .with_n("r_c_d", group.single_resistance);
        if let Some(pile) = doc.piles.iter().find(|p| p.id == group.pile_id) {
            let d = pile.diameter;
            c = c
                .with_n("d", d)
                .with_n("block_b", (f64::from(group.columns) - 1.0) * group.spacing + d)
                .with_n("block_l", (f64::from(group.rows) - 1.0) * group.spacing + d)
                .with_n("cu_h_sum", group.shaft_cohesion_sum)
                .with_n("cu_b", group.base_cohesion);
        }
        report.extend(retarget(evaluate("PILE-06", &c, |_| {}), "pileGroups", &group.id, &en, &de));
    }
}

pub fn check_lab_tests(doc: &En1997Snapshot, report: &mut CheckReport, scope: Scope<'_>) {
    for (_, test) in doc.lab_tests.iter().enumerate().filter(|(index, _)| scope.admits("labTests", *index)) {
        let (en, de) = (format!("Laboratory test {}", test.id), format!("Laborversuch {}", test.id));
        report.push(link_check(doc, "labTests", &test.id, &en, &de, &test.layer_id, "layerId", doc.layers.iter().map(|l| l.id.clone()).collect()));
        let layer = doc.layers.iter().find(|l| l.id == test.layer_id);
        let mut c = case(doc, &test.id);
        let (entry, x_key, y_key) = match test.kind {
            semio_s_artifact_norm_en1997_model::LabTestKind::Triaxial => ("GI-11", "p", "q"),
            semio_s_artifact_norm_en1997_model::LabTestKind::Oedometer => ("GI-10", "", ""),
            _ => ("GI-12", "sigma", "tau"),
        };
        if entry == "GI-10" {
            let (h, t90) = (test.drainage_height, test.consolidation_time);
            let outcomes: R = if h > 0.0 && t90 > 0.0 {
                Ok(vec![
                    Outcome::number("e0", "Initial void ratio e₀", "Anfangsporenzahl e₀", test.void_ratio),
                    Outcome::pascal("sigma1", "Preload stress σ₁", "Vorbelastung σ₁", test.preload_stress),
                    Outcome::number("cv", "Coefficient of consolidation c_v = T90·H²/t90", "Konsolidierungsbeiwert c_v = T90·H²/t90", crate::catalogue::settlement::t90_factor() * h * h / t90),
                ])
            } else {
                Err("input h_dr and t90 must be positive".into())
            };
            report.extend(retarget(outcomes_to_results("GI-10", &c, outcomes), "labTests", &test.id, &en, &de));
            if let Some(layer) = layer {
                entity_catalogue_cases::lab_settlement_catalogue(doc, test, layer, report);
            }
            continue;
        }
        for (k, point) in test.points.iter().enumerate() {
            c = c.with_n(&format!("{x_key}{}", k + 1), point.normal_stress).with_n(&format!("{y_key}{}", k + 1), point.shear_stress);
        }
        let layer_values = layer.map(|l| (l.phi_prime_deg, l.cohesion_effective));
        let mut series = case(doc, &test.id).with_s("kind", if entry == "GI-11" { "triaxial" } else { "shear" });
        for (k, point) in test.points.iter().enumerate() {
            series = series.with_n(&format!("{x_key}{}", k + 1), point.normal_stress).with_n(&format!("{y_key}{}", k + 1), point.shear_stress);
        }
        if let Some((layer_phi, layer_c)) = layer_values {
            series = series.with_n("phi_assumed_deg", layer_phi).with_n("c_assumed", layer_c);
        }
        report.extend(retarget(evaluate("GI-17", &series, |_| {}), "labTests", &test.id, &en, &de));
        report.extend(retarget(
            evaluate(entry, &c, |outcomes| {
                let lab = |key: &str| outcomes.iter().find(|o| o.key == key).map(|o| o.capacity);
                if let (Some((layer_phi, layer_c)), Some(lab_phi), Some(lab_c)) = (layer_values, lab("phi_deg"), lab("c")) {
                    outcomes.insert(0, Outcome::ratio("c_layer", "Layer c′ vs laboratory c′", "Schicht-c′ gegen Labor-c′", layer_c, lab_c.max(0.0)));
                    outcomes.insert(0, Outcome::ratio("phi_layer", "Layer φ′ vs laboratory φ′", "Schicht-φ′ gegen Labor-φ′", layer_phi, lab_phi));
                }
            }),
            "labTests",
            &test.id,
            &en,
            &de,
        ));
    }
}

pub fn check_layer_waters(doc: &En1997Snapshot, report: &mut CheckReport, scope: Scope<'_>) {
    for (_, water) in doc.layer_waters.iter().enumerate().filter(|(index, _)| scope.admits("layerWaters", *index)) {
        let (en, de) = (format!("Layer water {}", water.id), format!("Schichtwasser {}", water.id));
        report.push(link_check(doc, "layerWaters", &water.id, &en, &de, &water.layer_id, "layerId", doc.layers.iter().map(|l| l.id.clone()).collect()));
        let Some(layer) = doc.layers.iter().find(|l| l.id == water.layer_id) else { continue };
        entity_catalogue_cases::well_catalogue(doc, water, layer, report);
        let c = case(doc, &water.id).with_n("delta_h", water.head_difference).with_n("l_flow", water.flow_length).with_n("gamma_prime", layer.gamma_prime);
        let gamma_w = c.gamma_w();
        let mid = 0.5 * (layer.depth_top + layer.depth_bottom);
        let pore = gamma_w * (mid - water.piezometric_depth);
        report.extend(retarget(
            evaluate("HYD-01", &c, |outcomes| {
                outcomes.push(Outcome::pascal("u_mid", "Pore pressure at layer mid-depth u = γw·(z − z_piezometric)", "Porenwasserdruck in Schichtmitte u = γw·(z − z_Standrohr)", pore.max(0.0)));
                outcomes.push(Outcome::number("k", "Layer permeability k (m/s)", "Schichtdurchlässigkeit k (m/s)", water.permeability));
            }),
            "layerWaters",
            &water.id,
            &en,
            &de,
        ));
    }
}
