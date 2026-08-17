//#region 🔖️FlowPlayApp
#[derive(Default)]
struct FlowPlayApp {
    runtime: FlowPlayRuntime,
}

impl FlowPlayApp {
    /// 👁️ Parses the many selection-arg shapes (`ids`/`nodeIds` arrays or a single `nodeId`) into ids.
    fn parse_selection(args: Option<&Value>) -> Vec<String> {
        args.and_then(|value| value.get("ids").or_else(|| value.get("nodeIds")))
            .and_then(|value| {
                if value.is_array() {
                    serde_json::from_value(value.clone()).ok()
                } else {
                    value.as_str().map(|id| vec![id.to_string()])
                }
            })
            .or_else(|| {
                args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).map(|id| vec![id.to_string()])
            })
            .unwrap_or_default()
    }

    /// ✏️ Renames a widget id (rewiring synapses and layout) purely in the fixture; `None` if the target
    /// id is blank, unchanged, or already taken.
    fn renamed_fixture(fixture: &FlowFixture, old_id: &str, new_id: &str) -> Option<FlowFixture> {
        let trimmed = new_id.trim();
        if trimmed.is_empty() || trimmed == old_id || fixture.widgets.iter().any(|widget| widget_id(widget) == trimmed) {
            return None;
        }
        let mut next = fixture.clone();
        for widget in next.widgets.iter_mut() {
            if widget_id(widget) == old_id {
                match widget {
                    Widget::Neuron { id, .. }
                    | Widget::InputSlider { id, .. }
                    | Widget::InputStepper { id, .. }
                    | Widget::InputNote { id, .. }
                    | Widget::InputImage { id, .. }
                    | Widget::Variable { id, .. }
                    | Widget::OutputPreview { id, .. }
                    | Widget::OutputAction { id, .. }
                    | Widget::OutputExport { id, .. }
                    | Widget::Cluster { id, .. } => *id = trimmed.to_string(),
                }
            }
        }
        for synapse in next.synapses.iter_mut() {
            if synapse.from == old_id {
                synapse.from = trimmed.into();
            }
            if synapse.to == old_id {
                synapse.to = trimmed.into();
            }
        }
        if let Some(layout) = next.layout.remove(old_id) {
            next.layout.insert(trimmed.into(), layout);
        }
        Some(next)
    }

    /// ✏️ Patches slider values / note text on the selected widgets in the fixture, returning the clone.
    fn patched_widgets_fixture(fixture: &FlowFixture, widget_ids: &[String], field: &str, raw_value: Option<&Value>) -> FlowFixture {
        let mut next = fixture.clone();
        for widget in next.widgets.iter_mut() {
            if !widget_ids.iter().any(|id| id == widget_id(widget)) {
                continue;
            }
            match (field, widget) {
                ("value", Widget::InputSlider { value, .. }) => {
                    if let Some(v) = raw_value.and_then(|value| value.as_f64()) {
                        *value = v;
                    }
                }
                ("text", Widget::InputNote { text, .. }) => {
                    if let Some(v) = raw_value.and_then(|value| value.as_str()) {
                        *text = v.into();
                    }
                }
                _ => {}
            }
        }
        next
    }
}

impl DocumentApp for FlowPlayApp {
    type Projection = FlowFixture;
    type Op = FlowOp;

    fn app_id(&self) -> &str {
        FLOW_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        FLOW_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> FlowFixture {
        FlowFixture::default()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, FlowFixture>,
        _view_state: &ViewModel,
    ) -> ActionEmit<FlowOp> {
        let fixture = doc.projection;
        match action {
            // 👁️ View/config actions — mutate runtime, emit no ops (never pollute undo).
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                self.runtime.selected_node_ids = Self::parse_selection(args);
                ActionEmit::default()
            }
            "nodeGraphHover" => ActionEmit::default(),
            "graphPointerDown" => {
                self.runtime.selected_node_ids.clear();
                ActionEmit::default()
            }
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                    if let Ok(camera) = serde_json::from_str::<CameraJson>(viewport_json) {
                        self.runtime.camera = camera;
                    }
                }
                ActionEmit::default()
            }
            "evaluate" => {
                let mut host = host_from_fixture(fixture, &self.runtime);
                host.clear_computing_widget_ids();
                if let Ok(eval_json) = host.evaluate() {
                    host.apply_eval_outputs_json(&eval_json);
                    self.runtime.last_eval_json = eval_json;
                }
                ActionEmit::default()
            }
            "setLodMode" => {
                if let Some(mode) = args.and_then(|value| value.get("mode").or_else(|| value.get("value"))).and_then(|value| value.as_str()) {
                    if mode == FLOW_LOD_MODE_AUTOMATIC || DagDrawLod::from_id(mode).is_some() {
                        self.runtime.lod_mode = mode.into();
                    }
                }
                ActionEmit::default()
            }
            "setProximityDistance" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    self.runtime.proximity_distance = value.max(0.0);
                }
                ActionEmit::default()
            }
            "setCatalogueSections" => {
                if let Some(sections) = args.and_then(|value| value.get("sections")) {
                    self.runtime.catalogue_sections_json = sections.to_string();
                }
                ActionEmit::default()
            }
            "toggleExtension" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                let enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool());
                if let (Some(id), Some(enabled)) = (id, enabled) {
                    self.runtime.extension_enabled.insert(id.into(), enabled);
                }
                ActionEmit::default()
            }
            "addGeneration" | "removeGeneration" | "selectGeneration" | "renameGeneration" | "updateGenerationValues" => {
                let spec = flow_fixture_to_form_spec(fixture);
                let mut generation = self.runtime.generation.clone();
                if handle_generation_action(action, args, &mut generation, &spec, FLOW_PLAY_APP_ID) {
                    self.runtime.generation = generation;
                    if matches!(action, "addGeneration" | "selectGeneration" | "updateGenerationValues") {
                        refresh_generation_preview(fixture, &mut self.runtime);
                    }
                }
                ActionEmit::default()
            }
            // ✏️ Operation actions — run the stateful `FlowHost` mutation, diff into granular ops.
            "addWidget" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("inputSlider");
                let descriptor = match kind {
                    "neuron" => {
                        let neuron_kind = args.and_then(|value| value.get("neuronKind")).and_then(|value| value.as_str()).unwrap_or("math.add");
                        json!({ "kind": "neuron", "neuronKind": neuron_kind }).to_string()
                    }
                    other => json!({ "kind": other }).to_string(),
                };
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let mut new_id = None;
                let ops = host_ops(fixture, &self.runtime, |host| match host.add_widget(&descriptor, x, y) {
                    Ok(id) => {
                        new_id = Some(id);
                        true
                    }
                    Err(_) => false,
                });
                if let Some(id) = new_id {
                    self.runtime.selected_node_ids = vec![id];
                }
                ActionEmit::ops(ops)
            }
            "removeWidget" => {
                let widget_id = args
                    .and_then(|value| value.get("widgetId"))
                    .or_else(|| args.and_then(|value| value.get("id")))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                let Some(widget_id) = widget_id else {
                    return ActionEmit::default();
                };
                let ops = host_ops(fixture, &self.runtime, |host| host.remove_widget(&widget_id).is_ok());
                if !ops.is_empty() {
                    self.runtime.selected_node_ids.retain(|id| id != &widget_id);
                }
                ActionEmit::ops(ops)
            }
            "deleteSelection" => {
                let selected = self.runtime.selected_node_ids.clone();
                let ops = host_ops(fixture, &self.runtime, |host| {
                    sync_host_selection(host, &selected);
                    host.delete_selection().is_ok()
                });
                if !ops.is_empty() {
                    self.runtime.selected_node_ids.clear();
                }
                ActionEmit::ops(ops)
            }
            "disconnect" => {
                let synapse_id = args
                    .and_then(|value| value.get("synapseId"))
                    .or_else(|| args.and_then(|value| value.get("edgeId")))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                let Some(synapse_id) = synapse_id else {
                    return ActionEmit::default();
                };
                ActionEmit::ops(host_ops(fixture, &self.runtime, |host| host.disconnect(&synapse_id).is_ok()))
            }
            "connectMediaPorts" => {
                let from = args.and_then(|value| value.get("sourceNodeId")).and_then(|value| value.as_str()).map(str::to_string);
                let from_port = args.and_then(|value| value.get("sourcePortId")).and_then(|value| value.as_str()).map(str::to_string);
                let to = args.and_then(|value| value.get("targetNodeId")).and_then(|value| value.as_str()).map(str::to_string);
                let to_port = args.and_then(|value| value.get("targetPortId")).and_then(|value| value.as_str()).map(str::to_string);
                let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) else {
                    return ActionEmit::default();
                };
                ActionEmit::ops(host_ops(fixture, &self.runtime, |host| host.connect_ports(&from, &from_port, &to, &to_port).is_ok()))
            }
            "moveMediaNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).map(str::to_string);
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) else {
                    return ActionEmit::default();
                };
                let ops = host_ops(fixture, &self.runtime, |host| {
                    host.begin_change();
                    host.move_widget(&node_id, x, y).is_ok()
                });
                if ops.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit { ops, coalesce_key: Some(format!("move-{node_id}")), ..Default::default() }
            }
            "reorganize" => ActionEmit::ops(host_ops(fixture, &self.runtime, |host| host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok())),
            "patchFlowWidgets" => {
                let widget_ids: Vec<String> = args
                    .and_then(|value| value.get("widgetIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("").to_string();
                let raw_value = args.and_then(|value| value.get("value")).cloned();
                let next = Self::patched_widgets_fixture(fixture, &widget_ids, &field, raw_value.as_ref());
                let ops = flow_fixture_ops(fixture, &next);
                if ops.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit { ops, coalesce_key: Some(format!("patch-{field}-{}", widget_ids.join(","))), ..Default::default() }
            }
            "renameFlowWidget" => {
                let old_id = args.and_then(|value| value.get("oldId")).and_then(|value| value.as_str());
                let new_id = args.and_then(|value| value.get("value")).and_then(|value| value.as_str());
                let (Some(old_id), Some(new_id)) = (old_id, new_id) else {
                    return ActionEmit::default();
                };
                let Some(next) = Self::renamed_fixture(fixture, old_id, new_id) else {
                    return ActionEmit::default();
                };
                self.runtime.selected_node_ids = vec![new_id.trim().into()];
                ActionEmit::ops(flow_fixture_ops(fixture, &next))
            }
            "nodeGraphEdit" | "spotlightCommit" => {
                let raw_ops = args.and_then(|value| value.get("ops")).and_then(|value| value.as_array()).cloned().unwrap_or_default();
                let selected = self.runtime.selected_node_ids.clone();
                let mut clear_selection = false;
                let ops = host_ops(fixture, &self.runtime, |host| {
                    let mut changed = false;
                    for op in &raw_ops {
                        match op.get("op").and_then(|value| value.as_str()).unwrap_or("") {
                            "setFixture" => {
                                if let Some(fixture_json) = op.get("fixtureJson").and_then(|value| value.as_str()) {
                                    if let Ok(parsed) = serde_json::from_str::<FlowFixture>(fixture_json) {
                                        host.begin_change();
                                        host.set_fixture_preserving_history(parsed);
                                        changed = true;
                                    }
                                }
                            }
                            "deleteSelection" => {
                                sync_host_selection(host, &selected);
                                if host.delete_selection().is_ok() {
                                    clear_selection = true;
                                    changed = true;
                                }
                            }
                            "connect" => {
                                let from = op.get("sourceNodeId").and_then(|value| value.as_str());
                                let from_port = op.get("sourcePortId").and_then(|value| value.as_str());
                                let to = op.get("targetNodeId").and_then(|value| value.as_str());
                                let to_port = op.get("targetPortId").and_then(|value| value.as_str());
                                if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                                    if host.connect_ports(from, from_port, to, to_port).is_ok() {
                                        changed = true;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    changed
                });
                if clear_selection {
                    self.runtime.selected_node_ids.clear();
                }
                ActionEmit::ops(ops)
            }
            "runExtensionAction" => {
                let action_id = args.and_then(|value| value.get("actionId")).and_then(|value| value.as_str());
                let Some(action_id) = action_id else {
                    return ActionEmit::default();
                };
                let entry = FLOW_EXTENSIONS.iter().find(|(_, _, entry_action_id, ..)| *entry_action_id == action_id);
                let Some((id, _, _, _, effect)) = entry else {
                    return ActionEmit::default();
                };
                if !self.runtime.extension_enabled.get(*id).copied().unwrap_or(false) {
                    return ActionEmit::default();
                }
                match *effect {
                    "reorganize" => ActionEmit::ops(host_ops(fixture, &self.runtime, |host| host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok())),
                    "evaluate" => {
                        let mut host = host_from_fixture(fixture, &self.runtime);
                        if let Ok(eval_json) = host.evaluate() {
                            self.runtime.last_eval_json = eval_json;
                        }
                        ActionEmit::default()
                    }
                    _ => ActionEmit::default(),
                }
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, FlowFixture>, view_state: &ViewModel) -> UiNode {
        let fixture = doc.projection;
        let labels = flow_labels(view_state);
        match body_key {
            FLOW_PLAY_BODY_MAIN => render_main_graph(fixture, &self.runtime, labels),
            FLOW_PLAY_BODY_COMPILED => render_compiled_dag(fixture, &self.runtime),
            FLOW_PLAY_BODY_GENERATIONS => render_generate_generations(&self.runtime),
            FLOW_PLAY_BODY_GENERATE_FORM => render_generate_form(fixture, &self.runtime),
            FLOW_PLAY_BODY_GENERATE_PREVIEW => render_generate_preview(&self.runtime),
            FLOW_PLAY_BODY_DOCUMENT => build_document_tree(fixture, &self.runtime.selected_node_ids, labels),
            FLOW_PLAY_BODY_CATALOGUE => build_catalogue_tree(fixture, &self.runtime, labels),
            FLOW_PLAY_BODY_INSPECTOR => build_inspector_tree(fixture, &self.runtime.selected_node_ids, &self.runtime, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewModel) -> semio_framework_plugin::AppLabelsOverlay {
        let labels = flow_labels(view_state);
        semio_framework_plugin::AppLabelsOverlay {
            app_label: None,
            window_kind_labels: std::collections::HashMap::from([
                (FLOW_PLAY_WINDOW_MAIN.to_string(), labels.window_main.to_string()),
                (FLOW_PLAY_WINDOW_COMPILED.to_string(), labels.window_compiled.to_string()),
                (FLOW_PLAY_WINDOW_GENERATIONS.to_string(), labels.window_generations.to_string()),
                (FLOW_PLAY_WINDOW_GENERATE_FORM.to_string(), labels.window_generate_form.to_string()),
                (FLOW_PLAY_WINDOW_GENERATE_PREVIEW.to_string(), labels.window_generate_preview.to_string()),
            ]),
            panel_tab_labels: std::collections::HashMap::new(),
            mode_labels: std::collections::HashMap::new(),
        }
    }
}
//#endregion 🔖️FlowPlayApp
