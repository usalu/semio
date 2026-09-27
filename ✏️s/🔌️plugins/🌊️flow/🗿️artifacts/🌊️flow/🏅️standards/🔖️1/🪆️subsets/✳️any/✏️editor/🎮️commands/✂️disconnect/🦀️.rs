//! 🔗️ 🔗️ Flow play app commands command — `disconnect`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::flow::host_scene_edit;
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct Disconnect {
    pub synapse_id: String,
}

pub fn handle(payload: &Disconnect, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let composed = crate::flow_composed_snapshot(doc.snapshot, &doc.children)?;
    let emit = host_scene_edit(&composed, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session, |host| Ok(host.disconnect(&payload.synapse_id).is_ok()))?;
    if emit.child_emits.is_empty() {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("mutation.target-missing"), format!("disconnect found no synapse \"{}\"", payload.synapse_id)));
    }
    Ok(emit)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
