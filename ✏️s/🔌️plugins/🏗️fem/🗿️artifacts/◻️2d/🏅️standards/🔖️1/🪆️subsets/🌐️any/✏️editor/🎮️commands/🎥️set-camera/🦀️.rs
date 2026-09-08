//! 🎥️ 🎥️ Fem2d play app commands command — `set-camera`.

use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::FemCamera;
use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️SetCamera
//#endregion 🔖️SetCamera

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "camera")]
pub struct SetCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Fem2dConfigMutation::SetCamera { camera: FemCamera { x: payload.x, y: payload.y, zoom: payload.zoom } }]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
