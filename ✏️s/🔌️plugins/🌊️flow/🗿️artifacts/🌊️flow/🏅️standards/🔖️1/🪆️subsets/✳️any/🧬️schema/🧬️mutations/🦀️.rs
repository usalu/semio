//! 🧬️ Flow artifact — typed invertible semantic mutations over [`FlowSnapshot`]. Verbs drawn from
//! the closed taxonomy (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md`);
//! every variant wraps a `MutationKind<FlowSnapshot, FlowMutation>` payload from its own
//! `🧬️mutations/<kind>/` triad leaf. `impl Mutation`/`impl SemanticMutation` are
//! `#[derive(protocol::Mutations)]`-generated — never hand-written.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::diff::text::FlowDiff;
use crate::FlowSnapshot;
use protocol::{Mutation, MutationDiff};
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔹Operation
/// 🌊️ Typed, invertible flow-document semantic mutations.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, protocol::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = FlowSnapshot, diff = FlowDiff, schema = "flow.flow")]
pub enum FlowMutation {
    CreateWidget(super::create_widget::CreateWidget),
    DeleteWidget(super::delete_widget::DeleteWidget),
    ReorderWidgets(super::reorder_widgets::ReorderWidgets),
    ReplaceWidget(super::replace_widget::ReplaceWidget),
    ConnectWidgets(super::connect_widgets::ConnectWidgets),
    DisconnectWidgets(super::disconnect_widgets::DisconnectWidgets),
    ReorderSynapses(super::reorder_synapses::ReorderSynapses),
    UpdateSynapseEndpoints(super::update_synapse_endpoints::UpdateSynapseEndpoints),
    MoveWidgets(super::move_widgets::MoveWidgets),
    DuplicateWidget(super::duplicate_widget::mutation::DuplicateWidget),
}

/// 🏷️ The kebab spelling of every [`FlowMutation`] variant, in DECLARATION ORDER — the one list the
/// language-neutral test platform is measured against. It is duplicated in exactly two other places
/// on purpose: this subset's own oracle manifest catalog `flow-1-any`
/// (`../../🔣️oracle.json`), which the completeness gate counts, and the `🌊️mutate-flow-1`
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
/// — the shape the `🌊️mutate-flow-1` case's `Examples` rows carry — into a real [`FlowMutation`]. A
/// thin `serde_json` wrapper (already a direct dependency of this crate, used behind this interface
/// per CLAUDE.md's "external libraries behind an interface" rule, never a new one), so the case reads
/// the committed feature row instead of re-declaring it as a Rust literal beside it.
pub fn decode_flow_mutation_json(text: &str) -> Result<FlowMutation, String> {
    let json: serde_json::Value = serde_json::from_str(text).map_err(|error| error.to_string())?;
    let value: dsl::DslValue = json.into();
    dsl::FromValue::from_value(value).map_err(|error| error.to_string())
}

/// 📥️ Decodes a committed `{"widgets": [...], "synapses": [...], "layout": { … }}` document into the
/// real values a composed content child is seeded with. `Widget` is a typed UNION whose variant
/// decides its own field set, so a caller outside this crate cannot rebuild one by hand without
/// re-implementing that discriminant — which is exactly the knowledge this subset owns.
pub fn decode_flow_scene_json(text: &str) -> Result<(Vec<semio_framework_artifact_flow_flow::Widget>, Vec<semio_framework_artifact_flow_flow::SynapseSpec>, flow::OrderedMap<semio_framework_artifact_flow_flow::WidgetLayout>), String> {
    #[derive(value_derive::FromValue)]
    struct CommittedScene {
        #[value(default)]
        widgets: Vec<semio_framework_artifact_flow_flow::Widget>,
        #[value(default)]
        synapses: Vec<semio_framework_artifact_flow_flow::SynapseSpec>,
        #[value(default)]
        layout: flow::OrderedMap<semio_framework_artifact_flow_flow::WidgetLayout>,
    }
    let json: serde_json::Value = serde_json::from_str(text).map_err(|error| error.to_string())?;
    let value: dsl::DslValue = json.into();
    let scene: CommittedScene = dsl::FromValue::from_value(value).map_err(|error| error.to_string())?;
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
    let scene = crate::flow_working_scene(snapshot);
    let value = dsl::DslValue::object([
        ("schema".to_string(), dsl::ToValue::to_value(&snapshot.schema)),
        ("camera".to_string(), dsl::ToValue::to_value(&snapshot.camera)),
        ("widgets".to_string(), dsl::ToValue::to_value(&scene.widgets)),
        ("synapses".to_string(), dsl::ToValue::to_value(&scene.synapses)),
        ("layout".to_string(), dsl::ToValue::to_value(&scene.layout)),
    ]);
    let json: serde_json::Value = value.into();
    json.to_string()
}
//#endregion 🔖️CaseBridges

//#region 🌉️FrameworkBridge
/// 🌎️ Converts a framework kernel mutation into this plugin's semantic mutation vocabulary.
/// `ReplaceFlowFixture` (whole-fixture replace) has no semantic-mutation representation — banned
/// per the taxonomy's `set-snapshot` ruling, "it has NO replacement mutation" — so it returns
/// `None`; callers route that case through `store::ArtifactStore::reset` instead of the `Mutation`
/// enum. The framework's own diffing helper (`semio_framework_artifact_flow_flow::flow_fixture_operations`) never emits
/// `ReplaceFlowFixture` (only the add/remove/move/change leaves), so this arm is unreachable on the
/// live host-bridge path and only matters for a hand-authored/decoded `flow.op` line.
/// ✏️ Runs a stateful host mutation and diffs the result back into granular `FlowMutation`s — pure
/// over two snapshots, so it lives here beside [`from_framework_mutation`] rather than under an app.
/// Returns an empty vec when the two fixtures are identical, or when the framework diff itself fails.
pub fn snapshot_operations(before: &FlowSnapshot, after: &FlowSnapshot) -> Vec<FlowMutation> {
    semio_framework_artifact_flow_flow::flow_fixture_operations(&before.to_fixture(), &after.to_fixture()).unwrap_or_default().into_iter().filter_map(from_framework_mutation).collect()
}

pub fn from_framework_mutation(mutation: semio_framework_artifact_flow_flow::FlowMutation) -> Option<FlowMutation> {
    Some(match mutation {
        semio_framework_artifact_flow_flow::FlowMutation::AddWidget(payload) => FlowMutation::CreateWidget(super::create_widget::CreateWidget { index: payload.index as usize, widget: payload.widget }),
        semio_framework_artifact_flow_flow::FlowMutation::RemoveWidget(payload) => FlowMutation::DeleteWidget(super::delete_widget::DeleteWidget { id: payload.id }),
        semio_framework_artifact_flow_flow::FlowMutation::MoveWidget(payload) => FlowMutation::ReorderWidgets(super::reorder_widgets::ReorderWidgets { id: payload.id, to_index: payload.to_index as usize }),
        semio_framework_artifact_flow_flow::FlowMutation::ChangeWidget(payload) => FlowMutation::ReplaceWidget(super::replace_widget::ReplaceWidget { id: payload.id, widget: payload.widget }),
        semio_framework_artifact_flow_flow::FlowMutation::AddSynapse(payload) => FlowMutation::ConnectWidgets(super::connect_widgets::ConnectWidgets { index: payload.index as usize, id: payload.synapse.id, from: payload.synapse.from, from_port: payload.synapse.from_port, to: payload.synapse.to, to_port: payload.synapse.to_port }),
        semio_framework_artifact_flow_flow::FlowMutation::RemoveSynapse(payload) => FlowMutation::DisconnectWidgets(super::disconnect_widgets::DisconnectWidgets { id: payload.id }),
        semio_framework_artifact_flow_flow::FlowMutation::MoveSynapse(payload) => FlowMutation::ReorderSynapses(super::reorder_synapses::ReorderSynapses { id: payload.id, to_index: payload.to_index as usize }),
        semio_framework_artifact_flow_flow::FlowMutation::ChangeSynapse(payload) => FlowMutation::UpdateSynapseEndpoints(super::update_synapse_endpoints::UpdateSynapseEndpoints { id: payload.id, from: payload.synapse.from, from_port: payload.synapse.from_port, to: payload.synapse.to, to_port: payload.synapse.to_port }),
        semio_framework_artifact_flow_flow::FlowMutation::ChangeLayout(payload) => FlowMutation::MoveWidgets(super::move_widgets::MoveWidgets { entries: payload.entries }),
        semio_framework_artifact_flow_flow::FlowMutation::ReplaceFlowFixture(_) => return None,
    })
}

/// 🌎️ Converts this plugin's semantic mutation into the framework kernel mutation enum — `None` for
/// `DuplicateWidget`: a composite folds to a SINGLE `FlowDiff`, but it is not itself a single
/// framework-generic op (it plans two: an `AddWidget` then an `AddSynapse`), so there is no
/// framework-generic counterpart to bridge to — mirrors [`from_framework_mutation`]'s
/// `ReplaceFlowFixture` case, one direction over.
pub fn to_framework_mutation(mutation: &FlowMutation) -> Option<semio_framework_artifact_flow_flow::FlowMutation> {
    Some(match mutation {
        FlowMutation::CreateWidget(payload) => semio_framework_artifact_flow_flow::FlowMutation::AddWidget(semio_framework_artifact_flow_flow::AddWidget { index: payload.index as u32, widget: payload.widget.clone() }),
        FlowMutation::DeleteWidget(payload) => semio_framework_artifact_flow_flow::FlowMutation::RemoveWidget(semio_framework_artifact_flow_flow::RemoveWidget { id: payload.id.clone() }),
        FlowMutation::ReorderWidgets(payload) => semio_framework_artifact_flow_flow::FlowMutation::MoveWidget(semio_framework_artifact_flow_flow::MoveWidget { id: payload.id.clone(), to_index: payload.to_index as u32 }),
        FlowMutation::ReplaceWidget(payload) => semio_framework_artifact_flow_flow::FlowMutation::ChangeWidget(semio_framework_artifact_flow_flow::ChangeWidget { id: payload.id.clone(), widget: payload.widget.clone() }),
        FlowMutation::ConnectWidgets(payload) => semio_framework_artifact_flow_flow::FlowMutation::AddSynapse(semio_framework_artifact_flow_flow::AddSynapse {
            index: payload.index as u32,
            synapse: semio_framework_artifact_flow_flow::SynapseSpec { id: payload.id.clone(), from: payload.from.clone(), from_port: payload.from_port.clone(), to: payload.to.clone(), to_port: payload.to_port.clone() },
        }),
        FlowMutation::DisconnectWidgets(payload) => semio_framework_artifact_flow_flow::FlowMutation::RemoveSynapse(semio_framework_artifact_flow_flow::RemoveSynapse { id: payload.id.clone() }),
        FlowMutation::ReorderSynapses(payload) => semio_framework_artifact_flow_flow::FlowMutation::MoveSynapse(semio_framework_artifact_flow_flow::MoveSynapse { id: payload.id.clone(), to_index: payload.to_index as u32 }),
        FlowMutation::UpdateSynapseEndpoints(payload) => semio_framework_artifact_flow_flow::FlowMutation::ChangeSynapse(semio_framework_artifact_flow_flow::ChangeSynapse {
            id: payload.id.clone(),
            synapse: semio_framework_artifact_flow_flow::SynapseSpec { id: payload.id.clone(), from: payload.from.clone(), from_port: payload.from_port.clone(), to: payload.to.clone(), to_port: payload.to_port.clone() },
        }),
        FlowMutation::MoveWidgets(payload) => semio_framework_artifact_flow_flow::FlowMutation::ChangeLayout(semio_framework_artifact_flow_flow::ChangeLayout { entries: payload.entries.clone() }),
        FlowMutation::DuplicateWidget(_) => return None,
    })
}
//#endregion 🌉️FrameworkBridge

//#region 🔹WireCodecs
/// 🏷️ First byte of a `DuplicateWidget` op's binary encoding — reserved so it can never collide with
/// `store::os_dsl::variants_binary::OP_BINARY_FORMAT` (always `1`), the format every framework-bridged
/// leaf op decodes through. Any composite's own bytes are canonical-JSON of its payload (the same
/// idiom `HistoryOpMeta.origin` uses for a structured, non-hot-path field), not a `semio_framework_artifact_flow_flow::FlowMutation`
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
        let json: serde_json::Value = dsl::ToValue::to_value(payload).into();
        bytes.extend(serde_json::to_vec(&json).map_err(|error| protocol::ProtocolError::Malformed { what: "flow.op", offset: 0, detail: format!("duplicate-widget: {error}") })?);
        Ok(bytes)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        if bytes.first() == Some(&DUPLICATE_WIDGET_OP_BINARY_TAG) {
            let json: serde_json::Value = serde_json::from_slice(&bytes[1..]).map_err(|error| protocol::ProtocolError::Malformed { what: "flow.op", offset: 1, detail: format!("duplicate-widget: {error}") })?;
            let value: dsl::DslValue = json.into();
            let payload: super::duplicate_widget::mutation::DuplicateWidget = dsl::FromValue::from_value(value).map_err(|error| protocol::ProtocolError::Malformed { what: "flow.op", offset: 1, detail: format!("duplicate-widget: {error}") })?;
            return Ok(FlowMutation::DuplicateWidget(payload));
        }
        let framework_mutation = <semio_framework_artifact_flow_flow::FlowMutation as protocol::OpBinary>::decode_op(bytes)?;
        from_framework_mutation(framework_mutation).ok_or_else(|| protocol::ProtocolError::Malformed {
            what: "flow.op",
            offset: 0,
            detail: "replace-flow-fixture has no semantic mutation representation (whole-document replace is banned; route through ArtifactStore::reset)".into(),
        })
    }
}
impl protocol::OpText for FlowMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        if let Some(rest) = line.strip_prefix(DUPLICATE_WIDGET_OP_TEXT_KEYWORD) {
            let json: serde_json::Value = serde_json::from_str(rest).map_err(|error| store::TextError::new(format!("duplicate-widget: {error}"), store::TextSpan::at(1, 1)))?;
            let value: dsl::DslValue = json.into();
            let payload: super::duplicate_widget::mutation::DuplicateWidget = dsl::FromValue::from_value(value).map_err(|error| store::TextError::new(format!("duplicate-widget: {error}"), store::TextSpan::at(1, 1)))?;
            return Ok(FlowMutation::DuplicateWidget(payload));
        }
        let framework_mutation = <semio_framework_artifact_flow_flow::FlowMutation as protocol::OpText>::parse_op(line)?;
        from_framework_mutation(framework_mutation).ok_or_else(|| store::TextError::new("replace-flow-fixture has no semantic mutation representation (whole-document replace is banned; route through ArtifactStore::reset)", store::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        let FlowMutation::DuplicateWidget(payload) = self else {
            let framework_mutation = to_framework_mutation(self).expect("only DuplicateWidget has no framework-generic op");
            return protocol::OpText::print_op(&framework_mutation);
        };
        let json: serde_json::Value = dsl::ToValue::to_value(payload).into();
        format!("{DUPLICATE_WIDGET_OP_TEXT_KEYWORD}{json}")
    }
}
//#endregion 🔹WireCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
