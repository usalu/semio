//! 🧮️ VCS play app — view state (`VcsDemoConfig`) and its operation enum (`VcsDemoConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.vcsdemo` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection/locale edits are VCS'd exactly like document
//! content. Absorbs the old `VcsPlayApp::selected_checkpoint_ids` `RefCell` field (multi-selected
//! checkpoint ids in the document tree) plus the `locale` field the UI used to read off the deleted
//! `ViewModel` (mirrors `shooting_engine::ShootingConfig`'s identical `locale` field/doc).

use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "vcscfg")]
#[dsl(layout = "lines")]
pub struct VcsDemoConfig {
    /// 👁️ Multi-selected checkpoint ids in the document tree — was `VcsPlayApp::selected_checkpoint_ids`.
    pub selected_checkpoint_ids: Vec<String>,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

impl Default for VcsDemoConfig {
    fn default() -> Self {
        Self { selected_checkpoint_ids: Vec::new(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(VcsDemoConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`VcsDemoConfig`]'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `VcsPlayApp` field writes/deleted `ViewModel.locale`), plus a generic `Snapshot` every variant's
/// `backwards()` returns (see `shooting_op::ShootingConfigOperation`'s identical doc for why this
/// whole-config-snapshot-undo shape is correct and sufficient here).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum VcsDemoConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: VcsDemoConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { checkpoint_ids: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<VcsDemoConfig> for VcsDemoConfigOperation {
    type Diff = VcsDemoConfig;

    fn diff(&self, base: &VcsDemoConfig) -> VcsDemoConfig {
        let mut next = base.clone();
        match self {
            VcsDemoConfigOperation::Snapshot { config } => return config.clone(),
            VcsDemoConfigOperation::SetSelection { checkpoint_ids } => next.selected_checkpoint_ids = checkpoint_ids.clone(),
            VcsDemoConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &VcsDemoConfig) -> Vec<Self> {
        vec![VcsDemoConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vcs_demo_config_default_is_empty_selection_and_english_locale() {
        let config = VcsDemoConfig::default();
        assert!(config.selected_checkpoint_ids.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    /// 🧮️ Round-trip law (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE): a
    /// non-default fixture must survive `DocumentDsl`/`DocumentPack` byte-for-byte.
    #[test]
    fn vcs_demo_config_dsl_pack_round_trips() {
        let config = VcsDemoConfig { selected_checkpoint_ids: vec!["checkpoint-1".into(), "checkpoint-2".into()], locale: "de-DE".into() };
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    /// 🧮️ Round-trip law per `VcsDemoConfigOperation` variant (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-
    /// SCHEMA-FLOW-CONFIG-ON-NODE).
    #[test]
    fn vcs_demo_config_operation_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&VcsDemoConfigOperation::Snapshot { config: VcsDemoConfig { selected_checkpoint_ids: vec!["checkpoint-1".into()], locale: "de-DE".into() } });
        store::test_support::assert_op_line_round_trip(&VcsDemoConfigOperation::SetSelection { checkpoint_ids: vec!["checkpoint-1".into(), "checkpoint-2".into()] });
        store::test_support::assert_op_line_round_trip(&VcsDemoConfigOperation::SetLocale { value: "de-DE".into() });
    }

    /// ⏪️ `backwards()` always returns a `Snapshot` of the pre-operation config, so applying it after
    /// the forward op exactly restores the original — the "whole-config-snapshot-undo" law.
    #[test]
    fn vcs_demo_config_operation_backwards_restores_the_base_config() {
        let base = VcsDemoConfig { selected_checkpoint_ids: vec!["checkpoint-1".into()], locale: "en-US".into() };
        let operation = VcsDemoConfigOperation::SetLocale { value: "de-DE".into() };
        let forward = operation.diff(&base);
        assert_eq!(forward.locale, "de-DE");
        let backwards = operation.backwards(&base);
        assert_eq!(backwards, vec![VcsDemoConfigOperation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
