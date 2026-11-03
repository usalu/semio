//! 🎚️ The closed semantic mutation vocabulary for Norm Viewport-window configuration.

use crate::mutations::change_viewport_view::ChangeViewportView;
use crate::diff::NormViewportWindowConfigDiff;
use crate::schema::NormViewportWindowConfig;

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = NormViewportWindowConfig, diff = NormViewportWindowConfigDiff, schema = "s.norm.viewport-window.config", reference = NormViewportWindowConfig, reference_diff = NormViewportWindowConfigDiff)]
pub enum NormViewportWindowConfigMutation {
    ChangeViewportView(ChangeViewportView),
}

impl store::snapshot_clone_preparation::ConfigApplyMutation<NormViewportWindowConfig> for NormViewportWindowConfigMutation {
    fn exchange(self, post: &mut NormViewportWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::ChangeViewportView(ChangeViewportView { camera_preset, hidden_layers, colour_map, unit_system }) => Self::ChangeViewportView(ChangeViewportView {
                camera_preset: std::mem::replace(&mut post.camera_preset, camera_preset),
                hidden_layers: std::mem::replace(&mut post.hidden_layers, hidden_layers),
                colour_map: std::mem::replace(&mut post.colour_map, colour_map),
                unit_system: std::mem::replace(&mut post.unit_system, unit_system),
            }),
        })
    }
}

//#region 🪢️Mounts
#[path = "🎥️change-viewport-view/🦀️.rs"]
pub mod change_viewport_view;
//#endregion 🪢️Mounts
