//! 🔺️ Ordered sparse GIS 3D configuration changes.

use super::GisTerrainWindowConfig;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔺️Diff
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default, deny_unknown_fields))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GisTerrainWindowConfigDelta {
    pub camera_json: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisTerrainWindowConfigDiff {
    pub steps: Vec<GisTerrainWindowConfigDelta>,
}

impl From<GisTerrainWindowConfigDelta> for GisTerrainWindowConfigDiff {
    fn from(delta: GisTerrainWindowConfigDelta) -> Self {
        if delta == GisTerrainWindowConfigDelta::default() {
            Self::default()
        } else {
            Self { steps: vec![delta] }
        }
    }
}

impl protocol::MutationDiff<GisTerrainWindowConfig> for GisTerrainWindowConfigDiff {
    fn apply(&self, base: &GisTerrainWindowConfig) -> protocol::MutationApplyResult<GisTerrainWindowConfig> {
        let mut next = base.clone();
        for step in &self.steps {
            if let Some(value) = &step.camera_json {
                next.camera_json = value.clone();
            }
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        self.steps.extend(other.steps);
    }
}
//#endregion 🔺️Diff
