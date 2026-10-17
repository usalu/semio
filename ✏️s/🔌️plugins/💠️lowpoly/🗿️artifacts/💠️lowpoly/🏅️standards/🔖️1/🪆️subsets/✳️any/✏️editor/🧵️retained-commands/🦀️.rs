//! 🧵️ Lowpoly retained commands: the migrated tool ids, the retained payload limits, the command disposition and admission, the resumable reducer, the command work cursor and the owned tool job factory.

use super::*;
use crate::lowpoly_paint_step;

//#region 🧵️RetainedCommands
const LOWPOLY_RETAINED_PAYLOAD_SCHEMA: &str = "lowpoly.command.v1";

pub(super) const LOWPOLY_RETAINED_RAW_BYTES: usize = 16_384;

pub(super) const LOWPOLY_RETAINED_WORK_ITEMS: usize = 258;

pub(super) const LOWPOLY_RETAINED_PAINT_RUNS: usize = 4_096;

const LOWPOLY_RETAINED_FIELD_BYTES: usize = 4_096;

const LOWPOLY_RETAINED_OBJECTS: usize = 64;

const LOWPOLY_RETAINED_PAINT_LAYERS_PER_OBJECT: usize = 8;

const LOWPOLY_RETAINED_PAINT_LAYER_BYTES: usize = 4 * 1024 * 1024;

const LOWPOLY_MIGRATED_TOOL_IDS: &[&str] = &[
    "patchObject",
    "addPaintLayer",
    "setActiveObject",
    "setActivePaintLayer",
    "setUtilityParam",
    "engagementInput",
    "toggleShowEdges",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setCamera",
    "importSnapshotJson",
    "replaceSnapshotJson",
    "exportMesh",
    "loadMeshRequest",
    "importMeshFile",
    "deleteSelection",
    "duplicateObject",
    "paintSample",
    "extrude",
    "inset",
    "bevel",
    "loopCut",
    "subdivide",
    "triangulate",
    "mirror",
    "decimate",
    "flipFaces",
    "merge",
    "dissolve",
    "snap",
    "toggleSmooth",
    "unwrapActive",
    "markUvSeam",
    "clearSeam",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "paintFill",
    "fillBucket",
    "engagementSubmit",
    "paintStroke",
    "paintAt",
    "canvasPointerDown",
    "canvasPointerMove",
    "canvasPointerUp",
    "addPrimitive",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum LowpolyCommandDisposition {
    Artifact = 1,
    Config = 2,
    HostOnly = 3,
    ArtifactTransient = 6,
    /// 🌱️ `addPrimitive` only: its handler unconditionally emits both an `Artifact` mutation
    /// (`CreateObject`) and a `Config` mutation (`SetActiveObject`), AND — like every other
    /// `session::build_doc`/`mesh_edit` reacher — reads and writes the session-local `mesh_workspace`
    /// cache, so it needs the same rehydrate-then-republish treatment `ArtifactTransient` documents.
    /// The coordinator's own schema edit (`schema://s.lowpoly/LowpolyInteractiveJobPartition`'s 8th
    /// `oneOf` arm, `["Artifact","Config","Transient"]`/`["Artifact","Config"]`) is what makes this
    /// representable.
    ArtifactConfigTransient = 7,
}

pub(super) fn lowpoly_command_disposition(tool_id: &str) -> Option<LowpolyCommandDisposition> {
    Some(match tool_id {
        // 🧲️ A fill commits one one-shot transaction; the gumball reads the live mesh cache to resolve its selection but
        // writes only its relative leaves, so its rehydrated cache is never republished.
        "patchObject" | "addPaintLayer" | "paintFill" | "fillBucket" | "translateSelection" | "rotateSelection" | "scaleSelection" => LowpolyCommandDisposition::Artifact,
        // 🕸️ Every one of these reaches `session::build_doc`/`mesh_edit`, which needs the session-local `mesh_workspace`
        // cache seeded from the LIVE persisted `LowpolyTransient` — `LowpolyDocument::reload_meshes` rejects a stale one
        // with `StaleMeshWorkspace` — so `lowpoly_retained_reduce`'s `threaded!` arms rehydrate scratch from
        // `context.transient` and publish the post-handle cache back: `Artifact` (the edit) `+ Transient` (the cache).
        "extrude" | "inset" | "bevel" | "loopCut" | "subdivide" | "triangulate" | "mirror" | "decimate" | "flipFaces" | "merge" | "dissolve" | "snap" | "toggleSmooth" | "unwrapActive" | "markUvSeam" | "clearSeam" | "engagementSubmit" | "deleteSelection" => LowpolyCommandDisposition::ArtifactTransient,
        "importSnapshotJson" | "replaceSnapshotJson" | "exportMesh" | "loadMeshRequest" | "importMeshFile" => LowpolyCommandDisposition::HostOnly,
        // 🖌️ Every paint verb drives the window's paint gesture in the transient, commits its transaction on release (or as a
        // one-shot) and, under the eyedropper, samples into config — the tick cannot tell statically which.
        "paintStroke" | "paintAt" | "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" => LowpolyCommandDisposition::ArtifactConfigTransient,
        // 🌱️ `addPrimitive`'s handler unconditionally emits both a `CreateObject` Artifact mutation and
        // a `SetActiveObject` Config mutation, and it reaches `session::build_doc` — same as every
        // `ArtifactTransient` command above — so it needs the identical scratch rehydration/republication.
        "addPrimitive" | "duplicateObject" => LowpolyCommandDisposition::ArtifactConfigTransient,
        tool_id if LOWPOLY_MIGRATED_TOOL_IDS.contains(&tool_id) => LowpolyCommandDisposition::Config,
        _ => return None,
    })
}

fn lowpoly_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(LOWPOLY_RETAINED_RAW_BYTES, LOWPOLY_RETAINED_WORK_ITEMS, 1, 32 * 1024 * 1024, 7_500, 1, 1)
}

pub(super) fn lowpoly_snapshot_admitted(snapshot: &LowpolySnapshot) -> bool {
    snapshot.schema.len() <= LOWPOLY_RETAINED_FIELD_BYTES
        && snapshot.objects.len() <= LOWPOLY_RETAINED_OBJECTS
        && snapshot.objects.iter().all(|object| {
            object.id.len() <= LOWPOLY_RETAINED_FIELD_BYTES
                && object.name.len() <= LOWPOLY_RETAINED_FIELD_BYTES
                && object.mesh.as_ref().is_none_or(|mesh| mesh.child_id.len() <= LOWPOLY_RETAINED_FIELD_BYTES && mesh.target.to_uri().len() <= LOWPOLY_RETAINED_FIELD_BYTES)
                && object.paint_layers.len() <= LOWPOLY_RETAINED_PAINT_LAYERS_PER_OBJECT
                && object.paint_layers.iter().all(|layer| layer.name.len() <= LOWPOLY_RETAINED_FIELD_BYTES && layer.blend_mode.len() <= LOWPOLY_RETAINED_FIELD_BYTES && layer.pixels.len() <= LOWPOLY_RETAINED_PAINT_LAYER_BYTES)
        })
}

pub(super) fn lowpoly_command_admitted(command: &LowpolyCommand, snapshot: &LowpolySnapshot, config: &LowpolyConfig) -> bool {
    let field = |value: &str| value.len() <= LOWPOLY_RETAINED_FIELD_BYTES;
    lowpoly_snapshot_admitted(snapshot)
        && lowpoly_config_retained_bytes(config) <= LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES
        && match command {
            LowpolyCommand::PatchObject(payload) => field(&payload.object_id) && field(&payload.field) && payload.value_json.as_deref().is_none_or(field),
            LowpolyCommand::AddPaintLayer(payload) => payload.object_id.as_deref().is_none_or(field) && payload.name.as_deref().is_none_or(field),
            LowpolyCommand::SetActiveObject(payload) => field(&payload.object_id),
            LowpolyCommand::SetUtilityParam(payload) => field(&payload.key) && field(&payload.value_json),
            LowpolyCommand::EngagementInput(payload) => field(&payload.value),
            LowpolyCommand::ImportSnapshotJson(payload) => payload.json.len() <= LOWPOLY_RETAINED_RAW_BYTES,
            LowpolyCommand::ReplaceSnapshotJson(payload) => payload.json.len() <= LOWPOLY_RETAINED_RAW_BYTES,
            LowpolyCommand::ExportMesh(payload) => field(&payload.format),
            LowpolyCommand::LoadMeshRequest(_) => true,
            LowpolyCommand::ImportMeshFile(payload) => field(&payload.name) && payload.payload.len() <= media::LOWPOLY_MESH_FILE_BYTES,
            LowpolyCommand::DeleteSelection(_) => true,
            LowpolyCommand::DuplicateObject(payload) => payload.object_id.as_deref().is_none_or(field),
            LowpolyCommand::PaintSample(payload) => payload.object_id.as_deref().is_none_or(field),
            LowpolyCommand::SetActivePaintLayer(_)
            | LowpolyCommand::ToggleShowEdges(_)
            | LowpolyCommand::ToggleSun(_)
            | LowpolyCommand::SetSunAzimuth(_)
            | LowpolyCommand::SetSunElevation(_)
            | LowpolyCommand::SetSunIntensity(_)
            | LowpolyCommand::SetCamera(_) => true,
            LowpolyCommand::Extrude(_)
            | LowpolyCommand::Inset(_)
            | LowpolyCommand::Bevel(_)
            | LowpolyCommand::LoopCut(_)
            | LowpolyCommand::Subdivide(_)
            | LowpolyCommand::Triangulate(_)
            | LowpolyCommand::Decimate(_)
            | LowpolyCommand::Merge(_)
            | LowpolyCommand::Dissolve(_)
            | LowpolyCommand::Snap(_)
            | LowpolyCommand::ToggleSmooth(_)
            | LowpolyCommand::UnwrapActive(_)
            | LowpolyCommand::ClearSeam(_)
            | LowpolyCommand::TranslateSelection(_)
            | LowpolyCommand::RotateSelection(_)
            | LowpolyCommand::ScaleSelection(_)
            | LowpolyCommand::CanvasPointerUp(_) => true,
            LowpolyCommand::Mirror(payload) => payload.axis.as_deref().is_none_or(field),
            LowpolyCommand::FlipFaces(payload) => payload.face_ids.len() <= LOWPOLY_RETAINED_WORK_ITEMS,
            LowpolyCommand::MarkUvSeam(payload) => payload.edge_ids.as_ref().is_none_or(|ids| ids.len() <= LOWPOLY_RETAINED_WORK_ITEMS),
            LowpolyCommand::EngagementSubmit(payload) => payload.value.as_deref().is_none_or(field),
            LowpolyCommand::PaintFill(payload) => payload.object_id.as_deref().is_none_or(field),
            LowpolyCommand::FillBucket(payload) => payload.object_id.as_deref().is_none_or(field),
            LowpolyCommand::PaintStroke(payload) => payload.object_id.as_deref().is_none_or(field) && payload.phase.as_deref().is_none_or(field) && payload.reason.as_deref().is_none_or(field),
            LowpolyCommand::PaintAt(payload) => payload.object_id.as_deref().is_none_or(field) && payload.phase.as_deref().is_none_or(field) && payload.reason.as_deref().is_none_or(field),
            LowpolyCommand::CanvasPointerDown(payload) => payload.object_id.as_deref().is_none_or(field),
            LowpolyCommand::CanvasPointerMove(payload) => payload.object_id.as_deref().is_none_or(field) && payload.samples.as_ref().is_none_or(|samples| samples.len() <= LOWPOLY_RETAINED_WORK_ITEMS),
            LowpolyCommand::AddPrimitive(payload) => payload.kind.as_deref().is_none_or(field),
        }
}

fn lowpoly_sample_pixel(snapshot: &LowpolySnapshot, config: &LowpolyConfig, payload: &paint_sample::PaintSample) -> Emit<LowpolyMutation, LowpolyConfigMutation> {
    let Some((u, v)) = semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::session::paint_uv_from_command(payload.u, payload.v, payload.x, payload.y) else { return Emit::default() };
    let object_id = payload.object_id.clone().unwrap_or_else(|| resolve_active_object_id(snapshot, config));
    let Some(object) = snapshot.objects.iter().find(|object| object.id == object_id) else { return Emit::default() };
    let size = semio_s_artifact_lowpoly_lowpoly_model::snapshot::LOWPOLY_PAINT_TEXTURE_SIZE;
    let x = ((u.clamp(0.0, 1.0) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let y = (((1.0 - v.clamp(0.0, 1.0)) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let offset = (y * size + x) * 4;
    let mut color = [0_u8; 4];
    for layer in object.paint_layers.iter().filter(|layer| layer.visible) {
        // 🎨️ A sparse (never painted) layer IS opaque white everywhere; only a buffer too short for the
        // sample point is skipped.
        let source = if layer.pixels.is_empty() {
            [255_u8; 4]
        } else if offset.saturating_add(4) > layer.pixels.len() {
            continue;
        } else {
            [layer.pixels[offset], layer.pixels[offset + 1], layer.pixels[offset + 2], layer.pixels[offset + 3]]
        };
        let source_alpha = (source[3] as f32 / 255.0) * layer.opacity.clamp(0.0, 1.0);
        let destination_alpha = color[3] as f32 / 255.0;
        let alpha = source_alpha + destination_alpha * (1.0 - source_alpha);
        if alpha < 1e-6 {
            continue;
        }
        for channel in 0..3 {
            let source_channel = source[channel] as f32 / 255.0;
            let destination_channel = color[channel] as f32 / 255.0;
            color[channel] = ((source_channel * source_alpha + destination_channel * destination_alpha * (1.0 - source_alpha)) / alpha * 255.0).round().clamp(0.0, 255.0) as u8;
        }
        color[3] = (alpha * 255.0).round().clamp(0.0, 255.0) as u8;
    }
    Emit::config(vec![LowpolyConfigMutation::SetPaintColor(SetPaintColorEdit { r: color[0], g: color[1], b: color[2], a: color[3] })])
}

fn lowpoly_retained_reduce(
    command: &LowpolyCommand,
    snapshot: &LowpolySnapshot,
    config: &LowpolyConfig,
    history: &HistoryView,
    interaction: &protocol::InteractionState,
    context: &ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>,
    operation: &AppOperationContext,
) -> Result<ArtifactCommandWorkStep<EditorApp<LowpolyPlayApp>>, Fault> {
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: None };
    // 🕹️ The mesh domain's live selection for THIS dispatch, read straight from the retained job's own
    // scheduler-owned `InteractionState` — `selection_from_state`'s own doc comment names this exact
    // seam ("typed retained reducers read the same immutable domain selection directly from their
    // scheduler-owned request context, without manufacturing a host-only InteractionView").
    let empty_domain_selection = protocol::DomainSelection::default();
    let domain_selection = interaction.selection.get(MESH_INTERACTION_DOMAIN).unwrap_or(&empty_domain_selection);
    let active_object_id = semio_s_artifact_lowpoly_lowpoly_view::active_object_for_selection(snapshot, config, domain_selection);
    let selection = selection_from_state(&active_object_id, domain_selection);
    let selection_object_id = semio_s_artifact_lowpoly_lowpoly_view::selection_object_id(snapshot, domain_selection);
    // 🗿️ The whole objects the selection names (object-granularity rows), for delete/duplicate.
    let selected_object_ids: Vec<String> = domain_selection.ids.iter().filter_map(|raw| semio_s_artifact_lowpoly_lowpoly_view::parse_mesh_target_id(raw)).filter(|(_, component)| component.is_none()).map(|(object_id, _)| object_id).collect();
    // 🕸️ Commands whose handler reaches `session::build_doc`/`mesh_edit`, the mid-drag paint stroke
    // scratch, or the mid-drag gumball transform scratch cannot use a blank `LowpolyScratch::default()`
    // — `LowpolyDocument::reload_meshes` (`⚙️engine/🦀️.rs`) rejects every mesh edit past the
    // very first with `StaleMeshWorkspace` unless scratch is rehydrated from the LIVE persisted
    // `LowpolyTransient`. This macro does that rehydration, runs the handler, then publishes the
    // post-handle scratch back as the new transient root (`ArtifactToolPublicationLane::Transient`) —
    // see `LowpolyCommandDisposition::ArtifactTransient`/`ConfigTransient`'s doc comments.
    macro_rules! threaded {
        ($handle:expr) => {{
            let mut threaded = LowpolyScratch::from_transient(&context.transient, selection.clone());
            threaded.set_selection_object_id(selection_object_id.clone());
            threaded.set_selected_object_ids(selected_object_ids.clone());
            let step_emit = ($handle)(&doc, &cfg, &mut threaded)?;
            let transient = threaded.transient_snapshot();
            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit: step_emit, ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![LowpolyTransientMutation::Snapshot { transient }], window_transient: Vec::new() } });
        }};
    }
    // 🧲️ The gumball reads the live mesh cache to resolve its selection and publishes nothing but its transaction.
    macro_rules! read_threaded {
        ($handle:expr) => {{
            let mut threaded = LowpolyScratch::from_transient(&context.transient, selection.clone());
            threaded.set_selection_object_id(selection_object_id.clone());
            threaded.set_selected_object_ids(selected_object_ids.clone());
            return Ok(ArtifactCommandWorkStep::Complete(($handle)(&doc, &cfg, &mut threaded)?));
        }};
    }
    let mut bounded = LowpolyScratch::default();
    bounded.set_selection_object_id(selection_object_id.clone());
    let emit = match command {
        LowpolyCommand::PatchObject(payload) => patch_object::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::AddPaintLayer(payload) => add_paint_layer::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::SetActiveObject(payload) => set_active_object::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::SetActivePaintLayer(payload) => set_active_paint_layer::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::SetUtilityParam(payload) => set_utility_param::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::EngagementInput(payload) => engagement_input::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::ToggleShowEdges(payload) => toggle_show_edges::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::ToggleSun(payload) => toggle_sun::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::SetSunAzimuth(payload) => set_sun_azimuth::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::SetSunElevation(payload) => set_sun_elevation::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::SetSunIntensity(payload) => set_sun_intensity::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::SetCamera(payload) => set_camera::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::ImportSnapshotJson(payload) => set_snapshot_json::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::ReplaceSnapshotJson(payload) => replace_snapshot_json::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::ExportMesh(payload) => export_mesh::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::LoadMeshRequest(payload) => load_mesh_request::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::ImportMeshFile(payload) => import_mesh_file::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::PaintSample(payload) => return Ok(ArtifactCommandWorkStep::Complete(lowpoly_sample_pixel(snapshot, config, payload))),
        LowpolyCommand::Extrude(payload) => threaded!(|doc, cfg, ctx| extrude::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Inset(payload) => threaded!(|doc, cfg, ctx| inset::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Bevel(payload) => threaded!(|doc, cfg, ctx| bevel::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::LoopCut(payload) => threaded!(|doc, cfg, ctx| loop_cut::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Subdivide(payload) => threaded!(|doc, cfg, ctx| subdivide::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Triangulate(payload) => threaded!(|doc, cfg, ctx| triangulate::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Mirror(payload) => threaded!(|doc, cfg, ctx| mirror::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Decimate(payload) => threaded!(|doc, cfg, ctx| decimate::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::FlipFaces(payload) => threaded!(|doc, cfg, ctx| flip_faces::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Merge(payload) => threaded!(|doc, cfg, ctx| merge::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Dissolve(payload) => threaded!(|doc, cfg, ctx| dissolve::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Snap(payload) => threaded!(|doc, cfg, ctx| snap::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::ToggleSmooth(payload) => threaded!(|doc, cfg, ctx| toggle_smooth::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::UnwrapActive(payload) => threaded!(|doc, cfg, ctx| unwrap_active::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::MarkUvSeam(payload) => threaded!(|doc, cfg, ctx| mark_uv_seam::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::ClearSeam(payload) => threaded!(|doc, cfg, ctx| clear_seam::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::EngagementSubmit(payload) => threaded!(|doc, cfg, ctx| engagement_submit::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::TranslateSelection(payload) => read_threaded!(|doc, cfg, ctx| translate_selection::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::RotateSelection(payload) => read_threaded!(|doc, cfg, ctx| rotate_selection::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::ScaleSelection(payload) => read_threaded!(|doc, cfg, ctx| scale_selection::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::PaintStroke(payload) => return paint::lowpoly_paint_step(payload.phase.as_deref(), payload.reason.as_deref(), payload.object_id.as_deref(), &payload.points(), true, snapshot, config, context, operation),
        LowpolyCommand::PaintAt(payload) => return paint::lowpoly_paint_step(payload.phase.as_deref(), payload.reason.as_deref(), payload.object_id.as_deref(), &payload.points(), true, snapshot, config, context, operation),
        LowpolyCommand::CanvasPointerDown(payload) => return paint::lowpoly_paint_step(Some("stream"), None, payload.object_id.as_deref(), &payload.points(), true, snapshot, config, context, operation),
        LowpolyCommand::CanvasPointerMove(payload) => return paint::lowpoly_paint_step(Some("stream"), None, payload.object_id.as_deref(), &payload.points(), false, snapshot, config, context, operation),
        LowpolyCommand::CanvasPointerUp(payload) => {
            let (phase, reason) = payload.phase();
            return paint::lowpoly_paint_step(Some(phase), reason, None, &[], false, snapshot, config, context, operation);
        }
        // 🌱️ `add_primitive::handle` reaches `session::build_doc` (to read the live mesh state) AND
        // calls `ctx.set_mesh_workspace_map` (to record the new primitive's mesh) — both a read and a
        // write of the session-local cache, exactly the shape `ArtifactTransient` commands above are
        // threaded for. A blank `LowpolyScratch::default()` here made `build_doc` return `None` (hence
        // a silent no-op) for every `addPrimitive` after the first mesh edit, because
        // `LowpolyDocument::reload_meshes` rejects a `mesh_workspace` cache that doesn't cover every
        // object the persisted snapshot already has. The handler's own `Emit` still carries both the
        // `CreateObject` artifact mutation and the `SetActiveObject` config mutation; `threaded!` adds
        // the `Transient` republication `PUBLICATION_CONTRACTS`'s `["addPrimitive", lanes: &[Artifact,
        // Config, Transient]]` entry now admits.
        LowpolyCommand::AddPrimitive(payload) => threaded!(|doc, cfg, ctx| add_primitive::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::DeleteSelection(payload) => threaded!(|doc, cfg, ctx| delete_selection::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::DuplicateObject(payload) => threaded!(|doc, cfg, ctx| duplicate_object::handle(payload, doc, cfg, ctx)),
        // 🪣️ A one-shot fill reads only the committed layer, so it needs no transient rehydration.
        LowpolyCommand::PaintFill(payload) => paint_fill::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::FillBucket(payload) => fill_bucket::handle(payload, &doc, &cfg, &mut bounded),
    }?;
    Ok(ArtifactCommandWorkStep::Complete(emit))
}

fn lowpoly_tool_identity(tool_id: &str) -> u64 {
    tool_id.bytes().fold(0xcbf2_9ce4_8422_2325, |digest, byte| (digest ^ u64::from(byte)).wrapping_mul(0x1000_0000_01b3))
}

pub(super) struct LowpolyRetainedCommandWork {
    pub(super) tool_id: &'static str,
    pub(super) disposition: LowpolyCommandDisposition,
    pub(super) operation_id: u64,
    pub(super) generation: u64,
    pub(super) base_revision: [u8; 32],
    context_identity: u64,
    stage: u8,
    replay_target: Option<u8>,
    complete: bool,
    closing: bool,
}

impl LowpolyRetainedCommandWork {
    pub(super) fn new(tool_id: &'static str, disposition: LowpolyCommandDisposition, operation_id: u64, generation: u64, base_revision: [u8; 32], context_identity: u64) -> Self {
        Self { tool_id, disposition, operation_id, generation, base_revision, context_identity, stage: 0, replay_target: None, complete: false, closing: false }
    }
}

impl ArtifactCommandWork<EditorApp<LowpolyPlayApp>> for LowpolyRetainedCommandWork {
    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<LowpolyPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn workspace_identity(&self) -> u64 {
        lowpoly_tool_identity(self.tool_id) ^ self.operation_id.rotate_left(17) ^ self.generation.rotate_left(31) ^ self.context_identity.rotate_left(43) ^ (u64::from(self.disposition as u8) << 56)
    }

    fn extent(&self, command: &LowpolyCommand, _snapshot: &LowpolySnapshot, _interaction: &protocol::InteractionState, _context: Option<&ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>>) -> Option<usize> {
        (lowpoly_command_disposition(command.command_id()).is_some()).then_some(2)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<LowpolyPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<LowpolyPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config, history, interaction, hover: _hover, context, operation } = *input;
        if self.complete {
            return Err(Fault::from("lowpoly-retained-work-repeated"));
        }
        if self.closing {
            return Err(Fault::from("lowpoly-retained-work-closing"));
        }
        let context = context.ok_or_else(|| Fault::from("lowpoly-retained-context-absent"))?;
        if operation.operation_id != self.operation_id || operation.generation != self.generation || operation.canonical_base_revision != self.base_revision {
            return Err(Fault::from("lowpoly-retained-operation-freshness-drift"));
        }
        if context.identity_digest() != self.context_identity {
            return Err(Fault::from("lowpoly-retained-context-freshness-drift"));
        }
        if !lowpoly_command_admitted(command, snapshot, config) {
            return Err(Fault::from("lowpoly-retained-command-capacity"));
        }
        if self.stage == 0 {
            self.stage = 1;
            if let Some(target) = self.replay_target {
                if self.stage == target {
                    self.replay_target = None;
                }
                return Ok(ArtifactCommandWorkStep::Replay { stage: "lowpoly-command-workspace-replay", preview: b"{\"en\":\"Restoring Lowpoly workspace\",\"de\":\"Lowpoly-Arbeitsbereich wird wiederhergestellt\"}" });
            }
            return Ok(ArtifactCommandWorkStep::Progress { stage: "lowpoly-command-scan", preview: b"{\"en\":\"Preparing Lowpoly command\",\"de\":\"Lowpoly-Befehl wird vorbereitet\"}" });
        }
        let step = lowpoly_retained_reduce(command, snapshot, config, history, interaction, context, operation)?;
        self.complete = true;
        Ok(step)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn checkpoint_byte(&self, index: usize) -> Option<u8> {
        match index {
            0..=3 => Some(b"LPC3"[index]),
            4 => Some(self.disposition as u8),
            5 => Some(u8::from(self.complete)),
            6 => Some(self.stage),
            7 => Some(0),
            8..=15 => Some(lowpoly_tool_identity(self.tool_id).to_le_bytes()[index - 8]),
            16..=23 => Some(self.operation_id.to_le_bytes()[index - 16]),
            24..=31 => Some(self.generation.to_le_bytes()[index - 24]),
            32..=63 => Some(self.base_revision[index - 32]),
            64..=71 => Some(self.context_identity.to_le_bytes()[index - 64]),
            _ => None,
        }
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 72 || &checkpoint[..4] != b"LPC3" || checkpoint[4] != self.disposition as u8 || checkpoint[5] > 1 || checkpoint[6] > 1 || checkpoint[7] != 0 {
            return Err(Fault::from("lowpoly-retained-checkpoint-invalid"));
        }
        let tool = u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-tool"))?);
        let operation_id = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-operation"))?);
        let generation = u64::from_le_bytes(checkpoint[24..32].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-generation"))?);
        let context_identity = u64::from_le_bytes(checkpoint[64..72].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-context"))?);
        if tool != lowpoly_tool_identity(self.tool_id) || operation_id != self.operation_id || generation != self.generation || checkpoint[32..64] != self.base_revision || context_identity != self.context_identity {
            return Err(Fault::from("lowpoly-retained-checkpoint-identity-mismatch"));
        }
        self.stage = 0;
        self.replay_target = (checkpoint[6] != 0).then_some(checkpoint[6]);
        self.complete = checkpoint[5] == 1;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, _grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        if self.closing { InteractiveJobCloseStep::Complete { progress: Default::default() } } else { InteractiveJobCloseStep::Blocked }
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.closing.then_some(size_of::<Self>())
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}

pub(super) struct LowpolyCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl LowpolyCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: LOWPOLY_MIGRATED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for LowpolyCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<LowpolyPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<LowpolyPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        LOWPOLY_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        lowpoly_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > LOWPOLY_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Lowpoly retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl ArtifactOwnedToolJobFactory for LowpolyCommandJobFactory {
    type Owner = EditorApp<LowpolyPlayApp>;
    const TOOL_IDS: &'static [&'static str] = LOWPOLY_MIGRATED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = LOWPOLY_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchObject", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addPaintLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveObject", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActivePaintLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setUtilityParam", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleShowEdges", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importSnapshotJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "replaceSnapshotJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "exportMesh", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "loadMeshRequest", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importMeshFile", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract {
            tool_id: "duplicateObject",
            lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient],
        },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "paintSample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "extrude", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "inset", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "bevel", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "loopCut", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "subdivide", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "triangulate", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "mirror", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "decimate", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "flipFaces", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "merge", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "dissolve", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "snap", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleSmooth", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "unwrapActive", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "markUvSeam", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "clearSeam", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract {
            tool_id: "paintStroke",
            lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient],
        },
        semio_framework_plugin::ArtifactToolPublicationContract {
            tool_id: "paintAt",
            lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient],
        },
        semio_framework_plugin::ArtifactToolPublicationContract {
            tool_id: "canvasPointerDown",
            lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient],
        },
        semio_framework_plugin::ArtifactToolPublicationContract {
            tool_id: "canvasPointerMove",
            lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient],
        },
        semio_framework_plugin::ArtifactToolPublicationContract {
            tool_id: "canvasPointerUp",
            lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient],
        },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "paintFill", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "fillBucket", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract {
            tool_id: "addPrimitive",
            lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient],
        },
    ];
}
//#endregion 🧵️RetainedCommands
