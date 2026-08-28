//! 🧬️ Procedural2d artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload: eight live in the
//! `🧬️mutations/<slug>/` triad leaves wired by `📦️glue.rs` (their directory/module names are
//! leftovers of the generic slots they were repurposed from — see this ticket's wave2 report for
//! the glue.rs rename that would align them), the rest — those with no pre-wired slot — live inline
//! below as `mod <slug> { 🦠️mutation / 🔺️diff / ↩️inverse }` regions, same shape, same file.
//! `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<Procedural2dSnapshot>` and
//! `impl protocol::SemanticMutation<Procedural2dSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::procedural2d::diff::Procedural2dDiff;
use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};
use flow::playbook::GenerationMutation;
use flow::FlowFixture;
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::{ArtifactEnvelope, ArtifactStore};
/// 🧵 Sibling triad-leaf modules wired by `📦️glue.rs` under eight pre-existing (pre-semantic)
/// directory slots — their directory/module names are leftovers of the generic slots each was
/// repurposed from (`sharedFileRequests` in this ticket's wave2 report has the glue.rs rename that
/// would align them; not editable here — glue.rs is shared with the sibling `procedural3d` artifact).

//#region 🔖️Addressing
/// 🌡️ Resolves a widget's stable id to its BASE-state index in the fixture's widget list.
pub fn widget_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.widgets.iter().position(|widget| widget_id(widget) == id)
}

/// 🌡️ Resolves a synapse's stable id to its BASE-state index in the fixture's synapse list.
pub fn synapse_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.synapses.iter().position(|synapse| synapse.id == id)
}
//#endregion 🔖️Addressing

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[mutations(snapshot = Procedural2dSnapshot, diff = Procedural2dDiff, schema = "procedural.2d")]
pub enum Procedural2dMutation {
    CreateWidget(super::create_widget::mutation::CreateWidget),
    ReplaceWidget(super::replace_widget::mutation::ReplaceWidget),
    DeleteWidget(super::delete_widget::mutation::DeleteWidget),
    ConnectSynapse(super::connect_synapse::mutation::ConnectSynapse),
    ReplaceSynapse(super::replace_synapse::mutation::ReplaceSynapse),
    DisconnectSynapse(super::disconnect_synapse::mutation::DisconnectSynapse),
    MoveWidget(super::move_widget::mutation::MoveWidget),
    ClearWidgetLayout(super::clear_widget_layout::mutation::ClearWidgetLayout),
    UpdateCamera(super::set_camera::mutation::UpdateCamera),
    ChangeSchema(super::change_schema::mutation::ChangeSchema),
    CreateGeneration(super::create_generation::mutation::CreateGeneration),
    DeleteGeneration(super::delete_generation::mutation::DeleteGeneration),
    RenameGeneration(super::rename_generation::mutation::RenameGeneration),
    ChangeGenerationValue(super::change_generation_value::mutation::ChangeGenerationValue),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Procedural2dMutation`] variant, in declaration order — the exact
/// vocabulary the `procedural-2d-1-any` mutation catalog (`../../🧪️oracle/🔣️.json`) declares and
/// the `mutate-procedural-2d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "create-widget",
    "replace-widget",
    "delete-widget",
    "connect-synapse",
    "replace-synapse",
    "disconnect-synapse",
    "move-widget",
    "clear-widget-layout",
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
/// `Procedural2dMutation` variants, so app-layer callers that already hold a `GenerationMutation`
/// (from `flow::playbook::generation_operations`) need only swap the mapping function at the call
/// site, not learn this facet's internal triad-leaf module paths. Twin of procedural3d's
/// `generation_mutation_to_procedural3d` — the two facets' generation payloads differ only in field
/// naming (`name`/`value` here, `new_name`/`new_value` there).
pub fn generation_mutation_to_procedural2d(operation: GenerationMutation) -> Procedural2dMutation {
    match operation {
        GenerationMutation::Add { generation } => Procedural2dMutation::CreateGeneration(super::create_generation::mutation::CreateGeneration { generation }),
        GenerationMutation::Remove { id } => Procedural2dMutation::DeleteGeneration(super::delete_generation::mutation::DeleteGeneration { id }),
        GenerationMutation::Rename { id, name } => Procedural2dMutation::RenameGeneration(super::rename_generation::mutation::RenameGeneration { id, name }),
        GenerationMutation::UpdateValues { id, question_id, value } => Procedural2dMutation::ChangeGenerationValue(super::change_generation_value::mutation::ChangeGenerationValue { id, question_id, value }),
    }
}
//#endregion 🔖️GenerationBridge

//#region 🔖️Builders
pub use super::change_generation_value::mutation::change_generation_value;
pub use super::change_schema::mutation::change_schema;
pub use super::clear_widget_layout::mutation::clear_widget_layout;
pub use super::connect_synapse::mutation::connect_synapse;
pub use super::create_generation::mutation::create_generation;
pub use super::create_widget::mutation::create_widget;
pub use super::delete_generation::mutation::delete_generation;
pub use super::delete_widget::mutation::delete_widget;
pub use super::disconnect_synapse::mutation::disconnect_synapse;
pub use super::move_widget::mutation::move_widget;
pub use super::rename_generation::mutation::rename_generation;
pub use super::replace_synapse::mutation::replace_synapse;
pub use super::replace_widget::mutation::replace_widget;
pub use super::set_camera::mutation::update_camera;
//#endregion 🔖️Builders

//#region 🔖️FixtureOperations
/// 🔀️ Diffs two fixtures into a minimal, invertible, mergeable semantic operation set:
/// created/replaced/deleted widgets and synapses (keyed by id), moved/cleared layout entries, and
/// a changed fixture schema. The canvas camera is ephemeral view state (app config), never a
/// document operation.
pub fn procedural2d_fixture_operations(before: &FlowFixture, after: &FlowFixture) -> Vec<Procedural2dMutation> {
    let mut operations = Vec::new();
    for widget in &before.widgets {
        if !after.widgets.iter().any(|entry| widget_id(entry) == widget_id(widget)) {
            operations.push(delete_widget(widget_id(widget).to_string()));
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        match before.widgets.iter().find(|entry| widget_id(entry) == widget_id(widget)) {
            None => operations.push(create_widget(index, widget.clone())),
            Some(prior) if prior != widget => operations.push(replace_widget(widget.clone())),
            _ => {}
        }
    }
    for synapse in &before.synapses {
        if !after.synapses.iter().any(|entry| entry.id == synapse.id) {
            operations.push(disconnect_synapse(synapse.id.clone()));
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        match before.synapses.iter().find(|entry| entry.id == synapse.id) {
            None => operations.push(connect_synapse(index, synapse.clone())),
            Some(prior) if prior != synapse => operations.push(replace_synapse(synapse.clone())),
            _ => {}
        }
    }
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            operations.push(clear_widget_layout(id.clone()));
        }
    }
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            operations.push(move_widget(id.clone(), layout.clone()));
        }
    }
    if before.schema != after.schema {
        operations.push(change_schema(after.schema.clone()));
    }
    operations
}
//#endregion 🔖️FixtureOperations

pub type Procedural2dEnvelope = ArtifactEnvelope<Procedural2dSnapshot, Procedural2dMutation>;
pub type Procedural2dStore = ArtifactStore<Procedural2dSnapshot, Procedural2dMutation>;

/// 🧬️ Applies a mutation to a projection — generic over every variant, so it never needs edits
/// when the semantic vocabulary grows.
pub fn apply_procedural2d_mutation(projection: &mut Procedural2dSnapshot, mutation: &Procedural2dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;

    *projection = next;
    Ok(())
}

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_procedural2d_mutation(projection: &Procedural2dSnapshot, mutation: &Procedural2dMutation) -> Vec<Procedural2dMutation> {
    mutation.inverse(projection)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural2d::schema::empty_procedural2d_snapshot;
    use flow::{CameraJson, SynapseSpec, WidgetLayout};
    use protocol::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::{Mutation, MutationDiff, SemanticMutation};
    use vcs::apply_mutation;

    fn round_trip(projection: &Procedural2dSnapshot, mutation: &Procedural2dMutation) -> Procedural2dSnapshot {
        let (forward, _) = apply_mutation(projection, mutation).expect("valid mutation");
        let mut restored = forward.clone();
        for back in mutation.inverse(projection) {
            restored = apply_mutation(&restored, &back).expect("valid inverse mutation").0;
        }
        assert_eq!(&restored, projection, "inverse() must restore the pre-mutation document");
        forward
    }

    #[test]
    fn fixture_ops_ignore_camera() {
        let before = FlowFixture::default();
        let mut after = before.clone();
        after.camera = CameraJson { x: 7.0, y: 8.0, zoom: 2.0 };
        let operations = procedural2d_fixture_operations(&before, &after);
        assert!(operations.iter().all(|operation| !matches!(operation, Procedural2dMutation::UpdateCamera(_))));
    }

    #[test]
    fn delete_and_recreate_widget_round_trips() {
        let base = empty_procedural2d_snapshot();
        let removed_id = widget_id(&base.fixture.widgets[0]).to_string();
        let after = round_trip(&base, &delete_widget(removed_id.clone()));
        assert!(!after.fixture.widgets.iter().any(|w| widget_id(w) == removed_id));
    }

    #[test]
    fn fixture_ops_capture_widget_creation() {
        let before = FlowFixture::default();
        let mut after = before.clone();
        after.widgets.push(Widget::InputNote { id: "note-1".into(), text: String::new() });
        let operations = procedural2d_fixture_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, Procedural2dMutation::CreateWidget(payload) if widget_id(&payload.widget) == "note-1")));
    }

    #[test]
    fn fixture_ops_capture_widget_replacement() {
        let mut before = FlowFixture::default();
        before.widgets.clear();
        before.widgets.push(Widget::InputNote { id: "note-1".into(), text: "old".into() });
        let mut after = before.clone();
        after.widgets[0] = Widget::InputNote { id: "note-1".into(), text: "new".into() };
        let operations = procedural2d_fixture_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, Procedural2dMutation::ReplaceWidget(payload) if widget_id(&payload.widget) == "note-1")));
    }

    #[test]
    fn generation_lifecycle_round_trips() {
        let before = empty_procedural2d_snapshot();
        let generation = FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        let after = round_trip(&before, &create_generation(generation));
        assert_eq!(after.generation.generations.len(), 1);
    }

    //#region 🔖️MutationInverseLawTests
    #[test]
    fn create_widget_inverse_law() {
        let base = empty_procedural2d_snapshot();
        let mutation = create_widget(0, Widget::InputNote { id: "brand-new".into(), text: String::new() });
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[test]
    fn replace_widget_inverse_law() {
        let base = empty_procedural2d_snapshot();
        let id = widget_id(&base.fixture.widgets[1]).to_string();
        let mutation = replace_widget(Widget::InputNote { id, text: "replaced".into() });
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[test]
    fn replace_widget_on_unknown_id_is_a_noop_with_no_inverse() {
        let base = empty_procedural2d_snapshot();
        let mutation = replace_widget(Widget::InputNote { id: "does-not-exist".into(), text: String::new() });
        assert!(mutation.inverse(&base).is_empty());
    }

    #[test]
    fn delete_widget_inverse_law() {
        let base = empty_procedural2d_snapshot();
        let id = widget_id(&base.fixture.widgets[1]).to_string();
        let mutation = delete_widget(id);
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[test]
    fn delete_widget_on_unknown_id_is_a_noop_with_no_inverse() {
        let base = empty_procedural2d_snapshot();
        let mutation = delete_widget("does-not-exist".into());
        assert!(mutation.inverse(&base).is_empty());
        let after = round_trip(&base, &mutation);
        assert_eq!(after, base);
    }

    #[test]
    fn connect_synapse_inverse_law() {
        let base = empty_procedural2d_snapshot();
        let synapse = SynapseSpec { id: "brand-new-synapse".into(), from: "slider".into(), to: "add".into(), from_port: "number".into(), to_port: "b".into() };
        let mutation = connect_synapse(0, synapse);
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[test]
    fn replace_synapse_inverse_law() {
        let base = empty_procedural2d_snapshot();
        let id = base.fixture.synapses[0].id.clone();
        let mutation = replace_synapse(SynapseSpec { id, from: "add".into(), to: "preview".into(), from_port: "sum".into(), to_port: "changed".into() });
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[test]
    fn disconnect_synapse_inverse_law() {
        let base = empty_procedural2d_snapshot();
        let id = base.fixture.synapses[0].id.clone();
        let mutation = disconnect_synapse(id);
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[test]
    fn disconnect_synapse_on_unknown_id_is_a_noop_with_no_inverse() {
        let base = empty_procedural2d_snapshot();
        let mutation = disconnect_synapse("missing".into());
        assert!(mutation.inverse(&base).is_empty());
    }

    #[test]
    fn move_widget_inverse_law_over_prior_layout() {
        let mut base = empty_procedural2d_snapshot();
        let id = widget_id(&base.fixture.widgets[0]).to_string();
        base.fixture.layout.insert(id.clone(), WidgetLayout { x: 1.0, y: 1.0 });
        let mutation = move_widget(id, WidgetLayout { x: 9.0, y: 9.0 });
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[test]
    fn move_widget_creating_a_layout_entry_clears_on_undo() {
        let base = empty_procedural2d_snapshot();
        assert!(base.fixture.layout.is_empty());
        let mutation = move_widget("slider".into(), WidgetLayout { x: 2.0, y: 2.0 });
        let after = round_trip(&base, &mutation);
        assert!(after.fixture.layout.contains_key("slider"));
    }

    #[test]
    fn clear_widget_layout_inverse_law() {
        let mut base = empty_procedural2d_snapshot();
        base.fixture.layout.insert("slider".into(), WidgetLayout { x: 4.0, y: 5.0 });
        let mutation = clear_widget_layout("slider".into());
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[test]
    fn clear_widget_layout_on_unknown_id_is_a_noop_with_no_inverse() {
        let base = empty_procedural2d_snapshot();
        let mutation = clear_widget_layout("missing".into());
        assert!(mutation.inverse(&base).is_empty());
    }

    #[test]
    fn update_camera_inverse_law() {
        let base = empty_procedural2d_snapshot();
        let mutation = update_camera(CameraJson { x: 42.0, y: -3.0, zoom: 5.0 });
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[test]
    fn change_schema_inverse_law() {
        let base = empty_procedural2d_snapshot();
        let mutation = change_schema("changed.schema".into());
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[test]
    fn create_generation_inverse_law() {
        let base = empty_procedural2d_snapshot();
        let generation = FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        assert_mutation_inverse_law(&base, &create_generation(generation));
    }

    #[test]
    fn rename_generation_inverse_law() {
        let mut base = empty_procedural2d_snapshot();
        base.generation.cold_builder_mut().expect("unique cold generation owner").generations.push(FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() });
        assert_mutation_inverse_law(&base, &rename_generation("generation-1".into(), "Renamed".into()));
    }

    #[test]
    fn change_generation_value_diff_absorb_law() {
        let mut base = empty_procedural2d_snapshot();
        base.generation.cold_builder_mut().expect("unique cold generation owner").generations.push(FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() });
        let d1 = change_generation_value("generation-1".into(), "q1".into(), serde_json::json!(1)).diff(&base);
        let mid = d1.apply(&base).expect("valid mutation diff");
        let d2 = change_generation_value("generation-1".into(), "q1".into(), serde_json::json!(2)).diff(&mid);
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🔖️MutationInverseLawTests

    #[test]
    fn dispatch_registers_semantic_descriptors() {
        register_procedural2d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
        for kind in Procedural2dMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(Procedural2dMutation::kinds().len(), 14);
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
        let operations = procedural2d_fixture_operations(&before, &after);
        for widget in &widgets {
            let id = widget_id(widget);
            assert!(operations.iter().any(|op| matches!(op, Procedural2dMutation::CreateWidget(payload) if widget_id(&payload.widget) == id)));
        }
    }

    #[test]
    fn widgets_diff_apply_replaces_by_id_and_removes_by_id() {
        let mut widgets = vec![Widget::InputNote { id: "a".into(), text: "1".into() }, Widget::InputNote { id: "b".into(), text: "2".into() }];
        let diff = crate::artifacts::procedural2d::diff::WidgetsDiff { removed: vec!["b".into()], set: vec![(0, Widget::InputNote { id: "a".into(), text: "replaced".into() })] };
        crate::artifacts::procedural2d::diff::apply_widgets_diff(&mut widgets, &diff);
        assert_eq!(widgets, vec![Widget::InputNote { id: "a".into(), text: "replaced".into() }]);
    }

    #[test]
    fn synapses_diff_apply_replaces_by_id_and_removes_by_id() {
        let mut synapses = vec![SynapseSpec { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() }];
        let diff = crate::artifacts::procedural2d::diff::SynapsesDiff { removed: vec![], set: vec![(0, SynapseSpec { id: "s1".into(), from: "a".into(), to: "c".into(), from_port: "out".into(), to_port: "in".into() })] };
        crate::artifacts::procedural2d::diff::apply_synapses_diff(&mut synapses, &diff);
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
        let descriptors = <Procedural2dMutation as protocol::SemanticMutation<Procedural2dSnapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared Procedural2dMutation variant");
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
