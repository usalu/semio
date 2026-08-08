//! 🔗️ Flow play app commands — synapse (edge) wiring: connect / disconnect.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::artifacts::flow::engine::host_operations;
use crate::artifacts::flow::{op::FlowMutation, FlowFixture};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Disconnect
pub mod disconnect {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "disconnect")]
    pub struct Disconnect {
        pub synapse_id: String,
    }

    pub fn handle(payload: &Disconnect, doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        Ok(Emit::mutations(host_operations(doc.projection, cfg.projection, session, |host| host.disconnect(&payload.synapse_id).is_ok())))
    }
}
//#endregion 🔖️Disconnect

//#region 🔖️ConnectMediaPorts
pub mod connect_media_ports {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "connect-media-ports")]
    pub struct ConnectMediaPorts {
        pub source_node_id: String,
        pub source_port_id: String,
        pub target_node_id: String,
        pub target_port_id: String,
    }

    pub fn handle(payload: &ConnectMediaPorts, doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        Ok(Emit::mutations(host_operations(doc.projection, cfg.projection, session, |host| {
            host.connect_ports(&payload.source_node_id, &payload.source_port_id, &payload.target_node_id, &payload.target_port_id).is_ok()
        })))
    }
}
//#endregion 🔖️ConnectMediaPorts

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{dispatch, flow_app};
    use crate::apps::flow::FlowCommand;

    #[test]
    fn disconnecting_an_unknown_synapse_is_a_no_operation() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::Disconnect(disconnect::Disconnect { synapse_id: "nope".into() }));
        assert!(result.document_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
