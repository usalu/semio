
use super::*;

/// 🌉️ Once-guarded stdio registration so `render_drawing_to_svg`'s `io_dispatch` call can
/// resolve the `s.stdio.semio/v1/drawing` → `s.stdio.svg` bridge in a bare `cargo test`
/// process (production boots this via stdio's own plugin `setup()`, never gis).
fn ensure_stdio_semio_registered_for_tests() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        semio_s_artifact_stdio_semio::register();
    });
}

#[semio_framework_async_macros::async_test]
async fn gis_map_snapshot_to_drawing_builds_markers_and_polylines() {
    let mut document = GisMapSnapshot::default();
    document.positions.push(MapFeature { id: "p0".into(), data: value_to_dsl(&serde_json::json!({ "id": "p0", "lon": 5.5818, "lat": 50.603 })) });
    document.routes.push(MapFeature { id: "r0".into(), data: value_to_dsl(&serde_json::json!({ "id": "r0", "points": [[5.5818, 50.603], [5.5825, 50.6035]] })) });
    document.regions.push(MapFeature { id: "g0".into(), data: value_to_dsl(&serde_json::json!({ "id": "g0", "points": [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]] })) });

    let drawing = gis_map_snapshot_to_drawing(&document);
    assert!(drawing.canvas.width >= 256.0 && drawing.canvas.height >= 256.0);
    assert_eq!(drawing.styles.len(), 2);
    let DrawNode::Group { children, .. } = &drawing.layers[0].root else { panic!("expected a group root") };
    assert_eq!(children.len(), 3, "1 marker + 1 route path + 1 region path");

    let DrawNode::Path { segments: marker_segments, style: marker_style } = &children[0] else { panic!("expected the marker path first") };
    assert_eq!(marker_style.as_deref(), Some(GIS_POINT_STYLE));
    assert_eq!(marker_segments.len(), 4, "MoveTo + 2×ArcTo + Close");

    let DrawNode::Path { segments: route_segments, style: route_style } = &children[1] else { panic!("expected the route path second") };
    assert_eq!(route_style.as_deref(), Some(GIS_LINE_STYLE));
    assert!(matches!(route_segments.last(), Some(PathSegment::LineTo { .. })), "routes stay open");

    let DrawNode::Path { segments: region_segments, .. } = &children[2] else { panic!("expected the region path third") };
    assert!(matches!(region_segments.last(), Some(PathSegment::Close)), "regions close");
}

#[semio_framework_async_macros::async_test]
async fn svg_export_renders_real_svg_text_through_the_stdio_drawing_bridge() {
    ensure_stdio_semio_registered_for_tests();
    let document = default_document();
    let value = serde_json::from_str::<Value>(&dsl::json::to_json_string(&document)).expect("document json");
    let (svg, width, height) = gis2d_document_json_to_svg(&value).expect("svg export");
    assert!(svg.contains("<svg"), "real svg text: {svg}");
    assert!(svg.contains("<path"), "at least one path node rendered: {svg}");
    assert!(width > 0 && height > 0);
}

#[semio_framework_async_macros::async_test]
async fn svg_export_of_an_empty_document_still_renders_a_bare_canvas() {
    ensure_stdio_semio_registered_for_tests();
    let value = serde_json::from_str::<Value>(&dsl::json::to_json_string(&GisMapSnapshot::default())).expect("empty document json");
    let (svg, width, height) = gis2d_document_json_to_svg(&value).expect("svg export");
    assert!(svg.contains("<svg"), "{svg}");
    assert_eq!(width, 256);
    assert_eq!(height, 256);
}

#[semio_framework_async_macros::async_test]
async fn feature_collection_diffing_emits_create_replace_and_delete() {
    let feature = |id: &str, label: &str| MapFeature { id: id.into(), data: value_to_dsl(&serde_json::json!({ "id": id, "label": label })) };
    let before = vec![feature("keep", "a"), feature("gone", "b")];
    let after = vec![feature("keep", "changed"), feature("new", "c")];
    let operations = positions_operations(&before, &after);
    assert!(operations.iter().any(|operation| matches!(operation, GisMapMutation::DeletePosition(payload) if payload.id == "gone")));
    assert!(operations.iter().any(|operation| matches!(operation, GisMapMutation::ReplacePositionData(payload) if payload.id == "keep")));
    assert!(operations.iter().any(|operation| matches!(operation, GisMapMutation::CreatePosition(payload) if payload.item.id == "new")));
    assert!(routes_operations(&before, &before).is_empty(), "an unchanged collection produces no operations");
    assert!(regions_operations(&before, &before).is_empty());
}
