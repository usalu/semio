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
use crate::standards::v1::subsets::any::schema::mutations::{generation3d_document_replacement,generation3d_selection_removal,generation3d_widget_removal};

fn round_trip(projection: &Generation3dSnapshot, operation: &Generation3dMutation) -> crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead {
    let mut forward = crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new(projection.clone());
    apply_generation3d_mutation(&mut forward, operation).expect("valid mutation");
    let mut restored = crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new((*forward).clone());
    for back in operation.inverse(projection).expect("valid retained mutation inverse fixture") {
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
        Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: "flow.host_snapshot.v2".into() }),
        Generation3dMutation::CreateGeneration(CreateGeneration { generation: FormGeneration { id: "generation-fresh".into(), name: "Generation".into(), values: Default::default() }, index: None }),
        Generation3dMutation::DeleteGeneration(DeleteGeneration { id: "generation-1".into() }),
        Generation3dMutation::RenameGeneration(RenameGeneration { id: "generation-1".into(), new_name: "Renamed".into() }),
        Generation3dMutation::ChangeGenerationValue(ChangeGenerationValue { id: "generation-1".into(), question_id: "q1".into(), new_value: serde_json::json!(42).into() }),
        change_slider_value::change_slider_value("height", 7.5),
        drag_transforms::drag_transforms(vec!["extrude__gumball_translate".into()], [1.0, 0.0, 0.0]),
        rotate_transforms::rotate_transforms(vec!["extrude__gumball_rotate".into()], [0.0, 0.0, 1.0], 0.5),
        scale_transforms::scale_transforms(vec!["extrude__gumball_scale".into()], [2.0, 1.0, 1.0]),
        move_nodes::move_nodes(vec!["extrude".into()], 4.0, -2.0),
        change_widget_input::change_widget_input("extrude", "distance", change_widget_input::WidgetInputValue::Number(0.25)),
        Generation3dMutation::SelectGeneration(select_generation::SelectGeneration { generation_id: Some("generation-1".into()) }),
        Generation3dMutation::ChangeGenerationPreview(change_generation_preview::ChangeGenerationPreview { text: Some("preview".into()) }),
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
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![Generation3dMutation::CreateWidget(CreateWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } })], transaction: None }).await.expect("apply");
    assert!(store.snapshot().expect("snapshot").host_snapshot.widgets.iter().any(|w| widget_id(w) == "note-9"));
    crate::store_fixture::close(store);
}

#[test]
fn create_widget_round_trips() {
    let before = default_generation3d_snapshot();
    let after = round_trip(&before, &Generation3dMutation::CreateWidget(CreateWidget { index: 9, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }));
    assert!(after.host_snapshot.widgets.iter().any(|w| widget_id(w) == "note-9"));
}

#[test]
fn generation_op_round_trips() {
    let before = default_generation3d_snapshot();
    let generation = FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: Default::default() };
    let after = round_trip(&before, &Generation3dMutation::CreateGeneration(CreateGeneration { generation, index: None }));
    assert_eq!(after.generation.generations.len(), 1);
}

#[test]
fn generation_mutation_bridge_covers_every_variant() {
    let generation = FormGeneration { id: "g1".into(), name: "G1".into(), values: Default::default() };
    assert_eq!(generation_mutation_to_generation3d(GenerationMutation::Add { generation: generation.clone() }), Generation3dMutation::CreateGeneration(CreateGeneration { generation, index: None }));
    assert_eq!(generation_mutation_to_generation3d(GenerationMutation::Remove { id: "g1".into() }), Generation3dMutation::DeleteGeneration(DeleteGeneration { id: "g1".into() }));
    assert_eq!(generation_mutation_to_generation3d(GenerationMutation::Rename { id: "g1".into(), name: "New".into() }), Generation3dMutation::RenameGeneration(RenameGeneration { id: "g1".into(), new_name: "New".into() }));
    assert_eq!(
        generation_mutation_to_generation3d(GenerationMutation::UpdateValues { id: "g1".into(), question_id: "q1".into(), value: serde_json::json!(1).into() }),
        Generation3dMutation::ChangeGenerationValue(ChangeGenerationValue { id: "g1".into(), question_id: "q1".into(), new_value: serde_json::json!(1).into() })
    );
}

/// 🎬️ Replays `leaves` on a copy of `base`, the way the store does.
fn replay(base: &Generation3dSnapshot, leaves: &[Generation3dMutation]) -> crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead {
    let mut snapshot = crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new(base.clone());
    for leaf in leaves {
        apply_generation3d_mutation(&mut snapshot, leaf).unwrap_or_else(|refused| panic!("{leaf:?} refused: {refused:?}"));
    }
    snapshot
}

fn document(schema: &str, widgets: Vec<Widget>, synapses: Vec<SynapseSpec>, layout: Vec<(&str, f64)>, generations: &[&str], selected: Option<&str>, preview: Option<&str>) -> Generation3dSnapshot {
    let mut host_snapshot = FlowHostSnapshot { schema: schema.into(), camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, widgets, synapses, layout: Default::default() };
    for (id, x) in layout {
        host_snapshot.layout.insert(id.into(), WidgetLayout { x, y: x });
    }
    let state = semio_framework_artifact_playbook_playbook::GenerationPlayState {
        generations: generations.iter().map(|id| FormGeneration { id: (*id).into(), name: (*id).into(), values: Default::default() }).collect(),
        selected_generation_id: selected.map(str::to_string),
        preview_text: preview.map(str::to_string),
    };
    Generation3dSnapshot { host_snapshot, generation: state.into() }
}

fn wire(id: &str, from: &str, to: &str) -> SynapseSpec {
    SynapseSpec { id: id.into(), from: from.into(), to: to.into(), from_port: "out".into(), to_port: "in".into() }
}

fn note(id: &str, text: &str) -> Widget {
    Widget::InputNote { id: id.into(), text: text.into() }
}

#[test]
fn document_replacement_ignores_the_camera() {
    let before = default_generation3d_snapshot();
    let mut after = before.clone();
    after.host_snapshot.camera = CameraJson { x: 7.0, y: 8.0, zoom: 2.0 };
    let leaves = generation3d_document_replacement(&before, &after);
    assert!(leaves.is_empty(), "an equal document authors nothing, and the camera rides the config lane: {leaves:?}");
    before.retire_cold();
    after.retire_cold();
}

/// 🧱️ The plan names only what differs, in an order that replays: positions, wires and widgets that go come first (a position
/// leaf addresses a live widget), the schema and the entities that are new or changed follow, wires after both their ends exist.
#[test]
fn document_replacement_names_only_what_differs_in_replay_order() {
    let before = document("old-schema", vec![note("w-gone", ""), note("w-keep", "old"), note("w-end", "")], vec![wire("s-gone", "w-gone", "w-keep"), wire("s-keep", "w-keep", "w-end")], vec![("w-gone", 0.0), ("w-keep", 1.0)], &["g-a", "g-b"], Some("g-a"), Some("old preview"));
    let after = document("new-schema", vec![note("w-new", ""), note("w-keep", "new"), note("w-end", "")], vec![wire("s-keep", "w-keep", "w-end"), wire("s-new", "w-new", "w-keep")], vec![("w-keep", 2.0), ("w-new", 3.0)], &["g-b", "g-c"], Some("g-b"), None);
    let leaves = generation3d_document_replacement(&before, &after);
    let kinds: Vec<&str> = leaves.iter().map(|leaf| protocol::SemanticMutation::semantics(leaf).kind).collect();
    let count = |kind: &str| kinds.iter().filter(|candidate| **candidate == kind).count();
    let first = |kind: &str| kinds.iter().position(|candidate| *candidate == kind).unwrap_or_else(|| panic!("{kind} is authored"));
    let last = |kind: &str| kinds.iter().rposition(|candidate| *candidate == kind).unwrap_or_else(|| panic!("{kind} is authored"));
    assert!(last("delete-widget-position") < first("delete-widget"), "{kinds:?}");
    assert!(last("disconnect-synapse") < first("delete-widget"), "{kinds:?}");
    assert!(last("delete-widget") < first("create-widget"), "{kinds:?}");
    assert!(last("create-widget") < first("connect-synapse"), "{kinds:?}");
    assert!(last("create-widget") < first("move-widget"), "{kinds:?}");
    assert_eq!((count("delete-widget"), count("create-widget"), count("update-widget")), (1, 1, 1), "w-end is shared and equal, so it is not authored: {kinds:?}");
    assert_eq!((count("disconnect-synapse"), count("connect-synapse"), count("update-synapse")), (1, 1, 0), "s-keep is shared and equal, so it is not authored: {kinds:?}");
    assert!(kinds.contains(&"change-schema") && kinds.contains(&"change-generation-preview") && kinds.contains(&"select-generation"), "{kinds:?}");
    let replayed = replay(&before, &leaves);
    assert_eq!(replayed.host_snapshot.widgets, after.host_snapshot.widgets);
    assert_eq!(replayed.host_snapshot.synapses, after.host_snapshot.synapses);
    assert_eq!(replayed.host_snapshot.layout, after.host_snapshot.layout);
    assert_eq!(replayed.host_snapshot.schema, after.host_snapshot.schema);
    assert_eq!(replayed.generation, after.generation);
    for leaf in leaves {
        leaf.retire_cold();
    }
    before.retire_cold();
    after.retire_cold();
}

/// 🧱️ Re-loading the document in front of the user authors nothing; one edited text authors one `update-widget`.
#[test]
fn document_replacement_of_a_shared_document_authors_only_the_edit() {
    let base = document("s", vec![note("a", "one"), note("b", "")], vec![wire("ab", "a", "b")], vec![("a", 0.0)], &["g"], Some("g"), None);
    let same = base.clone();
    assert!(generation3d_document_replacement(&base, &same).is_empty());
    same.retire_cold();
    let edited = document("s", vec![note("a", "two"), note("b", "")], vec![wire("ab", "a", "b")], vec![("a", 0.0)], &["g"], Some("g"), None);
    assert_eq!(generation3d_document_replacement(&base, &edited), vec![Generation3dMutation::UpdateWidget(UpdateWidget { widget: note("a", "two") })]);
    base.retire_cold();
    edited.retire_cold();
}

/// 🧱️ A document that differs in its selection alone authors the selection alone.
#[test]
fn document_replacement_of_the_selection_alone_is_one_leaf() {
    let before = document("s", vec![note("w", "")], Vec::new(), Vec::new(), &["g-a", "g-b"], Some("g-a"), None);
    let after = document("s", vec![note("w", "")], Vec::new(), Vec::new(), &["g-a", "g-b"], Some("g-b"), None);
    assert_eq!(generation3d_document_replacement(&before, &after), vec![Generation3dMutation::SelectGeneration(select_generation::SelectGeneration { generation_id: Some("g-b".into()) })]);
    before.retire_cold();
    after.retire_cold();
}

/// 🧱️ Widgets and wires the two documents share in another relative order are rebuilt whole — the vocabulary has no reorder
/// verb, and the all-or-nothing rule replaces any choice of which survivors to keep.
#[test]
fn document_replacement_reaches_a_reordered_widget_and_wire_list() {
    let before = document("s", vec![note("a", ""), note("b", ""), note("c", "")], vec![wire("ab", "a", "b"), wire("bc", "b", "c")], Vec::new(), &[], None, None);
    let after = document("s", vec![note("c", ""), note("a", ""), note("b", "")], vec![wire("bc", "b", "c"), wire("ab", "a", "b")], Vec::new(), &[], None, None);
    let leaves = generation3d_document_replacement(&before, &after);
    let replayed = replay(&before, &leaves);
    assert_eq!(replayed.host_snapshot.widgets, after.host_snapshot.widgets);
    assert_eq!(replayed.host_snapshot.synapses, after.host_snapshot.synapses);
    for leaf in leaves {
        leaf.retire_cold();
    }
    before.retire_cold();
    after.retire_cold();
}

/// 🗑️ `delete-widget` does not cascade; the cascade is spelled by `generation3d_widget_removal` and is complete: no wire
/// naming the widget and no layout entry is left standing, and the leaves replay on their own.
#[test]
fn widget_removal_spells_its_cascade() {
    let before = document("s", vec![note("a", ""), note("b", ""), note("c", "")], vec![wire("ab", "a", "b"), wire("bc", "b", "c"), wire("ac", "a", "c")], vec![("a", 0.0), ("b", 1.0)], &[], None, None);
    let leaves = generation3d_widget_removal(&before.host_snapshot, "b");
    assert_eq!(leaves, vec![
        Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: "ab".into() }),
        Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: "bc".into() }),
        Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: "b".into() }),
        Generation3dMutation::DeleteWidget(DeleteWidget { id: "b".into() }),
    ]);
    let replayed = replay(&before, &leaves);
    assert_eq!(replayed.host_snapshot.synapses.iter().map(|synapse| synapse.id.as_str()).collect::<Vec<_>>(), ["ac"]);
    assert!(replayed.host_snapshot.layout.get("b").is_none() && replayed.host_snapshot.widgets.iter().all(|widget| widget_id(widget) != "b"));
    assert!(generation3d_widget_removal(&before.host_snapshot, "ghost").is_empty());
    before.retire_cold();
}

/// 🗑️ A selection of wires and widgets cuts each wire once, wires first.
#[test]
fn selection_removal_cuts_every_wire_once() {
    let before = document("s", vec![note("a", ""), note("b", "")], vec![wire("ab", "a", "b"), wire("ba", "b", "a")], vec![("a", 0.0)], &[], None, None);
    let leaves = generation3d_selection_removal(&before.host_snapshot, &["a".into(), "ab".into(), "ghost".into(), "a".into()]);
    assert_eq!(leaves, vec![
        Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: "ab".into() }),
        Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: "ba".into() }),
        Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: "a".into() }),
        Generation3dMutation::DeleteWidget(DeleteWidget { id: "a".into() }),
    ]);
    let replayed = replay(&before, &leaves);
    assert!(replayed.host_snapshot.synapses.is_empty());
    before.retire_cold();
}

#[test]
fn update_widget_round_trip_replaces_existing_widget_by_id() {
    let mut before = default_generation3d_snapshot();
    before.host_snapshot.widgets.clear();
    before.host_snapshot.widgets.push(Widget::InputNote { id: "note-9".into(), text: "old".into() });
    let after = round_trip(&before, &Generation3dMutation::UpdateWidget(UpdateWidget { widget: Widget::InputNote { id: "note-9".into(), text: "new".into() } }));
    assert_eq!(after.host_snapshot.widgets.len(), 1);
    assert_eq!(after.host_snapshot.widgets[0], Widget::InputNote { id: "note-9".into(), text: "new".into() });
}

#[test]
fn inverse_delete_widget_when_missing_returns_empty() {
    let projection = default_generation3d_snapshot();
    assert!(Generation3dMutation::DeleteWidget(DeleteWidget { id: "ghost".into() }).inverse(&projection).expect("valid retained mutation inverse fixture").is_empty());
}

#[test]
fn update_synapse_round_trip_replaces_existing_synapse_by_id() {
    let mut before = default_generation3d_snapshot();
    before.host_snapshot.synapses.clear();
    before.host_snapshot.synapses.push(SynapseSpec { id: "e1".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() });
    let after = round_trip(&before, &Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse: SynapseSpec { id: "e1".into(), from: "a".into(), to: "c".into(), from_port: "out".into(), to_port: "in".into() } }));
    assert_eq!(after.host_snapshot.synapses.len(), 1);
    assert_eq!(after.host_snapshot.synapses[0].to, "c");
}

#[test]
fn inverse_disconnect_synapse_when_missing_returns_empty() {
    let projection = default_generation3d_snapshot();
    assert!(Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: "ghost".into() }).inverse(&projection).expect("valid retained mutation inverse fixture").is_empty());
}

/// 📍️ `move-widget` addresses a widget that must already exist — the position map is an override
/// on a live widget, never a free-standing entry — so the base has to carry `extrude` itself
/// (`📍️move-widget/🔺️diff/🦀️.rs`'s `mutation.target-missing` branch).
fn snapshot_with_extrude_widget() -> Generation3dSnapshot {
    let mut base = default_generation3d_snapshot();
    base.host_snapshot.widgets.push(Widget::InputNote { id: "extrude".into(), text: String::new() });
    base
}

#[test]
fn move_widget_round_trip_inserts_when_absent() {
    let before = snapshot_with_extrude_widget();
    let after = round_trip(&before, &Generation3dMutation::MoveWidget(MoveWidget { id: "extrude".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } }));
    assert_eq!(after.host_snapshot.layout.get("extrude"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
    before.retire_cold();
}

#[test]
fn move_widget_round_trip_replaces_when_present() {
    let mut before = snapshot_with_extrude_widget();
    before.host_snapshot.layout.insert("extrude".into(), WidgetLayout { x: 1.0, y: 2.0 });
    let after = round_trip(&before, &Generation3dMutation::MoveWidget(MoveWidget { id: "extrude".into(), layout: WidgetLayout { x: 5.0, y: 6.0 } }));
    assert_eq!(after.host_snapshot.layout.get("extrude"), Some(&WidgetLayout { x: 5.0, y: 6.0 }));
    before.retire_cold();
}

/// 🚫️ A `move-widget` whose target widget does not exist is REJECTED, and the projection is left
/// byte-identical — `apply_generation3d_mutation` must never return the unchanged base as implicit
/// success (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`'s `MutationDiff` contract).
#[test]
fn move_widget_on_a_missing_widget_is_rejected_and_leaves_the_projection_untouched() {
    let mut projection = crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new(default_generation3d_snapshot());
    let mutation = Generation3dMutation::MoveWidget(MoveWidget { id: "ghost".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } });
    let refused = apply_generation3d_mutation(&mut projection, &mutation).expect_err("a missing move target must be rejected");
    assert_eq!(refused.iter().map(|message| (message.code.0.as_str(), message.level)).collect::<Vec<_>>(), [("mutation.target-missing", semio_framework_diagnostic::Severity::Error)]);
    assert_eq!(*projection, default_generation3d_snapshot());
}

/// ⚖️ Law: a checked apply refuses with the diff's own outcome messages — code, level, target and text unchanged, every
/// code at the level the frozen vocabulary fixes — and never re-types a vocabulary code as an apply error.
#[test]
fn checked_apply_propagates_the_vocabulary_outcome_unchanged() {
    let base = default_generation3d_snapshot();
    for mutation in [
        Generation3dMutation::MoveWidget(MoveWidget { id: "ghost".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } }),
        Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: "ghost".into() }),
    ] {
        let (diff, expected) = <Generation3dMutation as protocol::Mutation<Generation3dSnapshot>>::diff(&mutation, &base).into_parts();
        diff.retire_cold();
        let mut projection = crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new(base.clone());
        match apply_generation3d_mutation(&mut projection, &mutation) {
            Err(refused) => {
                assert_eq!(refused, expected, "{mutation:?}");
                assert!(refused.iter().all(|message| protocol::outcome_code_level(&message.code.0) == Some(message.level)), "{refused:?}");
                assert_eq!(*projection, base);
            }
            Ok(()) => assert!(expected.iter().all(|message| !matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)), "{mutation:?}"),
        }
    }
    base.retire_cold();
}

#[test]
fn delete_widget_position_inverse_present_restores_move_widget_missing_returns_empty() {
    let mut projection = default_generation3d_snapshot();
    projection.host_snapshot.layout.insert("extrude".into(), WidgetLayout { x: 1.0, y: 2.0 });
    assert_eq!(Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: "extrude".into() }).inverse(&projection).expect("valid retained mutation inverse fixture"), vec![Generation3dMutation::MoveWidget(MoveWidget { id: "extrude".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } })]);
    assert!(Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: "ghost".into() }).inverse(&projection).expect("valid retained mutation inverse fixture").is_empty());
    projection.retire_cold();
}

#[test]
fn update_camera_round_trip_updates_camera() {
    let before = default_generation3d_snapshot();
    let after = round_trip(&before, &Generation3dMutation::UpdateCamera(UpdateCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } }));
    assert_eq!(after.host_snapshot.camera, CameraJson { x: 1.0, y: 2.0, zoom: 3.0 });
}

#[test]
fn change_schema_round_trip_updates_schema() {
    let before = default_generation3d_snapshot();
    let after = round_trip(&before, &Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: "flow.host_snapshot.v2".into() }));
    assert_eq!(after.host_snapshot.schema, "flow.host_snapshot.v2");
}

//#region 🧪️MutationLaws
/// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️test/🦀️kit.rs`
/// (reachable here as `protocol::os_spr::protocol_laws`), exercised against the three most structurally
/// distinct new variants: an id-keyed create/delete pair (`create-widget`), a relationship
/// connect/disconnect pair (`connect-synapse`), and a document-level facet setter
/// (`update-camera`).
#[semio_framework_async_macros::async_test]
async fn create_widget_satisfies_the_inverse_and_absorb_laws() {
    let base = default_generation3d_snapshot();
    let mutation = Generation3dMutation::CreateWidget(CreateWidget { index: 0, widget: Widget::InputNote { id: "note-fresh".into(), text: String::new() } });
    semio_framework_os_kernel::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).into_parts().0;
    let d2 = Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: "flow.host_snapshot.v2".into() }).diff(&base).into_parts().0;
    semio_framework_os_kernel::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn connect_synapse_satisfies_the_inverse_and_absorb_laws() {
    let mut base = default_generation3d_snapshot();
    base.host_snapshot.widgets.push(Widget::InputNote { id: "a".into(), text: String::new() });
    base.host_snapshot.widgets.push(Widget::InputNote { id: "b".into(), text: String::new() });
    let base = base;
    let mutation = Generation3dMutation::ConnectSynapse(ConnectSynapse { index: 0, synapse: SynapseSpec { id: "e-fresh".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() } });
    semio_framework_os_kernel::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).into_parts().0;
    let d2 = Generation3dMutation::UpdateCamera(UpdateCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } }).diff(&base).into_parts().0;
    semio_framework_os_kernel::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn update_camera_satisfies_the_inverse_and_absorb_laws() {
    let base = default_generation3d_snapshot();
    let mutation = Generation3dMutation::UpdateCamera(UpdateCamera { camera: CameraJson { x: 4.0, y: 5.0, zoom: 6.0 } });
    semio_framework_os_kernel::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).into_parts().0;
    let d2 = Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: "flow.host_snapshot.v3".into() }).diff(&base).into_parts().0;
    semio_framework_os_kernel::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
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
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog

