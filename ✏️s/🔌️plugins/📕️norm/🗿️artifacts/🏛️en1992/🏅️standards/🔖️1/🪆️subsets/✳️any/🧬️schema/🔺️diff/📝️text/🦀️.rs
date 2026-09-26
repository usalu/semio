//! 🔺️ En1992 artifact — sparse field diff runtime.

use crate::artifact_schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifact_schema::En1992Artifact;
use crate::En1992Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
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
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &En1992Snapshot) -> En1992Diff {
    En1992Diff { artifact: Some(Box::new(En1992Artifact::from_snapshot(snapshot))), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type En1992DiffText = String;
//#endregion 🚚️Carrier
