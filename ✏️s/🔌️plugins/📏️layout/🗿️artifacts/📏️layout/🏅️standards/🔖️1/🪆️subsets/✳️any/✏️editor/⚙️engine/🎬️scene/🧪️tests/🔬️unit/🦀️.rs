use super::*;

fn sample_document() -> LayoutSnapshot {
    crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::LAYOUT_SAMPLE_TEXT).expect("sample fixture parses")
}

#[semio_framework_async_macros::async_test]
async fn builds_scene_from_empty_document() {
    let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":[],"layers":[],"frames":[],"overrides":[]}]}"#;
    let camera = Camera { x: 0.0, y: 0.0, zoom: 1.0 };
    let viewport = Viewport { width: 400, height: 300, dpr: 1.0 };
    let query = SceneQuery { page_id: "page-1", selected_ids: &[], hovered_id: None, chrome_blueprint: true, camera: &camera, viewport: &viewport };
    let mut engine = LayoutEngine::new();
    let scene = build_scene_from_document_json(&mut engine, json, &query, None).expect("scene");
    let _ = scene;
}

#[semio_framework_async_macros::async_test]
async fn hit_test_respects_camera_zoom() {
    let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":400,"height":400,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-1"]}],"frames":[{"id":"frame-1","layerId":"layer-1","kind":"rect","bounds":{"x":10,"y":10,"w":40,"h":40,"rotation":0},"fill":[1,1,1,1]}],"overrides":[]}]}"#;
    let camera = Camera { x: 0.0, y: 0.0, zoom: 0.5 };
    let viewport = Viewport { width: 400, height: 300, dpr: 1.0 };
    let query = SceneQuery { page_id: "page-1", selected_ids: &[], hovered_id: None, chrome_blueprint: true, camera: &camera, viewport: &viewport };
    let mut engine = LayoutEngine::new();
    let hit = hit_test_document_json(&mut engine, json, 210.0, 160.0, &query).expect("hit");
    assert_eq!(hit.as_deref(), Some("frame-1"));
}

#[semio_framework_async_macros::async_test]
async fn marks_hovered_frame_rect() {
    let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-1"]}],"frames":[{"id":"frame-1","layerId":"layer-1","kind":"rect","bounds":{"x":10,"y":10,"w":40,"h":40,"rotation":0},"fill":[1,1,1,1]}],"overrides":[]}]}"#;
    let doc = parse_layout_document(json).expect("doc");
    let page = doc.pages.first().expect("page");
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, &doc, page, "page-1", &[], Some("frame-1"), true);
    assert!(list.rects.iter().any(|rect| rect.object_id == "frame-1" && rect.hovered));
    assert!(list.rects.iter().all(|rect| rect.object_id != "frame-1" || rect.hovered));
}

#[semio_framework_async_macros::async_test]
async fn scene_and_hit_test_error_when_page_missing() {
    let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":100,"height":100,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":[],"layers":[],"frames":[],"overrides":[]}]}"#;
    let camera = Camera { x: 0.0, y: 0.0, zoom: 1.0 };
    let viewport = Viewport { width: 100, height: 100, dpr: 1.0 };
    let query = SceneQuery { page_id: "missing-page", selected_ids: &[], hovered_id: None, chrome_blueprint: true, camera: &camera, viewport: &viewport };
    let mut engine = LayoutEngine::new();
    assert!(matches!(build_scene_from_document_json(&mut engine, json, &query, None), Err(LayoutError::PageNotFound(id)) if id == "missing-page"));
    let hit = hit_test_document_json(&mut engine, json, 0.0, 0.0, &query);
    assert!(matches!(hit, Err(LayoutError::PageNotFound(id)) if id == "missing-page"));
}

#[semio_framework_async_macros::async_test]
async fn hit_test_returns_none_for_empty_space() {
    let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":400,"height":400,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-1"]}],"frames":[{"id":"frame-1","layerId":"layer-1","kind":"rect","bounds":{"x":10,"y":10,"w":40,"h":40,"rotation":0},"fill":[1,1,1,1]}],"overrides":[]}]}"#;
    let camera = Camera { x: 0.0, y: 0.0, zoom: 1.0 };
    let viewport = Viewport { width: 400, height: 400, dpr: 1.0 };
    let query = SceneQuery { page_id: "page-1", selected_ids: &[], hovered_id: None, chrome_blueprint: false, camera: &camera, viewport: &viewport };
    let mut engine = LayoutEngine::new();
    let hit = hit_test_document_json(&mut engine, json, 300.0, 300.0, &query).expect("hit test");
    assert!(hit.is_none());
}

#[semio_framework_async_macros::async_test]
async fn display_list_hit_test_matches_image_bounds_and_misses_elsewhere() {
    let list = DisplayList {
        page_id: "page-1".into(),
        page_width: 100.0,
        page_height: 100.0,
        rects: Vec::new(),
        text_runs: Vec::new(),
        images: vec![DisplayImage { object_id: "img-1".into(), x: 10.0, y: 10.0, width: 20.0, height: 20.0, placeholder: false }],
        guides: Vec::new(),
    };
    assert_eq!(list.hit_test(15.0, 15.0).as_deref(), Some("img-1"));
    assert!(list.hit_test(90.0, 90.0).is_none());
}

#[semio_framework_async_macros::async_test]
async fn guides_omitted_for_non_active_page_even_with_chrome_blueprint() {
    let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":10,"right":10,"bottom":10,"left":10},"columns":{"count":2,"gutter":4},"guides":[{"x":5,"y":5,"w":1,"h":1}],"layerIds":[],"layers":[],"frames":[],"overrides":[]}]}"#;
    let doc = parse_layout_document(json).expect("doc");
    let page = doc.pages.first().expect("page");
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, &doc, page, "different-active-page", &[], None, true);
    assert!(list.guides.is_empty(), "guides must only render for the active blueprint page");
}

#[semio_framework_async_macros::async_test]
async fn baseline_guides_only_emitted_when_grid_snaps() {
    let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":[],"layers":[],"frames":[],"overrides":[]}]}"#;
    let doc = parse_layout_document(json).expect("doc");
    let page = doc.pages.first().expect("page");
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, &doc, page, "page-1", &[], None, true);
    assert!(list.guides.iter().all(|guide| guide.kind != "baseline"));
}

#[semio_framework_async_macros::async_test]
async fn image_placeholder_reflects_link_lookup_and_state() {
    let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[{"id":"link-missing","path":"a.png","hash":"h","width":1,"height":1,"dpi":72,"state":"missing"},{"id":"link-ready","path":"b.png","hash":"h","width":1,"height":1,"dpi":72,"state":"ready","proxyDataUrl":"data:image/png;base64,AA=="}],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":400,"height":400,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["img-missing","img-ready","img-unlinked"]}],"frames":[{"id":"img-missing","layerId":"layer-1","kind":"image","bounds":{"x":0,"y":0,"w":10,"h":10,"rotation":0},"linkId":"link-missing"},{"id":"img-ready","layerId":"layer-1","kind":"image","bounds":{"x":20,"y":0,"w":10,"h":10,"rotation":0},"linkId":"link-ready"},{"id":"img-unlinked","layerId":"layer-1","kind":"image","bounds":{"x":40,"y":0,"w":10,"h":10,"rotation":0},"linkId":"link-gone"}],"overrides":[]}]}"#;
    let doc = parse_layout_document(json).expect("doc");
    let page = doc.pages.first().expect("page");
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, &doc, page, "page-1", &[], None, false);
    let by_id = |id: &str| list.images.iter().find(|i| i.object_id == id).expect("image present");
    assert!(by_id("img-missing").placeholder, "missing-state link stays a placeholder");
    assert!(!by_id("img-ready").placeholder, "ready link with a proxy is not a placeholder");
    assert!(by_id("img-unlinked").placeholder, "unresolved link falls back to placeholder");
}

#[semio_framework_async_macros::async_test]
async fn layout_story_in_frame_resolves_alignment_variants_and_detects_overset() {
    let mut engine = LayoutEngine::new();
    let story = TextStory { id: "story-1".into(), content: "Hello layout engine, this line should wrap across several lines of text.".into(), style_runs: Vec::new() };
    for alignment in ["left", "center", "middle", "right", "justify", "justified", "unrecognized"] {
        let paragraph = ParagraphStyle { id: "p".into(), name: "Body".into(), font_family: "Layout Sans".into(), font_size: 12.0, font_weight: 400, leading: 14.4, tracking: 0.0, alignment: alignment.into() };
        let (shaped, overset) = layout_story_in_frame(&mut engine, &story, &paragraph, 80.0, 10.0);
        assert!(shaped.height > 0.0, "alignment {alignment} should still measure a positive height");
        assert!(overset, "narrow/short frame with long content should overset for alignment {alignment}");
    }
    let paragraph = ParagraphStyle { id: "p".into(), name: "Body".into(), font_family: "Layout Sans".into(), font_size: 12.0, font_weight: 400, leading: 14.4, tracking: 0.0, alignment: "left".into() };
    let (_, not_overset) = layout_story_in_frame(&mut engine, &story, &paragraph, 2000.0, 2000.0);
    assert!(!not_overset);
}

#[semio_framework_async_macros::async_test]
async fn display_list_to_scene_handles_drop_preview_variants_and_rect_styles() {
    let camera = Camera { x: 0.0, y: 0.0, zoom: 1.0 };
    let viewport = Viewport { width: 200, height: 200, dpr: 1.0 };
    let list = DisplayList {
        page_id: "page-1".into(),
        page_width: 200.0,
        page_height: 200.0,
        rects: vec![
            DisplayRect {
                object_id: "r-explicit-stroke".into(),
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
                fill: Some(DisplayColor([1.0, 1.0, 1.0, 1.0])),
                stroke: Some(DisplayColor([0.0, 0.0, 0.0, 1.0])),
                inherited: false,
                selected: true,
                hovered: false,
            },
            DisplayRect { object_id: "r-implicit-hover".into(), x: 20.0, y: 0.0, width: 10.0, height: 10.0, fill: None, stroke: None, inherited: false, selected: false, hovered: true },
            DisplayRect { object_id: "r-implicit-select".into(), x: 40.0, y: 0.0, width: 10.0, height: 10.0, fill: None, stroke: None, inherited: false, selected: true, hovered: false },
        ],
        text_runs: vec![DisplayTextRun { object_id: "text-1".into(), glyphs: vec![DisplayGlyph { glyph_id: 1, font_size: 12.0, x: 0.0, y: 0.0, color: DisplayColor([0.0, 0.0, 0.0, 1.0]) }] }],
        images: vec![DisplayImage { object_id: "img-1".into(), x: 0.0, y: 60.0, width: 10.0, height: 10.0, placeholder: true }],
        guides: vec![DisplayGuide { rect: LayoutRect { x: 0.0, y: 0.0, width: 10.0, height: 0.0 }, kind: "unrecognized".into() }],
    };
    for kind in ["page", "rect", "text", "image", "unrecognized"] {
        let preview = LayoutDropPreview { kind: kind.into(), x: 5.0, y: 5.0 };
        let scene = display_list_to_scene(&list, true, &camera, &viewport, Some(&preview));
        let _ = scene;
    }
    let scene = display_list_to_scene(&list, false, &camera, &viewport, None);
    let _ = scene;
}

#[semio_framework_async_macros::async_test]
async fn screen_to_world_json_returns_a_point_object() {
    let camera = Camera { x: 100.0, y: 50.0, zoom: 2.0 };
    let viewport = Viewport { width: 400, height: 300, dpr: 1.0 };
    let json = screen_to_world_json(&camera, &viewport, 210.0, 160.0);
    let parsed: Value = serde_json::from_str(&json).expect("valid json point");
    assert!(parsed["x"].is_number());
    assert!(parsed["y"].is_number());
}

#[semio_framework_async_macros::async_test]
async fn png_cpu_export_writes_valid_rgba_png() {
    let doc = sample_document();
    let bytes = export_document_png_headless_batch(&doc, "page-1").expect("png export succeeds");
    assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G']));
}

#[semio_framework_async_macros::async_test]
async fn pdf_export_writes_pdf_header() {
    let doc = sample_document();
    let bytes = export_document_pdf_headless_batch(&doc, "page-1").expect("pdf export succeeds");
    assert!(bytes.starts_with(b"%PDF-1.4"));
}

#[semio_framework_async_macros::async_test]
async fn package_zip_bundles_document_and_preflight() {
    let doc = sample_document();
    let json = dsl::os_pack::to_json_string(&doc);
    let bytes = export_package_zip_headless_batch(&json, "[]").expect("package export succeeds");
    assert_eq!(doc.schema, crate::LAYOUT_DOCUMENT_SCHEMA);
    assert!(bytes.starts_with(b"PK"));
}

#[semio_framework_async_macros::async_test]
async fn svg_export_contains_path_and_wraps_a_valid_document() {
    crate::io::ensure_stdio_semio_drawing_registered();
    let doc = sample_document();
    let svg = export_document_svg_headless_batch(&doc, "page-1").expect("svg export succeeds");
    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("<rect"));
    assert!(svg.ends_with("</svg>"));
}

#[semio_framework_async_macros::async_test]
async fn exports_error_when_page_missing() {
    let doc = sample_document();
    assert!(matches!(export_document_svg_headless_batch(&doc, "no-such-page"), Err(LayoutError::PageNotFound(id)) if id == "no-such-page"));
    assert!(matches!(export_document_pdf_headless_batch(&doc, "no-such-page"), Err(LayoutError::PageNotFound(_))));
    assert!(matches!(export_document_png_headless_batch(&doc, "no-such-page"), Err(LayoutError::PageNotFound(_))));
}

#[semio_framework_async_macros::async_test]
async fn package_zip_rejects_invalid_document_json() {
    let error = export_package_zip_headless_batch("not json", "[]").expect_err("invalid json must fail");
    assert!(matches!(error, LayoutError::Json(_)));
}
