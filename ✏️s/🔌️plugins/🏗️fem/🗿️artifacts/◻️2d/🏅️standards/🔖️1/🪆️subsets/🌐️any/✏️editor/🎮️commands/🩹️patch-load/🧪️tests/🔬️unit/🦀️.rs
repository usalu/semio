use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use super::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::context::dispatch;
use crate::context::fem2d_app;
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::load_id;
use pack::snapshot::ArtifactDsl;
#[cfg(test)] use semio_framework_plugin::component::ArtifactView;
#[cfg(test)] use semio_framework_plugin::component::NoConfig;
#[cfg(test)] use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
#[cfg(test)] use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dLoadPatch;
#[cfg(test)] use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_load::PatchLoad;
#[cfg(test)] use semio_framework_plugin::component::Emit;
#[cfg(test)] use semio_framework_plugin::component::NoConfigMutation;
#[cfg(test)] use semio_framework::Fault;

fn demo() -> Fem2dSnapshot {
    semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::parse_dsl(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT).expect("the bundled demo fixture parses")
}

fn emit(snapshot: &Fem2dSnapshot, payload: PatchLoad) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

fn load_of(snapshot: &Fem2dSnapshot, id: &str) -> FemLoad {
    snapshot.load_cases.iter().flat_map(|case| case.loads.iter()).find(|load| load_id(load) == id).expect("the load survives the patch").clone()
}

#[semio_framework_async_macros::async_test]
async fn patch_load_edits_a_nodal_magnitude_on_the_demo_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: semio_s_artifact_fem_2d_io::examples::demo::ID.into() })).await;
    dispatch(&mut app, Fem2dCommand::PatchLoad(PatchLoad { id: "scope:1:4:load4:live2:l6".into(), field: "value".into(), value: "-8000".into() })).await;
    let FemLoad::Nodal { node_id, dof, value, .. } = load_of(&app.snapshot().expect("snapshot"), "l6") else { panic!("l6 stays nodal") };
    assert_eq!(value, -8000.0);
    assert_eq!((node_id.as_str(), dof), ("p8_l1", FemDof::Ty), "every field but the edited one is carried through");
}

#[semio_framework_async_macros::async_test]
async fn patch_load_edits_an_area_pressure_and_resolves_its_owning_case_2d() {
    let emitted = emit(&demo(), PatchLoad { id: "scope:1:4:load4:dead2:l5".into(), field: "pressure".into(), value: "1000".into() }).expect("handle");
    let [Fem2dMutation::ChangeLoad(change)] = emitted.artifact_mutations.as_slice() else { panic!("one change-load") };
    assert_eq!(change.case_id, "dead", "the owning case is resolved by lookup, never carried in the argument map");
    assert_eq!(change.id, "l5");
    let Fem2dLoadPatch::Area { pressure, region_id } = &change.patch else { panic!("area field intent") };
    assert_eq!(*pressure, Some(1000.0));
    assert!(region_id.is_none(), "pressure owns no region reference");
}

#[semio_framework_async_macros::async_test]
async fn patch_load_rejects_a_field_the_addressed_variant_does_not_have_2d() {
    let demo = demo();
    assert!(emit(&demo, PatchLoad { id: "scope:1:4:load4:live2:l6".into(), field: "pressure".into(), value: "1".into() }).is_err(), "a nodal load has no pressure");
    assert!(emit(&demo, PatchLoad { id: "scope:1:4:load4:dead2:l5".into(), field: "dof".into(), value: "Ty".into() }).is_err(), "an area load has no degree of freedom");
    assert!(emit(&demo, PatchLoad { id: "scope:1:4:load4:live2:l6".into(), field: "dof".into(), value: "sideways".into() }).is_err());
    assert!(emit(&demo, PatchLoad { id: "nowhere".into(), field: "value".into(), value: "1".into() }).is_err());
}

#[semio_framework_async_macros::async_test]
async fn patch_load_with_the_current_value_emits_nothing_2d() {
    let emitted = emit(&demo(), PatchLoad { id: "scope:1:4:load4:live2:l6".into(), field: "value".into(), value: "-12000".into() }).expect("handle");
    assert!(emitted.artifact_mutations.is_empty());
}

/// 👥️ Same-base field commands preserve both nodal edits and undo only their authored field.
#[test]
fn sibling_load_commands_preserve_independent_intent(){
    let base=semio_framework_pack_json::from_json_str::<Fem2dSnapshot>(include_str!("../../../../../../🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/🩹️changes/📸️snapshot/⬅️before/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let first=emit(&base,PatchLoad{id:"l1".into(),field:"value".into(),value:"20".into()}).unwrap().artifact_mutations.remove(0);
    let second=emit(&base,PatchLoad{id:"l1".into(),field:"nodeId".into(),value:"n2".into()}).unwrap().artifact_mutations.remove(0);
    let combined=vcs::apply_mutation(&vcs::apply_mutation(&base,&first).unwrap().0,&second).unwrap().0;
    assert_eq!(combined,vcs::apply_mutation(&vcs::apply_mutation(&base,&second).unwrap().0,&first).unwrap().0);
    let semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoad::Nodal{node_id,value,..}=&combined.load_cases[0].loads[0] else{panic!("nodal variant preserved")};
    assert_eq!(node_id,"n2");assert_eq!(*value,20.0);
    let mut undone=combined.clone();for inverse in protocol::Mutation::inverse(&first,&base).unwrap().iter().rev(){undone=vcs::apply_mutation(&undone,inverse).unwrap().0;}
    assert_eq!(undone,vcs::apply_mutation(&base,&second).unwrap().0);
    let mut sum=protocol::Mutation::diff(&first,&base).into_parts().0;protocol::ColdMutationDiff::absorb(&mut sum,protocol::Mutation::diff(&second,&base).into_parts().0);
    assert_eq!(protocol::apply_diff_cold(&sum,&base).unwrap(),combined);
    assert_eq!(protocol::apply_diff_cold(&protocol::DiffAlgebra::inverse(&sum,&base),&combined).unwrap(),base);
    eprintln!("[DEBUG] nodal magnitude/reference inspector intents compose and undo without sibling loss");
}