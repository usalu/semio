//! 🎛️ Editor wiring of the live lanes: the gesture adapter from host phases and keys to the drag statechart, the ephemeral reducer of the five live commands, the lane frame the windows paint from and the macros that stamp the command rows into an editor. See [`super`].

use crate::{with_clock, now_ms, view_write_events, DragPreview, FieldWrite, GhostPreview, GhostSource, LaneEvent, LanePresence, LaneTransient, NormTransient, PeerDrag};
use crate::direct_manipulation::{complete_preview, derived_yaw, standing_pose, handle_placed_fields, is_placement_write, insert_edit, snap_point, InsertSpec, key_intent, DragEffect, DragEvent, DragPhase, DragState, HandleDragTool, Key, KeyIntent, Modifiers};
use crate::{dispatch_live_command, resolve_view_steps, LiveCommand, LiveOutcome, NormEdit, NormRetainedEditor};
use semio_framework_plugin::{ArtifactView, Emit, Fault, NoConfigMutation, PresenceView, TransientView};
use semio_framework_value::{DslValue, FromValue};
use semio_s_plugin_norm_structural_scene::projection::ViewOptions;
use semio_s_plugin_norm_structural_scene::projection_kit::{placements_of, Placement as PlacedRow};
use semio_s_plugin_norm_structural_scene::{Diagram2d, EntityKind, EntityRef, Handle, StructuralScene, Vec3};

//#region 🔖️Gesture
/// 🏷️ Prefix of the pick id of a drag grip in the viewport (`handle:<handle id>`).
pub const HANDLE_PICK_PREFIX: &str = "handle:";

/// 🎬️ One host input of a handle gesture: a pointer phase or a key, with the drag parameter `s`, the press origin and the typed entry so far.
#[derive(Clone, Debug, PartialEq)]
pub enum GestureStep {
    Phase { phase: DragPhase, s: f64, origin: f64, modifiers: Modifiers, entry: String },
    Key { key: Key, s: f64, origin: f64, modifiers: Modifiers, entry: String },
}

/// 🕹️ A handle gesture addressed to the handle `handle`; without an `entity` the handle is looked up among the handles of every entity.
#[derive(Clone, Debug, PartialEq)]
pub struct Gesture {
    pub entity: Option<EntityRef>,
    pub handle: String,
    pub step: GestureStep,
}

/// 📤️ What one gesture step does: lane events for the transient lane and, on commit, the writes of the one `setFields`.
#[derive(Clone, Debug, PartialEq)]
pub struct GestureOutcome {
    pub events: Vec<LaneEvent>,
    pub commit: Option<Vec<FieldWrite>>,
}

fn key_of(raw: &str) -> Option<Key> {
    Some(match raw {
        "Tab" => Key::Tab,
        "Enter" => Key::Enter,
        "Escape" => Key::Escape,
        "Backspace" => Key::Backspace,
        "ArrowLeft" => Key::ArrowLeft,
        "ArrowRight" => Key::ArrowRight,
        "ArrowUp" => Key::ArrowUp,
        "ArrowDown" => Key::ArrowDown,
        other if other.chars().count() == 1 => Key::Char(other.chars().next()?),
        _ => return None,
    })
}

fn flag(value: &DslValue, key: &str) -> bool {
    value.get("modifiers").and_then(|modifiers| modifiers.get(key)).or_else(|| value.get(key)).and_then(DslValue::as_bool).unwrap_or(false)
}

impl Gesture {
    /// 🔡️ Reads `{entity: {kind, id}, handle, s, origin?, modifiers?, entry?, phase (+ reason) | key}`; the modifiers are the object `modifiers: {shift, alt, ctrl}` or the same flags at the top level (a flat touch button carries `ctrl: true` to free its nudge from snapping); the failure names the offending member.
    pub fn parse(json: &str) -> Result<Self, String> {
        let value = semio_framework_pack_json::from_json_str::<DslValue>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        let text = |key: &str| value.get(key).and_then(DslValue::as_str).map(str::to_string);
        let entity = value
            .get("entity")
            .map(|entity| {
                let kind = EntityKind::from_value(DslValue::String(entity.get("kind").and_then(DslValue::as_str).ok_or("the gesture entity needs a 'kind'")?.to_string())).map_err(|error| error.to_string())?;
                let id = entity.get("id").and_then(DslValue::as_str).ok_or("the gesture entity needs an 'id'")?.to_string();
                Ok::<_, String>(EntityRef { kind, id, path: String::new() })
            })
            .transpose()?;
        let handle = text("handle").ok_or("a gesture needs a 'handle'")?;
        let handle = handle.strip_prefix(HANDLE_PICK_PREFIX).unwrap_or(&handle).to_string();
        let s = value.get("s").and_then(DslValue::as_f64).ok_or("a gesture needs the number 's'")?;
        let origin = value.get("origin").and_then(DslValue::as_f64).unwrap_or(s);
        let modifiers = Modifiers { shift: flag(&value, "shift"), alt: flag(&value, "alt"), ctrl: flag(&value, "ctrl") };
        let entry = text("entry").unwrap_or_default();
        let step = match (text("phase"), text("key")) {
            (Some(phase), None) => GestureStep::Phase { phase: DragPhase::parse(&phase, text("reason").as_deref()).ok_or_else(|| format!("unknown gesture phase '{phase}'"))?, s, origin, modifiers, entry },
            (None, Some(key)) => GestureStep::Key { key: key_of(&key).ok_or_else(|| format!("unknown gesture key '{key}'"))?, s, origin, modifiers, entry },
            _ => return Err("a gesture needs exactly one of 'phase' and 'key'".into()),
        };
        Ok(Self { entity, handle, step })
    }
}

fn effect_events(effect: DragEffect, outcome: &mut GestureOutcome) {
    match effect {
        DragEffect::None => {}
        DragEffect::Preview(preview) => outcome.events.push(LaneEvent::DragPreviewed(preview)),
        DragEffect::Commit(writes) => {
            outcome.events.extend([LaneEvent::DragCleared, LaneEvent::GhostCleared]);
            outcome.commit = Some(writes);
        }
        DragEffect::Cleared => outcome.events.push(LaneEvent::DragCleared),
    }
}

/// 🎬️ One gesture step over the open drag of `transient` (else a fresh press at the gesture origin): pointer phases and keys drive the same statechart, so a nudge and a drag to the nudged value agree.
pub fn gesture_step(handles: &[Handle], transient: &NormTransient, gesture: &Gesture, seq: u64) -> GestureOutcome {
    let tool = HandleDragTool { handles, seq };
    let (event, s, origin) = match &gesture.step {
        GestureStep::Phase { phase, s, origin, modifiers, .. } => (Some(phase.event(*s, *modifiers)), *s, *origin),
        GestureStep::Key { key, s, origin, modifiers, .. } => (
            match key_intent(key, *modifiers) {
                KeyIntent::Drag(event) => Some(event),
                KeyIntent::Cycle(_) | KeyIntent::Ignored => None,
            },
            *s,
            *origin,
        ),
    };
    let mut outcome = GestureOutcome { events: Vec::new(), commit: None };
    let Some(event) = event else { return outcome };
    let state = match &transient.drag {
        Some(drag) if drag.handle == gesture.handle => DragState::Dragging { handle: drag.handle.clone(), origin: drag.origin, s: drag.s, entry: drag.entry.clone() },
        _ if matches!(gesture.step, GestureStep::Phase { .. }) => DragState::Armed { handle: gesture.handle.clone(), origin },
        _ => DragState::Hover { handle: gesture.handle.clone(), s },
    };
    effect_events(tool.step(state, event).1, &mut outcome);
    outcome
}

/// 🧾️ The writes a committing gesture releases, replayed statelessly (press at the origin, move to `s`, typed entry, release): a commit at the origin writes nothing, anything but a commit is `None`.
pub fn commit_writes(handles: &[Handle], gesture: &Gesture) -> Option<Vec<FieldWrite>> {
    let (s, origin, modifiers, entry) = match &gesture.step {
        GestureStep::Phase { phase: DragPhase::Commit, s, origin, modifiers, entry } | GestureStep::Key { key: Key::Enter, s, origin, modifiers, entry } => (*s, *origin, *modifiers, entry),
        _ => return None,
    };
    let tool = HandleDragTool { handles, seq: 0 };
    let mut state = tool.step(DragState::Armed { handle: gesture.handle.clone(), origin }, DragEvent::Move { s, modifiers }).0;
    for c in entry.chars() {
        state = tool.step(state, DragEvent::Entry(c)).0;
    }
    match tool.step(state, DragEvent::Release).1 {
        DragEffect::Commit(writes) => Some(writes),
        _ => None,
    }
}
//#endregion 🔖️Gesture

//#region 🔖️Invocation
/// ➕ A placement of the insert tool: the tool, the cursor and whether snapping is freed.
#[derive(Clone, Debug, PartialEq)]
pub struct Placement {
    pub spec: String,
    pub at: [f64; 3],
    pub free: bool,
}

impl Placement {
    /// 🔡️ Reads `{spec, at: [x, y, z], free?}`; a placement needs the tool and the position.
    pub fn parse(json: &str) -> Result<Self, String> {
        let value = semio_framework_pack_json::from_json_str::<DslValue>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        let (spec, at, free) = crate::insert_arg(Some(&object(vec![("insert", value)])))?;
        Ok(Self { spec: spec.ok_or("a placement needs the tool 'spec'")?, at: at.ok_or("a placement needs the position 'at'")?, free })
    }
}

/// 🫧️ One live command with its parsed gesture or placement, as the ephemeral reducer and the document lane both read it.
#[derive(Clone, Debug, PartialEq)]
pub struct LiveInvocation {
    pub command: LiveCommand,
    pub gesture: Option<Gesture>,
    pub placement: Option<Placement>,
}

fn object(members: Vec<(&str, DslValue)>) -> DslValue {
    DslValue::object(members.into_iter().map(|(key, value)| (key.to_string(), value)).collect::<Vec<_>>())
}

fn blank(json: &str) -> bool {
    json.trim().is_empty() || json.trim() == "null"
}

impl LiveInvocation {
    /// ✍️ `setFields` from its `writes` JSON array, an optional gesture JSON and an optional placement JSON; a gesture or a placement carries no writes.
    pub fn set_fields(writes_json: &str, gesture_json: Option<&str>, insert_json: Option<&str>) -> Result<Self, String> {
        let gesture = gesture_json.filter(|json| !blank(json)).map(Gesture::parse).transpose()?;
        let placement = insert_json.filter(|json| !blank(json)).map(Placement::parse).transpose()?;
        let writes = match (&gesture, &placement) {
            (None, None) => semio_framework_pack_json::from_json_str::<DslValue>(writes_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?,
            _ if blank(writes_json) => DslValue::Array(Vec::new()),
            _ => semio_framework_pack_json::from_json_str::<DslValue>(writes_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?,
        };
        let command = LiveCommand::parse("setFields", Some(&object(vec![("writes", writes)])))?;
        Ok(Self { command, gesture, placement })
    }

    /// 👻️ `previewRemedy` of one remedy option, or the insert-tool ghost when `insert_json` (`{spec?, at?, free?}`) is given.
    pub fn preview_remedy(check_id: &str, remedy_index: u32, option_index: u32, insert_json: Option<&str>) -> Result<Self, String> {
        let insert = insert_json.map(|json| semio_framework_pack_json::from_json_str::<DslValue>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())).transpose()?;
        let command = match insert {
            Some(insert) => LiveCommand::parse("previewRemedy", Some(&object(vec![("insert", insert)])))?,
            None => LiveCommand::PreviewRemedy { check_id: check_id.to_string(), index: remedy_index as usize, option: option_index as usize },
        };
        Ok(Self { command, gesture: None, placement: None })
    }

    /// 🧹️ `clearPreview`.
    pub fn clear_preview() -> Self {
        Self { command: LiveCommand::ClearPreview, gesture: None, placement: None }
    }

    /// 🛑️ `cancelEvaluation`.
    pub fn cancel_evaluation() -> Self {
        Self { command: LiveCommand::CancelEvaluation, gesture: None, placement: None }
    }

    /// ⏳️ `evaluateUnits` of a JSON array of unit ids (absent = the whole plan) for a reason.
    pub fn evaluate_units(units_json: Option<&str>, reason: &str) -> Result<Self, String> {
        let units = units_json.map(|json| semio_framework_pack_json::from_json_str::<DslValue>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())).transpose()?;
        let args = object(vec![("units", units.unwrap_or(DslValue::Null)), ("reason", DslValue::String(reason.to_string()))]);
        Ok(Self { command: LiveCommand::parse("evaluateUnits", Some(&args))?, gesture: None, placement: None })
    }
}
//#endregion 🔖️Invocation

//#region 🔖️Editor
/// 🎛️ What a norm editor adds to be live: its handles, its entities, its insert palette, its edit resolution and the reading of its five live command rows.
pub trait NormLiveEditor: NormRetainedEditor {
    fn live_invocation(command: &Self::Command) -> Option<Result<LiveInvocation, String>>;
    fn view_event(command: &Self::Command) -> Option<LaneEvent>;
    fn live_handles(document: &Self::Snapshot, selection: &[EntityRef]) -> Vec<Handle>;
    fn live_entities(document: &Self::Snapshot) -> Vec<EntityRef>;
    fn live_palette() -> Vec<InsertSpec>;
    fn live_resolve(document: &Self::Snapshot, edit: &NormEdit) -> Result<Vec<Self::Mutation>, String>;
    fn live_scene(document: &Self::Snapshot, view: &ViewOptions) -> StructuralScene;

    /// 📈️ Every diagram of the document, the set whose marker rows can be typed into; an editor without diagrams keeps the empty default.
    fn live_diagrams(document: &Self::Snapshot) -> Vec<Diagram2d> {
        let _ = document;
        Vec::new()
    }
}

/// 🧍️ The pose an entity without a row stands in: the grips' origin and the yaw of the derived scene, which is projected only when a pose is asked and only once per committed document ([`committed_scene`]).
pub fn standing_of<'a, A: NormLiveEditor>(document: &'a A::Snapshot, handles: &'a [Handle]) -> impl Fn(&str) -> Option<PlacedRow> + 'a {
    let scene = std::cell::OnceCell::new();
    move |id| standing_pose(handles, id, &|id| derived_yaw(scene.get_or_init(|| committed_scene::<A>(document)), id))
}

struct CommittedScene {
    document: Box<dyn std::any::Any + Send>,
    scene: std::sync::Arc<StructuralScene>,
}

/// 🎬️ The scene of the committed `document` without view choices, retained for the document it was projected from: a drag asks for it on every pointer tick (derived yaw, snap targets) while the committed document stays the same, and every projection would also switch the retained evaluation away from the preview document the viewport paints.
pub fn committed_scene<A: NormLiveEditor>(document: &A::Snapshot) -> std::sync::Arc<StructuralScene> {
    static MEMO: std::sync::Mutex<std::collections::BTreeMap<&'static str, CommittedScene>> = std::sync::Mutex::new(std::collections::BTreeMap::new());
    let editor = std::any::type_name::<A>();
    let document: &<A::Family as semio_s_plugin_norm_compliance::NormFamily>::Document = document;
    let known = MEMO.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(editor).filter(|memo| same_document(memo.document.as_ref(), document)).map(|memo| std::sync::Arc::clone(&memo.scene));
    if let Some(scene) = known {
        return scene;
    }
    let scene = std::sync::Arc::new(A::live_scene(document, ViewOptions::none()));
    MEMO.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(editor, CommittedScene { document: Box::new(document.clone()), scene: std::sync::Arc::clone(&scene) });
    scene
}

fn fault(code: &'static str, message: String) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message)
}

/// 📈️ The writes of a value typed into the table-twin row of a diagram marker (`350 cm`) among `diagrams`: `None` when the gesture does not address a marker grip, `Some(None)` when it addresses one without a committing Enter, an unknown marker or a refused value; the unit-aware parse, limits and quantisation are those of [`marker_row_commit`](crate::entity_editing::marker_row_commit), so a typed row and a drag released at that value are one commit.
pub fn marker_row_writes_in(diagrams: &[Diagram2d], gesture: &Gesture) -> Option<Option<Vec<FieldWrite>>> {
    gesture.handle.starts_with(crate::diagrams2d::MARKER_GRIP_PREFIX).then(|| {
        let GestureStep::Key { key: Key::Enter, entry, .. } = &gesture.step else { return None };
        let (diagram, marker) = diagrams.iter().find_map(|diagram| diagram.markers.iter().find(|marker| crate::diagrams2d::marker_grip_id(diagram, marker) == gesture.handle).map(|marker| (diagram, marker)))?;
        crate::entity_editing::marker_row_commit(diagram, marker, entry)
    })
}

/// 📈️ [`marker_row_writes_in`] over the diagrams of the editor's document; the diagrams are only built for a gesture that addresses a marker.
pub fn marker_row_writes<A: NormLiveEditor>(document: &A::Snapshot, gesture: &Gesture) -> Option<Option<Vec<FieldWrite>>> {
    gesture.handle.starts_with(crate::diagrams2d::MARKER_GRIP_PREFIX).then(|| marker_row_writes_in(&A::live_diagrams(document), gesture)).flatten()
}

/// 🕹️ The handles a gesture can address: those of its entity, else those of every entity filtered by the handle id. The dragged handle also snaps to the geometry of the scene (member ends and midpoints along its axis), so the preview and the commit agree. Resolved once per (committed document, entity, handle): every further tick of the same drag is answered from the retained entry, so the scene is not projected and the retained evaluation is not switched to the committed document on every pointer move.
pub fn gesture_handles<A: NormLiveEditor>(document: &A::Snapshot, gesture: &Gesture) -> Vec<Handle> {
    static MEMO: std::sync::Mutex<std::collections::BTreeMap<&'static str, GestureHandles>> = std::sync::Mutex::new(std::collections::BTreeMap::new());
    let editor = std::any::type_name::<A>();
    let document: &<A::Family as semio_s_plugin_norm_compliance::NormFamily>::Document = document;
    let known = MEMO.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(editor).filter(|memo| memo.entity == gesture.entity && memo.handle == gesture.handle && same_document(memo.document.as_ref(), document)).map(|memo| memo.handles.clone());
    if let Some(handles) = known {
        return handles;
    }
    let handles = gesture_handles_uncached::<A>(document, gesture);
    MEMO.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(editor, GestureHandles { document: Box::new(document.clone()), entity: gesture.entity.clone(), handle: gesture.handle.clone(), handles: handles.clone() });
    handles
}

fn same_document<D: PartialEq + 'static>(retained: &(dyn std::any::Any + Send), document: &D) -> bool {
    retained.downcast_ref::<D>() == Some(document)
}

struct GestureHandles {
    document: Box<dyn std::any::Any + Send>,
    entity: Option<EntityRef>,
    handle: String,
    handles: Vec<Handle>,
}

/// 🕹️ [`gesture_handles`] resolved from scratch: the oracle of the retained entry.
pub fn gesture_handles_uncached<A: NormLiveEditor>(document: &A::Snapshot, gesture: &Gesture) -> Vec<Handle> {
    let handles = match &gesture.entity {
        Some(entity) => A::live_handles(document, std::slice::from_ref(entity)),
        None => A::live_handles(document, &A::live_entities(document)).into_iter().filter(|handle| handle.id == gesture.handle).collect(),
    };
    if !handles.iter().any(|handle| handle.id == gesture.handle) {
        return handles;
    }
    let scene = committed_scene::<A>(document);
    handles.into_iter().map(|handle| if handle.id == gesture.handle { crate::entity_editing::with_scene_targets(handle, &scene) } else { handle }).collect()
}

fn placement_edit<A: NormLiveEditor>(document: &A::Snapshot, placement: &Placement) -> Result<NormEdit, String>
where
    A::Snapshot: semio_framework_value::ToValue,
{
    let spec = A::live_palette().into_iter().find(|spec| spec.id == placement.spec).ok_or_else(|| format!("unknown insert tool '{}'", placement.spec))?;
    insert_edit(&semio_framework_value::ToValue::to_value(document), &spec, snap_point(placement.at, placement.free))
}

/// 📤️ The document lane of `setFields`: a plain bundle commits all its writes as one edit, its placement leaves through the shared `PlacementCommit` (an entity without a row is ONE insert of its standing pose with the dragged leaves); a gesture commits only on release (or Enter) with the writes the statechart releases; a placement commits one `insertItem` edit at the snapped cursor; every other gesture step leaves the document alone.
pub fn live_set_fields<A: NormLiveEditor>(document: &A::Snapshot, writes_json: &str, gesture_json: Option<&str>, insert_json: Option<&str>) -> Result<Emit<A::Mutation, NoConfigMutation>, Fault> {
    let invocation = LiveInvocation::set_fields(writes_json, gesture_json, insert_json).map_err(|message| fault("norm.set-fields-args", message))?;
    if let Some(placement) = &invocation.placement {
        let edit = placement_edit::<A>(document, placement).map_err(|message| fault("norm.insert-placement", message))?;
        let mutations = A::live_resolve(document, &edit).map_err(|message| fault("norm.insert-placement", message))?;
        return crate::commit_snapshot_fields(mutations).map(|emit| Emit { artifact_mutations: emit.artifact_mutations, ..Emit::default() });
    }
    let dragged = invocation.gesture.as_ref().map(|gesture| gesture_handles::<A>(document, gesture));
    let writes = match (&invocation.gesture, &dragged, &invocation.command) {
        (Some(gesture), Some(handles), _) => match marker_row_writes::<A>(document, gesture) {
            Some(Some(writes)) => writes,
            Some(None) => return Ok(Emit::default()),
            None => match commit_writes(handles, gesture) {
                Some(writes) => writes,
                None => return Ok(Emit::default()),
            },
        },
        (_, _, LiveCommand::SetFields { writes }) => writes.clone(),
        _ => return Ok(Emit::default()),
    };
    let handles = match dragged {
        Some(handles) => handles,
        None if writes.iter().any(is_placement_write) => A::live_handles(document, &A::live_entities(document)),
        None => Vec::new(),
    };
    let standing = standing_of::<A>(document, &handles);
    let emit = handle_placed_fields(document, &writes, &standing, |d, e| A::live_resolve(d, e));
    emit
}

/// 📤️ The document lane of `duplicateEntities`, `pasteEntities`, `moveEntity` and `deleteEntities` (one atomic batch of the per-kind removals, last position first): copies get document-unique fresh ids and sit one `PASTE_STEP` off their originals (rows and persisted placement rows alike), a move becomes one `MoveItem` edit; the family edit rules resolve all of it into ONE batched edit (one undo step). `setFields`, the previews, the evaluation commands and `copyEntities` leave the document alone.
pub fn live_entity_edit<A: NormLiveEditor>(document: &A::Snapshot, command: &LiveCommand) -> Result<Emit<A::Mutation, NoConfigMutation>, Fault>
where
    A::Snapshot: semio_framework_value::ToValue,
{
    let tree = semio_framework_value::ToValue::to_value(document);
    let edits = match command {
        LiveCommand::DuplicateEntities { collection, ids } => crate::entity_editing::duplicate_edits(&tree, collection, ids, &crate::entity_editing::paste_offsets()).map(|pasted| pasted.edits),
        LiveCommand::PasteEntities { clip_json, at } => crate::entity_editing::clip_from_json(clip_json).and_then(|clip| crate::entity_editing::paste_edits(&tree, &clip, *at, &crate::entity_editing::paste_offsets())).map(|pasted| pasted.edits),
        LiveCommand::MoveEntity { path, id, target } => crate::move_edit(&tree, path, id, target).map(|edit| vec![edit]),
        LiveCommand::DeleteEntities { collection, ids } => Ok(crate::entity_editing::delete_edits(&tree, collection, ids).map_err(|refusal| fault(refusal.code(), refusal.message(false)))?),
        LiveCommand::SetFields { .. } | LiveCommand::PreviewRemedy { .. } | LiveCommand::PreviewInsert { .. } | LiveCommand::ClearPreview | LiveCommand::CancelEvaluation | LiveCommand::EvaluateUnits { .. } | LiveCommand::CopyEntities { .. } => return Ok(Emit::default()),
    }
    .map_err(|message| fault("norm.entity-edit", message))?;
    crate::entity_editing::handle_edits(document, &edits, |d, e| A::live_resolve(d, e)).map(|emit| Emit { artifact_mutations: emit.artifact_mutations, ..Emit::default() })
}

/// 📋️ The copy of `copyEntities`: the rows of the selection become a clip that the host keeps for the next paste; the document stays as it is.
pub fn live_copy_entities<A: NormLiveEditor>(document: &A::Snapshot, collection: &str, ids_json: &str) -> Result<Emit<A::Mutation, NoConfigMutation>, Fault>
where
    A::Snapshot: semio_framework_value::ToValue,
{
    let ids: Vec<String> = entities_args(collection, ids_json)?.get("ids").and_then(DslValue::as_array).map(|ids| ids.iter().filter_map(|id| id.as_str().map(str::to_owned)).collect()).unwrap_or_default();
    let clip = crate::entity_editing::copy_clip_json(&semio_framework_value::ToValue::to_value(document), collection, &ids).map_err(|message| fault("norm.entity-copy", message))?;
    crate::remember_clip(clip);
    Ok(Emit::default())
}

/// 🍽️ The context menu of a norm editor for a request: the pure rows of [`context_menu_items`] over the document tree, the published host context and the governing check of the report; `german` is the language of the viewer.
pub fn live_context_menu<A: NormRetainedEditor>(request: &semio_framework_plugin::ContextMenuRequest, document: &A::Snapshot, german: bool) -> Vec<semio_framework_plugin::ContextMenuItemSpec>
where
    A::Snapshot: semio_framework_value::ToValue,
{
    let report = <A::Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(document);
    crate::context_menu_items(&semio_framework_value::ToValue::to_value(document), &crate::request_probe(request), &crate::host_context(), &|id| crate::governing_check_index(&report, id), german)
}

/// 🏷️ The `{collection, ids}` args of an entity command from the payload strings (`ids_json` is a JSON string array).
pub fn entities_args(collection: &str, ids_json: &str) -> Result<DslValue, Fault> {
    let ids = semio_framework_pack_json::from_json_str::<DslValue>(ids_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| fault("norm.entity-args", error.to_string()))?;
    Ok(object(vec![("collection", DslValue::String(collection.to_string())), ("ids", ids)]))
}

fn armed_ghost(spec: &InsertSpec, at: [f64; 3]) -> LaneEvent {
    LaneEvent::GhostPreviewed(GhostPreview { writes: Vec::new(), source: GhostSource::Insert { spec: spec.id.clone(), kind: spec.kind, at: Vec3 { x: at[0], y: at[1], z: at[2] } } })
}

/// 🎬️ The lane events of one live invocation over the open transient.
pub fn lane_events<A: NormLiveEditor>(invocation: &LiveInvocation, document: &A::Snapshot, transient: &NormTransient) -> Vec<LaneEvent>
where
    A::Snapshot: Clone + semio_framework_value::ToValue,
{
    if let Some(gesture) = &invocation.gesture {
        let handles = gesture_handles::<A>(document, gesture);
        let standing = standing_of::<A>(document, &handles);
        return gesture_step(&handles, transient, gesture, 0)
            .events
            .into_iter()
            .map(|event| match event {
                LaneEvent::DragPreviewed(preview) => {
                    let rows = if preview.writes.iter().any(is_placement_write) { placements_of(document) } else { Vec::new() };
                    LaneEvent::DragPreviewed(DragPreview { writes: complete_preview(&rows, &standing, preview.writes), ..preview })
                }
                other => other,
            })
            .collect();
    }
    if invocation.placement.is_some() {
        return vec![LaneEvent::GhostCleared, LaneEvent::DragCleared];
    }
    if let LiveCommand::PreviewInsert { spec, at, free } = &invocation.command {
        let current = transient.ghost.as_ref().and_then(|ghost| match &ghost.source {
            GhostSource::Insert { spec, at, .. } => Some((spec.clone(), [at.x, at.y, at.z])),
            _ => None,
        });
        let palette = A::live_palette();
        let chosen = spec.clone().or_else(|| current.as_ref().map(|(spec, _)| spec.clone()));
        let tool = chosen.and_then(|id| palette.into_iter().find(|candidate| candidate.id == id));
        let position = at.or_else(|| current.map(|(_, at)| at)).unwrap_or([0.0; 3]);
        return tool.map_or_else(|| vec![LaneEvent::GhostCleared], |tool| vec![armed_ghost(&tool, snap_point(position, *free))]);
    }
    let handles = if matches!(&invocation.command, LiveCommand::SetFields { writes } if writes.iter().any(is_placement_write)) { A::live_handles(document, &A::live_entities(document)) } else { Vec::new() };
    let standing = standing_of::<A>(document, &handles);
    let events = match dispatch_live_command::<A::Family, _>(document, &invocation.command, &standing, |d, e| A::live_resolve(d, e)) {
        Ok(LiveOutcome::Lanes(events)) => events,
        Ok(LiveOutcome::Mutations(_)) => {
            let mut events = vec![LaneEvent::DragCleared, LaneEvent::GhostCleared];
            if let LiveCommand::SetFields { writes } = &invocation.command {
                events.extend(view_write_events(writes));
            }
            events
        }
        Err(_) => Vec::new(),
    };
    events
}

/// 🫧️ The ephemeral reducer of the live commands (an open touch Edit menu closes on any command but its own open and close, so an action ends the menu): folds the lane events of the command into the transient (and mirrors the open drags of the peers into it), publishes the open drag on the presence through the 30 Hz gate and drives the evaluation debounce; nothing here touches the document.
pub fn live_ephemeral<A: NormLiveEditor>(command: &A::Command, doc: &ArtifactView<'_, A::Snapshot>, presence: &PresenceView<'_, A::Presence>, transient: &TransientView<'_, A::Transient>) -> (Vec<A::PresenceMutation>, Vec<A::TransientMutation>)
where
    A::Snapshot: Clone + semio_framework_value::ToValue,
    A::Transient: LaneTransient<Mutation = A::TransientMutation>,
    A::Presence: LanePresence<Mutation = A::PresenceMutation>,
{
    let current = transient.snapshot.lane();
    let view_event = A::view_event(command);
    let closes_menu = crate::menu_closes(current.view.menu, view_event.as_ref());
    let mut events = match (view_event, A::live_invocation(command)) {
        (Some(event), _) => vec![event],
        (None, Some(Ok(invocation))) => lane_events::<A>(&invocation, doc.snapshot, &current),
        _ if closes_menu => Vec::new(),
        _ => return (Vec::new(), Vec::new()),
    };
    if closes_menu {
        events.push(LaneEvent::ViewMenu(false));
    }
    if events.iter().any(|event| matches!(event, LaneEvent::ViewStep { .. })) {
        let report = <A::Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(doc.snapshot);
        events = resolve_view_steps(events, &report, || A::live_scene(doc.snapshot, &crate::view_options_of(&current.view)), &current.view);
    }
    let peers: Vec<PeerDrag> = presence.peers.iter().filter_map(|(user, peer)| peer.lane().drag.map(|drag| PeerDrag { user: user.to_string(), drag })).collect();
    events.push(LaneEvent::PeersSeen(peers));
    let next = events.into_iter().fold(current.clone(), NormTransient::apply);
    let now = now_ms();
    with_clock(|clock| match (&current.drag, &next.drag) {
        (_, Some(drag)) if current.drag.as_ref() != Some(drag) => clock.tick(now),
        (Some(_), None) => clock.commit(),
        _ => {}
    });
    let transient_mutations = if next == current { Vec::new() } else { vec![A::Transient::replace(&next)] };
    let mut local = presence.local.lane();
    let presence_mutations = if local.drag == next.drag {
        Vec::new()
    } else {
        local.drag = next.drag.clone();
        if with_clock(|clock| clock.admit_presence(&local, now)) { vec![A::Presence::replace(&local)] } else { Vec::new() }
    };
    (presence_mutations, transient_mutations)
}

/// 🗺️ The interaction topology of a live editor for `ArtifactEditor::interaction_topology`: the entities of its scene, the rows of its outliner collections and its check rows, in the interaction `domain`, so the framework's `selectAll` needs no payload.
pub fn live_interaction_topology<A: NormLiveEditor>(document: &A::Snapshot, domain: &str) -> Result<semio_framework_plugin::InteractionTopology, semio_framework_value::ValueError>
where
    A::Snapshot: semio_framework_value::ToValue,
{
    let report = <A::Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(document);
    let scene = A::live_scene(document, ViewOptions::none());
    Ok(crate::norm_interaction_topology(domain, &semio_framework_value::ToValue::to_value(document), &scene, &report))
}

/// 🔗️ The view effects of the click on check row `check`: the framework selection of its owner entity in the interaction `domain` (the viewport then frames it, the lane carries the check); none for a document-level check or an unknown row.
pub fn live_check_focus_effects<A: NormLiveEditor>(document: &A::Snapshot, check: Option<u32>, domain: &str) -> Vec<semio_framework_plugin::Effect> {
    let Some(index) = check else { return Vec::new() };
    let report = <A::Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(document);
    crate::check_selection_effect(domain, &report, index as usize).into_iter().collect()
}

/// 🎹️ A check step publishes the same canonical interaction check identity as clicking its row.
pub fn live_view_focus_effects<A: NormLiveEditor>(command: &A::Command, document: &A::Snapshot, transient: &A::Transient, interaction: &protocol::InteractionState, domain: &str) -> Vec<semio_framework_plugin::Effect>
where
    A::Transient: LaneTransient<Mutation = A::TransientMutation>,
{
    let Some(event @ LaneEvent::ViewStep { step: crate::ViewStep::Check | crate::ViewStep::Governing, .. }) = A::view_event(command) else { return Vec::new() };
    let report = <A::Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(document);
    let lane = transient.lane().view;
    let picked = interaction.selection.get(domain).map(|selection| selection.ids.as_slice()).unwrap_or_default();
    crate::check_step_selection_effect(domain, &report, &lane, picked, event, || A::live_scene(document, &crate::view_options_of(&lane))).into_iter().collect()
}
//#endregion 🔖️Editor

/// 🧱️ Declares the five live command modules of an editor in the invoking module (`set_fields`, `preview_remedy`, `clear_preview`, `cancel_evaluation`, `evaluate_units`), each with its payload and `handle`.
#[macro_export]
macro_rules! norm_live_commands {
    ($app:ty) => {
        pub mod set_fields {
            #[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
            #[dsl(keyword = "set-fields")]
            pub struct SetFields {
                pub writes_json: String,
                pub gesture_json: Option<String>,
                pub insert_json: Option<String>,
            }
            pub fn handle(payload: &SetFields, doc: &semio_framework_plugin::ArtifactView<'_, <$app as semio_framework_plugin::ArtifactEditor>::Snapshot>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> Result<semio_framework_plugin::Emit<<$app as semio_framework_plugin::ArtifactEditor>::Mutation, semio_framework_plugin::NoConfigMutation>, semio_framework_plugin::Fault> {
                $crate::live_set_fields::<$app>(doc.snapshot, &payload.writes_json, payload.gesture_json.as_deref(), payload.insert_json.as_deref())
            }
        }
        pub mod preview_remedy {
            #[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
            #[dsl(keyword = "preview-remedy")]
            pub struct PreviewRemedy {
                pub check_id: String,
                pub remedy_index: u32,
                pub option_index: u32,
                pub insert_json: Option<String>,
            }
            pub fn handle(_payload: &PreviewRemedy, _doc: &semio_framework_plugin::ArtifactView<'_, <$app as semio_framework_plugin::ArtifactEditor>::Snapshot>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> Result<semio_framework_plugin::Emit<<$app as semio_framework_plugin::ArtifactEditor>::Mutation, semio_framework_plugin::NoConfigMutation>, semio_framework_plugin::Fault> {
                Ok(semio_framework_plugin::Emit::default())
            }
        }
        pub mod clear_preview {
            #[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
            #[dsl(keyword = "clear-preview")]
            pub struct ClearPreview {}
            pub fn handle(_payload: &ClearPreview, _doc: &semio_framework_plugin::ArtifactView<'_, <$app as semio_framework_plugin::ArtifactEditor>::Snapshot>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> Result<semio_framework_plugin::Emit<<$app as semio_framework_plugin::ArtifactEditor>::Mutation, semio_framework_plugin::NoConfigMutation>, semio_framework_plugin::Fault> {
                Ok(semio_framework_plugin::Emit::default())
            }
        }
        pub mod cancel_evaluation {
            #[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
            #[dsl(keyword = "cancel-evaluation")]
            pub struct CancelEvaluation {}
            pub fn handle(_payload: &CancelEvaluation, _doc: &semio_framework_plugin::ArtifactView<'_, <$app as semio_framework_plugin::ArtifactEditor>::Snapshot>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> Result<semio_framework_plugin::Emit<<$app as semio_framework_plugin::ArtifactEditor>::Mutation, semio_framework_plugin::NoConfigMutation>, semio_framework_plugin::Fault> {
                <<$app as $crate::NormRetainedEditor>::Family as semio_s_plugin_norm_compliance::NormFamily>::cancel_live();
                Ok(semio_framework_plugin::Emit::default())
            }
        }
        pub mod viewport_act {
            #[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
            #[dsl(keyword = "viewport-act")]
            pub struct ViewportAct {
                pub action: String,
                pub text: Option<String>,
                pub number: Option<f64>,
            }
            pub fn handle(_payload: &ViewportAct, _doc: &semio_framework_plugin::ArtifactView<'_, <$app as semio_framework_plugin::ArtifactEditor>::Snapshot>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> Result<semio_framework_plugin::Emit<<$app as semio_framework_plugin::ArtifactEditor>::Mutation, semio_framework_plugin::NoConfigMutation>, semio_framework_plugin::Fault> {
                Ok(semio_framework_plugin::Emit::default())
            }
        }
        pub mod copy_entities {
            #[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
            #[dsl(keyword = "copy-entities")]
            pub struct CopyEntities {
                pub collection: String,
                pub ids_json: String,
            }
            pub fn handle(payload: &CopyEntities, doc: &semio_framework_plugin::ArtifactView<'_, <$app as semio_framework_plugin::ArtifactEditor>::Snapshot>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> Result<semio_framework_plugin::Emit<<$app as semio_framework_plugin::ArtifactEditor>::Mutation, semio_framework_plugin::NoConfigMutation>, semio_framework_plugin::Fault> {
                $crate::live_copy_entities::<$app>(doc.snapshot, &payload.collection, &payload.ids_json)
            }
        }
        pub mod duplicate_entities {
            #[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
            #[dsl(keyword = "duplicate-entities")]
            pub struct DuplicateEntities {
                pub collection: String,
                pub ids_json: String,
            }
            pub fn handle(payload: &DuplicateEntities, doc: &semio_framework_plugin::ArtifactView<'_, <$app as semio_framework_plugin::ArtifactEditor>::Snapshot>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> Result<semio_framework_plugin::Emit<<$app as semio_framework_plugin::ArtifactEditor>::Mutation, semio_framework_plugin::NoConfigMutation>, semio_framework_plugin::Fault> {
                let command = $crate::LiveCommand::parse("duplicateEntities", Some(&$crate::entities_args(&payload.collection, &payload.ids_json)?)).map_err(|message| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("norm.entity-args"), message))?;
                $crate::live_entity_edit::<$app>(doc.snapshot, &command)
            }
        }
        pub mod paste_entities {
            #[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
            #[dsl(keyword = "paste-entities")]
            pub struct PasteEntities {
                pub clip_json: String,
                pub at: Option<u32>,
            }
            pub fn handle(payload: &PasteEntities, doc: &semio_framework_plugin::ArtifactView<'_, <$app as semio_framework_plugin::ArtifactEditor>::Snapshot>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> Result<semio_framework_plugin::Emit<<$app as semio_framework_plugin::ArtifactEditor>::Mutation, semio_framework_plugin::NoConfigMutation>, semio_framework_plugin::Fault> {
                $crate::live_entity_edit::<$app>(doc.snapshot, &$crate::LiveCommand::PasteEntities { clip_json: payload.clip_json.clone(), at: payload.at.map(|at| at as usize) })
            }
        }
        pub mod delete_entities {
            #[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
            #[dsl(keyword = "delete-entities")]
            pub struct DeleteEntities {
                pub collection: String,
                pub ids_json: String,
            }
            pub fn handle(payload: &DeleteEntities, doc: &semio_framework_plugin::ArtifactView<'_, <$app as semio_framework_plugin::ArtifactEditor>::Snapshot>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> Result<semio_framework_plugin::Emit<<$app as semio_framework_plugin::ArtifactEditor>::Mutation, semio_framework_plugin::NoConfigMutation>, semio_framework_plugin::Fault> {
                let command = $crate::LiveCommand::parse("deleteEntities", Some(&$crate::entities_args(&payload.collection, &payload.ids_json)?)).map_err(|message| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("norm.entity-args"), message))?;
                $crate::live_entity_edit::<$app>(doc.snapshot, &command)
            }
        }
        pub mod move_entity {
            #[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
            #[dsl(keyword = "move-entity")]
            pub struct MoveEntity {
                pub path: String,
                pub id: String,
                pub to: Option<u32>,
                pub over: Option<String>,
                pub step: Option<f64>,
            }
            pub fn handle(payload: &MoveEntity, doc: &semio_framework_plugin::ArtifactView<'_, <$app as semio_framework_plugin::ArtifactEditor>::Snapshot>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> Result<semio_framework_plugin::Emit<<$app as semio_framework_plugin::ArtifactEditor>::Mutation, semio_framework_plugin::NoConfigMutation>, semio_framework_plugin::Fault> {
                let target = $crate::move_target(payload.to, payload.over.as_deref(), payload.step).map_err(|message| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("norm.entity-args"), message))?;
                $crate::live_entity_edit::<$app>(doc.snapshot, &$crate::LiveCommand::MoveEntity { path: payload.path.clone(), id: payload.id.clone(), target })
            }
        }
        pub mod evaluate_units {
            #[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
            #[dsl(keyword = "evaluate-units")]
            pub struct EvaluateUnits {
                pub units_json: Option<String>,
                pub reason: String,
            }
            pub fn handle(_payload: &EvaluateUnits, doc: &semio_framework_plugin::ArtifactView<'_, <$app as semio_framework_plugin::ArtifactEditor>::Snapshot>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> Result<semio_framework_plugin::Emit<<$app as semio_framework_plugin::ArtifactEditor>::Mutation, semio_framework_plugin::NoConfigMutation>, semio_framework_plugin::Fault> {
                let _ = <<$app as $crate::NormRetainedEditor>::Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(doc.snapshot);
                Ok(semio_framework_plugin::Emit::default())
            }
        }
    };
}

/// 🎛️ Implements [`NormLiveEditor`] for an editor from its command enum (rows `SetFields`, `PreviewRemedy`, `ClearPreview`, `CancelEvaluation`, `EvaluateUnits`), its handle function `fn(&Snapshot, &[EntityRef]) -> Vec<Handle>`, its entity list, its insert palette and its edit resolver.
#[macro_export]
macro_rules! norm_live_editor {
    ($app:ty, $command:ident, handles = $handles:path, entities = $entities:path, palette = $palette:path, resolve = $resolve:path, scene = $scene:expr $(, diagrams = $diagrams:expr)?) => {
        impl $crate::NormLiveEditor for $app {
            fn live_invocation(command: &$command) -> Option<Result<$crate::LiveInvocation, String>> {
                Some(match command {
                    $command::SetFields(p) => $crate::LiveInvocation::set_fields(&p.writes_json, p.gesture_json.as_deref(), p.insert_json.as_deref()),
                    $command::PreviewRemedy(p) => $crate::LiveInvocation::preview_remedy(&p.check_id, p.remedy_index, p.option_index, p.insert_json.as_deref()),
                    $command::ClearPreview(_) => Ok($crate::LiveInvocation::clear_preview()),
                    $command::CancelEvaluation(_) => Ok($crate::LiveInvocation::cancel_evaluation()),
                    $command::EvaluateUnits(p) => $crate::LiveInvocation::evaluate_units(p.units_json.as_deref(), &p.reason),
                    _ => return None,
                })
            }

            fn view_event(command: &$command) -> Option<$crate::LaneEvent> {
                match command {
                    $command::ViewportAct(p) => $crate::view_event(&p.action, p.text.as_deref(), p.number),
                    $command::SetSelectedCheckIndex(p) => Some($crate::LaneEvent::ViewCheck(p.index)),
                    _ => None,
                }
            }

            fn live_handles(document: &<$app as semio_framework_plugin::ArtifactEditor>::Snapshot, selection: &[semio_s_plugin_norm_structural_scene::EntityRef]) -> Vec<semio_s_plugin_norm_structural_scene::Handle> {
                $handles(document, selection)
            }

            fn live_entities(document: &<$app as semio_framework_plugin::ArtifactEditor>::Snapshot) -> Vec<semio_s_plugin_norm_structural_scene::EntityRef> {
                $entities(document)
            }

            fn live_palette() -> Vec<$crate::direct_manipulation::InsertSpec> {
                $palette()
            }

            fn live_resolve(document: &<$app as semio_framework_plugin::ArtifactEditor>::Snapshot, edit: &$crate::NormEdit) -> Result<Vec<<$app as semio_framework_plugin::ArtifactEditor>::Mutation>, String> {
                $resolve(document, edit)
            }

            fn live_scene(document: &<$app as semio_framework_plugin::ArtifactEditor>::Snapshot, view: &semio_s_plugin_norm_structural_scene::projection::ViewOptions) -> semio_s_plugin_norm_structural_scene::StructuralScene {
                let report = <<$app as $crate::NormRetainedEditor>::Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(document);
                ($scene)(document, &*report, view)
            }
            $(
                fn live_diagrams(document: &<$app as semio_framework_plugin::ArtifactEditor>::Snapshot) -> Vec<semio_s_plugin_norm_structural_scene::Diagram2d> {
                    let report = <<$app as $crate::NormRetainedEditor>::Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(document);
                    ($diagrams)(document, &*report)
                }
            )?
        }
    };
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
