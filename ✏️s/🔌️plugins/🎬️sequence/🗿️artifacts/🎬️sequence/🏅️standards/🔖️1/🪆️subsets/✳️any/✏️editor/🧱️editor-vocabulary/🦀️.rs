//! 🧱️ Sequence editor core: ids and constants, the typed media I/O surface, camera conversions and the fault vocabulary.

use semio_framework_plugin::{AppIo, ArtifactView, Emit, Fault, NoConfigMutation};
use semio_framework_plugin::app::ChildEmitPreparation;
use neural_engine::ColdOwner;
use semio_framework_os_infinite_directed_presentation::DagCamera;
use semio_framework_ui_locale::LocalizedLabel;
use crate::{SEQUENCE_DOCUMENT_SCHEMA, SequenceCamera, SequenceHostSnapshot, SequenceSnapshot, SequenceWorkingScene};
use semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
use crate::mutations::SequenceMutation;
use std::collections::VecDeque;
use crate::editor::sequence::edit_rules;

//#region 🔖️Constants
pub const SEQUENCE_PLAY_APP_ID: &str = "s.sequence.sequence@1/*#editor";

/// 🔁️ The whole-document replacement `setActiveExample` publishes — the sanctioned non-history
/// "replace the whole document" gesture (`ArtifactStore::reset`, applied host-side), which a
/// composed-child document needs because the taxonomy forbids a whole-snapshot mutation variant.
/// The spr is a fresh, edit-free op-log built with `store::os_store::text_format::empty_document_spr` — NEVER a live
/// `ArtifactEnvelope` minted just to print one: such an envelope is a terminal store shell whose
/// `Drop` asserts that its app-owned bounded retirement authority detached every nested owner first,
/// and nothing on this path ever mounts or retires it, so the envelope route traps the guest
/// (`🧊️process3d` lost its whole editor to exactly this).
pub fn reset_sequence_document_effect(document: &SequenceSnapshot) -> semio_framework_plugin::Effect {
    let pack = crate::standards::v1::subsets::any::io::snapshot_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr(SEQUENCE_PLAY_APP_ID, SEQUENCE_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}
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
use crate::mirror::viewer::sequence::modes::view::windows::main;

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
/// 🌊️ Publishes child intent `leaves` as ONE edit of the exact composed Flow content child; nothing is the empty emit.
pub fn sequence_child_leaves_emit(snapshot: &SequenceSnapshot, leaves: Vec<SemioFlowMutation>) -> Emit<SequenceMutation, NoConfigMutation> {
    if leaves.is_empty() {
        return Emit::default();
    }
    Emit { child_preparations: VecDeque::from([ChildEmitPreparation::of::<SemioFlowSnapshot, _>("content", &snapshot.content.child_id, leaves)]), ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }
}

/// ✏️ Starts one editing gesture over the exact working scene of the composed content child `doc` carries.
pub fn sequence_scene_edit(doc: &ArtifactView<'_, SequenceSnapshot>) -> Result<edit_rules::SceneEdit, Fault> {
    let scene = sequence_working_scene_from_children(doc.snapshot, &doc.children)?;
    Ok(edit_rules::SceneEdit::new(SequenceWorkingScene { steps: scene.steps.clone(), edges: scene.edges.clone() }))
}

/// 🌊️ Publishes the concrete leaves one finished gesture emitted as ONE edit of the content child; no leaf is the empty emit.
pub fn sequence_edit_emit(doc: &ArtifactView<'_, SequenceSnapshot>, edit: edit_rules::SceneEdit) -> Emit<SequenceMutation, NoConfigMutation> {
    sequence_child_leaves_emit(doc.snapshot, edit.leaves)
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
pub fn sequence_fault(code: &'static str, detail: impl Into<String>) -> Fault {
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
