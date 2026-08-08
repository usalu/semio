//! 🧬️ Lowpoly artifact — document mutation dispatch enum + shared paint helpers. Per-mutation
//! apply/inverse/diff live under each `🧬️mutations/<emoji><name>/` leaf; this root wraps them.

use crate::artifacts::lowpoly::{LowpolyObject, LowpolyObjectPatch, LowpolyPaintLayer, LowpolyProjection};
use protocol::{CollectionMutation, Mutation};
use serde::{Deserialize, Serialize};

//#region 🔖️Patches
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyPaintLayerPatch {
    pub name: Option<String>,
    pub visible: Option<bool>,
    pub opacity: Option<f32>,
    pub blend_mode: Option<String>,
}

/// @emoji 🩸 A contiguous run of RGBA bytes written into a layer buffer at `offset`; a paint stroke is a
/// list of these, and its inverse holds the bytes that were overwritten (read from the pre-stroke state).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct PixelRun {
    pub offset: u32,
    #[serde(with = "run_bytes_base64")]
    #[dsl(base64)]
    pub bytes: Vec<u8>,
}

mod run_bytes_base64 {
    use base64::Engine;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&base64::engine::general_purpose::STANDARD.encode(bytes))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        let encoded = String::deserialize(deserializer)?;
        base64::engine::general_purpose::STANDARD.decode(encoded.as_bytes()).map_err(serde::de::Error::custom)
    }
}

/// @emoji 🩹 Applies a paint-layer metadata patch and returns the inverse patch.
pub fn apply_paint_layer_patch(layer: &mut LowpolyPaintLayer, patch: &LowpolyPaintLayerPatch) -> LowpolyPaintLayerPatch {
    let mut inverse = LowpolyPaintLayerPatch::default();
    if let Some(value) = &patch.name {
        inverse.name = Some(layer.name.clone());
        layer.name = value.clone();
    }
    if let Some(value) = patch.visible {
        inverse.visible = Some(layer.visible);
        layer.visible = value;
    }
    if let Some(value) = patch.opacity {
        inverse.opacity = Some(layer.opacity);
        layer.opacity = value;
    }
    if let Some(value) = &patch.blend_mode {
        inverse.blend_mode = Some(layer.blend_mode.clone());
        layer.blend_mode = value.clone();
    }
    inverse
}

/// @emoji 🖌️ Writes pixel runs into a layer buffer in place.
pub fn apply_pixel_runs(pixels: &mut [u8], runs: &[PixelRun]) {
    for run in runs {
        let start = run.offset as usize;
        let end = (start + run.bytes.len()).min(pixels.len());
        if start < pixels.len() {
            pixels[start..end].copy_from_slice(&run.bytes[..end - start]);
        }
    }
}
//#endregion 🔖️Patches

//#region 🔖️Mutations
/// @emoji 🧬️ The typed lowpoly document mutation. Mesh/object structure is flattened into one
/// keyword-tagged variant per `protocol::CollectionMutation` case (`ObjectsAdd`/`ObjectsRemove`/
/// `ObjectsMove`/`ObjectsPatch`) rather than wrapping that generic type directly — `CollectionMutation`
/// is foreign (defined in `protocol`) and generic, so it can never itself implement `dsl::DslField`/
/// `dsl::DslVariants` from this crate (the orphan rule). Apply/inverse for each variant live in the
/// matching `🧬️mutations/<emoji><name>/` modules.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum LowpolyMutation {
    ObjectsAdd {
        index: usize,
        #[dsl(block)]
        item: LowpolyObject,
    },
    ObjectsRemove {
        id: String,
    },
    ObjectsMove {
        id: String,
        to_index: usize,
    },
    ObjectsPatch {
        id: String,
        #[dsl(block)]
        patch: LowpolyObjectPatch,
    },
    AddPaintLayer {
        object_id: String,
        index: usize,
        #[dsl(block)]
        layer: LowpolyPaintLayer,
    },
    RemovePaintLayer {
        object_id: String,
        index: usize,
    },
    PatchPaintLayer {
        object_id: String,
        index: usize,
        #[dsl(block)]
        patch: LowpolyPaintLayerPatch,
    },
    PaintStroke {
        object_id: String,
        layer_index: usize,
        #[dsl(table)]
        runs: Vec<PixelRun>,
    },
    SetProjection {
        #[dsl(block)]
        projection: LowpolyProjection,
    },
}

/// 🔁️ Converts a generic objects `CollectionMutation` back into its flat `LowpolyMutation` variant.
pub fn objects_mutation_from_collection(mutation: CollectionMutation<String, LowpolyObject, LowpolyObjectPatch>) -> LowpolyMutation {
    match mutation {
        CollectionMutation::Add { index: at, item } => LowpolyMutation::ObjectsAdd { index: at, item },
        CollectionMutation::Remove { id } => LowpolyMutation::ObjectsRemove { id },
        CollectionMutation::Move { id, to_index: to } => LowpolyMutation::ObjectsMove { id, to_index: to },
        CollectionMutation::Patch { id, patch } => LowpolyMutation::ObjectsPatch { id, patch },
    }
}

/// @emoji ▶️ Applies one mutation to the projection in place.
pub fn apply_lowpoly_mutation(projection: &mut LowpolyProjection, mutation: &LowpolyMutation) {
    match mutation {
        LowpolyMutation::ObjectsAdd { index, item } => super::objects_add::mutation::apply(projection, *index, item),
        LowpolyMutation::ObjectsRemove { id } => super::objects_remove::mutation::apply(projection, id),
        LowpolyMutation::ObjectsMove { id, to_index } => super::objects_move::mutation::apply(projection, id, *to_index),
        LowpolyMutation::ObjectsPatch { id, patch } => super::objects_patch::mutation::apply(projection, id, patch),
        LowpolyMutation::AddPaintLayer { object_id, index, layer } => super::add_paint_layer::mutation::apply(projection, object_id, *index, layer),
        LowpolyMutation::RemovePaintLayer { object_id, index } => super::remove_paint_layer::mutation::apply(projection, object_id, *index),
        LowpolyMutation::PatchPaintLayer { object_id, index, patch } => super::patch_paint_layer::mutation::apply(projection, object_id, *index, patch),
        LowpolyMutation::PaintStroke { object_id, layer_index, runs } => super::paint_stroke::mutation::apply(projection, object_id, *layer_index, runs),
        LowpolyMutation::SetProjection { projection: replacement } => super::set_projection::mutation::apply(projection, replacement),
    }
}

/// @emoji ↩️ Computes the inverse mutations from pre-state.
pub fn inverse_lowpoly_mutation(projection: &LowpolyProjection, mutation: &LowpolyMutation) -> Vec<LowpolyMutation> {
    match mutation {
        LowpolyMutation::ObjectsAdd { index, item } => super::objects_add::inverse::inverse(projection, *index, item),
        LowpolyMutation::ObjectsRemove { id } => super::objects_remove::inverse::inverse(projection, id),
        LowpolyMutation::ObjectsMove { id, to_index } => super::objects_move::inverse::inverse(projection, id, *to_index),
        LowpolyMutation::ObjectsPatch { id, patch } => super::objects_patch::inverse::inverse(projection, id, patch),
        LowpolyMutation::AddPaintLayer { object_id, index, layer } => super::add_paint_layer::inverse::inverse(projection, object_id, *index, layer),
        LowpolyMutation::RemovePaintLayer { object_id, index } => super::remove_paint_layer::inverse::inverse(projection, object_id, *index),
        LowpolyMutation::PatchPaintLayer { object_id, index, patch } => super::patch_paint_layer::inverse::inverse(projection, object_id, *index, patch),
        LowpolyMutation::PaintStroke { object_id, layer_index, runs } => super::paint_stroke::inverse::inverse(projection, object_id, *layer_index, runs),
        LowpolyMutation::SetProjection { projection: replacement } => super::set_projection::inverse::inverse(projection, replacement),
    }
}

impl Mutation<LowpolyProjection> for LowpolyMutation {
    type Diff = crate::artifacts::lowpoly::diff::LowpolyDiff;

    fn diff(&self, _projection: &LowpolyProjection) -> Self::Diff {
        crate::artifacts::lowpoly::diff::LowpolyDiff { mutations: vec![self.clone()] }
    }

    fn inverse(&self, projection: &LowpolyProjection) -> Vec<Self> {
        inverse_lowpoly_mutation(projection, self)
    }
}

pub use super::objects_add::mutation::{objects_add, ObjectsAdd};
pub use super::objects_remove::mutation::{objects_remove, ObjectsRemove};
pub use super::objects_move::mutation::{objects_move, ObjectsMove};
pub use super::objects_patch::mutation::{objects_patch, ObjectsPatch};
pub use super::add_paint_layer::mutation::{add_paint_layer, AddPaintLayer};
pub use super::remove_paint_layer::mutation::{remove_paint_layer, RemovePaintLayer};
pub use super::patch_paint_layer::mutation::{patch_paint_layer, PatchPaintLayer};
pub use super::paint_stroke::mutation::{paint_stroke, PaintStroke};
pub use super::set_projection::mutation::{set_projection, SetProjection};
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::lowpoly::engine::default_projection;

    //#region 🔖️MutationsCoverage
    #[test]
    fn apply_mutations_on_missing_object_are_no_ops() {
        let projection = default_projection();
        let mut mutated = projection.clone();
        apply_lowpoly_mutation(&mut mutated, &LowpolyMutation::AddPaintLayer { object_id: "missing".into(), index: 0, layer: LowpolyPaintLayer::new("X") });
        apply_lowpoly_mutation(&mut mutated, &LowpolyMutation::RemovePaintLayer { object_id: "missing".into(), index: 0 });
        apply_lowpoly_mutation(&mut mutated, &LowpolyMutation::PatchPaintLayer { object_id: "missing".into(), index: 0, patch: LowpolyPaintLayerPatch::default() });
        apply_lowpoly_mutation(&mut mutated, &LowpolyMutation::PaintStroke { object_id: "missing".into(), layer_index: 0, runs: vec![] });
        assert_eq!(mutated, projection);
    }

    #[test]
    fn apply_remove_and_patch_and_stroke_out_of_range_are_no_ops() {
        let projection = default_projection();
        let object_id = projection.objects[0].id.clone();
        let mut mutated = projection.clone();
        apply_lowpoly_mutation(&mut mutated, &LowpolyMutation::RemovePaintLayer { object_id: object_id.clone(), index: 99 });
        apply_lowpoly_mutation(&mut mutated, &LowpolyMutation::PatchPaintLayer { object_id: object_id.clone(), index: 99, patch: LowpolyPaintLayerPatch { visible: Some(false), ..Default::default() } });
        apply_lowpoly_mutation(&mut mutated, &LowpolyMutation::PaintStroke { object_id, layer_index: 99, runs: vec![PixelRun { offset: 0, bytes: vec![1] }] });
        assert_eq!(mutated, projection);
    }

    #[test]
    fn apply_set_projection_replaces_entire_projection() {
        let mut projection = default_projection();
        let replacement = crate::artifacts::lowpoly::projection_from_mesh_json(&tiny_mesh_json(), "obj-x", "X");
        apply_lowpoly_mutation(&mut projection, &LowpolyMutation::SetProjection { projection: replacement.clone() });
        assert_eq!(projection, replacement);
    }

    #[test]
    fn inverse_add_paint_layer_produces_remove_at_same_index() {
        let projection = default_projection();
        let mutation = LowpolyMutation::AddPaintLayer { object_id: projection.objects[0].id.clone(), index: 1, layer: LowpolyPaintLayer::new("New") };
        let inverse = inverse_lowpoly_mutation(&projection, &mutation);
        match &inverse[..] {
            [LowpolyMutation::RemovePaintLayer { object_id, index }] => {
                assert_eq!(object_id, &projection.objects[0].id);
                assert_eq!(*index, 1);
            }
            other => panic!("expected RemovePaintLayer, got {other:?}"),
        }
    }

    #[test]
    fn inverse_remove_paint_layer_restores_the_removed_layer_by_content() {
        let projection = default_projection();
        let object_id = projection.objects[0].id.clone();
        let mutation = LowpolyMutation::RemovePaintLayer { object_id, index: 0 };
        let inverse = inverse_lowpoly_mutation(&projection, &mutation);
        let mut mutated = projection.clone();
        apply_lowpoly_mutation(&mut mutated, &mutation);
        assert_ne!(mutated, projection);
        for step in &inverse {
            apply_lowpoly_mutation(&mut mutated, step);
        }
        assert_eq!(mutated, projection);
    }

    #[test]
    fn inverse_remove_paint_layer_on_missing_layer_falls_back_to_default_layer() {
        let projection = default_projection();
        let mutation = LowpolyMutation::RemovePaintLayer { object_id: projection.objects[0].id.clone(), index: 99 };
        let inverse = inverse_lowpoly_mutation(&projection, &mutation);
        match &inverse[..] {
            [LowpolyMutation::AddPaintLayer { index, layer, .. }] => {
                assert_eq!(*index, 99);
                assert_eq!(layer.name, "Layer");
            }
            other => panic!("expected AddPaintLayer, got {other:?}"),
        }
    }

    #[test]
    fn inverse_patch_paint_layer_round_trips_through_apply() {
        let projection = default_projection();
        let object_id = projection.objects[0].id.clone();
        let patch = LowpolyPaintLayerPatch { name: Some("Renamed".into()), visible: Some(false), opacity: Some(0.3), blend_mode: Some("screen".into()) };
        let mutation = LowpolyMutation::PatchPaintLayer { object_id, index: 0, patch };
        let inverse = inverse_lowpoly_mutation(&projection, &mutation);
        let mut mutated = projection.clone();
        apply_lowpoly_mutation(&mut mutated, &mutation);
        assert_ne!(mutated, projection);
        for step in &inverse {
            apply_lowpoly_mutation(&mut mutated, step);
        }
        assert_eq!(mutated, projection);
    }

    #[test]
    fn inverse_set_projection_captures_pre_state() {
        let projection = default_projection();
        let replacement = crate::artifacts::lowpoly::projection_from_mesh_json(&tiny_mesh_json(), "obj-x", "X");
        let mutation = LowpolyMutation::SetProjection { projection: replacement };
        let inverse = inverse_lowpoly_mutation(&projection, &mutation);
        match &inverse[..] {
            [LowpolyMutation::SetProjection { projection: restored }] => assert_eq!(restored, &projection),
            other => panic!("expected SetProjection, got {other:?}"),
        }
    }

    #[test]
    fn paint_layer_patch_apply_mutates_and_inverse_restores_all_fields() {
        let mut layer = LowpolyPaintLayer::new("Base");
        let original = layer.clone();
        let patch = LowpolyPaintLayerPatch { name: Some("Top".into()), visible: Some(false), opacity: Some(0.25), blend_mode: Some("multiply".into()) };
        let inverse = apply_paint_layer_patch(&mut layer, &patch);
        assert_eq!(layer.name, "Top");
        assert!(!layer.visible);
        assert_eq!(layer.opacity, 0.25);
        assert_eq!(layer.blend_mode, "multiply");
        apply_paint_layer_patch(&mut layer, &inverse);
        assert_eq!(layer, original);
    }

    #[test]
    fn paint_stroke_mutation_inverse_restores_prior_pixels() {
        let projection = default_projection();
        let object_id = projection.objects[0].id.clone();
        let mutation = LowpolyMutation::PaintStroke { object_id, layer_index: 0, runs: vec![PixelRun { offset: 0, bytes: vec![1, 2, 3, 4] }] };
        let inverse = mutation.inverse(&projection);
        let mut painted = projection.clone();
        apply_lowpoly_mutation(&mut painted, &mutation);
        assert_eq!(&painted.objects[0].paint_layers[0].pixels[0..4], &[1, 2, 3, 4]);
        for step in &inverse {
            apply_lowpoly_mutation(&mut painted, step);
        }
        assert_eq!(painted, projection);
    }

    #[test]
    fn objects_patch_mutation_inverse_restores_prior_mesh_and_name() {
        let projection = default_projection();
        let object_id = projection.objects[0].id.clone();
        let mutation = LowpolyMutation::ObjectsPatch { id: object_id, patch: LowpolyObjectPatch { name: Some("Renamed".into()), ..Default::default() } };
        let inverse = mutation.inverse(&projection);
        let mut next = projection.clone();
        apply_lowpoly_mutation(&mut next, &mutation);
        assert_eq!(next.objects[0].name, "Renamed");
        for step in &inverse {
            apply_lowpoly_mutation(&mut next, step);
        }
        assert_eq!(next, projection);
    }
    //#endregion 🔖️MutationsCoverage

    fn tiny_mesh_json() -> String {
        semio_s_3d::mesh::HalfedgeMesh::box_prim(1.0, 1.0, 1.0).expect("box prim").to_json().expect("mesh json")
    }
}
//#endregion 🧪️Tests
