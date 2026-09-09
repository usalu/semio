use super::*;
use dsl::os_pack::json::Value;

#[test]
fn renders_canvas_2d_scene() {
    let deck = crate::default_presentation_snapshot();
    // 🌱️ `BuiltNode` deliberately has no `ToValue`/`FromValue` (framework `🦀️builder.rs`'s own
    // "DslValue-free exception" for `UiValue`-embedding types), so this reads the surface kind
    // back off the `Debug` rendering instead of round-tripping through JSON.
    let debug_str = format!("{:?}", render(&deck));
    assert!(debug_str.contains("canvas-2d") || debug_str.contains("Canvas2d"));
}

#[test]
fn definition_declares_the_canvas_2d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, BODY_KEY);
    assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
    assert!(definition.options.measures.is_empty(), "animate view declares no live chrome measures");
    assert!(definition.actions.is_empty(), "a viewer window declares no mutating actions");
}

#[test]
fn source_frame_renders_as_actual_image_layer_behind_tiles() {
    let deck = crate::default_presentation_snapshot();
    let layers_json = deck_to_canvas_layers(&deck);
    let layers: Vec<Value> = dsl::os_pack::json::parse(&layers_json).unwrap().as_array().cloned().unwrap_or_default();
    let (source, _) = crate::presentation_working_scene(&deck);
    assert!(!source.src.trim().is_empty());
    let source_layer = layers.first().expect("source layer is first (renders behind tiles)");
    assert_eq!(source_layer.get("id").and_then(|v| v.as_str()), Some("source-frame"));
    assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("image"));
    assert_eq!(source_layer.get("dataUrl").and_then(|v| v.as_str()), Some(source.src.as_str()));
}

#[test]
fn deck_to_canvas_layers_omits_data_url_when_source_has_no_image() {
    let base = crate::default_presentation_snapshot();
    let (mut source, tiles) = crate::presentation_working_scene(&base);
    source.src = String::new();
    let deck = crate::presentation_snapshot_with_tiles(&source, &tiles);
    let layers_json = deck_to_canvas_layers(&deck);
    let layers: Vec<Value> = dsl::os_pack::json::parse(&layers_json).unwrap().as_array().cloned().unwrap_or_default();
    let source_layer = layers.first().expect("source layer presentation");
    assert_eq!(source_layer.get("kind").and_then(|v| v.as_str()), Some("source"));
    assert!(source_layer.get("dataUrl").is_none() || source_layer.get("dataUrl") == Some(&Value::Null));
}
