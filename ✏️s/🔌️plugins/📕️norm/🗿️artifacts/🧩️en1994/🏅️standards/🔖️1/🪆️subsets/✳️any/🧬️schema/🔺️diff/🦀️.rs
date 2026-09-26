//! 🧬️ En1994 diff schema — sparse field delta over the composite structure subject.

use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1994 artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1994")]
pub struct En1994Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifact_schema::En1994Artifact>>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub structure_kind: Option<String>,
    #[state(artifact)]
    pub steel_f_y_pa: Option<f64>,
    #[state(artifact)]
    pub beams: Option<En1994BeamList>,
    #[state(artifact)]
    pub columns: Option<En1994ColumnList>,
    #[state(artifact)]
    pub slabs: Option<En1994SlabList>,
    #[state(artifact)]
    pub fire_rating: Option<String>,
    #[state(artifact)]
    pub insulation_thickness_m: Option<f64>,
    #[state(artifact)]
    pub fatigue_detail: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994StringList {
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994BeamList {
    pub values: Vec<crate::CompositeBeam>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994ColumnList {
    pub values: Vec<crate::CompositeColumn>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994SlabList {
    pub values: Vec<crate::CompositeSlab>,
}
//#endregion 🔖️DeltaHelpers
