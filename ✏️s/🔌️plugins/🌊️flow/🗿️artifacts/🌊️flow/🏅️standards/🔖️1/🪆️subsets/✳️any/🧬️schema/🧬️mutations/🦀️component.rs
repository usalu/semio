//! 🧬️ Flow artifact — typed invertible semantic mutations over [`FlowSnapshot`]. Verbs drawn from
//! the closed taxonomy (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md`);
//! every variant wraps a `MutationKind<FlowSnapshot, FlowMutation>` payload from its own
//! `🧬️mutations/<kind>/` triad leaf. `impl Mutation`/`impl SemanticMutation` are
//! `#[derive(protocol::Mutations)]`-generated — never hand-written.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::flow::schema::diff::text::FlowDiff;
use crate::artifacts::flow::FlowSnapshot;
use protocol::{CollectionMutation, Mutation, MutationDiff};
use serde::{Deserialize, Serialize};
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔹Operation
/// 🌊️ Typed, invertible flow-document semantic mutations.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, protocol::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = FlowSnapshot, diff = FlowDiff, schema = "flow.flow")]
pub enum FlowMutation {
    CreateWidget(super::create_widget::mutation::CreateWidget),
    DeleteWidget(super::delete_widget::mutation::DeleteWidget),
    ReorderWidgets(super::reorder_widgets::mutation::ReorderWidgets),
    ReplaceWidget(super::replace_widget::mutation::ReplaceWidget),
    ConnectWidgets(super::connect_widgets::mutation::ConnectWidgets),
    DisconnectWidgets(super::disconnect_widgets::mutation::DisconnectWidgets),
    ReorderSynapses(super::reorder_synapses::mutation::ReorderSynapses),
    UpdateSynapseEndpoints(super::update_synapse_endpoints::mutation::UpdateSynapseEndpoints),
    MoveWidgets(super::move_widgets::mutation::MoveWidgets),
    DuplicateWidget(super::duplicate_widget::mutation::DuplicateWidget),
}

/// 🏷️ The kebab spelling of every [`FlowMutation`] variant, in DECLARATION ORDER — the one list the
/// language-neutral test platform is measured against. It is duplicated in exactly two other places
/// on purpose: this subset's own oracle manifest catalog `flow-1-any`
/// (`../../🧪️oracle/🔣️component.json`), which the completeness gate counts, and the `mutate-flow-1`
/// case adapter, which must not link this crate in the oracle role.
/// [`tests::kinds_match_the_enum_and_the_catalog`] is what keeps all three honest.
pub const KINDS: &[&str] = &["create-widget", "delete-widget", "reorder-widgets", "replace-widget", "connect-widgets", "disconnect-widgets", "reorder-synapses", "update-synapse-endpoints", "move-widgets", "duplicate-widget"];

pub type FlowEnvelope = ArtifactEnvelope<FlowSnapshot, FlowMutation>;
pub type FlowStore = ArtifactStore<FlowSnapshot, FlowMutation>;

/// 🌈️ Applies a mutation onto a snapshot in place.
pub fn apply_flow_mutation(snapshot: &mut FlowSnapshot, mutation: &FlowMutation) -> protocol::MutationApplyResult<()> {
    let next = <FlowMutation as Mutation<FlowSnapshot>>::diff(mutation, snapshot).diff().apply(snapshot)?;

    *snapshot = next;
    Ok(())
}

/// ↩️ Inverse mutations for undo.
pub fn inverse_flow_mutation(snapshot: &FlowSnapshot, mutation: &FlowMutation) -> Vec<FlowMutation> {
    <FlowMutation as Mutation<FlowSnapshot>>::inverse(mutation, snapshot)
}
//#endregion 🔹Operation

//#region 🔖️CaseBridges
/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "createWidget", …}`) JSON projection
/// — the shape the `mutate-flow-1` case's `Examples` rows carry — into a real [`FlowMutation`]. A
/// thin `serde_json` wrapper (already a direct dependency of this crate, used behind this interface
/// per CLAUDE.md's "external libraries behind an interface" rule, never a new one), so the case reads
/// the committed feature row instead of re-declaring it as a Rust literal beside it.
pub fn decode_flow_mutation_json(text: &str) -> Result<FlowMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📥️ Decodes a committed `{"widgets": [...], "synapses": [...], "layout": { … }}` document into the
/// real values a composed content child is seeded with. `Widget` is a typed UNION whose variant
/// decides its own field set, so a caller outside this crate cannot rebuild one by hand without
/// re-implementing that discriminant — which is exactly the knowledge this subset owns.
pub fn decode_flow_scene_json(text: &str) -> Result<(Vec<flow::Widget>, Vec<flow::SynapseSpec>, flow::OrderedMap<flow::WidgetLayout>), String> {
    #[derive(serde::Deserialize)]
    struct CommittedScene {
        #[serde(default)]
        widgets: Vec<flow::Widget>,
        #[serde(default)]
        synapses: Vec<flow::SynapseSpec>,
        #[serde(default)]
        layout: flow::OrderedMap<flow::WidgetLayout>,
    }
    let scene: CommittedScene = serde_json::from_str(text).map_err(|error| error.to_string())?;
    Ok((scene.widgets, scene.synapses, scene.layout))
}

/// ⚖️ The SEMANTIC PROJECTION this subset is compared through — `(schema, camera, widgets, synapses,
/// layout)`, the inline document fields plus the composed content child's working scene. It belongs
/// to the subset rather than to a test adapter, because what counts as this document's meaning is
/// this subset's ruling, not a case's. The content handle is deliberately absent:
/// `flow_content_child_handle` content-addresses that triple with domain-separated SHA-256.
/// Dedicated cross-language identity fixtures pin its exact canonical bytes and digest, while this
/// projection measures semantic mutation behavior without comparing the same content twice.
pub fn encode_flow_projection_json(snapshot: &FlowSnapshot) -> String {
    let scene = crate::artifacts::flow::flow_working_scene(snapshot);
    serde_json::json!({ "schema": snapshot.schema, "camera": snapshot.camera, "widgets": scene.widgets, "synapses": scene.synapses, "layout": scene.layout }).to_string()
}
//#endregion 🔖️CaseBridges

//#region 🌉️FrameworkBridge
/// 🌎️ Converts a framework kernel mutation into this plugin's semantic mutation vocabulary.
/// `SetFixture` (whole-fixture replace) has no semantic-mutation representation — banned per the
/// taxonomy's `set-snapshot` ruling, "it has NO replacement mutation" — so it returns `None`;
/// callers route that case through `store::ArtifactStore::reset` instead of the `Mutation` enum.
/// The framework's own diffing helper (`flow::flow_fixture_operations`) never emits `SetFixture`
/// (only `Widgets`/`Synapses`/`SetLayout`), so this arm is unreachable on the live host-bridge path
/// and only matters for a hand-authored/decoded `flow.op` line.
/// ✏️ Runs a stateful host mutation and diffs the result back into granular `FlowMutation`s — pure
/// over two snapshots, so it lives here beside [`from_framework_mutation`] rather than under an app.
/// Returns an empty vec when the two fixtures are identical.
pub fn snapshot_operations(before: &FlowSnapshot, after: &FlowSnapshot) -> Vec<FlowMutation> {
    flow::flow_fixture_operations(&before.to_fixture(), &after.to_fixture()).into_iter().filter_map(from_framework_mutation).collect()
}

pub fn from_framework_mutation(mutation: flow::FlowMutation) -> Option<FlowMutation> {
    Some(match mutation {
        flow::FlowMutation::Widgets(operation) => match operation {
            CollectionMutation::Add { index, item } => FlowMutation::CreateWidget(super::create_widget::mutation::CreateWidget { index, widget: item }),
            CollectionMutation::Remove { id } => FlowMutation::DeleteWidget(super::delete_widget::mutation::DeleteWidget { id }),
            CollectionMutation::Move { id, to_index } => FlowMutation::ReorderWidgets(super::reorder_widgets::mutation::ReorderWidgets { id, to_index }),
            CollectionMutation::Patch { id, patch } => FlowMutation::ReplaceWidget(super::replace_widget::mutation::ReplaceWidget { id, widget: patch }),
        },
        flow::FlowMutation::Synapses(operation) => match operation {
            CollectionMutation::Add { index, item } => FlowMutation::ConnectWidgets(super::connect_widgets::mutation::ConnectWidgets { index, id: item.id, from: item.from, from_port: item.from_port, to: item.to, to_port: item.to_port }),
            CollectionMutation::Remove { id } => FlowMutation::DisconnectWidgets(super::disconnect_widgets::mutation::DisconnectWidgets { id }),
            CollectionMutation::Move { id, to_index } => FlowMutation::ReorderSynapses(super::reorder_synapses::mutation::ReorderSynapses { id, to_index }),
            CollectionMutation::Patch { id, patch } => FlowMutation::UpdateSynapseEndpoints(super::update_synapse_endpoints::mutation::UpdateSynapseEndpoints { id, from: patch.from, from_port: patch.from_port, to: patch.to, to_port: patch.to_port }),
        },
        flow::FlowMutation::SetLayout { entries } => FlowMutation::MoveWidgets(super::move_widgets::mutation::MoveWidgets { entries }),
        flow::FlowMutation::SetFixture { .. } => return None,
    })
}

/// 🌎️ Converts this plugin's semantic mutation into the framework kernel mutation enum — `None` for
/// `DuplicateWidget`: a composite folds to a SINGLE `FlowDiff`, but it is not itself a single
/// framework-generic op (it plans two: a `Widgets::Add` then a `Synapses::Add`), so there is no
/// framework-generic counterpart to bridge to — mirrors [`from_framework_mutation`]'s `SetFixture`
/// case, one direction over.
pub fn to_framework_mutation(mutation: &FlowMutation) -> Option<flow::FlowMutation> {
    Some(match mutation {
        FlowMutation::CreateWidget(payload) => flow::FlowMutation::Widgets(CollectionMutation::Add { index: payload.index, item: payload.widget.clone() }),
        FlowMutation::DeleteWidget(payload) => flow::FlowMutation::Widgets(CollectionMutation::Remove { id: payload.id.clone() }),
        FlowMutation::ReorderWidgets(payload) => flow::FlowMutation::Widgets(CollectionMutation::Move { id: payload.id.clone(), to_index: payload.to_index }),
        FlowMutation::ReplaceWidget(payload) => flow::FlowMutation::Widgets(CollectionMutation::Patch { id: payload.id.clone(), patch: payload.widget.clone() }),
        FlowMutation::ConnectWidgets(payload) => flow::FlowMutation::Synapses(CollectionMutation::Add {
            index: payload.index,
            item: flow::SynapseSpec { id: payload.id.clone(), from: payload.from.clone(), from_port: payload.from_port.clone(), to: payload.to.clone(), to_port: payload.to_port.clone() },
        }),
        FlowMutation::DisconnectWidgets(payload) => flow::FlowMutation::Synapses(CollectionMutation::Remove { id: payload.id.clone() }),
        FlowMutation::ReorderSynapses(payload) => flow::FlowMutation::Synapses(CollectionMutation::Move { id: payload.id.clone(), to_index: payload.to_index }),
        FlowMutation::UpdateSynapseEndpoints(payload) => flow::FlowMutation::Synapses(CollectionMutation::Patch {
            id: payload.id.clone(),
            patch: flow::SynapseSpec { id: payload.id.clone(), from: payload.from.clone(), from_port: payload.from_port.clone(), to: payload.to.clone(), to_port: payload.to_port.clone() },
        }),
        FlowMutation::MoveWidgets(payload) => flow::FlowMutation::SetLayout { entries: payload.entries.clone() },
        FlowMutation::DuplicateWidget(_) => return None,
    })
}
//#endregion 🌉️FrameworkBridge

//#region 🔹WireCodecs
/// 🏷️ First byte of a `DuplicateWidget` op's binary encoding — reserved so it can never collide with
/// `store::os_dsl::variants_binary::OP_BINARY_FORMAT` (always `1`), the format every framework-bridged
/// leaf op decodes through. Any composite's own bytes are canonical-JSON of its payload (the same
/// idiom `HistoryOpMeta.origin` uses for a structured, non-hot-path field), not a `flow::FlowMutation`
/// bridge — see [`to_framework_mutation`]'s doc comment for why one cannot exist.
const DUPLICATE_WIDGET_OP_BINARY_TAG: u8 = 0xD0;
const DUPLICATE_WIDGET_OP_TEXT_KEYWORD: &str = "duplicate-widget ";

impl protocol::OpBinary for FlowMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let FlowMutation::DuplicateWidget(payload) = self else {
            let framework_mutation = to_framework_mutation(self).expect("only DuplicateWidget has no framework-generic op");
            return protocol::OpBinary::encode_op(&framework_mutation);
        };
        let mut bytes = vec![DUPLICATE_WIDGET_OP_BINARY_TAG];
        bytes.extend(serde_json::to_vec(payload).map_err(|error| protocol::ProtocolError::Malformed { what: "flow.op", offset: 0, detail: format!("duplicate-widget: {error}") })?);
        Ok(bytes)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        if bytes.first() == Some(&DUPLICATE_WIDGET_OP_BINARY_TAG) {
            let payload: super::duplicate_widget::mutation::DuplicateWidget = serde_json::from_slice(&bytes[1..]).map_err(|error| protocol::ProtocolError::Malformed { what: "flow.op", offset: 1, detail: format!("duplicate-widget: {error}") })?;
            return Ok(FlowMutation::DuplicateWidget(payload));
        }
        let framework_mutation = <flow::FlowMutation as protocol::OpBinary>::decode_op(bytes)?;
        from_framework_mutation(framework_mutation).ok_or_else(|| protocol::ProtocolError::Malformed {
            what: "flow.op",
            offset: 0,
            detail: "set-fixture has no semantic mutation representation (whole-document replace is banned; route through ArtifactStore::reset)".into(),
        })
    }
}
impl protocol::OpText for FlowMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        if let Some(rest) = line.strip_prefix(DUPLICATE_WIDGET_OP_TEXT_KEYWORD) {
            let payload: super::duplicate_widget::mutation::DuplicateWidget = serde_json::from_str(rest).map_err(|error| store::TextError::new(format!("duplicate-widget: {error}"), store::TextSpan::at(1, 1)))?;
            return Ok(FlowMutation::DuplicateWidget(payload));
        }
        let framework_mutation = <flow::FlowMutation as protocol::OpText>::parse_op(line)?;
        from_framework_mutation(framework_mutation).ok_or_else(|| store::TextError::new("set-fixture has no semantic mutation representation (whole-document replace is banned; route through ArtifactStore::reset)", store::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        let FlowMutation::DuplicateWidget(payload) = self else {
            let framework_mutation = to_framework_mutation(self).expect("only DuplicateWidget has no framework-generic op");
            return protocol::OpText::print_op(&framework_mutation);
        };
        format!("{DUPLICATE_WIDGET_OP_TEXT_KEYWORD}{}", serde_json::to_string(payload).expect("DuplicateWidget's all-String fields always serialize"))
    }
}
//#endregion 🔹WireCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::flow::schema::mutations::connect_widgets::mutation::ConnectWidgets;
    use crate::artifacts::flow::schema::mutations::create_widget::mutation::CreateWidget;
    use crate::artifacts::flow::schema::mutations::delete_widget::mutation::DeleteWidget;
    use crate::artifacts::flow::schema::mutations::disconnect_widgets::mutation::DisconnectWidgets;
    use crate::artifacts::flow::schema::mutations::move_widgets::mutation::MoveWidgets;
    use crate::artifacts::flow::schema::mutations::reorder_synapses::mutation::ReorderSynapses;
    use crate::artifacts::flow::schema::mutations::reorder_widgets::mutation::ReorderWidgets;
    use crate::artifacts::flow::schema::mutations::replace_widget::mutation::ReplaceWidget;
    use crate::artifacts::flow::schema::mutations::update_synapse_endpoints::mutation::UpdateSynapseEndpoints;
    use flow::{FlowLayoutEntry, Widget, WidgetLayout};
    use protocol::testkit::{assert_fatal_never_applies, assert_missing_target_is_error};

    async fn widget_note(id: &str) -> Widget {
        Widget::InputNote { id: id.into(), text: String::new() }
    }
    async fn widget_slider(id: &str) -> Widget {
        Widget::InputSlider { id: id.into(), label: id.into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 }
    }

    async fn apply(base: &FlowSnapshot, mutation: &FlowMutation) -> FlowSnapshot {
        <FlowMutation as Mutation<FlowSnapshot>>::diff(mutation, base).diff().apply(base).expect("valid mutation diff")
    }

    async fn base_with_two_widgets() -> FlowSnapshot {
        let base = apply(&FlowSnapshot::default(), &FlowMutation::CreateWidget(CreateWidget { index: 0, widget: widget_note("w1") }));
        apply(&base, &FlowMutation::CreateWidget(CreateWidget { index: 1, widget: widget_slider("w2") }))
    }

    async fn base_with_synapse() -> FlowSnapshot {
        let base = base_with_two_widgets();
        apply(&base, &FlowMutation::ConnectWidgets(ConnectWidgets { index: 0, id: "s1".into(), from: "w1".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() }))
    }

    //#region 🔖️OutcomeLaws
    /// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
    /// one `assert_missing_target_is_error`/Fatal check per verb family this facet implements
    /// (create/delete/connect/disconnect/move/replace/reorder/update).
    #[semio_framework_async_macros::async_test]
    async fn create_widget_duplicate_id_is_fatal() {
        let base = base_with_two_widgets();
        let outcome = FlowMutation::CreateWidget(CreateWidget { index: 0, widget: widget_note("w1") }).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_widget_missing_target_is_error() {
        let base = FlowSnapshot::default();
        assert_missing_target_is_error(&base, &FlowMutation::DeleteWidget(DeleteWidget { id: "ghost".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_widget_cascades_severed_synapses() {
        let base = base_with_synapse();
        let outcome = FlowMutation::DeleteWidget(DeleteWidget { id: "w1".into() }).diff(&base);
        assert!(
            outcome.messages().iter().any(|message| message.level == protocol::Severity::Info && message.code.0 == "mutation.cascade"),
            "deleting a widget that severs a synapse must carry an Info mutation.cascade message, got {:?}",
            outcome.messages()
        );
        assert!(outcome.diff().apply(&base).is_ok());
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_widgets_missing_endpoint_is_error() {
        let base = base_with_two_widgets();
        assert_missing_target_is_error(&base, &FlowMutation::ConnectWidgets(ConnectWidgets { index: 0, id: "edge-99".into(), from: "ghost".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_widgets_duplicate_id_is_fatal() {
        let base = base_with_synapse();
        let outcome = FlowMutation::ConnectWidgets(ConnectWidgets { index: 0, id: "s1".into(), from: "w2".into(), from_port: "out".into(), to: "w1".into(), to_port: "in".into() }).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_widgets_parallel_is_no_op() {
        let base = base_with_synapse();
        let outcome = FlowMutation::ConnectWidgets(ConnectWidgets { index: 1, id: "s2".into(), from: "w1".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
        assert_eq!(outcome.diff(), &FlowDiff::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn disconnect_widgets_missing_target_is_error() {
        let base = base_with_two_widgets();
        assert_missing_target_is_error(&base, &FlowMutation::DisconnectWidgets(DisconnectWidgets { id: "ghost".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_widgets_missing_target_is_error() {
        let base = base_with_two_widgets();
        assert_missing_target_is_error(&base, &FlowMutation::MoveWidgets(MoveWidgets { entries: vec![FlowLayoutEntry { id: "ghost".into(), layout: Some(WidgetLayout { x: 1.0, y: 1.0 }) }] }));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_widgets_non_finite_is_fatal() {
        let base = base_with_two_widgets();
        let outcome = FlowMutation::MoveWidgets(MoveWidgets { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: Some(WidgetLayout { x: f64::NAN, y: 0.0 }) }] }).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_widgets_unchanged_is_no_op() {
        let base = base_with_two_widgets();
        let moved = apply(&base, &FlowMutation::MoveWidgets(MoveWidgets { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: Some(WidgetLayout { x: 5.0, y: 5.0 }) }] }));
        let outcome = FlowMutation::MoveWidgets(MoveWidgets { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: Some(WidgetLayout { x: 5.0, y: 5.0 }) }] }).diff(&moved);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_widget_missing_target_is_error() {
        let base = base_with_two_widgets();
        assert_missing_target_is_error(&base, &FlowMutation::ReplaceWidget(ReplaceWidget { id: "ghost".into(), widget: widget_note("ghost") }));
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_widget_unchanged_is_no_op() {
        let base = base_with_two_widgets();
        let outcome = FlowMutation::ReplaceWidget(ReplaceWidget { id: "w1".into(), widget: widget_note("w1") }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_widgets_missing_target_is_error() {
        let base = base_with_two_widgets();
        assert_missing_target_is_error(&base, &FlowMutation::ReorderWidgets(ReorderWidgets { id: "ghost".into(), to_index: 0 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_widgets_already_current_is_no_op() {
        let base = base_with_two_widgets();
        let outcome = FlowMutation::ReorderWidgets(ReorderWidgets { id: "w1".into(), to_index: 0 }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_synapses_missing_target_is_error() {
        let base = base_with_synapse();
        assert_missing_target_is_error(&base, &FlowMutation::ReorderSynapses(ReorderSynapses { id: "ghost".into(), to_index: 0 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_synapses_already_current_is_no_op() {
        let base = base_with_synapse();
        let outcome = FlowMutation::ReorderSynapses(ReorderSynapses { id: "s1".into(), to_index: 0 }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    }

    #[semio_framework_async_macros::async_test]
    async fn update_synapse_endpoints_missing_target_is_error() {
        let base = base_with_synapse();
        assert_missing_target_is_error(&base, &FlowMutation::UpdateSynapseEndpoints(UpdateSynapseEndpoints { id: "ghost".into(), from: "w1".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn update_synapse_endpoints_missing_endpoint_is_error() {
        let base = base_with_synapse();
        assert_missing_target_is_error(&base, &FlowMutation::UpdateSynapseEndpoints(UpdateSynapseEndpoints { id: "s1".into(), from: "ghost".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn update_synapse_endpoints_unchanged_is_no_op() {
        let base = base_with_synapse();
        let outcome = FlowMutation::UpdateSynapseEndpoints(UpdateSynapseEndpoints { id: "s1".into(), from: "w1".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    }
    //#endregion 🔖️OutcomeLaws

    //#region 🔖️KindsCatalog
    /// 🏷️ [`KINDS`] is the bridge between this enum and the language-neutral test platform, which
    /// never parses Rust. This proves it names every variant, in declaration order, with the same
    /// kebab spelling `#[derive(protocol::Mutations)]` derives — and that this subset's own committed
    /// catalog declares exactly the same set, so the completeness gate cannot be measuring a
    /// vocabulary that has drifted away from the code.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let declared: Vec<&str> = <FlowMutation as protocol::SemanticMutation<FlowSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        assert_eq!(KINDS, declared.as_slice(), "KINDS must name every FlowMutation variant, in declaration order, spelled as its own MutationKind::SEMANTICS.kind");
        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in this subset's committed oracle manifest catalog flow-1-any");
        }
    }
    //#endregion 🔖️KindsCatalog
}
//#endregion 🧪️Tests
