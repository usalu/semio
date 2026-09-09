use super::*;
use crate::{LayoutBounds, Page, PageColumns, PageMargins};

fn sample_with_composition() -> LayoutSnapshot {
    let mut snapshot = empty_layout_snapshot();
    snapshot.schema = LAYOUT_DOCUMENT_SCHEMA.into();
    snapshot.name = "Composed".into();
    snapshot.paragraph_styles = vec![ParagraphStyle { id: "paragraph.body".into(), name: "Body".into(), font_family: "Layout Sans".into(), font_size: 12.0, font_weight: 400, leading: 14.4, tracking: 0.0, alignment: "left".into() }];
    snapshot.pages = vec![Page {
        id: "page-1".into(),
        name: "Page 1".into(),
        spread_id: "spread-1".into(),
        parent_page_id: None,
        width: 200.0,
        height: 200.0,
        margins: PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
        columns: PageColumns { count: 1, gutter: 0.0 },
        guides: Vec::new(),
        layer_ids: Vec::new(),
        layers: Vec::new(),
        frames: vec![Frame::Rect { id: "frame-1".into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0 }, locked: None, visible: None, fill: Some([1.0, 1.0, 1.0, 1.0]), stroke: None }],
        overrides: Vec::new(),
    }];
    snapshot.background_drawing =
        Some(LayoutDrawingChild { handle: store::ArtifactChild::new("child-drawing-1".to_string(), store::os_io::ArtifactRef::parse_uri("doc-1!s.stdio.semio@v1/drawing").expect("valid child ref uri")), content: Default::default() });
    snapshot.referenced_model = Some(store::ArtifactLink { target: store::os_io::ArtifactRef::parse_uri("doc-2!s.stdio.semio@v1/model").expect("valid link ref uri"), pin: store::LinkPin::Head, role: "model".into() });
    snapshot
}

/// 🧪️ Every field on `LayoutSnapshot` — including the two new composition slots — must survive
/// both hand-rolled codecs (text and binary), independently. Codec completeness is not caught by
/// `cargo check`; this is the real round-trip proof the migration recipe requires.
#[semio_framework_async_macros::async_test]
async fn background_drawing_and_referenced_model_round_trip_through_text_and_binary() {
    let snapshot = sample_with_composition();
    let text = store::ArtifactDsl::print_dsl(&snapshot);
    let from_text = <LayoutSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse round-tripped text");
    assert_eq!(from_text, snapshot);

    let bytes = store::ArtifactPack::encode_pack(&snapshot);
    let from_binary = <LayoutSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode round-tripped binary");
    assert_eq!(from_binary, snapshot);
}

#[semio_framework_async_macros::async_test]
async fn absent_composition_slots_round_trip_as_none() {
    let mut snapshot = sample_with_composition();
    snapshot.background_drawing = None;
    snapshot.referenced_model = None;
    let text = store::ArtifactDsl::print_dsl(&snapshot);
    assert_eq!(<LayoutSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), snapshot);
    let bytes = store::ArtifactPack::encode_pack(&snapshot);
    assert_eq!(<LayoutSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode"), snapshot);
}
