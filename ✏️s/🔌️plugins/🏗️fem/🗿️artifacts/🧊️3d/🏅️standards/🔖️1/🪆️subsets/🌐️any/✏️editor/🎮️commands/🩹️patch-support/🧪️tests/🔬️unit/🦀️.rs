use super::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
use crate::context::dispatch;
use crate::context::Retiring;
use crate::context::fem3d_demo_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;
#[cfg(test)] use semio_plugin::component::ArtifactView;
#[cfg(test)] use semio_plugin::component::NoConfig;
#[cfg(test)] use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
#[cfg(test)] use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::{Fem3dSnapshot, Fem3dSnapshotValue};
#[cfg(test)] use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_support::{handle, PatchSupport};
#[cfg(test)] use semio_plugin::ConfigView;
#[cfg(test)] use semio_plugin::component::Emit;
#[cfg(test)] use semio_plugin::component::NoConfigMutation;
#[cfg(test)] use semio_framework::Fault;

fn emit(snapshot: &Fem3dSnapshot, payload: PatchSupport) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_plugin::HistoryView::empty();
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
    let [Fem3dMutation::ChangeSupport(change)] = emitted.artifact_mutations.as_slice() else { panic!("one change-support") };
    assert_eq!(change.patch.node_id.as_deref().unwrap(), "sc1");
    assert!(emit(&snapshot, PatchSupport { id: "ss_0".into(), field: "tz".into(), value: "maybe".into() }).is_err());
    assert!(emit(&snapshot, PatchSupport { id: "ss_0".into(), field: "tw".into(), value: "true".into() }).is_err());
    assert!(emit(&snapshot, PatchSupport { id: "ss_0".into(), field: "ty".into(), value: "true".into() }).expect("handle").artifact_mutations.is_empty());
}

/// 👥️ Two inspector commands authored on one base compose their independent intent in either replay order.
#[test]
fn sibling_field_commands_and_inverse_preserve_concurrent_intent() {
    let base_value = semio_framework_pack_json::from_json_str::<Fem3dSnapshotValue>(include_str!("../../../../../../🛡️boundary/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-support/🩹️changes/📸️snapshot/⬅️before/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral fixture");
    let base = Retiring::new(Fem3dSnapshot::from_reference(base_value.clone()).expect("persistent base"));
    let first = emit(&base, PatchSupport { id: "s1".into(), field: "tx".into(), value: "true".into() }).unwrap().artifact_mutations.remove(0);
    let second = emit(&base, PatchSupport { id: "s1".into(), field: "ty".into(), value: "true".into() }).unwrap().artifact_mutations.remove(0);
    let replay = |state: &Fem3dSnapshotValue, mutation: &Fem3dMutation| {
        let mut next = state.clone();
        semio_s_artifact_fem_3d_mutations::central_apply::apply_fem3d_mutation(&mut next, mutation).unwrap();
        next
    };
    let first_then_second = replay(&replay(&base_value, &first), &second);
    let combined = replay(&replay(&base_value, &second), &first);
    assert_eq!(combined, first_then_second);
    assert_eq!(combined.supports[0].fixed, vec![FemDof::Tx, FemDof::Ty, FemDof::Rz]);
    let mut undo = combined.clone();
    for inverse in protocol::ColdMutation::inverse(&first, &base_value).unwrap().iter().rev() { undo = replay(&undo, inverse); }
    assert_eq!(undo, replay(&base_value, &second), "undo owns no sibling field");
    let first_diff = protocol::ColdMutation::diff(&first, &base_value).into_parts().0;
    let second_diff = protocol::ColdMutation::diff(&second, &base_value).into_parts().0;
    let mut sum = first_diff.clone();
    protocol::ColdMutationDiff::<Fem3dSnapshotValue>::absorb(&mut sum, second_diff);
    assert_eq!(protocol::apply_diff_cold(&sum, &base_value).unwrap(), combined);
    assert_eq!(protocol::apply_diff_cold(&protocol::ColdDiffAlgebra::inverse(&sum, &base_value), &combined).unwrap(), base_value);
}
