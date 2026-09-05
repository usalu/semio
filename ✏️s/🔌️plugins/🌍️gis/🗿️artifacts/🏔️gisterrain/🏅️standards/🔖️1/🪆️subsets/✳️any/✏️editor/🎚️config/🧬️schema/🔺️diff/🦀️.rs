//! 🔺️ Ordered sparse GIS 3D configuration changes.

use super::Gis3dConfig;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔺️Diff
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default, deny_unknown_fields))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Gis3dConfigDelta { pub camera_json: Option<String>, pub locale: Option<String> }

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Gis3dConfigDiff { pub steps: Vec<Gis3dConfigDelta> }

impl From<Gis3dConfigDelta> for Gis3dConfigDiff {
    fn from(delta: Gis3dConfigDelta) -> Self { if delta == Gis3dConfigDelta::default() { Self::default() } else { Self { steps: vec![delta] } } }
}

impl protocol::MutationDiff<Gis3dConfig> for Gis3dConfigDiff {
    fn apply(&self, base: &Gis3dConfig) -> protocol::MutationApplyResult<Gis3dConfig> {
        let mut next = base.clone();
        for step in &self.steps {
            if let Some(value) = &step.camera_json { next.camera_json = value.clone(); }
            if let Some(value) = &step.locale { next.locale = value.clone(); }
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) { self.steps.extend(other.steps); }
}
//#endregion 🔺️Diff
