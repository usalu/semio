//! 📈️ 📈️ VCS play app commands command — `increment-counter`.

use crate::artifacts::vcs::{op::VcsDemoMutation, VcsSnapshot};
use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "increment-counter")]
pub struct IncrementCounter {}

pub async fn handle(_payload: &IncrementCounter, doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::artifacts::vcs::mutations::change_counter(doc.snapshot.counter + 1)]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::vcs::testkit::{app, dispatch};
    use crate::editor::vcs::VcsCommand;

    #[semio_framework_async_macros::async_test]
    async fn increment_counter_action_updates_projection() {
        let mut instance = app();
        let before = instance.snapshot().expect("materialize snapshot").counter;
        let result = dispatch(&mut instance, VcsCommand::IncrementCounter(IncrementCounter {}));
        assert_eq!(result.mutations.len(), 1);
        assert_eq!(instance.snapshot().expect("materialize snapshot").counter, before + 1);
    }
}
//#endregion 🧪️Tests
