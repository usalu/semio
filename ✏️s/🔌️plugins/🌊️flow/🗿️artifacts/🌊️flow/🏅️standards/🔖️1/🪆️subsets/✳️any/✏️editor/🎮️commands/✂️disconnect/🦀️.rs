//! 🔗️ 🔗️ Flow play app commands command — `disconnect`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use crate::editor::flow::host_operations;
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
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{dispatch, flow_app};
    use crate::editor::flow::FlowCommand;

    #[semio_framework_async_macros::async_test]
    async fn disconnecting_an_unknown_synapse_is_a_no_operation() {
        let mut app = flow_app().await;
        let result = dispatch(&mut app, FlowCommand::Disconnect(Disconnect { synapse_id: "nope".into() })).await;
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
