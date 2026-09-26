//! 🧬️ EN 1998 diff schema — sparse field delta.

use crate::{
    En1998Assessment, En1998Bridge, En1998Building, En1998Foundation, En1998RetainingWall, En1998Site, En1998Silo,
    En1998Tank, En1998Tower,
};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1998")]
pub struct En1998Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifact_schema::En1998Artifact>>,
    #[state(artifact)]
    pub annex: Option<String>,
    #[state(artifact)]
    pub site: Option<En1998Site>,
    #[state(artifact)]
    pub buildings: Option<Vec<En1998Building>>,
    #[state(artifact)]
    pub bridges: Option<Vec<En1998Bridge>>,
    #[state(artifact)]
    pub assessments: Option<Vec<En1998Assessment>>,
    #[state(artifact)]
    pub silos: Option<Vec<En1998Silo>>,
    #[state(artifact)]
    pub tanks: Option<Vec<En1998Tank>>,
    #[state(artifact)]
    pub foundations: Option<Vec<En1998Foundation>>,
    #[state(artifact)]
    pub retaining_walls: Option<Vec<En1998RetainingWall>>,
    #[state(artifact)]
    pub towers: Option<Vec<En1998Tower>>,
}
