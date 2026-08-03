//! 🖥️ Process 3d app — DocumentApp impl, render, manifest (constitutional: ui). B1: the pure-trait
//! pilot — `Process3dPlayApp` is a unit struct; every former `Process3dRuntime` field (selection,
//! hover, face pick, selection method, engagement input, camera, sun) now lives in
//! `process_3d_engine::Process3dConfig`, written via `process_3d_op::Process3dConfigOperation`s; every
//! action dispatches through the single typed `process_3d_protocol::Process3dCommand` channel via
//! `DocumentApp::handle`.

use base64::Engine;
use process_3d::{MeasureKind, Pose, Process3dDocument, ProcessMeasure, ProcessStep, ProcessStepPatch, SolidSpec, Stock, StepOrigin, WorkshopMachine, WorkshopMachinePatch};
use process_3d_engine::{
    axis_angle_from_up_to, capability_for_measure_kind, catalog_machine, default_document, export_process3d_model, find_capability, import_process3d_model, installed_catalogs, measure_for_capability, next_step_id, plate_document,
    processed_mesh, processed_volume, validate_capability, validation_context_for_stock, validation_reason, Process3dConfig,
};
use process_3d_op::{Process3dConfigOperation, Process3dOperation};
use process_3d_protocol::Process3dCommand;
use protocol::CollectionOperation;
use semio_framework_core::kernel::HostEffect;
use semio_framework_plugin::{
    app_labels, build_world_3d_scene, create_default_layout, localized_label_map, mesh_from_kind, tree_item_desc, tree_item_with_action, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_readonly_field, ui_text, world3d_camera_json, world3d_mesh_id_from_url, world3d_scene, world3d_selection_json, world3d_sun_measures, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App,
    AppIo, AppLabelsOverlay, AppLabelsOverlayExt, ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, Emit, LocaleLabels, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability, OsMediaFormat, PanelGroup,
    PanelTreeBuilder, SurfaceKind, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemAction, UiTreeItemNode, UtilityCategory, UtilityDefinition, WindowEngagement, WindowEngagementControl, WindowEngagementInput,
    WindowEngagementStatus, WindowMeasure, WorldSunConfig, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, SET_ACTIVE_UTILITY_ACTION_ID,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Mutex, OnceLock};
use store::DocumentPack;

//#region 🔖️Constants
const PROCESS_3D_PLAY_APP_ID: &str = "process3d-play";
const PROCESS_3D_PLAY_CONTROLLER_ID: &str = "process3d-play";
const PROCESS_3D_PLAY_SURFACE_MAIN: &str = "process.play";
const PROCESS_3D_PLAY_BODY_MAIN: &str = "process.play.main";
const PROCESS_3D_PLAY_BODY_DOCUMENT: &str = "process.play.document";
const PROCESS_3D_PLAY_BODY_CATALOGUE: &str = "process.play.catalogue";
const PROCESS_3D_PLAY_BODY_WORKSHOP: &str = "process.play.workshop";
const PROCESS_3D_PLAY_BODY_INSPECTION: &str = "process.play.inspection";
const PROCESS_3D_PLAY_WINDOW_MAIN: &str = "process-workpiece";
const PROCESS3D_FALLBACK_MESH_KIND: &str = "box";
const PROCESS3D_EXAMPLE_TIMBER: &str = "timber-beam-joinery";
const PROCESS3D_EXAMPLE_PLATE: &str = "drilled-plate";
/// 🧰️ The utility active when the config carries no explicit override — matches
/// `process_3d_engine::Process3dConfig::default().active_utility_id`.
const PROCESS3D_DEFAULT_UTILITY: &str = "select";
//#endregion 🔖️Constants

//#region 🔖️Locale
/// 🗣️ B1: `cfg.locale`-driven counterparts to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels`.
fn is_de_locale(cfg: &Process3dConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn resolve_labels<L: LocaleLabels>(cfg: &Process3dConfig) -> &'static L {
    if is_de_locale(cfg) { L::locale_labels_de() } else { L::locale_labels_en() }
}
//#endregion 🔖️Locale

//#region 🔖️DocumentHelpers
fn process3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PROCESS_3D_PLAY_CONTROLLER_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}

/// 🧰️ Host effect that programmatically switches the workpiece window's active utility — the active
/// utility is also mirrored into `Process3dConfig::active_utility_id` (via `SetActiveUtility`) for
/// rendering, but the window chrome itself is still driven by this host effect.
fn set_active_utility_effect(utility: &str) -> HostEffect {
    HostEffect::SetActiveUtility { window_id: PROCESS_3D_PLAY_WINDOW_MAIN.into(), utility_id: utility.into() }
}

/// 📇️ A non-palette action declaration (dispatched by UI wiring/keybindings, never surfaced in the
/// command palette) with the given execution kind.
fn internal_action(id: &str, label: &str, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, kind) }
}

fn selected_ids(cfg: &Process3dConfig) -> Vec<String> {
    cfg.selected_id.clone().into_iter().collect()
}

/// 🖱️ Extends the base object-selection JSON with face-picking/drag fields: `targets.face` lets the
/// renderer hit-test individual triangles; `engagementSessionActive` gates the ground-click placement
/// path used by the cut/drill/attach utilities; `faceDragActive` gates the push/pull drag gesture, only
/// while the select utility is active (so a click-to-place utility doesn't also start a face drag).
fn process3d_selection_json(cfg: &Process3dConfig, active_utility: &str) -> String {
    let mut value: Value = serde_json::from_str(&world3d_selection_json(&cfg.selection_method, &selected_ids(cfg), cfg.hovered_id.as_deref())).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("engagementSessionActive".into(), json!(active_utility != "select"));
        object.insert("selectionMode".into(), json!("face"));
        object.insert("targets".into(), json!({ "mesh": true, "face": true, "vertex": false, "edge": false }));
        object.insert("componentIds".into(), json!(cfg.selected_face_id.map(|id| vec![id]).unwrap_or_default()));
        object.insert("faceDragActive".into(), json!(active_utility == "select"));
    }
    value.to_string()
}

fn hash_value<T: Serialize>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    if let Ok(json) = serde_json::to_string(value) {
        json.hash(&mut hasher);
    }
    hasher.finish()
}

fn fixture_signature(fixture: &Process3dDocument) -> u64 {
    hash_value(fixture)
}

/// ✂️➕️🗑️ Read-only operation builders for the two structural collection edits every mutating action needs:
/// inserting a step at the resolved-up-to cursor (and advancing it), and removing a step by id (and
/// pulling the cursor back if it sat past the removed step). Building `Process3dOperation`s from an immutable
/// `&Process3dDocument` keeps `handle` free of manual mutation — the VCS store applies them.
fn insert_step_operations(fixture: &Process3dDocument, step: ProcessStep) -> Vec<Process3dOperation> {
    let cursor = fixture.resolved_up_to.unwrap_or(fixture.steps.len()).min(fixture.steps.len());
    let id = step.id.clone();
    vec![Process3dOperation::Steps { collection: CollectionOperation::Add { id, item: step, at: cursor } }, Process3dOperation::SetCursor { resolved_up_to: Some(cursor + 1) }]
}

fn remove_step_operations(fixture: &Process3dDocument, id: &str) -> Option<Vec<Process3dOperation>> {
    let index = fixture.steps.iter().position(|step| step.id == id)?;
    let mut operations = vec![Process3dOperation::Steps { collection: CollectionOperation::Remove { id: id.to_string() } }];
    if let Some(cursor) = fixture.resolved_up_to {
        if cursor > index {
            operations.push(Process3dOperation::SetCursor { resolved_up_to: Some(cursor - 1) });
        }
    }
    Some(operations)
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️WorkshopHelpers
fn add_workshop_machine_operation(fixture: &Process3dDocument, machine: WorkshopMachine) -> Option<Process3dOperation> {
    if fixture.workshop.machines.iter().any(|existing| existing.id == machine.id) {
        return None;
    }
    let at = fixture.workshop.machines.len();
    Some(Process3dOperation::Machines { collection: CollectionOperation::Add { id: machine.id.clone(), item: machine, at } })
}

fn remove_workshop_machine_operation(fixture: &Process3dDocument, id: &str) -> Option<Process3dOperation> {
    fixture.workshop.machines.iter().any(|machine| machine.id == id).then(|| Process3dOperation::Machines { collection: CollectionOperation::Remove { id: id.to_string() } })
}
//#endregion 🔖️WorkshopHelpers

//#region 🔖️InspectorPatch
fn apply_pose_patch(pose: &mut Pose, field: &str, value: f64) -> bool {
    match field {
        "posX" => pose.position[0] = value,
        "posY" => pose.position[1] = value,
        "posZ" => pose.position[2] = value,
        "angle" => pose.angle = value,
        _ => return false,
    }
    true
}

fn apply_solid_patch(solid: &mut SolidSpec, field: &str, value: f64) -> bool {
    let clamped = value.max(0.001);
    match solid {
        SolidSpec::Box { width, depth, height } => match field {
            "width" => *width = clamped,
            "depth" => *depth = clamped,
            "height" => *height = clamped,
            _ => return false,
        },
        SolidSpec::Cylinder { radius, height } => match field {
            "radius" => *radius = clamped,
            "height" => *height = clamped,
            _ => return false,
        },
        SolidSpec::Sphere { radius } => match field {
            "radius" => *radius = clamped,
            _ => return false,
        },
        SolidSpec::ImportedMesh { .. } | SolidSpec::ImportedSolid { .. } => return false,
    }
    true
}

fn apply_stock_patch(stock: &mut Stock, field: &str, value: Option<&Value>) -> bool {
    if field == "label" {
        return match value.and_then(Value::as_str) {
            Some(label) => {
                stock.label = label.into();
                true
            }
            None => false,
        };
    }
    let Some(number) = value.and_then(Value::as_f64) else { return false };
    apply_pose_patch(&mut stock.pose, field, number) || apply_solid_patch(&mut stock.solid, field, number)
}

/// 🔎️ Generic inspector edit dispatcher for a step's measure — dimension fields are scoped to the
/// measure's own solid ("radius"/"depth" for drill, "toolWidth..." for cut, "radius"/"height" for attach)
/// so field names never collide across measure kinds.
fn apply_step_patch(step: &mut ProcessStep, field: &str, value: Option<&Value>) -> bool {
    if field == "label" {
        return match value.and_then(Value::as_str) {
            Some(label) => {
                step.label = label.into();
                true
            }
            None => false,
        };
    }
    let Some(number) = value.and_then(Value::as_f64) else { return false };
    let clamped = number.max(0.001);
    match &mut step.measure {
        ProcessMeasure::Cut { tool, pose } => {
            if apply_pose_patch(pose, field, number) {
                return true;
            }
            let SolidSpec::Box { width, depth, height } = tool else { return false };
            match field {
                "toolWidth" => *width = clamped,
                "toolDepth" => *depth = clamped,
                "toolHeight" => *height = clamped,
                _ => return false,
            }
            true
        }
        ProcessMeasure::Drill { radius, depth, pose } => {
            if apply_pose_patch(pose, field, number) {
                return true;
            }
            match field {
                "radius" => *radius = clamped,
                "depth" => *depth = clamped,
                _ => return false,
            }
            true
        }
        ProcessMeasure::Attach { component, pose } => {
            if apply_pose_patch(pose, field, number) {
                return true;
            }
            let SolidSpec::Cylinder { radius, height } = component else { return false };
            match field {
                "radius" => *radius = clamped,
                "height" => *height = clamped,
                _ => return false,
            }
            true
        }
    }
}

/// 🔎️ Generic inspector edit dispatcher for a workshop machine's own label or a capability parameter
/// value, addressed as `"{capabilityId}.{parameterId}"` so field names never collide across capabilities.
fn apply_workshop_machine_patch(machine: &mut WorkshopMachine, field: &str, value: Option<&Value>) -> bool {
    if field == "label" {
        return match value.and_then(Value::as_str) {
            Some(label) => {
                machine.label = label.into();
                true
            }
            None => false,
        };
    }
    let Some((capability_id, parameter_id)) = field.split_once('.') else { return false };
    let Some(number) = value.and_then(Value::as_f64) else { return false };
    let clamped = number.max(0.001);
    let Some(capability) = machine.capabilities.iter_mut().find(|capability| capability.id == capability_id) else { return false };
    let Some(parameter) = capability.parameters.iter_mut().find(|parameter| parameter.id == parameter_id) else { return false };
    parameter.value = clamped;
    true
}

/// 🩹️ Builds the `Process3dOperation` for one inspector field edit — clones the target (stock, step, or
/// workshop machine), mutates the clone via `apply_stock_patch`/`apply_step_patch`/
/// `apply_workshop_machine_patch`, then wraps it back into a `SetStock`/`Steps::Patch`/`Machines::Patch`
/// operation so the store computes the true pre-state inverse.
fn process3d_inspector_patch_operation(fixture: &Process3dDocument, target: &str, field: &str, value: Option<&Value>) -> Option<Process3dOperation> {
    if let Some(machine_id) = target.strip_prefix("machine:") {
        let machine = fixture.workshop.machines.iter().find(|machine| machine.id == machine_id)?;
        let mut updated = machine.clone();
        return if apply_workshop_machine_patch(&mut updated, field, value) {
            let patch = WorkshopMachinePatch { label: Some(updated.label), icon_id: None, capabilities: Some(updated.capabilities) };
            Some(Process3dOperation::Machines { collection: CollectionOperation::Patch { id: machine_id.to_string(), patch } })
        } else {
            None
        };
    }
    if target == fixture.stock.id {
        let mut stock = fixture.stock.clone();
        return if apply_stock_patch(&mut stock, field, value) { Some(Process3dOperation::SetStock { stock }) } else { None };
    }
    let step_id = target.strip_prefix("step:")?;
    let step = fixture.steps.iter().find(|step| step.id == step_id)?;
    let mut updated = step.clone();
    if !apply_step_patch(&mut updated, field, value) {
        return None;
    }
    let patch = ProcessStepPatch { label: Some(updated.label), enabled: None, measure: Some(updated.measure), origin: None };
    Some(Process3dOperation::Steps { collection: CollectionOperation::Patch { id: step_id.to_string(), patch } })
}
//#endregion 🔖️InspectorPatch

//#region 🔖️FaceDrag
/// 🖱️➡️ Builds a push/pull step from a face-drag gesture: dragging into the solid (negative `distance`
/// along the face's outward `normal`) removes material (Cut); dragging outward (positive) adds material
/// (Attach). The tool box's local origin corner lands at `point + normal * distance.min(0.0)` so it spans
/// exactly the dragged region, flush with the picked face — `box_prim_sync` places a primitive's corner
/// (not its center) at the local origin, confirmed by `box_primitive_spans_from_local_origin_corner` in
/// `process_3d_engine`.
fn process3d_step_from_face_drag(normal: [f64; 3], point: [f64; 3], distance: f64, face_extent: Option<[f64; 2]>, labels: &Process3dLabels) -> Option<ProcessStep> {
    if distance.abs() < 1e-6 {
        return None;
    }
    let (width, depth) = face_extent.map(|[w, d]| (w.max(0.02), d.max(0.02))).unwrap_or((0.2, 0.2));
    let height = distance.abs();
    let (axis, angle) = axis_angle_from_up_to(normal);
    let offset = distance.min(0.0);
    let position = [point[0] + normal[0] * offset, point[1] + normal[1] * offset, point[2] + normal[2] * offset];
    let pose = Pose { position, axis, angle };
    let (measure, label, machine_id, capability_id) = if distance < 0.0 {
        (ProcessMeasure::Cut { tool: SolidSpec::Box { width, depth, height }, pose }, labels.push_cut, "saw", "cut")
    } else {
        (ProcessMeasure::Attach { component: SolidSpec::Box { width, depth, height }, pose }, labels.pull_attach, "attacher", "attach")
    };
    let origin = StepOrigin { machine_id: machine_id.to_string(), capability_id: capability_id.to_string() };
    Some(ProcessStep { id: next_step_id(), label: label.to_string(), enabled: true, origin: Some(origin), measure })
}
//#endregion 🔖️FaceDrag

//#region 🔖️PreviewCache
/// 🖼️ A GLB-imported reference mesh (`SolidSpec::ImportedMesh`) has no kernel-side geometry to
/// tessellate; it renders by pointing the world3d scene straight at `mesh_url`, mirroring `cad`'s
/// `resolve_object_mesh_url` → `world3d_mesh_id_from_url` bridge.
fn evaluated_preview_payload(fixture: &Process3dDocument) -> (String, String) {
    if let SolidSpec::ImportedMesh { mesh_url } = &fixture.stock.solid {
        let mesh_id = world3d_mesh_id_from_url(mesh_url);
        let meshes = json!([{ "id": mesh_id, "url": mesh_url }]);
        let instances = json!([{
            "id": "processed",
            "meshId": mesh_id,
            "position": [0.0, 0.0, 0.0],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [1.0, 1.0, 1.0],
            "label": fixture.stock.label,
            "selected": false,
            "hovered": false,
        }]);
        return (meshes.to_string(), instances.to_string());
    }
    let mesh = processed_mesh(fixture).unwrap_or_else(|| mesh_from_kind(PROCESS3D_FALLBACK_MESH_KIND));
    let meshes = json!([{ "id": "processed", "data": mesh }]);
    let instances = json!([{
        "id": "processed",
        "meshId": "processed",
        "position": [0.0, 0.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0],
        "label": fixture.stock.label,
        "selected": false,
        "hovered": false,
    }]);
    (meshes.to_string(), instances.to_string())
}

/// 🧊️ In-memory memo of the last evaluated preview payload, keyed by document signature — `render`
/// only sees `&self`, so this lives in a process-wide `Mutex` (mirrors `process_3d_engine`'s kernel
/// session) rather than the app struct.
struct Process3dPreviewCache {
    signature: u64,
    meshes_json: String,
    instances_json: String,
}

static PROCESS3D_PREVIEW_CACHE: OnceLock<Mutex<Option<Process3dPreviewCache>>> = OnceLock::new();

fn process3d_preview_cache() -> &'static Mutex<Option<Process3dPreviewCache>> {
    PROCESS3D_PREVIEW_CACHE.get_or_init(|| Mutex::new(None))
}

fn preview_payload_cached(fixture: &Process3dDocument) -> (String, String) {
    let signature = fixture_signature(fixture);
    if let Ok(cache) = process3d_preview_cache().lock() {
        if let Some(entry) = cache.as_ref() {
            if entry.signature == signature {
                return (entry.meshes_json.clone(), entry.instances_json.clone());
            }
        }
    }
    let (meshes_json, instances_json) = evaluated_preview_payload(fixture);
    if let Ok(mut cache) = process3d_preview_cache().lock() {
        *cache = Some(Process3dPreviewCache { signature, meshes_json: meshes_json.clone(), instances_json: instances_json.clone() });
    }
    (meshes_json, instances_json)
}
//#endregion 🔖️PreviewCache

//#region 🔖️Terminology
/// 🗣️ Complete UI label set for the 3D app; one field per label makes every locale combination compile-checked.
app_labels! {
    struct Process3dLabels {
        stock: native_en "Stock", native_de "Rohteil", reuse_en "Stock", reuse_de "Rohteil";
        steps: native_en "Steps", native_de "Schritte", reuse_en "Steps", reuse_de "Schritte";
        select: native_en "Select", native_de "Auswählen", reuse_en "Select", reuse_de "Auswählen";
        cut: native_en "Cut", native_de "Schnitt", reuse_en "Cut", reuse_de "Schnitt";
        drill: native_en "Drill", native_de "Bohrung", reuse_en "Drill", reuse_de "Bohrung";
        attach: native_en "Attach", native_de "Anbau", reuse_en "Attach", reuse_de "Anbau";
        push_cut: native_en "Push Cut", native_de "Schnitt (Drücken)", reuse_en "Push Cut", reuse_de "Schnitt (Drücken)";
        pull_attach: native_en "Pull Attach", native_de "Anbau (Ziehen)", reuse_en "Pull Attach", reuse_de "Anbau (Ziehen)";
        enabled: native_en "Enabled", native_de "Aktiviert", reuse_en "Enabled", reuse_de "Aktiviert";
        volume: native_en "Volume", native_de "Volumen", reuse_en "Volume", reuse_de "Volumen";
        label_field: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        no_selection: native_en "No selection", native_de "Keine Auswahl", reuse_en "No selection", reuse_de "Keine Auswahl";
        remove: native_en "Remove", native_de "Entfernen", reuse_en "Remove", reuse_de "Entfernen";
        provenance: native_en "Made By", native_de "Erstellt von", reuse_en "Made By", reuse_de "Erstellt von";
        validation_warning: native_en "Warning", native_de "Warnung", reuse_en "Warning", reuse_de "Warnung";
        source: native_en "Source", native_de "Quelle", reuse_en "Source", reuse_de "Quelle";
        window_main: native_en "Workpiece", native_de "Werkstück", reuse_en "Workpiece", reuse_de "Werkstück";
        field_width: native_en "Width", native_de "Breite", reuse_en "Width", reuse_de "Breite";
        field_depth: native_en "Depth", native_de "Tiefe", reuse_en "Depth", reuse_de "Tiefe";
        field_height: native_en "Height", native_de "Höhe", reuse_en "Height", reuse_de "Höhe";
        field_radius: native_en "Radius", native_de "Radius", reuse_en "Radius", reuse_de "Radius";
        field_pos_x: native_en "X", native_de "X", reuse_en "X", reuse_de "X";
        field_pos_y: native_en "Y", native_de "Y", reuse_en "Y", reuse_de "Y";
        field_pos_z: native_en "Z", native_de "Z", reuse_en "Z", reuse_de "Z";
        field_angle: native_en "Angle", native_de "Winkel", reuse_en "Angle", reuse_de "Winkel";
        stock_kind_box: native_en "Box", native_de "Quader", reuse_en "Box", reuse_de "Quader";
        stock_kind_cylinder: native_en "Cylinder", native_de "Zylinder", reuse_en "Cylinder", reuse_de "Zylinder";
        stock_kind_sphere: native_en "Sphere", native_de "Kugel", reuse_en "Sphere", reuse_de "Kugel";
        import_model: native_en "Import Model…", native_de "Modell importieren…", reuse_en "Import Model…", reuse_de "Modell importieren…";
        step_control: native_en "Step", native_de "Schritt", reuse_en "Step", reuse_de "Schritt";
        workshop: native_en "Workshop", native_de "Werkstatt", reuse_en "Workshop", reuse_de "Werkstatt";
        machines: native_en "Machines", native_de "Maschinen", reuse_en "Machines", reuse_de "Maschinen";
        remove_machine: native_en "Remove Machine", native_de "Maschine entfernen", reuse_en "Remove Machine", reuse_de "Maschine entfernen";
        installed: native_en "Installed", native_de "Installiert", reuse_en "Installed", reuse_de "Installiert";
    }
}

fn process3d_measure_icon(measure: &ProcessMeasure) -> &'static str {
    match measure {
        ProcessMeasure::Cut { .. } => "scissors",
        ProcessMeasure::Drill { .. } => "circle-dot",
        ProcessMeasure::Attach { .. } => "plus",
    }
}

fn process3d_measure_label<'a>(measure: &ProcessMeasure, labels: &'a Process3dLabels) -> &'a str {
    match measure {
        ProcessMeasure::Cut { .. } => labels.cut,
        ProcessMeasure::Drill { .. } => labels.drill,
        ProcessMeasure::Attach { .. } => labels.attach,
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id, en, de) for every operation/view-action/shell-action declared in
/// `create_process3d_app`'s static manifest — the manifest itself has no `view_state`/locale
/// parameter, so `localized_label_map` over these entries is how the command palette and Actions
/// rail get a translated label without threading locale through the whole builder chain.
const PROCESS3D_ACTION_LABEL_ENTRIES: &[(&str, &str, &str)] = &[
    ("addStep", "Add Step", "Schritt hinzufügen"),
    ("addWorkshopMachine", "Add Machine", "Maschine hinzufügen"),
    ("removeWorkshopMachine", "Remove Machine", "Maschine entfernen"),
    ("updateWorkshopMachine", "Update Machine", "Maschine aktualisieren"),
    ("setStock", "Set Stock", "Rohteil festlegen"),
    ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
    ("removeSelectedStep", "Remove Selected Step", "Ausgewählten Schritt entfernen"),
    ("exportModel", "Export Model", "Modell exportieren"),
    ("loadModelRequest", "Load Model…", "Modell laden…"),
    ("setDocument", "Set Document", "Dokument festlegen"),
    ("importModelFile", "Import Model File", "Modelldatei importieren"),
    ("removeStep", "Remove Step", "Schritt entfernen"),
    ("moveStep", "Move Step", "Schritt verschieben"),
    ("updateStep", "Update Step", "Schritt aktualisieren"),
    ("setStepEnabled", "Set Step Enabled", "Schrittaktivierung festlegen"),
    ("patchInspector", "Patch Inspector", "Inspektor aktualisieren"),
    ("worldPointerDown", "World Pointer Down", "Welt-Zeiger gedrückt"),
    ("worldFaceDragEnd", "World Face Drag End", "Welt-Flächenzug beendet"),
    ("setCursor", "Set Cursor", "Cursor festlegen"),
    ("stepCursor", "Step Cursor", "Cursor schrittweise bewegen"),
    ("stepCursorBack", "Step Cursor Back", "Cursor zurück"),
    ("stepCursorForward", "Step Cursor Forward", "Cursor vorwärts"),
    ("engagementSubmit", "Engagement Submit", "Eingabe bestätigen"),
    ("engagementInput", "Engagement Input", "Eingabe"),
    ("engagementAbort", "Engagement Abort", "Eingabe abbrechen"),
    ("setSelection", "Set Selection", "Auswahl festlegen"),
    ("setHover", "Set Hover", "Überfahren festlegen"),
    ("setCamera", "Set Camera", "Kamera festlegen"),
    ("worldPick", "World Pick", "Welt-Auswahl (Pick)"),
    ("toggleSun", "Toggle Sun", "Sonne umschalten"),
    ("setSunAzimuth", "Set Sun Azimuth", "Sonnenazimut festlegen"),
    ("setSunElevation", "Set Sun Elevation", "Sonnenhöhe festlegen"),
    ("setSunIntensity", "Set Sun Intensity", "Sonnenintensität festlegen"),
];

/// 🗣️ (utility id, en, de) for every `.utility(...)` declared in `create_process3d_app`.
const PROCESS3D_UTILITY_LABEL_ENTRIES: &[(&str, &str, &str)] = &[("select", "Select", "Auswählen"), ("cut", "Cut", "Schneiden"), ("drill", "Drill", "Bohren"), ("attach", "Attach", "Anbauen")];
//#endregion 🔖️CommandLabels

//#region 🔖️Panels
/// 🎨️ `tree_item_with_action` (SDK) carries no icon slot, so this app-specific wrapper layers
/// `icon_id` on top via struct-update syntax — the only piece of the item skeleton this app adds.
fn iconed_tree_item_with_action(id: impl Into<String>, label: impl Into<String>, icon_id: &str, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode { icon_id: Some(icon_id.into()), menu: None,
    ..tree_item_with_action(id, label, None, action) }
}

fn number_field(id: impl Into<String>, label: impl Into<String>, value: f64, target: &str, field: &str) -> UiNode {
    let id = id.into();
    UiNode::Field(UiFieldNode {
        id: id.clone(),
        label: label.into(),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        child: Box::new(UiNode::Input(UiInputNode { presence: UiPresence::default(),
            id: format!("{id}.input"),
            input_kind: "number".into(),
            value: value.to_string(),
            placeholder: None,
            commit: None,
            on_change: process3d_action("patchInspector", Some(json!({ "target": target, "field": field }))),
            min: None,
            max: None,
            step: None,
            accept: None,
            menu: None,
        })),
        menu: None,
    })
}

fn text_field(id: impl Into<String>, label: impl Into<String>, value: &str, target: &str, field: &str) -> UiNode {
    let id = id.into();
    UiNode::Field(UiFieldNode {
        id: id.clone(),
        label: label.into(),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        child: Box::new(UiNode::Input(UiInputNode { presence: UiPresence::default(),
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value: value.into(),
            placeholder: None,
            commit: None,
            on_change: process3d_action("patchInspector", Some(json!({ "target": target, "field": field }))),
            min: None,
            max: None,
            step: None,
            accept: None,
            menu: None,
        })),
        menu: None,
    })
}

fn build_document_tree(fixture: &Process3dDocument, cfg: &Process3dConfig, labels: &Process3dLabels) -> UiNode {
    let stock = &fixture.stock;
    let stock_item = UiTreeItemNode {
        icon_id: Some("box".into()),
        presence: UiPresence::selected(cfg.selected_id.as_deref() == Some(stock.id.as_str())),
        action: Some(process3d_action("setSelection", Some(json!({ "id": stock.id })))),
        menu: None,
        ..UiTreeItemNode::base(stock.id.clone(), stock.label.clone())
    };
    let cursor = fixture.resolved_up_to.unwrap_or(fixture.steps.len());
    let step_items: Vec<UiTreeItemNode> = fixture
        .steps
        .iter()
        .enumerate()
        .map(|(index, step)| UiTreeItemNode {
            description: if index >= cursor { Some("pending".into()) } else { None },
            icon_id: Some(process3d_measure_icon(&step.measure).into()),
            presence: UiPresence::selected(cfg.selected_id.as_deref() == Some(step.id.as_str())),
            action: Some(process3d_action("setSelection", Some(json!({ "id": step.id })))),
            hover_action: Some(process3d_action("setHover", Some(json!({ "id": step.id })))),
            unhover_action: Some(process3d_action("setHover", None)),
            actions: Some(vec![
                UiTreeItemAction {
                    icon_id: if step.enabled { "eye".into() } else { "eye-off".into() },
                    label: Some(labels.enabled.into()),
                    action: process3d_action("setStepEnabled", Some(json!({ "id": step.id, "enabled": !step.enabled }))),
                    reveal_on_hover: Some(true),
                },
                UiTreeItemAction { icon_id: "trash".into(), label: Some(labels.remove.into()), action: process3d_action("removeStep", Some(json!({ "id": step.id }))), reveal_on_hover: Some(true),
        },
            ]),
            dimmed: Some(!step.enabled),
            menu: None,
            ..UiTreeItemNode::base(step.id.clone(), step.label.clone())
        })
        .collect();
    PanelTreeBuilder::new("process3d-play-document").section("process3d-play-document.stock", Some(labels.stock.into()), true, vec![stock_item]).section("process3d-play-document.steps", Some(labels.steps.into()), true, step_items).build()
}

/// 🏷️ Display label for a catalog id, resolved against `installed_catalogs()` — falls back to the raw
/// id if the catalog that seeded a workshop machine was since uninstalled (never resolved back, per
/// `WorkshopMachine::catalog_id`'s informational-only contract).
fn catalog_label(catalog_id: &str) -> String {
    installed_catalogs().into_iter().find(|catalog| catalog.catalog_id() == catalog_id).map(|catalog| catalog.label().to_string()).unwrap_or_else(|| catalog_id.to_string())
}

/// 🏭️ Builds one catalogue tree item per workshop machine capability, grouped by the machine's source
/// catalog (uncataloged/generic machines first, open by default), disabling (non-clickable, with a
/// reason) any capability the current stock doesn't satisfy.
fn build_catalogue_tree(fixture: &Process3dDocument, labels: &Process3dLabels) -> UiNode {
    let ctx = validation_context_for_stock(&fixture.stock);
    let mut builder = PanelTreeBuilder::new("process3d-play-catalogue");
    let mut sections: Vec<(Option<&str>, Vec<&WorkshopMachine>)> = Vec::new();
    for machine in &fixture.workshop.machines {
        let key = machine.catalog_id.as_deref();
        match sections.iter_mut().find(|(existing, _)| *existing == key) {
            Some(section) => section.1.push(machine),
            None => sections.push((key, vec![machine])),
        }
    }
    sections.sort_by_key(|(key, _)| key.is_some());
    for (catalog_id, machines) in sections {
        let items: Vec<UiTreeItemNode> = machines
            .iter()
            .flat_map(|machine| {
                machine.capabilities.iter().map(move |capability| {
                    let failures = validate_capability(capability, &ctx);
                    let id = format!("process3d-catalogue.{}.{}", machine.id, capability.id);
                    let label = format!("{} — {}", machine.label, capability.label);
                    if failures.is_empty() {
                        iconed_tree_item_with_action(id, label, &capability.icon_id, process3d_action("addStep", Some(json!({ "machineId": machine.id, "capabilityId": capability.id }))))
                    } else {
                        UiTreeItemNode { icon_id: Some(capability.icon_id.as_str().into()), menu: None,
        ..tree_item_desc(id, label, Some(validation_reason(&failures)))
    }
                    }
                })
            })
            .collect();
        let section_id = format!("process3d-play-catalogue.{}", catalog_id.unwrap_or("workshop"));
        let section_label = catalog_id.map(catalog_label).unwrap_or_else(|| labels.workshop.into());
        builder = builder.section(section_id, Some(section_label), catalog_id.is_none(), items);
    }
    let stock_items = vec![
        iconed_tree_item_with_action("process3d-catalogue.stock-box", labels.stock_kind_box, "box", process3d_action("setStock", Some(json!({ "kind": "box" })))),
        iconed_tree_item_with_action("process3d-catalogue.stock-cylinder", labels.stock_kind_cylinder, "cylinder", process3d_action("setStock", Some(json!({ "kind": "cylinder" })))),
        iconed_tree_item_with_action("process3d-catalogue.stock-sphere", labels.stock_kind_sphere, "circle", process3d_action("setStock", Some(json!({ "kind": "sphere" })))),
        iconed_tree_item_with_action("process3d-catalogue.stock-import", labels.import_model, "folder-open", process3d_action("loadModelRequest", None)),
    ];
    builder.section("process3d-play-catalogue.stock", Some(labels.stock.into()), false, stock_items).build()
}

/// 🛠️ The workshop configuration panel: "Machines" (the document's own workshop machines — select,
/// remove) plus one section per installed catalog (its machines — add if not yet installed, dimmed
/// "installed" description otherwise).
fn build_workshop_tree(fixture: &Process3dDocument, cfg: &Process3dConfig, labels: &Process3dLabels) -> UiNode {
    let mut builder = PanelTreeBuilder::new("process3d-play-workshop");
    let machine_items: Vec<UiTreeItemNode> = fixture
        .workshop
        .machines
        .iter()
        .map(|machine| {
            let target = format!("machine:{}", machine.id);
            UiTreeItemNode {
                icon_id: Some(machine.icon_id.as_str().into()),
                presence: UiPresence::selected(cfg.selected_id.as_deref() == Some(target.as_str())),
                action: Some(process3d_action("setSelection", Some(json!({ "id": target })))),
                actions: Some(vec![UiTreeItemAction { icon_id: "trash".into(), label: Some(labels.remove_machine.into()), action: process3d_action("removeWorkshopMachine", Some(json!({ "id": machine.id }))), reveal_on_hover: Some(true) }]),
                menu: None,
                ..UiTreeItemNode::base(format!("process3d-workshop.machine.{}", machine.id), machine.label.clone())
            }
        })
        .collect();
    builder = builder.section("process3d-play-workshop.machines", Some(labels.machines.into()), true, machine_items);
    for catalog in installed_catalogs() {
        let catalog_id = catalog.catalog_id();
        let items: Vec<UiTreeItemNode> = catalog
            .machines()
            .into_iter()
            .map(|machine| {
                let id = format!("process3d-workshop.catalog.{catalog_id}.{}", machine.id);
                let already_installed = fixture.workshop.machines.iter().any(|existing| existing.id == machine.id);
                if already_installed {
                    UiTreeItemNode { icon_id: Some(machine.icon_id.as_str().into()), dimmed: Some(true), menu: None, ..tree_item_desc(id, machine.label.clone(), Some(labels.installed.to_string())) }
                } else {
                    iconed_tree_item_with_action(id, machine.label.clone(), &machine.icon_id, process3d_action("addWorkshopMachine", Some(json!({ "catalogId": catalog_id, "machineId": machine.id }))))
                }
            })
            .collect();
        builder = builder.section(format!("process3d-play-workshop.catalog.{catalog_id}"), Some(catalog.label().into()), false, items);
    }
    builder.build()
}

fn build_stock_inspector(stock: &Stock, fixture: &Process3dDocument, labels: &Process3dLabels) -> UiNode {
    let mut fields = vec![text_field("process3d-inspector.label", labels.label_field, &stock.label, &stock.id, "label")];
    match &stock.solid {
        SolidSpec::Box { width, depth, height } => {
            fields.push(number_field("process3d-inspector.width", labels.field_width, *width, &stock.id, "width"));
            fields.push(number_field("process3d-inspector.depth", labels.field_depth, *depth, &stock.id, "depth"));
            fields.push(number_field("process3d-inspector.height", labels.field_height, *height, &stock.id, "height"));
        }
        SolidSpec::Cylinder { radius, height } => {
            fields.push(number_field("process3d-inspector.radius", labels.field_radius, *radius, &stock.id, "radius"));
            fields.push(number_field("process3d-inspector.height", labels.field_height, *height, &stock.id, "height"));
        }
        SolidSpec::Sphere { radius } => {
            fields.push(number_field("process3d-inspector.radius", labels.field_radius, *radius, &stock.id, "radius"));
        }
        SolidSpec::ImportedMesh { mesh_url } => {
            fields.push(ui_inspector_readonly_field("process3d-inspector.source", labels.source, mesh_url.clone()));
        }
        SolidSpec::ImportedSolid { solid_handle } => {
            fields.push(ui_inspector_readonly_field("process3d-inspector.source", labels.source, format!("solid #{solid_handle}")));
        }
    }
    fields.push(number_field("process3d-inspector.posX", labels.field_pos_x, stock.pose.position[0], &stock.id, "posX"));
    fields.push(number_field("process3d-inspector.posY", labels.field_pos_y, stock.pose.position[1], &stock.id, "posY"));
    fields.push(number_field("process3d-inspector.posZ", labels.field_pos_z, stock.pose.position[2], &stock.id, "posZ"));
    fields.push(number_field("process3d-inspector.angle", labels.field_angle, stock.pose.angle, &stock.id, "angle"));
    if let Some(volume) = processed_volume(fixture) {
        fields.push(ui_inspector_readonly_field("process3d-inspector.volume", labels.volume, format!("{volume:.4} m³")));
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(), id: "process3d-inspector.stock".into(), label: labels.stock.into(), default_open: Some(true), fields }])
}

/// 🏷️ Human-readable summary of one capability rule, for the machine inspector's readonly rule list.
fn describe_capability_rule(rule: &process_3d::CapabilityRule) -> String {
    let (quantity, parameter, margin, is_min) = match rule {
        process_3d::CapabilityRule::Min { quantity, parameter, margin } => (*quantity, parameter, *margin, true),
        process_3d::CapabilityRule::Max { quantity, parameter, margin } => (*quantity, parameter, *margin, false),
    };
    let axis = match quantity {
        process_3d::StockQuantity::Width => "width",
        process_3d::StockQuantity::Depth => "depth",
        process_3d::StockQuantity::Height => "height",
        process_3d::StockQuantity::MaxDimension => "max dimension",
        process_3d::StockQuantity::MinDimension => "min dimension",
    };
    let comparator = if is_min { "≥" } else { "≤" };
    if margin != 0.0 {
        format!("{axis} {comparator} {parameter} ± {margin}m")
    } else {
        format!("{axis} {comparator} {parameter}")
    }
}

/// 🛠️ Inspector for a selected workshop machine: its label, plus one field group per capability with a
/// number field for every parameter and a readonly summary of its rules.
fn build_machine_inspector(machine: &WorkshopMachine, labels: &Process3dLabels) -> UiNode {
    let target = format!("machine:{}", machine.id);
    let machine_fields = vec![text_field("process3d-inspector.label", labels.label_field, &machine.label, &target, "label")];
    let mut groups = vec![UiInspectorFieldGroup { presence: UiPresence::default(), id: "process3d-inspector.machine".into(), label: labels.workshop.into(), default_open: Some(true), fields: machine_fields }];
    for capability in &machine.capabilities {
        let mut fields: Vec<UiNode> = capability
            .parameters
            .iter()
            .map(|parameter| {
                let field = format!("{}.{}", capability.id, parameter.id);
                number_field(format!("process3d-inspector.{field}"), parameter.label.clone(), parameter.value, &target, &field)
            })
            .collect();
        if !capability.rules.is_empty() {
            let summary = capability.rules.iter().map(describe_capability_rule).collect::<Vec<_>>().join("; ");
            fields.push(ui_inspector_readonly_field(format!("process3d-inspector.{}.rules", capability.id), labels.validation_warning, summary));
        }
        groups.push(UiInspectorFieldGroup { presence: UiPresence::default(), id: format!("process3d-inspector.{}", capability.id), label: capability.label.clone(), default_open: Some(true), fields });
    }
    ui_inspector_groups_to_tree(&groups)
}

fn build_step_inspector(step: &ProcessStep, fixture: &Process3dDocument, labels: &Process3dLabels) -> UiNode {
    let target = format!("step:{}", step.id);
    let mut fields = vec![text_field("process3d-inspector.label", labels.label_field, &step.label, &target, "label")];
    if let Some(origin) = &step.origin {
        match find_capability(&fixture.workshop, &origin.machine_id, &origin.capability_id) {
            Some((machine, capability)) => {
                fields.push(ui_inspector_readonly_field("process3d-inspector.origin", labels.provenance, format!("{} · {}", machine.label, capability.label)));
                let failures = validate_capability(capability, &validation_context_for_stock(&fixture.stock));
                if !failures.is_empty() {
                    fields.push(ui_inspector_readonly_field("process3d-inspector.validation", labels.validation_warning, validation_reason(&failures)));
                }
            }
            // 🏭️ Unresolvable provenance (the machine/capability was since removed from the workshop) —
            // StepOrigin is purely informational, so this shows the raw ids and never blocks the step.
            None => {
                fields.push(ui_inspector_readonly_field("process3d-inspector.origin", labels.provenance, format!("{} · {}", origin.machine_id, origin.capability_id)));
            }
        }
    }
    let pose = match &step.measure {
        ProcessMeasure::Cut { tool, pose } => {
            if let SolidSpec::Box { width, depth, height } = tool {
                fields.push(number_field("process3d-inspector.toolWidth", labels.field_width, *width, &target, "toolWidth"));
                fields.push(number_field("process3d-inspector.toolDepth", labels.field_depth, *depth, &target, "toolDepth"));
                fields.push(number_field("process3d-inspector.toolHeight", labels.field_height, *height, &target, "toolHeight"));
            }
            pose
        }
        ProcessMeasure::Drill { radius, depth, pose } => {
            fields.push(number_field("process3d-inspector.radius", labels.field_radius, *radius, &target, "radius"));
            fields.push(number_field("process3d-inspector.depth", labels.field_depth, *depth, &target, "depth"));
            pose
        }
        ProcessMeasure::Attach { component, pose } => {
            if let SolidSpec::Cylinder { radius, height } = component {
                fields.push(number_field("process3d-inspector.radius", labels.field_radius, *radius, &target, "radius"));
                fields.push(number_field("process3d-inspector.height", labels.field_height, *height, &target, "height"));
            }
            pose
        }
    };
    fields.push(number_field("process3d-inspector.posX", labels.field_pos_x, pose.position[0], &target, "posX"));
    fields.push(number_field("process3d-inspector.posY", labels.field_pos_y, pose.position[1], &target, "posY"));
    fields.push(number_field("process3d-inspector.posZ", labels.field_pos_z, pose.position[2], &target, "posZ"));
    fields.push(number_field("process3d-inspector.angle", labels.field_angle, pose.angle, &target, "angle"));
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(), id: "process3d-inspector.step".into(), label: process3d_measure_label(&step.measure, labels).into(), default_open: Some(true), fields }])
}

fn build_inspector_tree(fixture: &Process3dDocument, cfg: &Process3dConfig, labels: &Process3dLabels) -> UiNode {
    let Some(selected_id) = cfg.selected_id.as_deref() else {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "process3d-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text(labels.no_selection)],
            presence: UiPresence::default(),
            menu: None,
        }]);
    };
    if selected_id == fixture.stock.id {
        return build_stock_inspector(&fixture.stock, fixture, labels);
    }
    if let Some(step) = fixture.steps.iter().find(|step| step.id == selected_id) {
        return build_step_inspector(step, fixture, labels);
    }
    if let Some(machine_id) = selected_id.strip_prefix("machine:") {
        if let Some(machine) = fixture.workshop.machines.iter().find(|machine| machine.id == machine_id) {
            return build_machine_inspector(machine, labels);
        }
    }
    ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
        id: "process3d-play-inspector.missing".into(),
        label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
        default_open: Some(true),
        children: vec![ui_text(labels.no_selection)],
        presence: UiPresence::default(),
        menu: None,
    }])
}
//#endregion 🔖️Panels

//#region 🔖️Engagement
fn process3d_engagement(fixture: &Process3dDocument, cfg: &Process3dConfig, active_utility: &str, labels: &Process3dLabels) -> WindowEngagement {
    let len = fixture.steps.len();
    let cursor = fixture.resolved_up_to.unwrap_or(len);
    let volume = processed_volume(fixture).unwrap_or(0.0);
    WindowEngagement {
        session_active: Some(active_utility != "select"),
        // 🧰️ The select/cut/drill/attach switcher now lives in the framework utility bar (declared via `.utility` +
        // `.window_kind_utilities`), so the engagement no longer duplicates it as toggle options.
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("process3d-engagement".into()),
            value: Some(cfg.engagement_input.clone()),
            placeholder: Some("cut, drill, attach, back, forward, all".into()),
            disabled: None,
            on_change: Some(process3d_action("engagementInput", None)),
            on_submit: Some(process3d_action("engagementSubmit", None)),
            on_repeat_last: None,
            on_abort: Some(process3d_action("engagementAbort", None)),
        }),
        control: Some(WindowEngagementControl::Stepper {
            id: Some("process3d-cursor".into()),
            label: Some(labels.step_control.into()),
            value: cursor as f64,
            min: Some(0.0),
            max: Some(len as f64),
            step: Some(1.0),
            unit: None,
            disabled: None,
            on_change: Some(process3d_action("setCursor", None)),
            on_commit: None,
        }),
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "process3d-status".into(), text: format!("{cursor}/{len} steps · {volume:.4} m³") }]),
        possible_engagements: None,
    }
}
//#endregion 🔖️Engagement

//#region 🔖️Process3dPlayApp
/// 🧪️ B1: unit struct — every former `Process3dRuntime` field now lives in
/// `process_3d_engine::Process3dConfig` (see `DocumentApp::Config`), written through
/// `process_3d_op::Process3dConfigOperation`s.
#[derive(Default)]
pub struct Process3dPlayApp;

impl DocumentApp for Process3dPlayApp {
    type Projection = Process3dDocument;
    type Operation = Process3dOperation;
    type Config = Process3dConfig;
    type ConfigOperation = Process3dConfigOperation;
    type Command = Process3dCommand;

    fn app_id(&self) -> &str {
        PROCESS_3D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        process_3d::PROCESS_3D_SCHEMA
    }

    fn initial_projection(&self) -> Process3dDocument {
        default_document()
    }

    fn io(&self) -> Option<AppIo> {
        Some(process_3d_engine::process3d_io())
    }

    //#region 🔖️Media
    /// 🎞️ `brep:out` (see `process_3d_engine::export_process3d_model`, STEP text) plus the inherited
    /// `document:out` default (the pack of `doc.projection`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new one).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, Process3dDocument>) -> Result<Media, MediaError> {
        match port {
            "brep:out" => match export_process3d_model(doc.projection, "step") {
                Some(export) => {
                    let text = match export.data {
                        Value::String(text) => text,
                        other => serde_json::to_string(&other).unwrap_or_default(),
                    };
                    Ok(Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, payload: MediaPayload::Structured { schema: "3d.process".into(), json: text } })
                }
                None => Err(MediaError::Payload("brep:out".into(), "kernel replay failed".into())),
            },
            "document:out" => {
                let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
                let bytes = doc.projection.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn whole_document_operation(&self, projection: Process3dDocument) -> Option<Process3dOperation> {
        Some(Process3dOperation::SetDocument { document: projection })
    }

    /// 📥️ `geometry:in` (best-effort STEP-text import, see the ticket notes on the cross-app wire
    /// contract) plus the inherited `document:in` default (base64 pack via `whole_document_operation`,
    /// replicated inline — overriding `import_media` shadows the trait's provided body for every port).
    fn import_media(&self, port: &str, media: &Media, _doc: &DocumentView<'_, Process3dDocument>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, MediaError> {
        match port {
            "geometry:in" => {
                let MediaPayload::Structured { schema, json } = &media.payload else {
                    return Err(MediaError::Payload("geometry:in".into(), "expected a structured payload".into()));
                };
                if schema != process_3d::PROCESS_3D_SCHEMA && schema != "3d.process" {
                    return Err(MediaError::Payload("geometry:in".into(), format!("unrecognized schema: {schema}")));
                }
                // 📦️ `export_process3d_model("step")` hands back raw (non-base64) STEP text — see
                // `OsMediaFormat::Step::is_binary() == false` — so this re-encodes it as base64 to
                // satisfy `process3d_bytes_from_data_url`'s `data:...,<base64>` expectation.
                let data_url = format!("data:application/octet-stream;base64,{}", base64::engine::general_purpose::STANDARD.encode(json.as_bytes()));
                match import_process3d_model("geometry-in.step", &data_url) {
                    Some(document) => Ok(Emit::operations(vec![Process3dOperation::SetDocument { document }])),
                    None => Err(MediaError::Payload("geometry:in".into(), "STEP import failed".into())),
                }
            }
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let projection = <Process3dDocument as DocumentPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                match self.whole_document_operation(projection) {
                    Some(operation) => Ok(Emit::operations(vec![operation])),
                    None => Err(MediaError::NotImplemented),
                }
            }
            _ => Err(MediaError::NotImplemented),
        }
    }
    //#endregion 🔖️Media

    /// 🏷️ Maps each `Process3dCommand` variant back to the action id it was declared under in
    /// `create_process3d_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &Process3dCommand) -> &str {
        match command {
            Process3dCommand::SetDocument { .. } => "setDocument",
            Process3dCommand::SetActiveExample { .. } => "setActiveExample",
            Process3dCommand::AddStep { .. } => "addStep",
            Process3dCommand::AddWorkshopMachine { .. } => "addWorkshopMachine",
            Process3dCommand::RemoveWorkshopMachine { .. } => "removeWorkshopMachine",
            Process3dCommand::UpdateWorkshopMachine { .. } => "updateWorkshopMachine",
            Process3dCommand::RemoveStep { .. } => "removeStep",
            Process3dCommand::RemoveSelectedStep => "removeSelectedStep",
            Process3dCommand::MoveStep { .. } => "moveStep",
            Process3dCommand::UpdateStep { .. } => "updateStep",
            Process3dCommand::SetStepEnabled { .. } => "setStepEnabled",
            Process3dCommand::SetStock { .. } => "setStock",
            Process3dCommand::PatchInspector { .. } => "patchInspector",
            Process3dCommand::SetCursor { .. } => "setCursor",
            Process3dCommand::StepCursor { .. } => "stepCursor",
            Process3dCommand::StepCursorBack => "stepCursorBack",
            Process3dCommand::StepCursorForward => "stepCursorForward",
            Process3dCommand::EngagementSubmit => "engagementSubmit",
            Process3dCommand::WorldPointerDown { .. } => "worldPointerDown",
            Process3dCommand::WorldFaceDragEnd { .. } => "worldFaceDragEnd",
            Process3dCommand::ImportModelFile { .. } => "importModelFile",
            Process3dCommand::SetActiveUtility { .. } => SET_ACTIVE_UTILITY_ACTION_ID,
            Process3dCommand::EngagementInput { .. } => "engagementInput",
            Process3dCommand::EngagementAbort => "engagementAbort",
            Process3dCommand::SetSelection { .. } => "setSelection",
            Process3dCommand::SetHover { .. } => "setHover",
            Process3dCommand::SetCamera { .. } => "setCamera",
            Process3dCommand::WorldPick { .. } => "worldPick",
            Process3dCommand::ToggleSun => "toggleSun",
            Process3dCommand::SetSunAzimuth { .. } => "setSunAzimuth",
            Process3dCommand::SetSunElevation { .. } => "setSunElevation",
            Process3dCommand::SetSunIntensity { .. } => "setSunIntensity",
            Process3dCommand::SetLocale { .. } => "setLocale",
            Process3dCommand::ExportModel { .. } => "exportModel",
            Process3dCommand::LoadModelRequest => "loadModelRequest",
        }
    }

    //#region 🔖️Handle
    fn handle(&self, command: &Process3dCommand, doc: &DocumentView<'_, Process3dDocument>, cfg: &ConfigView<'_, Process3dConfig>) -> Emit<Process3dOperation, Process3dConfigOperation> {
        let fixture = doc.projection;
        let config = cfg.projection;
        match command {
            Process3dCommand::SetDocument { document } => Emit {
                document_operations: vec![Process3dOperation::SetDocument { document: document.clone() }],
                config_operations: vec![Process3dConfigOperation::SetSelectedId { value: None }],
                ..Default::default()
            },
            Process3dCommand::SetActiveExample { example_id } => {
                let document = match example_id.as_str() {
                    PROCESS3D_EXAMPLE_PLATE | "plate" => plate_document(),
                    "" => Process3dDocument::default(),
                    _ => default_document(),
                };
                Emit {
                    document_operations: vec![Process3dOperation::SetDocument { document }],
                    config_operations: vec![Process3dConfigOperation::SetSelectedId { value: None }],
                    ..Default::default()
                }
            }
            Process3dCommand::AddStep { measure, machine_id, capability_id, position } => {
                let resolved = if let (Some(machine_id), Some(capability_id)) = (machine_id.as_deref(), capability_id.as_deref()) {
                    find_capability(&fixture.workshop, machine_id, capability_id).map(|(machine, capability)| (machine.clone(), capability.clone()))
                } else {
                    let measure_kind = match measure.as_deref().unwrap_or("cut") {
                        "drill" => MeasureKind::Drill,
                        "attach" => MeasureKind::Attach,
                        _ => MeasureKind::Cut,
                    };
                    Some(capability_for_measure_kind(&fixture.workshop, measure_kind))
                };
                let Some((machine, capability)) = resolved else {
                    return Emit::default();
                };
                let failures = validate_capability(&capability, &validation_context_for_stock(&fixture.stock));
                if !failures.is_empty() {
                    return Emit::default();
                }
                let origin = StepOrigin { machine_id: machine.id.clone(), capability_id: capability.id.clone() };
                let step = ProcessStep { id: next_step_id(), label: capability.label.clone(), enabled: true, origin: Some(origin), measure: measure_for_capability(&capability, *position) };
                let step_id = step.id.clone();
                Emit { document_operations: insert_step_operations(fixture, step), config_operations: vec![Process3dConfigOperation::SetSelectedId { value: Some(step_id) }], ..Default::default() }
            }
            Process3dCommand::AddWorkshopMachine { catalog_id, machine_id } => match catalog_machine(catalog_id, machine_id) {
                Some(machine) => match add_workshop_machine_operation(fixture, machine) {
                    Some(operation) => {
                        let selected = format!("machine:{machine_id}");
                        Emit { document_operations: vec![operation], config_operations: vec![Process3dConfigOperation::SetSelectedId { value: Some(selected) }], ..Default::default() }
                    }
                    None => Emit::default(),
                },
                None => Emit::default(),
            },
            Process3dCommand::RemoveWorkshopMachine { id } => match remove_workshop_machine_operation(fixture, id) {
                Some(operation) => {
                    let mut config_operations = Vec::new();
                    if config.selected_id.as_deref() == Some(format!("machine:{id}").as_str()) {
                        config_operations.push(Process3dConfigOperation::SetSelectedId { value: None });
                    }
                    Emit { document_operations: vec![operation], config_operations, ..Default::default() }
                }
                None => Emit::default(),
            },
            Process3dCommand::UpdateWorkshopMachine { machine } => {
                if fixture.workshop.machines.iter().any(|existing| existing.id == machine.id) {
                    let patch = WorkshopMachinePatch { label: Some(machine.label.clone()), icon_id: Some(machine.icon_id.clone()), capabilities: Some(machine.capabilities.clone()) };
                    Emit::operations(vec![Process3dOperation::Machines { collection: CollectionOperation::Patch { id: machine.id.clone(), patch } }])
                } else {
                    Emit::default()
                }
            }
            Process3dCommand::RemoveStep { id } => match remove_step_operations(fixture, id) {
                Some(operations) => {
                    let mut config_operations = Vec::new();
                    if config.selected_id.as_deref() == Some(id.as_str()) {
                        config_operations.push(Process3dConfigOperation::SetSelectedId { value: None });
                    }
                    Emit { document_operations: operations, config_operations, ..Default::default() }
                }
                None => Emit::default(),
            },
            Process3dCommand::RemoveSelectedStep => match config.selected_id.clone() {
                Some(id) => match remove_step_operations(fixture, &id) {
                    Some(operations) => Emit { document_operations: operations, config_operations: vec![Process3dConfigOperation::SetSelectedId { value: None }], ..Default::default() },
                    None => Emit::default(),
                },
                None => Emit::default(),
            },
            Process3dCommand::MoveStep { id, index } => {
                if fixture.steps.iter().any(|step| &step.id == id) {
                    Emit::operations(vec![Process3dOperation::Steps { collection: CollectionOperation::Move { id: id.clone(), to: *index } }])
                } else {
                    Emit::default()
                }
            }
            Process3dCommand::UpdateStep { step } => {
                if fixture.steps.iter().any(|existing| existing.id == step.id) {
                    let patch = ProcessStepPatch { label: Some(step.label.clone()), enabled: Some(step.enabled), measure: Some(step.measure.clone()), origin: Some(step.origin.clone()) };
                    Emit::operations(vec![Process3dOperation::Steps { collection: CollectionOperation::Patch { id: step.id.clone(), patch } }])
                } else {
                    Emit::default()
                }
            }
            Process3dCommand::SetStepEnabled { id, enabled } => {
                if fixture.steps.iter().any(|step| &step.id == id) {
                    let patch = ProcessStepPatch { enabled: Some(*enabled), ..Default::default() };
                    Emit::operations(vec![Process3dOperation::Steps { collection: CollectionOperation::Patch { id: id.clone(), patch } }])
                } else {
                    Emit::default()
                }
            }
            Process3dCommand::SetStock { kind } => {
                let solid = match kind.as_str() {
                    "cylinder" => SolidSpec::Cylinder { radius: 0.3, height: 1.0 },
                    "sphere" => SolidSpec::Sphere { radius: 0.5 },
                    _ => SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 },
                };
                let stock = Stock { id: fixture.stock.id.clone(), label: resolve_labels::<Process3dLabels>(config).stock.into(), solid, pose: Pose::default() };
                let document = Process3dDocument { workshop: fixture.workshop.clone(), stock, steps: Vec::new(), resolved_up_to: None };
                Emit {
                    document_operations: vec![Process3dOperation::SetDocument { document }],
                    config_operations: vec![Process3dConfigOperation::SetSelectedId { value: None }],
                    ..Default::default()
                }
            }
            Process3dCommand::PatchInspector { target, field, number, text } => {
                let value = number.map(|n| json!(n)).or_else(|| text.clone().map(Value::String));
                match process3d_inspector_patch_operation(fixture, target, field, value.as_ref()) {
                    Some(operation) => Emit::operations(vec![operation]),
                    None => Emit::default(),
                }
            }
            Process3dCommand::SetCursor { value } => {
                let resolved = value.map(|n| (n as usize).min(fixture.steps.len()));
                Emit::operations(vec![Process3dOperation::SetCursor { resolved_up_to: resolved }])
            }
            Process3dCommand::StepCursor { delta } => {
                let len = fixture.steps.len();
                let current = fixture.resolved_up_to.unwrap_or(len) as i64;
                Emit::operations(vec![Process3dOperation::SetCursor { resolved_up_to: Some((current + delta).clamp(0, len as i64) as usize) }])
            }
            Process3dCommand::StepCursorBack => {
                let len = fixture.steps.len();
                let current = fixture.resolved_up_to.unwrap_or(len) as i64;
                Emit::operations(vec![Process3dOperation::SetCursor { resolved_up_to: Some((current - 1).clamp(0, len as i64) as usize) }])
            }
            Process3dCommand::StepCursorForward => {
                let len = fixture.steps.len();
                let current = fixture.resolved_up_to.unwrap_or(len) as i64;
                Emit::operations(vec![Process3dOperation::SetCursor { resolved_up_to: Some((current + 1).clamp(0, len as i64) as usize) }])
            }
            Process3dCommand::SetActiveUtility { utility_id } => {
                Emit::config(vec![Process3dConfigOperation::SetActiveUtility { utility_id: utility_id.clone() }, Process3dConfigOperation::SetSelectedFaceId { value: None }])
            }
            Process3dCommand::EngagementInput { value } => Emit::config(vec![Process3dConfigOperation::SetEngagementInput { value: value.clone() }]),
            Process3dCommand::EngagementAbort => {
                Emit { config_operations: vec![Process3dConfigOperation::SetEngagementInput { value: String::new() }], effects: vec![set_active_utility_effect("select")], ..Default::default() }
            }
            Process3dCommand::EngagementSubmit => {
                let command_word = config.engagement_input.trim().to_lowercase();
                let len = fixture.steps.len();
                let current = fixture.resolved_up_to.unwrap_or(len);
                let clear_input = Process3dConfigOperation::SetEngagementInput { value: String::new() };
                match command_word.split_whitespace().next() {
                    Some("cut") => Emit { config_operations: vec![clear_input], effects: vec![set_active_utility_effect("cut")], ..Default::default() },
                    Some("drill") => Emit { config_operations: vec![clear_input], effects: vec![set_active_utility_effect("drill")], ..Default::default() },
                    Some("attach") => Emit { config_operations: vec![clear_input], effects: vec![set_active_utility_effect("attach")], ..Default::default() },
                    Some("back") => Emit { document_operations: vec![Process3dOperation::SetCursor { resolved_up_to: Some(current.saturating_sub(1)) }], config_operations: vec![clear_input], ..Default::default() },
                    Some("forward") => Emit { document_operations: vec![Process3dOperation::SetCursor { resolved_up_to: Some((current + 1).min(len)) }], config_operations: vec![clear_input], ..Default::default() },
                    Some("all") => Emit { document_operations: vec![Process3dOperation::SetCursor { resolved_up_to: None }], config_operations: vec![clear_input], ..Default::default() },
                    _ => Emit::config(vec![clear_input]),
                }
            }
            Process3dCommand::WorldPointerDown { position } => {
                let utility = process3d_active_utility(config);
                if utility == "select" {
                    return Emit::default();
                }
                let measure_kind = match utility {
                    "drill" => MeasureKind::Drill,
                    "attach" => MeasureKind::Attach,
                    _ => MeasureKind::Cut,
                };
                let (machine, capability) = capability_for_measure_kind(&fixture.workshop, measure_kind);
                let origin = StepOrigin { machine_id: machine.id.clone(), capability_id: capability.id.clone() };
                let step = ProcessStep { id: next_step_id(), label: capability.label.clone(), enabled: true, origin: Some(origin), measure: measure_for_capability(&capability, Some(*position)) };
                let step_id = step.id.clone();
                Emit {
                    document_operations: insert_step_operations(fixture, step),
                    config_operations: vec![Process3dConfigOperation::SetSelectedId { value: Some(step_id) }],
                    effects: vec![set_active_utility_effect("select")],
                    ..Default::default()
                }
            }
            Process3dCommand::WorldPick { granularity, id } => {
                if granularity == "face" {
                    Emit::config(vec![Process3dConfigOperation::SetSelectedFaceId { value: *id }])
                } else {
                    Emit::default()
                }
            }
            Process3dCommand::WorldFaceDragEnd { normal, start_point, distance, face_extent } => {
                if process3d_active_utility(config) != "select" {
                    return Emit::default();
                }
                match process3d_step_from_face_drag(*normal, *start_point, *distance, *face_extent, resolve_labels::<Process3dLabels>(config)) {
                    Some(step) => {
                        let step_id = step.id.clone();
                        Emit {
                            document_operations: insert_step_operations(fixture, step),
                            config_operations: vec![Process3dConfigOperation::SetSelectedId { value: Some(step_id) }, Process3dConfigOperation::SetSelectedFaceId { value: None }],
                            ..Default::default()
                        }
                    }
                    None => Emit::default(),
                }
            }
            Process3dCommand::ExportModel { format } => match export_process3d_model(fixture, format) {
                Some(export) => Emit::effect(HostEffect::DownloadMediaExport {
                    filename: export.filename,
                    mime_type: export.mime_type,
                    data: match export.data {
                        Value::String(text) => text,
                        other => serde_json::to_string(&other).unwrap_or_default(),
                    },
                    encoding: export.encoding,
                }),
                None => Emit::default(),
            },
            Process3dCommand::LoadModelRequest => Emit::effect(HostEffect::RequestFileOpen { accept: ".stp,.step,.obj,.stl,.glb".into(), read_as: Some("dataUrl".into()), import_action: "importModelFile".into(), multiple: false }),
            Process3dCommand::ImportModelFile { name, payload } => match import_process3d_model(&name.to_ascii_lowercase(), payload) {
                Some(document) => Emit {
                    document_operations: vec![Process3dOperation::SetDocument { document }],
                    config_operations: vec![Process3dConfigOperation::SetSelectedId { value: None }],
                    ..Default::default()
                },
                None => Emit::default(),
            },
            Process3dCommand::ToggleSun => {
                Emit::config(vec![Process3dConfigOperation::SetSun { enabled: !config.sun_enabled, azimuth: config.sun_azimuth, elevation: config.sun_elevation, intensity: config.sun_intensity, color: config.sun_color.clone() }])
            }
            Process3dCommand::SetSunAzimuth { value } => {
                Emit::config(vec![Process3dConfigOperation::SetSun { enabled: config.sun_enabled, azimuth: *value, elevation: config.sun_elevation, intensity: config.sun_intensity, color: config.sun_color.clone() }])
            }
            Process3dCommand::SetSunElevation { value } => {
                Emit::config(vec![Process3dConfigOperation::SetSun { enabled: config.sun_enabled, azimuth: config.sun_azimuth, elevation: *value, intensity: config.sun_intensity, color: config.sun_color.clone() }])
            }
            Process3dCommand::SetSunIntensity { value } => {
                Emit::config(vec![Process3dConfigOperation::SetSun { enabled: config.sun_enabled, azimuth: config.sun_azimuth, elevation: config.sun_elevation, intensity: *value, color: config.sun_color.clone() }])
            }
            Process3dCommand::SetSelection { id } => Emit::config(vec![Process3dConfigOperation::SetSelectedId { value: id.clone() }]),
            Process3dCommand::SetHover { id } => Emit::config(vec![Process3dConfigOperation::SetHoveredId { value: id.clone() }]),
            Process3dCommand::SetCamera { position, target, fov } => Emit::config(vec![Process3dConfigOperation::SetCamera { position: *position, target: *target, fov: *fov }]),
            Process3dCommand::SetLocale { value } => Emit::config(vec![Process3dConfigOperation::SetLocale { value: value.clone() }]),
        }
    }
    //#endregion 🔖️Handle

    /// 🧮️ process3d exposes no genuinely settings-like sticky defaults (unlike shooting's default
    /// shot/asset format) — every `Process3dConfig` field is session-only view state, so this stays at
    /// the trait default.
    fn config_spec(&self) -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec::empty()
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Process3dDocument>, cfg: &ConfigView<'_, Process3dConfig>) -> UiNode {
        let config = cfg.projection;
        let labels = resolve_labels::<Process3dLabels>(config);
        match body_key {
            PROCESS_3D_PLAY_BODY_MAIN => {
                let (meshes_json, instances_json) = preview_payload_cached(doc.projection);
                build_world_3d_scene(
                    PROCESS_3D_PLAY_SURFACE_MAIN,
                    PROCESS_3D_PLAY_APP_ID,
                    world3d_scene(
                        world3d_camera_json(config.camera_position, config.camera_target, config.camera_fov),
                        meshes_json,
                        instances_json,
                        process3d_selection_json(config, process3d_active_utility(config)),
                        &config_sun(config),
                    ),
                )
            }
            PROCESS_3D_PLAY_BODY_DOCUMENT => build_document_tree(doc.projection, config, labels),
            PROCESS_3D_PLAY_BODY_CATALOGUE => build_catalogue_tree(doc.projection, labels),
            PROCESS_3D_PLAY_BODY_WORKSHOP => build_workshop_tree(doc.projection, config, labels),
            PROCESS_3D_PLAY_BODY_INSPECTION => build_inspector_tree(doc.projection, config, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, doc: &DocumentView<'_, Process3dDocument>, cfg: &ConfigView<'_, Process3dConfig>) -> HashMap<String, WindowEngagement> {
        HashMap::from([(PROCESS_3D_PLAY_WINDOW_MAIN.into(), process3d_engagement(doc.projection, cfg.projection, process3d_active_utility(cfg.projection), resolve_labels::<Process3dLabels>(cfg.projection)))])
    }

    fn app_labels(&self, cfg: &ConfigView<'_, Process3dConfig>) -> AppLabelsOverlay {
        let labels = resolve_labels::<Process3dLabels>(cfg.projection);
        let is_de = is_de_locale(cfg.projection);
        AppLabelsOverlay::default()
            .window_kind_label(PROCESS_3D_PLAY_WINDOW_MAIN, labels.window_main)
            .action_labels(localized_label_map(is_de, PROCESS3D_ACTION_LABEL_ENTRIES))
            .utility_labels(localized_label_map(is_de, PROCESS3D_UTILITY_LABEL_ENTRIES))
    }

    fn window_measures(&self, _doc: &DocumentView<'_, Process3dDocument>, cfg: &ConfigView<'_, Process3dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        HashMap::from([(PROCESS_3D_PLAY_WINDOW_MAIN.into(), vec![world3d_sun_measures("process3d", &config_sun(config), process3d_action)])])
    }
}

/// 🧰️ Resolves the config-owned active utility, falling back to the default (matches
/// `Process3dConfig::default().active_utility_id`, so the fallback only ever triggers if a config
/// value somehow arrives empty).
fn process3d_active_utility(cfg: &Process3dConfig) -> &str {
    if cfg.active_utility_id.is_empty() { PROCESS3D_DEFAULT_UTILITY } else { cfg.active_utility_id.as_str() }
}

/// 🌞️ Reconstructs the shared framework `WorldSunConfig` shape from `Process3dConfig`'s flattened sun
/// fields — `world3d_scene`/`world3d_sun_measures` are shared SDK primitives that still take the
/// nested struct.
fn config_sun(cfg: &Process3dConfig) -> WorldSunConfig {
    WorldSunConfig { enabled: cfg.sun_enabled, azimuth: cfg.sun_azimuth, elevation: cfg.sun_elevation, intensity: cfg.sun_intensity, color: cfg.sun_color.clone() }
}
//#endregion 🔖️Process3dPlayApp

//#region 🔖️Manifest
pub fn create_process3d_app() -> App {
    App::from_builder(
        App::builder(PROCESS_3D_PLAY_APP_ID, "Process 3D")
            .document(["semio", "process", "3d"])
            .artifact_kind(ArtifactKindSpec {
                id: "3d.process".into(),
                name: "3D Process".into(),
                source_format: "process.3d".into(),
                component_kind: "process3d".into(),
                dimension: "3d".into(),
                media_capability: OsMediaCapability::Brep,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
                schema: "process.3d".into(),
                export_formats: vec![OsMediaFormat::Step, OsMediaFormat::Obj, OsMediaFormat::Stl, OsMediaFormat::Glb],
                import_formats: vec![OsMediaFormat::Step, OsMediaFormat::Obj, OsMediaFormat::Stl],
            })
            .icon_id("hammer")
            .mode("edit", "Edit", "pencil")
            .default_mode_id("edit")
            .window_kind_with_engagement(
                PROCESS_3D_PLAY_WINDOW_MAIN,
                "Workpiece",
                PROCESS_3D_PLAY_BODY_MAIN,
                SurfaceKind::World3d,
                process3d_engagement(&default_document(), &Process3dConfig::default(), PROCESS3D_DEFAULT_UTILITY, &Process3dLabels::EN),
                "process-workpiece",
            )
            .default_layout(create_default_layout(&[PROCESS_3D_PLAY_WINDOW_MAIN.into()], "row", None, Some(&["Workpiece".into()])))
            .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, PROCESS_3D_PLAY_BODY_DOCUMENT)
            .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, PROCESS_3D_PLAY_BODY_CATALOGUE)
            .panel_tab("workshop", "Workshop", PanelGroup::Workbench, PROCESS_3D_PLAY_BODY_WORKSHOP)
            .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, PROCESS_3D_PLAY_BODY_INSPECTION)
            // 🔧️ Palette-visible create/mutate actions (staged arg forms attached below).
            .operation("addStep", "Add Step")
            .operation("setStock", "Set Stock")
            .operation("setActiveExample", "Set Active Example")
            .operation("removeSelectedStep", "Remove Selected Step")
            // 🐚️ Palette-visible host round-trips.
            .shell_action("exportModel", "Export Model")
            .shell_action("loadModelRequest", "Load Model…")
            // 🔧️ Internal document mutations dispatched by panel/viewport wiring (not palette-worthy).
            .action_with(internal_action("setDocument", "Set Document", ActionKind::Operation))
            .action_with(internal_action("addWorkshopMachine", "Add Machine", ActionKind::Operation))
            .action_with(internal_action("removeWorkshopMachine", "Remove Machine", ActionKind::Operation))
            .action_with(internal_action("updateWorkshopMachine", "Update Machine", ActionKind::Operation))
            .action_with(internal_action("importModelFile", "Import Model File", ActionKind::Operation))
            .action_with(internal_action("removeStep", "Remove Step", ActionKind::Operation))
            .action_with(internal_action("moveStep", "Move Step", ActionKind::Operation))
            .action_with(internal_action("updateStep", "Update Step", ActionKind::Operation))
            .action_with(internal_action("setStepEnabled", "Set Step Enabled", ActionKind::Operation))
            .action_with(internal_action("patchInspector", "Patch Inspector", ActionKind::Operation))
            .action_with(internal_action("worldPointerDown", "World Pointer Down", ActionKind::Operation))
            .action_with(internal_action("worldFaceDragEnd", "World Face Drag End", ActionKind::Operation))
            // ⏱️ Document-cursor navigation operations (NOT framework History — they move the replay cursor).
            .action_with(internal_action("setCursor", "Set Cursor", ActionKind::Operation))
            .action_with(internal_action("stepCursor", "Step Cursor", ActionKind::Operation))
            .action_with(internal_action("stepCursorBack", "Step Cursor Back", ActionKind::Operation))
            .action_with(internal_action("stepCursorForward", "Step Cursor Forward", ActionKind::Operation))
            // 🎛️ Engagement session command line (a separate system from utility selection).
            .action_with(internal_action("engagementSubmit", "Engagement Submit", ActionKind::Operation))
            .action_with(internal_action("engagementInput", "Engagement Input", ActionKind::View))
            .action_with(internal_action("engagementAbort", "Engagement Abort", ActionKind::View))
            // 👁️ Ephemeral view state — selection, hover, camera, face picking, sun.
            .action_with(internal_action("setSelection", "Set Selection", ActionKind::View))
            .action_with(internal_action("setHover", "Set Hover", ActionKind::View))
            .action_with(internal_action("setCamera", "Set Camera", ActionKind::View))
            .action_with(internal_action("worldPick", "World Pick", ActionKind::View))
            .action_with(internal_action("toggleSun", "Toggle Sun", ActionKind::View))
            .action_with(internal_action("setSunAzimuth", "Set Sun Azimuth", ActionKind::View))
            .action_with(internal_action("setSunElevation", "Set Sun Elevation", ActionKind::View))
            .action_with(internal_action("setSunIntensity", "Set Sun Intensity", ActionKind::View))
            // 📝️ Staged argument forms for the palette-visible create/export actions.
            .action_args("addStep", vec![
                ActionArgDef::select("measure", "Measure", vec![
                    ActionArgOption::new("cut", "Cut"),
                    ActionArgOption::new("drill", "Drill"),
                    ActionArgOption::new("attach", "Attach"),
                ]).default_value("cut"),
            ])
            .action_args("setStock", vec![
                ActionArgDef::select("kind", "Kind", vec![
                    ActionArgOption::new("box", "Box"),
                    ActionArgOption::new("cylinder", "Cylinder"),
                    ActionArgOption::new("sphere", "Sphere"),
                ]).default_value("box"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", "Example", vec![
                    ActionArgOption::new(PROCESS3D_EXAMPLE_TIMBER, "Timber Beam Joinery"),
                    ActionArgOption::new(PROCESS3D_EXAMPLE_PLATE, "Drilled Plate"),
                ]).required().default_value(PROCESS3D_EXAMPLE_TIMBER),
            ])
            .action_args("exportModel", vec![
                ActionArgDef::select("format", "Format", vec![
                    ActionArgOption::new("step", "STEP"),
                    ActionArgOption::new("obj", "OBJ"),
                    ActionArgOption::new("stl", "STL"),
                    ActionArgOption::new("glb", "GLB"),
                ]).required().default_value("step"),
            ])
            // 🧰️ Flat top-level exclusive utility bar scoped to the workpiece window (active utility is
            // host-owned). These four are the window's entire utility set — not a sub-collection — so
            // each carries `group: None` and renders as its own flat utility bar icon.
            .utility(UtilityDefinition { category: Some(UtilityCategory::Selection), ..UtilityDefinition::new("select", "Select", "mouse-pointer") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("cut", "Cut", "scissors") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("drill", "Drill", "circle-dot") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("attach", "Attach", "plus") })
            .window_kind_utilities(PROCESS_3D_PLAY_WINDOW_MAIN, vec!["select".into(), "cut".into(), "drill".into(), "attach".into()])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("bracketleft", "stepCursorBack")
            .keybinding("bracketright", "stepCursorForward")
            .keybinding("escape", "engagementAbort")
            .keybinding("delete", "removeSelectedStep")
            .keybinding("backspace", "removeSelectedStep")
            .config(Process3dPlayApp::default().config_spec())
            .io(process_3d_engine::process3d_io()),
    )
    .example(PROCESS3D_EXAMPLE_TIMBER, "Timber Beam Joinery", process_3d_engine::TIMBER_EXAMPLE_DSL, "file-text")
    .example(PROCESS3D_EXAMPLE_PLATE, "Drilled Plate", process_3d_engine::PLATE_EXAMPLE_DSL, "file-text")
    .workflow("process3d", "Process 3D", "brep")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, HistoryView, PluginApp, VcsDocumentApp, ViewState};

    fn new_app() -> VcsDocumentApp<Process3dPlayApp> {
        testkit::new_app::<Process3dPlayApp>()
    }

    /// 🧰️ Dispatches `SetActiveUtility` to persist a specific host-owned active utility into config —
    /// mirrors how the shell threads `active_utility_id` after a utility bar switch (was constructing a
    /// per-call `ViewState` before B1).
    fn set_utility(app: &mut VcsDocumentApp<Process3dPlayApp>, utility: &str) {
        app.dispatch_typed(Process3dCommand::SetActiveUtility { utility_id: utility.into() }, &testkit::meta("local")).expect("set utility");
    }

    #[test]
    fn utility_registry_declares_four_flat_utilities_scoped_to_workpiece_window() {
        let definition = create_process3d_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["select", "cut", "drill", "attach"], "utilities declared in registry order");
        assert!(
            definition.utilities.iter().all(|utility| utility.group.is_none()),
            "process's select/cut/drill/attach are the window's entire top-level utility set, so none carry a visual group (a shared group would fold them into one collection button)",
        );
        let window = definition.window_kinds.iter().find(|window| window.id == PROCESS_3D_PLAY_WINDOW_MAIN).expect("workpiece window");
        let scoped: Vec<&str> = window.utilities.iter().map(|utility| utility.as_str()).collect();
        assert_eq!(scoped, ["select", "cut", "drill", "attach"], "all four utilities scoped to the workpiece window kind");
    }

    #[test]
    fn labels_resolve_native_by_default_and_in_german() {
        let mut config = Process3dConfig::default();
        assert_eq!(resolve_labels::<Process3dLabels>(&config).stock, "Stock");
        config.locale = "de".into();
        assert_eq!(resolve_labels::<Process3dLabels>(&config).stock, "Rohteil");
    }

    #[test]
    fn toggle_sun_round_trips_through_config_and_defaults_off() {
        let mut app = new_app();
        let measures = app.window_measures();
        let sun_group = |measures: &HashMap<String, Vec<WindowMeasure>>| {
            measures[PROCESS_3D_PLAY_WINDOW_MAIN]
                .iter()
                .find_map(|measure| match measure {
                    WindowMeasure::Group { id, children, .. } if id == "process3d-measure-sun" => Some(children.clone()),
                    _ => None,
                })
                .expect("sun measure group")
        };
        let children = sun_group(&measures);
        assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { pressed, .. } if !*pressed)));
        app.dispatch_typed(Process3dCommand::ToggleSun, &testkit::meta("local")).expect("toggle");
        let measures = app.window_measures();
        let children = sun_group(&measures);
        assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { pressed, .. } if *pressed)));
    }

    #[test]
    fn add_step_action_inserts_and_selects() {
        let mut app = new_app();
        app.dispatch_typed(Process3dCommand::AddStep { measure: Some("drill".into()), machine_id: None, capability_id: None, position: None }, &testkit::meta("local")).expect("add step");
        let document = app.projection().expect("projection");
        assert_eq!(document.steps.len(), 5);
        let node = app.render(PROCESS_3D_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        let node_json = serde_json::to_string(&node).unwrap();
        assert!(!node_json.contains("No selection"), "expected the newly added step to be selected: {node_json}");
    }

    #[test]
    fn undo_after_add_step_restores_previous_step_count() {
        let mut app = new_app();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            Process3dCommand::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None },
            |app| app.projection().expect("projection").steps.len(),
            4,
            5,
        );
    }

    #[test]
    fn set_active_utility_emits_no_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(Process3dCommand::SetActiveUtility { utility_id: "cut".into() }, &testkit::meta("local")).expect("set utility");
        assert!(result.operations.is_empty(), "utility selection is host-owned config state and must never emit document operations or history");
    }

    #[test]
    fn engagement_exposes_no_utility_switch_options() {
        let doc = Process3dDocument::default();
        let engagement = process3d_engagement(&doc, &Process3dConfig::default(), "cut", &Process3dLabels::EN);
        assert!(engagement.options.is_none(), "select/cut/drill/attach switching lives only on the framework utility bar; the engagement must not duplicate it as options",);
    }

    #[test]
    fn arg_form_set_stock_emits_ops_reading_kind_arg() {
        let mut app = new_app();
        let result = app.dispatch_typed(Process3dCommand::SetStock { kind: "cylinder".into() }, &testkit::meta("local")).expect("set stock");
        assert!(!result.operations.is_empty(), "the setStock arg form must materialize into document operations");
        let document = app.projection().expect("projection");
        assert!(matches!(document.stock.solid, SolidSpec::Cylinder { .. }), "setStock kind=cylinder must swap the stock solid");
        assert!(document.steps.is_empty(), "swapping stock resets the step timeline");
    }

    fn step_pose(step: &ProcessStep) -> [f64; 3] {
        match &step.measure {
            ProcessMeasure::Cut { pose, .. } | ProcessMeasure::Drill { pose, .. } | ProcessMeasure::Attach { pose, .. } => pose.position,
        }
    }

    #[test]
    fn world_pointer_down_reads_position_field_not_point() {
        let mut app = new_app();
        set_utility(&mut app, "cut");
        let result = app.dispatch_typed(Process3dCommand::WorldPointerDown { position: [1.0, 2.0, 3.0] }, &testkit::meta("local")).expect("pointer down");
        assert!(!result.operations.is_empty(), "worldPointerDown must read the position the renderer actually sends");
        let document = app.projection().expect("projection");
        let last = document.steps.last().expect("inserted step");
        assert_eq!(step_pose(last), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn world_pointer_down_resets_active_utility_to_select() {
        let mut app = new_app();
        set_utility(&mut app, "cut");
        let result = app.dispatch_typed(Process3dCommand::WorldPointerDown { position: [1.0, 2.0, 3.0] }, &testkit::meta("local")).expect("pointer down");
        assert!(
            result.requested_effects.iter().any(|effect| matches!(effect, HostEffect::SetActiveUtility { utility_id, .. } if utility_id == "select")),
            "placing a step must hand the host a SetActiveUtility(select) effect so the click-to-place utility disengages",
        );
    }

    #[test]
    fn repeated_world_pointer_down_places_steps_at_distinct_positions() {
        let mut app = new_app();
        set_utility(&mut app, "cut");
        app.dispatch_typed(Process3dCommand::WorldPointerDown { position: [1.0, 0.0, 0.0] }, &testkit::meta("local")).expect("pointer 1");
        set_utility(&mut app, "cut");
        app.dispatch_typed(Process3dCommand::WorldPointerDown { position: [2.0, 0.0, 0.0] }, &testkit::meta("local")).expect("pointer 2");
        let document = app.projection().expect("projection");
        let last_two: Vec<&ProcessStep> = document.steps.iter().rev().take(2).collect();
        assert_ne!(step_pose(last_two[0]), step_pose(last_two[1]), "repeated clicks at different points must produce distinct step poses");
    }

    #[test]
    fn face_drag_negative_distance_yields_cut() {
        let step = process3d_step_from_face_drag([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], -0.5, None, &Process3dLabels::EN).expect("step");
        assert!(matches!(step.measure, ProcessMeasure::Cut { .. }));
        assert_eq!(step.label, "Push Cut");
    }

    #[test]
    fn face_drag_positive_distance_yields_attach() {
        let step = process3d_step_from_face_drag([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 0.5, None, &Process3dLabels::EN).expect("step");
        assert!(matches!(step.measure, ProcessMeasure::Attach { .. }));
        assert_eq!(step.label, "Pull Attach");
    }

    #[test]
    fn face_drag_zero_distance_is_noop() {
        assert!(process3d_step_from_face_drag([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 0.0, None, &Process3dLabels::EN).is_none());
    }

    #[test]
    fn world_face_drag_end_cut_reduces_volume_end_to_end() {
        let mut app = new_app();
        app.dispatch_typed(Process3dCommand::SetStock { kind: "box".into() }, &testkit::meta("local")).expect("set stock");
        let stock_volume = processed_volume(&app.projection().expect("projection")).expect("stock volume");
        let result = app
            .dispatch_typed(Process3dCommand::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: -0.5, face_extent: Some([1.0, 1.0]) }, &testkit::meta("local"))
            .expect("face drag");
        assert!(!result.operations.is_empty());
        let document = app.projection().expect("projection");
        assert_eq!(document.steps.len(), 1);
        assert!(matches!(document.steps[0].measure, ProcessMeasure::Cut { .. }));
        let new_volume = processed_volume(&document).expect("volume after cut");
        assert!(new_volume < stock_volume, "face-drag cut should reduce volume below stock ({new_volume} vs {stock_volume})");
    }

    #[test]
    fn world_face_drag_end_attach_increases_volume_end_to_end() {
        let mut app = new_app();
        app.dispatch_typed(Process3dCommand::SetStock { kind: "box".into() }, &testkit::meta("local")).expect("set stock");
        let stock_volume = processed_volume(&app.projection().expect("projection")).expect("stock volume");
        let result = app
            .dispatch_typed(Process3dCommand::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: 0.5, face_extent: Some([0.2, 0.2]) }, &testkit::meta("local"))
            .expect("face drag");
        assert!(!result.operations.is_empty());
        let document = app.projection().expect("projection");
        assert_eq!(document.steps.len(), 1);
        assert!(matches!(document.steps[0].measure, ProcessMeasure::Attach { .. }));
        let new_volume = processed_volume(&document).expect("volume after attach");
        assert!(new_volume > stock_volume, "face-drag attach should increase volume above stock ({new_volume} vs {stock_volume})");
    }

    #[test]
    fn world_face_drag_end_ignored_while_a_placement_utility_is_active() {
        let mut app = new_app();
        set_utility(&mut app, "cut");
        let result = app
            .dispatch_typed(Process3dCommand::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: -0.5, face_extent: None }, &testkit::meta("local"))
            .expect("face drag");
        assert!(result.operations.is_empty(), "worldFaceDragEnd should be a no-operation while a placement utility is active, not the select utility");
    }

    #[test]
    fn render_world_scene_contains_processed_mesh() {
        let mut app = new_app();
        let node = app.render(PROCESS_3D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let node_json = serde_json::to_string(&node).expect("scene json");
        assert!(node_json.contains("processed"), "expected the processed mesh id in scene json: {node_json}");
    }

    /// 🪵️ The default timber beam (0.24m tall) exceeds both the circular saw's 0.065m max cut depth
    /// and the table saw's 0.102m — both wood machines list, both are disabled with a reason.
    #[test]
    fn catalogue_lists_workshop_wood_machines_with_mixed_validity_on_default_stock() {
        let mut app = new_app();
        let node = app.render(PROCESS_3D_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let node_json = serde_json::to_string(&node).expect("catalogue json");
        assert!(node_json.contains("Circular Saw"), "expected wood's circular saw in the catalogue: {node_json}");
        assert!(node_json.contains("Table Saw"), "expected wood's table saw in the catalogue: {node_json}");
        assert!(node_json.contains("needs stock"), "expected at least one disabled-item validation reason: {node_json}");
    }

    #[test]
    fn add_step_via_catalogue_sets_origin_and_builds_capability_sized_tool() {
        let mut app = new_app();
        // 🪚️ Circular saw's realistic 0.065m max cut depth needs a shallower stock than the default 0.24m beam.
        app.dispatch_typed(Process3dCommand::PatchInspector { target: "beam".into(), field: "height".into(), number: Some(0.05), text: None }, &testkit::meta("local")).expect("shrink stock to fit circular saw");
        let result = app.dispatch_typed(Process3dCommand::AddStep { measure: None, machine_id: Some("circularSaw".into()), capability_id: Some("crosscut".into()), position: None }, &testkit::meta("local")).expect("add step");
        assert!(!result.operations.is_empty(), "circular saw crosscut should be valid against the shrunk stock");
        let document = app.projection().expect("projection");
        let last = document.steps.last().expect("inserted step");
        let origin = last.origin.as_ref().expect("origin");
        assert_eq!(origin.machine_id, "circularSaw");
        assert_eq!(origin.capability_id, "crosscut");
        let ProcessMeasure::Cut { tool: SolidSpec::Cylinder { radius, .. }, .. } = &last.measure else {
            panic!("expected a cylinder cut tool, got {:?}", last.measure);
        };
        assert!((radius - 0.092).abs() < 1e-9, "circular saw diameter 0.184 should size the tool to radius 0.092, got {radius}");
    }

    /// 🪵️ Table saw needs stock height <= 0.102m; the default timber beam is 0.24m tall.
    #[test]
    fn add_step_via_catalogue_rejected_when_validation_fails() {
        let mut app = new_app();
        let result = app.dispatch_typed(Process3dCommand::AddStep { measure: None, machine_id: Some("tableSaw".into()), capability_id: Some("crosscut".into()), position: None }, &testkit::meta("local")).expect("add step");
        assert!(result.operations.is_empty(), "table saw crosscut should be rejected server-side against oversized stock");
    }

    #[test]
    fn measure_arg_routes_to_generic_machine() {
        let mut app = new_app();
        app.dispatch_typed(Process3dCommand::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }, &testkit::meta("local")).expect("add step");
        let document = app.projection().expect("projection");
        let last = document.steps.last().expect("inserted step");
        let origin = last.origin.as_ref().expect("origin");
        assert_eq!(origin.machine_id, "saw");
        assert_eq!(origin.capability_id, "cut");
        assert!(matches!(last.measure, ProcessMeasure::Cut { .. }));
    }

    #[test]
    fn inspector_shows_validation_warning_after_stock_grows_above_step_requirement() {
        let mut app = new_app();
        app.dispatch_typed(Process3dCommand::PatchInspector { target: "beam".into(), field: "height".into(), number: Some(0.05), text: None }, &testkit::meta("local")).expect("shrink stock to fit circular saw");
        let add_result = app
            .dispatch_typed(Process3dCommand::AddStep { measure: None, machine_id: Some("circularSaw".into()), capability_id: Some("crosscut".into()), position: None }, &testkit::meta("local"))
            .expect("add step");
        assert!(!add_result.operations.is_empty());
        app.dispatch_typed(Process3dCommand::PatchInspector { target: "beam".into(), field: "height".into(), number: Some(0.5), text: None }, &testkit::meta("local")).expect("grow stock");
        let step_id = app.projection().expect("projection").steps.last().expect("step").id.clone();
        app.dispatch_typed(Process3dCommand::SetSelection { id: Some(step_id) }, &testkit::meta("local")).expect("select");
        let node = app.render(PROCESS_3D_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        let node_json = serde_json::to_string(&node).expect("inspector json");
        assert!(node_json.contains("needs stock"), "expected a validation warning after growing stock above the step's max cut depth: {node_json}");
    }

    //#region 🔖️WorkshopUiTests
    #[test]
    fn add_workshop_machine_action_installs_and_selects() {
        let mut app = new_app();
        let result = app.dispatch_typed(Process3dCommand::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }, &testkit::meta("local")).expect("add machine");
        assert!(!result.operations.is_empty(), "adding an uninstalled catalog machine must emit an operation");
        let document = app.projection().expect("projection");
        assert!(document.workshop.machines.iter().any(|machine| machine.id == "chopSaw"), "chopSaw should now be in the workshop");
        let node = app.render(PROCESS_3D_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        let node_json = serde_json::to_string(&node).unwrap();
        assert!(!node_json.contains("No selection"), "expected the newly added machine to be selected: {node_json}");
    }

    #[test]
    fn add_workshop_machine_action_is_idempotent_when_already_installed() {
        let mut app = new_app();
        app.dispatch_typed(Process3dCommand::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }, &testkit::meta("local")).expect("add machine");
        let count_after_first = app.projection().expect("projection").workshop.machines.len();
        let result = app.dispatch_typed(Process3dCommand::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }, &testkit::meta("local")).expect("add machine again");
        assert!(result.operations.is_empty(), "adding an already-installed machine must be a no-op");
        assert_eq!(app.projection().expect("projection").workshop.machines.len(), count_after_first);
    }

    #[test]
    fn undo_after_add_workshop_machine_restores_previous_machine_count() {
        let mut app = new_app();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            Process3dCommand::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() },
            |app| app.projection().expect("projection").workshop.machines.len(),
            7,
            8,
        );
    }

    #[test]
    fn remove_workshop_machine_action_removes_and_clears_selection() {
        let mut app = new_app();
        app.dispatch_typed(Process3dCommand::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }, &testkit::meta("local")).expect("add machine");
        let result = app.dispatch_typed(Process3dCommand::RemoveWorkshopMachine { id: "chopSaw".into() }, &testkit::meta("local")).expect("remove machine");
        assert!(!result.operations.is_empty());
        let document = app.projection().expect("projection");
        assert!(!document.workshop.machines.iter().any(|machine| machine.id == "chopSaw"));
        let node = app.render(PROCESS_3D_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        let node_json = serde_json::to_string(&node).unwrap();
        assert!(node_json.contains("No selection"), "removing the selected machine must clear the selection: {node_json}");
    }

    #[test]
    fn workshop_machine_parameter_edit_resizes_future_tool() {
        let mut app = new_app();
        app.dispatch_typed(Process3dCommand::PatchInspector { target: "beam".into(), field: "height".into(), number: Some(0.05), text: None }, &testkit::meta("local")).expect("shrink stock to fit circular saw");
        app.dispatch_typed(Process3dCommand::PatchInspector { target: "machine:circularSaw".into(), field: "crosscut.bladeDiameter".into(), number: Some(0.4), text: None }, &testkit::meta("local")).expect("edit parameter");
        let result =
            app.dispatch_typed(Process3dCommand::AddStep { measure: None, machine_id: Some("circularSaw".into()), capability_id: Some("crosscut".into()), position: None }, &testkit::meta("local")).expect("add step");
        assert!(!result.operations.is_empty());
        let document = app.projection().expect("projection");
        let last = document.steps.last().expect("inserted step");
        let ProcessMeasure::Cut { tool: SolidSpec::Cylinder { radius, .. }, .. } = &last.measure else {
            panic!("expected a cylinder cut tool, got {:?}", last.measure);
        };
        assert!((radius - 0.2).abs() < 1e-9, "edited blade diameter 0.4 should size the next tool to radius 0.2, got {radius}");
    }

    #[test]
    fn catalogue_reflects_workshop_after_machine_removal() {
        let mut app = new_app();
        let before = app.render(PROCESS_3D_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&before).unwrap().contains("Circular Saw"));
        app.dispatch_typed(Process3dCommand::RemoveWorkshopMachine { id: "circularSaw".into() }, &testkit::meta("local")).expect("remove machine");
        let after = app.render(PROCESS_3D_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        assert!(!serde_json::to_string(&after).unwrap().contains("Circular Saw"), "removed machine must disappear from the catalogue tree");
    }

    #[test]
    fn step_inspector_shows_raw_provenance_after_machine_removal() {
        let mut app = new_app();
        app.dispatch_typed(Process3dCommand::PatchInspector { target: "beam".into(), field: "height".into(), number: Some(0.05), text: None }, &testkit::meta("local")).expect("shrink stock to fit circular saw");
        app.dispatch_typed(Process3dCommand::AddStep { measure: None, machine_id: Some("circularSaw".into()), capability_id: Some("crosscut".into()), position: None }, &testkit::meta("local")).expect("add step");
        let step_id = app.projection().expect("projection").steps.last().expect("step").id.clone();
        app.dispatch_typed(Process3dCommand::RemoveWorkshopMachine { id: "circularSaw".into() }, &testkit::meta("local")).expect("remove machine");
        app.dispatch_typed(Process3dCommand::SetSelection { id: Some(step_id) }, &testkit::meta("local")).expect("select step");
        let node = app.render(PROCESS_3D_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        let node_json = serde_json::to_string(&node).unwrap();
        assert!(node_json.contains("circularSaw") && node_json.contains("crosscut"), "step provenance must survive machine removal as raw ids: {node_json}");
    }
    //#endregion 🔖️WorkshopUiTests

    //#region 🔖️MediaTests
    #[test]
    fn export_brep_out_returns_step_text_structured_payload() {
        let app = new_app();
        let document = app.projection().expect("projection");
        let history = HistoryView::empty();
        let doc = DocumentView { projection: &document, history: &history };
        let media = Process3dPlayApp.export_media("brep:out", &doc).expect("export brep:out");
        assert_eq!(media.media_type.class, MediaClass::ThreeD);
        assert_eq!(media.media_type.form, MediaForm::Brep);
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "3d.process");
                assert!(!json.is_empty());
            }
            MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
        }
    }

    #[test]
    fn export_unknown_port_is_not_implemented() {
        let app = new_app();
        let document = app.projection().expect("projection");
        let history = HistoryView::empty();
        let doc = DocumentView { projection: &document, history: &history };
        assert!(matches!(Process3dPlayApp.export_media("nonsense:out", &doc), Err(MediaError::NotImplemented)));
    }

    #[test]
    fn import_geometry_in_rejects_unrecognized_schema() {
        let app = new_app();
        let document = app.projection().expect("projection");
        let history = HistoryView::empty();
        let doc = DocumentView { projection: &document, history: &history };
        let media = Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, payload: MediaPayload::Structured { schema: "unknown.schema".into(), json: "irrelevant".into() } };
        assert!(matches!(Process3dPlayApp.import_media("geometry:in", &media, &doc), Err(MediaError::Payload(port, _)) if port == "geometry:in"));
    }
    //#endregion 🔖️MediaTests
}
//#endregion 🧪️Tests
