//! 🔗️ Sequence play app commands — connect/disconnect steps.

use crate::apps::sequence::config::{SequenceConfig, SequenceConfigMutation};
use crate::apps::sequence::ops_from_host_mutation;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️ConnectSteps
pub mod connect_steps {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "connect-steps")]
    pub struct ConnectSteps {
        pub source_node_id: String,
        pub target_node_id: String,
    }

    pub fn handle(payload: &ConnectSteps, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        Ok(Emit::mutations(ops_from_host_mutation(doc.snapshot, |host| {
            let _ = host.connect_steps(&payload.source_node_id, &payload.target_node_id);
        })))
    }
}
//#endregion 🔖️ConnectSteps

//#region 🔖️DisconnectSteps
pub mod disconnect_steps {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "disconnect-steps")]
    pub struct DisconnectSteps {
        pub from_id: String,
        pub to_id: String,
    }

    pub fn handle(payload: &DisconnectSteps, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        Ok(Emit::mutations(ops_from_host_mutation(doc.snapshot, |host| {
            host.disconnect_steps(&payload.from_id, &payload.to_id);
        })))
    }
}
//#endregion 🔖️DisconnectSteps

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::sequence::testkit::{dispatch, new_app};
    use crate::apps::sequence::SequenceCommand;

    use super::connect_steps::ConnectSteps;
    use super::disconnect_steps::DisconnectSteps;

    #[test]
    fn disconnect_then_reconnect_round_trips_the_edge() {
        let mut app = new_app();
        dispatch(&mut app, SequenceCommand::DisconnectSteps(DisconnectSteps { from_id: "step-1".into(), to_id: "step-2".into() }));
        assert!(app.snapshot().expect("projection").edges.is_empty());
        dispatch(&mut app, SequenceCommand::ConnectSteps(ConnectSteps { source_node_id: "step-1".into(), target_node_id: "step-2".into() }));
        assert_eq!(app.snapshot().expect("projection").edges.len(), 1);
    }
}
//#endregion 🧪️Tests
