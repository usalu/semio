//! 🧰️ Process 3d play app commands — the utility bar switch (select/cut/drill/attach), config-only.

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::{op::Process3dMutation, Process3dSnapshot};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetActiveUtility
pub mod set_active_utility {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-utility")]
    pub struct SetActiveUtility {
        pub utility_id: String,
    }

    pub fn handle(payload: &SetActiveUtility, _doc: &DocumentView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Process3dConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }, Process3dConfigMutation::SetSelectedFaceId { value: None }]))
    }
}
//#endregion 🔖️SetActiveUtility

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::process3d::testkit;

    #[test]
    fn set_active_utility_emits_no_operations() {
        let mut app = testkit::app();
        let result = testkit::dispatch(&mut app, crate::apps::process3d::Process3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "cut".into() }));
        assert!(result.mutations.is_empty(), "utility selection is host-owned config state and must never emit document operations or history");
    }
}
//#endregion 🧪️Tests
