//! ⚙️ S Home launcher app — headless compute (constitutional: engine).
//!
//! 🕳️ `SHomeDocument` is a two-field counter document (`schema` + `catalog_generation`) with no tree
//! structure, id generation, or media import/export of its own — the original monolith never factored
//! out a pure `empty_home_document()`/compute helper (every call site builds the literal
//! `SHomeDocument { schema: "s.home".into(), catalog_generation: N }` directly), so that part of this
//! layer stays deliberately empty. What this layer NOW owns is `HomeConfig` — the Home launcher's real
//! `DocumentApp::Config` (B1): the one `view_state.locale` read `home_ui`'s labels actually need (no
//! `DocumentApp::render`/`app_labels` call carries a `ViewState` anymore, see `HomeApp` in `home_ui`),
//! plus the `active_panel_tab` action already declared in the manifest but never wired to anything real
//! pre-B1 (`"setActivePanelTab"` fell through `handle_action`'s default arm) — genuinely finished now
//! that a config write is a one-liner.

use serde::{Deserialize, Serialize};

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "homecfg")]
#[dsl(layout = "lines")]
pub struct HomeConfig {
    /// 👁️ Active launcher panel tab — was declared (`"setActivePanelTab"`) but never actually wired to
    /// anything pre-B1.
    pub active_panel_tab: String,
    /// 🗣️ BCP-47 locale tag — was read off the deleted `ViewState.locale`.
    pub locale: String,
}

impl Default for HomeConfig {
    fn default() -> Self {
        Self { active_panel_tab: String::new(), locale: "en-US".into() }
    }
}

impl store::ConfigRecord for HomeConfig {}

/// @emoji 🧮️ Whole-record diff for `home_op::HomeConfigOperation` — mirrors `space_engine::SpaceConfig`'s
/// identical pattern (see its doc comment for the full rationale).
impl protocol::OperationDiff<HomeConfig> for HomeConfig {
    fn apply(&self, _base: &HomeConfig) -> HomeConfig {
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
    fn home_config_default_locale_is_english() {
        let config = HomeConfig::default();
        assert_eq!(config.locale, "en-US");
        assert!(config.active_panel_tab.is_empty());
    }
}
//#endregion 🧪️Tests
