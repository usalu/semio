//! Numeric worked examples + Wave D DoD tests for DIN 4108.

use crate::document::CheckStatus;
use crate::field_meta::din4108_field_meta;
use crate::standards::v1::subsets::any::schema::inferences::evaluate;
use crate::standards::v1::subsets::any::schema::snapshot::{encode_din4108_dsl};
use crate::standards::v1::subsets::any::schema::{
    layer_resistance, part_2, part_3, part_4, part_6, part_7, total_resistance, u_value, F_RSI_MINIMUM, R_SE, R_SI_WALL,
};
use crate::{Din4108Snapshot, EnvelopeElement, LayerDocument, LayerSegment};
use dsl::ToValue;
use std::path::PathBuf;
use std::process::Command;

fn family_any_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
}

fn ticket_generated() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS/🗑️generated/din4108")
}

#[semio_framework_async_macros::async_test]
async fn iso6946_u_for_etics_wall_matches_hand_calc() {
    let layers = [
        LayerDocument { id: "a".into(), material_id: "brick".into(), thickness_m: 0.24, lambda: 0.81, mu: 10.0, density: 1800.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
        LayerDocument { id: "b".into(), material_id: "eps".into(), thickness_m: 0.14, lambda: 0.035, mu: 40.0, density: 20.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
    ];
    let r = total_resistance(&layers, R_SI_WALL, R_SE);
    let u = u_value(r);
    assert!((r - 4.466296).abs() < 1e-4, "r={r}");
    assert!((u - 0.2239).abs() < 1e-3, "u={u}");
    let _ = layer_resistance(&layers[0]);
}

#[semio_framework_async_macros::async_test]
async fn table3_r_min_and_f_rsi_for_wall() {
    let snap = Din4108Snapshot::compliant_etics_dwelling();
    let wall = snap.elements.iter().find(|e| e.id == "wall-north").unwrap();
    let check = part_2::check_minimum_r(wall, &snap.usage, snap.t_int_c);
    assert_eq!(check.status, CheckStatus::Pass);
    let frsi = part_2::check_f_rsi(wall, 20.0, -14.0);
    assert!(frsi.computed.value >= F_RSI_MINIMUM - 1e-9);
    assert_eq!(frsi.status, CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn thin_insulation_fails_table3_and_frsi() {
    let snap = Din4108Snapshot::failing_thin_insulation();
    let wall = snap.elements.iter().find(|e| e.id == "wall-north").unwrap();
    let check = part_2::check_minimum_r(wall, &snap.usage, snap.t_int_c);
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(!check.remedies.is_empty());
    assert!(check.remedies[0].applicable);
    let frsi = part_2::check_f_rsi(wall, snap.t_int_c, snap.climate_zone.design_external_temperature_c());
    assert_eq!(
        frsi.status,
        CheckStatus::Fail,
        "failing example must fail f_Rsi, got {:?} f={}",
        frsi.status,
        frsi.computed.value
    );
}

#[semio_framework_async_macros::async_test]
async fn summer_heat_s_vorh_includes_orientation_and_inclination() {
    let snap = Din4108Snapshot::compliant_etics_dwelling();
    let zone = &snap.zones[0];
    let s = part_2::s_vorhanden(zone);
    assert!((s - 0.04075).abs() < 1e-6, "s={s}");
    let table = part_2::s_x_climate_construction_night('B', "heavy", "moderate");
    assert!((table - 0.10).abs() < 1e-12);
    let s_zul = part_2::s_zul_full('B', "heavy", "moderate", false, false);
    assert!((s_zul - table).abs() < 1e-12);
    let with_extras = part_2::s_zul_full('B', "heavy", "moderate", true, true);
    assert!((with_extras - (table + 0.03 + 0.04)).abs() < 1e-12);
    let check = part_2::check_summer_heat(zone, crate::document::ClimateZoneDe::Zone2, &snap.elements);
    assert_eq!(check.status, CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn airtightness_n50_limits() {
    assert!((part_7::n50_limit(true) - 1.5).abs() < 1e-12);
    assert!((part_7::n50_limit(false) - 3.0).abs() < 1e-12);
    assert_eq!(part_7::check_airtightness(4.5, false).status, CheckStatus::Fail);
    assert_eq!(part_7::check_airtightness(1.5, true).status, CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn magnus_saturation_pressure_at_20c() {
    let p = part_3::saturation_vapor_pressure_pa(20.0);
    assert!((p - 2338.0).abs() < 30.0, "p={p}");
}

#[semio_framework_async_macros::async_test]
async fn table3_light_construction_and_frame_kinds() {
    let light_wall = part_2::table3_r_min("wall", "exterior", "residential", 20.0, 80.0);
    let heavy_wall = part_2::table3_r_min("wall", "exterior", "residential", 20.0, 450.0);
    assert!((light_wall - 1.75).abs() < 1e-12, "light={light_wall}");
    assert!((heavy_wall - 1.2).abs() < 1e-12, "heavy={heavy_wall}");
    let low_temp = part_2::table3_r_min("wall", "exterior", "nonResidential", 15.0, 450.0);
    assert!((low_temp - 0.55).abs() < 1e-12, "low_temp={low_temp}");
    assert!((part_2::table3_r_min("frameOpaque", "exterior", "residential", 20.0, 450.0) - 1.0).abs() < 1e-12);
    assert!((part_2::table3_r_min("frameOpaque", "exterior", "residential", 20.0, 80.0) - 1.4).abs() < 1e-12);
    assert!((part_2::table3_r_min("rollerShutterBox", "exterior", "residential", 20.0, 450.0) - 1.0).abs() < 1e-12);
    let layers = vec![
        LayerDocument { id: "wood".into(), material_id: "softwood".into(), thickness_m: 0.12, lambda: 0.13, mu: 50.0, density: 500.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
        LayerDocument { id: "wool".into(), material_id: "mineral_wool".into(), thickness_m: 0.01, lambda: 0.035, mu: 1.0, density: 30.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
    ];
    let mass = part_2::surface_mass_kg_m2(&layers);
    assert!((mass - (0.12 * 500.0 + 0.01 * 30.0)).abs() < 1e-9);
    assert!(mass <= 100.0);
    let mut el = EnvelopeElement {
        id: "light-wall".into(),
        kind: "wall".into(),
        zone_id: "z".into(),
        orientation_deg: 0.0,
        inclination_deg: 90.0,
        adjacent: "exterior".into(),
        area_m2: 10.0,
        delta_u_g: 0.0,
        delta_u_f: 0.0,
        delta_u_r: 0.0,
        layers,
    };
    // R ≈ 0.13 + 0.923 + 0.286 + 0.04 ≈ 1.38 < light R_min 1.75
    assert_eq!(part_2::check_minimum_r(&el, "residential", 20.0).status, CheckStatus::Fail);
    el.layers[1].thickness_m = 0.08;
    let pass = part_2::check_minimum_r(&el, "residential", 20.0);
    assert_eq!(pass.status, CheckStatus::Pass, "util={}", pass.utilization);
}

#[semio_framework_async_macros::async_test]
async fn iso6946_67_timber_stud_upper_lower_bound_matches_hand_calc() {
    use crate::standards::v1::subsets::any::schema::{total_resistance, total_resistance_lower, total_resistance_upper, u_value, R_SE, R_SI_WALL};
    let layers = [
        LayerDocument { id: "plaster".into(), material_id: "gypsum".into(), thickness_m: 0.015, lambda: 0.70, mu: 10.0, density: 1400.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
        LayerDocument {
            id: "stud-bay".into(),
            material_id: "mineral_wool".into(),
            thickness_m: 0.16,
            lambda: 0.035,
            mu: 1.3,
            density: 30.0,
            application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![
                LayerSegment { id: "bay".into(), material_id: "mineral_wool".into(), fraction: 0.85, lambda: 0.035, mu: 1.3, density: 30.0 },
                LayerSegment { id: "stud".into(), material_id: "softwood".into(), fraction: 0.15, lambda: 0.13, mu: 50.0, density: 500.0 },
            ],
        },
        LayerDocument { id: "osb".into(), material_id: "osb".into(), thickness_m: 0.015, lambda: 0.13, mu: 50.0, density: 650.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
    ];
    let r_u = total_resistance_upper(&layers, R_SI_WALL, R_SE);
    let r_l = total_resistance_lower(&layers, R_SI_WALL, R_SE);
    let r = total_resistance(&layers, R_SI_WALL, R_SE);
    let u = u_value(r);
    assert!((r - 0.5 * (r_u + r_l)).abs() < 1e-12);
    assert!((r - 3.617367771).abs() < 1e-6, "r={r} upper={r_u} lower={r_l}");
    assert!((u - 0.276444).abs() < 1e-4, "u={u}");
    let snap = Din4108Snapshot::compliant_etics_dwelling().with_timber_frame_wall();
    let timber = snap.elements.iter().find(|e| e.id == "wall-timber-east").expect("timber wall");
    let check = part_6::check_u_value(timber, &snap.usage, snap.t_int_c);
    assert_eq!(check.clause.section.as_str(), "6.7");
    assert_eq!(check.status, CheckStatus::Pass);
}


#[semio_framework_async_macros::async_test]
async fn committed_demo_and_failing_assets_match_regenerated_dsl() {
    let demo_path = family_any_dir().join("🖼️assets/🎬️demo/🗣️.dsl.semio");
    let fail_path = family_any_dir().join("🖼️assets/🎬️failing-thin-insulation/🗣️.dsl.semio");
    let demo_committed = std::fs::read_to_string(&demo_path).expect("demo dsl");
    let fail_committed = std::fs::read_to_string(&fail_path).expect("failing dsl");
    let demo_regen = encode_din4108_dsl(&Din4108Snapshot::compliant_etics_dwelling());
    let fail_regen = encode_din4108_dsl(&Din4108Snapshot::failing_thin_insulation());
    assert_eq!(demo_committed, demo_regen, "demo DSL drifted from compliant_etics_dwelling()");
    assert_eq!(fail_committed, fail_regen, "failing DSL drifted from failing_thin_insulation()");
}

fn collect_leaf_paths(value: &dsl::DslValue, prefix: &str, out: &mut Vec<String>) {
    match value {
        dsl::DslValue::Object(fields) => {
            for (key, child) in fields {
                let next = if prefix.is_empty() { key.clone() } else { format!("{prefix}.{key}") };
                match child {
                    dsl::DslValue::Object(_) | dsl::DslValue::Array(_) => collect_leaf_paths(child, &next, out),
                    _ => out.push(next),
                }
            }
        }
        dsl::DslValue::Array(items) => {
            if items.is_empty() {
                out.push(format!("{prefix}[]"));
                return;
            }
            for (i, item) in items.iter().enumerate() {
                let selector = match item {
                    dsl::DslValue::Object(fields) => fields
                        .iter()
                        .find(|(k, _)| k == "id")
                        .and_then(|(_, v)| match v {
                            dsl::DslValue::String(s) => Some(format!("[id={s}]")),
                            _ => None,
                        })
                        .unwrap_or_else(|| format!("[{i}]")),
                    _ => format!("[{i}]"),
                };
                let next = format!("{prefix}{selector}");
                match item {
                    dsl::DslValue::Object(_) | dsl::DslValue::Array(_) => collect_leaf_paths(item, &next, out),
                    _ => out.push(next),
                }
            }
        }
        _ => out.push(prefix.to_string()),
    }
}

fn meta_path_for_leaf(path: &str) -> String {
    let mut out = String::new();
    let mut chars = path.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '[' {
            out.push_str("[]");
            while let Some(x) = chars.next() {
                if x == ']' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[semio_framework_async_macros::async_test]
async fn field_meta_covers_every_editable_leaf_on_default_snapshot() {
    let doc = Din4108Snapshot::default();
    let root = ToValue::to_value(&doc);
    let mut paths = Vec::new();
    collect_leaf_paths(&root, "", &mut paths);
    assert!(!paths.is_empty());
    for path in &paths {
        let wild = meta_path_for_leaf(path);
        let meta = din4108_field_meta(&wild)
            .or_else(|| din4108_field_meta(path))
            .unwrap_or_else(|| panic!("missing field meta for editable leaf '{path}' (wild='{wild}')"));
        assert!(!meta.label_en.trim().is_empty(), "{path} missing label_en");
        assert!(!meta.label_de.trim().is_empty(), "{path} missing label_de");
        let leaf = path.rsplit(['.', '[']).next().unwrap_or(path).trim_end_matches(']');
        let needs_unit = matches!(
            leaf,
            "tIntC"
                | "rhInt"
                | "airtightnessN50"
                | "floorAreaM2"
                | "inclinationDeg"
                | "areaM2"
                | "gValue"
                | "shadingFc"
                | "orientationDeg"
                | "deltaUG"
                | "deltaUF"
                | "deltaUR"
                | "thicknessM"
                | "lambda"
                | "mu"
                | "density"
                | "fraction"
                | "psi"
                | "lengthM"
        );
        if needs_unit {
            assert!(meta.unit.is_some(), "{path} requires SI unit");
        }
        if let Some(choices) = meta.choices {
            for choice in choices.iter() {
                assert!(!choice.label_en.is_empty() && !choice.label_de.is_empty(), "{path} choice missing labels");
                assert!(
                    choice.label_en != choice.value || choice.value.len() <= 2,
                    "{path} choice label_en must not be bare wire token '{}'",
                    choice.value
                );
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn every_emitted_subject_path_resolves_on_default_and_failing() {
    for (label, doc) in [("default", Din4108Snapshot::default()), ("failing", Din4108Snapshot::failing_thin_insulation())] {
        let report = evaluate(&doc);
        let root = ToValue::to_value(&doc);
        let mut seen = 0usize;
        for check in &report.checks {
            for path in std::iter::once(check.subject.path.as_str()).chain(check.remedies.iter().map(|r| r.target.path.as_str())) {
                if path.is_empty() {
                    continue;
                }
                crate::app_surface::parse_path(path).unwrap_or_else(|e| panic!("{label} parse '{path}': {e}"));
                crate::app_surface::get_value_at_path(&root, path).unwrap_or_else(|e| panic!("{label} resolve '{path}': {e}"));
                seen += 1;
                if path.contains('[') && !path.contains("[]") {
                    assert!(path.contains("[id="), "{label} entity path must use [id=…]: {path}");
                }
            }
        }
        assert!(seen > 0, "{label}: expected subject/remedy paths");
    }
}

fn apply_numeric_remedy(snap: &mut Din4108Snapshot, path: &str, value: f64) {
    let mut root = ToValue::to_value(&*snap);
    crate::app_surface::set_value_at_path(&mut root, path, dsl::DslValue::float(value)).unwrap_or_else(|e| panic!("set {path}: {e}"));
    *snap = dsl::FromValue::from_value(root).unwrap_or_else(|e| panic!("from_value: {e:?}"));
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_insulation_thickness_fixes_min_r() {
    let mut snap = Din4108Snapshot::failing_thin_insulation();
    let report = evaluate(&snap);
    let fail = report.failing().find(|c| c.id.contains("table3") && c.id.contains("wall-north")).expect("wall table3 fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable && r.target.path.contains("thicknessM")).expect("thickness remedy");
    assert!(remedy.target.path.contains("[id="), "path={}", remedy.target.path);
    apply_numeric_remedy(&mut snap, &remedy.target.path, remedy.required.value.max(remedy.current.value));
    let after = evaluate(&snap);
    let again = after.checks.iter().find(|c| c.id == fail.id).unwrap();
    assert_ne!(again.status, CheckStatus::Fail, "after remedy util={}", again.utilization);
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_airtightness_n50_fixes_n50() {
    let mut snap = Din4108Snapshot::failing_thin_insulation();
    let report = evaluate(&snap);
    let fail = report.failing().find(|c| c.id == "din4108-7.n50").expect("n50 fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable).expect("n50 remedy");
    assert_eq!(remedy.target.path, "airtightnessN50");
    apply_numeric_remedy(&mut snap, &remedy.target.path, remedy.required.value);
    let after = evaluate(&snap);
    let again = after.checks.iter().find(|c| c.id == fail.id).unwrap();
    assert_eq!(again.status, CheckStatus::Pass, "n50 remedy must flip to Pass");
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_shading_fc_fixes_summer() {
    let mut snap = Din4108Snapshot::failing_thin_insulation();
    let report = evaluate(&snap);
    let fail = report.failing().find(|c| c.id.contains("summer") && c.id.contains("zone-living")).expect("summer fail");
    // Apply every applicable numeric summer remedy (F_c then area) until Pass.
    for _ in 0..4 {
        let current = evaluate(&snap);
        let check = current.checks.iter().find(|c| c.id == fail.id).unwrap();
        if check.status != CheckStatus::Fail {
            break;
        }
        let remedy = check
            .remedies
            .iter()
            .find(|r| r.applicable && (r.target.path.contains("shadingFc") || r.target.path.contains("areaM2")))
            .expect("summer remedy");
        let path = remedy.target.path.clone();
        let required = remedy.required.value;
        apply_numeric_remedy(&mut snap, &path, required);
    }
    let after = evaluate(&snap);
    let again = after.checks.iter().find(|c| c.id == fail.id).unwrap();
    assert_ne!(again.status, CheckStatus::Fail, "summer remedies must clear Fail, got {:?} util={}", again.status, again.utilization);
}

#[semio_framework_async_macros::async_test]
async fn python_oracle_matches_rust_u_s_r_frsi_glaser_within_half_percent() {
    let tmp = ticket_generated();
    let _ = std::fs::create_dir_all(&tmp);
    let oracle = family_any_dir().join("🔮️oracles/🐍️.py");
    for (name, doc) in [("default", Din4108Snapshot::default()), ("failing", Din4108Snapshot::failing_thin_insulation())] {
        let report = evaluate(&doc);
        let snap_path = tmp.join(format!("{name}.snap.json"));
        let report_path = tmp.join(format!("{name}.report.json"));
        std::fs::write(&snap_path, serde_json::to_string_pretty(&doc).expect("snap")).unwrap();
        std::fs::write(&report_path, serde_json::to_string_pretty(&report).expect("report")).unwrap();
        let output = Command::new("python3")
            .arg(&oracle)
            .arg("--report")
            .arg(&snap_path)
            .stdin(std::fs::File::open(&report_path).unwrap())
            .output()
            .unwrap_or_else(|e| panic!("oracle spawn: {e}"));
        assert!(
            output.status.success(),
            "{name} oracle failed: {}",
            format!("{}{}", String::from_utf8_lossy(&output.stderr), String::from_utf8_lossy(&output.stdout))
        );
        let body: serde_json::Value = serde_json::from_slice(&output.stdout).expect("oracle json");
        assert_eq!(body["ok"], true, "{name} oracle body={body}");
    }
}

#[semio_framework_async_macros::async_test]
async fn jsonschema_validates_default_and_failing_snapshots() {
    let schema = family_any_dir().join("🧬️schema/📸️snapshot/🔣️.json");
    assert!(schema.exists(), "{}", schema.display());
    let tmp = ticket_generated();
    let _ = std::fs::create_dir_all(&tmp);
    let validate = family_any_dir().join("🧬️schema/🧪️tests/⚖️compliance/validate_snapshot.py");
    for (name, doc) in [("default", Din4108Snapshot::default()), ("failing", Din4108Snapshot::failing_thin_insulation())] {
        let snap = tmp.join(format!("{name}.schema.snap.json"));
        let json = serde_json::to_string_pretty(&doc).unwrap();
        std::fs::write(&snap, &json).unwrap();
        let output = Command::new("python3").arg(&validate).arg(&schema).arg(&snap).output().expect("jsonschema spawn");
        assert!(
            output.status.success(),
            "{name} jsonschema: {}",
            format!("{}{}", String::from_utf8_lossy(&output.stderr), String::from_utf8_lossy(&output.stdout))
        );
        let body: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(body["usage"], if name == "default" { "residential" } else { "residential" });
        assert!(body["elements"].as_array().unwrap().iter().any(|e| e["id"] == "wall-north"));
        if name == "failing" {
            assert!((body["airtightnessN50"].as_f64().unwrap() - 4.5).abs() < 1e-9);
        } else {
            assert!((body["airtightnessN50"].as_f64().unwrap() - 1.5).abs() < 1e-9);
        }
    }
}



fn check_signature(report: &crate::document::CheckReport) -> Vec<(String, String, u64, u64, u64)> {
    let mut rows: Vec<(String, String, u64, u64, u64)> = report
        .checks
        .iter()
        .map(|c| {
            (
                c.id.clone(),
                format!("{:?}", c.status),
                c.computed.value.to_bits(),
                c.limit.value.to_bits(),
                c.utilization.to_bits(),
            )
        })
        .collect();
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    rows
}

fn is_exempt_entity_id_leaf(path: &str) -> bool {
    let leaf = path.rsplit('.').next().unwrap_or(path);
    leaf == "id" || leaf.starts_with("id]")
}

fn perturb_json_leaf(value: &mut serde_json::Value, path: &str) -> bool {
    let wild = meta_path_for_leaf(path);
    let meta = din4108_field_meta(&wild).or_else(|| din4108_field_meta(path));
    let segs: Vec<&str> = path.split('.').collect();
    fn navigate<'a>(v: &'a mut serde_json::Value, segs: &[&str]) -> Option<&'a mut serde_json::Value> {
        if segs.is_empty() {
            return Some(v);
        }
        let head = segs[0];
        if let Some(bracket) = head.find('[') {
            let key = &head[..bracket];
            let inside = head[bracket + 1..].trim_end_matches(']');
            let arr = v.get_mut(key)?.as_array_mut()?;
            let child = if let Some(id) = inside.strip_prefix("id=") {
                arr.iter_mut().find(|x| x.get("id").and_then(|y| y.as_str()) == Some(id))?
            } else {
                let idx: usize = inside.parse().ok()?;
                arr.get_mut(idx)?
            };
            navigate(child, &segs[1..])
        } else {
            navigate(v.get_mut(head)?, &segs[1..])
        }
    }
    let Some(target) = navigate(value, &segs) else {
        return false;
    };
    match target {
        serde_json::Value::Bool(b) => {
            *b = !*b;
            true
        }
        serde_json::Value::Number(n) => {
            let x = n.as_f64().unwrap_or(0.0);
            *target = serde_json::json!(if (x - 0.0).abs() < f64::EPSILON { 1.0 } else { x * 1.10 });
            true
        }
        serde_json::Value::String(s) => {
            if let Some(m) = meta {
                if let Some(choices) = m.choices {
                    if let Some(alt) = choices.iter().map(|c| c.value).find(|v| *v != s.as_str()) {
                        *s = alt.to_string();
                        return true;
                    }
                }
            }
            s.push_str("-x");
            true
        }
        _ => false,
    }
}

#[semio_framework_async_macros::async_test]
async fn every_editable_leaf_perturbation_changes_some_check_on_default_snapshot() {
    // Exemptions (descriptive name/title/entity label leaves only): zones[].id, elements[].id,
    // layers[].id, windows[].id, thermalBridges[].id, segments[].id — used as report entity names.
    for (label, snap) in [
        ("default", Din4108Snapshot::compliant_etics_dwelling()),
        ("timber", Din4108Snapshot::compliant_etics_dwelling().with_timber_frame_wall()),
    ] {
        let baseline = evaluate(&snap);
        let baseline_sig = check_signature(&baseline);
        let root = ToValue::to_value(&snap);
        let mut paths = Vec::new();
        collect_leaf_paths(&root, "", &mut paths);
        let mut failures = Vec::new();
        for path in &paths {
            if is_exempt_entity_id_leaf(path) {
                continue;
            }
            let wild = meta_path_for_leaf(path);
            if din4108_field_meta(&wild).is_none() && din4108_field_meta(path).is_none() {
                continue;
            }
            let mut json = serde_json::to_value(&snap).expect("json");
            if !perturb_json_leaf(&mut json, path) {
                continue;
            }
            let Ok(clone) = serde_json::from_value::<Din4108Snapshot>(json) else {
                continue;
            };
            let report = evaluate(&clone);
            if check_signature(&report) == baseline_sig {
                failures.push(path.clone());
            }
        }
        assert!(
            failures.is_empty(),
            "{label}: editable leaves with no check status/computed change after perturbation: {failures:?}"
        );
    }
}

#[semio_framework_async_macros::async_test]
async fn flipping_bb2_type_on_default_bridge_changes_report() {
    let base = Din4108Snapshot::compliant_etics_dwelling();
    let before = evaluate(&base);
    let mut flipped = base.clone();
    flipped.thermal_bridges[0].bb2_type = "detailed".into();
    flipped.thermal_bridges[0].psi = 0.25;
    let after = evaluate(&flipped);
    assert_ne!(check_signature(&before), check_signature(&after));
}

#[semio_framework_async_macros::async_test]
async fn timber_segment_lambda_and_density_affect_checks() {
    let base = Din4108Snapshot::compliant_etics_dwelling().with_timber_frame_wall();
    let before = evaluate(&base);
    let mut lam = base.clone();
    let seg = lam
        .elements
        .iter_mut()
        .flat_map(|e| e.layers.iter_mut())
        .flat_map(|l| l.segments.iter_mut())
        .next()
        .expect("timber wall must have segments");
    seg.lambda *= 3.0;
    seg.density *= 0.5;
    seg.mu *= 2.0;
    let after = evaluate(&lam);
    assert_ne!(check_signature(&before), check_signature(&after));
}

#[semio_framework_async_macros::async_test]
async fn din4108_10_wrong_application_fails_and_remedy_passes() {
    let mut snap = Din4108Snapshot::compliant_etics_dwelling();
    let wall = snap.elements.iter_mut().find(|e| e.id == "wall-north").unwrap();
    let eps = wall.layers.iter_mut().find(|l| l.id == "eps").unwrap();
    eps.application_type = "DAD".into(); // roof type on exterior wall → unsuitable
    // keep compressive_class = dm (compliant for WAP once type is fixed)
    let report = evaluate(&snap);
    let fail = report
        .checks
        .iter()
        .find(|c| c.id.contains("din4108-10.app.wall-north.eps") && c.status == CheckStatus::Fail)
        .unwrap_or_else(|| {
            let ids: Vec<_> = report.checks.iter().map(|c| format!("{}:{:?}", c.id, c.status)).collect();
            panic!("expected DIN 4108-10 fail; checks={ids:?}");
        });
    let remedy = fail
        .remedies
        .iter()
        .find(|r| r.options.iter().any(|o| matches!(o.as_str(), "WAP" | "WAB" | "WAA" | "WH")))
        .expect("one_of application-type remedy");
    let choice = remedy.options.iter().find(|o| *o == "WAP").cloned().unwrap_or_else(|| remedy.options[0].clone());
    let wall = snap.elements.iter_mut().find(|e| e.id == "wall-north").unwrap();
    let eps = wall.layers.iter_mut().find(|l| l.id == "eps").unwrap();
    eps.application_type = choice;
    let fixed = evaluate(&snap);
    assert!(
        fixed.checks.iter().all(|c| !(c.id.contains("din4108-10.app.wall-north.eps") && c.status == CheckStatus::Fail)),
        "remedy application type should clear the 4108-10 fail"
    );
}

#[semio_framework_async_macros::async_test]
async fn oracle_manifest_mutation_vectors_match_kinds_len() {
    let text = std::fs::read_to_string(family_any_dir().join("🔮️oracles/🔣️.json")).expect("oracle manifest");
    let v: serde_json::Value = serde_json::from_str(&text).expect("json");
    let vectors = v["oracles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|o| o["id"] == "din4108-1-python-independent")
        .and_then(|o| o["nativeSecondImplementation"]["fixtureCoverage"]["vectors"].as_u64())
        .expect("vectors");
    assert_eq!(
        vectors as usize,
        crate::standards::v1::subsets::any::schema::mutations::KINDS.len(),
        "oracle fixtureCoverage.vectors must equal Din4108Mutation::KINDS len"
    );
}


#[semio_framework_async_macros::async_test]
async fn din4108_10_property_classes_affect_status_or_utilization() {
    let base = Din4108Snapshot::compliant_etics_dwelling();
    let before = evaluate(&base);
    let eps_path = |field: &str| {
        let mut s = base.clone();
        let layer = s.elements.iter_mut().flat_map(|e| e.layers.iter_mut()).find(|l| l.id == "eps").expect("eps");
        match field {
            "water" => layer.water_class = "wk".into(),
            "tensile" => layer.tensile_class = "tk".into(),
            "acoustic" => layer.acoustic_class = "sh".into(),
            _ => panic!("unknown"),
        }
        s
    };
    for field in ["water", "tensile", "acoustic"] {
        let after = evaluate(&eps_path(field));
        assert_ne!(
            check_signature(&before),
            check_signature(&after),
            "{field} class perturbation must change status/computed/limit/utilization"
        );
    }
}

#[semio_framework_async_macros::async_test]
async fn dangling_material_id_fails_referential_integrity() {
    let mut snap = Din4108Snapshot::compliant_etics_dwelling();
    snap.elements[0].layers[2].material_id = "not-in-catalogue".into();
    let report = evaluate(&snap);
    assert!(
        report.checks.iter().any(|c| c.id.contains("integrity") && c.id.contains("materialId") && c.status == crate::document::CheckStatus::Fail),
        "expected materialId integrity Fail, got {:?}",
        report.checks.iter().map(|c| (&c.id, &c.status)).collect::<Vec<_>>()
    );
}

#[semio_framework_async_macros::async_test]
async fn dangling_zone_id_fails_referential_integrity() {
    let mut snap = Din4108Snapshot::compliant_etics_dwelling();
    snap.elements[0].zone_id = "zone-missing".into();
    let report = evaluate(&snap);
    assert!(
        report.checks.iter().any(|c| c.id.contains("integrity") && c.id.contains("zoneId") && c.status == crate::document::CheckStatus::Fail),
        "expected zoneId integrity Fail"
    );
}

#[semio_framework_async_macros::async_test]
async fn duplicate_element_id_fails_integrity() {
    let mut snap = Din4108Snapshot::compliant_etics_dwelling();
    let dup = snap.elements[0].clone();
    snap.elements.push(dup);
    let report = evaluate(&snap);
    assert!(
        report.checks.iter().any(|c| c.id.contains("integrity.duplicate.elements") && c.status == crate::document::CheckStatus::Fail),
        "expected duplicate elements Fail"
    );
}

#[semio_framework_async_macros::async_test]
async fn moving_opaque_zone_id_changes_zone_transmission_loss() {
    let base = Din4108Snapshot::compliant_etics_dwelling();
    assert!(base.zones.len() >= 2, "need ≥2 zones");
    let before = evaluate(&base);
    let mut moved = base.clone();
    let other = moved.zones[1].id.clone();
    moved.elements[0].zone_id = other;
    let after = evaluate(&moved);
    let ht_before: Vec<_> = before.checks.iter().filter(|c| c.id.starts_with("din4108-2.zone-ht.")).map(|c| (c.id.clone(), c.computed.value.to_bits())).collect();
    let ht_after: Vec<_> = after.checks.iter().filter(|c| c.id.starts_with("din4108-2.zone-ht.")).map(|c| (c.id.clone(), c.computed.value.to_bits())).collect();
    assert_ne!(ht_before, ht_after, "zone H_T aggregation must change when opaque zoneId moves");
}

#[semio_framework_async_macros::async_test]
async fn glaser_climate_changes_condensation_or_limit() {
    let base = Din4108Snapshot::compliant_etics_dwelling();
    let before = evaluate(&base);
    let mut cold = base.clone();
    cold.climate_zone = crate::document::ClimateZoneDe::Zone1;
    let after = evaluate(&cold);
    let g = |r: &crate::document::CheckReport| {
        r.checks.iter().filter(|c| c.id.starts_with("din4108-3.glaser.")).map(|c| (c.computed.value.to_bits(), c.limit.value.to_bits())).collect::<Vec<_>>()
    };
    assert_ne!(g(&before), g(&after), "climate must change Glaser computed or limit");
}

#[test]
fn catalogue_design_lambda_matches_evaluated_limit() {
    let cell = part_4::design_lambda("eps").expect("eps λ");
    let snap = Din4108Snapshot::compliant_etics_dwelling();
    let element = snap.elements.iter().find(|e| e.layers.iter().any(|l| l.material_id == "eps")).expect("eps element");
    let li = element.layers.iter().position(|l| l.material_id == "eps").unwrap();
    let check = part_4::check_design_lambda(element, li);
    assert!((check.limit.value - cell).abs() < 1e-12, "evaluated λ limit {} vs catalogue const {}", check.limit.value, cell);
    // Same const the catalogue panel publishes as DESIGN_LAMBDA_ROWS.
    assert!(part_4::DESIGN_LAMBDA_ROWS.iter().any(|(id, v)| *id == "eps" && (*v - cell).abs() < 1e-12));
}

