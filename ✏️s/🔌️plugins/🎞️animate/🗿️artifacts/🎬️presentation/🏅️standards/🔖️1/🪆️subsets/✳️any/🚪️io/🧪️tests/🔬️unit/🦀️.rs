use super::*;
use semio_framework_os_kernel::json::{object, Object, Value};

#[test]
fn title_cards_match_the_neutral_xml_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️title-cards.json")).expect("neutral vectors");
    for row in vectors["cases"].as_array().expect("cases") {
        let value = dsl::os_pack::json::parse(&row["document"].to_string()).expect("owned JSON");
        let (svg, width, height) = animate_presentation_document_json_to_svg(&value).expect("title card");
        let parsed = roxmltree::Document::parse(&svg).expect("independent XML parser");
        let root = parsed.root_element();
        assert_eq!(root.tag_name().name(), "svg");
        assert_eq!(root.attribute("width"), Some(width.to_string().as_str()));
        assert_eq!(root.attribute("height"), Some(height.to_string().as_str()));
        assert_eq!(u64::from(width), vectors["width"].as_u64().expect("width"));
        assert_eq!(u64::from(height), vectors["height"].as_u64().expect("height"));
        let title = root.descendants().find(|node| node.has_tag_name("text")).expect("title");
        assert_eq!(title.text(), row["title"].as_str());
    }
}

#[test]
fn animate_presentation_document_json_to_svg_embeds_title() {
    let document = object([("title".to_string(), Value::from("My Deck"))]);
    let (svg, width, height) = animate_presentation_document_json_to_svg(&document).expect("svg");
    assert!(svg.contains("My Deck"));
    assert_eq!((width, height), (1280, 720));
}

#[test]
fn animate_presentation_document_json_to_svg_falls_back_to_app_label_without_title() {
    let document = Value::Object(Object::new());
    let (svg, _, _) = animate_presentation_document_json_to_svg(&document).expect("svg fallback");
    assert!(svg.contains("Animate Presentation"));
}

#[test]
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn from_dwg_builds_single_slide_deck_from_entity() {
    let drawing = semio_s_artifact_stdio_dwg::DwgDrawing {
        layers: vec![semio_s_artifact_stdio_dwg::DwgLayer::default()],
        entities: vec![semio_s_artifact_stdio_dwg::DwgEntity {
            layer: 0,
            color: semio_s_artifact_stdio_dwg::DwgColor::ByLayer,
            geometry: semio_s_artifact_stdio_dwg::DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]], bulges: vec![0.0, 0.0, 0.0, 0.0] },
        }],
        extmin: [0.0, 0.0, 0.0],
        extmax: [10.0, 10.0, 0.0],
    };
    let document = animate_presentation_document_json_from_dwg(&drawing).expect("from_dwg");
    let deck: crate::PresentationSnapshot = dsl::FromValue::from_value(document).expect("deck");
    assert_eq!(deck.schema, crate::PRESENTATION_DOCUMENT_SCHEMA);
    let (source, tiles) = crate::presentation_working_scene(&deck);
    assert_eq!(tiles.len(), 1);
    assert_eq!(tiles[0].name, "Imported Drawing");
    assert!(source.src.starts_with("data:image/png;base64,"));
}

#[test]
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn from_dwg_never_errors_on_empty_drawing() {
    let drawing = semio_s_artifact_stdio_dwg::DwgDrawing::default();
    let document = animate_presentation_document_json_from_dwg(&drawing).expect("from_dwg on empty drawing");
    let deck: crate::PresentationSnapshot = dsl::FromValue::from_value(document).expect("deck");
    let (_, tiles) = crate::presentation_working_scene(&deck);
    assert_eq!(tiles.len(), 1);
}
