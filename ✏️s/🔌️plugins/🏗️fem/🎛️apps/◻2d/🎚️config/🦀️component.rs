//! 🧮️ Fem2d play app — view state (`Fem2dConfig`) and its operation enum (`Fem2dConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.fem2d` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so result-display/camera/locale edits are VCS'd exactly
//! like document content.

use crate::artifacts::fem2d::FemCamera;
use protocol::Operation;
use serde::{Deserialize, Serialize};

// #region 🔖️Config
/// 🧮️ B1: fem2d's real `DocumentApp::Config` — the pure-trait pilot's config artifact. Absorbs both
/// former `Fem2dPlayApp` `RefCell` fields (`result_display`, `camera`) plus the locale the deleted
/// `ViewState` used to carry into label resolution — session-only view state now round-trips through
/// the config `DocumentStore` exactly like document content, with a real `backwards` per
/// [`Fem2dConfigOperation`] instead of never being VCS'd at all.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "fem2dcfg")]
#[dsl(layout = "lines")]
pub struct Fem2dConfig {
    /// 👁️ The results window's selected case/combination id — was `fem_shared::ResultDisplay::source_id`.
    pub result_source_id: Option<String>,
    /// 👁️ The results window's display mode (`"static"`/`"modal"`/`"buckling"`) — was
    /// `crate::core::shared::DisplayMode`'s discriminant. Kept as a flat string rather than depending on
    /// `crate::core::shared` from the artifact's `engine` (ui-scoped) — the app's window render
    /// translates to/from `crate::core::shared::DisplayMode` at the render boundary.
    pub result_mode: String,
    /// 👁️ The selected modal/buckling mode index — was `crate::core::shared::DisplayMode::Modal`/
    /// `Buckling`'s payload.
    pub result_mode_index: u32,
    /// 🎥️ The canvas camera (pan/zoom) — was `Fem2dPlayApp::camera`.
    #[dsl(block)]
    pub camera: FemCamera,
    /// 🗣️ BCP-47 locale tag — was read off the deleted `ViewState::locale`.
    pub locale: String,
}

impl Default for Fem2dConfig {
    fn default() -> Self {
        Self { result_source_id: None, result_mode: "static".into(), result_mode_index: 0, camera: FemCamera::default(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(Fem2dConfig);
// #endregion 🔖️Config

// #region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `Fem2dConfig`'s operation enum — one variant per settled interaction (mirrors the
/// pre-B1 `Fem2dPlayApp` `RefCell` field writes), plus a generic `Snapshot` every variant's
/// `backwards()` returns — mirrors `ShootingConfigOperation`'s identical B1 pilot recipe: since a
/// config-only dispatch is a plain `Apply` (not an `AmendLast`), each tick is its own distinct, real
/// config edit, and "undo this tick" is exactly "restore the whole-config snapshot from just before
/// it". `Operation::Diff` is the WHOLE `Fem2dConfig` (not a granular patch type): `diff()` returns "the
/// full config after this op", and `store::impl_whole_record_config!` supplies the
/// `OperationDiff<Fem2dConfig>` that returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Fem2dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Fem2dConfig,
    },
    /// 👁️ Was the `setResultDisplay` view action writing `Fem2dPlayApp::result_display`.
    #[dsl(key = "result-display")]
    SetResultDisplay { source_id: Option<String>, mode: String, mode_index: u32 },
    /// 🎥️ Was the `setCamera` view action writing `Fem2dPlayApp::camera`.
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: FemCamera,
    },
    /// 🗣️ Was read off the deleted `ViewState::locale`.
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<Fem2dConfig> for Fem2dConfigOperation {
    type Diff = Fem2dConfig;

    fn diff(&self, base: &Fem2dConfig) -> Fem2dConfig {
        let mut next = base.clone();
        match self {
            Fem2dConfigOperation::Snapshot { config } => return config.clone(),
            Fem2dConfigOperation::SetResultDisplay { source_id, mode, mode_index } => {
                next.result_source_id = source_id.clone();
                next.result_mode = mode.clone();
                next.result_mode_index = *mode_index;
            }
            Fem2dConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            Fem2dConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &Fem2dConfig) -> Vec<Self> {
        vec![Fem2dConfigOperation::Snapshot { config: base.clone() }]
    }
}
// #endregion 🔖️ConfigOperations

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fem2d_config_default_is_static_display_with_default_camera_and_locale() {
        let config = Fem2dConfig::default();
        assert_eq!(config.result_mode, "static");
        assert!(config.result_source_id.is_none());
        assert_eq!(config.result_mode_index, 0);
        assert_eq!(config.camera, FemCamera::default());
        assert_eq!(config.locale, "en-US");
    }

    /// 🧮️ `Fem2dConfig`'s `OperationDiff` is a whole-record replace, mirroring `ShootingConfig`'s
    /// identical B1 pilot pattern: `apply` ignores `base` entirely.
    #[test]
    fn fem2d_config_operation_diff_is_a_whole_record_replace() {
        let base = Fem2dConfig::default();
        let mut replacement = Fem2dConfig::default();
        replacement.locale = "de-DE".into();
        replacement.camera = FemCamera { x: 1.0, y: 2.0, zoom: 3.0 };
        let applied = protocol::OperationDiff::apply(&replacement, &base);
        assert_eq!(applied, replacement);
        let mut absorbed = base.clone();
        protocol::OperationDiff::absorb(&mut absorbed, replacement.clone());
        assert_eq!(absorbed, replacement);
    }

    #[test]
    fn config_operation_backwards_always_restores_the_pre_operation_snapshot() {
        let base = Fem2dConfig::default();
        let camera = FemCamera { x: 1.0, y: 2.0, zoom: 3.0 };
        let op = Fem2dConfigOperation::SetCamera { camera: camera.clone() };
        let next = op.diff(&base);
        assert_eq!(next.camera, camera);
        let backwards = op.backwards(&base);
        assert_eq!(backwards, vec![Fem2dConfigOperation::Snapshot { config: base.clone() }]);
        assert_eq!(backwards[0].diff(&next), base);
    }

    #[test]
    fn set_result_display_config_operation_round_trips() {
        let base = Fem2dConfig::default();
        let op = Fem2dConfigOperation::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 2 };
        let next = op.diff(&base);
        assert_eq!(next.result_source_id.as_deref(), Some("dead"));
        assert_eq!(next.result_mode, "modal");
        assert_eq!(next.result_mode_index, 2);
    }

    #[test]
    fn set_locale_config_operation_round_trips() {
        let base = Fem2dConfig::default();
        let op = Fem2dConfigOperation::SetLocale { value: "de-DE".into() };
        let next = op.diff(&base);
        assert_eq!(next.locale, "de-DE");
    }

    #[test]
    fn fem2d_config_operation_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&Fem2dConfigOperation::Snapshot { config: Fem2dConfig::default() });
        store::test_support::assert_op_line_round_trip(&Fem2dConfigOperation::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 1 });
        store::test_support::assert_op_line_round_trip(&Fem2dConfigOperation::SetCamera { camera: FemCamera { x: 1.0, y: 2.0, zoom: 1.5 } });
        store::test_support::assert_op_line_round_trip(&Fem2dConfigOperation::SetLocale { value: "de-DE".into() });
    }
}
// #endregion 🧪️Tests
