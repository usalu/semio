//! 👁️ GIS 2D play app commands — camera, layer visibility/weight and the render/style/LOD display
//! vocabulary. Every command here is config-only: it emits `config_mutations`, never document
//! operations.

use crate::op::GisMapMutation;
use crate::GisMapSnapshot;
use crate::editor::gis2d::config::{layer_visible, mutations as config_mutations, Gis2dConfig, Gis2dConfigMutation};
use crate::editor::gis2d::maphost::map_host_from;
use semio_framework_surface::tiled_map::clamp_map_layer_weight;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ToggleLayerVisibility
pub mod toggle_layer_visibility {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
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

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
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

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
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

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
