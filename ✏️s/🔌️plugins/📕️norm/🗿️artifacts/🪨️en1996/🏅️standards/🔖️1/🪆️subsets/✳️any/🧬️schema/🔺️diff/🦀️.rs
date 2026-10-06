//! 🧬️ EN 1996 diff schema — sparse field delta for masonry building subject.

use framework_schema::ArtifactSchema;

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1996WallList {
    pub values: Vec<crate::MasonryWall>,
}

use crate::artifact_schema::diff::*;
use crate::artifact_schema::En1996Artifact;
use crate::En1996Snapshot;
use protocol::MutationDiff;

impl En1996Diff {
    pub fn apply_to_artifact(&self, artifact: &En1996Artifact) -> protocol::MutationApplyResult<En1996Artifact> {
        if let Some(replacement) = &self.artifact {
            return Ok((**replacement).clone());
        }
        let mut next = artifact.clone();
        if let Some(value) = &self.annex { next.annex = *value; }
        if let Some(value) = &self.masonry_class { next.masonry_class = *value; }
        if let Some(value) = &self.design_situation { next.design_situation = *value; }
        if let Some(value) = &self.storeys { next.storeys = *value; }
        if let Some(list) = &self.walls { next.walls = list.values.clone(); }
        Ok(next)
    }
}

impl MutationDiff<En1996Snapshot> for En1996Diff {
    fn apply(&self, snapshot: &En1996Snapshot) -> protocol::MutationApplyResult<En1996Snapshot> {
        if let Some(replacement) = &self.artifact {
            return Ok(replacement.to_snapshot());
        }
        let mut next = snapshot.clone();
        if let Some(value) = &self.annex { next.annex = *value; }
        if let Some(value) = &self.masonry_class { next.masonry_class = *value; }
        if let Some(value) = &self.design_situation { next.design_situation = *value; }
        if let Some(value) = &self.storeys { next.storeys = *value; }
        if let Some(list) = &self.walls { next.walls = list.values.clone(); }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        if other.annex.is_some() { self.annex = other.annex; }
        if other.masonry_class.is_some() { self.masonry_class = other.masonry_class; }
        if other.design_situation.is_some() { self.design_situation = other.design_situation; }
        if other.storeys.is_some() { self.storeys = other.storeys; }
        if other.walls.is_some() { self.walls = other.walls; }
    }
}

pub fn diff_set_snapshot(snapshot: &En1996Snapshot) -> En1996Diff {
    En1996Diff { artifact: Some(Box::new(En1996Artifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
