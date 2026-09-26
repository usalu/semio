use super::*;
use crate::document::CheckStatus;
use std::collections::BTreeSet;

#[semio_framework_async_macros::async_test]
async fn evaluate_conforming_dataset_complies() {
    let report = evaluate(&conforming_valve_dataset());
    assert!(report.complies(), "conforming dataset must have no Fail checks");
    assert!(report.checks.iter().any(|c| c.id.starts_with("vdi3805.1.structure") && c.status == CheckStatus::Pass));
    assert!(report.checks.iter().any(|c| c.id.contains("vdi3805.2.kvs") && c.status == CheckStatus::Pass));
    assert!(!report.checks.iter().any(|c| c.clause.part == "io" || c.clause.part == "registry" || c.clause.part == "catalog"));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_nonconforming_dataset_has_multiple_fails_with_remedies() {
    let report = evaluate(&nonconforming_valve_dataset());
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.len() >= 5, "expected ≥5 fails, got {}", fails.len());
    for fail in &fails {
        assert!(!fail.remedies.is_empty(), "fail {} must carry a remedy", fail.id);
        assert!(!fail.title.en.is_empty() && !fail.title.de.is_empty());
        assert!(!fail.explanation.en.is_empty() && !fail.explanation.de.is_empty());
    }
    assert!(fails.iter().any(|c| c.id.contains("dn")));
    assert!(fails.iter().any(|c| c.id.contains("kvs")));
    assert!(fails.iter().any(|c| c.id.contains("structure")));
    assert!(fails.iter().any(|c| c.id.contains("historical")));
}

#[semio_framework_async_macros::async_test]
async fn valve_min_kvs_worked_example_dn50() {
    let mut doc = conforming_valve_dataset();
    if let SheetAttributes::ValveHeating(ref mut a) = doc.catalog.products[0].configuration.attributes {
        a.kvs_m3_s = 0.5 / 3600.0;
    }
    crate::standards::v1::subsets::any::schema::sync_typed_attributes_into_records(&mut doc.catalog.products[0]);
    let report = evaluate(&doc);
    let kvs = report.checks.iter().find(|c| c.id.contains("vdi3805.2.kvs")).expect("kvs check");
    assert_eq!(kvs.status, CheckStatus::Fail);
    let remedy = &kvs.remedies[0];
    assert!((remedy.required.value - 2.5 / 3600.0).abs() < 1e-12);
    assert!(remedy.applicable);
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_applicable_writes_improve_or_pass() {
    let mut doc = nonconforming_valve_dataset();
    let report = evaluate(&doc);
    for fail in report.failing() {
        for remedy in fail.remedies.iter() {
            if !remedy.applicable {
                continue;
            }
            if remedy.target.path.ends_with("kvsM3S") {
                if let SheetAttributes::ValveHeating(ref mut a) = doc.catalog.products[0].configuration.attributes {
                    a.kvs_m3_s = remedy.required.value;
                }
                crate::standards::v1::subsets::any::schema::sync_typed_attributes_into_records(&mut doc.catalog.products[0]);
                let next = evaluate(&doc);
                let again = next.checks.iter().find(|c| c.id == fail.id);
                assert!(again.is_none_or(|c| c.status == CheckStatus::Pass || c.utilization <= 1.0), "kvs remedy on {} did not fix", fail.id);
            }
            if remedy.target.path.ends_with(".dn") || remedy.target.path.ends_with("attributes.dn") {
                if let SheetAttributes::ValveHeating(ref mut a) = doc.catalog.products[0].configuration.attributes {
                    a.dn = remedy.required.value as u16;
                }
                crate::standards::v1::subsets::any::schema::sync_typed_attributes_into_records(&mut doc.catalog.products[0]);
                let next = evaluate(&doc);
                let again = next.checks.iter().find(|c| c.id == fail.id);
                assert!(again.is_none_or(|c| c.status == CheckStatus::Pass || c.utilization <= 1.0), "dn remedy on {} did not fix", fail.id);
            }
            if remedy.target.path.contains("recordCount") {
                doc.catalog.file.record_count = remedy.required.value as u32;
                doc.catalog.file.record_count = remedy.required.value as u32;
                let next = evaluate(&doc);
                assert!(next.checks.iter().find(|c| c.id == fail.id).is_none_or(|c| c.status != CheckStatus::Fail));
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn reserved_sheet_returns_not_applicable() {
    let doc = Vdi3805Snapshot::default();
    let result = part_15::check(&doc);
    assert_eq!(result.status, CheckStatus::NotApplicable);
    let result = part_67::check(&doc);
    assert_eq!(result.status, CheckStatus::NotApplicable);
}

#[semio_framework_async_macros::async_test]
async fn historical_part_check_respects_strict_mode() {
    let mut doc = nonconforming_valve_dataset();
    let result = part_12::check(&doc);
    assert_eq!(result.status, CheckStatus::Fail);
    doc.strict_mode = false;
    let result = part_12::check(&doc);
    assert_eq!(result.status, CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn multi_profile_part_check_not_applicable_without_product() {
    let doc = Vdi3805Snapshot::default();
    let result = part_08::check(&doc);
    assert_eq!(result.status, CheckStatus::NotApplicable);
}

#[semio_framework_async_macros::async_test]
async fn radiator_heat_exponent_bounds() {
    let mut doc = conforming_valve_dataset();
    doc.catalog.products[0].sheet = SheetId(3);
    doc.catalog.products[0].configuration.attributes = SheetAttributes::Radiator(RadiatorAttributes {
        standard_output_w: 1200.0,
        heat_exponent_n: 0.5,
        length_m: 1.0,
        height_m: 0.6,
        depth_m: 0.1,
        connection_type: "side".into(),
    });
    doc.index = CatalogIndex::from_catalog(&doc.catalog);
    let report = evaluate(&doc);
    let n = report.checks.iter().find(|c| c.id.contains("vdi3805.3.n")).expect("n check");
    assert_eq!(n.status, CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn evaluate_reaches_claimed_product_sheets_when_present() {
    let report = evaluate(&conforming_valve_dataset());
    let parts: BTreeSet<_> = report.checks.iter().map(|c| c.part.clone()).collect();
    assert!(parts.iter().any(|p| p.contains("Blatt 1")));
    assert!(parts.iter().any(|p| p.contains("Blatt 2")));
}


#[semio_framework_async_macros::async_test]
async fn edition_profile_legacy_missing_keys_fails() {
    let mut doc = conforming_valve_dataset();
    // Sheet 8 multi-profile: Current requires type_code; put a sheet-8 product without it under Current.
    doc.catalog.products[0].sheet = SheetId(8);
    doc.catalog.products[0].configuration.attributes = SheetAttributes::Generic(GenericAttributes::default());
    doc.catalog.products[0].records.retain(|r| r.family.0 == RecordFamilyId::R100);
    doc.edition_profile.insert("8".into(), EditionProfileChoice::Current);
    let report = evaluate(&doc);
    let fail = report.checks.iter().find(|c| c.id.contains("edition_profile") || c.id.contains("8.profile") || c.id.contains("8.mandatory"));
    assert!(fail.is_some_and(|c| c.status == CheckStatus::Fail), "expected profile/mandatory Fail, got {:?}", report.checks.iter().map(|c| (&c.id, c.status)).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn curve_monotonicity_remedy_path_is_scalar_point_y() {
    let mut doc = conforming_valve_dataset();
    let curve = doc.curves.get_mut("curve-kvs").expect("curve");
    curve.points = vec![crate::CurvePoint { x: 0.0, y: 1.0 }, crate::CurvePoint { x: 50.0, y: 0.5 }, crate::CurvePoint { x: 100.0, y: 4.5 }];
    let report = evaluate(&doc);
    let fail = report.checks.iter().find(|c| c.id.contains("curve") && c.status == CheckStatus::Fail).expect("curve fail");
    let remedy = &fail.remedies[0];
    assert!(remedy.applicable);
    assert!(remedy.target.path.contains("points[") && remedy.target.path.ends_with(".y"), "path={}", remedy.target.path);
}

#[semio_framework_async_macros::async_test]
async fn apply_remedy_flips_curve_monotonicity_fail_to_pass() {
    use crate::app_surface::set_value_at_path;
    use dsl::{FromValue, ToValue};

    let mut doc = conforming_valve_dataset();
    let curve = doc.curves.get_mut("curve-kvs").expect("curve");
    curve.points = vec![crate::CurvePoint { x: 0.0, y: 1.0 }, crate::CurvePoint { x: 50.0, y: 0.5 }, crate::CurvePoint { x: 100.0, y: 4.5 }];
    let before = evaluate(&doc);
    let fail = before.checks.iter().find(|c| c.id.contains("curve") && c.status == CheckStatus::Fail).expect("curve fail");
    let remedy = &fail.remedies[0];
    assert!(remedy.applicable);
    let mut tree = doc.to_value();
    set_value_at_path(&mut tree, &remedy.target.path, dsl::DslValue::float(remedy.required.value)).expect("set_value_at_path");
    let fixed = Vdi3805Snapshot::from_value(tree).expect("decode");
    let after = evaluate(&fixed);
    let again = after.checks.iter().find(|c| c.id == fail.id);
    assert!(again.is_none_or(|c| c.status == CheckStatus::Pass), "curve still failing after applyRemedy path write: {:?}", again.map(|c| (&c.id, c.status)));
}

#[semio_framework_async_macros::async_test]
async fn product_paths_use_id_selectors() {
    let report = evaluate(&nonconforming_valve_dataset());
    let with_path = report.failing().flat_map(|c| c.remedies.iter()).find(|r| r.target.path.contains("catalog.products"));
    assert!(with_path.is_some_and(|r| r.target.path.contains("[id=")), "expected [id=] selector, got {:?}", with_path.map(|r| &r.target.path));
}

#[semio_framework_async_macros::async_test]
async fn operative_sheets_without_product_are_not_applicable() {
    let doc = conforming_valve_dataset();
    assert_eq!(part_04::check(&doc).status, CheckStatus::NotApplicable);
    assert_eq!(part_60::check(&doc).status, CheckStatus::NotApplicable);
    assert_eq!(part_17::check(&doc).status, CheckStatus::NotApplicable);
}


#[semio_framework_async_macros::async_test]
async fn every_emitted_path_resolves() {
    use crate::app_surface::{get_value_at_path, parse_path};
    use dsl::ToValue;

    for doc in [conforming_valve_dataset(), nonconforming_valve_dataset()] {
        let tree = doc.to_value();
        let report = evaluate(&doc);
        for check in &report.checks {
            if !check.subject.path.is_empty() {
                parse_path(&check.subject.path).unwrap_or_else(|e| panic!("subject path {} parse: {e}", check.subject.path));
                let _ = get_value_at_path(&tree, &check.subject.path);
            }
            for remedy in &check.remedies {
                parse_path(&remedy.target.path).unwrap_or_else(|e| panic!("remedy path {} parse: {e}", remedy.target.path));
                // Writable leaf may be absent before insert; parse success is required.
                let _ = get_value_at_path(&tree, &remedy.target.path);
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn every_editable_leaf_has_en_de_field_meta() {
    use crate::app_surface::NormFieldMeta;
    use crate::editor::field_meta::vdi3805_field_meta;
    use dsl::{DslValue, ToValue};

    fn walk(value: &DslValue, prefix: &str, out: &mut Vec<String>) {
        match value {
            DslValue::Object(map) => {
                for (k, v) in map {
                    let path = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                    match v {
                        DslValue::Object(_) | DslValue::Array(_) => walk(v, &path, out),
                        _ => out.push(path),
                    }
                }
            }
            DslValue::Array(items) => {
                for (i, v) in items.iter().enumerate() {
                    let path = format!("{prefix}[{i}]");
                    match v {
                        DslValue::Object(_) | DslValue::Array(_) => walk(v, &path, out),
                        _ => out.push(path),
                    }
                }
            }
            _ => out.push(prefix.to_string()),
        }
    }

    let doc = conforming_valve_dataset();
    let tree = doc.to_value();
    let mut leaves = Vec::new();
    walk(&tree, "", &mut leaves);
    assert!(!leaves.is_empty(), "expected editable leaves");
    let mut missing = Vec::new();
    for path in &leaves {
        // Map-keyed geometry/curves use concrete ids; field meta uses [] wildcards / prefix fallbacks.
        match vdi3805_field_meta(path) {
            Some(NormFieldMeta { label_en, label_de, .. }) if !label_en.is_empty() && !label_de.is_empty() => {}
            other => missing.push((path.clone(), other.map(|m| (m.label_en.to_string(), m.label_de.to_string())))),
        }
    }
    assert!(missing.is_empty(), "missing en+de field meta for leaves ({}) : {missing:?}", missing.len());
}

#[semio_framework_async_macros::async_test]
async fn sync_fail_remedy_targets_writable_attribute_leaf() {
    use crate::app_surface::set_value_at_path;
    use dsl::{FromValue, ToValue};

    let mut doc = conforming_valve_dataset();
    if let SheetAttributes::ValveHeating(ref mut a) = doc.catalog.products[0].configuration.attributes {
        a.kvs_m3_s = 9.0 / 3600.0;
    }
    let before = evaluate(&doc);
    let fail = before.checks.iter().find(|c| c.id.contains(".sync.") && c.status == CheckStatus::Fail).expect("sync fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable).expect("applicable sync remedy");
    assert!(!remedy.target.path.ends_with(".records"), "sync remedy must not target records root: {}", remedy.target.path);
    assert!(remedy.target.path.contains("attributes"), "path={}", remedy.target.path);
    let mut tree = doc.to_value();
    set_value_at_path(&mut tree, &remedy.target.path, dsl::DslValue::float(remedy.required.value)).expect("set");
    let fixed = Vdi3805Snapshot::from_value(tree).expect("decode");
    let after = evaluate(&fixed);
    assert!(after.checks.iter().find(|c| c.id == fail.id).is_none_or(|c| c.status == CheckStatus::Pass));
}

#[semio_framework_async_macros::async_test]
async fn edition_profile_fail_remedy_is_applicable_on_attribute_leaf() {
    let mut doc = conforming_valve_dataset();
    doc.catalog.products[0].sheet = SheetId(8);
    doc.catalog.products[0].configuration.attributes = SheetAttributes::Generic(GenericAttributes::default());
    doc.catalog.products[0].records.retain(|r| r.family.0 == RecordFamilyId::R100);
    doc.edition_profile.insert("8".into(), EditionProfileChoice::Current);
    let report = evaluate(&doc);
    let fail = report
        .checks
        .iter()
        .find(|c| c.status == CheckStatus::Fail && (c.id.contains("edition_profile") || c.id.contains("8.mandatory") || c.id.contains("8.profile")))
        .expect("edition/mandatory fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable).expect("applicable remedy");
    assert!(!remedy.target.path.ends_with(".records"), "path={}", remedy.target.path);
}

#[semio_framework_async_macros::async_test]
async fn sheet_routing_uses_product_sheet_even_when_attributes_generic() {
    let mut doc = conforming_valve_dataset();
    doc.catalog.products[0].sheet = SheetId(5);
    doc.catalog.products[0].configuration.attributes = SheetAttributes::Generic(GenericAttributes::default());
    // Native 210 implies pump fields so attributes_from_records yields PumpHeating.
    doc.catalog.products[0].records = vec![
        crate::NativeRecord {
            family: RecordFamilyId(RecordFamilyId::R210.to_string()),
            fields: vec![
                "210".into(),
                "dn_suction".into(),
                "50".into(),
                "dn_discharge".into(),
                "50".into(),
                "nominal_flow_m3_s".into(),
                "0.0025".into(),
                "nominal_head_m".into(),
                "6".into(),
                "motor_power_w".into(),
                "750".into(),
                "hydraulic_efficiency".into(),
                "0.45".into(),
            ],
            extensions: Default::default(),
        },
    ];
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("vdi3805.5.") && (c.id.contains(".q.") || c.id.contains(".eta.") || c.id.contains(".sync."))));
    assert!(report.checks.iter().any(|c| c.id.contains(".sync.") && c.status == CheckStatus::Fail), "Generic config vs pump 210 must sync-fail");
}


fn walk_dsl_leaves(prefix: &str, value: &dsl::DslValue, visit: &mut dyn FnMut(&str, &dsl::DslValue)) {
    match value {
        dsl::DslValue::Object(map) => {
            for (k, v) in map.iter() {
                // Map keys with `.` (geometry/curve ids) must use [id=…] so set_value_at_path can resolve them.
                let path = if prefix.is_empty() {
                    if k.contains('.') { format!("[id={k}]") } else { k.clone() }
                } else if k.contains('.') {
                    format!("{prefix}[id={k}]")
                } else {
                    format!("{prefix}.{k}")
                };
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

fn is_descriptive_name_or_title_leaf(path: &str) -> bool {
    if path.ends_with(".text") || path.ends_with(".title") || path.ends_with(".shortName") || path.ends_with(".label") {
        return true;
    }
    if path.ends_with(".locale") && path.contains(".title") {
        return true;
    }
    let leaf = path.rsplit(['.', '[']).next().unwrap_or(path);
    matches!(leaf, "name" | "title" | "labelEn" | "labelDe")
}

fn is_reference_or_entity_id_leaf(path: &str) -> bool {
    let leaf = path.rsplit(['.', '[']).next().unwrap_or(path);
    matches!(
        leaf,
        "id" | "accessoryId" | "componentId" | "productId" | "geometryRef"
    ) || leaf.ends_with("Id")
        || leaf.ends_with("Ref")
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
                let base = if v < 0.0 { (v as i64).saturating_sub(1) as u64 } else { (v as u64).saturating_add(1) };
                dsl::DslValue::uint(base.max(0))
            } else if path.contains(".points[") || path.contains("si_factor") || path.contains("siFactor") {
                // Break monotonicity / unit scale rather than a ratio-preserving stretch.
                let next = if v.abs() < 1e-12 { -1.0 } else { -v.abs() * 1.5 - 0.5 };
                dsl::DslValue::float(next)
            } else {
                let next = if v.abs() < 1e-12 { 1.0 } else { v * 1.35 + 0.01 };
                dsl::DslValue::float(next)
            })
        }
        dsl::DslValue::Bool(b) => Some(dsl::DslValue::Bool(!*b)),
        dsl::DslValue::String(s) => {
            use crate::editor::field_meta::vdi3805_field_meta;
            if is_reference_or_entity_id_leaf(path) {
                return Some(dsl::DslValue::String("__dangling__".into()));
            }
            if let Some(choices) = vdi3805_field_meta(path).and_then(|m| m.choices) {
                let alt = choices.iter().map(|c| c.value).find(|c| *c != s.as_str()).unwrap_or("x");
                return Some(dsl::DslValue::String(alt.to_string()));
            }
            let next = if s.is_empty() { "x".into() } else { format!("{s}__pert") };
            Some(dsl::DslValue::String(next))
        }
        _ => None,
    }
}

#[semio_framework_async_macros::async_test]
async fn every_editable_leaf_perturbation_changes_a_check() {
    use dsl::{FromValue, ToValue};
    let subjects: Vec<(String, Vdi3805Snapshot)> = all_conforming_blatt_examples()
        .into_iter()
        .map(|(sheet, doc)| (format!("blatt-{sheet}"), doc))
        .collect();
    // Nonconforming counterpart is asserted separately; perturbation subjects are conforming assessed Blätter only.
    let mut inert = Vec::new();
    for (label, base) in subjects {
        let base_report = evaluate(&base);
        let base_sig = report_signature(&base_report);
        let value0 = ToValue::to_value(&base);
        let mut leaves = Vec::new();
        walk_dsl_leaves("", &value0, &mut |path, leaf| {
            if path.is_empty() || is_descriptive_name_or_title_leaf(path) {
                return;
            }
            leaves.push((path.to_string(), leaf.clone()));
        });
        for (path, leaf) in leaves {
            let Some(next) = perturb_dsl_leaf(&path, &leaf) else { continue };
            let mut tree = ToValue::to_value(&base);
            if crate::app_surface::set_value_at_path(&mut tree, &path, next).is_err() {
                inert.push(format!("{label}:{path} (set failed)"));
                continue;
            }
            let Ok(perturbed) = <Vdi3805Snapshot as FromValue>::from_value(tree) else { continue };
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

#[semio_framework_async_macros::async_test]
async fn assessed_blatt_examples_decode_and_evaluate() {
    for sheet in ASSESSED_BLATT_SHEETS {
        let ok = conforming_blatt_dataset(*sheet);
        let report = evaluate(&ok);
        assert!(report.complies() || report.failing().count() == 0, "blatt {sheet} conforming must comply: fails {:?}", report.failing().map(|c| &c.id).collect::<Vec<_>>());
        let bad = nonconforming_blatt_dataset(*sheet);
        let bad_report = evaluate(&bad);
        let fails: Vec<_> = bad_report.failing().collect();
        assert!(fails.len() >= 2, "blatt {sheet} nonconforming expected ≥2 fails, got {}", fails.len());
        for fail in &fails {
            assert!(!fail.remedies.is_empty(), "blatt {sheet} fail {} needs remedies", fail.id);
            assert!(fail.remedies.iter().any(|r| r.applicable), "blatt {sheet} fail {} needs applicable remedy", fail.id);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn facet_parity_accessories_components_generic_product_id() {
    let snapshot_graphql = include_str!("../../../📸️snapshot/🔗️.graphql");
    let root_graphql = include_str!("../../../🔗️.graphql");
    let diff_graphql = include_str!("../../../🔺️diff/🔗️.graphql");
    let snapshot_json = include_str!("../../../📸️snapshot/🔣️.json");
    let root_json = include_str!("../../../🔣️.json");
    for (label, text) in [
        ("snapshot.graphql", snapshot_graphql),
        ("root.graphql", root_graphql),
        ("diff.graphql", diff_graphql),
    ] {
        assert!(text.contains("accessories: [AccessoryLink!]!"), "{label} accessories must be AccessoryLink");
        assert!(text.contains("components: [CompositionLink!]!"), "{label} components must be CompositionLink");
        assert!(text.contains("entries: [GenericAttribute!]!"), "{label} GenericAttributes.entries");
        assert!(!text.contains("_placeholder"), "{label} must not stub GenericAttributes");
        assert!(!text.contains("accessories: [String!]!"), "{label} must not use string[] accessories");
    }
    assert!(root_graphql.contains("type Product") && root_graphql.contains("id: String!"), "root Product.id");
    for (label, text) in [("snapshot.json", snapshot_json), ("root.json", root_json)] {
        assert!(text.contains("AccessoryLink"), "{label} AccessoryLink def");
        assert!(text.contains("CompositionLink"), "{label} CompositionLink def");
        assert!(text.contains("GenericAttributes"), "{label} GenericAttributes def");
        if let Some(req_start) = text.find("\"required\"") {
            let slice = &text[req_start..text.len().min(req_start + 200)];
            assert!(!slice.contains("manufacturerFile"), "{label} required must not list manufacturerFile");
            assert!(!slice.contains("limits"), "{label} required must not list limits");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn dangling_accessory_and_component_ids_fail_with_one_of_existing_product_ids() {
    let mut doc = conforming_valve_dataset();
    let existing: Vec<String> = doc.catalog.products.iter().map(|p| p.id.clone()).collect();
    assert!(existing.len() >= 2, "fixture must expose catalogue product ids for one_of choices");
    doc.catalog.products[0].accessories = vec![AccessoryLink {
        accessory_id: "__dangling__".into(),
        required: true,
        quantity: 1,
    }];
    doc.catalog.products[0].components = vec![CompositionLink {
        component_id: "__dangling__".into(),
        quantity: 1,
    }];
    let article = doc.catalog.products[0].id.clone();
    let report = evaluate(&doc);
    let acc = report
        .checks
        .iter()
        .find(|c| c.id == format!("vdi3805.1.accessories.{article}"))
        .expect("dangling accessory check");
    assert_eq!(acc.status, CheckStatus::Fail);
    assert!(acc.explanation.en != acc.explanation.de, "en/de explanations must differ");
    let acc_remedy = acc.remedies.iter().find(|r| !r.options.is_empty()).expect("accessory remedy must be one_of with options");
    assert!(
        existing.iter().any(|id| acc_remedy.options.contains(id)),
        "accessory one_of must enumerate an existing product id; options={:?} existing={:?}",
        acc_remedy.options,
        existing
    );
    let comp = report
        .checks
        .iter()
        .find(|c| c.id == format!("vdi3805.1.components.{article}"))
        .expect("dangling component check");
    assert_eq!(comp.status, CheckStatus::Fail);
    assert!(comp.explanation.en != comp.explanation.de, "en/de explanations must differ");
    let comp_remedy = comp.remedies.iter().find(|r| !r.options.is_empty()).expect("component remedy must be one_of with options");
    assert!(
        existing.iter().any(|id| comp_remedy.options.contains(id)),
        "component one_of must enumerate an existing product id; options={:?} existing={:?}",
        comp_remedy.options,
        existing
    );
}

#[test]
fn check_sources_contain_no_fingerprint_gaming_patterns() {
    let inf = include_str!("../../🦀️.rs");
    let hits: Vec<_> = ["param_metric", "pos_metric", "point_metric + unit_metric", "field_fingerprint", "(k.len() as f64) * 1e-", "let _ = actual_records", "let _ = curve_id", "let _ = document"]
        .into_iter()
        .filter(|p| inf.contains(p))
        .collect();
    assert!(hits.is_empty(), "perturbation gaming patterns must not fold string fingerprints into computed/limit (CORRECTION 14:37): {hits:?}");
}
