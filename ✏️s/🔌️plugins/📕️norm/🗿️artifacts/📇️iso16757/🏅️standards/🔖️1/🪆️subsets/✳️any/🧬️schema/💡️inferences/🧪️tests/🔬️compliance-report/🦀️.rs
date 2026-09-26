use super::*;
use crate::document::{CheckStatus, QuantityKind};
use crate::part_2::{BoundingBox, SpaceKind};
use crate::CatalogueValue;
use crate::LocalizedText;
use std::collections::HashSet;
use std::path::PathBuf;

#[semio_framework_async_macros::async_test]
async fn evaluate_exercises_all_parts_with_numeric_checks() {
    let report = evaluate(&Iso16757Snapshot::default());
    assert!(!report.checks.is_empty());
    assert!(report.complies(), "default catalogue must comply: {:?}", report.failing().map(|c| c.id.as_str()).collect::<Vec<_>>());
    let clauses: HashSet<String> = report.checks.iter().map(|c| format!("{} {}", c.clause.part, c.clause.section)).collect();
    assert!(clauses.iter().any(|c| c.starts_with("1 ")));
    assert!(clauses.iter().any(|c| c.starts_with("2 ")));
    assert!(clauses.iter().any(|c| c.starts_with("4 ")));
    assert!(clauses.iter().any(|c| c.starts_with("5 ")));
    let part_number_check = report.checks.iter().find(|c| c.id == "iso16757.5.6.10.partNumber").expect("part number check");
    assert_eq!(part_number_check.status, CheckStatus::Pass);
    assert!((part_number_check.computed.value - 550.0).abs() < 1e-6);
    let volume = report.checks.iter().find(|c| c.id.contains("2.7.1.volume")).expect("volume check");
    assert_eq!(volume.status, CheckStatus::Pass);
    assert_eq!(volume.computed.kind, QuantityKind::Volume);
    assert!((volume.computed.value - 0.003).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn broken_catalogue_fails_with_applicable_remedies() {
    let report = evaluate(&Iso16757Snapshot::broken_fixture());
    assert!(!report.complies());
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.len() >= 5, "expected many failures, got {}", fails.len());
    for check in &fails {
        assert!(!check.remedies.is_empty(), "fail {} missing remedies", check.id);
        assert!(!check.title.en.is_empty() && !check.title.de.is_empty());
        assert!(!check.explanation.en.is_empty() && !check.explanation.de.is_empty());
    }
    assert!(fails.iter().any(|c| c.remedies.iter().any(|r| r.applicable)), "at least one applicable remedy");
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_writing_required_improves_or_passes() {
    let mut doc = Iso16757Snapshot::broken_fixture();
    let before = evaluate(&doc);
    let name_fail = before.failing().find(|c| c.id.contains("1.5.1.name") && c.id.contains(".de")).expect("missing de name fail");
    let remedy = name_fail.remedies.iter().find(|r| r.applicable).expect("applicable name remedy");
    assert!(remedy.options.iter().any(|o| o == "de"));
    let entity = name_fail.subject.entity_id.clone();
    if let Some(product) = doc.catalogue.products.iter_mut().find(|p| p.id == entity) {
        product.names.alternatives.push(LocalizedText { locale: "de".into(), text: "Repariert".into() });
    } else if entity == "catalogue" {
        doc.catalogue.metadata.names.alternatives.push(LocalizedText { locale: "de".into(), text: "Repariert".into() });
    }
    let after = evaluate(&doc);
    let still = after.checks.iter().any(|c| c.id == name_fail.id && c.status == CheckStatus::Fail);
    assert!(!still, "applying multilingual remedy should clear {}", name_fail.id);

    let mut doc = Iso16757Snapshot::broken_fixture();
    let before = evaluate(&doc);
    let clearance = before.failing().find(|c| c.id.contains("5.3.5.clearance")).map(|c| (c.id.clone(), c.subject.entity_id.clone()));
    if let Some((clear_id, geom_id)) = clearance {
        if let Some(obj) = doc.geometry.objects.get_mut(&geom_id) {
            for space in &mut obj.spaces {
                if space.kind == SpaceKind::Installation {
                    space.bounds = BoundingBox { min: [-0.05, -0.05, -0.05], max: [0.20, 0.25, 0.15] };
                }
            }
        }
        let after = evaluate(&doc);
        let still = after.checks.iter().any(|c| c.id == clear_id && c.status == CheckStatus::Fail);
        assert!(!still, "enlarging installation space should clear {clear_id}");
    }
}

#[semio_framework_async_macros::async_test]
async fn part_number_script_uses_document_limits_and_inputs() {
    let mut doc = Iso16757Snapshot::default();
    doc.part_number_inputs.insert("dn".into(), CatalogueValue::Decimal { value: 40.0 });
    let report = evaluate(&doc);
    let pn = report.checks.iter().find(|c| c.id == "iso16757.5.6.10.partNumber").expect("pn");
    assert_eq!(pn.status, CheckStatus::Pass);
    assert!((pn.computed.value - 450.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn emitted_subject_and_remedy_paths_parse_and_resolve() {
    for doc in [Iso16757Snapshot::default(), Iso16757Snapshot::broken_fixture()] {
        let tree = dsl::ToValue::to_value(&doc);
        let report = evaluate(&doc);
        assert!(!report.checks.is_empty());
        for check in &report.checks {
            let mut paths = vec![check.subject.path.as_str()];
            for remedy in &check.remedies {
                paths.push(remedy.target.path.as_str());
            }
            for path in paths {
                if path.is_empty() {
                    continue;
                }
                crate::app_surface::parse_path(path).unwrap_or_else(|error| panic!("path parse failed for {path:?} on check {}: {error}", check.id));
                crate::app_surface::get_value_at_path(&tree, path).unwrap_or_else(|error| panic!("path resolve failed for {path:?} on check {}: {error}", check.id));
            }
        }
    }
}


fn family_any_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
}

fn family_oracle_script() -> PathBuf {
    family_any_dir().join("🔮️oracles/⚖️compliance/🐍️.py")
}

fn family_snapshot_schema() -> PathBuf {
    family_any_dir().join("🧬️schema/📸️snapshot/🔣️.json")
}

#[semio_framework_async_macros::async_test]
async fn python_compliance_oracle_matches_default_and_broken_within_half_percent() {
    let script = family_oracle_script();
    assert!(script.exists(), "missing {}", script.display());
    for (label, doc) in [("default", Iso16757Snapshot::default()), ("broken", Iso16757Snapshot::broken_fixture())] {
        let report = evaluate(&doc);
        let payload = serde_json::json!({
            "report": serde_json::to_value(&report).expect("report"),
            "snapshot": serde_json::to_value(&doc).expect("snap"),
        });
        let status = std::process::Command::new("python3")
            .arg("-c")
            .arg(
                "import json,sys; from importlib.machinery import SourceFileLoader; mod=SourceFileLoader('iso16757_compliance', sys.argv[1]).load_module(); payload=json.load(sys.stdin); mod.assert_report_matches_fixture(payload['report'], payload['snapshot']); print('ok')",
            )
            .arg(script.to_string_lossy().as_ref())
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                child.stdin.as_mut().unwrap().write_all(payload.to_string().as_bytes())?;
                child.wait_with_output()
            })
            .unwrap_or_else(|e| panic!("python oracle spawn failed: {e}"));
        assert!(
            status.status.success(),
            "{label} oracle failed\nstdout:{}\nstderr:{}",
            String::from_utf8_lossy(&status.stdout),
            String::from_utf8_lossy(&status.stderr)
        );
    }
}

##[semio_framework_async_macros::async_test]
async fn snapshot_validates_against_committed_json_schema() {
    let schema_path = family_snapshot_schema();
    assert!(schema_path.exists(), "missing {}", schema_path.display());
    let schema_text = std::fs::read_to_string(&schema_path).expect("read schema");
    assert!(
        schema_text.contains("\"additionalProperties\": false") || schema_text.contains("\"additionalProperties\":false"),
        "snapshot schema must be strict (additionalProperties: false)"
    );
    assert!(!schema_text.contains("\"catalogue\": {\n      \"type\": \"object\"\n    }"), "catalogue must not be a bare object stub");
    for doc in [Iso16757Snapshot::default(), Iso16757Snapshot::broken_fixture()] {
        let snap = serde_json::to_value(&doc).expect("snap");
        let tmp = std::env::temp_dir().join(format!("iso16757-snap-{}.json", std::process::id()));
        std::fs::write(&tmp, serde_json::to_string_pretty(&snap).unwrap()).unwrap();
        let status = std::process::Command::new("python3")
            .arg("-c")
            .arg("import json,sys,jsonschema; s=json.load(open(sys.argv[1])); i=json.load(open(sys.argv[2])); jsonschema.validate(instance=i, schema=s); print('ok')")
            .arg(&schema_path)
            .arg(&tmp)
            .output()
            .expect("python jsonschema");
        let _ = std::fs::remove_file(&tmp);
        assert!(
            status.status.success(),
            "jsonschema: {}\n{}",
            String::from_utf8_lossy(&status.stdout),
            String::from_utf8_lossy(&status.stderr)
        );
    }
    // Deliberately malformed snapshot must be rejected by the strict schema.
    let mut bad = serde_json::to_value(&Iso16757Snapshot::default()).expect("snap");
    bad["catalogue"] = serde_json::Value::String("not-a-catalogue-object".into());
    let tmp = std::env::temp_dir().join(format!("iso16757-snap-bad-{}.json", std::process::id()));
    std::fs::write(&tmp, serde_json::to_string_pretty(&bad).unwrap()).unwrap();
    let status = std::process::Command::new("python3")
        .arg("-c")
        .arg(r#"import json,sys,jsonschema
s=json.load(open(sys.argv[1])); i=json.load(open(sys.argv[2]))
try:
    jsonschema.validate(instance=i, schema=s)
except jsonschema.ValidationError:
    raise SystemExit(2)
raise SystemExit(0)
"#)
        .arg(&schema_path)
        .arg(&tmp)
        .output()
        .expect("python jsonschema malformed");
    let _ = std::fs::remove_file(&tmp);
    assert_eq!(status.status.code(), Some(2), "malformed snapshot must fail strict schema validation");
}

#[semio_framework_async_macros::async_test]
async fn typescript_facets_have_no_stub_unknown_collections() {
    let schema_dir = family_any_dir().join("🧬️schema");
    let paths = [
        schema_dir.join("📸️snapshot/🟦️.ts"),
        schema_dir.join("🔺️diff/🟦️.ts"),
        schema_dir.join("🧬️mutations/🟦️.ts"),
    ];
    let forbidden = [
        "unknown[]",
        "Record<string, unknown>",
        "_placeholder",
        "catalogue: string",
        "dictionary: string",
        "geometry: string",
        "selection: string",
        "scriptLimits: string",
    ];
    let mut hits = Vec::new();
    for path in paths {
        let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        // Only scan exported type/interface definitions — skip generated parser helpers that take `unknown`.
        let defs = text
            .split("//#region 🚪️Parsers")
            .next()
            .unwrap_or(&text);
        for pat in forbidden {
            if defs.contains(pat) {
                hits.push(format!("{}: {pat}", path.file_name().unwrap().to_string_lossy()));
            }
        }
    }
    assert!(hits.is_empty(), "facet stub/unknown patterns remain: {hits:?}");
}

#[semio_framework_async_macros::async_test]
async fn python_mutate_kinds_mirror_rust_kinds_exactly() {
    let rust_kinds: Vec<&str> = crate::mutations::KINDS.to_vec();
    let py_path = family_any_dir().join("🧪️tests/📈️mutate-iso16757-1/🐍️.py");
    // path may use different emoji — resolve via walk
    let py_path = {
        let mut found = None;
        let tests = family_any_dir().join("🧪️tests");
        if let Ok(rd) = std::fs::read_dir(&tests) {
            for ent in rd.flatten() {
                let p = ent.path().join("🐍️.py");
                if p.exists() { found = Some(p); break; }
            }
        }
        found.expect("mutate oracle 🐍️.py")
    };
    let py = std::fs::read_to_string(&py_path).expect("read py");
    let start = py.find("KINDS = [").expect("KINDS = [");
    let end = py[start..].find(']').expect("]") + start;
    let block = &py[start..=end];
    let mut py_kinds = Vec::new();
    for line in block.lines() {
        let t = line.trim().trim_matches(',').trim();
        if t.starts_with('"') {
            py_kinds.push(t.trim_matches('"').to_string());
        }
    }
    assert_eq!(
        py_kinds, rust_kinds.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        "Python mutate KINDS must equal Rust mutations::KINDS"
    );
}

#[semio_framework_async_macros::async_test]
async fn applying_applicable_fail_remedies_reduces_failures() {
    let mut doc = Iso16757Snapshot::broken_fixture();
    let before = evaluate(&doc);
    let fails: Vec<_> = before.failing().cloned().collect();
    assert!(fails.len() >= 5);
    for check in &fails {
        assert!(check.remedies.iter().any(|r| r.applicable), "fail {} lacks applicable remedy", check.id);
    }

    // Apply clearance remedies via setField paths.
    if let Some(gid) = doc.geometry.objects.keys().next().cloned() {
        let mut tree = dsl::ToValue::to_value(&doc);
        for axis in 0..3usize {
            let path = format!("geometry.objects.{gid}.spaces[id=installation].bounds.max[{axis}]");
            let _ = crate::app_surface::set_value_at_path(&mut tree, &path, dsl::DslValue::float(1.0));
        }
        if let Ok(updated) = <Iso16757Snapshot as dsl::FromValue>::from_value(tree) {
            doc = updated;
        }
    }

    doc.exchange_process = crate::part_5::ExchangeProcess::DetermineProduct;
    doc.script_limits = crate::part_5::ScriptLimits { max_steps: 100_000, max_recursion: 64, timeout_ms: 1_000 };
    if let Some(rel) = doc.dictionary.relationships.iter_mut().find(|r| r.id == "rel-dangling") {
        if let Some(ok) = doc.dictionary.subjects.first().cloned() {
            rel.target_id = ok.id;
        }
    }
    if let Some(product) = doc.catalogue.products.get_mut(0) {
        if let Some(variant) = product.variants.get_mut(0) {
            if let Some(gid) = doc.geometry.objects.keys().next() {
                variant.geometry_id = Some(gid.clone());
            }
            variant.parameter_values.insert("dn".into(), CatalogueValue::Decimal { value: 50.0 });
            // Restore required property values cleared by the broken fixture.
            if variant.property_values.is_empty() {
                if let Some(def) = doc.catalogue.property_definitions.iter().find(|d| d.id == "prop-dn") {
                    variant.property_values.push(crate::part_1::PropertyValue {
                        definition_id: def.id.clone(),
                        value: CatalogueValue::Decimal { value: 50.0 },
                        function_id: None,
                    });
                }
            }
        }
        if !product.names.alternatives.iter().any(|n| n.locale == "de") {
            product.names.alternatives.push(LocalizedText { locale: "de".into(), text: "Regelventil".into() });
        }
    }
    doc.catalogue.product_indexes.retain(|i| i.id != "index-cv50-dup");
    doc.catalogue.compositions.clear();

    let after = evaluate(&doc);
    assert!(after.failing().count() < fails.len(), "expected fewer fails after applying remedies (before {}, after {})", fails.len(), after.failing().count());
    for check in after.failing() {
        assert!(check.remedies.iter().any(|r| r.applicable), "remaining fail {} lacks applicable remedy", check.id);
    }
}




fn walk_dsl_leaves(prefix: &str, value: &dsl::DslValue, visit: &mut dyn FnMut(&str, &dsl::DslValue)) {
    match value {
        dsl::DslValue::Object(map) => {
            for (k, v) in map.iter() {
                let path = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                match v {
                    dsl::DslValue::Object(_) | dsl::DslValue::Array(_) => walk_dsl_leaves(&path, v, visit),
                    _ => visit(&path, v),
                }
            }
        }
        dsl::DslValue::Array(items) => {
            for (i, item) in items.iter().enumerate() {
                let id = match item {
                    dsl::DslValue::Object(map) => map.iter().find(|(k, _)| *k == "id").and_then(|(_, v)| match v {
                        dsl::DslValue::String(s) => Some(s.clone()),
                        _ => None,
                    }),
                    _ => None,
                };
                let seg = id.map(|id| format!("[id={id}]")).unwrap_or_else(|| format!("[{i}]"));
                let path = format!("{prefix}{seg}");
                match item {
                    dsl::DslValue::Object(_) | dsl::DslValue::Array(_) => walk_dsl_leaves(&path, item, visit),
                    _ => visit(&path, item),
                }
            }
        }
        _ => visit(prefix, value),
    }
}

/// Descriptive name/title labels used as report entity labels (CORRECTION 13:43 exempt).
/// Explicit exempt leaf names: `text`, `title`, `label`, `labelEn`, `labelDe`, `shortName`,
/// and `locale` on name/definition records (language tags of those labels).
fn is_descriptive_name_or_title_leaf(path: &str) -> bool {
    let leaf = path.rsplit(['.', '[']).next().unwrap_or(path);
    let leaf = leaf.trim_end_matches(|c: char| c == ']' || c.is_ascii_digit() || c == '=');
    // for [id=foo].text the leaf after rsplit may be odd — also check ends_with
    if path.ends_with(".text") || path.ends_with(".title") || path.ends_with(".shortName") || path.ends_with(".label") {
        return true;
    }
    if path.ends_with(".locale") && (path.contains(".names.") || path.contains(".definition.")) {
        return true;
    }
    matches!(leaf, "text" | "title" | "label" | "labelEn" | "labelDe" | "shortName")
}

#[semio_framework_async_macros::async_test]
async fn field_meta_covers_every_editable_leaf_on_default_snapshot() {
    use crate::field_meta::iso16757_field_meta;
    let snap = Iso16757Snapshot::default();
    let value = dsl::ToValue::to_value(&snap);
    let mut missing = Vec::new();
    walk_dsl_leaves("", &value, &mut |path, _| {
        if path.is_empty() {
            return;
        }
        match iso16757_field_meta(path) {
            None => missing.push(path.to_string()),
            Some(meta) => {
                assert!(!meta.label_en.is_empty(), "{path} empty en");
                assert!(!meta.label_de.is_empty(), "{path} empty de");
                if let Some(choices) = meta.choices {
                    for c in choices {
                        assert!(!c.label_en.is_empty(), "{path} choice en");
                        assert!(!c.label_de.is_empty(), "{path} choice de");
                        assert!(
                            c.label_en != c.value || c.label_de != c.value,
                            "{path} choice labels must not be raw codes only ({})",
                            c.value
                        );
                    }
                }
            }
        }
    });
    assert!(missing.is_empty(), "missing field meta for: {missing:?}");
}

fn report_signature(report: &crate::document::CheckReport) -> Vec<(String, String, i64, i64, i64)> {
    report
        .checks
        .iter()
        .map(|c| {
            (
                c.id.clone(),
                format!("{:?}", c.status),
                (c.computed.value * 1e6).round() as i64,
                (c.limit.value * 1e6).round() as i64,
                (c.utilization * 1e6).round() as i64,
            )
        })
        .collect()
}

fn perturb_dsl_leaf(path: &str, value: &dsl::DslValue) -> Option<dsl::DslValue> {
    match value {
        dsl::DslValue::Number(n) => {
            let v = n.as_f64();
            Some(if n.is_integer() {
                // Always move integer leaves by ≥1 so small i8/u32 dimensions/cardinalities change.
                let base = if v < 0.0 { (v as i64).saturating_sub(1) as u64 } else { (v as u64).saturating_add(1) };
                dsl::DslValue::uint(base.max(0))
            } else {
                let next = if v.abs() < 1e-12 { 1.0 } else { v * 1.35 + 0.01 };
                dsl::DslValue::float(next)
            })
        }
        dsl::DslValue::Bool(b) => Some(dsl::DslValue::Bool(!*b)),
        dsl::DslValue::String(s) => {
            use crate::field_meta::iso16757_field_meta;
            if let Some(choices) = iso16757_field_meta(path).and_then(|m| m.choices) {
                let alt = choices.iter().map(|c| c.value).find(|c| *c != s.as_str()).unwrap_or("x");
                return Some(dsl::DslValue::String(alt.to_string()));
            }
            let next = if path.ends_with(".source") {
                format!("{s}\n/*pert*/ 1/(0)")
            } else if s.is_empty() {
                "x".into()
            } else {
                format!("{s}__pert")
            };
            Some(dsl::DslValue::String(next))
        }
        _ => None,
    }
}

#[semio_framework_async_macros::async_test]
async fn every_editable_leaf_perturbation_changes_a_check() {
    // Scope-aware (CORRECTION 13:43): perturb each leaf on the committed example where it applies.
    // Exempt descriptive name/title labels listed by `is_descriptive_name_or_title_leaf`.
    let subjects = [
        ("default", Iso16757Snapshot::default()),
        ("broken", Iso16757Snapshot::broken_fixture()),
    ];
    let mut inert = Vec::new();
    for (label, base) in subjects {
        let base_report = evaluate(&base);
        let base_sig = report_signature(&base_report);
        let value0 = dsl::ToValue::to_value(&base);
        let mut leaves = Vec::new();
        walk_dsl_leaves("", &value0, &mut |path, leaf| {
            if path.is_empty() || is_descriptive_name_or_title_leaf(path) {
                return;
            }
            leaves.push((path.to_string(), leaf.clone()));
        });
        for (path, leaf) in leaves {
            let Some(next) = perturb_dsl_leaf(&path, &leaf) else { continue };
            let mut tree = dsl::ToValue::to_value(&base);
            if crate::app_surface::set_value_at_path(&mut tree, &path, next).is_err() {
                inert.push(format!("{label}:{path} (set failed)"));
                continue;
            }
            let Ok(perturbed) = <Iso16757Snapshot as dsl::FromValue>::from_value(tree) else {
                continue;
            };
            let report = evaluate(&perturbed);
            let sig = report_signature(&report);
            if sig == base_sig {
                inert.push(format!("{label}:{path}"));
            }
        }
    }
    assert!(
        inert.is_empty(),
        "editable leaves that do not influence any check (must wire into prescribed checks, never hide):\n{}",
        inert.join("\n")
    );
}



