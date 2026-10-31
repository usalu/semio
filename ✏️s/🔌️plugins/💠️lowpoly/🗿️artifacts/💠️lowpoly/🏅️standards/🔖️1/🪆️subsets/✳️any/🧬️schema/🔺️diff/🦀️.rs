//! 🧬️ Lowpoly diff schema — sparse edits over the artifact: a positional object row delta whose patches carry the object's own
//! field patch, an ordered list of paint-layer edits and an ordered list of selection motions applied to its mesh.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🧩️Modules
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod component {
                        #[path = "."]
                        pub mod binary {
                            #[path = "../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                            pub mod diff;
                        }
                        #[path = "."]
                        pub mod text {
                            #[path = "../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                            pub mod diff;
                        }
                    }
                    pub use component::*;
                }
            }
        }
    }
}
//#endregion 🧩️Modules

use semio_s_artifact_lowpoly_lowpoly_model::snapshot::{LowpolyObject, LowpolyObjectPatch, LowpolyPaintLayer, LowpolySnapshot};
use framework_schema::ArtifactSchema;
use protocol::{MutationDiff, Patchable};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the lowpoly artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff). There is no
/// whole-document slot: objects are a positional row delta and every object patch is field-sparse.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolyDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub objects: Option<LowpolyObjectsDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
protocol::list_delta! {
    /// 🧩 Positional row delta for `objects`.
    pub LowpolyObjectsDelta { removal: LowpolyObjectRemoval, insertion: LowpolyObjectInsertion, relocation: LowpolyObjectRelocation, modification: LowpolyObjectsModification, row: LowpolyObject, patch: LowpolyObjectPatchEntry, list: Vec<LowpolyObject>, key: String = by LowpolyObjectKeys, values_only }
}

/// 🔑️ Keys the foreign `LowpolyObject` rows of their list delta by `id`.
pub(crate) struct LowpolyObjectKeys;

impl protocol::list_delta::KeyOf<LowpolyObject> for LowpolyObjectKeys {
    type Key = String;
    fn key_of(row: &LowpolyObject) -> String { row.id.clone() }
}

/// 🩹 One patched object entry: the object's own field patch, then its paint-layer edits, then its mesh vertex positions and the Normal channels they rewrote, in that order.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct LowpolyObjectPatchEntry {
    pub patch: LowpolyObjectPatch,
    pub paint_layers: Option<LowpolyPaintLayersDelta>,
    pub mesh_vertices: Vec<LowpolyVertexPosition>,
    pub mesh_attributes: Vec<semio_s_artifact_lowpoly_lowpoly_model::managed_mesh::LowpolyMeshAttribute>,
}

/// 🖌️ Ordered paint-layer edits under an object patch (layers are addressed by index and carry no identity).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct LowpolyPaintLayersDelta {
    pub edits: Vec<LowpolyPaintEdit>,
}

/// ✏️ One edit of an object's paint-layer list; edits apply in order.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(tag = "op", rename_all = "camelCase")]
pub enum LowpolyPaintEdit {
    Insert { index: u32, layer: LowpolyPaintLayer },
    Remove { index: u32 },
    Replace { index: u32, layer: LowpolyPaintLayer },
    Patch { index: u32, patch: LowpolyPaintLayerPatch },
    Stroke { index: u32, runs: Vec<PixelRun> },
}

/// 📍️ One vertex of an object's managed mesh at an absolute position; the central applier writes it, recomputes the normals and
/// re-derives the mesh handle.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct LowpolyVertexPosition {
    pub vertex: u32,
    pub position: [f32; 3],
}

/// 🩹 Paint-layer metadata patch.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct LowpolyPaintLayerPatch {
    pub name: Option<String>,
    pub visible: Option<bool>,
    pub opacity: Option<f32>,
    pub blend_mode: Option<String>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
//#endregion 🔁️Re-exports


impl protocol::list_delta::RowPatch<LowpolyObject> for LowpolyObjectPatchEntry {
    fn commit_into(&self, row: &mut LowpolyObject, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        row.apply_patch(&self.patch);
        if let Some(paint) = &self.paint_layers {
            row.paint_layers = paint_edits_after(&row.paint_layers, &paint.edits).map_err(|error| error.under(["paintLayers"]))?;
        }
        if !self.mesh_vertices.is_empty() || !self.mesh_attributes.is_empty() {
            let state = row.mesh_state.clone().ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "mesh vertex positions need the object's managed mesh").at(["meshVertices"]))?;
            let mut mesh = state.into_mesh().map_err(|_| protocol::MutationApplyError::new("mutation.apply.invalid-base", "the object's managed mesh does not decode").at(["meshVertices"]))?;
            for (index, vertex) in self.mesh_vertices.iter().enumerate() {
                mesh.set_vertex_position(semio_framework_3d::mesh::VertexId(vertex.vertex), semio_framework_3d::mesh::Vec3(vertex.position)).map_err(|_| protocol::MutationApplyError::new("mutation.apply.invalid-index", "the mesh vertex does not exist").at(["meshVertices".to_string(), index.to_string()]))?;
            }
            for (index, channel) in self.mesh_attributes.iter().enumerate() {
                let attribute = semio_framework_3d::mesh::MeshAttribute { domain: channel.domain, semantic: channel.semantic, interpolation: channel.interpolation, values: channel.values.clone(), indices: channel.indices.clone() };
                mesh.set_attribute(channel.name.clone(), attribute).map_err(|_| protocol::MutationApplyError::new("mutation.apply.invalid-target", "the mesh attribute channel does not fit the mesh").at(["meshAttributes".to_string(), index.to_string()]))?;
            }
            mesh.recompute_normals().map_err(|_| protocol::MutationApplyError::new("mutation.apply.invalid-motion", "the vertex positions degenerate the mesh").at(["meshVertices"]))?;
            let state = semio_s_artifact_lowpoly_lowpoly_model::managed_mesh::LowpolyMeshState::from_mesh(mesh);
            if self.patch.mesh.is_none() {
                row.mesh = Some(semio_s_artifact_lowpoly_lowpoly_model::snapshot::managed_mesh_child_handle(&row.id, &state));
            }
            row.mesh_state = Some(state);
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        let patch = compose_object_patch(&self.patch, &later.patch);
        let replaced = later.patch.mesh_state.is_some();
        let mut mesh_vertices: Vec<LowpolyVertexPosition> = if replaced { Vec::new() } else { self.mesh_vertices.iter().filter(|row| later.mesh_vertices.iter().all(|overwritten| overwritten.vertex != row.vertex)).cloned().collect() };
        mesh_vertices.extend(later.mesh_vertices.iter().cloned());
        mesh_vertices.sort_by_key(|row| row.vertex);
        let mut mesh_attributes: Vec<semio_s_artifact_lowpoly_lowpoly_model::managed_mesh::LowpolyMeshAttribute> = if replaced { Vec::new() } else { self.mesh_attributes.iter().filter(|row| later.mesh_attributes.iter().all(|overwritten| overwritten.name != row.name)).cloned().collect() };
        mesh_attributes.extend(later.mesh_attributes.iter().cloned());
        mesh_attributes.sort_by(|left, right| left.name.cmp(&right.name));
        let paint_layers = match (self.paint_layers.take(), later.paint_layers) {
            (None, None) => None,
            (first, later) => Some(LowpolyPaintLayersDelta { edits: canonical_paint_edits(first.iter().chain(later.iter()).flat_map(|delta| delta.edits.iter().cloned()).collect()) }),
        };
        *self = Self { patch, paint_layers, mesh_vertices, mesh_attributes };
    }
    fn inverse(&self, row: &LowpolyObject) -> Self {
        let restores_mesh = (!self.mesh_vertices.is_empty() || !self.mesh_attributes.is_empty()) && self.patch.mesh_state.is_none();
        let base_state = row.mesh_state.as_ref();
        let base_vertices = base_state.map(|state| state.vertices.as_slice()).unwrap_or_default();
        Self {
                        patch: LowpolyObjectPatch {
                name: self.patch.name.as_ref().map(|_| row.name.clone()),
                smooth_shading: self.patch.smooth_shading.map(|_| row.smooth_shading),
                position: self.patch.position.map(|_| row.transform.position),
                rotation: self.patch.rotation.map(|_| row.transform.rotation),
                scale: self.patch.scale.map(|_| row.transform.scale),
                mesh: (restores_mesh || self.patch.mesh.is_some()).then(|| row.mesh.clone()),
                mesh_content: self.patch.mesh_content.as_ref().map(|_| row.mesh_content.clone()),
                mesh_state: self.patch.mesh_state.as_ref().map(|_| row.mesh_state.clone()),
            },
            paint_layers: self.paint_layers.as_ref().map(|paint| LowpolyPaintLayersDelta { edits: paint_edits_inverse(&row.paint_layers, &paint.edits) }),
            mesh_vertices: if restores_mesh { self.mesh_vertices.iter().filter_map(|vertex| base_vertices.get(vertex.vertex as usize).map(|held| LowpolyVertexPosition { vertex: vertex.vertex, position: held.position })).collect() } else { Vec::new() },
            mesh_attributes: if restores_mesh { self.mesh_attributes.iter().filter_map(|channel| base_state.and_then(|state| state.attributes.iter().find(|held| held.name == channel.name)).cloned()).collect() } else { Vec::new() },
        }
    }
    fn is_empty(&self) -> bool {
        self.patch == LowpolyObjectPatch::default() && self.paint_layers.as_ref().is_none_or(|paint| paint.edits.is_empty()) && self.mesh_vertices.is_empty() && self.mesh_attributes.is_empty()
    }
}

/// ➕️ The patch `first` then `later` amount to: every later slot wins.
fn compose_object_patch(first: &LowpolyObjectPatch, later: &LowpolyObjectPatch) -> LowpolyObjectPatch {
    LowpolyObjectPatch {
        name: later.name.clone().or_else(|| first.name.clone()),
        smooth_shading: later.smooth_shading.or(first.smooth_shading),
        position: later.position.or(first.position),
        rotation: later.rotation.or(first.rotation),
        scale: later.scale.or(first.scale),
        mesh: later.mesh.clone().or_else(|| first.mesh.clone()),
        mesh_content: later.mesh_content.clone().or_else(|| first.mesh_content.clone()),
        mesh_state: later.mesh_state.clone().or_else(|| first.mesh_state.clone()),
    }
}

//#region 🔖️PaintEdits
fn paint_edit_error(code: &str, message: &str, index: usize) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at(["edits".to_string(), index.to_string()])
}

fn patch_layer(layer: &mut LowpolyPaintLayer, patch: &LowpolyPaintLayerPatch) {
    if let Some(value) = &patch.name {
        layer.name = value.clone();
    }
    if let Some(value) = patch.visible {
        layer.visible = value;
    }
    if let Some(value) = patch.opacity {
        layer.opacity = value;
    }
    if let Some(value) = &patch.blend_mode {
        layer.blend_mode = value.clone();
    }
}

fn stroke_layer(layer: &mut LowpolyPaintLayer, runs: &[PixelRun]) -> Option<()> {
    let mut pixels = layer.materialized_pixels();
    for run in runs {
        let start = run.offset as usize;
        let end = start.checked_add(run.bytes.len()).filter(|end| *end <= pixels.len())?;
        pixels[start..end].copy_from_slice(&run.bytes);
    }
    layer.pixels = pixels;
    if layer.pixels.iter().all(|byte| *byte == 255) {
        layer.pixels = Vec::new();
    }
    Some(())
}

/// ▶️ Applies the paint-layer edits in order onto `layers`.
pub fn paint_edits_after(layers: &[LowpolyPaintLayer], edits: &[LowpolyPaintEdit]) -> Result<Vec<LowpolyPaintLayer>, protocol::MutationApplyError> {
    let mut next: Vec<LowpolyPaintLayer> = Vec::new();
    next.extend_from_slice(layers);
    for (position, edit) in edits.iter().enumerate() {
        match edit {
            LowpolyPaintEdit::Insert { index, layer } if *index as usize <= next.len() => next.insert(*index as usize, layer.clone()),
            LowpolyPaintEdit::Insert { .. } => return Err(paint_edit_error("mutation.apply.invalid-index", "added paint layer index is out of range", position)),
            LowpolyPaintEdit::Remove { index } if (*index as usize) < next.len() => {
                next.remove(*index as usize);
            }
            LowpolyPaintEdit::Replace { index, layer } if (*index as usize) < next.len() => next[*index as usize] = layer.clone(),
            LowpolyPaintEdit::Patch { index, patch } if (*index as usize) < next.len() => patch_layer(&mut next[*index as usize], patch),
            LowpolyPaintEdit::Stroke { index, runs } if (*index as usize) < next.len() => {
                stroke_layer(&mut next[*index as usize], runs).ok_or_else(|| paint_edit_error("mutation.apply.invalid-index", "paint stroke byte range is out of bounds", position))?;
            }
            _ => return Err(paint_edit_error("mutation.apply.invalid-index", "paint layer index is out of range", position)),
        }
    }
    Ok(next)
}

fn compose_layer_patches(first: &LowpolyPaintLayerPatch, later: &LowpolyPaintLayerPatch) -> LowpolyPaintLayerPatch {
    LowpolyPaintLayerPatch { name: later.name.clone().or_else(|| first.name.clone()), visible: later.visible.or(first.visible), opacity: later.opacity.or(first.opacity), blend_mode: later.blend_mode.clone().or_else(|| first.blend_mode.clone()) }
}

fn merge_paint_edits(top: &LowpolyPaintEdit, next: &LowpolyPaintEdit) -> Option<Option<LowpolyPaintEdit>> {
    use LowpolyPaintEdit::{Insert, Patch, Remove, Replace, Stroke};
    let applied = |layer: &LowpolyPaintLayer, edit: &LowpolyPaintEdit| -> Option<LowpolyPaintLayer> {
        let at_zero = match edit {
            Patch { patch, .. } => Patch { index: 0, patch: patch.clone() },
            Stroke { runs, .. } => Stroke { index: 0, runs: runs.clone() },
            _ => return None,
        };
        paint_edits_after(std::slice::from_ref(layer), &[at_zero]).ok()?.into_iter().next()
    };
    match (top, next) {
        (Insert { index, .. }, Remove { index: removed }) if index == removed => Some(None),
        (Insert { index, layer }, Patch { index: at, .. } | Stroke { index: at, .. }) if index == at => applied(layer, next).map(|layer| Some(Insert { index: *index, layer })),
        (Insert { index, .. }, Replace { index: at, layer }) if index == at => Some(Some(Insert { index: *index, layer: layer.clone() })),
        (Replace { index, layer }, Patch { index: at, .. } | Stroke { index: at, .. }) if index == at => applied(layer, next).map(|layer| Some(Replace { index: *index, layer })),
        (Replace { index, .. }, Replace { index: at, layer }) if index == at => Some(Some(Replace { index: *index, layer: layer.clone() })),
        (Replace { index, .. } | Patch { index, .. } | Stroke { index, .. }, Remove { index: removed }) if index == removed => Some(Some(Remove { index: *index })),
        (Patch { index, .. } | Stroke { index, .. }, Replace { index: at, layer }) if index == at => Some(Some(Replace { index: *index, layer: layer.clone() })),
        (Patch { index, patch: first }, Patch { index: at, patch: later }) if index == at => Some(Some(Patch { index: *index, patch: compose_layer_patches(first, later) })),
        (Stroke { index, runs: first }, Stroke { index: at, runs: later }) if index == at => Some(Some(Stroke { index: *index, runs: first.iter().chain(later).cloned().collect() })),
        (Remove { index }, Insert { index: at, layer }) if index == at => Some(Some(Replace { index: *index, layer: layer.clone() })),
        _ => None,
    }
}

/// ➕️ Canonical form of an edit sequence: adjacent edits of one layer fold (insert∘remove cancels, patches compose, strokes
/// concatenate, remove∘insert replaces).
pub fn canonical_paint_edits(edits: Vec<LowpolyPaintEdit>) -> Vec<LowpolyPaintEdit> {
    let mut stack: Vec<LowpolyPaintEdit> = Vec::new();
    for edit in edits {
        let mut pending = Some(edit);
        while let Some(current) = pending.take() {
            match stack.last().and_then(|top| merge_paint_edits(top, &current)) {
                Some(merged) => {
                    stack.pop();
                    pending = merged;
                }
                None => stack.push(current),
            }
        }
    }
    stack
}

/// ↩️ The edits that turn the layer list `edits` leaves behind back into `base`, read row by row off `base`: layers the edits
/// inserted are removed (highest index first), layers they removed are reinserted at their base index (lowest first), and every
/// surviving base layer they touched is written back — replaced whole, or only the metadata fields and pixel runs they touched.
fn paint_edits_inverse(base: &[LowpolyPaintLayer], edits: &[LowpolyPaintEdit]) -> Vec<LowpolyPaintEdit> {
    #[derive(Default)]
    struct Touch {
        replaced: bool,
        name: bool,
        visible: bool,
        opacity: bool,
        blend_mode: bool,
        runs: Vec<(u32, usize)>,
    }
    let mut slots: Vec<Option<usize>> = (0..base.len()).map(Some).collect();
    let mut touched: std::collections::BTreeMap<usize, Touch> = std::collections::BTreeMap::new();
    for edit in edits {
        match edit {
            LowpolyPaintEdit::Insert { index, .. } if *index as usize <= slots.len() => slots.insert(*index as usize, None),
            LowpolyPaintEdit::Remove { index } if (*index as usize) < slots.len() => {
                slots.remove(*index as usize);
            }
            LowpolyPaintEdit::Replace { index, .. } => {
                if let Some(Some(origin)) = slots.get(*index as usize) {
                    touched.entry(*origin).or_default().replaced = true;
                }
            }
            LowpolyPaintEdit::Patch { index, patch } => {
                if let Some(Some(origin)) = slots.get(*index as usize) {
                    let touch = touched.entry(*origin).or_default();
                    touch.name |= patch.name.is_some();
                    touch.visible |= patch.visible.is_some();
                    touch.opacity |= patch.opacity.is_some();
                    touch.blend_mode |= patch.blend_mode.is_some();
                }
            }
            LowpolyPaintEdit::Stroke { index, runs } => {
                if let Some(Some(origin)) = slots.get(*index as usize) {
                    touched.entry(*origin).or_default().runs.extend(runs.iter().map(|run| (run.offset, run.bytes.len())));
                }
            }
            _ => {}
        }
    }
    let survivors: Vec<usize> = slots.iter().flatten().copied().collect();
    let removals = slots.iter().enumerate().rev().filter(|(_, origin)| origin.is_none()).map(|(index, _)| LowpolyPaintEdit::Remove { index: index as u32 });
    let reinserts = base.iter().enumerate().filter(|(index, _)| !survivors.contains(index)).map(|(index, layer)| LowpolyPaintEdit::Insert { index: index as u32, layer: layer.clone() });
    let restores = touched.iter().filter(|(origin, _)| survivors.contains(origin)).flat_map(|(origin, touch)| {
        let layer = &base[*origin];
        let index = *origin as u32;
        let rows: Vec<LowpolyPaintEdit> = if touch.replaced {
            vec![LowpolyPaintEdit::Replace { index, layer: layer.clone() }]
        } else {
            let patch = LowpolyPaintLayerPatch { name: touch.name.then(|| layer.name.clone()), visible: touch.visible.then_some(layer.visible), opacity: touch.opacity.then_some(layer.opacity), blend_mode: touch.blend_mode.then(|| layer.blend_mode.clone()) };
            let metadata = (patch != LowpolyPaintLayerPatch::default()).then_some(LowpolyPaintEdit::Patch { index, patch });
            let pixels = layer.materialized_pixels();
            let runs: Vec<PixelRun> = touch.runs.iter().map(|(offset, length)| PixelRun { offset: *offset, bytes: pixels.get(*offset as usize..*offset as usize + *length).map(<[u8]>::to_vec).unwrap_or_default() }).collect();
            metadata.into_iter().chain((!runs.is_empty()).then_some(LowpolyPaintEdit::Stroke { index, runs })).collect()
        };
        rows
    });
    canonical_paint_edits(removals.chain(reinserts).chain(restores).collect())
}
//#endregion 🔖️PaintEdits

impl MutationDiff<LowpolySnapshot> for LowpolyDiff {}

impl protocol::ColdMutationDiff<LowpolySnapshot> for LowpolyDiff {
    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        self.objects = match (self.objects.take(), other.objects) {
            (Some(mut first), Some(later)) => {
                first.absorb(later);
                Some(first)
            }
            (first, later) => later.or(first),
        };
    }

    fn apply(&self, unpublished: &mut LowpolySnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let snapshot = &*unpublished;
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.objects {
            next.objects = delta.commit_onto(&next.objects, capability).map_err(|error| error.under(["objects"]))?;
        }
        *unpublished = next;
        Ok(())
    }
}

impl protocol::DiffAlgebra<LowpolySnapshot> for LowpolyDiff {
    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.objects.as_ref().is_none_or(LowpolyObjectsDelta::is_empty)
    }
}

impl protocol::ColdDiffAlgebra<LowpolySnapshot> for LowpolyDiff {
    fn inverse(&self, base: &LowpolySnapshot) -> Self {
        Self { schema: self.schema.as_ref().map(|_| base.schema.clone()), objects: self.objects.as_ref().map(|delta| delta.inverse(&base.objects)) }
    }
}

//#region 🔖️Builders
/// 🏗️ Objects-add field delta (inserted at `index`, or appended when `index` is past the end).
pub fn diff_objects_add(index: usize, item: LowpolyObject, base: &LowpolySnapshot) -> LowpolyDiff {
    LowpolyDiff { objects: Some(LowpolyObjectsDelta::insertion(index.min(base.objects.len()), item)), ..LowpolyDiff::default() }
}

/// 🏗️ Objects-remove field delta: the object at `index` of `base`.
pub fn diff_objects_remove(index: usize, base: &LowpolySnapshot) -> LowpolyDiff {
    LowpolyDiff { objects: Some(LowpolyObjectsDelta::removal(&base.objects, index)), ..LowpolyDiff::default() }
}

/// 🏗️ Objects-move field delta: the object `id` moves to position `to_index` of the after list (clamped).
pub fn diff_objects_move(id: &str, to_index: usize, base: &LowpolySnapshot) -> LowpolyDiff {
    let objects = match base.objects.iter().position(|existing| existing.id == id) {
        Some(from) => LowpolyObjectsDelta::relocation(&base.objects, from, to_index.min(base.objects.len() - 1)),
        None => LowpolyObjectsDelta::default(),
    };
    LowpolyDiff { objects: Some(objects), ..LowpolyDiff::default() }
}

fn diff_object_entry(id: String, entry: LowpolyObjectPatchEntry) -> LowpolyDiff {
    LowpolyDiff { objects: Some(LowpolyObjectsDelta::modification(id, entry)), ..LowpolyDiff::default() }
}

/// 🏗️ Objects-patch field delta.
pub fn diff_objects_patch(id: String, patch: LowpolyObjectPatch) -> LowpolyDiff {
    diff_object_entry(id, LowpolyObjectPatchEntry { patch, ..Default::default() })
}

fn diff_paint_edit(object_id: String, edit: LowpolyPaintEdit) -> LowpolyDiff {
    diff_object_entry(object_id, LowpolyObjectPatchEntry { paint_layers: Some(LowpolyPaintLayersDelta { edits: vec![edit] }), ..Default::default() })
}

/// 🏗️ Add-paint-layer field delta.
pub fn diff_add_paint_layer(object_id: String, index: usize, layer: LowpolyPaintLayer) -> LowpolyDiff {
    diff_paint_edit(object_id, LowpolyPaintEdit::Insert { index: index as u32, layer })
}

/// 🏗️ Remove-paint-layer field delta.
pub fn diff_remove_paint_layer(object_id: String, index: usize) -> LowpolyDiff {
    diff_paint_edit(object_id, LowpolyPaintEdit::Remove { index: index as u32 })
}

/// 🏗️ Patch-paint-layer field delta.
pub fn diff_patch_paint_layer(object_id: String, index: usize, patch: LowpolyPaintLayerPatch) -> LowpolyDiff {
    diff_paint_edit(object_id, LowpolyPaintEdit::Patch { index: index as u32, patch })
}

/// 🏗️ Paint-stroke field delta.
pub fn diff_paint_stroke(object_id: String, layer_index: usize, runs: Vec<PixelRun>) -> LowpolyDiff {
    diff_paint_edit(object_id, LowpolyPaintEdit::Stroke { index: layer_index as u32, runs })
}

/// 🏗️ Mesh-vertex field delta: the absolute positions the named vertices of the object's managed mesh move to, and the absolute
/// content of the Normal channels the same motion rewrote.
pub fn diff_mesh_vertices(object_id: String, mesh_vertices: Vec<LowpolyVertexPosition>, mesh_attributes: Vec<semio_s_artifact_lowpoly_lowpoly_model::managed_mesh::LowpolyMeshAttribute>) -> LowpolyDiff {
    diff_object_entry(object_id, LowpolyObjectPatchEntry { mesh_vertices, mesh_attributes, ..Default::default() })
}
//#endregion 🔖️Builders

use semio_s_artifact_lowpoly_lowpoly_model::PixelRun;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🧲️SelectionMotion
/// 🧲️ What one relative selection leaf (`move-selection`, `rotate-selection`, `scale-selection`) does to the vertices it
/// names on one object's persisted mesh: an offset, a turn by `angle` radians about `axis` through `pivot`, or per-axis
/// `factor`s about `pivot` — the parameters the gesture stated, never the geometry it left behind, so the leaf replays
/// onto whatever mesh the object holds by then (design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, §17.6).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LowpolySelectionMotion {
    Offset([f32; 3]),
    Turn { pivot: [f32; 3], axis: [f32; 3], angle: f32 },
    Stretch { pivot: [f32; 3], factor: [f32; 3] },
}
impl LowpolySelectionMotion {
    /// 🫥️ Whether the motion leaves every vertex where it is: no offset, no angle, unit factors.
    pub fn is_identity(&self) -> bool {
        match self {
            Self::Offset(offset) => *offset == [0.0; 3],
            Self::Turn { angle, .. } => *angle == 0.0,
            Self::Stretch { factor, .. } => *factor == [1.0; 3],
        }
    }

    /// 🚫️ Whether the motion's own parameters degenerate any mesh: a non-finite offset, axis, pivot, angle or factor, a zero-length
    /// axis, a zero factor.
    pub fn is_degenerate(&self) -> bool {
        match self {
            Self::Offset(offset) => offset.iter().any(|value| !value.is_finite()),
            Self::Turn { pivot, axis, angle } => axis.iter().chain(pivot.iter()).any(|value| !value.is_finite()) || !angle.is_finite() || axis.iter().map(|value| f64::from(*value)).fold(0.0_f64, f64::hypot) == 0.0,
            Self::Stretch { pivot, factor } => factor.iter().chain(pivot.iter()).any(|value| !value.is_finite()) || factor.contains(&0.0),
        }
    }

    /// 📍️ Where `point` ends up under this motion, computed in `f64` and narrowed to `f32` exactly as the mesh kernel does; `None`
    /// when the result is not finite.
    pub fn position_of(&self, point: [f32; 3]) -> Option<[f32; 3]> {
        let point = point.map(f64::from);
        let moved: [f64; 3] = match *self {
            Self::Offset(offset) => std::array::from_fn(|axis| point[axis] + f64::from(offset[axis])),
            Self::Turn { pivot, axis, angle } => {
                let axis = axis.map(f64::from);
                let length = axis[0].hypot(axis[1]).hypot(axis[2]);
                let [x, y, z] = axis.map(|value| value / length);
                let pivot = pivot.map(f64::from);
                let (c, s) = (f64::from(angle).cos(), f64::from(angle).sin());
                let t = 1.0 - c;
                let [px, py, pz] = std::array::from_fn::<_, 3, _>(|axis| point[axis] - pivot[axis]);
                let rx = (t * x * x + c) * px + (t * x * y - s * z) * py + (t * x * z + s * y) * pz;
                let ry = (t * x * y + s * z) * px + (t * y * y + c) * py + (t * y * z - s * x) * pz;
                let rz = (t * x * z - s * y) * px + (t * y * z + s * x) * py + (t * z * z + c) * pz;
                [pivot[0] + rx, pivot[1] + ry, pivot[2] + rz]
            }
            Self::Stretch { pivot, factor } => std::array::from_fn(|axis| f64::from(pivot[axis]) + (point[axis] - f64::from(pivot[axis])) * f64::from(factor[axis])),
        };
        let narrowed = moved.map(|value| value as f32);
        narrowed.iter().all(|value| value.is_finite()).then_some(narrowed)
    }

    /// 🏷️ The history label of moving `vertices` (every vertex when 0) of `object_id`: "Move 2 vertices of "obj-1" by
    /// (0.5, 0, 0)" / "2 Eckpunkte von "obj-1" um (0,5; 0; 0) verschieben", and the rotate and scale twins.
    pub fn label(&self, object_id: &str, vertices: usize) -> semio_framework_ui_locale::LocalizedLabel {
        let number = |value: f32, german: bool| {
            let text = format!("{}", (f64::from(value) * 1_000.0).round() / 1_000.0);
            if german { text.replace('.', ",") } else { text }
        };
        let triple = |values: &[f32; 3], german: bool| values.iter().map(|value| number(*value, german)).collect::<Vec<_>>().join(if german { "; " } else { ", " });
        let (en, de) = match vertices {
            0 => (format!("\"{object_id}\""), format!("\"{object_id}\"")),
            1 => (format!("1 vertex of \"{object_id}\""), format!("1 Eckpunkt von \"{object_id}\"")),
            count => (format!("{count} vertices of \"{object_id}\""), format!("{count} Eckpunkte von \"{object_id}\"")),
        };
        match self {
            Self::Offset(offset) => semio_framework_ui_locale::LocalizedLabel::native(&format!("Move {en} by ({})", triple(offset, false)), &format!("{de} um ({}) verschieben", triple(offset, true))),
            Self::Turn { axis, angle, .. } => {
                let degrees = angle.to_degrees();
                semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate {en} by {}° about ({})", number(degrees, false), triple(axis, false)), &format!("{de} um die Achse ({}) um {}° drehen", triple(axis, true), number(degrees, true)))
            }
            Self::Stretch { factor, .. } => semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale {en} by ({})", triple(factor, false)), &format!("{de} mit ({}) skalieren", triple(factor, true))),
        }
    }
}
impl LowpolySelectionMotion {
    /// 🧭️ The matrix the mesh kernel's `map_vertices` applies to the Normal-semantic attribute channels of the selected vertices:
    /// the rotation for a turn, the inverse scale for a stretch; `None` for an offset, which leaves normals alone.
    fn normal_matrix(&self) -> Option<[[f64; 3]; 3]> {
        match *self {
            Self::Offset(_) => None,
            Self::Turn { axis, angle, .. } => {
                let axis = axis.map(f64::from);
                let length = axis[0].hypot(axis[1]).hypot(axis[2]);
                let [x, y, z] = axis.map(|value| value / length);
                let (c, s) = (f64::from(angle).cos(), f64::from(angle).sin());
                let t = 1.0 - c;
                Some([[t * x * x + c, t * x * y - s * z, t * x * z + s * y], [t * x * y + s * z, t * y * y + c, t * y * z - s * x], [t * x * z - s * y, t * y * z + s * x, t * z * z + c]])
            }
            Self::Stretch { factor, .. } => Some([[1.0 / f64::from(factor[0]), 0.0, 0.0], [0.0, 1.0 / f64::from(factor[1]), 0.0], [0.0, 0.0, 1.0 / f64::from(factor[2])]]),
        }
    }
}
/// 🧭️ The Normal-semantic attribute channels a relative selection motion rewrites, as whole absolute channels: every sample of every
/// Normal channel whose vertex is among `selected` is multiplied by the motion's normal matrix and normalised, the others stay —
/// exactly the mesh kernel's `map_vertices` (`🧰️framework/🔨️modules/🧊️3d/🥽️mesh`), read from the BASE state and the motion alone.
/// Only the channels whose content actually changes are returned; `None` when the motion degenerates a channel.
pub fn lowpoly_selection_normal_channels(state: &semio_s_artifact_lowpoly_lowpoly_model::managed_mesh::LowpolyMeshState, selected: &[u32], motion: &LowpolySelectionMotion) -> Option<Vec<semio_s_artifact_lowpoly_lowpoly_model::managed_mesh::LowpolyMeshAttribute>> {
    let Some(matrix) = motion.normal_matrix() else {
        return Some(Vec::new());
    };
    let ids: std::collections::BTreeSet<usize> = selected.iter().map(|id| *id as usize).collect();
    let mut channels = Vec::new();
    for attribute in state.attributes.iter().filter(|attribute| attribute.semantic == semio_s_artifact_lowpoly_lowpoly_model::managed_mesh::LowpolyMeshAttributeSemantic::Normal) {
        let length = attribute.indices.as_ref().map_or(attribute.values.len(), Vec::len);
        let mut values = Vec::new();
        let mut indices = Vec::new();
        let mut samples: std::collections::BTreeMap<(u32, bool), u32> = std::collections::BTreeMap::new();
        for domain in 0..length {
            let vertex = if attribute.domain == semio_s_artifact_lowpoly_lowpoly_model::managed_mesh::LowpolyMeshAttributeDomain::Corner { state.halfedges.get(domain)?.vertex as usize } else { domain };
            let affected = ids.contains(&vertex);
            let source = attribute.indices.as_ref().map_or(domain as u32, |indices| indices[domain]);
            let sample = match samples.get(&(source, affected)) {
                Some(sample) => *sample,
                None => {
                    let original = attribute.values.get(source as usize)?.as_array()?;
                    let mut components = [0.0_f64; 3];
                    for axis in 0..3 {
                        components[axis] = original.get(axis)?.as_f64()?;
                    }
                    if affected {
                        components = [0, 1, 2].map(|axis| matrix[axis][0] * components[0] + matrix[axis][1] * components[1] + matrix[axis][2] * components[2]);
                        let norm = components[0].hypot(components[1]).hypot(components[2]);
                        components = components.map(|value| value / norm);
                        if components.iter().any(|value| !value.is_finite() || !(*value as f32).is_finite()) {
                            return None;
                        }
                    }
                    let sample = values.len() as u32;
                    values.push(semio_framework_value::DslValue::Array(components.into_iter().map(semio_framework_value::DslValue::float).collect()));
                    samples.insert((source, affected), sample);
                    sample
                }
            };
            indices.push(sample);
        }
        let rewritten = semio_s_artifact_lowpoly_lowpoly_model::managed_mesh::LowpolyMeshAttribute { name: attribute.name.clone(), domain: attribute.domain, semantic: attribute.semantic, interpolation: attribute.interpolation, values, indices: Some(indices) };
        if rewritten != *attribute {
            channels.push(rewritten);
        }
    }
    Some(channels)
}
/// 🪪️ The vertex-set half of every selection leaf's own invariant: each vertex named at most once.
pub fn lowpoly_selection_vertex_violation(vertex_ids: &[u32]) -> Option<String> {
    let mut seen = std::collections::BTreeSet::new();
    vertex_ids.iter().find(|id| !seen.insert(**id)).map(|twice| format!("Vertex {twice} is named more than once."))
}
/// 📍️ The `(vertex, new position, base position)` rows of every vertex among `present` (ascending, each once) the invariant-clean
/// `motion` actually moves, computed from the BASE vertex positions and the motion alone; `None` when the motion degenerates the mesh.
pub fn lowpoly_selection_moves(state: &semio_s_artifact_lowpoly_lowpoly_model::managed_mesh::LowpolyMeshState, present: &[u32], motion: &LowpolySelectionMotion) -> Option<Vec<(u32, [f32; 3], [f32; 3])>> {
    if motion.is_degenerate() {
        return None;
    }
    let sorted: std::collections::BTreeSet<u32> = present.iter().copied().collect();
    let rows = sorted.into_iter().map(|id| {
        let base = state.vertices[id as usize].position;
        motion.position_of(base).map(|position| (id, position, base))
    });
    Some(rows.collect::<Option<Vec<_>>>()?.into_iter().filter(|(_, position, base)| position.map(f32::to_bits) != base.map(f32::to_bits)).collect())
}
/// 🔺️ The sparse diff of applying an invariant-clean `motion` to `vertex_ids` (every vertex when empty) of `object_id`'s
/// persisted mesh: one absolute position row per vertex the motion actually moves, computed from the BASE vertex positions and
/// the motion alone — no mesh is cloned or edited, and the mesh handle is re-derived by the central applier. Each leaf's diff
/// refuses its own payload invariant first; here Fatal `invariant` is a motion that degenerates the mesh, Error `target-missing`
/// an absent object, mesh or every named vertex, Warning `partial` the named vertices it skips and Warning `no-op` a motion that
/// moves nothing.
pub fn lowpoly_selection_motion_diff(base: &LowpolySnapshot, object_id: &str, vertex_ids: &[u32], motion: &LowpolySelectionMotion) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(object) = base.objects.iter().find(|object| object.id == object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{object_id}\" does not exist."), [object_id.to_string()]);
    };
    let Some(state) = object.mesh.as_ref().and(object.mesh_state.as_ref()) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{object_id}\" carries no mesh."), [object_id.to_string()]);
    };
    let count = u32::try_from(state.vertices.len()).unwrap_or(u32::MAX);
    let (present, skipped): (Vec<u32>, Vec<u32>) = if vertex_ids.is_empty() { ((0..count).collect(), Vec::new()) } else { vertex_ids.iter().partition(|id| **id < count) };
    if present.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} named vertices exists on object \"{object_id}\".", vertex_ids.len()), [object_id.to_string()]);
    }
    let partial = (!skipped.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} named vertices skipped (not on object \"{object_id}\"): {skipped:?}", skipped.len(), vertex_ids.len())).at([object_id.to_string()]));
    let no_op = || protocol::MutationMessage::warning("mutation.no-op", format!("The motion moves no vertex of object \"{object_id}\".")).at([object_id.to_string()]);
    if motion.is_identity() {
        return protocol::MutationOutcome::new(LowpolyDiff::default()).absorb_messages(partial.into_iter().chain([no_op()]));
    }
    let degenerate = || protocol::MutationOutcome::fatal("mutation.invariant", format!("The motion degenerates the mesh of object \"{object_id}\"."), [object_id.to_string()]);
    let Some(moved) = lowpoly_selection_moves(state, &present, motion) else {
        return degenerate();
    };
    let Some(channels) = lowpoly_selection_normal_channels(state, &present, motion) else {
        return degenerate();
    };
    let rows: Vec<LowpolyVertexPosition> = moved.into_iter().map(|(vertex, position, _)| LowpolyVertexPosition { vertex, position }).collect();
    if rows.is_empty() && channels.is_empty() {
        return protocol::MutationOutcome::new(LowpolyDiff::default()).absorb_messages(partial.into_iter().chain([no_op()]));
    }
    protocol::MutationOutcome::new(crate::diff_mesh_vertices(object_id.to_string(), rows, channels)).absorb_messages(partial)
}
//#endregion 🧲️SelectionMotion

/// 🔺️ The diff facet's schema leaves (rust, typescript, graphql, json schema, proto).
pub const DIFF_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto"), };
