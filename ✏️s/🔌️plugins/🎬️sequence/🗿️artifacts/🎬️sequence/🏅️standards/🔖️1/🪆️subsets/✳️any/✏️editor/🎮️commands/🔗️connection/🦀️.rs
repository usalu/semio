//! 🔗️ Sequence play app commands — connect/disconnect steps.

use crate::mutations::SequenceMutation;
use crate::SequenceSnapshot;
use crate::editor::sequence::config::{SequenceConfig, SequenceConfigMutation};
use crate::editor::sequence::ops_from_host_mutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ConnectSteps
pub mod connect_steps {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
