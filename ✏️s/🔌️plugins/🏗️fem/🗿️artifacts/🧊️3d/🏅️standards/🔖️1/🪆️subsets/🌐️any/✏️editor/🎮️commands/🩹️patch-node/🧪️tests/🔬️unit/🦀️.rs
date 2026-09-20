use super::*;
use crate::editor::fem3d::unit_tests::context::{dispatch, fem3d_demo_app};
use crate::editor::fem3d::Fem3dCommand;

fn demo() -> Fem3dSnapshot {
    crate::standards::v1::subsets::any::schema::snapshot::text::fem3d_demo_snapshot()
}

fn emit(snapshot: &Fem3dSnapshot, payload: PatchNode) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_node_moves_an_ordinate_on_the_live_demo_3d() {
    let mut app = fem3d_demo_app().await;
    dispatch(&mut app, Fem3dCommand::PatchNode(PatchNode { id: "n20_l1".into(), field: "z".into(), value: "3.1".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let node = snapshot.nodes.iter().find(|node| node.id == "n20_l1").expect("n20_l1 survives the patch");
    assert_eq!((node.x, node.y, node.z), (8.0, 0.0, 3.1), "the untouched ordinates are carried through the whole-record replace");
}

#[semio_framework_async_macros::async_test]
async fn patch_node_emits_one_whole_record_replace_3d() {
    let emitted = emit(&demo(), PatchNode { id: "n00_l2".into(), field: "y".into(), value: "0.5".into() }).expect("handle");
    let [Fem3dMutation::ReplaceNode(replace)] = emitted.artifact_mutations.as_slice() else { panic!("one replace-node") };
    assert_eq!(replace.id, "n00_l2");
    assert_eq!((replace.new_node.x, replace.new_node.y, replace.new_node.z), (0.0, 0.5, 5.6));
}

#[semio_framework_async_macros::async_test]
async fn patch_node_rejects_an_unknown_field_an_unparsable_value_and_a_missing_node_3d() {
    let demo = demo();
    assert!(emit(&demo, PatchNode { id: "n00_g".into(), field: "w".into(), value: "1".into() }).is_err());
    assert!(emit(&demo, PatchNode { id: "n00_g".into(), field: "x".into(), value: "over there".into() }).is_err());
    assert!(emit(&demo, PatchNode { id: "nowhere".into(), field: "x".into(), value: "1".into() }).is_err());
    assert!(emit(&demo, PatchNode { id: "n00_g".into(), field: "x".into(), value: "0".into() }).expect("handle").artifact_mutations.is_empty(), "an edit that changes nothing must not open a revision");
}
