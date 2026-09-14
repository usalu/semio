//! 🔁️ The `brep` extension, served IN-PROCESS — the host half of the preview round trip.
//!
//! `flowEvalTick` declares its geometry work through `Emit::extension_invocations`
//! (`✏️editor/🦀️.rs`'s `preview_tessellate_invocations`, `⏱️flow-eval-tick/🦀️.rs`); the SDK mints the
//! request id and queues one `Effect::InvokeExtension`, and the SHELL is what runs the capability
//! and answers with `Event::Completed`. A `--lib` fixture has no shell, so without this the tick
//! chain converges with every preview handle still pending and the preview paints nothing.
//!
//! The two handlers below are the SAME two expressions the packaged extension's guest bundle
//! declares (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs`'s `bundle()`), so this serves the real
//! kernel, not a stub (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use semio_framework_plugin::artifact_app_laws::{settle_extension_invocations, PendingExtensionInvocation, SettledExtensionInvocations};
use semio_framework_plugin::{ActionMeta, Fault, FaultCode, FaultOrigin, PluginApp};

fn bad_request(capability: &str, detail: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::Plugin, FaultCode::new(format!("extension.{capability}.bad-request")), detail.into())
}

/// 🧠️ The packaged operator registry, built once per test binary — `evaluate`'s own dispatch table.
fn module_registry() -> &'static semio_framework_os_flow::neural::Registry {
    static REGISTRY: std::sync::OnceLock<semio_framework_os_flow::neural::Registry> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| {
        let mut registry = semio_framework_os_flow::neural::Registry::new();
        crate::flow_operators::install(&mut registry);
        registry
    })
}

/// 🔌️ Runs one declared capability exactly the way the packaged guest bundle's handler does.
///
/// 🪪️ `extension_id` is matched against the CONTRIBUTING PLUGIN's id, never the flow manifest's own
/// `math`/`brep` id — that is precisely the resolution `dispatchInvokeExtensionEffect`
/// (`🏛️ShellHost/🟦️.tsx`) performs against `handle.pluginId`, and a fixture that answered the flow id
/// instead would hide the exact miss that stalled the served app
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn serve(pending: &PendingExtensionInvocation) -> Result<Vec<u8>, Fault> {
    use crate::flow_operators::{BREP_EXTENSION_PLUGIN_ID, MATH_EXTENSION_PLUGIN_ID};
    match (pending.extension_id.as_str(), pending.capability.as_str()) {
        (BREP_EXTENSION_PLUGIN_ID | MATH_EXTENSION_PLUGIN_ID, "evaluate") => semio_framework_os_flow::evaluate_invoke_json(module_registry(), pending.request_json.as_bytes()).map_err(|error| bad_request("evaluate", error)),
        (BREP_EXTENSION_PLUGIN_ID, "tessellate") => {
            let request = dsl::json::parse(&pending.request_json).map_err(|error| bad_request("tessellate", error.to_string()))?;
            let handle = request.get("handle").and_then(dsl::json::Value::as_str).ok_or_else(|| bad_request("tessellate", "missing field `handle`"))?;
            let tolerance = request.get("tolerance").and_then(dsl::json::Value::as_f64).unwrap_or(0.05);
            let budget = request.get("budget").and_then(dsl::json::Value::as_f64).map_or(24, |value| (value as usize).max(1));
            let wall_micros = request.get("wallMicros").and_then(dsl::json::Value::as_f64).map_or(semio_framework_os_flow::brep_geometry::TESSELLATE_STEP_WALL_MICROS, |value| value.max(0.0) as u64);
            let chunk = request.get("chunk").and_then(dsl::json::Value::as_f64).map_or(0, |value| value.max(0.0) as usize);
            Ok(semio_framework_os_flow::brep_geometry::tessellate_step_envelope_json(handle, tolerance, budget, wall_micros, chunk).into_bytes())
        }
        (extension, capability) => Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.missing"), format!("no loaded plugin is addressed by '{extension}' (capability '{capability}')"))),
    }
}

/// 🔁️ Answers every invocation this app has queued, redispatching each response action back into it
/// under the CALLER'S OWN `ActionMeta` — the shell answers a continuation with its live view
/// attached, and the effects those response actions emit (a re-armed `flowEvalTick` among them) come
/// back to the caller instead of being dropped (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// 🪟️ The view is not optional. `flowEvalResolve` and `flowTessellateResolve` are window-addressed
/// routes — they publish the wave they continue into the addressed preview window's transient — so a
/// fixture that answered them with a viewless meta would be modelling a shell that does not exist and
/// would fault `targeted window transient capture requires an exact ViewModel roster`.
pub async fn settle<P: PluginApp>(app: &mut P, receiver: u32, action_meta: &ActionMeta) -> SettledExtensionInvocations {
    settle_extension_invocations(app, receiver, action_meta, &mut |pending| {
        let outcome = serve(pending);
        eprintln!("[DEBUG] extension runner received extension={} capability={} ok={}", pending.extension_id, pending.capability, outcome.is_ok());
        outcome
    })
    .await
    .expect("in-process brep extension round trip")
}

/// 🏛️ Redispatches one `Effect::DispatchAction` naming an app COMMAND exactly the way
/// `makeEffectDispatchOne` (`🛠️ShellHelpers/🟦️.tsx`) does: through the typed command channel with the
/// shell's own live view attached, never the scoped action channel.
pub async fn dispatch_effect_command<P: PluginApp>(app: &mut P, command_id: &str, args: Option<&dsl::DslValue>, action_meta: &ActionMeta) -> Result<(), Fault> {
    use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};
    let arguments = match args {
        Some(dsl::DslValue::Object(entries)) => entries.iter().cloned().collect(),
        _ => std::collections::BTreeMap::new(),
    };
    let app_id = app.app_id().await.to_string();
    let invocation = CommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::App { plugin_id: String::new(), app_id }, command_id: command_id.to_string() }, arguments };
    app.handle_command(&invocation, None, action_meta).await.map(|_| ())
}

/// 🧾️ What one driven `previewEval` run did: the hops it dispatched (and to which windows), the
/// extension answers served, the release hops a closed run owed, the run actions the host fed back and
/// the state the run settled in.
#[derive(Clone, Debug, Default)]
pub struct PreviewRunReceipt {
    pub hops: usize,
    pub hop_windows: Vec<String>,
    pub answered: usize,
    pub releases: usize,
    pub run_actions: Vec<String>,
    pub state: Option<String>,
}

/// 🔁️ The REAL served loop for EITHER surface, with nothing hand-addressed: `pending_effects` off the
/// host's attached-window roster, every framework-reserved run action fed back through
/// `PluginApp::handle_action`, every hop the run's port hands the host redispatched as the typed command
/// the shell sends, every extension invocation answered by `serve`. Stops once nothing is owed, pending
/// or outstanding and the run rests in a terminal or complete state.
pub async fn drive_preview_run<P: PluginApp>(app: &mut P, shell_view: &semio_framework_plugin::ViewModel, initial: &[semio_framework_plugin::Effect], serve: &mut dyn FnMut(&PendingExtensionInvocation) -> Result<Vec<u8>, Fault>) -> PreviewRunReceipt {
    use semio_framework_plugin::artifact_app_laws::{meta, settle_registered_typed_operation};
    use semio_framework_plugin::Effect;
    let action_meta = ActionMeta { view_state: Some(shell_view.clone()), ..meta("local") };
    let mut receipt = PreviewRunReceipt::default();
    let mut effects: Vec<Effect> = initial.to_vec();
    for _ in 0..100_000 {
        effects.extend(app.pending_effects(Some(shell_view)).await);
        if app.has_pending_typed_operations() {
            effects.extend(settle_registered_typed_operation(app, action_meta.instance_id).await.expect("the run's driver turns settle").effects);
        }
        let settled = settle_extension_invocations(app, action_meta.instance_id, &action_meta, serve).await.expect("in-process extension round trip");
        receipt.answered += settled.answered;
        effects.extend(settled.effects);
        let dispatches: Vec<(String, Option<dsl::DslValue>)> = std::mem::take(&mut effects)
            .into_iter()
            .filter_map(|effect| match effect {
                Effect::DispatchAction { action, args, .. } => Some((action, args)),
                _ => None,
            })
            .collect();
        receipt.state = app.tool_run_presence().map(|presence| presence.state.wire_name().to_string());
        if dispatches.is_empty() && settled.answered == 0 && !app.has_pending_typed_operations() && receipt.state.as_deref().is_none_or(|state| matches!(state, "complete" | "aborted" | "faulted" | "finalized")) {
            return receipt;
        }
        for (action, args) in dispatches {
            if semio_framework_plugin::is_tool_run_action_id(&action) {
                receipt.run_actions.push(action.clone());
                app.handle_action(&action, args.as_ref(), &action_meta).await.unwrap_or_else(|fault| panic!("{action} dispatch: {fault:?}"));
                continue;
            }
            if action == "flowEvalTick" {
                receipt.hops += 1;
                receipt.hop_windows.push(args.as_ref().and_then(|args| args.get("windowId")).and_then(dsl::DslValue::as_str).unwrap_or_default().to_string());
            }
            receipt.releases += usize::from(action == "flowEvalRelease");
            dispatch_effect_command(app, &action, args.as_ref(), &action_meta).await.unwrap_or_else(|fault| panic!("the shell redispatches the run's {action} hop: {fault:?}"));
            let hop = settle_registered_typed_operation(app, action_meta.instance_id).await.expect("retained publication");
            assert!(!hop.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Fault), "the run's {action} hop faulted in the retained job ladder: args={args:?}");
            effects.extend(hop.effects);
        }
    }
    panic!("the previewEval run did not settle within 100 000 host turns; last state {:?}", receipt.state);
}

/// 🚦️ The framework run actions a refresh over `shell_view` owes the `previewEval` run right now.
pub async fn owed_run_actions<P: PluginApp>(app: &mut P, shell_view: &semio_framework_plugin::ViewModel) -> Vec<String> {
    run_actions(&app.pending_effects(Some(shell_view)).await)
}

/// 🚦️ The framework run actions an effect list carries.
pub fn run_actions(effects: &[semio_framework_plugin::Effect]) -> Vec<String> {
    effects
        .iter()
        .filter_map(|effect| match effect {
            semio_framework_plugin::Effect::DispatchAction { action, .. } if semio_framework_plugin::is_tool_run_action_id(action) => Some(action.clone()),
            _ => None,
        })
        .collect()
}
