//! 🧪️ Scene laws of ISO 16757: determinism, valid ids, layers, diagrams and the 1000-object timing.

use super::*;
use model::Iso16757Snapshot;

fn report_of(document: &Iso16757Snapshot) -> CheckReport {
    semio_s_artifact_norm_iso16757_inferences::evaluate(document)
}

fn input(document: &Iso16757Snapshot, report: &CheckReport) -> SceneInput<'static> {
    let leaked: &'static Iso16757Snapshot = Box::leak(Box::new(document.clone()));
    let report: &'static CheckReport = Box::leak(Box::new(report.clone()));
    SceneInput { document: leaked, report, placements: &[], revision: SceneRevision { document_seq: 1, report_seq: 1 } }
}

#[test]
fn the_reference_fixture_projects_a_valid_deterministic_scene() {
    let document = Iso16757Snapshot::reference_fixture();
    let report = report_of(&document);
    let first = project(&input(&document, &report));
    let second = project(&input(&document, &report));
    assert_eq!(first, second);
    assert!(first.entities.iter().any(|e| e.entity.id == "geom-valve-50"));
    assert!(first.entities.iter().any(|e| e.entity.id == "geom-valve-50:space-installation"));
    assert!(first.entities.iter().any(|e| e.entity.id == "geom-valve-50:port-port-in"));
    assert!(first.entities.iter().all(|e| first.layers.iter().any(|l| l.id == e.layer)));
}

#[test]
fn every_object_has_a_plan_and_an_elevation_with_table_twins() {
    let document = Iso16757Snapshot::reference_fixture();
    let all = diagrams(&document, &report_of(&document), None);
    assert_eq!(all.len(), 2 * document.geometry.objects.len());
    assert!(all.iter().all(|d| !d.table_twin.columns.is_empty()));
    assert_eq!(diagrams(&document, &report_of(&document), Some("geom-valve-50:port-port-in")).len(), 2);
}

#[test]
fn handles_exist_for_the_selected_object_and_are_unique() {
    let document = Iso16757Snapshot::reference_fixture();
    let selection = entities_of(&document);
    let grips = handles(&document, &[], &selection);
    let spaces = 3 * document.geometry.objects.values().map(|o| o.spaces.len()).sum::<usize>();
    let shape: Vec<_> = grips.iter().filter(|h| h.id.contains(":shape-")).collect();
    assert!(!shape.is_empty() && grips.len() == spaces + shape.len(), "primitive dimension grips come on top of the envelope grips");
    let width = grips.iter().find(|h| h.id == "geom-valve-50:shape-width").expect("width grip");
    assert_eq!(width.writes[0].path, "geometry.objects.geom-valve-50.shape.parameters.width");
    let stored = match document.geometry.objects["geom-valve-50"].shape.as_ref() {
        Some(model::part_2::GeometryNode::Primitive { parameters, .. }) => parameters["width"],
        other => panic!("expected a primitive, got {other:?}"),
    };
    assert!((width.writes[0].map.apply(stored) - stored).abs() < 1e-12);
    let mut ids: Vec<&str> = grips.iter().map(|h| h.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), grips.len());
}

#[test]
fn a_thousand_objects_project_within_the_budget() {
    let mut document = Iso16757Snapshot::reference_fixture();
    let template = document.geometry.objects["geom-valve-50"].clone();
    for index in 0..1000 {
        let id = format!("geom-{index}");
        document.geometry.objects.insert(id.clone(), model::part_2::GeometryObject { id, ..template.clone() });
    }
    let started = std::time::Instant::now();
    let scene = project(&input(&document, &report_of(&document)));
    assert!(scene.entities.len() >= 1000);
    assert!(started.elapsed().as_secs() < 30);
}

const ORACLE: &str = include_str!("../🧫️fixtures/🔣️.json");

fn shoelace(points: &[scene::Vec2]) -> f64 {
    0.5 * points.windows(2).map(|w| w[0].x * w[1].y - w[1].x * w[0].y).sum::<f64>().abs()
}

fn rect_of(points: &[scene::Vec2]) -> (f64, f64, f64, f64) {
    let xs = points.iter().map(|p| p.x);
    let ys = points.iter().map(|p| p.y);
    (xs.clone().fold(f64::MAX, f64::min), xs.fold(f64::MIN, f64::max), ys.clone().fold(f64::MAX, f64::min), ys.fold(f64::MIN, f64::max))
}

#[test]
fn the_plan_diagram_matches_the_shapely_oracle() {
    let cases: serde_json::Value = serde_json::from_str(ORACLE).expect("oracle fixture");
    for case in cases.as_array().expect("cases") {
        let num = |v: &serde_json::Value| v.as_f64().expect("number");
        let vec3 = |v: &serde_json::Value| -> [f64; 3] { let a = v.as_array().expect("vec3"); [num(&a[0]), num(&a[1]), num(&a[2])] };
        let id = case["id"].as_str().expect("id");
        let mut document = Iso16757Snapshot::reference_fixture();
        let parameters = ["width", "depth", "height"].map(|k| (k.to_string(), num(&case["body"][k])));
        let mut object = document.geometry.objects["geom-valve-50"].clone();
        object.id = id.to_string();
        object.shape = Some(model::part_2::GeometryNode::Primitive { kind: "box".into(), parameters: parameters.into_iter().collect() });
        object.ports.clear();
        object.spaces = case["spaces"].as_array().expect("spaces").iter().map(|s| model::part_2::SpaceEnvelope { id: s["id"].as_str().expect("space id").into(), kind: model::part_2::SpaceKind::Installation, bounds: model::part_2::BoundingBox { min: vec3(&s["min"]), max: vec3(&s["max"]) } }).collect();
        document.geometry.objects.clear();
        document.geometry.objects.insert(id.to_string(), object);
        let plan = diagrams(&document, &report_of(&document), Some(id)).into_iter().find(|d| d.id == format!("plan:{id}")).expect("plan diagram");
        let ring = |series_id: &str| match plan.series.iter().find(|s| matches!(s, scene::Series::Polyline { id, .. } if id == series_id)) {
            Some(scene::Series::Polyline { points, .. }) => points.clone(),
            _ => panic!("series {series_id}"),
        };
        let expected = &case["expected"];
        let body = ring("body-0");
        assert!((shoelace(&body) - num(&expected["bodyPlanArea"])).abs() < 1e-12, "{id} body area");
        let (bx0, bx1, by0, by1) = rect_of(&body);
        for (index, space) in case["spaces"].as_array().expect("spaces").iter().enumerate() {
            let points = ring(&format!("space-{}", space["id"].as_str().expect("space id")));
            assert!((shoelace(&points) - num(&expected["spacePlanAreas"][index])).abs() < 1e-12, "{id} space area");
            let (sx0, sx1, sy0, sy1) = rect_of(&points);
            let overlap = (bx1.min(sx1) - bx0.max(sx0)).max(0.0) * (by1.min(sy1) - by0.max(sy0)).max(0.0);
            assert!((overlap - num(&expected["spaceOverlapAreas"][index])).abs() < 1e-12, "{id} overlap");
        }
        let extent = crate::layout::extent(&document, &document.geometry.objects[id]);
        assert!((extent.min[0] - num(&expected["extentX"][0])).abs() < 1e-12 && (extent.max[0] - num(&expected["extentX"][1])).abs() < 1e-12, "{id} extent x");
        assert!((extent.min[2] - num(&expected["extentZ"][0])).abs() < 1e-12 && (extent.max[2] - num(&expected["extentZ"][1])).abs() < 1e-12, "{id} extent z");
    }
}

#[test]
fn scene_bounds_equal_check_bounds_for_a_rotated_primitive() {
    let mut document = Iso16757Snapshot::reference_fixture();
    let mut object = document.geometry.objects["geom-valve-50"].clone();
    object.spaces.clear();
    object.ports.clear();
    object.surfaces.clear();
    let primitive = model::part_2::GeometryNode::Primitive { kind: "box".into(), parameters: [("width".to_string(), 2.0), ("height".to_string(), 1.0), ("depth".to_string(), 0.5)].into_iter().collect() };
    object.shape = Some(model::part_2::GeometryNode::Transform { translation: [0.3, 0.0, 0.0], rotation_deg: [0.0, 0.0, 45.0], child: Box::new(primitive) });
    document.geometry.objects.clear();
    document.geometry.objects.insert(object.id.clone(), object.clone());
    let check = model::part_2::solid::bounding_box(object.shape.as_ref().expect("shape"), &document.geometry).expect("check bounds");
    let extent = crate::layout::extent(&document, &object);
    for axis in 0..3 {
        assert!((extent.min[axis] - check.min[axis]).abs() < 1e-12 && (extent.max[axis] - check.max[axis]).abs() < 1e-12, "extent axis {axis}");
    }
    assert!((check.max[0] - check.min[0] - (2.0 + 1.0) * std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-12, "rotation widens the width extent");
    let scene = project(&input(&document, &report_of(&document)));
    let body = scene.entities.iter().find(|e| e.entity.id == object.id).expect("body");
    let scene::Geometry::Solid(scene::SolidGeometry { shape: scene::SolidShape::Mesh { vertices, .. } }) = &body.geometry else { panic!("the body is no mesh") };
    let span = |pick: &dyn Fn(&scene::Vec3) -> f64| vertices.iter().map(pick).fold(f64::MIN, f64::max) - vertices.iter().map(pick).fold(f64::MAX, f64::min);
    let size = [check.max[0] - check.min[0], check.max[1] - check.min[1], check.max[2] - check.min[2]];
    assert!((span(&|v| v.y) - size[0]).abs() < 1e-9 && (span(&|v| v.z) - size[1]).abs() < 1e-9 && (span(&|v| v.x) - size[2]).abs() < 1e-9, "world Y, Z, X span the width, height, depth of the check bounds");
}
