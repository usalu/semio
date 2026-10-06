//! 🧬️ EN 1997 diff schema — sparse field delta over the geotechnical project.

use crate::{Pile, RetainingWall, Slope, SoilLayer, SpreadFoundation, UpliftCase};
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1997")]
pub struct En1997Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifact_schema::En1997Artifact>>,
    #[state(artifact)]
    pub structure_id: Option<String>,
    #[state(artifact)]
    pub geotechnical_category: Option<u8>,
    #[state(artifact)]
    pub design_situation: Option<String>,
    #[state(artifact)]
    pub design_approach: Option<String>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub groundwater_level: Option<f64>,
    #[state(artifact)]
    pub investigation_depth: Option<f64>,
    #[state(artifact)]
    pub layers: Option<En1997SoilLayerList>,
    #[state(artifact)]
    pub footings: Option<En1997FootingList>,
    #[state(artifact)]
    pub piles: Option<En1997PileList>,
    #[state(artifact)]
    pub retaining_walls: Option<En1997WallList>,
    #[state(artifact)]
    pub slopes: Option<En1997SlopeList>,
    #[state(artifact)]
    pub uplift_cases: Option<En1997UpliftList>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997SoilLayerList { pub values: Vec<SoilLayer> }

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997FootingList { pub values: Vec<SpreadFoundation> }

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997PileList { pub values: Vec<Pile> }

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997WallList { pub values: Vec<RetainingWall> }

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997SlopeList { pub values: Vec<Slope> }

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997UpliftList { pub values: Vec<UpliftCase> }
//#endregion 🔖️DeltaHelpers

use crate::artifact_schema::diff::*;
use crate::artifact_schema::En1997Artifact;
use crate::En1997Snapshot;
use protocol::MutationDiff;

macro_rules! apply_scalar {
    ($self:ident, $next:ident, $($field:ident),+ $(,)?) => {
        $(
            if let Some(value) = &$self.$field {
                $next.$field = value.clone();
            }
        )+
    };
}

impl En1997Diff {
    pub fn apply_to_artifact(&self, artifact: &En1997Artifact) -> protocol::MutationApplyResult<En1997Artifact> {
        if let Some(replacement) = &self.artifact {
            return Ok((**replacement).clone());
        }
        let mut next = artifact.clone();
        apply_scalar!(self, next, structure_id, geotechnical_category, design_situation, design_approach, annex, groundwater_level, investigation_depth);
        if let Some(list) = &self.layers { next.layers = list.values.clone(); }
        if let Some(list) = &self.footings { next.footings = list.values.clone(); }
        if let Some(list) = &self.piles { next.piles = list.values.clone(); }
        if let Some(list) = &self.retaining_walls { next.retaining_walls = list.values.clone(); }
        if let Some(list) = &self.slopes { next.slopes = list.values.clone(); }
        if let Some(list) = &self.uplift_cases { next.uplift_cases = list.values.clone(); }
        Ok(next)
    }
}

impl MutationDiff<En1997Snapshot> for En1997Diff {
    fn apply(&self, snapshot: &En1997Snapshot) -> protocol::MutationApplyResult<En1997Snapshot> {
        if let Some(replacement) = &self.artifact {
            return Ok(replacement.to_snapshot());
        }
        let mut next = snapshot.clone();
        apply_scalar!(self, next, structure_id, geotechnical_category, design_situation, design_approach, annex, groundwater_level, investigation_depth);
        if let Some(list) = &self.layers { next.layers = list.values.clone(); }
        if let Some(list) = &self.footings { next.footings = list.values.clone(); }
        if let Some(list) = &self.piles { next.piles = list.values.clone(); }
        if let Some(list) = &self.retaining_walls { next.retaining_walls = list.values.clone(); }
        if let Some(list) = &self.slopes { next.slopes = list.values.clone(); }
        if let Some(list) = &self.uplift_cases { next.uplift_cases = list.values.clone(); }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(structure_id);
        take!(geotechnical_category);
        take!(design_situation);
        take!(design_approach);
        take!(annex);
        take!(groundwater_level);
        take!(investigation_depth);
        take!(layers);
        take!(footings);
        take!(piles);
        take!(retaining_walls);
        take!(slopes);
        take!(uplift_cases);
    }
}

pub fn diff_set_snapshot(snapshot: &En1997Snapshot) -> En1997Diff {
    En1997Diff { artifact: Some(Box::new(En1997Artifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
