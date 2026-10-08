//! 🔺️ Ordered sparse GIS 3D configuration changes.

use super::GisTerrainWindowConfig;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔺️Diff
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default, deny_unknown_fields))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GisTerrainWindowConfigDelta {
    pub camera_json: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

impl GisTerrainWindowConfigDiff {
    fn folded(&self) -> GisTerrainWindowConfigDelta {
        self.steps.iter().fold(GisTerrainWindowConfigDelta::default(), |mut folded, step| {
            if step.camera_json.is_some() {
                folded.camera_json = step.camera_json.clone();
            }
            folded
        })
    }
}

impl protocol::MutationDiff<GisTerrainWindowConfig> for GisTerrainWindowConfigDiff {
    fn apply(&self, base: &GisTerrainWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GisTerrainWindowConfig> {
        let mut next = base.clone();
        for step in &self.steps {
            if let Some(value) = &step.camera_json {
                next.camera_json = value.clone();
            }
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        let mut folded = self.folded();
        if let Some(camera) = other.folded().camera_json {
            folded.camera_json = Some(camera);
        }
        *self = Self::from(folded);
    }
}

impl protocol::DiffAlgebra<GisTerrainWindowConfig> for GisTerrainWindowConfigDiff {
    fn inverse(&self, base: &GisTerrainWindowConfig) -> Self {
        Self::from(GisTerrainWindowConfigDelta { camera_json: self.folded().camera_json.map(|_| base.camera_json.clone()) })
    }
    fn between(base: &GisTerrainWindowConfig, other: &GisTerrainWindowConfig) -> Self {
        Self::from(GisTerrainWindowConfigDelta { camera_json: (base.camera_json != other.camera_json).then(|| other.camera_json.clone()) })
    }
    fn is_empty(&self) -> bool {
        self.steps.iter().all(|step| *step == GisTerrainWindowConfigDelta::default())
    }
}
//#endregion 🔺️Diff
