//! 🧬️ En1994 diff schema — sparse field delta over the composite structure subject.

use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1994 artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994StringList {
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994BeamList {
    pub values: Vec<crate::CompositeBeam>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994ColumnList {
    pub values: Vec<crate::CompositeColumn>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994SlabList {
    pub values: Vec<crate::CompositeSlab>,
}
//#endregion 🔖️DeltaHelpers

use crate::artifact_schema::diff::*;
use crate::artifact_schema::En1994Artifact;
use crate::En1994Snapshot;
use protocol::MutationDiff;

impl En1994Diff {
    pub fn apply_to_artifact(&self, artifact: &En1994Artifact) -> protocol::MutationApplyResult<En1994Artifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(value) = &self.annex { next.annex = *value; }
            if let Some(value) = &self.structure_kind { next.structure_kind = value.clone(); }
            if let Some(value) = &self.steel_f_y_pa { next.steel_f_y_pa = *value; }
            if let Some(list) = &self.beams { next.beams = list.values.clone(); }
            if let Some(list) = &self.columns { next.columns = list.values.clone(); }
            if let Some(list) = &self.slabs { next.slabs = list.values.clone(); }
            if let Some(value) = &self.fire_rating { next.fire_rating = value.clone(); }
            if let Some(value) = &self.insulation_thickness_m { next.insulation_thickness_m = *value; }
            if let Some(value) = &self.fatigue_detail { next.fatigue_detail = value.clone(); }
            next
        })
    }
}

impl MutationDiff<En1994Snapshot> for En1994Diff {
    fn apply(&self, snapshot: &En1994Snapshot) -> protocol::MutationApplyResult<En1994Snapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(value) = &self.annex { next.annex = *value; }
            if let Some(value) = &self.structure_kind { next.structure_kind = value.clone(); }
            if let Some(value) = &self.steel_f_y_pa { next.steel_f_y_pa = *value; }
            if let Some(list) = &self.beams { next.beams = list.values.clone(); }
            if let Some(list) = &self.columns { next.columns = list.values.clone(); }
            if let Some(list) = &self.slabs { next.slabs = list.values.clone(); }
            if let Some(value) = &self.fire_rating { next.fire_rating = value.clone(); }
            if let Some(value) = &self.insulation_thickness_m { next.insulation_thickness_m = *value; }
            if let Some(value) = &self.fatigue_detail { next.fatigue_detail = value.clone(); }
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
        take!(structure_kind);
        take!(steel_f_y_pa);
        take!(beams);
        take!(columns);
        take!(slabs);
        take!(fire_rating);
        take!(insulation_thickness_m);
        take!(fatigue_detail);
    }
}

pub fn diff_set_snapshot(snapshot: &En1994Snapshot) -> En1994Diff {
    En1994Diff { artifact: Some(Box::new(En1994Artifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
