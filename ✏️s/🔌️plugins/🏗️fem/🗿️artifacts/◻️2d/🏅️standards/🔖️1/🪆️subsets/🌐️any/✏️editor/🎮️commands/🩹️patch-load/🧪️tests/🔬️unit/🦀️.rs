use super::*;
use crate::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app};
use crate::editor::fem2d::Fem2dCommand;
use crate::load_id;
use store::ArtifactDsl;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("the bundled demo fixture parses")
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
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    dispatch(&mut app, Fem2dCommand::PatchLoad(PatchLoad { id: "l6".into(), field: "value".into(), value: "-8000".into() })).await;
    let FemLoad::Nodal { node_id, dof, value, .. } = load_of(&app.snapshot().expect("snapshot"), "l6") else { panic!("l6 stays nodal") };
    assert_eq!(value, -8000.0);
    assert_eq!((node_id.as_str(), dof), ("p8_l1", FemDof::Ty), "every field but the edited one is carried through");
}

#[semio_framework_async_macros::async_test]
async fn patch_load_edits_an_area_pressure_and_resolves_its_owning_case_2d() {
    let emitted = emit(&demo(), PatchLoad { id: "l5".into(), field: "pressure".into(), value: "1000".into() }).expect("handle");
    let [Fem2dMutation::ReplaceLoad(replace)] = emitted.artifact_mutations.as_slice() else { panic!("one replace-load") };
    assert_eq!(replace.case_id, "dead", "the owning case is resolved by lookup, never carried in the argument map");
    assert_eq!(replace.load_id, "l5");
    let FemLoad::Area { pressure, region_id, .. } = replace.new_load.as_ref() else { panic!("l5 stays an area load") };
    assert_eq!((*pressure, region_id.as_str()), (1000.0, "r1"));
}

#[semio_framework_async_macros::async_test]
async fn patch_load_rejects_a_field_the_addressed_variant_does_not_have_2d() {
    let demo = demo();
    assert!(emit(&demo, PatchLoad { id: "l6".into(), field: "pressure".into(), value: "1".into() }).is_err(), "a nodal load has no pressure");
    assert!(emit(&demo, PatchLoad { id: "l5".into(), field: "dof".into(), value: "Ty".into() }).is_err(), "an area load has no degree of freedom");
    assert!(emit(&demo, PatchLoad { id: "l6".into(), field: "dof".into(), value: "sideways".into() }).is_err());
    assert!(emit(&demo, PatchLoad { id: "nowhere".into(), field: "value".into(), value: "1".into() }).is_err());
}

#[semio_framework_async_macros::async_test]
async fn patch_load_with_the_current_value_emits_nothing_2d() {
    let emitted = emit(&demo(), PatchLoad { id: "l6".into(), field: "value".into(), value: "-12000".into() }).expect("handle");
    assert!(emitted.artifact_mutations.is_empty());
}
