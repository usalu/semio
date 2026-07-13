//#region 🔖LayoutPlayApp
#[derive(Default)]
struct LayoutPlayApp {
    runtime: LayoutPlayRuntime,
}

impl DocumentApp for LayoutPlayApp {
    type Projection = LayoutDocument;
    type Op = LayoutOp;

    fn app_id(&self) -> &str {
        LAYOUT_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        LAYOUT_FIXTURE_SCHEMA
    }

    fn initial_projection(&self) -> LayoutDocument {
        default_document()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, LayoutDocument>,
        view_state: &ViewState,
    ) -> ActionEmit<LayoutOp> {
        let document = doc.projection;
        match action {
            //#region 👁️View
            "setSelection" => {
                self.runtime.selected_ids = selection_ids(args);
                ActionEmit::default()
            }
            "setActivePage" => {
                if let Some(page_id) = args.and_then(|value| value.get("pageId")).and_then(|value| value.as_str()) {
                    self.runtime.active_page_id = page_id.into();
                }
                ActionEmit::default()
            }
            "setHover" => {
                self.runtime.hovered_id = args
                    .and_then(|value| value.get("id"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                ActionEmit::default()
            }
            "focusPreflightIssue" => {
                if let Some(issue) = args.and_then(|value| value.get("issue")) {
                    if let Some(object_id) = issue.get("objectId").and_then(|value| value.as_str()) {
                        self.runtime.selected_ids = vec![object_id.into()];
                    }
                    if let Some(page_id) = issue.get("pageId").and_then(|value| value.as_str()) {
                        self.runtime.active_page_id = page_id.into();
                    }
                }
                ActionEmit::default()
            }
            "engagementInput" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(Value::as_str) {
                    self.runtime.engagement_input = value.into();
                }
                ActionEmit::default()
            }
            "canvasPointerDown" => {
                let blueprint = surface_is_blueprint(args);
                let button = args.and_then(|value| value.get("button")).and_then(Value::as_i64).unwrap_or(0);
                if !blueprint || button != 0 {
                    return ActionEmit::default();
                }
                let extend = args.and_then(|value| value.get("extend")).and_then(Value::as_bool).unwrap_or(false);
                let hit = hit_test_at(document, &self.runtime, args, blueprint);
                self.runtime.selected_ids = match hit {
                    Some(id) if extend => {
                        let mut ids = self.runtime.selected_ids.clone();
                        if let Some(position) = ids.iter().position(|existing| *existing == id) {
                            ids.remove(position);
                        } else {
                            ids.push(id);
                        }
                        ids
                    }
                    Some(id) => vec![id],
                    None => Vec::new(),
                };
                ActionEmit::default()
            }
            "canvasPointerMove" => {
                let blueprint = surface_is_blueprint(args);
                if !blueprint {
                    return ActionEmit::default();
                }
                self.runtime.hovered_id = hit_test_at(document, &self.runtime, args, blueprint);
                ActionEmit::default()
            }
            "canvasPointerUp" => ActionEmit::default(),
            "canvasDragOver" => {
                let blueprint = surface_is_blueprint(args);
                if !blueprint {
                    return ActionEmit::default();
                }
                let kind = args
                    .and_then(|value| value.get("types"))
                    .and_then(|value| value.as_array())
                    .and_then(|types| {
                        types
                            .iter()
                            .find_map(|entry| entry.as_str().and_then(|type_value| type_value.strip_prefix(LAYOUT_CATALOGUE_KIND_MIME_PREFIX)).map(str::to_string))
                    })
                    .unwrap_or_else(|| "unknown".into());
                let (sx, sy, width, height) = pointer_args(args);
                let (wx, wy) = screen_to_world_for_surface(document, blueprint, sx, sy, width, height);
                self.runtime.drop_preview = Some(LayoutDropPreviewState { kind, x: wx, y: wy });
                ActionEmit::default()
            }
            "canvasDragLeave" => {
                self.runtime.drop_preview = None;
                ActionEmit::default()
            }
            //#endregion 👁️View
            //#region 🔧Operations
            "addFrame" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("rect");
                let drop_x = args.and_then(|value| value.get("x")).and_then(Value::as_f64);
                let drop_y = args.and_then(|value| value.get("y")).and_then(Value::as_f64);
                let page_id = self.runtime.active_page_id.clone();
                let Some(page) = document.pages.iter().find(|page| page.id == page_id) else {
                    return ActionEmit::default();
                };
                let index = page.frames.len();
                let frame_id = format!("frame-{}", index + 1);
                let layer_id = page.layer_ids.first().cloned().unwrap_or_else(|| "layer-1".into());
                let frame = match kind {
                    "text" => Frame::Text {
                        id: frame_id.clone(),
                        layer_id: layer_id.clone(),
                        bounds: layout_rs::LayoutBounds { x: drop_x.unwrap_or(48.0), y: drop_y.unwrap_or(120.0), width: 200.0, height: 120.0, rotation: 0.0 },
                        locked: None,
                        visible: None,
                        story_id: document.stories.first().map(|story| story.id.clone()).unwrap_or_else(|| "story-1".into()),
                        thread_next: None,
                        columns: 1,
                        inset: layout_rs::LayoutRect { x: 4.0, y: 4.0, width: 192.0, height: 112.0 },
                        wrap_mode: "box".into(),
                    },
                    "image" => Frame::Image {
                        id: frame_id.clone(),
                        layer_id: layer_id.clone(),
                        bounds: layout_rs::LayoutBounds { x: drop_x.unwrap_or(48.0), y: drop_y.unwrap_or(280.0), width: 160.0, height: 120.0, rotation: 0.0 },
                        locked: None,
                        visible: None,
                        link_id: document.links.first().map(|link| link.id.clone()).unwrap_or_else(|| "link-missing".into()),
                    },
                    _ => Frame::Rect {
                        id: frame_id.clone(),
                        layer_id: layer_id.clone(),
                        bounds: layout_rs::LayoutBounds { x: drop_x.unwrap_or(48.0), y: drop_y.unwrap_or(48.0), width: 120.0, height: 64.0, rotation: 0.0 },
                        locked: None,
                        visible: None,
                        fill: Some([0.2, 0.24, 0.3, 1.0]),
                        stroke: None,
                    },
                };
                self.runtime.selected_ids = vec![frame_id];
                ActionEmit::ops(vec![LayoutOp::AddFrame { page_id, index, frame, layer_id: Some(layer_id) }])
            }
            "addPage" => {
                let template = document
                    .pages
                    .iter()
                    .find(|page| page.id == self.runtime.active_page_id)
                    .or_else(|| document.pages.first());
                let (width, height, spread_id, parent_page_id, margins, columns) = template
                    .map(|page| {
                        (
                            page.width,
                            page.height,
                            page.spread_id.clone(),
                            page.parent_page_id.clone(),
                            page.margins.clone(),
                            page.columns.clone(),
                        )
                    })
                    .unwrap_or((
                        595.0,
                        842.0,
                        "spread-1".into(),
                        None,
                        PageMargins { top: 48.0, right: 36.0, bottom: 48.0, left: 36.0 },
                        PageColumns { count: 1, gutter: 0.0 },
                    ));
                let page_id = format!("page-{}", document.pages.len() + 1);
                let layer_id = format!("layer-{page_id}");
                let page = Page {
                    id: page_id.clone(),
                    name: format!("Page {}", document.pages.len() + 1),
                    spread_id,
                    parent_page_id,
                    width,
                    height,
                    margins,
                    columns,
                    guides: Vec::new(),
                    layer_ids: vec![layer_id.clone()],
                    layers: vec![layout_rs::Layer { id: layer_id, name: "Content".into(), visible: true, locked: false, object_ids: Vec::new() }],
                    frames: Vec::new(),
                    overrides: Vec::new(),
                };
                self.runtime.active_page_id = page_id.clone();
                self.runtime.selected_ids = vec![page_id];
                ActionEmit::ops(vec![LayoutOp::Pages(CollectionOp::Add { index: document.pages.len(), item: page })])
            }
            "patchPage" => {
                let page_id = args
                    .and_then(|value| value.get("pageId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| self.runtime.active_page_id.clone());
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                match page_patch_for_field(field, &value) {
                    Some(patch) if document.pages.iter().any(|page| page.id == page_id) => {
                        ActionEmit::ops(vec![LayoutOp::Pages(CollectionOp::Patch { id: page_id, patch })])
                    }
                    _ => ActionEmit::default(),
                }
            }
            "patchFrame" => {
                let frame_id = args.and_then(|value| value.get("frameId")).and_then(|value| value.as_str()).unwrap_or("").to_string();
                let page_id = args
                    .and_then(|value| value.get("pageId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| self.runtime.active_page_id.clone());
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("").to_string();
                let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                if frame_id.is_empty() {
                    return ActionEmit::default();
                }
                let Some(page) = document.pages.iter().find(|page| page.id == page_id) else {
                    return ActionEmit::default();
                };
                let Some(frame) = page.frames.iter().find(|frame| frame.id() == frame_id) else {
                    return ActionEmit::default();
                };
                match field.as_str() {
                    "x" | "y" | "width" | "w" | "height" | "h" => match value.as_f64() {
                        Some(number) => ActionEmit::ops(vec![LayoutOp::PatchFrame {
                            page_id,
                            frame_id,
                            patch: frame_bounds_patch(&field, number),
                        }]),
                        None => ActionEmit::default(),
                    },
                    "fill" | "stroke" => {
                        let rgba = text_to_rgba(value.as_str().unwrap_or(""));
                        let patch = if field == "fill" {
                            FramePatch { fill: Some(rgba), ..Default::default() }
                        } else {
                            FramePatch { stroke: Some(rgba), ..Default::default() }
                        };
                        ActionEmit::ops(vec![LayoutOp::PatchFrame { page_id, frame_id, patch }])
                    }
                    "wrapMode" => match value.as_str() {
                        Some(mode) => ActionEmit::ops(vec![LayoutOp::PatchFrame {
                            page_id,
                            frame_id,
                            patch: FramePatch { wrap_mode: Some(mode.into()), ..Default::default() },
                        }]),
                        None => ActionEmit::default(),
                    },
                    "columns" => match value.as_f64() {
                        Some(count) => ActionEmit::ops(vec![LayoutOp::PatchFrame {
                            page_id,
                            frame_id,
                            patch: FramePatch { columns: Some(count.max(0.0) as u32), ..Default::default() },
                        }]),
                        None => ActionEmit::default(),
                    },
                    "storyContent" => {
                        let story_id = match frame {
                            Frame::Text { story_id, .. } => Some(story_id.clone()),
                            _ => None,
                        };
                        match (story_id, value.as_str()) {
                            (Some(story_id), Some(content)) if document.stories.iter().any(|story| story.id == story_id) => {
                                ActionEmit::ops(vec![LayoutOp::Stories(CollectionOp::Patch {
                                    id: story_id,
                                    patch: TextStoryPatch { content: Some(content.into()) },
                                })])
                            }
                            _ => ActionEmit::default(),
                        }
                    }
                    "linkPath" => {
                        let link_id = match frame {
                            Frame::Image { link_id, .. } => Some(link_id.clone()),
                            _ => None,
                        };
                        match (link_id, value.as_str()) {
                            (Some(link_id), Some(path)) if document.links.iter().any(|link| link.id == link_id) => {
                                ActionEmit::ops(vec![LayoutOp::Links(CollectionOp::Patch {
                                    id: link_id,
                                    patch: ImageLinkPatch { path: Some(path.into()) },
                                })])
                            }
                            _ => ActionEmit::default(),
                        }
                    }
                    _ => ActionEmit::default(),
                }
            }
            "setCamera" => {
                let blueprint = surface_is_blueprint(args);
                if let Some(camera_value) = args.and_then(|value| value.get("camera")) {
                    let x = camera_value.get("x").and_then(Value::as_f64);
                    let y = camera_value.get("y").and_then(Value::as_f64);
                    let zoom = camera_value.get("zoom").and_then(Value::as_f64);
                    if let (Some(x), Some(y), Some(zoom)) = (x, y, zoom) {
                        return ActionEmit {
                            ops: vec![LayoutOp::SetCamera { blueprint, camera: LayoutCamera { x, y, zoom } }],
                            coalesce_key: Some(if blueprint { "camera-blueprint".into() } else { "camera-preview".into() }),
                            ..Default::default()
                        };
                    }
                }
                ActionEmit::default()
            }
            "canvasDrop" => {
                let blueprint = surface_is_blueprint(args);
                self.runtime.drop_preview = None;
                if !blueprint {
                    return ActionEmit::default();
                }
                let Some(payload) = args
                    .and_then(|value| value.get("dragData"))
                    .and_then(Value::as_str)
                    .and_then(|raw| serde_json::from_str::<Value>(raw).ok())
                else {
                    return ActionEmit::default();
                };
                let Some(kind) = payload.get("kind").and_then(Value::as_str).map(str::to_string) else {
                    return ActionEmit::default();
                };
                let (sx, sy, width, height) = pointer_args(args);
                let (wx, wy) = screen_to_world_for_surface(document, blueprint, sx, sy, width, height);
                if kind == "page" {
                    self.handle_action("addPage", None, doc, view_state)
                } else {
                    self.handle_action("addFrame", Some(&json!({ "kind": kind, "x": wx, "y": wy })), doc, view_state)
                }
            }
            //#endregion 🔧Operations
            //#region 🐚Shell
            "exportPng" => {
                let page_id = args
                    .and_then(|value| value.get("pageId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or(self.runtime.active_page_id.as_str())
                    .to_string();
                match export_document_png_cpu(document, &page_id) {
                    Ok(bytes) => ActionEmit::effect(HostEffect::DownloadMediaExport {
                        filename: format!("{page_id}.png"),
                        mime_type: "image/png".into(),
                        data: base64::engine::general_purpose::STANDARD.encode(bytes),
                        encoding: Some("base64".into()),
                    }),
                    Err(_) => ActionEmit::default(),
                }
            }
            "exportSvg" => {
                let page_id = args
                    .and_then(|value| value.get("pageId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or(self.runtime.active_page_id.as_str())
                    .to_string();
                match export_document_svg(document, &page_id) {
                    Ok(svg) => ActionEmit::effect(HostEffect::DownloadMediaExport {
                        filename: format!("{page_id}.svg"),
                        mime_type: "image/svg+xml".into(),
                        data: svg,
                        encoding: None,
                    }),
                    Err(_) => ActionEmit::default(),
                }
            }
            "exportPdf" => {
                let page_id = args
                    .and_then(|value| value.get("pageId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or(self.runtime.active_page_id.as_str())
                    .to_string();
                match export_document_pdf(document, &page_id) {
                    Ok(bytes) => ActionEmit::effect(HostEffect::DownloadMediaExport {
                        filename: format!("{page_id}.pdf"),
                        mime_type: "application/pdf".into(),
                        data: base64::engine::general_purpose::STANDARD.encode(bytes),
                        encoding: Some("base64".into()),
                    }),
                    Err(_) => ActionEmit::default(),
                }
            }
            "exportPackage" => {
                let preflight_json = serde_json::to_string(&run_layout_preflight(document)).unwrap_or_else(|_| "[]".into());
                let doc_json = serde_json::to_string(document).unwrap_or_default();
                match export_package_zip(&doc_json, &preflight_json) {
                    Ok(bytes) => ActionEmit::effect(HostEffect::DownloadMediaExport {
                        filename: format!("{}.layout-package.zip", document.name),
                        mime_type: "application/zip".into(),
                        data: base64::engine::general_purpose::STANDARD.encode(bytes),
                        encoding: Some("base64".into()),
                    }),
                    Err(_) => ActionEmit::default(),
                }
            }
            "engagementSubmit" => {
                let typed = args.and_then(|value| value.get("value")).and_then(Value::as_str).map(str::trim).unwrap_or_default();
                let export = if engagement_token_matches(typed, "export png") || engagement_token_matches(typed, "png") {
                    Some("exportPng")
                } else if engagement_token_matches(typed, "export svg") || engagement_token_matches(typed, "svg") {
                    Some("exportSvg")
                } else if engagement_token_matches(typed, "export pdf") || engagement_token_matches(typed, "pdf") {
                    Some("exportPdf")
                } else if engagement_token_matches(typed, "export package") || engagement_token_matches(typed, "package") {
                    Some("exportPackage")
                } else {
                    None
                };
                match export {
                    Some(export) => self.handle_action(export, None, doc, view_state),
                    None => ActionEmit::default(),
                }
            }
            //#endregion 🐚Shell
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, LayoutDocument>, view_state: &ViewState) -> UiNode {
        let document = doc.projection;
        let labels = layout_labels(view_state);
        match body_key {
            LAYOUT_PLAY_BODY_BLUEPRINT => render_blueprint(document, &self.runtime),
            LAYOUT_PLAY_BODY_PREVIEW => render_preview(document, &self.runtime),
            LAYOUT_PLAY_BODY_DOCUMENT => build_document_tree(document, &self.runtime, labels),
            LAYOUT_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            LAYOUT_PLAY_BODY_INSPECTION => build_inspector_tree(document, &self.runtime, labels),
            LAYOUT_PLAY_BODY_PREFLIGHT => build_preflight_tree(document, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn tools(&self, _doc: &DocumentView<'_, LayoutDocument>, view_state: &ViewState) -> Vec<ToolNode> {
        layout_toolbar_tools(layout_labels(view_state))
    }

    fn window_engagements(&self, _doc: &DocumentView<'_, LayoutDocument>, _view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        HashMap::from([
            (LAYOUT_PLAY_WINDOW_BLUEPRINT.to_string(), layout_window_engagement(&self.runtime, "blueprint")),
            (LAYOUT_PLAY_WINDOW_PREVIEW.to_string(), layout_window_engagement(&self.runtime, "preview")),
        ])
    }

    fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
        let labels = layout_labels(view_state);
        semio_framework_plugin::AppLabelsOverlay {
            app_label: None,
            window_kind_labels: std::collections::HashMap::from([
                (LAYOUT_PLAY_WINDOW_BLUEPRINT.to_string(), labels.window_blueprint.to_string()),
                (LAYOUT_PLAY_WINDOW_PREVIEW.to_string(), labels.window_preview.to_string()),
            ]),
            panel_tab_labels: std::collections::HashMap::new(),
            mode_labels: std::collections::HashMap::new(),
        }
    }
}
//#endregion 🔖LayoutPlayApp
