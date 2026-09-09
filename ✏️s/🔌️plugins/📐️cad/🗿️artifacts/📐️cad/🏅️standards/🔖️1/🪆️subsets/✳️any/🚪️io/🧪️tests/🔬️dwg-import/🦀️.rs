use super::*;
use crate::CadSnapshot;
#[semio_framework_async_macros::async_test]
async fn cad_document_from_dwg_creates_one_object_per_layer_with_geometry() {
    let mut drawing = semio_s_artifact_stdio_dwg::DwgDrawing::default();
    let outline = drawing.ensure_layer("outline");
    let empty_layer = drawing.ensure_layer("empty");
    let _ = empty_layer;
    drawing.entities.push(semio_s_artifact_stdio_dwg::DwgEntity {
        layer: outline,
        color: semio_s_artifact_stdio_dwg::DwgColor::ByLayer,
        geometry: semio_s_artifact_stdio_dwg::DwgGeometry::PolyfaceMesh { vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], faces: vec![[1, 2, 3, 4]] },
    });
    let working = cad_working_scene_from_dwg(&drawing);
    assert_eq!(working.objects.len(), 1, "the empty layer must not contribute an object");
    assert_eq!(working.objects[0].label, "outline");
    let value = cad_document_from_dwg(&drawing).expect("cad document from dwg");
    let scene: CadSnapshot = protocol::FromValue::from_value(value).expect("valid cad scene");
    assert!(scene.shape_model.is_some(), "a real per-layer object must mint a shape-model child");
}

#[semio_framework_async_macros::async_test]
async fn cad_document_from_empty_dwg_mints_no_shape_model_child() {
    let drawing = semio_s_artifact_stdio_dwg::DwgDrawing::default();
    let working = cad_working_scene_from_dwg(&drawing);
    assert!(working.objects.is_empty());
    let value = cad_document_from_dwg(&drawing).expect("cad document from empty dwg");
    let scene: CadSnapshot = protocol::FromValue::from_value(value).expect("valid cad scene");
    assert!(scene.shape_model.is_none(), "no layers means no real geometry to mint a child from");
}
