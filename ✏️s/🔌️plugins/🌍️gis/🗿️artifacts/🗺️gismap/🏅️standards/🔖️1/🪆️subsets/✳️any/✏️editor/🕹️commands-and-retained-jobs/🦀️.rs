//! 🕹️ Commands of the GIS 2D play app: the aggregated command enum, the app type and the retained tool job factories with their publication contracts. See [`super`].

use super::*;

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Gis2dPlayApp::Command` — the SOLE dispatch surface for gis2d's own behavior, covering every
    /// action `create_gis2d_app` declares. Row order is the binary variant ordinal: appending is safe,
    /// reordering is a wire-format break.
    pub enum Gis2dCommand for GisMapSnapshot, GisMapMutation, NoConfig, NoConfigMutation {
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "patchPositions" as "patch-positions" => patch_positions::PatchPositions,
        "patchRoutes" as "patch-routes" => patch_routes::PatchRoutes,
        "patchRoute" as "patch-route" => patch_route::PatchRoute,
        "toggleLayerVisibility" as "toggle-layer-visibility" => toggle_layer_visibility::ToggleLayerVisibility,
        "fitWorld" as "fit-world" => fit_world::FitWorld,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setRenderMode" as "render-mode" => set_render_mode::SetRenderMode,
        "setVectorStyle" as "vector-style" => set_vector_style::SetVectorStyle,
        "setLodMode" as "lod-mode" => set_lod_mode::SetLodMode,
        "focusFeature" as "focus-feature" => focus_feature::FocusFeature,
        "setLayerStrokeScale" as "layer-stroke-scale" => set_layer_stroke_scale::SetLayerStrokeScale,
        "openSource" as "open-source" => open_source::OpenSource,
        "proposeBoundsRegion" as "propose-bounds-region" => propose_bounds_region::ProposeBoundsRegion,
        "addFeature" as "add-feature" => add_feature::AddFeature,
        "moveFeature" as "move-feature" => move_feature::MoveFeature,
        "renameFeature" as "rename-feature" => rename_feature::RenameFeature,
        "deleteFeature" as "delete-feature" => delete_feature::DeleteFeature,
    }
}
//#endregion 🔖️Commands

//#region 🔖️Gis2dPlayApp
/// 🗺️ GIS 2D map play app. The document holds positions/routes/regions; everything else (camera,
/// render mode, style, LOD, layer visibility, and stroke weights) is [`MapWindowConfig`] — a
/// session-only but real, undoable config artifact. Layer AND feature selection/hover now live in
/// the framework-owned `"features"` interaction domain (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
#[derive(Default)]
pub struct Gis2dPlayApp;

//#region 🧵️RetainedCommands
pub(super) const GIS2D_RETAINED_TOOL_IDS: &[&str] =
    &["setActiveExample", "patchPositions", "patchRoutes", "patchRoute", "addFeature", "moveFeature", "renameFeature", "deleteFeature", "toggleLayerVisibility", "fitWorld", "setCamera", "setRenderMode", "setVectorStyle", "setLodMode", "focusFeature", "setLayerStrokeScale", "openSource", "proposeBoundsRegion"];

const GIS2D_RETAINED_PAYLOAD_SCHEMA: &str = "gis.map.tool-command.v1";

pub(super) const GIS2D_RETAINED_RAW_BYTES: usize = 8_192;

pub(super) const GIS2D_RETAINED_WORK_ITEMS: usize = 64;

pub(super) const GIS2D_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "patchPositions", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchRoutes", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchRoute", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addFeature", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "moveFeature", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "renameFeature", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "deleteFeature", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "toggleLayerVisibility", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "fitWorld", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setRenderMode", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setVectorStyle", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "focusFeature", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setLayerStrokeScale", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "openSource", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "proposeBoundsRegion", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];

fn gis2d_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GIS2D_RETAINED_RAW_BYTES, 64, GIS2D_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

pub(super) fn gis2d_map_window_mutation(command: &Gis2dCommand, document: &GisMapSnapshot, config: &MapWindowConfig) -> Option<MapWindowConfigMutation> {
    match command {
        Gis2dCommand::SetActiveExample(payload) if !payload.example_id.is_empty() => {
            let mut host = crate::editor::gis2d::maphost::map_host_from(&crate::standards::v1::subsets::any::io::text::snapshot::default_document(), config);
            host.fit_world_camera();
            Some(MapWindowConfigMutation::SetCamera(map_config_mutations::SetCamera { camera_json: host.camera_json() }))
        }
        Gis2dCommand::ToggleLayerVisibility(payload) => {
            let visible = !layer_visible(config, &payload.layer_id);
            Some(MapWindowConfigMutation::SetLayerVisibility(map_config_mutations::SetLayerVisibility { layer_id: payload.layer_id.clone(), visible: (!visible).then_some(false) }))
        }
        Gis2dCommand::FitWorld(_) => {
            let mut host = crate::editor::gis2d::maphost::map_host_from(document, config);
            host.fit_world_camera();
            Some(MapWindowConfigMutation::SetCamera(map_config_mutations::SetCamera { camera_json: host.camera_json() }))
        }
        Gis2dCommand::SetCamera(payload) => Some(MapWindowConfigMutation::SetCamera(map_config_mutations::SetCamera { camera_json: payload.camera_json.clone() })),
        Gis2dCommand::SetRenderMode(payload) => Some(MapWindowConfigMutation::SetRenderMode(map_config_mutations::SetRenderMode { value: payload.value.clone() })),
        Gis2dCommand::SetVectorStyle(payload) => Some(MapWindowConfigMutation::SetVectorStyle(map_config_mutations::SetVectorStyle { value: payload.value.clone() })),
        Gis2dCommand::SetLodMode(payload) => Some(MapWindowConfigMutation::SetLodMode(map_config_mutations::SetLodMode { value: payload.value.clone() })),
        Gis2dCommand::FocusFeature(payload) => {
            let mut host = crate::editor::gis2d::maphost::map_host_from(document, config);
            host.focus_feature(&payload.feature_kind, &payload.feature_id).then(|| MapWindowConfigMutation::SetCamera(map_config_mutations::SetCamera { camera_json: host.camera_json() }))
        }
        Gis2dCommand::SetLayerStrokeScale(payload) => {
            let value = semio_framework_surface::tiled_map::clamp_map_layer_weight(payload.value);
            Some(MapWindowConfigMutation::SetLayerStrokeScale(map_config_mutations::SetLayerStrokeScale { layer_id: payload.layer_id.clone(), value: (value != 1.0).then_some(value) }))
        }
        _ => None,
    }
}

pub(super) struct GisMapWindowWork {
    pub(super) tool_id: &'static str,
    completed: bool,
}

impl GisMapWindowWork {
    pub(super) fn new(tool_id: &'static str) -> Self { Self { tool_id, completed: false } }
}

impl ArtifactCommandWork<EditorApp<Gis2dPlayApp>> for GisMapWindowWork {
    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Gis2dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }
    fn tool_id(&self) -> &'static str { self.tool_id }

    fn extent(
        &self,
        command: &Gis2dCommand,
        _snapshot: &GisMapSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Gis2dPlayApp>>>,
    ) -> Option<usize> {
        (!self.completed && command.command_id() == self.tool_id).then_some(1)
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Gis2dPlayApp>>,
    _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Gis2dPlayApp>>, Fault> {
        if self.completed || input.command.command_id() != self.tool_id {
            return Err(Fault::from("gis-map-window-work-terminal"));
        }
        let window = input.context.and_then(|context| context.window_config.as_ref());
        let config = map_config::from_snapshot(window);
        let config_view = ConfigView { snapshot: input.config, window };
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let mut emit = input.command.dispatch(&doc, &config_view)?;
        if let Some(mutation) = gis2d_map_window_mutation(input.command, input.snapshot, &config) {
            let view = input.context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("gis-map-window-view-required"))?;
            emit.window_config_mutations.push(map_config::addressed(view, mutation)?);
        }
        self.completed = true;
        Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(emit))
    }
}

pub(super) struct Gis2dRetainedCommandJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl Gis2dRetainedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: GIS2D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Gis2dRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Gis2dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Gis2dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GIS2D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        gis2d_retained_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > GIS2D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("GIS map retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Gis2dRetainedCommandJobFactory {
    type Owner = EditorApp<Gis2dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GIS2D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GIS_MAP_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = GIS2D_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️Gis2dPlayApp
