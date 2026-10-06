//! 🧬️ EN 1999 diff schema — sparse field delta over the aluminium-structure subject.

use framework_schema::ArtifactSchema;

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1999")]
pub struct En1999Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifact_schema::En1999Artifact>>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub materials: Option<Vec<crate::snapshot::AluminiumMaterial>>,
    #[state(artifact)]
    pub sections: Option<Vec<crate::snapshot::AluminiumSection>>,
    #[state(artifact)]
    pub members: Option<Vec<crate::snapshot::AluminiumMember>>,
    #[state(artifact)]
    pub connections: Option<Vec<crate::snapshot::AluminiumConnection>>,
    #[state(artifact)]
    pub fire_scenarios: Option<Vec<crate::snapshot::FireScenario>>,
    #[state(artifact)]
    pub fatigue_details: Option<Vec<crate::snapshot::FatigueDetail>>,
    #[state(artifact)]
    pub cold_formed: Option<Vec<crate::snapshot::ColdFormedSheet>>,
    #[state(artifact)]
    pub shells: Option<Vec<crate::snapshot::AluminiumShell>>,
}

//#endregion 🔖️Diff

use crate::artifact_schema::diff::*;
use crate::artifact_schema::En1999Artifact;
use crate::En1999Snapshot;
use protocol::MutationDiff;

impl En1999Diff {
    pub fn apply_to_artifact(&self, artifact: &En1999Artifact) -> protocol::MutationApplyResult<En1999Artifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(value) = &self.annex { next.annex = *value; }
            if let Some(value) = &self.materials { next.materials = value.clone(); }
            if let Some(value) = &self.sections { next.sections = value.clone(); }
            if let Some(value) = &self.members { next.members = value.clone(); }
            if let Some(value) = &self.connections { next.connections = value.clone(); }
            if let Some(value) = &self.fire_scenarios { next.fire_scenarios = value.clone(); }
            if let Some(value) = &self.fatigue_details { next.fatigue_details = value.clone(); }
            if let Some(value) = &self.cold_formed { next.cold_formed = value.clone(); }
            if let Some(value) = &self.shells { next.shells = value.clone(); }
            next
        })
    }
}

impl MutationDiff<En1999Snapshot> for En1999Diff {
    fn apply(&self, snapshot: &En1999Snapshot) -> protocol::MutationApplyResult<En1999Snapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(value) = &self.annex { next.annex = *value; }
            if let Some(value) = &self.materials { next.materials = value.clone(); }
            if let Some(value) = &self.sections { next.sections = value.clone(); }
            if let Some(value) = &self.members { next.members = value.clone(); }
            if let Some(value) = &self.connections { next.connections = value.clone(); }
            if let Some(value) = &self.fire_scenarios { next.fire_scenarios = value.clone(); }
            if let Some(value) = &self.fatigue_details { next.fatigue_details = value.clone(); }
            if let Some(value) = &self.cold_formed { next.cold_formed = value.clone(); }
            if let Some(value) = &self.shells { next.shells = value.clone(); }
            next
        })
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
        take!(materials);
        take!(sections);
        take!(members);
        take!(connections);
        take!(fire_scenarios);
        take!(fatigue_details);
        take!(cold_formed);
        take!(shells);
    }
}

pub fn diff_set_snapshot(snapshot: &En1999Snapshot) -> En1999Diff {
    En1999Diff { artifact: Some(Box::new(En1999Artifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
