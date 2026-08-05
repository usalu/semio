//! 🧮️ Architect play app — the view state (`Config`) and its operation surface.
//!
//! Everything the pre-B1 `RefCell<ArchitectPlayRuntime>` held (selection, active register, search,
//! cached report/analysis JSON, adjacency filter, graph camera) lives here, written via whole-snapshot
//! `ArchitectConfigOperation::Snapshot`s from the `🎮️commands/*` handlers.

use crate::artifacts::program::engine::report::ProgramReport;
use crate::artifacts::program::engine::search::SearchQuery;
use crate::artifacts::program::registers::AdjacencyKind;
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// @emoji 🧮️ B1: `ArchitectPlayApp`'s `DocumentApp::Config` — the pure replacement for the pre-B1
/// `RefCell<ArchitectPlayRuntime>` app-struct field (mirrors `norm::NormConfig`'s single-shared-shape
/// precedent for a monolithic, non-crate-split app).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "architectcfg")]
#[dsl(layout = "lines")]
pub struct ArchitectConfig {
    pub selected_ids: Vec<String>,
    pub active_register: String,
    pub search_query: String,
    /// 🔎️ `Vec<SearchQuery>` serialized as JSON — `SearchQuery` has no `dsl::DslField` binding of its
    /// own, so (like `positions_json`/`camera_json` on other migrated apps) it round-trips as text.
    pub search_history_json: String,
    /// 📋️ The currently rendered `ProgramReport` (the report window), serialized as JSON.
    pub active_report_json: String,
    /// 🐛️ Generic last-action-result debug dump (search hits / validation diagnostics / analysis
    /// result / report) — the pre-B1 `last_report_json` field, renamed since it no longer overlaps
    /// with `active_report_json` above.
    pub last_result_json: String,
    /// 🧮️ The last computed `AnalysisResult`, serialized as JSON — write-only state today (no render
    /// path reads it back), kept for state fidelity with the pre-B1 runtime.
    pub last_analysis_json: String,
    pub adjacency_kind_filter: Option<AdjacencyKind>,
    pub graph_camera_x: f64,
    pub graph_camera_y: f64,
    pub graph_camera_zoom: f64,
}

impl Default for ArchitectConfig {
    fn default() -> Self {
        Self {
            selected_ids: Vec::new(),
            active_register: String::new(),
            search_query: String::new(),
            search_history_json: String::new(),
            active_report_json: String::new(),
            last_result_json: String::new(),
            last_analysis_json: String::new(),
            adjacency_kind_filter: None,
            graph_camera_x: 0.0,
            graph_camera_y: 0.0,
            graph_camera_zoom: 1.0,
        }
    }
}

impl store::ConfigRecord for ArchitectConfig {}

impl OperationDiff<ArchitectConfig> for ArchitectConfig {
    fn apply(&self, _base: &ArchitectConfig) -> ArchitectConfig {
        self.clone()
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

/// @emoji 🧮️ `ArchitectConfig`'s operation enum — a single whole-snapshot `Snapshot` variant is the
/// generic inverse every `🎮️commands/*` config edit uses (mirrors `norm::NormConfigOperation`
/// and `cad`'s `snapshot_of` helper; architect's config has no single hot-path field worth its own
/// granular operation variant the way `NormConfig::selected_check_index` did).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum ArchitectConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: ArchitectConfig,
    },
}

impl Operation<ArchitectConfig> for ArchitectConfigOperation {
    type Diff = ArchitectConfig;

    fn diff(&self, _base: &ArchitectConfig) -> ArchitectConfig {
        match self {
            ArchitectConfigOperation::Snapshot { config } => config.clone(),
        }
    }

    fn backwards(&self, base: &ArchitectConfig) -> Vec<Self> {
        vec![ArchitectConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️Config

//#region 🔖️Readers
/// 🧮️ Reads `cfg.active_register`, defaulting to `"elements"` for a config that predates
/// `ArchitectPlayApp::initial_config`'s default (or was constructed bare in a test).
pub fn active_register(cfg: &ArchitectConfig) -> &str {
    if cfg.active_register.is_empty() {
        "elements"
    } else {
        cfg.active_register.as_str()
    }
}

pub fn parse_search_history(cfg: &ArchitectConfig) -> Vec<SearchQuery> {
    serde_json::from_str(&cfg.search_history_json).unwrap_or_default()
}

pub fn parse_active_report(cfg: &ArchitectConfig) -> Option<ProgramReport> {
    if cfg.active_report_json.is_empty() {
        return None;
    }
    serde_json::from_str(&cfg.active_report_json).ok()
}

/// 🧮️ The whole-snapshot config edit every command handler emits.
pub fn snapshot(next: ArchitectConfig) -> Vec<ArchitectConfigOperation> {
    vec![ArchitectConfigOperation::Snapshot { config: next }]
}
//#endregion 🔖️Readers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_register_falls_back_to_elements() {
        assert_eq!(active_register(&ArchitectConfig::default()), "elements");
        assert_eq!(active_register(&ArchitectConfig { active_register: "risks".into(), ..ArchitectConfig::default() }), "risks");
    }

    #[test]
    fn a_snapshot_operation_replaces_the_whole_config_and_inverts_to_the_base() {
        let base = ArchitectConfig::default();
        let next = ArchitectConfig { search_query: "hall".into(), ..ArchitectConfig::default() };
        let operation = ArchitectConfigOperation::Snapshot { config: next.clone() };
        assert_eq!(operation.diff(&base), next);
        assert_eq!(operation.backwards(&base), vec![ArchitectConfigOperation::Snapshot { config: base }]);
    }

    #[test]
    fn an_empty_active_report_parses_to_none() {
        assert!(parse_active_report(&ArchitectConfig::default()).is_none());
        assert!(parse_search_history(&ArchitectConfig::default()).is_empty());
    }
}
//#endregion 🧪️Tests
