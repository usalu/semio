//! 👁️ Lowpoly play app command — the show-edges chrome toggle. Config-only.

use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;
use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::LowpolyScratch;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️ToggleShowEdges
pub mod toggle_show_edges {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
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

    #[semio_framework_async_macros::async_test]
    async fn toggle_show_edges_emits_config_operation() {
        let mut a = app().await;
        let result = dispatch(&mut a, LowpolyCommand::ToggleShowEdges(super::toggle_show_edges::ToggleShowEdges {})).await;
        assert!(result.mutations.is_empty(), "chrome toggle is config-only");
    }
}
//#endregion 🧪️Tests
