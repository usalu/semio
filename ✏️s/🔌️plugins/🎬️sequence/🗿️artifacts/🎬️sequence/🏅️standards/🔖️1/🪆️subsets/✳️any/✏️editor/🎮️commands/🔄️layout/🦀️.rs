//! 🔄️ Sequence play app commands — auto-layout (`reorganize`) and its flow-direction setting.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::sequence::modes::edit::windows::main::config::current;
use crate::editor::sequence::sequence_child_emit_from_host_mutation;
use crate::mutations::SequenceMutation;
use crate::SequenceSnapshot;
use infinite_board_port_directed_dag::{DagLayoutOptions, DagLayoutOrientation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Orientation
/// 🌳️ Converts the main window's persisted orientation into the DAG kernel's layout axis.
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

    pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, SequenceSnapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        let orientation = orientation_from_config(&current(cfg).orientation);
        sequence_child_emit_from_host_mutation(doc, |host| {
            let opts = DagLayoutOptions { orientation, ..DagLayoutOptions::default() };
            let _ = host.reorganize(&opts);
        })
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

    pub fn handle(_payload: &SetOrientation, _doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️SetOrientation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
