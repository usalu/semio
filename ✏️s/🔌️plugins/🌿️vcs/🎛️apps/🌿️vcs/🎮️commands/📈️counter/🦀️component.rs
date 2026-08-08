//! 📈️ VCS play app commands — the counter increment.

use crate::apps::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::artifacts::vcs::{op::VcsDemoMutation, VcsDemoProjection};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️IncrementCounter
pub mod increment_counter {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "increment-counter")]
    pub struct IncrementCounter {}

    pub fn handle(_payload: &IncrementCounter, doc: &DocumentView<'_, VcsDemoProjection>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![VcsDemoMutation::SetCounter { counter: doc.projection.counter + 1 }]))
    }
}
//#endregion 🔖️IncrementCounter

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::vcs::testkit::{app, dispatch};
    use crate::apps::vcs::VcsCommand;

    #[test]
    fn increment_counter_action_updates_projection() {
        let mut instance = app();
        let before = instance.projection().expect("materialize projection").counter;
        let result = dispatch(&mut instance, VcsCommand::IncrementCounter(increment_counter::IncrementCounter {}));
        assert_eq!(result.document_mutations.len(), 1);
        assert_eq!(instance.projection().expect("materialize projection").counter, before + 1);
    }
}
//#endregion 🧪️Tests
