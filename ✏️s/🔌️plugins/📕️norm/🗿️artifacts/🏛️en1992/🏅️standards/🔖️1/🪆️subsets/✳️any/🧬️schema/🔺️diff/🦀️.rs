//! 🧬️ En1992 sparse diff over the hierarchical structure subject.

use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for En1992.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1992")]
pub struct En1992Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifact_schema::En1992Artifact>>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    pub design_working_life_years: Option<f64>,
    #[state(artifact)]
    pub delta_c_dev: Option<f64>,
    #[state(artifact)]
    pub cement_type: Option<String>,
    #[state(artifact)]
    pub concrete_grades: Option<En1992ConcreteGradeList>,
    #[state(artifact)]
    pub reinforcement_grades: Option<En1992ReinforcementGradeList>,
    #[state(artifact)]
    pub prestress_steels: Option<En1992PrestressSteelList>,
    #[state(artifact)]
    pub members: Option<En1992MemberList>,
    #[state(artifact)]
    pub anchors: Option<En1992AnchorList>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992ConcreteGradeList {
    pub values: Vec<crate::ConcreteGrade>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992ReinforcementGradeList {
    pub values: Vec<crate::ReinforcementGrade>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992PrestressSteelList {
    pub values: Vec<crate::PrestressSteel>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992MemberList {
    pub values: Vec<crate::RcMember>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992AnchorList {
    pub values: Vec<crate::Anchor>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992StringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers

use crate::artifact_schema::diff::*;
use crate::artifact_schema::En1992Artifact;
use crate::En1992Snapshot;
use protocol::MutationDiff;

impl En1992Diff {
    pub fn apply_to_artifact(&self, artifact: &En1992Artifact) -> protocol::MutationApplyResult<En1992Artifact> {
        if let Some(replacement) = &self.artifact {
            return Ok((**replacement).clone());
        }
        let mut next = artifact.clone();
        if let Some(value) = &self.annex {
            next.annex = *value;
        }
        if let Some(value) = &self.title {
            next.title = value.clone();
        }
        if let Some(value) = &self.design_working_life_years {
            next.design_working_life_years = *value;
        }
        if let Some(value) = &self.delta_c_dev {
            next.delta_c_dev = *value;
        }
        if let Some(value) = &self.cement_type {
            next.cement_type = value.clone();
        }
        if let Some(list) = &self.concrete_grades {
            next.concrete_grades = list.values.clone();
        }
        if let Some(list) = &self.reinforcement_grades {
            next.reinforcement_grades = list.values.clone();
        }
        if let Some(list) = &self.prestress_steels {
            next.prestress_steels = list.values.clone();
        }
        if let Some(list) = &self.members {
            next.members = list.values.clone();
        }
        if let Some(list) = &self.anchors {
            next.anchors = list.values.clone();
        }
        Ok(next)
    }
}

impl MutationDiff<En1992Snapshot> for En1992Diff {
    fn apply(&self, snapshot: &En1992Snapshot) -> protocol::MutationApplyResult<En1992Snapshot> {
        if let Some(replacement) = &self.artifact {
            return Ok(replacement.to_snapshot());
        }
        let mut next = snapshot.clone();
        if let Some(value) = &self.annex {
            next.annex = *value;
        }
        if let Some(value) = &self.title {
            next.title = value.clone();
        }
        if let Some(value) = &self.design_working_life_years {
            next.design_working_life_years = *value;
        }
        if let Some(value) = &self.delta_c_dev {
            next.delta_c_dev = *value;
        }
        if let Some(value) = &self.cement_type {
            next.cement_type = value.clone();
        }
        if let Some(list) = &self.concrete_grades {
            next.concrete_grades = list.values.clone();
        }
        if let Some(list) = &self.reinforcement_grades {
            next.reinforcement_grades = list.values.clone();
        }
        if let Some(list) = &self.prestress_steels {
            next.prestress_steels = list.values.clone();
        }
        if let Some(list) = &self.members {
            next.members = list.values.clone();
        }
        if let Some(list) = &self.anchors {
            next.anchors = list.values.clone();
        }
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
        take!(title);
        take!(design_working_life_years);
        take!(delta_c_dev);
        take!(cement_type);
        take!(concrete_grades);
        take!(reinforcement_grades);
        take!(prestress_steels);
        take!(members);
        take!(anchors);
    }
}

pub fn diff_set_snapshot(snapshot: &En1992Snapshot) -> En1992Diff {
    En1992Diff { artifact: Some(Box::new(En1992Artifact::from_snapshot(snapshot))), ..Default::default() }
}
