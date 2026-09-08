//! 🗣️ GIS 2D play app command — the host-pushed locale switch (undeclared in the manifest, never in
//! the command palette; host/test infra dispatches it directly).

use crate::op::GisMapMutation;
use crate::GisMapSnapshot;
use crate::editor::gis2d::config::{mutations as config_mutations, Gis2dConfig, Gis2dConfigMutation};
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

    pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetLocale(config_mutations::SetLocale { value: payload.value.clone() })]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
