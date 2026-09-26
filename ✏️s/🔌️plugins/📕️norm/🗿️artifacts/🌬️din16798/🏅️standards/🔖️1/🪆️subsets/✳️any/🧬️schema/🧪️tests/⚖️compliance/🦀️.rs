//! ⚖️ DIN EN 16798 compliance numeric worked examples + Wave D DoD tests.

use crate::artifact_schema::part_1::{self, ComfortCategory, PollutionClass};
use crate::artifact_schema::part_3;
use crate::artifact_schema::{check_full_environment, parse_category, ComfortModel, VentMethod, VentSystemType};
use crate::document::{AnnexChoice, CheckStatus};
use crate::field_meta::din16798_field_meta;
use crate::Din16798Snapshot;
use dsl::ToValue;
use std::path::PathBuf;
use std::process::Command;

fn family_any_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
}

fn ticket_generated() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS/🗑️generated/din16798",
    )
}

#[test]
fn office_ventilation_table_b6_b7_ida_cat_ii_low_pollution() {
    // 20 persons × 7 l/s + 200 m² × 0.7 l/(s·m²) = 280 l/s = 1008 m³/h
    let q = part_1::required_outdoor_air_m3_h(20, 200.0, ComfortCategory::II, PollutionClass::Low);
    assert!((q - 1008.0).abs() < 1e-6, "q={q}");
}

#[test]
fn pmv_iso7730_near_neutral_at_office_defaults() {
    let pmv = part_1::pmv_iso7730(24.5, 45.0, 0.1, 1.2, 0.5);
    assert!(pmv.abs() < 0.5, "pmv={pmv}");
}

#[test]
fn ppd_from_pmv_zero_is_five_percent() {
    let ppd = part_1::ppd_from_pmv(0.0);
    assert!((ppd - 5.0).abs() < 0.1, "ppd={ppd}");
}

#[test]
fn sfp_class_3_bound_is_1250() {
    assert_eq!(part_3::sfp_bound(3), 1250.0);
}

#[test]
fn heat_recovery_min_for_mechanical_systems() {
    assert_eq!(part_3::heat_recovery_eta_min(VentSystemType::CentralMech.as_str()), Some(0.73));
    assert_eq!(part_3::heat_recovery_eta_min(VentSystemType::DecentralMech.as_str()), Some(0.73));
    assert_eq!(part_3::heat_recovery_eta_min(VentSystemType::Natural.as_str()), None);
}

#[test]
fn de_annex_co2_absolute_tighter_than_en() {
    use crate::artifact_schema::annex_params::AnnexParams;
    let de = AnnexParams::de();
    let en = AnnexParams::en();
    assert!(de.co2_absolute_other_ppm < en.co2_absolute_other_ppm);
    assert!(de.acoustic_limit_residential_db < en.acoustic_limit_residential_db);
}

#[test]
fn category_iv_parses() {
    assert_eq!(parse_category("IV"), ComfortCategory::IV);
}

#[test]
fn compliant_office_complies() {
    let report = check_full_environment(&Din16798Snapshot::compliant_office());
    assert!(!report.checks.is_empty());
    assert!(report.complies(), "fails: {:?}", report.failing().map(|c| &c.id).collect::<Vec<_>>());
}

#[test]
fn noncompliant_office_has_multiple_fails_with_remedies() {
    let report = check_full_environment(&Din16798Snapshot::noncompliant_office());
    assert!(!report.complies());
    let fails: Vec<_> = report.checks.iter().filter(|c| c.status == CheckStatus::Fail).collect();
    assert!(fails.len() >= 5, "fails={:?}", fails.iter().map(|c| &c.id).collect::<Vec<_>>());
    assert!(fails.iter().all(|c| !c.remedies.is_empty()), "every Fail needs ≥1 remedy");
    assert!(fails.iter().all(|c| c.remedies.iter().any(|r| r.applicable)), "every Fail needs an applicable remedy");
}

#[test]
fn de_annex_diverges_from_en_on_same_subject() {
    let mut de = Din16798Snapshot::compliant_office();
    de.annex = AnnexChoice::De;
    let mut en = de.clone();
    en.annex = AnnexChoice::En;
    de.zones[0].co2_ppm = 1150.0;
    en.zones[0].co2_ppm = 1150.0;
    let r_de = check_full_environment(&de);
    let r_en = check_full_environment(&en);
    let de_co2 = r_de.checks.iter().find(|c| c.id.contains("co2")).unwrap();
    let en_co2 = r_en.checks.iter().find(|c| c.id.contains("co2")).unwrap();
    assert_ne!(de_co2.status, en_co2.status, "DE absolute CO₂ cap must diverge from EN at 1150 ppm");
}

#[test]
fn adaptive_model_runs_annex_b2_and_skips_pmv_ppd() {
    let mut doc = Din16798Snapshot::compliant_office();
    doc.zones[0].comfort_model = ComfortModel::Adaptive.as_str().into();
    doc.theta_rm_c = 15.0;
    doc.zones[0].t_op_summer_c = 24.5; // |24.5 - 23.75| = 0.75 < Cat II band 3 K
    let report = check_full_environment(&doc);
    let adaptive = report.checks.iter().find(|c| c.id.contains("adaptive")).expect("adaptive check");
    assert_ne!(adaptive.status, CheckStatus::NotApplicable, "adaptive must run when comfortModel=adaptive");
    assert_eq!(adaptive.status, CheckStatus::Pass, "Cat II band should pass at θ_rm=15");
    for id_part in ["pmv.summer", "ppd.summer", "pmv.winter", "ppd.winter"] {
        let c = report.checks.iter().find(|c| c.id.contains(id_part)).unwrap_or_else(|| panic!("missing {id_part}"));
        assert_eq!(c.status, CheckStatus::NotApplicable, "{id_part} must be N/A under adaptive");
    }
}

#[test]
fn decentral_mech_runs_heat_recovery_with_eta_min_0_73() {
    let mut doc = Din16798Snapshot::compliant_office();
    doc.vent_systems[0].system_type = VentSystemType::DecentralMech.as_str().into();
    doc.vent_systems[0].heat_recovery_eta = 0.80;
    let report = check_full_environment(&doc);
    let hr = report.checks.iter().find(|c| c.id.contains("din16798-3.hr")).expect("HR check");
    assert_ne!(hr.status, CheckStatus::NotApplicable, "decentral_mech must require heat recovery");
    assert!((hr.limit.value - 0.73).abs() < 1e-9, "η_min must be 0.73, got {}", hr.limit.value);
    assert_eq!(hr.status, CheckStatus::Pass);
}

#[test]
fn every_emitted_subject_path_parses_and_resolves_on_default_and_noncompliant() {
    for (label, doc) in [("compliant", Din16798Snapshot::compliant_office()), ("noncompliant", Din16798Snapshot::noncompliant_office())] {
        let report = check_full_environment(&doc);
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
                if path.contains("zones[") {
                    assert!(path.contains("zones[id="), "{label} zone path must use [id=…]: {path}");
                }
                if path.contains("ventSystems[") {
                    assert!(path.contains("ventSystems[id="), "{label} vent path must use [id=…]: {path}");
                }
            }
        }
        assert!(seen > 0, "{label}: expected subject/remedy paths");
    }
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
            // Prefer id selectors when present
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

#[test]
fn default_snapshot_editable_leaves_have_en_de_field_meta() {
    let doc = Din16798Snapshot::default();
    let root = ToValue::to_value(&doc);
    let mut paths = Vec::new();
    collect_leaf_paths(&root, "", &mut paths);
    assert!(!paths.is_empty());
    // Also assert wildcard coverage for list containers
    for wild in ["zones[]", "ventSystems[]"] {
        let meta = din16798_field_meta(wild.trim_end_matches("[]")).or_else(|| din16798_field_meta(wild));
        let _ = meta; // container meta via exact("zones") / exact("ventSystems")
    }
    assert!(din16798_field_meta("zones").is_some());
    assert!(din16798_field_meta("ventSystems").is_some());
    for path in &paths {
        let meta = din16798_field_meta(path).unwrap_or_else(|| panic!("missing field meta for editable leaf '{path}'"));
        assert!(!meta.label_en.trim().is_empty(), "{path} missing label_en");
        assert!(!meta.label_de.trim().is_empty(), "{path} missing label_de");
        // Quantity-ish leaves must declare a unit
        let leaf = path.rsplit(['.', '[']).next().unwrap_or(path).trim_end_matches(']');
        let needs_unit = matches!(
            leaf,
            "thetaRmC"
                | "outdoorCo2Ppm"
                | "envelopeN50HInv"
                | "envelopeVolumeM3"
                | "cellarAreaM2"
                | "cellarVentilationM3H"
                | "nightSetbackK"
                | "floorAreaM2"
                | "tOpWinterC"
                | "tOpSummerC"
                | "airSpeedMS"
                | "clothingClo"
                | "metabolicRateMet"
                | "rhPercent"
                | "outdoorAirSuppliedM3H"
                | "co2Ppm"
                | "illuminanceLx"
                | "noiseDb"
                | "sfpWM3S"
                | "heatRecoveryEta"
                | "yearsSinceInspection"
                | "humidificationRequiredKgH"
                | "humidificationProvidedKgH"
                | "fanQVM3S"
                | "fanTRunH"
                | "ductTestPressurePa"
                | "ductLeakageM3SM2"
                | "designAirflowM3H"
                | "turbulenceIntensityPercent"
        );
        if needs_unit {
            assert!(meta.unit.is_some(), "{path} requires a unit");
        }
        if let Some(choices) = meta.choices {
            for choice in choices.iter() {
                assert!(!choice.label_en.is_empty() && !choice.label_de.is_empty(), "{path} choice missing labels");
                assert!(choice.label_en != choice.value || choice.value.len() <= 2, "{path} choice label_en must not be bare wire token '{}'", choice.value);
            }
        }
    }
}

fn apply_required_numeric(doc: &mut Din16798Snapshot, path: &str, value: f64) {
    if let Some(rest) = path.strip_prefix("zones[id=").and_then(|s| s.split_once(']')).map(|(id, rest)| (id, rest.trim_start_matches('.'))) {
        let (id, field) = rest;
        let zone = doc.zones.iter_mut().find(|z| z.id == id).expect("zone");
        match field {
            "outdoorAirSuppliedM3H" => zone.outdoor_air_supplied_m3_h = value,
            "tOpSummerC" => zone.t_op_summer_c = value,
            "tOpWinterC" => zone.t_op_winter_c = value,
            "co2Ppm" => zone.co2_ppm = value,
            "rhPercent" => zone.rh_percent = value,
            "illuminanceLx" => zone.illuminance_lx = value,
            "noiseDb" => zone.noise_db = value,
            other => panic!("unsupported zone field {other}"),
        }
        return;
    }
    if let Some(rest) = path.strip_prefix("ventSystems[id=").and_then(|s| s.split_once(']')).map(|(id, rest)| (id, rest.trim_start_matches('.'))) {
        let (id, field) = rest;
        let vent = doc.vent_systems.iter_mut().find(|v| v.id == id).expect("vent");
        match field {
            "sfpWM3S" => vent.sfp_w_m3_s = value,
            "heatRecoveryEta" => vent.heat_recovery_eta = value,
            "yearsSinceInspection" => vent.years_since_inspection = value as u32,
            "ductLeakageM3SM2" => vent.duct_leakage_m3_s_m2 = value,
            other => panic!("unsupported vent field {other}"),
        }
        return;
    }
    match path {
        "envelopeN50HInv" => doc.envelope_n50_h_inv = value,
        "cellarVentilationM3H" => doc.cellar_ventilation_m3_h = value,
        "nightSetbackK" => doc.night_setback_k = value,
        other => panic!("unsupported path {other}"),
    }
}

#[test]
fn apply_remedy_flips_vent_fail_to_pass() {
    let mut doc = Din16798Snapshot::noncompliant_office();
    let before = check_full_environment(&doc);
    let check = before.checks.iter().find(|c| c.id.starts_with("din16798-1.vent.") && c.status == CheckStatus::Fail).expect("vent fail");
    let remedy = check.remedies.iter().find(|r| r.applicable).expect("applicable vent remedy");
    apply_required_numeric(&mut doc, &remedy.target.path, remedy.required.value);
    let after = check_full_environment(&doc);
    let again = after.checks.iter().find(|c| c.id == check.id).expect("vent check after");
    assert_eq!(again.status, CheckStatus::Pass, "vent remedy must flip to Pass, got {:?} u={}", again.status, again.utilization);
}

#[test]
fn apply_remedy_flips_sfp_fail_to_pass() {
    let mut doc = Din16798Snapshot::noncompliant_office();
    let before = check_full_environment(&doc);
    let check = before.checks.iter().find(|c| c.id.starts_with("din16798-3.sfp.") && c.status == CheckStatus::Fail).expect("sfp fail");
    let remedy = check.remedies.iter().find(|r| r.applicable).expect("applicable sfp remedy");
    apply_required_numeric(&mut doc, &remedy.target.path, remedy.required.value);
    let after = check_full_environment(&doc);
    let again = after.checks.iter().find(|c| c.id == check.id).expect("sfp check after");
    assert_eq!(again.status, CheckStatus::Pass, "sfp remedy must flip to Pass");
}

#[test]
fn apply_remedy_flips_filter_fail_to_pass_via_oneof() {
    let mut doc = Din16798Snapshot::noncompliant_office();
    let before = check_full_environment(&doc);
    let check = before.checks.iter().find(|c| c.id.starts_with("din16798-3.filter.") && c.status == CheckStatus::Fail).expect("filter fail");
    let remedy = check.remedies.iter().find(|r| r.applicable && matches!(r.bound, crate::document::RemedyBound::OneOf)).expect("applicable OneOf filter remedy");
    let required = remedy.options.first().expect("filter option").clone();
    let path = &remedy.target.path;
    let vent_id = path.strip_prefix("ventSystems[id=").and_then(|s| s.split_once(']')).map(|(id, _)| id).expect("vent id");
    let vent = doc.vent_systems.iter_mut().find(|v| v.id == vent_id).unwrap();
    vent.filter_sup_class = required;
    let after = check_full_environment(&doc);
    let again = after.checks.iter().find(|c| c.id == check.id).unwrap();
    assert_eq!(again.status, CheckStatus::Pass, "filter OneOf remedy must flip to Pass");
}

#[test]
fn python_oracle_matches_rust_q_sfp_co2_within_half_percent() {
    let tmp = ticket_generated();
    let _ = std::fs::create_dir_all(&tmp);
    for (name, doc) in [("compliant", Din16798Snapshot::compliant_office()), ("noncompliant", Din16798Snapshot::noncompliant_office())] {
        let report = check_full_environment(&doc);
        let snap_path = tmp.join(format!("{name}.snap.json"));
        let report_path = tmp.join(format!("{name}.report.json"));
        std::fs::write(&snap_path, serde_json::to_string_pretty(&doc).expect("snap json")).unwrap();
        std::fs::write(&report_path, serde_json::to_string_pretty(&report).expect("report json")).unwrap();
        let oracle = family_any_dir().join("🧬️schema/🧪️tests/⚖️compliance/🐍️.py");
        let output = Command::new("python3")
            .arg(&oracle)
            .arg("--report")
            .arg(&snap_path)
            .stdin(std::fs::File::open(&report_path).unwrap())
            .output()
            .unwrap_or_else(|e| panic!("oracle spawn: {e}"));
        assert!(
            output.status.success(),
            "{name} oracle failed: {}{}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
        let body: serde_json::Value = serde_json::from_slice(&output.stdout).expect("oracle json");
        assert_eq!(body["ok"], true, "{name} oracle body={body}");
    }
}

#[test]
fn jsonschema_validates_compliant_and_noncompliant_snapshots() {
    let schema = family_any_dir().join("🧬️schema/📸️snapshot/🔣️.json");
    assert!(schema.exists(), "{}", schema.display());
    let tmp = ticket_generated();
    let _ = std::fs::create_dir_all(&tmp);
    let validate = family_any_dir().join("🔮️oracles/🧬️snapshot-schema/🐍️.py");
    for (name, doc) in [("compliant", Din16798Snapshot::compliant_office()), ("noncompliant", Din16798Snapshot::noncompliant_office())] {
        let snap = tmp.join(format!("{name}.schema.snap.json"));
        std::fs::write(&snap, serde_json::to_string_pretty(&doc).unwrap()).unwrap();
        let output = Command::new("python3").arg(&validate).arg(&schema).arg(&snap).output().expect("jsonschema spawn");
        assert!(
            output.status.success(),
            "{name} jsonschema: {}{}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

#[test]
fn bundled_dsl_examples_decode_and_evaluate() {
    let demo = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let failing = include_str!("../../../🖼️assets/⚠️noncompliant-office/🗣️.dsl.semio");
    let compliant = crate::artifact_schema::snapshot::decode_din16798_dsl(demo).expect("demo dsl");
    let noncompliant = crate::artifact_schema::snapshot::decode_din16798_dsl(failing).expect("failing dsl");
    assert!(check_full_environment(&compliant).complies());
    let fails = check_full_environment(&noncompliant).failing().count();
    assert!(fails >= 2, "noncompliant DSL must fail ≥2 checks, got {fails}");
}

#[test]
fn winter_and_summer_operative_bands_both_evaluated() {
    let report = check_full_environment(&Din16798Snapshot::compliant_office());
    assert!(report.checks.iter().any(|c| c.id.contains("top.summer")));
    assert!(report.checks.iter().any(|c| c.id.contains("top.winter")));
    assert!(report.checks.iter().any(|c| c.id.contains("pmv.summer")));
    assert!(report.checks.iter().any(|c| c.id.contains("pmv.winter")));
}


#[test]
fn two_vent_relink_changes_capacity_on_both_systems() {
    let mut doc = Din16798Snapshot::compliant_office();
    let mut v2 = crate::VentSystemDocument::default();
    v2.id = "vent-other".into();
    v2.name = "Secondary AHU".into();
    v2.design_airflow_m3_h = 200.0;
    v2.fan_q_v_m3_s = 200.0 / 3600.0;
    doc.vent_systems.push(v2);
    let before = check_full_environment(&doc);
    let cap_central = before.checks.iter().find(|c| c.id == "din16798-3.capacity.vent-central").unwrap().clone();
    let cap_other = before.checks.iter().find(|c| c.id == "din16798-3.capacity.vent-other").unwrap().clone();
    assert_eq!(cap_other.status, CheckStatus::NotApplicable, "unlinked secondary must be N/A");
    assert!(cap_central.computed.value > 0.0);
    doc.zones[0].vent_system_id = "vent-other".into();
    let after = check_full_environment(&doc);
    let cap_central2 = after.checks.iter().find(|c| c.id == "din16798-3.capacity.vent-central").unwrap();
    let cap_other2 = after.checks.iter().find(|c| c.id == "din16798-3.capacity.vent-other").unwrap();
    assert_eq!(cap_central2.status, CheckStatus::NotApplicable, "central becomes unlinked N/A");
    assert_ne!(cap_other2.status, CheckStatus::NotApplicable, "secondary now serves the zone");
    assert_ne!(cap_other2.computed.value, cap_other.computed.value);
    assert_ne!(cap_central2.status, cap_central.status);
}

#[test]
fn dangling_vent_system_id_fail_apply_remedy_to_pass() {
    let mut doc = Din16798Snapshot::compliant_office();
    doc.zones[0].vent_system_id = "vent-missing".into();
    let before = check_full_environment(&doc);
    let check = before
        .checks
        .iter()
        .find(|c| c.id.starts_with("din16798.zone.ventSystem.") && c.status == CheckStatus::Fail)
        .expect("dangling link Fail");
    let remedy = check.remedies.iter().find(|r| r.applicable).expect("applicable OneOf");
    assert!(!remedy.options.is_empty());
    doc.zones[0].vent_system_id = remedy.options[0].clone();
    let after = check_full_environment(&doc);
    let again = after.checks.iter().find(|c| c.id == check.id).unwrap();
    assert_eq!(again.status, CheckStatus::Pass);
}

#[test]
fn duplicate_zone_id_fails_integrity() {
    let mut doc = Din16798Snapshot::compliant_office();
    let mut twin = doc.zones[0].clone();
    twin.name = "Clone office".into();
    doc.zones.push(twin);
    let report = check_full_environment(&doc);
    let check = report
        .checks
        .iter()
        .find(|c| c.id == "din16798.integrity.duplicate.zones.zone-office")
        .expect("duplicate zone id check");
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(check.subject.path.contains("zones[id=zone-office].id"));
    let remedy = check.remedies.iter().find(|r| r.applicable).expect("applicable OneOf");
    assert!(matches!(remedy.bound, crate::document::RemedyBound::OneOf));
    assert!(!remedy.options.is_empty());
    assert!(!remedy.options.iter().any(|o| o == "zone-office"));
    assert_ne!(check.explanation.en, check.explanation.de);
}

#[test]
fn duplicate_vent_system_id_fails_integrity() {
    let mut doc = Din16798Snapshot::compliant_office();
    let mut twin = doc.vent_systems[0].clone();
    twin.name = "Clone AHU".into();
    doc.vent_systems.push(twin);
    let report = check_full_environment(&doc);
    let check = report
        .checks
        .iter()
        .find(|c| c.id == "din16798.integrity.duplicate.ventSystems.vent-central")
        .expect("duplicate vent system id check");
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(check.subject.path.contains("ventSystems[id=vent-central].id"));
    let remedy = check.remedies.iter().find(|r| r.applicable).expect("applicable OneOf");
    assert!(matches!(remedy.bound, crate::document::RemedyBound::OneOf));
    assert!(!remedy.options.is_empty());
    assert!(!remedy.options.iter().any(|o| o == "vent-central"));
    assert_ne!(check.explanation.en, check.explanation.de);
}

#[test]
fn residential_method3_uses_predefined_rates() {
    let doc = Din16798Snapshot::residential_method3();
    assert_eq!(VentMethod::parse(&doc.zones[0].vent_method), VentMethod::Method3PredefinedRates);
    let report = check_full_environment(&doc);
    let vent = report.checks.iter().find(|c| c.id.contains("din16798-1.vent.")).unwrap();
    assert!((vent.limit.value - 363.6).abs() < 0.5, "method3 limit={}", vent.limit.value);
    assert!(report.complies(), "fails: {:?}", report.failing().map(|c| &c.id).collect::<Vec<_>>());
}

#[test]
fn draught_rate_respects_category_limit() {
    let dr = crate::artifact_schema::part_1::draught_rate_percent(24.5, 0.1, 40.0);
    let lim = crate::artifact_schema::part_1::draught_limit_percent(crate::artifact_schema::part_1::ComfortCategory::II);
    assert!(dr < lim, "DR={dr} lim={lim}");
    let mut doc = Din16798Snapshot::compliant_office();
    doc.zones[0].air_speed_m_s = 0.35;
    let report = check_full_environment(&doc);
    let d = report.checks.iter().find(|c| c.id.contains("draught")).unwrap();
    assert_eq!(d.status, CheckStatus::Fail);
    assert!(d.remedies.iter().any(|r| r.applicable && r.target.path.contains("airSpeedMS")));
}

fn report_fingerprint(report: &crate::document::CheckReport) -> Vec<(String, CheckStatus, u64, u64, u64)> {
    report
        .checks
        .iter()
        .map(|c| (c.id.clone(), c.status, c.computed.value.to_bits(), c.limit.value.to_bits(), c.utilization.to_bits()))
        .collect()
}

fn perturb_leaf_value(doc: &mut Din16798Snapshot, path: &str) {
    let mut tree = ToValue::to_value(&*doc);
    let cur = crate::app_surface::get_value_at_path(&tree, path).expect("get").clone();
    let next = match cur {
        dsl::DslValue::Bool(b) => dsl::DslValue::Bool(!b),
        dsl::DslValue::Number(n) => {
            let leaf = path.rsplit(['.', '[']).next().unwrap_or(path).trim_end_matches(']');
            let int_leaf = matches!(
                leaf,
                "occupants" | "yearsSinceInspection" | "sfpRequiredClass" | "index"
            );
            match n {
                dsl::Number::Int(i) => {
                    let bumped = if i == 0 { 1 } else { i + 1 };
                    dsl::DslValue::Number(dsl::Number::Int(bumped))
                }
                dsl::Number::UInt(u) => {
                    let bumped = if u == 0 { 1 } else { u + 1 };
                    dsl::DslValue::Number(dsl::Number::UInt(bumped))
                }
                dsl::Number::Float(f) => {
                    if int_leaf {
                        let base = f.round() as i64;
                        let bumped = if base == 0 { 1 } else { base + 1 };
                        dsl::DslValue::Number(dsl::Number::Int(bumped))
                    } else {
                        let bumped = if path.contains("Percent") || path.ends_with("rhPercent") {
                            (f + 8.0).clamp(5.0, 95.0)
                        } else if f.abs() < 1e-9 {
                            1.0
                        } else {
                            f * 1.2 + 1.0
                        };
                        dsl::DslValue::float(bumped)
                    }
                }
            }
        }
        dsl::DslValue::String(s) => {
            let leaf = path.rsplit(['.', '[']).next().unwrap_or(path).trim_end_matches(']');
            let alt: String = match leaf {
                "annex" => if s.eq_ignore_ascii_case("de") { "En" } else { "De" }.into(),
                "usageType" => "classroom".into(),
                "comfortCategory" => "III".into(),
                "pollutionClass" => "non_low".into(),
                "comfortModel" => if s == "adaptive" { "fixed_hvac" } else { "adaptive" }.into(),
                "ventMethod" => if s.contains("method_3") {
                    "method_1_perceived_air_quality".into()
                } else {
                    "method_3_predefined_rates".into()
                },
                "systemType" => if s == "natural" { "central_mech" } else { "natural" }.into(),
                "odaClass" => match s.as_str() {
                    "ODA1" => "ODA4".into(),
                    "ODA4" => "ODA2".into(),
                    _ => "ODA1".into(),
                },
                "filterSupClass" => "ePM1_80".into(),
                "ductClass" => if s == "A" { "D" } else { "A" }.into(),
                "ventSystemId" => if s == "vent-central" { "vent-other" } else { "vent-central" }.into(),
                _ => format!("{s}-x"),
            };
            dsl::DslValue::String(alt)
        }
        other => panic!("unsupported leaf value at {path}: {other:?}"),
    };
    crate::app_surface::set_value_at_path(&mut tree, path, next).expect("set");
    *doc = <Din16798Snapshot as dsl::FromValue>::from_value(tree).expect("from_value");
}


fn zone_id_from_leaf_path(path: &str) -> Option<&str> {
    let start = path.find("zones[id=")?;
    let rest = &path[start + "zones[id=".len()..];
    let end = rest.find(']')?;
    Some(&rest[..end])
}

fn leaf_applies_in_example(doc: &Din16798Snapshot, path: &str, leaf: &str) -> bool {
    match leaf {
        "thetaRmC" => doc
            .zones
            .iter()
            .any(|z| ComfortModel::parse(&z.comfort_model) == ComfortModel::Adaptive),
        "clothingClo" => {
            let Some(id) = zone_id_from_leaf_path(path) else {
                return true;
            };
            doc.zones
                .iter()
                .find(|z| z.id == id)
                .map(|z| ComfortModel::parse(&z.comfort_model) == ComfortModel::FixedHvac)
                .unwrap_or(true)
        }
        "metabolicRateMet" => {
            let Some(id) = zone_id_from_leaf_path(path) else {
                return true;
            };
            doc.zones
                .iter()
                .find(|z| z.id == id)
                .map(|z| {
                    ComfortModel::parse(&z.comfort_model) == ComfortModel::FixedHvac
                        || VentMethod::parse(&z.vent_method) == VentMethod::Method2LimitConcentration
                })
                .unwrap_or(true)
        }
        "cellarVentilationM3H" => doc.cellar_area_m2 > 0.0,
        _ => true,
    }
}

#[test]
fn editable_leaves_perturb_at_least_one_check_across_examples() {
    const EXEMPT_LEAVES: &[&str] = &["id", "name"];
    let mut compliant = Din16798Snapshot::compliant_office();
    let mut v2 = crate::VentSystemDocument::default();
    v2.id = "vent-other".into();
    v2.name = "Secondary AHU".into();
    v2.design_airflow_m3_h = 500.0;
    v2.fan_q_v_m3_s = 500.0 / 3600.0;
    compliant.vent_systems.push(v2);

    let mut adaptive = Din16798Snapshot::compliant_office();
    adaptive.zones[0].comfort_model = "adaptive".into();
    adaptive.theta_rm_c = 18.0;

    let mut seen_theta_rm = false;
    let mut seen_clothing = false;
    let mut seen_cellar_vent = false;

    for (label, base) in [
        ("compliant-two-vent", compliant),
        ("noncompliant", Din16798Snapshot::noncompliant_office()),
        ("residential-method3", Din16798Snapshot::residential_method3()),
        ("adaptive-office", adaptive),
    ] {
        let root = ToValue::to_value(&base);
        let mut paths = Vec::new();
        collect_leaf_paths(&root, "", &mut paths);
        let baseline = report_fingerprint(&check_full_environment(&base));
        assert!(!paths.is_empty(), "{label}: no leaves");
        for path in &paths {
            let leaf = path.rsplit(['.', '[']).next().unwrap_or(path).trim_end_matches(']');
            if EXEMPT_LEAVES.contains(&leaf) {
                continue;
            }
            if !leaf_applies_in_example(&base, path, leaf) {
                continue;
            }
            let mut doc = base.clone();
            perturb_leaf_value(&mut doc, path);
            let after = report_fingerprint(&check_full_environment(&doc));
            assert_ne!(
                after, baseline,
                "{label}: perturbing editable leaf '{path}' must change ≥1 check computed/limit/utilization/status"
            );
            match leaf {
                "thetaRmC" => seen_theta_rm = true,
                "clothingClo" => seen_clothing = true,
                "cellarVentilationM3H" => seen_cellar_vent = true,
                _ => {}
            }
        }
    }
    assert!(seen_theta_rm, "thetaRmC must be perturbed in an adaptive example");
    assert!(seen_clothing, "clothingClo must be perturbed under fixed_hvac");
    assert!(seen_cellar_vent, "cellarVentilationM3H must be perturbed when cellarAreaM2>0");
}

#[test]
fn oda4_requires_stricter_filter_than_oda3() {
    let (_, rank3) = part_3::required_filter_for_oda("ODA3");
    let (filt4, rank4) = part_3::required_filter_for_oda("ODA4");
    assert!(rank4 > rank3, "ODA4 rank {rank4} must exceed ODA3 rank {rank3}");
    assert_eq!(filt4, "ePM1_80_G");
    assert_eq!(part_3::filter_rank(filt4), rank4);
    let meta = din16798_field_meta("odaClass").expect("odaClass meta");
    let choices = meta.choices.expect("odaClass choices");
    assert!(choices.iter().any(|c| c.value == "ODA4"), "ODA4 must be a field-meta choice");
}

#[test]
fn sfp_catalogue_class3_matches_sfp_bound_and_check_limit() {
    let tables = crate::editor::din16798::panels::catalogue::reference_tables();
    let sfp = tables.iter().find(|t| t.id == "sfp-class-bounds").expect("sfp table");
    let row = sfp.rows.iter().find(|r| r.id == "3").expect("class 3 row");
    let cell = match &row.cells[1] {
        crate::app_surface::CatalogueCell::Number { value, .. } => *value,
        other => panic!("expected number cell, got {other:?}"),
    };
    let bound = part_3::sfp_bound(3);
    assert_eq!(cell, bound, "catalogue SFP class-3 cell must equal sfp_bound(3)");
    let mut doc = Din16798Snapshot::compliant_office();
    doc.vent_systems[0].sfp_required_class = 3;
    doc.vent_systems[0].sfp_w_m3_s = bound - 50.0;
    let report = check_full_environment(&doc);
    let check = report
        .checks
        .iter()
        .find(|c| c.id.contains("sfp") && c.id.contains(&doc.vent_systems[0].id))
        .expect("sfp check");
    assert!(
        (check.limit.value - bound).abs() < 1e-9,
        "SFP check limit {} must equal sfp_bound(3)={}",
        check.limit.value,
        bound
    );
}

#[test]
fn schema_ts_facets_forbid_unknown_record_and_placeholder() {
    let root = family_any_dir().join("🧬️schema");
    for rel in ["🟦️.ts", "📸️snapshot/🟦️.ts"] {
        let text = std::fs::read_to_string(root.join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"));
        assert!(!text.contains("unknown[]"), "{rel} must not use unknown[]");
        assert!(!text.contains("Record<string, unknown>"), "{rel} must not use Record<string, unknown>");
        assert!(!text.contains("_placeholder"), "{rel} must not use _placeholder");
    }
    fn scan_ts(dir: &std::path::Path) {
        let Ok(rd) = std::fs::read_dir(dir) else { return };
        for entry in rd.flatten() {
            let p = entry.path();
            if p.is_dir() {
                scan_ts(&p);
                continue;
            }
            if p.extension().and_then(|e| e.to_str()) != Some("ts") {
                continue;
            }
            let text = std::fs::read_to_string(&p).unwrap();
            assert!(
                !text.contains("Record<string, unknown>"),
                "{} must not export Record<string, unknown>",
                p.display()
            );
            assert!(!text.contains("_placeholder"), "{} must not use _placeholder", p.display());
        }
    }
    scan_ts(&root);
}
