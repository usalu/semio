//! 🔺️ Sparse configuration writes with an actual identity and exact keyed removals.

use super::MapWindowConfig;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔺️Payload
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

impl MapWindowConfigDelta {
    fn absorb(&mut self, later: &Self) {
        for (id, value) in &later.layer_visibility {
            self.layer_visibility.insert(id.clone(), *value);
        }
        if later.camera_json.is_some() {
            self.camera_json = later.camera_json.clone();
        }
        if later.render_mode.is_some() {
            self.render_mode = later.render_mode.clone();
        }
        if later.vector_style.is_some() {
            self.vector_style = later.vector_style.clone();
        }
        if later.lod_mode.is_some() {
            self.lod_mode = later.lod_mode.clone();
        }
        for (id, value) in &later.layer_stroke_scale {
            if !self.layer_stroke_scale.get(id).is_some_and(|prior| prior.is_some_and(|prior| !prior.is_finite())) {
                self.layer_stroke_scale.insert(id.clone(), *value);
            }
        }
    }
}

impl MapWindowConfigDiff {
    fn folded(&self) -> MapWindowConfigDelta {
        self.steps.iter().fold(MapWindowConfigDelta::default(), |mut folded, step| {
            folded.absorb(step);
            folded
        })
    }
}

impl protocol::MutationDiff<MapWindowConfig> for MapWindowConfigDiff {
    fn apply(&self, base: &MapWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<MapWindowConfig> {
        let mut next = base.clone();
        for step in &self.steps {
            step.apply_into(&mut next)?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        let mut folded = self.folded();
        folded.absorb(&other.folded());
        *self = Self::from(folded);
    }
}

impl protocol::DiffAlgebra<MapWindowConfig> for MapWindowConfigDiff {
    fn inverse(&self, base: &MapWindowConfig) -> Self {
        let folded = self.folded();
        Self::from(MapWindowConfigDelta {
            layer_visibility: folded.layer_visibility.keys().map(|id| (id.clone(), base.layer_visibility.get(id).copied())).collect(),
            camera_json: folded.camera_json.as_ref().map(|_| base.camera_json.clone()),
            render_mode: folded.render_mode.as_ref().map(|_| base.render_mode.clone()),
            vector_style: folded.vector_style.as_ref().map(|_| base.vector_style.clone()),
            lod_mode: folded.lod_mode.as_ref().map(|_| base.lod_mode.clone()),
            layer_stroke_scale: folded.layer_stroke_scale.keys().map(|id| (id.clone(), base.layer_stroke_scale.get(id).copied())).collect(),
        })
    }

    fn between(base: &MapWindowConfig, other: &MapWindowConfig) -> Self {
        Self::from(MapWindowConfigDelta {
            layer_visibility: base.layer_visibility.keys().chain(other.layer_visibility.keys()).collect::<std::collections::BTreeSet<_>>().into_iter().filter(|id| base.layer_visibility.get(*id) != other.layer_visibility.get(*id)).map(|id| (id.clone(), other.layer_visibility.get(id).copied())).collect(),
            camera_json: (base.camera_json != other.camera_json).then(|| other.camera_json.clone()),
            render_mode: (base.render_mode != other.render_mode).then(|| other.render_mode.clone()),
            vector_style: (base.vector_style != other.vector_style).then(|| other.vector_style.clone()),
            lod_mode: (base.lod_mode != other.lod_mode).then(|| other.lod_mode.clone()),
            layer_stroke_scale: base.layer_stroke_scale.keys().chain(other.layer_stroke_scale.keys()).collect::<std::collections::BTreeSet<_>>().into_iter().filter(|id| base.layer_stroke_scale.get(*id) != other.layer_stroke_scale.get(*id)).map(|id| (id.clone(), other.layer_stroke_scale.get(id).copied())).collect(),
        })
    }

    fn is_empty(&self) -> bool {
        self.steps.iter().all(|step| *step == MapWindowConfigDelta::default())
    }
}
//#endregion ⚙️Application
