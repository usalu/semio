use super::*;
use crate::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app};
use crate::editor::fem2d::Fem2dCommand;
use store::ArtifactDsl;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("the bundled demo fixture parses")
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
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
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
    let [Fem2dMutation::ReplaceElement(replace)] = emitted.artifact_mutations.as_slice() else { panic!("one replace-element") };
    let FemElement::Bar { id, start, end, material_id, section_id } = replace.new_element.as_ref() else { panic!("the kind edit swaps the variant") };
    assert_eq!((id.as_str(), start.as_str(), end.as_str(), material_id.as_str(), section_id.as_str()), ("e3", "n1", "n2", "steel", "chs76"));
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
