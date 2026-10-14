use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_element::*;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_element;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::context::{dispatch, fem3d_demo_app};
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
    let [Fem3dMutation::ReplaceElement(replace)] = emitted.artifact_mutations.as_slice() else { panic!("one replace-element") };
    assert!(matches!(*replace.new_element, FemElement::Frame { roll, .. } if roll == 0.0), "a bar becomes a frame with zero roll");
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
