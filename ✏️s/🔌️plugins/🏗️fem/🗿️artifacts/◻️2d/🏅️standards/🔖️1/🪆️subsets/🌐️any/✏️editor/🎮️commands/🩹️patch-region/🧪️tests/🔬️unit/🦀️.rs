use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_region::*;
use semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::replace_region;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::context::dispatch;
use crate::context::fem2d_app;
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use pack::snapshot::ArtifactDsl;

fn demo() -> Fem2dSnapshot {
    semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::parse_dsl(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT).expect("the bundled demo fixture parses")
}

fn emit(snapshot: &Fem2dSnapshot, payload: PatchRegion) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_region_edits_the_thickness_on_the_demo_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: semio_s_artifact_fem_2d_io::examples::demo::ID.into() })).await;
    dispatch(&mut app, Fem2dCommand::PatchRegion(PatchRegion { id: "r1".into(), field: "thickness".into(), value: "0.25".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let region = snapshot.regions.iter().find(|region| region.id == "r1").expect("r1 survives the patch");
    assert_eq!(region.thickness, 0.25);
    assert_eq!(region.outline.len(), 4, "the polygon is carried through untouched");
    assert_eq!(region.material_id, "concrete");
}

#[semio_framework_async_macros::async_test]
async fn patch_region_emits_one_whole_record_replace_2d() {
    let emitted = emit(&demo(), PatchRegion { id: "r1".into(), field: "meshSize".into(), value: "0.5".into() }).expect("handle");
    let [Fem2dMutation::ChangeRegion(change)] = emitted.artifact_mutations.as_slice() else { panic!("one change-region") };
    assert_eq!(change.id, "r1");
    assert_eq!(change.patch.mesh_size.unwrap(), 0.5);
    assert!(change.patch.outline.is_none(), "the event owns no polygon field");
}

#[semio_framework_async_macros::async_test]
async fn patch_region_rejects_an_unknown_field_an_unparsable_value_and_a_missing_region_2d() {
    let demo = demo();
    assert!(emit(&demo, PatchRegion { id: "r1".into(), field: "outline".into(), value: "[]".into() }).is_err());
    assert!(emit(&demo, PatchRegion { id: "r1".into(), field: "meshSize".into(), value: "fine".into() }).is_err());
    assert!(emit(&demo, PatchRegion { id: "nowhere".into(), field: "thickness".into(), value: "1".into() }).is_err());
}

#[semio_framework_async_macros::async_test]
async fn patch_region_with_the_current_value_emits_nothing_2d() {
    let emitted = emit(&demo(), PatchRegion { id: "r1".into(), field: "materialId".into(), value: "concrete".into() }).expect("handle");
    assert!(emitted.artifact_mutations.is_empty());
}

/// 👥️ Two inspector commands authored on one base compose their independent intent in either replay order.
#[test]
fn sibling_field_commands_and_inverse_preserve_concurrent_intent(){
    let base=semio_framework_pack_json::from_json_str::<Fem2dSnapshot>(include_str!("../../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-region/🩹️changes/📸️snapshot/⬅️before/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral fixture");
    let first=emit(&base,PatchRegion{id:"r1".into(),field:"name".into(),value:"Mine".into()}).unwrap().artifact_mutations.remove(0);
    let second=emit(&base,PatchRegion{id:"r1".into(),field:"thickness".into(),value:"0.3".into()}).unwrap().artifact_mutations.remove(0);
    let first_then_second=vcs::apply_mutation(&vcs::apply_mutation(&base,&first).unwrap().0,&second).unwrap().0;
    let combined=vcs::apply_mutation(&vcs::apply_mutation(&base,&second).unwrap().0,&first).unwrap().0;
    assert_eq!(combined,first_then_second);
    assert_eq!(combined.regions[0].name,"Mine");assert_eq!(combined.regions[0].thickness,0.3);
    let mut undo=combined.clone();for inverse in protocol::Mutation::inverse(&first,&base).unwrap().iter().rev(){undo=vcs::apply_mutation(&undo,inverse).unwrap().0;}
    assert_eq!(undo,vcs::apply_mutation(&base,&second).unwrap().0,"undo owns no sibling field");
    let first_diff=protocol::Mutation::diff(&first,&base).into_parts().0;let second_diff=protocol::Mutation::diff(&second,&base).into_parts().0;
    let mut sum=first_diff.clone();protocol::ColdMutationDiff::absorb(&mut sum,second_diff);
    assert_eq!(protocol::apply_diff_cold(&sum,&base).unwrap(),combined);
    assert_eq!(protocol::apply_diff_cold(&protocol::DiffAlgebra::inverse(&sum,&base),&combined).unwrap(),base);
    eprintln!("[DEBUG] region same-base inspector fields and sparse inverse preserve concurrent sibling intent");
}