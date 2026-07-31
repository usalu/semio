//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};

    fn meta(actor: &str) -> ActionMeta {
        ActionMeta { actor: actor.into(), instance_id: 1 }
    }

    fn new_app() -> VcsDocumentApp<LayoutPlayApp> {
        VcsDocumentApp::new(LayoutPlayApp::default())
    }

    fn render_json(app: &mut VcsDocumentApp<LayoutPlayApp>, body: &str) -> String {
        let node = app.render(body, None, &ViewState::default()).expect("render");
        serde_json::to_string(&node).unwrap()
    }

    fn render_json_locale(app: &mut VcsDocumentApp<LayoutPlayApp>, body: &str, locale: &str) -> String {
        let view_state = ViewState { locale: Some(locale.into()), ..ViewState::default() };
        let node = app.render(body, None, &view_state).expect("render");
        serde_json::to_string(&node).unwrap()
    }

    fn scene_layers_json(node: &UiNode) -> String {
        let value: Value = serde_json::to_value(node).unwrap();
        value["canvas2d"]["layersJson"].as_str().expect("layersJson string").to_string()
    }

    fn test_screen_point(camera_x: f64, camera_y: f64, zoom: f64, width: f64, height: f64, world_x: f64, world_y: f64) -> (f64, f64) {
        let camera = layout_rs::cavas::camera::Camera { x: camera_x, y: camera_y, zoom };
        let viewport = layout_rs::cavas::camera::Viewport { width: width as u32, height: height as u32, dpr: 1.0 };
        let screen = layout_rs::cavas::camera::world_to_screen(&camera, &viewport, layout_rs::cavas::Point::new(world_x, world_y));
        (screen.x, screen.y)
    }

    #[test]
    fn renders_blueprint_canvas_scene() {
        let mut app = new_app();
        assert!(render_json(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT).contains("canvas-2d"));
    }

    #[test]
    fn renders_preview_canvas_scene() {
        let mut app = new_app();
        assert!(render_json(&mut app, LAYOUT_PLAY_BODY_PREVIEW).contains("canvas-2d"));
    }

    #[test]
    fn document_lists_sample_pages() {
        let mut app = new_app();
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("layout-document.page.page-1"));
        assert!(json.contains("Page 1"));
    }

    #[test]
    fn catalogue_lists_frame_kinds() {
        let mut app = new_app();
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_CATALOGUE);
        assert!(json.contains("layout-catalogue.rect"));
        assert!(json.contains("Text Frame"));
    }

    #[test]
    fn layout_labels_resolve_native_english_by_default() {
        let mut app = new_app();
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("\"Frames\""));
        assert!(json.contains("\"Layers\""));
        let catalogue = render_json(&mut app, LAYOUT_PLAY_BODY_CATALOGUE);
        assert!(catalogue.contains("Rectangle"));
        assert!(!json.contains("Rahmen"));
    }

    #[test]
    fn layout_labels_translate_document_tree_in_german() {
        let mut app = new_app();
        let json = render_json_locale(&mut app, LAYOUT_PLAY_BODY_DOCUMENT, "de");
        assert!(json.contains("\"Rahmen\""));
        assert!(json.contains("\"Ebenen\""));
        let catalogue = render_json_locale(&mut app, LAYOUT_PLAY_BODY_CATALOGUE, "de");
        assert!(catalogue.contains("Rechteck"));
        assert!(!json.contains("\"Frames\""));
    }

    #[test]
    fn preflight_finds_missing_asset() {
        let issues = run_layout_preflight(&default_document());
        assert!(issues.iter().any(|issue| issue.code == "asset.missing"));
        let mut app = new_app();
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_PREFLIGHT);
        assert!(json.contains("asset.missing") || json.contains("Linked asset missing"));
    }

    #[test]
    fn set_selection_reflects_in_inspector() {
        let mut app = new_app();
        app.handle_action("setSelection", Some(&json!({ "ids": ["frame-text-1"] })), &ViewState::default(), &meta("local")).expect("select");
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_INSPECTION);
        assert!(json.contains("frame-text-1"));
    }

    #[test]
    fn sample_fixture_parses() {
        let doc = parse_layout_document(LAYOUT_SAMPLE_JSON).expect("sample fixture");
        assert_eq!(doc.schema, LAYOUT_FIXTURE_SCHEMA);
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn add_frame_action_appends_rect() {
        let mut app = new_app();
        let before = app.projection().expect("projection").pages[0].frames.len();
        let result = app.handle_action("addFrame", Some(&json!({ "kind": "rect" })), &ViewState::default(), &meta("local")).expect("add");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").pages[0].frames.len(), before + 1);
    }

    #[test]
    fn undo_redo_round_trips_add_frame() {
        let mut app = new_app();
        let before = app.projection().expect("projection").pages[0].frames.len();
        app.handle_action("addFrame", Some(&json!({ "kind": "rect" })), &ViewState::default(), &meta("local")).expect("add");
        assert_eq!(app.projection().expect("projection").pages[0].frames.len(), before + 1);
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").pages[0].frames.len(), before);
        app.handle_action("redo", None, &ViewState::default(), &meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").pages[0].frames.len(), before + 1);
    }

    #[test]
    fn patch_page_supports_margins_and_columns() {
        let mut app = new_app();
        for (field, value) in [
            ("marginTop", 60.0),
            ("marginRight", 40.0),
            ("marginBottom", 60.0),
            ("marginLeft", 40.0),
            ("columnsGutter", 18.0),
        ] {
            let result = app
                .handle_action("patchPage", Some(&json!({ "pageId": "page-1", "field": field, "value": value })), &ViewState::default(), &meta("local"))
                .expect("patch");
            assert_eq!(result.operations.len(), 1, "field {field} should apply");
        }
        app.handle_action("patchPage", Some(&json!({ "pageId": "page-1", "field": "columnsCount", "value": 3 })), &ViewState::default(), &meta("local")).expect("cols");
        let page = app.projection().expect("projection").pages.into_iter().find(|page| page.id == "page-1").unwrap();
        assert_eq!(page.columns.count, 3);
    }

    #[test]
    fn patch_frame_supports_rect_fill_and_stroke() {
        let mut app = new_app();
        let before = app.projection().expect("projection").pages[0].frames.len();
        app.handle_action("addFrame", Some(&json!({ "kind": "rect" })), &ViewState::default(), &meta("local")).expect("add");
        let frame_id = format!("frame-{}", before + 1);
        let result = app
            .handle_action(
                "patchFrame",
                Some(&json!({ "frameId": frame_id, "pageId": "page-1", "field": "fill", "value": "0.5, 0.4, 0.3, 1" })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("patch");
        assert_eq!(result.operations.len(), 1);
        let doc = app.projection().expect("projection");
        let frame = doc.pages[0].frames.iter().find(|frame| frame.id() == frame_id).unwrap();
        let Frame::Rect { fill, .. } = frame else { panic!("expected rect frame") };
        assert_eq!(fill.unwrap(), [0.5, 0.4, 0.3, 1.0]);
    }

    #[test]
    fn patch_frame_supports_text_story_content_and_wrap_mode() {
        let mut app = new_app();
        app.handle_action(
            "patchFrame",
            Some(&json!({ "frameId": "frame-text-1", "pageId": "page-1", "field": "storyContent", "value": "Edited story body." })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("story");
        let story = app.projection().expect("projection").stories.into_iter().find(|story| story.id == "story-1").unwrap();
        assert_eq!(story.content, "Edited story body.");

        app.handle_action(
            "patchFrame",
            Some(&json!({ "frameId": "frame-text-1", "pageId": "page-1", "field": "wrapMode", "value": "contour" })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("wrap");
        let doc = app.projection().expect("projection");
        let frame = doc.pages[0].frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap();
        let Frame::Text { wrap_mode, .. } = frame else { panic!("expected text frame") };
        assert_eq!(wrap_mode, "contour");
    }

    #[test]
    fn patch_frame_supports_image_link_path() {
        let mut app = new_app();
        app.handle_action(
            "patchFrame",
            Some(&json!({ "frameId": "frame-image-1", "pageId": "page-1", "field": "linkPath", "value": "assets/updated.png" })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("link");
        let link = app.projection().expect("projection").links.into_iter().find(|link| link.id == "link-missing").unwrap();
        assert_eq!(link.path, "assets/updated.png");
    }

    #[test]
    fn export_actions_wire_to_real_layout_rs_exporters() {
        let mut app = new_app();
        for (action, mime_type) in [
            ("exportPng", "image/png"),
            ("exportSvg", "image/svg+xml"),
            ("exportPdf", "application/pdf"),
            ("exportPackage", "application/zip"),
        ] {
            let result = app.handle_action(action, Some(&json!({ "pageId": "page-1" })), &ViewState::default(), &meta("local")).expect("export");
            match result.requested_effects.first() {
                Some(HostEffect::DownloadMediaExport { mime_type: mime, data, .. }) => {
                    assert_eq!(mime, mime_type, "{action}");
                    assert!(!data.is_empty(), "{action} data");
                }
                other => panic!("{action} expected DownloadMediaExport, got {other:?}"),
            }
        }
    }

    #[test]
    fn blueprint_scene_has_page_background_and_guides() {
        let mut app = new_app();
        let node = app.render(LAYOUT_PLAY_BODY_BLUEPRINT, None, &ViewState::default()).expect("render");
        let layers_json = scene_layers_json(&node);
        assert!(layers_json.contains("layout.page-bg"));
        assert!(layers_json.contains("0.97"));
        assert!(layers_json.contains("layout.guide.margin"));
        assert!(layers_json.contains("layout.guide.column"));
        assert!(layers_json.contains("\"segments\""));
        assert!(layers_json.contains("\"fill\":{\"color\""));
        assert!(!layers_json.contains("\"linkId\""));
    }

    #[test]
    fn preview_scene_has_white_background_and_no_guides() {
        let mut app = new_app();
        let node = app.render(LAYOUT_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("render");
        let layers_json = scene_layers_json(&node);
        assert!(layers_json.contains("layout.page-bg"));
        assert!(!layers_json.contains("layout.guide."));
    }

    #[test]
    fn inherited_frame_gets_dashed_stroke_in_blueprint() {
        let mut app = new_app();
        let node = app.render(LAYOUT_PLAY_BODY_BLUEPRINT, None, &ViewState::default()).expect("render");
        let layers_json = scene_layers_json(&node);
        assert!(layers_json.contains("\"dash\":[4.0,3.0]"));
    }

    #[test]
    fn selected_and_hovered_frames_get_chrome_strokes() {
        let mut app = new_app();
        app.handle_action("setSelection", Some(&json!({ "ids": ["frame-text-1"] })), &ViewState::default(), &meta("local")).expect("select");
        assert!(render_json(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT).contains("2.5"));

        app.handle_action("setHover", Some(&json!({ "id": "frame-image-1" })), &ViewState::default(), &meta("local")).expect("hover");
        assert!(render_json(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT).contains("1.75"));
    }

    #[test]
    fn set_camera_updates_surface_camera() {
        let mut app = new_app();
        let result = app
            .handle_action(
                "setCamera",
                Some(&json!({ "surfaceId": LAYOUT_PLAY_SURFACE_BLUEPRINT, "camera": { "x": 10.0, "y": 20.0, "zoom": 1.5 } })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("camera");
        assert_eq!(result.operations.len(), 1);
        let doc = app.projection().expect("projection");
        assert_eq!(doc.camera.x, 10.0);
        assert_eq!(doc.camera.y, 20.0);
        assert_eq!(doc.camera.zoom, 1.5);
        assert_eq!(doc.preview_camera.x, 0.0);
    }

    #[test]
    fn pointer_down_selects_frame_via_hit_test() {
        let mut app = new_app();
        let (sx, sy) = test_screen_point(0.0, 0.0, 0.5, 800.0, 600.0, 136.0, 435.0);
        app.handle_action(
            "canvasPointerDown",
            Some(&json!({ "surfaceId": LAYOUT_PLAY_SURFACE_BLUEPRINT, "x": sx, "y": sy, "width": 800.0, "height": 600.0, "button": 0 })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("pointer");
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("layout-document.frame.frame-image-1"));
    }

    #[test]
    fn pointer_move_updates_hover_highlight() {
        let mut app = new_app();
        let (sx, sy) = test_screen_point(0.0, 0.0, 0.5, 800.0, 600.0, 156.0, 220.0);
        let args = json!({ "surfaceId": LAYOUT_PLAY_SURFACE_BLUEPRINT, "x": sx, "y": sy, "width": 800.0, "height": 600.0 });
        let result = app.handle_action("canvasPointerMove", Some(&args), &ViewState::default(), &meta("local")).expect("move");
        assert!(result.operations.is_empty(), "hover is a view action, not an operation");
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("layout-document.frame.frame-text-1"));
    }

    #[test]
    fn canvas_drop_adds_frame_at_world_coords() {
        let mut app = new_app();
        let (sx, sy) = test_screen_point(0.0, 0.0, 0.5, 800.0, 600.0, 100.0, 200.0);
        let drag_data = json!({ "kind": "rect" }).to_string();
        let result = app
            .handle_action(
                "canvasDrop",
                Some(&json!({ "surfaceId": LAYOUT_PLAY_SURFACE_BLUEPRINT, "x": sx, "y": sy, "width": 800.0, "height": 600.0, "dragData": drag_data })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("drop");
        assert_eq!(result.operations.len(), 1);
        let doc = app.projection().expect("projection");
        let frame = doc.pages[0].frames.last().unwrap();
        let bounds = frame.bounds();
        assert!((bounds.x - 100.0).abs() < 0.01);
        assert!((bounds.y - 200.0).abs() < 0.01);
    }

    #[test]
    fn canvas_drop_page_kind_adds_page() {
        let mut app = new_app();
        let before = app.projection().expect("projection").pages.len();
        let drag_data = json!({ "kind": "page" }).to_string();
        let result = app
            .handle_action(
                "canvasDrop",
                Some(&json!({ "surfaceId": LAYOUT_PLAY_SURFACE_BLUEPRINT, "x": 0.0, "y": 0.0, "width": 800.0, "height": 600.0, "dragData": drag_data })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("drop");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").pages.len(), before + 1);
    }

    #[test]
    fn drag_over_emits_ghost_and_leave_clears() {
        let mut app = new_app();
        app.handle_action(
            "canvasDragOver",
            Some(&json!({
                "surfaceId": LAYOUT_PLAY_SURFACE_BLUEPRINT,
                "x": 400.0, "y": 300.0, "width": 800.0, "height": 600.0,
                "types": [format!("{LAYOUT_CATALOGUE_KIND_MIME_PREFIX}rect")],
            })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("over");
        assert!(render_json(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT).contains("layout.drop-preview"));

        app.handle_action("canvasDragLeave", Some(&json!({ "surfaceId": LAYOUT_PLAY_SURFACE_BLUEPRINT })), &ViewState::default(), &meta("local")).expect("leave");
        assert!(!render_json(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT).contains("layout.drop-preview"));
    }

    #[test]
    fn catalogue_items_are_draggable() {
        let mut app = new_app();
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_CATALOGUE);
        assert!(json.contains(LAYOUT_CATALOGUE_DRAG_MIME));
        assert!(json.contains("\"draggable\":true"));
        assert!(json.contains("layout-catalogue.page"));
    }

    #[test]
    fn document_tree_has_nine_sections() {
        let mut app = new_app();
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        for section_id in [
            "layout-document.document",
            "layout-document.spreads",
            "layout-document.pages",
            "layout-document.frames",
            "layout-document.parentPages",
            "layout-document.layers",
            "layout-document.stories",
            "layout-document.links",
            "layout-document.styles",
        ] {
            assert!(json.contains(section_id), "missing section {section_id}");
        }
    }

    #[test]
    fn preflight_reports_all_expected_issue_codes() {
        let json = r#"{
            "schema": "layout.fixture",
            "name": "Preflight Fixture",
            "camera": {"x":0,"y":0,"zoom":1},
            "previewCamera": {"x":0,"y":0,"zoom":1},
            "grid": {"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},
            "paragraphStyles": [{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],
            "characterStyles": [
                {"id":"character.small","fontFamily":"Layout Sans","fontSize":6},
                {"id":"character.exotic","fontFamily":"Comic Sans","fontSize":10}
            ],
            "stories": [
                {"id":"story-small","content":"Small caption text.","styleRuns":[{"start":0,"end":10,"paragraphStyleId":"paragraph.body","characterStyleId":"character.small"}]},
                {"id":"story-exotic","content":"Exotic font text.","styleRuns":[{"start":0,"end":10,"paragraphStyleId":"paragraph.body","characterStyleId":"character.exotic"}]},
                {"id":"story-overset","content":"placeholder","styleRuns":[]}
            ],
            "links": [
                {"id":"link-missing","path":"a.png","hash":"sha256:missing","width":100,"height":100,"dpi":300,"state":"missing"},
                {"id":"link-modified","path":"b.png","hash":"sha256:abc","width":100,"height":100,"dpi":300,"state":"modified"},
                {"id":"link-lowres","path":"c.png","hash":"sha256:def","width":100,"height":100,"dpi":72},
                {"id":"link-rgb","path":"d.png","hash":"sha256:ghi","width":100,"height":100,"dpi":300,"colorProfile":"RGB"}
            ],
            "parentPages": [],
            "spreads": [{"id":"spread-1","name":"Spread 1","pageIds":["page-1"]}],
            "pages": [{
                "id":"page-1","name":"Page 1","spreadId":"spread-1","width":200,"height":200,
                "margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},
                "guides":[], "layerIds":["layer-1"],
                "layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-oob","frame-missing","frame-modified","frame-lowres","frame-no-story","frame-small","frame-exotic","frame-overset"]}],
                "frames":[
                    {"id":"frame-oob","layerId":"layer-1","kind":"rect","bounds":{"x":150,"y":150,"w":100,"h":100,"rotation":0},"fill":[0,0,0,1]},
                    {"id":"frame-missing","layerId":"layer-1","kind":"image","bounds":{"x":0,"y":0,"w":20,"h":20,"rotation":0},"linkId":"link-missing"},
                    {"id":"frame-modified","layerId":"layer-1","kind":"image","bounds":{"x":20,"y":0,"w":20,"h":20,"rotation":0},"linkId":"link-modified"},
                    {"id":"frame-lowres","layerId":"layer-1","kind":"image","bounds":{"x":40,"y":0,"w":20,"h":20,"rotation":0},"linkId":"link-lowres"},
                    {"id":"frame-no-story","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":40,"w":50,"h":20,"rotation":0},"storyId":"story-absent","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"},
                    {"id":"frame-small","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":60,"w":50,"h":20,"rotation":0},"storyId":"story-small","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"},
                    {"id":"frame-exotic","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":80,"w":50,"h":20,"rotation":0},"storyId":"story-exotic","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"},
                    {"id":"frame-overset","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":100,"w":50,"h":20,"rotation":0},"storyId":"story-overset","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"}
                ],
                "overrides":[]
            }],
            "printTarget":"print"
        }"#;
        let mut doc = parse_layout_document(json).expect("preflight fixture");
        if let Some(story) = doc.stories.iter_mut().find(|story| story.id == "story-overset") {
            story.content = "a".repeat(450);
        }
        let issues = run_layout_preflight(&doc);
        let codes: Vec<&str> = issues.iter().map(|issue| issue.code.as_str()).collect();
        for expected in [
            "object.out_of_bounds",
            "asset.missing",
            "asset.modified",
            "asset.low_resolution",
            "image.empty_frame",
            "text.missing_story",
            "text.below_minimum_size",
            "font.missing",
            "text.overset",
            "asset.rgb_in_print",
        ] {
            assert!(codes.contains(&expected), "missing preflight code: {expected}");
        }
    }

    #[test]
    fn window_engagements_cover_both_windows() {
        let mut app = new_app();
        let engagements = app.window_engagements(&ViewState::default());
        let blueprint = engagements.get(LAYOUT_PLAY_WINDOW_BLUEPRINT).expect("blueprint engagement");
        let status = blueprint.status.as_ref().and_then(|rows| rows.first()).expect("status");
        assert!(status.text.contains("Page"));
        let input = blueprint.input.as_ref().expect("input");
        assert_eq!(input.placeholder.as_deref(), Some("undo, redo, export png"));
        assert!(engagements.contains_key(LAYOUT_PLAY_WINDOW_PREVIEW));
    }

    #[test]
    fn tools_expose_undo_redo_and_exports() {
        let mut app = new_app();
        let tools = app.tools(&ViewState::default());
        let json = serde_json::to_string(&tools).unwrap();
        for needle in [
            "layout-tools-undo",
            "layout-tools-redo",
            "layout-tools-export-png",
            "layout-tools-export-svg",
            "layout-tools-export-pdf",
            "layout-tools-export-package",
        ] {
            assert!(json.contains(needle), "missing tool {needle}");
        }
    }

    #[test]
    fn engagement_submit_triggers_export() {
        let mut app = new_app();
        let result = app.handle_action("engagementSubmit", Some(&json!({ "value": "export png" })), &ViewState::default(), &meta("local")).expect("submit");
        assert!(matches!(result.requested_effects.first(), Some(HostEffect::DownloadMediaExport { mime_type, .. }) if mime_type == "image/png"));
    }

    #[test]
    fn engagement_submit_triggers_export_from_normalized_shell_draft() {
        // The React shell PascalCases and strips separators from every draft before submitting it
        // (`normalizeEngagementActionText`), so "export png" arrives as "ExportPng".
        let mut app = new_app();
        let result = app.handle_action("engagementSubmit", Some(&json!({ "value": "ExportPng" })), &ViewState::default(), &meta("local")).expect("submit");
        assert!(matches!(result.requested_effects.first(), Some(HostEffect::DownloadMediaExport { mime_type, .. }) if mime_type == "image/png"));
    }

    #[test]
    fn dwg_import_frames_page_to_rectangular_polyline() {
        let mut drawing = semio_framework_os::DwgDrawing::default();
        drawing.entities.push(semio_framework_os::DwgEntity {
            layer: 0,
            color: semio_framework_os::DwgColor::ByLayer,
            geometry: semio_framework_os::DwgGeometry::LwPolyline {
                closed: true,
                elevation: 0.0,
                vertices: vec![[10.0, 20.0], [110.0, 20.0], [110.0, 70.0], [10.0, 70.0]],
                bulges: vec![0.0; 4],
            },
        });
        let value = layout_document_json_from_dwg(&drawing).expect("import dwg");
        let document: LayoutDocument = serde_json::from_value(value).expect("valid layout document");
        assert_eq!(document.pages.len(), 1);
        assert_eq!(document.pages[0].width, 100.0);
        assert_eq!(document.pages[0].height, 50.0);
    }

    #[test]
    fn dwg_import_without_rectangles_falls_back_to_extents() {
        let mut drawing = semio_framework_os::DwgDrawing::default();
        drawing.entities.push(semio_framework_os::DwgEntity {
            layer: 0,
            color: semio_framework_os::DwgColor::ByLayer,
            geometry: semio_framework_os::DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [200.0, 150.0, 0.0] },
        });
        drawing.extmin = [0.0, 0.0, 0.0];
        drawing.extmax = [200.0, 150.0, 0.0];
        let value = layout_document_json_from_dwg(&drawing).expect("import dwg");
        let document: LayoutDocument = serde_json::from_value(value).expect("valid layout document");
        assert_eq!(document.pages.len(), 1);
        assert_eq!(document.pages[0].width, 200.0);
        assert_eq!(document.pages[0].height, 150.0);
    }
}
//#endregion 🧪️Tests
