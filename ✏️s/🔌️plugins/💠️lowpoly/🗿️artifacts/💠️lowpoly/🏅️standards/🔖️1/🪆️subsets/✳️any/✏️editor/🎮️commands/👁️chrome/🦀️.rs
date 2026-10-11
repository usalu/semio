//! 👁️ Lowpoly play app command — the show-edges chrome toggle. Config-only.

use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation, SetShowEdgesEdit};
use crate::editor::lowpoly::session::LowpolyScratch;
use crate::standards::v1::subsets::any::schema::mutations::LowpolyMutation;
use crate::LowpolySnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️ToggleShowEdges
pub mod toggle_show_edges {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "toggle-show-edges")]
    pub struct ToggleShowEdges {}

    pub fn handle(_payload: &ToggleShowEdges, _doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![LowpolyConfigMutation::SetShowEdges(SetShowEdgesEdit { value: !cfg.snapshot.show_edges })]))
    }
}
//#endregion 🔖️ToggleShowEdges

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
