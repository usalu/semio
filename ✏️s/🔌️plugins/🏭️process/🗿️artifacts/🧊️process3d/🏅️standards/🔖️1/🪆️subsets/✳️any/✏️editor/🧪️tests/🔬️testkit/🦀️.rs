
use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

/// ✏️ `Process3dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<Process3dPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<Process3dPlayApp>` builds it.
pub type Process3dRawApp = VcsArtifactApp<EditorApp<Process3dPlayApp>>;

/// 🧪️ A fixture app that retires its stores on the way out. A registry-backed `VcsArtifactApp`
/// asserts on `Drop` that its artifact store reached the terminal-empty shallow shell
/// (`🏪️store/🦀️.rs`'s witness), so a bare drop aborts the whole test binary. The framework's own
/// `testkit::close_registered_fixture_app` caps the drain at 64 turns of one item each
/// (`🔌️plugin/🦀️.rs:6699-6709`), which this app — 33 tool jobs plus five stores — outgrows, so the
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
            match PluginApp::close_step(&mut self.0, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
                Ok(semio_framework_plugin::PluginCloseStep::Complete) => return,
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
/// examples }` shape `testkit::assert_declared_actions_bridge_to_commands`/`new_app_with_registry`
/// still expect — framework testkit gap, not modifiable here (`🧰️framework/**` is outside this
/// packet's lease).
pub fn process3d_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_process3d_app(), examples: Vec::new() }
}

/// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.

/// 🧪 Seeds wood/metal contribution catalogs so panel tests can install machines without the host.
fn seed_domain_catalog_contributions(app: &mut Process3dRawApp) {
    use crate::{Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, StockQuantity, WorkshopMachine};
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
                    ("machinesJson".to_string(), DslValue::String(semio_framework_os_kernel::json::to_json_string(&wood_machines))),
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
                    ("machinesJson".to_string(), DslValue::String(semio_framework_os_kernel::json::to_json_string(&metal_machines))),
                ]),
            )),
        },
    ];
    let json = dsl::json::to_json_string(&entries);
    action(app, "setContributions", Some(&DslValue::object([("json".to_string(), DslValue::String(json))])));
}

/// 🧪️ Every testkit app is wired to the real manifest registry. The registry-less `new_app` path
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
    let mut app = semio_framework_plugin::resolve_ready(new_app_with_registry::<EditorApp<Process3dPlayApp>>(process3d_app_manifest_for_testkit));
    semio_framework_plugin::resolve_ready(PluginApp::bind_instance_id(&mut app, meta("local").instance_id));
    Process3dApp(app)
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub fn app_with_registry() -> Process3dApp {
    let mut app = semio_framework_plugin::resolve_ready(new_app_with_registry::<EditorApp<Process3dPlayApp>>(process3d_app_manifest_for_testkit));
    semio_framework_plugin::resolve_ready(PluginApp::bind_instance_id(&mut app, meta("local").instance_id));
    seed_domain_catalog_contributions(&mut app);
    Process3dApp(app)
}

pub fn dispatch(app: &mut Process3dRawApp, command: Process3dCommand) -> InvocationResult {
    semio_framework_plugin::resolve_ready(app.dispatch_typed(command, &meta("local"))).expect("dispatch")
}

pub fn action(app: &mut Process3dRawApp, action: &str, args: Option<&DslValue>) -> InvocationResult {
    semio_framework_plugin::resolve_ready(app.handle_action(action, args, &meta("local"))).expect("action dispatch")
}

pub fn render(app: &mut Process3dRawApp, body_key: &str) -> String {
    serde_json::to_string(&semio_framework_plugin::resolve_ready(app.render(body_key, None, &ViewModel::default())).expect("render").root).expect("render json")
}

pub fn main_window_measures(app: &mut Process3dRawApp) -> Vec<WindowMeasure> {
    semio_framework_plugin::resolve_ready(app.window_measures()).get(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN).cloned().expect("main window measures")
}
