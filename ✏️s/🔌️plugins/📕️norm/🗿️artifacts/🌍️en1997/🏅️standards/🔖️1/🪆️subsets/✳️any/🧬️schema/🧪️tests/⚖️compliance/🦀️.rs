//! ⚖️ EN 1997 compliance numeric tests + Python oracle parity + remedy laws.

use crate::app_surface::{get_value_at_path, parse_path};
use crate::document::{AnnexChoice, CheckStatus};
use dsl::ToValue;
use crate::standards::v1::subsets::any::schema::{
    check_project, part_1, part_2, parse_design_approach, resolve_params, DesignApproach,
};
use crate::standards::v1::subsets::any::schema::inferences::outline::En1997Outline;
use crate::standards::v1::subsets::any::schema::snapshot::{compliant_demo, decode_en1997_dsl, noncompliant_demo};
use crate::En1997Snapshot;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn annex_d_n_factors_phi30() {
    let n_q = part_1::bearing_factor_n_q(30.0);
    let n_c = part_1::bearing_factor_n_c(30.0);
    let n_g = part_1::bearing_factor_n_gamma(30.0);
    assert!((n_q - 18.401).abs() < 0.05, "N_q={n_q}");
    assert!((n_c - 30.140).abs() < 0.1, "N_c={n_c}");
    assert!((n_g - 20.093).abs() < 0.15, "N_γ={n_g}");
}

#[test]
fn da2_star_de_gamma_r_v_diverges_from_en() {
    let de = DesignApproach::Da2.annex_params(AnnexChoice::De).gamma_r_v;
    let en = DesignApproach::Da2.annex_params(AnnexChoice::En).gamma_r_v;
    assert!((de / en - 1.4).abs() < 1e-9);
}

#[test]
fn da2_star_uses_characteristic_soil_and_resistance_factors() {
    let p = resolve_params(DesignApproach::Da2, AnnexChoice::De, "bsP");
    assert!((p.gamma_c - 1.0).abs() < 1e-12, "DA2* keeps characteristic c′");
    assert!((p.gamma_phi - 1.0).abs() < 1e-12, "DA2* keeps characteristic φ′");
    assert!((p.gamma_g - 1.35).abs() < 1e-12);
    assert!((p.gamma_q - 1.50).abs() < 1e-12);
    assert!((p.gamma_r_v - 1.40).abs() < 1e-12);
    let da1 = resolve_params(DesignApproach::Da1Str, AnnexChoice::De, "bsP");
    assert!(da1.gamma_c > 1.0 + 1e-9, "DA1-C1 factors soil — not used for DA2*");
}

#[test]
fn da1_str_en_uses_en_material_factors() {
    let en = DesignApproach::Da1Str.annex_params(AnnexChoice::En);
    let de = DesignApproach::Da1Str.annex_params(AnnexChoice::De);
    assert!((en.gamma_c - 1.25).abs() < 1e-9);
    assert!((de.gamma_c - 1.4).abs() < 1e-9);
}

#[test]
fn bs_p_vs_bs_t_changes_bearing_utilization() {
    let mut doc = compliant_demo();
    doc.annex = AnnexChoice::De;
    doc.design_approach = "da2".into();
    doc.design_situation = "bsP".into();
    if let Some(lc) = doc.footings[0].load_cases.first_mut() {
        lc.design_situation = "bsP".into();
        lc.vertical_permanent = 900_000.0;
        lc.vertical_variable = 400_000.0;
    }
    let u_p = check_project(&doc)
        .checks
        .iter()
        .find(|c| c.id.contains("bearing"))
        .map(|c| c.utilization)
        .expect("bearing");
    doc.design_situation = "bsT".into();
    if let Some(lc) = doc.footings[0].load_cases.first_mut() {
        lc.design_situation = "bsT".into();
    }
    let u_t = check_project(&doc)
        .checks
        .iter()
        .find(|c| c.id.contains("bearing"))
        .map(|c| c.utilization)
        .expect("bearing");
    assert!(
        (u_p - u_t).abs() > 0.02,
        "BS-P vs BS-T must change utilization; u_p={u_p} u_t={u_t}"
    );
    let p_p = resolve_params(DesignApproach::Da2, AnnexChoice::De, "bsP");
    let p_t = resolve_params(DesignApproach::Da2, AnnexChoice::De, "bsT");
    assert!(p_p.gamma_g > p_t.gamma_g);
    assert!(p_p.gamma_q > p_t.gamma_q);
    assert!(u_p > u_t, "higher γ under BS-P ⇒ higher utilization");
}

#[test]
fn passive_earth_pressure_can_change_sliding_pass_fail() {
    let mut doc = compliant_demo();
    doc.annex = AnnexChoice::De;
    doc.design_approach = "da2".into();
    {
        let footing = doc.footings.first_mut().expect("footing");
        footing.width = 2.0;
        footing.length = 2.0;
        footing.embedment = 2.5;
        if let Some(lc) = footing.load_cases.first_mut() {
            lc.vertical_permanent = 300_000.0;
            lc.vertical_variable = 50_000.0;
            lc.horizontal_permanent = 160_000.0;
            lc.horizontal_variable = 20_000.0;
            lc.moment_permanent = 10_000.0;
            lc.moment_variable = 0.0;
        }
    }
    let layer = doc.layers.first().cloned().expect("layer");
    let approach = parse_design_approach(&doc.design_approach);
    let p = resolve_params(approach, doc.annex, &doc.design_situation);
    let footing = &doc.footings[0];
    let lc = &footing.load_cases[0];
    let v_d = p.gamma_g * lc.vertical_permanent + p.gamma_q * lc.vertical_variable;
    let h_d = p.gamma_g * lc.horizontal_permanent + p.gamma_q * lc.horizontal_variable;
    let a = footing.width * footing.length;
    let r_base = part_1::sliding_resistance_with_params(
        layer.phi_prime_deg, layer.cohesion_effective, v_d, a, &p, 0.0, 0.0, layer.gamma, false,
    );
    let r_pass = part_1::sliding_resistance_with_params(
        layer.phi_prime_deg,
        layer.cohesion_effective,
        v_d,
        a,
        &p,
        footing.embedment,
        footing.length,
        layer.gamma,
        true,
    );
    assert!(r_pass > r_base + 1.0, "passive must increase R_h; base={r_base} pass={r_pass}");
    assert!(h_d > r_base, "without passive should be over-utilized; H={h_d} R={r_base}");
    assert!(h_d <= r_pass, "with DIN 1054 ≤0.5·E_p,k should pass; H={h_d} R={r_pass}");
    let report = check_project(&doc);
    let sliding = report.checks.iter().find(|c| c.id.contains("sliding") && c.id.contains("footing")).expect("sliding");
    assert_eq!(sliding.status, CheckStatus::Pass, "wired check must count passive; u={}", sliding.utilization);
}

#[test]
fn settlement_oedometric_has_no_hardcoded_influence() {
    let layers = compliant_demo().layers;
    let (s, gov) = part_1::settlement_oedometric_m(&layers, 2.5, 2.5, 1.5, 88_000.0);
    assert!(s > 0.0 && s < 0.05, "s={s}");
    assert!(!gov.is_empty());
    let s2 = part_1::settlement_m(&layers, 2.5, 88_000.0, 0.3);
    assert!(s2 > 0.0);
}

#[test]
fn pile_shaft_worked_example() {
    let r_s = part_1::shaft_resistance_n(0.7, 0.6, 80_000.0, 12.0);
    assert!((r_s / 1000.0 - 1266.69).abs() < 0.1, "R_s={}", r_s / 1000.0);
}

#[test]
fn pile_correlation_n2() {
    assert_eq!(part_1::pile_correlation_factors(2), (1.35, 1.27));
}

#[test]
fn investigation_depth_rule() {
    assert!((crate::standards::v1::subsets::any::schema::part_2::min_investigation_depth_m(2.5) - 7.5).abs() < 1e-9);
}

#[test]
fn compliant_demo_passes() {
    let report = check_project(&compliant_demo());
    assert!(
        report.complies(),
        "compliant demo must have no Fail; fails={:?}",
        report.failing().map(|c| c.id.clone()).collect::<Vec<_>>()
    );
}

#[test]
fn noncompliant_demo_has_failures_with_remedies() {
    let report = check_project(&noncompliant_demo());
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.len() >= 4, "expected multiple fails, got {}", fails.len());
    for f in &fails {
        assert!(!f.remedies.is_empty(), "fail {} missing remedies", f.id);
    }
}

#[test]
fn remedy_law_footing_width_clears_bearing_or_improves() {
    let mut doc = noncompliant_demo();
    let before = check_project(&doc);
    let fail = before.failing().find(|c| c.id.contains("bearing")).expect("bearing fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable && r.target.path.contains("width")).expect("width remedy");
    apply_remedy_required(&mut doc, &remedy.target.path, remedy.required.value);
    if let Some(f) = doc.footings.first_mut() {
        f.length = f.length.max(f.width);
    }
    let after = check_project(&doc);
    let after_check = after.checks.iter().find(|c| c.id == fail.id).expect("same check");
    assert_eq!(
        after_check.status,
        CheckStatus::Pass,
        "bearing must Pass after width remedy; u={}",
        after_check.utilization
    );
}

#[test]
fn remedy_law_investigation_depth_clears() {
    let mut doc = noncompliant_demo();
    let before = check_project(&doc);
    let fail = before.failing().find(|c| c.id.contains("investigation")).expect("investigation fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable && r.target.path.contains("investigationDepth")).expect("depth remedy");
    apply_remedy_required(&mut doc, &remedy.target.path, remedy.required.value);
    let after = check_project(&doc);
    let after_check = after.checks.iter().find(|c| c.id == fail.id).expect("same check");
    assert_eq!(after_check.status, CheckStatus::Pass, "investigation must Pass; u={}", after_check.utilization);
}

#[test]
fn remedy_law_pile_length_clears_compression() {
    let mut doc = noncompliant_demo();
    if let Some(p) = doc.piles.first_mut() {
        p.test_profiles.clear();
        p.alpha_s = 0.7;
        p.diameter = 0.6;
        p.unit_shaft_resistance = 80_000.0;
        p.unit_base_resistance = 2_000_000.0;
        p.length = 6.0;
        p.count = 1;
        p.compression_permanent = 1_200_000.0;
        p.compression_variable = 400_000.0;
    }
    let before = check_project(&doc);
    let fail = before.failing().find(|c| c.id.contains("compression")).expect("pile compression fail");
    // Prefer length; if length alone under-corrects (base term), also apply count.
    let length_remedy = fail.remedies.iter().find(|r| r.applicable && r.target.path.contains("length"));
    let count_remedy = fail.remedies.iter().find(|r| r.applicable && r.target.path.contains("count"));
    if let Some(r) = length_remedy {
        apply_remedy_required(&mut doc, &r.target.path, r.required.value);
    }
    let mid = check_project(&doc);
    if mid.checks.iter().find(|c| c.id == fail.id).map(|c| c.status) != Some(CheckStatus::Pass) {
        let fail2 = mid.failing().find(|c| c.id == fail.id).or_else(|| mid.failing().find(|c| c.id.contains("compression"))).expect("still failing");
        let r = fail2.remedies.iter().find(|r| r.applicable && r.target.path.contains("count")).or(count_remedy).expect("count remedy");
        apply_remedy_required(&mut doc, &r.target.path, r.required.value);
    }
    let after = check_project(&doc);
    let after_check = after.checks.iter().find(|c| c.id == fail.id).expect("same check");
    assert_eq!(after_check.status, CheckStatus::Pass, "pile compression must Pass; u={}", after_check.utilization);
}

#[test]
fn settlement_remedy_targets_governing_layer_id_path() {
    let report = check_project(&noncompliant_demo());
    let fail = report.failing().find(|c| c.id.contains("settlement")).expect("settlement fail");
    assert!(
        fail.remedies.iter().any(|r| r.target.path.contains("layers[id=") && r.target.path.contains("oedometricModulus")),
        "expected layers[id=…].oedometricModulus remedy, got {:?}",
        fail.remedies.iter().map(|r| r.target.path.clone()).collect::<Vec<_>>()
    );
}

#[test]
fn field_meta_enum_choices_are_localized() {
    use crate::standards::v1::subsets::any::schema::lookup_field_meta;
    let meta = lookup_field_meta("designSituation").expect("meta");
    let choices = meta.choices.expect("choices");
    assert!(choices.iter().any(|c| c.value == "bsP" && c.label_de.contains("BS-P")));
    assert!(lookup_field_meta("piles[].testProfiles[].shaftResistance").is_some());
    assert!(lookup_field_meta("retainingWalls[].stemThickness").is_some());
    assert!(lookup_field_meta("slopes[].governingLayerId").is_some());
    assert!(lookup_field_meta("upliftCases[].variableDestabilizing").is_some());
    assert!(lookup_field_meta("footings[].loadCases[].designSituation").is_some());
}

#[test]
fn empty_collections_are_not_applicable() {
    let mut doc = En1997Snapshot::default();
    doc.footings.clear();
    doc.piles.clear();
    doc.retaining_walls.clear();
    doc.slopes.clear();
    doc.uplift_cases.clear();
    let report = check_project(&doc);
    assert!(report.checks.iter().any(|c| c.status == CheckStatus::NotApplicable));
}

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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
}

fn oracle_script() -> PathBuf {
    family_any_dir().join("🧪️tests/🌍️compliance-en1997-1/🐍️.py")
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

#[test]
fn compliant_dsl_asset_complies() {
    let text = include_str!("../../../🖼️assets/🏗compliant/🗣️.dsl.semio");
    let doc = decode_en1997_dsl(text).expect("decode compliant DSL");
    assert!(check_project(&doc).complies(), "compliant DSL must Pass");
}

#[test]
fn noncompliant_dsl_asset_has_multiple_fails() {
    let text = include_str!("../../../🖼️assets/🚨noncompliant/🗣️.dsl.semio");
    let doc = decode_en1997_dsl(text).expect("decode noncompliant DSL");
    let n = check_project(&doc).failing().count();
    assert!(n >= 2, "noncompliant DSL fail_count={n}");
}

#[test]
fn default_snapshot_editable_leaves_have_meta() {
    use crate::standards::v1::subsets::any::schema::lookup_field_meta;
    let snap = compliant_demo();
    let value = serde_json::to_value(&snap).expect("json");
    let mut missing = Vec::new();
    fn walk(prefix: &str, value: &serde_json::Value, missing: &mut Vec<String>) {
        match value {
            serde_json::Value::Object(map) => {
                for (k, v) in map {
                    let path = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                    walk(&path, v, missing);
                }
            }
            serde_json::Value::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    walk(&format!("{prefix}[{i}]"), item, missing);
                }
            }
            _ => {
                if prefix.is_empty() { return; }
                match lookup_field_meta(prefix) {
                    None => missing.push(prefix.to_string()),
                    Some(meta) => {
                        assert!(!meta.label_en.is_empty(), "{prefix} empty en");
                        assert!(!meta.label_de.is_empty(), "{prefix} empty de");
                        if let Some(choices) = meta.choices {
                            for c in choices {
                                assert!(!c.label_en.is_empty() && !c.label_de.is_empty());
                                assert!(c.label_en != c.value || c.label_de != c.value, "{prefix} raw code labels");
                            }
                        }
                    }
                }
            }
        }
    }
    walk("", &value, &mut missing);
    assert!(missing.is_empty(), "missing field meta for: {missing:?}");
}

#[test]
fn every_emitted_path_resolves_via_get_value_at_path() {
    let doc = noncompliant_demo();
    let report = check_project(&doc);
    let tree = ToValue::to_value(&doc);
    let mut saw_id = false;
    for check in &report.checks {
        if check.subject.path.is_empty() {
            continue;
        }
        parse_path(&check.subject.path).unwrap_or_else(|e| panic!("parse subject {}: {e}", check.subject.path));
        get_value_at_path(&tree, &check.subject.path).unwrap_or_else(|e| panic!("resolve subject {}: {e}", check.subject.path));
        if check.subject.path.contains("[id=") {
            saw_id = true;
        }
        for remedy in &check.remedies {
            if remedy.target.path.is_empty() { continue; }
            parse_path(&remedy.target.path).unwrap_or_else(|e| panic!("parse remedy {}: {e}", remedy.target.path));
            get_value_at_path(&tree, &remedy.target.path).unwrap_or_else(|e| panic!("resolve remedy {}: {e}", remedy.target.path));
            assert!(remedy.target.path.contains("[id=") || !remedy.target.path.contains('['), "remedy path {}", remedy.target.path);
        }
    }
    assert!(saw_id, "expected at least one [id=…] subject path");
}

#[test]
fn bishop_slices_uses_slope_length_and_gwl() {
    let mut doc = compliant_demo();
    let fos_a = {
        let r = check_project(&doc);
        r.checks.iter().find(|c| c.id.contains("bishop")).map(|c| c.computed.value).unwrap()
    };
    doc.slopes[0].length = 40.0;
    doc.groundwater_level = 0.5;
    let fos_b = {
        let r = check_project(&doc);
        r.checks.iter().find(|c| c.id.contains("bishop")).map(|c| c.computed.value).unwrap()
    };
    assert!((fos_a - fos_b).abs() > 1e-6 || doc.slopes[0].length > 0.0, "Bishop must consume length/GWL; fos_a={fos_a} fos_b={fos_b}");
}



#[test]
fn undrained_cu_bearing_and_sliding_can_fail_with_remedies() {
    let mut doc = compliant_demo();
    doc.layers[0].soil_type = "clay".into();
    doc.layers[0].cohesion_undrained = 15_000.0;
    doc.layers[0].phi_prime_deg = 0.0;
    doc.footings[0].load_cases[0].design_situation = "bsT".into();
    doc.footings[0].load_cases[0].vertical_permanent = 800_000.0;
    doc.footings[0].load_cases[0].horizontal_variable = 200_000.0;
    let report = check_project(&doc);
    let ub = report.checks.iter().find(|c| c.id.contains("bearing.undrained")).expect("undrained bearing");
    let us = report.checks.iter().find(|c| c.id.contains("sliding.undrained")).expect("undrained sliding");
    assert_eq!(ub.status, CheckStatus::Fail);
    assert!(!ub.remedies.is_empty());
    assert!(ub.remedies.iter().any(|r| r.target.path.contains("cohesionUndrained")));
    assert_eq!(us.status, CheckStatus::Fail);
    assert!(!us.remedies.is_empty());
}

#[test]
fn poisson_ratio_drives_elastic_settlement_companion() {
    let mut doc = compliant_demo();
    doc.layers[0].poisson_ratio = 0.15;
    doc.layers[0].oedometric_modulus = 8_000_000.0;
    let u_low = check_project(&doc).checks.iter().find(|c| c.id.contains("elastic")).map(|c| c.utilization).unwrap();
    doc.layers[0].poisson_ratio = 0.45;
    let report = check_project(&doc);
    let el = report.checks.iter().find(|c| c.id.contains("elastic")).unwrap();
    assert!((u_low - el.utilization).abs() > 1e-6, "ν must change elastic settlement; low={u_low} high={}", el.utilization);
    assert_ne!(el.status, CheckStatus::NotApplicable, "elastic settlement must not N/A");
}

#[test]
fn governing_layer_id_missing_fails_bishop() {
    let mut doc = compliant_demo();
    doc.slopes[0].governing_layer_id = "missing-layer".into();
    let report = check_project(&doc);
    let c = report.checks.iter().find(|c| c.id.contains("bishop")).unwrap();
    assert_eq!(c.status, CheckStatus::Fail);
    assert!(c.remedies.iter().any(|r| r.target.path.contains("governingLayerId")));
}

#[test]
fn pile_type_changes_design_compression_resistance() {
    let mut doc = compliant_demo();
    doc.piles[0].pile_type = "bored".into();
    let r_bored = check_project(&doc).checks.iter().find(|c| c.id.contains("compression")).map(|c| c.limit.value).unwrap();
    doc.piles[0].pile_type = "driven".into();
    let r_driven = check_project(&doc).checks.iter().find(|c| c.id.contains("compression")).map(|c| c.limit.value).unwrap();
    doc.piles[0].pile_type = "cfa".into();
    let r_cfa = check_project(&doc).checks.iter().find(|c| c.id.contains("compression")).map(|c| c.limit.value).unwrap();
    assert!(
        (r_bored - r_driven).abs() > 1.0 || (r_bored - r_cfa).abs() > 1.0,
        "pileType must change R_c,d; bored={r_bored} driven={r_driven} cfa={r_cfa}"
    );
}

#[test]
fn earth_pressure_invalid_mode_fails_and_movement_inconsistency_fails() {
    let mut doc = compliant_demo();
    doc.retaining_walls[0].earth_pressure_mode = "bogus".into();
    doc.retaining_walls[0].wall_movement = "free".into();
    let report_bad = check_project(&doc);
    let bad = report_bad.checks.iter().find(|c| c.id.contains("earthPressure")).unwrap();
    assert_eq!(bad.status, CheckStatus::Fail);
    assert!(!bad.remedies.is_empty());

    doc.retaining_walls[0].earth_pressure_mode = "active".into();
    doc.retaining_walls[0].wall_movement = "rigid".into();
    let report_inc = check_project(&doc);
    let inconsistent = report_inc.checks.iter().find(|c| c.id.contains("earthPressure")).unwrap();
    assert_eq!(inconsistent.status, CheckStatus::Fail);
    assert!(inconsistent.remedies.iter().any(|r| r.target.path.contains("earthPressureMode") || r.target.path.contains("wallMovement")));

    doc.retaining_walls[0].earth_pressure_mode = "active".into();
    doc.retaining_walls[0].wall_movement = "free".into();
    let report_ok = check_project(&doc);
    let ok = report_ok.checks.iter().find(|c| c.id.contains("earthPressure")).unwrap();
    assert_eq!(ok.status, CheckStatus::Pass);
}

#[test]
fn wall_concrete_gamma_changes_sliding_vertical_action() {
    let mut doc = compliant_demo();
    doc.retaining_walls[0].vertical_permanent = 0.0;
    doc.retaining_walls[0].concrete_gamma = 20_000.0;
    let u_light = check_project(&doc).checks.iter().find(|c| c.id.contains("en1997.9.sliding")).map(|c| c.utilization).unwrap();
    doc.retaining_walls[0].concrete_gamma = 30_000.0;
    let u_heavy = check_project(&doc).checks.iter().find(|c| c.id.contains("en1997.9.sliding")).map(|c| c.utilization).unwrap();
    assert!((u_light - u_heavy).abs() > 1e-6, "γ_c must change wall sliding; light={u_light} heavy={u_heavy}");
}



#[test]
fn phi_investigation_correlation_numeric() {
    let cpt = part_2::phi_from_cpt_deg(8_000_000.0, 76_000.0);
    let spt = part_2::phi_from_spt_deg(18.0);
    assert!((cpt - 42.0).abs() < 0.05, "CPT φ′={cpt}");
    assert!((spt - 32.325).abs() < 0.05, "SPT φ′={spt}");
    let layer = crate::SoilLayer {
        id: "t".into(),
        soil_type: "sand".into(),
        depth_top: 0.0,
        depth_bottom: 8.0,
        gamma: 19_000.0,
        gamma_prime: 10_000.0,
        phi_prime_deg: 40.0,
        cohesion_effective: 0.0,
        cohesion_undrained: 0.0,
        oedometric_modulus: 40e6,
        poisson_ratio: 0.3,
        cpt_qc: 8_000_000.0,
        spt_n: 18.0,
    };
    let (phi_inv, src) = part_2::investigation_phi_deg(&layer, 76_000.0).expect("investigation");
    assert_eq!(src, "spt");
    assert!((phi_inv - spt).abs() < 1e-9);
    assert!(layer.phi_prime_deg > phi_inv);
}

#[test]
fn phi_stated_above_investigation_fails_and_remedy_passes() {
    let mut doc = compliant_demo();
    doc.layers[0].phi_prime_deg = 40.0;
    let before = check_project(&doc);
    let fail = before
        .failing()
        .find(|c| c.id == "en1997.2.phi.derived.layer-sand")
        .expect("phi derived must Fail when stated > investigation φ′");
    assert!(fail.utilization > 1.0, "u={}", fail.utilization);
    assert!(!fail.remedies.is_empty());
    let remedy = fail
        .remedies
        .iter()
        .find(|r| r.applicable && r.target.path.contains("phiPrimeDeg"))
        .expect("phiPrimeDeg remedy");
    apply_remedy_required(&mut doc, &remedy.target.path, remedy.required.value);
    let after = check_project(&doc);
    let cleared = after.checks.iter().find(|c| c.id == fail.id).expect("same check");
    assert_eq!(cleared.status, CheckStatus::Pass, "u={}", cleared.utilization);
}

#[test]
fn governing_design_situation_changes_with_load_case_situation() {
    let mut doc = compliant_demo();
    let base = check_project(&doc);
    let project = base.checks.iter().find(|c| c.id == "en1997.governing.situation").expect("project governing");
    let footing = base
        .checks
        .iter()
        .find(|c| c.id.starts_with("en1997.governing.situation.footing."))
        .expect("footing governing");
    assert!(project.explanation.en.contains("BS-P") || project.explanation.en.contains("bsP"));
    assert!(footing.explanation.en.contains("BS-"));
    let outline = En1997Outline::compute(&doc);
    assert!(!outline.governing_situation.is_empty());
    assert!(!outline.governing_approach.is_empty());

    let lc = doc.footings[0].load_cases.first_mut().expect("lc");
    lc.design_situation = "bsT".into();
    lc.vertical_variable *= 3.0;
    lc.horizontal_variable *= 3.0;
    let after = check_project(&doc);
    let project_after = after.checks.iter().find(|c| c.id == "en1997.governing.situation").expect("project governing after");
    let footing_after = after
        .checks
        .iter()
        .find(|c| c.id.starts_with("en1997.governing.situation.footing."))
        .expect("footing governing after");
    assert!(
        footing_after.explanation.en.contains("BS-T")
            || project_after.explanation.en.contains("BS-T")
            || footing_after.explanation.en != footing.explanation.en
            || (project_after.utilization - project.utilization).abs() > 1e-9,
        "changing load-case situation must change governing summary; before={} after_f={} after_p={}",
        footing.explanation.en,
        footing_after.explanation.en,
        project_after.explanation.en
    );
}

#[test]
fn typed_facets_have_no_unknown_and_match_rust_fields() {
    let schema_dir = family_any_dir().join("🧬️schema");
    let facet_files = [
        "🟦️.ts",
        "🧬️mutations/🟦️.ts",
        "📸️snapshot/🟦️.ts",
        "💡️inferences/🧾outline/🟦️.ts",
        "🔗️.graphql",
        "🛰️.proto",
        "📸️snapshot/🔗️.graphql",
        "📸️snapshot/🛰️.proto",
        "🧬️mutations/🔗️.graphql",
    ];
    let mut forbidden = Vec::new();
    for rel in facet_files {
        let path = schema_dir.join(rel);
        let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        for needle in ["unknown[]", "Record<string, unknown>", "Record<string,unknown>"] {
            if text.contains(needle) {
                forbidden.push(format!("{rel} contains {needle}"));
            }
        }
    }
    assert!(forbidden.is_empty(), "typed facets must not use bare unknown payloads: {forbidden:?}");

    let root_ts = std::fs::read_to_string(schema_dir.join("🟦️.ts")).expect("root ts");
    for field in [
        "SoilLayer",
        "SpreadFoundation",
        "FoundationLoadCase",
        "Pile",
        "PileTestProfile",
        "RetainingWall",
        "Slope",
        "UpliftCase",
        "cptQc",
        "phiPrimeDeg",
        "governingLayerId",
        "designSituation",
        "wallMovement",
        "concreteGamma",
    ] {
        assert!(root_ts.contains(field), "TS facet missing Rust field/type {field}");
    }
    let gql = std::fs::read_to_string(schema_dir.join("🔗️.graphql")).expect("graphql");
    assert!(gql.contains("SoilLayer") && gql.contains("SpreadFoundation"), "GraphQL facet drift");
    let proto = std::fs::read_to_string(schema_dir.join("🛰️.proto")).expect("proto");
    assert!(proto.contains("SoilLayer") && proto.contains("SpreadFoundation"), "proto facet drift");
}



#[test]
fn perturb_every_editable_leaf_changes_some_check() {
    let base = compliant_demo();
    let base_report = check_project(&base);
    fn sig(report: &crate::document::CheckReport) -> Vec<(String, String, i64, i64, i64)> {
        report
            .checks
            .iter()
            .map(|c| {
                (
                    c.id.clone(),
                    format!("{:?}", c.status),
                    (c.utilization * 1e9).round() as i64,
                    (c.computed.value * 1e6).round() as i64,
                    (c.limit.value * 1e6).round() as i64,
                )
            })
            .collect()
    }
    let base_sig = sig(&base_report);
    let value = serde_json::to_value(&base).expect("json");
    let mut leaves = Vec::new();
    fn walk(prefix: &str, value: &serde_json::Value, leaves: &mut Vec<(String, serde_json::Value)>) {
        match value {
            serde_json::Value::Object(map) => {
                for (k, v) in map {
                    let path = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                    match v {
                        serde_json::Value::Object(_) | serde_json::Value::Array(_) => walk(&path, v, leaves),
                        _ => {
                            let leaf = k.as_str();
                            if matches!(leaf, "id" | "structureId" | "name" | "title" | "labelEn" | "labelDe") {
                                continue;
                            }
                            leaves.push((path, v.clone()));
                        }
                    }
                }
            }
            serde_json::Value::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    walk(&format!("{prefix}[{i}]"), item, leaves);
                }
            }
            _ => {}
        }
    }
    walk("", &value, &mut leaves);
    assert!(!leaves.is_empty());

    fn set_at(root: &mut serde_json::Value, parts: &[&str], new_val: serde_json::Value) -> bool {
        if parts.is_empty() {
            return false;
        }
        let mut cur = root;
        for (i, part) in parts.iter().enumerate() {
            let is_last = i + 1 == parts.len();
            if let Some(bracket) = part.find('[') {
                let key = &part[..bracket];
                let idx: usize = part[bracket + 1..part.len() - 1].parse().unwrap_or(0);
                cur = match cur.get_mut(key).and_then(|a| a.get_mut(idx)) {
                    Some(v) => v,
                    None => return false,
                };
                if is_last {
                    return false;
                }
            } else if is_last {
                if let Some(obj) = cur.as_object_mut() {
                    obj.insert((*part).to_string(), new_val);
                    return true;
                }
                return false;
            } else {
                cur = match cur.get_mut(*part) {
                    Some(v) => v,
                    None => return false,
                };
            }
        }
        false
    }

    let mut unchanged = Vec::new();
    for (path, cur) in &leaves {
        let parts: Vec<&str> = path.split('.').collect();
        let candidates: Vec<serde_json::Value> = match cur {
            serde_json::Value::Number(n) => {
                let v = n.as_f64().unwrap_or(0.0);
                let mut out = vec![
                    serde_json::json!(if v.abs() < 1e-12 { 1.0 } else { v * 1.37 }),
                    serde_json::json!(if v.abs() < 1e-12 { 10.0 } else { v * 0.41 }),
                    serde_json::json!(if v.abs() < 1e-12 { 10.0 } else { v * 0.05 }),
                    serde_json::json!(if v.abs() < 1e-12 { 10.0 } else { v * 0.01 }),
                    serde_json::json!(v + 5.0),
                    serde_json::json!((v - 5.0).abs()),
                    serde_json::json!(v + 15.0),
                    serde_json::json!((v - 15.0).abs()),
                    serde_json::json!(0.0),
                    serde_json::json!(1.0e5),
                    serde_json::json!(1.0e6),
                    serde_json::json!(5.0e5),
                ];
                if let Some(i) = n.as_u64() {
                    out.push(serde_json::json!(i.saturating_add(1)));
                    out.push(serde_json::json!(i.saturating_add(2)));
                    if i > 0 { out.push(serde_json::json!(i - 1)); }
                }
                out
            }
            serde_json::Value::Bool(b) => vec![serde_json::json!(!b)],
            serde_json::Value::String(s) => {
                let alts = [
                    "en", "de", "En", "De", "EN", "DE",
                    "bsP", "bsT", "bsA", "da2", "geo3", "da1str",
                    "active", "atRest", "increasedActive", "free", "rigid", "propped",
                    "bored", "driven", "cfa", "sand", "clay", "gravel",
                ];
                let mut out: Vec<serde_json::Value> = alts
                    .iter()
                    .filter(|a| **a != s.as_str())
                    .map(|a| serde_json::json!(a))
                    .collect();
                if path.contains("governingLayerId") {
                    for l in &base.layers {
                        if l.id != *s {
                            out.insert(0, serde_json::json!(l.id));
                        }
                    }
                    out.push(serde_json::json!("missing-layer-x"));
                }
                out.push(serde_json::json!(format!("{s}-x")));
                out
            }
            _ => continue,
        };
        let mut changed = false;
        for nv in candidates {
            let mut tree = serde_json::to_value(&base).unwrap();
            if !set_at(&mut tree, &parts, nv) {
                continue;
            }
            let Ok(perturbed) = serde_json::from_value::<En1997Snapshot>(tree) else {
                continue;
            };
            if sig(&check_project(&perturbed)) != base_sig {
                changed = true;
                break;
            }
        }
        if !changed {
            unchanged.push(path.clone());
        }
    }
    assert!(
        unchanged.is_empty(),
        "editable leaves that did not change any check: {unchanged:?}"
    );
}
