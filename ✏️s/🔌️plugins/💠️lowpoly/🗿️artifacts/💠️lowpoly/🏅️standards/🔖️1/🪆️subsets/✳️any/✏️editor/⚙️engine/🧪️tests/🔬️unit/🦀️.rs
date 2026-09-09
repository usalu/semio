use super::*;
use crate::schema::{default_mesh_workspace, default_snapshot};

#[semio_framework_async_macros::async_test]
async fn document_loads_meshes() {
    let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    assert_eq!(doc.meshes.len(), 1);
    assert!(doc.meshes[0].face_count() > 0);
}

#[semio_framework_async_macros::async_test]
async fn active_mesh_tessellates() {
    let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    let transfer = doc.active_mesh().unwrap().tessellate().unwrap();
    assert!(!transfer.positions.is_empty());
    assert!(!transfer.indices.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn add_primitive_box() {
    let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    let id = doc.add_primitive("box").unwrap();
    assert!(doc.snapshot.objects.iter().any(|o| o.id == id));
    assert_eq!(doc.meshes.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn tessellate_all_returns_every_object() {
    let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    let _ = doc.add_primitive("box").unwrap();
    let json = doc.tessellate_all_json().unwrap();
    let items: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
    assert_eq!(items.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn projection_json_embeds_paint_pixels_as_base64() {
    let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    let json = serde_json::to_string(&Into::<serde_json::Value>::into(dsl::ToValue::to_value(&doc.snapshot))).unwrap();
    assert!(json.contains("\"pixels\""));
    // base64 white, never a raw integer array.
    assert!(!json.contains("255,255,255"));
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_mesh_has_unwrapped_uvs() {
    let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    let transfer = doc.active_mesh().unwrap().tessellate().unwrap();
    assert!(transfer.uvs.iter().any(|uv| *uv > 0.0));
}

#[semio_framework_async_macros::async_test]
async fn paint_stroke_writes_pixels() {
    let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    let object_id = doc.active_object_id.clone();
    doc.paint_stroke(&object_id, 0, 0.5, 0.5, 4.0, [255, 0, 0, 255], 0.5, 1.0, false).unwrap();
    let composite = doc.composite_layers(&object_id).unwrap();
    let size = LOWPOLY_PAINT_TEXTURE_SIZE;
    let center = (size / 2 * size + size / 2) * 4;
    assert!(composite[center] > 200);
}

//#region 🔖️ComputeSessionCoverage
#[semio_framework_async_macros::async_test]
async fn add_primitive_supports_every_known_kind() {
    let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    for kind in ["plane", "cylinder", "cone", "ico_sphere"] {
        let id = doc.add_primitive(kind).unwrap();
        assert_eq!(doc.active_object_id(), id);
        assert!(doc.snapshot().objects.iter().any(|o| o.id == id));
    }
    assert_eq!(doc.snapshot().objects.len(), 5);
}

#[semio_framework_async_macros::async_test]
async fn add_primitive_unknown_kind_errors() {
    let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    let result = doc.add_primitive("teapot");
    assert!(matches!(result, Err(LowpolyCoreError::UnknownPrimitive(kind)) if kind == "teapot"));
}

#[semio_framework_async_macros::async_test]
async fn object_index_errors_for_unknown_id() {
    let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    assert!(matches!(doc.object_index("missing"), Err(LowpolyCoreError::ObjectNotFound)));
}

#[semio_framework_async_macros::async_test]
async fn ensure_paint_layer_errors_for_unknown_object_and_out_of_range_index() {
    let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    let object_id = doc.active_object_id().to_string();
    assert!(matches!(doc.ensure_paint_layer("missing", 0), Err(LowpolyCoreError::ObjectNotFound)));
    assert!(matches!(doc.ensure_paint_layer(&object_id, 99), Err(LowpolyCoreError::LayerIndexOutOfRange)));
}

#[semio_framework_async_macros::async_test]
async fn layer_pixels_errors_for_out_of_range_index() {
    let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    let object_id = doc.active_object_id().to_string();
    assert!(matches!(doc.layer_pixels(&object_id, 5), Err(LowpolyCoreError::LayerIndexOutOfRange)));
}

#[semio_framework_async_macros::async_test]
async fn active_mesh_errors_when_active_object_id_is_unknown() {
    let doc = LowpolyDocument::with_context(default_snapshot(), "does-not-exist".into(), LowpolySelection::default(), default_mesh_workspace()).unwrap();
    assert!(matches!(doc.active_mesh(), Err(LowpolyCoreError::NoActiveObject)));
}

#[semio_framework_async_macros::async_test]
async fn mesh_at_returns_none_past_object_count() {
    let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    assert!(doc.mesh_at(0).is_some());
    assert!(doc.mesh_at(99).is_none());
}

#[semio_framework_async_macros::async_test]
async fn sync_meshes_to_snapshot_writes_back_mesh_json() {
    let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    doc.add_primitive("box").unwrap();
    doc.active_mesh_mut().unwrap().translate(Vec3::new(1.0, 0.0, 0.0)).unwrap();
    let idx = doc.active_index().unwrap();
    let object_id = doc.snapshot().objects[idx].id.clone();
    let before = doc.mesh_workspace().get(&object_id).cloned();
    doc.sync_meshes_to_snapshot().unwrap();
    assert_ne!(doc.mesh_workspace().get(&object_id).cloned(), before);
}

#[semio_framework_async_macros::async_test]
async fn normalize_selection_mode_maps_object_to_mesh_and_passes_through_others() {
    assert_eq!(LowpolyDocument::normalize_selection_mode("object"), "mesh");
    assert_eq!(LowpolyDocument::normalize_selection_mode("face"), "face");
    assert_eq!(LowpolyDocument::normalize_selection_mode("vertex"), "vertex");
}

#[semio_framework_async_macros::async_test]
async fn apply_selection_normalizes_mode_and_stores_ids() {
    let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    doc.apply_selection("object", vec![3, 4]);
    assert_eq!(doc.selection().mode, "mesh");
    assert_eq!(doc.selection().ids, vec![3, 4]);
}

#[semio_framework_async_macros::async_test]
async fn selected_ids_are_empty_when_selection_mode_mismatches() {
    let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    doc.apply_selection("face", vec![1, 2]);
    assert!(doc.selected_vertex_ids().is_empty());
    assert!(doc.selected_edge_ids().is_empty());
    assert_eq!(doc.selected_face_ids().len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn selection_vertex_ids_face_mode_dedupes_shared_vertices() {
    let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    doc.add_primitive("box").unwrap();
    doc.apply_selection("face", vec![0, 1]);
    let verts = doc.selection_vertex_ids().unwrap();
    let mesh = doc.active_mesh().unwrap();
    let mut expected: Vec<u32> = Vec::new();
    for face in [FaceId(0), FaceId(1)] {
        for vid in mesh.face_vertex_ids(face).unwrap() {
            if !expected.contains(&vid.0) {
                expected.push(vid.0);
            }
        }
    }
    assert_eq!(verts.into_iter().map(|v| v.0).collect::<Vec<_>>(), expected);
}

#[semio_framework_async_macros::async_test]
async fn selection_vertex_ids_edge_mode_returns_endpoints() {
    let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    doc.add_primitive("box").unwrap();
    doc.apply_selection("edge", vec![0]);
    let verts = doc.selection_vertex_ids().unwrap();
    let mesh = doc.active_mesh().unwrap();
    let (v0, v1) = mesh.edge_endpoints(EdgeId(0)).unwrap();
    assert_eq!(verts, vec![v0, v1]);
}

#[semio_framework_async_macros::async_test]
async fn selection_vertex_ids_mesh_mode_is_empty() {
    let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    assert!(doc.selection_vertex_ids().unwrap().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn selection_transform_pivot_mesh_mode_averages_all_vertices() {
    let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    doc.add_primitive("box").unwrap();
    let mesh = doc.active_mesh().unwrap();
    let count = mesh.vertex_count();
    let mut sum = Vec3::new(0.0, 0.0, 0.0);
    for index in 0..count {
        sum = sum.add(mesh.vertex_position(VertexId(index as u32)).unwrap());
    }
    let expected = sum.scale(1.0 / count as f32);
    let pivot = doc.selection_transform_pivot().unwrap();
    assert!((pivot.x() - expected.x()).abs() < 1e-5);
    assert!((pivot.y() - expected.y()).abs() < 1e-5);
    assert!((pivot.z() - expected.z()).abs() < 1e-5);
}

#[semio_framework_async_macros::async_test]
async fn selection_transform_pivot_vertex_mode_averages_selected_vertices() {
    let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    doc.add_primitive("box").unwrap();
    doc.apply_selection("vertex", vec![0, 1]);
    let mesh = doc.active_mesh().unwrap();
    let p0 = mesh.vertex_position(VertexId(0)).unwrap();
    let p1 = mesh.vertex_position(VertexId(1)).unwrap();
    let expected = p0.add(p1).scale(0.5);
    let pivot = doc.selection_transform_pivot().unwrap();
    assert!((pivot.x() - expected.x()).abs() < 1e-5);
}

#[semio_framework_async_macros::async_test]
async fn selection_transform_pivot_empty_vertex_selection_is_origin() {
    let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    doc.apply_selection("vertex", vec![]);
    let pivot = doc.selection_transform_pivot().unwrap();
    assert_eq!((pivot.x(), pivot.y(), pivot.z()), (0.0, 0.0, 0.0));
}

#[semio_framework_async_macros::async_test]
async fn ensure_all_paint_buffers_adds_missing_layer_and_fixes_wrong_size() {
    let mut projection = default_snapshot();
    projection.objects[0].paint_layers.clear();
    let mut doc = LowpolyDocument::new(projection, default_mesh_workspace()).unwrap();
    assert_eq!(doc.snapshot().objects[0].paint_layers.len(), 1);
    assert_eq!(doc.snapshot().objects[0].paint_layers[0].name, "Base");
    doc.snapshot_mut().objects[0].paint_layers[0].pixels = vec![1, 2, 3];
    doc.ensure_all_paint_buffers();
    assert_eq!(doc.snapshot().objects[0].paint_layers[0].pixels.len(), LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4);
}

#[semio_framework_async_macros::async_test]
async fn fill_bucket_and_sample_pixel_reflect_new_color() {
    let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
    let object_id = doc.active_object_id().to_string();
    doc.fill_bucket(&object_id, 0, 0.5, 0.5, [10, 20, 30, 255]).unwrap();
    assert_eq!(doc.sample_pixel(&object_id, 0.5, 0.5).unwrap(), [10, 20, 30, 255]);
}
//#endregion 🔖️ComputeSessionCoverage
