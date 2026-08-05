//! 🏃️ Sequence play app commands — run the compiled path and clear the last run result.

use crate::apps::sequence::config::{SequenceConfig, SequenceConfigOperation};
use crate::artifacts::sequence::engine::host_from_fixture;
use crate::artifacts::sequence::op::SequenceOperation;
use crate::artifacts::sequence::SequenceFixture;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Run
// 🧭️ Submodules are named `run_command`/`stop_command` (not `run`/`stop`) to dodge clippy's
// `module_inception` — the owning Rust module (wired as `commands::playback` in `📦️lib.rs`, distinct
// from this directory's `🏃️run` taxonomy name) would otherwise contain a child module of the exact
// same name.
pub mod run_command {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "run")]
    pub struct Run {}

    pub fn handle(_payload: &Run, doc: &DocumentView<'_, SequenceFixture>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceOperation, SequenceConfigOperation>, Fault> {
        let result = host_from_fixture(doc.projection).run();
        let json = serde_json::to_string(&result).unwrap_or_default();
        Ok(Emit::config(vec![SequenceConfigOperation::SetLastRun { json }]))
    }
}
//#endregion 🔖️Run

//#region 🔖️Stop
pub mod stop_command {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "stop")]
    pub struct Stop {}

    pub fn handle(_payload: &Stop, _doc: &DocumentView<'_, SequenceFixture>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceOperation, SequenceConfigOperation>, Fault> {
        Ok(Emit::config(vec![SequenceConfigOperation::SetLastRun { json: String::new() }]))
    }
}
//#endregion 🔖️Stop

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::sequence::testkit::{dispatch, new_app, render};
    use crate::apps::sequence::SequenceCommand;

    use super::run_command::Run;
    use super::stop_command::Stop;

    #[test]
    fn run_stores_result_and_renders_in_script() {
        let mut app = new_app();
        dispatch(&mut app, SequenceCommand::Run(Run {}));
        assert!(render(&mut app, crate::apps::sequence::modes::edit::windows::script::SEQUENCE_PLAY_BODY_SCRIPT).contains("run result"));
    }

    #[test]
    fn stop_command_clears_last_run_result() {
        let mut app = new_app();
        dispatch(&mut app, SequenceCommand::Run(Run {}));
        dispatch(&mut app, SequenceCommand::Stop(Stop {}));
        assert!(!render(&mut app, crate::apps::sequence::modes::edit::windows::script::SEQUENCE_PLAY_BODY_SCRIPT).contains("run result"));
    }
}
//#endregion 🧪️Tests
