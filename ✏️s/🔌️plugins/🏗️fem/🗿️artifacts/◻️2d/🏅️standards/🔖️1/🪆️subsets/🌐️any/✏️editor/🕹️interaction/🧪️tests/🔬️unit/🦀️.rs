use super::*;
use store::ArtifactDsl;

const CANVAS_WIDTH: f64 = 800.0;
const CANVAS_HEIGHT: f64 = 600.0;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("demo document parses")
}

fn canvas_point(camera: &Viewport2d, model: (f64, f64)) -> (f64, f64) {
    fem2d_layer_to_canvas(camera, screen_2d(model.0, model.1), CANVAS_WIDTH, CANVAS_HEIGHT)
}

fn pick(doc: &Fem2dSnapshot, camera: &Viewport2d, point: (f64, f64)) -> Option<Fem2dPick> {
    fem2d_hit_test(doc, camera, point.0, point.1, CANVAS_WIDTH, CANVAS_HEIGHT)
}

#[test]
fn canvas_and_model_projections_round_trip() {
    let camera = Viewport2d { x: 137.0, y: -42.0, zoom: 2.25 };
    let model = (3.5, -1.25);
    let canvas = canvas_point(&camera, model);
    let back = fem2d_canvas_to_model(&camera, canvas.0, canvas.1, CANVAS_WIDTH, CANVAS_HEIGHT);
    assert!((back.0 - model.0).abs() < 1e-9 && (back.1 - model.1).abs() < 1e-9, "{back:?} != {model:?}");
}

#[test]
fn hit_test_picks_the_node_under_the_pointer_and_nothing_in_empty_space() {
    let doc = demo();
    let camera = Viewport2d::default();
    let n1 = find_node_2d(&doc.nodes, "n1").expect("demo carries n1");
    assert_eq!(screen_2d(n1.x, n1.y), (40.0, 120.0), "demo n1 sits at model (0,-4)");
    assert_eq!(pick(&doc, &camera, canvas_point(&camera, (n1.x, n1.y))), Some((FEM2D_GRANULARITY_NODE, "n1".to_string())));
    assert_eq!(pick(&doc, &camera, (CANVAS_WIDTH - 4.0, 4.0)), None, "the far corner of the canvas hits nothing");
}

#[test]
fn hit_test_resolves_members_regions_supports_and_loads() {
    let doc = demo();
    let camera = Viewport2d::default();
    let n1 = find_node_2d(&doc.nodes, "n1").expect("n1");
    let n2 = find_node_2d(&doc.nodes, "n2").expect("n2");
    let midpoint = ((n1.x + n2.x) * 0.5, (n1.y + n2.y) * 0.5);
    assert_eq!(pick(&doc, &camera, canvas_point(&camera, midpoint)), Some((FEM2D_GRANULARITY_ELEMENT, "e3".to_string())), "the column e3 spans n1..n2");

    let region = doc.regions.iter().find(|region| region.id == "r1").expect("r1");
    let centroid = fem2d_region_centroid(region).expect("centroid");
    assert_eq!(pick(&doc, &camera, canvas_point(&camera, centroid)), Some((FEM2D_GRANULARITY_LOAD, "l5".to_string())), "the area-load arrow is anchored at the region centroid and wins over the region");
    let inside = (region.outline[0][0] + 1.4, (region.outline[0][1] + region.outline[2][1]) * 0.5);
    assert_eq!(pick(&doc, &camera, canvas_point(&camera, inside)), Some((FEM2D_GRANULARITY_REGION, "r1".to_string())));

    let rc0 = find_node_2d(&doc.nodes, "rc0").expect("rc0");
    let ring = canvas_point(&camera, (rc0.x, rc0.y));
    assert_eq!(pick(&doc, &camera, (ring.0 - 10.0, ring.1)), Some((FEM2D_GRANULARITY_SUPPORT, "s3".to_string())), "just outside the node radius the support glyph wins");
}

#[test]
fn hit_test_follows_the_camera() {
    let doc = demo();
    let camera = Viewport2d { x: 240.0, y: -80.0, zoom: 3.0 };
    let n2 = find_node_2d(&doc.nodes, "n2").expect("n2");
    assert_eq!(pick(&doc, &camera, canvas_point(&camera, (n2.x, n2.y))), Some((FEM2D_GRANULARITY_NODE, "n2".to_string())));
    assert_eq!(pick(&doc, &Viewport2d::default(), canvas_point(&camera, (n2.x, n2.y))), None, "the same pixel under the default camera is empty space");
}

#[test]
fn entity_kind_and_load_owner_resolve_every_granularity() {
    let doc = demo();
    for (id, granularity) in [
        ("n1", FEM2D_GRANULARITY_NODE),
        ("e3", FEM2D_GRANULARITY_ELEMENT),
        ("s1", FEM2D_GRANULARITY_SUPPORT),
        ("l6", FEM2D_GRANULARITY_LOAD),
        ("r1", FEM2D_GRANULARITY_REGION),
        ("steel", FEM2D_GRANULARITY_MATERIAL),
        ("chs76", FEM2D_GRANULARITY_SECTION),
        ("dead", FEM2D_GRANULARITY_LOAD_CASE),
        ("uls", FEM2D_GRANULARITY_COMBINATION),
    ] {
        assert_eq!(fem2d_entity_kind(&doc, id), Some(granularity), "{id}");
    }
    assert_eq!(fem2d_entity_kind(&doc, "nope"), None);
    let (case_id, load) = fem2d_load_owner(&doc, "l6").expect("l6 belongs to a case");
    assert_eq!(case_id, "live");
    assert!(matches!(load, FemLoad::Nodal { node_id, .. } if node_id == "p8_l1"));
    assert_eq!(fem2d_load_owner(&doc, "l5").map(|(case, _)| case), Some("dead"));
}

#[test]
fn entity_model_point_frames_every_kind() {
    let doc = demo();
    assert_eq!(fem2d_entity_model_point(&doc, "n1"), Some((0.0, -4.0)));
    assert_eq!(fem2d_entity_model_point(&doc, "e3"), Some((0.0, -2.0)));
    assert_eq!(fem2d_entity_model_point(&doc, "s1"), Some((0.0, -4.0)));
    assert_eq!(fem2d_entity_model_point(&doc, "l6"), Some((8.0, 2.8)));
    assert_eq!(fem2d_entity_model_point(&doc, "r1"), Some((11.0, 2.8)));
    assert_eq!(fem2d_entity_model_point(&doc, "nope"), None);
}

#[test]
fn merge_mode_maps_every_modifier_combination() {
    assert_eq!(selection_merge_mode(false, false, false), "replace");
    assert_eq!(selection_merge_mode(true, false, false), "additive");
    assert_eq!(selection_merge_mode(false, true, false), "subtractive");
    assert_eq!(selection_merge_mode(false, false, true), "subtractive");
    assert_eq!(selection_merge_mode(true, true, false), "invertive");
    assert_eq!(selection_merge_mode(true, false, true), "invertive");
}

#[test]
fn select_and_hover_effects_carry_the_framework_wire_contract() {
    let targets = [(FEM2D_GRANULARITY_NODE, "n1".to_string())];
    let Effect::ReplayShellCommand { action_id, args } = interaction_select_effect(&targets, "additive", "pick") else {
        panic!("interaction_select_effect must request a shell replay");
    };
    assert_eq!(action_id, semio_framework::INTERACTION_SELECT_ACTION_ID);
    let args = args.expect("select args");
    assert_eq!(args.get("domainId").and_then(dsl::DslValue::as_str), Some(FEM2D_INTERACTION_DOMAIN));
    assert_eq!(args.get("merge").and_then(dsl::DslValue::as_str), Some("additive"));
    assert_eq!(args.get("method").and_then(dsl::DslValue::as_str), Some("pick"));
    let raw = args.get("targets").and_then(dsl::DslValue::as_str).expect("targets json");
    assert!(raw.contains("\"granularity\":\"node\"") && raw.contains("\"id\":\"n1\""), "targets is a JSON-encoded Vec<InteractionTarget>: {raw}");

    let Effect::ReplayShellCommand { action_id, args } = interaction_hover_effect(&targets) else {
        panic!("interaction_hover_effect must request a shell replay");
    };
    assert_eq!(action_id, semio_framework::INTERACTION_HOVER_ACTION_ID);
    let args = args.expect("hover args");
    assert_eq!(args.get("channel").and_then(dsl::DslValue::as_str), Some(FEM2D_POINTER_CHANNEL));
    assert!(args.get("targets").and_then(dsl::DslValue::as_str).is_some_and(|raw| raw.contains("\"id\":\"n1\"")));
    let empty: [(&str, String); 0] = [];
    let Effect::ReplayShellCommand { args, .. } = interaction_select_effect(&empty, "replace", "pick") else { panic!("replay") };
    assert_eq!(args.expect("args").get("targets").and_then(dsl::DslValue::as_str), Some("[]"), "a background click clears with an explicit empty batch");
    let decoded: Vec<protocol::InteractionTarget> = serde_json::from_str(raw).expect("the framework decodes targets with serde_json");
    assert_eq!(decoded.len(), 1);
    assert_eq!(decoded[0].granularity, FEM2D_GRANULARITY_NODE);
    assert_eq!(decoded[0].id, "n1");
}

#[test]
fn structure_layers_paint_stable_selected_and_hovered_emphasis() {
    let doc = demo();
    let snapshot = Fem2dInteractionSnapshot { selected_ids: vec!["n1".into(), "e3".into(), "r1".into(), "s1".into(), "l6".into()], hovered_ids: vec!["n2".into()] };
    let bare = dsl::json::to_string(&dsl::json::Value::Array(model_window::fem2d_structure_layers(&doc, "#38bdf8", "#94a3b8", "#f97316")));
    let painted = dsl::json::to_string(&dsl::json::Value::Array(model_window::fem2d_structure_layers_with(&doc, "#38bdf8", "#94a3b8", "#f97316", &snapshot)));
    for id in ["sel-node-n1", "sel-el-e3", "sel-region-r1", "sel-support-s1", "sel-load-l6", "hov-node-n2"] {
        assert!(painted.contains(id), "missing emphasis layer {id}");
        assert!(!bare.contains(id), "the bare structure layers must stay free of interaction state ({id})");
    }
    assert!(painted.contains(model_window::SELECTION_COLOR_2D), "the selected emphasis carries its own colour");
    assert!(painted.contains(model_window::HOVER_COLOR_2D), "the hovered emphasis carries the lighter colour");
    assert_eq!(dsl::json::to_string(&dsl::json::Value::Array(model_window::fem2d_structure_layers_with(&doc, "#38bdf8", "#94a3b8", "#f97316", &Fem2dInteractionSnapshot::default()))), bare, "an empty interaction snapshot paints exactly the bare layers");
}
