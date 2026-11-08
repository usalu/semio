use super::*;
use scene::projection_kit::test_support::{assert_golden, assert_basis_follows_annex, assert_diagram_laws, assert_handle_laws, assert_scene_laws, entity, revision};
use scene::{canonical_json, Diagram2d, Series, Validate};
use std::collections::BTreeSet;

fn scene_of(document: &Din16798Snapshot) -> StructuralScene {
    let report = inferences::evaluate(document);
    project(&SceneInput { document, report: &report, revision: revision() })
}

fn fixtures() -> Vec<(&'static str, Din16798Snapshot)> {
    let mut two_systems = Din16798Snapshot::compliant_office();
    let mut second = two_systems.zones[0].clone();
    second.id = "zone-meeting".into();
    second.name = "Meeting room".into();
    second.comfort_category = "I".into();
    second.floor_area_m2 = 60.0;
    second.occupants = 8;
    two_systems.zones.push(second);
    vec![
        ("compliant", Din16798Snapshot::compliant_office()),
        ("residential", Din16798Snapshot::residential_method3()),
        ("noncompliant", Din16798Snapshot::noncompliant_office()),
        ("two-zones", two_systems),
        ("empty", Din16798Snapshot { zones: vec![], vent_systems: vec![], ..Din16798Snapshot::compliant_office() }),
    ]
}

#[test]
fn the_scene_of_every_fixture_validates() {
    for (name, document) in fixtures() {
        let scene = scene_of(&document);
        assert_scene_laws(&scene);
        assert_eq!(scene.validate(), Ok(()), "violations in {name}");
    }
}

#[test]
fn ids_are_unique_and_layers_exist() {
    for (name, document) in fixtures() {
        let scene = scene_of(&document);
        let ids: BTreeSet<_> = scene.entities.iter().map(|e| e.entity.id.clone()).collect();
        assert_eq!(ids.len(), scene.entities.len(), "{name}");
        let layers: BTreeSet<_> = scene.layers.iter().map(|l| l.id.as_str()).collect();
        assert!(scene.entities.iter().all(|e| layers.contains(e.layer.as_str())), "{name}");
    }
}

#[test]
fn projection_is_deterministic() {
    let document = Din16798Snapshot::noncompliant_office();
    assert_eq!(canonical_json(&scene_of(&document)), canonical_json(&scene_of(&document)));
}

#[test]
fn the_live_scene_equals_the_plain_projection() {
    for (name, document) in fixtures() {
        let report = inferences::evaluate(&document);
        let (mut live, mut full) = (live_scene_of(&document, &report), full_scene_of(&document, &report));
        live.revision = SceneRevision { document_seq: 0, report_seq: 0 };
        full.revision = SceneRevision { document_seq: 0, report_seq: 0 };
        assert_eq!(canonical_json(&live), canonical_json(&full), "{name}");
    }
}

#[test]
fn every_row_of_the_document_is_a_scene_entity() {
    for (name, document) in fixtures() {
        let scene = scene_of(&document);
        let ids: BTreeSet<_> = scene.entities.iter().map(|e| e.entity.id.as_str()).collect();
        for entity in entities_of(&document) {
            assert!(ids.contains(entity.id.as_str()), "{name}: {} is not drawn", entity.id);
        }
    }
}

#[test]
fn a_room_is_a_box_of_its_floor_area_coloured_by_its_category() {
    let mut document = Din16798Snapshot::compliant_office();
    let scene = scene_of(&document);
    let room = entity(&scene, "zone-office");
    let scene::Geometry::Solid(scene::SolidGeometry { shape: scene::SolidShape::Box { size, .. } }) = &room.geometry else { panic!("zone-office is no box") };
    assert!((size.x * size.y - document.zones[0].floor_area_m2).abs() < 1e-9, "the footprint draws the floor area");
    assert_eq!(room.appearance.role, scene::MaterialRole::Aluminium);
    for (token, role) in [("I", scene::MaterialRole::Glass), ("II", scene::MaterialRole::Aluminium), ("III", scene::MaterialRole::Timber), ("IV", scene::MaterialRole::Masonry)] {
        document.zones[0].comfort_category = token.into();
        assert_eq!(entity(&scene_of(&document), "zone-office").appearance.role, role, "category {token}");
    }
}

#[test]
fn the_failing_check_colours_its_room_and_ventilation_system() {
    let scene = scene_of(&Din16798Snapshot::noncompliant_office());
    assert_eq!(entity(&scene, "zone-office-fail").appearance.style, scene::StyleState::Error);
    assert_eq!(entity(&scene, "vent-weak").appearance.style, scene::StyleState::Error);
    let compliant = scene_of(&Din16798Snapshot::compliant_office());
    assert_eq!(entity(&compliant, "zone-office").appearance.style, scene::StyleState::Normal);
}

#[test]
fn diagrams_obey_the_laws() {
    for (name, document) in fixtures() {
        let report = inferences::evaluate(&document);
        let all = diagrams(&document, &report, None);
        assert_diagram_laws(&all);
        let expected = document.zones.len() * 5 + usize::from(!document.vent_systems.is_empty());
        assert_eq!(all.len(), expected, "{name}");
    }
}

#[test]
fn the_basis_badge_follows_the_annex() {
    assert_basis_follows_annex(norm::NormFamilyId::DinEn16798, |annex| {
        let document = Din16798Snapshot { annex, ..Din16798Snapshot::compliant_office() };
        let report = inferences::evaluate(&document);
        (live_scene_of(&document, &report), diagrams(&document, &report, None))
    });
}

#[test]
fn an_unknown_focus_has_no_diagrams_and_a_room_focus_has_its_own() {
    let document = Din16798Snapshot::compliant_office();
    let report = inferences::evaluate(&document);
    assert!(diagrams(&document, &report, Some("nope")).is_empty());
    let own = diagrams(&document, &report, Some("zone-office"));
    for kind in ["adaptive", "operative", "ventilation", "co2", "humidity"] {
        assert!(own.iter().any(|d| d.id == format!("{kind}:zone-office")), "{kind}");
    }
    assert!(own.iter().any(|d| d.id == "capacity:building"));
}

fn series<'a>(diagram: &'a Diagram2d, id: &str) -> &'a Series {
    diagram.series.iter().find(|s| s.id() == id).unwrap_or_else(|| panic!("{} has no series {id}", diagram.id))
}

#[test]
fn the_adaptive_bands_follow_the_closed_form() {
    let document = Din16798Snapshot::compliant_office();
    let report = inferences::evaluate(&document);
    let all = diagrams(&document, &report, Some("zone-office"));
    let adaptive = all.iter().find(|d| d.id == "adaptive:zone-office").expect("adaptive diagram");
    let Series::Band { lower, upper, .. } = series(adaptive, "category-II") else { panic!("band") };
    for (low, high) in lower.iter().zip(upper) {
        let centre = 0.33 * low.x + 18.8;
        assert!((low.y - (centre - 3.0)).abs() < 1e-9 && (high.y - (centre + 3.0)).abs() < 1e-9);
    }
    let running_mean = adaptive.markers.iter().find(|m| m.id == "running-mean").expect("running-mean marker");
    assert_eq!(running_mean.binding.as_ref().map(|b| b.path.as_str()), Some("thetaRmC"));
    assert!((running_mean.at.x - document.theta_rm_c).abs() < 1e-12);
}

#[test]
fn the_ventilation_bars_stack_persons_and_building_components() {
    let document = Din16798Snapshot::compliant_office();
    let zone = &document.zones[0];
    let (persons, building) = diagrams::ventilation_components(zone, checks::ComfortCategory::II);
    assert!((persons - 20.0 * 7.0 * 3.6).abs() < 1e-9);
    assert!((building - 200.0 * 0.7 * 3.6).abs() < 1e-9);
    assert!((persons + building - 1008.0).abs() < 1e-9, "the default supply equals the category II requirement");
    let report = inferences::evaluate(&document);
    let all = diagrams(&document, &report, Some("zone-office"));
    let rates = all.iter().find(|d| d.id == "ventilation:zone-office").expect("ventilation diagram");
    assert_eq!(rates.table_twin.columns.len(), 4);
    let supplied = rates.markers.iter().find(|m| m.id == "supplied").expect("supplied marker");
    assert_eq!(supplied.binding.as_ref().map(|b| b.path.as_str()), Some("zones[id=zone-office].outdoorAirSuppliedM3H"));
}

#[test]
fn handles_are_lawful_and_write_document_leaves() {
    let document = Din16798Snapshot::compliant_office();
    let selection: Vec<_> = entities_of(&document);
    let all = handles(&document, &selection);
    assert!(!all.is_empty());
    assert_handle_laws(&all);
    let ids: BTreeSet<_> = all.iter().map(|h| h.id.as_str()).collect();
    for id in ["zone-office:t-op-summer", "zone-office:t-op-winter", "zone-office:floor-area", "zone-office:outdoor-air", "zone-office:co2", "zone-office:occupants", "zone-office:category", "vent-central:design-airflow", "vent-central:heat-recovery", "vent-central:sfp"] {
        assert!(ids.contains(id), "{id}");
    }
    let summer = all.iter().find(|h| h.id == "zone-office:t-op-summer").expect("summer grip");
    assert_eq!(summer.writes[0].path, "zones[id=zone-office].tOpSummerC");
    assert!((summer.writes[0].value_at(summer.anchor.z) - document.zones[0].t_op_summer_c).abs() < 1e-9, "the grip rests on the stored value");
    let area = all.iter().find(|h| h.id == "zone-office:floor-area").expect("area grip");
    assert!((area.writes[0].value_at(area.anchor.x) - document.zones[0].floor_area_m2).abs() < 1e-9);
    let airflow = all.iter().find(|h| h.id == "vent-central:design-airflow").expect("airflow grip");
    assert!((airflow.writes[0].value_at(airflow.anchor.x) - document.vent_systems[0].design_airflow_m3_h).abs() < 1e-9);
}

#[test]
fn every_handle_rests_inside_its_limits() {
    for (name, document) in fixtures() {
        for handle in handles(&document, &entities_of(&document)) {
            let rest = match handle.kind {
                scene::HandleKind::ScalarAlong { axis } => handle.anchor.x * axis.x + handle.anchor.y * axis.y + handle.anchor.z * axis.z,
                _ => continue,
            };
            let limits = handle.limits.expect("limits");
            assert!(rest >= limits.min - 1e-9 && rest <= limits.max + 1e-9 || name == "noncompliant", "{name}: {} rests at {rest} outside {limits:?}", handle.id);
        }
    }
}

#[test]
fn every_check_is_drawn_on_its_row_or_the_badge() {
    for (name, document) in fixtures() {
        let report = inferences::evaluate(&document);
        for check in &report.checks {
            assert!(route_of(&document, check).is_some(), "{name}: {} has no route", check.id);
        }
    }
}

fn polyline_json(points: &[scene::Vec2]) -> String {
    format!("[{}]", points.iter().map(|p| format!("[{},{}]", p.x, p.y)).collect::<Vec<_>>().join(","))
}

#[test]
fn the_independent_oracle_agrees() {
    let mut rows: Vec<String> = Vec::new();
    for (_, document) in fixtures().into_iter().filter(|(name, _)| *name != "empty") {
        let report = inferences::evaluate(&document);
        let all = diagrams(&document, &report, None);
        for zone in &document.zones {
            let adaptive = all.iter().find(|d| d.id == format!("adaptive:{}", zone.id)).expect("adaptive diagram");
            let Series::Band { lower, upper, .. } = series(adaptive, &format!("category-{}", rooms::category_name(rooms::category_number(zone)))) else { panic!("band") };
            let ventilation = all.iter().find(|d| d.id == format!("ventilation:{}", zone.id)).expect("ventilation diagram");
            let totals = &ventilation.table_twin.columns[3].values;
            let co2 = all.iter().find(|d| d.id == format!("co2:{}", zone.id)).expect("co2 diagram");
            rows.push(format!(
                "{{\"category\":{},\"annex\":\"{}\",\"usage\":\"{}\",\"pollution\":\"{}\",\"occupants\":{},\"area\":{},\"outdoor_co2\":{},\"lower\":{},\"upper\":{},\"ventilation_totals\":{:?},\"co2_limits\":{:?}}}",
                rooms::category_number(zone),
                if document.annex == norm::AnnexChoice::De { "de" } else { "en" },
                zone.usage_type,
                zone.pollution_class,
                zone.occupants,
                zone.floor_area_m2,
                document.outdoor_co2_ppm,
                polyline_json(lower),
                polyline_json(upper),
                totals,
                co2.table_twin.columns[1].values,
            ));
        }
    }
    let input = format!("{{\"zones\":[{}]}}", rows.join(","));
    let script = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔮️oracles/🐍️.py");
    let output = norm::python_oracle::run_ok(script, &[], Some(input.as_bytes()));
    let verdict = String::from_utf8_lossy(&output.stdout);
    assert!(verdict.contains("\"ok\": true"), "oracle verdict: {verdict}");
}

#[test]
fn projecting_a_thousand_rooms_stays_interactive() {
    let mut document = Din16798Snapshot::compliant_office();
    let template = document.zones[0].clone();
    document.zones = (0..1000).map(|i| model::ZoneDocument { id: format!("z{i}"), ..template.clone() }).collect();
    let report = inferences::evaluate(&document);
    let started = std::time::Instant::now();
    let scene = project(&SceneInput { document: &document, report: &report, revision: revision() });
    assert!(scene.entities.len() >= 1000);
    assert!(started.elapsed() < std::time::Duration::from_secs(5), "projection took {:?}", started.elapsed());
}

#[test]
fn the_golden_scene_matches_the_committed_bytes() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧫️fixtures/🏛️scene/🔣️.json");
    assert_golden(path, &canonical_json(&scene_of(&Din16798Snapshot::compliant_office())));
}
