//! 🔄️ Sequence play app commands — auto-layout (`reorganize`) and its flow-direction setting.

use crate::mutations::SequenceMutation;
use crate::SequenceSnapshot;
use crate::editor::sequence::config::{SequenceConfig, SequenceConfigMutation};
use crate::editor::sequence::ops_from_host_mutation;
use infinite_board_port_directed_dag::{DagLayoutOptions, DagLayoutOrientation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Orientation
/// 🌳️ `SequenceConfig::orientation`'s string encoding <-> the DAG kernel's real
/// `DagLayoutOrientation` — see `SequenceConfig::orientation`'s doc comment for why the config field
/// itself stays a string. Single consumer (`reorganize`), so it lives here rather than the artifact
/// engine.
pub fn orientation_from_config(value: &str) -> DagLayoutOrientation {
    match value {
        "topBottom" => DagLayoutOrientation::TopBottom,
        _ => DagLayoutOrientation::LeftRight,
    }
}
//#endregion 🔖️Orientation

//#region 🔖️Reorganize
pub mod reorganize {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "reorganize")]
    pub struct Reorganize {}

    pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, SequenceSnapshot>, cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let orientation = orientation_from_config(&cfg.snapshot.orientation);
        Ok(Emit::mutations(ops_from_host_mutation(doc.snapshot, |host| {
            let opts = DagLayoutOptions { orientation, ..DagLayoutOptions::default() };
            let _ = host.reorganize(&opts);
        })))
    }
}
//#endregion 🔖️Reorganize

//#region 🔖️SetOrientation
pub mod set_orientation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-orientation")]
    pub struct SetOrientation {
        pub value: String,
    }

    pub fn handle(payload: &SetOrientation, _doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        Ok(Emit::config(vec![SequenceConfigMutation::SetOrientation(crate::editor::sequence::config::SetOrientation { value: payload.value.clone() })]))
    }
}
//#endregion 🔖️SetOrientation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
