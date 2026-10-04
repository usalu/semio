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
        strokes: Vec::new(),
        frame_strokes: Vec::new(),
        images: vec![DisplayImage { object_id: "img-1".into(), x: 10.0, y: 10.0, width: 20.0, height: 20.0, rotation: 0.0, placeholder: false, proxy_data_url: None, preview: String::new(), stack: 0 }],
        guides: Vec::new(),
    };
    assert_eq!(list.hit_test(15.0, 15.0).as_deref(), Some("img-1"));
    assert!(list.hit_test(90.0, 90.0).is_none());
}

#[semio_framework_async_macros::async_test]
async fn bounds_hit_test_finds_a_text_frame_without_shaping() {
    let doc = crate::standards::v1::subsets::any::schema::default_document();
    let page = doc.pages.iter().find(|page| page.id == "page-1").expect("page");
    assert_eq!(hit_test_page_frames(&doc, page, 160.0, 230.0).as_deref(), Some("frame-text-1"));
    assert!(hit_test_page_frames(&doc, page, 1.0, 1.0).is_none());
}

#[semio_framework_async_macros::async_test]
async fn hit_test_uses_frame_rotation() {
    let mut doc = crate::standards::v1::subsets::any::schema::default_document();
    let frame = doc.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-1").expect("frame");
    let crate::Frame::Rect { bounds, .. } = frame else { panic!("rect") };
    *bounds = crate::LayoutBounds { x: 0.0, y: 0.0, width: 100.0, height: 20.0, rotation: std::f64::consts::FRAC_PI_2 };
    let page = doc.pages.first().expect("page");
    assert_eq!(hit_test_page_frames(&doc, page, 50.0, 50.0).as_deref(), Some("frame-1"), "a point on the rotated long axis hits");
    assert!(hit_test_page_frames(&doc, page, 0.0, 80.0).is_none(), "a point outside the rotated frame misses");
}

#[semio_framework_async_macros::async_test]
async fn interactive_display_skips_frames_outside_the_view_and_keeps_the_selection() {
    let mut doc = crate::standards::v1::subsets::any::schema::default_document();
    doc.pages[0].frames.push(crate::Frame::Rect {
        id: "far".into(),
        layer_id: "layer-1".into(),
        bounds: crate::LayoutBounds { x: 100_000.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0 },
        locked: None,
        visible: None,
        fill: None,
        stroke: None,
    });
    let page = doc.pages.first().expect("page");
    let culled = build_interactive_display_list(&doc, page, &page.id, &[], None, true, 0.0, 0.0, 1.0);
    assert!(culled.rects.iter().any(|rect| rect.object_id == "frame-1"));
    assert!(culled.rects.iter().all(|rect| rect.object_id != "far"));
    let kept = build_interactive_display_list(&doc, page, &page.id, &["far".into()], None, true, 0.0, 0.0, 1.0);
    assert!(kept.rects.iter().any(|rect| rect.object_id == "far"));
}

#[semio_framework_async_macros::async_test]
async fn interactive_display_omits_the_baseline_lattice() {
    let doc = crate::standards::v1::subsets::any::schema::default_document();
    let page = doc.pages.first().expect("page");
    let interactive = build_interactive_display_list(&doc, page, &page.id, &[], None, true, 0.0, 0.0, 1.0);
    assert!(interactive.guides.iter().all(|guide| guide.kind != "baseline"));
    let mut engine = LayoutEngine::new();
    let accurate = build_display_list_for_page(&mut engine, &doc, page, &page.id, &[], None, true);
    assert!(accurate.guides.iter().any(|guide| guide.kind == "baseline"));
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
                rotation: 0.0,
                fill: Some(DisplayColor([1.0, 1.0, 1.0, 1.0])),
                stroke: Some(DisplayColor([0.0, 0.0, 0.0, 1.0])),
                inherited: false,
                selected: true,
                hovered: false,
                stack: 0,
            },
            DisplayRect { object_id: "r-implicit-hover".into(), x: 20.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0, fill: None, stroke: None, inherited: false, selected: false, hovered: true, stack: 0 },
            DisplayRect { object_id: "r-implicit-select".into(), x: 40.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0, fill: None, stroke: None, inherited: false, selected: true, hovered: false, stack: 0 },
        ],
        text_runs: vec![DisplayTextRun { object_id: "text-1".into(), glyphs: vec![DisplayGlyph { glyph_id: 1, font_size: 12.0, x: 0.0, y: 12.0, color: DisplayColor([0.0, 0.0, 0.0, 1.0]), italic: false }], content: "Hi".into(), origin_x: 0.0, origin_y: 0.0, font_size: 12.0, stack: 0, color: [0.0, 0.0, 0.0, 1.0] }],
        strokes: Vec::new(),
        frame_strokes: Vec::new(),
        images: vec![DisplayImage { object_id: "img-1".into(), x: 0.0, y: 60.0, width: 10.0, height: 10.0, rotation: 0.0, placeholder: true, proxy_data_url: None, preview: String::new(), stack: 0 }],
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
    let json = semio_framework_pack_json::to_json_string(&doc);
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

#[semio_framework_async_macros::async_test]
async fn placed_drawing_shapes_its_kind_and_keeps_a_stroke_mark() {
    let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[{"id":"link-drawing","path":"plan.dwg","hash":"h","width":1,"height":1,"dpi":72,"state":"missing","artifactKind":"drawing","artifactRef":"drawing-1"}],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":400,"height":400,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["img-drawing"]}],"frames":[{"id":"img-drawing","layerId":"layer-1","kind":"image","bounds":{"x":10,"y":20,"w":80,"h":50,"rotation":0},"linkId":"link-drawing"}],"overrides":[]}]}"#;
    let doc = parse_layout_document(json).expect("doc");
    let page = doc.pages.first().expect("page");
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, &doc, page, "page-1", &[], None, false);
    let image = list.images.iter().find(|image| image.object_id == "img-drawing").expect("image");
    assert_eq!(image.preview, "stroke");
    let run = list.text_runs.iter().find(|run| run.content == "drawing").expect("kind label");
    assert_eq!(run.glyphs.len(), "drawing".chars().count());
}

#[semio_framework_async_macros::async_test]
async fn character_style_run_changes_glyph_size_and_color() {
    let mut doc = crate::standards::v1::subsets::any::schema::default_document();
    doc.character_styles.push(crate::CharacterStyle { id: "character-1".into(), name: Some("Emphasis".into()), font_family: None, font_size: Some(24.0), font_weight: None, italic: None, color: Some([1.0, 0.0, 0.0, 1.0]), tracking: None });
    let story = doc.stories.iter_mut().find(|story| story.id == "story-1").unwrap();
    story.style_runs.push(crate::TextStyleRun { start: 0, end: 5, paragraph_style_id: None, character_style_id: Some("character-1".into()) });
    let page = doc.pages.iter().find(|page| page.id == "page-1").unwrap().clone();
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, &doc, &page, "page-1", &[], None, false);
    let run = list.text_runs.iter().find(|run| run.content == "Hello layout").expect("story");
    assert_eq!(run.glyphs.len(), "Hello layout".chars().count());
    assert!(run.glyphs[..5].iter().all(|glyph| glyph.font_size == 24.0 && glyph.color.0[0] == 1.0));
    assert!(run.glyphs[5..].iter().all(|glyph| glyph.font_size == 12.0 && glyph.color.0 == [0.0, 0.0, 0.0, 1.0]));
}

#[semio_framework_async_macros::async_test]
async fn page_override_moves_and_hides_an_inherited_frame() {
    let mut doc = crate::standards::v1::subsets::any::schema::default_document();
    doc.pages[0].overrides.push(crate::PageOverride { object_id: "frame-inherited".into(), bounds: Some(crate::LayoutBounds { x: 90.0, y: 50.0, width: 100.0, height: 80.0, rotation: 0.0 }), visible: None, locked: None });
    let page = doc.pages[0].clone();
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, &doc, &page, "page-1", &[], None, false);
    let rect = list.rects.iter().find(|rect| rect.object_id == "frame-inherited").expect("inherited");
    assert_eq!(rect.x, 90.0);
    doc.pages[0].overrides[0].visible = Some(false);
    let page = doc.pages[0].clone();
    let list = build_display_list_for_page(&mut engine, &doc, &page, "page-1", &[], None, false);
    assert!(list.rects.iter().all(|rect| rect.object_id != "frame-inherited"));
}

#[semio_framework_async_macros::async_test]
async fn hidden_layer_drops_its_frames_and_keeps_the_master() {
    let mut doc = crate::standards::v1::subsets::any::schema::default_document();
    doc.pages[0].layers[0].visible = false;
    let page = doc.pages[0].clone();
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, &doc, &page, "page-1", &[], None, false);
    assert!(list.rects.iter().all(|rect| rect.object_id != "frame-1"));
    assert!(list.images.iter().all(|image| image.object_id != "frame-image-1"));
    assert!(list.text_runs.iter().all(|run| run.object_id != "frame-text-1"));
    assert!(list.rects.iter().any(|rect| rect.object_id == "frame-inherited"));
}

#[semio_framework_async_macros::async_test]
async fn moving_a_frame_onto_a_hidden_layer_drops_only_that_frame() {
    use crate::mutations::create_layer::CreateLayer;
    use crate::mutations::set_frame_layer::SetFrameLayer;
    use crate::mutations::LayoutMutation;
    use protocol::{Mutation, MutationDiff};
    let base = crate::standards::v1::subsets::any::schema::default_document();
    let created = LayoutMutation::CreateLayer(CreateLayer { page_id: "page-1".into(), id: "layer-2".into(), name: "Notes".into(), remove: false }).diff(&base).diff().apply(&base).expect("layer");
    let mut doc = LayoutMutation::SetFrameLayer(SetFrameLayer { page_id: "page-1".into(), frame_id: "frame-1".into(), layer_id: "layer-2".into() }).diff(&created).diff().apply(&created).expect("move");
    doc.pages[0].layers.iter_mut().find(|layer| layer.id == "layer-2").unwrap().visible = false;
    let page = doc.pages[0].clone();
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, &doc, &page, "page-1", &[], None, false);
    assert!(list.rects.iter().all(|rect| rect.object_id != "frame-1"));
    assert!(list.text_runs.iter().any(|run| run.content == "Hello layout"));
}

#[test]
fn background_drawing_fits_an_offset_plan_onto_the_page() {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioTransform};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawLayer, DrawNode, PathSegment, SemioDrawingSnapshot};
    let point = |x: f64, y: f64| SemioPoint2 { x, y };
    let content = SemioDrawingSnapshot {
        schema: "stdio.semio.drawing".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![DrawLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform::identity(),
                children: vec![DrawNode::Path {
                    style: None,
                    segments: vec![
                        PathSegment::MoveTo { to: point(10.0, 20.0) },
                        PathSegment::LineTo { to: point(110.0, 20.0) },
                        PathSegment::LineTo { to: point(110.0, 70.0) },
                        PathSegment::LineTo { to: point(10.0, 70.0) },
                        PathSegment::Close,
                    ],
                }],
            },
        }],
    };
    let mut document = crate::standards::v1::subsets::any::schema::default_document();
    document.pages[0].width = 100.0;
    document.pages[0].height = 50.0;
    document.background_drawing = Some(crate::background_drawing_child_handle("dwg", &content));
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, &document, &document.pages[0], "", &[], None, false);
    let stroke = &list.strokes[0];
    assert!(stroke.closed);
    assert_eq!(stroke.color, [0.15, 0.2, 0.28, 1.0]);
    assert_eq!(stroke.width, 1.0);
    assert!(stroke.fill.is_none());
    assert_eq!(stroke.points[0], (0.0, 0.0));
    assert_eq!(stroke.points[1], (100.0, 0.0));
    assert_eq!(stroke.points[2], (100.0, 50.0));
    document.background_drawing.as_mut().unwrap().content.layers[0].visible = false;
    let hidden = build_display_list_for_page(&mut engine, &document, &document.pages[0], "", &[], None, false);
    assert!(hidden.strokes.is_empty());
}

fn unit_square_drawing() -> semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioTransform};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawLayer, DrawNode, PathSegment, SemioDrawingSnapshot};
    let point = |x: f64, y: f64| SemioPoint2 { x, y };
    SemioDrawingSnapshot {
        schema: "stdio.semio.drawing".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![DrawLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform::identity(),
                children: vec![DrawNode::Path {
                    style: None,
                    segments: vec![
                        PathSegment::MoveTo { to: point(0.0, 0.0) },
                        PathSegment::LineTo { to: point(1.0, 0.0) },
                        PathSegment::LineTo { to: point(1.0, 1.0) },
                        PathSegment::LineTo { to: point(0.0, 1.0) },
                        PathSegment::Close,
                    ],
                }],
            },
        }],
    }
}

#[test]
fn a_placed_drawing_frame_fits_the_plan_inside_its_bounds() {
    let mut document = crate::standards::v1::subsets::any::schema::default_document();
    document.links[0].artifact_kind = "s.draw.drawing".into();
    document.links[0].artifact_ref = "plan-1".into();
    document.background_drawing = Some(crate::background_drawing_child_handle("dwg", &unit_square_drawing()));
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, &document, &document.pages[0], "", &[], None, false);
    assert_eq!(list.frame_strokes[0].points, vec![(146.0, 435.0), (186.0, 435.0), (186.0, 475.0), (146.0, 475.0)]);
    assert!(list.text_runs.iter().all(|run| run.content != "s.draw.drawing"));
    document.links[0].proxy_data_url = Some("data:image/png;base64,AA==".into());
    let proxied = build_display_list_for_page(&mut engine, &document, &document.pages[0], "", &[], None, false);
    assert!(proxied.frame_strokes.is_empty());
}

#[test]
fn drawing_text_lands_on_the_page_and_inside_the_placed_frame() {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioTransform};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawLayer, DrawNode, PathSegment, SemioDrawingSnapshot};
    let point = |x: f64, y: f64| SemioPoint2 { x, y };
    let content = SemioDrawingSnapshot {
        schema: "stdio.semio.drawing".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![DrawLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform::identity(),
                children: vec![
                    DrawNode::Path {
                        style: None,
                        segments: vec![
                            PathSegment::MoveTo { to: point(0.0, 0.0) },
                            PathSegment::LineTo { to: point(1.0, 0.0) },
                            PathSegment::LineTo { to: point(1.0, 1.0) },
                            PathSegment::LineTo { to: point(0.0, 1.0) },
                            PathSegment::Close,
                        ],
                    },
                    DrawNode::Text { value: "Plan".into(), at: point(0.0, 0.0), style: None },
                ],
            },
        }],
    };
    let mut document = crate::standards::v1::subsets::any::schema::default_document();
    document.links[0].artifact_kind = "s.draw.drawing".into();
    document.links[0].artifact_ref = "plan-1".into();
    document.background_drawing = Some(crate::background_drawing_child_handle("dwg", &content));
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, &document, &document.pages[0], "", &[], None, false);
    let labels: Vec<_> = list.text_runs.iter().filter(|run| run.content == "Plan").collect();
    assert_eq!(labels.len(), 2);
    assert!(labels.iter().any(|run| run.origin_x == 0.0 && run.origin_y == 50.0));
    assert!(labels.iter().any(|run| run.origin_x == 146.0 && run.origin_y == 435.0));
    assert!(labels.iter().all(|run| run.glyphs.len() == 4));
}

fn red_png() -> Vec<u8> {
    let mut encoded = semio_s_artifact_stdio_png::io::PngProjection {width:1,height:1,bit_depth:8,color_type:semio_s_artifact_stdio_png::schema::snapshot::PngColorType::Rgba,interlace:false,plte:None,trns:None,gama:None,chrm:None,srgb:None,phys:None,time:None,bkgd:None,text_chunks:Vec::new(),pixels:Vec::new(),chunk_order:Vec::new(),unknown_chunks:Vec::new()};
    encoded.width = 1;
    encoded.height = 1;
    encoded.pixels = vec![255, 0, 0, 255];
    semio_s_artifact_stdio_png::io::author_png_projection(&encoded).expect("png")
}

fn drawing_with_red_png() -> semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioTransform};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawLayer, DrawNode, PathSegment, SemioDrawingSnapshot};
    let point = |x: f64, y: f64| SemioPoint2 { x, y };
    SemioDrawingSnapshot {
        schema: "stdio.semio.drawing".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![DrawLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform::identity(),
                children: vec![
                    DrawNode::Path {
                        style: None,
                        segments: vec![
                            PathSegment::MoveTo { to: point(0.0, 0.0) },
                            PathSegment::LineTo { to: point(1.0, 0.0) },
                            PathSegment::LineTo { to: point(1.0, 1.0) },
                            PathSegment::LineTo { to: point(0.0, 1.0) },
                            PathSegment::Close,
                        ],
                    },
                    DrawNode::Image { at: point(0.0, 0.0), width: 1.0, height: 1.0, mime: "image/png".into(), bytes: red_png() },
                ],
            },
        }],
    }
}

#[test]
fn an_embedded_drawing_png_fits_the_page_and_the_placed_frame() {
    let mut document = crate::standards::v1::subsets::any::schema::default_document();
    document.links[0].artifact_kind = "s.draw.drawing".into();
    document.links[0].artifact_ref = "plan-1".into();
    document.background_drawing = Some(crate::background_drawing_child_handle("dwg", &drawing_with_red_png()));
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, &document, &document.pages[0], "", &[], None, false);
    let rasters: Vec<_> = list.images.iter().filter(|image| image.proxy_data_url.as_deref().is_some_and(|url| url.starts_with("data:image/png;base64,"))).collect();
    assert!(rasters.iter().any(|image| image.x == 0.0 && image.y == 50.0 && image.width == 400.0 && image.height == 400.0));
    assert!(rasters.iter().any(|image| image.x == 146.0 && image.y == 435.0 && image.width == 40.0 && image.height == 40.0));
}

#[test]
fn an_imported_arc_bulges_off_its_chord() {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioTransform};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawLayer, DrawNode, PathSegment, SemioDrawingSnapshot};
    let point = |x: f64, y: f64| SemioPoint2 { x, y };
    let content = SemioDrawingSnapshot {
        schema: "stdio.semio.drawing".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![DrawLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform::identity(),
                children: vec![DrawNode::Path {
                    style: None,
                    segments: vec![PathSegment::MoveTo { to: point(0.0, 0.0) }, PathSegment::ArcTo { rx: 1.0, ry: 1.0, x_rotation: 0.0, large_arc: false, sweep: true, to: point(2.0, 0.0) }],
                }],
            },
        }],
    };
    let mut document = crate::standards::v1::subsets::any::schema::default_document();
    document.pages[0].width = 200.0;
    document.pages[0].height = 100.0;
    document.background_drawing = Some(crate::background_drawing_child_handle("dwg", &content));
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, &document, &document.pages[0], "", &[], None, false);
    let stroke = &list.strokes[0];
    assert!(stroke.points.len() > 2, "an arc is more than its endpoints");
    assert!(stroke.points.iter().any(|(_, y)| *y < 20.0), "the semicircle leaves the chord: {:?}", stroke.points);
}

#[test]
fn a_rotated_drawing_frame_turns_the_plan_and_keeps_its_png() {
    let mut document = crate::standards::v1::subsets::any::schema::default_document();
    document.links[0].artifact_kind = "s.draw.drawing".into();
    document.links[0].artifact_ref = "plan-1".into();
    let frame = document.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-image-1").expect("image");
    let crate::Frame::Image { bounds, .. } = frame else { panic!("image") };
    bounds.rotation = std::f64::consts::FRAC_PI_2;
    document.background_drawing = Some(crate::background_drawing_child_handle("dwg", &drawing_with_red_png()));
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, &document, &document.pages[0], "", &[], None, false);
    assert_eq!(list.frame_strokes[0].points[0], (186.0, 435.0));
    let raster = list.images.iter().find(|image| image.width == 40.0 && image.proxy_data_url.as_deref().is_some_and(|url| url.starts_with("data:image/png;base64,"))).expect("rotated raster");
    assert!((raster.rotation - std::f32::consts::FRAC_PI_2).abs() < 1.0e-4);
    assert_eq!((raster.x, raster.y, raster.width, raster.height), (146.0, 435.0, 40.0, 40.0));
    let json = crate::editor::layout::canvas::canvas_layers(
        &document,
        &crate::editor::layout::modes::edit::windows::blueprint::config::LayoutWindowConfig::default(),
        &crate::editor::layout::modes::edit::windows::blueprint::transient::LayoutWindowTransient::default(),
        &crate::editor::layout::LayoutInteractionSnapshot::default(),
        false,
    );
    assert!(json.contains("\"kind\":\"image\"") && json.contains("data:image/png;base64,"), "{json}");
}
