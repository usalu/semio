//! 👁️ GIS 3D play app command — the free/live viewport camera. Config-only: it emits
//! `window_config_mutations`, never document operations.

use crate::standards::v1::subsets::any::schema::mutations::GisTerrainMutation;
use crate::GisTerrainSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        pub camera_json: String,
    }

    pub fn handle(_payload: &SetCamera, _doc: &ArtifactView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️SetCamera

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
