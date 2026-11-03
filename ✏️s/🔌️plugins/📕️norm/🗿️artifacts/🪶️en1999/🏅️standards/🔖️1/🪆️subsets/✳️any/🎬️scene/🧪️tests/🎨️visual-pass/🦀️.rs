//! 🎨️ The visual pass of EN 1999: the routing law (every check outcome tones its 3D owner or the document badge), the schema-driven grip coverage with the gauges, the torsional and welded-member buckling diagrams and the DRAWN/OUTCOME split of the check families.

use super::*;
use norm::incremental_evaluation::evaluate_units;
use scene::projection_kit::test_support::{assert_handle_laws, revision};
use scene::Validate;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

fn documents() -> Vec<En1999Snapshot> {
    vec![En1999Snapshot::compliant_roof_purlin(), En1999Snapshot::noncompliant_multi_fail(), En1999Snapshot::full_showcase()]
}

fn reports(document: &En1999Snapshot) -> Vec<CheckReport> {
    vec![checks::evaluate_structure(document), evaluate_units::<checks::En1999Units>(document)]
}

fn scene_of(document: &En1999Snapshot, report: &CheckReport) -> StructuralScene {
    project(&SceneInput { document, report, placements: &[], revision: revision() })
}

fn is_registry_row(document: &En1999Snapshot, id: &str) -> bool {
    !id.is_empty() && (document.sections.iter().any(|e| e.id == id) || document.materials.iter().any(|e| e.id == id) || document.outline_vertices.iter().any(|e| e.id == id))
}

fn finite_applicable(c: &norm::CheckResult) -> bool {
    c.utilization.is_finite() && c.status != norm::CheckStatus::NotApplicable
}

/// 🧭️ Every check outcome of the filed and the unfiled report is routed to a drawn row, whose scene entity exists and which `focus_of_outcome` selects, or to the document badge.
#[test]
fn every_check_outcome_routes_to_its_row_or_the_document_badge() {
    use kit::{base_id, focus_of_outcome, is_document_check, route_of, CheckRoute};
    let mut seen = 0;
    for document in documents() {
        for report in reports(&document) {
            let scene = scene_of(&document, &report);
            let drawn: BTreeSet<&str> = scene.entities.iter().map(|e| base_id(&e.entity.id)).collect();
            for check in &report.checks {
                seen += 1;
                let route = route_of(&document, check).unwrap_or_else(|| panic!("check {} (subject {:?}) has no owner", check.id, check.subject.entity_id));
                match route {
                    CheckRoute::Entity(id) => {
                        assert!(drawn.contains(id.as_str()), "check {} routes to {id}, which the scene does not draw", check.id);
                        assert_eq!(focus_of_outcome(&document, &report, &check.id).as_deref(), Some(id.as_str()), "{}", check.id);
                    }
                    CheckRoute::Document => {
                        assert!(is_document_check(&check.id) || is_registry_row(&document, base_id(&check.subject.entity_id)), "{} is routed to the document without being a document-level check", check.id);
                        assert!(focus_of_outcome(&document, &report, &check.id).is_none(), "{}", check.id);
                    }
                }
            }
        }
    }
    assert!(seen > 400, "{seen} checks routed");
}

/// 🌡️ A sheet, a shell and a fire scenario are toned by the worst applicable check routed to them, in the filed and the unfiled report.
#[test]
fn a_row_is_toned_by_the_worst_check_routed_to_it() {
    let document = En1999Snapshot::full_showcase();
    for report in reports(&document) {
        let scene = scene_of(&document, &report);
        let rows = kit::Rows::of(&document);
        let ids = document.cold_formed.iter().map(|e| e.id.as_str()).chain(document.shells.iter().map(|e| e.id.as_str())).chain(document.fire_scenarios.iter().map(|e| e.id.as_str()));
        for id in ids {
            let worst = report.checks.iter().filter(|c| finite_applicable(c) && rows.owner(c).as_deref() == Some(id)).map(|c| c.utilization.max(0.0)).fold(None, |best: Option<f64>, u| Some(best.map_or(u, |b| b.max(u))));
            let bound = scene.entities.iter().find(|e| e.entity.id == id).unwrap_or_else(|| panic!("{id} is not drawn")).utilisation.as_ref().map(|b| b.ratio);
            match (worst, bound) {
                (Some(w), Some(b)) => assert!((w - b).abs() < 1e-9, "{id}: bound {b} vs worst routed {w}"),
                (None, None) => {}
                other => panic!("{id}: worst routed and bound disagree: {other:?}"),
            }
        }
    }
}

/// 🧭️ A check whose subject is a section, an alloy or an outline vertex is owned by the first row that uses it, a check whose id embeds a row id by that row (the longest wins), and an unknown subject by nobody.
#[test]
fn a_check_on_a_section_an_alloy_or_a_vertex_is_owned_by_the_row_that_uses_it() {
    let document = En1999Snapshot::full_showcase();
    let template = checks::evaluate_structure(&document).checks.first().cloned().expect("a check");
    let owner = |id: &str, subject: &str| {
        let mut check = template.clone();
        check.id = id.into();
        check.subject.entity_id = subject.into();
        kit::owner_entity(&document, &check)
    };
    let member = &document.members[0];
    let first_with_section = document.members.iter().find(|m| m.section_id == member.section_id).map(|m| m.id.clone());
    let first_with_alloy = document.members.iter().find(|m| m.material_id == member.material_id).map(|m| m.id.clone());
    assert_eq!(owner("en1999.ref.test", &member.section_id), first_with_section);
    assert_eq!(owner("en1999.ref.test", &member.material_id), first_with_alloy);
    let vertex = &document.outline_vertices[0];
    assert_eq!(owner("en1999.ref.test", &vertex.id), document.members.iter().find(|m| m.section_id == vertex.section_id).map(|m| m.id.clone()));
    assert_eq!(owner("en1999.test.purlin-1.purlin-welded", ""), Some("purlin-welded".into()), "the longest embedded row id wins");
    assert_eq!(owner("en1999.test", "no-such-row"), None);
    assert_eq!(owner("en1999.test", ""), None);
}

/// 📊️ The drawn families are unique, every carve-out extends a drawn family, and the split classifies representative check ids.
#[test]
fn the_drawn_and_outcome_families_split_the_check_ids_without_overlap() {
    use kit::{is_drawn_family, DRAWN_FAMILIES, OUTCOME_FAMILIES};
    assert_eq!(DRAWN_FAMILIES.iter().collect::<BTreeSet<_>>().len(), DRAWN_FAMILIES.len());
    for carve in OUTCOME_FAMILIES {
        assert!(!DRAWN_FAMILIES.contains(carve) && DRAWN_FAMILIES.iter().any(|drawn| carve.starts_with(drawn)), "{carve} carves out of no drawn family");
    }
    for (id, drawn) in [
        ("en1999.6.3.1.fb.p", true),
        ("en1999.6.3.1.fb.z.p", true),
        ("en1999.6.3.1.tb.p", true),
        ("en1999.6.3.1.haz.p", true),
        ("en1999.6.3.2.ltb.p", true),
        ("en1999.6.2.5.m.p", true),
        ("en1999.6.2.5.mz.p", false),
        ("en1999.6.2.3.n.p", false),
        ("en1999.6.2.6.vy.p", false),
        ("en1999.6.2.9.nm.p", true),
        ("en1999.1-3.fat.fat-1", true),
        ("en1999.1-3.fat.miner.fat-1", true),
        ("en1999.1-3.fat.category.c", false),
        ("en1999.1-3.fat.site-ref.c", false),
        ("en1999.7.2.defl-qp.p", false),
        ("en1999.8.5.block.c", false),
        ("en1999.8.5.bolt.c", true),
        ("en1999.8.6.weld.c", true),
        ("en1999.1-5.ring.s", false),
        ("en1999.1-5.buckle.s", true),
        ("en1999.6.7.6.web-ref.s", false),
        ("en1999.6.7.6.web-strength.s", true),
    ] {
        assert_eq!(is_drawn_family(id), drawn, "{id}");
    }
}

/// 📊️ Every applicable check of a row outside the drawn families is a bar of its outcome diagram (none twice, no drawn check among them), the diagram has the limit at 1 and a table twin, and the focus selects it.
#[test]
fn every_applicable_check_is_drawn_by_a_dedicated_diagram_or_is_an_outcome_bar() {
    let mut diagrams_seen = 0;
    for document in [En1999Snapshot::full_showcase(), En1999Snapshot::noncompliant_multi_fail()] {
        let report = checks::evaluate_structure(&document);
        let rows = kit::Rows::of(&document);
        let all = diagrams(&document, &report, None);
        for owner in entities_of(&document).into_iter().map(|e| e.id) {
            let expected: Vec<&norm::CheckResult> = report.checks.iter().filter(|c| finite_applicable(c) && !kit::is_drawn_family(&c.id) && rows.owner(c).as_deref() == Some(owner.as_str())).collect();
            let found = all.iter().find(|d| d.id == format!("check-outcomes:{owner}"));
            assert_eq!(found.is_some(), !expected.is_empty(), "{owner}");
            let Some(found) = found else { continue };
            diagrams_seen += 1;
            assert_eq!(found.validate(), Ok(()), "{}", found.id);
            assert_eq!(found.markers.iter().filter_map(|m| m.bound_check.clone()).collect::<Vec<_>>(), expected.iter().map(|c| c.id.clone()).collect::<Vec<_>>(), "{owner}");
            assert_eq!(found.table_twin.rows(), expected.len());
            assert!(found.markers.iter().all(|m| !kit::is_drawn_family(m.bound_check.as_deref().unwrap_or(""))));
            assert!(found.series.iter().any(|s| matches!(s, scene::Series::Limit { value, .. } if (*value - 1.0).abs() < 1e-12)));
            assert!(!found.title.en.is_empty() && !found.title.de.is_empty() && found.title.en != found.title.de);
            assert!(diagrams(&document, &report, Some(&owner)).iter().any(|d| d.id == found.id), "the focus on {owner} keeps its outcome diagram");
        }
        assert!(diagrams(&document, &report, Some("no-such-row")).iter().all(|d| !d.id.starts_with("check-outcomes:")));
    }
    assert!(diagrams_seen >= 6, "{diagrams_seen} outcome diagrams");
}

fn column(diagram: &scene::Diagram2d, index: usize) -> &[f64] {
    &diagram.table_twin.columns[index].values
}

fn point_series<'a>(diagram: &'a scene::Diagram2d, id: &str) -> &'a [scene::Vec2] {
    diagram.series.iter().find_map(|s| match s {
        scene::Series::Polyline { id: found, points, .. } if found == id => Some(points.as_slice()),
        _ => None,
    }).unwrap_or_else(|| panic!("{}: no series {id}", diagram.id))
}

/// 🌀️ The torsional curve and the length diagram of a member carry the design point of the torsional check, the resistance falls with the torsional length of an unwelded member, and the length marker drags `bucklingLengthT` one to one.
#[test]
fn the_torsional_diagrams_carry_the_design_point_and_drag_the_torsional_length() {
    let mut seen = 0;
    for document in [En1999Snapshot::compliant_roof_purlin(), En1999Snapshot::full_showcase()] {
        let report = checks::evaluate_structure(&document);
        let all = diagrams(&document, &report, None);
        for member in &document.members {
            let Some((section, _)) = facts::member_parts(&document, member).filter(|(s, _)| !checks::is_free(s)) else { continue };
            let Some(check) = report.checks.iter().find(|c| c.id == format!("en1999.6.3.1.tb.{}", member.id) && c.utilization.is_finite()) else { continue };
            seen += 1;
            let curve = all.iter().find(|d| d.id == format!("torsional:{}", member.id)).unwrap_or_else(|| panic!("missing torsional:{}", member.id));
            let length = all.iter().find(|d| d.id == format!("torsional-length:{}", member.id)).unwrap_or_else(|| panic!("missing torsional-length:{}", member.id));
            for diagram in [curve, length] {
                assert_eq!(diagram.validate(), Ok(()), "{}", diagram.id);
                assert!(diagram.table_twin.rows() > 10 && !diagram.title.de.is_empty());
                assert_eq!(diagram.markers[0].bound_check.as_deref(), Some(check.id.as_str()));
                assert!((diagram.markers[0].utilisation.expect("utilisation") - check.utilization).abs() < 1e-12);
            }
            let chi = point_series(curve, "chi-t");
            assert!((chi[0].y - 1.0).abs() < 1e-12 && chi.windows(2).all(|w| w[1].y <= w[0].y + 1e-12), "{}: χ_T starts at 1 and falls", curve.id);
            let drag = length.markers[0].binding.as_ref().expect("length marker binding");
            assert_eq!(drag.path, format!("members[id={}].bucklingLengthT", member.id));
            assert!((drag.map.k - 1.0).abs() < 1e-12 && drag.map.base.abs() < 1e-12);
            let demand = length.series.iter().find_map(|s| match s { scene::Series::Limit { id, value, .. } if id == "n-ed" => Some(*value), _ => None }).expect("demand line");
            assert!((demand - check.computed.value.abs() / 1.0e3).abs() < 1e-9);
            if section.elements.iter().all(|e| !e.welded) {
                let resistance = column(length, 1);
                assert!(resistance.windows(2).all(|w| w[1] <= w[0] + 1e-9), "{}: N_b,T,Rd falls with the torsional length", length.id);
            }
        }
    }
    assert!(seen >= 1, "no member has a torsional diagram");
}

/// 🔥️ The welded-member diagram holds κ·χ = κ times χ column by column, reaches κ = 1 at large slenderness, and binds its design point to the HAZ buckling check.
#[test]
fn the_welded_member_diagram_multiplies_the_haz_factor_with_the_reduction_factor() {
    let mut seen = 0;
    for document in [En1999Snapshot::compliant_roof_purlin(), En1999Snapshot::full_showcase()] {
        let report = checks::evaluate_structure(&document);
        let all = diagrams(&document, &report, None);
        for member in &document.members {
            let Some(check) = report.checks.iter().find(|c| c.id == format!("en1999.6.3.1.haz.{}", member.id) && c.utilization.is_finite()) else { continue };
            let Some(diagram) = all.iter().find(|d| d.id == format!("haz-buckling:{}", member.id)) else { continue };
            seen += 1;
            assert_eq!(diagram.validate(), Ok(()), "{}", diagram.id);
            let (chi, kappa, both) = (column(diagram, 1), column(diagram, 2), column(diagram, 3));
            assert!(chi.iter().zip(kappa).zip(both).all(|((x, k), b)| (x * k - b).abs() < 1e-12), "{}: κ·χ", diagram.id);
            assert!((kappa[kappa.len() - 1] - 1.0).abs() < 1e-12 && kappa.iter().all(|k| *k > 0.0 && *k <= 1.0 + 1e-12), "{}: κ rises to 1", diagram.id);
            assert_eq!(diagram.markers[0].bound_check.as_deref(), Some(check.id.as_str()));
            assert!(diagram.markers[0].label.en.contains("ω_x") && diagram.markers[0].label.de.contains("ω_x"));
        }
    }
    assert!(seen >= 1, "no welded member has a HAZ buckling diagram");
}

/// 🔥️ The heat-affected zone drawn at a weld run names its strength factors ρ_o,haz and ρ_u,haz of the alloy at the plate thickness in its detail, in both languages, and a leader label sits at the zone.
#[test]
fn a_haz_zone_in_3d_carries_the_strength_factors_of_its_plate() {
    let document = En1999Snapshot::full_showcase();
    let report = checks::evaluate_structure(&document);
    let scene = scene_of(&document, &report);
    let zones: Vec<&scene::SceneEntity> = scene.entities.iter().filter(|e| e.entity.id.ends_with(":haz") && document.weld_runs.iter().any(|r| e.entity.id == format!("{}:haz", r.id))).collect();
    assert!(!zones.is_empty(), "no weld run draws a HAZ zone");
    for zone in zones {
        assert!(zone.accessible_detail.en.contains("ρ_o,haz") && zone.accessible_detail.en.contains("ρ_u,haz"), "{}", zone.entity.id);
        assert!(zone.accessible_detail.de.contains("ρ_o,haz") && zone.accessible_detail.de.contains("b_haz"), "{}", zone.entity.id);
        let run = zone.entity.id.trim_end_matches(":haz");
        assert!(scene.annotations.iter().any(|a| a.id() == format!("{run}:haz-rho")), "{run}: no ρ_haz label");
    }
}

fn normalise(path: &str) -> String {
    let mut out = String::new();
    let mut rest = path;
    while let Some(at) = rest.find("[id=") {
        out.push_str(&rest[..at]);
        out.push_str("[]");
        let end = rest[at..].find(']').expect("closing bracket") + at;
        rest = &rest[end + 1..];
    }
    out.push_str(rest);
    out
}

fn resolve<'a>(node: &'a Value, defs: &'a Value) -> &'a Value {
    match node.get("$ref").and_then(Value::as_str) {
        Some(reference) => &defs[reference.rsplit('/').next().unwrap_or("")],
        None => node,
    }
}

fn numeric_fields(node: &Value, defs: &Value, path: String, out: &mut BTreeMap<String, Option<f64>>) {
    let node = resolve(node, defs);
    for key in ["anyOf", "oneOf"] {
        if let Some(list) = node.get(key).and_then(Value::as_array) {
            list.iter().for_each(|item| numeric_fields(item, defs, path.clone(), out));
            return;
        }
    }
    if matches!(node.get("type").and_then(Value::as_str), Some("number" | "integer")) {
        out.insert(path.trim_start_matches('.').to_owned(), node["x-semio-ui"]["step"].as_f64());
    }
    if let Some(properties) = node.get("properties").and_then(Value::as_object) {
        properties.iter().for_each(|(name, item)| numeric_fields(item, defs, format!("{path}.{name}"), out));
    }
    if let Some(items) = node.get("items") {
        numeric_fields(items, defs, format!("{path}[]"), out);
    }
}

fn schema_fields() -> BTreeMap<String, Option<f64>> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../🧬️schema/📸️snapshot/🔣️.json");
    let schema: Value = serde_json::from_slice(&std::fs::read(path).expect("snapshot schema")).expect("json");
    let defs = schema.get("$defs").or_else(|| schema.get("definitions")).cloned().unwrap_or(Value::Null);
    let mut out = BTreeMap::new();
    numeric_fields(&schema, &defs, String::new(), &mut out);
    out
}

fn showcase_handles() -> (En1999Snapshot, Vec<scene::Handle>) {
    let document = En1999Snapshot::full_showcase();
    let report = checks::evaluate_structure(&document);
    let scene = scene_of(&document, &report);
    let selection: Vec<scene::EntityRef> = scene.entities.iter().map(|e| e.entity.clone()).collect();
    let all = handles(&document, &[], &selection);
    (document, all)
}

/// 🎚️ Every numeric field of the snapshot schema is written by a viewport grip of the full showcase (geometric grip or gauge) or listed in the documented exemptions, no leaf is written by two grips of one entity, and every gauge names a numeric schema field with the schema step.
#[test]
fn every_numeric_field_of_a_row_has_a_viewport_grip_or_a_documented_exemption() {
    let (document, all) = showcase_handles();
    assert_handle_laws(&all);
    let written: BTreeSet<String> = all.iter().flat_map(|h| h.writes.iter().map(|w| normalise(&w.path))).collect();
    let fields = schema_fields();
    assert!(fields.len() > 90, "{} numeric schema fields", fields.len());
    let present = |path: &str| {
        let nested = |prefix: &str, any: bool| !path.starts_with(prefix) || any;
        nested("members[].actions[]", document.members.iter().any(|m| !m.actions.is_empty()))
            && nested("connections[].actions[]", document.connections.iter().any(|m| !m.actions.is_empty()))
            && nested("coldFormed[].actions[]", document.cold_formed.iter().any(|m| !m.actions.is_empty()))
            && nested("shells[].actions[]", document.shells.iter().any(|m| !m.actions.is_empty()))
            && nested("sections[].elements[]", document.sections.iter().any(|m| !m.elements.is_empty()))
    };
    for path in fields.keys().filter(|p| present(p)) {
        let exempt = handles::gauges::EXEMPT.iter().any(|(exempt, _)| exempt == path);
        assert!(written.contains(path) || exempt, "{path} has no viewport grip");
        assert!(!(written.contains(path) && exempt), "{path} is exempt and written");
    }
    for (path, step) in handles::gauges::coverage() {
        let schema_step = fields.get(&path).unwrap_or_else(|| panic!("gauge {path} names no numeric schema field")).expect("schema step");
        let expected = handles::gauges::STEP_OVERRIDES.iter().find(|(p, _)| *p == path).map_or(schema_step, |(_, s)| *s);
        assert!((step - expected).abs() < 1e-12, "gauge {path}: step {step}, schema step {expected}");
    }
    let rules = semio_s_artifact_norm_en1999_mutations::EDIT_RULES;
    let gauges: Vec<&scene::Handle> = all.iter().filter(|h| h.id.contains(":gauge-")).collect();
    assert!(gauges.len() >= 40, "{} gauges", gauges.len());
    for h in &gauges {
        assert!(h.writes.iter().all(|w| rules.set_field.iter().any(|rule| rule.path == normalise(&w.path))), "gauge {} writes a leaf without a typed kind", h.id);
    }
    for entity in entities_of(&document) {
        let built = handles::build(&document, &layout::Layout::of(&document, &[]), &entity.id);
        let geometric: BTreeSet<&String> = built.iter().filter(|h| !h.id.contains(":gauge-")).flat_map(|h| h.writes.iter().map(|w| &w.path)).collect();
        let doubled: Vec<&String> = built.iter().filter(|h| h.id.contains(":gauge-")).flat_map(|h| h.writes.iter().map(|w| &w.path)).filter(|p| geometric.contains(p)).collect();
        assert!(doubled.is_empty(), "{}: gauge and geometric grip write {doubled:?}", entity.id);
    }
}

/// 🎚️ A gauge writes the current value of its field at zero drag, moves it by twenty schema steps per metre, and the dragged document re-evaluates with the dragged value.
#[test]
fn gauges_write_the_current_value_move_by_steps_and_the_dragged_document_re_evaluates() {
    let (document, all) = showcase_handles();
    let ids: BTreeSet<&str> = all.iter().map(|h| h.id.as_str()).collect();
    assert_eq!(ids.len(), all.len(), "handle ids are unique");
    let member = document.members.iter().find(|m| m.id == "purlin-welded").expect("welded member");
    let gauge = all.iter().find(|h| h.id == "purlin-welded:gauge-buckling-t").expect("torsional buckling length gauge");
    assert_eq!(gauge.writes[0].path, "members[id=purlin-welded].bucklingLengthT");
    assert!((gauge.writes_at(0.0)[0].value - member.buckling_length_t).abs() < 0.05, "zero drag keeps the value");
    let dragged = gauge.writes_at(1.0)[0].value;
    assert!((dragged - (member.buckling_length_t + 1.0)).abs() < 0.051, "one metre of drag is twenty steps of 0.05 m: {dragged}");
    let report = checks::evaluate_structure(&document);
    let mut moved = document.clone();
    moved.members.iter_mut().find(|m| m.id == "purlin-welded").expect("member").buckling_length_t = dragged;
    let tb = |d: &En1999Snapshot| checks::evaluate_structure(d).checks.into_iter().find(|c| c.id == "en1999.6.3.1.tb.purlin-welded").map(|c| c.limit.value);
    assert_ne!(tb(&moved), tb(&document), "the dragged torsional length changes N_b,T,Rd");
    let drawn = |d: &En1999Snapshot| diagrams(d, &checks::evaluate_structure(d), Some("purlin-welded")).into_iter().find(|x| x.id == "torsional-length:purlin-welded").map(|x| x.markers[0].at.x);
    assert_ne!(drawn(&moved), drawn(&document));
    assert!(report.checks.iter().any(|c| c.id == "en1999.6.3.1.tb.purlin-welded"));
}

/// 🏹 A gauge of an action row skips the leaf its load arrow already writes, so no leaf has two grips, and the fields without an arrow (here the torque) keep their gauge.
#[test]
fn an_action_gauge_skips_the_leaf_of_its_load_arrow() {
    let (document, all) = showcase_handles();
    let member = &document.members[0];
    for action in &member.actions {
        let path = |leaf: &str| format!("members[id={}].actions[id={}].{leaf}", member.id, action.id);
        let writes: Vec<&scene::Handle> = all.iter().filter(|h| h.writes.iter().any(|w| w.path == path("nK"))).collect();
        assert!(writes.len() <= 1, "{}: nK is written by {} grips", action.id, writes.len());
        let gauged = all.iter().any(|h| h.id == format!("{}:gauge-act-{}-torsion", member.id, action.id));
        assert!(gauged, "{}: the torque has no arrow, so it has a gauge", action.id);
    }
}
