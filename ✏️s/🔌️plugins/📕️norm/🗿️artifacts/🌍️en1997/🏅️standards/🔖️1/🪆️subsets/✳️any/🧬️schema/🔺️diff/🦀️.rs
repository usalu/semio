//! 🧬️ EN 1997 diff schema — sparse field delta over the geotechnical project.

use crate::{Pile, RetainingWall, Slope, SoilLayer, SpreadFoundation, UpliftCase};
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997SoilLayerList { pub values: Vec<SoilLayer> }

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997FootingList { pub values: Vec<SpreadFoundation> }

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997PileList { pub values: Vec<Pile> }

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997WallList { pub values: Vec<RetainingWall> }

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997SlopeList { pub values: Vec<Slope> }

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997UpliftList { pub values: Vec<UpliftCase> }
//#endregion 🔖️DeltaHelpers
