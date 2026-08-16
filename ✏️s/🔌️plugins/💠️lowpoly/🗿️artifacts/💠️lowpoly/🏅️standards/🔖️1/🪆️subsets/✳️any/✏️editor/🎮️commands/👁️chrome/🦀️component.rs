//! 👁️ Lowpoly play app command — the show-edges chrome toggle. Config-only.

use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::LowpolyScratch;
use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️ToggleShowEdges
pub mod toggle_show_edges {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "toggle-show-edges")]
    pub struct ToggleShowEdges {}

    pub fn handle(_payload: &ToggleShowEdges, _doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![LowpolyConfigMutation::SetShowEdges { value: !cfg.snapshot.show_edges }]))
    }
}
//#endregion 🔖️ToggleShowEdges

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::lowpoly::testkit::{app, dispatch};
    use crate::editor::lowpoly::LowpolyCommand;

    #[test]
    fn toggle_show_edges_emits_config_operation() {
        let mut a = app();
        let result = dispatch(&mut a, LowpolyCommand::ToggleShowEdges(super::toggle_show_edges::ToggleShowEdges {}));
        assert!(result.mutations.is_empty(), "chrome toggle is config-only");
    }
}
//#endregion 🧪️Tests
