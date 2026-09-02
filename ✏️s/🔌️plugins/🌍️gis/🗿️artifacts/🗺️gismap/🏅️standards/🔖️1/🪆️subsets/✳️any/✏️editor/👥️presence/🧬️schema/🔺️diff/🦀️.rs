//! 🔺️ Ordered sparse camera writes for GIS 2D presence.

use super::Gis2dPresence;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔺️Payload
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Gis2dPresenceDelta {
    pub camera_json: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Gis2dPresenceDiff {
    pub steps: Vec<Gis2dPresenceDelta>,
}

impl From<Gis2dPresenceDelta> for Gis2dPresenceDiff {
    fn from(delta: Gis2dPresenceDelta) -> Self {
        if delta == Gis2dPresenceDelta::default() { Self::default() } else { Self { steps: vec![delta] } }
    }
}
//#endregion 🔺️Payload

//#region ⚙️Application
impl protocol::MutationDiff<Gis2dPresence> for Gis2dPresenceDiff {
    fn apply(&self, base: &Gis2dPresence) -> protocol::MutationApplyResult<Gis2dPresence> {
        let mut next = base.clone();
        for step in &self.steps {
            if let Some(camera_json) = &step.camera_json { next.camera_json = camera_json.clone(); }
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.steps.extend(other.steps);
    }
}
//#endregion ⚙️Application
