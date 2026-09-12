//! 🔺️ Sparse configuration writes with an actual identity and exact keyed removals.

use super::MapWindowConfig;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔺️Payload
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default, deny_unknown_fields))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MapWindowConfigDelta {
    pub layer_visibility: BTreeMap<String, Option<bool>>,
    pub camera_json: Option<String>,
    pub render_mode: Option<String>,
    pub vector_style: Option<String>,
    pub lod_mode: Option<String>,
    #[cfg_attr(test, serde(serialize_with = "serialize_scales"))]
    pub layer_stroke_scale: BTreeMap<String, Option<f64>>,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct MapWindowConfigDiff {
    pub steps: Vec<MapWindowConfigDelta>,
}

impl From<MapWindowConfigDelta> for MapWindowConfigDiff {
    fn from(delta: MapWindowConfigDelta) -> Self {
        if delta == MapWindowConfigDelta::default() {
            Self::default()
        } else {
            Self { steps: vec![delta] }
        }
    }
}

#[cfg(test)]
fn serialize_scales<S: serde::Serializer>(values: &BTreeMap<String, Option<f64>>, serializer: S) -> Result<S::Ok, S::Error> {
    if values.values().any(|value| value.is_some_and(|value| !value.is_finite())) {
        return Err(serde::ser::Error::custom("layer stroke scale must be finite"));
    }
    serde::Serialize::serialize(values, serializer)
}
//#endregion 🔺️Payload

//#region ⚙️Application
impl MapWindowConfigDelta {
    fn apply_into(&self, next: &mut MapWindowConfig) -> protocol::MutationApplyResult<()> {
        for (id, value) in &self.layer_stroke_scale {
            if value.is_some_and(|value| !value.is_finite()) {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-number", "Layer stroke scale must be finite.").at(["layerStrokeScale", id.as_str()]));
            }
        }
        for (id, value) in &self.layer_visibility {
            match value {
                Some(value) => {
                    next.layer_visibility.insert(id.clone(), *value);
                }
                None => {
                    next.layer_visibility.remove(id);
                }
            }
        }
        if let Some(value) = &self.camera_json {
            next.camera_json = value.clone();
        }
        if let Some(value) = &self.render_mode {
            next.render_mode = value.clone();
        }
        if let Some(value) = &self.vector_style {
            next.vector_style = value.clone();
        }
        if let Some(value) = &self.lod_mode {
            next.lod_mode = value.clone();
        }
        for (id, value) in &self.layer_stroke_scale {
            match value {
                Some(value) => {
                    next.layer_stroke_scale.insert(id.clone(), *value);
                }
                None => {
                    next.layer_stroke_scale.remove(id);
                }
            }
        }
        Ok(())
    }
}

impl protocol::MutationDiff<MapWindowConfig> for MapWindowConfigDiff {
    fn apply(&self, base: &MapWindowConfig) -> protocol::MutationApplyResult<MapWindowConfig> {
        let mut next = base.clone();
        for step in &self.steps {
            step.apply_into(&mut next)?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.steps.extend(other.steps);
    }
}
//#endregion ⚙️Application
