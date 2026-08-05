//! 🧮️ Wires play app — view state (`WiresConfig`) and its operation enum (`WiresConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.wires` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection/drag/locale edits are VCS'd exactly like
//! document content. Absorbs everything that used to live in the pre-B1 `ReasoningWiresPlayApp`'s
//! ephemeral `WiresPlayRuntime` (selection + in-flight pointer drag of one board node) plus the `locale`
//! the deleted `ViewState` used to carry.

use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ `ReasoningWiresPlayApp::Config` — the pure-trait `DocumentApp::Config` for the wires app.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "wirescfg")]
#[dsl(layout = "lines")]
pub struct WiresConfig {
    /// 👁️ Selected node/edge ids — was `WiresPlayRuntime::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 🖱️ In-flight pointer-drag target node id — was `WiresDragState::node_id`
    /// (`WiresPlayRuntime::drag`); `None` means no drag is in progress.
    pub drag_node_id: Option<String>,
    /// 🖱️ Last observed drag pointer X (screen space) — was `WiresDragState::last_x`.
    pub drag_last_x: f64,
    /// 🖱️ Last observed drag pointer Y (screen space) — was `WiresDragState::last_y`.
    pub drag_last_y: f64,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

impl Default for WiresConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), drag_node_id: None, drag_last_x: 0.0, drag_last_y: 0.0, locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(WiresConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`WiresConfig`]'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `WiresPlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()` returns —
/// mirrors `shooting_op::ShootingConfigOperation`'s identical "undo is the whole-config snapshot from
/// just before this tick" shape: since a config-only dispatch is a plain `Apply` (not an `AmendLast`,
/// except when explicitly coalesced via `Emit::amend`/`Emit::amend_config` — see
/// `crate::apps::wires::commands::pointer`), each tick is its own distinct, real config edit, and the
/// simplest correct inverse needs no per-field reverse-patch bookkeeping. `Operation::Diff` is the WHOLE
/// `WiresConfig` (not a granular patch type): `diff()` returns "the full config after this op", and
/// `store::impl_whole_record_config!` supplies the `OperationDiff<WiresConfig>` that returns that
/// snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum WiresConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: WiresConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "drag")]
    SetDrag { node_id: Option<String>, last_x: f64, last_y: f64 },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<WiresConfig> for WiresConfigOperation {
    type Diff = WiresConfig;

    fn diff(&self, base: &WiresConfig) -> WiresConfig {
        let mut next = base.clone();
        match self {
            WiresConfigOperation::Snapshot { config } => return config.clone(),
            WiresConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            WiresConfigOperation::SetDrag { node_id, last_x, last_y } => {
                next.drag_node_id = node_id.clone();
                next.drag_last_x = *last_x;
                next.drag_last_y = *last_y;
            }
            WiresConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &WiresConfig) -> Vec<Self> {
        vec![WiresConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️ConfigTests
    #[test]
    fn wires_config_default_matches_no_selection_no_drag_and_en_locale() {
        let config = WiresConfig::default();
        assert!(config.selected_ids.is_empty());
        assert!(config.drag_node_id.is_none());
        assert_eq!(config.locale, "en-US");
    }

    /// 🔁️ B1 dsl/pack round-trip law for `WiresConfig` — a non-default fixture exercising every field.
    #[test]
    fn wires_config_dsl_pack_round_trip() {
        let config = WiresConfig { selected_ids: vec!["node-1".into(), "edge-1".into()], drag_node_id: Some("node-1".into()), drag_last_x: 12.5, drag_last_y: -7.25, locale: "de-DE".into() };
        store::test_support::assert_dsl_pack_equivalence(&config);
    }
    //#endregion 🔖️ConfigTests

    //#region 🔖️ConfigOperationTests
    #[test]
    fn config_snapshot_and_selection_op_text_round_trip() {
        store::test_support::assert_op_line_round_trip(&WiresConfigOperation::Snapshot { config: WiresConfig::default() });
        store::test_support::assert_op_line_round_trip(&WiresConfigOperation::SetSelection { ids: vec!["node-1".into(), "edge-1".into()] });
    }

    #[test]
    fn config_drag_op_text_round_trip() {
        store::test_support::assert_op_line_round_trip(&WiresConfigOperation::SetDrag { node_id: Some("node-1".into()), last_x: 12.5, last_y: -7.25 });
        store::test_support::assert_op_line_round_trip(&WiresConfigOperation::SetDrag { node_id: None, last_x: 0.0, last_y: 0.0 });
    }

    #[test]
    fn config_locale_op_text_round_trip() {
        store::test_support::assert_op_line_round_trip(&WiresConfigOperation::SetLocale { value: "de-DE".into() });
    }

    /// ⏪️ `backwards()` always returns a single whole-config `Snapshot` of the pre-op state, regardless
    /// of which field the forward op touched — the same "undo restores the prior snapshot" law
    /// `shooting_op::ShootingConfigOperation` establishes.
    #[test]
    fn config_backwards_always_snapshots_the_base() {
        let base = WiresConfig { selected_ids: vec!["node-1".into()], ..Default::default() };
        let forward = WiresConfigOperation::SetSelection { ids: vec!["node-2".into()] };
        let inverse = forward.backwards(&base);
        assert_eq!(inverse, vec![WiresConfigOperation::Snapshot { config: base.clone() }]);
        assert_eq!(forward.diff(&base), WiresConfig { selected_ids: vec!["node-2".into()], ..base });
    }
    //#endregion 🔖️ConfigOperationTests
}
//#endregion 🧪️Tests
