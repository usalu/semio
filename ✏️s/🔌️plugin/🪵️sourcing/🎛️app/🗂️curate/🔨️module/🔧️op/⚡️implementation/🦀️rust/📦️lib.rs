//! ⚡️ Sourcing curate app — operation enum + laws (constitutional: op).

use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};
use sourcing::CurateDocument;

//#region 🔖️Operations
/// 🛒️ Curate document operation: currently always a wholesale swap — every action recomputes the
/// full document and this carries it, with a true inverse restoring the exact prior document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SourcingOperation {
    SetDocument {
        #[dsl(block)]
        document: CurateDocument,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcingDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<CurateDocument>,
}

impl OperationDiff<CurateDocument> for SourcingDiff {
    fn apply(&self, projection: &CurateDocument) -> CurateDocument {
        self.document.clone().unwrap_or_else(|| projection.clone())
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            self.document = other.document;
        }
    }
}

impl Operation<CurateDocument> for SourcingOperation {
    type Diff = SourcingDiff;

    fn diff(&self, _projection: &CurateDocument) -> Self::Diff {
        match self {
            SourcingOperation::SetDocument { document } => SourcingDiff { document: Some(document.clone()) },
        }
    }

    fn backwards(&self, projection: &CurateDocument) -> Vec<Self> {
        match self {
            SourcingOperation::SetDocument { .. } => vec![SourcingOperation::SetDocument { document: projection.clone() }],
        }
    }
}
//#endregion 🔖️Operations

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `sourcing_engine::SourcingCurateConfig`'s operation enum — mirrors
/// `shooting_op::ShootingConfigOperation`'s shape exactly: one variant per settled interaction (search
/// query, module/typology/availability filters, sort, selection, locale), plus a generic `Snapshot`
/// every variant's `backwards()` returns. Since a config-only dispatch is a plain `Apply` (never an
/// `AmendLast`), each tick is its own distinct, real config edit, and "undo this tick" is exactly
/// "restore the whole-config snapshot from just before it" — no per-field reverse-patch bookkeeping
/// needed. `Operation::Diff` is the WHOLE `SourcingCurateConfig` (not a granular patch type): `diff()`
/// returns "the full config after this op", and `OperationDiff<SourcingCurateConfig>::apply` for
/// `SourcingCurateConfig` itself (`sourcing_engine`) just returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum SourcingCurateConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: sourcing_engine::SourcingCurateConfig,
    },
    #[dsl(key = "filter-query")]
    SetFilterQuery { value: String },
    #[dsl(key = "filter-modules")]
    SetFilterModules { module_ids: Vec<String> },
    #[dsl(key = "filter-typology")]
    SetFilterTypology { path: Vec<String> },
    #[dsl(key = "filter-min-availability")]
    SetFilterMinAvailability { value: u32 },
    #[dsl(key = "sort")]
    SetSort {
        #[dsl(block)]
        sort: Option<sourcing::TableSort>,
    },
    #[dsl(key = "selected-object")]
    SetSelectedObject { object_id: Option<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<sourcing_engine::SourcingCurateConfig> for SourcingCurateConfigOperation {
    type Diff = sourcing_engine::SourcingCurateConfig;

    fn diff(&self, base: &sourcing_engine::SourcingCurateConfig) -> sourcing_engine::SourcingCurateConfig {
        let mut next = base.clone();
        match self {
            SourcingCurateConfigOperation::Snapshot { config } => return config.clone(),
            SourcingCurateConfigOperation::SetFilterQuery { value } => next.filters.query = value.clone(),
            SourcingCurateConfigOperation::SetFilterModules { module_ids } => next.filters.module_ids = module_ids.clone(),
            SourcingCurateConfigOperation::SetFilterTypology { path } => next.filters.typology_path = path.clone(),
            SourcingCurateConfigOperation::SetFilterMinAvailability { value } => next.filters.min_availability = *value,
            SourcingCurateConfigOperation::SetSort { sort } => next.filters.sort = sort.clone(),
            SourcingCurateConfigOperation::SetSelectedObject { object_id } => next.selected_object_id = object_id.clone(),
            SourcingCurateConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &sourcing_engine::SourcingCurateConfig) -> Vec<Self> {
        vec![SourcingCurateConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🌱️ Mirrors `sourcing_engine`'s private test-only helper — a small, self-contained fixture
    /// assembly (not business logic), duplicated here rather than shared. `sourcing_engine` is a real
    /// (non-dev) dependency of this crate now — `SourcingCurateConfigOperation::Snapshot` carries a real
    /// `sourcing_engine::SourcingCurateConfig` in production, same as `shooting_op`/`shooting_engine`.
    fn sample_document() -> CurateDocument {
        CurateDocument { stock: sourcing_engine::sourcing_modules().iter().flat_map(|module| module.demo_kinds()).collect(), ..Default::default() }
    }

    #[test]
    fn set_document_op_text_round_trips() {
        store::test_support::assert_op_text_binary_equivalence(&SourcingOperation::SetDocument { document: sample_document() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingOperation::SetDocument { document: CurateDocument::default() });
    }

    fn sample_config() -> sourcing_engine::SourcingCurateConfig {
        sourcing_engine::SourcingCurateConfig {
            filters: sourcing::Filters {
                query: "glulam".into(),
                module_ids: vec!["beams".into()],
                typology_path: vec!["beams".into(), "steel".into()],
                min_availability: 5,
                sort: Some(sourcing::TableSort { column_id: "availability".into(), direction: sourcing::SortDirection::Desc }),
            },
            selected_object_id: Some("beam-glulam-gl24h".into()),
            locale: "de-DE".into(),
        }
    }

    /// 🎞️ Every variant's `backwards()` must exactly restore the pre-operation config, mirroring
    /// `shooting_op`'s `round_trip` test helper (`vcs::apply_operation` isn't a dependency here, so this
    /// drives `Operation::diff`/`backwards` directly instead).
    fn round_trip(config: &sourcing_engine::SourcingCurateConfig, operation: &SourcingCurateConfigOperation) -> sourcing_engine::SourcingCurateConfig {
        let forward = operation.diff(config);
        let backwards = operation.backwards(config);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = back.diff(&restored);
        }
        assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_operations_round_trip_every_variant() {
        let config = sample_config();
        round_trip(&config, &SourcingCurateConfigOperation::SetFilterQuery { value: "kvh".into() });
        round_trip(&config, &SourcingCurateConfigOperation::SetFilterModules { module_ids: vec!["windows".into(), "slabs".into()] });
        round_trip(&config, &SourcingCurateConfigOperation::SetFilterTypology { path: vec!["slabs".into()] });
        round_trip(&config, &SourcingCurateConfigOperation::SetFilterMinAvailability { value: 12 });
        round_trip(&config, &SourcingCurateConfigOperation::SetSort { sort: None });
        round_trip(&config, &SourcingCurateConfigOperation::SetSelectedObject { object_id: None });
        round_trip(&config, &SourcingCurateConfigOperation::SetLocale { value: "en-US".into() });
        let snapshot = round_trip(&config, &SourcingCurateConfigOperation::Snapshot { config: sourcing_engine::SourcingCurateConfig::default() });
        assert_eq!(snapshot, sourcing_engine::SourcingCurateConfig::default());
    }

    #[test]
    fn config_op_text_round_trips_every_variant() {
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::Snapshot { config: sample_config() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetFilterQuery { value: "kvh".into() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetFilterModules { module_ids: vec!["beams".into(), "slabs".into()] });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetFilterTypology { path: vec!["beams".into(), "steel".into()] });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetFilterMinAvailability { value: 7 });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetSort { sort: Some(sourcing::TableSort { column_id: "name".into(), direction: sourcing::SortDirection::Asc }) });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetSort { sort: None });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetSelectedObject { object_id: Some("beam-glulam-gl24h".into()) });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetSelectedObject { object_id: None });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests
