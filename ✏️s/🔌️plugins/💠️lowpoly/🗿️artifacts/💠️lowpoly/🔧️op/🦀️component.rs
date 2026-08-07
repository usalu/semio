//! ⚡️ Lowpoly artifact — the document operation enum + apply/invert laws (constitutional: op). The
//! config-only counterpart (`LowpolyConfigOperation`) lives at `crate::apps::lowpoly::config` — it
//! patches ephemeral view state, never the document.

use crate::artifacts::lowpoly::engine::{layer_pixels_at, object_mut};
use crate::artifacts::lowpoly::{LowpolyObject, LowpolyObjectPatch, LowpolyPaintLayer, LowpolyProjection};
use protocol::{apply_collection_operation, invert_collection_operation, CollectionOperation, Operation};
use serde::{Deserialize, Serialize};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Patches
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyPaintLayerPatch {
    pub name: Option<String>,
    pub visible: Option<bool>,
    pub opacity: Option<f32>,
    pub blend_mode: Option<String>,
}

/// @emoji 🩹️ A contiguous run of RGBA bytes written into a layer buffer at `offset`; a paint stroke is a
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
//#endregion 🔖️Patches

//#region 🔖️Operations
/// @emoji 🧩️ The typed lowpoly document operation. Mesh/object structure is flattened into one
/// keyword-tagged variant per `protocol::CollectionOperation` case (`ObjectsAdd`/`ObjectsRemove`/
/// `ObjectsMove`/`ObjectsPatch`) rather than wrapping that generic type directly — `CollectionOperation`
/// is foreign (defined in `protocol`) and generic, so it can never itself implement `dsl::DslField`/
/// `dsl::DslVariants` from this crate (the orphan rule requires a local type to anchor the impl on,
/// and its own outer type isn't local either). {@link apply_lowpoly_operation}/
/// {@link invert_lowpoly_operation} reconstruct a `CollectionOperation` ad hoc per match arm to keep
/// reusing `protocol`'s generic collection apply/invert helpers. Per-object paint-layer structure and
/// pixel edits get dedicated variants whose inverses restore the exact prior layers / overwritten
/// pixel runs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum LowpolyOperation {
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

/// 🔁️ Converts a generic objects `CollectionOperation` (as produced by
/// `protocol::invert_collection_operation`) back into its flat `LowpolyOperation` variant.
fn objects_operation_from_collection(operation: CollectionOperation<String, LowpolyObject, LowpolyObjectPatch>) -> LowpolyOperation {
    match operation {
        CollectionOperation::Add { index: at, item } => LowpolyOperation::ObjectsAdd { index: at, item },
        CollectionOperation::Remove { id } => LowpolyOperation::ObjectsRemove { id },
        CollectionOperation::Move { id, to_index: to } => LowpolyOperation::ObjectsMove { id, to_index: to },
        CollectionOperation::Patch { id, patch } => LowpolyOperation::ObjectsPatch { id, patch },
    }
}

fn apply_paint_layer_patch(layer: &mut LowpolyPaintLayer, patch: &LowpolyPaintLayerPatch) -> LowpolyPaintLayerPatch {
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

fn apply_pixel_runs(pixels: &mut [u8], runs: &[PixelRun]) {
    for run in runs {
        let start = run.offset as usize;
        let end = (start + run.bytes.len()).min(pixels.len());
        if start < pixels.len() {
            pixels[start..end].copy_from_slice(&run.bytes[..end - start]);
        }
    }
}

/// @emoji ▶️ Applies one operation to the projection in place. Pure; the store clones the projection
/// before calling so this never observes shared state.
pub fn apply_lowpoly_operation(projection: &mut LowpolyProjection, operation: &LowpolyOperation) {
    match operation {
        LowpolyOperation::ObjectsAdd { index, item } => apply_collection_operation(&mut projection.objects, &CollectionOperation::Add { index: *index, item: item.clone() }),
        LowpolyOperation::ObjectsRemove { id } => apply_collection_operation(&mut projection.objects, &CollectionOperation::Remove { id: id.clone() }),
        LowpolyOperation::ObjectsMove { id, to_index } => apply_collection_operation(&mut projection.objects, &CollectionOperation::Move { id: id.clone(), to_index: *to_index }),
        LowpolyOperation::ObjectsPatch { id, patch } => apply_collection_operation(&mut projection.objects, &CollectionOperation::Patch { id: id.clone(), patch: patch.clone() }),
        LowpolyOperation::AddPaintLayer { object_id, index, layer } => {
            if let Some(object) = object_mut(projection, object_id) {
                let at = (*index).min(object.paint_layers.len());
                object.paint_layers.insert(at, layer.clone());
            }
        }
        LowpolyOperation::RemovePaintLayer { object_id, index } => {
            if let Some(object) = object_mut(projection, object_id) {
                if *index < object.paint_layers.len() {
                    object.paint_layers.remove(*index);
                }
            }
        }
        LowpolyOperation::PatchPaintLayer { object_id, index, patch } => {
            if let Some(object) = object_mut(projection, object_id) {
                if let Some(layer) = object.paint_layers.get_mut(*index) {
                    apply_paint_layer_patch(layer, patch);
                }
            }
        }
        LowpolyOperation::PaintStroke { object_id, layer_index, runs } => {
            if let Some(object) = object_mut(projection, object_id) {
                if let Some(layer) = object.paint_layers.get_mut(*layer_index) {
                    apply_pixel_runs(&mut layer.pixels, runs);
                }
            }
        }
        LowpolyOperation::SetProjection { projection: replacement } => {
            *projection = replacement.clone();
        }
    }
}

/// @emoji ↩️ Computes the inverse operation from pre-state. For `PaintStroke` this reads the
/// currently-stored bytes at each run's offset so undo restores the exact overwritten pixels (not
/// merely "clear paint").
pub fn invert_lowpoly_operation(projection: &LowpolyProjection, operation: &LowpolyOperation) -> LowpolyOperation {
    match operation {
        LowpolyOperation::ObjectsAdd { index, item } => objects_operation_from_collection(invert_collection_operation(&projection.objects, &CollectionOperation::Add { index: *index, item: item.clone() })),
        LowpolyOperation::ObjectsRemove { id } => objects_operation_from_collection(invert_collection_operation(&projection.objects, &CollectionOperation::Remove { id: id.clone() })),
        LowpolyOperation::ObjectsMove { id, to_index } => objects_operation_from_collection(invert_collection_operation(&projection.objects, &CollectionOperation::Move { id: id.clone(), to_index: *to_index })),
        LowpolyOperation::ObjectsPatch { id, patch } => objects_operation_from_collection(invert_collection_operation(&projection.objects, &CollectionOperation::Patch { id: id.clone(), patch: patch.clone() })),
        LowpolyOperation::AddPaintLayer { object_id, index, .. } => LowpolyOperation::RemovePaintLayer { object_id: object_id.clone(), index: *index },
        LowpolyOperation::RemovePaintLayer { object_id, index } => {
            let layer = projection.objects.iter().find(|object| object.id == *object_id).and_then(|object| object.paint_layers.get(*index)).cloned().unwrap_or_else(|| LowpolyPaintLayer::new("Layer"));
            LowpolyOperation::AddPaintLayer { object_id: object_id.clone(), index: *index, layer }
        }
        LowpolyOperation::PatchPaintLayer { object_id, index, patch } => {
            let mut probe = projection.objects.iter().find(|object| object.id == *object_id).and_then(|object| object.paint_layers.get(*index)).cloned().unwrap_or_else(|| LowpolyPaintLayer::new("Layer"));
            let inverse = apply_paint_layer_patch(&mut probe, patch);
            LowpolyOperation::PatchPaintLayer { object_id: object_id.clone(), index: *index, patch: inverse }
        }
        LowpolyOperation::PaintStroke { object_id, layer_index, runs } => {
            let pixels = layer_pixels_at(projection, object_id, *layer_index);
            let inverse_runs = runs
                .iter()
                .map(|run| {
                    let start = run.offset as usize;
                    let bytes = pixels
                        .map(|buffer| {
                            let end = (start + run.bytes.len()).min(buffer.len());
                            if start < buffer.len() {
                                buffer[start..end].to_vec()
                            } else {
                                Vec::new()
                            }
                        })
                        .unwrap_or_default();
                    PixelRun { offset: run.offset, bytes }
                })
                .collect();
            LowpolyOperation::PaintStroke { object_id: object_id.clone(), layer_index: *layer_index, runs: inverse_runs }
        }
        LowpolyOperation::SetProjection { .. } => LowpolyOperation::SetProjection { projection: projection.clone() },
    }
}

impl Operation<LowpolyProjection> for LowpolyOperation {
    type Diff = crate::artifacts::lowpoly::diff::LowpolyDiff;

    fn diff(&self, _projection: &LowpolyProjection) -> Self::Diff {
        crate::artifacts::lowpoly::diff::LowpolyDiff { operations: vec![self.clone()] }
    }

    fn backwards(&self, projection: &LowpolyProjection) -> Vec<Self> {
        vec![invert_lowpoly_operation(projection, self)]
    }
}
//#endregion 🔖️Operations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::lowpoly::engine::default_projection;

    //#region 🔖️OperationsCoverage
    #[test]
    fn apply_operations_on_missing_object_are_no_ops() {
        let projection = default_projection();
        let mut mutated = projection.clone();
        apply_lowpoly_operation(&mut mutated, &LowpolyOperation::AddPaintLayer { object_id: "missing".into(), index: 0, layer: LowpolyPaintLayer::new("X") });
        apply_lowpoly_operation(&mut mutated, &LowpolyOperation::RemovePaintLayer { object_id: "missing".into(), index: 0 });
        apply_lowpoly_operation(&mut mutated, &LowpolyOperation::PatchPaintLayer { object_id: "missing".into(), index: 0, patch: LowpolyPaintLayerPatch::default() });
        apply_lowpoly_operation(&mut mutated, &LowpolyOperation::PaintStroke { object_id: "missing".into(), layer_index: 0, runs: vec![] });
        assert_eq!(mutated, projection);
    }

    #[test]
    fn apply_remove_and_patch_and_stroke_out_of_range_are_no_ops() {
        let projection = default_projection();
        let object_id = projection.objects[0].id.clone();
        let mut mutated = projection.clone();
        apply_lowpoly_operation(&mut mutated, &LowpolyOperation::RemovePaintLayer { object_id: object_id.clone(), index: 99 });
        apply_lowpoly_operation(&mut mutated, &LowpolyOperation::PatchPaintLayer { object_id: object_id.clone(), index: 99, patch: LowpolyPaintLayerPatch { visible: Some(false), ..Default::default() } });
        apply_lowpoly_operation(&mut mutated, &LowpolyOperation::PaintStroke { object_id, layer_index: 99, runs: vec![PixelRun { offset: 0, bytes: vec![1] }] });
        assert_eq!(mutated, projection);
    }

    #[test]
    fn apply_set_projection_replaces_entire_projection() {
        let mut projection = default_projection();
        let replacement = crate::artifacts::lowpoly::projection_from_mesh_json(&tiny_mesh_json(), "obj-x", "X");
        apply_lowpoly_operation(&mut projection, &LowpolyOperation::SetProjection { projection: replacement.clone() });
        assert_eq!(projection, replacement);
    }

    #[test]
    fn invert_add_paint_layer_produces_remove_at_same_index() {
        let projection = default_projection();
        let operation = LowpolyOperation::AddPaintLayer { object_id: projection.objects[0].id.clone(), index: 1, layer: LowpolyPaintLayer::new("New") };
        let inverse = invert_lowpoly_operation(&projection, &operation);
        match inverse {
            LowpolyOperation::RemovePaintLayer { object_id, index } => {
                assert_eq!(object_id, projection.objects[0].id);
                assert_eq!(index, 1);
            }
            other => panic!("expected RemovePaintLayer, got {other:?}"),
        }
    }

    #[test]
    fn invert_remove_paint_layer_restores_the_removed_layer_by_content() {
        let projection = default_projection();
        let object_id = projection.objects[0].id.clone();
        let operation = LowpolyOperation::RemovePaintLayer { object_id, index: 0 };
        let inverse = invert_lowpoly_operation(&projection, &operation);
        let mut mutated = projection.clone();
        apply_lowpoly_operation(&mut mutated, &operation);
        assert_ne!(mutated, projection);
        apply_lowpoly_operation(&mut mutated, &inverse);
        assert_eq!(mutated, projection);
    }

    #[test]
    fn invert_remove_paint_layer_on_missing_layer_falls_back_to_default_layer() {
        let projection = default_projection();
        let operation = LowpolyOperation::RemovePaintLayer { object_id: projection.objects[0].id.clone(), index: 99 };
        let inverse = invert_lowpoly_operation(&projection, &operation);
        match inverse {
            LowpolyOperation::AddPaintLayer { index, layer, .. } => {
                assert_eq!(index, 99);
                assert_eq!(layer.name, "Layer");
            }
            other => panic!("expected AddPaintLayer, got {other:?}"),
        }
    }

    #[test]
    fn invert_patch_paint_layer_round_trips_through_apply() {
        let projection = default_projection();
        let object_id = projection.objects[0].id.clone();
        let patch = LowpolyPaintLayerPatch { name: Some("Renamed".into()), visible: Some(false), opacity: Some(0.3), blend_mode: Some("screen".into()) };
        let operation = LowpolyOperation::PatchPaintLayer { object_id, index: 0, patch };
        let inverse = invert_lowpoly_operation(&projection, &operation);
        let mut mutated = projection.clone();
        apply_lowpoly_operation(&mut mutated, &operation);
        assert_ne!(mutated, projection);
        apply_lowpoly_operation(&mut mutated, &inverse);
        assert_eq!(mutated, projection);
    }

    #[test]
    fn invert_set_projection_captures_pre_state() {
        let projection = default_projection();
        let replacement = crate::artifacts::lowpoly::projection_from_mesh_json(&tiny_mesh_json(), "obj-x", "X");
        let operation = LowpolyOperation::SetProjection { projection: replacement };
        let inverse = invert_lowpoly_operation(&projection, &operation);
        match inverse {
            LowpolyOperation::SetProjection { projection: restored } => assert_eq!(restored, projection),
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
    fn paint_stroke_op_backwards_restores_prior_pixels() {
        let projection = default_projection();
        let object_id = projection.objects[0].id.clone();
        let operation = LowpolyOperation::PaintStroke { object_id, layer_index: 0, runs: vec![PixelRun { offset: 0, bytes: vec![1, 2, 3, 4] }] };
        let backwards = operation.backwards(&projection);
        let mut painted = projection.clone();
        apply_lowpoly_operation(&mut painted, &operation);
        assert_eq!(&painted.objects[0].paint_layers[0].pixels[0..4], &[1, 2, 3, 4]);
        for operation in &backwards {
            apply_lowpoly_operation(&mut painted, operation);
        }
        assert_eq!(painted, projection);
    }

    #[test]
    fn objects_patch_op_backwards_restores_prior_mesh_and_name() {
        let projection = default_projection();
        let object_id = projection.objects[0].id.clone();
        let operation = LowpolyOperation::ObjectsPatch { id: object_id, patch: LowpolyObjectPatch { name: Some("Renamed".into()), ..Default::default() } };
        let backwards = operation.backwards(&projection);
        let mut next = projection.clone();
        apply_lowpoly_operation(&mut next, &operation);
        assert_eq!(next.objects[0].name, "Renamed");
        for operation in &backwards {
            apply_lowpoly_operation(&mut next, operation);
        }
        assert_eq!(next, projection);
    }
    //#endregion 🔖️OperationsCoverage

    //#region 🔖️OpText
    fn tiny_mesh_json() -> String {
        semio_s_3d::mesh::HalfedgeMesh::box_prim(1.0, 1.0, 1.0).expect("box prim").to_json().expect("mesh json")
    }

    fn tiny_object(id: &str, name: &str) -> LowpolyObject {
        LowpolyObject { id: id.into(), name: name.into(), transform: Default::default(), smooth_shading: false, mesh_json: tiny_mesh_json(), paint_layers: vec![LowpolyPaintLayer::new("Base")] }
    }

    #[test]
    fn op_text_round_trip_objects_add() {
        let operation = LowpolyOperation::ObjectsAdd { index: 1, item: tiny_object("obj-100", "Box") };
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn op_text_round_trip_objects_remove() {
        let operation: LowpolyOperation = LowpolyOperation::ObjectsRemove { id: "obj-1".into() };
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn op_text_round_trip_objects_move() {
        let operation = LowpolyOperation::ObjectsMove { id: "obj-1".into(), to_index: 2 };
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn op_text_round_trip_objects_patch_without_mesh() {
        let operation = LowpolyOperation::ObjectsPatch {
            id: "obj-1".into(),
            patch: LowpolyObjectPatch { name: Some("Renamed".into()), smooth_shading: Some(true), transform: Some(crate::artifacts::lowpoly::LowpolyTransform { position: [1.0, 2.0, 3.0], ..Default::default() }), mesh_json: None },
        };
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn op_text_round_trip_objects_patch_with_mesh() {
        let operation = LowpolyOperation::ObjectsPatch { id: "obj-1".into(), patch: LowpolyObjectPatch { mesh_json: Some(tiny_mesh_json()), ..Default::default() } };
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn op_text_round_trip_add_paint_layer() {
        let operation = LowpolyOperation::AddPaintLayer { object_id: "obj-1".into(), index: 1, layer: LowpolyPaintLayer::new("Detail") };
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn op_text_round_trip_remove_paint_layer() {
        let operation = LowpolyOperation::RemovePaintLayer { object_id: "obj-1".into(), index: 0 };
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn op_text_round_trip_patch_paint_layer() {
        let operation = LowpolyOperation::PatchPaintLayer { object_id: "obj-1".into(), index: 0, patch: LowpolyPaintLayerPatch { name: Some("Top".into()), visible: Some(false), opacity: Some(0.5), blend_mode: Some("multiply".into()) } };
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn op_text_round_trip_paint_stroke() {
        let operation = LowpolyOperation::PaintStroke { object_id: "obj-1".into(), layer_index: 0, runs: vec![PixelRun { offset: 12, bytes: vec![255, 0, 0, 255] }, PixelRun { offset: 400, bytes: vec![0, 255, 0, 255, 0, 0, 0, 128] }] };
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn op_text_round_trip_set_projection() {
        let operation = LowpolyOperation::SetProjection { projection: default_projection() };
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn op_text_parse_rejects_unknown_operation_kind() {
        let result = <LowpolyOperation as protocol::OpText>::parse_op("bogusOperation foo=bar");
        assert!(result.is_err());
    }

    #[test]
    fn op_text_parse_rejects_unknown_objects_suboperation() {
        let result = <LowpolyOperation as protocol::OpText>::parse_op("objects.frobnicate id=obj-1");
        assert!(result.is_err());
    }
    //#endregion 🔖️OpText
}
//#endregion 🧪️Tests
