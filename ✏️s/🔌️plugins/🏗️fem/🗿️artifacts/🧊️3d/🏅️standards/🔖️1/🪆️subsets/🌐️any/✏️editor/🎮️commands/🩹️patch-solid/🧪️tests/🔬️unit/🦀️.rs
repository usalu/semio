use super::*;

/// 🔢️ The layer count is an exact unsigned value and owns only its authored field.
#[test]
fn layer_count_text_admission_is_exact_and_sparse() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut base = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot();
    base.solids.iter_mut().find(|solid| solid.id == "sol1").unwrap().layers = 2;
    for row in fixture["layerCounts"].as_array().unwrap() {
        let result = emit(&base, PatchSolid { id: "sol1".into(), field: "layers".into(), value: row["text"].as_str().unwrap().into() });
        if let Some(layers) = row["layers"].as_u64() {
            let emitted = result.unwrap();
            assert_eq!(emitted.artifact_mutations, vec![Fem3dMutation::ChangeSolid(ChangeSolid { id: "sol1".into(), patch: Fem3dSolidPatch { layers: Some(layers as usize), ..Default::default() } })]);
        } else {
            assert!(result.is_err(), "fractional and negative counts must be refused");
        }
    }
    eprintln!("[DEBUG] solid layer count: one sparse unsigned field and two malformed count refusals");
}

use crate::context::dispatch;

use crate::context::fem3d_demo_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;
#[cfg(test)] use semio_framework_plugin::component::ArtifactView;
#[cfg(test)] use semio_framework_plugin::component::NoConfig;
#[cfg(test)] use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
#[cfg(test)] use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_solid::PatchSolid;
#[cfg(test)] use semio_framework_plugin::component::Emit;
#[cfg(test)] use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
#[cfg(test)] use semio_framework_plugin::component::NoConfigMutation;
#[cfg(test)] use semio_framework::Fault;

fn emit(snapshot: &Fem3dSnapshot, payload: PatchSolid) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_solid_edits_extrusion_axis_and_material_on_the_live_demo_3d() {
    let mut app = fem3d_demo_app().await;
    dispatch(&mut app, Fem3dCommand::PatchSolid(PatchSolid { id: "sol1".into(), field: "height".into(), value: "0.8".into() })).await;
    dispatch(&mut app, Fem3dCommand::PatchSolid(PatchSolid { id: "sol1".into(), field: "axis".into(), value: "y".into() })).await;
    dispatch(&mut app, Fem3dCommand::PatchSolid(PatchSolid { id: "sol1".into(), field: "layers".into(), value: "3".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let solid = snapshot.solids.iter().find(|solid| solid.id == "sol1").expect("sol1");
    assert_eq!((solid.height, solid.axis, solid.layers), (0.8, FemAxis::Y, 3));
    assert!(emit(&snapshot, PatchSolid { id: "sol1".into(), field: "axis".into(), value: "w".into() }).is_err());
    assert!(emit(&snapshot, PatchSolid { id: "sol1".into(), field: "meshSize".into(), value: "coarse".into() }).is_err());
    assert!(emit(&snapshot, PatchSolid { id: "sol1".into(), field: "colour".into(), value: "grey".into() }).is_err());
    assert!(emit(&snapshot, PatchSolid { id: "sol1".into(), field: "height".into(), value: "0.8".into() }).expect("handle").artifact_mutations.is_empty());
}

/// 👥️ Two inspector commands authored on one base compose their independent intent in either replay order.
#[test]
fn sibling_field_commands_and_inverse_preserve_concurrent_intent(){
    let base=semio_framework_pack_json::from_json_str::<Fem3dSnapshot>(include_str!("../../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-solid/🩹️changes/📸️snapshot/⬅️before/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral fixture");
    let first=emit(&base,PatchSolid{id:"sol1".into(),field:"name".into(),value:"Mine".into()}).unwrap().artifact_mutations.remove(0);
    let second=emit(&base,PatchSolid{id:"sol1".into(),field:"height".into(),value:"0.3".into()}).unwrap().artifact_mutations.remove(0);
    let first_then_second=vcs::apply_mutation(&vcs::apply_mutation(&base,&first).unwrap().0,&second).unwrap().0;
    let combined=vcs::apply_mutation(&vcs::apply_mutation(&base,&second).unwrap().0,&first).unwrap().0;
    assert_eq!(combined,first_then_second);
    assert_eq!(combined.solids[0].name,"Mine");assert_eq!(combined.solids[0].height,0.3);
    let mut undo=combined.clone();for inverse in protocol::ColdMutation::inverse(&first,&base).unwrap().iter().rev(){undo=vcs::apply_mutation(&undo,inverse).unwrap().0;}
    assert_eq!(undo,vcs::apply_mutation(&base,&second).unwrap().0,"undo owns no sibling field");
    let first_diff=protocol::ColdMutation::diff(&first,&base).into_parts().0;let second_diff=protocol::ColdMutation::diff(&second,&base).into_parts().0;
    let mut sum=first_diff.clone();protocol::ColdMutationDiff::absorb(&mut sum,second_diff);
    assert_eq!(protocol::apply_diff_cold(&sum,&base).unwrap(),combined);
    assert_eq!(protocol::apply_diff_cold(&protocol::DiffAlgebra::inverse(&sum,&base),&combined).unwrap(),base);
    eprintln!("[DEBUG] solid same-base inspector fields and sparse inverse preserve concurrent sibling intent");
}