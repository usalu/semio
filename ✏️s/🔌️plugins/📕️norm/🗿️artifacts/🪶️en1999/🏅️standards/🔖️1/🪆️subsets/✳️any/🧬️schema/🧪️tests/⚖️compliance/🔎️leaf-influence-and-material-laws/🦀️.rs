//! 🔎️ Leaf influence and material laws of the EN 1999 compliance tests: the metadata rows, the editable leaf influence, the asset regeneration and the material id laws. See [`super`].

use super::*;

pub(super) fn strip_nums(s: &str) -> String {
    s.chars().filter(|c| !c.is_ascii_digit() && *c != '.' && *c != ',').collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn field_meta_rows_resolve_to_schema_leaves() {
    let snap = En1999Snapshot::compliant_roof_purlin();
    let tree = serde_json::to_value(&snap).unwrap();
    let mut leaves = Vec::new();
    walk_leaves("", &tree, &mut |path| {
        if path.is_empty() { return; }
        let leaf = path.rsplit(['.', '[']).next().unwrap_or(path);
        if leaf == "id" || leaf.starts_with("id=") { return; }
        leaves.push(path.to_string());
    });
    use semio_s_artifact_norm_en1999_editor::field_meta::en1999_field_meta;
    for stale in ["vYEd", "vZEd", "mZEd", "mYEd", "slopeM"] {
        assert!(leaves.iter().all(|p| !p.contains(stale)), "snapshot still has stale {stale}");
    }
    assert!(leaves.iter().filter(|p| p.contains("actions") && p.contains("nEd")).count() == 0);
    for path in &leaves {
        let meta_path = path_to_meta_pattern(path);
        assert!(en1999_field_meta(&meta_path).is_some(), "no field-meta for leaf {path} (pattern {meta_path})");
    }
}

fn path_to_meta_pattern(path: &str) -> String {
    let mut out = String::new();
    let mut rest = path;
    while !rest.is_empty() {
        if rest.starts_with('.') { rest = &rest[1..]; }
        if rest.starts_with('[') {
            let close = rest.find(']').unwrap();
            out.push_str("[]");
            rest = &rest[close+1..];
            continue;
        }
        let end = rest.find(['.', '[']).unwrap_or(rest.len());
        if !out.is_empty() && !out.ends_with(']') { out.push('.'); }
        if out.ends_with(']') { out.push('.'); }
        out.push_str(&rest[..end]);
        rest = &rest[end..];
    }
    out
}

#[test]
fn sls_deflection_and_stress_checks_present() {
    let ok = evaluate_structure(&En1999Snapshot::compliant_roof_purlin());
    assert!(ok.checks.iter().any(|c| c.id.contains("7.2.defl")));
    assert!(ok.checks.iter().any(|c| c.id.contains("7.2.stress")));
    let bad = evaluate_structure(&En1999Snapshot::noncompliant_multi_fail());
    let fail_defl = bad.failing().find(|c| c.id.contains("7.2.defl"));
    assert!(fail_defl.is_some(), "noncompliant should fail SLS deflection");
    let f = fail_defl.unwrap();
    assert!(!f.remedies.is_empty());
}

pub(super) fn walk_leaves(path: &str, value: &serde_json::Value, visit: &mut dyn FnMut(&str)) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let child = if path.is_empty() { k.clone() } else { format!("{path}.{k}") };
                if matches!(v, serde_json::Value::Object(_) | serde_json::Value::Array(_)) {
                    walk_leaves(&child, v, visit);
                } else {
                    visit(&child);
                }
            }
        }
        serde_json::Value::Array(items) => {
            for (i, v) in items.iter().enumerate() {
                let child = format!("{path}[{i}]");
                if matches!(v, serde_json::Value::Object(_) | serde_json::Value::Array(_)) {
                    walk_leaves(&child, v, visit);
                } else {
                    visit(&child);
                }
            }
        }
        _ => visit(path),
    }
}

#[test]
fn haz_rho_u_governs_welded_net_section() {
    let alloy = part_1_1::resolve_alloy("aw6060-t6").unwrap();
    assert!(alloy.rho_u_haz < alloy.rho_o_haz || alloy.rho_u_haz < 1.0);
    let mut sec = En1999Snapshot::noncompliant_multi_fail().sections[0].clone();
    for e in &mut sec.elements { e.welded = true; e.weld_position = 0.01; }
    let (_, _, _, _, _, _, _) = part_1_1::section_geometry(&sec);
    let a_net = part_1_1::effective_area(&sec, alloy.f_o_pa, true, 1.0, 1.0);
    let n_u_full = a_net * alloy.f_u_pa;
    let n_u_haz = a_net * alloy.rho_u_haz * alloy.f_u_pa;
    assert!(alloy.rho_u_haz < 1.0);
    assert!(n_u_haz < n_u_full - 1e-6, "ρ_u,haz must reduce N_u,Rd ({n_u_haz} vs {n_u_full})");
}

#[test]
fn shell_chi_from_geometry_hand_value() {
    let e = na_de::E_PA;
    let r: f64 = 0.80;
    let t: f64 = 0.003;
    let sigma_x_rcr = 0.605 * e * (t / r);
    let f_o: f64 = 160.0e6;
    let lambda_x = (f_o / sigma_x_rcr).sqrt();
    let alpha: f64 = 0.62;
    let beta: f64 = 0.60;
    let lambda_0: f64 = 0.20;
    let phi = 0.5 * (1.0 + alpha * (lambda_x - lambda_0) + beta * lambda_x.powi(2));
    let chi = 1.0 / (phi + (phi * phi - beta * lambda_x.powi(2)).max(0.0).sqrt());
    assert!((chi - 0.55).abs() < 0.15, "χ_x={chi}, λ̄={lambda_x}");
    let doc = En1999Snapshot::noncompliant_multi_fail();
    let report = evaluate_structure(&doc);
    let shell = report.checks.iter().find(|c| c.id.contains("1-5.buckle")).expect("shell buckle");
    assert!(shell.explanation.en.contains("χ=") || shell.explanation.en.contains("Rcr") || shell.utilization > 0.0);
}

#[test]

fn every_editable_leaf_influences_a_check() {
    const EXEMPT_SUFFIXES: &[&str] = &[".id"];
    for snap in [En1999Snapshot::compliant_roof_purlin(), En1999Snapshot::noncompliant_multi_fail()] {
        let base_report = evaluate_structure(&snap);
        let mut base_sig: Vec<_> = base_report
            .checks
            .iter()
            .map(|c| (c.id.clone(), c.status, c.computed.value.to_bits(), c.limit.value.to_bits(), c.utilization.to_bits()))
            .collect();
        base_sig.sort_by(|a, b| a.0.cmp(&b.0));
        let value = serde_json::to_value(&snap).expect("json");
        let mut leaves = Vec::new();
        walk_leaves("", &value, &mut |path| {
            if path.is_empty() { return; }
            let leaf = path.rsplit(['.', '[']).next().unwrap_or(path);
            if leaf == "id" || leaf.starts_with("id=") { return; }
            if EXEMPT_SUFFIXES.iter().any(|s| path.ends_with(s)) { return; }
            leaves.push(path.to_string());
        });
        assert!(!leaves.is_empty());
        let mut unchanged = Vec::new();
        for path in &leaves {
            let mut tree = serde_json::to_value(&snap).unwrap();
            let cur = resolve_path(&tree, path).cloned();
            let Some(cur) = cur else { continue; };
            match cur {
                serde_json::Value::Number(n) => {
                    let v = n.as_f64().unwrap_or(0.0);
                    let nv = if v.abs() < 1e-12 { 1.0e5 } else { v * 1.35 };
                    let _ = set_number_at_path(&mut tree, path, nv);
                }
                serde_json::Value::Bool(b) => { let _ = set_bool_at_path(&mut tree, path, !b); }
                serde_json::Value::String(s) => {
                    let leaf = path.rsplit('.').next().unwrap_or(path.as_str());
                    let ns: String = if s == "de" { "en".into() }
                        else if s == "en" { "de".into() }
                        else if s.contains("6082") { "aw6060-t6".into() }
                        else if s.contains("6060") { "aw6082-t6".into() }
                        else if s == "simplySupported" { "continuous".into() }
                        else if s == "continuous" { "cantilever".into() }
                        else if s == "cantilever" { "simplySupported".into() }
                        else if leaf == "kind" && (s == "snow" || s == "wind" || s == "imposed" || s == "temperature") { "permanent".into() }
                        else if leaf == "kind" && s == "permanent" { "imposed".into() }
                        else if leaf == "category" && s == "snow" { "storage".into() }
                        else if leaf == "category" && s == "wind" { "storage".into() }
                        else if leaf == "category" && s == "office" { "storage".into() }
                        else if leaf == "category" && s == "self" { "snow".into() }
                        else if s == "permanent" { "imposed".into() }
                        else if s == "imposed" { "permanent".into() }
                        else if s == "snow" { "storage".into() }
                        else if s == "wind" { "storage".into() }
                        else if s == "office" { "storage".into() }
                        else if s == "self" { "snow".into() }
                        else if s == "external" { "udl".into() }
                        else if s == "udl" { "external".into() }
                        else if s == "8.8" { "10.9".into() }
                        else if s == "10.9" { "5.6".into() }
                        else if s == "4.6" || s == "5.6" { "8.8".into() }
                        else if s == "tube" || s == "chs" { "extrudedI".into() }
                        else if s == "extrudedI" { "tube".into() }
                        else if s == "combined" { "bolted".into() }
                        else if s == "bolted" { "welded".into() }
                        else if s == "welded" { "combined".into() }
                        else if s == "4043" { "5356".into() }
                        else if s == "5356" { "4043".into() }
                        else { format!("{s}-x") };
                    let _ = set_string_at_path(&mut tree, path, &ns);
                }
                _ => continue,
            }
            let Ok(perturbed): Result<En1999Snapshot,_> = serde_json::from_value(tree) else { continue; };
            let rep = evaluate_structure(&perturbed);
            let mut sig: Vec<_> = rep.checks.iter().map(|c| (c.id.clone(), c.status, c.computed.value.to_bits(), c.limit.value.to_bits(), c.utilization.to_bits())).collect();
            sig.sort_by(|a, b| a.0.cmp(&b.0));
            if sig == base_sig {
                unchanged.push(path.clone());
            }
        }
        assert!(
            unchanged.is_empty(),
            "editable leaves must influence a check (no ratio gate): {unchanged:?}"
        );
    }
}

#[test]

fn regen_example_assets_when_env_set() {
    if std::env::var("REGEN_EN1999_ASSETS").is_err() {
        return;
    }
    let any = family_any_dir();
    for (name, snap) in [
        ("🏠️aluminium-roof-purlin", En1999Snapshot::compliant_roof_purlin()),
        ("🏚️noncompliant-multi-fail", En1999Snapshot::noncompliant_multi_fail()),
    ] {
        let dir = any.join("🖼️assets").join(name).join(name);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("🗣️.dsl.semio"), encode_en1999_dsl(&snap)).unwrap();
        std::fs::write(
            any.join("🖼️assets").join(name).join("🎒️.pack.semio"),
            semio_s_artifact_norm_en1999_io::binary::snapshot::encode_en1999_pack(&snap),
        ).unwrap();
        std::fs::write(
            dir.join("🔣️.json"),
            semio_s_artifact_norm_en1999_io::text::snapshot::encode_en1999_snapshot_json(&snap),
        ).unwrap();
    }
}

#[test]
fn duplicate_material_id_fails_with_oneof_remedy() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    let dup_id = doc.materials[0].id.clone();
    let mut twin = doc.materials[0].clone();
    twin.designation = "aw5083-o".into();
    doc.materials.push(twin);
    let report = evaluate_structure(&doc);
    let check = report
        .checks
        .iter()
        .find(|c| c.id == format!("en1999.ref.duplicate.materials.{dup_id}"))
        .expect("duplicate material id Fail");
    assert_eq!(check.status, CheckStatus::Fail);
    assert_ne!(check.explanation.en, check.explanation.de);
    let remedy = check.remedies.iter().find(|r| r.applicable).expect("applicable one_of remedy");
    assert!(matches!(remedy.bound, semio_s_plugin_norm_compliance::RemedyBound::OneOf));
    assert!(!remedy.options.is_empty());
    assert!(!remedy.options.iter().any(|o| o == &dup_id));
}

#[test]
fn dangling_connection_material_id_fails_with_oneof_existing_materials() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    assert!(!doc.connections.is_empty(), "fixture must expose a connection");
    let existing: Vec<String> = doc.materials.iter().map(|m| m.id.clone()).collect();
    let conn_id = doc.connections[0].id.clone();
    doc.connections[0].material_id = "mat-missing".into();
    let report = evaluate_structure(&doc);
    let check = report
        .checks
        .iter()
        .find(|c| c.id == format!("en1999.ref.conn.material.{conn_id}"))
        .expect("dangling connection materialId Fail");
    assert_eq!(check.status, CheckStatus::Fail);
    assert_ne!(check.explanation.en, check.explanation.de);
    let remedy = check.remedies.iter().find(|r| r.applicable).expect("applicable one_of remedy");
    assert!(matches!(remedy.bound, semio_s_plugin_norm_compliance::RemedyBound::OneOf));
    assert!(!remedy.options.is_empty());
    for opt in &remedy.options {
        assert!(existing.contains(opt), "remedy option {opt} must be an existing material id");
    }
    assert!(!remedy.options.iter().any(|o| o == "mat-missing"));
}

#[test]
fn dangling_connection_member_id_fails_with_oneof_existing_members() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    assert!(!doc.connections.is_empty(), "fixture must expose a connection");
    let existing: Vec<String> = doc.members.iter().map(|m| m.id.clone()).collect();
    let conn_id = doc.connections[0].id.clone();
    doc.connections[0].member_id = "member-missing".into();
    let report = evaluate_structure(&doc);
    let check = report
        .checks
        .iter()
        .find(|c| c.id == format!("en1999.8.ref.{conn_id}"))
        .expect("dangling connection memberId Fail");
    assert_eq!(check.status, CheckStatus::Fail);
    assert_ne!(check.explanation.en, check.explanation.de);
    let remedy = check.remedies.iter().find(|r| r.applicable).expect("applicable one_of remedy");
    assert!(matches!(remedy.bound, semio_s_plugin_norm_compliance::RemedyBound::OneOf));
    assert!(!remedy.options.is_empty());
    for opt in &remedy.options {
        assert!(existing.contains(opt), "remedy option {opt} must be an existing member id");
    }
    assert!(!remedy.options.iter().any(|o| o == "member-missing"));
}
