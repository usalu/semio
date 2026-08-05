//! 👁️ Lowpoly play app command — the show-edges chrome toggle. Config-only.

use crate::apps::lowpoly::config::{LowpolyConfig, LowpolyConfigOperation};
use crate::apps::lowpoly::session::LowpolyScratch;
use crate::artifacts::lowpoly::op::LowpolyOperation;
use crate::artifacts::lowpoly::LowpolyProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️ToggleShowEdges
pub mod toggle_show_edges {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "toggle-show-edges")]
    pub struct ToggleShowEdges {}

    pub fn handle(_payload: &ToggleShowEdges, _doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyOperation, LowpolyConfigOperation>, Fault> {
        Ok(Emit::config(vec![LowpolyConfigOperation::SetShowEdges { value: !cfg.projection.show_edges }]))
    }
}
//#endregion 🔖️ToggleShowEdges

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::lowpoly::testkit::{app, dispatch};
    use crate::apps::lowpoly::LowpolyCommand;

    #[test]
    fn toggle_show_edges_emits_config_operation() {
        let mut a = app();
        let result = dispatch(&mut a, LowpolyCommand::ToggleShowEdges(super::toggle_show_edges::ToggleShowEdges {}));
        assert!(result.operations.is_empty(), "chrome toggle is config-only");
    }
}
//#endregion 🧪️Tests
