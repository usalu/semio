//! 🧮️ Layout play app — view state (`LayoutConfig`) and its operation enum (`LayoutConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.layout` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection/camera/hover edits are VCS'd exactly like
//! document content.

use crate::artifacts::layout::LayoutCamera;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// @emoji 👻️ WORKFLOWS-END-TO-END-TYPED-PORTS Config recipe: ephemeral catalogue drag-ghost state (was
/// `layout_ui::LayoutDropPreviewState`, a private UI-crate struct) — `kind.is_empty()` means "no live
/// drop preview" (the B1 config idiom favors an always-present default record over `Option<Record>` so
/// every field round-trips through the `dsl`/pack machinery uniformly).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct LayoutDropPreviewState {
    pub kind: String,
    pub x: f64,
    pub y: f64,
}

/// 🧮️ B1: layout's real `DocumentApp::Config` — absorbs every field that used to live on
/// `layout_ui::LayoutPlayApp`'s `RefCell<LayoutPlayRuntime>` (active page, selection, hover, drop-ghost,
/// engagement draft, and the two independent blueprint/preview camera poses) plus `locale`, the one
/// `ViewState` field the layout UI actually reads — session-only view state now round-trips through the
/// config `DocumentStore` exactly like document content, with a real `backwards` per
/// `LayoutConfigOperation` instead of never being VCS'd at all.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "layoutcfg")]
#[dsl(layout = "lines")]
pub struct LayoutConfig {
    /// 👁️ Active page shown/edited on the Blueprint surface — was `LayoutPlayRuntime::active_page_id`.
    pub active_page_id: String,
    /// 👁️ Selected page/frame ids — was `LayoutPlayRuntime::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 👁️ Hovered page/frame id — was `LayoutPlayRuntime::hovered_id`.
    pub hovered_id: Option<String>,
    /// 👁️ Live catalogue drag-ghost — was `LayoutPlayRuntime::drop_preview` (`Option<LayoutDropPreviewState>`).
    #[dsl(block)]
    pub drop_preview: LayoutDropPreviewState,
    /// 👁️ In-progress engagement-bar input draft — was `LayoutPlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 📷️ The Blueprint surface's ephemeral camera pose — was `LayoutPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: LayoutCamera,
    /// 📷️ The Preview surface's ephemeral camera pose — was `LayoutPlayRuntime::preview_camera`.
    #[dsl(block)]
    pub preview_camera: LayoutCamera,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            active_page_id: "page-1".into(),
            selected_ids: Vec::new(),
            hovered_id: None,
            drop_preview: LayoutDropPreviewState::default(),
            engagement_input: String::new(),
            camera: LayoutCamera::default(),
            preview_camera: LayoutCamera::default(),
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(LayoutConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`LayoutConfig`]'s operation enum — one variant per settled interaction, plus a generic
/// `Snapshot` every variant's `backwards()` returns. `Operation::Diff` is the WHOLE `LayoutConfig` (not a
/// granular patch type): `diff()` returns "the full config after this op", and
/// `store::impl_whole_record_config!` supplies the `OperationDiff<LayoutConfig>` that returns that
/// snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum LayoutConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: LayoutConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "active-page")]
    SetActivePage { page_id: String },
    #[dsl(key = "hover")]
    SetHover { id: Option<String> },
    #[dsl(key = "drop-preview")]
    SetDropPreview {
        #[dsl(block)]
        preview: LayoutDropPreviewState,
    },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: LayoutCamera,
    },
    #[dsl(key = "preview-camera")]
    SetPreviewCamera {
        #[dsl(block)]
        camera: LayoutCamera,
    },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<LayoutConfig> for LayoutConfigOperation {
    type Diff = LayoutConfig;

    fn diff(&self, base: &LayoutConfig) -> LayoutConfig {
        let mut next = base.clone();
        match self {
            LayoutConfigOperation::Snapshot { config } => return config.clone(),
            LayoutConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            LayoutConfigOperation::SetActivePage { page_id } => next.active_page_id = page_id.clone(),
            LayoutConfigOperation::SetHover { id } => next.hovered_id = id.clone(),
            LayoutConfigOperation::SetDropPreview { preview } => next.drop_preview = preview.clone(),
            LayoutConfigOperation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            LayoutConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            LayoutConfigOperation::SetPreviewCamera { camera } => next.preview_camera = camera.clone(),
            LayoutConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &LayoutConfig) -> Vec<Self> {
        vec![LayoutConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_config_default_matches_the_existing_runtime_defaults() {
        let config = LayoutConfig::default();
        assert_eq!(config.active_page_id, "page-1");
        assert!(config.selected_ids.is_empty());
        assert!(config.hovered_id.is_none());
        assert_eq!(config.drop_preview, LayoutDropPreviewState::default());
        assert_eq!(config.camera, LayoutCamera::default());
        assert_eq!(config.preview_camera, LayoutCamera::default());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn layout_config_dsl_and_pack_round_trip() {
        let config = LayoutConfig {
            active_page_id: "page-2".into(),
            selected_ids: vec!["frame-1".into(), "frame-2".into()],
            hovered_id: Some("frame-3".into()),
            drop_preview: LayoutDropPreviewState { kind: "text".into(), x: 12.0, y: 34.0 },
            engagement_input: "export svg".into(),
            camera: LayoutCamera { x: 5.0, y: 6.0, zoom: 1.25 },
            preview_camera: LayoutCamera { x: 7.0, y: 8.0, zoom: 0.75 },
            locale: "de-DE".into(),
        };
        store::test_support::assert_dsl_round_trip(&config);
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    fn sample_config() -> LayoutConfig {
        LayoutConfig {
            active_page_id: "page-2".into(),
            selected_ids: vec!["frame-1".into()],
            hovered_id: Some("frame-2".into()),
            drop_preview: LayoutDropPreviewState { kind: "rect".into(), x: 1.0, y: 2.0 },
            engagement_input: "export png".into(),
            camera: LayoutCamera { x: 10.0, y: 20.0, zoom: 1.5 },
            preview_camera: LayoutCamera { x: 3.0, y: 4.0, zoom: 2.0 },
            locale: "de-DE".into(),
        }
    }

    fn config_round_trip(base: &LayoutConfig, operation: &LayoutConfigOperation) -> LayoutConfig {
        let forward = operation.diff(base);
        let backwards = operation.backwards(base);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = back.diff(&restored);
        }
        assert_eq!(&restored, base, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_operations_apply_and_restore_every_field() {
        let base = LayoutConfig::default();
        assert_eq!(config_round_trip(&base, &LayoutConfigOperation::SetSelection { ids: vec!["a".into()] }).selected_ids, vec!["a".to_string()]);
        assert_eq!(config_round_trip(&base, &LayoutConfigOperation::SetActivePage { page_id: "page-9".into() }).active_page_id, "page-9");
        assert_eq!(config_round_trip(&base, &LayoutConfigOperation::SetHover { id: Some("frame-9".into()) }).hovered_id, Some("frame-9".to_string()));
        let previewed = config_round_trip(&base, &LayoutConfigOperation::SetDropPreview { preview: LayoutDropPreviewState { kind: "rect".into(), x: 5.0, y: 6.0 } });
        assert_eq!(previewed.drop_preview.kind, "rect");
        assert_eq!(config_round_trip(&base, &LayoutConfigOperation::SetEngagementInput { value: "undo".into() }).engagement_input, "undo");
        let cam = config_round_trip(&base, &LayoutConfigOperation::SetCamera { camera: LayoutCamera { x: 1.0, y: 2.0, zoom: 3.0 } });
        assert_eq!(cam.camera, LayoutCamera { x: 1.0, y: 2.0, zoom: 3.0 });
        let preview_cam = config_round_trip(&base, &LayoutConfigOperation::SetPreviewCamera { camera: LayoutCamera { x: 4.0, y: 5.0, zoom: 6.0 } });
        assert_eq!(preview_cam.preview_camera, LayoutCamera { x: 4.0, y: 5.0, zoom: 6.0 });
        assert_eq!(config_round_trip(&base, &LayoutConfigOperation::SetLocale { value: "de-DE".into() }).locale, "de-DE");
    }

    #[test]
    fn config_snapshot_op_text_round_trips() {
        let config = sample_config();
        store::test_support::assert_op_line_round_trip(&LayoutConfigOperation::Snapshot { config });
        store::test_support::assert_op_line_round_trip(&LayoutConfigOperation::SetSelection { ids: vec!["a".into(), "b".into()] });
        store::test_support::assert_op_line_round_trip(&LayoutConfigOperation::SetHover { id: None });
        store::test_support::assert_op_line_round_trip(&LayoutConfigOperation::SetLocale { value: "en-US".into() });
    }
}
//#endregion 🧪️Tests
