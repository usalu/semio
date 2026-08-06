//! 🧮️ Shooting play app — view state (`ShootingConfig`) and its operation enum
//! (`ShootingConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.shooting` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection/camera/utility edits are VCS'd exactly like
//! document content.

use crate::artifacts::shooting::ShootingCamera;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ B1: shooting's real `DocumentApp::Config` — the pure-trait pilot's config artifact. Absorbs
/// both the old sticky `ActionArgDef` defaults (`default_shot_format`/`shape`/`default_asset_format`)
/// AND everything that used to live in an app-struct `RefCell` runtime (selection, hover, selection
/// method, center-model toggle, fit-revision counter, camera draft label, and the free/live viewport
/// camera) — session-only view state now round-trips through the config `DocumentStore` exactly like
/// document content, with a real `backwards` per [`ShootingConfigOperation`] instead of never being
/// VCS'd at all. `locale`/`active_utility_id` are the two view-state fields the shooting UI actually
/// reads (`resolve_labels`/the transform-gumball utility) — see `crate::apps::shooting::render`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "shootingcfg")]
#[dsl(layout = "lines")]
pub struct ShootingConfig {
    /// 🖼️ Mirrors `addShot`'s `format` `ActionArgDef` default (`"png"`).
    pub default_shot_format: String,
    /// 🖼️ Mirrors `addShot`'s `shape` `ActionArgDef` default (`"rectangle"`).
    pub default_shot_shape: String,
    /// 🧱️ Mirrors `addAsset`'s `format` `ActionArgDef` default (`"glb"`).
    pub default_asset_format: String,
    /// 👁️ Selected shot ids.
    pub selected_shot_ids: Vec<String>,
    /// 👁️ Selected asset ids.
    pub selected_asset_ids: Vec<String>,
    /// 👁️ Marquee selection method (`"rectangle"`/…).
    pub selection_method: String,
    /// 👁️ Hovered asset id.
    pub hovered_asset_id: Option<String>,
    /// 👁️ "Center model in viewport" toggle.
    pub center_model: bool,
    /// 👁️ Bumped whenever the active asset changes to re-trigger a viewport fit.
    pub fit_revision: u32,
    /// 👁️ In-progress "save camera" label draft.
    pub camera_draft_label: String,
    /// 🎥️ The free/live viewport camera — session-only, never a document field.
    #[dsl(block)]
    pub camera: ShootingCamera,
    /// 🧰️ The active transform-gumball utility for the scene window.
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
}

impl Default for ShootingConfig {
    fn default() -> Self {
        Self {
            default_shot_format: "png".into(),
            default_shot_shape: "rectangle".into(),
            default_asset_format: "glb".into(),
            selected_shot_ids: Vec::new(),
            selected_asset_ids: Vec::new(),
            selection_method: "rectangle".into(),
            hovered_asset_id: None,
            center_model: true,
            fit_revision: 0,
            camera_draft_label: String::new(),
            camera: ShootingCamera::default(),
            active_utility_id: "move".into(),
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(ShootingConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ B1: [`ShootingConfig`]'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// ephemeral runtime field writes), plus a generic `Snapshot` every variant's `backwards()` returns:
/// since a config-only "View" dispatch is a plain `Apply` (not an `AmendLast`), each tick is its own
/// distinct, real config edit, and "undo this tick" is exactly "restore the whole-config snapshot from
/// just before it" — the simplest correct inverse, needing no per-field reverse-patch bookkeeping.
/// `Operation::Diff` is the WHOLE `ShootingConfig` (not a granular patch type, unlike `ShootingDiff`):
/// `diff()` returns "the full config after this op", and `store::impl_whole_record_config!` supplies the
/// `OperationDiff<ShootingConfig>` that returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[allow(clippy::large_enum_variant, reason = "Snapshot{config: ShootingConfig} mirrors the pre-migration shape verbatim (a whole-record config snapshot, not a size regression this migration introduced); boxing it would change the wire shape")]
pub enum ShootingConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: ShootingConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { shot_ids: Vec<String>, asset_ids: Vec<String> },
    #[dsl(key = "hovered-asset")]
    SetHoveredAsset { asset_id: Option<String> },
    #[dsl(key = "selection-method")]
    SetSelectionMethod { method: String },
    #[dsl(key = "center-model")]
    SetCenterModel { value: bool },
    #[dsl(key = "fit-revision")]
    SetFitRevision { value: u32 },
    #[dsl(key = "camera-draft-label")]
    SetCameraDraftLabel { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: ShootingCamera,
    },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "defaults")]
    SetDefaults { shot_format: String, shot_shape: String, asset_format: String },
}

impl Operation<ShootingConfig> for ShootingConfigOperation {
    type Diff = ShootingConfig;

    fn diff(&self, base: &ShootingConfig) -> ShootingConfig {
        let mut next = base.clone();
        match self {
            ShootingConfigOperation::Snapshot { config } => return config.clone(),
            ShootingConfigOperation::SetSelection { shot_ids, asset_ids } => {
                next.selected_shot_ids = shot_ids.clone();
                next.selected_asset_ids = asset_ids.clone();
            }
            ShootingConfigOperation::SetHoveredAsset { asset_id } => next.hovered_asset_id = asset_id.clone(),
            ShootingConfigOperation::SetSelectionMethod { method } => next.selection_method = method.clone(),
            ShootingConfigOperation::SetCenterModel { value } => next.center_model = *value,
            ShootingConfigOperation::SetFitRevision { value } => next.fit_revision = *value,
            ShootingConfigOperation::SetCameraDraftLabel { value } => next.camera_draft_label = value.clone(),
            ShootingConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            ShootingConfigOperation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            ShootingConfigOperation::SetLocale { value } => next.locale = value.clone(),
            ShootingConfigOperation::SetDefaults { shot_format, shot_shape, asset_format } => {
                next.default_shot_format = shot_format.clone();
                next.default_shot_shape = shot_shape.clone();
                next.default_asset_format = asset_format.clone();
            }
        }
        next
    }

    fn backwards(&self, base: &ShootingConfig) -> Vec<Self> {
        vec![ShootingConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shooting_config_default_matches_the_existing_action_arg_sticky_defaults() {
        let config = ShootingConfig::default();
        assert_eq!(config.default_shot_format, "png");
        assert_eq!(config.default_shot_shape, "rectangle");
        assert_eq!(config.default_asset_format, "glb");
    }

    /// 🎞️ A fixture exercising every field — the dsl/pack round-trip law for `ShootingConfig`.
    #[test]
    fn shooting_config_dsl_pack_round_trip() {
        let config = ShootingConfig {
            selected_shot_ids: vec!["s1".into()],
            selected_asset_ids: vec!["a1".into(), "a2".into()],
            selection_method: "lasso".into(),
            hovered_asset_id: Some("a1".into()),
            center_model: false,
            fit_revision: 3,
            camera_draft_label: "Hero".into(),
            camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..ShootingCamera::default() },
            active_utility_id: "rotate".into(),
            locale: "de-DE".into(),
            ..ShootingConfig::default()
        };
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn shooting_config_operation_text_binary_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&ShootingConfigOperation::Snapshot { config: ShootingConfig { selected_shot_ids: vec!["s1".into()], locale: "de-DE".into(), ..ShootingConfig::default() } });
        store::test_support::assert_op_line_round_trip(&ShootingConfigOperation::SetSelection { shot_ids: vec!["s1".into()], asset_ids: vec!["a1".into(), "a2".into()] });
        store::test_support::assert_op_line_round_trip(&ShootingConfigOperation::SetHoveredAsset { asset_id: Some("a1".into()) });
        store::test_support::assert_op_line_round_trip(&ShootingConfigOperation::SetHoveredAsset { asset_id: None });
        store::test_support::assert_op_line_round_trip(&ShootingConfigOperation::SetSelectionMethod { method: "rectangle".into() });
        store::test_support::assert_op_line_round_trip(&ShootingConfigOperation::SetCenterModel { value: true });
        store::test_support::assert_op_line_round_trip(&ShootingConfigOperation::SetFitRevision { value: 4 });
        store::test_support::assert_op_line_round_trip(&ShootingConfigOperation::SetCameraDraftLabel { value: "Hero".into() });
        store::test_support::assert_op_line_round_trip(&ShootingConfigOperation::SetCamera { camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..ShootingCamera::default() } });
        store::test_support::assert_op_line_round_trip(&ShootingConfigOperation::SetActiveUtility { utility_id: "rotate".into() });
        store::test_support::assert_op_line_round_trip(&ShootingConfigOperation::SetLocale { value: "de-DE".into() });
        store::test_support::assert_op_line_round_trip(&ShootingConfigOperation::SetDefaults { shot_format: "svg".into(), shot_shape: "ellipse".into(), asset_format: "glb".into() });
    }

    #[test]
    fn shooting_config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = ShootingConfig { selected_shot_ids: vec!["s1".into()], locale: "en-US".into(), ..ShootingConfig::default() };
        let operation = ShootingConfigOperation::SetSelection { shot_ids: vec!["s2".into()], asset_ids: Vec::new() };
        let forward = operation.diff(&base);
        assert_eq!(forward.selected_shot_ids, vec!["s2".to_string()]);
        let backwards = operation.backwards(&base);
        assert_eq!(backwards, vec![ShootingConfigOperation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
