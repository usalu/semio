//! 🔗️ 🔗️ Flow play app commands command — `disconnect`.

use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use crate::editor::flow::host_operations;
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct Disconnect {
    pub synapse_id: String,
}

pub fn handle(payload: &Disconnect, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(Emit::mutations(host_operations(doc.snapshot, cfg.snapshot, session, |host| host.disconnect(&payload.synapse_id).is_ok())))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
