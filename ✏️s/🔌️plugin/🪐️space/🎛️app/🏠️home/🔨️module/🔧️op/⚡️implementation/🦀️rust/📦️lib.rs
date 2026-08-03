//! ⚡️ S Home launcher app — operation enum + laws (constitutional: op).

use home::SHomeDocument;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
/// @emoji 🔢️ The Home launcher's only document operation: pins the catalog-generation counter that forces a
/// re-materialize of the studio list after a create/import/delete side-effect on the catalog port.
/// It is its own {@link protocol::OperationDiff} (idempotent set), so forward/backward are symmetric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum SHomeOperation {
    /// 🫙️ The identity operation — an `OperationDiff` needs `Default`; never emitted by `handle_action`.
    #[default]
    NoOperation,
    SetCatalogGeneration { value: u64 },
}

impl protocol::OperationDiff<SHomeDocument> for SHomeOperation {
    fn apply(&self, projection: &SHomeDocument) -> SHomeDocument {
        match self {
            SHomeOperation::NoOperation => projection.clone(),
            SHomeOperation::SetCatalogGeneration { value } => {
                SHomeDocument { catalog_generation: *value, ..projection.clone() }
            }
        }
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, SHomeOperation::NoOperation) {
            *self = other;
        }
    }
}

impl protocol::Operation<SHomeDocument> for SHomeOperation {
    type Diff = SHomeOperation;

    fn diff(&self, _projection: &SHomeDocument) -> SHomeOperation {
        self.clone()
    }

    fn backwards(&self, projection: &SHomeDocument) -> Vec<Self> {
        vec![SHomeOperation::SetCatalogGeneration { value: projection.catalog_generation }]
    }
}
//#endregion 🔖️Types

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `home_engine::HomeConfig`'s operation enum — mirrors `space_op::SpaceConfigOperation`'s
/// whole-record-diff design (see its doc comment for the full rationale).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum HomeConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: home_engine::HomeConfig,
    },
    #[dsl(key = "active-panel-tab")]
    SetActivePanelTab { tab_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl protocol::Operation<home_engine::HomeConfig> for HomeConfigOperation {
    type Diff = home_engine::HomeConfig;

    fn diff(&self, base: &home_engine::HomeConfig) -> home_engine::HomeConfig {
        let mut next = base.clone();
        match self {
            HomeConfigOperation::Snapshot { config } => return config.clone(),
            HomeConfigOperation::SetActivePanelTab { tab_id } => next.active_panel_tab = tab_id.clone(),
            HomeConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &home_engine::HomeConfig) -> Vec<Self> {
        vec![HomeConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Operation;

    #[test]
    fn home_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&SHomeOperation::NoOperation);
        store::test_support::assert_op_line_round_trip(&SHomeOperation::SetCatalogGeneration { value: 7 });
    }

    #[test]
    fn home_config_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&HomeConfigOperation::Snapshot { config: home_engine::HomeConfig::default() });
        store::test_support::assert_op_line_round_trip(&HomeConfigOperation::SetActivePanelTab { tab_id: "tab-1".into() });
        store::test_support::assert_op_line_round_trip(&HomeConfigOperation::SetLocale { value: "de".into() });
    }

    #[test]
    fn home_config_operation_round_trips_via_apply_and_backwards() {
        let config = home_engine::HomeConfig::default();
        let operation = HomeConfigOperation::SetLocale { value: "de".into() };
        let next = operation.diff(&config);
        assert_eq!(next.locale, "de");
        let backwards = operation.backwards(&config);
        let restored = backwards[0].diff(&next);
        assert_eq!(restored, config);
    }
}
//#endregion 🧪️Tests
