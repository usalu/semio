//! 🧵️ Retained command factories of the layout play app: the retained tool ids and contract, the window work record, the retained command job factory with its owned tool job factory implementation and the retained and export proof catalogues. See [`super`].

use super::*;
#[path="🖼️page-drawing/🦀️.rs"]mod page_drawing;

//#region 🧵️RetainedCommands
/// 🧵️ Every shell-reachable verb is a bounded first-step tool (ticket 26/09/16/LAYOUT-PLUGIN-END-TO-END):
/// the framework refuses UI dispatch of any command not classified `Migrated`, so the six content verbs
/// that stayed `BatchOnlyPendingRewrite` (`addFrame`/`addPage`/`patchPage`/`patchFrame` and the pointer
/// down/move gestures) were dead in the running app. Exports keep their own resumable factory.
pub(super) const LAYOUT_RETAINED_TOOL_IDS: &[&str] = &[
    "setActivePage", "focusPreflightIssue", "engagementInput", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDragOver", "canvasDragLeave", "setCamera", "addFrame", "addPage", "patchPage", "patchFrame", "deleteSelection", "engagementSubmit", "canvasDrop", "translateSelection", "rotateSelection", "scaleSelection", "patchDocument",
];

const LAYOUT_RETAINED_PAYLOAD_SCHEMA: &str = "layout.layout.tool-command.v1";

pub(super) const LAYOUT_RETAINED_RAW_BYTES: usize = 8_192;

pub(super) const LAYOUT_RETAINED_WORK_ITEMS: usize = 1;

fn layout_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(LAYOUT_RETAINED_RAW_BYTES, 131_072, 1, 33_554_432, 2_000, 64, 1)
}

pub(super) struct LayoutWindowWork {
    pub(super) tool_id: &'static str,
    pub(super) completed: bool,
    page_drawing:page_drawing::PageDrawingWork,
}

impl LayoutWindowWork {
    pub(super) fn new(tool_id: &'static str) -> Self { Self { tool_id, completed: false,page_drawing:page_drawing::PageDrawingWork::new() } }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<LayoutPlayApp>> for LayoutWindowWork {
    fn receive_original_command(&mut self,command:&mut LayoutCommand,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{self.page_drawing.receive(command,cx)}
    fn begin_close(&mut self){self.page_drawing.begin_close()}
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{self.page_drawing.close_step(cx)}
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.page_drawing.close_demand(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.page_drawing.close_demand(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.page_drawing.close_demand(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.page_drawing.close_demand(0)?.depth)}
    fn terminal_is_empty(&self)->bool{self.page_drawing.owners.terminal_is_empty()}

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<LayoutPlayApp>>, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if self.page_drawing.active(){return self.page_drawing.demand(maximum_copy_bytes)}
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }
    fn tool_id(&self) -> &'static str { self.tool_id }

    fn extent(
        &self,
        command: &LayoutCommand,
        _snapshot: &LayoutSnapshot,
        _interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<LayoutPlayApp>>>,
    ) -> Option<usize> {
        let _ = context;
        (!self.completed && command.command_id() == self.tool_id).then_some(1)
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<LayoutPlayApp>>,
    cx: &mut StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<LayoutPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        if self.page_drawing.active(){
            if let Some(emit)=self.page_drawing.step(input,cx)?{self.completed=true;return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(emit))}
            return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Pending{stage:"layout.page.drawing.artboard",preview:b""});
        }
        let original: Result<_, Fault> = (|| {
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || input.command.command_id() != self.tool_id {
            return Err(Fault::from("layout-window-work-terminal"));
        }
        let window_config_snapshot = input.context.and_then(|context| context.window_config.as_ref());
        let window = input.context.and_then(|context| context.view_state.as_ref());
        let addressed_window = || window.ok_or_else(|| Fault::from("layout-window-view-required"));
        let mut config = blueprint::config::from_snapshot(window_config_snapshot);
        let mut transient = blueprint::transient::from_snapshot(input.context.and_then(|context| context.window_transient.as_ref()));
        let config_view = ConfigView { snapshot: input.config, window: window_config_snapshot };
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let selected = input.interaction.selection.get(LAYOUT_INTERACTION_ELEMENTS).map(|selection| selection.ids.as_slice()).unwrap_or(&[]);
        let gumball_verb = match input.command {
            LayoutCommand::TranslateSelection(payload) => Some(payload.verb()),
            LayoutCommand::RotateSelection(payload) => Some(payload.verb()),
            LayoutCommand::ScaleSelection(payload) => Some(payload.verb()),
            _ => None,
        };
        let mut transform_tool = None;
        let mut emit = match (input.command, gumball_verb) {
            (_, Some(verb)) => {
                let notice = window.map(|view| layout_labels(view).frames_locked.as_str());
                let dispatch = gumball::transform(verb, &doc, &config_view, selected, transient.transform_tool.as_deref(), notice);
                transform_tool = dispatch.tool;
                dispatch.emit
            }
            (LayoutCommand::DeleteSelection(_), None) => delete_selection::apply_frame_ids(&doc, selected)?,
            _ => input.command.dispatch(&doc, &config_view)?,
        };
        let mut window_config = None;
        let mut window_transient = None;
        match input.command {
            LayoutCommand::SetActivePage(payload) => {
                config.active_page_id = payload.page_id.clone();
                window_config = Some(blueprint::config::addressed(addressed_window()?, config)?);
            }
            LayoutCommand::FocusPreflightIssue(payload) => {
                if let Some(page_id) = &payload.page_id {
                    config.active_page_id = page_id.clone();
                    window_config = Some(blueprint::config::addressed(addressed_window()?, config)?);
                }
            }
            LayoutCommand::SetCamera(payload) => {
                config.camera = payload.camera.clone();
                window_config = Some(blueprint::config::addressed(addressed_window()?, config)?);
            }
            LayoutCommand::EngagementInput(payload) => {
                transient.engagement_input = payload.value.clone();
                window_transient = Some(blueprint::transient::addressed(addressed_window()?, transient)?);
            }
            LayoutCommand::AddPage(_) => {
                if let Some(view) = window {
                    config.active_page_id = format!("page-{}", input.snapshot.pages.len() + 1);
                    window_config = Some(blueprint::config::addressed(view, config)?);
                }
            }
            LayoutCommand::CanvasDragOver(payload) => {
                let view = addressed_window()?;
                if view.window_instances.iter().any(|window| view.window_id.as_deref() == Some(&window.id) && window.window_kind_id == LAYOUT_PLAY_WINDOW_BLUEPRINT) {
                    let camera = semio_framework_canvas::camera::Camera { x: config.camera.x, y: config.camera.y, zoom: config.camera.zoom.max(0.0001) };
                    let viewport = semio_framework_canvas::camera::Viewport { width: payload.width.max(1.0) as u32, height: payload.height.max(1.0) as u32, dpr: 1.0 };
                    let world = semio_framework_canvas::camera::screen_to_world(&camera, &viewport, semio_framework_canvas::Point::new(payload.x, payload.y));
                    transient.drop_preview = semio_s_artifact_layout_layout_model::LayoutDropPreviewState { kind: payload.kind.clone(), x: world.x, y: world.y };
                    window_transient = Some(blueprint::transient::addressed(view, transient)?);
                }
            }
            LayoutCommand::CanvasDragLeave(_) | LayoutCommand::CanvasDrop(_) => {
                let view = addressed_window()?;
                transient.drop_preview = semio_s_artifact_layout_layout_model::LayoutDropPreviewState::default();
                window_transient = Some(blueprint::transient::addressed(view, transient)?);
                if let LayoutCommand::CanvasDrop(payload) = input.command {
                    if payload.kind == "page" {
                        let page_id = format!("page-{}", input.snapshot.pages.len() + 1);
                        config.active_page_id = page_id;
                        emit.window_config_mutations.push(blueprint::config::addressed(view, config)?);
                    }
                }
            }
            LayoutCommand::TranslateSelection(_) | LayoutCommand::RotateSelection(_) | LayoutCommand::ScaleSelection(_) => {
                if let Some(tool) = transform_tool {
                    transient.transform_tool = tool;
                    window_transient = Some(blueprint::transient::addressed(addressed_window()?, transient)?);
                }
            }
            LayoutCommand::AddFrame(_) | LayoutCommand::PatchPage(_) | LayoutCommand::PatchFrame(_) | LayoutCommand::PatchDocument(_) | LayoutCommand::DeleteSelection(_) | LayoutCommand::CanvasPointerDown(_) | LayoutCommand::CanvasPointerMove(_) | LayoutCommand::CanvasPointerUp(_) | LayoutCommand::EngagementSubmit(_) => {}
            _ => return Err(Fault::from("layout-window-work-route-rejected")),
        }
        if let Some(mutation) = window_config { emit.window_config_mutations.push(mutation); }
        self.completed = true;
        match window_transient {
            Some(mutation) => Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit,
                ephemeral: semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: vec![mutation] },
            }),
            None => Ok(ArtifactCommandWorkStep::Complete(emit)),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

pub(super) struct LayoutRetainedCommandJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl LayoutRetainedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: LAYOUT_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for LayoutRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<LayoutPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<LayoutPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        LAYOUT_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        layout_retained_contract()
    }

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: LAYOUT_RETAINED_RAW_BYTES, checkpoint_resume: false);

}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for LayoutRetainedCommandJobFactory {
    type Owner = EditorApp<LayoutPlayApp>;
    const TOOL_IDS: &'static [&'static str] = LAYOUT_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_layout_layout_model::LAYOUT_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "setActivePage", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "focusPreflightIssue", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasDragOver", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "canvasDragLeave", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "addFrame", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addPage", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "patchPage", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchFrame", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasDrop", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "patchDocument", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ];
}

//#region 🧾️ProofCatalogs
pub(super) struct LayoutRetainedProofs;

impl LayoutRetainedProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<LayoutPlayApp>,
        owner_file: "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.layout.layout@1/*#editor",
        artifact_schema: "layout.layout",
        factory: "LayoutRetainedCommandJobFactory",
        factory_type: LayoutRetainedCommandJobFactory,
        contract: ToolExecutionContract::resumable(8_192, 131_072, 1, 33_554_432, 2_000, 64, 1),
        tools: ["setActivePage", "focusPreflightIssue", "engagementInput", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDragOver", "canvasDragLeave", "setCamera", "addFrame", "addPage", "patchPage", "patchFrame", "deleteSelection", "engagementSubmit", "canvasDrop", "translateSelection", "rotateSelection", "scaleSelection", "patchDocument"]
    }
}

pub(super) struct LayoutExportProofs;

impl LayoutExportProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<LayoutPlayApp>,
        owner_file: "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.layout.layout@1/*#editor",
        artifact_schema: "layout.layout",
        factory: "LayoutExportJobFactory",
        factory_type: LayoutExportJobFactory,
        tools: {
            "exportPng" => ToolExecutionContract::resumable(4_096, 131_072, 1, 33_554_432, 2_000, 64, 1),
            "exportSvg" => ToolExecutionContract::resumable(4_096, 131_072, 1, 33_554_432, 2_000, 64, 1),
            "exportPdf" => ToolExecutionContract::resumable(4_096, 131_072, 1, 33_554_432, 2_000, 64, 1),
            "exportPackage" => ToolExecutionContract::resumable(4_096, 131_072, 1, 33_554_432, 2_000, 64, 1),
            "exportStep" => ToolExecutionContract::resumable(4_096, 131_072, 1, 33_554_432, 2_000, 64, 1),
            "exportIdml" => ToolExecutionContract::resumable(4_096, 131_072, 1, 33_554_432, 2_000, 64, 1),
        }
    }
}
//#endregion 🧾️ProofCatalogs
//#endregion 🧵️RetainedCommands
