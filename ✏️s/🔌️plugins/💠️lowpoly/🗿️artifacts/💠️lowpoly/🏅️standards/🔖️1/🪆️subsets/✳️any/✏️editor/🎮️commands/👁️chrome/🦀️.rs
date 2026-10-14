//! 👁️ Lowpoly play app command — the show-edges chrome toggle. Config-only.

use semio_s_artifact_lowpoly_lowpoly_config::{LowpolyConfig, LowpolyConfigMutation, SetShowEdgesEdit};
use semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::session::LowpolyScratch;
use semio_s_artifact_lowpoly_lowpoly_mutations::LowpolyMutation;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;
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
//#endregion 🧪️Tests
