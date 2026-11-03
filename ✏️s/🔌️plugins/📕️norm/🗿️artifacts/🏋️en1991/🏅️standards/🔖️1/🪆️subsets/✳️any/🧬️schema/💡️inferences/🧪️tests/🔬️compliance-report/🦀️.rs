use semio_framework_test_fixtures::fixture_path;
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckStatus};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
use semio_s_artifact_norm_en1991_inferences::{evaluate, check_full_actions};
use semio_s_artifact_norm_en1991_examples::example_subjects::{de_office_compliant, multi_fail_noncompliant};
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn default_subject_complies_de_na() {
    let report = evaluate(&En1991Snapshot::example(AnnexChoice::De));
    assert!(report.checks.iter().all(|c| c.status != CheckStatus::Fail), "{:?}", report.checks.iter().filter(|c| c.status == CheckStatus::Fail).map(|c| &c.id).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn de_office_compliant_example_passes() {
    let report = evaluate(&de_office_compliant());
    let fails: Vec<_> = report.checks.iter().filter(|c| c.status == CheckStatus::Fail).map(|c| c.id.clone()).collect();
    assert!(fails.is_empty(), "unexpected fails: {fails:?}");
    assert!(fails.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn the_fire_load_check_uses_exactly_the_chosen_basis_and_cites_it() {
    use semio_s_artifact_norm_en1991_model::FireLoadBasis;
    use semio_s_artifact_norm_en1991_examples::example_subjects::de_fire_parametric_compliant;
    let fire_load = |doc: &En1991Snapshot| evaluate(doc).checks.into_iter().find(|c| c.id == "en1991.1-2.fire-load").expect("fire-load check");
    let table = En1991Snapshot { fire_load_basis: FireLoadBasis::OccupancyTable, fire_load_density_qf: 900.0e6, ..de_fire_parametric_compliant() };
    let user = En1991Snapshot { fire_load_basis: FireLoadBasis::UserSpecified, ..table.clone() };
    let (by_table, by_user) = (fire_load(&table), fire_load(&user));
    assert!(by_table.explanation.en.contains("Table E.4 row \"office\"") && by_table.explanation.de.contains("Tabelle E.4 Zeile \"office\""), "{}", by_table.explanation.en);
    assert!(by_user.explanation.en.contains("user-specified q_f,k = 900 MJ/m²") && by_user.explanation.de.contains("vorgegebenes q_f,k = 900 MJ/m²"), "{}", by_user.explanation.en);
    assert!((by_user.computed.value / by_table.computed.value - 900.0 / 511.0).abs() < 1e-9, "the required load is q_f,k of the chosen basis, never the larger of both");
    assert!(by_table.ndps.iter().any(|id| id.starts_with("ndp.en1991.1-2.fire-load-fractile.")) && !by_user.ndps.iter().any(|id| id.starts_with("ndp.en1991.1-2.fire-load-fractile.")));
    let missing = En1991Snapshot { fire_load_basis: FireLoadBasis::UserSpecified, fire_load_density_qf: 0.0, ..table.clone() };
    assert!(evaluate(&missing).checks.iter().any(|c| c.id == "en1991.input.fireLoadDensityQf"), "a non-positive user value is an invalid input, not a silent table fallback");
}

#[semio_framework_async_macros::async_test]
async fn noncompliant_raises_fail_with_remedy() {
    let report = evaluate(&multi_fail_noncompliant());
    let fails: Vec<_> = report.checks.iter().filter(|c| c.status == CheckStatus::Fail).collect();
    assert!(fails.len() >= 3, "expected multiple fails, got {}", fails.len());
    assert!(fails.iter().all(|c| !c.remedies.is_empty()), "every Fail needs a remedy");
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_writing_required_makes_check_pass() {
    let mut doc = multi_fail_noncompliant();
    let report = evaluate(&doc);
    let fail = report.checks.iter().find(|c| c.status == CheckStatus::Fail && c.remedies.iter().any(|r| r.applicable)).expect("applicable fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable).unwrap();
    let path = remedy.target.path.as_str();
    let required = remedy.required.value;
    apply_si_path(&mut doc, path, required);
    let after = evaluate(&doc);
    let same = after.checks.iter().find(|c| c.id == fail.id).expect("check still present");
    assert!(same.status != CheckStatus::Fail || semio_s_plugin_norm_compliance::within_limit(same.utilization), "remedy did not fix {}: status={:?} util={}", fail.id, same.status, same.utilization);
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_two_distinct_checks_flip_to_pass() {
    let mut doc = multi_fail_noncompliant();
    let report = evaluate(&doc);
    let mut targets = Vec::new();
    for fail in report.checks.iter().filter(|c| c.status == CheckStatus::Fail) {
        if let Some(remedy) = fail.remedies.iter().find(|r| r.applicable) {
            if fail.id.contains("imposed") || fail.id.contains("snow") {
                targets.push((fail.id.clone(), remedy.target.path.clone(), remedy.required.value));
            }
        }
    }
    assert!(targets.len() >= 2, "need imposed and snow fails, got {targets:?}");
    targets.truncate(2);
    for (_, path, required) in &targets {
        apply_si_path(&mut doc, path, *required);
    }
    let after = evaluate(&doc);
    for (id, _, _) in &targets {
        let same = after.checks.iter().find(|c| c.id == *id).expect("check present");
        assert!(same.status != CheckStatus::Fail || semio_s_plugin_norm_compliance::within_limit(same.utilization), "remedy did not fix {id}: {:?}", same.status);
    }
}

#[semio_framework_async_macros::async_test]
async fn emitted_entity_paths_use_id_selectors_and_resolve() {
    let doc = En1991Snapshot::example(AnnexChoice::De);
    let report = evaluate(&doc);
    let mut seen = 0usize;
    for check in &report.checks {
        let path = &check.subject.path;
        if path.contains('[') {
            assert!(path.contains("[id="), "entity path must use [id=…]: {path}");
            assert!(path_resolves(&doc, path), "emitted path must resolve: {path}");
            seen += 1;
        }
        for remedy in &check.remedies {
            let rp = &remedy.target.path;
            if rp.contains('[') {
                assert!(rp.contains("[id="), "remedy path must use [id=…]: {rp}");
                assert!(path_resolves(&doc, rp), "remedy path must resolve: {rp}");
            }
        }
    }
    assert!(seen >= 3, "expected several entity paths, got {seen}");
}

fn path_resolves(doc: &En1991Snapshot, path: &str) -> bool {
    if let Some(rest) = path.strip_prefix("floors[id=") {
        let Some((id, _)) = rest.split_once("].") else { return false };
        return doc.floors.iter().any(|f| f.id == id);
    }
    if let Some(rest) = path.strip_prefix("selfWeightElements[id=") {
        let Some((id, _)) = rest.split_once("].") else { return false };
        return doc.self_weight_elements.iter().any(|e| e.id == id);
    }
    if let Some(rest) = path.strip_prefix("roofs[id=") {
        let Some((id, _)) = rest.split_once("].") else { return false };
        return doc.roofs.iter().any(|r| r.id == id);
    }
    if let Some(rest) = path.strip_prefix("windFaces[id=") {
        let Some((id, _)) = rest.split_once("].") else { return false };
        return doc.wind_faces.iter().any(|f| f.id == id);
    }
    if let Some(rest) = path.strip_prefix("accidentalCases[id=") {
        let Some((id, _)) = rest.split_once("].") else { return false };
        return doc.accidental_cases.iter().any(|c| c.id == id);
    }
    if let Some(rest) = path.strip_prefix("bridgeLanes[id=") {
        let Some((id, _)) = rest.split_once("].") else { return false };
        return doc.bridge_lanes.iter().any(|l| l.id == id);
    }
    if let Some(rest) = path.strip_prefix("constructionStages[id=") {
        let Some((id, _)) = rest.split_once("].") else { return false };
        return doc.construction_stages.iter().any(|s| s.id == id);
    }
    if let Some(rest) = path.strip_prefix("roofSurfaces[id=") {
        let Some((id, _)) = rest.split_once("].") else { return false };
        return doc.roof_surfaces.iter().any(|s| s.id == id);
    }
    for (prefix, ids) in [
        ("specialVehicles[id=", doc.special_vehicles.iter().map(|r| r.id.as_str()).collect::<Vec<_>>()),
        ("windBridges[id=", doc.wind_bridges.iter().map(|r| r.id.as_str()).collect()),
        ("dynamicElements[id=", doc.dynamic_elements.iter().map(|r| r.id.as_str()).collect()),
        ("impactActions[id=", doc.impact_actions.iter().map(|r| r.id.as_str()).collect()),
        ("internalExplosions[id=", doc.internal_explosions.iter().map(|r| r.id.as_str()).collect()),
        ("thermalSections[id=", doc.thermal_sections.iter().map(|r| r.id.as_str()).collect()),
        ("helicopterLandings[id=", doc.helicopter_landings.iter().map(|r| r.id.as_str()).collect()),
        ("tieRuns[id=", doc.tie_runs.iter().map(|r| r.id.as_str()).collect()),
        ("constructionLoads[id=", doc.construction_loads.iter().map(|r| r.id.as_str()).collect()),
        ("fallingLoads[id=", doc.falling_loads.iter().map(|r| r.id.as_str()).collect()),
        ("executionImperfections[id=", doc.execution_imperfections.iter().map(|r| r.id.as_str()).collect()),
        ("derailmentCases[id=", doc.derailment_cases.iter().map(|r| r.id.as_str()).collect()),
        ("fatigueModels[id=", doc.fatigue_models.iter().map(|r| r.id.as_str()).collect()),
        ("supportCollisions[id=", doc.support_collisions.iter().map(|r| r.id.as_str()).collect()),
        ("localFires[id=", doc.local_fires.iter().map(|r| r.id.as_str()).collect()),
        ("snowGuards[id=", doc.snow_guards.iter().map(|r| r.id.as_str()).collect()),
        ("dustExplosions[id=", doc.dust_explosions.iter().map(|r| r.id.as_str()).collect()),
        ("railLoadGroups[id=", doc.rail_load_groups.iter().map(|r| r.id.as_str()).collect()),
        ("railDerailments[id=", doc.rail_derailments.iter().map(|r| r.id.as_str()).collect()),
    ] {
        if let Some(rest) = path.strip_prefix(prefix) {
            let Some((id, _)) = rest.split_once("].") else { return false };
            return ids.contains(&id);
        }
    }
    !path.contains('[')
}

#[semio_framework_async_macros::async_test]
async fn snow_zone2_150m_roof_mu08() {
    let sk = semio_s_artifact_norm_en1991_checks::part_1_3::ground_snow_pa("2", 150.0).expect("zone 2");
    assert!((sk - 850.0).abs() < 1e-6);
    let s = sk * 0.8;
    assert!((s - 680.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn imposed_category_b1_de() {
    let q = semio_s_artifact_norm_en1991_checks::part_1_1::imposed_qk_pa("B1", AnnexChoice::De).expect("B1");
    assert!((q - 2000.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn bridge_crane_silo_claim_gates() {
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.structure_kind = semio_s_artifact_norm_en1991_model::StructureKind::Building;
    doc.crane_claimed = false;
    doc.silo_claimed = false;
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("bridge.none") || c.id.contains("2.bridge")));
}

fn apply_si_path(doc: &mut En1991Snapshot, path: &str, value: f64) {
    if let Some(rest) = path.strip_prefix("floors[id=") {
        let (id, field) = rest.split_once("].").expect("floor path");
        if let Some(f) = doc.floors.iter_mut().find(|f| f.id == id) {
            match field {
                "assumedQk" => f.assumed_qk = value,
                "assumedQkConcentrated" => f.assumed_qk_concentrated = value,
                "assumedPartitions" => f.assumed_partitions = value,
                _ => panic!("unknown floor field {field}"),
            }
            return;
        }
    }
    if let Some(rest) = path.strip_prefix("selfWeightElements[id=") {
        let (id, field) = rest.split_once("].").expect("sw path");
        if let Some(el) = doc.self_weight_elements.iter_mut().find(|e| e.id == id) {
            match field {
                "assumedGk" => el.assumed_gk = value,
                _ => panic!("unknown sw field {field}"),
            }
            return;
        }
    }
    if let Some(rest) = path.strip_prefix("roofs[id=") {
        let (id, field) = rest.split_once("].").expect("roof path");
        if let Some(r) = doc.roofs.iter_mut().find(|r| r.id == id) {
            match field {
                "assumedSk" => r.assumed_sk = value,
                _ => panic!("unknown roof field {field}"),
            }
            return;
        }
    }
    if let Some(rest) = path.strip_prefix("windFaces[id=") {
        let (id, field) = rest.split_once("].").expect("wind path");
        if let Some(f) = doc.wind_faces.iter_mut().find(|f| f.id == id) {
            match field {
                "assumedWp" => f.assumed_wp = value,
                "cPe10" => f.c_pe10 = value,
                "cPe1" => f.c_pe1 = value,
                _ => panic!("unknown wind field {field}"),
            }
            return;
        }
    }
    if let Some(rest) = path.strip_prefix("bridgeLanes[id=") {
        let (id, field) = rest.split_once("].").expect("lane path");
        if let Some(l) = doc.bridge_lanes.iter_mut().find(|l| l.id == id) {
            match field {
                "width" => l.width = value,
                "offset" => l.offset = value,
                "lm1Lane" => l.lm1_lane = value as u8,
                _ => panic!("unknown lane field {field}"),
            }
            return;
        }
    }
    if let Some(rest) = path.strip_prefix("constructionStages[id=") {
        let (id, field) = rest.split_once("].").expect("stage path");
        if let Some(s) = doc.construction_stages.iter_mut().find(|s| s.id == id) {
            match field {
                "durationDays" => s.duration_days = value,
                "returnYears" => s.return_years = value,
                "assumedQk" => s.assumed_qk = value,
                _ => panic!("unknown stage field {field}"),
            }
            return;
        }
    }
    if let Some(rest) = path.strip_prefix("roofSurfaces[id=") {
        let (id, field) = rest.split_once("].").expect("surface path");
        if let Some(s) = doc.roof_surfaces.iter_mut().find(|s| s.id == id) {
            match field {
                "pitchDeg" => s.pitch_deg = value,
                "extent" => s.extent = value,
                _ => panic!("unknown surface field {field}"),
            }
            return;
        }
    }
    if let Some(rest) = path.strip_prefix("windBridges[id=") {
        let (id, field) = rest.split_once("].").expect("wind bridge path");
        if let Some(r) = doc.wind_bridges.iter_mut().find(|r| r.id == id) {
            match field {
                "assumedForce" => r.assumed_force = value,
                _ => panic!("unknown wind bridge field {field}"),
            }
            return;
        }
    }
    if let Some(rest) = path.strip_prefix("dynamicElements[id=") {
        let (id, field) = rest.split_once("].").expect("dynamic path");
        if let Some(r) = doc.dynamic_elements.iter_mut().find(|r| r.id == id) {
            match field {
                "assumedCscd" => r.assumed_cscd = value,
                _ => panic!("unknown dynamic element field {field}"),
            }
            return;
        }
    }
    if let Some(rest) = path.strip_prefix("impactActions[id=") {
        let (id, field) = rest.split_once("].").expect("impact path");
        if let Some(r) = doc.impact_actions.iter_mut().find(|r| r.id == id) {
            match field {
                "assumedForce" => r.assumed_force = value,
                _ => panic!("unknown impact field {field}"),
            }
            return;
        }
    }
    for (prefix, field_name) in [("helicopterLandings[id=", "assumedForce"), ("tieRuns[id=", "providedForce"), ("fallingLoads[id=", "assumedForce"), ("executionImperfections[id=", "assumedForce"), ("derailmentCases[id=", "assumedForce")] {
        if let Some(rest) = path.strip_prefix(prefix) {
            let (id, field) = rest.split_once("].").expect("hazard path");
            assert_eq!(field, field_name, "unknown field of {prefix}");
            match prefix {
                "helicopterLandings[id=" => doc.helicopter_landings.iter_mut().filter(|r| r.id == id).for_each(|r| r.assumed_force = value),
                "tieRuns[id=" => doc.tie_runs.iter_mut().filter(|r| r.id == id).for_each(|r| r.provided_force = value),
                "fallingLoads[id=" => doc.falling_loads.iter_mut().filter(|r| r.id == id).for_each(|r| r.assumed_force = value),
                "executionImperfections[id=" => doc.execution_imperfections.iter_mut().filter(|r| r.id == id).for_each(|r| r.assumed_force = value),
                _ => doc.derailment_cases.iter_mut().filter(|r| r.id == id).for_each(|r| r.assumed_force = value),
            }
            return;
        }
    }
    if let Some(rest) = path.strip_prefix("internalExplosions[id=") {
        let (id, field) = rest.split_once("].").expect("explosion path");
        if let Some(r) = doc.internal_explosions.iter_mut().find(|r| r.id == id) {
            match field {
                "assumedPressure" => r.assumed_pressure = value,
                _ => panic!("unknown explosion field {field}"),
            }
            return;
        }
    }
    if let Some(rest) = path.strip_prefix("thermalSections[id=") {
        let (id, field) = rest.split_once("].").expect("section path");
        if let Some(r) = doc.thermal_sections.iter_mut().find(|r| r.id == id) {
            match field {
                "assumedRange" => r.assumed_range = value,
                _ => panic!("unknown section field {field}"),
            }
            return;
        }
    }
    if let Some(rest) = path.strip_prefix("accidentalCases[id=") {
        let (id, field) = rest.split_once("].").expect("acc path");
        if let Some(c) = doc.accidental_cases.iter_mut().find(|c| c.id == id) {
            if field == "assumedForce" || field.ends_with("assumedForce") {
                if let Some(imp) = c.impact.first_mut() { imp.assumed_force = value; return; }
            }
            if field == "assumedPressure" || field.ends_with("assumedPressure") {
                if let Some(ex) = c.explosion.first_mut() { ex.assumed_pressure = value; return; }
            }
            if let Some(rest) = field.strip_prefix("impact[0].") {
                if let Some(imp) = c.impact.first_mut() {
                    match rest {
                        "assumedForce" => { imp.assumed_force = value; return; }
                        "vehicleMass" => { imp.vehicle_mass = value; return; }
                        "vehicleSpeed" => { imp.vehicle_speed = value; return; }
                        _ => {}
                    }
                }
            }
            if let Some(rest) = field.strip_prefix("explosion[0].") {
                if let Some(ex) = c.explosion.first_mut() {
                    match rest {
                        "assumedPressure" => { ex.assumed_pressure = value; return; }
                        "explosionMass" => { ex.explosion_mass = value; return; }
                        "standoff" => { ex.standoff = value; return; }
                        _ => {}
                    }
                }
            }
            panic!("unknown acc field {field}");
        }
    }
    match path {
        "assumedBridgeTandem" => doc.assumed_bridge_tandem = value,
        "assumedBridgeUdl" => doc.assumed_bridge_udl = value,
        "assumedBridgeLm2" => doc.assumed_bridge_lm2 = value,
        "assumedBridgeLm3" => doc.assumed_bridge_lm3 = value,
        "assumedBridgeLm4" => doc.assumed_bridge_lm4 = value,
        "assumedBridgeFootway" => doc.assumed_bridge_footway = value,
        "assumedCraneWheel" => doc.assumed_crane_wheel = value,
        "assumedCraneHorizontal" => doc.assumed_crane_horizontal = value,
        "assumedSiloPressure" => doc.assumed_silo_pressure = value,
        "assumedSiloPatch" => doc.assumed_silo_patch = value,
        "assumedSiloWallFriction" => doc.assumed_silo_wall_friction = value,
        "bridgeTandemX" => doc.bridge_tandem_x = value,
        "railAlpha" => doc.rail_alpha = value,
        "assumedRailLm71" => doc.assumed_rail_lm71 = value,
        "assumedRailLm71Udl" => doc.assumed_rail_lm71_udl = value,
        "assumedRailSw0" => doc.assumed_rail_sw0 = value,
        "assumedRailSw2" => doc.assumed_rail_sw2 = value,
        "craneX" => doc.crane_x = value,
        "craneRunwayLength" => doc.crane_runway_length = value,
        "craneWheelBase" => doc.crane_wheel_base = value,
        "siloDiameter" => doc.silo_diameter = value,
        "siloEccentricity" => doc.silo_eccentricity = value,
        "siloHopperAngleDeg" => doc.silo_hopper_angle_deg = value,
        "bridgeCurveRadius" => doc.bridge_curve_radius = value,
        "assumedDeltaT" => doc.assumed_delta_t = value,
        "assumedConstructionQk" => doc.assumed_construction_qk = value,
        "assumedGasTemperature" => doc.assumed_gas_temperature = value,
        "assumedHNet" => doc.assumed_h_net = value,
        "assumedQfD" => doc.assumed_qf_d = value,
        other => panic!("unsupported SI path {other}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn structure_kind_runs_lm1_udl_lm2_footway() {
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.structure_kind = semio_s_artifact_norm_en1991_model::StructureKind::Bridge;
    doc.assumed_bridge_udl = 3600.0;
    doc.assumed_bridge_tandem = 270_000.0;
    doc.assumed_bridge_lm2 = 360_000.0;
    doc.assumed_bridge_lm3 = 200_000.0;
    doc.assumed_bridge_lm4 = 5000.0;
    doc.assumed_bridge_footway = 5000.0;
    let report = evaluate(&doc);
    let ids: Vec<_> = report.checks.iter().map(|c| c.id.as_str()).collect();
    assert!(ids.iter().any(|id| id.contains("lm1-tandem")));
    assert!(ids.iter().any(|id| id.contains("lm1-udl")));
    assert!(ids.iter().any(|id| id.contains("lm2")));
    assert!(ids.iter().any(|id| id.contains("lm3")));
    assert!(ids.iter().any(|id| id.contains("lm4")));
    assert!(ids.iter().any(|id| id.contains("footway")));
}

#[semio_framework_async_macros::async_test]
async fn crane_and_silo_applicability_gating() {
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.crane_claimed = false;
    doc.silo_claimed = false;
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("crane") && c.status == CheckStatus::NotApplicable));
    assert!(report.checks.iter().any(|c| c.id.contains("silo") && c.status == CheckStatus::NotApplicable));
}

#[test]
fn regen_dsl_and_pack_assets_match_the_codecs_or_bless() {
    use semio_s_artifact_norm_en1991_io::binary::snapshot::encode_en1991_pack;
    use semio_s_artifact_norm_en1991_io::text::snapshot::{decode_en1991_dsl, encode_en1991_dsl};
    let assets = fixture_path!("../..").join("🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets");
    semio_s_plugin_norm_compliance::regen::bless_dsl_and_pack_assets(&assets, &|text| {
        let document = decode_en1991_dsl(text)?;
        Ok((encode_en1991_dsl(&document), encode_en1991_pack(&document)))
    });
}

#[semio_framework_async_macros::async_test]
async fn python_oracle_within_half_percent() {
    use semio_s_artifact_norm_en1991_io::text::snapshot::encode_en1991_snapshot_json;
    let doc = En1991Snapshot::example(AnnexChoice::De);
    let report = evaluate(&doc);
    let json = encode_en1991_snapshot_json(&doc);
    let any = fixture_path!("../..")
        .join("🏅️standards").read_dir().unwrap().next().unwrap().unwrap().path()
        .read_dir().unwrap().map(|e| e.unwrap().path()).find(|p| p.is_dir()).unwrap()
        .read_dir().unwrap().map(|e| e.unwrap().path()).find(|p| p.is_dir()).unwrap();
    let oracle = any.join("🔮️oracles").join("⚖️compliance").join("🐍️.py");
    let out = semio_s_plugin_norm_compliance::python_oracle::run_ok(&oracle, &[], Some(json.as_bytes()));
    let parsed: serde_json::Value = serde_json::from_slice(&out.stdout).expect("oracle json");
    let arr = parsed.as_array().expect("array");
    for item in arr {
        let id = item["id"].as_str().unwrap().replace("selfweight", "self-weight");
        let req = item["required"].as_f64().unwrap();
        if let Some(check) = report.checks.iter().find(|c| c.id == id) {
            let rel = ((check.computed.value - req) / req.max(1.0)).abs();
            assert!(rel <= 0.005, "oracle mismatch on {id}: rust={} py={} rel={rel}", check.computed.value, req);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn snapshot_json_validates_against_schema() {
    use semio_s_artifact_norm_en1991_io::text::snapshot::encode_en1991_snapshot_json;
    use std::process::Command;
    let doc = En1991Snapshot::example(AnnexChoice::De);
    let json = encode_en1991_snapshot_json(&doc);
    let any = fixture_path!("../..")
        .join("🏅️standards").read_dir().unwrap().next().unwrap().unwrap().path()
        .read_dir().unwrap().map(|e| e.unwrap().path()).find(|p| p.is_dir()).unwrap()
        .read_dir().unwrap().map(|e| e.unwrap().path()).find(|p| p.is_dir()).unwrap();
    let schema = any.join("🧬️schema").join("📸️snapshot").join("🔣️.json");
    let py = "import json,sys\nimport jsonschema\njsonschema.validate(json.loads(sys.argv[2]), json.load(open(sys.argv[1])))\nprint('ok')\n";
    let out = semio_s_plugin_norm_compliance::python_oracle::command().arg("-c").arg(py).arg(schema.to_str().unwrap()).arg(&json).output().expect("jsonschema");
    assert!(out.status.success(), "jsonschema failed (third-party jsonschema required): status={} stdout={} stderr={}", out.status, String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    assert!(!String::from_utf8_lossy(&out.stdout).contains("skip"), "jsonschema must not skip");
}

#[semio_framework_async_macros::async_test]
async fn field_meta_coverage_all_committed_examples() {
    use semio_s_artifact_norm_en1991_editor::field_meta::en1991_field_meta;
    use semio_s_artifact_norm_en1991_examples::example_subjects::{
        de_accidental_variants, de_bridge_compliant, de_bridge_noncompliant, de_fire_parametric_compliant,
        de_fire_parametric_noncompliant, de_office_compliant, multi_fail_noncompliant,
    };
    use semio_s_artifact_norm_en1991_io::text::snapshot::encode_en1991_snapshot_json;
    for doc in [
        de_office_compliant(),
        multi_fail_noncompliant(),
        de_bridge_compliant(),
        de_bridge_noncompliant(),
        de_fire_parametric_compliant(),
        de_fire_parametric_noncompliant(),
        de_accidental_variants(),
    ] {
        let json: serde_json::Value = serde_json::from_str(&encode_en1991_snapshot_json(&doc)).unwrap();
        fn walk(v: &serde_json::Value, path: &str, out: &mut Vec<String>) {
            match v {
                serde_json::Value::Object(map) => {
                    for (k, child) in map {
                        let next = if path.is_empty() { k.clone() } else { format!("{path}.{k}") };
                        out.push(next.clone());
                        walk(child, &next, out);
                    }
                }
                serde_json::Value::Array(items) => {
                    for (i, child) in items.iter().enumerate() {
                        if let Some(id) = child.get("id").and_then(|x| x.as_str()) {
                            walk(child, &format!("{path}[id={id}]"), out);
                        } else {
                            walk(child, &format!("{path}[{i}]"), out);
                        }
                    }
                }
                _ => {}
            }
        }
        let mut paths = Vec::new();
        walk(&json, "", &mut paths);
        for path in paths {
            let leaf = path.rsplit(['.', '[']).next().unwrap_or(&path).trim_end_matches(']');
            if leaf == "id" {
                continue;
            }
            let meta = en1991_field_meta(&path).or_else(|| en1991_field_meta(leaf));
            assert!(meta.is_some(), "missing field meta for path={path} leaf={leaf}");
            let m = meta.unwrap();
            assert!(!m.label.en.is_empty() && !m.label.de.is_empty(), "empty labels for {path}");
            if let Some(choices) = m.choices() {
                for c in choices {
                    assert!(!c.label_en.is_empty() && !c.label_de.is_empty());
                    assert!(
                        c.label_en != c.value || c.value.len() <= 3,
                        "raw en choice for {path}: {}",
                        c.label_en
                    );
                    assert!(
                        c.label_de != c.value || c.value.len() <= 3,
                        "raw de choice for {path}: {}",
                        c.label_de
                    );
                }
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn scope_aware_perturbation_of_editable_leaves() {
    use semio_s_artifact_norm_en1991_examples::example_subjects::{
        de_accidental_variants, de_bridge_compliant, de_fire_parametric_compliant, de_office_compliant,
    };
    use semio_s_artifact_norm_en1991_model::{FireMode, StructureKind};
    let office_na = [
        "enSk", "enVb", "thermalBridgeType", "mixedTerrainUpwind",
        "fireCurve", "fireDuration", "assumedGasTemperature", "assumedHNet", "fireCompartmentArea",
        "fireCompartmentHeight", "fireOpeningFactor", "fireThermalInertia", "fireOccupancy", "fireLoadBasis", "fireLoadDensityQf",
        "assumedQfD", "bridgeLane", "bridgeSpan", "bridgeLaneWidth", "bridgeTandemX", "railAlpha", "assumedRailLm71", "assumedRailLm71Udl", "assumedRailSw0", "assumedRailSw2", "craneX", "craneRunwayLength", "craneWheelBase", "siloDiameter", "siloEccentricity", "siloHopperAngleDeg", "bridgeCurveRadius", "assumedBridgeTandem", "assumedBridgeUdl",
        "assumedBridgeLm2", "assumedBridgeFootway", "assumedBridgeLm3", "assumedBridgeLm4", "bridgeLoadGroup",
    ];
    let bridge_na = [
        "enSk", "enVb", "mixedTerrainUpwind", "tMax", "tMin", "t0",
        "airDensity", "windZone", "terrainCategory", "orographyFactor", "coastOrIsland", "mixedTerrainDistance",
        "height", "width", "depth", "altitude", "snowZone", "northGermanLowlandSnow",
        "fireMode", "fireCurve", "fireDuration", "assumedGasTemperature", "assumedHNet", "fireCompartmentArea",
        "fireCompartmentHeight", "fireOpeningFactor", "fireThermalInertia", "fireOccupancy", "fireLoadBasis", "fireLoadDensityQf",
        "assumedQfD", "floors", "selfWeightElements", "roofs", "windFaces",
        "craneClass", "hoistClass", "hoistingSpeed", "assumedCraneWheel", "assumedCraneHorizontal",
        "siloKind", "siloBulkDensity", "siloHeight", "siloHydraulicRadius", "siloMu", "siloK",
        "assumedSiloPressure", "assumedSiloPatch", "assumedSiloWallFriction", "accidentalCases",
    ];
    let fire_na = [
        "enSk", "enVb", "thermalBridgeType", "mixedTerrainUpwind", "fireCurve", "fireLoadDensityQf",
        "bridgeLane", "bridgeSpan", "bridgeLaneWidth", "bridgeTandemX", "railAlpha", "assumedRailLm71", "assumedRailLm71Udl", "assumedRailSw0", "assumedRailSw2", "craneX", "craneRunwayLength", "craneWheelBase", "siloDiameter", "siloEccentricity", "siloHopperAngleDeg", "bridgeCurveRadius", "assumedBridgeTandem", "assumedBridgeUdl", "assumedBridgeLm2",
        "assumedBridgeFootway", "assumedBridgeLm3", "assumedBridgeLm4", "bridgeLoadGroup",
        "craneClass", "hoistClass", "hoistingSpeed", "assumedCraneWheel", "assumedCraneHorizontal",
        "siloKind", "siloBulkDensity", "siloHeight", "siloHydraulicRadius", "siloMu", "siloK",
        "assumedSiloPressure", "assumedSiloPatch", "assumedSiloWallFriction",
    ];
    let accidental_na = [
        "enSk", "enVb", "thermalBridgeType", "mixedTerrainUpwind",
        "fireCurve", "fireDuration", "assumedGasTemperature", "assumedHNet", "fireCompartmentArea",
        "fireCompartmentHeight", "fireOpeningFactor", "fireThermalInertia", "fireOccupancy", "fireLoadBasis", "fireLoadDensityQf",
        "assumedQfD", "bridgeLane", "bridgeSpan", "bridgeLaneWidth", "bridgeTandemX", "railAlpha", "assumedRailLm71", "assumedRailLm71Udl", "assumedRailSw0", "assumedRailSw2", "craneX", "craneRunwayLength", "craneWheelBase", "siloDiameter", "siloEccentricity", "siloHopperAngleDeg", "bridgeCurveRadius", "assumedBridgeTandem", "assumedBridgeUdl",
        "assumedBridgeLm2", "assumedBridgeFootway", "assumedBridgeLm3", "assumedBridgeLm4", "bridgeLoadGroup",
        "craneClass", "hoistClass", "hoistingSpeed", "assumedCraneWheel", "assumedCraneHorizontal",
        "siloKind", "siloBulkDensity", "siloHeight", "siloHydraulicRadius", "siloMu", "siloK",
        "assumedSiloPressure", "assumedSiloPatch", "assumedSiloWallFriction",
    ];
    const EXEMPT_LEAVES: &[&str] = &["id", "name", "title", "labelEn", "labelDe"];
    fn collect_leaves(v: &serde_json::Value, path: &str, out: &mut Vec<(String, serde_json::Value)>) {
        match v {
            serde_json::Value::Object(map) => {
                for (k, child) in map {
                    let next = if path.is_empty() { k.clone() } else { format!("{path}.{k}") };
                    match child {
                        serde_json::Value::Object(_) | serde_json::Value::Array(_) => collect_leaves(child, &next, out),
                        _ => out.push((next, child.clone())),
                    }
                }
            }
            serde_json::Value::Array(items) => {
                for (i, child) in items.iter().enumerate() {
                    let next = if let Some(id) = child.get("id").and_then(|x| x.as_str()) {
                        format!("{path}[id={id}]")
                    } else {
                        format!("{path}[{i}]")
                    };
                    match child {
                        serde_json::Value::Object(_) | serde_json::Value::Array(_) => collect_leaves(child, &next, out),
                        _ => out.push((next, child.clone())),
                    }
                }
            }
            _ => {}
        }
    }
    fn leaf_name(path: &str) -> &str {
        path.rsplit(['.', '[']).next().unwrap_or(path).trim_end_matches(']')
    }
    fn top_key(path: &str) -> &str {
        path.split(['.', '[']).next().unwrap_or(path)
    }
    fn next_string(leaf: &str, s: &str) -> Option<String> {
        Some(match leaf {
            "annex" => if s == "de" { "en" } else { "de" }.into(),
            "snowZone" => if s == "2" { "3" } else { "2" }.into(),
            "constructionActivity" => if s == "scaffolding" { "formwork" } else { "scaffolding" }.into(),
            "thermalElementType" => if s == "building" { "bridge" } else { "building" }.into(),
            "bridgeLoadGroup" => if s == "gr1a" { "gr1b" } else { "gr1a" }.into(),
            "craneClass" | "hoistClass" => if s == "HC2" { "HC4" } else { "HC2" }.into(),
            "siloKind" => if s == "silo" { "tank" } else { "silo" }.into(),
            "fireOccupancy" => if s == "Office" { "Dwelling" } else { "Office" }.into(),
            "fireLoadBasis" => if s == "occupancyTable" { "userSpecified" } else { "occupancyTable" }.into(),
            "fireCurve" => if s == "parametric" { "standard" } else { "parametric" }.into(),
            "fireMode" => if s == "parametric" { "nominal" } else { "parametric" }.into(),
            "structureKind" => if s == "bridge" { "building" } else { "bridge" }.into(),
            "category" => if s.to_ascii_uppercase().starts_with('B') { "C1" } else { "B1" }.into(),
            "material" => if s == "concrete" || s == "reinforced_concrete" { "steel" } else { "concrete" }.into(),
            "roofType" => if s.contains("multi") { "duopitch" } else { "multispan" }.into(),
            "zone" => match s.to_ascii_uppercase().as_str() {
                "A" => "B".into(),
                "B" => "C".into(),
                "C" => "D".into(),
                "D" => "E".into(),
                "E" => "A".into(),
                "F" => "G".into(),
                "G" => "H".into(),
                _ => "A".into(),
            },
            _ => return None,
        })
    }
    fn next_number(leaf: &str, f: f64, height_hint: f64) -> f64 {
        match leaf {
            "altitude" => f.max(0.0) + 400.0,
            "mixedTerrainDistance" => if f <= 0.0 { 500.0 } else { f * 0.25 },
            "depth" => (height_hint / 5.5).max(0.5),
            "height" => f.max(1.0) * 3.0 + 5.0,
            "width" => f.max(1.0) * 0.2,
            "windZone" | "terrainCategory" | "mixedTerrainUpwind" | "bridgeLane" | "thermalBridgeType" => {
                let i = (f as u8).max(1);
                f64::from(if i >= 4 { 1 } else { i + 1 })
            }
            "storeyCount" => f64::from(if (f as u8) < 2 { 4 } else { 1 }),
            "loadedArea" => if f >= 10.0 - 1e-9 { 5.0 } else if f <= 1.0 + 1e-9 { 5.0 } else { 15.0 },
            _ => if f.abs() < 1e-12 { 1.0 } else { f * 1.37 + 25.0 },
        }
    }
    fn assign_at(root: &mut serde_json::Value, target: &str, current: &str, leaf: &str, height_hint: f64) -> bool {
        match root {
            serde_json::Value::Object(map) => {
                let keys: Vec<String> = map.keys().cloned().collect();
                for k in keys {
                    let next = if current.is_empty() { k.clone() } else { format!("{current}.{k}") };
                    if next == target {
                        let child = map.get_mut(&k).unwrap();
                        return match child {
                            serde_json::Value::Number(n) => {
                                if let Some(f) = n.as_f64() {
                                    *child = serde_json::json!(next_number(leaf, f, height_hint));
                                    true
                                } else {
                                    false
                                }
                            }
                            serde_json::Value::Bool(b) => {
                                *child = serde_json::json!(!*b);
                                true
                            }
                            serde_json::Value::String(s) => {
                                if let Some(n) = next_string(leaf, s) {
                                    *child = serde_json::json!(n);
                                    true
                                } else {
                                    false
                                }
                            }
                            _ => false,
                        };
                    }
                    if target.starts_with(&(next.clone() + ".")) || target.starts_with(&(next.clone() + "[")) {
                        if assign_at(map.get_mut(&k).unwrap(), target, &next, leaf, height_hint) {
                            return true;
                        }
                    }
                }
                false
            }
            serde_json::Value::Array(items) => {
                for (i, child) in items.iter_mut().enumerate() {
                    let next = if let Some(id) = child.get("id").and_then(|x| x.as_str()) {
                        format!("{current}[id={id}]")
                    } else {
                        format!("{current}[{i}]")
                    };
                    if target == next || target.starts_with(&(next.clone() + ".")) || target.starts_with(&(next.clone() + "[")) {
                        if assign_at(child, target, &next, leaf, height_hint) {
                            return true;
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }
    let suites = [
        ("office", de_office_compliant(), &office_na[..]),
        ("bridge", de_bridge_compliant(), &bridge_na[..]),
        ("fire", de_fire_parametric_compliant(), &fire_na[..]),
        ("accidental", de_accidental_variants(), &accidental_na[..]),
    ];
    for (name, base, na) in suites {
        let baseline = evaluate(&base);
        if name == "bridge" { assert_eq!(base.structure_kind, StructureKind::Bridge); }
        if name == "fire" {
            assert_eq!(base.fire_mode, FireMode::Parametric);
        }
        let json = serde_json::to_value(&base).expect("serde snapshot");
        let height_hint = json.get("height").and_then(|x| x.as_f64()).unwrap_or(12.0);
        let mut leaves = Vec::new();
        collect_leaves(&json, "", &mut leaves);
        for (path, _val) in leaves {
            let leaf = leaf_name(&path);
            if EXEMPT_LEAVES.contains(&leaf) {
                continue;
            }
            if na.contains(&top_key(&path)) {
                continue;
            }
            let mut j = json.clone();
            if !assign_at(&mut j, &path, "", leaf, height_hint) {
                continue;
            }
            let Ok(perturbed) = serde_json::from_value::<En1991Snapshot>(j) else { continue };
            let after = evaluate(&perturbed);
            let delta = baseline.checks.len() != after.checks.len()
                || baseline.checks.iter().zip(after.checks.iter()).any(|(a, b)| {
                    a.id != b.id
                        || a.status != b.status
                        || (a.limit.value - b.limit.value).abs() > 1e-6
                        || (a.computed.value - b.computed.value).abs() > 1e-6
                        || (a.utilization - b.utilization).abs() > 1e-6
                });
            assert!(delta, "perturbation of `{path}` on {name} did not change any check (exempt: id/name/title/labelEn/labelDe)");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn duplicate_floor_id_fails_integrity() {
    let mut doc = de_office_compliant();
    let twin = doc.floors[0].clone();
    doc.floors.push(twin);
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.duplicate.floors") && c.status == CheckStatus::Fail));
    assert!(!report.complies());
}

#[semio_framework_async_macros::async_test]
async fn duplicate_wind_face_id_fails_integrity() {
    let mut doc = de_office_compliant();
    assert!(!doc.wind_faces.is_empty());
    let twin = doc.wind_faces[0].clone();
    doc.wind_faces.push(twin);
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.duplicate.windFaces") && c.status == CheckStatus::Fail));
    assert!(!report.complies());
}

#[semio_framework_async_macros::async_test]
async fn duplicate_roof_id_fails_integrity() {
    let mut doc = de_office_compliant();
    assert!(!doc.roofs.is_empty());
    let twin = doc.roofs[0].clone();
    doc.roofs.push(twin);
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.duplicate.roofs") && c.status == CheckStatus::Fail));
    assert!(!report.complies());
}

#[semio_framework_async_macros::async_test]
async fn duplicate_accidental_id_fails_integrity() {
    use semio_s_artifact_norm_en1991_examples::example_subjects::de_accidental_variants;
    let mut doc = de_accidental_variants();
    assert!(!doc.accidental_cases.is_empty());
    let twin = doc.accidental_cases[0].clone();
    doc.accidental_cases.push(twin);
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.duplicate.accidentalCases") && c.status == CheckStatus::Fail));
    assert!(!report.complies());
}

#[semio_framework_async_macros::async_test]
async fn oracle_manifest_kind_count_matches_mutation_enum() {
    use semio_s_artifact_norm_en1991_mutations::KINDS;
    let any = fixture_path!("../..")
        .join("🏅️standards").read_dir().unwrap().next().unwrap().unwrap().path()
        .read_dir().unwrap().map(|e| e.unwrap().path()).find(|p| p.is_dir()).unwrap()
        .read_dir().unwrap().map(|e| e.unwrap().path()).find(|p| p.is_dir()).unwrap();
    let text = std::fs::read_to_string(any.join("🔮️oracles").join("🔣️.json")).unwrap();
    assert!(!text.contains("32 kinds"), "oracle manifest narrative still says 32 kinds");
    let mut found = 0usize;
    for k in KINDS {
        if text.contains(k) {
            found += 1;
        }
    }
    assert_eq!(found, KINDS.len(), "manifest must reference all {} KINDS (found {found})", KINDS.len());
}

/// 🔮️ EN 1991 third-party oracle cases: the shared harness recomputes every blessed case in `🧪️tests/🧫️fixtures/🔮️oracle-cases` with the registered libraries and independent formulations and fails on any mismatch beyond the tolerance profile or between references.
#[test]
fn third_party_libraries_reproduce_the_blessed_oracle_cases() {
    let verified = semio_s_plugin_norm_compliance::python_oracle::verify_cases_above(env!("CARGO_MANIFEST_DIR"));
    assert!(verified >= 6, "expected at least 6 verified oracle cases, got {verified}");
}

#[test]
fn annex_switch_reads_the_registry_and_changes_only_the_expected_values() {
    use semio_s_artifact_norm_en1991_checks::{na_de::WindZone, part_1_1, part_1_3};
    assert_eq!(part_1_1::density_kn_m3("steel", AnnexChoice::En).map(|row| row.2), Some(77.0));
    assert_eq!(part_1_1::density_kn_m3("steel", AnnexChoice::De).map(|row| row.2), Some(78.5));
    assert_eq!(part_1_1::density_kn_m3("concrete", AnnexChoice::En), part_1_1::density_kn_m3("concrete", AnnexChoice::De));
    assert_eq!(part_1_1::partition_allowance_pa(1.0, AnnexChoice::En), Some(500.0));
    assert_eq!(part_1_1::partition_allowance_pa(1.0, AnnexChoice::De), Some(800.0));
    assert_eq!(part_1_3::obstruction_drift(5.0, 500.0, AnnexChoice::En).1, 4.0);
    assert_eq!(part_1_3::obstruction_drift(5.0, 500.0, AnnexChoice::De).1, 2.0);
    assert_eq!(WindZone::Zone2.v_b0_m_s(), 25.0);
}

fn status_of(report: &semio_s_plugin_norm_compliance::CheckReport, id: &str) -> CheckStatus {
    report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("check {id} missing")).status
}

#[semio_framework_async_macros::async_test]
async fn collection_rows_are_checked_with_remedies_and_flag_the_unverified_return_periods() {
    use semio_s_artifact_norm_en1991_model::{BridgeLane, ConstructionStage, RoofSurface, StructureKind, WindFace};
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.structure_kind = StructureKind::Bridge;
    doc.bridge_span = 24.0;
    doc.bridge_lane_width = 6.0;
    doc.assumed_bridge_tandem = 150_000.0;
    doc.assumed_bridge_udl = 9_000.0;
    doc.bridge_lanes = vec![
        BridgeLane { id: "lane-1".into(), width: 3.5, offset: -1.5, lm1_lane: 1 },
        BridgeLane { id: "lane-2".into(), width: 3.0, offset: 1.5, lm1_lane: 2 },
        BridgeLane { id: "lane-3".into(), width: 3.0, offset: 1.0, lm1_lane: 2 },
    ];
    doc.construction_stages = vec![
        ConstructionStage { id: "short".into(), activity: "personnel".into(), duration_days: 30.0, return_years: 1.0, assumed_qk: 500.0 },
        ConstructionStage { id: "long".into(), activity: "personnel".into(), duration_days: 400.0, return_years: 100.0, assumed_qk: 2_000.0 },
    ];
    doc.wind_faces = vec![WindFace { id: "roof-f".into(), zone: "F".into(), z: 9.0, c_pe10: -1.0, c_pe1: -1.5, c_pi: 0.2, c_s: 1.0, c_d: 1.0, loaded_area: 10.0, assumed_wp: 5_000.0 }];
    doc.roof_surfaces = vec![
        RoofSurface { id: "south".into(), face: "roof-f".into(), shape: "duopitch".into(), pitch_deg: 15.0, extent: 8.0 },
        RoofSurface { id: "lost".into(), face: "missing".into(), shape: "duopitch".into(), pitch_deg: 15.0, extent: 8.0 },
    ];
    let report = evaluate(&doc);
    let remedied = |id: &str| report.checks.iter().find(|c| c.id == id).is_some_and(|c| c.status == CheckStatus::Fail && !c.remedies.is_empty());
    assert!(remedied("en1991.2.lane-width.lane-1"));
    assert!(remedied("en1991.2.lm1-lane.lane-1"));
    assert!(remedied("en1991.integrity.bridgeLanes.number.lane-3"));
    assert!(remedied("en1991.integrity.bridgeLanes.overlap.lane-3"));
    assert!(remedied("en1991.integrity.roofSurfaces.lost.face"));
    assert!(remedied("en1991.1-6.stage-load.short"));
    assert_eq!(status_of(&report, "en1991.1-6.stage-return-period.short"), CheckStatus::UnverifiedBasis);
    assert!(find_in(&report, "en1991.1-6.stage-return-period.short").is_some_and(|c| c.remedies.is_empty()), "no remedy while the DE NA table is unset");
    assert_eq!(status_of(&report, "en1991.1-6.stage-return-period.long"), CheckStatus::UnverifiedBasis);
    assert_eq!(status_of(&report, "en1991.1-6.stage-load.long"), CheckStatus::Pass);
    let ratio = semio_s_artifact_norm_en1991_checks::part_1_6::wind_pressure_ratio(2.0);
    assert!((ratio - 0.602_847_663_502_777_2).abs() < 1e-12, "{ratio}");
    assert_eq!(semio_s_artifact_norm_en1991_checks::part_1_6::stage_return_period_years(AnnexChoice::En, 400.0), Some(50.0));
    assert_eq!(semio_s_artifact_norm_en1991_checks::part_1_6::stage_return_period_years(AnnexChoice::De, 400.0), None);
    assert!(report.checks.iter().any(|c| c.id == "en1991.1-3.roof-surface-mu.south"));
    assert!(path_resolves(&doc, "bridgeLanes[id=lane-1].width") && !path_resolves(&doc, "bridgeLanes[id=none].width"));
}

#[semio_framework_async_macros::async_test]
async fn wave_3_collections_are_checked_against_hand_computed_values_with_remedies_and_unverified_basis() {
    use semio_s_artifact_norm_en1991_checks::{part_1_4, part_1_5, part_1_7, part_2};
    let near = |actual: f64, expected: f64| assert!((actual - expected).abs() <= 1e-9 * expected.abs().max(1.0), "{actual} != {expected}");
    near(part_1_4::bridge_c_fx0(12.0, 2.5), 1.3);
    near(part_1_4::bridge_c_fx0(2.0, 4.0), 2.4);
    near(part_1_4::bridge_c_fx0(4.5, 3.0), 2.4 - 1.1 / 3.5);
    near(part_1_4::bridge_depth_m(2.0, 1, 1.1, 0).expect("open parapets"), 2.6);
    near(part_1_4::bridge_depth_m(2.0, 2, 1.1, 1).expect("solid parapets and road traffic"), 4.2);
    near(part_1_7::dynamic_impact_force_n(1500.0, 15.0, 300_000.0), 15.0 * (300_000.0_f64 * 1500.0).sqrt());
    near(part_1_7::gas_explosion_governing_kn_m2(5.0, 0.02), 105.5);
    near(part_1_7::gas_explosion_governing_kn_m2(5.0, 1.0), 8.0);
    assert_eq!(part_2::special_vehicle("3000/200").map(|v| (v.0, v.1, v.2)), Some((3_000_000.0, 200_000.0, 15)));
    assert!(part_2::special_vehicle("3000/175").is_none() && part_2::special_vehicle("1000/150").is_none() && part_2::special_vehicle("tank").is_none());
    assert_eq!(part_1_5::bridge_delta_t_m(1, 2), Some((18.0, 13.0)));
    assert_eq!(part_1_5::bridge_delta_t_m(2, 0), Some((15.0, 18.0)));
    assert!(part_1_5::bridge_delta_t_m(6, 0).is_none() && part_1_5::bridge_delta_t_m(1, 5).is_none());
    near(part_1_5::combined_delta_t(40.0, 18.0), (40.0 + 0.75 * 18.0_f64).max(0.35 * 40.0 + 18.0));
    let doc = wave_3_document();
    let report = evaluate(&doc);
    let find = |id: &str| report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing check {id}"));
    let remedied = |id: &str| find(id).status == CheckStatus::Fail && !find(id).remedies.is_empty();
    for id in ["en1991.2.lm3-vehicle.sv", "en1991.2.lm3-vehicle-length.sv", "en1991.1-4.bridge-wind.wb", "en1991.1-4.dyn-cscd.tower", "en1991.1-7.impact-force.car", "en1991.1-7.gas-explosion.room", "en1991.1-5.section-range.deck", "en1991.input.specialVehicles.bad.designation"] {
        assert!(remedied(id), "{id} must fail with a remedy");
    }
    assert_eq!(find("en1991.1-7.impact-force.ship").status, CheckStatus::NotApplicable);
    assert_ne!(find("en1991.1-4.vortex-shedding.tower").status, CheckStatus::Fail);
    assert!(find("en1991.1-4.vortex-shedding.tower").ndps.iter().any(|n| n == "ndp.en1991.e14-14.1"));
    near(find("en1991.1-5.section-range.deck").utilization, 8.0);
    for check in &report.checks {
        for path in std::iter::once(&check.subject.path).chain(check.remedies.iter().map(|r| &r.target.path)) {
            assert!(!path.contains('[') || path_resolves(&doc, path), "{} emits an unresolved path {path}", check.id);
        }
    }
    for check in report.checks.iter().filter(|c| c.status == CheckStatus::Fail && !c.remedies.is_empty() && c.id.starts_with("en1991.") && !c.id.starts_with("en1991.input.") && !c.id.starts_with("en1991.integrity.")) {
        for remedy in check.remedies.iter().filter(|r| r.applicable && r.target.path.contains("assumed")) {
            let mut fixed = doc.clone();
            apply_si_path(&mut fixed, &remedy.target.path, remedy.required.value);
            assert_ne!(find_in(&evaluate(&fixed), &check.id).map(|c| c.status), Some(CheckStatus::Fail), "writing the remedy of {} must clear the failure", check.id);
        }
    }
}

fn find_in<'a>(report: &'a semio_s_plugin_norm_compliance::CheckReport, id: &str) -> Option<&'a semio_s_plugin_norm_compliance::CheckResult> {
    report.checks.iter().find(|c| c.id == id)
}

fn wave_6_document() -> En1991Snapshot {
    use semio_s_artifact_norm_en1991_model::{ConstructionLoad, ConstructionStage, DerailmentCase, ExecutionImperfection, FallingLoad, HelicopterLanding, TieRun};
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.annex = AnnexChoice::En;
    doc.altitude = 500.0;
    doc.t_max = 30.0;
    doc.t_min = -20.0;
    doc.helicopter_landings = vec![
        HelicopterLanding { id: "heli-low".into(), mass: 3_000.0, x: 5.0, y: 5.0, height: 20.0, assumed_force: 100_000.0 },
        HelicopterLanding { id: "heli-ok".into(), mass: 3_000.0, x: 5.0, y: 12.0, height: 20.0, assumed_force: 200_000.0 },
        HelicopterLanding { id: "heli-bad".into(), mass: 0.0, x: 5.0, y: 20.0, height: 20.0, assumed_force: 1.0 },
    ];
    doc.tie_runs = vec![
        TieRun { id: "tie-p".into(), kind: 0, spacing: 5.0, span: 6.0, dead_load: 5_000.0, imposed_load: 3_000.0, psi: 0.3, x: 0.0, y: 0.0, provided_force: 60_000.0 },
        TieRun { id: "tie-i".into(), kind: 1, spacing: 5.0, span: 6.0, dead_load: 5_000.0, imposed_load: 3_000.0, psi: 0.3, x: 0.0, y: 3.0, provided_force: 80_000.0 },
        TieRun { id: "tie-bad".into(), kind: 2, spacing: 5.0, span: 6.0, dead_load: 5_000.0, imposed_load: 3_000.0, psi: 0.3, x: 0.0, y: 6.0, provided_force: 80_000.0 },
    ];
    doc.construction_stages = vec![ConstructionStage { id: "short".into(), activity: "personnel".into(), duration_days: 30.0, return_years: 2.0, assumed_qk: 500.0 }];
    doc.construction_loads = vec![
        ConstructionLoad { id: "q-ca".into(), stage: "short".into(), kind: 0, magnitude: 1_000.0 },
        ConstructionLoad { id: "q-cb".into(), stage: "short".into(), kind: 1, magnitude: 200.0 },
        ConstructionLoad { id: "q-lost".into(), stage: "missing".into(), kind: 0, magnitude: 100.0 },
    ];
    doc.falling_loads = vec![FallingLoad { id: "drop".into(), mass: 100.0, drop_height: 2.0, stopping_distance: 0.1, x: 2.0, y: 2.0, assumed_force: 10_000.0 }];
    doc.execution_imperfections = vec![ExecutionImperfection { id: "column".into(), height: 4.0, tolerance: 0.02, vertical_load: 1.0e6, x: 1.0, y: 1.0, assumed_force: 1_000.0 }];
    doc.derailment_cases = vec![DerailmentCase { id: "track".into(), distance: 3.0, design_force_x: 1_000_000.0, design_force_y: 300_000.0, x: 0.0, y: -6.0, assumed_force: 800_000.0 }];
    doc
}

#[semio_framework_async_macros::async_test]
async fn wave_6_collections_follow_their_hand_computed_formulas_with_remedies_and_unverified_basis() {
    let doc = wave_6_document();
    let report = evaluate(&doc);
    let find = |id: &str| report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing check {id}"));
    let near = |a: f64, b: f64| assert!((a - b).abs() <= 1e-6 * b.abs().max(1.0), "{a} != {b}");
    near(find("en1991.1-7.helicopter-force.heli-low").computed.value, 3_000.0 * 3_000.0_f64.sqrt());
    near(find("en1991.1-7.tie-force.tie-p").computed.value, 70_800.0);
    near(find("en1991.1-7.tie-force.tie-i").computed.value, 75_000.0);
    near(find("en1991.1-6.stage-combination.short").computed.value, 1_200.0);
    near(find("en1991.1-6.falling-load.drop").computed.value, 100.0 * 9.80665 * 2.1 / 0.1);
    near(find("en1991.1-6.execution-imperfection.column").computed.value, 5_000.0);
    near(find("en1991.1-7.derailment-force.track").computed.value, 1_000_000.0);
    let remedied = |id: &str| find(id).status == CheckStatus::Fail && !find(id).remedies.is_empty();
    for id in ["en1991.1-7.helicopter-force.heli-low", "en1991.1-7.tie-force.tie-p", "en1991.1-6.stage-combination.short", "en1991.1-6.falling-load.drop", "en1991.1-6.execution-imperfection.column", "en1991.1-7.derailment-force.track", "en1991.input.helicopterLandings.heli-bad.mass", "en1991.input.tieRuns.tie-bad.kind", "en1991.input.constructionLoads.q-lost.stage"] {
        assert!(remedied(id), "{id} must fail with a remedy");
    }
    assert_ne!(find("en1991.1-7.helicopter-force.heli-ok").status, CheckStatus::Fail);
    assert_ne!(find("en1991.1-7.tie-force.tie-i").status, CheckStatus::Fail);
    for id in ["en1991.1-7.helicopter-force.heli-ok", "en1991.1-7.tie-force.tie-p", "en1991.1-7.derailment-force.track"] {
        assert!(!find(id).ndps.is_empty(), "{id} cites its unverified registry rows");
    }
    assert!(find("en1991.1-5.isotherm").explanation.en.contains("34.5") && find("en1991.1-5.isotherm").explanation.en.contains("-26.5"));
    for check in report.checks.iter().filter(|c| c.status == CheckStatus::Fail && !c.remedies.is_empty() && !c.id.starts_with("en1991.input.")) {
        for remedy in check.remedies.iter().filter(|r| r.applicable && (r.target.path.contains("assumed") || r.target.path.contains("provided"))) {
            let mut fixed = doc.clone();
            apply_si_path(&mut fixed, &remedy.target.path, remedy.required.value);
            assert_ne!(find_in(&evaluate(&fixed), &check.id).map(|c| c.status), Some(CheckStatus::Fail), "writing the remedy of {} must clear the failure", check.id);
        }
    }
}

fn wave_3_document() -> En1991Snapshot {
    use semio_s_artifact_norm_en1991_model::{DynamicElement, ImpactAction, InternalExplosion, RoofSurface, SpecialVehicle, StructureKind, ThermalSection, WindBridge, WindFace};
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.annex = AnnexChoice::En;
    doc.en_vb = 25.0;
    doc.terrain_category = 2;
    doc.air_density = 1.25;
    doc.orography_factor = 1.0;
    doc.mixed_terrain_distance = 0.0;
    doc.structure_kind = StructureKind::Bridge;
    doc.bridge_span = 12.0;
    doc.bridge_lane_width = 7.0;
    doc.assumed_bridge_lm3 = 100_000.0;
    doc.t_min = -24.0;
    doc.t_max = 37.0;
    doc.t_0 = 10.0;
    doc.special_vehicles = vec![SpecialVehicle { id: "sv".into(), designation: "3000/200".into(), x: 6.0, y: 1.0 }, SpecialVehicle { id: "bad".into(), designation: "tank".into(), x: 6.0, y: 3.5 }];
    doc.wind_bridges = vec![WindBridge { id: "wb".into(), deck_width: 12.0, deck_depth: 2.0, parapet_kind: 1, parapet_height: 1.1, traffic_kind: 0, height_above_ground: 20.0, span: 40.0, assumed_force: 1_000.0 }];
    doc.dynamic_elements = vec![DynamicElement { id: "tower".into(), height: 80.0, width: 4.0, first_frequency: 0.0, log_decrement: 0.05, mass_per_length: 500.0, strouhal: 0.18, assumed_cscd: 0.5 }];
    doc.impact_actions = vec![ImpactAction { id: "car".into(), kind: 0, mass: 1500.0, speed: 15.0, stiffness: 0.0, height: 1.0, x: 0.0, y: 0.0, assumed_force: 100_000.0 }, ImpactAction { id: "ship".into(), kind: 1, mass: 1.0e6, speed: 3.0, stiffness: 0.0, height: 2.0, x: 0.0, y: 5.0, assumed_force: 1.0 }];
    doc.internal_explosions = vec![InternalExplosion { id: "room".into(), volume: 100.0, vent_area: 2.0, vent_pressure: 5_000.0, assumed_pressure: 5_000.0 }];
    doc.thermal_sections = vec![ThermalSection { id: "deck".into(), deck_type: 1, surfacing: 2, length: 60.0, depth: 1.5, alpha_micro: 12.0, assumed_range: 10.0 }];
    doc.en_sk = 1_000.0;
    doc.wind_faces = vec![WindFace { id: "roof-f".into(), zone: "F".into(), z: 9.0, c_pe10: -1.0, c_pe1: -1.5, c_pi: 0.2, c_s: 1.0, c_d: 1.0, loaded_area: 10.0, assumed_wp: 5_000.0 }];
    doc.roof_surfaces = vec![RoofSurface { id: "upper".into(), face: "roof-f".into(), shape: "monopitch".into(), pitch_deg: 20.0, extent: 6.0 }, RoofSurface { id: "lower".into(), face: "roof-f".into(), shape: "flat".into(), pitch_deg: 0.0, extent: 8.0 }];
    doc
}

fn claims_document(tank: bool) -> En1991Snapshot {
    use semio_s_artifact_norm_en1991_model::{BridgeLane, StructureKind};
    let mut doc = En1991Snapshot::example(AnnexChoice::En);
    doc.structure_kind = StructureKind::Bridge;
    doc.bridge_span = 24.0;
    doc.bridge_lane_width = 6.0;
    doc.bridge_lanes = vec![BridgeLane { id: "lane-1".into(), width: 3.0, offset: -1.5, lm1_lane: 1 }, BridgeLane { id: "lane-2".into(), width: 3.0, offset: 1.5, lm1_lane: 2 }];
    doc.crane_claimed = true;
    doc.silo_claimed = true;
    if tank {
        doc.silo_kind = "tank".into();
    }
    doc
}

#[semio_framework_async_macros::async_test]
async fn every_spec_id_is_traced_to_a_check_the_documents_or_design_cases_emit() {
    let mut lowland = de_office_compliant();
    lowland.north_german_lowland_snow = true;
    let documents = [claims_document(false), claims_document(true), En1991Snapshot::example(AnnexChoice::De), de_office_compliant(), lowland, semio_s_artifact_norm_en1991_examples::example_subjects::de_bridge_compliant(), semio_s_artifact_norm_en1991_examples::example_subjects::de_fire_parametric_compliant(), semio_s_artifact_norm_en1991_examples::example_subjects::de_accidental_variants(), multi_fail_noncompliant(), wave_3_document(), wave_6_document()];
    let emitted = documents
        .iter()
        .fold(semio_s_plugin_norm_compliance::spec_trace::Emitted::default(), |emitted, doc| emitted.with_checks(evaluate(doc).checks.into_iter().map(|c| c.id)))
        .with_case_catalogue::<semio_s_artifact_norm_en1991_checks::catalogue::En1991Cases>(semio_s_plugin_norm_compliance::AnnexChoice::De);
    semio_s_plugin_norm_compliance::spec_trace::assert_traced(&semio_s_artifact_norm_en1991_checks::spec_trace::TABLE, &emitted);
}

#[semio_framework_async_macros::async_test]
async fn drift_mu_w_reading_floor_wins_by_default_and_gamma_h_caps_on_the_alternative_reading() {
    use semio_s_artifact_norm_en1991_checks::part_1_3::{drift_mu_w, drift_mu_w_reading};
    assert!((drift_mu_w_reading(0.3, 1.0, 1.0, 1500.0, 4.0, 0.0) - 0.8).abs() < 1e-12);
    assert!((drift_mu_w_reading(0.3, 1.0, 1.0, 1500.0, 4.0, 1.0) - 0.4).abs() < 1e-12);
    assert!((drift_mu_w_reading(1.5, 8.0, 12.0, 1100.0, 4.0, 0.0) - drift_mu_w_reading(1.5, 8.0, 12.0, 1100.0, 4.0, 1.0)).abs() < 1e-12);
    assert!((drift_mu_w(0.3, 1.0, 1.0, 1500.0, AnnexChoice::En) - 0.8).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn hipped_roof_zones_report_awaiting_values_while_table_7_5_is_unset() {
    use semio_s_artifact_norm_en1991_model::{RoofArea, WindFace};
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    for zone in ["F", "G", "H", "I", "J", "K", "L", "M", "N"] {
        for kind in ["cpe10", "cpe1"] {
            let id = semio_s_artifact_norm_en1991_checks::part_1_4::hipped_cpe_ndp_id(kind, zone);
            assert!(semio_s_plugin_norm_compliance::ndp(&id).is_some(), "{id} is registered");
            assert_eq!(semio_s_plugin_norm_compliance::ndp_number_opt(&id, En), None, "{id} stays unset until the table is supplied");
        }
    }
    let roof = RoofArea { id: "roof-hip".into(), roof_type: "hipped".into(), pitch_deg: 25.0, c_e: 1.0, c_t: 1.0, has_parapet: false, parapet_height: 0.0, drift_obstruction_height: 0.0, multi_span: false, assumed_sk: 1_000.0 };
    let face = |id: &str, zone: &str| WindFace { id: id.into(), zone: zone.into(), z: 8.0, c_pe10: -1.0, c_pe1: -1.5, c_pi: 0.2, c_s: 1.0, c_d: 1.0, loaded_area: 10.0, assumed_wp: 5_000.0 };
    let doc = En1991Snapshot { annex: En, roofs: vec![roof], wind_faces: vec![face("hip-f", "F"), face("hip-k", "K"), face("wall-d", "D")], ..En1991Snapshot::example(AnnexChoice::De) };
    let report = evaluate(&doc);
    for (id, zone) in [("hip-f", "F"), ("hip-k", "K")] {
        for kind in ["cpe10", "cpe1"] {
            let check = report.checks.iter().find(|c| c.id == format!("en1991.1-4.{kind}.{id}")).unwrap_or_else(|| panic!("missing {kind} of {id}"));
            assert_eq!(check.status, CheckStatus::UnverifiedBasis, "{id} {kind}: awaiting values is a visible unverified basis, never a silent pass");
            assert!(check.explanation.en.starts_with("Awaiting values") && check.explanation.de.starts_with("Werte ausstehend"), "{:?}", check.explanation);
            assert!(check.ndps.iter().any(|ndp| ndp.ends_with(&format!("hipped-{kind}.{}", zone.to_lowercase()))));
        }
    }
    assert_ne!(report.checks.iter().find(|c| c.id == "en1991.1-4.cpe10.wall-d").expect("wall check").status, CheckStatus::UnverifiedBasis, "wall zones keep their tabulated coefficients");
    assert!(report.checks.iter().any(|c| c.id == "en1991.1-4.wind.hip-f"), "the pressure check still runs on the entered coefficients");
}

#[semio_framework_async_macros::async_test]
async fn bridge_braking_centrifugal_and_thermal_combination_checks_follow_the_registry_formulas() {
    use semio_s_artifact_norm_en1991_checks::part_2;
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.structure_kind = semio_s_artifact_norm_en1991_model::StructureKind::Bridge;
    doc.bridge_span = 30.0;
    doc.bridge_curve_radius = 400.0;
    doc.delta_t_m = 8.0;
    doc.thermal_element_type = "bridge".to_owned();
    doc.thermal_bridge_type = 1;
    let (lanes, w_1, _) = part_2::lane_layout(doc.bridge_lane_width, &[]);
    let report = check_full_actions(&doc);
    let find = |id: &str| report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("check {id}"));
    let braking = find("en1991.2.braking");
    let expected = (0.6 * 2.0 * 300.0 + 0.10 * 9.0 * w_1 * 30.0).clamp(180.0, 900.0) * 1000.0;
    assert!((braking.computed.value - expected).abs() < 1e-6, "Q_lk = 0.6 (2 Q_1k) + 0.1 q_1k w_1 L: {}", braking.computed.value);
    assert!((braking.limit.value - 900_000.0).abs() < 1e-9 && braking.utilization <= 1.0);
    let centrifugal = find("en1991.2.centrifugal");
    let q_v = 2.0 * 300.0 * (1..=lanes.min(3)).map(|l| [1.0, 2.0 / 3.0, 1.0 / 3.0][usize::from(l) - 1]).sum::<f64>();
    assert!((centrifugal.computed.value - 1000.0 * 40.0 * q_v / 400.0).abs() < 1e-6, "Q_tk = 40 Q_v / r with Q_v = {q_v} kN: {}", centrifugal.computed.value);
    let combination = find("en1991.1-5.combination");
    assert!(combination.explanation.en.contains("ψ_0/ψ_1/ψ_2 = 0.60/0.60/0.50") || combination.explanation.en.contains("ψ_0/ψ_1/ψ_2 = 0.80/0.60/0.50"), "{}", combination.explanation.en);
    assert!(combination.ndps.iter().any(|id| id.starts_with("ndp.en1990.psi.temperature_bridge")), "the psi rows are cited from the shared registry");
    doc.bridge_curve_radius = 0.0;
    assert_eq!(check_full_actions(&doc).checks.iter().find(|c| c.id == "en1991.2.centrifugal").map(|c| c.status), Some(CheckStatus::NotApplicable));
    doc.bridge_curve_radius = -5.0;
    assert!(check_full_actions(&doc).checks.iter().any(|c| c.id == "en1991.input.bridgeCurveRadius"));
}

#[semio_framework_async_macros::async_test]
async fn crane_runway_silo_class_and_discharge_checks_follow_the_registry_formulas() {
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.crane_claimed = true;
    doc.silo_claimed = true;
    doc.silo_kind = "silo".into();
    let report = evaluate(&doc);
    let find = |id: &str| report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("check {id} missing"));
    let runway = find("en1991.3.crane-runway");
    assert!(runway.explanation.en.contains("R_max") && runway.explanation.en.contains("M_max"), "{}", runway.explanation.en);
    assert!(find("en1991.4.silo-class").explanation.en.contains("action assessment class"));
    let required_h = 1.15 * semio_s_artifact_norm_en1991_checks::part_4::horizontal_max_pa(doc.silo_bulk_density, doc.silo_hydraulic_radius, doc.silo_mu, doc.silo_k, doc.silo_height);
    assert!(find("en1991.4.silo-discharge").explanation.en.contains(&format!("{:.1} kN/m²", required_h / 1000.0)));
    doc.assumed_silo_pressure = 1.0;
    let failing = evaluate(&doc);
    assert_eq!(failing.checks.iter().find(|c| c.id == "en1991.4.silo-discharge").map(|c| c.status), Some(CheckStatus::Fail));
}

#[semio_framework_async_macros::async_test]
async fn crane_hoist_rows_report_the_release_factor_the_test_loads_and_reject_unknown_devices() {
    use semio_s_artifact_norm_en1991_model::CraneHoist;
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.crane_claimed = true;
    doc.crane_hoists.push(CraneHoist { id: "hoist-1".into(), rated_load: 100_000.0, release_fraction: 0.4, release_device: "magnet".into() });
    doc.crane_hoists.push(CraneHoist { id: "hoist-2".into(), rated_load: 50_000.0, release_fraction: 0.5, release_device: "grab".into() });
    let report = evaluate(&doc);
    let find = |id: &str| report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("check {id} missing"));
    let phi = find("en1991.3.hoist-release.hoist-1");
    assert!(phi.explanation.en.contains("= 0.200"), "phi_3 = 1 - 0.4 (1 + 1.0) = 0.2: {}", phi.explanation.en);
    assert!(find("en1991.3.hoist-release.hoist-2").explanation.en.contains("= 0.250"), "phi_3 = 1 - 0.5 (1 + 0.5) = 0.25");
    assert!((find("en1991.3.hoist-test-static.hoist-1").computed.value - 110_000.0).abs() < 1e-6);
    assert!((find("en1991.3.hoist-test-dynamic.hoist-1").computed.value - 125_000.0).abs() < 1e-6);
    assert!((find("en1991.3.hoist-test-static.hoist-2").computed.value - 55_000.0).abs() < 1e-6);
    assert!(find("en1991.3.hoist-test-static.hoist-1").subject.entity_id == "hoist-1", "the hoist row owns its checks");
    doc.crane_hoists[0].release_device = "claw".into();
    doc.crane_hoists[1].release_fraction = 1.5;
    let invalid = evaluate(&doc);
    assert!(invalid.checks.iter().any(|c| c.id == "en1991.input.craneHoists.hoist-1.releaseDevice"));
    assert!(invalid.checks.iter().any(|c| c.id == "en1991.input.craneHoists.hoist-2.releaseFraction"));
    assert!(invalid.checks.iter().all(|c| c.id != "en1991.3.hoist-release.hoist-1" && c.id != "en1991.3.hoist-release.hoist-2"));
}

#[semio_framework_async_macros::async_test]
async fn the_v12_rows_report_hand_computed_footbridge_rail_dispersal_bulk_and_machine_values() {
    use semio_s_artifact_norm_en1991_model::{BulkSolid, Footbridge, LoadDispersal, RailDynamic, RotatingMachine};
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.footbridges.push(Footbridge { id: "fb-1".into(), loaded_length: 30.0, deck_width: 3.0, natural_frequency: 1.9, assumed_service_axle: 0.0 });
    doc.footbridges.push(Footbridge { id: "fb-2".into(), loaded_length: 1000.0, deck_width: 4.0, natural_frequency: 3.5, assumed_service_axle: 0.0 });
    doc.footbridges.push(Footbridge { id: "fb-3".into(), loaded_length: 30.0, deck_width: 3.0, natural_frequency: 6.0, assumed_service_axle: 0.0 });
    doc.rail_dynamics.push(RailDynamic { id: "rail-1".into(), determinant_length: 4.0, maintenance: "careful".into() });
    doc.rail_dynamics.push(RailDynamic { id: "rail-2".into(), determinant_length: 4.0, maintenance: "standard".into() });
    doc.load_dispersals.push(LoadDispersal { id: "disp-1".into(), wheel_length: 0.4, wheel_width: 0.4, pavement_thickness: 0.1, slab_thickness: 0.3 });
    doc.bulk_solids.push(BulkSolid { id: "bulk-1".into(), internal_friction: 30.0_f64.to_radians() });
    doc.rotating_machines.push(RotatingMachine { id: "machine-1".into(), mass: 1000.0, eccentricity: 0.002, frequency: 25.0 });
    let report = evaluate(&doc);
    let find = |id: &str| report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("check {id} missing"));
    assert!((find("en1991.2.footbridge-load.fb-1").computed.value - 4000.0).abs() < 1e-6, "q_fk = 2.0 + 120/60 = 4.0 kN/m2");
    assert!((find("en1991.2.footbridge-horizontal.fb-1").computed.value - 36_000.0).abs() < 1e-6, "Q_flk = 0.1 · 4000 · 30 · 3 = 36 kN");
    assert!((find("en1991.2.footbridge-load.fb-2").computed.value - 2500.0).abs() < 1e-6, "the lower bound 2.5 kN/m2 governs at L = 1000 m");
    assert!(find("en1991.2.footbridge-walking.fb-1").explanation.en.contains("inside"), "1.9 Hz is inside 1.7 to 2.1 Hz");
    assert!(find("en1991.2.footbridge-walking.fb-2").explanation.en.contains("outside"), "3.5 Hz is outside 1.7 to 2.1 Hz");
    assert!(find("en1991.2.footbridge-walking.fb-1").explanation.en.contains("is required"), "1.9 Hz is below 5 Hz: the comfort verification of EN 1990 A2.4.3.2(1) is triggered");
    assert!(find("en1991.2.footbridge-walking.fb-2").explanation.en.contains("is required"), "3.5 Hz is below 5 Hz: the comfort verification is triggered outside the walking range too");
    let relaxed = &find("en1991.2.footbridge-walking.fb-3").explanation;
    assert!(relaxed.en.contains("triggers no vertical comfort verification") && !relaxed.en.contains("walking range"), "6.0 Hz is not below 5 Hz: no comfort verification, no walking verdict");
    assert!((find("en1991.2.rail-dynamic.rail-1").computed.value - 1.62).abs() < 1e-9, "Phi_2 = 1.44/1.8 + 0.82 = 1.62");
    assert!((find("en1991.2.rail-dynamic.rail-2").computed.value - 1.93).abs() < 1e-9, "Phi_3 = 2.16/1.8 + 0.73 = 1.93");
    assert!((find("en1991.2.dispersal-footprint.disp-1").computed.value - 0.81).abs() < 1e-9, "(0.4 + 2 · 0.25)^2 = 0.81 m2");
    assert!((find("en1991.4.bulk-k-mean.bulk-1").computed.value - 0.55).abs() < 1e-9, "K_m = 1.1 (1 - sin 30°) = 0.55");
    assert!((find("en1991.3.machine-force.machine-1").computed.value - 49_348.022).abs() < 1e-2, "F = 1000 · 0.002 · (2π · 25)^2 = 49.348 kN");
    assert!(find("en1991.3.machine-force.machine-1").subject.entity_id == "machine-1", "the machine row owns its check");
    doc.rail_dynamics[0].maintenance = "poor".into();
    doc.bulk_solids[0].internal_friction = 2.0;
    doc.rotating_machines[0].frequency = 0.0;
    doc.load_dispersals[0].slab_thickness = -0.1;
    doc.footbridges[0].deck_width = 0.0;
    let invalid = evaluate(&doc);
    for id in ["en1991.input.railDynamics.rail-1.maintenance", "en1991.input.bulkSolids.bulk-1.internalFriction", "en1991.input.rotatingMachines.machine-1.frequency", "en1991.input.loadDispersals.disp-1.slabThickness", "en1991.input.footbridges.fb-1.deckWidth"] {
        assert!(invalid.checks.iter().any(|c| c.id == id), "{id} missing");
    }
    assert!(invalid.checks.iter().all(|c| c.id != "en1991.2.rail-dynamic.rail-1" && c.id != "en1991.4.bulk-k-mean.bulk-1" && c.id != "en1991.3.machine-force.machine-1" && c.id != "en1991.2.dispersal-footprint.disp-1" && c.id != "en1991.2.footbridge-load.fb-1"));
}

#[semio_framework_async_macros::async_test]
async fn the_wave_3_rows_report_hand_computed_fatigue_collision_fire_and_snow_guard_values() {
    use semio_s_artifact_norm_en1991_checks::part_1_3;
    use semio_s_artifact_norm_en1991_model::{FatigueModel, LocalFire, SnowGuard, StructureKind, SupportCollision};
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.structure_kind = StructureKind::Bridge;
    doc.bridge_span = 30.0;
    doc.fatigue_models.push(FatigueModel { id: "fl-1".into(), model: "flm1".into(), lane: 1, assumed_axle_load: 210_000.0, assumed_udl: 2_700.0 });
    doc.fatigue_models.push(FatigueModel { id: "fl-2".into(), model: "flm1".into(), lane: 2, assumed_axle_load: 100_000.0, assumed_udl: 750.0 });
    doc.fatigue_models.push(FatigueModel { id: "fl-3".into(), model: "flm3".into(), lane: 1, assumed_axle_load: 120_000.0, assumed_udl: 0.0 });
    doc.support_collisions.push(SupportCollision { id: "co-1".into(), location: "deck".into(), assumed_force_x: 1_000_000.0, assumed_force_y: 500_000.0, height: 1.25, position_x: 10.0 });
    doc.support_collisions.push(SupportCollision { id: "co-2".into(), location: "soffit".into(), assumed_force_x: 400_000.0, assumed_force_y: 0.0, height: 1.25, position_x: 20.0 });
    doc.local_fires.push(LocalFire { id: "lf-1".into(), fire_diameter: 2.0, heat_release_rate: 5.0e6, ceiling_height: 3.0, horizontal_distance: 1.0, assumed_flux: 60_000.0 });
    doc.local_fires.push(LocalFire { id: "lf-2".into(), fire_diameter: 2.0, heat_release_rate: 1.0e6, ceiling_height: 6.0, horizontal_distance: 1.0, assumed_flux: 0.0 });
    doc.roofs[0].pitch_deg = 30.0;
    let roof = doc.roofs[0].id.clone();
    doc.snow_guards.push(SnowGuard { id: "sg-1".into(), roof_id: roof.clone(), slope_length: 6.0, assumed_force: 1.0 });
    let report = evaluate(&doc);
    let find = |id: &str| report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("check {id} missing"));
    let near = |id: &str, expected: f64, tolerance: f64| assert!((find(id).computed.value - expected).abs() <= tolerance, "{id}: {} against {expected}", find(id).computed.value);
    near("en1991.2.fatigue-axle.fl-1", 210_000.0, 1e-6);
    near("en1991.2.fatigue-udl.fl-1", 2_700.0, 1e-6);
    near("en1991.2.fatigue-axle.fl-2", 140_000.0, 1e-6);
    near("en1991.2.fatigue-udl.fl-2", 750.0, 1e-6);
    near("en1991.2.fatigue-axle.fl-3", 120_000.0, 1e-6);
    near("en1991.2.fatigue-vehicle.fl-3", 8.4, 1e-9);
    assert_ne!(find("en1991.2.fatigue-axle.fl-1").status, CheckStatus::Fail);
    assert!(find("en1991.2.fatigue-axle.fl-2").status == CheckStatus::Fail && !find("en1991.2.fatigue-axle.fl-2").remedies.is_empty(), "100 kN does not cover 0.7 · 200 kN");
    assert!(find("en1991.2.fatigue-axle.fl-1").ndps.iter().any(|id| id == "ndp.en1991.2.fatigue-flm1-q-factor"), "the unverified factor is cited");
    near("en1991.2.collision-x.co-1", 1_000_000.0, 1e-6);
    near("en1991.2.collision-y.co-1", 500_000.0, 1e-6);
    near("en1991.2.collision-x.co-2", 500_000.0, 1e-6);
    assert!(find("en1991.2.collision-x.co-2").status == CheckStatus::Fail && !find("en1991.2.collision-x.co-2").remedies.is_empty());
    assert!(report.checks.iter().all(|c| c.id != "en1991.2.collision-y.co-2"), "the soffit has no transverse force");
    near("en1991.1-2.local-flame.lf-1", 5.037008498, 1e-6);
    near("en1991.1-2.local-flux.lf-1", 50_907.1348, 1e-3);
    assert_ne!(find("en1991.1-2.local-flux.lf-1").status, CheckStatus::Fail, "60 kW/m² covers 50.9 kW/m²");
    near("en1991.1-2.local-plume.lf-2", 383.0952891508, 1e-6);
    assert!(report.checks.iter().all(|c| c.id != "en1991.1-2.local-flux.lf-2"), "the flame of 1 MW does not reach a ceiling at 6 m");
    let sk = part_1_3::design_ground_snow_pa(AnnexChoice::De, &doc.snow_zone, doc.altitude, doc.en_sk).expect("snow zone");
    let s = 0.8 * sk * doc.roofs[0].c_e * doc.roofs[0].c_t;
    near("en1991.1-3.snow-guard.sg-1", s * 6.0 * 30.0_f64.to_radians().sin(), 1e-6);
    assert!(find("en1991.1-3.snow-guard.sg-1").status == CheckStatus::Fail && !find("en1991.1-3.snow-guard.sg-1").remedies.is_empty());
    for check in &report.checks {
        for path in std::iter::once(&check.subject.path).chain(check.remedies.iter().map(|r| &r.target.path)) {
            assert!(!path.contains('[') || path_resolves(&doc, path), "{} emits an unresolved path {path}", check.id);
        }
    }
    doc.fatigue_models[0].model = "flm2".into();
    doc.fatigue_models[1].lane = 4;
    doc.support_collisions[0].position_x = 31.0;
    doc.support_collisions[1].location = "pier".into();
    doc.local_fires[0].fire_diameter = 0.0;
    doc.snow_guards[0].roof_id = "none".into();
    let invalid = evaluate(&doc);
    for id in ["en1991.input.fatigueModels.fl-1.model", "en1991.input.fatigueModels.fl-2.lane", "en1991.input.supportCollisions.co-1.positionX", "en1991.input.supportCollisions.co-2.location", "en1991.input.localFires.lf-1.fireDiameter", "en1991.input.snowGuards.sg-1.roofId"] {
        assert!(invalid.checks.iter().any(|c| c.id == id && c.status == CheckStatus::Fail && !c.remedies.is_empty()), "{id} missing or without remedy");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_rail_derailment_load_group_service_vehicle_and_dust_rows_report_hand_computed_values() {
    use semio_s_artifact_norm_en1991_model::{DustExplosion, Footbridge, RailDerailment, RailLoadGroup, StructureKind};
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.structure_kind = StructureKind::Bridge;
    doc.bridge_span = 30.0;
    doc.rail_alpha = 1.21;
    doc.rail_derailments.push(RailDerailment { id: "dr-1".into(), situation: "i".into(), assumed_line_load: 50_000.0, loaded_length: 20.0, offset: 1.0 });
    doc.rail_derailments.push(RailDerailment { id: "dr-2".into(), situation: "ii".into(), assumed_line_load: 100_000.0, loaded_length: 15.0, offset: 3.0 });
    doc.rail_load_groups.push(RailLoadGroup { id: "g-1".into(), load_group: "gr11".into(), factor_vertical: 1.0, factor_longitudinal: 0.5, factor_lateral: 1.0, assumed_axle_load: 302_500.0, assumed_udl: 96_800.0, assumed_longitudinal: 495_000.0, assumed_lateral: 121_000.0 });
    doc.rail_load_groups.push(RailLoadGroup { id: "g-2".into(), load_group: "gr13".into(), factor_vertical: 0.8, factor_longitudinal: 1.0, factor_lateral: 0.5, assumed_axle_load: 100_000.0, assumed_udl: 50_000.0, assumed_longitudinal: 400_000.0, assumed_lateral: 10_000.0 });
    doc.footbridges.push(Footbridge { id: "fb-s".into(), loaded_length: 20.0, deck_width: 3.0, natural_frequency: 0.0, assumed_service_axle: 50_000.0 });
    doc.footbridges.push(Footbridge { id: "fb-0".into(), loaded_length: 20.0, deck_width: 3.0, natural_frequency: 0.0, assumed_service_axle: 0.0 });
    doc.dust_explosions.push(DustExplosion { id: "de-1".into(), kst: 150.0, p_max: 800_000.0, volume: 100.0, vent_area: 8.0, static_opening_pressure: 20_000.0, design_pressure: 50_000.0 });
    doc.dust_explosions.push(DustExplosion { id: "de-2".into(), kst: 150.0, p_max: 800_000.0, volume: 100.0, vent_area: 2.0, static_opening_pressure: 20_000.0, design_pressure: 50_000.0 });
    let report = evaluate(&doc);
    let find = |id: &str| report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("check {id} missing"));
    let near = |id: &str, expected: f64, tolerance: f64| assert!((find(id).computed.value - expected).abs() <= tolerance, "{id}: {} against {expected}", find(id).computed.value);
    near("en1991.2.derail-line-load.dr-1", 45_000.0, 1e-6);
    near("en1991.2.derail-line-load.dr-2", 1.4 * 1.21 * 80_000.0, 1e-6);
    near("en1991.2.derail-length.dr-1", 20.0, 1e-9);
    near("en1991.2.derail-offset.dr-1", 1.0, 1e-9);
    assert_ne!(find("en1991.2.derail-line-load.dr-1").status, CheckStatus::Fail);
    for id in ["en1991.2.derail-line-load.dr-2", "en1991.2.derail-length.dr-2", "en1991.2.derail-offset.dr-2"] {
        assert!(find(id).status == CheckStatus::Fail && !find(id).remedies.is_empty(), "{id} must fail with a remedy");
    }
    assert!(find("en1991.2.derail-line-load.dr-1").ndps.iter().any(|id| id == "ndp.en1991.2.derail-i-line-load"), "the unverified line load is cited");
    near("en1991.2.rail-group-axle.g-1", 302_500.0, 1e-6);
    near("en1991.2.rail-group-udl.g-1", 96_800.0, 1e-6);
    near("en1991.2.rail-group-longitudinal.g-1", 495_000.0, 1e-6);
    near("en1991.2.rail-group-lateral.g-1", 121_000.0, 1e-6);
    near("en1991.2.rail-group-axle.g-2", 0.8 * 302_500.0, 1e-6);
    assert_ne!(find("en1991.2.rail-group-axle.g-1").status, CheckStatus::Fail);
    for id in ["en1991.2.rail-group-axle.g-2", "en1991.2.rail-group-udl.g-2", "en1991.2.rail-group-longitudinal.g-2", "en1991.2.rail-group-lateral.g-2"] {
        assert!(find(id).status == CheckStatus::Fail && !find(id).remedies.is_empty(), "{id} must fail with a remedy");
    }
    near("en1991.2.footbridge-service.fb-s", 40_000.0, 1e-9);
    assert_ne!(find("en1991.2.footbridge-service.fb-s").status, CheckStatus::Fail);
    assert!(report.checks.iter().all(|c| c.id != "en1991.2.footbridge-service.fb-0"), "no service vehicle given: no check, no assumption");
    near("en1991.4.dust-vent-area.de-1", 3.087294152503173, 1e-9);
    near("en1991.4.dust-vent-area.de-2", 3.087294152503173, 1e-9);
    near("en1991.4.dust-pressure.de-1", 0.0625, 1e-12);
    assert_ne!(find("en1991.4.dust-vent-area.de-1").status, CheckStatus::Fail);
    assert!(find("en1991.4.dust-vent-area.de-2").status == CheckStatus::Fail && !find("en1991.4.dust-vent-area.de-2").remedies.is_empty(), "2 m² do not cover 3.09 m²");
    for check in &report.checks {
        for path in std::iter::once(&check.subject.path).chain(check.remedies.iter().map(|r| &r.target.path)) {
            assert!(!path.contains('[') || path_resolves(&doc, path), "{} emits an unresolved path {path}", check.id);
        }
    }
    doc.rail_derailments[0].situation = "iii".into();
    doc.rail_load_groups[0].load_group = "gr99".into();
    doc.rail_load_groups[1].factor_lateral = 1.5;
    doc.dust_explosions[0].kst = 350.0;
    doc.dust_explosions[1].vent_area = 0.0;
    doc.footbridges[0].assumed_service_axle = -1.0;
    let invalid = evaluate(&doc);
    for id in ["en1991.input.railDerailments.dr-1.situation", "en1991.input.railLoadGroups.g-1.loadGroup", "en1991.input.railLoadGroups.g-2.factorLateral", "en1991.input.dustExplosions.de-1.kst", "en1991.input.dustExplosions.de-2.ventArea", "en1991.input.footbridges.fb-s.assumedServiceAxle"] {
        assert!(invalid.checks.iter().any(|c| c.id == id && c.status == CheckStatus::Fail && !c.remedies.is_empty()), "{id} missing or without remedy");
    }
}

#[semio_framework_async_macros::async_test]
async fn execution_stage_return_periods_follow_the_en_table_and_await_the_de_na() {
    use semio_s_artifact_norm_en1991_checks::part_1_6::stage_return_period_years as years;
    use semio_s_artifact_norm_en1991_model::ConstructionStage;
    let hand = [(3.0, 2.0), (3.5, 5.0), (30.0, 5.0), (90.0, 5.0), (91.0, 10.0), (200.0, 10.0), (365.0, 10.0), (366.0, 50.0), (400.0, 50.0)];
    for (days, expected) in hand {
        assert_eq!(years(AnnexChoice::En, days), Some(expected), "{days} d");
        assert_eq!(years(AnnexChoice::De, days), None, "{days} d awaits the DE NA");
    }
    let stage = |id: &str, days: f64, t: f64| ConstructionStage { id: id.into(), activity: "personnel".into(), duration_days: days, return_years: t, assumed_qk: 1_500.0 };
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.annex = AnnexChoice::En;
    doc.construction_stages = vec![stage("short", 30.0, 2.0), stage("mid", 200.0, 5.0), stage("brief", 2.0, 4.0), stage("long", 400.0, 100.0)];
    let report = evaluate(&doc);
    let check = |id: &str| find_in(&report, &format!("en1991.1-6.stage-return-period.{id}")).expect("check");
    for (id, required) in [("short", 5.0), ("mid", 10.0)] {
        let c = check(id);
        assert_eq!(c.status, CheckStatus::Fail, "{id}");
        assert!(c.remedies.iter().any(|r| r.applicable), "{id} remedy");
        assert!((c.computed.value - required).abs() < 1e-9, "{id}: {}", c.computed.value);
    }
    assert_eq!(check("brief").status, CheckStatus::UnverifiedBasis);
    assert_eq!(check("long").status, CheckStatus::UnverifiedBasis);
    doc.annex = AnnexChoice::De;
    let report = evaluate(&doc);
    for id in ["short", "mid", "brief", "long"] {
        let c = find_in(&report, &format!("en1991.1-6.stage-return-period.{id}")).expect("check");
        assert_eq!(c.status, CheckStatus::UnverifiedBasis, "{id}");
        assert!(c.remedies.is_empty() && c.ndps.iter().any(|n| n.starts_with("ndp.en1991.1-6.stage-return.")), "{id} awaits the DE NA table");
    }
}

#[semio_framework_async_macros::async_test]
async fn braking_force_uses_the_width_of_lane_one_not_the_mean_of_the_lanes() {
    use semio_s_artifact_norm_en1991_checks::part_2;
    use semio_s_artifact_norm_en1991_model::{BridgeLane, StructureKind};
    let lanes = [(2, 3.0), (1, 3.5), (2, 3.0)];
    assert_eq!(part_2::lane_one_width_m(7.0, &lanes), 3.5);
    assert_eq!(part_2::lane_one_width_m(7.0, &[(2, 3.0)]), 3.0);
    assert_eq!(part_2::lane_one_width_m(7.0, &[]), 3.0);
    let hand_kn = 0.6 * 2.0 * 300.0 + 0.10 * 9.0 * 3.5 * 30.0;
    assert!((part_2::braking_force_of_deck_kn(AnnexChoice::En, 30.0, 10.0, &lanes) - hand_kn).abs() < 1e-9, "{hand_kn}");
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.annex = AnnexChoice::En;
    doc.structure_kind = StructureKind::Bridge;
    doc.bridge_span = 30.0;
    doc.bridge_lane_width = 10.0;
    doc.bridge_lanes = lanes.iter().enumerate().map(|(index, (number, width))| BridgeLane { id: format!("lane-{index}"), width: *width, offset: 3.0 * index as f64, lm1_lane: *number }).collect();
    let report = check_full_actions(&doc);
    let braking = find_in(&report, "en1991.2.braking").expect("braking");
    assert!((braking.computed.value - hand_kn * 1000.0).abs() < 1e-6, "Q_lk with w_1 = 3.5 m: {}", braking.computed.value);
}
