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

use semio_framework_plugin::testkit::{settle_extension_invocations, PendingExtensionInvocation};
use semio_framework_plugin::{Fault, FaultCode, FaultOrigin, PluginApp};

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
pub fn serve(pending: &PendingExtensionInvocation) -> Result<Vec<u8>, Fault> {
    match (pending.extension_id.as_str(), pending.capability.as_str()) {
        ("brep" | "math", "evaluate") => semio_framework_os_flow::evaluate_invoke_json(module_registry(), pending.request_json.as_bytes()).map_err(|error| bad_request("evaluate", error)),
        ("brep", "tessellate") => {
            let request = dsl::json::parse(&pending.request_json).map_err(|error| bad_request("tessellate", error.to_string()))?;
            let handle = request.get("handle").and_then(dsl::json::Value::as_str).ok_or_else(|| bad_request("tessellate", "missing field `handle`"))?;
            let tolerance = request.get("tolerance").and_then(dsl::json::Value::as_f64).unwrap_or(0.05);
            let budget = request.get("budget").and_then(dsl::json::Value::as_f64).map_or(24, |value| (value as usize).max(1));
            let chunk = request.get("chunk").and_then(dsl::json::Value::as_f64).map_or(0, |value| value.max(0.0) as usize);
            Ok(semio_framework_os_flow::brep_geometry::tessellate_step_envelope_json(handle, tolerance, budget, chunk).into_bytes())
        }
        (extension, capability) => Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.unsupported"), format!("no in-process handler for {extension}/{capability}"))),
    }
}

/// 🔁️ Answers every invocation this app has queued, redispatching each response action back into it.
pub async fn settle<P: PluginApp>(app: &mut P, receiver: u32) -> usize {
    settle_extension_invocations(app, receiver, &mut |pending| serve(pending)).await.expect("in-process brep extension round trip")
}
