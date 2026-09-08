
use super::*;

fn rect_frame(id: &str, visible: Option<bool>) -> crate::Frame {
    crate::Frame::Rect { id: id.into(), layer_id: "layer-1".into(), bounds: crate::LayoutBounds { x: 0.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0 }, locked: None, visible, fill: None, stroke: None }
}

fn base_doc() -> crate::LayoutSnapshot {
    crate::LayoutSnapshot {
        schema: LAYOUT_DOCUMENT_SCHEMA.into(),
        name: "t".into(),
        grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
        paragraph_styles: Vec::new(),
        character_styles: Vec::new(),
        stories: Vec::new(),
        links: Vec::new(),
        parent_pages: Vec::new(),
        spreads: Vec::new(),
        pages: Vec::new(),
        print_target: None,
        data_fields_json: None,
        background_drawing: None,
        referenced_model: None,
    }
}

#[semio_framework_async_macros::async_test]
async fn resolve_page_marks_overridden_parent_frames_and_ignores_missing_parent() {
    let mut doc = base_doc();
    doc.parent_pages.push(ParentPage { id: "parent-1".into(), name: "Master".into(), width: 100.0, height: 100.0, layer_ids: vec!["layer-1".into()], layers: Vec::new(), frames: vec![rect_frame("frame-a", None), rect_frame("frame-b", None)] });

    let page_with_parent = Page {
        id: "page-1".into(),
        name: "P1".into(),
        spread_id: "spread-1".into(),
        parent_page_id: Some("parent-1".into()),
        width: 100.0,
        height: 100.0,
        margins: crate::PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
        columns: crate::PageColumns { count: 1, gutter: 0.0 },
        guides: Vec::new(),
        layer_ids: Vec::new(),
        layers: Vec::new(),
        frames: Vec::new(),
        overrides: vec![crate::PageOverride { object_id: "frame-a".into(), bounds: None, visible: None, locked: None }],
    };
    let resolved = resolve_page(&doc, &page_with_parent);
    assert_eq!(resolved.len(), 2);
    let a = resolved.iter().find(|r| r.frame.id() == "frame-a").expect("frame-a resolved");
    assert!(!a.inherited, "overridden parent frame must not be marked inherited");
    let b = resolved.iter().find(|r| r.frame.id() == "frame-b").expect("frame-b resolved");
    assert!(b.inherited, "non-overridden parent frame stays inherited");

    let mut page_missing_parent = page_with_parent.clone();
    page_missing_parent.parent_page_id = Some("no-such-parent".into());
    assert!(resolve_page(&doc, &page_missing_parent).is_empty());

    let mut page_no_parent = page_with_parent;
    page_no_parent.parent_page_id = None;
    page_no_parent.frames = vec![rect_frame("frame-own", None)];
    let own_only = resolve_page(&doc, &page_no_parent);
    assert_eq!(own_only.len(), 1);
    assert!(!own_only[0].inherited);
}

#[semio_framework_async_macros::async_test]
async fn parse_layout_document_rejects_wrong_schema_and_invalid_json() {
    let wrong_schema = r#"{"schema":"other.schema","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[]}"#;
    let error = parse_layout_document(wrong_schema).expect_err("wrong schema must fail");
    assert!(matches!(error, crate::io::LayoutError::UnexpectedSchema(schema) if schema == "other.schema"));

    let invalid_json = "not json";
    let error = parse_layout_document(invalid_json).expect_err("invalid json must fail");
    assert!(matches!(error, crate::io::LayoutError::Json(_)));
}

#[semio_framework_async_macros::async_test]
async fn rgba_text_round_trips() {
    assert_eq!(rgba_to_text(&Some([0.1, 0.2, 0.3, 1.0])), "0.1, 0.2, 0.3, 1");
    assert_eq!(rgba_to_text(&None), "");
    assert_eq!(text_to_rgba("0.5, 0.4, 0.3, 1"), Some([0.5, 0.4, 0.3, 1.0]));
    assert_eq!(text_to_rgba("not, a, color"), None);
}
