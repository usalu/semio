//! 🧬️ EN 1995 diff schema — sparse field delta.

use framework_schema::ArtifactSchema;

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1995")]
pub struct En1995Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifact_schema::En1995Artifact>>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub members: Option<En1995MemberList>,
    #[state(artifact)]
    pub connections: Option<En1995ConnectionList>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1995MemberList {
    pub values: Vec<crate::TimberMember>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1995ConnectionList {
    pub values: Vec<crate::TimberConnection>,
}
//#endregion 🔖️DeltaHelpers

use crate::artifact_schema::diff::*;
use crate::artifact_schema::En1995Artifact;
use crate::En1995Snapshot;
use protocol::MutationDiff;

impl En1995Diff {
    pub fn apply_to_artifact(&self, artifact: &En1995Artifact) -> protocol::MutationApplyResult<En1995Artifact> {
        if let Some(replacement) = &self.artifact {
            return Ok((**replacement).clone());
        }
        let mut next = artifact.clone();
        if let Some(value) = &self.annex { next.annex = *value; }
        if let Some(list) = &self.members { next.members = list.values.clone(); }
        if let Some(list) = &self.connections { next.connections = list.values.clone(); }
        Ok(next)
    }
}

impl MutationDiff<En1995Snapshot> for En1995Diff {
    fn apply(&self, snapshot: &En1995Snapshot) -> protocol::MutationApplyResult<En1995Snapshot> {
        if let Some(replacement) = &self.artifact {
            return Ok(replacement.to_snapshot());
        }
        let mut next = snapshot.clone();
        if let Some(value) = &self.annex { next.annex = *value; }
        if let Some(list) = &self.members { next.members = list.values.clone(); }
        if let Some(list) = &self.connections { next.connections = list.values.clone(); }
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
        take!(annex);
        take!(members);
        take!(connections);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
