//! 🖌️ Lowpoly play app — the `app_commands!` dispatch context (`LowpolyScratch`: the session-local mesh workspace
//! cache, the dispatch's mesh-domain selection and the render-side paint texture cache) and the artifact-level
//! local-only transient (`LowpolyTransient`): that cache plus every window's open paint gesture — its tool statechart
//! configuration by stable ids, the admission and base revision it opened on, and its open `ToolTransaction`'s one
//! provisional leaf. The gesture is artifact-level rather than window-level so the SIBLING windows paint its preview
//! too (a stroke on the UV canvas shows on the model and back), keyed by the owning window id so two windows never
//! clobber each other; never history, never shared (design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, the FEM gumball's
//! documented deviation).
#![allow(unexpected_cfgs)]

use crate::editor::lowpoly::config::LowpolyConfig;
use crate::editor::lowpoly::engine::LowpolyDocument;
use crate::editor::lowpoly::view::try_build_doc;
use crate::op::LowpolyMutation;
use crate::schema::composite_layer_pixels;
use crate::{LowpolyObject, LowpolyObjectPatch, LowpolySelection, LowpolySnapshot, LOWPOLY_PAINT_TEXTURE_SIZE};
use machine::Command;
use protocol::Mutation;
use semio_framework_plugin::Emit;
use semio_framework_tool_machine::{ToolAbortReason, ToolMachineRunner, ToolRefusal, ToolStep, ToolTransaction, ToolTransactionState, ToolYield};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use store::ArtifactPack;

//#region 🔖️MeshEdit
/// 🧯️ `clippy::needless_pass_by_value` — takes `MeshKernelError` by value on purpose: every call site uses it as a
/// `.map_err(map_kernel_err)` callback, and `map_err`'s closure signature hands the error by value.
#[allow(clippy::needless_pass_by_value)]
pub fn map_kernel_err(error: semio_framework_3d::mesh::MeshKernelError) -> String {
    format!("{error:?}")
}

/// 🎯️ Extracts UV (0..1) from a paint command's fields — either direct `u`/`v` (world 3d picks) or canvas `x`/`y`
/// positions mapped through the paint-texture extent (UV canvas).
pub fn paint_uv_from_command(u: Option<f32>, v: Option<f32>, x: Option<f32>, y: Option<f32>) -> Option<(f32, f32)> {
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

/// 🧮️ The changed-field patch turning `before` into `after` — an internal diff-fragment type (never a mutation
/// payload itself), consumed by `semantic_mutation_for_patch` below to pick the one real semantic mutation kind a
/// kernel edit touched. The `mesh_workspace` content comparison lives OUTSIDE this patch (see
/// `semantic_mutation_for_patch`'s own `before_mesh_workspace`/`after_mesh_workspace` params).
pub fn object_patch_diff(before: &LowpolyObject, after: &LowpolyObject) -> LowpolyObjectPatch {
    LowpolyObjectPatch {
        name: (before.name != after.name).then(|| after.name.clone()),
        smooth_shading: (before.smooth_shading != after.smooth_shading).then_some(after.smooth_shading),
        transform: (before.transform != after.transform).then(|| after.transform.clone()),
        mesh: (before.mesh != after.mesh).then(|| after.mesh.clone()),
        mesh_content: (before.mesh_content != after.mesh_content).then(|| after.mesh_content.clone()),
    }
}

/// 🎯️ Maps an `object_patch_diff` result (plus the edit's before/after `mesh_workspace` content) to the one semantic
/// `LowpolyMutation` it represents — a kernel mesh edit changes exactly one facet per commit (name XOR smooth-shading
/// XOR one transform axis XOR mesh), so the first populated field wins.
pub fn semantic_mutation_for_patch(id: String, before_transform: &crate::LowpolyTransform, patch: &LowpolyObjectPatch, before_mesh_workspace: &str, after_mesh_workspace: &str) -> Option<LowpolyMutation> {
    if let Some(new_name) = &patch.name {
        return Some(LowpolyMutation::RenameObject(crate::mutations::rename_object::RenameObject { id, new_name: new_name.clone() }));
    }
    if let Some(new_smooth_shading) = patch.smooth_shading {
        return Some(LowpolyMutation::ChangeObjectSmoothShading(crate::mutations::change_object_smooth_shading::ChangeObjectSmoothShading { id, new_smooth_shading }));
    }
    if let Some(transform) = &patch.transform {
        if transform.position != before_transform.position {
            return Some(LowpolyMutation::MoveObject(crate::mutations::move_object::MoveObject { id, new_position: transform.position }));
        }
        if transform.rotation != before_transform.rotation {
            return Some(LowpolyMutation::RotateObject(crate::mutations::rotate_object::RotateObject { id, new_rotation: transform.rotation }));
        }
        if transform.scale != before_transform.scale {
            return Some(LowpolyMutation::ScaleObject(crate::mutations::scale_object::ScaleObject { id, new_scale: transform.scale }));
        }
    }
    if before_mesh_workspace != after_mesh_workspace {
        if after_mesh_workspace.is_empty() {
            return Some(LowpolyMutation::DeleteMesh(crate::mutations::delete_mesh::DeleteMesh { id }));
        }
        let handle = crate::mesh_child_handle(&id, after_mesh_workspace);
        return Some(LowpolyMutation::CreateMesh(crate::mutations::create_mesh::CreateMesh { id, child_id: handle.child_id, target: handle.target, mesh_workspace: after_mesh_workspace.to_string() }));
    }
    None
}

fn encode_rgba_png(pixels: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
    let image = semio_framework_pixels::RasterImage { width, height, pixels: pixels.to_vec() };
    semio_framework_pixels::encode_png(&image).map_err(|error| error.to_string())
}

fn fnv1a_u64(mut hash: u64, bytes: &[u8]) -> u64 {
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

/// 🔧️ Runs a kernel mesh edit against a compute session built from the projection + config, then returns the
/// resulting single semantic mutation. Takes `ctx` because the compute session's live `mesh_workspace` content for a
/// legacy handle-only object lives session-side; building the doc and reading back its post-edit content both need
/// the cache.
///
/// 🔊️ Every refusal is an `Err` naming its cause — a session that cannot be built, an edit the kernel rejects, an
/// active object the projection does not carry — so a command that does nothing says why. An edit that changes
/// nothing is still `Ok(Emit::default())`.
pub fn mesh_edit(projection: &LowpolySnapshot, config: &LowpolyConfig, ctx: &mut LowpolyScratch, edit: impl FnOnce(&mut LowpolyDocument) -> Result<(), String>) -> Result<Emit<LowpolyMutation, crate::editor::lowpoly::config::LowpolyConfigMutation>, String> {
    let mut doc = try_build_doc(projection, config, ctx).map_err(|error| format!("lowpoly mesh edit: compute session refused: {error}"))?;
    let object_id = doc.active_object_id().to_string();
    let before = projection.objects.iter().find(|object| object.id == object_id).cloned().ok_or_else(|| format!("lowpoly mesh edit: active object {object_id} is not in the document"))?;
    // 🕸️ The "before" geometry is the DOCUMENT's — `reload_meshes` just resolved every object from its persisted
    // `mesh_content`, so the compute session's cache is authoritative here and a stale transient is not.
    ctx.set_mesh_workspace_map(doc.mesh_workspace().clone());
    let before_mesh_workspace = ctx.mesh_workspace(&object_id).to_string();
    edit(&mut doc).map_err(|error| format!("lowpoly mesh edit on {object_id} (selection {:?}): {error}", doc.selection()))?;
    doc.sync_meshes_to_snapshot().map_err(|error| format!("lowpoly mesh edit: sync meshes: {error}"))?;
    ctx.set_mesh_workspace_map(doc.mesh_workspace().clone());
    let after = doc.snapshot().objects.iter().find(|object| object.id == object_id).cloned().ok_or_else(|| format!("lowpoly mesh edit: active object {object_id} vanished from the edited document"))?;
    let after_mesh_workspace = ctx.mesh_workspace(&object_id).to_string();
    let patch = object_patch_diff(&before, &after);
    Ok(match semantic_mutation_for_patch(object_id, &before.transform, &patch, &before_mesh_workspace, &after_mesh_workspace) {
        Some(mutation) => Emit::mutations(vec![mutation]),
        None => Emit::default(),
    })
}
//#endregion 🔖️MeshEdit

//#region 🔖️LowpolyScratch
/// 🗃️ Pure render-side cache of composited paint textures (base64 PNG per object), invalidated by a fingerprint over
/// the rendered projection's paint pixels. Never serialized.
#[derive(Default)]
pub struct PaintTextureLut {
    fingerprint: Option<u64>,
    pub textures: HashMap<String, String>,
}

/// 🖌️ The dispatch context every `🎮️commands/*` handler receives: the session-local `mesh_workspace` cache (live
/// half-edge-mesh JSON per object id, the fallback for a legacy handle-only object whose content the document does not
/// persist), the dispatch's mesh-domain selection, the paint texture cache and the window paint gestures the
/// transient carries, passed through untouched so a republished cache never drops an open stroke.
pub struct LowpolyScratch {
    texture_cache: PaintTextureLut,
    mesh_workspace: HashMap<String, String>,
    current_selection: LowpolySelection,
    selection_object_id: Option<String>,
    selected_object_ids: Vec<String>,
    paint: BTreeMap<String, LowpolyPaintGesture>,
}

impl Default for LowpolyScratch {
    fn default() -> Self {
        Self { texture_cache: PaintTextureLut::default(), mesh_workspace: crate::schema::default_mesh_workspace(), current_selection: LowpolySelection::default(), selection_object_id: None, selected_object_ids: Vec::new(), paint: BTreeMap::new() }
    }
}

impl LowpolyScratch {
    /// 🕹️ Sets THIS dispatch's mesh-domain selection.
    pub fn set_current_selection(&mut self, selection: LowpolySelection) {
        self.current_selection = selection;
    }

    /// 🕹️ THIS dispatch's mesh-domain selection (the default/empty one outside a command dispatch, e.g. `render`).
    pub fn current_selection(&self) -> &LowpolySelection {
        &self.current_selection
    }

    /// 🎯️ Sets the object ids THIS dispatch's selection names at object granularity.
    pub fn set_selected_object_ids(&mut self, object_ids: Vec<String>) {
        self.selected_object_ids = object_ids;
    }

    pub fn selected_object_ids(&self) -> &[String] {
        &self.selected_object_ids
    }

    pub fn set_selection_object_id(&mut self, object_id: Option<String>) {
        self.selection_object_id = object_id;
    }

    pub fn selection_object_id(&self) -> Option<&str> {
        self.selection_object_id.as_deref()
    }

    /// 🕸️ The live half-edge-mesh JSON cached for `object_id`, or `""` when this session has no working content for it.
    pub fn mesh_workspace(&self, object_id: &str) -> &str {
        self.mesh_workspace.get(object_id).map(String::as_str).unwrap_or_default()
    }

    /// 🕸️ A clone of the full session-local mesh-workspace cache — `LowpolyDocument::with_context`'s input.
    pub fn mesh_workspace_map(&self) -> HashMap<String, String> {
        self.mesh_workspace.clone()
    }

    /// 🕸️ Replaces the whole session-local mesh-workspace cache after a successful edit.
    pub fn set_mesh_workspace_map(&mut self, map: HashMap<String, String>) {
        self.mesh_workspace = map;
    }

    fn paint_fingerprint(projection: &LowpolySnapshot) -> u64 {
        let mut hash = 0xcbf29ce484222325u64;
        for object in &projection.objects {
            hash = fnv1a_u64(hash, object.id.as_bytes());
            for layer in &object.paint_layers {
                hash = fnv1a_u64(hash, &[layer.visible as u8]);
                hash = fnv1a_u64(hash, &layer.opacity.to_le_bytes());
                hash = fnv1a_u64(hash, &layer.pixels);
            }
        }
        hash
    }

    /// 🎨️ Composites every object's paint stack of `projection` (the document with every open paint gesture previewed)
    /// into the texture cache, unless its pixels did not change since the last refresh.
    pub fn refresh_texture_cache(&mut self, projection: &LowpolySnapshot) {
        let fingerprint = Self::paint_fingerprint(projection);
        if self.texture_cache.fingerprint == Some(fingerprint) {
            return;
        }
        let mut textures = HashMap::new();
        for object in &projection.objects {
            let composite = composite_layer_pixels(&object.paint_layers);
            if let Ok(png_bytes) = encode_rgba_png(&composite, LOWPOLY_PAINT_TEXTURE_SIZE as u32, LOWPOLY_PAINT_TEXTURE_SIZE as u32) {
                textures.insert(object.id.clone(), base64_codec::base64_standard_encode(png_bytes));
            }
        }
        self.texture_cache = PaintTextureLut { fingerprint: Some(fingerprint), textures };
    }

    pub fn textures(&self) -> &HashMap<String, String> {
        &self.texture_cache.textures
    }
}
//#endregion 🔖️LowpolyScratch

//#region 🖌️PaintGesture
/// 💾️ One window's in-flight paint gesture between dispatches: the paint tool's statechart configuration by stable
/// ids, the admission's authoring seed and the base revision it opened on, the open transaction's ref and its ONE
/// provisional leaf — an `apply-paint-stroke` whose dabs grow tick by tick, or the `edit-paint-layer` of a fill.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LowpolyPaintGesture {
    pub states: Vec<String>,
    pub authoring_seed: String,
    pub base_revision: String,
    pub transaction: protocol::TransactionRef,
    pub leaf: LowpolyMutation,
}
//#endregion 🖌️PaintGesture

//#region 🔖️Transient
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LowpolyTransientState {
    mesh_workspace: Arc<BTreeMap<String, String>>,
    paint: BTreeMap<String, LowpolyPaintGesture>,
}

impl Default for LowpolyTransientState {
    fn default() -> Self {
        Self { mesh_workspace: Arc::new(crate::schema::default_mesh_workspace().into_iter().collect()), paint: BTreeMap::new() }
    }
}

struct LowpolyTransientStateRef<'a> {
    mesh_workspace: &'a BTreeMap<String, String>,
    paint: &'a BTreeMap<String, LowpolyPaintGesture>,
}

/// 🌉️ Hand-written, not derived: the reference fields would need `ToValue` for reference types, which the codec
/// deliberately never provides; each field converts through its owned type's impl, in the derive's camelCase shape.
impl<'a> dsl::ToValue for LowpolyTransientStateRef<'a> {
    fn to_value(&self) -> dsl::DslValue {
        dsl::DslValue::Object(vec![("meshWorkspace".to_string(), dsl::ToValue::to_value(self.mesh_workspace)), ("paint".to_string(), dsl::ToValue::to_value(self.paint))])
    }
}

#[derive(value_derive::FromValue, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
struct LowpolyTransientStateWire {
    mesh_workspace: BTreeMap<String, String>,
    #[value(default)]
    paint: BTreeMap<String, LowpolyPaintGesture>,
}

/// 🫧️ Immutable request-owned Lowpoly editing session snapshot: the live mesh bytes behind one shared typed root, and
/// every window's open paint gesture.
#[derive(Clone, Debug, PartialEq)]
pub struct LowpolyTransient {
    state: Arc<LowpolyTransientState>,
}

impl Default for LowpolyTransient {
    fn default() -> Self {
        Self { state: Arc::new(LowpolyTransientState::default()) }
    }
}

impl LowpolyTransient {
    #[cfg(test)]
    pub(crate) fn with_test_workspace_bytes(bytes: usize) -> Self {
        let mut state = LowpolyTransientState::default();
        Arc::make_mut(&mut state.mesh_workspace).insert("test-padding".into(), "x".repeat(bytes));
        Self { state: Arc::new(state) }
    }

    /// 🖌️ The paint gesture `window` has in flight.
    pub fn paint(&self, window: &str) -> Option<&LowpolyPaintGesture> {
        self.state.paint.get(window)
    }

    /// 🪟️ This transient with `window`'s paint gesture replaced by `gesture` (`None` clears it); the mesh root is shared.
    pub fn with_paint(&self, window: &str, gesture: Option<LowpolyPaintGesture>) -> Self {
        let mut paint = self.state.paint.clone();
        match gesture {
            Some(gesture) => paint.insert(window.to_string(), gesture),
            None => paint.remove(window),
        };
        Self { state: Arc::new(LowpolyTransientState { mesh_workspace: self.state.mesh_workspace.clone(), paint }) }
    }

    /// 👁️ `document` with every window's provisional paint leaf applied — the preview every window of this instance
    /// paints, never history; `None` when no gesture is open. A leaf that does not apply is skipped.
    pub fn paint_preview(&self, document: &LowpolySnapshot) -> Option<LowpolySnapshot> {
        (!self.state.paint.is_empty()).then(|| self.state.paint.values().fold(document.clone(), |state, gesture| protocol::apply_mutation(&state, &gesture.leaf).map_or(state, |(next, _)| next)))
    }
}

/// 🔀️ Hand-written, not derived: `state` is an `Arc<LowpolyTransientState>` whose mesh root is itself shared; bridges
/// through `LowpolyTransientStateRef`/`LowpolyTransientStateWire`.
impl dsl::ToValue for LowpolyTransient {
    fn to_value(&self) -> dsl::DslValue {
        dsl::ToValue::to_value(&LowpolyTransientStateRef { mesh_workspace: &self.state.mesh_workspace, paint: &self.state.paint })
    }
}

impl dsl::FromValue for LowpolyTransient {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let wire: LowpolyTransientStateWire = dsl::FromValue::from_value(value)?;
        Ok(Self { state: Arc::new(LowpolyTransientState { mesh_workspace: Arc::new(wire.mesh_workspace), paint: wire.paint }) })
    }
}

impl store::ArtifactDsl for LowpolyTransient {
    const EXTENSION: &'static str = "lowpoly.transient";
    fn envelope_id() -> &'static str {
        "lowpoly.transient"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        dsl::json::from_json_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = dsl::json::to_json_string(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid lowpoly transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for LowpolyTransient {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = dsl::json::to_json_string(self).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let text = std::str::from_utf8(&inner).map_err(|error| store::PackError::Schema(error.to_string()))?;
        dsl::json::from_json_str(text).map_err(|error| store::PackError::Schema(error.to_string()))
    }
}

impl protocol::MutationDiff<LowpolyTransient> for LowpolyTransient {
    fn apply(&self, _base: &LowpolyTransient) -> protocol::MutationApplyResult<LowpolyTransient> {
        Ok(self.clone())
    }

    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum LowpolyTransientMutation {
    Snapshot { transient: LowpolyTransient },
}

impl Mutation<LowpolyTransient> for LowpolyTransientMutation {
    type Diff = LowpolyTransient;

    /// 🧷️ Per-variant leaf metadata for this hand-written (non-derived) aggregate — one entry for the sole `Snapshot`
    /// variant.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🖌️set-snapshot",
        semantic_kind: "set-snapshot",
        display_name: "Set Snapshot",
        emoji: "🖌️",
        aggregate_variant: "Snapshot",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            LowpolyTransientMutation::Snapshot { .. } => &Self::DESCRIPTORS[0],
        }
    }

    fn diff(&self, _base: &LowpolyTransient) -> protocol::MutationOutcome<LowpolyTransient> {
        protocol::MutationOutcome::new(match self {
            Self::Snapshot { transient } => transient.clone(),
        })
    }

    fn inverse(&self, base: &LowpolyTransient) -> Vec<Self> {
        vec![Self::Snapshot { transient: base.clone() }]
    }
}

impl protocol::OpText for LowpolyTransientMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let body = line.strip_prefix("snapshot ").ok_or_else(|| store::TextError::new("expected Lowpoly transient snapshot", store::TextSpan::at(1, 1)))?;
        dsl::json::from_json_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_op(&self) -> String {
        format!("snapshot {}", dsl::json::to_json_string(self))
    }
}

impl protocol::OpBinary for LowpolyTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(dsl::json::to_json_string(self).into_bytes())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

impl LowpolyScratch {
    /// 🫧️ The dispatch context rehydrated from the live transient: its mesh cache and its window paint gestures.
    pub fn from_transient(transient: &LowpolyTransient, current_selection: LowpolySelection) -> Self {
        Self {
            texture_cache: PaintTextureLut::default(),
            mesh_workspace: transient.state.mesh_workspace.iter().map(|(key, value)| (key.clone(), value.clone())).collect(),
            current_selection,
            selection_object_id: None,
            selected_object_ids: Vec::new(),
            paint: transient.state.paint.clone(),
        }
    }

    /// 🫧️ The transient this context republishes: its (possibly updated) mesh cache and the window paint gestures it
    /// rehydrated, untouched.
    pub fn transient_snapshot(&self) -> LowpolyTransient {
        LowpolyTransient { state: Arc::new(LowpolyTransientState { mesh_workspace: Arc::new(self.mesh_workspace.clone().into_iter().collect()), paint: self.paint.clone() }) }
    }
}
//#endregion 🔖️Transient

//#region 🛠️Tool
/// 🪪️ The editor id every lowpoly tool transaction's `tool` is scoped by: `<appId>#<verb>`.
pub const LOWPOLY_TOOL_APP_ID: &str = "s.lowpoly.lowpoly@1/*#editor";

/// 🔑️ The transaction key of a streamed gesture's one leaf; a one-shot keys its leaves `"leaf:<index>"`.
pub const LOWPOLY_TOOL_STREAM_KEY: &str = "stream:0";

/// 📨️ One tool event's leaves: a one-shot's whole set, or a stream tick's single leaf (none for a bare release).
#[derive(Clone, Debug)]
pub struct LowpolyToolRequest {
    pub leaves: Vec<LowpolyMutation>,
}

/// 🧰️ The tool's context: the net leaf a streamed gesture has accumulated so far (`None` at rest).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LowpolyToolContext {
    pub stream: Option<LowpolyMutation>,
}

/// ➕️ `net` followed by `tick` as ONE net leaf: a paint stroke appends the tick's dabs when the brush is unchanged;
/// everything else (a fill, a changed brush) keeps the net leaf the gesture opened with.
pub fn lowpoly_stream_then(net: LowpolyMutation, tick: &LowpolyMutation) -> LowpolyMutation {
    let merged = match (&net, tick) {
        (LowpolyMutation::ApplyPaintStroke(stroke), LowpolyMutation::ApplyPaintStroke(next)) => stroke.then(next),
        _ => None,
    };
    merged.map_or(net, LowpolyMutation::ApplyPaintStroke)
}

fn lowpoly_tool_context(input: LowpolyToolContext) -> LowpolyToolContext {
    input
}

fn request_moves(_context: &LowpolyToolContext, event: Option<&lowpoly_tool::Event>) -> bool {
    matches!(event, Some(lowpoly_tool::Event::Once(request) | lowpoly_tool::Event::Stream(request)) if !request.leaves.is_empty())
}

fn yield_once(_context: &mut LowpolyToolContext, event: Option<&lowpoly_tool::Event>, sink: &mut Vec<Command<lowpoly_tool::LowpolyTool>>) {
    let Some(lowpoly_tool::Event::Once(request)) = event else { return };
    sink.extend(request.leaves.iter().enumerate().map(|(index, leaf)| Command::Effect(ToolYield::upsert(format!("leaf:{index}"), leaf.clone()))));
    sink.push(Command::Effect(ToolYield::Commit));
}

fn begin_stream(context: &mut LowpolyToolContext, event: Option<&lowpoly_tool::Event>, sink: &mut Vec<Command<lowpoly_tool::LowpolyTool>>) {
    let Some(lowpoly_tool::Event::Stream(LowpolyToolRequest { leaves })) = event else { return };
    let Some(tick) = leaves.first() else { return };
    sink.push(Command::Effect(ToolYield::upsert(LOWPOLY_TOOL_STREAM_KEY, tick.clone())));
    context.stream = Some(tick.clone());
}

fn continue_stream(context: &mut LowpolyToolContext, event: Option<&lowpoly_tool::Event>, sink: &mut Vec<Command<lowpoly_tool::LowpolyTool>>) {
    let (Some(lowpoly_tool::Event::Stream(request)), Some(stream)) = (event, context.stream.take()) else { return };
    let net = match request.leaves.first() {
        Some(tick) => lowpoly_stream_then(stream, tick),
        None => stream,
    };
    sink.push(Command::Effect(ToolYield::upsert(LOWPOLY_TOOL_STREAM_KEY, net.clone())));
    context.stream = Some(net);
}

fn finish_stream(context: &mut LowpolyToolContext, event: Option<&lowpoly_tool::Event>, sink: &mut Vec<Command<lowpoly_tool::LowpolyTool>>) {
    let (Some(lowpoly_tool::Event::Finish(request)), Some(stream)) = (event, context.stream.take()) else { return };
    let net = match request.leaves.first() {
        Some(tick) => lowpoly_stream_then(stream, tick),
        None => stream,
    };
    sink.push(Command::Effect(ToolYield::upsert(LOWPOLY_TOOL_STREAM_KEY, net)));
    sink.push(Command::Effect(ToolYield::Commit));
}

fn cancel_stream(context: &mut LowpolyToolContext, _event: Option<&lowpoly_tool::Event>, sink: &mut Vec<Command<lowpoly_tool::LowpolyTool>>) {
    context.stream = None;
    sink.push(Command::Effect(ToolYield::Abort));
}

machine::statechart! {
    machine lowpoly_tool {
        context: LowpolyToolContext;
        event Event { Once(LowpolyToolRequest), Stream(LowpolyToolRequest), Finish(LowpolyToolRequest), Cancel }
        input: LowpolyToolContext;
        output: ();
        effect: ToolYield<LowpolyMutation>;
        context_from_input: lowpoly_tool_context;
        initial: idle;
        state idle {
            on Once if request_moves => idle do yield_once;
            on Stream if request_moves => streaming do begin_stream;
        }
        state streaming {
            on Stream => streaming do continue_stream;
            on Finish => idle do finish_stream;
            on Cancel => idle do cancel_stream;
        }
    }
}

/// 🧷️ The lowpoly tool's host: its chart declares no timer, no invoke and no foreign effect.
pub struct LowpolyToolHost;

impl machine::Host<lowpoly_tool::LowpolyTool> for LowpolyToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<LowpolyMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

fn lowpoly_tool_clock() -> protocol::HybridLogicalTimestamp {
    protocol::HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 }
}

/// 🛠️ Runs one gesture through a lowpoly tool at rest as ONE transaction of `<appId>#<verb>`, its ref minted from the
/// admission's `authoring_seed` and the host clock. `None` when the gesture yields nothing: zero trace.
pub fn lowpoly_tool_once(verb: &str, authoring_seed: &str, leaves: Vec<LowpolyMutation>) -> Option<(protocol::TransactionRef, Vec<LowpolyMutation>)> {
    let mut runner = ToolMachineRunner::<lowpoly_tool::LowpolyTool, LowpolyToolHost>::start(format!("{LOWPOLY_TOOL_APP_ID}#{verb}"), protocol::ActorId(authoring_seed.to_string()), LowpolyToolContext::default(), LowpolyToolHost).ok()?;
    match runner.send(lowpoly_tool::Event::Once(LowpolyToolRequest { leaves }), lowpoly_tool_clock()).ok()? {
        ToolStep::Committed(transaction, mutations) => Some((transaction, mutations)),
        _ => None,
    }
}

/// 📤️ The emission of one one-shot gesture: its committed transaction as ONE edit stamped with the ref — plain when the
/// view carries no admission (a render or test view without command authority) — or nothing.
pub fn lowpoly_tool_emit(verb: &str, doc: &semio_framework_plugin::ArtifactView<'_, LowpolySnapshot>, leaves: Vec<LowpolyMutation>) -> Emit<LowpolyMutation, crate::editor::lowpoly::config::LowpolyConfigMutation> {
    let seed = doc.operation_optional().map(|operation| operation.authoring_seed.as_str()).unwrap_or_default();
    match lowpoly_tool_once(verb, seed, leaves) {
        Some((transaction, mutations)) if !seed.is_empty() => Emit::commit_transaction(transaction, mutations),
        Some((_, mutations)) => Emit::mutations(mutations),
        None => Emit::default(),
    }
}

/// 🎚️ Where one paint dispatch sits in its gesture: a one-shot `Once`, a `Stream` tick into the window's open
/// transaction, the `Commit` that ends it, or a host `Abort` with its reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LowpolyToolPhase {
    Once,
    Stream,
    Commit,
    Abort(ToolAbortReason),
}

impl LowpolyToolPhase {
    /// 🧩️ Reads a paint verb's `phase` (`stream` | `commit` | `abort`, absent = one-shot) and an abort's `reason`
    /// (`blur`, `captureLost`, `baseMoved`, `frozen`, `retired`; absent = `tool`); `None` for an unknown one.
    pub fn parse(phase: Option<&str>, reason: Option<&str>) -> Option<Self> {
        match phase {
            None => Some(Self::Once),
            Some("stream") => Some(Self::Stream),
            Some("commit") => Some(Self::Commit),
            Some("abort") => reason.map_or(Some(ToolAbortReason::Tool), ToolAbortReason::parse).map(Self::Abort),
            Some(_) => None,
        }
    }
}

/// 🛠️ One window's paint tool for one dispatch, a `🛠️tool-machine` runner scoped `<appId>#paint`, started at rest or
/// resumed from the gesture its window persisted.
struct LowpolyPaintTool {
    runner: ToolMachineRunner<lowpoly_tool::LowpolyTool, LowpolyToolHost>,
    authoring_seed: String,
    base_revision: String,
}

impl LowpolyPaintTool {
    fn tool_id() -> String {
        format!("{LOWPOLY_TOOL_APP_ID}#paint")
    }

    fn start(authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal> {
        let runner = ToolMachineRunner::start(Self::tool_id(), protocol::ActorId(authoring_seed.to_string()), LowpolyToolContext::default(), LowpolyToolHost)?;
        Ok(Self { runner, authoring_seed: authoring_seed.to_string(), base_revision: base_revision.to_string() })
    }

    fn resume(gesture: &LowpolyPaintGesture) -> Result<Self, ToolRefusal> {
        let context = LowpolyToolContext { stream: Some(gesture.leaf.clone()) };
        let persisted = machine::PersistedSnapshot { version: 1, fingerprint: <lowpoly_tool::LowpolyTool as machine::Machine>::definition().fingerprint, states: gesture.states.clone(), history: Vec::new(), done: false };
        let snapshot = machine::restore::<lowpoly_tool::LowpolyTool, machine::NoMigrations>(&persisted, context, &[]).map_err(|_| ToolRefusal::Closed)?;
        let transaction = ToolTransaction::resume(gesture.transaction.clone(), vec![(LOWPOLY_TOOL_STREAM_KEY.to_string(), gesture.leaf.clone())]);
        let runner = ToolMachineRunner::resume(Self::tool_id(), protocol::ActorId(gesture.authoring_seed.clone()), LowpolyToolContext::default(), snapshot, Some(transaction), LowpolyToolHost)?;
        Ok(Self { runner, authoring_seed: gesture.authoring_seed.clone(), base_revision: gesture.base_revision.clone() })
    }

    fn send(&mut self, phase: LowpolyToolPhase, tick: Option<LowpolyMutation>) -> Result<ToolStep<LowpolyMutation>, ToolRefusal> {
        let request = LowpolyToolRequest { leaves: tick.into_iter().collect() };
        let event = match phase {
            LowpolyToolPhase::Stream => lowpoly_tool::Event::Stream(request),
            LowpolyToolPhase::Commit if !self.runner.at_rest() => lowpoly_tool::Event::Finish(request),
            LowpolyToolPhase::Abort(_) => lowpoly_tool::Event::Cancel,
            LowpolyToolPhase::Once | LowpolyToolPhase::Commit => lowpoly_tool::Event::Once(request),
        };
        self.runner.send(event, lowpoly_tool_clock())
    }

    fn persist(self) -> Option<LowpolyPaintGesture> {
        let (snapshot, transaction) = self.runner.into_parts();
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        let leaf = transaction.entries().iter().find(|(key, _)| key == LOWPOLY_TOOL_STREAM_KEY).map(|(_, leaf)| leaf.clone())?;
        Some(LowpolyPaintGesture { states: machine::persist(&snapshot).states, authoring_seed: self.authoring_seed, base_revision: self.base_revision, transaction: transaction.reference().clone(), leaf })
    }
}

/// 🧮️ What one paint dispatch did: the transaction it committed (publish it as ONE edit) and the next transient when
/// the dispatch opened, advanced, committed or dropped the window's gesture.
pub struct LowpolyPaintDrive {
    pub committed: Option<(protocol::TransactionRef, Vec<LowpolyMutation>)>,
    pub transient: Option<LowpolyTransient>,
}

/// 🛠️ Drives `window`'s paint tool through ONE dispatch. `Once` commits `tick` as one transaction; `Stream` upserts it
/// into the window's open transaction (opening it on the first tick), `Commit` folds it in and commits the whole
/// gesture, `Abort` drops the open gesture with zero trace. An open gesture a one-shot interrupts is aborted
/// `captureLost`; one whose base moved under it is aborted `baseMoved`, and a stream tick or commit that found it is
/// dropped with it.
pub fn lowpoly_paint_drive(transient: &LowpolyTransient, window: &str, phase: LowpolyToolPhase, tick: Option<LowpolyMutation>, authoring_seed: &str, base_revision: &str) -> LowpolyPaintDrive {
    let persisted = transient.paint(window);
    let open = persisted.and_then(|gesture| LowpolyPaintTool::resume(gesture).ok());
    let dropped = LowpolyPaintDrive { committed: None, transient: persisted.map(|_| transient.with_paint(window, None)) };
    let open = match (open, phase) {
        (Some(mut tool), LowpolyToolPhase::Abort(reason)) => {
            tool.runner.abort(reason);
            return dropped;
        }
        (None, LowpolyToolPhase::Abort(_)) => return dropped,
        (Some(mut tool), _) if tool.base_revision != base_revision => {
            tool.runner.abort(ToolAbortReason::BaseMoved);
            if phase != LowpolyToolPhase::Once {
                return dropped;
            }
            None
        }
        (Some(mut tool), LowpolyToolPhase::Once) => {
            tool.runner.abort(ToolAbortReason::CaptureLost);
            None
        }
        (open, _) => open,
    };
    let Some(mut tool) = open.or_else(|| LowpolyPaintTool::start(authoring_seed, base_revision).ok()) else { return dropped };
    let step = tool.send(phase, tick);
    let next = transient.with_paint(window, tool.persist());
    let changed = (next != *transient).then_some(next);
    match step {
        Ok(ToolStep::Committed(reference, mutations)) => LowpolyPaintDrive { committed: Some((reference, mutations)), transient: changed },
        Ok(_) | Err(_) => LowpolyPaintDrive { committed: None, transient: changed },
    }
}
//#endregion 🛠️Tool

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
