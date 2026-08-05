//! 🧮️ FEM 3D app — view-state config + its operation enum (constitutional: engine's `Config` region +
//! op's `ConfigOperations` region, both moved here since config is app-level view state, not document
//! content).

use crate::artifacts::fem3d::FemCamera;
use protocol::Operation;
use serde::{Deserialize, Serialize};

// #region 🔖️Config
/// 🧮️ B1: fem3d's real `DocumentApp::Config` — absorbs both former `Fem3dPlayApp` `RefCell` fields
/// (`result_display`, `camera`); session-only view state now round-trips through the config
/// `DocumentStore` exactly like document content, with a real `backwards` per `Fem3dConfigOperation`
/// instead of never being VCS'd at all. Mirrors `Fem2dConfig`'s identical B1 recipe, minus a `locale`
/// field (fem3d never carried a `ViewState::locale` the way fem2d did).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "fem3dcfg")]
#[dsl(layout = "lines")]
pub struct Fem3dConfig {
    /// 👁️ The results window's selected case/combination id — was `fem_shared::ResultDisplay::source_id`.
    pub result_source_id: Option<String>,
    /// 👁️ The results window's display mode (`"static"`/`"modal"`/`"buckling"`) — was
    /// `fem_shared::DisplayMode`'s discriminant. Kept as a flat string rather than depending on
    /// `crate::core::shared::DisplayMode` from `Fem3dConfig` itself — the app translates to/from
    /// `crate::core::shared::DisplayMode` at the render boundary (see
    /// `crate::apps::fem3d::modes::edit::windows::results::config_result_display`).
    pub result_mode: String,
    /// 👁️ The selected modal/buckling mode index — was `fem_shared::DisplayMode::Modal`/`Buckling`'s payload.
    pub result_mode_index: u32,
    /// 🎥️ The world-3d camera (opaque host JSON) — was `Fem3dPlayApp::camera`.
    #[dsl(block)]
    pub camera: FemCamera,
}

impl Default for Fem3dConfig {
    fn default() -> Self {
        Self { result_source_id: None, result_mode: "static".into(), result_mode_index: 0, camera: FemCamera::default() }
    }
}

store::impl_whole_record_config!(Fem3dConfig);
// #endregion 🔖️Config

// #region 🔖️ConfigOperations
/// 🧮️ B1: `Fem3dConfig`'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `Fem3dPlayApp` `RefCell` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns — mirrors `Fem2dConfigOperation`'s identical B1 pilot recipe: since a config-only dispatch is
/// a plain `Apply` (not an `AmendLast`), each tick is its own distinct, real config edit, and "undo this
/// tick" is exactly "restore the whole-config snapshot from just before it". `Operation::Diff` is the
/// WHOLE `Fem3dConfig` (not a granular patch type): `diff()` returns "the full config after this op", and
/// `OperationDiff<Fem3dConfig>::apply` for `Fem3dConfig` itself (`store::impl_whole_record_config!`) just
/// returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Fem3dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Fem3dConfig,
    },
    /// 👁️ Was the `setResultDisplay` view action writing `Fem3dPlayApp::result_display`.
    #[dsl(key = "result-display")]
    SetResultDisplay { source_id: Option<String>, mode: String, mode_index: u32 },
    /// 🎥️ Was the `setCamera` view action writing `Fem3dPlayApp::camera`.
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: FemCamera,
    },
}

impl Operation<Fem3dConfig> for Fem3dConfigOperation {
    type Diff = Fem3dConfig;

    fn diff(&self, base: &Fem3dConfig) -> Fem3dConfig {
        let mut next = base.clone();
        match self {
            Fem3dConfigOperation::Snapshot { config } => return config.clone(),
            Fem3dConfigOperation::SetResultDisplay { source_id, mode, mode_index } => {
                next.result_source_id = source_id.clone();
                next.result_mode = mode.clone();
                next.result_mode_index = *mode_index;
            }
            Fem3dConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
        }
        next
    }

    fn backwards(&self, base: &Fem3dConfig) -> Vec<Self> {
        vec![Fem3dConfigOperation::Snapshot { config: base.clone() }]
    }
}
// #endregion 🔖️ConfigOperations

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fem3d_config_default_is_static_display_with_default_camera() {
        let config = Fem3dConfig::default();
        assert_eq!(config.result_mode, "static");
        assert!(config.result_source_id.is_none());
        assert_eq!(config.result_mode_index, 0);
        assert_eq!(config.camera, FemCamera::default());
    }

    /// 🧮️ `Fem3dConfig`'s `OperationDiff` is a whole-record replace, mirroring `Fem2dConfig`'s identical
    /// B1 pilot pattern: `apply` ignores `base` entirely.
    #[test]
    fn fem3d_config_operation_diff_is_a_whole_record_replace() {
        let base = Fem3dConfig::default();
        let replacement = Fem3dConfig { result_source_id: Some("dead".into()), result_mode: "modal".into(), result_mode_index: 2, camera: FemCamera { json: "{\"x\":1}".into() } };
        let applied = protocol::OperationDiff::apply(&replacement, &base);
        assert_eq!(applied, replacement);
        let mut absorbed = base.clone();
        protocol::OperationDiff::absorb(&mut absorbed, replacement.clone());
        assert_eq!(absorbed, replacement);
    }

    #[test]
    fn config_operation_backwards_always_restores_the_pre_operation_snapshot() {
        let base = Fem3dConfig::default();
        let camera = FemCamera { json: "{\"x\":1}".into() };
        let op = Fem3dConfigOperation::SetCamera { camera: camera.clone() };
        let next = op.diff(&base);
        assert_eq!(next.camera, camera);
        let backwards = op.backwards(&base);
        assert_eq!(backwards, vec![Fem3dConfigOperation::Snapshot { config: base.clone() }]);
        assert_eq!(backwards[0].diff(&next), base);
    }

    #[test]
    fn set_result_display_config_operation_round_trips() {
        let base = Fem3dConfig::default();
        let op = Fem3dConfigOperation::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 2 };
        let next = op.diff(&base);
        assert_eq!(next.result_source_id.as_deref(), Some("dead"));
        assert_eq!(next.result_mode, "modal");
        assert_eq!(next.result_mode_index, 2);
    }

    #[test]
    fn fem3d_config_operation_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&Fem3dConfigOperation::Snapshot { config: Fem3dConfig::default() });
        store::test_support::assert_op_line_round_trip(&Fem3dConfigOperation::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 1 });
        store::test_support::assert_op_line_round_trip(&Fem3dConfigOperation::SetCamera { camera: FemCamera { json: "{\"x\":1}".into() } });
    }
}
// #endregion 🧪️Tests
