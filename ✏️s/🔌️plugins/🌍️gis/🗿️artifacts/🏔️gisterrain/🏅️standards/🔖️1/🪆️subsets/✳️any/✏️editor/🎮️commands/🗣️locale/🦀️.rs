//! 🗣️ GIS 3D play app command — the host-pushed locale switch (undeclared in the manifest, never in
//! the command palette; host/test infra dispatches it directly).

use crate::op::GisTerrainMutation;
use crate::GisTerrainSnapshot;
use crate::editor::gis3d::config::{Gis3dConfig, Gis3dConfigMutation, SetLocale as SetLocaleMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, Gis3dConfig>) -> Result<Emit<GisTerrainMutation, Gis3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis3dConfigMutation::SetLocale(SetLocaleMutation { value: payload.value.clone() })]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
