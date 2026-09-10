//! 🧮️ 🧮️ Generation2d play app commands command — `flow-eval-tick`.

use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::with_host_session;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, ExtensionInvocation, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-tick")]
pub struct FlowEvalTick {}

/// 🔁️ The self-redispatch every hop of the chain arms. Unaddressed, unlike generation3d's: this
/// editor publishes its evaluation on the app-wide `HostOnly` lane rather than into one preview
/// window's retained transient, so there is no window to name.
pub fn rearm(req: u64) -> semio_framework_plugin::Effect {
    semio_framework_plugin::Effect::DispatchAction { req: semio_framework_plugin::RequestId(req), action: "flowEvalTick".into(), args: None, delay_ms: 0 }
}

pub fn handle(_payload: &FlowEvalTick, doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let (more, pending_extension_eval) = with_host_session(fixture, session, |host, session| {
        let more = session.tick(host);
        (more, host.take_pending_extension_eval())
    });
    let effects = if more { vec![rearm(100)] } else { Vec::new() };
    let mut extension_invocations = Vec::new();
    if let Some(pending) = pending_extension_eval {
        let request_json = dsl::json::to_json_string(&dsl::DslValue::object([
            ("operatorId".to_string(), dsl::DslValue::String(pending.operator_id.clone())),
            ("inputJson".to_string(), dsl::DslValue::String(pending.input_json.clone())),
            ("nodeHash".to_string(), dsl::DslValue::uint(pending.node_hash)),
        ]));
        extension_invocations.push(ExtensionInvocation::new(pending.extension_id, "evaluate", request_json, "flowEvalResolve"));
    }
    Ok(Emit { effects, extension_invocations, ..Default::default() })
}
