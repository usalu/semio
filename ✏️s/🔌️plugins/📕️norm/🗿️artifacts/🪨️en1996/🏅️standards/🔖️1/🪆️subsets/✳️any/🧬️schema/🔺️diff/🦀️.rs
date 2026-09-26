//! 🧬️ EN 1996 diff schema — sparse field delta for masonry building subject.

use framework_schema::ArtifactSchema;

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1996")]
pub struct En1996Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifact_schema::En1996Artifact>>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub masonry_class: Option<crate::MasonryClass>,
    #[state(artifact)]
    pub design_situation: Option<crate::document::DesignSituation>,
    #[state(artifact)]
    pub storeys: Option<u32>,
    #[state(artifact)]
    pub walls: Option<En1996WallList>,
}
//#endregion 🔖️Diff

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1996WallList {
    pub values: Vec<crate::MasonryWall>,
}
