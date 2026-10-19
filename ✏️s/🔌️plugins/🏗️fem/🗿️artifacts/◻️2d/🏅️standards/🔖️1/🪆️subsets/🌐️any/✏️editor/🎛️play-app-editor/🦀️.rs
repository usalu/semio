//! 🎛️ Play app editor: the fem2d play app and its `ArtifactEditor` implementation. See [`super`].

use super::*;

//#region 🔖️Fem2dPlayApp
/// 🪟️ FEM application commands and rendering use the concrete Model and Results window owners.
#[derive(Default)]
pub struct Fem2dPlayApp;

impl ArtifactEditor for Fem2dPlayApp {
    /// 📢️ The localized notices of the gumball and canvas refusals (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        crate::editor::fem2d::commands::gumball::fem_fault_notices()
    }

    type Snapshot = Fem2dSnapshot;
    type Mutation = Fem2dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_s_artifact_fem_2d_editor_transient::editor::fem2d::transient::FemGumballTransient;
    type TransientMutation = semio_s_artifact_fem_2d_editor_transient::editor::fem2d::transient::FemGumballTransientMutation;

    type Command = Fem2dCommand;

    const DIALECT: Dialect = semio_s_artifact_fem_2d_io::standards::v1::subsets::any::io::FEM2D_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FEM_2D_SCHEMA;




    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::bounded_presence_store_disposer::<Self::Presence, Self::PresenceMutation>())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_presence_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_presence_root_retirement_factory::<Self::Presence>())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
    }

    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<model_window::config::Fem2dModelWindowConfigOwner>()?;
        registry.register::<results_window::config::Fem2dResultsWindowConfigOwner>()
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        registry.register::<results_window::transient::Fem2dResultsWindowTransientOwner>()
    }

    /// 🎯️ The playback chain publishes into ONE results window's transient: the tick names it off its PAYLOAD (the shell
    /// redispatches a self-armed `DispatchAction` under whichever pane is current), and a panel-tagged transport gesture
    /// names it the same way so a pause can rest where the running clock is. The runtime validates the id against the
    /// trusted roster and captures that exact window's mutation authority.
    fn retained_window_transient_target(command: &Fem2dCommand) -> Option<(&str, &'static str)> {
        match command {
            Fem2dCommand::ResultAnimationTick(payload) if !payload.window_id.is_empty() => Some((payload.window_id.as_str(), results_window::transient::WINDOW_KIND_ID)),
            Fem2dCommand::SetResultAnimation(payload) => payload.window_id.as_deref().filter(|id| !id.is_empty()).map(|id| (id, results_window::transient::WINDOW_KIND_ID)),
            _ => None,
        }
    }



    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Fem2dArtifactPreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Fem2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs",
        controller: "s.fem.fem2d@1/*#editor",
        artifact_schema: "fem.2d",
        factory: "Fem2dRetainedCommandJobFactory",
        factory_type: Fem2dRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "addNode",
            "addBar",
            "addBeam",
            "addMaterial",
            "addSection",
            "addSupport",
            "addNodalLoad",
            "addMemberUdl",
            "addAreaLoad",
            "addRegion",
            "addLoadCase",
            "addCombination",
            "setSelfWeight",
            "setAnalysisSettings",
            "removeSelection",
            "setActiveExample",
            "setCamera",
            "setResultDisplay",
            "canvasPointerDown",
            "canvasPointerMove",
            "canvasPointerUp",
            "patchNode",
            "patchElement",
            "patchMaterial",
            "patchSection",
            "patchSupport",
            "patchRegion",
            "patchLoad",
            "patchLoadCase",
            "patchCombination",
            "setResultAnimation",
            "resultAnimationTick",
            "focusEntity",
            "translateSelection",
            "rotateSelection",
            "scaleSelection",
            "setTransformGumballFlag",
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Fem2dRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original FEM request is absent"))?;
        if !FEM2D_RETAINED_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "fem2d retained command does not match its exact registered tool").into());
        }

        let tool_id = original.command.command_id();
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, Fem2dCommand::command_id, FEM2D_RETAINED_RAW_BYTES, FEM2D_RETAINED_WORK_ITEMS, false, |_original| Fem2dRetainedWork { tool_id, consumed: false })
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        None
    }

    /// 🌱️ Boots on the bundled `📚️examples/🎬️demo` document so the playground paints a real structure
    /// at first frame instead of an empty canvas — the same snapshot `Fem2dViewer` already booted on.
    fn initial_snapshot() -> Fem2dSnapshot {
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(fem2d_io())
    }

    fn mounted_job_maintenance_phase(instance_id: u32) -> Option<semio_framework_plugin::MountedOwnerPhaseV1> {
        (!semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::terminal_is_empty(instance_id)).then_some(semio_framework_plugin::MountedOwnerPhaseV1::Maintenance)
    }

    fn mounted_job_maintenance_demands(instance_id: u32, _body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::close_demands(instance_id)
    }

    fn mounted_job_close_demands(instance_id: u32, _body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::close_demands(instance_id)
    }

    fn mounted_job_maintenance_step(instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,grant: semio_framework_value::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep,semio_framework_plugin::PluginLifecycleFailure> {
        semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::maintenance_step(instance_id, cx, grant)
    }

    fn mounted_job_close_step(instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,grant: semio_framework_value::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep,semio_framework_plugin::PluginLifecycleFailure> {
        semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::close_step(instance_id, cx, grant)
    }

    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::terminal_is_empty(instance_id)
    }

    fn mounted_job_prepare_snapshot_read(operation: semio_framework_plugin::AppRenderOperationContext, snapshot: &Self::Snapshot, windows: &semio_framework_plugin::WindowConfigOwnerRegistry, view: Option<&ViewModel>, grant: semio_framework_value::RetainedCloneGrant) -> semio_framework_plugin::MountedSnapshotReadPreparation {
        semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::prepare_snapshot_read(operation, snapshot, windows, view, grant)
    }

    /// 🎞️ `"artifact:out"` reproduces the trait's default whole-document pack (overriding `export_media`
    /// shadows the trait's provided body for every port on this app, not just the new one). `"results:out"`
    /// runs every load case/combination's analysis fresh and returns them as plain JSON text in a
    /// `Structured` payload — `MediaPayload::Structured.json` doesn't require a `pack`-encoded value. A
    /// document with no load cases, or a solve failure, is reported as `MediaError::Payload` rather than
    /// an empty/panicking export.
    fn export_media(port: &str, doc: &ArtifactView<'_, Fem2dSnapshot>) -> Result<Media, MediaError> {
        match port {
            "artifact:out" => {
                let media_type = fem2d_io().artifact_media_type;
                let bytes = <Fem2dSnapshot as pack::snapshot::ArtifactPack>::encode_pack(doc.snapshot);
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            "results:out" => {
                if doc.snapshot.load_cases.is_empty() {
                    return Err(MediaError::Payload("results:out".into(), "no load cases defined".into()));
                }
                let results = semio_s_artifact_fem_2d_engine::fem2d_engine::fem2d_solve_all(doc.snapshot).map_err(|error| MediaError::Payload("results:out".into(), error.to_string()))?;
                let json = results_map_json(&results).to_string();
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.fem2d".into(), json } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `"artifact:in"` swaps the whole live document via `reset_document_effect` (a
    /// `Effect::LoadDocument`, the sanctioned non-history whole-doc-replace path — see
    /// `reset_document_effect`'s own doc comment).
    /// `"geometry:in"` decodes a minimal, app-owned `{"outline": [[f64;2]...], "holes": [[[f64;2]...]...]}`
    /// polygon-with-holes contract into a new `FemRegion` via `create-region`, defaulted to the
    /// document's first existing material if any, else an `"unassigned"` placeholder id.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Fem2dSnapshot>) -> Result<Emit<Fem2dMutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "artifact:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let snapshot = <Fem2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
            }
            "geometry:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "geometry:in only accepts a Structured JSON payload".into()));
                };
                let value = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let outline: Vec<[f64; 2]> = semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&value.get("outline").cloned().unwrap_or(Value::Null))).map_err(|error| MediaError::Payload(port.to_string(), format!("outline: {error}")))?;
                let holes: Vec<Vec<[f64; 2]>> = match value.get("holes").cloned() {
                    Some(holes_value) => semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&holes_value)).map_err(|error| MediaError::Payload(port.to_string(), format!("holes: {error}")))?,
                    None => Vec::new(),
                };
                let material_id = doc.snapshot.materials.first().map_or_else(|| "unassigned".into(), |material| material.id.clone());
                let id = doc.operation().and_then(|operation| operation.entity_id("r", "regions")).map_err(|fault| MediaError::Payload(port.into(), format!("{fault:?}")))?;
                let region = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemRegion { id, name: "Imported Geometry".into(), outline, holes, thickness: 0.02, material_id, mesh_size: 0.25 };
                Ok(Emit::mutations(vec![Fem2dMutation::CreateRegion(semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::create_region::CreateRegion { region, index: None })]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &Fem2dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + staged args onto `Fem2dCommand` — the React/wgpu shells still speak the
    /// stringly `{action, args}` wire (`ShellHost`'s action pane and its example switcher both do), and
    /// the trait's default rejects every app action outright, so without this bridge none of fem2d's
    /// nineteen declared actions can reach `dispatch`. Every key here is the `ActionArgDef.id` declared
    /// for that action in `🔖️Manifest` below.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let text = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
        let number = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_f64);
        let flag = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_bool);
        let list = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_array).map(|items| items.iter().filter_map(semio_framework_value::DslValue::as_str).map(str::to_string).collect::<Vec<_>>());
        // 🧵️ `samples: [[x, y], …]` — every well-formed 2-number pair in order; malformed entries are skipped.
        let point_list = |key: &str| {
            args.and_then(|value| value.get(key))
                .and_then(semio_framework_value::DslValue::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|item| {
                            let pair = item.as_array()?;
                            Some([pair.first()?.as_f64()?, pair.get(1)?.as_f64()?])
                        })
                        .collect::<Vec<[f64; 2]>>()
                })
                .unwrap_or_default()
        };
        // 🩹️ A control's `Trigger::Change` value arrives typed (number for sliders/number inputs,
        // bool for toggles, text for selects/inputs); every patch command carries it as text.
        let scalar_text = |key: &str| {
            args.and_then(|value| value.get(key)).and_then(|value| match value {
                semio_framework_value::DslValue::String(text) => Some(text.clone()),
                semio_framework_value::DslValue::Bool(flag) => Some(flag.to_string()),
                other => other.as_f64().map(|number| number.to_string()),
            })
        };
        match action {
            "addNode" => Ok(Fem2dCommand::AddNode(add_node::AddNode { x: number("x").unwrap_or_default(), y: number("y").unwrap_or_default() })),
            "addBar" => {
                Ok(Fem2dCommand::AddBar(add_bar::AddBar { start: text("start").unwrap_or_default(), end: text("end").unwrap_or_default(), material_id: text("materialId").unwrap_or_default(), section_id: text("sectionId").unwrap_or_default() }))
            }
            "addBeam" => {
                Ok(Fem2dCommand::AddBeam(add_beam::AddBeam { start: text("start").unwrap_or_default(), end: text("end").unwrap_or_default(), material_id: text("materialId").unwrap_or_default(), section_id: text("sectionId").unwrap_or_default() }))
            }
            "addMaterial" => Ok(Fem2dCommand::AddMaterial(add_material::AddMaterial { name: text("name").unwrap_or_default(), e: number("e").unwrap_or(2.1e11) })),
            "addSection" => Ok(Fem2dCommand::AddSection(add_section::AddSection { name: text("name").unwrap_or_default(), area: number("area").unwrap_or_default(), iy: number("iy").unwrap_or_default() })),
            "addSupport" => Ok(Fem2dCommand::AddSupport(add_support::AddSupport { node_id: text("nodeId").unwrap_or_default(), fixed: fem2d_dofs(text("fixed").as_deref()) })),
            "addNodalLoad" => Ok(Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad {
                node_id: text("nodeId").unwrap_or_default(),
                dof: fem2d_dof(text("dof").as_deref()).unwrap_or(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Ty),
                value: number("value").unwrap_or_default(),
                case_id: text("caseId").filter(|id| !id.is_empty()),
            })),
            "addMemberUdl" => Ok(Fem2dCommand::AddMemberUdl(add_member_udl::AddMemberUdl {
                element_id: text("elementId").unwrap_or_default(),
                wx: number("wx").unwrap_or_default(),
                wy: number("wy").unwrap_or_default(),
                case_id: text("caseId").filter(|id| !id.is_empty()),
            })),
            "addAreaLoad" => Ok(Fem2dCommand::AddAreaLoad(add_area_load::AddAreaLoad { region_id: text("regionId").unwrap_or_default(), pressure: number("pressure").unwrap_or_default(), case_id: text("caseId").filter(|id| !id.is_empty()) })),
            "addRegion" => Ok(Fem2dCommand::AddRegion(add_region::AddRegion {
                x: number("x").unwrap_or_default(),
                y: number("y").unwrap_or_default(),
                width: number("width").unwrap_or_default(),
                height: number("height").unwrap_or_default(),
                material_id: text("materialId").unwrap_or_default(),
                thickness: number("thickness"),
                mesh_size: number("meshSize"),
            })),
            "addLoadCase" => Ok(Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: text("name").unwrap_or_default(), self_weight: flag("selfWeight").unwrap_or(false) })),
            "addCombination" => Ok(Fem2dCommand::AddCombination(add_combination::AddCombination { name: text("name").unwrap_or_default(), terms: Vec::new() })),
            "setSelfWeight" => Ok(Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: text("caseId").unwrap_or_default(), enabled: flag("enabled").unwrap_or(false) })),
            "setAnalysisSettings" => Ok(Fem2dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings {
                modal_count: number("modalCount").map(|value| value.max(0.0) as u32),
                buckling_count: number("bucklingCount").map(|value| value.max(0.0) as u32),
                deformation_scale: number("deformationScale"),
                field: text("field"),
                value: scalar_text("value"),
                window_id: text("windowId"),
            })),
            "removeSelection" => Ok(Fem2dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: list("ids").unwrap_or_default() })),
            "setActiveExample" => Ok(Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: text("exampleId").or_else(|| text("id")).unwrap_or_default() })),
            "setCamera" => Ok(Fem2dCommand::SetCamera(set_camera::SetCamera { x: number("x").unwrap_or_default(), y: number("y").unwrap_or_default(), zoom: number("zoom").unwrap_or(1.0) })),
            "setResultDisplay" => Ok(Fem2dCommand::SetResultDisplay(set_result_display::SetResultDisplay {
                source_id: text("sourceId").filter(|id| !id.is_empty()),
                mode: text("mode").unwrap_or_else(|| "static".into()),
                mode_index: number("modeIndex").map(|value| value.max(0.0) as u32).unwrap_or_default(),
                field: text("field"),
                value: scalar_text("value"),
                window_id: text("windowId"),
            })),
            "canvasPointerDown" => Ok(Fem2dCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
                x: number("x").unwrap_or_default(),
                y: number("y").unwrap_or_default(),
                width: number("width").unwrap_or_default(),
                height: number("height").unwrap_or_default(),
                button: number("button").map(|value| value.max(0.0) as u32).unwrap_or_default(),
                shift: flag("shift").unwrap_or(false),
                ctrl: flag("ctrl").unwrap_or(false),
                meta: flag("meta").unwrap_or(false),
                alt: flag("alt").unwrap_or(false),
            })),
            "canvasPointerMove" => {
                let x = number("x").unwrap_or_default();
                let y = number("y").unwrap_or_default();
                let mut samples = point_list("samples");
                if samples.is_empty() {
                    samples.push([x, y]);
                }
                Ok(Fem2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x, y, width: number("width").unwrap_or_default(), height: number("height").unwrap_or_default(), samples }))
            }
            "canvasPointerUp" => Ok(Fem2dCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {
                x: number("x").unwrap_or_default(),
                y: number("y").unwrap_or_default(),
                width: number("width").unwrap_or_default(),
                height: number("height").unwrap_or_default(),
                shift: flag("shift").unwrap_or(false),
                ctrl: flag("ctrl").unwrap_or(false),
                meta: flag("meta").unwrap_or(false),
                alt: flag("alt").unwrap_or(false),
                cancelled: flag("cancelled").unwrap_or(false),
            })),
            "patchNode" => Ok(Fem2dCommand::PatchNode(patch_node::PatchNode { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchElement" => Ok(Fem2dCommand::PatchElement(patch_element::PatchElement { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchMaterial" => Ok(Fem2dCommand::PatchMaterial(patch_material::PatchMaterial { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchSection" => Ok(Fem2dCommand::PatchSection(patch_section::PatchSection { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchSupport" => Ok(Fem2dCommand::PatchSupport(patch_support::PatchSupport { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchRegion" => Ok(Fem2dCommand::PatchRegion(patch_region::PatchRegion { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchLoad" => Ok(Fem2dCommand::PatchLoad(patch_load::PatchLoad { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchLoadCase" => Ok(Fem2dCommand::PatchLoadCase(patch_load_case::PatchLoadCase { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "patchCombination" => Ok(Fem2dCommand::PatchCombination(patch_combination::PatchCombination { id: text("id").unwrap_or_default(), field: text("field").unwrap_or_default(), value: scalar_text("value").unwrap_or_default() })),
            "setResultAnimation" => Ok(Fem2dCommand::SetResultAnimation(set_result_animation::SetResultAnimation { phase: number("phase"), playing: flag("playing"), speed: number("speed"), loop_mode: text("loopMode"), waveform: text("waveform"), field: text("field"), value: scalar_text("value"), window_id: text("windowId") })),
            "resultAnimationTick" => Ok(Fem2dCommand::ResultAnimationTick(result_animation_tick::ResultAnimationTick { window_id: text("windowId").unwrap_or_default() })),
            "focusEntity" => Ok(Fem2dCommand::FocusEntity(focus_entity::FocusEntity { id: text("id").unwrap_or_default() })),
            "translateSelection" => Ok(Fem2dCommand::TranslateSelection(translate_selection::TranslateSelection {
                ids: list("ids").unwrap_or_default(),
                dx: number("dx").unwrap_or_default(),
                dy: number("dy").unwrap_or_default(),
                dz: number("dz").unwrap_or_default(),
                phase: text("phase"),
                reason: text("reason"),
            })),
            "rotateSelection" => Ok(Fem2dCommand::RotateSelection(rotate_selection::RotateSelection {
                ids: list("ids").unwrap_or_default(),
                ax: number("ax").unwrap_or_default(),
                ay: number("ay").unwrap_or_default(),
                az: number("az").unwrap_or(1.0),
                angle: number("angle").unwrap_or_default(),
                phase: text("phase"),
                reason: text("reason"),
            })),
            "scaleSelection" => Ok(Fem2dCommand::ScaleSelection(scale_selection::ScaleSelection {
                ids: list("ids").unwrap_or_default(),
                sx: number("sx").unwrap_or(1.0),
                sy: number("sy").unwrap_or(1.0),
                sz: number("sz").unwrap_or(1.0),
                phase: text("phase"),
                reason: text("reason"),
            })),
            "setTransformGumballFlag" => Ok(Fem2dCommand::SetTransformGumballFlag(set_transform_gumball_flag::SetTransformGumballFlag { flag: text("flag").unwrap_or_default(), pressed: flag("pressed") })),
            other => Err(Fault::from(format!("action '{other}' is not a declared fem2d action — every app action is dispatched through the typed command channel (see `dispatch_typed_command`)"))),
        }
    }

    fn handle(
        command: &Fem2dCommand,
        doc: &ArtifactView<'_, Fem2dSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Fem2dMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            // 🗑️ The delete keybinding dispatches `removeSelection` without ids: the live framework-owned
            // `"fem2d"` selection is what gets removed.
            Fem2dCommand::RemoveSelection(payload) if payload.ids.is_empty() => remove_selection::handle(&remove_selection::RemoveSelection { ids: interaction.selection(FEM2D_INTERACTION_DOMAIN).ids.clone() }, doc, cfg),
            Fem2dCommand::TranslateSelection(payload) => {
                let payload = if payload.ids.is_empty() {
                    translate_selection::TranslateSelection { ids: interaction.selection(FEM2D_INTERACTION_DOMAIN).ids.clone(), ..payload.clone() }
                } else {
                    payload.clone()
                };
                translate_selection::handle(&payload, doc, cfg)
            }
            Fem2dCommand::RotateSelection(payload) => {
                let payload = if payload.ids.is_empty() {
                    rotate_selection::RotateSelection { ids: interaction.selection(FEM2D_INTERACTION_DOMAIN).ids.clone(), ..payload.clone() }
                } else {
                    payload.clone()
                };
                rotate_selection::handle(&payload, doc, cfg)
            }
            Fem2dCommand::ScaleSelection(payload) => {
                let payload = if payload.ids.is_empty() {
                    scale_selection::ScaleSelection { ids: interaction.selection(FEM2D_INTERACTION_DOMAIN).ids.clone(), ..payload.clone() }
                } else {
                    payload.clone()
                };
                scale_selection::handle(&payload, doc, cfg)
            }
            Fem2dCommand::SetTransformGumballFlag(payload) => set_transform_gumball_flag::handle_window(payload, view_state.ok_or_else(|| Fault::from("fem2d.gumball-flag.window-context-required"))?),
            Fem2dCommand::SetCamera(payload) => set_camera::handle_window(payload, cfg, view_state.ok_or_else(|| Fault::from("fem2d.camera.window-context-required"))?),
            Fem2dCommand::SetResultDisplay(payload) => {
                set_result_display::handle_window(payload, cfg, view_state.ok_or_else(|| Fault::from("fem2d.results.window-context-required"))?)
            },
            Fem2dCommand::CanvasPointerDown(payload) => canvas_pointer_down::handle_window(payload, doc, cfg, view_state.ok_or_else(|| Fault::from("fem2d.pointer.window-context-required"))?),
            Fem2dCommand::CanvasPointerMove(payload) => canvas_pointer_move::handle_window(payload, doc, cfg, view_state.ok_or_else(|| Fault::from("fem2d.pointer.window-context-required"))?),
            Fem2dCommand::CanvasPointerUp(payload) => canvas_pointer_up::handle_window(payload, doc, cfg, view_state.ok_or_else(|| Fault::from("fem2d.pointer.window-context-required"))?),
            Fem2dCommand::SetResultAnimation(payload) => set_result_animation::handle_window(payload, doc, cfg, view_state.ok_or_else(|| Fault::from("fem2d.results.window-context-required"))?),
            Fem2dCommand::ResultAnimationTick(payload) => result_animation_tick::handle_window(payload, doc, cfg, view_state.ok_or_else(|| Fault::from("fem2d.results.window-context-required"))?),
            Fem2dCommand::FocusEntity(payload) => focus_entity::handle_window(payload, doc, cfg, view_state.ok_or_else(|| Fault::from("fem2d.camera.window-context-required"))?),
            _ => command.dispatch(doc, cfg),
        }
    }

    fn pending_effects(_owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, doc: &semio_framework_plugin::ArtifactPendingView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view: Option<&ViewModel>) -> Vec<semio_framework::kernel::Effect> {
        semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::reconcile(doc)
    }

    /// 🎯️ Fem2d has no user-visible config defaults to expose (all of `addRegion`'s
    /// `thickness`/`meshSize` defaults are baked directly into its handler, not user-configurable
    /// settings) — declaring `ConfigSpec::empty()` explicitly keeps the typed channel surface
    /// consistent with the sibling apps' convention.
    fn config_spec() -> ConfigSpec {
        ConfigSpec::default()
    }

    /// 🕹️ Interaction-less twin of [`Self::render_with_request_context`] — an empty `"fem2d"` domain,
    /// so nothing paints selected and the inspector shows the document summary.
    fn render(body_key: &str, doc: &ArtifactView<'_, Fem2dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(body_key, doc, None, cfg, view_state, Fem2dInteractionSnapshot::default(), None)
    }

    /// 🕹️ Reads the framework-owned `"fem2d"` selection/hover once per render and threads it through
    /// every body — the windows paint it, the artifact tree marks it, the inspector edits it.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Fem2dSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_s_artifact_fem_2d_editor_transient::editor::fem2d::transient::FemGumballTransient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        let clock = transient.window::<results_window::transient::Fem2dResultsWindowTransientOwner>().and_then(|window| window.clock);
        let preview = transient.snapshot.preview::<Fem2dSnapshot, Fem2dMutation>(doc.snapshot);
        Self::render_body(body_key, doc, preview.as_ref(), cfg, view_state, Fem2dInteractionSnapshot::from_interaction(interaction), clock)

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_measures(_doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = fem2d_labels(view_state);
        let measures = crate::editor::fem2d::window_measures::fem2d_window_measures(view_state, labels);
        HashMap::from([(model_window::WINDOW_KIND_ID.into(), measures.clone()), (results_window::WINDOW_KIND_ID.into(), measures)])
    }
}

impl Fem2dPlayApp {
    /// 🖼️ Body-key routing table shared by both render entry points: two Canvas2d windows and the
    /// three dock panels. While a gumball gesture is open both canvas windows paint its `preview` (the committed
    /// document with the open transaction applied, never history); the panels keep the committed document. The results
    /// window draws its transport with the running playback `clock` folded in while it plays.
    fn render_body(body_key: &str, doc: &ArtifactView<'_, Fem2dSnapshot>, preview: Option<&Fem2dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel, interaction: Fem2dInteractionSnapshot, clock: Option<results_window::transient::FemPlaybackClock>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let labels = fem2d_labels(view_state);
        let canvas = preview.unwrap_or(doc.snapshot);
        match body_key {
            model_window::BODY_KEY => {
                let window = model_window::config::current(cfg);
                let window_id = semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::canvas_gesture::fem2d_gesture_window_id_for_render(view_state, model_window::WINDOW_KIND_ID);
                let active_utility = semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::canvas_gesture::fem2d_active_utility(view_state);
                semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::with_live_visual(doc.render_operation(), |visual| {
                    model_window::render_with_progress(canvas, &window.camera, visual, &interaction, window_id.as_deref(), active_utility)
                })
            }
            results_window::BODY_KEY => {
                let window = results_window::config::effective(&results_window::config::current(cfg), clock.as_ref());
                let window_id = semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::canvas_gesture::fem2d_gesture_window_id_for_render(view_state, results_window::WINDOW_KIND_ID);
                let active_utility = semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::canvas_gesture::fem2d_active_utility(view_state);
                results_window::render(
                    canvas,
                    &config_result_display(&window),
                    &window.camera,
                    &window,
                    &interaction,
                    doc.render_operation(),
                    window_id.as_deref(),
                    cfg.window.map(|window|window.authority()),
                    active_utility,
                    labels,
                )
            }
            artifact_panel::BODY_KEY => artifact_panel::render(doc.snapshot, &interaction, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, artifact_panel::BODY_KEY)),
            inspection_panel::BODY_KEY => inspection_panel::render(doc.snapshot, &interaction, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, inspection_panel::BODY_KEY)),
            results_panel::BODY_KEY => results_panel::render(doc.snapshot, results_window::config::captured(cfg).as_ref(), clock.as_ref(), &results_window_instance_id(view_state).unwrap_or_default(), labels),
            _ => built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "fem2d unknown-body label admission failed")),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }
}
//#endregion 🔖️Fem2dPlayApp
