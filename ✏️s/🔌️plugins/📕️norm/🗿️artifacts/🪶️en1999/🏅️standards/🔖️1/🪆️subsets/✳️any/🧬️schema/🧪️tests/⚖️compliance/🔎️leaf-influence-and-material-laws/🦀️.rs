//! 🔎️ Leaf influence and material laws of the EN 1999 compliance tests: the metadata rows, the editable leaf influence, the asset regeneration and the material id laws. See [`super`].

use super::*;
use semio_s_artifact_norm_en1999_checks::joints;

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
    assert!(alloy.rho_u_haz < 1.0);
    let mut sec = En1999Snapshot::noncompliant_multi_fail().sections[0].clone();
    for e in &mut sec.elements { e.welded = true; e.weld_position = 0.01; }
    let gross = part_1_1::props(&sec, &part_1_1::EffCtx::gross(alloy.f_o_pa, true));
    let haz = part_1_1::props(&sec, &part_1_1::EffCtx { f_o_pa: alloy.f_o_pa, class_a: true, eta_web: 1.0, local: false, haz_rho: Some(alloy.rho_u_haz), haz_extents: &[] });
    assert!(haz.a * alloy.f_u_pa < gross.a * alloy.f_u_pa - 1e-6, "ρ_u,haz must reduce N_u,Rd");
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

fn index_in(path: &str, head: &str) -> Option<usize> {
    path.strip_prefix(head)?.split(']').next()?.parse().ok()
}

/// 🧵️ Whether the HAZ boundary resistance of the fillet weld of connection `idx` stays below the weld-metal resistance for every filler alloy and a β_w raised by the perturbation (EN 1999-1-1 §8.6.3.1: the resistance is the lesser of the two), so that β_w and the filler alloy cannot move the check.
fn haz_boundary_governs_the_weld(snap: &En1999Snapshot, idx: usize) -> bool {
    let Some(conn) = snap.connections.get(idx) else { return false };
    let Some(material) = snap.materials.iter().find(|m| m.id == conn.material_id) else { return false };
    let w = &conn.welds;
    let Some(band) = part_1_1::resolve_alloy_for_thickness(&material.designation, conn.bolts.plate_thickness.max(w.throat / 0.7)) else { return false };
    let params = na_de::AnnexParams::for_choice(snap.annex);
    let weakest_filler = joints::filler_fw_pa(&w.filler_alloy, band.f_u_pa).min(joints::filler_fw_pa("4043", band.f_u_pa));
    let (metal, haz) = joints::weld_resistance_per_length(w.throat, w.beta_w * 1.35, weakest_filler, band.f_u_pa, band.rho_u_haz, params.gamma_mw, params.gamma_m2);
    haz <= metal
}

/// 🔥️ Whether the T-junction flag of plate element `element` of section `section` leaves the HAZ strip of the §6.1.6.3 table unchanged: the element is not welded, or the strip of b_haz (×1.33) around the weld already spans the same part of the element width.
fn t_junction_leaves_the_strip(snap: &En1999Snapshot, section: usize, element: usize) -> bool {
    let Some(el) = snap.sections.get(section).and_then(|s| s.elements.get(element)) else { return false };
    let strip = |junction: bool| {
        let mut e = el.clone();
        e.weld_t_junction = junction;
        let b = part_1_1::haz_width_for(&e);
        if e.weld_direction == semio_s_artifact_norm_en1999_model::WeldDirection::Transverse { (0.0, e.width) } else { ((e.weld_position - b).max(0.0).min(e.width), (e.weld_position + b).max(0.0).min(e.width)) }
    };
    !el.welded || strip(false) == strip(true)
}

/// 🌡️ Whether the member temperature at the end of fire scenario `idx` is independent of the start temperature θ_a (EN 1999-1-2 §4.2.5): a thin member with a time constant far below the duration has forgotten its start temperature, so the leaf cannot move any check.
fn fire_start_temperature_is_forgotten(snap: &En1999Snapshot, idx: usize) -> bool {
    let Some(fire) = snap.fire_scenarios.get(idx) else { return false };
    let Some(section) = snap.members.iter().find(|m| m.id == fire.member_id).and_then(|m| snap.sections.iter().find(|s| s.id == m.section_id)) else { return false };
    let cover = snap.fire_insulations.iter().find(|i| i.fire_id == fire.id).map(part_1_2::Insulation::from).filter(|i| i.thickness > 0.0);
    let at = |theta: f64| part_1_2::member_temperature_c(theta, part_1_2::section_factor(section), fire.duration_s.max(0.0), cover.as_ref());
    let perturbed = if fire.theta_a.abs() < 1e-12 { 1.0e5 } else { fire.theta_a * 1.35 };
    at(fire.theta_a).to_bits() == at(perturbed).to_bits()
}

/// 🧯 Leaves that carry no information for the physical situation they sit in: hollow sections are immune to lateral-torsional and torsional buckling, tubes carry no web for a patch load (§6.7.5), the second S–N slope is inactive while γ·Δσ stays above the knee stress, the weld-metal leaves are inert while the HAZ boundary governs the weld, the T-junction factor is inert once the HAZ strip spans the element and the start temperature of a fire is inert once the member has forgotten it.
fn physically_inactive(snap: &En1999Snapshot, path: &str) -> bool {
    if path.ends_with(".welds.betaW") || path.ends_with(".welds.fillerAlloy") {
        return index_in(path, "connections[").is_some_and(|idx| haz_boundary_governs_the_weld(snap, idx));
    }
    if let (Some(idx), true) = (index_in(path, "fireScenarios["), path.ends_with(".thetaA")) {
        if fire_start_temperature_is_forgotten(snap, idx) {
            return true;
        }
    }
    if path.ends_with(".weldTJunction") {
        return index_in(path, "sections[").zip(path.split("elements[").nth(1).and_then(|r| r.split(']').next()).and_then(|i| i.parse().ok())).is_some_and(|(section, element)| t_junction_leaves_the_strip(snap, section, element));
    }
    if let (Some(idx), true) = (index_in(path, "members["), path.ends_with(".localLoadK") || path.ends_with(".bearingLength")) {
        if snap.members.get(idx).and_then(|m| snap.sections.iter().find(|s| s.id == m.section_id)).is_some_and(part_1_1::is_tube) {
            return true;
        }
    }
    if (path.ends_with(".torsionK") || path.ends_with(".localLoadK")) && !path.starts_with("members[") {
        return true;
    }
    if path.ends_with(".transverseWeld") {
        let idx: usize = path.strip_prefix("members[").and_then(|r| r.split(']').next()).and_then(|i| i.parse().ok()).unwrap_or(usize::MAX);
        return snap.members.get(idx).is_none_or(|m| m.transverse_weld_offset == 0.0);
    }
    if path.ends_with(".transverseWeldOffset") {
        let idx: usize = path.strip_prefix("members[").and_then(|r| r.split(']').next()).and_then(|i| i.parse().ok()).unwrap_or(usize::MAX);
        return snap.members.get(idx).is_none_or(|m| !m.transverse_weld);
    }
    if let Some(rest) = path.strip_prefix("members[") {
        let idx: usize = rest.split(']').next().and_then(|i| i.parse().ok()).unwrap_or(usize::MAX);
        let leaf = path.rsplit('.').next().unwrap_or("");
        if let Some(member) = snap.members.get(idx) {
            let hollow = snap.sections.iter().find(|s| s.id == member.section_id).is_some_and(|s| matches!(s.kind, semio_s_artifact_norm_en1999_model::SectionKind::Chs | semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs));
            return hollow && matches!(leaf, "bucklingLengthT" | "ltbLength" | "c1" | "restrainedLtb");
        }
    }
    if let Some(rest) = path.strip_prefix("fatigueDetails[") {
        let idx: usize = rest.split(']').next().and_then(|i| i.parse().ok()).unwrap_or(usize::MAX);
        if let (Some(f), true) = (snap.fatigue_details.get(idx), path.ends_with(".m2")) {
            let params = na_de::AnnexParams::for_choice(snap.annex);
            let dc = part_1_3::detail_category_delta_sigma_c(&f.detail_category).unwrap_or(f.delta_sigma_c);
            return f.delta_sigma_ed * params.gamma_ff * params.gamma_mf >= part_1_3::delta_sigma_d(dc, f.m1);
        }
    }
    false
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
            if physically_inactive(&snap, path) { continue; }
            match cur {
                serde_json::Value::Number(n) => {
                    let v = n.as_f64().unwrap_or(0.0);
                    let nv = if let Some(parent) = path.strip_suffix(".weldPosition") {
                        resolve_path(&tree, &format!("{parent}.width")).and_then(|w| w.as_f64()).unwrap_or(v * 1.35)
                    } else if v.abs() < 1e-12 { 1.0e5 } else { v * 1.35 };
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
