use super::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement;
use crate::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::context::dispatch;
use crate::context::fem2d_app;
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use store::os_store::serialization::ArtifactDsl;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT).expect("the bundled demo fixture parses")
}

fn emit(snapshot: &Fem2dSnapshot, payload: PatchElement) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_element_repoints_a_section_on_the_demo_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: semio_s_artifact_fem_2d_io::examples::demo::ID.into() })).await;
    dispatch(&mut app, Fem2dCommand::PatchElement(PatchElement { id: "e3".into(), field: "sectionId".into(), value: "post140".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let element = snapshot.elements.iter().find(|element| element_id(element) == "e3").expect("e3 survives the patch");
    let FemElement::Beam { section_id, material_id, .. } = element else { panic!("e3 stays a beam") };
    assert_eq!(section_id, "post140");
    assert_eq!(material_id, "steel", "every field but the edited one is carried through");
}

#[semio_framework_async_macros::async_test]
async fn patch_element_kind_swaps_the_variant_and_keeps_every_id_2d() {
    let emitted = emit(&demo(), PatchElement { id: "e3".into(), field: "kind".into(), value: "bar".into() }).expect("handle");
    let [Fem2dMutation::ChangeElement(change)] = emitted.artifact_mutations.as_slice() else { panic!("one change-element") };
    assert_eq!(change.id, "e3");
    assert_eq!(change.patch, Fem2dElementPatch { kind: Some("bar".into()), ..Default::default() });
}

#[semio_framework_async_macros::async_test]
async fn patch_element_rejects_an_unknown_field_an_unknown_kind_and_a_missing_element_2d() {
    let demo = demo();
    assert!(emit(&demo, PatchElement { id: "e3".into(), field: "colour".into(), value: "red".into() }).is_err());
    assert!(emit(&demo, PatchElement { id: "e3".into(), field: "kind".into(), value: "truss".into() }).is_err());
    assert!(emit(&demo, PatchElement { id: "nowhere".into(), field: "start".into(), value: "n1".into() }).is_err());
}

#[semio_framework_async_macros::async_test]
async fn patch_element_with_the_current_reference_emits_nothing_2d() {
    let emitted = emit(&demo(), PatchElement { id: "e3".into(), field: "materialId".into(), value: "steel".into() }).expect("handle");
    assert!(emitted.artifact_mutations.is_empty());
}

/// 👥️ Two inspector commands authored on one base compose their independent intent in either replay order.
#[test]
fn sibling_field_commands_and_inverse_preserve_concurrent_intent(){
    let base=semio_framework_pack_json::from_json_str::<Fem2dSnapshot>(include_str!("../../../../../../🕸️mesh/🧫️fixtures/🧬️mutations/🩹️change-element/🩹️changes/📸️snapshot/⬅️before/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral fixture");
    let first=emit(&base,PatchElement{id:"e1".into(),field:"start".into(),value:"n3".into()}).unwrap().artifact_mutations.remove(0);
    let second=emit(&base,PatchElement{id:"e1".into(),field:"kind".into(),value:"beam".into()}).unwrap().artifact_mutations.remove(0);
    let first_then_second=vcs::apply_mutation(&vcs::apply_mutation(&base,&first).unwrap().0,&second).unwrap().0;
    let combined=vcs::apply_mutation(&vcs::apply_mutation(&base,&second).unwrap().0,&first).unwrap().0;
    assert_eq!(combined,first_then_second);
    assert!(matches!(&combined.elements[0],crate::FemElement::Beam{start,..} if start=="n3"));
    let mut undo=combined.clone();for inverse in protocol::Mutation::inverse(&first,&base).unwrap().iter().rev(){undo=vcs::apply_mutation(&undo,inverse).unwrap().0;}
    assert_eq!(undo,vcs::apply_mutation(&base,&second).unwrap().0,"undo owns no sibling field");
    let first_diff=protocol::Mutation::diff(&first,&base).into_parts().0;let second_diff=protocol::Mutation::diff(&second,&base).into_parts().0;
    let mut sum=first_diff.clone();protocol::ColdMutationDiff::absorb(&mut sum,second_diff);
    assert_eq!(protocol::apply_diff_cold(&sum,&base).unwrap(),combined);
    assert_eq!(protocol::apply_diff_cold(&protocol::DiffAlgebra::inverse(&sum,&base),&combined).unwrap(),base);
    eprintln!("[DEBUG] element same-base inspector fields and sparse inverse preserve concurrent sibling intent");
}