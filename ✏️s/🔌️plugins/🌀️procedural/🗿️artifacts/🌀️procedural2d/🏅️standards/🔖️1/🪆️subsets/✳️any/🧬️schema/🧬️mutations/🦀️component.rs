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
use flow::{FlowFixture, Widget};
use flow::playbook::{FormGeneration, GenerationMutation};
use protocol::{Mutation, MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use store::{ArtifactEnvelope, ArtifactStore};
/// 🧵 Sibling triad-leaf modules wired by `📦️glue.rs` under eight pre-existing (pre-semantic)
/// directory slots — their directory/module names are leftovers of the generic slots each was
/// repurposed from (`sharedFileRequests` in this ticket's wave2 report has the glue.rs rename that
/// would align them; not editable here — glue.rs is shared with the sibling `procedural3d` artifact).
use super::{change_schema, clear_widget_layout, connect_synapse, create_widget, delete_widget, disconnect_synapse, move_widget, set_camera};

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

//#region 🔖️ReplaceWidget
/// ♻️ Whole-payload swap of an EXISTING id-keyed widget. Overflow variant — no pre-wired `📦️glue.rs`
/// triad slot, so its leaves live inline as nested modules rather than a separate directory.
pub mod replace_widget {
    use super::*;

    //#region 🦠️Mutation
    /// ♻️ `replace-widget` payload — the widget's new payload (id embedded, addresses the target).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ReplaceWidget {
        pub widget: Widget,
    }

    /// 🏗️ Builder — wraps the payload in its dispatch variant.
    pub fn replace_widget(widget: Widget) -> super::Procedural2dMutation {
        super::Procedural2dMutation::ReplaceWidget(ReplaceWidget { widget })
    }

    impl MutationKind<Procedural2dSnapshot, super::Procedural2dMutation> for ReplaceWidget {
        const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "widget", kind: "replace-widget", record: "ReplacedWidget" };

        fn diff(&self, base: &Procedural2dSnapshot) -> Procedural2dDiff {
            diff::diff(self, base)
        }
        fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<super::Procedural2dMutation> {
            inverse::inverse(self, base)
        }
        fn label(&self) -> String {
            format!("Replace widget \"{}\"", widget_id(&self.widget))
        }
        fn target(&self) -> Vec<String> {
            vec![widget_id(&self.widget).to_string()]
        }
    }
    //#endregion 🦠️Mutation

    //#region 🔺️Diff
    pub mod diff {
        use super::*;
        use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};

        pub fn diff(payload: &ReplaceWidget, base: &Procedural2dSnapshot) -> Procedural2dDiff {
            let index = widget_index(&base.fixture, widget_id(&payload.widget)).unwrap_or(base.fixture.widgets.len());
            diff_fixture_from_helpers(base, WidgetsDiff { removed: vec![], set: vec![(index, payload.widget.clone())] }, SynapsesDiff::default(), LayoutDiff::default(), None, None)
        }
    }
    //#endregion 🔺️Diff

    //#region ↩️Inverse
    pub mod inverse {
        use super::*;

        pub fn inverse(payload: &ReplaceWidget, base: &Procedural2dSnapshot) -> Vec<super::super::Procedural2dMutation> {
            match widget_index(&base.fixture, widget_id(&payload.widget)) {
                Some(index) => vec![replace_widget(base.fixture.widgets[index].clone())],
                None => Vec::new(),
            }
        }
    }
    //#endregion ↩️Inverse
}
//#endregion 🔖️ReplaceWidget

//#region 🔖️ReplaceSynapse
/// ♻️ Whole-payload swap of an EXISTING id-keyed synapse (rewires endpoints/ports). Overflow
/// variant — no pre-wired `📦️glue.rs` triad slot, so its leaves live inline.
pub mod replace_synapse {
    use super::*;
    use flow::SynapseSpec;

    //#region 🦠️Mutation
    /// ♻️ `replace-synapse` payload — the edge's new payload (id embedded, addresses the target).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ReplaceSynapse {
        pub synapse: SynapseSpec,
    }

    /// 🏗️ Builder — wraps the payload in its dispatch variant.
    pub fn replace_synapse(synapse: SynapseSpec) -> super::Procedural2dMutation {
        super::Procedural2dMutation::ReplaceSynapse(ReplaceSynapse { synapse })
    }

    impl MutationKind<Procedural2dSnapshot, super::Procedural2dMutation> for ReplaceSynapse {
        const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "synapse", kind: "replace-synapse", record: "ReplacedSynapse" };

        fn diff(&self, base: &Procedural2dSnapshot) -> Procedural2dDiff {
            diff::diff(self, base)
        }
        fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<super::Procedural2dMutation> {
            inverse::inverse(self, base)
        }
        fn label(&self) -> String {
            format!("Replace synapse \"{}\"", self.synapse.id)
        }
        fn target(&self) -> Vec<String> {
            vec![self.synapse.id.clone()]
        }
    }
    //#endregion 🦠️Mutation

    //#region 🔺️Diff
    pub mod diff {
        use super::*;
        use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};

        pub fn diff(payload: &ReplaceSynapse, base: &Procedural2dSnapshot) -> Procedural2dDiff {
            let index = synapse_index(&base.fixture, &payload.synapse.id).unwrap_or(base.fixture.synapses.len());
            diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff { removed: vec![], set: vec![(index, payload.synapse.clone())] }, LayoutDiff::default(), None, None)
        }
    }
    //#endregion 🔺️Diff

    //#region ↩️Inverse
    pub mod inverse {
        use super::*;

        pub fn inverse(payload: &ReplaceSynapse, base: &Procedural2dSnapshot) -> Vec<super::super::Procedural2dMutation> {
            match synapse_index(&base.fixture, &payload.synapse.id) {
                Some(index) => vec![replace_synapse(base.fixture.synapses[index].clone())],
                None => Vec::new(),
            }
        }
    }
    //#endregion ↩️Inverse
}
//#endregion 🔖️ReplaceSynapse

//#region 🔖️CreateGeneration
/// 🌱 Brings a new generation into existence. Delegates to `flow::playbook`'s already-semantic
/// `GenerationMutation::Add` for the underlying apply/diff machinery. Overflow variant — no
/// pre-wired `📦️glue.rs` triad slot, so its leaves live inline.
pub mod create_generation {
    use super::*;
    use crate::artifacts::procedural2d::diff::diff_generation_from_ops;

    //#region 🦠️Mutation
    /// 🌱 `create-generation` payload — the full new generation.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct CreateGeneration {
        pub generation: FormGeneration,
    }

    /// 🏗️ Builder — wraps the payload in its dispatch variant.
    pub fn create_generation(generation: FormGeneration) -> super::Procedural2dMutation {
        super::Procedural2dMutation::CreateGeneration(CreateGeneration { generation })
    }

    impl MutationKind<Procedural2dSnapshot, super::Procedural2dMutation> for CreateGeneration {
        const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "generation", kind: "create-generation", record: "CreatedGeneration" };

        fn diff(&self, base: &Procedural2dSnapshot) -> Procedural2dDiff {
            diff_generation_from_ops(base, vec![GenerationMutation::Add { generation: self.generation.clone() }])
        }
        fn inverse(&self, _base: &Procedural2dSnapshot) -> Vec<super::Procedural2dMutation> {
            vec![super::delete_generation::delete_generation(self.generation.id.clone())]
        }
        fn label(&self) -> String {
            format!("Create generation \"{}\"", self.generation.name)
        }
        fn target(&self) -> Vec<String> {
            vec![self.generation.id.clone()]
        }
    }
    //#endregion 🦠️Mutation
}
//#endregion 🔖️CreateGeneration

//#region 🔖️DeleteGeneration
/// 🗑️ Removes a generation by id (captures it for its inverse). Overflow variant — no pre-wired
/// `📦️glue.rs` triad slot, so its leaves live inline.
pub mod delete_generation {
    use super::*;
    use crate::artifacts::procedural2d::diff::diff_generation_from_ops;

    //#region 🦠️Mutation
    /// 🗑️ `delete-generation` payload — removes the generation with `id`.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DeleteGeneration {
        pub id: String,
    }

    /// 🏗️ Builder — wraps the payload in its dispatch variant.
    pub fn delete_generation(id: String) -> super::Procedural2dMutation {
        super::Procedural2dMutation::DeleteGeneration(DeleteGeneration { id })
    }

    impl MutationKind<Procedural2dSnapshot, super::Procedural2dMutation> for DeleteGeneration {
        const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "generation", kind: "delete-generation", record: "DeletedGeneration" };

        fn diff(&self, base: &Procedural2dSnapshot) -> Procedural2dDiff {
            diff_generation_from_ops(base, vec![GenerationMutation::Remove { id: self.id.clone() }])
        }
        fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<super::Procedural2dMutation> {
            match base.generation.generations.iter().find(|entry| entry.id == self.id) {
                Some(entry) => vec![super::create_generation::create_generation(entry.clone())],
                None => Vec::new(),
            }
        }
        fn label(&self) -> String {
            format!("Delete generation \"{}\"", self.id)
        }
        fn target(&self) -> Vec<String> {
            vec![self.id.clone()]
        }
    }
    //#endregion 🦠️Mutation
}
//#endregion 🔖️DeleteGeneration

//#region 🔖️RenameGeneration
/// ✏️ Changes a generation's display name. Overflow variant — no pre-wired `📦️glue.rs` triad slot,
/// so its leaves live inline.
pub mod rename_generation {
    use super::*;
    use crate::artifacts::procedural2d::diff::diff_generation_from_ops;

    //#region 🦠️Mutation
    /// ✏️ `rename-generation` payload — the generation's new display name.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RenameGeneration {
        pub id: String,
        pub name: String,
    }

    /// 🏗️ Builder — wraps the payload in its dispatch variant.
    pub fn rename_generation(id: String, name: String) -> super::Procedural2dMutation {
        super::Procedural2dMutation::RenameGeneration(RenameGeneration { id, name })
    }

    impl MutationKind<Procedural2dSnapshot, super::Procedural2dMutation> for RenameGeneration {
        const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "generation", kind: "rename-generation", record: "RenamedGeneration" };

        fn diff(&self, base: &Procedural2dSnapshot) -> Procedural2dDiff {
            diff_generation_from_ops(base, vec![GenerationMutation::Rename { id: self.id.clone(), name: self.name.clone() }])
        }
        fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<super::Procedural2dMutation> {
            match base.generation.generations.iter().find(|entry| entry.id == self.id) {
                Some(entry) => vec![rename_generation(self.id.clone(), entry.name.clone())],
                None => Vec::new(),
            }
        }
        fn label(&self) -> String {
            format!("Rename generation \"{}\" to \"{}\"", self.id, self.name)
        }
        fn target(&self) -> Vec<String> {
            vec![self.id.clone()]
        }
    }
    //#endregion 🦠️Mutation
}
//#endregion 🔖️RenameGeneration

//#region 🔖️ChangeGenerationValue
/// 🔧 Sets one form field's value on a generation's answer map. Overflow variant — no pre-wired
/// `📦️glue.rs` triad slot, so its leaves live inline.
pub mod change_generation_value {
    use super::*;
    use crate::artifacts::procedural2d::diff::diff_generation_from_ops;

    //#region 🦠️Mutation
    /// 🔧 `change-generation-value` payload — a generation's new value for one question.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ChangeGenerationValue {
        pub id: String,
        pub question_id: String,
        pub value: serde_json::Value,
    }

    /// 🏗️ Builder — wraps the payload in its dispatch variant.
    pub fn change_generation_value(id: String, question_id: String, value: serde_json::Value) -> super::Procedural2dMutation {
        super::Procedural2dMutation::ChangeGenerationValue(ChangeGenerationValue { id, question_id, value })
    }

    impl MutationKind<Procedural2dSnapshot, super::Procedural2dMutation> for ChangeGenerationValue {
        const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "generation-value", kind: "change-generation-value", record: "ChangedGenerationValue" };

        fn diff(&self, base: &Procedural2dSnapshot) -> Procedural2dDiff {
            diff_generation_from_ops(base, vec![GenerationMutation::UpdateValues { id: self.id.clone(), question_id: self.question_id.clone(), value: self.value.clone() }])
        }
        fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<super::Procedural2dMutation> {
            match base.generation.generations.iter().find(|entry| entry.id == self.id) {
                Some(entry) => vec![change_generation_value(self.id.clone(), self.question_id.clone(), entry.values.get(&self.question_id).cloned().unwrap_or(serde_json::Value::Null))],
                None => Vec::new(),
            }
        }
        fn label(&self) -> String {
            format!("Change generation \"{}\" value \"{}\"", self.id, self.question_id)
        }
        fn target(&self) -> Vec<String> {
            vec![self.id.clone(), self.question_id.clone()]
        }
    }
    //#endregion 🦠️Mutation
}
//#endregion 🔖️ChangeGenerationValue

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[mutations(snapshot = Procedural2dSnapshot, diff = Procedural2dDiff, schema = "procedural.2d")]
pub enum Procedural2dMutation {
    CreateWidget(create_widget::mutation::CreateWidget),
    ReplaceWidget(replace_widget::ReplaceWidget),
    DeleteWidget(delete_widget::mutation::DeleteWidget),
    ConnectSynapse(connect_synapse::mutation::ConnectSynapse),
    ReplaceSynapse(replace_synapse::ReplaceSynapse),
    DisconnectSynapse(disconnect_synapse::mutation::DisconnectSynapse),
    MoveWidget(move_widget::mutation::MoveWidget),
    ClearWidgetLayout(clear_widget_layout::mutation::ClearWidgetLayout),
    UpdateCamera(set_camera::mutation::UpdateCamera),
    ChangeSchema(change_schema::mutation::ChangeSchema),
    CreateGeneration(create_generation::CreateGeneration),
    DeleteGeneration(delete_generation::DeleteGeneration),
    RenameGeneration(rename_generation::RenameGeneration),
    ChangeGenerationValue(change_generation_value::ChangeGenerationValue),
}
//#endregion 🔖️Mutations

//#region 🔖️Builders
pub use create_generation::create_generation;
pub use delete_generation::delete_generation;
pub use rename_generation::rename_generation;
pub use change_generation_value::change_generation_value;
pub use replace_widget::replace_widget;
pub use replace_synapse::replace_synapse;
pub use create_widget::mutation::create_widget;
pub use delete_widget::mutation::delete_widget;
pub use connect_synapse::mutation::connect_synapse;
pub use disconnect_synapse::mutation::disconnect_synapse;
pub use move_widget::mutation::move_widget;
pub use clear_widget_layout::mutation::clear_widget_layout;
pub use set_camera::mutation::update_camera;
pub use change_schema::mutation::change_schema;
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
pub fn apply_procedural2d_mutation(projection: &mut Procedural2dSnapshot, mutation: &Procedural2dMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_procedural2d_mutation(projection: &Procedural2dSnapshot, mutation: &Procedural2dMutation) -> Vec<Procedural2dMutation> {
    mutation.inverse(projection)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural2d::engine::empty_procedural2d_snapshot;
    use flow::{CameraJson, SynapseSpec, WidgetLayout};
    use protocol::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::{Mutation, MutationDiff, SemanticMutation};
    use vcs::apply_mutation;

    fn round_trip(projection: &Procedural2dSnapshot, mutation: &Procedural2dMutation) -> Procedural2dSnapshot {
        let forward = apply_mutation(projection, mutation);
        let mut restored = forward.clone();
        for back in mutation.inverse(projection) {
            restored = apply_mutation(&restored, &back);
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
        base.generation.generations.push(FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() });
        assert_mutation_inverse_law(&base, &rename_generation("generation-1".into(), "Renamed".into()));
    }

    #[test]
    fn change_generation_value_diff_absorb_law() {
        let mut base = empty_procedural2d_snapshot();
        base.generation.generations.push(FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() });
        let d1 = change_generation_value("generation-1".into(), "q1".into(), serde_json::json!(1)).diff(&base);
        let mid = d1.apply(&base);
        let d2 = change_generation_value("generation-1".into(), "q1".into(), serde_json::json!(2)).diff(&mid);
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🔖️MutationInverseLawTests

    #[test]
    fn dispatch_registers_semantic_descriptors() {
        register_procedural2d_mutation_descriptors();
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
            Widget::InputSlider { id: "w-slider".into(), value: 1.0, min: 0.0, max: 2.0, step: 0.5 },
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
}
//#endregion 🧪️Tests
