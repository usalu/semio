//! 🎥️ 🎥️ FEM 3D app commands command — `set-camera`.

use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::{Fem3dSnapshot, FemCamera};
use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "camera")]
pub struct SetCamera {
    pub json: String,
}

pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Fem3dConfigMutation::SetCamera { camera: FemCamera { json: payload.json.clone() } }]))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
