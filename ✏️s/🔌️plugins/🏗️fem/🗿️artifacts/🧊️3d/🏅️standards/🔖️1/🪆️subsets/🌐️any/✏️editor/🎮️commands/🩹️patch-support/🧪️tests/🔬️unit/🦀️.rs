use super::*;
use crate::editor::fem3d::unit_tests::context::{dispatch, fem3d_app};
use crate::editor::fem3d::Fem3dCommand;

fn emit(snapshot: &Fem3dSnapshot, payload: PatchSupport) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_support_toggles_dofs_in_canonical_order_and_repoints_the_node_3d() {
    let mut app = fem3d_app();
    dispatch(&mut app, Fem3dCommand::PatchSupport(PatchSupport { id: "ss_0".into(), field: "rz".into(), value: "true".into() })).await;
    dispatch(&mut app, Fem3dCommand::PatchSupport(PatchSupport { id: "ss_0".into(), field: "tx".into(), value: "false".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let support = snapshot.supports.iter().find(|support| support.id == "ss_0").expect("ss_0");
    assert_eq!(support.fixed, vec![FemDof::Ty, FemDof::Tz, FemDof::Rz]);
    let emitted = emit(&snapshot, PatchSupport { id: "ss_0".into(), field: "nodeId".into(), value: "sc1".into() }).expect("handle");
    let [Fem3dMutation::ReplaceSupport(replace)] = emitted.artifact_mutations.as_slice() else { panic!("one replace-support") };
    assert_eq!(replace.new_support.node_id, "sc1");
    assert!(emit(&snapshot, PatchSupport { id: "ss_0".into(), field: "tz".into(), value: "maybe".into() }).is_err());
    assert!(emit(&snapshot, PatchSupport { id: "ss_0".into(), field: "tw".into(), value: "true".into() }).is_err());
    assert!(emit(&snapshot, PatchSupport { id: "ss_0".into(), field: "ty".into(), value: "true".into() }).expect("handle").artifact_mutations.is_empty());
}
