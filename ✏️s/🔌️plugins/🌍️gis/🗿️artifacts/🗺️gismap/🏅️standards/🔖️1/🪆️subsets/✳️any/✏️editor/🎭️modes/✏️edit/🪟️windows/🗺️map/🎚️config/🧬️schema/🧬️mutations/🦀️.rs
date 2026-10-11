//! 🧬️ Transparent direct configuration mutation roster.

use super::{MapWindowConfig, MapWindowConfigDiff};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Leaves
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
#[path = "📏️set-layer-stroke-scale/🦀️.rs"]
mod set_layer_stroke_scale;
#[path = "👁️set-layer-visibility/🦀️.rs"]
mod set_layer_visibility;
#[path = "🔽️set-lod-mode/🦀️.rs"]
mod set_lod_mode;
#[path = "🖼️set-render-mode/🦀️.rs"]
mod set_render_mode;
#[path = "🎨️set-vector-style/🦀️.rs"]
mod set_vector_style;
pub use set_camera::SetCamera;
pub use set_layer_stroke_scale::SetLayerStrokeScale;
pub use set_layer_visibility::SetLayerVisibility;
pub use set_lod_mode::SetLodMode;
pub use set_render_mode::SetRenderMode;
pub use set_vector_style::SetVectorStyle;
//#endregion 🧬️Leaves

//#region 🧬️Aggregate
#[derive(Clone, Debug, PartialEq, dsl::Mutations, semio_framework_dsl_record_derive::DslEnum, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = MapWindowConfig, diff = MapWindowConfigDiff, schema = "gis.mapwindowcfg")]
pub enum MapWindowConfigMutation {
    SetLayerVisibility(SetLayerVisibility),
    SetCamera(SetCamera),
    SetRenderMode(SetRenderMode),
    SetVectorStyle(SetVectorStyle),
    SetLodMode(SetLodMode),
    SetLayerStrokeScale(SetLayerStrokeScale),
}
impl store::snapshot_clone_preparation::ConfigApplyMutation<MapWindowConfig> for MapWindowConfigMutation {
    fn exchange(self, post: &mut MapWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::SetLayerVisibility(SetLayerVisibility { layer_id, visible }) => {
                let displaced = match visible {
                    Some(value) => post.layer_visibility.insert(layer_id.clone(), value),
                    None => post.layer_visibility.remove(&layer_id),
                };
                Self::SetLayerVisibility(SetLayerVisibility { layer_id, visible: displaced })
            }
            Self::SetCamera(SetCamera { camera_json }) => Self::SetCamera(SetCamera { camera_json: std::mem::replace(&mut post.camera_json, camera_json) }),
            Self::SetRenderMode(SetRenderMode { value }) => Self::SetRenderMode(SetRenderMode { value: std::mem::replace(&mut post.render_mode, value) }),
            Self::SetVectorStyle(SetVectorStyle { value }) => Self::SetVectorStyle(SetVectorStyle { value: std::mem::replace(&mut post.vector_style, value) }),
            Self::SetLodMode(SetLodMode { value }) => Self::SetLodMode(SetLodMode { value: std::mem::replace(&mut post.lod_mode, value) }),
            Self::SetLayerStrokeScale(SetLayerStrokeScale { layer_id, value }) => {
                let displaced = match value {
                    Some(scale) => post.layer_stroke_scale.insert(layer_id.clone(), scale),
                    None => post.layer_stroke_scale.remove(&layer_id),
                };
                Self::SetLayerStrokeScale(SetLayerStrokeScale { layer_id, value: displaced })
            }
        })
    }
    fn payload_bytes(&self) -> usize {
        match self {
            Self::SetLayerVisibility(SetLayerVisibility { layer_id, .. }) | Self::SetLayerStrokeScale(SetLayerStrokeScale { layer_id, .. }) => layer_id.len(),
            Self::SetCamera(SetCamera { camera_json }) => camera_json.len(),
            Self::SetRenderMode(SetRenderMode { value }) | Self::SetVectorStyle(SetVectorStyle { value }) | Self::SetLodMode(SetLodMode { value }) => value.len(),
        }
    }
}

//#endregion 🧬️Aggregate

//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
