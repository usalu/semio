use super::*;
use crate::editor::bim::unit_tests::context::view;
use crate::editor::bim::unit_tests::support::run;
use protocol::Inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_framework_ui_locale::Locale;

const FRAME: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🧨️clash-sets/🏢️frame/📸️snapshot/🔣️.json");

fn frame() -> ModelSnapshot {
    from_json_str(FRAME, JsonMemberPolicy::Reject).expect("the committed frame decodes")
}

fn first_clash(snapshot: &ModelSnapshot) -> (String, Clash) {
    let inference = crate::ModelInference::infer(snapshot).expect("the frame infers");
    inference.clash_sets.iter().find_map(|(id, result)| result.clashes.iter().find(|clash| clash.kind == ClashKind::Hard).map(|clash| (id.clone(), clash.clone()))).expect("the frame has a hard clash")
}

fn window(kind: &str) -> BimDispatchCtx {
    let view = view(Locale::En, &[("window", kind)], Some("window"));
    BimDispatchCtx::new(Vec::new(), Vec::new(), Some(&view), None, None)
}

fn code<T>(result: Result<T, Fault>) -> Option<String> {
    result.err().map(|fault| fault.code.0)
}

#[semio_framework_async_macros::async_test]
async fn framing_a_clash_centres_the_camera_on_its_box_and_keeps_the_minimum_distance() {
    let (_, clash) = first_clash(&frame());
    let camera = framing(&clash);
    assert!((camera.target.x - (clash.min.x + clash.max.x) / 2.0).abs() < 1e-12 && (camera.target_height - (clash.min.z + clash.max.z) / 2.0).abs() < 1e-12);
    assert!(camera.distance >= MIN_DISTANCE);
    let section = section_around(&clash);
    assert!((section.min.x - (clash.min.x - SECTION_MARGIN)).abs() < 1e-12 && (section.max.z - (clash.max.z + SECTION_MARGIN)).abs() < 1e-12);
    let (eye, target) = camera.eye_and_target();
    assert!(eye[2] > target[2], "the camera looks down on the clash");
}

#[semio_framework_async_macros::async_test]
async fn viewing_a_clash_writes_only_the_window_config_of_the_world_window() {
    let snapshot = frame();
    let (_, clash) = first_clash(&snapshot);
    let mut ctx = window(world::WINDOW_KIND_ID);
    let payload = ViewClash { first: clash.first.clone(), second: clash.second.clone(), mode: "both".into() };
    let emit = run(&snapshot, |doc, cfg| handle(&payload, doc, cfg, &mut ctx)).expect("frames and isolates");
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert!(emit.artifact_mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_clash_view_needs_a_world_window_a_known_mode_and_a_clash_that_exists() {
    let snapshot = frame();
    let (_, clash) = first_clash(&snapshot);
    let payload = |mode: &str| ViewClash { first: clash.first.clone(), second: clash.second.clone(), mode: mode.into() };
    let mut plan = window(crate::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID);
    assert_eq!(code(run(&snapshot, |doc, cfg| handle(&payload("zoom"), doc, cfg, &mut plan))), Some("bim.clash.world-required".to_string()));
    let mut none = BimDispatchCtx::default();
    assert_eq!(code(run(&snapshot, |doc, cfg| handle(&payload("zoom"), doc, cfg, &mut none))), Some("bim.clash.window-required".to_string()));
    let mut world = window(world::WINDOW_KIND_ID);
    assert_eq!(code(run(&snapshot, |doc, cfg| handle(&payload("sideways"), doc, cfg, &mut world))), Some("bim.clash.mode-unknown".to_string()));
    let missing = ViewClash { first: "w-1".into(), second: "c-1".into(), mode: "zoom".into() };
    assert_eq!(code(run(&snapshot, |doc, cfg| handle(&missing, doc, cfg, &mut world))), Some("bim.clash.missing".to_string()));
    assert!(run(&snapshot, |doc, cfg| handle(&payload("clear"), doc, cfg, &mut world)).is_ok(), "clearing needs no clash");
}

#[semio_framework_async_macros::async_test]
async fn raising_an_issue_from_a_clash_names_the_pair_the_clash_and_a_viewpoint_that_shows_it() {
    let snapshot = frame();
    let (set, clash) = first_clash(&snapshot);
    let payload = RaiseIssue { set: set.clone(), first: clash.second.clone(), second: clash.first.clone() };
    let mut ctx = BimDispatchCtx::default();
    let emit = run(&snapshot, |doc, cfg| handle(&payload, doc, cfg, &mut ctx)).expect("raises the issue");
    let [ModelMutation::CreateIssue(create)] = emit.artifact_mutations.as_slice() else { panic!("one create-issue: {:?}", emit.artifact_mutations) };
    let issue = &create.issue;
    assert_eq!(issue.elements, [clash.first.clone(), clash.second.clone()]);
    assert_eq!(issue.clash, Some(ClashRef { set, first: clash.first.clone(), second: clash.second.clone() }));
    assert_eq!((issue.status, issue.priority), (IssueStatus::Open, IssuePriority::High));
    let viewpoint = issue.viewpoint.as_ref().expect("a viewpoint");
    assert_eq!(viewpoint.isolate, [clash.first.clone(), clash.second.clone()]);
    assert!(viewpoint.section.is_some());
    assert_eq!(crate::issue_problem(&snapshot, &create.id, issue), None, "the issue is writable");
    let missing = RaiseIssue { set: String::new(), first: "w-1".into(), second: "c-1".into() };
    assert_eq!(code(run(&snapshot, |doc, cfg| handle(&missing, doc, cfg, &mut ctx))), Some("bim.clash.missing".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn a_viewpoint_is_captured_from_the_world_window_and_restored_into_it() {
    let mut snapshot = frame();
    let (set, clash) = first_clash(&snapshot);
    let raised = RaiseIssue { set, first: clash.first.clone(), second: clash.second.clone() };
    let mut ctx = BimDispatchCtx::default();
    let Ok(emit) = run(&snapshot, |doc, cfg| handle(&raised, doc, cfg, &mut ctx)) else { panic!("raises") };
    let [ModelMutation::CreateIssue(create)] = emit.artifact_mutations.as_slice() else { panic!("one create-issue") };
    snapshot.issues.insert(create.id.clone(), Issue { viewpoint: None, ..create.issue.clone() });

    let mut world = window(world::WINDOW_KIND_ID);
    world.world.camera = store::Viewport3dOrbit { position: [10.0, -6.0, 8.0], target: [3.0, 3.0, 1.5], zoom: 1.0, up: None };
    world.world.isolated_elements = vec![clash.first.clone(), "gone".into()];
    world.world.section_box = vec![0.0, 0.0, 0.0, 6.0, 6.0, 4.0];
    let capture = CaptureViewpoint { issue: create.id.clone() };
    let emit = run(&snapshot, |doc, cfg| handle(&capture, doc, cfg, &mut world)).expect("captures");
    let [ModelMutation::SetIssue(set_issue)] = emit.artifact_mutations.as_slice() else { panic!("one set-issue: {:?}", emit.artifact_mutations) };
    let Some(Assigned { value: Some(viewpoint) }) = set_issue.viewpoint.clone() else { panic!("a viewpoint") };
    assert_eq!(viewpoint.isolate, [clash.first.clone()], "an element that is gone is not captured");
    assert_eq!(viewpoint.section, Some(SectionBox { min: Point3 { x: 0.0, y: 0.0, z: 0.0 }, max: Point3 { x: 6.0, y: 6.0, z: 4.0 } }));
    let (eye, target) = viewpoint.camera.eye_and_target();
    assert!(eye.iter().zip([10.0, -6.0, 8.0]).all(|(a, b)| (a - b).abs() < 1e-9) && target.iter().zip([3.0, 3.0, 1.5]).all(|(a, b)| (a - b).abs() < 1e-9));

    snapshot.issues.get_mut(&create.id).expect("issue").viewpoint = Some(viewpoint);
    let restore = RestoreViewpoint { issue: create.id.clone() };
    let mut back = window(world::WINDOW_KIND_ID);
    let emit = run(&snapshot, |doc, cfg| handle(&restore, doc, cfg, &mut back)).expect("restores");
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert!(emit.artifact_mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn viewpoints_need_a_world_window_and_an_issue_and_for_restoring_a_stored_viewpoint() {
    let mut snapshot = frame();
    snapshot.issues.insert("i-1".into(), Issue { title: "x".into(), description: String::new(), status: IssueStatus::Open, priority: IssuePriority::Normal, assignee: String::new(), author: "UG".into(), created: "2026-10-09".into(), labels: Vec::new(), elements: Vec::new(), clash: None, viewpoint: None });
    let mut world = window(world::WINDOW_KIND_ID);
    let restore = RestoreViewpoint { issue: "i-1".into() };
    assert_eq!(code(run(&snapshot, |doc, cfg| handle(&restore, doc, cfg, &mut world))), Some("bim.issue.viewpoint-missing".to_string()));
    let unknown = RestoreViewpoint { issue: "i-9".into() };
    assert_eq!(code(run(&snapshot, |doc, cfg| handle(&unknown, doc, cfg, &mut world))), Some("bim.issue.missing".to_string()));
    let mut none = BimDispatchCtx::default();
    assert_eq!(code(run(&snapshot, |doc, cfg| handle(&CaptureViewpoint { issue: "i-1".into() }, doc, cfg, &mut none))), Some("bim.clash.window-required".to_string()));
}
