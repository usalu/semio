//! 🧬️ Lowpoly diff schema — sparse edits over the artifact: a positional object row delta whose patches carry the object's own
//! field patch, an ordered list of paint-layer edits and an ordered list of selection motions applied to its mesh.

use crate::{LowpolyObject, LowpolyPaintLayer, LowpolySnapshot};
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
    pub LowpolyObjectsDelta { removal: LowpolyObjectRemoval, insertion: LowpolyObjectInsertion, relocation: LowpolyObjectRelocation, modification: LowpolyObjectsModification, row: LowpolyObject, patch: LowpolyObjectPatchEntry, key: id, values_only }
}

/// 🩹 One patched object entry: the object's own field patch, then its paint-layer edits, then its mesh vertex positions and the Normal channels they rewrote, in that order.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct LowpolyObjectPatchEntry {
    pub patch: LowpolyObjectPatch,
    pub paint_layers: Option<LowpolyPaintLayersDelta>,
    pub mesh_vertices: Vec<LowpolyVertexPosition>,
    pub mesh_attributes: Vec<crate::LowpolyMeshAttribute>,
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
pub use crate::LowpolyObjectPatch;
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
            let state = crate::LowpolyMeshState::from_mesh(mesh);
            if self.patch.mesh.is_none() {
                row.mesh = Some(crate::managed_mesh_child_handle(&row.id, &state));
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
        let mut mesh_attributes: Vec<crate::LowpolyMeshAttribute> = if replaced { Vec::new() } else { self.mesh_attributes.iter().filter(|row| later.mesh_attributes.iter().all(|overwritten| overwritten.name != row.name)).cloned().collect() };
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

impl MutationDiff<LowpolySnapshot> for LowpolyDiff {
    fn apply(&self, snapshot: &LowpolySnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<LowpolySnapshot> {
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.objects {
            next.objects = delta.commit_onto(&next.objects, capability).map_err(|error| error.under(["objects"]))?;
        }
        Ok(next)
    }

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
}

impl protocol::DiffAlgebra<LowpolySnapshot> for LowpolyDiff {
    fn inverse(&self, base: &LowpolySnapshot) -> Self {
        Self { schema: self.schema.as_ref().map(|_| base.schema.clone()), objects: self.objects.as_ref().map(|delta| delta.inverse(&base.objects)) }
    }

    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.objects.as_ref().is_none_or(LowpolyObjectsDelta::is_empty)
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
pub fn diff_mesh_vertices(object_id: String, mesh_vertices: Vec<LowpolyVertexPosition>, mesh_attributes: Vec<crate::LowpolyMeshAttribute>) -> LowpolyDiff {
    diff_object_entry(object_id, LowpolyObjectPatchEntry { mesh_vertices, mesh_attributes, ..Default::default() })
}
//#endregion 🔖️Builders

use crate::schema::PixelRun;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
