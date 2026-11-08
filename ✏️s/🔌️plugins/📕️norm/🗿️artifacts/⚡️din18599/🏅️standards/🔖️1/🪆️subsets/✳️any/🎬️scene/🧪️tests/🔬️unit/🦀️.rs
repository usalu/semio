use super::*;
use scene::projection_kit::test_support::{assert_diagram_laws, assert_handle_laws, assert_scene_laws, entity, revision};
use scene::canonical_json;
use std::collections::BTreeSet;

fn documents() -> Vec<Din18599Snapshot> {
    vec![model::subjects::compliant_detached_house(), model::subjects::noncompliant_detached_house(), model::subjects::compliant_two_zone_house(), model::subjects::cooled_office_building()]
}

fn scene_of(document: &Din18599Snapshot) -> (StructuralScene, CheckReport) {
    let report = checks::evaluate_document(document);
    (project(&SceneInput { document, report: &report, revision: revision() }), report)
}

fn selection(document: &Din18599Snapshot) -> Vec<EntityRef> {
    entities_of(document)
}

#[test]
fn the_scene_of_every_fixture_obeys_the_scene_laws() {
    for document in documents() {
        let (scene, _) = scene_of(&document);
        assert_scene_laws(&scene);
        assert_eq!(scene.entities.len(), document.zones.len() + document.elements.len());
    }
}

#[test]
fn every_zone_and_element_stands_in_the_scene_with_its_collection_path() {
    let document = model::subjects::compliant_two_zone_house();
    let (scene, _) = scene_of(&document);
    for zone in &document.zones {
        assert_eq!(entity(&scene, &zone.id).entity.path, format!("zones[id={}]", zone.id));
    }
    for element in &document.elements {
        assert_eq!(entity(&scene, &element.id).entity.path, format!("elements[id={}]", element.id));
    }
}

#[test]
fn projection_is_deterministic_and_bilingual() {
    let document = model::subjects::cooled_office_building();
    let (a, _) = scene_of(&document);
    let (b, _) = scene_of(&document);
    assert_eq!(canonical_json(&a), canonical_json(&b));
}

#[test]
fn live_scene_equals_the_full_projection() {
    for document in documents() {
        let (full, report) = scene_of(&document);
        let mut live = live_scene_of(&document, &report);
        live.revision = full.revision.clone();
        assert_eq!(canonical_json(&live), canonical_json(&full));
    }
}

#[test]
fn zone_boxes_have_the_documented_size_and_never_overlap() {
    let document = model::subjects::compliant_two_zone_house();
    let layout = Layout::of(&document);
    let mut right = f64::NEG_INFINITY;
    for zone in &document.zones {
        let placed = layout.zone(&zone.id).expect("zone placed");
        assert!((placed.side * placed.side - zone.area_m2).abs() < 1e-9);
        assert!(placed.centre.x - placed.side / 2.0 >= right);
        right = placed.centre.x + placed.side / 2.0;
    }
}

#[test]
fn a_failing_element_is_coloured_by_its_check() {
    let document = model::subjects::noncompliant_detached_house();
    let (scene, report) = scene_of(&document);
    let failing: BTreeSet<&str> = report.checks.iter().filter(|c| c.status == norm::CheckStatus::Fail).map(|c| c.subject.entity_id.as_str()).collect();
    for element in document.elements.iter().filter(|e| failing.contains(e.id.as_str())) {
        assert!(entity(&scene, &element.id).utilisation.is_some(), "{} carries its binding", element.id);
    }
}

#[test]
fn diagrams_obey_the_diagram_laws_and_the_monthly_balance_sums_to_the_annual_demand() {
    for document in documents() {
        let report = checks::evaluate_document(&document);
        let all = diagrams(&document, &report, None);
        assert_diagram_laws(&all);
        for (id, months) in checks::zone_heating_months(&document) {
            let balance = checks::zone_balances(&document, &document.elements, document.delta_u_wb_w_m2k);
            let zone = balance.iter().find(|z| z.zone_id == id).expect("zone balance");
            assert!((months.iter().map(|m| m.q_h_kwh).sum::<f64>() - zone.q_h_nd_kwh).abs() < 1e-9);
            assert!((months.iter().map(|m| m.q_c_kwh).sum::<f64>() - zone.q_c_nd_kwh).abs() < 1e-9);
        }
    }
}

#[test]
fn the_energy_flow_primary_energy_matches_the_balance() {
    let document = model::subjects::cooled_office_building();
    let derived = checks::derive_balance(&document);
    let primary: f64 = flows(&document).iter().map(|f| f.2[2]).sum();
    let credit = derived.q_pv_kwh * checks::primary_energy_factor("electricity");
    assert!((primary - credit - derived.q_p_kwh).abs() < 1e-6, "{primary} - {credit} vs {}", derived.q_p_kwh);
}

#[test]
fn a_focus_restricts_the_diagrams_to_the_entity_and_its_zone() {
    let document = model::subjects::compliant_two_zone_house();
    let report = checks::evaluate_document(&document);
    let element = &document.elements[0];
    let ids: Vec<String> = diagrams(&document, &report, Some(&element.id)).into_iter().map(|d| d.id).collect();
    assert!(ids.contains(&format!("element-u:{}", element.id)));
    assert!(ids.contains(&format!("monthly-balance:{}", element.zone_id)));
    assert!(ids.contains(&"energy-flow:building".to_string()));
    assert!(ids.len() < diagrams(&document, &report, None).len());
}

#[test]
fn handles_are_bilingual_write_one_leaf_and_round_trip_the_current_value() {
    for document in documents() {
        let all = handles(&document, &selection(&document));
        assert!(!all.is_empty());
        assert_handle_laws(&all);
        for h in &all {
            let write = &h.writes[0];
            let s = h.anchor.x * axis(h).x + h.anchor.y * axis(h).y + h.anchor.z * axis(h).z;
            let value = write.map.apply(s);
            if let Some(id) = write.path.strip_prefix("elements[id=").and_then(|r| r.split(']').next()) {
                let element = document.elements.iter().find(|e| e.id == id).expect("element");
                assert!((value - element.u_value_w_m2k).abs() < 1e-9, "{}: {value} vs {}", h.id, element.u_value_w_m2k);
            }
        }
    }
}

fn axis(handle: &scene::Handle) -> scene::Vec3 {
    match &handle.kind {
        scene::HandleKind::ScalarAlong { axis } => axis.clone(),
        other => panic!("unexpected handle kind {other:?}"),
    }
}

#[test]
fn the_monthly_balance_matches_the_independent_numpy_oracle() {
    let oracle = semio_framework_test_fixtures::fixture_path!("../../🔮️oracles/🐍️.py");
    let tmp = std::env::temp_dir().join("semio-norm-din18599-scene-oracle");
    std::fs::create_dir_all(&tmp).expect("tmp");
    for (name, document) in [("house", model::subjects::compliant_detached_house()), ("poor", model::subjects::noncompliant_detached_house()), ("two-zone", model::subjects::compliant_two_zone_house()), ("office", model::subjects::cooled_office_building())] {
        let snapshot = tmp.join(format!("{name}.json"));
        std::fs::write(&snapshot, semio_s_artifact_norm_din18599_io::text::snapshot::encode_din18599_snapshot_json(&document)).expect("snapshot");
        let out = norm::python_oracle::command().arg(&oracle).arg(&snapshot).output().expect("run the oracle");
        assert!(out.status.success(), "{name}: {}", String::from_utf8_lossy(&out.stderr));
        let expected: serde_json::Value = serde_json::from_slice(&out.stdout).expect("oracle json");
        let near = |a: f64, b: f64| (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.0);
        for ((id, months), reference) in checks::zone_heating_months(&document).iter().zip(expected["zones"].as_array().expect("zones")) {
            assert_eq!(reference["id"], serde_json::Value::String(id.clone()));
            for (key, pick) in [("qT", (|m: &checks::HeatingMonth| m.q_t_kwh) as fn(&checks::HeatingMonth) -> f64), ("qV", |m| m.q_v_kwh), ("qI", |m| m.q_i_kwh), ("qS", |m| m.q_s_kwh), ("eta", |m| m.eta_utilisation), ("qH", |m| m.q_h_kwh), ("qC", |m| m.q_c_kwh)] {
                for (month, value) in months.iter().zip(reference[key].as_array().expect("twelve values")) {
                    let value = value.as_f64().expect("number");
                    assert!(near(pick(month), value), "{name}/{id}/{key}: {} vs {value}", pick(month));
                }
            }
        }
    }
}
