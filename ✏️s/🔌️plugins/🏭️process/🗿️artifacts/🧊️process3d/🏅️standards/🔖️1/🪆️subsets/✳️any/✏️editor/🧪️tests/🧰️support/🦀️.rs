//! 🧰️ The one shared app test harness — every test target of the family mounts it as `context`.

use semio_s_artifact_process_process3d_editor::*;
use semio_s_artifact_process_process3d_kit::builtin_installed_catalogs;
use semio_s_artifact_process_process3d_commands::{camera, contribution, cursor, document, engagement, inspector, media, step, stock, sun, workshop, world};
use semio_s_artifact_process_process3d_config::{Process3dConfig, Process3dConfigMutation, PROCESS3D_DEFAULT_UTILITY, Process3dConfigSetCamera, Process3dConfigSetContributions, Process3dConfigSetCursor, Process3dConfigSetEngagementInput, Process3dConfigSetSun};
use semio_s_artifact_process_process3d_modes;
use semio_s_artifact_process_process3d_modes::windows::workpiece;
use semio_s_artifact_process_process3d_panels::{catalogue, document as document_panel, inspection, workshop as workshop_panel};
use semio_s_artifact_process_process3d_presence::{Process3dPresence, Process3dPresenceMutation};
use semio_s_artifact_process_process3d_kit::editor::process3d::terminology::process3d_labels;
use semio_s_artifact_process_process3d_mutations::Process3dMutation;
use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText as _;
use semio_s_artifact_process_process3d_model::{snapshot::Capability, snapshot::CapabilityRule, snapshot::MachineCatalog, snapshot::MachineCatalogs, snapshot::MeasureRecipe, snapshot::Process3dSnapshot, snapshot::ProcessMeasure, snapshot::ProcessStep, snapshot::StepOrigin, snapshot::Stock, snapshot::WorkingSolid, snapshot::WorkshopMachine};
use semio_framework::kernel::Effect;
use semio_framework::{DslValue, InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppActionRegistry;
use semio_framework_plugin::AppDefinition;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactKindSpec;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::CommandDefinition;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::ContextMenuItemSpec;
use semio_framework_plugin::ContextMenuRequest;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::Menu;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::OsMediaCapability;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use pack::snapshot::ArtifactPack;
use semio_framework_2d::compute::EngineHandles;
use camera::set_camera;
use contribution::set_contributions;
use cursor::{set_cursor, step_cursor, step_cursor_back, step_cursor_forward};
use document::{set_active_example, set_snapshot};
use engagement::{engagement_abort, engagement_input, engagement_submit};
use inspector::patch_inspector;
use media::{export_model, import_model_file, load_model_request};
use step::{add_step, move_step, remove_selected_step, remove_step, set_step_enabled, update_step};
use stock::set_stock;
use sun::{set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use workshop::{add_workshop_machine, remove_workshop_machine, update_workshop_machine};
use world::{world_face_drag_end, world_pointer_down};
use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry_and_members};
use semio_framework_plugin::{ActionMeta, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

/// ✏️ `Process3dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<Process3dPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<Process3dPlayApp>` builds it.
pub type Process3dRawApp = VcsArtifactApp<EditorApp<Process3dPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>;

/// 🧪️ A fixture app that retires its stores on the way out. A registry-backed `VcsArtifactApp`
/// asserts on `Drop` that its artifact store reached the terminal-empty shallow shell
/// (`🏪️store/🦀️.rs`'s witness), so a bare drop aborts the whole test binary. The framework's own
/// `context::close_registered_fixture_app` caps the drain at 64 turns of one item each
/// (`🔌️plugin/🦀️.rs:6699-6709`), which this app — 31 tool jobs plus five stores — outgrows, so the
/// pump runs here instead and is bounded only far enough to be a runaway guard.
/// `Deref`/`DerefMut` keep every call site writing `&mut app` exactly as before. The drain never
/// panics while the thread is ALREADY unwinding — a second panic in a destructor aborts the whole
/// binary and hides the assertion that actually failed.
pub struct Process3dApp(Process3dRawApp);

impl std::ops::Deref for Process3dApp {
    type Target = Process3dRawApp;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Process3dApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for Process3dApp {
    fn drop(&mut self) {
        for _ in 0..1_048_576 {
            if self.0.close_terminal_is_empty() {
                return;
            }
            let demand = match PluginApp::close_retirement_demands(&self.0, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
                Ok(demand) => demand,
                Err(error) => {
                    assert!(std::thread::panicking(), "Process3d fixture close demand faulted: {error:?}");
                    return;
                }
            };
            match PluginApp::close_step(&mut self.0, semio_s_artifact_process_process3d_host::process3d_demand_grant(demand)) {
                Ok(semio_framework_plugin::PluginLifecycleStep::Complete(_)) => return,
                Ok(_) => continue,
                Err(fault) => {
                    assert!(std::thread::panicking(), "Process3d fixture close faulted: {fault:?}");
                    return;
                }
            }
        }
        assert!(std::thread::panicking(), "Process3d fixture never reached its terminal-empty witness");
    }
}

/// ✏️ Adapts `create_process3d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `artifact_app_laws::assert_declared_actions_bridge_to_commands`/`new_app_with_registry`
/// still expect — framework test context gap, not modifiable here (`🧰️framework/**` is outside this
/// packet's lease).
pub fn process3d_app_manifest_for_tests() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_process3d_app(), examples: Vec::new() }
}

semio_framework_plugin::history_edit_acceptance_law!("process", Process3dPlayApp, process3d_app_manifest_for_tests, "../..");

/// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.

/// 🧪 Seeds wood/metal contribution catalogs so panel tests can install machines without the host.
fn seed_domain_catalog_contributions(app: &mut Process3dRawApp) {
    use semio_s_artifact_process_process3d_model::{snapshot::Capability, snapshot::CapabilityParameter, snapshot::CapabilityRule, snapshot::MeasureRecipe, snapshot::StockQuantity, snapshot::WorkshopMachine};
    use semio_framework::{ProgramContributionEntry, TopicContribution};
    fn param(id: &str, label: &str, value: f64) -> CapabilityParameter {
        CapabilityParameter { id: id.into(), label: label.into(), value }
    }
    let wood_machines = vec![
        WorkshopMachine {
            id: "circularSaw".into(),
            label: "Circular Saw".into(),
            icon_id: "scissors".into(),
            catalog_id: None,
            capabilities: vec![Capability {
                id: "crosscut".into(),
                label: "Crosscut".into(),
                icon_id: "scissors".into(),
                recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                parameters: vec![param("bladeDiameter", "Blade Diameter", 0.184), param("kerf", "Kerf", 0.002), param("maxCutDepth", "Max Cut Depth", 0.065)],
                rules: vec![CapabilityRule::Max { quantity: StockQuantity::Height, parameter: "maxCutDepth".into(), margin: 0.0 }],
            }],
        },
        WorkshopMachine {
            id: "tableSaw".into(),
            label: "Table Saw".into(),
            icon_id: "scissors".into(),
            catalog_id: None,
            capabilities: vec![Capability {
                id: "rip".into(),
                label: "Rip".into(),
                icon_id: "scissors".into(),
                recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                parameters: vec![param("bladeDiameter", "Blade Diameter", 0.315), param("kerf", "Kerf", 0.0032), param("maxCutDepth", "Max Cut Depth", 0.102), param("fenceWidth", "Fence Width", 0.8)],
                rules: vec![CapabilityRule::Max { quantity: StockQuantity::Height, parameter: "maxCutDepth".into(), margin: 0.0 }, CapabilityRule::Max { quantity: StockQuantity::Width, parameter: "fenceWidth".into(), margin: 0.0 }],
            }],
        },
    ];
    let metal_machines = vec![WorkshopMachine {
        id: "chopSaw".into(),
        label: "Chop Saw".into(),
        icon_id: "scissors".into(),
        catalog_id: None,
        capabilities: vec![Capability {
            id: "crosscut".into(),
            label: "Crosscut".into(),
            icon_id: "scissors".into(),
            recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
            parameters: vec![param("bladeDiameter", "Blade Diameter", 0.35), param("kerf", "Kerf", 0.002), param("maxCutDepth", "Max Cut Depth", 0.12)],
            rules: vec![],
        }],
    }];
    let entries = vec![
        ProgramContributionEntry {
            plugin_id: "process-wood".into(),
            topic_contribution: Some(TopicContribution::new(
                "process.machines",
                DslValue::object([
                    ("appId".to_string(), DslValue::String("process3d-play".to_string())),
                    ("moduleId".to_string(), DslValue::String("wood".to_string())),
                    ("label".to_string(), DslValue::String("Wood".to_string())),
                    ("iconId".to_string(), DslValue::String("beam".to_string())),
                    ("machinesJson".to_string(), DslValue::String(semio_framework_pack_json::to_json_string(&wood_machines))),
                ]),
            )),
        },
        ProgramContributionEntry {
            plugin_id: "process-metal".into(),
            topic_contribution: Some(TopicContribution::new(
                "process.machines",
                DslValue::object([
                    ("appId".to_string(), DslValue::String("process3d-play".to_string())),
                    ("moduleId".to_string(), DslValue::String("metal".to_string())),
                    ("label".to_string(), DslValue::String("Metal".to_string())),
                    ("iconId".to_string(), DslValue::String("wrench".to_string())),
                    ("machinesJson".to_string(), DslValue::String(semio_framework_pack_json::to_json_string(&metal_machines))),
                ]),
            )),
        },
    ];
    let json = semio_framework_pack_json::to_json_string(&entries);
    // 🧵️ B1: an app's own behaviour dispatches only through the typed command channel —
    // `handle_action` serves the framework-reserved verbs and rejects app command ids.
    dispatch(app, Process3dCommand::SetContributions(set_contributions::SetContributions { json }));
}

/// 🧪️ Every test context app is wired to the real manifest registry. The registry-less `new_app` path
/// leaves `AppActionRegistry::{actions, app_commands, mode_commands}` empty, so
/// `migrated_tool_ids()` (`🔌️plugin/🦀️.rs:12058`) returns NOTHING and `tool_job_registration`
/// (`:19140`) rejects every one of this app's proof rows with `interactive-job.catalog-authority` —
/// an app that declares tool proofs at all cannot be constructed without its manifest.
pub fn app() -> Process3dApp {
    app_with_registry()
}

/// 🧪️ A registry-backed app with NOTHING dispatched into it — no seeded contribution catalogs, so
/// its artifact generation is still the genesis one. Publication-authority admission is keyed on
/// that generation, so a test that swaps a production envelope must start from here; seeding first
/// bumps the generation and the decode comes back `Fault` instead of `Ready`.
pub fn unseeded_app_with_registry() -> Process3dApp {
    let mut app = ::semio_framework_async::poll::resolve_ready(new_app_with_registry_and_members::<EditorApp<Process3dPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(process3d_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())));
    ::semio_framework_async::poll::resolve_ready(PluginApp::bind_instance_id(&mut app, meta("local").instance_id));
    Process3dApp(app)
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub fn app_with_registry() -> Process3dApp {
    let mut app = ::semio_framework_async::poll::resolve_ready(new_app_with_registry_and_members::<EditorApp<Process3dPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(process3d_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())));
    ::semio_framework_async::poll::resolve_ready(PluginApp::bind_instance_id(&mut app, meta("local").instance_id));
    seed_domain_catalog_contributions(&mut app);
    Process3dApp(app)
}

pub fn dispatch(app: &mut Process3dRawApp, command: Process3dCommand) -> InvocationResult {
    dispatch_with_utility(app, command, PROCESS3D_DEFAULT_UTILITY)
}

pub fn dispatch_with_utility(app: &mut Process3dRawApp, command: Process3dCommand, active_utility_id: &str) -> InvocationResult {
    settled_dispatch_with_utility(app, command, active_utility_id).0
}

/// 🏁️ The settling dispatch. This app is MOUNTED (`bind_instance_id`), so a migrated command is
/// answered BEFORE its retained operation publishes: without driving the publication home the
/// document never changes, `snapshot()` stays stale, and — because the store still owes a
/// publication — the app can never reach the terminal-empty shallow shell [`Process3dApp::drop`]
/// drains for. The settled receipt's effects are folded into the answer, since either surface may
/// be the one carrying a `LoadDocument`; its `lanes` are the observable proof that a command wrote
/// a store (`InvocationResult.mutations` is empty by construction on this path).
pub fn settled_dispatch(app: &mut Process3dRawApp, command: Process3dCommand) -> (InvocationResult, semio_framework_plugin::artifact_app_laws::TypedOperationFixtureReceipt) {
    settled_dispatch_with_utility(app, command, PROCESS3D_DEFAULT_UTILITY)
}

pub fn settled_dispatch_with_utility(app: &mut Process3dRawApp, command: Process3dCommand, active_utility_id: &str) -> (InvocationResult, semio_framework_plugin::artifact_app_laws::TypedOperationFixtureReceipt) {
    let mut view_state = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    view_state.active_utility_id = Some(active_utility_id.into());
    let action_meta = ActionMeta { view_state: Some(view_state), ..meta("local") };
    let mut result = ::semio_framework_async::poll::resolve_ready(app.dispatch_typed(command, &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity())).expect("dispatch");
    let receipt = settle(app);
    result.requested_effects.extend(receipt.effects.iter().cloned());
    (result, receipt)
}

pub fn action(app: &mut Process3dRawApp, action: &str, args: Option<&DslValue>) -> InvocationResult {
    settled_action(app, action, args).0
}

/// 🏁️ The settling action bridge — same reason as [`settled_dispatch`]: the host verb's retained
/// operation publishes after the call answers.
pub fn settled_action(app: &mut Process3dRawApp, action: &str, args: Option<&DslValue>) -> (InvocationResult, semio_framework_plugin::artifact_app_laws::TypedOperationFixtureReceipt) {
    let admitted = ::semio_framework_async::poll::resolve_ready(app.handle_action(action, args, &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity())).expect("action dispatch");
    // 🛂️ A framework-reserved verb (`interactionSelect`, `interactionHover`, undo/redo, …) is only
    // ADMITTED by `handle_action` — it finishes in a reserved job handed back as an
    // `Effect::SpawnJob`. Without driving that job home the selection never lands in the
    // interaction store and every selection-addressed render still projects the empty state.
    let mut result = ::semio_framework_async::poll::resolve_ready(semio_framework_plugin::app::settle_framework_reserved_admission(app, admitted)).expect("settle the framework-reserved admission");
    let receipt = settle(app);
    result.requested_effects.extend(receipt.effects.iter().cloned());
    (result, receipt)
}

/// 🏁️ Drives every pending typed operation of the bound instance to its publication, the way the
/// plugin host's continuation does.
pub fn settle(app: &mut Process3dRawApp) -> semio_framework_plugin::artifact_app_laws::TypedOperationFixtureReceipt {
    ::semio_framework_async::poll::resolve_ready(semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity())).expect("settle the typed operation")
}

/// 🧾️ Whether a settled publication reached the DOCUMENT lane — the mounted app's observable
/// "this command wrote an in-history mutation".
pub fn published_a_document_mutation(receipt: &semio_framework_plugin::artifact_app_laws::TypedOperationFixtureReceipt) -> bool {
    receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact)
}

/// 🧩️ A `BuiltNode` carries retained page children, so the rendered tree is projected through the
/// fixture transport rather than serialized directly — a bare `serde_json::to_string(&tree.root)`
/// now fails with `BuiltChildren requires retained page transport`.
pub fn render(app: &mut Process3dRawApp, body_key: &str) -> String {
    render_with_view(app, body_key, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))
}

/// 🪟️ Renders one body against a host view state — the seam every tree-window law drives, since
/// `ViewModel.tree_windows` is the ONLY way a caller states which containers are open and which
/// rows the viewport holds.
pub fn render_with_view(app: &mut Process3dRawApp, body_key: &str, view_state: &ViewModel) -> String {
    let tree = ::semio_framework_async::poll::resolve_ready(app.render(body_key, None, view_state)).expect("render");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("render projection")
}

/// 🪟️ The host's own view state for the single main window. `VcsArtifactApp::window_measures`
/// projects ONE entry per declared window INSTANCE (`ViewModel.window_instances`) and keys the
/// answer by that instance id — a default, instance-less view therefore answers an EMPTY map no
/// matter what `ArtifactEditor::window_measures` returns.
pub fn main_window_view() -> ViewModel {
    ViewModel {
        window_id: Some(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into(), window_kind_id: workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into() }],
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    }
}

pub fn main_window_measures(app: &mut Process3dRawApp) -> Vec<WindowMeasure> {
    ::semio_framework_async::poll::resolve_ready(app.window_measures(&main_window_view())).get(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN).cloned().expect("main window measures")
}

/// 🧩️ The four shipped `process-extension-*` packs, exactly as those crates emit them
/// (`✏️s/🔌️plugins/🏭️process/🧩️extensions/*/🦀️.rs` builds this payload from the same `MachineCatalog`
/// this crate compiles in), plus the foreign `cad.computer` / `sourcing.module` entries the
/// DEMONSTRATOR's `consumes` row puts in the same pack. This is the real host closure, not a toy.
pub fn demonstrator_contributions_pack() -> String {
    let mut entries: Vec<semio_framework::ProgramContributionEntry> = Vec::new();
    for catalog in builtin_installed_catalogs().into_iter().filter(|catalog| catalog.catalog_id() != "geometry") {
        entries.push(semio_framework::ProgramContributionEntry {
            plugin_id: format!("process-extension-{}", catalog.catalog_id()),
            topic_contribution: Some(semio_framework::TopicContribution::new(
                "process.machines",
                semio_framework::DslValue::object([
                    ("appId".to_string(), semio_framework::DslValue::String(PROCESS_3D_PLAY_APP_ID.to_string())),
                    ("moduleId".to_string(), semio_framework::DslValue::String(catalog.catalog_id().to_string())),
                    ("label".to_string(), semio_framework::DslValue::String(catalog.label().to_string())),
                    ("iconId".to_string(), semio_framework::DslValue::String(catalog.icon_id().to_string())),
                    ("machinesJson".to_string(), semio_framework::DslValue::String(semio_framework_pack_json::to_json_string(&catalog.machines()))),
                ]),
            )),
        });
    }
    for (plugin, topic, app) in [
        ("cad-extension-aec-building", "cad.computer", "s.cad.cad@1/*#editor"),
        ("cad-extension-spatial-shape", "cad.computer", "s.cad.cad@1/*#editor"),
        ("sourcing-module-beams", "sourcing.module", "sourcing-curation"),
    ] {
        entries.push(semio_framework::ProgramContributionEntry {
            plugin_id: plugin.to_string(),
            topic_contribution: Some(semio_framework::TopicContribution::new(
                topic,
                semio_framework::DslValue::object([
                    ("appId".to_string(), semio_framework::DslValue::String(app.to_string())),
                    ("moduleId".to_string(), semio_framework::DslValue::String(plugin.to_string())),
                ]),
            )),
        });
    }
    semio_framework_pack_json::to_json_string(&entries)
}
