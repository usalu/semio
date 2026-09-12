//! 🏃️ Sequence play app commands — run the compiled path and clear the last run result.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::sequence::{host_from_fixture, retire_run_result_cold, sequence_fixture_from_children};
use crate::mutations::SequenceMutation;
use crate::SequenceSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Run
// 🧭️ Submodules are named `run_command`/`stop_command` (not `run`/`stop`) to dodge clippy's
// `module_inception` — the owning Rust module (wired as `commands::playback` in `🦀️.rs`, distinct
// from this directory's `🏃️run` taxonomy name) would otherwise contain a child module of the exact
// same name.
pub mod run_command {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "run")]
    pub struct Run {}

    pub fn handle(_payload: &Run, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        let fixture = sequence_fixture_from_children(doc.snapshot, &doc.children)?;
        let host = neural_engine::ColdOwner::new(host_from_fixture(&fixture));
        retire_run_result_cold(host.run());
        Ok(Emit::default())
    }
}
//#endregion 🔖️Run

//#region 🔖️Stop
pub mod stop_command {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "stop")]
    pub struct Stop {}

    pub fn handle(_payload: &Stop, _doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️Stop

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
