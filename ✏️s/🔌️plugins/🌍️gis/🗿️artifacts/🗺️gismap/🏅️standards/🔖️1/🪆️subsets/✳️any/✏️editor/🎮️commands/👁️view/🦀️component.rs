//! 👁️ GIS 2D play app commands — camera, layer visibility/weight and the render/style/LOD display
//! vocabulary. Every command here is config-only: it emits `config_mutations`, never document
//! operations.

use crate::artifacts::gismap::op::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;
use crate::editor::gis2d::config::{layer_visible, mutations as config_mutations, Gis2dConfig, Gis2dConfigMutation};
use crate::editor::gis2d::maphost::map_host_from;
use framework_surface::tiled_map::clamp_map_layer_weight;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️ToggleLayerVisibility
pub mod toggle_layer_visibility {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "toggle-layer-visibility")]
    pub struct ToggleLayerVisibility {
        pub layer_id: String,
    }

    pub fn handle(payload: &ToggleLayerVisibility, _doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        let visible = !layer_visible(cfg.snapshot, &payload.layer_id);
        Ok(Emit::config(vec![Gis2dConfigMutation::SetLayerVisibility(config_mutations::SetLayerVisibility { layer_id: payload.layer_id.clone(), visible: (!visible).then_some(false) })]))
    }
}
//#endregion 🔖️ToggleLayerVisibility

//#region 🔖️FitWorld
pub mod fit_world {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "fit-world")]
    pub struct FitWorld {}

    pub fn handle(_payload: &FitWorld, doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        let mut host = map_host_from(doc.snapshot, cfg.snapshot);
        host.fit_world_camera();
        Ok(Emit::config(vec![Gis2dConfigMutation::SetCamera(config_mutations::SetCamera { camera_json: host.camera_json() })]))
    }
}
//#endregion 🔖️FitWorld

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        pub camera_json: String,
    }

    pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetCamera(config_mutations::SetCamera { camera_json: payload.camera_json.clone() })]))
    }
}
//#endregion 🔖️SetCamera

//#region 🔖️SetRenderMode
pub mod set_render_mode {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "render-mode")]
    pub struct SetRenderMode {
        pub value: String,
    }

    pub fn handle(payload: &SetRenderMode, _doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetRenderMode(config_mutations::SetRenderMode { value: payload.value.clone() })]))
    }
}
//#endregion 🔖️SetRenderMode

//#region 🔖️SetVectorStyle
pub mod set_vector_style {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "vector-style")]
    pub struct SetVectorStyle {
        pub value: String,
    }

    pub fn handle(payload: &SetVectorStyle, _doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetVectorStyle(config_mutations::SetVectorStyle { value: payload.value.clone() })]))
    }
}
//#endregion 🔖️SetVectorStyle

//#region 🔖️SetLodMode
pub mod set_lod_mode {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "lod-mode")]
    pub struct SetLodMode {
        pub value: String,
    }

    pub fn handle(payload: &SetLodMode, _doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetLodMode(config_mutations::SetLodMode { value: payload.value.clone() })]))
    }
}
//#endregion 🔖️SetLodMode

//#region 🔖️FocusFeature
/// 🕹️ Relocated from the deleted `🎮️commands/🗂️selection` node (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM): frames the camera on one named feature —
/// never reads or writes selection state, so it survives the mechanism migration unchanged.
pub mod focus_feature {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "focus-feature")]
    pub struct FocusFeature {
        pub feature_id: String,
        pub feature_kind: String,
    }

    pub fn handle(payload: &FocusFeature, doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        let mut host = map_host_from(doc.snapshot, cfg.snapshot);
        if host.focus_feature(&payload.feature_kind, &payload.feature_id) {
            Ok(Emit::config(vec![Gis2dConfigMutation::SetCamera(config_mutations::SetCamera { camera_json: host.camera_json() })]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️FocusFeature

//#region 🔖️SetLayerStrokeScale
pub mod set_layer_stroke_scale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "layer-stroke-scale")]
    pub struct SetLayerStrokeScale {
        pub layer_id: String,
        pub value: f64,
    }

    pub fn handle(payload: &SetLayerStrokeScale, _doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        let value = clamp_map_layer_weight(payload.value);
        Ok(Emit::config(vec![Gis2dConfigMutation::SetLayerStrokeScale(config_mutations::SetLayerStrokeScale { layer_id: payload.layer_id.clone(), value: (value != 1.0).then_some(value) })]))
    }
}
//#endregion 🔖️SetLayerStrokeScale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis2d::modes::edit::windows::map::GIS2D_PLAY_BODY_COMPOSITE;
    use crate::editor::gis2d::testkit::{app, app_with_registry, dispatch, render};
    use crate::editor::gis2d::Gis2dCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_render_mode_is_view_state() {
        let mut app = app();
        let result = dispatch(&mut app, Gis2dCommand::SetRenderMode(set_render_mode::SetRenderMode { value: "vector".into() }));
        assert!(result.mutations.is_empty());
        assert!(render(&mut app, GIS2D_PLAY_BODY_COMPOSITE).contains("\"renderMode\":\"vector\""));
    }

    /// 👁️ A representative View action mutates only config state, so under the real registry it
    /// emits no operations and never trips the View → emits-operations guard.
    #[semio_framework_async_macros::async_test]
    async fn view_actions_emit_no_ops_under_registry_kind_discipline() {
        let mut app = app_with_registry();
        let render_mode = dispatch(&mut app, Gis2dCommand::SetRenderMode(set_render_mode::SetRenderMode { value: "vector".into() }));
        assert!(render_mode.mutations.is_empty(), "render mode is ephemeral config state");
        let fit = dispatch(&mut app, Gis2dCommand::FitWorld(fit_world::FitWorld {}));
        assert!(fit.mutations.is_empty(), "framing the world only moves the config camera");
    }

    #[semio_framework_async_macros::async_test]
    async fn toggling_a_layer_flips_its_visibility_in_the_rendered_scene() {
        let mut app = app();
        assert!(render(&mut app, GIS2D_PLAY_BODY_COMPOSITE).contains("\\\"water\\\":true"));
        dispatch(&mut app, Gis2dCommand::ToggleLayerVisibility(toggle_layer_visibility::ToggleLayerVisibility { layer_id: "water".into() }));
        assert!(render(&mut app, GIS2D_PLAY_BODY_COMPOSITE).contains("\\\"water\\\":false"));
    }

    #[semio_framework_async_macros::async_test]
    async fn layer_stroke_scale_is_clamped_to_the_surface_crates_range() {
        let mut app = app();
        dispatch(&mut app, Gis2dCommand::SetLayerStrokeScale(set_layer_stroke_scale::SetLayerStrokeScale { layer_id: "roads".into(), value: 99.0 }));
        let json = render(&mut app, GIS2D_PLAY_BODY_COMPOSITE);
        assert!(!json.contains("\\\"roads\\\":99"), "an out-of-range weight is clamped before it reaches the config");
    }

    #[semio_framework_async_macros::async_test]
    async fn camera_writes_straight_through_to_the_config() {
        let mut app = app();
        dispatch(&mut app, Gis2dCommand::SetCamera(set_camera::SetCamera { camera_json: r#"{"x":5,"y":6,"zoom":7}"#.into() }));
        let json = render(&mut app, GIS2D_PLAY_BODY_COMPOSITE);
        assert!(json.contains("\\\"zoom\\\":7"));
    }

    #[semio_framework_async_macros::async_test]
    async fn focus_feature_on_an_unknown_id_emits_nothing() {
        let mut app = app();
        let result = dispatch(&mut app, Gis2dCommand::FocusFeature(focus_feature::FocusFeature { feature_id: "nope".into(), feature_kind: "position".into() }));
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
