use crate::document::{AnnexChoice, CheckStatus};
use crate::En1991Snapshot;
use crate::artifact_schema::inferences::{evaluate, check_full_actions};
use crate::example_subjects::{de_office_compliant, multi_fail_noncompliant};
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn default_subject_complies_de_na() {
    let report = evaluate(&En1991Snapshot::default());
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
    assert!(same.status != CheckStatus::Fail || same.utilization <= 1.0 + 1e-9, "remedy did not fix {}: status={:?} util={}", fail.id, same.status, same.utilization);
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
        assert!(same.status != CheckStatus::Fail || same.utilization <= 1.0 + 1e-9, "remedy did not fix {id}: {:?}", same.status);
    }
}

#[semio_framework_async_macros::async_test]
async fn emitted_entity_paths_use_id_selectors_and_resolve() {
    let doc = En1991Snapshot::default();
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
    !path.contains('[')
}


#[semio_framework_async_macros::async_test]
async fn snow_zone2_150m_roof_mu08() {
    let sk = crate::standards::v1::subsets::any::schema::part_1_3::ground_snow_pa("2", 150.0);
    assert!((sk - 850.0).abs() < 1e-6);
    let s = sk * 0.8;
    assert!((s - 680.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn imposed_category_b1_de() {
    let q = crate::standards::v1::subsets::any::schema::part_1_1::imposed_qk_pa("B1", AnnexChoice::De);
    assert!((q - 2000.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn bridge_crane_silo_claim_gates() {
    let mut doc = En1991Snapshot::default();
    doc.structure_kind = crate::StructureKind::Building;
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
    if let Some(rest) = path.strip_prefix("accidentalCases[id=") {
        let (id, field) = rest.split_once("].").expect("acc path");
        if let Some(c) = doc.accidental_cases.iter_mut().find(|c| c.id == id) {
            if field == "assumedForce" || field.ends_with("assumedForce") {
                if let Some(imp) = c.impact.first_mut() { imp.assumed_force = value; return; }
            }
            if field == "assumedPressure" || field.ends_with("assumedPressure") {
                if let Some(ex) = c.explosion.first_mut() { ex.assumed_pressure = value; return; }
            }
            // Nested path impact[0].assumedForce / explosion[0].assumedPressure
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
    let mut doc = En1991Snapshot::default();
    doc.structure_kind = crate::StructureKind::Bridge;
    doc.assumed_bridge_udl = 3600.0;
    doc.assumed_bridge_tandem = 270_000.0;
    doc.assumed_bridge_lm2 = 360_000.0;
    doc.assumed_bridge_lm3 = 600_000.0;
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
    let mut doc = En1991Snapshot::default();
    doc.crane_claimed = false;
    doc.silo_claimed = false;
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("crane") && c.status == CheckStatus::NotApplicable));
    assert!(report.checks.iter().any(|c| c.id.contains("silo") && c.status == CheckStatus::NotApplicable));
}

#[semio_framework_async_macros::async_test]
async fn regenerate_example_assets_once() {
    use crate::example_subjects::{de_office_compliant, multi_fail_noncompliant};
    use crate::standards::v1::subsets::any::schema::snapshot::{encode_en1991_dsl, encode_en1991_pack};
    use std::fs;
    use std::path::PathBuf;
    if std::env::var("EN1991_REGEN_ASSETS").ok().as_deref() != Some("1") { return; }
    let fam = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let assets = fam.join("🏅️standards").read_dir().unwrap().next().unwrap().unwrap().path()
        .read_dir().unwrap().map(|e| e.unwrap().path()).find(|p| p.is_dir()).unwrap()
        .read_dir().unwrap().map(|e| e.unwrap().path()).find(|p| p.is_dir()).unwrap()
        .join("🖼️assets");
    for (name, subject) in [("🏢de-office-compliant", de_office_compliant()), ("⚠️multi-fail-noncompliant", multi_fail_noncompliant())] {
        let dir = assets.join(name).join(name);
        fs::create_dir_all(&dir).ok();
        fs::write(dir.join("🗣️.dsl.semio"), encode_en1991_dsl(&subject)).unwrap();
        fs::write(dir.join("📦️.pack.semio"), encode_en1991_pack(&subject)).unwrap();
    }
}

#[semio_framework_async_macros::async_test]
async fn python_oracle_within_half_percent() {
    use crate::standards::v1::subsets::any::schema::snapshot::encode_en1991_snapshot_json;
    use std::process::Command;
    let doc = En1991Snapshot::default();
    let report = evaluate(&doc);
    let json = encode_en1991_snapshot_json(&doc);
    let any = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
        .join("🏅️standards").read_dir().unwrap().next().unwrap().unwrap().path()
        .read_dir().unwrap().map(|e| e.unwrap().path()).find(|p| p.is_dir()).unwrap()
        .read_dir().unwrap().map(|e| e.unwrap().path()).find(|p| p.is_dir()).unwrap();
    let oracle = any.join("🔮️oracles").join("evaluate_en1991.py");
    let tmp = std::env::temp_dir().join("en1991-oracle-snap.json");
    std::fs::write(&tmp, &json).unwrap();
    let out = Command::new("python3").arg(&oracle).arg(&tmp).output().expect("run oracle");
    assert!(out.status.success(), "oracle stderr {}", String::from_utf8_lossy(&out.stderr));
    let parsed: serde_json::Value = serde_json::from_slice(&out.stdout).expect("oracle json");
    let arr = parsed.as_array().expect("array");
    for item in arr {
        let id = item["id"].as_str().unwrap().replace("selfweight", "self-weight");
        let req = item["required"].as_f64().unwrap();
        if let Some(check) = report.checks.iter().find(|c| c.id == id) {
            let limit = check.limit;
            let rel = ((limit.value - req) / req.max(1.0)).abs();
            assert!(rel <= 0.005, "oracle mismatch on {id}: rust={} py={} rel={rel}", limit.value, req);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn snapshot_json_validates_against_schema() {
    use crate::standards::v1::subsets::any::schema::snapshot::encode_en1991_snapshot_json;
    use std::process::Command;
    let doc = En1991Snapshot::default();
    let json = encode_en1991_snapshot_json(&doc);
    let any = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
        .join("🏅️standards").read_dir().unwrap().next().unwrap().unwrap().path()
        .read_dir().unwrap().map(|e| e.unwrap().path()).find(|p| p.is_dir()).unwrap()
        .read_dir().unwrap().map(|e| e.unwrap().path()).find(|p| p.is_dir()).unwrap();
    let schema = any.join("🧬️schema").join("📸️snapshot").join("🔣️.json");
    let py = "import json,sys\nimport jsonschema\njsonschema.validate(json.loads(sys.argv[2]), json.load(open(sys.argv[1])))\nprint('ok')\n";
    let out = Command::new("python3").arg("-c").arg(py).arg(schema.to_str().unwrap()).arg(&json).output().expect("jsonschema");
    assert!(out.status.success(), "jsonschema failed (third-party jsonschema required): status={} stdout={} stderr={}", out.status, String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    assert!(!String::from_utf8_lossy(&out.stdout).contains("skip"), "jsonschema must not skip");
}


#[semio_framework_async_macros::async_test]
async fn field_meta_coverage_all_committed_examples() {
    use crate::field_meta::en1991_field_meta;
    use crate::example_subjects::{
        de_accidental_variants, de_bridge_compliant, de_bridge_noncompliant, de_fire_parametric_compliant,
        de_fire_parametric_noncompliant, de_office_compliant, multi_fail_noncompliant,
    };
    use crate::standards::v1::subsets::any::schema::snapshot::encode_en1991_snapshot_json;
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
            assert!(!m.label_en.is_empty() && !m.label_de.is_empty(), "empty labels for {path}");
            if let Some(choices) = m.choices {
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
    use crate::example_subjects::{
        de_accidental_variants, de_bridge_compliant, de_fire_parametric_compliant, de_office_compliant,
    };
    use crate::{FireMode, StructureKind};
    // Exempt: descriptive entity `id` labels only. N/A leaves listed per example (bridge/fire/accidental scope).
    // N/A when fireMode=none / structureKind=building — discriminators fireMode/structureKind stay in scope.
    let office_na = [
        "enSk", "enVb", "thermalBridgeType", "mixedTerrainUpwind",
        "fireCurve", "fireDuration", "assumedGasTemperature", "assumedHNet", "fireCompartmentArea",
        "fireCompartmentHeight", "fireOpeningFactor", "fireThermalInertia", "fireOccupancy", "fireLoadDensityQf",
        "assumedQfD", "bridgeLane", "bridgeSpan", "bridgeLaneWidth", "assumedBridgeTandem", "assumedBridgeUdl",
        "assumedBridgeLm2", "assumedBridgeFootway", "assumedBridgeLm3", "assumedBridgeLm4", "bridgeLoadGroup",
    ];
    let bridge_na = [
        "enSk", "enVb", "thermalBridgeType", "mixedTerrainUpwind",
        // No wind/snow entities on bridge example — site wind/snow leaves are out of scope.
        "airDensity", "windZone", "terrainCategory", "orographyFactor", "coastOrIsland", "mixedTerrainDistance",
        "height", "width", "depth", "altitude", "snowZone", "exceptionalSnowNorthGermanLowlands",
        "fireMode", "fireCurve", "fireDuration", "assumedGasTemperature", "assumedHNet", "fireCompartmentArea",
        "fireCompartmentHeight", "fireOpeningFactor", "fireThermalInertia", "fireOccupancy", "fireLoadDensityQf",
        "assumedQfD", "floors", "selfWeightElements", "roofs", "windFaces",
        "craneClass", "hoistClass", "hoistingSpeed", "assumedCraneWheel", "assumedCraneHorizontal",
        "siloKind", "siloBulkDensity", "siloHeight", "siloHydraulicRadius", "siloMu", "siloK",
        "assumedSiloPressure", "assumedSiloPatch", "assumedSiloWallFriction", "accidentalCases",
    ];
    let fire_na = [
        "enSk", "enVb", "thermalBridgeType", "mixedTerrainUpwind", "fireCurve",
        "bridgeLane", "bridgeSpan", "bridgeLaneWidth", "assumedBridgeTandem", "assumedBridgeUdl", "assumedBridgeLm2",
        "assumedBridgeFootway", "assumedBridgeLm3", "assumedBridgeLm4", "bridgeLoadGroup",
        "craneClass", "hoistClass", "hoistingSpeed", "assumedCraneWheel", "assumedCraneHorizontal",
        "siloKind", "siloBulkDensity", "siloHeight", "siloHydraulicRadius", "siloMu", "siloK",
        "assumedSiloPressure", "assumedSiloPatch", "assumedSiloWallFriction",
    ];
    let accidental_na = [
        "enSk", "enVb", "thermalBridgeType", "mixedTerrainUpwind",
        "fireCurve", "fireDuration", "assumedGasTemperature", "assumedHNet", "fireCompartmentArea",
        "fireCompartmentHeight", "fireOpeningFactor", "fireThermalInertia", "fireOccupancy", "fireLoadDensityQf",
        "assumedQfD", "bridgeLane", "bridgeSpan", "bridgeLaneWidth", "assumedBridgeTandem", "assumedBridgeUdl",
        "assumedBridgeLm2", "assumedBridgeFootway", "assumedBridgeLm3", "assumedBridgeLm4", "bridgeLoadGroup",
        "craneClass", "hoistClass", "hoistingSpeed", "assumedCraneWheel", "assumedCraneHorizontal",
        "siloKind", "siloBulkDensity", "siloHeight", "siloHydraulicRadius", "siloMu", "siloK",
        "assumedSiloPressure", "assumedSiloPatch", "assumedSiloWallFriction",
    ];
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
        let obj = json.as_object().unwrap();
        for (key, val) in obj {
            if key == "id" || na.contains(&key.as_str()) {
                continue;
            }
            let mut j = json.clone();
            let mut changed = false;
            match val {
                serde_json::Value::Number(n) => {
                    if let Some(f) = n.as_f64() {
                        // Push past DE snow altitude plateaus (zone 2: ≤285 m) and similar clamps.
                        let next = match key.as_str() {
                            "altitude" => f.max(0.0) + 400.0,
                            "mixedTerrainDistance" => if f <= 0.0 { 500.0 } else { f * 0.25 },
                            // Cross EN 1991-1-4 Table 7.1 h/d thresholds (1 and 5) for zone D/E.
                            "depth" => {
                                let h = j.get("height").and_then(|x| x.as_f64()).unwrap_or(12.0);
                                (h / 5.5).max(0.5)
                            }
                            "height" => f.max(1.0) * 3.0 + 5.0,
                            "width" => f.max(1.0) * 0.2,
                            _ => if f.abs() < 1e-12 { 1.0 } else { f * 1.37 + 25.0 },
                        };
                        j[key] = serde_json::json!(next);
                        changed = true;
                    }
                }
                serde_json::Value::Bool(b) => {
                    j[key] = serde_json::json!(!*b);
                    changed = true;
                }
                serde_json::Value::String(s) => {
                    let next = match key.as_str() {
                        "annex" => if s == "de" { "en" } else { "de" },
                        "snowZone" => if s == "2" { "3" } else { "2" },
                        "constructionActivity" => if s == "scaffolding" { "formwork" } else { "scaffolding" },
                        "thermalElementType" => if s == "building" { "bridge1" } else { "building" },
                        "bridgeLoadGroup" => if s == "gr1a" { "gr5" } else { "gr1a" },
                        "craneClass" | "hoistClass" => if s == "HC2" { "HC4" } else { "HC2" },
                        "siloKind" => if s == "silo" { "tank" } else { "silo" },
                        "fireOccupancy" => if s == "office" { "warehouse" } else { "office" },
                        "fireCurve" => if s == "parametric" { "standard" } else { "parametric" },
                        "fireMode" => if s == "parametric" { "nominal" } else { "parametric" },
                        "structureKind" => if s == "bridge" { "building" } else { "bridge" },
                        _ => continue,
                    };
                    j[key] = serde_json::json!(next);
                    changed = true;
                }
                serde_json::Value::Array(arr) if !arr.is_empty() => {
                    if let Some(item) = j[key].as_array_mut().and_then(|a| a.get_mut(0)) {
                        fn bump_numeric_leaves(v: &mut serde_json::Value, changed: &mut bool) {
                            match v {
                                serde_json::Value::Object(map) => {
                                    for (k, child) in map.iter_mut() {
                                        if k == "id" { continue; }
                                        match child {
                                            serde_json::Value::Number(n) => {
                                                if let Some(f) = n.as_f64() {
                                                    *child = serde_json::json!(if f.abs() < 1e-12 { 1.0 } else { f * 0.55 });
                                                    *changed = true;
                                                }
                                            }
                                            serde_json::Value::Bool(b) => {
                                                *child = serde_json::json!(!*b);
                                                *changed = true;
                                            }
                                            serde_json::Value::Array(items) => {
                                                for it in items.iter_mut() { bump_numeric_leaves(it, changed); }
                                            }
                                            serde_json::Value::Object(_) => bump_numeric_leaves(child, changed),
                                            _ => {}
                                        }
                                    }
                                }
                                serde_json::Value::Array(items) => {
                                    for it in items.iter_mut() { bump_numeric_leaves(it, changed); }
                                }
                                _ => {}
                            }
                        }
                        bump_numeric_leaves(item, &mut changed);
                    }
                }
                _ => {}
            }
            if !changed {
                continue;
            }
            let Ok(perturbed) = serde_json::from_value::<En1991Snapshot>(j) else { continue };
            let after = evaluate(&perturbed);
            // CORRECTION 14:37/14:42: (id, status, computed, limit, utilization) — explanation-only changes do not count.
            let delta = baseline.checks.len() != after.checks.len()
                || baseline.checks.iter().zip(after.checks.iter()).any(|(a, b)| {
                    a.id != b.id
                        || a.status != b.status
                        || (a.limit.value - b.limit.value).abs() > 1e-6
                        || (a.computed.value - b.computed.value).abs() > 1e-6
                        || (a.utilization - b.utilization).abs() > 1e-6
                });
            assert!(delta, "perturbation of `{key}` on {name} did not change any check (exempt labels: id only; N/A listed in suite)");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn oracle_manifest_kind_count_matches_mutation_enum() {
    use crate::standards::v1::subsets::any::schema::mutations::KINDS;
    let any = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
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
