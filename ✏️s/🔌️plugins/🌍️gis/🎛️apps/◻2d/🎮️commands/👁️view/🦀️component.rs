//! 👁️ GIS 2D play app commands — camera, layer visibility/weight and the render/style/LOD display
//! vocabulary. Every command here is config-only: it emits `config_mutations`, never document
//! operations.

use crate::apps::gis2d::config::{layer_visible, Gis2dConfig, Gis2dConfigMutation};
use crate::apps::gis2d::maphost::map_host_from;
use crate::artifacts::gismap::op::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;
use framework_surface::tiled_map::clamp_map_layer_weight;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️ToggleLayerVisibility
pub mod toggle_layer_visibility {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "toggle-layer-visibility")]
    pub struct ToggleLayerVisibility {
        pub layer_id: String,
    }

    pub fn handle(payload: &ToggleLayerVisibility, _doc: &DocumentView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        let visible = !layer_visible(cfg.snapshot, &payload.layer_id);
        Ok(Emit::config(vec![Gis2dConfigMutation::SetLayerVisibility { layer_id: payload.layer_id.clone(), visible }]))
    }
}
//#endregion 🔖️ToggleLayerVisibility

//#region 🔖️FitWorld
pub mod fit_world {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "fit-world")]
    pub struct FitWorld {}

    pub fn handle(_payload: &FitWorld, doc: &DocumentView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        let mut host = map_host_from(doc.snapshot, cfg.snapshot);
        host.fit_world_camera();
        Ok(Emit::config(vec![Gis2dConfigMutation::SetCamera { camera_json: host.camera_json() }]))
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

    pub fn handle(payload: &SetCamera, _doc: &DocumentView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetCamera { camera_json: payload.camera_json.clone() }]))
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

    pub fn handle(payload: &SetRenderMode, _doc: &DocumentView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetRenderMode { value: payload.value.clone() }]))
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

    pub fn handle(payload: &SetVectorStyle, _doc: &DocumentView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetVectorStyle { value: payload.value.clone() }]))
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

    pub fn handle(payload: &SetLodMode, _doc: &DocumentView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetLodMode { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLodMode

//#region 🔖️SetHover
pub mod set_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "hover")]
    pub struct SetHover {
        pub hover_json: String,
    }

    pub fn handle(payload: &SetHover, _doc: &DocumentView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetHover { value_json: payload.hover_json.clone() }]))
    }
}
//#endregion 🔖️SetHover

//#region 🔖️SetLayerStrokeScale
pub mod set_layer_stroke_scale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "layer-stroke-scale")]
    pub struct SetLayerStrokeScale {
        pub layer_id: String,
        pub value: f64,
    }

    pub fn handle(payload: &SetLayerStrokeScale, _doc: &DocumentView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetLayerStrokeScale { layer_id: payload.layer_id.clone(), value: clamp_map_layer_weight(payload.value) }]))
    }
}
//#endregion 🔖️SetLayerStrokeScale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::gis2d::modes::edit::windows::map::GIS2D_PLAY_BODY_COMPOSITE;
    use crate::apps::gis2d::testkit::{app, app_with_registry, dispatch, render};
    use crate::apps::gis2d::Gis2dCommand;

    #[test]
    fn set_render_mode_is_view_state() {
        let mut app = app();
        let result = dispatch(&mut app, Gis2dCommand::SetRenderMode(set_render_mode::SetRenderMode { value: "vector".into() }));
        assert!(result.mutations.is_empty());
        assert!(render(&mut app, GIS2D_PLAY_BODY_COMPOSITE).contains("\"renderMode\":\"vector\""));
    }

    /// 👁️ A representative View action mutates only config state, so under the real registry it
    /// emits no operations and never trips the View → emits-operations guard.
    #[test]
    fn view_actions_emit_no_ops_under_registry_kind_discipline() {
        let mut app = app_with_registry();
        let render_mode = dispatch(&mut app, Gis2dCommand::SetRenderMode(set_render_mode::SetRenderMode { value: "vector".into() }));
        assert!(render_mode.mutations.is_empty(), "render mode is ephemeral config state");
        let fit = dispatch(&mut app, Gis2dCommand::FitWorld(fit_world::FitWorld {}));
        assert!(fit.mutations.is_empty(), "framing the world only moves the config camera");
    }

    #[test]
    fn toggling_a_layer_flips_its_visibility_in_the_rendered_scene() {
        let mut app = app();
        assert!(render(&mut app, GIS2D_PLAY_BODY_COMPOSITE).contains("\\\"water\\\":true"));
        dispatch(&mut app, Gis2dCommand::ToggleLayerVisibility(toggle_layer_visibility::ToggleLayerVisibility { layer_id: "water".into() }));
        assert!(render(&mut app, GIS2D_PLAY_BODY_COMPOSITE).contains("\\\"water\\\":false"));
    }

    #[test]
    fn layer_stroke_scale_is_clamped_to_the_surface_crates_range() {
        let mut app = app();
        dispatch(&mut app, Gis2dCommand::SetLayerStrokeScale(set_layer_stroke_scale::SetLayerStrokeScale { layer_id: "roads".into(), value: 99.0 }));
        let json = render(&mut app, GIS2D_PLAY_BODY_COMPOSITE);
        assert!(!json.contains("\\\"roads\\\":99"), "an out-of-range weight is clamped before it reaches the config");
    }

    #[test]
    fn hover_and_camera_write_straight_through_to_the_config() {
        let mut app = app();
        dispatch(&mut app, Gis2dCommand::SetHover(set_hover::SetHover { hover_json: r#"{"id":"p1"}"#.into() }));
        dispatch(&mut app, Gis2dCommand::SetCamera(set_camera::SetCamera { camera_json: r#"{"x":5,"y":6,"zoom":7}"#.into() }));
        let json = render(&mut app, GIS2D_PLAY_BODY_COMPOSITE);
        assert!(json.contains("\\\"zoom\\\":7"));
    }
}
//#endregion 🧪️Tests
