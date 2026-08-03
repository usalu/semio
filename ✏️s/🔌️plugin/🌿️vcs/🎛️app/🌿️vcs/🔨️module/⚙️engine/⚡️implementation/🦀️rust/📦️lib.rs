//! ⚙️ VCS app — headless compute (constitutional: engine).

use protocol::OperationDiff;
use serde::{Deserialize, Serialize};
use vcs::{VcsDemoProjection, VCS_DEMO_SCHEMA};

//#region 🔖️DocumentHelpers
pub fn empty_vcs_demo_projection() -> VcsDemoProjection {
    VcsDemoProjection {
        schema: VCS_DEMO_SCHEMA.into(),
        title: "VCS Demo".into(),
        counter: 0,
        notes: String::new(),
        status: "new".into(),
        tags: Vec::new(),
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Config
/// 🧮️ `VcsPlayApp`'s real `DocumentApp::Config` — absorbs the old `VcsPlayApp::selected_checkpoint_ids`
/// `RefCell` field (multi-selected checkpoint ids in the document tree) plus the `locale` field the UI
/// used to read off the deleted `ViewState` (mirrors `shooting_engine::ShootingConfig`'s identical
/// `locale` field/doc — see that struct for the pattern this follows).
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

impl store::ConfigRecord for VcsDemoConfig {}

/// 🧮️ Whole-record diff for `vcs_op::VcsDemoConfigOperation` (lives here, not in `vcs_op`, since
/// `protocol::OperationDiff`/`VcsDemoConfig` are both foreign to that crate — the orphan rule requires
/// at least one local type). Mirrors `shooting_engine::ShootingConfig`'s identical impl.
impl OperationDiff<VcsDemoConfig> for VcsDemoConfig {
    fn apply(&self, _base: &VcsDemoConfig) -> VcsDemoConfig {
        self.clone()
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️Config

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_projection_matches_schema() {
        let projection = empty_vcs_demo_projection();
        assert_eq!(projection.schema, VCS_DEMO_SCHEMA);
        assert_eq!(projection.status, "new");
    }

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
}
//#endregion 🧪️Tests
