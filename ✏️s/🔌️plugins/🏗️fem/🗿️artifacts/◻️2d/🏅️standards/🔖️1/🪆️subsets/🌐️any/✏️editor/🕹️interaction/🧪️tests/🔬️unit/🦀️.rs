use super::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::{Fem2dSnapshot, Fem2dSnapshotRetiring, Fem2dSnapshotValue};

const CANVAS_WIDTH: f64 = 800.0;
const CANVAS_HEIGHT: f64 = 600.0;

fn demo() -> Fem2dSnapshotRetiring {
    Fem2dSnapshotRetiring::new(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot().expect("demo document parses"))
}

fn canvas_point(camera: &Viewport2d, model: (f64, f64)) -> (f64, f64) {
    fem2d_layer_to_canvas(camera, screen_2d(model.0, model.1), CANVAS_WIDTH, CANVAS_HEIGHT)
}

fn pick(doc: &Fem2dSnapshot, camera: &Viewport2d, point: (f64, f64)) -> Option<Fem2dPick> {
    fem2d_hit_test(doc, camera, point.0, point.1, CANVAS_WIDTH, CANVAS_HEIGHT).expect("scoped pick")
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
    let n1 = find_node_2d(doc.nodes.get(), "n1").expect("demo carries n1");
    assert_eq!(screen_2d(n1.x, n1.y), (40.0, 120.0), "demo n1 sits at model (0,-4)");
    assert_eq!(pick(&doc, &camera, canvas_point(&camera, (n1.x, n1.y))), Some((FEM2D_GRANULARITY_NODE, "scope:1:4:node2:n1".to_string())));
    assert_eq!(pick(&doc, &camera, (CANVAS_WIDTH - 4.0, 4.0)), None, "the far corner of the canvas hits nothing");
}

#[test]
fn hit_test_resolves_members_regions_supports_and_loads() {
    let doc = demo();
    let camera = Viewport2d::default();
    let n1 = find_node_2d(doc.nodes.get(), "n1").expect("n1");
    let n2 = find_node_2d(doc.nodes.get(), "n2").expect("n2");
    let midpoint = ((n1.x + n2.x) * 0.5, (n1.y + n2.y) * 0.5);
    assert_eq!(pick(&doc, &camera, canvas_point(&camera, midpoint)), Some((FEM2D_GRANULARITY_ELEMENT, "scope:1:7:element2:e3".to_string())), "the column e3 spans n1..n2");

    let region = doc.regions.iter().find(|region| region.id == "r1").expect("r1");
    let centroid = fem2d_region_centroid(region).expect("centroid");
    assert_eq!(pick(&doc, &camera, canvas_point(&camera, centroid)), Some((FEM2D_GRANULARITY_LOAD, "scope:1:4:load4:dead2:l5".to_string())), "the area-load arrow is anchored at the region centroid and wins over the region");
    let inside = (region.outline[0][0] + 1.4, (region.outline[0][1] + region.outline[2][1]) * 0.5);
    assert_eq!(pick(&doc, &camera, canvas_point(&camera, inside)), Some((FEM2D_GRANULARITY_REGION, "scope:1:6:region2:r1".to_string())));

    let rc0 = find_node_2d(doc.nodes.get(), "rc0").expect("rc0");
    let ring = canvas_point(&camera, (rc0.x, rc0.y));
    assert_eq!(pick(&doc, &camera, (ring.0 - 10.0, ring.1)), Some((FEM2D_GRANULARITY_SUPPORT, "scope:1:7:support2:s3".to_string())), "just outside the node radius the support glyph wins");
}

#[test]
fn hit_test_follows_the_camera() {
    let doc = demo();
    let camera = Viewport2d { x: 240.0, y: -80.0, zoom: 3.0 };
    let n2 = find_node_2d(doc.nodes.get(), "n2").expect("n2");
    assert_eq!(pick(&doc, &camera, canvas_point(&camera, (n2.x, n2.y))), Some((FEM2D_GRANULARITY_NODE, "scope:1:4:node2:n2".to_string())));
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
        let target = if granularity == FEM2D_GRANULARITY_LOAD { fem2d_load_target_id("live", id).expect("load target") } else { fem2d_entity_target_id(granularity, id).expect("entity target") };
        assert_eq!(fem2d_entity_kind(&doc, &target), Some(granularity), "{id}");
    }
    assert_eq!(fem2d_entity_kind(&doc, "nope"), None);
    let (case_id, load) = fem2d_load_owner(&doc, "scope:1:4:load4:live2:l6").expect("l6 belongs to a case");
    assert_eq!(case_id, "live");
    assert!(matches!(load, FemLoad::Nodal { node_id, .. } if node_id == "p8_l1"));
    assert_eq!(fem2d_load_owner(&doc, "scope:1:4:load4:dead2:l5").map(|(case, _)| case), Some("dead"));
}

#[test]
fn entity_model_point_frames_every_kind() {
    let doc = demo();
    assert_eq!(fem2d_entity_model_point(&doc, "scope:1:4:node2:n1"), Some((0.0, -4.0)));
    assert_eq!(fem2d_entity_model_point(&doc, "scope:1:7:element2:e3"), Some((0.0, -2.0)));
    assert_eq!(fem2d_entity_model_point(&doc, "scope:1:7:support2:s1"), Some((0.0, -4.0)));
    assert_eq!(fem2d_entity_model_point(&doc, "scope:1:4:load4:live2:l6"), Some((8.0, 2.8)));
    assert_eq!(fem2d_entity_model_point(&doc, "scope:1:6:region2:r1"), Some((11.0, 2.8)));
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
    let targets = [(FEM2D_GRANULARITY_NODE, "scope:1:4:node2:n1".to_string())];
    let Effect::ReplayShellCommand { action_id, args } = interaction_select_effect(&targets, "additive", "pick") else {
        panic!("interaction_select_effect must request a shell replay");
    };
    assert_eq!(action_id, semio_framework::INTERACTION_SELECT_ACTION_ID);
    let args = args.expect("select args");
    assert_eq!(args.get("domainId").and_then(semio_framework_value::DslValue::as_str), Some(FEM2D_INTERACTION_DOMAIN));
    assert_eq!(args.get("merge").and_then(semio_framework_value::DslValue::as_str), Some("additive"));
    assert_eq!(args.get("method").and_then(semio_framework_value::DslValue::as_str), Some("pick"));
    let raw = args.get("targets").and_then(semio_framework_value::DslValue::as_str).expect("targets json");
    assert!(raw.contains("\"granularity\":\"node\"") && raw.contains("\"id\":\"scope:1:4:node2:n1\""), "targets is a JSON-encoded Vec<InteractionTarget>: {raw}");

    let Effect::ReplayShellCommand { action_id, args } = interaction_hover_effect(&targets) else {
        panic!("interaction_hover_effect must request a shell replay");
    };
    assert_eq!(action_id, semio_framework::INTERACTION_HOVER_ACTION_ID);
    let args = args.expect("hover args");
    assert_eq!(args.get("channel").and_then(semio_framework_value::DslValue::as_str), Some(FEM2D_POINTER_CHANNEL));
    assert!(args.get("targets").and_then(semio_framework_value::DslValue::as_str).is_some_and(|raw| raw.contains("\"id\":\"scope:1:4:node2:n1\"")));
    let empty: [(&str, String); 0] = [];
    let Effect::ReplayShellCommand { args, .. } = interaction_select_effect(&empty, "replace", "pick") else { panic!("replay") };
    assert_eq!(args.expect("args").get("targets").and_then(semio_framework_value::DslValue::as_str), Some("[]"), "a background click clears with an explicit empty batch");
    let decoded: Vec<protocol::InteractionTarget> = serde_json::from_str(raw).expect("the framework decodes targets with serde_json");
    assert_eq!(decoded.len(), 1);
    assert_eq!(decoded[0].granularity, FEM2D_GRANULARITY_NODE);
    assert_eq!(decoded[0].id, "scope:1:4:node2:n1");
}

fn identity_fixture() -> (serde_json::Value, Fem2dSnapshotValue) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🎮️commands/🗂️remove-selection/🧪️tests/🔬️unit/🔣️.json")).expect("shared JSON identity scenarios");
    let value = semio_framework_pack_json::from_json_str(&fixture["before"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authoring snapshot wire");
    (fixture, value)
}

fn sealed(value: Fem2dSnapshotValue) -> Fem2dSnapshotRetiring {
    Fem2dSnapshotRetiring::new(Fem2dSnapshot::from_reference(value).expect("the authoring value seals"))
}

#[test]
fn scoped_identity_admission_refuses_unaddressable_entities_and_duplicate_load_owners() {
    use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_node;
    use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::create_load_case;
    use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::add_load;
    use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
    use protocol::{ColdMutation, Mutation};

    let (fixture, value) = identity_fixture();
    let mut invalid = value.clone();
    invalid.nodes[0].id.clear();
    assert!(invalid.validate_interaction_identities().is_err());
    let mut duplicate = value.clone();
    let repeated_load = duplicate.load_cases[0].loads[0].clone();
    duplicate.load_cases[0].loads.push(repeated_load);
    assert!(duplicate.validate_interaction_identities().is_err());
    let mut case = value.load_cases[0].clone();
    case.id = "new-case".into();
    case.loads.push(case.loads[0].clone());
    let mut load = value.load_cases[0].loads[0].clone();
    let before = sealed(value);
    match &mut load { semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoad::Nodal { id, .. } | semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoad::MemberUdl { id, .. } | semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoad::Area { id, .. } => id.clear() }
    for (operation, expected) in [
        (Fem2dMutation::CreateNode(semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_node::CreateNode { node: semio_s_fem_node::planar::FemNodeValue { id: String::new(), x: 0.0, y: 0.0 }, index: None }), "emptyIdentityCode"),
        (Fem2dMutation::AddLoad(semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::add_load::AddLoad { case_id: "dead".into(), load: Box::new(load), index: None }), "emptyIdentityCode"),
        (Fem2dMutation::CreateLoadCase(semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::create_load_case::CreateLoadCase { load_case: case, index: None }), "duplicateLoadCode"),
    ] {
        let outcome = operation.diff(&*before);
        assert_eq!(outcome.diff(), &semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff::default());
        assert_eq!(outcome.messages()[0].code.0, fixture["identityAdmission"][expected].as_str().expect("declared diagnostic"));
        assert!(operation.inverse(&*before).expect("refused aggregate inverse").is_empty());
    }
}

#[test]
fn borrowed_identity_admission_precedes_native_snapshot_construction_and_observes_cancellation() {
    let (_, before) = identity_fixture();
    let record = before.__dsl_to_record();
    let mut allow = |_| true;
    let mut control = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1 << 20, &mut allow);
    semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot::validate_interaction_record(&record, &mut control).expect("borrowed identities admit");
    let mut allow = |_| true;
    let mut limited = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(0, &mut allow);
    assert!(Fem2dSnapshot::validate_interaction_record(&record, &mut limited).is_err());
    assert_eq!(limited.owned_bytes(), 0);
    let mut invalid = before.clone();
    let repeated = invalid.load_cases[0].loads[0].clone();
    invalid.load_cases[0].loads.push(repeated);
    let record = invalid.__dsl_to_record();
    let mut allow = |_| true;
    let mut control = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1 << 20, &mut allow);
    assert!(Fem2dSnapshot::validate_interaction_record(&record, &mut control).is_err());
    let record = before.__dsl_to_record();
    let mut callbacks = 0;
    let mut cancel = |_| { callbacks += 1; false };
    let mut control = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1 << 20, &mut cancel);
    assert!(Fem2dSnapshot::validate_interaction_record(&record, &mut control).is_err());
    drop(control);
    assert!(callbacks > 0);
    assert_eq!(before.nodes[0].id, "n1");
}

#[test]
fn public_json_snapshot_admission_refuses_invalid_owners_and_accepts_distinct_namespaces() {
    let (fixture, before) = identity_fixture();
    for row in fixture["publicAdmission"]["refused"].as_array().expect("declared refusal matrix") {
        let mut candidate = serde_json::to_value(&fixture["before"]).expect("independent JSON tree");
        match row.as_str().expect("scenario") {
            "empty" => candidate["nodes"][0]["id"] = "".into(),
            "oversized" => candidate["nodes"][0]["id"] = "x".repeat(protocol::wire::SCOPED_IDENTITY_MAX_BYTES).into(),
            "duplicateNode" => candidate["nodes"][1]["id"] = candidate["nodes"][0]["id"].clone(),
            "duplicateLoad" => { let load = candidate["loadCases"][0]["loads"][0].clone(); candidate["loadCases"][0]["loads"].as_array_mut().expect("loads").push(load); }
            _ => panic!("unknown portable admission scenario"),
        }
        assert!(semio_framework_pack_json::from_json_str::<Fem2dSnapshot>(&candidate.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "{row}");
        let value = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(&candidate.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("borrowed native value fixture");
        let mut allow = |_| true;
        let mut control = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1 << 22, &mut allow);
        assert!(<Fem2dSnapshotValue as semio_framework_value::FromValue>::from_value_controlled(&value, &mut control).is_err(), "controlled {row}");
    }
    let mut accepted = fixture["before"].clone();
    let old_material = accepted["materials"][0]["id"].clone();
    accepted["materials"][0]["id"] = accepted["nodes"][0]["id"].clone();
    let new_material = accepted["materials"][0]["id"].clone();
    for element in accepted["elements"].as_array_mut().expect("elements") {
        if element["materialId"] == old_material {
            element["materialId"] = new_material.clone();
        }
    }
    let mut case = accepted["loadCases"][0].clone();
    case["id"] = fixture["collision"]["liveCaseId"].clone();
    accepted["loadCases"].as_array_mut().expect("cases").push(case);
    let snapshot = semio_framework_pack_json::from_json_str::<Fem2dSnapshot>(&accepted.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("separate namespaces admit");
    assert_eq!(snapshot.nodes.get().len(), before.nodes.len());
    assert_eq!(snapshot.load_cases.len(), 2);
    let authoring = semio_framework_pack_json::from_json_str::<Fem2dSnapshotValue>(&accepted.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("separate namespaces admit as authoring rows");
    let mut allow = |_| true;
    let mut control = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1 << 20, &mut allow);
    authoring.validate_interaction_identities_controlled(&mut control).expect("controlled cross namespaces admit");
    assert!(control.owned_bytes() > 0, "semantic identity scratch carries original allocation authority");
    <Fem2dSnapshot as semio_framework_value::FromValue>::retire_decoded(snapshot);
}

/// 🗂️ LAW: the topology `selectAll` walks lists every selectable entity of the demo exactly once, each under the granularity `fem2d_entity_kind` resolves it to, elements first (the default Select All scope) — an empty topology made Select All a silent no-op.
#[test]
fn select_all_topology_lists_every_selectable_entity_2d() {
    let doc = demo();
    let topology = fem2d_interaction_topology(&doc);
    let ordered = &topology.domains.get(FEM2D_INTERACTION_DOMAIN).expect("fem2d domain topology").ordered;
    let count = |granularity: &str| ordered.iter().filter(|node| node.granularity == granularity).count();
    assert_eq!(count(FEM2D_GRANULARITY_ELEMENT), doc.elements.len());
    assert_eq!(count(FEM2D_GRANULARITY_NODE), doc.nodes.len());
    assert_eq!(count(FEM2D_GRANULARITY_REGION), doc.regions.len());
    assert_eq!(count(FEM2D_GRANULARITY_SUPPORT), doc.supports.len());
    assert_eq!(count(FEM2D_GRANULARITY_LOAD), doc.load_cases.iter().map(|case| case.loads.len()).sum::<usize>());
    assert_eq!(count(FEM2D_GRANULARITY_LOAD_CASE), doc.load_cases.len());
    assert_eq!(count(FEM2D_GRANULARITY_COMBINATION), doc.combinations.len());
    assert_eq!(count(FEM2D_GRANULARITY_ANALYSIS), 1);
    assert!(!ordered.is_empty() && ordered[0].granularity == FEM2D_GRANULARITY_ELEMENT, "elements lead the topology");
    assert!(ordered.iter().all(|node| fem2d_entity_kind(&doc, &node.id) == Some(node.granularity.as_str())), "every listed id resolves to its own granularity");
    let unique: std::collections::BTreeSet<&str> = ordered.iter().map(|node| node.id.as_str()).collect();
    assert_eq!(unique.len(), ordered.len(), "no id is listed twice");
}
