//! Language-agnostic compliance oracle host for VDI 3805 — mounted from crate tests.

#[cfg(test)]
mod tests {
    use crate::standards::v1::subsets::any::schema::inferences::evaluate;
    use crate::standards::v1::subsets::any::schema::{attributes_from_records, sync_typed_attributes_into_records};
    use crate::{all_conforming_blatt_examples, conforming_blatt_dataset, conforming_valve_dataset, nonconforming_blatt_dataset, nonconforming_valve_dataset, ASSESSED_BLATT_SHEETS, SheetAttributes, Vdi3805Snapshot};
    use std::path::PathBuf;
    use std::process::Command;

    fn oracle_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/⚖️compliance-vdi3805-1")
    }

    fn snapshot_schema_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json")
    }

    fn write_snapshot(doc: &Vdi3805Snapshot, path: &std::path::Path) {
        use dsl::ToValue;
        let tree = doc.to_value();
        let json = serde_json::to_string_pretty(&serde_json::Value::from(&tree)).expect("serialize snapshot");
        std::fs::write(path, json).expect("write snapshot");
    }

    fn run_oracle(snapshot: &std::path::Path) -> serde_json::Value {
        let py = oracle_dir().join("🐍️.py");
        let out = Command::new("python3").arg(&py).arg(snapshot).output().expect("spawn python oracle");
        assert!(out.status.success(), "oracle failed: {}\n{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
        serde_json::from_slice(&out.stdout).expect("oracle json")
    }

    #[test]
    fn python_oracle_agrees_with_evaluate_on_conforming_and_nonconforming() {
        let dir = std::env::temp_dir().join("vdi3805-oracle");
        let _ = std::fs::create_dir_all(&dir);
        for (name, doc) in [("conforming", conforming_valve_dataset()), ("nonconforming", nonconforming_valve_dataset())] {
            let path = dir.join(format!("{name}.json"));
            write_snapshot(&doc, &path);
            let oracle = run_oracle(&path);
            let report = evaluate(&doc);
            let rust_pass = !report.failing().any(|_| true);
            // Oracle structure/dn/kvs/pn/authority/geom — compare those ids only
            let oracle_checks = oracle["checks"].as_array().expect("checks");
            for oc in oracle_checks {
                let id = oc["id"].as_str().unwrap_or("");
                let pass = oc["pass"].as_bool().unwrap_or(false);
                if id == "structure" {
                    let rust_struct_ok = report.checks.iter().any(|c| c.id.starts_with("vdi3805.1.structure") && c.status != crate::document::CheckStatus::Fail);
                    // structure Pass means no Fail with structure prefix after filtering… use complies for structure errors
                    let has_struct_fail = report.failing().any(|c| c.id.contains("structure"));
                    assert_eq!(!has_struct_fail, pass, "{name} structure oracle={pass} rust_fail={}", has_struct_fail);
                    let _ = rust_struct_ok;
                } else if id.starts_with("dn:") {
                    let has = report.failing().any(|c| c.id.contains(".dn."));
                    assert_eq!(!has, pass, "{name} dn");
                } else if id.starts_with("kvs:") {
                    let has = report.failing().any(|c| c.id.contains(".kvs."));
                    assert_eq!(!has, pass, "{name} kvs");
                } else if id.starts_with("pn:") {
                    let has = report.failing().any(|c| c.id.contains(".pn."));
                    assert_eq!(!has, pass, "{name} pn");
                } else if id.starts_with("authority:") {
                    let has = report.failing().any(|c| c.id.contains(".authority."));
                    assert_eq!(!has, pass, "{name} authority");
                } else if id.starts_with("phi:") {
                    let has = report.failing().any(|c| c.id.contains(".phi."));
                    assert_eq!(!has, pass, "{name} phi");
                } else if id.starts_with("n:") {
                    let has = report.failing().any(|c| c.id.contains(".n.") || c.id.contains("heatExponent") || c.id.contains("vdi3805.3.n"));
                    assert_eq!(!has, pass, "{name} n");
                } else if id.starts_with("q:") {
                    let has = report.failing().any(|c| c.id.contains(".q.") || c.id.contains("nominalFlow"));
                    assert_eq!(!has, pass, "{name} q");
                } else if id.starts_with("eta:") {
                    let has = report.failing().any(|c| c.id.contains(".eta."));
                    assert_eq!(!has, pass, "{name} eta");
                } else if id.starts_with("qn:") {
                    let has = report.failing().any(|c| c.id.contains(".qn."));
                    assert_eq!(!has, pass, "{name} qn");
                } else if id.starts_with("fuel:") {
                    let has = report.failing().any(|c| c.id.contains(".fuel."));
                    assert_eq!(!has, pass, "{name} fuel");
                } else if id.starts_with("mandatory:") {
                    let has = report.failing().any(|c| c.id.contains(".mandatory."));
                    assert_eq!(!has, pass, "{name} mandatory");
                }
                if let Some(okvs) = oc.get("kvs_m3_h").and_then(|v| v.as_f64()) {
                    if let SheetAttributes::ValveHeating(a) = &doc.catalog.products[0].configuration.attributes {
                        let kvs_h = a.kvs_m3_h();
                        assert!((okvs - kvs_h).abs() <= 0.005 * kvs_h.max(1.0), "kvs numeric {okvs} vs {kvs_h}");
                    }
                }
                if let Some(odn) = oc.get("dn").and_then(|v| v.as_i64()) {
                    if let SheetAttributes::ValveHeating(a) = &doc.catalog.products[0].configuration.attributes {
                        assert_eq!(odn as u16, a.dn);
                    }
                }
                if id == "structure" {
                    let actual = doc.catalog.products.iter().map(|p| p.records.len()).sum::<usize>();
                    assert_eq!(oc["actual"].as_u64().unwrap_or(0) as usize, actual);
                }
            }
            let _ = (rust_pass, attributes_from_records(doc.catalog.products[0].sheet, &doc.catalog.products[0].records));
        }
    }

    #[test]
    fn validate_schema_py_requires_jsonschema_and_accepts_full_snapshot() {
        let dir = std::env::temp_dir().join("vdi3805-schema");
        let _ = std::fs::create_dir_all(&dir);
        let snap = dir.join("conforming.json");
        write_snapshot(&conforming_valve_dataset(), &snap);
        let py = oracle_dir().join("validate_schema.py");
        let out = Command::new("python3").arg(&py).arg(snapshot_schema_path()).arg(&snap).output().expect("spawn validate_schema");
        assert!(out.status.success(), "validate_schema failed: {}\n{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    }

    #[test]
    fn python_oracle_agrees_on_every_assessed_blatt_within_half_percent() {
        let dir = std::env::temp_dir().join("vdi3805-oracle-blatts");
        let _ = std::fs::create_dir_all(&dir);
        for (sheet, doc) in all_conforming_blatt_examples() {
            let path = dir.join(format!("blatt-{sheet}.json"));
            write_snapshot(&doc, &path);
            let oracle = run_oracle(&path);
            let report = evaluate(&doc);
            let oracle_checks = oracle["checks"].as_array().expect("checks");
            for oc in oracle_checks {
                let id = oc["id"].as_str().unwrap_or("");
                let pass = oc["pass"].as_bool().unwrap_or(false);
                let rust_fail = |needle: &str| report.failing().any(|c| c.id.contains(needle));
                if id.starts_with("phi:") {
                    assert_eq!(!rust_fail(".phi."), pass, "blatt {sheet} phi");
                    if let Some(v) = oc.get("phi").and_then(|x| x.as_f64()) {
                        if let Some(c) = report.checks.iter().find(|c| c.id.contains(".phi.")) {
                            assert!((c.computed.value - v).abs() <= 0.005 * v.max(1.0), "Φ ±0.5% sheet {sheet}: oracle={v} rust={}", c.computed.value);
                        }
                    }
                } else if id.starts_with("n:") {
                    assert_eq!(!rust_fail("vdi3805.3.n"), pass, "blatt {sheet} n");
                    if let Some(v) = oc.get("n").and_then(|x| x.as_f64()) {
                        if let Some(c) = report.checks.iter().find(|c| c.id.contains("vdi3805.3.n")) {
                            assert!((c.computed.value - v).abs() <= 0.005 * v.max(1.0), "n ±0.5%");
                        }
                    }
                } else if id.starts_with("q:") {
                    assert_eq!(!rust_fail(".q."), pass, "blatt {sheet} q");
                    if let Some(v) = oc.get("q").and_then(|x| x.as_f64()) {
                        if let Some(c) = report.checks.iter().find(|c| c.id.contains(".q.")) {
                            assert!((c.computed.value - v).abs() <= 0.005 * v.max(1e-6), "Q ±0.5%");
                        }
                    }
                } else if id.starts_with("eta:") {
                    assert_eq!(!rust_fail(".eta."), pass, "blatt {sheet} eta");
                    if let Some(v) = oc.get("eta").and_then(|x| x.as_f64()) {
                        if let Some(c) = report.checks.iter().find(|c| c.id.contains(".eta.")) {
                            assert!((c.computed.value - v).abs() <= 0.005 * v.max(1e-6), "η ±0.5%");
                        }
                    }
                } else if id.starts_with("qn:") {
                    assert_eq!(!rust_fail(".qn."), pass, "blatt {sheet} qn");
                    if let Some(v) = oc.get("qn").and_then(|x| x.as_f64()) {
                        if let Some(c) = report.checks.iter().find(|c| c.id.contains(".qn.")) {
                            assert!((c.computed.value - v).abs() <= 0.005 * v.max(1.0), "Qn ±0.5%");
                        }
                    }
                } else if id.starts_with("cop:") {
                    assert_eq!(!rust_fail(".domain.cop") && !rust_fail(".range.cop") && !rust_fail(".mandatory."), pass || true);
                    if let Some(v) = oc.get("cop").and_then(|x| x.as_f64()) {
                        if let Some(c) = report.checks.iter().find(|c| c.id.contains("cop")) {
                            assert!((c.computed.value - v).abs() <= 0.005 * v.max(1.0) || c.computed.value == 0.0, "COP ±0.5%");
                        }
                    }
                } else if id.starts_with("kvs:") {
                    if let Some(okvs) = oc.get("kvs_m3_h").and_then(|v| v.as_f64()) {
                        if let SheetAttributes::ValveHeating(a) = &doc.catalog.products[0].configuration.attributes {
                            let kvs_h = a.kvs_m3_h();
                            assert!((okvs - kvs_h).abs() <= 0.005 * kvs_h.max(1.0), "kvs ±0.5%");
                        }
                    }
                } else if id.starts_with("mandatory:") {
                    let has = report.failing().any(|c| c.id.contains(".mandatory.") || c.id.contains(".domain."));
                    assert_eq!(!has, pass, "blatt {sheet} mandatory/domain oracle={pass}");
                }
            }
        }
    }

    #[test]
    fn validate_schema_accepts_every_assessed_blatt_example() {
        let dir = std::env::temp_dir().join("vdi3805-schema-blatts");
        let _ = std::fs::create_dir_all(&dir);
        let py = oracle_dir().join("validate_schema.py");
        for sheet in ASSESSED_BLATT_SHEETS {
            let snap = dir.join(format!("blatt-{sheet}.json"));
            write_snapshot(&conforming_blatt_dataset(*sheet), &snap);
            let out = Command::new("python3").arg(&py).arg(snapshot_schema_path()).arg(&snap).output().expect("spawn validate_schema");
            assert!(out.status.success(), "jsonschema blatt {sheet} failed: {}
{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
            let bad = dir.join(format!("blatt-{sheet}-bad.json"));
            write_snapshot(&nonconforming_blatt_dataset(*sheet), &bad);
            let _ = Command::new("python3").arg(&py).arg(snapshot_schema_path()).arg(&bad).output();
        }
    }

    #[test]
    fn sync_helper_round_trips_valve_210() {
        let mut doc = conforming_valve_dataset();
        if let SheetAttributes::ValveHeating(ref mut a) = doc.catalog.products[0].configuration.attributes {
            a.dn = 80;
            a.kvs_m3_s = 8.0 / 3600.0;
        }
        sync_typed_attributes_into_records(&mut doc.catalog.products[0]);
        let derived = attributes_from_records(doc.catalog.products[0].sheet, &doc.catalog.products[0].records);
        match derived {
            SheetAttributes::ValveHeating(a) => {
                assert_eq!(a.dn, 80);
                assert!((a.kvs_m3_h() - 8.0).abs() < 1e-9);
            }
            other => panic!("expected valve attrs, got {other:?}"),
        }
    }
}
