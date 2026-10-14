use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_support::*;
use semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::replace_support;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::context::{dispatch, fem3d_demo_app};
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;

fn emit(snapshot: &Fem3dSnapshot, payload: PatchSupport) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_support_toggles_dofs_in_canonical_order_and_repoints_the_node_3d() {
    let mut app = fem3d_demo_app().await;
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
