//! 🖥️ Play app of the FEM 3D editor: the `Fem3dPlayApp` and its `ArtifactEditor` implementation. See [`super`].

use super::*;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Fem3dPlayApp
/// 🧮️ v0 design: results are recomputed fresh inside `render()`, no cache, no `RunAnalysis` operation.
/// Persisted viewport preferences belong to their exact Model or Results window configuration.
#[derive(Default)]
pub struct Fem3dPlayApp;

impl ArtifactEditor for Fem3dPlayApp {
    /// 📢️ The localized notices of the gumball and canvas refusals (design §20.12), the fem 2d table.
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        semio_s_artifact_fem_2d_editor::editor::fem2d::commands::gumball::fem_fault_notices()
    }

    type Snapshot = Fem3dSnapshot;
    type Mutation = Fem3dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_s_artifact_fem_2d_editor_transient::editor::fem2d::transient::FemGumballTransient;
    type TransientMutation = semio_s_artifact_fem_2d_editor_transient::editor::fem2d::transient::FemGumballTransientMutation;

    type Command = Fem3dCommand;

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        None
    }

    /// 🪪️ W2 packet P7: the canonical `ArtifactEditor::DIALECT`, derived from the artifact-level
    /// `FEM3D_DIALECT` constant (`🗿️artifacts/🧊️3d/🦀️.rs`) so the sibling `👁️viewer` surface
    /// can read the very same value without ever importing through this `editor` module.
    const DIALECT: Dialect = semio_s_artifact_fem_3d_io::standards::v1::subsets::any::io::FEM3D_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FEM_3D_SCHEMA;

    /// 📬️ Required by the Artifact publication lane — see `Fem3dArtifactPreparationFactory`.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Fem3dArtifactPreparationFactory))
    }




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
        registry.register::<window_model::config::Fem3dModelWindowConfigOwner>()?;
        registry.register::<window_results::config::Fem3dResultsWindowConfigOwner>()
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        registry.register::<window_results::transient::Fem3dResultsWindowTransientOwner>()
    }

    /// 🎯️ The playback chain publishes into ONE results window's transient: the tick names it off its
    /// PAYLOAD (the shell redispatches a self-armed `DispatchAction` under whichever pane is
    /// current), and a panel-tagged transport gesture names it the same way so a pause can rest
    /// where the running clock is. The runtime validates the id against the trusted roster and
    /// captures that exact window's mutation authority.
    fn retained_window_transient_target(command: &Fem3dCommand) -> Option<(&str, &'static str)> {
        match command {
            Fem3dCommand::ResultAnimationTick(payload) if !payload.window_id.is_empty() => Some((payload.window_id.as_str(), window_results::transient::WINDOW_KIND_ID)),
            Fem3dCommand::SetResultAnimation(payload) => payload.window_id.as_deref().filter(|id| !id.is_empty()).map(|id| (id, window_results::transient::WINDOW_KIND_ID)),
            _ => None,
        }
    }



    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Fem3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs",
        controller: "s.fem.fem3d@1/*#editor",
        artifact_schema: "fem.3d",
        factory: "Fem3dRetainedCommandJobFactory",
        factory_type: Fem3dRetainedCommandJobFactory,
        contract: fem3d_retained_contract(),
        tools: [
            "addNode",
            "addBar",
            "addFrame",
            "addMaterial",
            "addSection",
            "addSupport",
            "addNodalLoad",
            "addMemberUdl",
            "addAreaLoad",
            "addSolid",
            "addLoadCase",
            "addCombination",
            "setSelfWeight",
            "setAnalysisSettings",
            "removeSelection",
            "setActiveExample",
            "setCamera",
            "setResultDisplay",
            "patchNode",
            "patchElement",
            "patchMaterial",
            "patchSection",
            "patchSupport",
            "patchSolid",
            "patchLoad",
            "patchLoadCase",
            "patchCombination",
            "setResultAnimation",
            "resultAnimationTick",
            "focusEntity",
            "translateSelection",
            "rotateSelection",
            "scaleSelection",
            "setTransformGumballFlag"
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Fem3dRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original FEM request is absent"))?;
        if !FEM3D_RETAINED_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "fem3d retained command does not match its exact registered tool").into());
        }
        if fem3d_retained_extent(&original.command, &original.snapshot, &original.interaction_state).is_none() {
            return Err(Fault::from("fem3d-command-payload-too-large").into());
        }

        let tool_id = original.command.command_id();
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, Fem3dCommand::command_id, FEM3D_RETAINED_RAW_BYTES, FEM3D_RETAINED_WORK_ITEMS, true, |_original| Fem3dCommandWork::new(tool_id))
    }

    /// 🚀️ Boots on the bundled `default` example instead of the empty document — the Model window is a
    /// `World3d` surface fed by `live_visual`, and an empty boot document meshes to nothing, so the very
    /// first paint was a blank scene until a client dispatched `setActiveExample`. Mirrors the sibling
    /// `Fem3dViewer::initial_snapshot` (and block3d's `block3d_boot_snapshot`) so editor and viewer boot
    /// the same geometry. See `semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::fem3d_boot_snapshot`.
    fn initial_snapshot() -> Fem3dSnapshot {
        let snapshot = semio_s_artifact_fem_3d_io::standards::v1::subsets::any::io::fem3d_boot_snapshot();
        eprintln!("[TRACE] fem3d editor boot snapshot: nodes={} elements={} solids={} materials={} loadCases={}", snapshot.nodes.len(), snapshot.elements.len(), snapshot.solids.len(), snapshot.materials.len(), snapshot.load_cases.len());
        snapshot
    }

    fn io() -> Option<AppIo> {
        Some(fem3d_io())
    }

    fn mounted_job_maintenance_phase(instance_id: u32) -> Option<semio_framework_plugin::MountedOwnerPhaseV1> {
        (!semio_s_artifact_fem_3d_editor_session::editor::fem3d::session::terminal_is_empty(instance_id)).then_some(semio_framework_plugin::MountedOwnerPhaseV1::Maintenance)
    }

    fn mounted_job_maintenance_demands(instance_id: u32, _body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_artifact_fem_3d_editor_session::editor::fem3d::session::close_demands(instance_id)
    }

    fn mounted_job_close_demands(instance_id: u32, _body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_artifact_fem_3d_editor_session::editor::fem3d::session::close_demands(instance_id)
    }

    fn mounted_job_maintenance_step(instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,grant: semio_framework_value::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep,semio_framework_plugin::PluginLifecycleFailure> {
        semio_s_artifact_fem_3d_editor_session::editor::fem3d::session::maintenance_step(instance_id, cx, grant)
    }

    fn mounted_job_close_step(instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,grant: semio_framework_value::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep,semio_framework_plugin::PluginLifecycleFailure> {
        semio_s_artifact_fem_3d_editor_session::editor::fem3d::session::close_step(instance_id, cx, grant)
    }

    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        semio_s_artifact_fem_3d_editor_session::editor::fem3d::session::terminal_is_empty(instance_id)
    }

    fn mounted_job_prepare_snapshot_read(operation: AppRenderOperationContext, snapshot: &Self::Snapshot, windows: &semio_framework_plugin::WindowConfigOwnerRegistry, view: Option<&semio_framework_plugin::ViewModel>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_plugin::MountedSnapshotReadPreparation {
        crate::live_visual::prepare_snapshot_read(operation, snapshot, windows, view, grant)
    }

    /// 🎞️ `"artifact:out"` reproduces the trait's default whole-document pack (overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new
    /// one). `"results:out"` runs every load case/combination's analysis fresh and returns them as plain
    /// JSON text in a `Structured` payload. A document with no load cases, or a solve failure, is
    /// reported as `MediaError::Payload` rather than an empty/panicking export.
    fn export_media(port: &str, doc: &ArtifactView<'_, Fem3dSnapshot>) -> Result<Media, MediaError> {
        match port {
            "artifact:out" => {
                let media_type = fem3d_io().artifact_media_type;
                let bytes = <Fem3dSnapshot as pack::snapshot::ArtifactPack>::encode_pack(doc.snapshot);
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            "results:out" => {
                if doc.snapshot.load_cases.is_empty() {
                    return Err(MediaError::Payload("results:out".into(), "no load cases defined".into()));
                }
                let results = semio_s_artifact_fem_3d_engine::fem3d_engine::fem3d_solve_all(doc.snapshot).map_err(|error| MediaError::Payload("results:out".into(), error.to_string()))?;
                let json = fem3d_results_map_json(&results).to_string();
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.fem3d".into(), json } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `"artifact:in"` swaps the whole live document via `reset_document_effect` (a
    /// `Effect::LoadDocument`, the sanctioned non-history whole-doc-replace path — see
    /// `reset_document_effect`'s own doc comment).
    /// `"geometry:in"` decodes a minimal, app-owned `{"outline": [[f64;2]...], "holes": [[[f64;2]...]...],
    /// "baseZ"?: f64, "height"?: f64, "layers"?: usize}` extruded-footprint contract into a new
    /// `FemSolid`, defaulted to the document's first existing material if any, else an `"unassigned"`
    /// placeholder id — the solid simply won't solve until a real material is assigned.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Fem3dSnapshot>) -> Result<Emit<Fem3dMutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "artifact:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let snapshot = <Fem3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
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
                let base_z = value.get("baseZ").and_then(Value::as_f64).unwrap_or(0.0);
                let height = value.get("height").and_then(Value::as_f64).unwrap_or(1.0);
                let layers = value.get("layers").and_then(Value::as_u64).map_or(1, |v| v as usize);
                let axis = value.get("axis").and_then(Value::as_str).and_then(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemAxis::from_key).unwrap_or_default();
                let material_id = doc.snapshot.materials.first().map_or_else(|| "unassigned".into(), |material| material.id.clone());
                let id = doc.operation().and_then(|operation| operation.entity_id("sol", "solids")).map_err(|fault| MediaError::Payload(port.into(), format!("{fault:?}")))?;
                let solid = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSolid { id, name: "Imported Geometry".into(), outline, holes, base_z, height, layers, mesh_size: 0.5, material_id, axis };
                Ok(Emit::mutations(vec![Fem3dMutation::CreateSolid(semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::create_solid::CreateSolid { solid, index: None })]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧮️ No sticky `ActionArgDef` defaults are mirrored here (all of `addSolid`'s
    /// `baseZ`/`layers`/`meshSize` defaults are baked directly into its handler, not user-configurable
    /// settings).
    fn config_spec() -> ConfigSpec {
        ConfigSpec::default()
    }

    fn command_id(command: &Fem3dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + staged args onto `Fem3dCommand` — the React/wgpu shells still speak the
    /// stringly `{action, args}` wire (`ShellHost`'s action pane and its example switcher both do), and
    /// the trait's default rejects every app action outright, so without this bridge none of fem3d's
    /// eighteen declared actions can reach `dispatch`. Every key here is the `ActionArgDef.id` declared
    /// for that action in `🔖️Manifest` below; `setCamera` carries the host's whole orbit record.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let text = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
        let number = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_f64);
        let flag = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_bool);
        let list = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_array).map(|items| items.iter().filter_map(semio_framework_value::DslValue::as_str).map(str::to_string).collect::<Vec<_>>());
        // 🩹️ A control's `Trigger::Change` value arrives typed (number for sliders/number inputs,
        // bool for toggles, text for selects/inputs); every patch command carries it as text.
        let scalar_text = |key: &str| {
            args.and_then(|value| value.get(key)).and_then(|value| match value {
                semio_framework_value::DslValue::String(text) => Some(text.clone()),
                semio_framework_value::DslValue::Bool(flag) => Some(flag.to_string()),
                other => other.as_f64().map(|number| number.to_string()),
            })
        };
        let patch = |key: &str| (text("id").unwrap_or_default(), text("field").unwrap_or_default(), scalar_text(key).unwrap_or_default());
        match action {
            "addNode" => Ok(Fem3dCommand::AddNode(add_node::AddNode { x: number("x").unwrap_or_default(), y: number("y").unwrap_or_default(), z: number("z").unwrap_or_default() })),
            "addBar" => {
                Ok(Fem3dCommand::AddBar(add_bar::AddBar { start: text("start").unwrap_or_default(), end: text("end").unwrap_or_default(), material_id: text("materialId").unwrap_or_default(), section_id: text("sectionId").unwrap_or_default() }))
            }
            "addFrame" => Ok(Fem3dCommand::AddFrame(add_frame::AddFrame {
                start: text("start").unwrap_or_default(),
                end: text("end").unwrap_or_default(),
                material_id: text("materialId").unwrap_or_default(),
                section_id: text("sectionId").unwrap_or_default(),
                roll: number("roll").unwrap_or_default(),
            })),
            "addMaterial" => Ok(Fem3dCommand::AddMaterial(add_material::AddMaterial { name: text("name").unwrap_or_default(), e: number("e").unwrap_or(2.1e11), g: number("g").unwrap_or(8.1e10) })),
            "addSection" => Ok(Fem3dCommand::AddSection(add_section::AddSection {
                name: text("name").unwrap_or_default(),
                area: number("area").unwrap_or_default(),
                iy: number("iy").unwrap_or_default(),
                iz: number("iz").unwrap_or_default(),
                j: number("j").unwrap_or_default(),
            })),
            "addSupport" => Ok(Fem3dCommand::AddSupport(add_support::AddSupport { node_id: text("nodeId").unwrap_or_default(), fixed: fem3d_dofs(text("fixed").as_deref()) })),
            "addNodalLoad" => Ok(Fem3dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad {
                node_id: text("nodeId").unwrap_or_default(),
                dof: fem3d_dof(text("dof").as_deref()).unwrap_or(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Tz),
                value: number("value").unwrap_or_default(),
                case_id: text("caseId").filter(|id| !id.is_empty()),
            })),
            "addMemberUdl" => Ok(Fem3dCommand::AddMemberUdl(add_member_udl::AddMemberUdl {
                element_id: text("elementId").unwrap_or_default(),
                wx: number("wx").unwrap_or_default(),
                wy: number("wy").unwrap_or_default(),
                wz: number("wz").unwrap_or_default(),
                case_id: text("caseId").filter(|id| !id.is_empty()),
            })),
            "addAreaLoad" => Ok(Fem3dCommand::AddAreaLoad(add_area_load::AddAreaLoad { solid_id: text("solidId").unwrap_or_default(), pressure: number("pressure").unwrap_or_default(), case_id: text("caseId").filter(|id| !id.is_empty()) })),
            "addSolid" => Ok(Fem3dCommand::AddSolid(add_solid::AddSolid {
                x: number("x").unwrap_or_default(),
                y: number("y").unwrap_or_default(),
                width: number("width").unwrap_or_default(),
                depth: number("depth").unwrap_or_default(),
                height: number("height").unwrap_or_default(),
                material_id: text("materialId").unwrap_or_default(),
                base_z: number("baseZ"),
                layers: number("layers").map(|value| value.max(0.0) as u32),
                mesh_size: number("meshSize"),
                axis: text("axis").and_then(|value| semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemAxis::from_key(&value)),
            })),
            "addLoadCase" => Ok(Fem3dCommand::AddLoadCase(add_load_case::AddLoadCase { name: text("name").unwrap_or_default(), self_weight: flag("selfWeight").unwrap_or(false) })),
            "addCombination" => Ok(Fem3dCommand::AddCombination(add_combination::AddCombination { name: text("name").unwrap_or_default(), terms: text("terms").unwrap_or_else(|| "[]".into()) })),
            "setSelfWeight" => Ok(Fem3dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: text("caseId").unwrap_or_default(), enabled: flag("enabled").unwrap_or(false) })),
            "setAnalysisSettings" => Ok(Fem3dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings {
                modal_count: number("modalCount").map(|value| value.max(0.0) as u32),
                buckling_count: number("bucklingCount").map(|value| value.max(0.0) as u32),
                deformation_scale: number("deformationScale"),
                field: text("field"),
                value: scalar_text("value"),
                window_id: text("windowId"),
            })),
            "removeSelection" => Ok(Fem3dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: list("ids").unwrap_or_default() })),
            "setActiveExample" => Ok(Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: text("exampleId").or_else(|| text("id")).unwrap_or_default() })),
            "setCamera" => {
                let camera = args.and_then(|value| value.get("camera")).cloned().ok_or_else(|| Fault::from("fem3d.camera.orbit-required"))?;
                let camera: semio_framework_os_kernel::Viewport3dOrbit = semio_framework_value::FromValue::from_value(camera).map_err(|error| Fault::from(format!("fem3d.camera.orbit-invalid: {error}")))?;
                Ok(Fem3dCommand::SetCamera(set_camera::SetCamera { camera }))
            }
            "setResultDisplay" => Ok(Fem3dCommand::SetResultDisplay(set_result_display::SetResultDisplay {
                source_id: text("sourceId").filter(|id| !id.is_empty()),
                mode: text("mode").unwrap_or_else(|| "static".into()),
                mode_index: number("modeIndex").map(|value| value.max(0.0) as u32).unwrap_or_default(),
                field: text("field"),
                value: scalar_text("value"),
                window_id: text("windowId"),
            })),
            "patchNode" => {
                let (id, field, value) = patch("value");
                Ok(Fem3dCommand::PatchNode(patch_node::PatchNode { id, field, value }))
            }
            "patchElement" => {
                let (id, field, value) = patch("value");
                Ok(Fem3dCommand::PatchElement(patch_element::PatchElement { id, field, value }))
            }
            "patchMaterial" => {
                let (id, field, value) = patch("value");
                Ok(Fem3dCommand::PatchMaterial(patch_material::PatchMaterial { id, field, value }))
            }
            "patchSection" => {
                let (id, field, value) = patch("value");
                Ok(Fem3dCommand::PatchSection(patch_section::PatchSection { id, field, value }))
            }
            "patchSupport" => {
                let (id, field, value) = patch("value");
                Ok(Fem3dCommand::PatchSupport(patch_support::PatchSupport { id, field, value }))
            }
            "patchSolid" => {
                let (id, field, value) = patch("value");
                Ok(Fem3dCommand::PatchSolid(patch_solid::PatchSolid { id, field, value }))
            }
            "patchLoad" => {
                let (id, field, value) = patch("value");
                Ok(Fem3dCommand::PatchLoad(patch_load::PatchLoad { id, field, value }))
            }
            "patchLoadCase" => {
                let (id, field, value) = patch("value");
                Ok(Fem3dCommand::PatchLoadCase(patch_load_case::PatchLoadCase { id, field, value }))
            }
            "patchCombination" => {
                let (id, field, value) = patch("value");
                Ok(Fem3dCommand::PatchCombination(patch_combination::PatchCombination { id, field, value }))
            }
            "setResultAnimation" => Ok(Fem3dCommand::SetResultAnimation(set_result_animation::SetResultAnimation { phase: number("phase"), playing: flag("playing"), speed: number("speed"), loop_mode: text("loopMode"), waveform: text("waveform"), field: text("field"), value: scalar_text("value"), window_id: text("windowId") })),
            "resultAnimationTick" => Ok(Fem3dCommand::ResultAnimationTick(result_animation_tick::ResultAnimationTick { window_id: text("windowId").unwrap_or_default() })),
            "focusEntity" => Ok(Fem3dCommand::FocusEntity(focus_entity::FocusEntity { id: text("id").unwrap_or_default() })),
            "translateSelection" => Ok(Fem3dCommand::TranslateSelection(translate_selection::TranslateSelection {
                ids: list("ids").unwrap_or_default(),
                dx: number("dx").unwrap_or_default(),
                dy: number("dy").unwrap_or_default(),
                dz: number("dz").unwrap_or_default(),
                phase: text("phase"),
                reason: text("reason"),
            })),
            "rotateSelection" => Ok(Fem3dCommand::RotateSelection(rotate_selection::RotateSelection {
                ids: list("ids").unwrap_or_default(),
                ax: number("ax").unwrap_or_default(),
                ay: number("ay").unwrap_or_default(),
                az: number("az").unwrap_or(1.0),
                angle: number("angle").unwrap_or_default(),
                phase: text("phase"),
                reason: text("reason"),
            })),
            "scaleSelection" => Ok(Fem3dCommand::ScaleSelection(scale_selection::ScaleSelection {
                ids: list("ids").unwrap_or_default(),
                sx: number("sx").unwrap_or(1.0),
                sy: number("sy").unwrap_or(1.0),
                sz: number("sz").unwrap_or(1.0),
                phase: text("phase"),
                reason: text("reason"),
            })),
            "setTransformGumballFlag" => Ok(Fem3dCommand::SetTransformGumballFlag(set_transform_gumball_flag::SetTransformGumballFlag { flag: text("flag").unwrap_or_default(), pressed: flag("pressed") })),
            other => Err(Fault::from(format!("action '{other}' is not a declared fem3d action — every app action is dispatched through the typed command channel (see `dispatch_typed_command`)"))),
        }
    }

    fn handle(
        command: &Fem3dCommand,
        doc: &ArtifactView<'_, Fem3dSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Fem3dMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        fem3d_route(command, doc, cfg, || interaction.selection(FEM3D_INTERACTION_DOMAIN).ids.clone(), view_state)
    }

    fn pending_effects(_owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, doc: &semio_framework_plugin::ArtifactPendingView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view: Option<&ViewModel>) -> Vec<semio_framework::kernel::Effect> {
        crate::live_visual::reconcile(doc)
    }
    /// 🕹️ Interaction-less twin of [`Self::render_with_request_context`] — an empty `"fem3d"` domain,
    /// so nothing paints selected and the inspector shows the document summary.
    fn render(body_key: &str, doc: &ArtifactView<'_, Fem3dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(body_key, doc, None, cfg, view_state, Fem3dInteractionSnapshot::default(), None)
    }

    /// 🕹️ Reads the framework-owned `"fem3d"` selection/hover once per render and threads it through
    /// every body — the windows paint it, the artifact tree marks it, the inspector edits it — plus the
    /// open gumball gestures' preview from the FEM gumball transient.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Fem3dSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_s_artifact_fem_2d_editor_transient::editor::fem2d::transient::FemGumballTransient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        let clock = transient.window::<window_results::transient::Fem3dResultsWindowTransientOwner>().and_then(|window| window.clock);
        let preview = transient.snapshot.preview::<Fem3dSnapshot, Fem3dMutation>(doc.snapshot);
        Self::render_body(body_key, doc, preview.as_ref(), cfg, view_state, Fem3dInteractionSnapshot::from_interaction(interaction), clock)

        })();
        rendered.map(Some).map_err(Into::into)
        }

    /// 🎛️ The model window's utility options — the transform gumball's handle flags, read from the
    /// captured model-window config (defaults when another pane is focused).
    fn window_measures(_doc: &ArtifactView<'_, Fem3dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = fem3d_labels(view_state);
        let gumball = window_model::config::captured(cfg).map(|window| window.gumball).unwrap_or_default();
        HashMap::from([(window_model::FEM3D_WINDOW_MODEL.into(), crate::editor::fem3d::window_measures::fem3d_window_measures(&gumball, labels))])
    }
}

impl Fem3dPlayApp {
    /// 🖼️ Body-key routing table shared by both render entry points: two World3d windows and the
    /// three dock panels. While a gumball gesture is open both world windows paint its `preview` (the committed
    /// document with the open transaction applied, never history): the model window from its own meshes rather than
    /// the committed revision's live visual pages, the results window re-solved uncached; the panels keep the
    /// committed document.
    pub(super) fn render_body(
        body_key: &str,
        doc: &ArtifactView<'_, Fem3dSnapshot>,
        preview: Option<&Fem3dSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &ViewModel,
        interaction: Fem3dInteractionSnapshot,
        clock: Option<window_results::transient::FemPlaybackClock>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let labels = fem3d_labels(view_state);
        match body_key {
            window_model::FEM3D_BODY_MODEL => {
                let window = window_model::config::current(cfg);
                let transform_armed = fem3d_transform_armed(view_state);
                match preview {
                    Some(preview) => window_model::render_with_progress(preview, &window, &interaction, transform_armed, None),
                    None => semio_s_artifact_fem_3d_editor_session::editor::fem3d::session::with_live_visual(doc.render_operation(), |visual| window_model::render_with_progress(doc.snapshot, &window, &interaction, transform_armed, visual)),
                }
            }
            window_results::FEM3D_BODY_RESULTS => {
                let window = window_results::config::effective(&window_results::config::current(cfg), clock.as_ref());
                match preview {
                    Some(preview) => window_results::render_pending(preview, &window, &interaction, doc.render_operation(), cfg.window.map(|window| window.authority())),
                    None => window_results::render(doc.snapshot, &window, &interaction, doc.render_operation(), cfg.window.map(|window| window.authority())),
                }
            }
            artifact_panel::BODY_KEY => artifact_panel::render(doc.snapshot, &interaction, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, artifact_panel::BODY_KEY)),
            inspection_panel::BODY_KEY => inspection_panel::render(doc.snapshot, &interaction, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, inspection_panel::BODY_KEY)),
            results_panel::BODY_KEY => results_panel::render(doc.snapshot, window_results::config::captured(cfg).as_ref(), clock.as_ref(), &results_window_instance_id(view_state).unwrap_or_default(), labels),
            _ => built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "fem3d unknown-body label admission failed")),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }
}
//#endregion 🔖️Fem3dPlayApp
