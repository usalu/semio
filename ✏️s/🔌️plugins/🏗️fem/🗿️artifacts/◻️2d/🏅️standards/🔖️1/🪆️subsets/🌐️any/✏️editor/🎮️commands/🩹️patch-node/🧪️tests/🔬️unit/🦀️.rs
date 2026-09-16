use super::*;
use crate::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app};
use crate::editor::fem2d::Fem2dCommand;
use store::ArtifactDsl;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("the bundled demo fixture parses")
}

fn emit(snapshot: &Fem2dSnapshot, payload: PatchNode) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_node_moves_an_ordinate_on_the_demo_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    dispatch(&mut app, Fem2dCommand::PatchNode(PatchNode { id: "n2".into(), field: "x".into(), value: "1.5".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let node = snapshot.nodes.iter().find(|node| node.id == "n2").expect("n2 survives the patch");
    assert_eq!(node.x, 1.5);
    assert_eq!(node.y, 0.0, "the untouched ordinate is carried through the whole-record replace");
}

#[semio_framework_async_macros::async_test]
async fn patch_node_emits_one_whole_record_replace_2d() {
    let emitted = emit(&demo(), PatchNode { id: "ridge".into(), field: "y".into(), value: "8.2".into() }).expect("handle");
    let [Fem2dMutation::ReplaceNode(replace)] = emitted.artifact_mutations.as_slice() else { panic!("one replace-node") };
    assert_eq!(replace.id, "ridge");
    assert_eq!((replace.new_node.id.as_str(), replace.new_node.x, replace.new_node.y), ("ridge", 4.0, 8.2));
}

#[semio_framework_async_macros::async_test]
async fn patch_node_rejects_an_unknown_field_an_unparsable_value_and_a_missing_node_2d() {
    let demo = demo();
    assert!(emit(&demo, PatchNode { id: "n2".into(), field: "z".into(), value: "1".into() }).is_err());
    assert!(emit(&demo, PatchNode { id: "n2".into(), field: "x".into(), value: "over there".into() }).is_err());
    assert!(emit(&demo, PatchNode { id: "nowhere".into(), field: "x".into(), value: "1".into() }).is_err());
}

#[semio_framework_async_macros::async_test]
async fn patch_node_with_the_current_ordinate_emits_nothing_2d() {
    let emitted = emit(&demo(), PatchNode { id: "n2".into(), field: "y".into(), value: "0".into() }).expect("handle");
    assert!(emitted.artifact_mutations.is_empty(), "an edit that changes nothing must not open a revision");
}
