use super::*;
use crate::standards::v1::subsets::any::schema::default_generation3d_snapshot;
use change_generation_value::ChangeGenerationValue;
use change_schema::ChangeSchema;
use connect_synapse::ConnectSynapse;
use create_generation::CreateGeneration;
use create_widget::CreateWidget;
use delete_generation::DeleteGeneration;
use delete_widget::DeleteWidget;
use delete_widget_position::DeleteWidgetPosition;
use disconnect_synapse::DisconnectSynapse;
use move_widget::MoveWidget;
use protocol::Mutation;
use semio_framework_artifact_flow_flow::{CameraJson, SynapseSpec, Widget, WidgetLayout};
use semio_framework_artifact_playbook_playbook::FormGeneration;

use rename_generation::RenameGeneration;
use update_camera::UpdateCamera;
use update_synapse::UpdateSynapse;
use update_widget::UpdateWidget;

fn round_trip(projection: &Generation3dSnapshot, operation: &Generation3dMutation) -> crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead {
    let mut forward = crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new(projection.clone());
    apply_generation3d_mutation(&mut forward, operation).expect("valid mutation");
    let mut restored = crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new((*forward).clone());
    for back in operation.inverse(projection) {
        apply_generation3d_mutation(&mut restored, &back).expect("valid inverse mutation");
    }
    assert_eq!(restored, *projection, "inverse(base) must restore the pre-operation document");
    forward
}

/// ⚖️ One value per `Generation3dMutation` variant — the closed set the wire/semantics tests
/// below iterate.
fn every_mutation() -> Vec<Generation3dMutation> {
    vec![
        Generation3dMutation::CreateWidget(CreateWidget { index: 0, widget: Widget::InputNote { id: "note-fresh".into(), text: String::new() } }),
        Generation3dMutation::UpdateWidget(UpdateWidget { widget: Widget::InputNote { id: "note-9".into(), text: "new".into() } }),
        Generation3dMutation::DeleteWidget(DeleteWidget { id: "note-9".into() }),
        Generation3dMutation::ConnectSynapse(ConnectSynapse { index: 0, synapse: SynapseSpec { id: "e-fresh".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() } }),
        Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse: SynapseSpec { id: "e1".into(), from: "a".into(), to: "c".into(), from_port: "out".into(), to_port: "in".into() } }),
        Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: "e1".into() }),
        Generation3dMutation::MoveWidget(MoveWidget { id: "extrude".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } }),
        Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: "extrude".into() }),
        Generation3dMutation::UpdateCamera(UpdateCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } }),
        Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: "flow.fixture.v2".into() }),
        Generation3dMutation::CreateGeneration(CreateGeneration { generation: FormGeneration { id: "generation-fresh".into(), name: "Generation".into(), values: Default::default() } }),
        Generation3dMutation::DeleteGeneration(DeleteGeneration { id: "generation-1".into() }),
        Generation3dMutation::RenameGeneration(RenameGeneration { id: "generation-1".into(), new_name: "Renamed".into() }),
        Generation3dMutation::ChangeGenerationValue(ChangeGenerationValue { id: "generation-1".into(), question_id: "q1".into(), new_value: serde_json::json!(42).into() }),
    ]
}

#[test]
fn every_variant_registers_an_approved_semantic_descriptor() {
    for op in every_mutation() {
        let descriptor = protocol::SemanticMutation::semantics(&op);
        assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {op:?}", descriptor.verb);
    }
    assert_eq!(<Generation3dMutation as protocol::SemanticMutation<Generation3dSnapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
}

#[semio_framework_async_macros::async_test]
async fn store_applies_widget_create() {
    let mut store = crate::store_fixture::document_store(default_generation3d_snapshot()).await;
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![Generation3dMutation::CreateWidget(CreateWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } })], description: None }).await.expect("apply");
    assert!(store.snapshot().expect("snapshot").fixture.widgets.iter().any(|w| widget_id(w) == "note-9"));
    crate::store_fixture::close(store);
}

#[test]
fn create_widget_round_trips() {
    let before = default_generation3d_snapshot();
    let after = round_trip(&before, &Generation3dMutation::CreateWidget(CreateWidget { index: 9, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }));
    assert!(after.fixture.widgets.iter().any(|w| widget_id(w) == "note-9"));
}

#[test]
fn generation_op_round_trips() {
    let before = default_generation3d_snapshot();
    let generation = FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: Default::default() };
    let after = round_trip(&before, &Generation3dMutation::CreateGeneration(CreateGeneration { generation }));
    assert_eq!(after.generation.generations.len(), 1);
}

#[test]
fn generation_mutation_bridge_covers_every_variant() {
    let generation = FormGeneration { id: "g1".into(), name: "G1".into(), values: Default::default() };
    assert_eq!(generation_mutation_to_generation3d(GenerationMutation::Add { generation: generation.clone() }), Generation3dMutation::CreateGeneration(CreateGeneration { generation }));
    assert_eq!(generation_mutation_to_generation3d(GenerationMutation::Remove { id: "g1".into() }), Generation3dMutation::DeleteGeneration(DeleteGeneration { id: "g1".into() }));
    assert_eq!(generation_mutation_to_generation3d(GenerationMutation::Rename { id: "g1".into(), name: "New".into() }), Generation3dMutation::RenameGeneration(RenameGeneration { id: "g1".into(), new_name: "New".into() }));
    assert_eq!(
        generation_mutation_to_generation3d(GenerationMutation::UpdateValues { id: "g1".into(), question_id: "q1".into(), value: serde_json::json!(1).into() }),
        Generation3dMutation::ChangeGenerationValue(ChangeGenerationValue { id: "g1".into(), question_id: "q1".into(), new_value: serde_json::json!(1).into() })
    );
}

#[test]
fn fixture_ops_ignore_camera() {
    let before = FlowFixture::default();
    let mut after = before.clone();
    after.camera = CameraJson { x: 7.0, y: 8.0, zoom: 2.0 };
    let operations = generation3d_fixture_operations(&before, &after);
    assert!(operations.iter().all(|operation| !matches!(operation, Generation3dMutation::UpdateCamera { .. })));
}

#[test]
fn generation3d_fixture_operations_detects_widget_synapse_layout_schema_changes() {
    let mut before = FlowFixture { schema: "old-schema".into(), ..Default::default() };
    before.widgets = vec![Widget::InputNote { id: "w-gone".into(), text: String::new() }, Widget::InputNote { id: "w-keep".into(), text: "old".into() }];
    before.synapses =
        vec![SynapseSpec { id: "s-gone".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() }, SynapseSpec { id: "s-keep".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "old".into() }];
    before.layout.insert("l-gone".into(), WidgetLayout { x: 0.0, y: 0.0 });
    before.layout.insert("l-keep".into(), WidgetLayout { x: 1.0, y: 1.0 });

    let mut after = FlowFixture { schema: "new-schema".into(), ..Default::default() };
    after.widgets = vec![Widget::InputNote { id: "w-keep".into(), text: "new".into() }, Widget::InputNote { id: "w-new".into(), text: String::new() }];
    after.synapses =
        vec![SynapseSpec { id: "s-keep".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "new".into() }, SynapseSpec { id: "s-new".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() }];
    after.layout.insert("l-keep".into(), WidgetLayout { x: 2.0, y: 2.0 });
    after.layout.insert("l-new".into(), WidgetLayout { x: 3.0, y: 3.0 });

    let operations = generation3d_fixture_operations(&before, &after);
    assert!(operations.contains(&Generation3dMutation::DeleteWidget(DeleteWidget { id: "w-gone".into() })));
    assert!(operations.contains(&Generation3dMutation::UpdateWidget(UpdateWidget { widget: Widget::InputNote { id: "w-keep".into(), text: "new".into() } })));
    assert!(operations.contains(&Generation3dMutation::CreateWidget(CreateWidget { index: 1, widget: Widget::InputNote { id: "w-new".into(), text: String::new() } })));
    assert!(operations.contains(&Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: "s-gone".into() })));
    assert!(operations.contains(&Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse: SynapseSpec { id: "s-keep".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "new".into() } })));
    assert!(operations.contains(&Generation3dMutation::ConnectSynapse(ConnectSynapse { index: 1, synapse: SynapseSpec { id: "s-new".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() } })));
    assert!(operations.contains(&Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: "l-gone".into() })));
    assert!(operations.contains(&Generation3dMutation::MoveWidget(MoveWidget { id: "l-keep".into(), layout: WidgetLayout { x: 2.0, y: 2.0 } })));
    assert!(operations.contains(&Generation3dMutation::MoveWidget(MoveWidget { id: "l-new".into(), layout: WidgetLayout { x: 3.0, y: 3.0 } })));
    assert!(operations.contains(&Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: "new-schema".into() })));
    before.retire_cold();
    after.retire_cold();
}

#[test]
fn update_widget_round_trip_replaces_existing_widget_by_id() {
    let mut before = default_generation3d_snapshot();
    before.fixture.widgets.clear();
    before.fixture.widgets.push(Widget::InputNote { id: "note-9".into(), text: "old".into() });
    let after = round_trip(&before, &Generation3dMutation::UpdateWidget(UpdateWidget { widget: Widget::InputNote { id: "note-9".into(), text: "new".into() } }));
    assert_eq!(after.fixture.widgets.len(), 1);
    assert_eq!(after.fixture.widgets[0], Widget::InputNote { id: "note-9".into(), text: "new".into() });
}

#[test]
fn inverse_delete_widget_when_missing_returns_empty() {
    let projection = default_generation3d_snapshot();
    assert!(Generation3dMutation::DeleteWidget(DeleteWidget { id: "ghost".into() }).inverse(&projection).is_empty());
}

#[test]
fn update_synapse_round_trip_replaces_existing_synapse_by_id() {
    let mut before = default_generation3d_snapshot();
    before.fixture.synapses.clear();
    before.fixture.synapses.push(SynapseSpec { id: "e1".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() });
    let after = round_trip(&before, &Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse: SynapseSpec { id: "e1".into(), from: "a".into(), to: "c".into(), from_port: "out".into(), to_port: "in".into() } }));
    assert_eq!(after.fixture.synapses.len(), 1);
    assert_eq!(after.fixture.synapses[0].to, "c");
}

#[test]
fn inverse_disconnect_synapse_when_missing_returns_empty() {
    let projection = default_generation3d_snapshot();
    assert!(Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: "ghost".into() }).inverse(&projection).is_empty());
}

/// 📍️ `move-widget` addresses a widget that must already exist — the position map is an override
/// on a live widget, never a free-standing entry — so the base has to carry `extrude` itself
/// (`📍️move-widget/🔺️diff/🦀️.rs`'s `mutation.target-missing` branch).
fn snapshot_with_extrude_widget() -> Generation3dSnapshot {
    let mut base = default_generation3d_snapshot();
    base.fixture.widgets.push(Widget::InputNote { id: "extrude".into(), text: String::new() });
    base
}

#[test]
fn move_widget_round_trip_inserts_when_absent() {
    let before = snapshot_with_extrude_widget();
    let after = round_trip(&before, &Generation3dMutation::MoveWidget(MoveWidget { id: "extrude".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } }));
    assert_eq!(after.fixture.layout.get("extrude"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
    before.retire_cold();
}

#[test]
fn move_widget_round_trip_replaces_when_present() {
    let mut before = snapshot_with_extrude_widget();
    before.fixture.layout.insert("extrude".into(), WidgetLayout { x: 1.0, y: 2.0 });
    let after = round_trip(&before, &Generation3dMutation::MoveWidget(MoveWidget { id: "extrude".into(), layout: WidgetLayout { x: 5.0, y: 6.0 } }));
    assert_eq!(after.fixture.layout.get("extrude"), Some(&WidgetLayout { x: 5.0, y: 6.0 }));
    before.retire_cold();
}

/// 🚫️ A `move-widget` whose target widget does not exist is REJECTED, and the projection is left
/// byte-identical — `apply_generation3d_mutation` must never return the unchanged base as implicit
/// success (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`'s `MutationDiff` contract).
#[test]
fn move_widget_on_a_missing_widget_is_rejected_and_leaves_the_projection_untouched() {
    let mut projection = crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new(default_generation3d_snapshot());
    let error = apply_generation3d_mutation(&mut projection, &Generation3dMutation::MoveWidget(MoveWidget { id: "ghost".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } })).expect_err("a missing move target must be rejected");
    assert_eq!(error.code, "mutation.target-missing");
    assert_eq!(*projection, default_generation3d_snapshot());
}

#[test]
fn delete_widget_position_inverse_present_restores_move_widget_missing_returns_empty() {
    let mut projection = default_generation3d_snapshot();
    projection.fixture.layout.insert("extrude".into(), WidgetLayout { x: 1.0, y: 2.0 });
    assert_eq!(Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: "extrude".into() }).inverse(&projection), vec![Generation3dMutation::MoveWidget(MoveWidget { id: "extrude".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } })]);
    assert!(Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: "ghost".into() }).inverse(&projection).is_empty());
    projection.retire_cold();
}

#[test]
fn update_camera_round_trip_updates_camera() {
    let before = default_generation3d_snapshot();
    let after = round_trip(&before, &Generation3dMutation::UpdateCamera(UpdateCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } }));
    assert_eq!(after.fixture.camera, CameraJson { x: 1.0, y: 2.0, zoom: 3.0 });
}

#[test]
fn change_schema_round_trip_updates_schema() {
    let before = default_generation3d_snapshot();
    let after = round_trip(&before, &Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: "flow.fixture.v2".into() }));
    assert_eq!(after.fixture.schema, "flow.fixture.v2");
}

//#region 🧪️MutationLaws
/// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️test/🦀️kit.rs`
/// (reachable here as `protocol::testkit`), exercised against the three most structurally
/// distinct new variants: an id-keyed create/delete pair (`create-widget`), a relationship
/// connect/disconnect pair (`connect-synapse`), and a document-level facet setter
/// (`update-camera`).
#[semio_framework_async_macros::async_test]
async fn create_widget_satisfies_the_inverse_and_absorb_laws() {
    let base = default_generation3d_snapshot();
    let mutation = Generation3dMutation::CreateWidget(CreateWidget { index: 0, widget: Widget::InputNote { id: "note-fresh".into(), text: String::new() } });
    semio_framework_os_kernel::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).into_parts().0;
    let d2 = Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: "flow.fixture.v2".into() }).diff(&base).into_parts().0;
    semio_framework_os_kernel::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn connect_synapse_satisfies_the_inverse_and_absorb_laws() {
    let mut base = default_generation3d_snapshot();
    base.fixture.widgets.push(Widget::InputNote { id: "a".into(), text: String::new() });
    base.fixture.widgets.push(Widget::InputNote { id: "b".into(), text: String::new() });
    let base = base;
    let mutation = Generation3dMutation::ConnectSynapse(ConnectSynapse { index: 0, synapse: SynapseSpec { id: "e-fresh".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() } });
    semio_framework_os_kernel::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).into_parts().0;
    let d2 = Generation3dMutation::UpdateCamera(UpdateCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } }).diff(&base).into_parts().0;
    semio_framework_os_kernel::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn update_camera_satisfies_the_inverse_and_absorb_laws() {
    let base = default_generation3d_snapshot();
    let mutation = Generation3dMutation::UpdateCamera(UpdateCamera { camera: CameraJson { x: 4.0, y: 5.0, zoom: 6.0 } });
    semio_framework_os_kernel::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).into_parts().0;
    let d2 = Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: "flow.fixture.v3".into() }).diff(&base).into_parts().0;
    semio_framework_os_kernel::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🧪️MutationLaws

//#region 🧪️KindsCatalog
/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed oracle
/// manifest's catalog — the framework never parses Rust, so this is the only thing that keeps the
/// declared vocabulary and the measured one from drifting apart.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <Generation3dMutation as protocol::SemanticMutation<Generation3dSnapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared Generation3dMutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog

