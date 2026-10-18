//! 🗃️ Direct store emit and work: the direct store tool ids, the emit, the generation preview, the generation window transient and the direct store work. See [`super`].

use super::*;

//#region 🧵️DirectStoreLaneRoutes
pub const FLOW_DIRECT_STORE_TOOL_IDS: &[&str] = &[
    "removeWidget",
    "deleteSelection",
    "disconnect",
    "patchFlowWidgets",
    "duplicateWidget",
    "focusSelection",
    "nodeGraphViewport",
    "setLodMode",
    "setProximityDistance",
    "setGridVisible",
    "setGridSnapEnabled",
    "setGridFactor",
    "setPreviewOff",
    "setCatalogueSections",
    "toggleExtension",
    "addGeneration",
    "removeGeneration",
    "selectGeneration",
    "renameGeneration",
    "updateGenerationValues",
];

pub(super) const FLOW_DIRECT_STORE_RAW_BYTES: usize = 16_384;

pub(super) fn flow_direct_store_emit(command: &FlowCommand, config: &FlowMainWindowConfig, view: &semio_framework_plugin::ViewModel) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let mut next = config.clone();
    match command {
        FlowCommand::NodeGraphViewport(payload) => {
            next.camera = CameraJson { x: payload.viewport.x, y: payload.viewport.y, zoom: payload.viewport.zoom };
        }
        FlowCommand::SetLodMode(payload) => {
            if payload.value == FLOW_LOD_MODE_AUTOMATIC || DagDrawLod::from_id(&payload.value).is_some() {
                next.lod_mode = payload.value.clone();
            }
        }
        FlowCommand::SetProximityDistance(payload) => next.proximity_distance = payload.value.max(0.0),
        FlowCommand::SetGridVisible(payload) => next.grid_visible = payload.pressed.unwrap_or(!config.grid_visible),
        FlowCommand::SetGridSnapEnabled(payload) => next.grid_snap_enabled = payload.pressed.unwrap_or(!config.grid_snap_enabled),
        FlowCommand::SetGridFactor(payload) => next.grid_factor = payload.value.clamp(0.5, 50.0),
        FlowCommand::SetCatalogueSections(payload) => next.catalogue_sections_json = payload.sections_json.clone(),
        FlowCommand::ToggleExtension(payload) => {
            if config.automation_enabled_json.len() > FLOW_STORE_MAX_TEXT_BYTES || payload.id.len() > FLOW_STORE_MAX_TEXT_BYTES {
                return Err(Fault::from("flow-retained-extension-capacity"));
            }
            let mut enabled = serde_json::from_str::<HashMap<String, bool>>(&config.automation_enabled_json).unwrap_or_default();
            if enabled.len() >= FLOW_STORE_MAX_SCENE_ITEMS && !enabled.contains_key(&payload.id) {
                return Err(Fault::from("flow-retained-extension-item-capacity"));
            }
            enabled.insert(payload.id.clone(), payload.enabled);
            next.automation_enabled_json = serde_json::to_string(&enabled).map_err(|_| Fault::from("flow-retained-extension-encode"))?;
        }
        _ => return Err(Fault::from("flow-retained-direct-route-mismatch")),
    }
    Ok(Emit { window_config_mutations: main::config::addressed(view, &config, next)?, ..Default::default() })
}

pub(super) fn duplicate_widget_id(source: &str, suffix: u64) -> String {
    if suffix == 1 { format!("{source}-copy") } else { format!("{source}-copy-{suffix}") }
}

pub(super) fn duplicate_edge_id(source: &str, target: &str) -> String {
    format!("{source}-to-{target}")
}

fn evaluate_generation_preview(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, values: &crate::playbook::PlaybookValues) -> String {
    let live = snapshot.to_host_snapshot();
    let snapshot_json = semio_framework_pack_json::to_json_string(&live);
    let values: semio_framework_pack_json::Object = values.iter().map(|(key, value)| (key.clone(), semio_framework_pack_json::from_dsl_value(value))).collect();
    let patched = flow::forms_bridge::apply_generation_values_to_host_snapshot(&snapshot_json, &values);
    let patched_fixture = match FlowHost::parse_host_snapshot_json(&patched) {
        Ok(parsed) => {
            live.retire_cold();
            parsed
        }
        Err(_) => live,
    };
    let mut host = FlowHost::from_host_snapshot(patched_fixture).with_history_store_factory(::semio_framework_artifact_flow_flow::FLOW_HISTORY_STORE_FACTORY);
    seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    let preview = host.evaluate().unwrap_or_default();
    host.retire_cold();
    preview
}

pub(super) fn generation_window_transient(
    command: &FlowCommand,
    snapshot: &FlowSnapshot,
    config: &FlowMainWindowConfig,
    current: &main::transient::FlowWindowTransient,
    view: &semio_framework_plugin::ViewModel,
) -> Result<Option<semio_framework_plugin::WindowTransientMutation>, Fault> {
    let (action, args) = match command {
        FlowCommand::AddGeneration(_) => ("addGeneration", None),
        FlowCommand::RemoveGeneration(payload) => ("removeGeneration", Some(semio_framework_value::DslValue::object([("id".to_string(), semio_framework_value::DslValue::String(payload.id.clone()))]))),
        FlowCommand::SelectGeneration(payload) => ("selectGeneration", Some(semio_framework_value::DslValue::object([("id".to_string(), semio_framework_value::DslValue::String(payload.id.clone()))]))),
        FlowCommand::RenameGeneration(payload) => (
            "renameGeneration",
            Some(semio_framework_value::DslValue::object([("id".to_string(), semio_framework_value::DslValue::String(payload.id.clone())), ("name".to_string(), semio_framework_value::DslValue::String(payload.name.clone()))])),
        ),
        FlowCommand::UpdateGenerationValues(payload) => (
            "updateGenerationValues",
            Some(semio_framework_value::DslValue::object([
                ("generationId".to_string(), payload.generation_id.clone().map(semio_framework_value::DslValue::String).unwrap_or(semio_framework_value::DslValue::Null)),
                ("questionId".to_string(), semio_framework_value::DslValue::String(payload.question_id.clone())),
                ("value".to_string(), payload.value.clone()),
            ])),
        ),
        _ => return Ok(None),
    };
    let live = snapshot.to_host_snapshot();
    let spec = flow::forms_bridge::flow_host_snapshot_to_form_spec(&live, &PlaybookFlowFormProducer);
    live.retire_cold();
    let mut generation_owner = current.generation().map_err(|error| Fault::from(error.into_message()))?;
    let generation = generation_owner.as_mut();
    if !crate::playbook::handle_generation_action(action, args.as_ref(), generation, &spec, FLOW_PLAY_APP_ID) {
        return Ok(None);
    }
    if matches!(command, FlowCommand::AddGeneration(_) | FlowCommand::SelectGeneration(_) | FlowCommand::UpdateGenerationValues(_)) {
        match crate::playbook::selected_generation(&generation) {
            Some(active) => generation.preview_text = Some(evaluate_generation_preview(snapshot, config, &active.values)),
            None => generation.preview_text = None,
        }
    }
    let mut transient = current.clone();
    transient.generation_json = semio_framework_pack_json::to_json_string(generation);
    main::transient::addressed(view, transient).map(Some)
}

pub struct FlowDirectStoreWork {
    pub(super) tool_id: &'static str,
    pub(super) cursor: usize,
    pub(super) scan_cursor: usize,
    pub(super) replay_target: usize,
    pub(super) replay_scan_target: usize,
    pub preview_off: Option<Vec<String>>,
    pub(super) preview_next: Option<Vec<String>>,
    pub(super) preview_found: bool,
    pub(super) edge_ids: Option<Vec<String>>,
    pub(super) node_ids: Option<Vec<String>>,
    pub(super) duplicate_phase: u8,
    pub(super) duplicate_source: Option<usize>,
    pub(super) duplicate_suffix: u64,
    pub(super) completed: bool,
    pub(super) closing: bool,
    pub(super) retirement: retained::Retirement,
}

impl FlowDirectStoreWork {
    pub fn new(tool_id: &'static str) -> Self {
        Self {
            tool_id,
            cursor: 0,
            scan_cursor: 0,
            replay_target: 0,
            replay_scan_target: 0,
            preview_off: None,
            preview_next: None,
            preview_found: false,
            edge_ids: None,
            node_ids: None,
            duplicate_phase: 0,
            duplicate_source: None,
            duplicate_suffix: 1,
            completed: false,
            closing: false,
            retirement: retained::Retirement::default(),
        }
    }

    pub(super) fn replaying(&self) -> bool {
        self.cursor < self.replay_target || (self.cursor == self.replay_target && self.scan_cursor <= self.replay_scan_target)
    }
}
//#endregion 🧵️DirectStoreLaneRoutes
