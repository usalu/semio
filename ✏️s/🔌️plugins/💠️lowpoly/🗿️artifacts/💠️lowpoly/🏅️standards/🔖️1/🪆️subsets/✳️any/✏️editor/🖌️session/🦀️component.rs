//! 🖌️ Lowpoly play app — the `app_commands!` dispatch context (`LowpolyScratch`): the mid-drag paint
//! stroke scratch, gumball transform-drag scratch, paint texture cache and preview sequence counter.
//! These are genuine mid-gesture scratch buffers, never document or config state — the "scratch +
//! commit" pattern the `ArtifactApp` trait itself sanctions. Held behind one `RefCell<LowpolyScratch>`
//! on `LowpolyPlayApp` (mirrors `flow`'s `Mutex<FlowEvalSession>` pattern) so `render(&self, ..)` can
//! still read texture/transform preview state while `handle(&self, ..)` locks it mutably for dispatch.

use crate::editor::lowpoly::config::LowpolyConfig;
use crate::editor::lowpoly::engine::LowpolyDocument;
use crate::editor::lowpoly::view::build_doc;
use crate::artifacts::lowpoly::op::{LowpolyMutation, PixelRun};
use crate::artifacts::lowpoly::schema::{composite_layer_pixels, flood_fill, pixel_runs_from_diff, sample_pixel_from, stamp_brush};
use crate::artifacts::lowpoly::{empty_paint_pixels, LowpolyObject, LowpolyObjectPatch, LowpolySelection, LowpolySnapshot, LOWPOLY_PAINT_TEXTURE_SIZE};
use base64::Engine;
use semio_framework_3d::mesh::Vec3;
use semio_framework_plugin::Emit;
use std::collections::HashMap;

//#region 🔖️Sessions
/// @emoji 🖌️ In-progress paint drag: the pre-stroke layer buffer and the accumulating scratch buffer.
/// Mid-drag ticks mutate `scratch` (view state); the stroke commits as ONE `PaintStroke` operation on end.
pub struct PaintStrokeSession {
    object_id: String,
    layer_index: usize,
    base: Vec<u8>,
    scratch: Vec<u8>,
}

/// @emoji 🧲️ In-progress gumball transform drag. The mesh-transform operation re-serializes the WHOLE
/// `mesh_workspace` buffer per apply, so a per-tick `amend` would `combined.extend` N full-mesh patches and
/// replay them all (O(N) retained megabyte-scale JSON + O(N²) replay). Instead every mid-drag tick
/// applies its delta to this scratch `LowpolyDocument` emitting ZERO operations, and the whole gesture
/// commits as ONE `Objects(Patch)` (base → final mesh) on drag end (`Emit::commit`, coalesce-key `None`).
pub struct TransformSession {
    object_id: String,
    before: LowpolyObject,
    /// 🕸️ The `mesh_workspace` content the drag-start compute session was built from — `before`
    /// (a `LowpolyObject`) no longer carries it (round 2 of this ticket's round-trip law fix), so it
    /// is snapshotted alongside `before` here for `commit_transform`'s before/after comparison.
    before_mesh_workspace: String,
    doc: LowpolyDocument,
}

/// @emoji 🗃️ Pure render-side cache of composited paint textures (base64 PNG per object), invalidated
/// by a fingerprint over the document's paint pixels + the live stroke dirty counter. Never serialized.
#[derive(Default)]
pub struct PaintTextureLut {
    fingerprint: Option<u64>,
    pub textures: HashMap<String, String>,
}
//#endregion 🔖️Sessions

//#region 🔖️Transform
#[derive(Clone, Copy)]
pub enum Transform {
    Translate(Vec3),
    Rotate { axis: Vec3, angle: f32 },
    Scale(Vec3),
}

/// 🧯️ `clippy::needless_pass_by_value` — takes `MeshKernelError` by value (not `&MeshKernelError`) on
/// purpose: every call site uses it directly as a `.map_err(map_kernel_err)` callback, and `map_err`'s
/// closure signature is `FnOnce(E) -> F`, which always hands the error by value.
#[allow(clippy::needless_pass_by_value)]
pub async fn map_kernel_err(error: semio_framework_3d::mesh::MeshKernelError) -> String {
    format!("{error:?}")
}

pub async fn apply_transform(doc: &mut LowpolyDocument, transform: Transform) -> Result<(), String> {
    let selection_mode = doc.selection().mode.clone();
    let pivot = doc.selection_transform_pivot().map_err(|e| e.to_string())?;
    let component_verts = match selection_mode.as_str() {
        "vertex" | "face" | "edge" => Some(doc.selection_vertex_ids().map_err(|e| e.to_string())?),
        _ => None,
    };
    let component = matches!(selection_mode.as_str(), "vertex" | "face" | "edge");
    let verts = if component {
        let verts = component_verts.ok_or_else(|| "no vertices".to_string())?;
        if verts.is_empty() {
            return Err("no component vertices in selection".into());
        }
        Some(verts)
    } else {
        None
    };
    let mesh = doc.active_mesh_mut().map_err(|e| e.to_string())?;
    match transform {
        Transform::Translate(delta) => match &verts {
            Some(verts) => mesh.move_vertices(verts, delta).map_err(map_kernel_err)?,
            None => mesh.translate(delta).map_err(map_kernel_err)?,
        },
        Transform::Rotate { axis, angle } => match &verts {
            Some(verts) => mesh.rotate_vertices(verts, axis, angle, pivot).map_err(map_kernel_err)?,
            None => mesh.rotate(axis, angle).map_err(map_kernel_err)?,
        },
        Transform::Scale(scale) => match &verts {
            Some(verts) => mesh.scale_vertices(verts, scale, pivot).map_err(map_kernel_err)?,
            None => mesh.scale(scale).map_err(map_kernel_err)?,
        },
    }
    doc.sync_meshes_to_snapshot().map_err(|e| e.to_string())
}

/// 🎯️ Extracts UV (0..1) from a paint command's fields — either direct `u`/`v` (world 3d picks) or
/// canvas `x`/`y` positions mapped through the paint-texture extent (UV canvas).
pub async fn paint_uv_from_command(u: Option<f32>, v: Option<f32>, x: Option<f32>, y: Option<f32>) -> Option<(f32, f32)> {
    if let (Some(u), Some(v)) = (u, v) {
        return Some((u, v));
    }
    let x = x?;
    let y = y?;
    let size = LOWPOLY_PAINT_TEXTURE_SIZE as f64;
    let u = ((x as f64 / size) + 0.5).clamp(0.0, 1.0);
    let v = (1.0 - ((y as f64 / size) + 0.5).clamp(0.0, 1.0)).clamp(0.0, 1.0);
    Some((u as f32, v as f32))
}

/// @emoji 🧮️ The changed-field patch turning `before` into `after` — an internal diff-fragment type
/// (never a mutation payload itself, per `📓️taxonomy.md`'s option-bag rule), consumed by
/// `semantic_mutation_for_patch` below to pick the one real semantic mutation kind a kernel edit or
/// gumball drag actually touched. The `mesh_workspace` content comparison lives OUTSIDE this patch
/// (see `semantic_mutation_for_patch`'s own `before_mesh_workspace`/`after_mesh_workspace` params) —
/// `LowpolyObject`/`LowpolyObjectPatch` carry no mesh content field at all (round 2 of this ticket's
/// round-trip law fix); only the `mesh` handle is comparable here.
pub async fn object_patch_diff(before: &LowpolyObject, after: &LowpolyObject) -> LowpolyObjectPatch {
    LowpolyObjectPatch {
        name: (before.name != after.name).then(|| after.name.clone()),
        smooth_shading: (before.smooth_shading != after.smooth_shading).then_some(after.smooth_shading),
        transform: (before.transform != after.transform).then(|| after.transform.clone()),
        mesh: (before.mesh != after.mesh).then(|| after.mesh.clone()),
    }
}

/// @emoji 🎯️ Maps an `object_patch_diff` result (plus the drag/edit's before/after `mesh_workspace`
/// session-cache content, no longer reachable through `patch` itself) to the one semantic
/// `LowpolyMutation` it represents — a kernel mesh edit or gumball drag changes exactly one facet per
/// commit (name XOR smooth-shading XOR one transform axis XOR mesh), never several at once, so the
/// first populated field wins. Transform sub-field priority (position, then rotation, then scale)
/// matches the gumball's own single-axis-per-drag gesture (`translate_selection`/`rotate_selection`/
/// `scale_selection` each mutate exactly one `LowpolyTransform` field via `apply_transform`).
pub async fn semantic_mutation_for_patch(id: String, before_transform: &crate::artifacts::lowpoly::LowpolyTransform, patch: &LowpolyObjectPatch, before_mesh_workspace: &str, after_mesh_workspace: &str) -> Option<LowpolyMutation> {
    if let Some(new_name) = &patch.name {
        return Some(LowpolyMutation::RenameObject(crate::artifacts::lowpoly::mutations::rename_object::mutation::RenameObject { id, new_name: new_name.clone() }));
    }
    if let Some(new_smooth_shading) = patch.smooth_shading {
        return Some(LowpolyMutation::ChangeObjectSmoothShading(crate::artifacts::lowpoly::mutations::change_object_smooth_shading::mutation::ChangeObjectSmoothShading { id, new_smooth_shading }));
    }
    if let Some(transform) = &patch.transform {
        if transform.position != before_transform.position {
            return Some(LowpolyMutation::MoveObject(crate::artifacts::lowpoly::mutations::move_object::mutation::MoveObject { id, new_position: transform.position }));
        }
        if transform.rotation != before_transform.rotation {
            return Some(LowpolyMutation::RotateObject(crate::artifacts::lowpoly::mutations::rotate_object::mutation::RotateObject { id, new_rotation: transform.rotation }));
        }
        if transform.scale != before_transform.scale {
            return Some(LowpolyMutation::ScaleObject(crate::artifacts::lowpoly::mutations::scale_object::mutation::ScaleObject { id, new_scale: transform.scale }));
        }
    }
    if before_mesh_workspace != after_mesh_workspace {
        if after_mesh_workspace.is_empty() {
            return Some(LowpolyMutation::DeleteMesh(crate::artifacts::lowpoly::mutations::delete_mesh::mutation::DeleteMesh { id }));
        }
        let handle = crate::artifacts::lowpoly::mesh_child_handle(&id, after_mesh_workspace);
        return Some(LowpolyMutation::CreateMesh(crate::artifacts::lowpoly::mutations::create_mesh::mutation::CreateMesh {
            id,
            child_id: handle.child_id,
            target: handle.target,
            mesh_workspace: after_mesh_workspace.to_string(),
        }));
    }
    None
}

async fn encode_rgba_png(pixels: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|error| error.to_string())?;
        writer.write_image_data(pixels).map_err(|error| error.to_string())?;
    }
    Ok(bytes)
}

async fn fnv1a_u64(mut hash: u64, bytes: &[u8]) -> u64 {
    let (chunks, remainder) = bytes.as_chunks::<8>();
    for chunk in chunks {
        let word = u64::from_le_bytes(*chunk);
        hash ^= word;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for &byte in remainder {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// @emoji 🔧️ Runs a kernel mesh edit against a compute session built from the projection + config,
/// then returns the resulting `Objects(Patch)` capturing only the changed object fields. Takes
/// `ctx: &mut LowpolyScratch` (round 2 of this ticket's round-trip law fix) — the compute session's
/// live `mesh_workspace` content now lives session-side, never on `LowpolyObject`, so building the
/// doc and reading back its post-edit content both need the cache.
pub async fn mesh_edit(projection: &LowpolySnapshot, config: &LowpolyConfig, ctx: &mut LowpolyScratch, edit: impl FnOnce(&mut LowpolyDocument) -> Result<(), String>) -> Emit<LowpolyMutation, crate::editor::lowpoly::config::LowpolyConfigMutation> {
    let Some(mut doc) = build_doc(projection, config, ctx) else {
        return Emit::default();
    };
    let object_id = doc.active_object_id().to_string();
    let Some(before) = projection.objects.iter().find(|object| object.id == object_id).cloned() else {
        return Emit::default();
    };
    let before_mesh_workspace = ctx.mesh_workspace(&object_id).to_string();
    if edit(&mut doc).is_err() {
        return Emit::default();
    }
    if doc.sync_meshes_to_snapshot().is_err() {
        return Emit::default();
    }
    ctx.set_mesh_workspace_map(doc.mesh_workspace().clone());
    let Some(after) = doc.snapshot().objects.iter().find(|object| object.id == object_id).cloned() else {
        return Emit::default();
    };
    let after_mesh_workspace = ctx.mesh_workspace(&object_id).to_string();
    let patch = object_patch_diff(&before, &after);
    match semantic_mutation_for_patch(object_id, &before.transform, &patch, &before_mesh_workspace, &after_mesh_workspace) {
        Some(mutation) => Emit::mutations(vec![mutation]),
        None => Emit::default(),
    }
}
//#endregion 🔖️Transform

//#region 🔖️LowpolyScratch
/// @emoji 🖌️ B1: `LowpolyPlayApp` sheds `RefCell<LowpolyPlayRuntime>` entirely — every former runtime
/// field now lives in `LowpolyConfig`, written through `LowpolyConfigMutation`s emitted from `handle`.
/// This struct holds the genuine mid-gesture scratch state the `ArtifactApp` trait sanctions, PLUS
/// (round 2 of ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM's round-trip law fix) the
/// session-local `mesh_workspace` cache — live half-edge-mesh JSON per object id, formerly a field on
/// `LowpolyObject` itself, moved here because a codec-excluded field cannot legitimately live on a
/// persisted snapshot type (`store::os_store::test_support::assert_document_text_round_trip` is a
/// general law every `ArtifactDsl + ArtifactPack` snapshot type must satisfy). Exactly the pattern
/// `draw`'s `DrawSession` and DKM's `EngineRep` already establish elsewhere in this ticket: an
/// ephemeral value threaded alongside the persisted view, never embedded in the persisted snapshot.
pub struct LowpolyScratch {
    stroke: Option<PaintStrokeSession>,
    stroke_drag_active: bool,
    stroke_dirty: u64,
    transform: Option<TransformSession>,
    transform_drag_active: bool,
    texture_cache: PaintTextureLut,
    /// 👻️ Per-`key` monotone counter for `gesture_preview` — see `//#region 🔖️GesturePreview`.
    preview_seq: u64,
    /// 🕸️ Live half-edge-mesh JSON per object id — see this struct's own doc comment. Seeded from
    /// `crate::artifacts::lowpoly::schema::default_mesh_workspace()` on `Default::default()` so a
    /// freshly booted session can immediately reload the mesh `ArtifactApp::initial_snapshot()`
    /// (`default_snapshot()`) describes; real interactive objects get their own entry from
    /// `mesh_edit`/`LowpolyDocument::add_primitive` as they are created/edited. NEVER the persisted
    /// document representation, NEVER round-tripped through undo/redo (store-level undo/redo bypass
    /// `ArtifactApp::handle` entirely, so this cache can go stale relative to the document's `mesh`
    /// handle across an undo/redo of a `create-mesh`/`delete-mesh` — `LowpolyDocument::reload_meshes`
    /// detects that staleness and fails closed rather than silently computing wrong geometry; a real
    /// fix needs child-document resolution, which no WASM-guest plugin in this repo has yet).
    mesh_workspace: HashMap<String, String>,
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the mesh domain's CURRENT selection,
    /// resolved from `InteractionView` by `LowpolyPlayApp::handle` right before it delegates to
    /// `LowpolyCommand::dispatch` — the `app_commands!`-generated dispatcher calls every leaf command's
    /// `handle(payload, doc, cfg, ctx)` uniformly, with no `interaction` parameter of its own, so this
    /// scratch field is the one channel by which those handlers (via `build_doc`/`mesh_edit`) see the
    /// framework-owned selection without every one of them threading a fifth argument. Never persisted,
    /// never read outside the dispatch that set it.
    current_selection: LowpolySelection,
}

impl Default for LowpolyScratch {
    fn default() -> Self {
        Self {
            stroke: None,
            stroke_drag_active: false,
            stroke_dirty: 0,
            transform: None,
            transform_drag_active: false,
            texture_cache: PaintTextureLut::default(),
            preview_seq: 0,
            mesh_workspace: crate::artifacts::lowpoly::schema::default_mesh_workspace(),
            current_selection: LowpolySelection::default(),
        }
    }
}

impl LowpolyScratch {
    /// 🕹️ Sets THIS dispatch's mesh-domain selection — see `current_selection`'s own doc comment.
    pub async fn set_current_selection(&mut self, selection: LowpolySelection) {
        self.current_selection = selection;
    }

    /// 🕹️ THIS dispatch's mesh-domain selection (or the default/empty one outside a command dispatch,
    /// e.g. `render`).
    pub async fn current_selection(&self) -> &LowpolySelection {
        &self.current_selection
    }

    /// 🕸️ The live half-edge-mesh JSON cached for `object_id`, or `""` when this session has no
    /// working content for it yet (e.g. an object loaded from a real document import, pending child-
    /// document resolution — see this struct's own doc comment).
    pub async fn mesh_workspace(&self, object_id: &str) -> &str {
        self.mesh_workspace.get(object_id).map(String::as_str).unwrap_or_default()
    }

    /// 🕸️ A clone of the full session-local mesh-workspace cache — `LowpolyDocument::with_context`'s
    /// input, since it needs one entry per object to reload every mesh, not just the active one.
    pub async fn mesh_workspace_map(&self) -> HashMap<String, String> {
        self.mesh_workspace.clone()
    }

    /// 🕸️ Replaces the whole session-local mesh-workspace cache — called after a successful edit with
    /// `LowpolyDocument::mesh_workspace()`'s post-`sync_meshes_to_snapshot` content.
    pub async fn set_mesh_workspace_map(&mut self, map: HashMap<String, String>) {
        self.mesh_workspace = map;
    }

    pub async fn stroke_drag_active(&self) -> bool {
        self.stroke_drag_active
    }

    pub async fn set_stroke_drag_active(&mut self, value: bool) {
        self.stroke_drag_active = value;
    }

    pub async fn set_transform_drag_active(&mut self, value: bool) {
        self.transform_drag_active = value;
    }

    pub async fn transform_projection(&self) -> Option<LowpolySnapshot> {
        self.transform.as_ref().map(|session| session.doc.snapshot().clone())
    }

    /// 🧰️ Clears every mid-gesture scratch — used by `SetActiveUtility` so switching tools never leaves
    /// a stale paint/transform drag behind.
    pub async fn reset_gestures(&mut self) {
        self.stroke = None;
        self.stroke_drag_active = false;
        self.transform = None;
        self.transform_drag_active = false;
    }

    /// ▶️ `paintStrokeBegin`: arms the drag flag and clears any stale scratch from a previous gesture.
    pub async fn begin_stroke_drag(&mut self) {
        self.stroke_drag_active = true;
        self.stroke = None;
    }

    /// ⏹️ `paintStrokeEnd`: disarms the drag flag and commits the accumulated scratch as one edit.
    pub async fn end_stroke_drag(&mut self) -> Emit<LowpolyMutation, crate::editor::lowpoly::config::LowpolyConfigMutation> {
        self.stroke_drag_active = false;
        self.commit_stroke()
    }

    /// ▶️ `transformBegin`: arms the drag flag and clears any stale scratch from a previous gesture.
    pub async fn begin_transform_drag(&mut self) {
        self.transform_drag_active = true;
        self.transform = None;
    }

    /// ⏹️ `transformEnd`: disarms the drag flag and commits the accumulated scratch as one edit.
    pub async fn end_transform_drag(&mut self) -> Emit<LowpolyMutation, crate::editor::lowpoly::config::LowpolyConfigMutation> {
        self.transform_drag_active = false;
        self.commit_transform()
    }

    /// @emoji 🖼️ The layers to composite for `object`, overlaying the live stroke scratch when the drag
    /// targets that object so the in-progress stroke previews before it commits.
    async fn composite_layers_for(&self, object: &LowpolyObject) -> Vec<u8> {
        if let Some(session) = &self.stroke {
            if session.object_id == object.id {
                let mut layers = object.paint_layers.clone();
                if let Some(layer) = layers.get_mut(session.layer_index) {
                    layer.pixels = session.scratch.clone();
                }
                return composite_layer_pixels(&layers);
            }
        }
        composite_layer_pixels(&object.paint_layers)
    }

    async fn paint_fingerprint(&self, projection: &LowpolySnapshot) -> u64 {
        let mut hash = 0xcbf29ce484222325u64;
        for object in &projection.objects {
            hash = fnv1a_u64(hash, object.id.as_bytes());
            for layer in &object.paint_layers {
                hash = fnv1a_u64(hash, &[layer.visible as u8]);
                hash = fnv1a_u64(hash, &layer.opacity.to_le_bytes());
                hash = fnv1a_u64(hash, &layer.pixels);
            }
        }
        fnv1a_u64(hash, &self.stroke_dirty.to_le_bytes())
    }

    pub async fn refresh_texture_cache(&mut self, projection: &LowpolySnapshot) {
        let fingerprint = self.paint_fingerprint(projection);
        if self.texture_cache.fingerprint == Some(fingerprint) {
            return;
        }
        let mut textures = HashMap::new();
        for object in &projection.objects {
            let composite = self.composite_layers_for(object);
            if let Ok(png_bytes) = encode_rgba_png(&composite, LOWPOLY_PAINT_TEXTURE_SIZE as u32, LOWPOLY_PAINT_TEXTURE_SIZE as u32) {
                textures.insert(object.id.clone(), base64::engine::general_purpose::STANDARD.encode(png_bytes));
            }
        }
        self.texture_cache = PaintTextureLut { fingerprint: Some(fingerprint), textures };
    }

    pub async fn textures(&self) -> &HashMap<String, String> {
        &self.texture_cache.textures
    }

    /// @emoji 📌️ Commits the accumulated paint scratch as ONE described `PaintStroke` edit (scratch-commit
    /// pattern b — the whole drag is one undoable edit; megabyte pixel buffers never coalesce per tick).
    pub async fn commit_stroke(&mut self) -> Emit<LowpolyMutation, crate::editor::lowpoly::config::LowpolyConfigMutation> {
        let Some(session) = self.stroke.take() else {
            return Emit::default();
        };
        self.stroke_dirty += 1;
        let runs: Vec<PixelRun> = pixel_runs_from_diff(&session.base, &session.scratch).into_iter().map(|(offset, bytes)| PixelRun { offset, bytes }).collect();
        if runs.is_empty() {
            return Emit::default();
        }
        Emit::commit(vec![LowpolyMutation::EditPaintLayer(crate::artifacts::lowpoly::mutations::edit_paint_layer::mutation::EditPaintLayer { object_id: session.object_id, layer_index: session.layer_index, runs })], "Paint stroke")
    }

    /// @emoji 🖌️ One mid-drag paint tick: brush/eraser/fill mutate the stroke scratch, eyedropper samples
    /// the paint color (as a `SetPaintColor` config op). Emits ZERO document operations — the stroke
    /// commits only on `paintStrokeEnd` (View-kind safe).
    pub async fn paint_tick(&mut self, projection: &LowpolySnapshot, config: &LowpolyConfig, object_id: &str, u: f32, v: f32) -> Emit<LowpolyMutation, crate::editor::lowpoly::config::LowpolyConfigMutation> {
        let utility = config.paint_utility.clone();
        if utility == "eyedropper" {
            let Some(object) = projection.objects.iter().find(|object| object.id == object_id) else {
                return Emit::default();
            };
            let composite = composite_layer_pixels(&object.paint_layers);
            let color = sample_pixel_from(&composite, u, v);
            return Emit::config(vec![crate::editor::lowpoly::config::LowpolyConfigMutation::SetPaintColor { r: color[0], g: color[1], b: color[2], a: color[3] }]);
        }
        let layer_index = config.active_paint_layer as usize;
        let need_new = match &self.stroke {
            Some(session) => session.object_id != object_id || session.layer_index != layer_index,
            None => true,
        };
        if need_new {
            let base = projection.objects.iter().find(|object| object.id == object_id).and_then(|object| object.paint_layers.get(layer_index)).map_or_else(empty_paint_pixels, |layer| layer.pixels.clone());
            self.stroke = Some(PaintStrokeSession { object_id: object_id.to_string(), layer_index, scratch: base.clone(), base });
        }
        let color = [config.paint_color_r, config.paint_color_g, config.paint_color_b, config.paint_color_a];
        let params = crate::editor::lowpoly::view::utility_params_value(config);
        if let Some(session) = self.stroke.as_mut() {
            if utility == "fill" {
                flood_fill(&mut session.scratch, u, v, color);
            } else {
                let radius = crate::editor::lowpoly::view::utility_param_f32(&params, "brushSize", 16.0);
                let opacity = crate::editor::lowpoly::view::utility_param_f32(&params, "brushOpacity", 1.0);
                let hardness = crate::editor::lowpoly::view::utility_param_f32(&params, "brushHardness", 0.5);
                stamp_brush(&mut session.scratch, u, v, radius, color, hardness, opacity, utility == "eraser");
            }
        }
        self.stroke_dirty += 1;
        Emit::default()
    }

    /// @emoji 🪣️ A single-shot flood fill emitted as ONE `PaintStroke` edit (the `fillBucket`/`paintFill`
    /// operation path — not drag-bracketed, so it commits immediately).
    pub async fn fill_at(&mut self, projection: &LowpolySnapshot, config: &LowpolyConfig, object_id: String, u: f32, v: f32) -> Emit<LowpolyMutation, crate::editor::lowpoly::config::LowpolyConfigMutation> {
        let layer_index = config.active_paint_layer as usize;
        let color = [config.paint_color_r, config.paint_color_g, config.paint_color_b, config.paint_color_a];
        let Some(layer) = projection.objects.iter().find(|object| object.id == object_id).and_then(|object| object.paint_layers.get(layer_index)) else {
            return Emit::default();
        };
        let mut scratch = layer.pixels.clone();
        flood_fill(&mut scratch, u, v, color);
        let runs: Vec<PixelRun> = pixel_runs_from_diff(&layer.pixels, &scratch).into_iter().map(|(offset, bytes)| PixelRun { offset, bytes }).collect();
        if runs.is_empty() {
            return Emit::default();
        }
        self.stroke_dirty += 1;
        Emit::commit(vec![LowpolyMutation::EditPaintLayer(crate::artifacts::lowpoly::mutations::edit_paint_layer::mutation::EditPaintLayer { object_id, layer_index, runs })], "Fill")
    }

    /// @emoji 🧲️ Runs one gumball transform delta against a working scratch document. Mid-drag it emits
    /// nothing; only `transformEnd` (or an unbracketed single dispatch) commits the accumulated diff.
    pub async fn transform_selection(&mut self, projection: &LowpolySnapshot, config: &LowpolyConfig, mode: &str, ids: Vec<u32>, transform: Transform, description: &str) -> Emit<LowpolyMutation, crate::editor::lowpoly::config::LowpolyConfigMutation> {
        if self.transform_drag_active {
            if self.transform.is_none() {
                self.begin_transform_session(projection, config);
            }
            if let Some(session) = self.transform.as_mut() {
                if !ids.is_empty() {
                    session.doc.apply_selection(mode, ids);
                }
                let _ = apply_transform(&mut session.doc, transform);
            }
            self.preview_seq = self.preview_seq.wrapping_add(1);
            return Emit::default();
        }
        let emitted = mesh_edit(projection, config, self, move |doc| {
            if !ids.is_empty() {
                doc.apply_selection(mode, ids);
            }
            apply_transform(doc, transform)
        });
        if emitted.artifact_mutations.is_empty() {
            Emit::default()
        } else {
            Emit::commit(emitted.artifact_mutations, description)
        }
    }

    /// @emoji 🎬️ Snapshots the active object as the transform-drag base and builds the working scratch doc.
    async fn begin_transform_session(&mut self, projection: &LowpolySnapshot, config: &LowpolyConfig) {
        let Some(doc) = build_doc(projection, config, self) else {
            return;
        };
        let object_id = doc.active_object_id().to_string();
        let Some(before) = projection.objects.iter().find(|object| object.id == object_id).cloned() else {
            return;
        };
        let before_mesh_workspace = doc.mesh_workspace().get(&object_id).cloned().unwrap_or_default();
        self.transform = Some(TransformSession { object_id, before, before_mesh_workspace, doc });
    }

    /// @emoji 📌️ Commits the whole gumball drag as ONE `Objects(Patch)` diff (base → final mesh).
    pub async fn commit_transform(&mut self) -> Emit<LowpolyMutation, crate::editor::lowpoly::config::LowpolyConfigMutation> {
        let Some(mut session) = self.transform.take() else {
            return Emit::default();
        };
        if session.doc.sync_meshes_to_snapshot().is_err() {
            return Emit::default();
        }
        self.set_mesh_workspace_map(session.doc.mesh_workspace().clone());
        let Some(after) = session.doc.snapshot().objects.iter().find(|object| object.id == session.object_id).cloned() else {
            return Emit::default();
        };
        let after_mesh_workspace = self.mesh_workspace(&session.object_id).to_string();
        let patch = object_patch_diff(&session.before, &after);
        match semantic_mutation_for_patch(session.object_id, &session.before.transform, &patch, &session.before_mesh_workspace, &after_mesh_workspace) {
            Some(mutation) => Emit::commit(vec![mutation], "Transform selection"),
            None => Emit::default(),
        }
    }

    //#region 🔖️GesturePreview
    /// 👻️ CW7 db+protocol+vcs-slimming campaign, "preview law for gesture apps": the live gumball
    /// drag's current object state, expressed as a patch anchored to the drag-start snapshot
    /// (`session.before`) via the same `object_patch_diff` `commit_transform` uses for the eventual real
    /// commit. Anchoring to a fixed base (not the previous preview tick) keeps this correct even when
    /// the lossy, uncredited preview lane drops every message but the latest — a receiver only ever
    /// needs the last-synced canonical object (`before`, already has it) plus this one patch, never a
    /// chain of prior preview messages. `apply_transform` already calls `sync_meshes_to_snapshot`
    /// every tick (mid-drag world-scene rendering needs it regardless), so reading
    /// `session.doc.snapshot()` here adds no new per-tick cost. `None` outside an active drag; this
    /// reads `TransformSession` only, never emits or mutates a `LowpolyMutation`.
    ///
    /// 🚧️ Deliberately unwired beyond this accessor — same gap as `draw-plugin`'s
    /// `draw_gesture_preview_payload`: `framework/sync::SyncSession::publish_preview` is host-only
    /// ("WASI-P2 plugins never link this crate") and this crate compiles as a WASI-P2 component; the
    /// one cross-sandbox channel this crate can reach, `store::BackboneMessage`, has no preview-shaped
    /// variant. See `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/cw7-preview-law.txt`.
    /// `#[allow(dead_code)]`: exercised by `🧪️Tests` only until a host bridge exists.
    #[allow(dead_code)]
    pub async fn gesture_preview(&self) -> Option<(&'static str, u64, Vec<u8>)> {
        let session = self.transform.as_ref()?;
        let after = session.doc.snapshot().objects.iter().find(|object| object.id == session.object_id)?.clone();
        let patch = object_patch_diff(&session.before, &after);
        let payload = serde_json::json!({ "objectId": session.object_id.clone(), "patch": patch });
        Some(("gesture:transform", self.preview_seq, serde_json::to_vec(&payload).ok()?))
    }
    //#endregion 🔖️GesturePreview
}
//#endregion 🔖️LowpolyScratch

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::lowpoly::schema::default_snapshot;

    #[semio_framework_async_macros::async_test]
    async fn gesture_preview_is_none_without_an_active_transform_drag() {
        let scratch = LowpolyScratch::default();
        assert!(scratch.gesture_preview().is_none(), "no live gumball drag, nothing to preview");
    }

    #[semio_framework_async_macros::async_test]
    async fn gesture_preview_reflects_the_live_gumball_drag_and_clears_on_commit() {
        let mut scratch = LowpolyScratch::default();
        let projection = default_snapshot();
        let config = LowpolyConfig::default();
        scratch.set_transform_drag_active(true);

        let tick_a = scratch.transform_selection(&projection, &config, "mesh", vec![], Transform::Translate(Vec3::new(0.5, 0.0, 0.0)), "translate");
        assert!(tick_a.artifact_mutations.is_empty(), "mid-drag ticks emit zero operations (scratch-commit pattern)");
        let (key, seq_after_a, payload_a) = scratch.gesture_preview().expect("a live gumball drag is previewable");
        assert_eq!(key, "gesture:transform");
        let value_a: serde_json::Value = serde_json::from_slice(&payload_a).expect("payload is valid json");
        assert_eq!(value_a["objectId"], serde_json::json!(projection.objects[0].id));
        assert_ne!(value_a["patch"], serde_json::json!(LowpolyObjectPatch::default()), "the patch anchored to the drag-start snapshot must reflect the first tick");

        let tick_b = scratch.transform_selection(&projection, &config, "mesh", vec![], Transform::Translate(Vec3::new(0.25, 0.0, 0.0)), "translate");
        assert!(tick_b.artifact_mutations.is_empty());
        let (_, seq_after_b, payload_b) = scratch.gesture_preview().expect("still live mid-drag");
        assert!(seq_after_b > seq_after_a, "seq is monotone per tick, for staleness detection on the receiving end");
        assert_ne!(payload_a, payload_b, "the base-anchored patch accumulates both ticks, not just the latest one");

        let end = scratch.commit_transform();
        assert_eq!(end.artifact_mutations.len(), 1, "the whole drag commits as exactly one real operation");
        assert!(scratch.gesture_preview().is_none(), "the drag ended: nothing left to preview, and the commit above already carried the real operation");
    }

    #[semio_framework_async_macros::async_test]
    async fn gesture_preview_is_a_pure_read_never_mutating_the_transform_session() {
        let mut scratch = LowpolyScratch::default();
        let projection = default_snapshot();
        let config = LowpolyConfig::default();
        scratch.set_transform_drag_active(true);
        scratch.transform_selection(&projection, &config, "mesh", vec![], Transform::Translate(Vec3::new(1.0, 0.0, 0.0)), "translate");
        let object_id = scratch.transform.as_ref().unwrap().object_id.clone();
        let mesh_before = scratch.transform.as_ref().unwrap().doc.mesh_workspace().get(&object_id).cloned();
        let _ = scratch.gesture_preview();
        let _ = scratch.gesture_preview();
        assert_eq!(scratch.transform.as_ref().unwrap().doc.mesh_workspace().get(&object_id).cloned(), mesh_before, "gesture_preview must never mutate the live transform scratch it reads");
    }
}
//#endregion 🧪️Tests
