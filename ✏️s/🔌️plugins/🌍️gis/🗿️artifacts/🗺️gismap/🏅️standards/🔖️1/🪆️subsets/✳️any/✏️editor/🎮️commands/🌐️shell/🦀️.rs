//! 🌐️ GIS 2D play app command — the Shell-kind effect that opens a picked feature's source URL
//! through the host.

use crate::editor::gis2d::modes::edit::windows::map::config;
use crate::editor::gis2d::maphost::map_host_from;
use crate::op::GisMapMutation;
use crate::GisMapSnapshot;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️OpenSource
pub mod open_source {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "open-source")]
    pub struct OpenSource {
        pub feature_id: String,
    }

    pub fn handle(payload: &OpenSource, doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisMapMutation, NoConfigMutation>, Fault> {
        let config = config::current(cfg);
        let host = map_host_from(doc.snapshot, &config);
        match host.features.positions.get(&payload.feature_id).and_then(|row| row.source_url.clone()) {
            Some(url) => Ok(Emit::effect(Effect::OpenExternalUrl { url })),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️OpenSource

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
