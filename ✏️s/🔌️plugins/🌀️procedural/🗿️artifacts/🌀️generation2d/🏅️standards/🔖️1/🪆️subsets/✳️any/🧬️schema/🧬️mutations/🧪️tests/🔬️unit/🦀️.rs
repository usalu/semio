use super::*;
use crate::standards::v1::subsets::any::schema::empty_generation2d_snapshot;
use protocol::{Mutation, MutationDiff, SemanticMutation};
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_artifact_flow_flow::{CameraJson, SynapseSpec, WidgetLayout};
use crate::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshotRead;
use semio_framework_os_kernel::os_spr::testkit::{assert_mutation_diff_absorb_law_cold, assert_mutation_inverse_law, assert_mutation_inverse_law_cold};

/// 🧊️ Every owned projection this suite materialises is CLOSED, never dropped — `fixture.layout`
/// is an `OrderedMap` root and `generation` carries its own retirement ladder.
fn retire_snapshot(snapshot: Generation2dSnapshot) {
    snapshot.retire_cold();
}

/// 🧊️ The diff twin of [`retire_snapshot`] — a raised delta owns the projections it displaces.
fn retire_diff(diff: Generation2dDiff) {
    diff.retire_cold();
}

fn round_trip(projection: &Generation2dSnapshot, mutation: &Generation2dMutation) -> Generation2dSnapshotRead {
    let mut forward = Generation2dSnapshotRead::new(projection.clone());
    apply_generation2d_mutation(&mut forward, mutation).expect("valid mutation");
    let mut restored = Generation2dSnapshotRead::new((*forward).clone());
    for back in mutation.inverse(projection) {
        apply_generation2d_mutation(&mut restored, &back).expect("valid inverse mutation");
    }
    assert_eq!(restored, *projection, "inverse() must restore the pre-mutation document");
    forward
}

#[test]
fn fixture_ops_ignore_camera() {
    let before = FlowFixture::default();
    let mut after = before.clone();
    after.camera = CameraJson { x: 7.0, y: 8.0, zoom: 2.0 };
    let operations = generation2d_fixture_operations(&before, &after);
    assert!(operations.iter().all(|operation| !matches!(operation, Generation2dMutation::UpdateCamera(_))));
}

#[test]
fn delete_and_recreate_widget_round_trips() {
    let base = empty_generation2d_snapshot();
    let removed_id = widget_id(&base.fixture.widgets[0]).to_string();
    let after = round_trip(&base, &delete_widget(removed_id.clone()));
    assert!(!after.fixture.widgets.iter().any(|w| widget_id(w) == removed_id));
}

#[test]
fn fixture_ops_capture_widget_creation() {
    let before = FlowFixture::default();
    let mut after = before.clone();
    after.widgets.push(Widget::InputNote { id: "note-1".into(), text: String::new() });
    let operations = generation2d_fixture_operations(&before, &after);
    assert!(operations.iter().any(|operation| matches!(operation, Generation2dMutation::CreateWidget(payload) if widget_id(&payload.widget) == "note-1")));
}

#[test]
fn fixture_ops_capture_widget_replacement() {
    let mut before = FlowFixture::default();
    before.widgets.clear();
    before.widgets.push(Widget::InputNote { id: "note-1".into(), text: "old".into() });
    let mut after = before.clone();
    after.widgets[0] = Widget::InputNote { id: "note-1".into(), text: "new".into() };
    let operations = generation2d_fixture_operations(&before, &after);
    assert!(operations.iter().any(|operation| matches!(operation, Generation2dMutation::ReplaceWidget(payload) if widget_id(&payload.widget) == "note-1")));
}

#[test]
fn generation_lifecycle_round_trips() {
    let before = Generation2dSnapshotRead::new(empty_generation2d_snapshot());
    let generation = FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: Default::default() };
    let after = round_trip(&before, &create_generation(generation));
    assert_eq!(after.generation.generations.len(), 1);
}

//#region 🔖️MutationInverseLawTests
#[semio_framework_async_macros::async_test]
async fn create_widget_inverse_law() {
    let base = empty_generation2d_snapshot();
    let mutation = create_widget(0, Widget::InputNote { id: "brand-new".into(), text: String::new() });
    assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn replace_widget_inverse_law() {
    let base = empty_generation2d_snapshot();
    let id = widget_id(&base.fixture.widgets[1]).to_string();
    let mutation = replace_widget(Widget::InputNote { id, text: "replaced".into() });
    assert_mutation_inverse_law(&base, &mutation).await;
}

#[test]
fn replace_widget_on_unknown_id_is_a_noop_with_no_inverse() {
    let base = empty_generation2d_snapshot();
    let mutation = replace_widget(Widget::InputNote { id: "does-not-exist".into(), text: String::new() });
    assert!(mutation.inverse(&base).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn delete_widget_inverse_law() {
    let base = empty_generation2d_snapshot();
    let id = widget_id(&base.fixture.widgets[1]).to_string();
    let mutation = delete_widget(id);
    assert_mutation_inverse_law(&base, &mutation).await;
}

#[test]
fn delete_widget_on_unknown_id_is_a_noop_with_no_inverse() {
    let base = empty_generation2d_snapshot();
    let mutation = delete_widget("does-not-exist".into());
    assert!(mutation.inverse(&base).is_empty());
    let after = round_trip(&base, &mutation);
    assert_eq!(after, base);
}

#[semio_framework_async_macros::async_test]
async fn connect_synapse_inverse_law() {
    let base = empty_generation2d_snapshot();
    let synapse = SynapseSpec { id: "brand-new-synapse".into(), from: "slider".into(), to: "add".into(), from_port: "number".into(), to_port: "b".into() };
    let mutation = connect_synapse(0, synapse);
    assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn replace_synapse_inverse_law() {
    let base = empty_generation2d_snapshot();
    let id = base.fixture.synapses[0].id.clone();
    let mutation = replace_synapse(SynapseSpec { id, from: "add".into(), to: "preview".into(), from_port: "sum".into(), to_port: "changed".into() });
    assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn disconnect_synapse_inverse_law() {
    let base = empty_generation2d_snapshot();
    let id = base.fixture.synapses[0].id.clone();
    let mutation = disconnect_synapse(id);
    assert_mutation_inverse_law(&base, &mutation).await;
}

#[test]
fn disconnect_synapse_on_unknown_id_is_a_noop_with_no_inverse() {
    let base = empty_generation2d_snapshot();
    let mutation = disconnect_synapse("missing".into());
    assert!(mutation.inverse(&base).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn move_widget_inverse_law_over_prior_layout() {
    let mut base = Generation2dSnapshotRead::new(empty_generation2d_snapshot());
    let id = widget_id(&base.fixture.widgets[0]).to_string();
    base.fixture.layout.insert(id.clone(), WidgetLayout { x: 1.0, y: 1.0 });
    let mutation = move_widget(id, WidgetLayout { x: 9.0, y: 9.0 });
    assert_mutation_inverse_law_cold(&*base, &mutation, retire_snapshot, retire_diff).await;
}

#[test]
fn move_widget_creating_a_layout_entry_clears_on_undo() {
    let base = Generation2dSnapshotRead::new(empty_generation2d_snapshot());
    assert!(base.fixture.layout.is_empty());
    let mutation = move_widget("slider".into(), WidgetLayout { x: 2.0, y: 2.0 });
    let after = round_trip(&base, &mutation);
    assert!(after.fixture.layout.contains_key("slider"));
}

#[semio_framework_async_macros::async_test]
async fn clear_widget_layout_inverse_law() {
    let mut base = Generation2dSnapshotRead::new(empty_generation2d_snapshot());
    base.fixture.layout.insert("slider".into(), WidgetLayout { x: 4.0, y: 5.0 });
    let mutation = clear_widget_layout("slider".into());
    assert_mutation_inverse_law_cold(&*base, &mutation, retire_snapshot, retire_diff).await;
}

#[test]
fn clear_widget_layout_on_unknown_id_is_a_noop_with_no_inverse() {
    let base = empty_generation2d_snapshot();
    let mutation = clear_widget_layout("missing".into());
    assert!(mutation.inverse(&base).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn update_camera_inverse_law() {
    let base = empty_generation2d_snapshot();
    let mutation = update_camera(CameraJson { x: 42.0, y: -3.0, zoom: 5.0 });
    assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn change_schema_inverse_law() {
    let base = empty_generation2d_snapshot();
    let mutation = change_schema("changed.schema".into());
    assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn create_generation_inverse_law() {
    let base = Generation2dSnapshotRead::new(empty_generation2d_snapshot());
    let generation = FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: Default::default() };
    assert_mutation_inverse_law_cold(&*base, &create_generation(generation), retire_snapshot, retire_diff).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_generation_inverse_law() {
    let mut base = Generation2dSnapshotRead::new(empty_generation2d_snapshot());
    base.generation.cold_builder_mut().expect("unique cold generation owner").generations.push(FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: Default::default() });
    assert_mutation_inverse_law_cold(&*base, &rename_generation("generation-1".into(), "Renamed".into()), retire_snapshot, retire_diff).await;
}

#[semio_framework_async_macros::async_test]
async fn change_generation_value_diff_absorb_law() {
    let mut base = Generation2dSnapshotRead::new(empty_generation2d_snapshot());
    base.generation.cold_builder_mut().expect("unique cold generation owner").generations.push(FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: Default::default() });
    let d1 = change_generation_value("generation-1".into(), "q1".into(), dsl::DslValue::float(1.0)).diff(&*base).into_parts().0;
    let mid = Generation2dSnapshotRead::new(d1.apply(&*base).expect("valid mutation diff"));
    let d2 = change_generation_value("generation-1".into(), "q1".into(), dsl::DslValue::float(2.0)).diff(&*mid).into_parts().0;
    assert_mutation_diff_absorb_law_cold(&*base, d1, d2, retire_snapshot, retire_diff).await;
}
//#endregion 🔖️MutationInverseLawTests

#[test]
fn dispatch_registers_semantic_descriptors() {
    register_generation2d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in Generation2dMutation::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(Generation2dMutation::kinds().len(), 14);
}

//#region 🔖️FixtureOpsTests
#[test]
fn fixture_ops_widget_id_matches_every_widget_kind() {
    let widgets = vec![
        Widget::Neuron { id: "w-neuron".into(), neuron_kind: "math.add".into(), params: Default::default(), input_ports: vec![], output_ports: vec![], preview: true },
        Widget::InputSlider { id: "w-slider".into(), label: "Width".into(), value: 1.0, min: 0.0, max: 2.0, step: 0.5 },
        Widget::InputNote { id: "w-note".into(), text: String::new() },
        Widget::InputImage { id: "w-image".into(), src: String::new() },
        Widget::Variable { id: "w-variable".into(), name: "value".into(), schema: "dictionary".into() },
        Widget::OutputPreview { id: "w-preview".into(), preview: Default::default(), expanded: Default::default() },
        Widget::OutputAction { id: "w-action".into(), action: String::new() },
        Widget::OutputExport { id: "w-export".into(), format: "svg".into() },
        Widget::Cluster { id: "w-cluster".into(), name: String::new(), tree: Default::default(), flow: Default::default() },
    ];
    let mut before = FlowFixture::default();
    before.widgets.clear();
    let mut after = before.clone();
    after.widgets = widgets.clone();
    let operations = generation2d_fixture_operations(&before, &after);
    for widget in &widgets {
        let id = widget_id(widget);
        assert!(operations.iter().any(|op| matches!(op, Generation2dMutation::CreateWidget(payload) if widget_id(&payload.widget) == id)));
    }
}

#[test]
fn widgets_diff_apply_replaces_by_id_and_removes_by_id() {
    let mut widgets = vec![Widget::InputNote { id: "a".into(), text: "1".into() }, Widget::InputNote { id: "b".into(), text: "2".into() }];
    let diff = crate::standards::v1::subsets::any::schema::diff::WidgetsDiff { removed: vec!["b".into()], set: vec![(0, Widget::InputNote { id: "a".into(), text: "replaced".into() })] };
    crate::standards::v1::subsets::any::schema::diff::apply_widgets_diff(&mut widgets, &diff);
    assert_eq!(widgets, vec![Widget::InputNote { id: "a".into(), text: "replaced".into() }]);
}

#[test]
fn synapses_diff_apply_replaces_by_id_and_removes_by_id() {
    let mut synapses = vec![SynapseSpec { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() }];
    let diff = crate::standards::v1::subsets::any::schema::diff::SynapsesDiff { removed: vec![], set: vec![(0, SynapseSpec { id: "s1".into(), from: "a".into(), to: "c".into(), from_port: "out".into(), to_port: "in".into() })] };
    crate::standards::v1::subsets::any::schema::diff::apply_synapses_diff(&mut synapses, &diff);
    assert_eq!(synapses[0].to, "c");
}
//#endregion 🔖️FixtureOpsTests

//#region 🧪️KindsCatalog
/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed oracle
/// manifest's catalog — the framework never parses Rust, so this is the only thing that keeps the
/// declared vocabulary and the measured one from drifting apart.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <Generation2dMutation as SemanticMutation<Generation2dSnapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared Generation2dMutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog

