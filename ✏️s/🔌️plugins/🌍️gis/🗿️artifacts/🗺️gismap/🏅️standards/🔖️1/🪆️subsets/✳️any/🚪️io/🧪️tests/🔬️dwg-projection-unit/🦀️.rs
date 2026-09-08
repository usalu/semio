mod tests {
    use super::*;
    use semio_s_artifact_stdio_dwg::{DwgColor, DwgEntity};
    #[semio_framework_async_macros::async_test]
    async fn dwg_import_collects_point_and_line_vertices() {
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 2.0, 0.0] } });
        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [3.0, 4.0, 0.0] } });
        let value = gis2d_document_json_from_dwg(&drawing).expect("import dwg");
        let positions = value.get("positions").and_then(|v| v.as_array()).expect("positions array");
        assert_eq!(positions.len(), 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn dwg_import_falls_back_to_default_document_when_empty() {
        let drawing = DwgDrawing::default();
        let value = gis2d_document_json_from_dwg(&drawing).expect("import empty dwg");
        let snapshot: GisMapSnapshot = dsl::json::from_json_str(&value.to_string()).expect("document");
        assert!(!snapshot.positions.is_empty(), "fallback seeds the reuse-map document");
    }

    #[semio_framework_async_macros::async_test]
    async fn dwg_import_lowers_a_closed_polyline_through_a_draw_node_and_carries_the_close_segment() {
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]], bulges: vec![0.0, 0.0, 0.0] } });
        let scene = dwg_drawing_to_semio_drawing(&drawing);
        let DrawNode::Group { children, .. } = &scene.layers[0].root else { panic!("expected a group root") };
        let DrawNode::Path { segments, .. } = &children[0] else { panic!("expected a path node") };
        assert!(matches!(segments.first(), Some(PathSegment::MoveTo { .. })));
        assert!(matches!(segments.last(), Some(PathSegment::Close)));
        assert_eq!(segments.len(), 4, "3 vertices + Close");

        let value = gis2d_document_json_from_dwg(&drawing).expect("import dwg");
        let positions = value.get("positions").and_then(|v| v.as_array()).expect("positions array");
        assert_eq!(positions.len(), 3, "one position feature per polyline vertex");
    }
}
