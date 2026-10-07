//! 🖥️ Sequence play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum, the
//! manifest stitch, and this app's own typed media I/O surface + plugin registration + editing host
//! (below — constitutional: general, an artifact must never depend on an app, so all three live here
//! rather than under `🗿️artifacts`).
//!
//! Command bodies live in `🎮️commands/*`, window renders in `🎭️modes/✏️edit/🪟️windows/*`, panel trees in
//! `📌️panels/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`. `handle` →
//! `SequenceCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node. `SequenceHost` (below) is the UI-editing engine shared by more than one
//! taxonomy node (commands, windows, panels, the wasm bridge) — a helper with exactly one consumer
//! lives in that consumer's own component file instead.

use crate::editor::sequence::commands::connection::{connect_steps, disconnect_steps};
use crate::editor::sequence::commands::example::set_active_example;
use crate::editor::sequence::commands::layout::{reorganize, set_orientation};
use crate::editor::sequence::commands::node_graph::{node_graph_edit, set_viewport};
use crate::editor::sequence::commands::playback::{run_command, stop_command};
use crate::editor::sequence::commands::step::{add_step, add_step_dropped, add_step_to_slot, delete_selection, move_step, remove_step, set_step_collapsed, set_step_params};
use crate::editor::sequence::modes::edit;
use crate::editor::sequence::modes::edit::windows::{compiled, main, script};
use crate::editor::sequence::modes::edit::windows::main::config::SequenceMainWindowConfig;
use crate::editor::sequence::modes::edit::windows::script::transient::SequenceScriptWindowTransient;
use crate::editor::sequence::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::sequence::terminology::sequence_play_labels;
use crate::mutations::SequenceMutation;
use crate::{SequenceCamera, SequenceEdge, SequenceHostSnapshot, SequenceSnapshot, SequenceStep, SequenceWorkingScene, SlotRef, StepParams, SEQUENCE_DOCUMENT_SCHEMA};
use dag::{would_create_cycle, DagHost, DagLayoutOptions};
use graph::manifest::PropertyBag;
use imperative_engine::{compile_to_text as imperative_compile_to_text, imperative_catalogue_json, imperative_module_registry, Executor, Path, RunResult, Step};
use infinite_board_port_directed_dag as dag;
use neural_engine::{ChannelSpec, ColdOwner, Dictionary, Registry, RegistryRetirement, SharedRegistry, Value as NeuralValue, ValueRetirement, ValueRetirementStep};
use semio_framework_artifact_infinite_dag::{dag_host_snapshot_to_wire_literal, DagCamera, DagHostSnapshot, DagHostSnapshotEdge, DagNodeSpec, EdgeRouteStyle, IoPortSpec, PortShape};
use semio_framework_plugin::app::{ChildEmit,ChildEmitPreparation};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppActionRegistry;
use semio_framework_plugin::AppDefinition;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::ContextMenuItemSpec;
use semio_framework_plugin::ContextMenuRequest;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DomainTopology;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::DslValue;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTopology;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::TopologyNode;
use semio_framework_pack_json::{self as json, json};
use semio_framework_pack_json::Value;
use semio_framework_tool_machine::{node_drag_emit, NodeDragEmit, NodeGraphEditRow};
use std::collections::{BTreeMap, HashMap, VecDeque};
use semio_framework_2d::compute::EngineHandles;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::{self as semio_flow_mutations, SemioFlowMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

//#region 🔖️Constants
pub const SEQUENCE_PLAY_APP_ID: &str = "s.sequence.sequence@1/*#editor";

/// 🔁️ The whole-document replacement `setActiveExample` publishes — the sanctioned non-history
/// "replace the whole document" gesture (`ArtifactStore::reset`, applied host-side), which a
/// composed-child document needs because the taxonomy forbids a whole-snapshot mutation variant.
/// The spr is a fresh, edit-free op-log built with `store::empty_document_spr` — NEVER a live
/// `ArtifactEnvelope` minted just to print one: such an envelope is a terminal store shell whose
/// `Drop` asserts that its app-owned bounded retirement authority detached every nested owner first,
/// and nothing on this path ever mounts or retires it, so the envelope route traps the guest
/// (`🧊️process3d` lost its whole editor to exactly this).
pub fn reset_sequence_document_effect(document: &SequenceSnapshot) -> semio_framework_plugin::Effect {
    let pack = crate::standards::v1::subsets::any::io::snapshot_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr(SEQUENCE_PLAY_APP_ID, SEQUENCE_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}
pub use catalogue_panel::SEQUENCE_PLAY_BODY_CATALOGUE;
pub use compiled::SEQUENCE_PLAY_BODY_COMPILED;
pub use document_panel::SEQUENCE_PLAY_BODY_ARTIFACT;
pub use inspection_panel::SEQUENCE_PLAY_BODY_INSPECTOR;
pub use main::SEQUENCE_PLAY_BODY_MAIN;
pub use script::SEQUENCE_PLAY_BODY_SCRIPT;
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "steps" interaction domain — one
/// granularity ("step"), `HierarchyProvider::Topology` from each step's own `SlotRef.owner`
/// control-flow nesting (see `SequencePlayApp::interaction_topology`). Ids are the steps' own raw
/// document ids — the SAME ids the main node-graph canvas's `NodeGraphNodeRecord.id` and the document
/// panel tree's row ids both use, so a selection made through either surface resolves identically.
pub const SEQUENCE_INTERACTION_STEPS: &str = "steps";

/// 🕹️ The only granularity the "steps" domain declares — stamped on every pick row of the document
/// tree so the host synthesizes `interactionSelect` without a per-row argument map.
pub const SEQUENCE_INTERACTION_GRANULARITY: &str = "step";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn sequence_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(SEQUENCE_PLAY_APP_ID).action(action, args)
}

/// 🏷️ Admits one semantic label for Sequence panels.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "sequence label admission failed"))
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub use semio_framework_plugin::ui_node_list;

//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors `create_sequence_app`'s
/// `.artifact_kind(...)` literal (schema/media type copied verbatim) plus the extra `steps:in` input
/// port (Wave-2 port recipe): incoming computation results from an upstream workflow node insert as
/// new steps in the sequence document (see `SequencePlayApp::import_media` below).
pub fn sequence_io() -> AppIo {
    AppIo {
        artifact_schema: SEQUENCE_DOCUMENT_SCHEMA.into(),
        artifact_media_type: semio_framework::MediaType { class: semio_framework::MediaClass::Computation, form: semio_framework::MediaForm::Sequence },
        ports: vec![semio_framework::MediaPortSpec {
            id: "steps:in".into(),
            label: "Steps".into(),
            direction: semio_framework::MediaPortDirection::In,
            media_type: semio_framework::MediaType { class: semio_framework::MediaClass::Computation, form: semio_framework::MediaForm::Any },
            kind_id: None,
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework::ArtifactPresentation { id: "computation.sequence".into(), name: "Sequence".into(), dimension: "graph".into(), component_kind: "sequence".into() },
    }
}

/// 🧸️ Resolves the document's exact published content child through its captured member-store view.
pub fn sequence_working_scene_from_children(snapshot: &SequenceSnapshot, children: &semio_framework_plugin::app::ChildContentView) -> Result<ColdOwner<SequenceWorkingScene>, Fault> {
    let child_id = &snapshot.content.child_id;
    let dialect = children.dialect("content", child_id).ok_or_else(|| sequence_fault("sequence.content.dialect", "sequence-content-child-dialect-required"))?;
    if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != "flow" {
        return Err(sequence_fault("sequence.content.dialect", "sequence-content-child-dialect-mismatch"));
    }
    let content = children.typed_read::<SemioFlowSnapshot>("content", child_id)?;
    let (steps, edges) = crate::working_from_sequence_content_snapshot(&content);
    Ok(ColdOwner::new(SequenceWorkingScene { steps, edges }))
}

/// 🎬️ Projects the published content child into the editor's plain execution snapshot.
pub fn sequence_host_snapshot_from_children(snapshot: &SequenceSnapshot, children: &semio_framework_plugin::app::ChildContentView) -> Result<ColdOwner<SequenceHostSnapshot>, Fault> {
    let SequenceWorkingScene { steps, edges } = sequence_working_scene_from_children(snapshot, children)?.into_inner();
    Ok(ColdOwner::new(SequenceHostSnapshot { schema: snapshot.schema.clone(), steps, edges }))
}

//#region 🔖️ChildIntentLeaves
/// 📏️ How far two drag offsets may differ and still be one drag: a host moves every dragged step by one world delta, and
/// recomputing `after - before` per step only differs by rounding.
const SEQUENCE_DRAG_OFFSET_TOLERANCE: f64 = 1e-6;

/// 🧮️ The intent leaves of the composed Flow content child that carry `base` to `next`, never a whole `set-snapshot`:
/// severed edges first (`remove-edge`), removed steps (`remove-node`), inserted steps (`insert-node`), each kept step's
/// kind/label/param changes (`set-node-kind`, `set-node-label`, `set-node-param`, `remove-node-param`), its position
/// changes as relative `drag-nodes` (one leaf per drag offset), then edge endpoint/kind changes and inserted edges.
pub fn sequence_content_leaves(base: &SemioFlowSnapshot, next: &SemioFlowSnapshot) -> Vec<SemioFlowMutation> {
    let mut leaves = Vec::new();
    for edge in &base.edges {
        if !next.edges.iter().any(|entry| entry.id == edge.id) {
            leaves.push(SemioFlowMutation::RemoveEdge(semio_flow_mutations::remove_edge::RemoveEdge { id: edge.id.clone() }));
        }
    }
    for node in &base.nodes {
        if !next.nodes.iter().any(|entry| entry.id == node.id) {
            leaves.push(SemioFlowMutation::RemoveNode(semio_flow_mutations::remove_node::RemoveNode { id: node.id.clone() }));
        }
    }
    let mut drags: Vec<(f64, f64, Vec<String>)> = Vec::new();
    for node in &next.nodes {
        let Some(prior) = base.nodes.iter().find(|entry| entry.id == node.id) else {
            leaves.push(SemioFlowMutation::InsertNode(semio_flow_mutations::insert_node::InsertNode::new(node.clone())));
            continue;
        };
        if prior.kind != node.kind {
            leaves.push(SemioFlowMutation::SetNodeKind(semio_flow_mutations::set_node_kind::SetNodeKind { id: node.id.clone(), kind: node.kind.clone() }));
        }
        if prior.label != node.label {
            leaves.push(SemioFlowMutation::SetNodeLabel(semio_flow_mutations::set_node_label::SetNodeLabel { id: node.id.clone(), label: node.label.clone() }));
        }
        for param in &node.params {
            if !prior.params.iter().any(|entry| entry.key == param.key && entry.value == param.value) {
                leaves.push(SemioFlowMutation::SetNodeParam(semio_flow_mutations::set_node_param::SetNodeParam { id: node.id.clone(), key: param.key.clone(), value: param.value.clone() }));
            }
        }
        for param in &prior.params {
            if !node.params.iter().any(|entry| entry.key == param.key) {
                leaves.push(SemioFlowMutation::RemoveNodeParam(semio_flow_mutations::remove_node_param::RemoveNodeParam { id: node.id.clone(), key: param.key.clone() }));
            }
        }
        let (dx, dy) = (node.position.x - prior.position.x, node.position.y - prior.position.y);
        if (dx, dy) != (0.0, 0.0) && dx.is_finite() && dy.is_finite() {
            match drags.iter_mut().find(|(gx, gy, _)| (gx - dx).abs() <= SEQUENCE_DRAG_OFFSET_TOLERANCE && (gy - dy).abs() <= SEQUENCE_DRAG_OFFSET_TOLERANCE) {
                Some((_, _, targets)) => targets.push(node.id.clone()),
                None => drags.push((dx, dy, vec![node.id.clone()])),
            }
        }
    }
    leaves.extend(drags.into_iter().map(|(dx, dy, targets)| SemioFlowMutation::DragNodes(semio_flow_mutations::drag_nodes::DragNodes { targets, dx, dy })));
    for edge in &next.edges {
        match base.edges.iter().find(|entry| entry.id == edge.id) {
            None => leaves.push(SemioFlowMutation::InsertEdge(semio_flow_mutations::insert_edge::InsertEdge::new(edge.clone()))),
            Some(prior) => {
                if prior.from != edge.from || prior.to != edge.to {
                    leaves.push(SemioFlowMutation::SetEdgeEndpoints(semio_flow_mutations::set_edge_endpoints::SetEdgeEndpoints { id: edge.id.clone(), from: edge.from.clone(), to: edge.to.clone() }));
                }
                if prior.kind != edge.kind {
                    leaves.push(SemioFlowMutation::SetEdgeKind(semio_flow_mutations::set_edge_kind::SetEdgeKind { id: edge.id.clone(), kind: edge.kind.clone() }));
                }
            }
        }
    }
    leaves
}

/// 🧮️ The child intent leaves that carry the working scene `base` to `next`.
pub fn sequence_scene_leaves(base: &SequenceWorkingScene, next: &SequenceWorkingScene) -> Vec<SemioFlowMutation> {
    sequence_content_leaves(&crate::sequence_content_snapshot_from_working(&base.steps, &base.edges), &crate::sequence_content_snapshot_from_working(&next.steps, &next.edges))
}

/// 🌊️ Publishes child intent `leaves` as ONE edit of the exact composed Flow content child; nothing is the empty emit.
pub fn sequence_child_leaves_emit(snapshot: &SequenceSnapshot, leaves: Vec<SemioFlowMutation>) -> Emit<SequenceMutation, NoConfigMutation> {
    if leaves.is_empty() {
        return Emit::default();
    }
    Emit { child_preparations: std::collections::VecDeque::from([ChildEmitPreparation::of::<SemioFlowSnapshot, _>("content", &snapshot.content.child_id, leaves)]), ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }
}

/// 🧰️ The child intent leaves one editor host mutation yields against the captured typed child.
pub fn sequence_child_leaves_from_host_mutation(doc: &ArtifactView<'_, SequenceSnapshot>, mutate: impl FnOnce(&mut SequenceHost)) -> Result<Vec<SemioFlowMutation>, Fault> {
    let live = sequence_host_snapshot_from_children(doc.snapshot, &doc.children)?;
    let mut host = ColdOwner::new(host_from_host_snapshot(&live));
    mutate(&mut host);
    let base = ColdOwner::new(SequenceWorkingScene { steps: live.steps.clone(), edges: live.edges.clone() });
    let next = ColdOwner::new(SequenceWorkingScene { steps: host.snapshot.steps.clone(), edges: host.snapshot.edges.clone() });
    Ok(sequence_scene_leaves(&base, &next))
}

/// 🧰️ Applies one editor host mutation to the captured typed child and publishes its intent leaves as ONE child edit.
pub fn sequence_child_emit_from_host_mutation(doc: &ArtifactView<'_, SequenceSnapshot>, mutate: impl FnOnce(&mut SequenceHost)) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
    Ok(sequence_child_leaves_emit(doc.snapshot, sequence_child_leaves_from_host_mutation(doc, mutate)?))
}
//#endregion 🔖️ChildIntentLeaves

//#endregion 🔖️Io

//#region 🔖️Camera
/// 🎥️ `SequenceCamera` <-> `DagCamera` conversions — plain functions rather than `From`/`Into` trait
/// impls, because `SequenceCamera` is defined in the artifact's own `🦀️.rs` and `DagCamera`
/// is foreign (from the DAG layout kernel): neither type nor trait would be local to THIS file, so a
/// trait impl here would violate the orphan rule. Only `SequenceHost` (which already depends on the DAG
/// kernel for `DagHost`) needs the conversion, so plain functions here are both legal and sufficient.
pub fn sequence_camera_from_dag(value: &DagCamera) -> SequenceCamera {
    SequenceCamera { x: value.x, y: value.y, zoom: value.zoom }
}

pub fn dag_camera_from_sequence(value: &SequenceCamera) -> DagCamera {
    DagCamera { x: value.x, y: value.y, zoom: value.zoom }
}
//#endregion 🔖️Camera

//#region ⚠️ Errors
/// 🚨️ `SequenceHost`'s fallible operations.
#[derive(Debug)]
pub enum SequenceCoreError {
    Json(String),
    UnsupportedSchema(String),
    SelfConnect,
    StepNotFound(String),
    MismatchedSlotScope,
    CycleDetected,
    OutgoingFlowExists(String),
    UnknownStep(String),
    Dag(String),
}

impl std::fmt::Display for SequenceCoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
            Self::UnsupportedSchema(schema) => write!(formatter, "unsupported schema: {schema}"),
            Self::SelfConnect => formatter.write_str("cannot connect step to itself"),
            Self::StepNotFound(step) => write!(formatter, "{step} not found"),
            Self::MismatchedSlotScope => formatter.write_str("steps must share the same slot scope"),
            Self::CycleDetected => formatter.write_str("connection would create cycle"),
            Self::OutgoingFlowExists(step) => write!(formatter, "{step} already has outgoing flow"),
            Self::UnknownStep(step) => write!(formatter, "unknown step: {step}"),
            Self::Dag(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for SequenceCoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(_) => None,
            _ => None,
        }
    }
}

/// 🚧️ One named refusal of the sequence editor: `code` is a row of [`sequence_fault_notices`] (or a framework code), `detail`
/// the English diagnostic the logs keep; the person reads the notice.
pub(crate) fn sequence_fault(code: &'static str, detail: impl Into<String>) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, code, detail)
}

/// 📢️ The app fault notices of the sequence editor (`code → {en, de}`): every refusal its retained steps, windows and
/// commands raise through [`sequence_fault`].
pub fn sequence_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: std::sync::OnceLock<Vec<(&'static str, LocalizedLabel)>> = std::sync::OnceLock::new();
    NOTICES
        .get_or_init(|| {
            vec![
                ("sequence.content.dialect", LocalizedLabel::native("The sequence's step graph is not a Semio flow graph.", "Der Schrittgraph der Sequenz ist kein Semio-Flussgraph.")),
                ("sequence.content.unavailable", LocalizedLabel::native("The sequence's step graph is not loaded yet.", "Der Schrittgraph der Sequenz ist noch nicht geladen.")),
                ("sequence.window.unavailable", LocalizedLabel::native("This action needs an open sequence window.", "Diese Aktion braucht ein geöffnetes Sequenzfenster.")),
                ("sequence.editor.capacity", LocalizedLabel::native("This sequence is too large for this action.", "Diese Sequenz ist für diese Aktion zu groß.")),
                ("sequence.run.capacity", LocalizedLabel::native("The run produced more than the editor can hold; reduce repeats or effects.", "Der Lauf hat mehr erzeugt, als der Editor fassen kann; Wiederholungen oder Effekte reduzieren.")),
                ("sequence.run.step-missing", LocalizedLabel::native("The run reached a step that no longer exists.", "Der Lauf hat einen Schritt erreicht, der nicht mehr existiert.")),
                ("sequence.resume.invalid", LocalizedLabel::native("The interrupted action could not be resumed; start it again.", "Die unterbrochene Aktion konnte nicht fortgesetzt werden; bitte erneut starten.")),
                ("sequence.publication.lane", LocalizedLabel::native("This change could not be recorded in the sequence's step graph.", "Diese Änderung konnte nicht im Schrittgraph der Sequenz aufgezeichnet werden.")),
                ("sequence.retained.artifact-command", LocalizedLabel::native("This edit does not fit the sequence editor's current step.", "Diese Bearbeitung passt nicht zum aktuellen Schritt des Sequenzeditors.")),
                ("sequence.retained.config-command", LocalizedLabel::native("This view change does not fit the sequence editor's current step.", "Diese Ansichtsänderung passt nicht zum aktuellen Schritt des Sequenzeditors.")),
                ("sequence.retained.example-command", LocalizedLabel::native("This example could not be loaded into the sequence editor.", "Dieses Beispiel konnte nicht in den Sequenzeditor geladen werden.")),
                ("sequence.retained.close", LocalizedLabel::native("The sequence editor could not finish closing this action.", "Der Sequenzeditor konnte das Schließen dieser Aktion nicht abschließen.")),
                ("sequence.node-graph.malformed", LocalizedLabel::native("The node graph edit is malformed.", "Die Knotengraph-Bearbeitung ist fehlerhaft.")),
                ("sequence.node-graph.unsupported", LocalizedLabel::native("A sequence has no sliders and no variadic ports.", "Eine Sequenz hat keine Schieberegler und keine variadischen Anschlüsse.")),
                ("sequence.import-media.missing", LocalizedLabel::native("Choose a media file to import.", "Eine Mediendatei zum Importieren auswählen.")),
                ("sequence.import-media.undecoded", LocalizedLabel::native("The media could not be decoded for import.", "Das Medium konnte für den Import nicht dekodiert werden.")),
                ("sequence.viewport.camera", LocalizedLabel::native("The viewport change needs a valid camera.", "Die Ansichtsänderung braucht eine gültige Kamera.")),
                ("sequence.action.unhandled", LocalizedLabel::native("This action is not available in the sequence editor.", "Diese Aktion ist im Sequenzeditor nicht verfügbar.")),
                ("sequence.example.unparsable", LocalizedLabel::native("The bundled sequence example could not be read.", "Das mitgelieferte Sequenzbeispiel konnte nicht gelesen werden.")),
                ("sequence.child.projection", LocalizedLabel::native("The sequence document's step graph could not be restored.", "Der Schrittgraph des Sequenzdokuments konnte nicht wiederhergestellt werden.")),
            ]
        })
        .as_slice()
}

//#endregion ⚠️ Errors

//#region 🔖️Host
const SEQUENCE_DAG_COMPONENT_WIDTH: f64 = 200.0;
const SEQUENCE_DAG_CHANNEL_ROW_HEIGHT: f64 = 24.0;

fn sequence_computation_node_width(_name: &str, _inputs: &[IoPortSpec], _outputs: &[IoPortSpec]) -> f64 {
    SEQUENCE_DAG_COMPONENT_WIDTH
}

fn sequence_computation_node_height(input_count: usize, output_count: usize, _variadic_inputs: bool, _variadic_outputs: bool) -> f64 {
    let rows = input_count.max(output_count).max(1);
    rows as f64 * SEQUENCE_DAG_CHANNEL_ROW_HEIGHT
}
const FLOW_INPUT_PORT: &str = "prev";
const FLOW_OUTPUT_PORT: &str = "next";

fn property_bag_from_dictionary(dict: &Dictionary) -> PropertyBag {
    semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(dict)).unwrap_or_default()
}

/// 🧭️ `pub` — reused by other app taxonomy nodes (panels/commands: control-flow nesting, catalogue slots).
pub fn is_control_kind(kind: &str) -> bool {
    matches!(kind, "control.if" | "control.while" | "control.repeat")
}

fn is_function_kind(kind: &str) -> bool {
    kind.starts_with("math.") || kind.starts_with("logic.") || kind.starts_with("text.")
}

fn parse_serial_suffix(prefix: &str, id: &str) -> Option<u64> {
    id.strip_prefix(prefix)?.parse().ok()
}

fn max_serial_in_snapshot(snapshot: &SequenceHostSnapshot) -> u64 {
    let mut max = 0u64;
    for step in &snapshot.steps {
        if let Some(serial) = parse_serial_suffix("step-", &step.id) {
            max = max.max(serial);
        }
    }
    for edge in &snapshot.edges {
        if let Some(serial) = parse_serial_suffix("edge-", &edge.id) {
            max = max.max(serial);
        }
    }
    max
}

fn default_control_slot(kind: &str) -> &'static str {
    if kind == "control.if" {
        "then"
    } else {
        "body"
    }
}

fn neural_value_to_dsl_value(value: &NeuralValue) -> DslValue {
    semio_framework_value::ToValue::to_value(value)
}

// 🧯️ `unnecessary_wraps` — mirrors `IoPortSpec::value_type`'s `Option<String>` field shape; every
// branch here happens to be populated today, but the field itself is genuinely optional.
#[allow(clippy::unnecessary_wraps)]
fn channel_spec_value_type(spec: &ChannelSpec) -> Option<String> {
    if spec.operators.is_empty() {
        Some("value".into())
    } else {
        Some(spec.operators.join(","))
    }
}

fn channel_spec_to_output_port(spec: &ChannelSpec) -> IoPortSpec {
    let mut port = IoPortSpec::named(&spec.code, &spec.abbreviation, &spec.name, &spec.full_name);
    port.label = spec.label.clone().unwrap_or_else(|| spec.code.clone());
    port.value_type = channel_spec_value_type(spec);
    port.default = spec.default.as_ref().map(neural_value_to_dsl_value);
    port.cardinality = spec.cardinality.symbol();
    port
}

fn input_spec_to_port(spec: &ChannelSpec, params: &Dictionary) -> IoPortSpec {
    let value = params.get(&spec.name).or(spec.default.as_ref()).map(neural_value_to_dsl_value);
    let mut port = IoPortSpec::named(&spec.code, &spec.abbreviation, &spec.name, &spec.full_name);
    port.label = spec.label.clone().unwrap_or_else(|| spec.code.clone());
    port.value_type = channel_spec_value_type(spec);
    port.default = spec.default.as_ref().map(neural_value_to_dsl_value);
    port.value = value;
    port.connected = Some(false);
    port.cardinality = spec.cardinality.symbol();
    port
}

fn hidden_flow_input_port() -> IoPortSpec {
    let mut port = IoPortSpec::named("", "", FLOW_INPUT_PORT, "");
    port.cardinality = String::new();
    port.visible = false;
    port
}

fn hidden_flow_output_port() -> IoPortSpec {
    let mut port = IoPortSpec::named("", "", FLOW_OUTPUT_PORT, "");
    port.cardinality = String::new();
    port.visible = false;
    port
}

fn visible_flow_input_port() -> IoPortSpec {
    let mut port = IoPortSpec::named("", "", FLOW_INPUT_PORT, "Previous");
    port.shape = PortShape::Triangle;
    port.cardinality = String::new();
    port
}

fn visible_flow_output_port() -> IoPortSpec {
    let mut port = IoPortSpec::named("", "", FLOW_OUTPUT_PORT, "Next");
    port.shape = PortShape::Triangle;
    port.cardinality = String::new();
    port
}

/// 🧭️ `pub` — reused by other app taxonomy nodes (panels/commands: control-flow nesting, catalogue slots).
pub fn control_slots(kind: &str) -> &'static [&'static str] {
    match kind {
        "control.if" => &["then", "else"],
        "control.while" | "control.repeat" => &["body"],
        _ => &[],
    }
}

fn slot_key(slot: Option<&SlotRef>) -> Option<(String, String)> {
    slot.map(|entry| (entry.owner.clone(), entry.name.clone()))
}


pub struct SequenceHost {
    /// 🌊️ The plain pre-migration document shape (`{schema, steps, edges}`) — this plugin's own
    /// working representation, matching `SequenceHostSnapshot`'s doc comment. `SequenceHost` edits this
    /// in place exactly as it edited `SequenceSnapshot.steps`/`.edges` directly before the
    /// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` migration (`sequence→C:flow`) — only the
    /// boundary conversions (`from_snapshot`/`replace_snapshot`/`to_json`/`load_json`) changed.
    pub snapshot: SequenceHostSnapshot,
    /// 🎥️ The canvas camera — session-only host state (never a `SequenceSnapshot` document field; see
    /// the exact main-window camera). Persists across `rebuild_dag()` calls
    /// within this `SequenceHost` instance (each document mutation rebuilds `dag` from scratch, so
    /// this is what the rebuilt `dag`'s camera gets reseeded from).
    pub camera: SequenceCamera,
    pub dag: DagHost,
    registry: Registry,
    next_serial: u64,
}

impl neural_engine::ColdRetire for SequenceHost {
    fn retire_cold(self) {
        let Self { snapshot, registry, .. } = self;
        neural_engine::ColdRetire::retire_cold(snapshot);
        neural_engine::ColdRetire::retire_cold(registry);
    }
}

/// 🧊️ The canonical genesis content, moved into the host (its `StepParams` dictionaries are then owned by the host's own
/// cold boundary, [`SequenceHost`]'s `ColdRetire`).
impl Default for SequenceHost {
    fn default() -> Self {
        Self::from_host_snapshot(crate::snapshot::schema::default_host_snapshot())
    }
}

impl SequenceHost {
    /// 🌊️ Builds a live host directly from a plain snapshot (the WASM bridge's `loadSnapshotJson`/
    /// `SequenceHost::load_json` entry point).
    pub fn from_host_snapshot(host_snapshot: SequenceHostSnapshot) -> Self {
        let next_serial = max_serial_in_snapshot(&host_snapshot).max(100);
        let mut host = Self {
            snapshot: host_snapshot,
            camera: SequenceCamera::default(),
            dag: DagHost::from_host_snapshot_without_layout(DagHostSnapshot { schema: "dag.host_snapshot".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] }),
            registry: imperative_module_registry(),
            next_serial,
        };
        host.rebuild_dag();
        host
    }

    pub fn replace_snapshot(&mut self, snapshot: SequenceHostSnapshot) -> Result<(), SequenceCoreError> {
        if snapshot.schema != "sequence.sequence" {
            return Err(SequenceCoreError::UnsupportedSchema(snapshot.schema));
        }
        self.next_serial = self.next_serial.max(max_serial_in_snapshot(&snapshot));
        self.snapshot = snapshot;
        self.rebuild_dag();
        Ok(())
    }

    pub fn load_json(json: &str) -> Result<Self, SequenceCoreError> {
        let snapshot: SequenceHostSnapshot = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| SequenceCoreError::Json(error.to_string()))?;
        if snapshot.schema != "sequence.sequence" {
            return Err(SequenceCoreError::UnsupportedSchema(snapshot.schema));
        }
        Ok(Self::from_host_snapshot(snapshot))
    }

    pub fn to_json(&self) -> Result<String, SequenceCoreError> {
        Ok(semio_framework_pack_json::to_json_string(&self.snapshot))
    }

    pub fn catalogue_json(&self) -> String {
        imperative_catalogue_json(&self.registry)
    }

    pub fn pick_step_id_at_screen(&self, sx: f64, sy: f64, width: u32, height: u32, dpr: f64) -> Option<String> {
        use infinite_canvas::camera::{screen_to_world, Camera as CanvasCamera, Viewport};
        use infinite_canvas::Point;
        let viewport = Viewport { width: width.max(1), height: height.max(1), dpr: dpr.max(1.0) };
        let camera = CanvasCamera { x: self.dag.host_snapshot.camera.x, y: self.dag.host_snapshot.camera.y, zoom: self.dag.host_snapshot.camera.zoom };
        let world = screen_to_world(&camera, &viewport, Point::new(sx, sy));
        for node in self.dag.host_snapshot.nodes.iter().rev() {
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            if world.x >= node.x - hw && world.x <= node.x + hw && world.y >= node.y - hh && world.y <= node.y + hh {
                return Some(node.id.clone());
            }
        }
        None
    }

    pub fn add_step(&mut self, kind: &str, x: f64, y: f64) -> String {
        self.add_step_in_slot(kind, x, y, None)
    }

    pub fn add_step_dropped(&mut self, kind: &str, x: f64, y: f64, picked_step_id: Option<&str>) -> String {
        if let Some(owner_id) = picked_step_id {
            if let Some(owner) = self.snapshot.steps.iter().find(|step| step.id == owner_id) {
                if is_control_kind(&owner.kind) && !owner.collapsed {
                    return self.add_step_in_slot(kind, x, y, Some(SlotRef { owner: owner_id.into(), name: default_control_slot(&owner.kind).into() }));
                }
            }
        }
        self.add_step(kind, x, y)
    }

    fn next_step_id(&mut self) -> String {
        loop {
            self.next_serial += 1;
            let id = format!("step-{}", self.next_serial);
            if !self.snapshot.steps.iter().any(|step| step.id == id) {
                return id;
            }
        }
    }

    fn next_edge_id(&mut self) -> String {
        loop {
            self.next_serial += 1;
            let id = format!("edge-{}", self.next_serial);
            if !self.snapshot.edges.iter().any(|edge| edge.id == id) {
                return id;
            }
        }
    }

    pub fn add_step_in_slot(&mut self, kind: &str, x: f64, y: f64, slot: Option<SlotRef>) -> String {
        self.clear_ghost_step();
        let id = self.next_step_id();
        self.snapshot.steps.push(SequenceStep { id: id.clone(), kind: kind.into(), params: StepParams::new(), x, y, slot, collapsed: false });
        self.rebuild_dag();
        id
    }

    pub fn set_step_collapsed(&mut self, id: &str, collapsed: bool) -> bool {
        let Some(step) = self.snapshot.steps.iter_mut().find(|step| step.id == id) else {
            return false;
        };
        if !is_control_kind(&step.kind) {
            return false;
        }
        step.collapsed = collapsed;
        self.rebuild_dag();
        true
    }

    pub fn remove_step(&mut self, id: &str) -> bool {
        let before = self.snapshot.steps.len();
        let mut remove_ids = vec![id.to_string()];
        if self.snapshot.steps.iter().any(|step| step.id == id && is_control_kind(&step.kind)) {
            for step in &self.snapshot.steps {
                if step.slot.as_ref().is_some_and(|slot| slot.owner == id) {
                    remove_ids.push(step.id.clone());
                }
            }
        }
        self.snapshot.steps.retain(|step| !remove_ids.iter().any(|remove_id| remove_id == &step.id));
        self.snapshot.edges.retain(|edge| !remove_ids.iter().any(|remove_id| remove_id == &edge.from || remove_id == &edge.to));
        if self.snapshot.steps.len() == before {
            return false;
        }
        self.rebuild_dag();
        true
    }

    pub fn set_step_params_json(&mut self, id: &str, json: &str) -> Result<(), SequenceCoreError> {
        let params: StepParams = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| SequenceCoreError::Json(error.to_string()))?;
        let Some(step) = self.snapshot.steps.iter_mut().find(|step| step.id == id) else {
            return Err(SequenceCoreError::UnknownStep(id.into()));
        };
        step.params = params;
        self.rebuild_dag();
        Ok(())
    }

    pub fn connect_steps(&mut self, from_id: &str, to_id: &str) -> Result<String, SequenceCoreError> {
        if from_id == to_id {
            return Err(SequenceCoreError::SelfConnect);
        }
        let from_step = self.snapshot.steps.iter().find(|step| step.id == from_id).ok_or_else(|| SequenceCoreError::StepNotFound(from_id.into()))?;
        let to_step = self.snapshot.steps.iter().find(|step| step.id == to_id).ok_or_else(|| SequenceCoreError::StepNotFound(to_id.into()))?;
        if slot_key(from_step.slot.as_ref()) != slot_key(to_step.slot.as_ref()) {
            return Err(SequenceCoreError::MismatchedSlotScope);
        }
        let existing: Vec<(String, String)> = self.snapshot.edges.iter().map(|edge| (edge.from.clone(), edge.to.clone())).collect();
        if would_create_cycle(&existing, from_id, to_id) {
            return Err(SequenceCoreError::CycleDetected);
        }
        if self.snapshot.edges.iter().any(|edge| edge.from == from_id) {
            return Err(SequenceCoreError::OutgoingFlowExists(from_id.into()));
        }
        if self.snapshot.edges.iter().any(|edge| edge.to == to_id) {
            self.snapshot.edges.retain(|edge| edge.to != to_id);
        }
        let id = self.next_edge_id();
        self.snapshot.edges.push(SequenceEdge { id: id.clone(), from: from_id.into(), to: to_id.into() });
        self.rebuild_dag();
        Ok(id)
    }

    pub fn disconnect_steps(&mut self, from_id: &str, to_id: &str) -> bool {
        let before = self.snapshot.edges.len();
        self.snapshot.edges.retain(|edge| !(edge.from == from_id && edge.to == to_id));
        if self.snapshot.edges.len() == before {
            return false;
        }
        self.rebuild_dag();
        true
    }

    pub fn sync_edges_from_dag(&mut self) {
        let dag_pairs: Vec<(String, String)> = self
            .dag
            .host_snapshot
            .edges
            .iter()
            .filter_map(|dag_edge| {
                let from = dag_edge.source.split('@').next()?;
                let to = dag_edge.target.split('@').next()?;
                if from == to {
                    return None;
                }
                Some((from.into(), to.into()))
            })
            .collect();
        let mut edges = Vec::new();
        for (from, to) in dag_pairs {
            // 🧯️ `map_unwrap_or` — `map_or_else` would need to build the `&mut self`-capturing
            // fallback closure alongside the `self.snapshot.edges`-borrowing lookup in one call,
            // which the borrow checker rejects; the two-step form sequences the borrows correctly.
            #[allow(clippy::map_unwrap_or)]
            let id = self.snapshot.edges.iter().find(|edge| edge.from == from && edge.to == to).map(|edge| edge.id.clone()).unwrap_or_else(|| self.next_edge_id());
            edges.push(SequenceEdge { id, from, to });
        }
        self.snapshot.edges = edges;
    }

    pub fn sync_from_dag(&mut self) {
        self.camera = sequence_camera_from_dag(&self.dag.host_snapshot.camera);
        self.sync_edges_from_dag();
        for step in &mut self.snapshot.steps {
            let Some(node) = self.dag.host_snapshot.nodes.iter().find(|node| node.id == step.id) else {
                continue;
            };
            step.x = node.x;
            step.y = node.y;
        }
    }

    pub fn build_path(&self) -> Path {
        self.build_path_for_slot(None)
    }

    pub fn build_path_json(&self) -> Result<String, SequenceCoreError> {
        Ok(semio_framework_pack_json::to_json_string(&self.build_path()))
    }

    fn build_path_for_slot(&self, slot: Option<&SlotRef>) -> Path {
        let slot_filter = slot_key(slot);
        let scoped_steps: Vec<&SequenceStep> = self.snapshot.steps.iter().filter(|step| slot_key(step.slot.as_ref()) == slot_filter).collect();
        let incoming: HashMap<&str, &str> = self.snapshot.edges.iter().map(|edge| (edge.to.as_str(), edge.from.as_str())).collect();
        let outgoing: HashMap<&str, &str> = self.snapshot.edges.iter().map(|edge| (edge.from.as_str(), edge.to.as_str())).collect();
        let heads: Vec<&SequenceStep> = scoped_steps.iter().copied().filter(|step| !incoming.contains_key(step.id.as_str())).collect();
        let start = if heads.len() == 1 {
            heads[0].id.as_str()
        } else if scoped_steps.len() == 1 {
            scoped_steps[0].id.as_str()
        } else {
            return Path { steps: scoped_steps.iter().map(|step| self.step_to_imperative_step(step)).collect() };
        };
        let mut ordered = Vec::new();
        let mut by_id: BTreeMap<&str, &SequenceStep> = scoped_steps.into_iter().map(|step| (step.id.as_str(), step)).collect();
        let mut current = Some(start);
        let mut visited = std::collections::HashSet::new();
        while let Some(id) = current {
            if !visited.insert(id) {
                break;
            }
            if let Some(step) = by_id.remove(id) {
                ordered.push(self.step_to_imperative_step(step));
            }
            current = outgoing.get(id).copied();
        }
        for step in by_id.values() {
            ordered.push(self.step_to_imperative_step(step));
        }
        Path { steps: ordered }
    }

    fn step_to_imperative_step(&self, step: &SequenceStep) -> Step {
        let mut bodies = BTreeMap::new();
        if is_control_kind(&step.kind) {
            for slot_name in control_slots(&step.kind) {
                let slot_ref = SlotRef { owner: step.id.clone(), name: slot_name.to_string() };
                bodies.insert(slot_name.to_string(), self.build_path_for_slot(Some(&slot_ref)));
            }
        }
        Step { id: step.id.clone(), kind: step.kind.clone(), params: step.params.0.clone(), bodies }
    }

    fn is_step_visible(&self, step: &SequenceStep) -> bool {
        let Some(slot) = &step.slot else {
            return true;
        };
        let Some(owner) = self.snapshot.steps.iter().find(|entry| entry.id == slot.owner) else {
            return false;
        };
        !owner.collapsed
    }

    fn slot_member_count(&self, owner_id: &str) -> usize {
        self.snapshot.steps.iter().filter(|step| step.slot.as_ref().is_some_and(|slot| slot.owner == owner_id)).count()
    }

    pub fn layout_expanded_slots(&mut self) {
        let control_steps: Vec<(String, String, bool)> = self.snapshot.steps.iter().filter(|step| is_control_kind(&step.kind)).map(|step| (step.id.clone(), step.kind.clone(), step.collapsed)).collect();
        for (owner_id, kind, collapsed) in control_steps {
            if collapsed {
                continue;
            }
            let owner = self.snapshot.steps.iter().find(|step| step.id == owner_id);
            let Some(owner) = owner else { continue };
            let base_x = owner.x;
            let base_y = owner.y + 160.0;
            for (index, slot_name) in control_slots(&kind).iter().enumerate() {
                let slot_ref = SlotRef { owner: owner_id.clone(), name: (*slot_name).into() };
                let members: Vec<String> = self.snapshot.steps.iter().filter(|step| step.slot.as_ref() == Some(&slot_ref)).map(|step| step.id.clone()).collect();
                let offset_x = base_x + (index as f64 - (control_slots(&kind).len() as f64 - 1.0) * 0.5) * 320.0;
                for (member_index, member_id) in members.iter().enumerate() {
                    if let Some(step) = self.snapshot.steps.iter_mut().find(|step| step.id == *member_id) {
                        step.x = offset_x + member_index as f64 * 280.0;
                        step.y = base_y;
                    }
                }
            }
        }
        self.rebuild_dag();
    }

    /// 🌳️ Recomputes visible step positions using the shared layered DAG tree layout, then rebuilds the DAG view.
    pub fn reorganize(&mut self, opts: &DagLayoutOptions) -> Result<(), SequenceCoreError> {
        self.dag.reorganize(opts).map_err(|e| SequenceCoreError::Dag(e.to_string()))?;
        let positions: HashMap<String, (f64, f64)> = self.dag.host_snapshot.nodes.iter().map(|node| (node.id.clone(), (node.x, node.y))).collect();
        for step in self.snapshot.steps.iter_mut() {
            if let Some(&(x, y)) = positions.get(&step.id) {
                step.x = x;
                step.y = y;
            }
        }
        self.rebuild_dag();
        Ok(())
    }

    pub fn run(&self) -> RunResult {
        Executor::new(&self.registry).run(&self.build_path(), &Dictionary::new())
    }

    pub fn compile_text(&self) -> String {
        imperative_compile_to_text(&self.build_path())
    }

    /// 📝️ Renders the compiled DAG snapshot as wire-literal text.
    pub fn compiled_wire_literal(&self) -> String {
        dag_host_snapshot_to_wire_literal(&self.build_dag_host_snapshot())
    }

    fn rebuild_dag(&mut self) {
        let selected = self.dag.selected_node_ids();
        let dag_fixture = self.build_dag_host_snapshot();
        self.dag = DagHost::from_host_snapshot_without_layout(dag_fixture);
        self.dag.set_camera(self.camera.x, self.camera.y, self.camera.zoom);
        if !selected.is_empty() {
            self.dag.set_selection(&selected);
        }
    }

    fn build_dag_host_snapshot(&self) -> DagHostSnapshot {
        let nodes: Vec<DagNodeSpec> = self.snapshot.steps.iter().filter(|step| self.is_step_visible(step)).map(|step| self.step_to_dag_node(step)).collect();
        let visible_ids: std::collections::HashSet<String> = nodes.iter().map(|node| node.id.clone()).collect();
        let existing: Vec<(String, String)> = self.snapshot.edges.iter().map(|edge| (edge.from.clone(), edge.to.clone())).collect();
        let edges: Vec<DagHostSnapshotEdge> = self
            .snapshot
            .edges
            .iter()
            .filter(|edge| visible_ids.contains(&edge.from) && visible_ids.contains(&edge.to))
            .filter(|edge| !would_create_cycle(&existing, &edge.from, &edge.to))
            .map(|edge| DagHostSnapshotEdge { id: edge.id.clone(), source: format!("{}@{}", edge.from, FLOW_OUTPUT_PORT), target: format!("{}@{}", edge.to, FLOW_INPUT_PORT), route_style: EdgeRouteStyle::SharpSz, properties: PropertyBag::new() })
            .collect();
        DagHostSnapshot { schema: "dag.host_snapshot".into(), camera: dag_camera_from_sequence(&self.camera), nodes, edges }
    }

    fn step_to_dag_node(&self, step: &SequenceStep) -> DagNodeSpec {
        let info = self.registry.operator_info(&step.kind);
        let (name, mut abbreviation, icon) = info.as_ref().map_or_else(|| (step.kind.clone(), step.kind.clone(), "emoji:⚡️".into()), |entry| (entry.name.clone(), entry.abbreviation.clone(), entry.icon.clone()));
        if is_control_kind(&step.kind) {
            let count = self.slot_member_count(&step.id);
            abbreviation = if step.collapsed { format!("▸️ {count}") } else { format!("▾️ {count}") };
        }
        // 🛡️ falls back to execution-only ports for a function-kind step whose kind isn't (yet) registered,
        // rather than assuming the registry always resolves it — matches the non-function-kind fallback below.
        let (inputs, outputs) = match info.filter(|_| is_function_kind(&step.kind)) {
            Some(info) => {
                let mut inputs: Vec<IoPortSpec> = info.inputs.iter().map(|spec| input_spec_to_port(spec, &step.params)).collect();
                let mut outputs: Vec<IoPortSpec> = info.outputs.iter().map(channel_spec_to_output_port).collect();
                if outputs.is_empty() {
                    outputs.push(channel_spec_to_output_port(&ChannelSpec::wildcard()));
                }
                inputs.push(hidden_flow_input_port());
                outputs.push(hidden_flow_output_port());
                (inputs, outputs)
            }
            None => (vec![visible_flow_input_port()], vec![visible_flow_output_port()]),
        };
        let width = sequence_computation_node_width(&name, &inputs, &outputs);
        let height = sequence_computation_node_height(inputs.len(), outputs.len(), false, false);
        let mut node = DagNodeSpec::computation(step.id.clone(), &name, &abbreviation, icon, inputs, outputs, false, false, step.x, step.y, width, height);
        node.operator_kind = Some(step.kind.clone());
        node.properties = property_bag_from_dictionary(&step.params);
        node
    }

    pub fn set_ghost_step(&mut self, kind: &str, x: f64, y: f64) {
        let ghost = SequenceStep { id: "__ghost__".into(), kind: kind.into(), params: StepParams::new(), x, y, slot: None, collapsed: false };
        let node = self.step_to_dag_node(&ghost);
        self.dag.set_ghost_node(Some(node));
    }

    pub fn clear_ghost_step(&mut self) {
        self.dag.set_ghost_node(None);
    }
}
//#endregion 🔖️Host

//#region 🔖️HostHelpers
/// 🧸️ Builds a host from an already resolved typed host snapshot projection.
pub fn host_from_host_snapshot(host_snapshot: &SequenceHostSnapshot) -> SequenceHost {
    SequenceHost::from_host_snapshot(host_snapshot.clone())
}

/// 🧊️ Retires one synchronous execution result through its domain-owned dictionaries.
pub fn retire_run_result_cold(result: RunResult) {
    let RunResult { scope, effects } = result;
    neural_engine::ColdRetire::retire_cold(scope);
    for effect in effects {
        neural_engine::ColdRetire::retire_cold(effect.input);
        if let Some(output) = effect.output {
            neural_engine::ColdRetire::retire_cold(output);
        }
    }
}

//#endregion 🔖️HostHelpers

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `SequencePlayApp::Command` — the SOLE dispatch surface for sequence's own behavior,
    /// assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword
    /// (the kebab-case `#[dsl(key = ..)]` the codec uses) — every row's wire keyword happens to be the
    /// plain kebab-case of its id (no `flow`-style divergence here), but the two are still copied
    /// independently from the pre-migration `sequence_protocol` enum's `command_id()` match arm and
    /// `#[dsl(key = ..)]` attribute respectively, never derived one from the other. **Row order is the
    /// binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum SequenceCommand for SequenceSnapshot, SequenceMutation, NoConfig, NoConfigMutation {
        "addStep" as "add-step" => add_step::AddStep,
        "addStepToSlot" as "add-step-to-slot" => add_step_to_slot::AddStepToSlot,
        "addStepDropped" as "add-step-dropped" => add_step_dropped::AddStepDropped,
        "removeStep" as "remove-step" => remove_step::RemoveStep,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "moveStep" as "move-step" => move_step::MoveStep,
        "connectSteps" as "connect-steps" => connect_steps::ConnectSteps,
        "disconnectSteps" as "disconnect-steps" => disconnect_steps::DisconnectSteps,
        "setStepParams" as "set-step-params" => set_step_params::SetStepParams,
        "setStepCollapsed" as "set-step-collapsed" => set_step_collapsed::SetStepCollapsed,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "setOrientation" as "set-orientation" => set_orientation::SetOrientation,
        "run" as "run" => run_command::Run,
        "stop" as "stop" => stop_command::Stop,
        "setViewport" as "set-viewport" => set_viewport::SetViewport,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
    }
}
//#endregion 🔖️Commands

//#region 📏️RetainedCaps
const SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS: usize = 256;
const SEQUENCE_STORE_MAXIMUM_BYTES: usize = 65_536;

fn sequence_bounded_child_emit_bytes(child_emits: &[ChildEmit], maximum_bytes: usize) -> Result<usize, String> {
    let mut retained_bytes = 0usize;
    let mut admit = |bytes: usize| {
        retained_bytes = retained_bytes.checked_add(bytes).ok_or_else(|| "Sequence child publication byte count overflowed".to_string())?;
        if retained_bytes > maximum_bytes {
            return Err("Sequence child publication exceeds its fixed retained envelope".to_string());
        }
        Ok(())
    };
    for child in child_emits {
        admit(child.slot.len())?;
        admit(child.child_id.len())?;
        admit(child.op_schema.0.len())?;
        for label in &child.labels {
            admit(label.retained_bytes())?;
        }
        for op in &child.ops {
            admit(op.len())?;
        }
    }
    Ok(retained_bytes)
}
//#endregion 📏️RetainedCaps


//#region 🧵️RetainedArtifactRoutes
const SEQUENCE_RETAINED_ARTIFACT_PAYLOAD_SCHEMA: &str = "sequence.play/retained-artifact-command.v1";
const SEQUENCE_RETAINED_ARTIFACT_TOOL_IDS: &[&str] = &["addStep", "addStepToSlot", "addStepDropped", "removeStep", "deleteSelection", "moveStep", "connectSteps", "disconnectSteps", "setStepParams", "setStepCollapsed"];
const SEQUENCE_RETAINED_ARTIFACT_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addStepToSlot", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addStepDropped", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "connectSteps", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "disconnectSteps", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setStepParams", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setStepCollapsed", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
];

fn sequence_retained_id_admitted(value: &str) -> bool {
    !value.is_empty() && value.len() <= 128
}

fn sequence_retained_artifact_command_admitted(command: &SequenceCommand) -> bool {
    match command {
        SequenceCommand::AddStep(payload) => sequence_retained_id_admitted(&payload.kind) && payload.x.is_finite() && payload.y.is_finite(),
        SequenceCommand::AddStepToSlot(payload) => sequence_retained_id_admitted(&payload.kind) && sequence_retained_id_admitted(&payload.owner) && sequence_retained_id_admitted(&payload.slot_name) && payload.x.is_finite() && payload.y.is_finite(),
        SequenceCommand::AddStepDropped(payload) => sequence_retained_id_admitted(&payload.kind) && payload.picked_step_id.as_deref().is_none_or(sequence_retained_id_admitted) && payload.x.is_finite() && payload.y.is_finite(),
        SequenceCommand::RemoveStep(payload) => sequence_retained_id_admitted(&payload.id),
        SequenceCommand::DeleteSelection(_) => true,
        SequenceCommand::MoveStep(payload) => sequence_retained_id_admitted(&payload.node_id) && payload.x.is_finite() && payload.y.is_finite(),
        SequenceCommand::ConnectSteps(payload) => sequence_retained_id_admitted(&payload.source_node_id) && sequence_retained_id_admitted(&payload.target_node_id),
        SequenceCommand::DisconnectSteps(payload) => sequence_retained_id_admitted(&payload.from_id) && sequence_retained_id_admitted(&payload.to_id),
        SequenceCommand::SetStepParams(payload) => sequence_retained_id_admitted(&payload.id) && payload.params_json.len() <= SEQUENCE_RETAINED_RAW_BYTES,
        SequenceCommand::SetStepCollapsed(payload) => sequence_retained_id_admitted(&payload.id),
        _ => false,
    }
}

fn sequence_retained_serial(prefix: &str, id: &str) -> Option<u64> {
    id.strip_prefix(prefix)?.parse().ok()
}

fn sequence_retained_next_id(scene: &SequenceWorkingScene, prefix: &str) -> String {
    let step_max = scene.steps.iter().filter_map(|step| sequence_retained_serial("step-", &step.id)).max().unwrap_or(0);
    let edge_max = scene.edges.iter().filter_map(|edge| sequence_retained_serial("edge-", &edge.id)).max().unwrap_or(0);
    format!("{prefix}-{}", step_max.max(edge_max).max(100).saturating_add(1))
}

fn sequence_retained_is_control(kind: &str) -> bool {
    matches!(kind, "control.if" | "control.while" | "control.repeat")
}

fn sequence_retained_default_slot(kind: &str) -> &'static str {
    if kind == "control.if" {
        "then"
    } else {
        "body"
    }
}

fn sequence_retained_create_step(scene: &SequenceWorkingScene, kind: String, x: f64, y: f64, slot: Option<SlotRef>) -> SequenceStep {
    SequenceStep { id: sequence_retained_next_id(scene, "step"), kind, params: StepParams::new(), x, y, slot, collapsed: false }
}

fn sequence_retained_remove(target: &mut SequenceWorkingScene, ids: &[String]) {
    target.steps.retain(|step| !ids.contains(&step.id));
    target.edges.retain(|edge| !ids.contains(&edge.from) && !ids.contains(&edge.to));
}

fn sequence_retained_delete_ids(scene: &SequenceWorkingScene, roots: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut selected = Vec::new();
    for root in roots {
        if !selected.iter().any(|id| id == &root) && scene.steps.iter().any(|step| step.id == root) {
            selected.push(root.clone());
        }
        if scene.steps.iter().any(|step| step.id == root && sequence_retained_is_control(&step.kind)) {
            for step in &scene.steps {
                if step.slot.as_ref().is_some_and(|slot| slot.owner == root) && !selected.iter().any(|id| id == &step.id) {
                    selected.push(step.id.clone());
                }
            }
        }
    }
    scene.steps.iter().filter(|step| selected.iter().any(|id| id == &step.id)).map(|step| step.id.clone()).collect()
}

fn sequence_retained_artifact_emit(command: &SequenceCommand, snapshot: &SequenceSnapshot, scene: &SequenceWorkingScene, interaction: &protocol::InteractionState) -> Result<(Emit<SequenceMutation, NoConfigMutation>, Option<Dictionary>), Fault> {
    if scene.steps.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS || scene.edges.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
        return Err(sequence_fault("sequence.editor.capacity", "sequence-retained-scene-capacity"));
    }
    let mut discarded_params = None;
    let mut target = scene.clone();
    match command {
        SequenceCommand::AddStep(payload) => target.steps.push(sequence_retained_create_step(scene, payload.kind.clone(), payload.x, payload.y, None)),
        SequenceCommand::AddStepToSlot(payload) => target.steps.push(sequence_retained_create_step(scene, payload.kind.clone(), payload.x, payload.y, Some(SlotRef { owner: payload.owner.clone(), name: payload.slot_name.clone() }))),
        SequenceCommand::AddStepDropped(payload) => {
            let slot = payload.picked_step_id.as_ref().and_then(|owner_id| {
                scene.steps.iter().find(|step| step.id == *owner_id && sequence_retained_is_control(&step.kind) && !step.collapsed).map(|owner| SlotRef { owner: owner_id.clone(), name: sequence_retained_default_slot(&owner.kind).into() })
            });
            target.steps.push(sequence_retained_create_step(scene, payload.kind.clone(), payload.x, payload.y, slot));
        }
        SequenceCommand::RemoveStep(payload) => sequence_retained_remove(&mut target, &sequence_retained_delete_ids(scene, [payload.id.clone()])),
        SequenceCommand::DeleteSelection(_) => {
            let selected = interaction.selection.get(SEQUENCE_INTERACTION_STEPS).map(|selection| selection.ids.clone()).unwrap_or_default();
            if selected.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                return Err(sequence_fault("sequence.editor.capacity", "sequence-retained-selection-capacity"));
            }
            sequence_retained_remove(&mut target, &sequence_retained_delete_ids(scene, selected));
        }
        SequenceCommand::MoveStep(payload) => {
            if let Some(step) = target.steps.iter_mut().find(|step| step.id == payload.node_id) {
                step.x = payload.x;
                step.y = payload.y;
            }
        }
        SequenceCommand::SetStepParams(payload) => {
            let params = semio_framework_pack_json::from_json_str::<StepParams>(&payload.params_json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok();
            match (target.steps.iter_mut().find(|step| step.id == payload.id), params) {
                (Some(step), Some(params)) if step.params != params => step.params = params,
                (_, Some(mut params)) => discarded_params = Some(std::mem::take(&mut params.0)),
                (_, None) => {}
            }
        }
        SequenceCommand::SetStepCollapsed(payload) => {
            if let Some(step) = target.steps.iter_mut().find(|step| step.id == payload.id && sequence_retained_is_control(&step.kind)) {
                step.collapsed = !step.collapsed;
            }
        }
        SequenceCommand::DisconnectSteps(payload) => target.edges.retain(|edge| edge.from != payload.from_id || edge.to != payload.to_id),
        SequenceCommand::ConnectSteps(payload) => {
            let from = scene.steps.iter().find(|step| step.id == payload.source_node_id);
            let to = scene.steps.iter().find(|step| step.id == payload.target_node_id);
            let same_slot = from.zip(to).is_some_and(|(from, to)| from.slot.as_ref().map(|slot| (&slot.owner, &slot.name)) == to.slot.as_ref().map(|slot| (&slot.owner, &slot.name)));
            let existing: Vec<(String, String)> = scene.edges.iter().map(|edge| (edge.from.clone(), edge.to.clone())).collect();
            if payload.source_node_id != payload.target_node_id && same_slot && !would_create_cycle(&existing, &payload.source_node_id, &payload.target_node_id) && !scene.edges.iter().any(|edge| edge.from == payload.source_node_id) {
                target.edges.retain(|edge| edge.to != payload.target_node_id);
                target.edges.push(SequenceEdge { id: sequence_retained_next_id(scene, "edge"), from: payload.source_node_id.clone(), to: payload.target_node_id.clone() });
            }
        }
        _ => return Err(sequence_fault("app.command.tool-mismatch", "sequence-retained-artifact-route-mismatch")),
    }
    let leaves = sequence_scene_leaves(scene, &target);
    if leaves.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
        return Err(sequence_fault("sequence.editor.capacity", "sequence-retained-artifact-output-capacity"));
    }
    Ok((sequence_child_leaves_emit(snapshot, leaves), discarded_params))
}

#[derive(Default)]
struct SequenceRetainedSceneOwner {
    scene: Option<SequenceWorkingScene>,
    retirement: ValueRetirement,
}

impl SequenceRetainedSceneOwner {
    fn capture(&mut self, snapshot: &SequenceSnapshot, children: &semio_framework_plugin::app::ChildContentView) -> Result<(), Fault> {
        if self.scene.is_none() {
            self.scene = Some(sequence_working_scene_from_children(snapshot, children)?.into_inner());
        }
        Ok(())
    }

    fn scene(&self) -> Result<&SequenceWorkingScene, Fault> {
        self.scene.as_ref().ok_or_else(|| sequence_fault("sequence.content.unavailable", "sequence-retained-scene-required"))
    }

    fn release_one(&mut self, maximum_bytes: usize) -> SequencePersistentRelease {
        if !self.retirement.terminal_is_empty() {
            return match self.retirement.close_step(1, maximum_bytes) {
                ValueRetirementStep::Pending { released_bytes, .. } => SequencePersistentRelease::Progress(released_bytes),
                ValueRetirementStep::Complete if self.retirement.terminal_is_empty() => SequencePersistentRelease::Progress(0),
                ValueRetirementStep::Complete | ValueRetirementStep::Blocked => SequencePersistentRelease::Blocked,
            };
        }
        if let Some(scene) = self.scene.as_mut() {
            if let Some(step) = scene.steps.pop() {
                let SequenceStep { mut params, .. } = step;
                self.retirement.push_dictionary(std::mem::take(&mut params.0));
                return SequencePersistentRelease::Progress(0);
            }
            if scene.edges.pop().is_some() {
                return SequencePersistentRelease::Progress(0);
            }
        }
        if self.scene.take().is_some() {
            return SequencePersistentRelease::Progress(0);
        }
        SequencePersistentRelease::Complete
    }

    fn empty(&self) -> bool {
        self.scene.is_none() && self.retirement.terminal_is_empty()
    }
}

struct SequenceRetainedArtifactWork {
    tool_id: &'static str,
    workspace_identity: u64,
    cursor: usize,
    replay_target: Option<usize>,
    scene_owner: SequenceRetainedSceneOwner,
    completed: bool,
    closing: bool,
}

impl SequenceRetainedArtifactWork {
    fn new(tool_id: &'static str, operation: &semio_framework_plugin::AppOperationContext) -> Self {
        let scope = format!("{}:{}:{}:{}", operation.app_instance_id, operation.parent_document_id, operation.operation_id, operation.generation);
        let workspace_identity = scope.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325_u64, |state, byte| (state ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3));
        Self { tool_id, workspace_identity, cursor: 0, replay_target: None, scene_owner: SequenceRetainedSceneOwner::default(), completed: false, closing: false }
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<SequencePlayApp>> for SequenceRetainedArtifactWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }
    fn workspace_identity(&self) -> u64 {
        self.workspace_identity
    }
    fn extent(
        &self,
        _command: &SequenceCommand,
        snapshot: &SequenceSnapshot,
        interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<SequencePlayApp>>>,
    ) -> Option<usize> {
        let scene = sequence_working_scene_from_children(snapshot, context?.children.as_ref()).ok()?;
        (scene.steps.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS
            && scene.edges.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS
            && interaction.selection.get(SEQUENCE_INTERACTION_STEPS).is_none_or(|selection| selection.ids.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS))
        .then_some(SEQUENCE_RETAINED_MAXIMUM_UNITS)
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<SequencePlayApp>>,
    _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<SequencePlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config: _config, history: _history, interaction, hover: _hover, context, operation: _operation } = *input;
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || self.cursor >= SEQUENCE_RETAINED_MAXIMUM_UNITS || !sequence_retained_artifact_command_admitted(command) {
            return Err(sequence_fault("sequence.retained.artifact-command", "sequence-retained-artifact-envelope"));
        }
        self.cursor += 1;
        if let Some(target) = self.replay_target {
            if self.cursor <= target {
                if self.cursor == target {
                    self.replay_target = None;
                }
                return Ok(ArtifactCommandWorkStep::Replay { stage: "sequence-artifact-replay", preview: b"{\"en\":\"Restoring Sequence edit\",\"de\":\"Sequenzbearbeitung wird wiederhergestellt\"}" });
            }
        }
        if self.cursor == 1 {
            return Ok(ArtifactCommandWorkStep::Progress { stage: "sequence-artifact-prepare", preview: b"{\"en\":\"Preparing Sequence edit\",\"de\":\"Sequenzbearbeitung wird vorbereitet\"}" });
        }
        let context = context.ok_or_else(|| sequence_fault("sequence.content.unavailable", "sequence-content-child-context-required"))?;
        self.scene_owner.capture(snapshot, context.children.as_ref())?;
        let scene = self.scene_owner.scene()?;
        let (emit, discarded_params) = sequence_retained_artifact_emit(command, snapshot, scene, interaction)?;
        if let Some(params) = discarded_params {
            self.scene_owner.retirement.push_dictionary(params);
        }
        let exact_child = emit.child_emits.first().is_none_or(|child| child.slot == "content" && child.child_id == snapshot.content.child_id);
        if !emit.config_mutations.is_empty() || !emit.draft_mutations.is_empty() || !emit.artifact_mutations.is_empty() || emit.child_emits.len() > 1 || !exact_child {
            return Err(sequence_fault("sequence.publication.lane", "sequence-retained-artifact-publication-lane"));
        }
        sequence_bounded_child_emit_bytes(&emit.child_emits, SEQUENCE_STORE_MAXIMUM_BYTES).map_err(|_| sequence_fault("sequence.editor.capacity", "sequence-retained-artifact-output-bytes"))?;
        self.completed = true;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 24 {
            return Err(sequence_fault("sequence.editor.capacity", "sequence-retained-artifact-checkpoint-capacity"));
        }
        target[..24].fill(0);
        target[..4].copy_from_slice(b"SRA1");
        target[4] = u8::from(self.completed);
        target[8..16].copy_from_slice(&(self.cursor as u64).to_le_bytes());
        target[16..24].copy_from_slice(&self.workspace_identity.to_le_bytes());
        Ok(24)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 24 || &checkpoint[..4] != b"SRA1" || checkpoint[4] > 1 || checkpoint[5..8] != [0, 0, 0] {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-retained-artifact-checkpoint-invalid"));
        }
        let cursor = usize::try_from(u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-artifact-checkpoint-cursor"))?)).map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-artifact-checkpoint-cursor"))?;
        let identity = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-artifact-checkpoint-identity"))?);
        if identity != self.workspace_identity || cursor > SEQUENCE_RETAINED_MAXIMUM_UNITS || !self.scene_owner.empty() {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-retained-artifact-checkpoint-owner-mismatch"));
        }
        self.cursor = 0;
        self.replay_target = (cursor != 0).then_some(cursor);
        self.completed = false;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.replay_target = None;
        match self.scene_owner.release_one(maximum_bytes) {
            SequencePersistentRelease::Progress(released_bytes) => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes },
            SequencePersistentRelease::Blocked => semio_framework_job::InteractiveJobCloseStep::Blocked,
            SequencePersistentRelease::Complete => semio_framework_job::InteractiveJobCloseStep::Complete,
        }
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.replay_target.is_none() && self.scene_owner.empty()
    }
}

struct SequenceRetainedArtifactJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}
impl SequenceRetainedArtifactJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: SEQUENCE_RETAINED_ARTIFACT_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}
impl semio_framework::ToolJobFactory for SequenceRetainedArtifactJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        SEQUENCE_RETAINED_ARTIFACT_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(SEQUENCE_RETAINED_RAW_BYTES, SEQUENCE_RETAINED_MAXIMUM_UNITS, 1, SEQUENCE_STORE_MAXIMUM_BYTES, 2_000, 1, 1)
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SEQUENCE_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((semio_framework::ToolJobFactoryError::new("Sequence retained artifact command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}
impl semio_framework_plugin::ArtifactOwnedToolJobFactory for SequenceRetainedArtifactJobFactory {
    type Owner = semio_framework_plugin::EditorApp<SequencePlayApp>;
    const TOOL_IDS: &'static [&'static str] = SEQUENCE_RETAINED_ARTIFACT_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SEQUENCE_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = SEQUENCE_RETAINED_ARTIFACT_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedArtifactRoutes

//#region 🧵️PersistentRemainingRoutes
const SEQUENCE_PERSISTENT_MAXIMUM_UNITS: usize = 66_049;
const SEQUENCE_PERSISTENT_TOOL_IDS: &[&str] = &["reorganize", "nodeGraphEdit", "run"];
const SEQUENCE_PERSISTENT_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "run", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
];

enum SequencePersistentAdvance {
    Progress(&'static str, &'static [u8]),
    Complete(Vec<SemioFlowMutation>),
    CompleteRun(String),
}

#[derive(Default)]
struct SequenceReorganizeState {
    initialized: usize,
    pass: usize,
    edge: usize,
    emit: usize,
    depths: Vec<usize>,
    moves: Vec<(usize, f64, f64)>,
}

impl SequenceReorganizeState {
    fn advance(&mut self, scene: &SequenceWorkingScene, config: &SequenceMainWindowConfig) -> Result<SequencePersistentAdvance, Fault> {
        let node_count = scene.steps.len();
        let edge_count = scene.edges.len();
        if node_count > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS || edge_count > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
            return Err(sequence_fault("sequence.editor.capacity", "sequence-reorganize-capacity"));
        }
        if self.initialized < node_count {
            self.depths.push(0);
            self.initialized += 1;
            return Ok(SequencePersistentAdvance::Progress("sequence-reorganize-nodes", b"{\"en\":\"Indexing layout node\",\"de\":\"Layoutknoten wird indiziert\"}"));
        }
        if self.pass < node_count && edge_count != 0 {
            let edge = &scene.edges[self.edge];
            let from = scene.steps.iter().position(|step| step.id == edge.from);
            let to = scene.steps.iter().position(|step| step.id == edge.to);
            if let (Some(from), Some(to)) = (from, to) {
                self.depths[to] = self.depths[to].max(self.depths[from].saturating_add(1).min(node_count));
            }
            self.edge += 1;
            if self.edge == edge_count {
                self.edge = 0;
                self.pass += 1;
            }
            return Ok(SequencePersistentAdvance::Progress("sequence-reorganize-edges", b"{\"en\":\"Relaxing layout edge\",\"de\":\"Layoutkante wird verarbeitet\"}"));
        }
        if self.emit < node_count {
            let index = self.emit;
            let step = &scene.steps[index];
            let primary = self.depths[index] as f64 * 280.0;
            let secondary = self.depths[..index].iter().filter(|depth| **depth == self.depths[index]).count() as f64 * 160.0;
            let (x, y) = if config.orientation == "topBottom" { (secondary, primary) } else { (primary, secondary) };
            if step.x != x || step.y != y {
                self.moves.push((index, x, y));
            }
            self.emit += 1;
            return Ok(SequencePersistentAdvance::Progress("sequence-reorganize-publish-plan", b"{\"en\":\"Planning node position\",\"de\":\"Knotenposition wird geplant\"}"));
        }
        let mut target = scene.clone();
        for (index, x, y) in self.moves.drain(..) {
            let step = &mut target.steps[index];
            step.x = x;
            step.y = y;
        }
        Ok(SequencePersistentAdvance::Complete(sequence_scene_leaves(scene, &target)))
    }

    fn release_one(&mut self) -> bool {
        self.depths.pop().is_some() || self.moves.pop().is_some()
    }

    fn empty(&self) -> bool {
        self.depths.is_empty() && self.moves.is_empty()
    }
}

#[derive(Clone, Copy, Default)]
enum SequenceNodeGraphStage {
    #[default]
    Parse,
    Apply,
    DeleteDiscover,
    DeleteApply,
    Complete,
}

#[derive(Default)]
struct SequenceNodeGraphState {
    stage: SequenceNodeGraphStage,
    operations: Vec<NodeGraphEditRow>,
    operation: usize,
    base: Option<SequenceWorkingScene>,
    target: Option<SequenceWorkingScene>,
    gesture: Option<String>,
    delete_frontier: VecDeque<String>,
    delete_current: Option<String>,
    delete_scan: usize,
    delete_discovered: Vec<String>,
    discarded_steps: VecDeque<SequenceStep>,
    retirement: ValueRetirement,
}

impl SequenceNodeGraphState {
    /// ✋️ Publishes the batch's child leaves (design §12, §13.3): a batch that moved steps is a drag, committed as ONE
    /// composed-child tool transaction through the ONE node-drag machine of `🛠️tool-machine` from the admission's authoring
    /// seed (a view without command authority publishes the leaves plainly); every other batch is one child edit.
    fn publish(&mut self, snapshot: &SequenceSnapshot, leaves: Vec<SemioFlowMutation>, authoring_seed: &str) -> Emit<SequenceMutation, NoConfigMutation> {
        match self.gesture.take() {
            Some(gesture) => match node_drag_emit(SEQUENCE_PLAY_APP_ID, node_graph_edit::NODE_GRAPH_EDIT_VERB, authoring_seed, &gesture, leaves) {
                NodeDragEmit::Nothing => Emit::default(),
                drag => Emit { ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Emit::node_drag_child::<SemioFlowSnapshot, _>(drag, "content", &snapshot.content.child_id) },
            },
            None => sequence_child_leaves_emit(snapshot, leaves),
        }
    }

    fn advance(&mut self, command: &SequenceCommand, scene: &SequenceWorkingScene) -> Result<SequencePersistentAdvance, Fault> {
        match self.stage {
            SequenceNodeGraphStage::Parse => {
                let SequenceCommand::NodeGraphEdit(payload) = command else {
                    return Err(sequence_fault("app.command.tool-mismatch", "sequence-node-graph-route"));
                };
                if payload.operations_json.len() > SEQUENCE_RETAINED_RAW_BYTES {
                    return Err(sequence_fault("sequence.node-graph.malformed", "sequence-node-graph-bytes"));
                }
                let Ok(Value::Array(operations)) = json::parse(&payload.operations_json, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
                    return Err(sequence_fault("sequence.node-graph.malformed", "sequence-node-graph-json"));
                };
                self.operations = operations.iter().map(|row| node_graph_edit::sequence_node_graph_row(&json::to_dsl_value(row))).collect::<Result<_, _>>()?;
                if self.operations.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                    return Err(sequence_fault("sequence.node-graph.malformed", "sequence-node-graph-items"));
                }
                self.base = Some(scene.clone());
                self.target = Some(scene.clone());
                self.stage = SequenceNodeGraphStage::Apply;
                Ok(SequencePersistentAdvance::Progress("sequence-node-graph-parse", b"{\"en\":\"Decoded graph edit\",\"de\":\"Graphbearbeitung wurde dekodiert\"}"))
            }
            SequenceNodeGraphStage::Apply if self.operation < self.operations.len() => {
                let operation = &self.operations[self.operation];
                let target = self.target.as_mut().ok_or_else(|| sequence_fault("sequence.content.unavailable", "sequence-node-graph-target"))?;
                match operation {
                    NodeGraphEditRow::Move(record) => {
                        let mut moved = false;
                        for step in target.steps.iter_mut().filter(|step| record.node_ids.contains(&step.id)) {
                            step.x += record.dx;
                            step.y += record.dy;
                            moved = true;
                        }
                        if moved && record.moves() {
                            self.gesture.get_or_insert_with(|| record.gesture_id.clone());
                        }
                    }
                    NodeGraphEditRow::Connect { source_node_id: from, target_node_id: to, .. } => {
                        let existing: Vec<(String, String)> = target.edges.iter().map(|edge| (edge.from.clone(), edge.to.clone())).collect();
                        if from != to && target.steps.iter().any(|step| &step.id == from) && target.steps.iter().any(|step| &step.id == to) && !would_create_cycle(&existing, from, to) && !target.edges.iter().any(|edge| &edge.from == from) {
                            target.edges.retain(|edge| &edge.to != to);
                            target.edges.push(SequenceEdge { id: sequence_retained_next_id(target, "edge"), from: from.clone(), to: to.clone() });
                        }
                    }
                    NodeGraphEditRow::Disconnect { synapse_id } => target.edges.retain(|edge| &edge.id != synapse_id),
                    NodeGraphEditRow::Delete { node_ids, synapse_ids } => {
                        if node_ids.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                            return Err(sequence_fault("sequence.editor.capacity", "sequence-node-graph-delete-capacity"));
                        }
                        target.edges.retain(|edge| !synapse_ids.contains(&edge.id));
                        self.delete_frontier = node_ids.iter().cloned().collect();
                        self.stage = SequenceNodeGraphStage::DeleteDiscover;
                        return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-delete", b"{\"en\":\"Preparing bounded graph removal\",\"de\":\"Begrenzte Graphentfernung wird vorbereitet\"}"));
                    }
                    NodeGraphEditRow::SetSlider { .. } | NodeGraphEditRow::InsertPort { .. } => return Err(sequence_fault("sequence.node-graph.unsupported", "sequence-node-graph-row-unsupported")),
                }
                self.operation += 1;
                Ok(SequencePersistentAdvance::Progress("sequence-node-graph-operation", b"{\"en\":\"Applying graph operation\",\"de\":\"Graphoperation wird angewendet\"}"))
            }
            SequenceNodeGraphStage::DeleteDiscover => {
                if self.delete_current.is_none() {
                    if let Some(id) = self.delete_frontier.pop_front() {
                        if !self.delete_discovered.contains(&id) {
                            self.delete_discovered.push(id.clone());
                            self.delete_current = Some(id);
                            self.delete_scan = 0;
                        }
                        return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-root", "{\"en\":\"Traversing one selected graph root\",\"de\":\"Eine ausgewählte Graphwurzel wird durchlaufen\"}".as_bytes()));
                    }
                    self.stage = SequenceNodeGraphStage::DeleteApply;
                    self.delete_scan = 0;
                    return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-apply", "{\"en\":\"Preparing selected graph removal\",\"de\":\"Ausgewählte Graphentfernung wird vorbereitet\"}".as_bytes()));
                }
                let target = self.target.as_ref().ok_or_else(|| sequence_fault("sequence.content.unavailable", "sequence-node-graph-target"))?;
                if self.delete_scan < target.steps.len() {
                    let step = &target.steps[self.delete_scan];
                    if step.slot.as_ref().is_some_and(|slot| self.delete_current.as_ref().is_some_and(|id| slot.owner == *id)) && !self.delete_discovered.contains(&step.id) && !self.delete_frontier.contains(&step.id) {
                        self.delete_frontier.push_back(step.id.clone());
                    }
                    self.delete_scan += 1;
                    return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-child", b"{\"en\":\"Traversing one nested graph step\",\"de\":\"Ein verschachtelter Graphschritt wird durchlaufen\"}"));
                }
                self.delete_current = None;
                self.delete_scan = 0;
                Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-next", "{\"en\":\"Advancing selected graph traversal\",\"de\":\"Ausgewählter Graphdurchlauf wird fortgesetzt\"}".as_bytes()))
            }
            SequenceNodeGraphStage::DeleteApply => {
                if let Some(id) = self.delete_discovered.pop() {
                    let target = self.target.as_mut().ok_or_else(|| sequence_fault("sequence.content.unavailable", "sequence-node-graph-target"))?;
                    if let Some(index) = target.steps.iter().position(|step| step.id == id) {
                        self.discarded_steps.push_back(target.steps.remove(index));
                    }
                    target.edges.retain(|edge| edge.from != id && edge.to != id);
                    return Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-delete", "{\"en\":\"Removing one selected graph step\",\"de\":\"Ein ausgewählter Graphschritt wird entfernt\"}".as_bytes()));
                }
                self.operation += 1;
                self.stage = SequenceNodeGraphStage::Apply;
                Ok(SequencePersistentAdvance::Progress("sequence-node-graph-selection-complete", "{\"en\":\"Completed selected graph removal\",\"de\":\"Ausgewählte Graphentfernung wurde abgeschlossen\"}".as_bytes()))
            }
            SequenceNodeGraphStage::Apply => {
                self.stage = SequenceNodeGraphStage::Complete;
                self.advance(command, scene)
            }
            SequenceNodeGraphStage::Complete => {
                let base = self.base.as_ref().ok_or_else(|| sequence_fault("sequence.content.unavailable", "sequence-node-graph-base"))?;
                let target = self.target.as_ref().ok_or_else(|| sequence_fault("sequence.content.unavailable", "sequence-node-graph-target"))?;
                let leaves = sequence_scene_leaves(base, target);
                if leaves.len() > SEQUENCE_PERSISTENT_MAXIMUM_UNITS {
                    return Err(sequence_fault("sequence.editor.capacity", "sequence-node-graph-output-items"));
                }
                Ok(SequencePersistentAdvance::Complete(leaves))
            }
        }
    }

    fn release_one(&mut self, maximum_bytes: usize) -> SequencePersistentRelease {
        if !self.retirement.terminal_is_empty() {
            return match self.retirement.close_step(1, maximum_bytes) {
                ValueRetirementStep::Pending { released_bytes, .. } => SequencePersistentRelease::Progress(released_bytes),
                ValueRetirementStep::Complete if self.retirement.terminal_is_empty() => SequencePersistentRelease::Progress(0),
                ValueRetirementStep::Complete | ValueRetirementStep::Blocked => SequencePersistentRelease::Blocked,
            };
        }
        if self.operations.pop().is_some()
            || self.gesture.take().is_some()
            || self.delete_frontier.pop_front().is_some()
            || self.delete_current.take().is_some()
            || self.delete_discovered.pop().is_some()
        {
            return SequencePersistentRelease::Progress(0);
        }
        let step = self.discarded_steps.pop_front().or_else(|| self.base.as_mut().and_then(|scene| scene.steps.pop())).or_else(|| self.target.as_mut().and_then(|scene| scene.steps.pop()));
        if let Some(SequenceStep { mut params, .. }) = step {
            self.retirement.push_dictionary(std::mem::take(&mut params.0));
            return SequencePersistentRelease::Progress(0);
        }
        if self.base.as_mut().is_some_and(|scene| scene.edges.pop().is_some()) || self.target.as_mut().is_some_and(|scene| scene.edges.pop().is_some()) {
            return SequencePersistentRelease::Progress(0);
        }
        if self.base.take().is_some() || self.target.take().is_some() {
            return SequencePersistentRelease::Progress(0);
        }
        SequencePersistentRelease::Complete
    }
    fn empty(&self) -> bool {
        self.operations.is_empty()
            && self.gesture.is_none()
            && self.delete_frontier.is_empty()
            && self.delete_current.is_none()
            && self.delete_discovered.is_empty()
            && self.discarded_steps.is_empty()
            && self.retirement.terminal_is_empty()
            && self.base.is_none()
            && self.target.is_none()
    }
}

#[derive(Clone, Copy, Default)]
enum SequenceRunOrderStage {
    #[default]
    Steps,
    Edges,
    Heads,
    Choose,
    Walk,
    Remainder,
    Complete,
}

#[derive(Default)]
struct SequenceRunOrder {
    owner: Option<String>,
    name: Option<String>,
    stage: SequenceRunOrderStage,
    cursor: usize,
    scoped: Vec<usize>,
    incoming: Vec<(String, String)>,
    outgoing: Vec<(String, String)>,
    heads: Vec<usize>,
    ordered: Vec<usize>,
    current: Option<String>,
}

impl SequenceRunOrder {
    fn new(slot: Option<(&str, &str)>) -> Self {
        Self { owner: slot.map(|value| value.0.into()), name: slot.map(|value| value.1.into()), ..Self::default() }
    }

    fn matches(&self, step: &SequenceStep) -> bool {
        match ((self.owner.as_deref(), self.name.as_deref()), step.slot.as_ref()) {
            ((None, None), None) => true,
            ((Some(owner), Some(name)), Some(slot)) => slot.owner == owner && slot.name == name,
            _ => false,
        }
    }

    fn advance(&mut self, scene: &SequenceWorkingScene) -> &'static str {
        match self.stage {
            SequenceRunOrderStage::Steps if self.cursor < scene.steps.len() => {
                if self.matches(&scene.steps[self.cursor]) {
                    self.scoped.push(self.cursor);
                }
                self.cursor += 1;
                "sequence-run-order-step"
            }
            SequenceRunOrderStage::Steps => {
                self.stage = SequenceRunOrderStage::Edges;
                self.cursor = 0;
                "sequence-run-order-edges"
            }
            SequenceRunOrderStage::Edges if self.cursor < scene.edges.len() => {
                let edge = &scene.edges[self.cursor];
                if let Some(entry) = self.incoming.iter_mut().find(|entry| entry.0 == edge.to) {
                    entry.1 = edge.from.clone();
                } else {
                    self.incoming.push((edge.to.clone(), edge.from.clone()));
                }
                if let Some(entry) = self.outgoing.iter_mut().find(|entry| entry.0 == edge.from) {
                    entry.1 = edge.to.clone();
                } else {
                    self.outgoing.push((edge.from.clone(), edge.to.clone()));
                }
                self.cursor += 1;
                "sequence-run-order-edge"
            }
            SequenceRunOrderStage::Edges => {
                self.stage = SequenceRunOrderStage::Heads;
                self.cursor = 0;
                "sequence-run-order-heads"
            }
            SequenceRunOrderStage::Heads if self.cursor < self.scoped.len() => {
                let index = self.scoped[self.cursor];
                if !self.incoming.iter().any(|entry| entry.0 == scene.steps[index].id) {
                    self.heads.push(index);
                }
                self.cursor += 1;
                "sequence-run-order-head"
            }
            SequenceRunOrderStage::Heads => {
                self.stage = SequenceRunOrderStage::Choose;
                "sequence-run-order-choose"
            }
            SequenceRunOrderStage::Choose => {
                self.current = if self.heads.len() == 1 {
                    Some(scene.steps[self.heads[0]].id.clone())
                } else if self.scoped.len() == 1 {
                    Some(scene.steps[self.scoped[0]].id.clone())
                } else {
                    None
                };
                self.stage = if self.current.is_some() { SequenceRunOrderStage::Walk } else { SequenceRunOrderStage::Remainder };
                "sequence-run-order-start"
            }
            SequenceRunOrderStage::Walk => {
                let Some(id) = self.current.take() else {
                    self.stage = SequenceRunOrderStage::Remainder;
                    return "sequence-run-order-remainder";
                };
                if let Some(index) = self.scoped.iter().copied().find(|index| scene.steps[*index].id == id && !self.ordered.contains(index)) {
                    self.ordered.push(index);
                }
                self.current = self.outgoing.iter().find(|entry| entry.0 == id).map(|entry| entry.1.clone());
                if self.current.as_ref().is_some_and(|next| self.ordered.iter().any(|index| scene.steps[*index].id == *next)) {
                    self.current = None;
                }
                "sequence-run-order-walk"
            }
            SequenceRunOrderStage::Remainder => {
                let next = self.scoped.iter().copied().filter(|index| !self.ordered.contains(index)).min_by(|left, right| scene.steps[*left].id.cmp(&scene.steps[*right].id));
                if let Some(index) = next {
                    self.ordered.push(index);
                } else {
                    self.stage = SequenceRunOrderStage::Complete;
                }
                "sequence-run-order-remainder"
            }
            SequenceRunOrderStage::Complete => "sequence-run-order-complete",
        }
    }

    fn complete(&self) -> bool {
        matches!(self.stage, SequenceRunOrderStage::Complete)
    }
    fn release_one(&mut self) -> bool {
        self.scoped.pop().is_some()
            || self.incoming.pop().is_some()
            || self.outgoing.pop().is_some()
            || self.heads.pop().is_some()
            || self.ordered.pop().is_some()
            || self.current.take().is_some()
            || self.owner.take().is_some()
            || self.name.take().is_some()
    }
}

struct SequenceRunFrame {
    order: SequenceRunOrder,
    cursor: usize,
    repeat_remaining: usize,
    repeat_total: usize,
    while_key: Option<String>,
    while_iterations: usize,
}

/// 🧊️ Rebinding a live `Dictionary` root by plain assignment drops the PREVIOUS one without
/// retiring it, which the neural engine refuses in the guest with `final Dictionary ownership must
/// be explicitly retired or owned by a cold boundary`. `imperative_engine` has carried this exact
/// helper since its own sweep; `SequenceRunState::advance`'s `self.scope = result.scope` runs on
/// every executed step of a `run` verb, so it is the hottest of the four rebinds here.
fn replace_scope_cold(scope: &mut Dictionary, next: Dictionary) {
    neural_engine::ColdRetire::retire_cold(std::mem::replace(scope, next));
}

#[derive(Default)]
struct SequenceRunState {
    initialized: bool,
    registry: Option<SharedRegistry>,
    registry_retirement: Option<RegistryRetirement>,
    retirement: ValueRetirement,
    scope: Dictionary,
    effects: Vec<imperative_engine::EffectLogEntry>,
    frames: Vec<SequenceRunFrame>,
}

fn sequence_run_string(params: &Dictionary, key: &str) -> String {
    params.get(key).and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or_default().to_string()
}

fn sequence_run_number(params: &Dictionary, key: &str) -> usize {
    params.get(key).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()).unwrap_or(0.0).max(0.0) as usize
}

fn sequence_run_scope_bool(scope: &Dictionary, key: &str) -> bool {
    scope.get(key).and_then(|value| value.as_atom()).and_then(|atom| atom.as_bool()).unwrap_or(false)
}

impl SequenceRunState {
    fn advance(&mut self, scene: &SequenceWorkingScene) -> Result<SequencePersistentAdvance, Fault> {
        if scene.steps.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS || scene.edges.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
            return Err(sequence_fault("sequence.run.capacity", "sequence-run-scene-capacity"));
        }
        if !self.initialized {
            let (registry, retirement) = SharedRegistry::new(imperative_module_registry());
            self.registry = Some(registry);
            self.registry_retirement = Some(retirement);
            replace_scope_cold(&mut self.scope, Dictionary::new());
            self.frames.push(SequenceRunFrame { order: SequenceRunOrder::new(None), cursor: 0, repeat_remaining: 1, repeat_total: 1, while_key: None, while_iterations: 0 });
            self.initialized = true;
            return Ok(SequencePersistentAdvance::Progress("sequence-run-initialize", "{\"en\":\"Preparing execution\",\"de\":\"Ausführung wird vorbereitet\"}".as_bytes()));
        }
        let Some(frame) = self.frames.last_mut() else {
            let result = RunResult { scope: self.scope.clone(), effects: self.effects.clone() };
            let json = semio_framework_pack_json::to_json_string(&result);
            if json.len() > SEQUENCE_STORE_MAXIMUM_BYTES {
                return Err(sequence_fault("sequence.run.capacity", "sequence-run-result-capacity"));
            }
            return Ok(SequencePersistentAdvance::CompleteRun(json));
        };
        if !frame.order.complete() {
            let stage = frame.order.advance(scene);
            return Ok(SequencePersistentAdvance::Progress(stage, b"{\"en\":\"Ordering Sequence graph\",\"de\":\"Sequenzgraph wird geordnet\"}"));
        }
        if frame.cursor >= frame.order.ordered.len() {
            if let Some(key) = frame.while_key.as_ref() {
                if sequence_run_scope_bool(&self.scope, key) {
                    if frame.while_iterations == SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                        return Err(sequence_fault("sequence.run.capacity", "sequence-run-while-capacity"));
                    }
                    frame.cursor = 0;
                    frame.while_iterations += 1;
                    return Ok(SequencePersistentAdvance::Progress("sequence-run-while-cursor", b"{\"en\":\"Continuing bounded while body\",\"de\":\"Begrenzter Solange-Block wird fortgesetzt\"}"));
                }
            } else if frame.repeat_remaining > 1 {
                frame.repeat_remaining -= 1;
                frame.cursor = 0;
                let index = frame.repeat_total - frame.repeat_remaining;
                let next_scope = self.scope.clone().insert("index", NeuralValue::Atom(neural_engine::Atom::Integer(index as i64)));
                replace_scope_cold(&mut self.scope, next_scope);
                return Ok(SequencePersistentAdvance::Progress("sequence-run-repeat-cursor", b"{\"en\":\"Continuing bounded repeat body\",\"de\":\"Begrenzter Wiederholungsblock wird fortgesetzt\"}"));
            }
            self.frames.pop();
            return Ok(SequencePersistentAdvance::Progress("sequence-run-retire-frame", "{\"en\":\"Completed nested execution frame\",\"de\":\"Verschachtelter Ausführungsrahmen wurde abgeschlossen\"}".as_bytes()));
        }
        let index = frame.order.ordered[frame.cursor];
        frame.cursor += 1;
        let step = scene.steps.get(index).ok_or_else(|| sequence_fault("sequence.run.step-missing", "sequence-run-step-index"))?;
        match step.kind.as_str() {
            "control.if" => {
                let key = sequence_run_string(&step.params.0, "key");
                let slot = if sequence_run_scope_bool(&self.scope, &key) { "then" } else { "else" };
                let depth_fault = self.frames.len() >= 65;
                let required_effects = if depth_fault { 2 } else { 1 };
                if self.effects.len().saturating_add(required_effects) > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                    return Err(sequence_fault("sequence.run.capacity", "sequence-run-effect-capacity"));
                }
                let input = self.scope.merge(&step.params.0);
                self.effects.push(imperative_engine::EffectLogEntry {
                    step_id: step.id.clone(),
                    kind: step.kind.clone(),
                    input,
                    output: Some(Dictionary::new().insert("branch", NeuralValue::Atom(neural_engine::Atom::String(slot.into())))),
                    error: None,
                });
                if depth_fault {
                    self.effects.push(imperative_engine::EffectLogEntry { step_id: String::new(), kind: "control.depth".into(), input: Dictionary::new(), output: None, error: Some("nesting depth exceeded 64".into()) });
                } else {
                    self.frames.push(SequenceRunFrame { order: SequenceRunOrder::new(Some((&step.id, slot))), cursor: 0, repeat_remaining: 1, repeat_total: 1, while_key: None, while_iterations: 0 });
                }
            }
            "control.repeat" => {
                let count = sequence_run_number(&step.params.0, "count");
                if count > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                    return Err(sequence_fault("sequence.run.capacity", "sequence-run-repeat-capacity"));
                }
                if self.frames.len() >= 65 {
                    if self.effects.len() == SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                        return Err(sequence_fault("sequence.run.capacity", "sequence-run-effect-capacity"));
                    }
                    self.effects.push(imperative_engine::EffectLogEntry { step_id: String::new(), kind: "control.depth".into(), input: Dictionary::new(), output: None, error: Some("nesting depth exceeded 64".into()) });
                } else if count != 0 {
                    let next_scope = self.scope.clone().insert("index", NeuralValue::Atom(neural_engine::Atom::Integer(0)));
                    replace_scope_cold(&mut self.scope, next_scope);
                    self.frames.push(SequenceRunFrame { order: SequenceRunOrder::new(Some((&step.id, "body"))), cursor: 0, repeat_remaining: count, repeat_total: count, while_key: None, while_iterations: 0 });
                }
            }
            "control.while" => {
                let key = sequence_run_string(&step.params.0, "key");
                if sequence_run_scope_bool(&self.scope, &key) {
                    if self.frames.len() >= 65 {
                        if self.effects.len() == SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                            return Err(sequence_fault("sequence.run.capacity", "sequence-run-effect-capacity"));
                        }
                        self.effects.push(imperative_engine::EffectLogEntry { step_id: String::new(), kind: "control.depth".into(), input: Dictionary::new(), output: None, error: Some("nesting depth exceeded 64".into()) });
                    } else {
                        self.frames.push(SequenceRunFrame { order: SequenceRunOrder::new(Some((&step.id, "body"))), cursor: 0, repeat_remaining: 1, repeat_total: 1, while_key: Some(key), while_iterations: 1 });
                    }
                }
            }
            _ => {
                let registry = self.registry.as_ref().ok_or_else(|| sequence_fault("sequence.content.unavailable", "sequence-run-registry"))?;
                let result = Executor::new(registry).run(&Path { steps: vec![Step { id: step.id.clone(), kind: step.kind.clone(), params: step.params.0.clone(), bodies: BTreeMap::new() }] }, &self.scope);
                if self.effects.len().saturating_add(result.effects.len()) > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                    self.retirement.push_dictionary(result.scope);
                    for effect in result.effects {
                        self.retirement.push_dictionary(effect.input);
                        if let Some(output) = effect.output { self.retirement.push_dictionary(output); }
                    }
                    return Err(sequence_fault("sequence.run.capacity", "sequence-run-effect-capacity"));
                }
                let halt_frame = result.effects.iter().any(|effect| effect.error.is_some());
                replace_scope_cold(&mut self.scope, result.scope);
                self.effects.extend(result.effects);
                if halt_frame {
                    if let Some(frame) = self.frames.last_mut() {
                        frame.cursor = frame.order.ordered.len();
                    }
                }
            }
        }
        Ok(SequencePersistentAdvance::Progress("sequence-run-step", "{\"en\":\"Executed Sequence step\",\"de\":\"Sequenzschritt wurde ausgeführt\"}".as_bytes()))
    }

    fn release_one(&mut self, maximum_bytes: usize) -> SequencePersistentRelease {
        if !self.retirement.terminal_is_empty() {
            return match self.retirement.close_step(1, maximum_bytes) {
                ValueRetirementStep::Pending { released_bytes, .. } => SequencePersistentRelease::Progress(released_bytes),
                ValueRetirementStep::Complete if self.retirement.terminal_is_empty() => SequencePersistentRelease::Progress(0),
                ValueRetirementStep::Complete | ValueRetirementStep::Blocked => SequencePersistentRelease::Blocked,
            };
        }
        if let Some(effect) = self.effects.pop() {
            self.retirement.push_dictionary(effect.input);
            if let Some(output) = effect.output { self.retirement.push_dictionary(output); }
            return SequencePersistentRelease::Progress(0);
        }
        if let Some(frame) = self.frames.last_mut() {
            if frame.order.release_one() {
                return SequencePersistentRelease::Progress(0);
            }
        }
        if self.frames.pop().is_some() { return SequencePersistentRelease::Progress(0); }
        if let Some(registry) = self.registry.take() {
            drop(registry);
            return SequencePersistentRelease::Progress(0);
        }
        if let Some(retirement) = self.registry_retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes) {
                Ok(ValueRetirementStep::Pending { released_bytes, .. }) => SequencePersistentRelease::Progress(released_bytes),
                Ok(ValueRetirementStep::Complete) if retirement.terminal_is_empty() => {
                    self.registry_retirement = None;
                    SequencePersistentRelease::Progress(0)
                }
                Ok(ValueRetirementStep::Complete) | Ok(ValueRetirementStep::Blocked) | Err(_) => SequencePersistentRelease::Blocked,
            };
        }
        if self.initialized {
            self.retirement.push_dictionary(std::mem::take(&mut self.scope));
            self.initialized = false;
            return SequencePersistentRelease::Progress(0);
        }
        SequencePersistentRelease::Complete
    }
    fn empty(&self) -> bool {
        !self.initialized && self.registry.is_none() && self.registry_retirement.is_none() && self.retirement.terminal_is_empty() && self.effects.is_empty() && self.frames.is_empty()
    }
}

enum SequencePersistentRelease {
    Progress(usize),
    Blocked,
    Complete,
}

enum SequencePersistentWorkspace {
    Reorganize(SequenceReorganizeState),
    NodeGraph(SequenceNodeGraphState),
    Run(SequenceRunState),
}
impl SequencePersistentWorkspace {
    fn new(tool_id: &str) -> Self {
        match tool_id {
            "reorganize" => Self::Reorganize(SequenceReorganizeState::default()),
            "nodeGraphEdit" => Self::NodeGraph(SequenceNodeGraphState::default()),
            _ => Self::Run(SequenceRunState::default()),
        }
    }
    fn advance(&mut self, command: &SequenceCommand, scene: &SequenceWorkingScene, config: &SequenceMainWindowConfig) -> Result<SequencePersistentAdvance, Fault> {
        match self {
            Self::Reorganize(state) => state.advance(scene, config),
            Self::NodeGraph(state) => state.advance(command, scene),
            Self::Run(state) => state.advance(scene),
        }
    }
    fn release_one(&mut self, maximum_bytes: usize) -> SequencePersistentRelease {
        match self {
            Self::Reorganize(state) => if state.release_one() { SequencePersistentRelease::Progress(0) } else { SequencePersistentRelease::Complete },
            Self::NodeGraph(state) => state.release_one(maximum_bytes),
            Self::Run(state) => state.release_one(maximum_bytes),
        }
    }
    fn empty(&self) -> bool {
        match self {
            Self::Reorganize(state) => state.empty(),
            Self::NodeGraph(state) => state.empty(),
            Self::Run(state) => state.empty(),
        }
    }
}

struct SequencePersistentWork {
    tool_id: &'static str,
    workspace_identity: u64,
    progress: usize,
    replay_target: Option<usize>,
    scene_owner: SequenceRetainedSceneOwner,
    workspace: SequencePersistentWorkspace,
    completed: bool,
    closing: bool,
}
impl SequencePersistentWork {
    fn new(tool_id: &'static str, operation: &semio_framework_plugin::AppOperationContext) -> Self {
        let scope = format!("{}:{}:{}:{}", operation.app_instance_id, operation.parent_document_id, operation.operation_id, operation.generation);
        let identity = scope.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325_u64, |state, byte| (state ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3));
        Self {
            tool_id,
            workspace_identity: identity,
            progress: 0,
            replay_target: None,
            scene_owner: SequenceRetainedSceneOwner::default(),
            workspace: SequencePersistentWorkspace::new(tool_id),
            completed: false,
            closing: false,
        }
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<SequencePlayApp>> for SequencePersistentWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }
    fn workspace_identity(&self) -> u64 {
        self.workspace_identity
    }
    fn extent(
        &self,
        _command: &SequenceCommand,
        snapshot: &SequenceSnapshot,
        _interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<SequencePlayApp>>>,
    ) -> Option<usize> {
        let scene = sequence_working_scene_from_children(snapshot, context?.children.as_ref()).ok()?;
        (scene.steps.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS && scene.edges.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS).then_some(SEQUENCE_PERSISTENT_MAXIMUM_UNITS)
    }
    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<SequencePlayApp>>,
    _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<SequencePlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context, operation } = *input;
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || self.progress >= SEQUENCE_PERSISTENT_MAXIMUM_UNITS || command.command_id() != self.tool_id {
            return Err(sequence_fault("sequence.editor.capacity", "sequence-persistent-progress-capacity"));
        }
        let context = context.ok_or_else(|| sequence_fault("sequence.window.unavailable", "sequence-window-context-required"))?;
        let config = main::config::from_snapshot(context.window_config.as_ref());
        self.scene_owner.capture(snapshot, context.children.as_ref())?;
        let scene = self.scene_owner.scene()?;
        match self.workspace.advance(command, scene, &config)? {
            SequencePersistentAdvance::Progress(stage, preview) => {
                self.progress += 1;
                if let Some(target) = self.replay_target {
                    if self.progress == target {
                        self.replay_target = None;
                    }
                    Ok(ArtifactCommandWorkStep::Replay { stage, preview })
                } else {
                    Ok(ArtifactCommandWorkStep::Progress { stage, preview })
                }
            }
            SequencePersistentAdvance::Complete(leaves) => {
                if self.replay_target.is_some() {
                    return Err(sequence_fault("sequence.resume.invalid", "sequence-persistent-replay-overrun"));
                }
                let emit = match &mut self.workspace {
                    SequencePersistentWorkspace::NodeGraph(state) => state.publish(snapshot, leaves, &operation.authoring_seed),
                    _ => sequence_child_leaves_emit(snapshot, leaves),
                };
                let exact_child = emit.child_emits.first().is_none_or(|child| child.slot == "content" && child.child_id == snapshot.content.child_id);
                let exact_lane = emit.config_mutations.is_empty() && emit.draft_mutations.is_empty() && emit.artifact_mutations.is_empty() && emit.child_emits.len() <= 1 && exact_child;
                if !exact_lane {
                    return Err(sequence_fault("sequence.publication.lane", "sequence-persistent-publication-lane"));
                }
                sequence_bounded_child_emit_bytes(&emit.child_emits, SEQUENCE_STORE_MAXIMUM_BYTES).map_err(|_| sequence_fault("sequence.editor.capacity", "sequence-persistent-output-bytes"))?;
                self.completed = true;
                Ok(ArtifactCommandWorkStep::Complete(emit))
            }
            SequencePersistentAdvance::CompleteRun(json) => {
                if self.replay_target.is_some() || self.tool_id != "run" {
                    return Err(sequence_fault("sequence.resume.invalid", "sequence-run-replay-overrun"));
                }
                let view = context.view_state.as_ref().ok_or_else(|| sequence_fault("sequence.window.unavailable", "sequence-script-window-view-required"))?;
                let transient = SequenceScriptWindowTransient { last_run_json: json };
                self.completed = true;
                Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                    emit: Emit::default(),
                    ephemeral: semio_framework_plugin::EphemeralEmit {
                        presence: Vec::new(),
                        transient: Vec::new(),
                        window_transient: vec![script::transient::addressed(view, transient)?],
                    },
                })
            }
        }
    }
    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 24 {
            return Err(sequence_fault("sequence.editor.capacity", "sequence-persistent-checkpoint-capacity"));
        }
        target[..24].fill(0);
        target[..4].copy_from_slice(b"SRP1");
        target[8..16].copy_from_slice(&(self.progress as u64).to_le_bytes());
        target[16..24].copy_from_slice(&self.workspace_identity.to_le_bytes());
        Ok(24)
    }
    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 24 || &checkpoint[..4] != b"SRP1" || checkpoint[4..8] != [0, 0, 0, 0] {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-persistent-checkpoint-invalid"));
        }
        let progress = usize::try_from(u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-persistent-checkpoint-cursor"))?)).map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-persistent-checkpoint-cursor"))?;
        let identity = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-persistent-checkpoint-identity"))?);
        if identity != self.workspace_identity || progress > SEQUENCE_PERSISTENT_MAXIMUM_UNITS {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-persistent-checkpoint-owner"));
        }
        if !self.workspace.empty() || !self.scene_owner.empty() {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-persistent-restore-live-workspace"));
        }
        self.progress = 0;
        self.replay_target = (progress != 0).then_some(progress);
        self.workspace = SequencePersistentWorkspace::new(self.tool_id);
        self.completed = false;
        Ok(())
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.replay_target = None;
        match self.workspace.release_one(maximum_bytes) {
            SequencePersistentRelease::Progress(released_bytes) => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes },
            SequencePersistentRelease::Blocked => semio_framework_job::InteractiveJobCloseStep::Blocked,
            SequencePersistentRelease::Complete => match self.scene_owner.release_one(maximum_bytes) {
                SequencePersistentRelease::Progress(released_bytes) => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes },
                SequencePersistentRelease::Blocked => semio_framework_job::InteractiveJobCloseStep::Blocked,
                SequencePersistentRelease::Complete => semio_framework_job::InteractiveJobCloseStep::Complete,
            },
        }
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.replay_target.is_none() && self.workspace.empty() && self.scene_owner.empty()
    }
}

struct SequencePersistentJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}
impl SequencePersistentJobFactory {
    fn new(controller: &str) -> Self {
        Self { keys: SEQUENCE_PERSISTENT_TOOL_IDS.iter().map(|id| semio_framework::ToolFactoryKey::new(controller, *id)).collect() }
    }
}
impl semio_framework::ToolJobFactory for SequencePersistentJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        "sequence.play/persistent-command.v1"
    }
    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(SEQUENCE_RETAINED_RAW_BYTES, SEQUENCE_PERSISTENT_MAXIMUM_UNITS, 1, SEQUENCE_STORE_MAXIMUM_BYTES, 7_500, 1, 1)
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SEQUENCE_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((semio_framework::ToolJobFactoryError::new("Sequence persistent command wire or checkpoint exceeds cap"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}
impl semio_framework_plugin::ArtifactOwnedToolJobFactory for SequencePersistentJobFactory {
    type Owner = semio_framework_plugin::EditorApp<SequencePlayApp>;
    const TOOL_IDS: &'static [&'static str] = SEQUENCE_PERSISTENT_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SEQUENCE_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = SEQUENCE_PERSISTENT_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️PersistentRemainingRoutes

//#region 🧵️RetainedConfigRoutes
const SEQUENCE_RETAINED_PAYLOAD_SCHEMA: &str = "sequence.play/retained-config-command.v1";
const SEQUENCE_RETAINED_RAW_BYTES: usize = 4_096;
const SEQUENCE_RETAINED_MAXIMUM_UNITS: usize = 2;
const SEQUENCE_RETAINED_CONFIG_TOOL_IDS: &[&str] = &["setViewport", "setOrientation", "stop"];
const SEQUENCE_RETAINED_CONFIG_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setViewport", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setOrientation", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "stop", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
];

fn sequence_retained_config_command_admitted(command: &SequenceCommand) -> bool {
    match command {
        SequenceCommand::SetViewport(_) | SequenceCommand::Stop(_) => true,
        SequenceCommand::SetOrientation(payload) => payload.value.len() <= 32,
        _ => false,
    }
}

struct SequenceRetainedConfigWork {
    tool_id: &'static str,
    workspace_identity: u64,
    cursor: usize,
    replay_target: Option<usize>,
    completed: bool,
    closing: bool,
}

impl SequenceRetainedConfigWork {
    fn new(tool_id: &'static str, operation: &semio_framework_plugin::AppOperationContext) -> Self {
        let scope = format!("{}:{}:{}:{}", operation.app_instance_id, operation.parent_document_id, operation.operation_id, operation.generation);
        let workspace_identity = scope.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325_u64, |state, byte| (state ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3));
        Self { tool_id, workspace_identity, cursor: 0, replay_target: None, completed: false, closing: false }
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<SequencePlayApp>> for SequenceRetainedConfigWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn workspace_identity(&self) -> u64 {
        self.workspace_identity
    }

    fn extent(
        &self,
        _command: &SequenceCommand,
        _snapshot: &SequenceSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<SequencePlayApp>>>,
    ) -> Option<usize> {
        Some(SEQUENCE_RETAINED_MAXIMUM_UNITS)
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<SequencePlayApp>>,
    _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<SequencePlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot: _snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context, operation: _operation } = *input;
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || self.cursor >= SEQUENCE_RETAINED_MAXIMUM_UNITS || !sequence_retained_config_command_admitted(command) {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("sequence.retained.config-command"), "Sequence retained config command exceeded its exact route or payload envelope"));
        }
        self.cursor += 1;
        if let Some(target) = self.replay_target {
            if self.cursor <= target {
                if self.cursor == target {
                    self.replay_target = None;
                }
                return Ok(ArtifactCommandWorkStep::Replay { stage: "sequence-config-replay", preview: b"{\"en\":\"Restoring Sequence setting\",\"de\":\"Sequenzeinstellung wird wiederhergestellt\"}" });
            }
        }
        if self.cursor == 1 {
            return Ok(ArtifactCommandWorkStep::Progress { stage: "sequence-config-prepare", preview: b"{\"en\":\"Preparing Sequence setting\",\"de\":\"Sequenzeinstellung wird vorbereitet\"}" });
        }
        let context = context.ok_or_else(|| sequence_fault("sequence.window.unavailable", "sequence-window-context-required"))?;
        let view = context.view_state.as_ref().ok_or_else(|| sequence_fault("sequence.window.unavailable", "sequence-window-view-required"))?;
        let mut config = main::config::from_snapshot(context.window_config.as_ref());
        self.completed = true;
        match command {
            SequenceCommand::SetViewport(payload) => {
                config.camera = payload.camera.clone();
                Ok(ArtifactCommandWorkStep::Complete(Emit { window_config_mutations: vec![main::config::addressed(view, config)?], ..Default::default() }))
            }
            SequenceCommand::SetOrientation(payload) => {
                config.orientation = payload.value.clone();
                Ok(ArtifactCommandWorkStep::Complete(Emit { window_config_mutations: vec![main::config::addressed(view, config)?], ..Default::default() }))
            }
            SequenceCommand::Stop(_) => Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit: Emit::default(),
                ephemeral: semio_framework_plugin::EphemeralEmit {
                    presence: Vec::new(),
                    transient: Vec::new(),
                    window_transient: vec![script::transient::addressed(view, SequenceScriptWindowTransient::default())?],
                },
            }),
            _ => Err(sequence_fault("app.command.tool-mismatch", "sequence-window-route-rejected")),
        }
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 24 {
            return Err(sequence_fault("sequence.editor.capacity", "sequence-retained-checkpoint-capacity"));
        }
        target[..24].fill(0);
        target[..4].copy_from_slice(b"SRC1");
        target[4] = u8::from(self.completed);
        target[8..16].copy_from_slice(&(self.cursor as u64).to_le_bytes());
        target[16..24].copy_from_slice(&self.workspace_identity.to_le_bytes());
        Ok(24)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 24 || &checkpoint[..4] != b"SRC1" || checkpoint[4] > 1 || checkpoint[5..8] != [0, 0, 0] {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-invalid"));
        }
        let cursor = usize::try_from(u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-cursor"))?)).map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-cursor"))?;
        let identity = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-identity"))?);
        if identity != self.workspace_identity || cursor > SEQUENCE_RETAINED_MAXIMUM_UNITS {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-owner-mismatch"));
        }
        self.cursor = 0;
        self.replay_target = (cursor != 0).then_some(cursor);
        self.completed = false;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.replay_target = None;
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.replay_target.is_none()
    }
}

struct SequenceRetainedConfigJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl SequenceRetainedConfigJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: SEQUENCE_RETAINED_CONFIG_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for SequenceRetainedConfigJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<SequencePlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        SEQUENCE_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(SEQUENCE_RETAINED_RAW_BYTES, SEQUENCE_RETAINED_MAXIMUM_UNITS, 1, 4_096, 2_000, 1, 1)
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SEQUENCE_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((semio_framework::ToolJobFactoryError::new("Sequence retained config command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for SequenceRetainedConfigJobFactory {
    type Owner = semio_framework_plugin::EditorApp<SequencePlayApp>;
    const TOOL_IDS: &'static [&'static str] = SEQUENCE_RETAINED_CONFIG_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SEQUENCE_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = SEQUENCE_RETAINED_CONFIG_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedConfigRoutes

//#region 🧵️RetainedExampleRoutes
/// 🎬️ `setActiveExample`'s own retained route. It is a FOURTH factory rather than a row on any of
/// the three above because those three route document, child and window-config mutations through
/// their work impls, while this verb emits a host-applied effect and nothing else — the
/// `🔱️trinity/🔌️jack` one-verb-factory shape. Its lane is therefore `HostOnly`: it publishes on no
/// store at all.
const SEQUENCE_RETAINED_EXAMPLE_PAYLOAD_SCHEMA: &str = "sequence.play/retained-example-command.v1";
const SEQUENCE_RETAINED_EXAMPLE_RAW_BYTES: usize = 4_096;
const SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_UNITS: usize = 2;
const SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_ID_BYTES: usize = 256;
const SEQUENCE_RETAINED_EXAMPLE_TOOL_IDS: &[&str] = &["setActiveExample"];
const SEQUENCE_RETAINED_EXAMPLE_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] =
    &[semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] }];

fn sequence_retained_example_command_admitted(command: &SequenceCommand) -> bool {
    match command {
        SequenceCommand::SetActiveExample(payload) => payload.example_id.len() <= SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_ID_BYTES,
        _ => false,
    }
}

struct SequenceRetainedExampleWork {
    tool_id: &'static str,
    workspace_identity: u64,
    cursor: usize,
    replay_target: Option<usize>,
    completed: bool,
    closing: bool,
}

impl SequenceRetainedExampleWork {
    fn new(tool_id: &'static str, operation: &semio_framework_plugin::AppOperationContext) -> Self {
        let scope = format!("{}:{}:{}:{}", operation.app_instance_id, operation.parent_document_id, operation.operation_id, operation.generation);
        let workspace_identity = scope.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325_u64, |state, byte| (state ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3));
        Self { tool_id, workspace_identity, cursor: 0, replay_target: None, completed: false, closing: false }
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<SequencePlayApp>> for SequenceRetainedExampleWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn workspace_identity(&self) -> u64 {
        self.workspace_identity
    }

    /// 📏️ Answered without touching the snapshot or the working scene: loading an example is what
    /// makes a scene exist, so measuring this verb against one would refuse it at exactly the boot
    /// moment the playground navbar dispatches it.
    fn extent(
        &self,
        command: &SequenceCommand,
        _snapshot: &SequenceSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<SequencePlayApp>>>,
    ) -> Option<usize> {
        sequence_retained_example_command_admitted(command).then_some(SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_UNITS)
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<SequencePlayApp>>,
    _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<SequencePlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, .. } = *input;
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || self.cursor >= SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_UNITS || !sequence_retained_example_command_admitted(command) {
            return Err(Fault::new(
                semio_framework_plugin::FaultOrigin::App,
                semio_framework_plugin::FaultCode::new("sequence.retained.example-command"),
                "Sequence retained example command exceeded its exact route or payload envelope",
            ));
        }
        self.cursor += 1;
        if let Some(target) = self.replay_target {
            if self.cursor <= target {
                if self.cursor == target {
                    self.replay_target = None;
                }
                return Ok(ArtifactCommandWorkStep::Replay { stage: "sequence-example-replay", preview: b"{\"en\":\"Restoring Sequence example\",\"de\":\"Sequenzbeispiel wird wiederhergestellt\"}" });
            }
        }
        if self.cursor == 1 {
            return Ok(ArtifactCommandWorkStep::Progress { stage: "sequence-example-prepare", preview: b"{\"en\":\"Preparing Sequence example\",\"de\":\"Sequenzbeispiel wird vorbereitet\"}" });
        }
        self.completed = true;
        match command {
            SequenceCommand::SetActiveExample(payload) => set_active_example::emit(&payload.example_id).map(ArtifactCommandWorkStep::Complete),
            _ => Err(sequence_fault("app.command.tool-mismatch", "sequence-example-route-rejected")),
        }
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 24 {
            return Err(sequence_fault("sequence.editor.capacity", "sequence-retained-checkpoint-capacity"));
        }
        target[..24].fill(0);
        target[..4].copy_from_slice(b"SRE1");
        target[4] = u8::from(self.completed);
        target[8..16].copy_from_slice(&(self.cursor as u64).to_le_bytes());
        target[16..24].copy_from_slice(&self.workspace_identity.to_le_bytes());
        Ok(24)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 24 || &checkpoint[..4] != b"SRE1" || checkpoint[4] > 1 || checkpoint[5..8] != [0, 0, 0] {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-invalid"));
        }
        let cursor = usize::try_from(u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-cursor"))?)).map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-cursor"))?;
        let identity = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-identity"))?);
        if identity != self.workspace_identity || cursor > SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_UNITS {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-owner-mismatch"));
        }
        self.cursor = 0;
        self.replay_target = (cursor != 0).then_some(cursor);
        self.completed = false;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.replay_target = None;
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.replay_target.is_none()
    }
}

struct SequenceRetainedExampleJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl SequenceRetainedExampleJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: SEQUENCE_RETAINED_EXAMPLE_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for SequenceRetainedExampleJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<SequencePlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        SEQUENCE_RETAINED_EXAMPLE_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(SEQUENCE_RETAINED_EXAMPLE_RAW_BYTES, SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_UNITS, 1, 65_536, 7_500, 1, 1)
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SEQUENCE_RETAINED_EXAMPLE_RAW_BYTES || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((semio_framework::ToolJobFactoryError::new("Sequence retained example command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for SequenceRetainedExampleJobFactory {
    type Owner = semio_framework_plugin::EditorApp<SequencePlayApp>;
    const TOOL_IDS: &'static [&'static str] = SEQUENCE_RETAINED_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SEQUENCE_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = SEQUENCE_RETAINED_EXAMPLE_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedExampleRoutes

//#region 🎞️ReservedImport
/// 🎞️ The reserved tool id the framework registers for every app's media import.
const SEQUENCE_IMPORT_TOOL_ID: &str = "import-media";

/// 🎞️ The only media port Sequence declares as an importer (`sequence_io`).
const SEQUENCE_IMPORT_PORT: &str = "steps:in";

/// 🎞️ The ONE concrete resumable importer this app owns. `VcsArtifactApp::dispatch_import_media`
/// builds its emit EXCLUSIVELY from the resumable job — `A::import_media` is never reached on a
/// mounted app — so while this returned the trait default `None` every `steps:in` delivery died
/// `interactive-job.missing-reserved-builder` although the synchronous importer below was correct
/// (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, media §8, S10-E). Original admitted media
/// remains owned through native parsing/projection, typed binding and the same composed child
/// publication. Every source candidate drains through its existing retirement authority.
struct SequenceImportJob {
    port: String,
    media: Option<MediaPayload>,
    parser: Option<semio_framework_pack_json::JsonParseCursor>,
    projection: Option<semio_framework_pack_json::JsonValueProjection>,
    decode_receipt: Option<semio_framework_value::native_decoding::NativeDecodeContinuation>,
    input: Option<neural_engine::retirement::RetainedDictionaryInput>,
    params: Option<StepParams>,
    retirement: Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    snapshot: Option<std::sync::Arc<SequenceSnapshot>>,
    children: Option<semio_framework_plugin::app::ChildContentView>,
    emit: Option<Emit<SequenceMutation, NoConfigMutation>>,
    decoded: bool,
    completed: bool,
    closing: bool,
    completion: Option<semio_framework_plugin::ArtifactToolCompletion<semio_framework_plugin::EditorApp<SequencePlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<semio_framework_plugin::EditorApp<SequencePlayApp>>>,
}

/// 🧾️ Admits one bounded job payload page, answering the empty page when the context refuses it.
fn sequence_job_payload(cx: &mut semio_framework_job::StepContext<'_>, stream: semio_framework_job::JobPayloadStream, bytes: &[u8]) -> semio_framework_job::RetainedJobPayload {
    match cx.payload_from_bytes(stream, bytes) {
        Ok(payload) => payload,
        Err(rejected) => {
            drop(rejected.into_source());
            semio_framework_job::RetainedJobPayload::empty(stream)
        }
    }
}

/// 🧯️ One bounded job fault carrying its own detail page.
fn sequence_job_fault(cx: &mut semio_framework_job::StepContext<'_>, detail: &str) -> semio_framework_job::StepOutcome {
    let bytes = detail.as_bytes();
    let bounded = &bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)];
    semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: sequence_job_payload(cx, semio_framework_job::JobPayloadStream::Fault, bounded) })
}

/// 📦️ Takes the original parameter value into its canonical dictionary shape.
fn sequence_import_parameter_value(value: semio_framework_value::DslValue) -> semio_framework_value::DslValue {
    if matches!(value, semio_framework_value::DslValue::Object(_)) { value } else { semio_framework_value::DslValue::Object(vec![("value".into(), value)]) }
}

impl SequenceImportJob {
    fn new(request: semio_framework_plugin::ArtifactReservedToolJobRequest<semio_framework_plugin::EditorApp<SequencePlayApp>>, port: String, media: Media) -> Self {
        Self {
            port,
            media: Some(media.payload), parser: None, projection: None, decode_receipt: None, input: None, params: None, retirement: None,
            snapshot: Some(request.snapshot),
            children: Some(request.children),
            emit: None,
            decoded: false,
            completed: false,
            closing: false,
            completion: Some(request.completion),
            pending_completion_rejection: None,
        }
    }

    fn decode(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Option<semio_framework_job::StepOutcome> {
        if self.port != SEQUENCE_IMPORT_PORT {
            return Some(sequence_job_fault(cx, "sequence import only implements steps:in"));
        }
        if self.params.is_none() {
            if self.input.is_none() && self.parser.is_none() && self.projection.is_none() {
                match self.media.as_mut() {
                    Some(MediaPayload::Intrinsic { value, .. }) => {
                        let value = std::mem::replace(value, semio_framework_value::DslValue::Null);
                        self.input = Some(neural_engine::retirement::RetainedDictionaryInput::new(sequence_import_parameter_value(value)));
                    },
                    Some(MediaPayload::Structured { .. }) => self.parser = Some(semio_framework_pack_json::JsonParseCursor::new(semio_framework_pack_json::JsonMemberPolicy::Reject)),
                    Some(MediaPayload::Binary { .. }) | None => return Some(sequence_job_fault(cx, "sequence steps:in requires an intrinsic or structured parameter value")),
                }
                cx.consume_fuel(1);
            }
            while !cx.should_yield() {
                if cx.is_cancelled() { return Some(semio_framework_job::StepOutcome::Cancelled); }
                if let Some(input) = self.input.as_mut() {
                    cx.set_stage(input.progress().2);
                    match input.step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
                        Ok(Some(dictionary)) => { self.params = Some(StepParams(dictionary)); cx.consume_fuel(1); break; },
                        Ok(None) => {},
                        Err(error) if error.kind == semio_framework_value::ValueRefusalKind::Canceled => return Some(semio_framework_job::StepOutcome::Cancelled), Err(error) => return Some(sequence_job_fault(cx, &error.message)),
                    }
                } else {
                    let result = {
                        let mut accept = |_| !cx.is_cancelled();
                        let control = match self.decode_receipt.take() { Some(receipt) => semio_framework_value::NativeDecodeControl::resume(receipt, &mut accept), None => Ok(semio_framework_value::NativeDecodeControl::new(SEQUENCE_STORE_MAXIMUM_BYTES, &mut accept)) };
                        match control {
                            Ok(mut control) => {
                                let result = if let Some(projection) = self.projection.as_mut() { projection.step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, &mut control) }
                                else if let (Some(parser), Some(MediaPayload::Structured { json, .. })) = (self.parser.as_mut(), self.media.as_ref()) {
                                    match parser.step(json, 1, &mut control) { Ok(Some(value)) => { self.projection = Some(semio_framework_pack_json::JsonValueProjection::new(value)); Ok(None) }, Ok(None) => Ok(None), Err(semio_framework_pack_json::JsonError::Native(error)) => Err(error), Err(error) => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())) }
                                } else { Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "sequence import source disappeared")) };
                                match control.pause() { Ok(receipt) => { self.decode_receipt = Some(receipt); result }, Err(error) => Err(error) }
                            },
                            Err(error) => Err(error),
                        }
                    };
                    match result { Ok(Some(value)) => self.input = Some(neural_engine::retirement::RetainedDictionaryInput::new(sequence_import_parameter_value(value))), Ok(None) => {}, Err(error) if error.kind == semio_framework_value::ValueRefusalKind::Canceled => return Some(semio_framework_job::StepOutcome::Cancelled), Err(error) => return Some(sequence_job_fault(cx, &error.message)) }
                    cx.set_stage(if self.projection.is_some() { "sequence-import-project" } else { "sequence-import-parse" });
                }
                cx.consume_fuel(1);
            }
            return Some(semio_framework_job::StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: sequence_job_payload(cx, semio_framework_job::JobPayloadStream::CheckpointState, &[0]), applied_progress: 0 }));
        }
        let (Some(snapshot), Some(children)) = (self.snapshot.as_ref(), self.children.as_ref()) else {
            return Some(sequence_job_fault(cx, "sequence import lost its snapshot authority"));
        };
        let Ok(mut live) = sequence_host_snapshot_from_children(snapshot.as_ref(), children) else {
            return Some(sequence_job_fault(cx, "sequence import could not read its composed flow child"));
        };
        let id = format!("step-{}", max_serial_in_snapshot(&live).max(100) + 1);
        let x = live.steps.iter().map(|step| step.x).fold(0.0_f64, f64::max) + if live.steps.is_empty() { 0.0 } else { 280.0 };
        let base = ColdOwner::new(SequenceWorkingScene { steps: live.steps.clone(), edges: live.edges.clone() });
        live.steps.push(SequenceStep { id, kind: "computation.import".into(), params: self.params.take().expect("retained parameter binding completed"), x, y: 0.0, slot: None, collapsed: false });
        let next = ColdOwner::new(SequenceWorkingScene { steps: live.steps.clone(), edges: live.edges.clone() });
        self.emit = Some(sequence_child_leaves_emit(snapshot.as_ref(), sequence_scene_leaves(&base, &next)));
        self.children = None;
        self.decoded = true;
        None
    }
}

impl semio_framework_job::InteractiveJob for SequenceImportJob {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if cx.should_yield() { return semio_framework_job::StepOutcome::Yield; }
        if self.pending_completion_rejection.is_some() {
            return sequence_job_fault(cx, "sequence import completion remains rejected");
        }
        if !self.decoded {
            cx.set_stage("sequence-import-decode");
            if let Some(outcome) = self.decode(cx) {
                return outcome;
            }
            cx.consume_fuel(1);
            return semio_framework_job::StepOutcome::CheckpointReady(semio_framework_job::Checkpoint {
                state: sequence_job_payload(cx, semio_framework_job::JobPayloadStream::CheckpointState, &[1]),
                applied_progress: 1,
            });
        }
        cx.set_stage("sequence-import-publish");
        if let Some(emit)=self.emit.as_mut(){
            let bytes=emit.next_child_preparation_byte_demand().max(1);
            match emit.prepare_child_one(1,bytes){
                Ok(semio_framework_plugin::app::ChildEmitPreparationStep::Ready)=>{},
                Ok(semio_framework_plugin::app::ChildEmitPreparationStep::Pending)=>{cx.consume_fuel(1);return semio_framework_job::StepOutcome::Yield;},
                Ok(semio_framework_plugin::app::ChildEmitPreparationStep::Refused(fault))|Err(fault)=>return sequence_job_fault(cx,&fault.message),
            }
        }
        if !self.completed {
            let Some(emit) = self.emit.take() else {
                return sequence_job_fault(cx, "sequence import lost its decoded child publication");
            };
            let Some(completion) = self.completion.as_ref() else {
                return sequence_job_fault(cx, "sequence import lost its completion authority");
            };
            if !completion.has_mounted_consumer() {
                self.emit = Some(emit);
                return sequence_job_fault(cx, "sequence import completion consumer is absent");
            }
            if let Err(rejected) = completion.complete(Ok(emit), semio_framework_plugin::EphemeralEmit::default()) {
                let message = rejected.fault.message.clone();
                self.pending_completion_rejection = Some(rejected);
                return sequence_job_fault(cx, &message);
            }
            self.completed = true;
        }
        semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        match semio_framework_plugin::ArtifactReservedJob::close_step(self, maximum_items, maximum_bytes) {
            Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes }) => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
            Ok(semio_framework_plugin::PluginCloseStep::AwaitingInput { .. } | semio_framework_plugin::PluginCloseStep::Blocked { .. }) | Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
            Ok(semio_framework_plugin::PluginCloseStep::Complete) if semio_framework_plugin::ArtifactReservedJob::terminal_is_empty(self) => semio_framework_job::InteractiveJobCloseStep::Complete,
            Ok(semio_framework_plugin::PluginCloseStep::Complete) => semio_framework_job::InteractiveJobCloseStep::Blocked,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        semio_framework_plugin::ArtifactReservedJob::terminal_is_empty(self)
    }
}

impl semio_framework_plugin::ArtifactReservedJob for SequenceImportJob {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        self.closing = true;
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            if let Ok(emit) = rejected.emit.as_mut() {
                if let Some(step) = emit.close_child_one(maximum_items, maximum_bytes) {
                    return Ok(step);
                }
            }
            self.pending_completion_rejection = None;
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(emit) = self.emit.as_mut() {
            if let Some(step) = emit.close_child_one(maximum_items, maximum_bytes) {
                return Ok(step);
            }
            self.emit = None;
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.children.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(input) = self.input.as_mut() {
            input.cancel();
            let step = input.close_step(maximum_items, maximum_bytes);
            if input.terminal_is_empty() { self.input = None; }
            return Ok(match step { ValueRetirementStep::Pending { released_items, released_bytes } => semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes }, ValueRetirementStep::Complete => semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }, ValueRetirementStep::Blocked => semio_framework_plugin::PluginCloseStep::Blocked { reason: "sequence parameter input retirement is blocked" } });
        }
        if let Some(close) = self.retirement.as_mut() {
            let step = close.close_step(maximum_items, maximum_bytes).map_err(|error| sequence_fault("sequence.retained.close", error.message))?;
            if close.terminal_is_empty() { self.retirement = None; }
            return Ok(match step { store::SnapshotRetirementStep::Pending { released_items, released_bytes } => semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes }, store::SnapshotRetirementStep::Complete => semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }, store::SnapshotRetirementStep::Blocked => semio_framework_plugin::PluginCloseStep::Blocked { reason: "sequence media source retirement is blocked" } });
        }
        if let Some(mut params) = self.params.take() {
            self.retirement = Some(semio_framework_value::retirement::owned_retirement(std::mem::take(&mut params.0)));
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(media) = self.media.take() {
            self.retirement = Some(match media { MediaPayload::Intrinsic { schema, value } => semio_framework_value::retirement::owned_retirement((schema, value)), MediaPayload::Structured { schema, json } => semio_framework_value::retirement::owned_retirement((schema, json)), MediaPayload::Binary { format_kind, blob_hash } => semio_framework_value::retirement::owned_retirement((format_kind, blob_hash)) });
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(parser) = self.parser.take() {
            self.retirement = Some(semio_framework_value::retirement::owned_retirement(parser));
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(projection) = self.projection.take() {
            self.retirement = Some(semio_framework_value::retirement::owned_retirement(projection));
            self.decode_receipt = None;
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.port.is_empty() || self.port.capacity() > 0 {
            self.retirement = Some(semio_framework_value::retirement::owned_retirement(std::mem::take(&mut self.port)));
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.snapshot.as_ref().is_some_and(|snapshot| std::sync::Arc::strong_count(snapshot) == 1) {
            return Ok(semio_framework_plugin::PluginCloseStep::Blocked { reason: "sequence import snapshot has no mounted retained authority" });
        }
        if self.snapshot.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.completion.as_ref().is_some_and(|completion| !completion.has_mounted_consumer()) {
            return Ok(semio_framework_plugin::PluginCloseStep::Blocked { reason: "sequence import completion has no mounted consumer authority" });
        }
        if self.completion.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(semio_framework_plugin::PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.port.is_empty()
            && self.port.capacity() == 0
            && self.media.is_none() && self.parser.is_none() && self.projection.is_none() && self.input.is_none() && self.params.is_none() && self.retirement.is_none()
            && self.children.is_none()
            && self.snapshot.is_none()
            && self.emit.is_none()
            && self.completion.is_none()
            && self.pending_completion_rejection.is_none()
    }
}
//#endregion 🎞️ReservedImport

//#region 🔖️SequencePlayApp
/// 🧪️ Stateless app shell; exact window owners hold graph preferences and run output.
#[derive(Default)]
pub struct SequencePlayApp;

//#region 🧾️ProofCatalogs
struct SequenceArtifactProofs;
impl SequenceArtifactProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<SequencePlayApp>,
        owner_file: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.sequence.sequence@1/*#editor",
        artifact_schema: "sequence.sequence",
        factory: "SequenceRetainedArtifactJobFactory",
        factory_type: SequenceRetainedArtifactJobFactory,
        tools: {
            "addStep" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "addStepToSlot" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "addStepDropped" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "removeStep" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "deleteSelection" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "moveStep" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "connectSteps" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "disconnectSteps" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "setStepParams" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
            "setStepCollapsed" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 2_000, 1, 1),
        }
    }
}

struct SequencePersistentProofs;
impl SequencePersistentProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<SequencePlayApp>,
        owner_file: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.sequence.sequence@1/*#editor",
        artifact_schema: "sequence.sequence",
        factory: "SequencePersistentJobFactory",
        factory_type: SequencePersistentJobFactory,
        tools: {
            "reorganize" => semio_framework::ToolExecutionContract::resumable(4_096, 66_049, 1, 65_536, 7_500, 1, 1),
            "nodeGraphEdit" => semio_framework::ToolExecutionContract::resumable(4_096, 66_049, 1, 65_536, 7_500, 1, 1),
            "run" => semio_framework::ToolExecutionContract::resumable(4_096, 66_049, 1, 65_536, 7_500, 1, 1),
        }
    }
}

struct SequenceConfigProofs;
impl SequenceConfigProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<SequencePlayApp>,
        owner_file: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.sequence.sequence@1/*#editor",
        artifact_schema: "sequence.sequence",
        factory: "SequenceRetainedConfigJobFactory",
        factory_type: SequenceRetainedConfigJobFactory,
        tools: {
            "setViewport" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 4_096, 2_000, 1, 1),
            "setOrientation" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 4_096, 2_000, 1, 1),
            "stop" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 4_096, 2_000, 1, 1),
        }
    }
}

struct SequenceExampleProofs;
impl SequenceExampleProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<SequencePlayApp>,
        owner_file: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.sequence.sequence@1/*#editor",
        artifact_schema: "sequence.sequence",
        factory: "SequenceRetainedExampleJobFactory",
        factory_type: SequenceRetainedExampleJobFactory,
        tools: {
            "setActiveExample" => semio_framework::ToolExecutionContract::resumable(4_096, 2, 1, 65_536, 7_500, 1, 1),
        }
    }
}
//#endregion 🧾️ProofCatalogs

impl ArtifactEditor for SequencePlayApp {
    /// 🧩️ Composes `s.stdio.semio@v1/*` children, so every bundle of this surface opens them through the same roster.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = SequenceSnapshot;
    type Mutation = SequenceMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = SequenceCommand;

    const DIALECT: Dialect = crate::SEQUENCE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SEQUENCE_DOCUMENT_SCHEMA;

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("sequence.child.projection"), error.to_string()))
    }

    /// 🔔️ The localized notices of the editor's own refusal codes (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        sequence_fault_notices()
    }

    /// 🌱️ The derivable `content` member — see `crate::genesis_sequence_child_pack`.
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::genesis_sequence_child_pack(snapshot, slot, child_id)
    
})())
}

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<main::config::SequenceMainWindowConfigOwner>()
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        registry.register::<script::transient::SequenceScriptWindowTransientOwner>()
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        SequenceArtifactProofs::bounded_first_step_tool_proofs()
            .into_iter()
            .chain(SequencePersistentProofs::bounded_first_step_tool_proofs())
            .chain(SequenceConfigProofs::bounded_first_step_tool_proofs())
            .chain(SequenceExampleProofs::bounded_first_step_tool_proofs())
            .collect()
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(SequenceRetainedArtifactJobFactory::new(&controller))?;
        registry.register(SequencePersistentJobFactory::new(&controller))?;
        registry.register(SequenceRetainedConfigJobFactory::new(&controller))?;
        registry.register(SequenceRetainedExampleJobFactory::new(&controller))
    }

    /// 🎞️ `import-media` is the only reserved route Sequence owns. The framework registers the
    /// reserved factory but never a concrete importer, so every inbound `steps:in` delivery is routed
    /// exclusively through this builder (`dispatch_import_media` → `build_artifact_reserved_media_job`);
    /// `copy`/`cut`/`paste` stay on the framework's own reserved factories.
    fn build_reserved_tool_job(request: semio_framework_plugin::ArtifactReservedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ArtifactReservedToolJob>, Fault> {
        if request.tool_id.as_str() != SEQUENCE_IMPORT_TOOL_ID {
            return Ok(None);
        }
        if !request.raw_wire.is_empty() {
            return Err(sequence_fault("sequence.import-media.undecoded", "sequence import-media admits a decoded media value, never a wire payload"));
        }
        let semio_framework_plugin::ArtifactReservedToolInput::Media { port, media } = &request.input else {
            return Err(sequence_fault("sequence.import-media.missing", "sequence import-media requires media input"));
        };
        let (port, media) = (port.clone(), media.clone());
        Ok(Some(semio_framework_plugin::ArtifactReservedToolJob::new(SequenceImportJob::new(request, port, media))))
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        let artifact_route = SEQUENCE_RETAINED_ARTIFACT_TOOL_IDS.contains(&request.tool_id.as_str());
        let config_route = SEQUENCE_RETAINED_CONFIG_TOOL_IDS.contains(&request.tool_id.as_str());
        let persistent_route = SEQUENCE_PERSISTENT_TOOL_IDS.contains(&request.tool_id.as_str());
        let example_route = SEQUENCE_RETAINED_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str());
        if !artifact_route && !config_route && !persistent_route && !example_route {
            return Ok(None);
        }
        let persistent_admitted = match request.command.as_ref() {
            SequenceCommand::NodeGraphEdit(payload) => payload.operations_json.len() <= SEQUENCE_RETAINED_RAW_BYTES,
            SequenceCommand::Reorganize(_) | SequenceCommand::Run(_) => true,
            _ => false,
        };
        if request.command.command_id() != request.tool_id
            || (artifact_route && !sequence_retained_artifact_command_admitted(&request.command))
            || (config_route && !sequence_retained_config_command_admitted(&request.command))
            || (persistent_route && !persistent_admitted)
            || (example_route && !sequence_retained_example_command_admitted(&request.command))
        {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "Sequence command does not match its exact retained route or payload envelope"));
        }
        let tool_id = request.command.command_id();
        let operation_context = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<Self>>> = if persistent_route {
            Box::new(SequencePersistentWork::new(tool_id, &operation_context))
        } else if artifact_route {
            Box::new(SequenceRetainedArtifactWork::new(tool_id, &operation_context))
        } else if example_route {
            Box::new(SequenceRetainedExampleWork::new(tool_id, &operation_context))
        } else {
            Box::new(SequenceRetainedConfigWork::new(tool_id, &operation_context))
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            SequenceCommand::command_id,
            SEQUENCE_RETAINED_RAW_BYTES,
            if persistent_route {
                SEQUENCE_PERSISTENT_MAXIMUM_UNITS
            } else if artifact_route {
                SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS
            } else if example_route {
                SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_UNITS
            } else {
                SEQUENCE_RETAINED_MAXIMUM_UNITS
            },
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn initial_snapshot() -> SequenceSnapshot {
        crate::snapshot::schema::default_persisted_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(sequence_io())
    }

    /// 🎞️ `steps:in` (Wave-2 port recipe): inserts incoming computation results as a new step at the
    /// far right of the flow — an object payload becomes that step's params verbatim, a bare
    /// scalar/array is wrapped under a single `"value"` key. Never mutates anything directly (matches
    /// every other `import_media` override): the caller (a headless runner or the UI) applies the
    /// returned `create-step` mutation through the ordinary, undoable document store.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, SequenceSnapshot>) -> Result<Emit<SequenceMutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "steps:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "steps:in importer only accepts a Structured (JSON) payload".into()));
        };
        let value = json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let params_value = if value.as_object().is_some() { value } else { json!({ "value": value }) };
        let params: StepParams = semio_framework_pack_json::from_json_str(&params_value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let mut live = sequence_host_snapshot_from_children(doc.snapshot, &doc.children).map_err(|error| MediaError::Payload(port.to_string(), format!("{error:?}")))?;
        let id = format!("step-{}", max_serial_in_snapshot(&live).max(100) + 1);
        let x = live.steps.iter().map(|step| step.x).fold(0.0_f64, f64::max) + if live.steps.is_empty() { 0.0 } else { 280.0 };
        let base = ColdOwner::new(SequenceWorkingScene { steps: live.steps.clone(), edges: live.edges.clone() });
        live.steps.push(SequenceStep { id, kind: "computation.import".into(), params, x, y: 0.0, slot: None, collapsed: false });
        let next = ColdOwner::new(SequenceWorkingScene { steps: live.steps.clone(), edges: live.edges.clone() });
        Ok(sequence_child_leaves_emit(doc.snapshot, sequence_scene_leaves(&base, &next)))
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &SequenceCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps a host action id + its staged args onto `SequenceCommand`. The React/wgpu shells
    /// still dispatch `{action, args}` while the guest channel is typed-only, and the trait default
    /// refuses EVERY id (`app.command.unsupported`) — without this bridge no Actions-pane row, no
    /// palette drop and no graph gesture ever reached `handle`. Key aliases mirror the shells' own
    /// vocabularies (`value`/`id`, `nodeId`, `sourceNodeId`) rather than adding shell-side shims.
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<SequenceCommand, Fault> {
        let text_arg = |keys: &[&str]| keys.iter().find_map(|key| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string));
        let number_arg = |keys: &[&str]| keys.iter().find_map(|key| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_f64)).unwrap_or_default();
        let json_arg = |key: &str, fallback: &str| args.and_then(|value| value.get(key)).map_or_else(|| fallback.to_string(), json::to_json_string);
        match action {
            "addStep" => Ok(SequenceCommand::AddStep(add_step::AddStep { kind: text_arg(&["kind", "value"]).unwrap_or_else(|| "computation.import".into()), x: number_arg(&["x"]), y: number_arg(&["y"]) })),
            "addStepToSlot" => Ok(SequenceCommand::AddStepToSlot(add_step_to_slot::AddStepToSlot {
                kind: text_arg(&["kind", "value"]).unwrap_or_else(|| "computation.import".into()),
                x: number_arg(&["x"]),
                y: number_arg(&["y"]),
                owner: text_arg(&["owner"]).unwrap_or_default(),
                slot_name: text_arg(&["slotName", "slot_name", "slot"]).unwrap_or_default(),
            })),
            "addStepDropped" => Ok(SequenceCommand::AddStepDropped(add_step_dropped::AddStepDropped {
                kind: text_arg(&["kind", "value"]).unwrap_or_else(|| "computation.import".into()),
                x: number_arg(&["x"]),
                y: number_arg(&["y"]),
                picked_step_id: text_arg(&["pickedStepId", "picked_step_id"]),
            })),
            "removeStep" => Ok(SequenceCommand::RemoveStep(remove_step::RemoveStep { id: text_arg(&["id", "stepId", "value"]).unwrap_or_default() })),
            "deleteSelection" => Ok(SequenceCommand::DeleteSelection(delete_selection::DeleteSelection {})),
            "moveStep" => Ok(SequenceCommand::MoveStep(move_step::MoveStep { node_id: text_arg(&["nodeId", "node_id", "id"]).unwrap_or_default(), x: number_arg(&["x"]), y: number_arg(&["y"]) })),
            "connectSteps" => Ok(SequenceCommand::ConnectSteps(connect_steps::ConnectSteps { source_node_id: text_arg(&["sourceNodeId", "source_node_id", "from"]).unwrap_or_default(), target_node_id: text_arg(&["targetNodeId", "target_node_id", "to"]).unwrap_or_default() })),
            "disconnectSteps" => Ok(SequenceCommand::DisconnectSteps(disconnect_steps::DisconnectSteps { from_id: text_arg(&["fromId", "from_id", "from"]).unwrap_or_default(), to_id: text_arg(&["toId", "to_id", "to"]).unwrap_or_default() })),
            "setStepParams" => Ok(SequenceCommand::SetStepParams(set_step_params::SetStepParams { id: text_arg(&["id", "stepId"]).unwrap_or_default(), params_json: json_arg("params", "null") })),
            "setStepCollapsed" => Ok(SequenceCommand::SetStepCollapsed(set_step_collapsed::SetStepCollapsed { id: text_arg(&["id", "stepId", "value"]).unwrap_or_default() })),
            "reorganize" => Ok(SequenceCommand::Reorganize(reorganize::Reorganize {})),
            "nodeGraphEdit" => Ok(SequenceCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: json_arg("operations", "[]") })),
            "setOrientation" => Ok(SequenceCommand::SetOrientation(set_orientation::SetOrientation { value: text_arg(&["value", "orientation"]).unwrap_or_else(|| "horizontal".into()) })),
            "run" => Ok(SequenceCommand::Run(run_command::Run {})),
            "stop" => Ok(SequenceCommand::Stop(stop_command::Stop {})),
            "setViewport" => {
                let value = args.and_then(|value| value.get("camera")).or_else(|| args.and_then(|value| value.get("viewport"))).cloned().ok_or_else(|| sequence_fault("sequence.viewport.camera", "sequence setViewport requires a camera"))?;
                Ok(SequenceCommand::SetViewport(set_viewport::SetViewport { camera: semio_framework_value::FromValue::from_value(value).map_err(|error| sequence_fault("sequence.viewport.camera", format!("invalid sequence setViewport camera: {error}")))? }))
            }
            "setActiveExample" => Ok(SequenceCommand::SetActiveExample(set_active_example::SetActiveExample {
                example_id: text_arg(&["exampleId", "example_id", "id", "value"]).unwrap_or_else(|| crate::examples::demo::ID.into()),
            })),
            other => Err(sequence_fault("sequence.action.unhandled", format!("sequence: unhandled action id {other}"))),
        }
    }

    /// 🕹️ `deleteSelection`/`nodeGraphEdit` read the "steps" interaction domain directly (bypassing
    /// the `app_commands!`-generated `dispatch`, whose per-row `$module::handle(payload, doc, cfg)`
    /// signature is framework-fixed and has no `interaction` slot) — ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    fn handle(
        command: &SequenceCommand,
        doc: &ArtifactView<'_, SequenceSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<SequenceMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            SequenceCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction),
            _ => command.dispatch(doc, cfg),
        }
    }

    /// 🕹️ `steps`'s `HierarchyProvider::Topology` — every step is registered at the "step"
    /// granularity, parented to its control-flow slot owner (`SlotRef.owner`) when nested inside a
    /// `then`/`else`/`body` slot, or as a root otherwise — mirrors the document panel's own nesting
    /// (`build_step_tree_item`) so a deleted step's id auto-prunes out of the live selection.
    fn interaction_topology(doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        let ordered = sequence_working_scene_from_children(doc.snapshot, &doc.children)
            .map(|scene| scene.steps.iter().map(|step| TopologyNode { id: step.id.clone(), granularity: "step".into(), parent: step.slot.as_ref().map(|slot| slot.owner.clone()) }).collect())
            .unwrap_or_default();
        let mut domains = BTreeMap::new();
        domains.insert(SEQUENCE_INTERACTION_STEPS.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    
})())
}

    fn render(body_key: &str, doc: &ArtifactView<'_, SequenceSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let live = sequence_host_snapshot_from_children(doc.snapshot, &doc.children)
            .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("sequence.child-content", format!("{error:?}")))?;
        let config = main::config::current(cfg);
        let transient = SequenceScriptWindowTransient::default();
        let labels = sequence_play_labels(view_state);
        match body_key {
            SEQUENCE_PLAY_BODY_MAIN => main::render(&live, &config),
            SEQUENCE_PLAY_BODY_SCRIPT => script::render(&live, &transient),
            SEQUENCE_PLAY_BODY_COMPILED => compiled::render(&live),
            SEQUENCE_PLAY_BODY_ARTIFACT => document_panel::render(&live, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, SEQUENCE_PLAY_BODY_ARTIFACT)),
            SEQUENCE_PLAY_BODY_CATALOGUE => catalogue_panel::render(&live, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, SEQUENCE_PLAY_BODY_CATALOGUE)),
            // 🕹️ `render` carries no `InteractionView` (same gap as `context_menu` below — see ticket
            // 26/08/14's w3b-summary.md), so this always takes the "nothing selected" branch rather
            // than reading a stale/wrong selection.
            SEQUENCE_PLAY_BODY_INSPECTOR => inspection_panel::render(&live, &[], labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "sequence diagnostic admission failed")),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }

    /// 🕹️ `context_menu` carries no `InteractionView` (same gap as `render` above — see ticket
    /// 26/08/14's w3b-summary.md), so the selection-dependent rows built by
    /// `sequence_context_menu_items` below always take the "nothing selected" branch here rather than
    /// reading a stale/wrong selection.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, SequenceSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        _interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let live = sequence_host_snapshot_from_children(doc.snapshot, &doc.children)
            .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("sequence.child-content", format!("{error:?}")))?;
        let config = main::config::current(cfg);
        let transient = script::transient::current(transient);
        let labels = sequence_play_labels(view_state);
        match body_key {
            SEQUENCE_PLAY_BODY_MAIN => main::render(&live, &config),
            SEQUENCE_PLAY_BODY_SCRIPT => script::render(&live, &transient),
            SEQUENCE_PLAY_BODY_COMPILED => compiled::render(&live),
            SEQUENCE_PLAY_BODY_ARTIFACT => document_panel::render(&live, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, SEQUENCE_PLAY_BODY_ARTIFACT)),
            SEQUENCE_PLAY_BODY_CATALOGUE => catalogue_panel::render(&live, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, SEQUENCE_PLAY_BODY_CATALOGUE)),
            SEQUENCE_PLAY_BODY_INSPECTOR => inspection_panel::render(&live, &[], labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "sequence diagnostic admission failed")),
        }.map(semio_framework_plugin::built_to_component_tree)
    }

    fn context_menu(request: &ContextMenuRequest, _doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        sequence_context_menu_items(registry, view_state, request.surface.as_ref(), &[])
    }
}

/// 🗂️ Grouped disclosure: `run`/`stop`/`addStep` stay top-level (the most frequent verbs);
/// `reorganize` folds into the `transform` group and a single-node hit's `setStepCollapsed` folds
/// into the `selection` group; `deleteSelection` stays a direct destructive item last —
/// `organize_context_menu` (applied automatically at the `VcsArtifactApp::context_menu` funnel)
/// sorts the groups into `RIBBON_PARENT_CATEGORIES` order and inserts the pre-destructive separator
/// itself. Factored out of `ArtifactApp::context_menu` (which carries no `InteractionView` — ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) so a test can exercise the selection-dependent
/// rows directly with a real `selected` slice, matching `space`'s own precedent.
fn sequence_context_menu_items(registry: &AppActionRegistry, view_state: &semio_framework_plugin::ViewModel, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>, selected: &[String]) -> Vec<ContextMenuItemSpec> {
    use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};
    let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;

    let (nodes, edges) = selection_domains_from_surface(surface, selected, &[]);

    let mut menu = Menu::of(registry, view_state).action("run").action("stop").action("addStep").group("transform", |m| m.action("reorganize"));

    if nodes.len() == 1 {
        let id = nodes[0].clone();
        menu = menu.group("selection", |m| {
            m.item(ContextMenuItemSpec {
                id: "setStepCollapsed".into(),
                label: Some(if is_de { "Schritt einklappen".into() } else { "Toggle Collapsed".into() }),
                icon: Some("chevrons-up-down".into()),
                action: Some("setStepCollapsed".into()),
                args: Some(json::to_dsl_value(&json!({ "id": id }))),
                ..Default::default()
            })
        });
    }

    menu.item(node_graph_delete_selection_spec(semio_framework_plugin::delete_selection().resolve(view_state.terminology, view_state.locale), view_state, &nodes, &edges, NodeGraphDeleteDispatch::Direct)).build()
}
//#endregion 🔖️SequencePlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own
/// `definition()`. Only the leaf action/keybinding declarations (which have no dedicated `_def`
/// passthrough) are written out inline.
pub fn create_sequence_app() -> AppDefinition {
    Editor::builder(crate::SEQUENCE_DIALECT)
            .document(["semio", "sequence"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("sequence")
            .mode_def(edit::definition())
            .default_mode_id(edit::SEQUENCE_PLAY_MODE_EDIT)
            .window_kind_def(main::definition())
            .window_kind_def(script::definition())
            .window_kind_def(compiled::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
            .action_with(ActionDefinition::bounded_catalog("addStep", LocalizedLabel::native("Add Step", "Schritt hinzufügen"), ActionKind::Mutation).with_category("create"))
            .mutation("addStepToSlot", LocalizedLabel::native("Add Step To Slot", "Schritt zu Slot hinzufügen"))
            .mutation("addStepDropped", LocalizedLabel::native("Add Step Dropped", "Schritt per Ablegen hinzufügen"))
            .action_audience("addStepDropped", semio_framework_plugin::CapabilityAudience::Input)
            .mutation("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"))
            .action_destructive("removeStep")
            .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .action_destructive("deleteSelection")
            .mutation("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"))
            .mutation("connectSteps", LocalizedLabel::native("Connect Steps", "Schritte verbinden"))
            .mutation("disconnectSteps", LocalizedLabel::native("Disconnect Steps", "Schritte trennen"))
            .mutation("setStepParams", LocalizedLabel::native("Set Step Params", "Schrittparameter festlegen"))
            .action_with(ActionDefinition::bounded_catalog("setStepCollapsed", LocalizedLabel::native("Set Step Collapsed", "Schritt einklappen"), ActionKind::Mutation).with_category("selection"))
            .action_with(ActionDefinition::new("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "rotate-cw").with_category("transform"))
            .mutation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
            .view_action("setViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"))
            // 👁️ Main-window orientation is persisted by its exact config owner; run output is
            // ephemeral on the invoking Script window.
            .view_action("setOrientation", LocalizedLabel::native("Set Orientation", "Ausrichtung festlegen"))
            .action_with(ActionDefinition::new("run", LocalizedLabel::native("Run", "Ausführen"), ActionKind::View, "play").with_category("actions"))
            .action_with(ActionDefinition::new("stop", LocalizedLabel::native("Stop", "Stopp"), ActionKind::View, "play").with_category("actions"))
            // 📚️ The playground navbar dispatches `setActiveExample` on boot for its example
            // combobox, and the shell offers that combobox only to an app that declares the verb.
            // Undeclared, it was dropped before dispatch — this app's console ERROR at boot, and the
            // reason its committed example never reached the document store at all.
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            // 📝️ Staged argument forms for the panel-visible create + layout actions.
            .action_args("addStep", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("state.set", LocalizedLabel::native("Set State", "Zustand setzen")),
                    ActionArgOption::new("log.print", LocalizedLabel::native("Print", "Ausgeben")),
                    ActionArgOption::new("control.if", LocalizedLabel::native("If", "Wenn")),
                    ActionArgOption::new("control.while", LocalizedLabel::native("While", "Solange")),
                    ActionArgOption::new("math.add", LocalizedLabel::native("Add", "Addieren")),
                ]).default_value(&"log.print"),
            ])
            .action_args("setOrientation", vec![
                ActionArgDef::select("orientation", LocalizedLabel::native("Orientation", "Ausrichtung"), vec![
                    ActionArgOption::new("leftRight", LocalizedLabel::native("Left to Right", "Links nach rechts")),
                    ActionArgOption::new("topBottom", LocalizedLabel::native("Top to Bottom", "Oben nach unten")),
                ]).required(),
            ])
            .action_interactive_job("addStep", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addStepToSlot", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addStepDropped", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("removeStep", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("moveStep", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("connectSteps", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("disconnectSteps", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setStepParams", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setStepCollapsed", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("reorganize", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("nodeGraphEdit", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setOrientation", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("run", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("stop", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setViewport", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new(crate::examples::demo::ID, crate::examples::demo::label())])
                    .required()
                    .default_value(&crate::examples::demo::ID),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // 🕹️ First-class hover/selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
            // one domain over the step graph, `HierarchyProvider::Topology` (see
            // `SequencePlayApp::interaction_topology` above) from each step's own control-flow slot
            // nesting — `transitive: false` matches the pre-migration behavior exactly (deleting a
            // selected control step never cascaded into its `then`/`else`/`body` children).
            .interaction(InteractionDefinition {
                id: SEQUENCE_INTERACTION_STEPS.into(),
                label: LocalizedLabel::native("Steps", "Schritte"),
                granularities: vec![GranularityDefinition { id: SEQUENCE_INTERACTION_GRANULARITY.into(), label: LocalizedLabel::native("Step", "Schritt"), icon_id: "box".into() }],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(main::SEQUENCE_PLAY_WINDOW_MAIN, vec![InteractionRef::new(SEQUENCE_INTERACTION_STEPS)])
            .window_kind_action_refs(main::SEQUENCE_PLAY_WINDOW_MAIN, vec![
                "reorganize".into(), "nodeGraphEdit".into(), "setOrientation".into(), "setViewport".into(),
            ])
            .window_kind_action_refs(script::SEQUENCE_PLAY_WINDOW_SCRIPT, vec!["run".into(), "stop".into()])
            .io(sequence_io())
            // 🕳️ SDK gap (contract §2.4, confirmed absent on `EditorBuilder` as of this packet): the
            // pre-migration chain's trailing `.example_source(art_sequence_demo::source())` /
            // `.workflow("sequence", "Sequence", "graph")` calls are dropped here, not silently ported
            // — `Editor::builder(...)` has no such methods, and `PluginBuilder::editor::<E>(def)`
            // wraps `def` in `App { definition: def, examples: Vec::new() }`, discarding `App.examples`
            // even if it were populated. The subset's own `📚️examples/🎬️demo` facet (mounted at the
            // plugin root as `artifacts::sequence::examples::demo`) is the closest surviving carrier of this
            // content today.
            .action_describe("reorganize", LocalizedLabel::native("Lays out every step of the sequence automatically along the window's flow direction, overwriting their manual positions.", "Ordnet alle Schritte der Sequenz automatisch entlang der Flussrichtung des Fensters an und überschreibt ihre manuellen Positionen."))
            .action_describe("setOrientation", LocalizedLabel::native("Sets the flow direction (such as left to right or top to bottom) that Reorganize lays the steps out along; only the window's setting changes.", "Legt die Flussrichtung fest (etwa links nach rechts oder oben nach unten), entlang der Neu anordnen die Schritte auslegt; nur die Einstellung des Fensters ändert sich."))
            .action_describe("run", LocalizedLabel::native("Runs the sequence's compiled path once from its first step; the document is not changed.", "Führt den kompilierten Pfad der Sequenz einmal ab dem ersten Schritt aus; das Dokument ändert sich nicht."))
            .action_describe("stop", LocalizedLabel::native("Stops a running sequence; the document is not changed.", "Hält eine laufende Sequenz an; das Dokument ändert sich nicht."))
            .action_describe("addStep", LocalizedLabel::native("Adds a new step of the given kind to the sequence canvas at x, y.", "Fügt der Sequenzfläche an x, y einen neuen Schritt der angegebenen Art hinzu."))
            .action_describe("addStepToSlot", LocalizedLabel::native("Adds a new step of the given kind into a named slot of an owner step (such as the body of a loop) at x, y.", "Fügt einen neuen Schritt der angegebenen Art an x, y in einen benannten Slot eines Besitzerschritts ein (etwa den Rumpf einer Schleife)."))
            .action_describe("removeStep", LocalizedLabel::native("Removes one step by id from the sequence together with the flow edges attached to it.", "Entfernt einen Schritt anhand seiner Id samt der angeschlossenen Flusskanten aus der Sequenz."))
            .action_describe("deleteSelection", LocalizedLabel::native("Removes every currently selected step from the sequence together with their flow edges.", "Entfernt alle aktuell ausgewählten Schritte samt ihrer Flusskanten aus der Sequenz."))
            .action_describe("moveStep", LocalizedLabel::native("Moves one step to an absolute position x, y on the sequence canvas.", "Verschiebt einen Schritt an die absolute Position x, y der Sequenzfläche."))
            .action_describe("connectSteps", LocalizedLabel::native("Adds a flow edge from one step to another, so the second runs after the first.", "Fügt eine Flusskante von einem Schritt zu einem anderen hinzu, sodass der zweite nach dem ersten läuft."))
            .action_describe("disconnectSteps", LocalizedLabel::native("Removes the flow edge between two steps, so they no longer run in that order.", "Entfernt die Flusskante zwischen zwei Schritten, sodass sie nicht mehr in dieser Reihenfolge laufen."))
            .action_describe("setStepParams", LocalizedLabel::native("Replaces the parameters of one step with the given name-to-value map.", "Ersetzt die Parameter eines Schritts durch die angegebene Zuordnung von Namen zu Werten."))
            .action_describe("setStepCollapsed", LocalizedLabel::native("Collapses or expands one step on the canvas, hiding or showing its nested steps.", "Klappt einen Schritt auf der Fläche ein oder aus und verbirgt oder zeigt seine verschachtelten Schritte."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole sequence with the bundled demo sequence; any other example id changes nothing.", "Ersetzt die gesamte Sequenz durch die mitgelieferte Demo-Sequenz; jede andere Beispiel-Id ändert nichts."))
            .action_audience("nodeGraphEdit", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("setViewport", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_destructive("reorganize")
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must
/// be able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests
