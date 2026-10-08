//! 🧬️ Lowpoly diff schema — sparse edits over the artifact: an id-keyed object row delta whose patches carry the object's own
//! field patch, an ordered list of paint-layer edits and an ordered list of selection motions applied to its mesh.

use crate::{LowpolyObject, LowpolyObjectPatch, LowpolyPaintLayer, LowpolySnapshot};
use framework_schema::ArtifactSchema;
use protocol::{MutationDiff, Patchable};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the lowpoly artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff). There is no
/// whole-document slot: objects are an id-keyed row delta and every object patch is field-sparse.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
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
/// 🧩 Identified-collection delta for `objects`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct LowpolyObjectsDelta {
    pub added: Vec<LowpolyObject>,
    pub removed: Vec<String>,
    pub patched: Vec<LowpolyObjectPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched object entry: the object's own field patch, then its paint-layer edits, then its mesh motions, in that order.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct LowpolyObjectPatchEntry {
    pub id: String,
    pub patch: LowpolyObjectPatch,
    pub paint_layers: Option<LowpolyPaintLayersDelta>,
    pub mesh_motions: Vec<LowpolyMeshMotion>,
}

/// 🖌️ Ordered paint-layer edits under an object patch (layers are addressed by index and carry no identity).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct LowpolyPaintLayersDelta {
    pub edits: Vec<LowpolyPaintEdit>,
}

/// ✏️ One edit of an object's paint-layer list; edits apply in order.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "op", rename_all = "camelCase")]
pub enum LowpolyPaintEdit {
    Insert { index: u32, layer: LowpolyPaintLayer },
    Remove { index: u32 },
    Replace { index: u32, layer: LowpolyPaintLayer },
    Patch { index: u32, patch: LowpolyPaintLayerPatch },
    Stroke { index: u32, runs: Vec<PixelRun> },
}

/// 🧲️ One selection motion applied to the named vertices of an object's mesh, replayed by the central applier.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "motion", rename_all = "camelCase")]
pub enum LowpolyMeshMotion {
    Offset { vertex_ids: Vec<u32>, offset: [f32; 3] },
    Turn { vertex_ids: Vec<u32>, pivot: [f32; 3], axis: [f32; 3], angle: f32 },
    Stretch { vertex_ids: Vec<u32>, pivot: [f32; 3], factor: [f32; 3] },
}

/// 🩹 Paint-layer metadata patch.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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

//#region 🧺️KeyedDelta
/// 🧺️ Id-keyed ordered-collection delta (`added`/`removed`/`patched`/`reordered`) and its algebra: apply, composition (create∘delete
/// cancels, delete∘create replaces, patch∘patch composes, patch∘create folds), negative delta and state delta.
pub trait KeyedDelta: Sized {
    type Row: Clone;
    type Patch: Clone;
    fn added(&self) -> &[Self::Row];
    fn removed(&self) -> &[String];
    fn patched(&self) -> &[Self::Patch];
    fn reordered(&self) -> Option<&[String]>;
    fn assemble(added: Vec<Self::Row>, removed: Vec<String>, patched: Vec<Self::Patch>, reordered: Option<Vec<String>>) -> Self;
    fn row_key(row: &Self::Row) -> &str;
    fn patch_key(patch: &Self::Patch) -> &str;
    fn patch_fold(patch: &Self::Patch, row: &mut Self::Row) -> Result<(), protocol::MutationApplyError>;
    fn patch_compose(first: &Self::Patch, later: &Self::Patch) -> Self::Patch;
    fn patch_inverse(patch: &Self::Patch, base: &Self::Row) -> Self::Patch;
    fn patch_between(base: &Self::Row, other: &Self::Row) -> Option<Self::Patch>;
    fn patch_is_empty(patch: &Self::Patch) -> bool;
}

fn keyed_error(code: &str, message: &str, at: [&str; 2]) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at(at)
}

pub fn keyed_apply<D: KeyedDelta>(rows: &[D::Row], delta: &D) -> Result<Vec<D::Row>, protocol::MutationApplyError> {
    let key = D::row_key;
    for (index, id) in delta.removed().iter().enumerate() {
        if delta.removed()[..index].contains(id) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is removed more than once", ["removed", &index.to_string()]));
        }
        if !rows.iter().any(|row| key(row) == id) {
            return Err(keyed_error("mutation.apply.missing-target", "removed row does not exist", ["removed", &index.to_string()]));
        }
    }
    let mut next: Vec<D::Row> = rows.iter().filter(|row| !delta.removed().iter().any(|id| id == key(row))).cloned().collect();
    for (index, row) in delta.added().iter().enumerate() {
        if next.iter().any(|existing| key(existing) == key(row)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "added row identity already exists", ["added", &index.to_string()]));
        }
        next.push(row.clone());
    }
    for (index, patch) in delta.patched().iter().enumerate() {
        if delta.patched()[..index].iter().any(|earlier| D::patch_key(earlier) == D::patch_key(patch)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is patched more than once", ["patched", &index.to_string()]));
        }
        let row = next.iter_mut().find(|row| key(row) == D::patch_key(patch)).ok_or_else(|| keyed_error("mutation.apply.missing-target", "patched row does not exist", ["patched", &index.to_string()]))?;
        D::patch_fold(patch, row).map_err(|error| error.under(["patched".to_string(), index.to_string()]))?;
    }
    let Some(order) = delta.reordered() else { return Ok(next) };
    if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|row| key(row) == id)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered".to_string()]));
    }
    Ok(order.iter().filter_map(|id| next.iter().find(|row| key(row) == id).cloned()).collect())
}

fn canonical<D: KeyedDelta>(mut added: Vec<D::Row>, mut removed: Vec<String>, mut patched: Vec<D::Patch>, reordered: Option<Vec<String>>) -> D {
    if let Some(order) = &reordered {
        added.sort_by_key(|row| order.iter().position(|id| id == D::row_key(row)).unwrap_or(usize::MAX));
    }
    let reordered = reordered.filter(|order| {
        let tail = added.len();
        !(order.len() <= tail + 1 && order.len() >= tail && order[order.len() - tail..].iter().map(String::as_str).eq(added.iter().map(D::row_key)))
    });
    removed.sort();
    removed.dedup();
    patched.retain(|patch| !D::patch_is_empty(patch));
    patched.sort_by(|left, right| D::patch_key(left).cmp(D::patch_key(right)));
    D::assemble(added, removed, patched, reordered)
}

/// ➕️ Normal form of `first` then `later`: create∘delete cancels, delete∘create replaces, patch∘patch composes, patch∘create folds.
pub fn keyed_absorb<D: KeyedDelta>(first: &D, later: &D) -> D {
    let mut added: Vec<D::Row> = first.added().to_vec();
    let mut removed: Vec<String> = first.removed().to_vec();
    let mut patched: Vec<D::Patch> = Vec::new();
    for patch in first.patched() {
        match added.iter_mut().find(|row| D::row_key(row) == D::patch_key(patch)) {
            Some(row) => {
                let _ = D::patch_fold(patch, row);
            }
            None => patched.push(patch.clone()),
        }
    }
    for id in later.removed() {
        if let Some(position) = added.iter().position(|row| D::row_key(row) == id) {
            added.remove(position);
        } else {
            patched.retain(|patch| D::patch_key(patch) != id);
            if !removed.contains(id) {
                removed.push(id.clone());
            }
        }
    }
    added.extend(later.added().iter().cloned());
    for patch in later.patched() {
        let key = D::patch_key(patch);
        if let Some(row) = added.iter_mut().find(|row| D::row_key(row) == key) {
            let _ = D::patch_fold(patch, row);
        } else if let Some(existing) = patched.iter_mut().find(|existing| D::patch_key(existing) == key) {
            *existing = D::patch_compose(existing, patch);
        } else {
            patched.push(patch.clone());
        }
    }
    let reordered = match (later.reordered(), first.reordered()) {
        (Some(order), _) => Some(order.to_vec()),
        (None, Some(order)) => Some(order.iter().filter(|id| !later.removed().contains(id)).cloned().chain(later.added().iter().map(|row| D::row_key(row).to_string())).collect()),
        (None, None) => None,
    };
    canonical::<D>(added, removed, patched, reordered)
}

fn ids_after<D: KeyedDelta>(base: &[String], delta: &D) -> Vec<String> {
    match delta.reordered() {
        Some(order) => order.to_vec(),
        None => base.iter().filter(|id| !delta.removed().contains(id)).cloned().chain(delta.added().iter().map(|row| D::row_key(row).to_string())).collect(),
    }
}

/// 🔁️ The negative delta: removes what `delta` added, restores what it removed, undoes its patches, restores the base order.
pub fn keyed_inverse<D: KeyedDelta>(delta: &D, base: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = delta.added().iter().map(|row| D::row_key(row).to_string()).collect();
    let added: Vec<D::Row> = delta.removed().iter().filter_map(|id| base.iter().find(|row| D::row_key(row) == id).cloned()).collect();
    let patched: Vec<D::Patch> = delta
        .patched()
        .iter()
        .filter(|patch| !removed.iter().any(|id| id == D::patch_key(patch)))
        .filter_map(|patch| base.iter().find(|row| D::row_key(row) == D::patch_key(patch)).map(|row| D::patch_inverse(patch, row)))
        .collect();
    let after = ids_after(&base_ids, delta);
    let natural: Vec<String> = after.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != base_ids).then_some(base_ids);
    canonical::<D>(added, removed, patched, reordered)
}

/// 🧭️ The delta turning `base` into `other` (sync/import only).
pub fn keyed_between<D: KeyedDelta>(base: &[D::Row], other: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let other_ids: Vec<String> = other.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = base_ids.iter().filter(|id| !other_ids.contains(id)).cloned().collect();
    let added: Vec<D::Row> = other.iter().filter(|row| !base_ids.iter().any(|id| id == D::row_key(row))).cloned().collect();
    let patched: Vec<D::Patch> = other.iter().filter_map(|row| base.iter().find(|candidate| D::row_key(candidate) == D::row_key(row)).and_then(|candidate| D::patch_between(candidate, row))).collect();
    let natural: Vec<String> = base_ids.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != other_ids).then_some(other_ids);
    canonical::<D>(added, removed, patched, reordered)
}

pub fn keyed_is_empty<D: KeyedDelta>(delta: &D) -> bool {
    delta.added().is_empty() && delta.removed().is_empty() && delta.patched().iter().all(D::patch_is_empty) && delta.reordered().is_none()
}
//#endregion 🧺️KeyedDelta

impl KeyedDelta for LowpolyObjectsDelta {
    type Row = LowpolyObject;
    type Patch = LowpolyObjectPatchEntry;
    fn added(&self) -> &[LowpolyObject] {
        &self.added
    }
    fn removed(&self) -> &[String] {
        &self.removed
    }
    fn patched(&self) -> &[LowpolyObjectPatchEntry] {
        &self.patched
    }
    fn reordered(&self) -> Option<&[String]> {
        self.reordered.as_deref()
    }
    fn assemble(added: Vec<LowpolyObject>, removed: Vec<String>, patched: Vec<LowpolyObjectPatchEntry>, reordered: Option<Vec<String>>) -> Self {
        Self { added, removed, patched, reordered }
    }
    fn row_key(row: &LowpolyObject) -> &str {
        &row.id
    }
    fn patch_key(patch: &LowpolyObjectPatchEntry) -> &str {
        &patch.id
    }
    fn patch_fold(patch: &LowpolyObjectPatchEntry, row: &mut LowpolyObject) -> Result<(), protocol::MutationApplyError> {
        row.apply_patch(&patch.patch);
        if let Some(paint) = &patch.paint_layers {
            row.paint_layers = paint_edits_after(&row.paint_layers, &paint.edits).map_err(|error| error.under(["paintLayers"]))?;
        }
        if !patch.mesh_motions.is_empty() {
            let state = row.mesh_state.clone().ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "mesh motion needs the object's managed mesh").at(["meshMotions"]))?;
            let mut mesh = state.into_mesh().map_err(|_| protocol::MutationApplyError::new("mutation.apply.invalid-base", "the object's managed mesh does not decode").at(["meshMotions"]))?;
            for (index, motion) in patch.mesh_motions.iter().enumerate() {
                protocol::apply_diff(&motion, &mut mesh).map_err(|_| protocol::MutationApplyError::new("mutation.apply.invalid-motion", "the selection motion degenerates the mesh").at(["meshMotions".to_string(), index.to_string()]))?;
            }
            row.mesh_state = Some(crate::LowpolyMeshState::from_mesh(mesh));
        }
        Ok(())
    }
    fn patch_compose(first: &LowpolyObjectPatchEntry, later: &LowpolyObjectPatchEntry) -> LowpolyObjectPatchEntry {
        let patch = compose_object_patch(&first.patch, &later.patch);
        let mut mesh_motions = if later.patch.mesh_state.is_some() { Vec::new() } else { first.mesh_motions.clone() };
        mesh_motions.extend(later.mesh_motions.iter().cloned());
        let paint_layers = match (&first.paint_layers, &later.paint_layers) {
            (None, None) => None,
            (first, later) => Some(LowpolyPaintLayersDelta { edits: canonical_paint_edits(first.iter().chain(later.iter()).flat_map(|delta| delta.edits.iter().cloned()).collect()) }),
        };
        LowpolyObjectPatchEntry { id: first.id.clone(), patch, paint_layers, mesh_motions }
    }
    fn patch_inverse(patch: &LowpolyObjectPatchEntry, base: &LowpolyObject) -> LowpolyObjectPatchEntry {
        let restores_mesh = !patch.mesh_motions.is_empty();
        LowpolyObjectPatchEntry {
            id: patch.id.clone(),
            patch: LowpolyObjectPatch {
                name: patch.patch.name.as_ref().map(|_| base.name.clone()),
                smooth_shading: patch.patch.smooth_shading.map(|_| base.smooth_shading),
                position: patch.patch.position.map(|_| base.transform.position),
                rotation: patch.patch.rotation.map(|_| base.transform.rotation),
                scale: patch.patch.scale.map(|_| base.transform.scale),
                mesh: (restores_mesh || patch.patch.mesh.is_some()).then(|| base.mesh.clone()),
                mesh_content: (restores_mesh || patch.patch.mesh_content.is_some()).then(|| base.mesh_content.clone()),
                mesh_state: (restores_mesh || patch.patch.mesh_state.is_some()).then(|| base.mesh_state.clone()),
            },
            paint_layers: patch.paint_layers.as_ref().map(|paint| LowpolyPaintLayersDelta { edits: paint_edits_inverse(&base.paint_layers, &paint.edits) }),
            mesh_motions: Vec::new(),
        }
    }
    fn patch_between(base: &LowpolyObject, other: &LowpolyObject) -> Option<LowpolyObjectPatchEntry> {
        let edits = paint_edits_replacing(&base.paint_layers, &other.paint_layers);
        let entry = LowpolyObjectPatchEntry { id: other.id.clone(), patch: base.diff_patch(other).unwrap_or_default(), paint_layers: (!edits.is_empty()).then_some(LowpolyPaintLayersDelta { edits }), mesh_motions: Vec::new() };
        (!Self::patch_is_empty(&entry)).then_some(entry)
    }
    fn patch_is_empty(patch: &LowpolyObjectPatchEntry) -> bool {
        patch.patch == LowpolyObjectPatch::default() && patch.paint_layers.as_ref().is_none_or(|paint| paint.edits.is_empty()) && patch.mesh_motions.is_empty()
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

//#region 🔖️MeshMotions
impl LowpolyMeshMotion {
    /// 🧲️ Applies the motion to the vertices it names through the mesh kernel.
    pub fn apply(&self, mesh: &mut semio_framework_3d::mesh::HalfedgeMesh) -> semio_framework_3d::mesh::MeshResult<()> {
        use semio_framework_3d::mesh::{Vec3, VertexId};
        let ids = |vertex_ids: &[u32]| vertex_ids.iter().map(|id| VertexId(*id)).collect::<Vec<_>>();
        match self {
            Self::Offset { vertex_ids, offset } => mesh.move_vertices(&ids(vertex_ids), Vec3(*offset)),
            Self::Turn { vertex_ids, pivot, axis, angle } => mesh.rotate_vertices(&ids(vertex_ids), Vec3(*axis), *angle, Vec3(*pivot)),
            Self::Stretch { vertex_ids, pivot, factor } => mesh.scale_vertices(&ids(vertex_ids), Vec3(*factor), Vec3(*pivot)),
        }
    }
}
//#endregion 🔖️MeshMotions

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
    let mut next = layers.to_vec();
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

fn paint_edits_inverse(base: &[LowpolyPaintLayer], edits: &[LowpolyPaintEdit]) -> Vec<LowpolyPaintEdit> {
    let mut state = base.to_vec();
    let mut inverse = Vec::new();
    for edit in edits {
        match edit {
            LowpolyPaintEdit::Insert { index, layer } if *index as usize <= state.len() => {
                state.insert(*index as usize, layer.clone());
                inverse.push(LowpolyPaintEdit::Remove { index: *index });
            }
            LowpolyPaintEdit::Remove { index } if (*index as usize) < state.len() => {
                let old = state.remove(*index as usize);
                inverse.push(LowpolyPaintEdit::Insert { index: *index, layer: old });
            }
            LowpolyPaintEdit::Replace { index, layer } if (*index as usize) < state.len() => {
                let old = std::mem::replace(&mut state[*index as usize], layer.clone());
                inverse.push(LowpolyPaintEdit::Replace { index: *index, layer: old });
            }
            LowpolyPaintEdit::Patch { index, patch } if (*index as usize) < state.len() => {
                let old = &mut state[*index as usize];
                inverse.push(LowpolyPaintEdit::Patch {
                    index: *index,
                    patch: LowpolyPaintLayerPatch { name: patch.name.as_ref().map(|_| old.name.clone()), visible: patch.visible.map(|_| old.visible), opacity: patch.opacity.map(|_| old.opacity), blend_mode: patch.blend_mode.as_ref().map(|_| old.blend_mode.clone()) },
                });
                patch_layer(old, patch);
            }
            LowpolyPaintEdit::Stroke { index, runs } if (*index as usize) < state.len() => {
                let layer = &mut state[*index as usize];
                let pixels = layer.materialized_pixels();
                let restored = runs.iter().map(|run| PixelRun { offset: run.offset, bytes: pixels.get(run.offset as usize..run.offset as usize + run.bytes.len()).map(<[u8]>::to_vec).unwrap_or_default() }).collect();
                inverse.push(LowpolyPaintEdit::Stroke { index: *index, runs: restored });
                let _ = stroke_layer(layer, runs);
            }
            _ => {}
        }
    }
    inverse.reverse();
    canonical_paint_edits(inverse)
}

/// 🧭️ The edits turning the layer list `base` into `replacement`: layers set pairwise (metadata as a patch, pixels as a replace),
/// the surplus removed or appended.
pub fn paint_edits_replacing(base: &[LowpolyPaintLayer], replacement: &[LowpolyPaintLayer]) -> Vec<LowpolyPaintEdit> {
    let common = base.len().min(replacement.len());
    let mut edits = Vec::new();
    for index in 0..common {
        let (old, new) = (&base[index], &replacement[index]);
        if old == new {
            continue;
        }
        if old.pixels == new.pixels {
            edits.push(LowpolyPaintEdit::Patch {
                index: index as u32,
                patch: LowpolyPaintLayerPatch { name: (old.name != new.name).then(|| new.name.clone()), visible: (old.visible != new.visible).then_some(new.visible), opacity: (old.opacity != new.opacity).then_some(new.opacity), blend_mode: (old.blend_mode != new.blend_mode).then(|| new.blend_mode.clone()) },
            });
        } else {
            edits.push(LowpolyPaintEdit::Replace { index: index as u32, layer: new.clone() });
        }
    }
    edits.extend((common..base.len()).map(|_| LowpolyPaintEdit::Remove { index: common as u32 }));
    edits.extend((common..replacement.len()).map(|index| LowpolyPaintEdit::Insert { index: index as u32, layer: replacement[index].clone() }));
    canonical_paint_edits(edits)
}
//#endregion 🔖️PaintEdits

impl MutationDiff<LowpolySnapshot> for LowpolyDiff {
    fn apply(&self, snapshot: &LowpolySnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<LowpolySnapshot> {
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.objects {
            next.objects = keyed_apply(&next.objects, delta).map_err(|error| error.under(["objects"]))?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        self.objects = match (self.objects.take(), other.objects) {
            (Some(first), Some(later)) => Some(keyed_absorb(&first, &later)),
            (first, later) => later.or(first),
        };
    }
}

impl protocol::DiffAlgebra<LowpolySnapshot> for LowpolyDiff {
    fn inverse(&self, base: &LowpolySnapshot) -> Self {
        Self { schema: self.schema.as_ref().map(|_| base.schema.clone()), objects: self.objects.as_ref().map(|delta| keyed_inverse(delta, &base.objects)) }
    }

    fn between(base: &LowpolySnapshot, other: &LowpolySnapshot) -> Self {
        let objects = keyed_between::<LowpolyObjectsDelta>(&base.objects, &other.objects);
        Self { schema: (base.schema != other.schema).then(|| other.schema.clone()), objects: (!keyed_is_empty(&objects)).then_some(objects) }
    }

    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.objects.as_ref().is_none_or(keyed_is_empty)
    }
}

//#region 🔖️Builders
/// 🏗️ Objects-add field delta (inserted at `index`, or appended when `index` is past the end).
pub fn diff_objects_add(index: usize, item: LowpolyObject, base: &LowpolySnapshot) -> LowpolyDiff {
    let reordered = (index < base.objects.len()).then(|| {
        let mut order: Vec<String> = base.objects.iter().map(|object| object.id.clone()).collect();
        order.insert(index, item.id.clone());
        order
    });
    LowpolyDiff { objects: Some(LowpolyObjectsDelta { added: vec![item], reordered, ..Default::default() }), ..LowpolyDiff::default() }
}

/// 🏗️ Objects-remove field delta.
pub fn diff_objects_remove(id: String) -> LowpolyDiff {
    LowpolyDiff { objects: Some(LowpolyObjectsDelta { removed: vec![id], ..Default::default() }), ..LowpolyDiff::default() }
}

/// 🏗️ Objects-move field delta.
pub fn diff_objects_move(id: &str, to_index: usize, base: &LowpolySnapshot) -> LowpolyDiff {
    let mut order: Vec<String> = base.objects.iter().map(|object| object.id.clone()).collect();
    if let Some(from) = order.iter().position(|existing| existing == id) {
        let moved = order.remove(from);
        order.insert(to_index.min(order.len()), moved);
    }
    LowpolyDiff { objects: Some(LowpolyObjectsDelta { reordered: Some(order), ..Default::default() }), ..LowpolyDiff::default() }
}

fn diff_object_entry(entry: LowpolyObjectPatchEntry) -> LowpolyDiff {
    LowpolyDiff { objects: Some(LowpolyObjectsDelta { patched: vec![entry], ..Default::default() }), ..LowpolyDiff::default() }
}

/// 🏗️ Objects-patch field delta.
pub fn diff_objects_patch(id: String, patch: LowpolyObjectPatch) -> LowpolyDiff {
    diff_object_entry(LowpolyObjectPatchEntry { id, patch, ..Default::default() })
}

fn diff_paint_edit(object_id: String, edit: LowpolyPaintEdit) -> LowpolyDiff {
    diff_object_entry(LowpolyObjectPatchEntry { id: object_id, paint_layers: Some(LowpolyPaintLayersDelta { edits: vec![edit] }), ..Default::default() })
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

/// 🏗️ Mesh-motion field delta: the motion plus the content-addressed handle of the mesh it leaves behind.
pub fn diff_mesh_motion(object_id: String, handle: store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot>, motion: LowpolyMeshMotion) -> LowpolyDiff {
    diff_object_entry(LowpolyObjectPatchEntry { id: object_id, patch: LowpolyObjectPatch { mesh: Some(Some(handle)), ..LowpolyObjectPatch::default() }, mesh_motions: vec![motion], ..Default::default() })
}
//#endregion 🔖️Builders

use crate::schema::PixelRun;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
