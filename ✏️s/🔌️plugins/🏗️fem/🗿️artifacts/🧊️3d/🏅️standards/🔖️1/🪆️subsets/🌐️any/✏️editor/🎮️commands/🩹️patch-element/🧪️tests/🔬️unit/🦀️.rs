use super::*;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement;
use crate::context::dispatch;
use crate::context::fem3d_demo_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;

fn demo() -> Fem3dSnapshot {
    semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot()
}

fn emit(snapshot: &Fem3dSnapshot, payload: PatchElement) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_element_re_spells_kind_references_and_roll_3d() {
    let mut app = fem3d_demo_app().await;
    dispatch(&mut app, Fem3dCommand::PatchElement(PatchElement { id: "e1".into(), field: "roll".into(), value: "0.25".into() })).await;
    dispatch(&mut app, Fem3dCommand::PatchElement(PatchElement { id: "fb1_0".into(), field: "kind".into(), value: "bar".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    assert!(matches!(snapshot.elements.iter().find(|element| element_id(element) == "e1"), Some(FemElement::Frame { roll, .. }) if *roll == 0.25));
    assert!(matches!(snapshot.elements.iter().find(|element| element_id(element) == "fb1_0"), Some(FemElement::Bar { .. })));
    let emitted = emit(&snapshot, PatchElement { id: "fb1_0".into(), field: "kind".into(), value: "frame".into() }).expect("bar → frame");
    let [Fem3dMutation::ChangeElement(change)] = emitted.artifact_mutations.as_slice() else { panic!("one change-element") };
    assert_eq!(change.patch, Fem3dElementPatch { kind: Some("frame".into()), ..Default::default() });
}

#[semio_framework_async_macros::async_test]
async fn patch_element_refuses_a_roll_on_a_bar_and_unknown_fields_3d() {
    let mut demo = demo();
    demo.elements[0] = FemElement::Bar { id: "e1".into(), start: "n00_g".into(), end: "n00_l1".into(), material_id: "steel".into(), section_id: "hea200".into() };
    assert!(emit(&demo, PatchElement { id: "e1".into(), field: "roll".into(), value: "0.1".into() }).is_err());
    assert!(emit(&demo, PatchElement { id: "e1".into(), field: "colour".into(), value: "red".into() }).is_err());
    assert!(emit(&demo, PatchElement { id: "ghost".into(), field: "start".into(), value: "n00_g".into() }).is_err());
    assert!(emit(&demo, PatchElement { id: "e1".into(), field: "start".into(), value: "n00_g".into() }).expect("handle").artifact_mutations.is_empty());
}

/// 👥️ Two inspector commands authored on one base compose their independent intent in either replay order.
#[test]
fn sibling_field_commands_and_inverse_preserve_concurrent_intent(){
    let base=semio_framework_pack_json::from_json_str::<Fem3dSnapshot>(include_str!("../../../../../../🕸️mesh/🧫️fixtures/🧬️mutations/🩹️change-element/🩹️changes/📸️snapshot/⬅️before/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral fixture");
    let first=emit(&base,PatchElement{id:"e1".into(),field:"start".into(),value:"n3".into()}).unwrap().artifact_mutations.remove(0);
    let second=emit(&base,PatchElement{id:"e1".into(),field:"kind".into(),value:"frame".into()}).unwrap().artifact_mutations.remove(0);
    let first_then_second=vcs::apply_mutation(&vcs::apply_mutation(&base,&first).unwrap().0,&second).unwrap().0;
    let combined=vcs::apply_mutation(&vcs::apply_mutation(&base,&second).unwrap().0,&first).unwrap().0;
    assert_eq!(combined,first_then_second);
    assert!(matches!(&combined.elements[0],crate::FemElement::Frame{start,..} if start=="n3"));
    let mut undo=combined.clone();for inverse in protocol::Mutation::inverse(&first,&base).unwrap().iter().rev(){undo=vcs::apply_mutation(&undo,inverse).unwrap().0;}
    assert_eq!(undo,vcs::apply_mutation(&base,&second).unwrap().0,"undo owns no sibling field");
    let first_diff=protocol::Mutation::diff(&first,&base).into_parts().0;let second_diff=protocol::Mutation::diff(&second,&base).into_parts().0;
    let mut sum=first_diff.clone();protocol::ColdMutationDiff::absorb(&mut sum,second_diff);
    assert_eq!(protocol::apply_diff_cold(&sum,&base).unwrap(),combined);
    assert_eq!(protocol::apply_diff_cold(&protocol::DiffAlgebra::inverse(&sum,&base),&combined).unwrap(),base);
    eprintln!("[DEBUG] element same-base inspector fields and sparse inverse preserve concurrent sibling intent");
}