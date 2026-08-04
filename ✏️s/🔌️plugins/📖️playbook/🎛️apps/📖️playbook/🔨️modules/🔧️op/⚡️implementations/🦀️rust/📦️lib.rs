//! ⚡️ Playbook-play app — operation enum + constructors (constitutional: op).
//!
//! `PlaybookOperation`/`PlaybookDiff` and their `protocol::Operation`/`OperationDiff` impls (plus the
//! private `apply_playbook_edit_operation` match) are owned by the kernel crate
//! `s/kernel/playbook/rs` — this crate re-exposes the operation type and its constructor helpers
//! under the app's own constitutional `op` slot so `protocol`/`ui` depend on `op` per the standard
//! layout instead of reaching into the kernel directly.

pub use playbook::{add_block_operation, add_step_operation, move_block_operation, move_step_operation, remove_block_operation, remove_step_operation, update_playbook_title_operation, PlaybookOperation};

//#region 🔖️ConfigOperations
use playbook_engine::PlaybookConfig;
use protocol::Operation;

/// @emoji 🧮️ B1: `playbook_engine::PlaybookConfig`'s operation enum — one variant per settled
/// interaction (mirrors the pre-B1 `PlaybookPlayApp::selected_ids` field write), plus a generic
/// `Snapshot` every variant's `backwards()` returns — mirrors `writer_op::WriterConfigOperation`/
/// `shooting_op::ShootingConfigOperation` exactly (see either's doc comment for the whole-config-
/// snapshot inverse rationale). Lives here, not in the kernel `playbook` crate, since `PlaybookConfig`
/// is this app's own config artifact, not shared domain state.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslOps)]
pub enum PlaybookConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: PlaybookConfig,
    },
    #[dsl(key = "selected-ids")]
    SetSelectedIds { ids: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<PlaybookConfig> for PlaybookConfigOperation {
    type Diff = PlaybookConfig;

    fn diff(&self, base: &PlaybookConfig) -> PlaybookConfig {
        let mut next = base.clone();
        match self {
            PlaybookConfigOperation::Snapshot { config } => return config.clone(),
            PlaybookConfigOperation::SetSelectedIds { ids } => next.selected_ids = ids.clone(),
            PlaybookConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &PlaybookConfig) -> Vec<Self> {
        vec![PlaybookConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playbook_config_operation_backwards_restores_pre_state() {
        let pre = PlaybookConfig::default();
        store::test_support::assert_operation_round_trip(&pre, PlaybookConfigOperation::SetLocale { value: "de-DE".into() });
        store::test_support::assert_operation_round_trip(&pre, PlaybookConfigOperation::SetSelectedIds { ids: vec!["block-1".into()] });
    }

    #[test]
    fn playbook_config_operation_binary_matches_text() {
        store::test_support::assert_op_text_binary_equivalence(&PlaybookConfigOperation::SetLocale { value: "de-DE".into() });
        store::test_support::assert_op_text_binary_equivalence(&PlaybookConfigOperation::Snapshot { config: PlaybookConfig::default() });
    }
}
//#endregion 🧪️Tests
