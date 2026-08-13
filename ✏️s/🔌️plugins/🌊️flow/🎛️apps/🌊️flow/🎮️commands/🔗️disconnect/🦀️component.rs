//! 🔗️ 🔗️ Flow play app commands command — `disconnect`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::apps::flow::host_operations;
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
    use crate::apps::flow::testkit::{dispatch, flow_app};
    use crate::apps::flow::FlowCommand;

    #[test]
    fn disconnecting_an_unknown_synapse_is_a_no_operation() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::Disconnect(Disconnect { synapse_id: "nope".into() }));
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
