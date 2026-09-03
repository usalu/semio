//! ⚡️ Generation3d artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `Generation3dSnapshot`'s shape per `📓️derivation-rules.md`: an id-keyed widget
//! collection (`create`/`update`/`delete-widget`), a relationship/edge collection of synapses
//! (`connect`/`update`/`disconnect-synapse`), a per-widget position map (`move-widget` /
//! `delete-widget-position`), two document-level scalars (`update-camera`, `change-schema`), and an
//! id-keyed generation collection bridged from `flow::playbook::GenerationMutation`
//! (`create`/`delete`/`rename-generation`, `change-generation-value`). Every variant wraps exactly
//! one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<Generation3dSnapshot, Generation3dMutation>`; `#[derive(dsl::Mutations)]`
//! below generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to each
//! payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` for the reference shape.
//!
//! `SetWidget`/`RemoveWidget`/`SetSynapse`/`RemoveSynapse`/`SetLayout`/`RemoveLayout`/`SetCamera`/
//! `SetSchema`/`Generation(GenerationMutation)` — the pre-migration generic vocabulary — are gone.
//! Eight triad-leaf directories keep their pre-migration `➖remove-*`/`🎛set-*` names: glue.rs
//! path-includes those exact files and this facet's writable boundary excludes glue.rs, so the
//! directories couldn't be renamed alongside their content — see the migration report's
//! `sharedFileRequests` for the exact rename once a later pass can touch glue.rs.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::generation3d::diff::Generation3dDiff;
use crate::artifacts::generation3d::{widget_id, Generation3dSnapshot};
use flow::playbook::GenerationMutation;
use flow::FlowFixture;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔖️AddressHelpers
/// 🔎️ BASE-state widget index lookup by id — shared by every widget triad leaf's inverse.
pub(crate) fn widget_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.widgets.iter().position(|widget| widget_id(widget) == id)
}

/// 🔎️ BASE-state synapse index lookup by id — shared by every synapse triad leaf's inverse.
pub(crate) fn synapse_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.synapses.iter().position(|synapse| synapse.id == id)
}
//#endregion 🔖️AddressHelpers

//#region 🔖️NewLeaves
// 🌱️ Triad leaves that needed a fresh directory (no pre-migration slot to repurpose) — self-wired
// here since glue.rs is outside this facet's writable boundary; the eight leaves already carrying a
// semantic name (`delete_widget_position`/`disconnect_synapse`/`delete_widget`/`update_camera`/
// `move_widget`/`change_schema`/`update_synapse`/`update_widget`) stay wired by glue.rs's existing
// sibling `pub mod` blocks, unchanged — imported by those names just below.
#[path = "."]
pub mod create_widget {
    #[path = "🌱create-widget/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌱create-widget/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🌱create-widget/🦀️.rs"]
    mod component;
    pub use component::*;
    #[cfg(test)]
    #[path = "🌱create-widget/🧪️tests/inserts-node-c-at-index-2/🦀️.rs"]
    mod tests_inserts_node_c_at_index_2;
}

#[path = "."]
pub mod connect_synapse {
    #[path = "🔗connect-synapse/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔗connect-synapse/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🔗connect-synapse/🦀️.rs"]
    mod component;
    pub use component::*;
    #[cfg(test)]
    #[path = "🔗connect-synapse/🧪️tests/wires-node-b-to-node-c-at-index-1/🦀️.rs"]
    mod tests_wires_node_b_to_node_c_at_index_1;
}

#[path = "."]
pub mod create_generation {
    #[path = "➕create-generation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "➕create-generation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "➕create-generation/🦀️.rs"]
    mod component;
    pub use component::*;
    #[cfg(test)]
    #[path = "➕create-generation/🧪️tests/appends-generation-2-and-moves-the-selection/🦀️.rs"]
    mod tests_appends_generation_2_and_moves_the_selection;
}

#[path = "."]
pub mod delete_generation {
    #[path = "🗑delete-generation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🗑delete-generation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🗑delete-generation/🦀️.rs"]
    mod component;
    pub use component::*;
    #[cfg(test)]
    #[path = "🗑delete-generation/🧪️tests/removes-the-selected-generation-2-and-falls-back/🦀️.rs"]
    mod tests_removes_the_selected_generation_2_and_falls_back;
}

#[path = "."]
pub mod rename_generation {
    #[path = "🏷rename-generation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏷rename-generation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🏷rename-generation/🦀️.rs"]
    mod component;
    pub use component::*;
    #[cfg(test)]
    #[path = "🏷rename-generation/🧪️tests/retitles-generation-1-via-new-name/🦀️.rs"]
    mod tests_retitles_generation_1_via_new_name;
}

#[path = "."]
pub mod change_generation_value {
    #[path = "🔧change-generation-value/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔧change-generation-value/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🔧change-generation-value/🦀️.rs"]
    mod component;
    pub use component::*;
    #[cfg(test)]
    #[path = "🔧change-generation-value/🧪️tests/raises-the-storeys-answer-in-generation-1/🦀️.rs"]
    mod tests_raises_the_storeys_answer_in_generation_1;
}
//#endregion 🔖️NewLeaves

//#region 🔖️RepurposedLeaves
// 🌱️ Triad leaves that repurpose a pre-migration `➖remove-*`/`🎛set-*` directory glue.rs already
// path-includes as a sibling of `component` (this file) under `pub mod mutations { ... }` — brought
// into this file's own scope the same way `cad`'s already-migrated `🧬️mutations/🦀️.rs`
// reaches its own siblings (`use super::create_object;` etc.): `pub use component::*` only lifts
// `component`'s items UP into `mutations`, it doesn't inject `mutations`'s OTHER children back down.
use super::change_schema;
use super::delete_widget;
use super::delete_widget_position;
use super::disconnect_synapse;
use super::move_widget;
use super::update_camera;
use super::update_synapse;
use super::update_widget;
//#endregion 🔖️RepurposedLeaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the generation3d document, derived per
/// `📓️derivation-rules.md` from `Generation3dSnapshot`'s shape.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Generation3dSnapshot, diff = Generation3dDiff, schema = "generation.3d")]
pub enum Generation3dMutation {
    CreateWidget(create_widget::CreateWidget),
    UpdateWidget(update_widget::UpdateWidget),
    DeleteWidget(delete_widget::DeleteWidget),
    ConnectSynapse(connect_synapse::ConnectSynapse),
    UpdateSynapse(update_synapse::UpdateSynapse),
    DisconnectSynapse(disconnect_synapse::DisconnectSynapse),
    MoveWidget(move_widget::MoveWidget),
    DeleteWidgetPosition(delete_widget_position::DeleteWidgetPosition),
    UpdateCamera(update_camera::UpdateCamera),
    ChangeSchema(change_schema::ChangeSchema),
    CreateGeneration(create_generation::CreateGeneration),
    DeleteGeneration(delete_generation::DeleteGeneration),
    RenameGeneration(rename_generation::RenameGeneration),
    ChangeGenerationValue(change_generation_value::ChangeGenerationValue),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Generation3dMutation`] variant, in declaration order — the exact
/// vocabulary the `procedural-3d-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `mutate-procedural-3d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "create-widget",
    "update-widget",
    "delete-widget",
    "connect-synapse",
    "update-synapse",
    "disconnect-synapse",
    "move-widget",
    "delete-widget-position",
    "update-camera",
    "change-schema",
    "create-generation",
    "delete-generation",
    "rename-generation",
    "change-generation-value",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

//#region 🔖️GenerationBridge
/// 🌉️ Bridges one `flow::playbook::GenerationMutation` (the framework's own generation-editing
/// vocabulary — `Add`/`Remove`/`Rename`/`UpdateValues`) onto this facet's semantic
/// `Generation3dMutation` variants, so app-layer callers that already hold a `GenerationMutation`
/// (from `flow::playbook::generation_operations`) need only swap the mapping function at the call
/// site, not learn this facet's internal triad-leaf module paths.
pub fn generation_mutation_to_generation3d(operation: GenerationMutation) -> Generation3dMutation {
    match operation {
        GenerationMutation::Add { generation } => Generation3dMutation::CreateGeneration(create_generation::CreateGeneration { generation }),
        GenerationMutation::Remove { id } => Generation3dMutation::DeleteGeneration(delete_generation::DeleteGeneration { id }),
        GenerationMutation::Rename { id, name } => Generation3dMutation::RenameGeneration(rename_generation::RenameGeneration { id, new_name: name }),
        GenerationMutation::UpdateValues { id, question_id, value } => Generation3dMutation::ChangeGenerationValue(change_generation_value::ChangeGenerationValue { id, question_id, new_value: value }),
    }
}
//#endregion 🔖️GenerationBridge

//#region 🔖️FixtureDiffing
/// 🔀️ Diffs two fixtures into a minimal, invertible, mergeable semantic mutation set — signature
/// preserved from the pre-migration generic-vocabulary version (`🏗️builder`/app callers reach this
/// via `crate::artifacts::generation3d::schema::commit_fixture`, unchanged) but every pushed
/// mutation is now a real semantic variant.
pub fn generation3d_fixture_operations(before: &FlowFixture, after: &FlowFixture) -> Vec<Generation3dMutation> {
    let mut operations = Vec::new();
    for widget in &before.widgets {
        if !after.widgets.iter().any(|entry| widget_id(entry) == widget_id(widget)) {
            operations.push(Generation3dMutation::DeleteWidget(delete_widget::DeleteWidget { id: widget_id(widget).to_string() }));
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        let prior = before.widgets.iter().find(|entry| widget_id(entry) == widget_id(widget));
        match prior {
            Some(previous) if previous != widget => operations.push(Generation3dMutation::UpdateWidget(update_widget::UpdateWidget { widget: widget.clone() })),
            None => operations.push(Generation3dMutation::CreateWidget(create_widget::CreateWidget { index, widget: widget.clone() })),
            _ => {}
        }
    }
    for synapse in &before.synapses {
        if !after.synapses.iter().any(|entry| entry.id == synapse.id) {
            operations.push(Generation3dMutation::DisconnectSynapse(disconnect_synapse::DisconnectSynapse { id: synapse.id.clone() }));
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        let prior = before.synapses.iter().find(|entry| entry.id == synapse.id);
        match prior {
            Some(previous) if previous != synapse => operations.push(Generation3dMutation::UpdateSynapse(update_synapse::UpdateSynapse { synapse: synapse.clone() })),
            None => operations.push(Generation3dMutation::ConnectSynapse(connect_synapse::ConnectSynapse { index, synapse: synapse.clone() })),
            _ => {}
        }
    }
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            operations.push(Generation3dMutation::DeleteWidgetPosition(delete_widget_position::DeleteWidgetPosition { id: id.clone() }));
        }
    }
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            operations.push(Generation3dMutation::MoveWidget(move_widget::MoveWidget { id: id.clone(), layout: layout.clone() }));
        }
    }
    if before.schema != after.schema {
        operations.push(Generation3dMutation::ChangeSchema(change_schema::ChangeSchema { new_schema: after.schema.clone() }));
    }
    operations
}
//#endregion 🔖️FixtureDiffing

pub type Generation3dEnvelope = ArtifactEnvelope<Generation3dSnapshot, Generation3dMutation>;
pub type Generation3dStore = ArtifactStore<Generation3dSnapshot, Generation3dMutation>;

//#region 🔖️Apply
/// 🎬️ Fallible in-place `vcs::apply_mutation` boundary.
pub fn apply_generation3d_mutation(projection: &mut Generation3dSnapshot, mutation: &Generation3dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;

    *projection = next;
    Ok(())
}

pub fn inverse_generation3d_mutation(projection: &Generation3dSnapshot, mutation: &Generation3dMutation) -> Vec<Generation3dMutation> {
    protocol::Mutation::inverse(mutation, projection)
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::generation3d::schema::empty_generation3d_snapshot;
    use change_generation_value::ChangeGenerationValue;
    use change_schema::ChangeSchema;
    use connect_synapse::ConnectSynapse;
    use create_generation::CreateGeneration;
    use create_widget::CreateWidget;
    use delete_generation::DeleteGeneration;
    use delete_widget::DeleteWidget;
    use delete_widget_position::DeleteWidgetPosition;
    use disconnect_synapse::DisconnectSynapse;
    use flow::playbook::FormGeneration;
    use flow::{CameraJson, SynapseSpec, Widget, WidgetLayout};
    use move_widget::MoveWidget;
    use protocol::Mutation;
    use protocol::SemanticMutation;
    use rename_generation::RenameGeneration;
    use update_camera::UpdateCamera;
    use update_synapse::UpdateSynapse;
    use update_widget::UpdateWidget;

    fn round_trip(projection: &Generation3dSnapshot, operation: &Generation3dMutation) -> Generation3dSnapshot {
        let forward = vcs::apply_mutation(projection, operation).expect("valid mutation").0;
        let mut restored = forward.clone();
        for back in operation.inverse(projection) {
            restored = vcs::apply_mutation(&restored, &back).expect("valid inverse mutation").0;
        }
        assert_eq!(&restored, projection, "inverse(base) must restore the pre-operation document");
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
            Generation3dMutation::CreateGeneration(CreateGeneration { generation: FormGeneration { id: "generation-fresh".into(), name: "Generation".into(), values: serde_json::Map::new() } }),
            Generation3dMutation::DeleteGeneration(DeleteGeneration { id: "generation-1".into() }),
            Generation3dMutation::RenameGeneration(RenameGeneration { id: "generation-1".into(), new_name: "Renamed".into() }),
            Generation3dMutation::ChangeGenerationValue(ChangeGenerationValue { id: "generation-1".into(), question_id: "q1".into(), new_value: serde_json::json!(42) }),
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

    #[test]
    fn store_applies_widget_create() {
        let mut store = ArtifactStore::<Generation3dSnapshot, Generation3dMutation>::new(store::create_document_envelope(crate::artifacts::generation3d::GENERATION_3D_SCHEMA, "generation3d", empty_generation3d_snapshot(), None))
            .expect("valid artifact store fixture");
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![Generation3dMutation::CreateWidget(CreateWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } })], description: None }).expect("apply");
        assert!(store.snapshot().expect("snapshot").fixture.widgets.iter().any(|w| widget_id(w) == "note-9"));
    }

    #[test]
    fn create_widget_round_trips() {
        let before = empty_generation3d_snapshot();
        let after = round_trip(&before, &Generation3dMutation::CreateWidget(CreateWidget { index: 9, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }));
        assert!(after.fixture.widgets.iter().any(|w| widget_id(w) == "note-9"));
    }

    #[test]
    fn generation_op_round_trips() {
        let before = empty_generation3d_snapshot();
        let generation = FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        let after = round_trip(&before, &Generation3dMutation::CreateGeneration(CreateGeneration { generation }));
        assert_eq!(after.generation.generations.len(), 1);
    }

    #[test]
    fn generation_mutation_bridge_covers_every_variant() {
        let generation = FormGeneration { id: "g1".into(), name: "G1".into(), values: serde_json::Map::new() };
        assert_eq!(generation_mutation_to_generation3d(GenerationMutation::Add { generation: generation.clone() }), Generation3dMutation::CreateGeneration(CreateGeneration { generation }));
        assert_eq!(generation_mutation_to_generation3d(GenerationMutation::Remove { id: "g1".into() }), Generation3dMutation::DeleteGeneration(DeleteGeneration { id: "g1".into() }));
        assert_eq!(generation_mutation_to_generation3d(GenerationMutation::Rename { id: "g1".into(), name: "New".into() }), Generation3dMutation::RenameGeneration(RenameGeneration { id: "g1".into(), new_name: "New".into() }));
        assert_eq!(
            generation_mutation_to_generation3d(GenerationMutation::UpdateValues { id: "g1".into(), question_id: "q1".into(), value: serde_json::json!(1) }),
            Generation3dMutation::ChangeGenerationValue(ChangeGenerationValue { id: "g1".into(), question_id: "q1".into(), new_value: serde_json::json!(1) })
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
        before.synapses = vec![
            SynapseSpec { id: "s-gone".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() },
            SynapseSpec { id: "s-keep".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "old".into() },
        ];
        before.layout.insert("l-gone".into(), WidgetLayout { x: 0.0, y: 0.0 });
        before.layout.insert("l-keep".into(), WidgetLayout { x: 1.0, y: 1.0 });

        let mut after = FlowFixture { schema: "new-schema".into(), ..Default::default() };
        after.widgets = vec![Widget::InputNote { id: "w-keep".into(), text: "new".into() }, Widget::InputNote { id: "w-new".into(), text: String::new() }];
        after.synapses = vec![
            SynapseSpec { id: "s-keep".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "new".into() },
            SynapseSpec { id: "s-new".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() },
        ];
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
    }

    #[test]
    fn update_widget_round_trip_replaces_existing_widget_by_id() {
        let mut before = empty_generation3d_snapshot();
        before.fixture.widgets.clear();
        before.fixture.widgets.push(Widget::InputNote { id: "note-9".into(), text: "old".into() });
        let after = round_trip(&before, &Generation3dMutation::UpdateWidget(UpdateWidget { widget: Widget::InputNote { id: "note-9".into(), text: "new".into() } }));
        assert_eq!(after.fixture.widgets.len(), 1);
        assert_eq!(after.fixture.widgets[0], Widget::InputNote { id: "note-9".into(), text: "new".into() });
    }

    #[test]
    fn inverse_delete_widget_when_missing_returns_empty() {
        let projection = empty_generation3d_snapshot();
        assert!(Generation3dMutation::DeleteWidget(DeleteWidget { id: "ghost".into() }).inverse(&projection).is_empty());
    }

    #[test]
    fn update_synapse_round_trip_replaces_existing_synapse_by_id() {
        let mut before = empty_generation3d_snapshot();
        before.fixture.synapses.clear();
        before.fixture.synapses.push(SynapseSpec { id: "e1".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() });
        let after = round_trip(&before, &Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse: SynapseSpec { id: "e1".into(), from: "a".into(), to: "c".into(), from_port: "out".into(), to_port: "in".into() } }));
        assert_eq!(after.fixture.synapses.len(), 1);
        assert_eq!(after.fixture.synapses[0].to, "c");
    }

    #[test]
    fn inverse_disconnect_synapse_when_missing_returns_empty() {
        let projection = empty_generation3d_snapshot();
        assert!(Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: "ghost".into() }).inverse(&projection).is_empty());
    }

    #[test]
    fn move_widget_round_trip_inserts_when_absent() {
        let before = empty_generation3d_snapshot();
        let after = round_trip(&before, &Generation3dMutation::MoveWidget(MoveWidget { id: "extrude".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } }));
        assert_eq!(after.fixture.layout.get("extrude"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
    }

    #[test]
    fn move_widget_round_trip_replaces_when_present() {
        let mut before = empty_generation3d_snapshot();
        before.fixture.layout.insert("extrude".into(), WidgetLayout { x: 1.0, y: 2.0 });
        let after = round_trip(&before, &Generation3dMutation::MoveWidget(MoveWidget { id: "extrude".into(), layout: WidgetLayout { x: 5.0, y: 6.0 } }));
        assert_eq!(after.fixture.layout.get("extrude"), Some(&WidgetLayout { x: 5.0, y: 6.0 }));
    }

    #[test]
    fn delete_widget_position_inverse_present_restores_move_widget_missing_returns_empty() {
        let mut projection = empty_generation3d_snapshot();
        projection.fixture.layout.insert("extrude".into(), WidgetLayout { x: 1.0, y: 2.0 });
        assert_eq!(Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: "extrude".into() }).inverse(&projection), vec![Generation3dMutation::MoveWidget(MoveWidget { id: "extrude".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } })]);
        assert!(Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: "ghost".into() }).inverse(&projection).is_empty());
    }

    #[test]
    fn update_camera_round_trip_updates_camera() {
        let before = empty_generation3d_snapshot();
        let after = round_trip(&before, &Generation3dMutation::UpdateCamera(UpdateCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } }));
        assert_eq!(after.fixture.camera, CameraJson { x: 1.0, y: 2.0, zoom: 3.0 });
    }

    #[test]
    fn change_schema_round_trip_updates_schema() {
        let before = empty_generation3d_snapshot();
        let after = round_trip(&before, &Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: "flow.fixture.v2".into() }));
        assert_eq!(after.fixture.schema, "flow.fixture.v2");
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️test/🦀️kit.rs`
    /// (reachable here as `protocol::testkit`), exercised against the three most structurally
    /// distinct new variants: an id-keyed create/delete pair (`create-widget`), a relationship
    /// connect/disconnect pair (`connect-synapse`), and a document-level facet setter
    /// (`update-camera`).
    #[test]
    fn create_widget_satisfies_the_inverse_and_absorb_laws() {
        let base = empty_generation3d_snapshot();
        let mutation = Generation3dMutation::CreateWidget(CreateWidget { index: 0, widget: Widget::InputNote { id: "note-fresh".into(), text: String::new() } });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: "flow.fixture.v2".into() }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn connect_synapse_satisfies_the_inverse_and_absorb_laws() {
        let base = empty_generation3d_snapshot();
        let mutation = Generation3dMutation::ConnectSynapse(ConnectSynapse { index: 0, synapse: SynapseSpec { id: "e-fresh".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() } });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = Generation3dMutation::UpdateCamera(UpdateCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn update_camera_satisfies_the_inverse_and_absorb_laws() {
        let base = empty_generation3d_snapshot();
        let mutation = Generation3dMutation::UpdateCamera(UpdateCamera { camera: CameraJson { x: 4.0, y: 5.0, zoom: 6.0 } });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: "flow.fixture.v3".into() }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
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
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
    //#endregion 🧪️KindsCatalog
}
//#endregion 🧪️Tests
