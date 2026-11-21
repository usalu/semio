//! 🔺️ Ordered sparse GIS 3D configuration changes.

use super::{GisTerrainWindowConfig, TerrainBrushSettings};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔺️Diff
#[derive(Clone, Copy, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(default, deny_unknown_fields))]
#[value(default, deny_unknown_fields)]
pub struct TerrainBrushChange {
    #[dsl(block)]
    pub brush: Option<TerrainBrushSettings>,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default, deny_unknown_fields))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GisTerrainWindowConfigDelta {
    pub camera_json: Option<String>,
    #[dsl(block)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub brush: Option<TerrainBrushChange>,
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
            if step.brush.is_some() {
                folded.brush = step.brush;
            }
            folded
        })
    }
}

impl protocol::MutationDiff<GisTerrainWindowConfig> for GisTerrainWindowConfigDiff {}

impl protocol::ColdMutationDiff<GisTerrainWindowConfig> for GisTerrainWindowConfigDiff {
    fn absorb(&mut self, other: Self) {
        let mut folded = self.folded();
        let other = other.folded();
        if let Some(camera) = other.camera_json {
            folded.camera_json = Some(camera);
        }
        if other.brush.is_some() {
            folded.brush = other.brush;
        }
        *self = Self::from(folded);
    }

    fn apply(&self, unpublished: &mut GisTerrainWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        for step in &self.steps {
            if let Some(value) = &step.camera_json {
                next.camera_json = value.clone();
            }
            if let Some(change) = step.brush {
                next.brush = change.brush.map(TerrainBrushSettings::validate).transpose()
                    .map_err(|error| protocol::MutationApplyError::new("mutation.invalid", error.to_string()).at(["brush"]))?;
            }
        }
        *unpublished = next;
        Ok(())
    }
}

impl protocol::DiffAlgebra<GisTerrainWindowConfig> for GisTerrainWindowConfigDiff {
    fn is_empty(&self) -> bool {
        self.steps.iter().all(|step| *step == GisTerrainWindowConfigDelta::default())
    }
}

impl protocol::ColdDiffAlgebra<GisTerrainWindowConfig> for GisTerrainWindowConfigDiff {
    fn inverse(&self, base: &GisTerrainWindowConfig) -> Self {
        let folded = self.folded();
        Self::from(GisTerrainWindowConfigDelta {
            camera_json: folded.camera_json.map(|_| base.camera_json.clone()),
            brush: folded.brush.map(|_| TerrainBrushChange { brush: base.brush }),
        })
    }
}
//#endregion 🔺️Diff
