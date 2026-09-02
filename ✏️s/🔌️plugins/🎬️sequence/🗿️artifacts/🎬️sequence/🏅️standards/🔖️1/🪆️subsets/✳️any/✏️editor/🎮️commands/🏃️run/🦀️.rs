//! 🏃️ Sequence play app commands — run the compiled path and clear the last run result.

use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceSnapshot;
use crate::editor::sequence::config::{SequenceConfig, SequenceConfigMutation};
use crate::editor::sequence::host_from_snapshot;
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

    pub async fn handle(_payload: &Run, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let result = host_from_snapshot(doc.snapshot).run();
        let json = serde_json::to_string(&result).unwrap_or_default();
        Ok(Emit::config(vec![SequenceConfigMutation::SetLastRun { json }]))
    }
}
//#endregion 🔖️Run

//#region 🔖️Stop
pub mod stop_command {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "stop")]
    pub struct Stop {}

    pub async fn handle(_payload: &Stop, _doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        Ok(Emit::config(vec![SequenceConfigMutation::SetLastRun { json: String::new() }]))
    }
}
//#endregion 🔖️Stop

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::sequence::testkit::{dispatch, new_app, render};
    use crate::editor::sequence::SequenceCommand;

    use super::run_command::Run;
    use super::stop_command::Stop;

    #[semio_framework_async_macros::async_test]
    async fn run_stores_result_and_renders_in_script() {
        let mut app = new_app();
        dispatch(&mut app, SequenceCommand::Run(Run {}));
        assert!(render(&mut app, crate::editor::sequence::modes::edit::windows::script::SEQUENCE_PLAY_BODY_SCRIPT).contains("run result"));
    }

    #[semio_framework_async_macros::async_test]
    async fn stop_command_clears_last_run_result() {
        let mut app = new_app();
        dispatch(&mut app, SequenceCommand::Run(Run {}));
        dispatch(&mut app, SequenceCommand::Stop(Stop {}));
        assert!(!render(&mut app, crate::editor::sequence::modes::edit::windows::script::SEQUENCE_PLAY_BODY_SCRIPT).contains("run result"));
    }
}
//#endregion 🧪️Tests
