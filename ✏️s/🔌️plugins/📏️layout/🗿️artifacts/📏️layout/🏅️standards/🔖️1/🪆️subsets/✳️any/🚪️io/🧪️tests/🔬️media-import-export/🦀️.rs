
use super::*;

#[semio_framework_async_macros::async_test]
async fn dwg_import_frames_page_to_rectangular_polyline() {
    let mut drawing = DwgDrawing::default();
    drawing.entities.push(DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[10.0, 20.0], [110.0, 20.0], [110.0, 70.0], [10.0, 70.0]], bulges: vec![0.0; 4] } });
    let value = layout_document_json_from_dwg(&drawing).expect("import dwg");
    let document: LayoutSnapshot = LayoutSnapshot::from_value(value).expect("valid layout document");
    assert_eq!(document.pages.len(), 1);
    assert_eq!(document.pages[0].width, 100.0);
    assert_eq!(document.pages[0].height, 50.0);
}

#[semio_framework_async_macros::async_test]
async fn dwg_import_without_rectangles_falls_back_to_extents() {
    let mut drawing = DwgDrawing::default();
    drawing.entities.push(DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [200.0, 150.0, 0.0] } });
    drawing.extmin = [0.0, 0.0, 0.0];
    drawing.extmax = [200.0, 150.0, 0.0];
    let value = layout_document_json_from_dwg(&drawing).expect("import dwg");
    let document: LayoutSnapshot = LayoutSnapshot::from_value(value).expect("valid layout document");
    assert_eq!(document.pages.len(), 1);
    assert_eq!(document.pages[0].width, 200.0);
    assert_eq!(document.pages[0].height, 150.0);
}

/// 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: DWG import now mints a real
/// `background_drawing` composed child from the FULL decoded drawing (not just page-boundary
/// rects) instead of discarding it — this asserts the mint, the handle shape, and that the
/// content is owned by the durable child record the mint call returns.
#[semio_framework_async_macros::async_test]
async fn dwg_import_mints_a_durable_background_drawing_child() {
    let mut drawing = DwgDrawing::default();
    drawing.entities.push(DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [50.0, 0.0], [50.0, 30.0], [0.0, 30.0]], bulges: vec![0.0; 4] } });
    let value = layout_document_json_from_dwg(&drawing).expect("import dwg");
    let document: LayoutSnapshot = LayoutSnapshot::from_value(value).expect("valid layout document");
    let child = document.background_drawing.as_ref().expect("dwg import mints a background_drawing child");
    assert_eq!(child.handle.target.dialect.subset, "drawing");
    let content = crate::background_drawing_content(&document).expect("mint call retained real content");
    assert_eq!(content.layers.len(), 1, "one imported layer, matching dwg_drawing_to_semio_drawing's single 'imported' layer");
}

/// 🌉️ SVG export merges the snapshot-owned background content's layers behind the document's
/// own page layers, so an imported trace an author
/// draws pages on top of survives export instead of only ever informing import-time framing.
#[semio_framework_async_macros::async_test]
async fn svg_export_merges_owned_background_drawing_behind_pages() {
    ensure_stdio_semio_drawing_registered();
    let mut drawing = DwgDrawing::default();
    drawing.entities.push(DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [50.0, 0.0], [50.0, 30.0], [0.0, 30.0]], bulges: vec![0.0; 4] } });
    let value = layout_document_json_from_dwg(&drawing).expect("import dwg");
    let document: LayoutSnapshot = LayoutSnapshot::from_value(value).expect("valid layout document");
    assert!(document.background_drawing.is_some());
    let (svg, _width, _height) = layout_document_json_to_svg(&document.to_value()).expect("svg export succeeds");
    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("<path"));
}

/// 🌉️ Real end-to-end proof that `layout_document_json_to_svg` composes through stdio's actual
/// `s.stdio.semio/v1/drawing`→svg bridge (`io_dispatch`) rather than hand-rolling SVG text — the
/// two demo pages (400x500 each, 24px gap) lay out canvas-wide, and the resulting markup uses
/// `<path>` (the drawing subset's SVG vocabulary has no `<rect>` element).
#[semio_framework_async_macros::async_test]
async fn svg_export_composes_through_semio_drawing_bridge() {
    ensure_stdio_semio_drawing_registered();
    let doc = crate::schema::default_document();
    let value = doc.to_value();
    let (svg, width, height) = layout_document_json_to_svg(&value).expect("svg export succeeds");
    assert!(svg.starts_with("<svg"), "{svg}");
    assert!(svg.contains("<path"), "{svg}");
    assert!(svg.ends_with("</svg>"), "{svg}");
    assert_eq!(width, 824);
    assert_eq!(height, 500);
}

#[semio_framework_async_macros::async_test]
async fn svg_export_rejects_invalid_document_json() {
    let value = Value::object([("not".into(), Value::String("a layout document".into()))]);
    assert!(layout_document_json_to_svg(&value).is_err());
}
