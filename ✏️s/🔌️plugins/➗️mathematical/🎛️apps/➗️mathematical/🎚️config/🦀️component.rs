//! 🧮️ Mathematical play app — view state (`MathConfig`) and its operation enum
//! (`MathConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.mathematical` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so camera/locale edits are VCS'd exactly like document
//! content — absorbs the former app-struct `RefCell` (`MathPlayRuntime::camera`, the node-graph viewport)
//! plus the locale the UI used to read off the deleted `ViewState`.

use crate::artifacts::mathematical::MathCamera;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "mathematicalcfg")]
#[dsl(layout = "lines")]
pub struct MathConfig {
    /// 🎥️ Node-graph viewport camera — session-only, never a document field. Was
    /// `MathPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: MathCamera,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

impl Default for MathConfig {
    fn default() -> Self {
        Self { camera: MathCamera::default(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(MathConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `MathConfig`'s operation enum — one variant per settled interaction (mirrors the pre-migration
/// `MathPlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()` returns —
/// mirrors `shooting_op::ShootingConfigOperation`'s "undo this tick is exactly restore the whole-config
/// snapshot from just before it" pattern: `Operation::Diff` is the WHOLE `MathConfig` (not a granular
/// patch type), `diff()` returns "the full config after this op", and
/// `protocol::OperationDiff<MathConfig>::apply` for `MathConfig` itself (see `store::impl_whole_record_config!`)
/// just returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum MathConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: MathConfig,
    },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: MathCamera,
    },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<MathConfig> for MathConfigOperation {
    type Diff = MathConfig;

    fn diff(&self, base: &MathConfig) -> MathConfig {
        let mut next = base.clone();
        match self {
            MathConfigOperation::Snapshot { config } => return config.clone(),
            MathConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            MathConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &MathConfig) -> Vec<Self> {
        vec![MathConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_config_default_is_the_identity_camera_and_english_locale() {
        let config = MathConfig::default();
        assert_eq!(config.camera, MathCamera::default());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn math_config_dsl_round_trips() {
        let config = MathConfig { camera: MathCamera { x: 5.0, y: 6.0, zoom: 2.0 }, locale: "de-DE".into() };
        store::test_support::assert_dsl_round_trip(&config);
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn config_operation_snapshot_diff_ignores_base() {
        let base = MathConfig::default();
        let mut snapshot = base.clone();
        snapshot.locale = "de-DE".into();
        let operation = MathConfigOperation::Snapshot { config: snapshot.clone() };
        assert_eq!(Operation::diff(&operation, &base), snapshot);
    }

    #[test]
    fn config_operation_set_camera_round_trips() {
        let base = MathConfig::default();
        let camera = MathCamera { x: 5.0, y: 6.0, zoom: 2.0 };
        let operation = MathConfigOperation::SetCamera { camera: camera.clone() };
        let next = Operation::diff(&operation, &base);
        assert_eq!(next.camera, camera);
        let backwards = Operation::backwards(&operation, &base);
        assert_eq!(backwards, vec![MathConfigOperation::Snapshot { config: base }]);
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn config_operation_set_locale_round_trips() {
        store::test_support::assert_op_line_round_trip(&MathConfigOperation::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests
