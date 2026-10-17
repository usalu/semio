/// ⚖️ Oracle and remedy laws: 5 tests covering python oracle matches check project within half percent; example snapshot validates against json schema; groundwater level changes bearing utilization; and more.
use semio_framework_test_fixtures::fixture_path;

/// 🔮️ Python oracle parity, snapshot schema, remedy application and sensitivity laws.

#[test]
fn python_oracle_matches_check_project_within_half_percent() {
    for (label, snap) in [("compliant", compliant_demo()), ("noncompliant", noncompliant_demo())] {
        let rust = check_project(&snap);
        let input = serde_json::to_string(&snap).expect("json");
        let mut child = Command::new("python3")
            .arg(oracle_script())
            .arg("--json")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap_or_else(|e| panic!("oracle spawn failed: {e}"));
        {
            use std::io::Write;
            child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
        }
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{label} oracle stderr={}",
            String::from_utf8_lossy(&output.stderr)
        );
        let py: serde_json::Value = serde_json::from_slice(&output.stdout).expect("oracle json");
        let py_checks = py.get("checks").and_then(|c| c.as_array()).expect("checks array");
        assert!(!py_checks.is_empty(), "{label} oracle returned no checks");
        for pc in py_checks {
            let id = pc.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let Some(pu) = pc.get("utilization").and_then(|v| v.as_f64()) else {
                continue;
            };
            let Some(rc) = rust.checks.iter().find(|c| c.id == id) else {
                panic!("{label} missing rust check id {id}");
            };
            let denom = pu.abs().max(rc.utilization.abs()).max(1e-9);
            let rel = (pu - rc.utilization).abs() / denom;
            assert!(
                rel <= 0.005 || (pu - rc.utilization).abs() <= 0.005,
                "{label} check {id}: python={pu} rust={} rel={rel}",
                rc.utilization
            );
        }
    }
}

#[test]
fn example_snapshot_validates_against_json_schema() {
    let schema_path = snapshot_schema_path();
    for snap in [compliant_demo(), noncompliant_demo()] {
        let mut instance_val = serde_json::to_value(&snap).expect("json value");
        if let Some(serde_json::Value::String(a)) = instance_val.get("annex").cloned() {
            instance_val["annex"] = serde_json::Value::String(a.to_ascii_lowercase());
        }
        let instance = serde_json::to_string(&instance_val).expect("instance");
        let tmp = std::env::temp_dir().join(format!("en1997-instance-{}.json", snap.structure_id));
        std::fs::write(&tmp, &instance).unwrap();
        let status = Command::new("python3")
            .arg("-c")
            .arg("import json,sys,jsonschema; s=json.load(open(sys.argv[1])); i=json.load(open(sys.argv[2])); jsonschema.validate(instance=i, schema=s); print('ok')")
            .arg(&schema_path)
            .arg(&tmp)
            .output()
            .expect("python jsonschema");
        assert!(
            status.status.success(),
            "jsonschema: {}\n{}",
            String::from_utf8_lossy(&status.stdout),
            String::from_utf8_lossy(&status.stderr)
        );
    }
}

fn family_any_dir() -> PathBuf {
    fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
}

fn oracle_script() -> PathBuf {
    family_any_dir().join("🔮️oracles/⚖️compliance/🐍️.py")
}

fn snapshot_schema_path() -> PathBuf {
    family_any_dir().join("🧬️schema/📸️snapshot/🔣️.json")
}

fn apply_remedy_required(doc: &mut En1997Snapshot, path: &str, value: f64) {
    if path == "investigationDepth" {
        doc.investigation_depth = value;
        return;
    }
    if path == "groundwaterLevel" {
        doc.groundwater_level = value;
        return;
    }
    if path == "geotechnicalCategory" {
        doc.geotechnical_category = value.round().clamp(1.0, 3.0) as u8;
        return;
    }
    if let Some(rest) = path.strip_prefix("footings[id=") {
        let (id, field) = split_id_field(rest);
        if let Some(f) = doc.footings.iter_mut().find(|x| x.id == id) {
            match field {
                "width" => f.width = value,
                "length" => f.length = value,
                "embedment" => f.embedment = value,
                "settlementLimit" => f.settlement_limit = value,
                _ => panic!("unsupported footing field {field}"),
            }
        }
        return;
    }
    if let Some(rest) = path.strip_prefix("footings[") {
        let (idx, field) = split_index_field(rest);
        if let Some(f) = doc.footings.get_mut(idx) {
            match field {
                "width" => f.width = value,
                "length" => f.length = value,
                "embedment" => f.embedment = value,
                "settlementLimit" => f.settlement_limit = value,
                _ => panic!("unsupported footing field {field}"),
            }
        }
        return;
    }
    if let Some(rest) = path.strip_prefix("piles[id=") {
        let (id, field) = split_id_field(rest);
        if let Some(p) = doc.piles.iter_mut().find(|x| x.id == id) {
            match field {
                "length" => p.length = value,
                "count" => p.count = value.round().max(1.0) as u32,
                "diameter" => p.diameter = value,
                "tensionPermanent" => p.tension_permanent = value,
                "tensionVariable" => p.tension_variable = value,
                _ => panic!("unsupported pile field {field}"),
            }
        }
        return;
    }
    if let Some(rest) = path.strip_prefix("piles[") {
        let (idx, field) = split_index_field(rest);
        if let Some(p) = doc.piles.get_mut(idx) {
            match field {
                "length" => p.length = value,
                "count" => p.count = value.round().max(1.0) as u32,
                "diameter" => p.diameter = value,
                "tensionPermanent" => p.tension_permanent = value,
                "tensionVariable" => p.tension_variable = value,
                _ => panic!("unsupported pile field {field}"),
            }
        }
        return;
    }
    if let Some(rest) = path.strip_prefix("layers[id=") {
        let (id, field) = split_id_field(rest);
        if let Some(layer) = doc.layers.iter_mut().find(|l| l.id == id) {
            match field {
                "oedometricModulus" => layer.oedometric_modulus = value,
                "phiPrimeDeg" => layer.phi_prime_deg = value,
                "cohesionUndrained" => layer.cohesion_undrained = value,
                "poissonRatio" => layer.poisson_ratio = value,
                _ => panic!("unsupported layer field {field}"),
            }
        }
        return;
    }
    if let Some(rest) = path.strip_prefix("retainingWalls[id=") {
        let (id, field) = split_id_field(rest);
        if let Some(w) = doc.retaining_walls.iter_mut().find(|x| x.id == id) {
            match field {
                "baseWidth" => w.base_width = value,
                "embedment" => w.embedment = value,
                "height" => w.height = value,
                "concreteGamma" => w.concrete_gamma = value,
                _ => panic!("unsupported wall field {field}"),
            }
        }
        return;
    }
    if let Some(rest) = path.strip_prefix("slopes[id=") {
        let (id, field) = split_id_field(rest);
        if let Some(s) = doc.slopes.iter_mut().find(|x| x.id == id) {
            match field {
                "angleDeg" => s.angle_deg = value,
                "length" => s.length = value,
                "height" => s.height = value,
                _ => panic!("unsupported slope field {field}"),
            }
        }
        return;
    }
    if let Some(rest) = path.strip_prefix("upliftCases[id=") {
        let (id, field) = split_id_field(rest);
        if let Some(u) = doc.uplift_cases.iter_mut().find(|x| x.id == id) {
            match field {
                "permanentStabilizing" => u.permanent_stabilizing = value,
                "totalStress" => u.total_stress = value,
                "porePressure" => u.pore_pressure = value,
                _ => panic!("unsupported uplift field {field}"),
            }
        }
        return;
    }
    panic!("unsupported remedy path {path}");
}

fn split_id_field(rest: &str) -> (&str, &str) {
    let close = rest.find(']').expect("id]");
    let id = &rest[..close];
    let field = rest[close + 1..].trim_start_matches('.');
    (id, field)
}

fn split_index_field(rest: &str) -> (usize, &str) {
    let close = rest.find(']').expect("]");
    let idx: usize = rest[..close].parse().expect("index");
    let field = rest[close + 1..].trim_start_matches('.');
    (idx, field)
}

#[test]
fn groundwater_level_changes_bearing_utilization() {
    let mut doc = compliant_demo();
    doc.groundwater_level = 0.2; // below footing base → γ′
    let u_wet = check_project(&doc).checks.iter().find(|c| c.id.contains("bearing")).map(|c| c.utilization).unwrap();
    doc.groundwater_level = 20.0; // dry
    let u_dry = check_project(&doc).checks.iter().find(|c| c.id.contains("bearing")).map(|c| c.utilization).unwrap();
    assert!((u_wet - u_dry).abs() > 0.01, "GWL must change bearing u; wet={u_wet} dry={u_dry}");
}

#[test]
fn base_inclination_changes_bearing_utilization() {
    let mut doc = compliant_demo();
    doc.footings[0].base_inclination_deg = 0.0;
    let u0 = check_project(&doc).checks.iter().find(|c| c.id.contains("bearing")).map(|c| c.utilization).unwrap();
    doc.footings[0].base_inclination_deg = 12.0;
    let u1 = check_project(&doc).checks.iter().find(|c| c.id.contains("bearing")).map(|c| c.utilization).unwrap();
    assert!(u1 > u0 + 0.01, "base inclination must reduce resistance / raise u; u0={u0} u1={u1}");
}

#[test]
fn rankine_vs_at_rest_changes_wall_earth_pressure() {
    let mut doc = compliant_demo();
    doc.retaining_walls[0].earth_pressure_mode = "active".into();
    let u_a = check_project(&doc).checks.iter().find(|c| c.id.contains("earthPressure") || c.id.contains("9.sliding")).map(|c| c.utilization).unwrap();
    doc.retaining_walls[0].earth_pressure_mode = "atRest".into();
    let u_r = check_project(&doc).checks.iter().find(|c| c.id.contains("9.sliding")).map(|c| c.utilization).unwrap();
    assert!((u_a - u_r).abs() > 0.01, "Rankine vs K0 must change wall sliding u; active={u_a} atRest={u_r}");
}
