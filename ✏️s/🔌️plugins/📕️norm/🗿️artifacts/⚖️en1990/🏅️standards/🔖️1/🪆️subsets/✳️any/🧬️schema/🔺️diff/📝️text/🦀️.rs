//! 🔺️ En1990 artifact — sparse field diff runtime.

use crate::artifact_schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifact_schema::En1990Artifact;
use crate::En1990Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl En1990Diff {
    pub fn apply_to_artifact(&self, artifact: &En1990Artifact) -> protocol::MutationApplyResult<En1990Artifact> {
        Ok({
            let mut next = artifact.clone();
            if let Some(value) = &self.annex {
                next.annex = *value;
            }
            if let Some(value) = &self.project_id {
                next.project_id = value.clone();
            }
            if let Some(value) = &self.structure_kind {
                next.structure_kind = value.clone();
            }
            if let Some(value) = self.altitude_m {
                next.altitude_m = value;
            }
            if let Some(value) = &self.consequence_class {
                next.consequence_class = *value;
            }
            if let Some(value) = &self.reliability_class {
                next.reliability_class = *value;
            }
            if let Some(value) = &self.design_working_life_category {
                next.design_working_life_category = *value;
            }
            if let Some(value) = &self.design_working_life_years {
                next.design_working_life_years = *value;
            }
            if let Some(value) = &self.reference_period_years {
                next.reference_period_years = *value;
            }
            if let Some(value) = &self.supervision_level {
                next.supervision_level = value.clone();
            }
            if let Some(value) = &self.inspection_level {
                next.inspection_level = value.clone();
            }
            if let Some(value) = &self.beta_computed {
                next.beta_computed = *value;
            }
            if let Some(value) = &self.permanents {
                next.permanents = value.clone();
            }
            if let Some(value) = &self.variables {
                next.variables = value.clone();
            }
            if let Some(value) = &self.accidentals {
                next.accidentals = value.clone();
            }
            if let Some(value) = &self.seismics {
                next.seismics = value.clone();
            }
            if let Some(value) = &self.members {
                next.members = value.clone();
            }
            if let Some(value) = &self.bridge_sls {
                next.bridge_sls = value.clone();
            }
            if let Some(value) = &self.effects {
                next.effects = value.clone();
            }
            next
        })
    }
}

impl MutationDiff<En1990Snapshot> for En1990Diff {
    fn apply(&self, snapshot: &En1990Snapshot) -> protocol::MutationApplyResult<En1990Snapshot> {
        Ok({
            let mut next = snapshot.clone();
            if let Some(value) = &self.annex {
                next.annex = *value;
            }
            if let Some(value) = &self.project_id {
                next.project_id = value.clone();
            }
            if let Some(value) = &self.structure_kind {
                next.structure_kind = value.clone();
            }
            if let Some(value) = self.altitude_m {
                next.altitude_m = value;
            }
            if let Some(value) = &self.consequence_class {
                next.consequence_class = *value;
            }
            if let Some(value) = &self.reliability_class {
                next.reliability_class = *value;
            }
            if let Some(value) = &self.design_working_life_category {
                next.design_working_life_category = *value;
            }
            if let Some(value) = &self.design_working_life_years {
                next.design_working_life_years = *value;
            }
            if let Some(value) = &self.reference_period_years {
                next.reference_period_years = *value;
            }
            if let Some(value) = &self.supervision_level {
                next.supervision_level = value.clone();
            }
            if let Some(value) = &self.inspection_level {
                next.inspection_level = value.clone();
            }
            if let Some(value) = &self.beta_computed {
                next.beta_computed = *value;
            }
            if let Some(value) = &self.permanents {
                next.permanents = value.clone();
            }
            if let Some(value) = &self.variables {
                next.variables = value.clone();
            }
            if let Some(value) = &self.accidentals {
                next.accidentals = value.clone();
            }
            if let Some(value) = &self.seismics {
                next.seismics = value.clone();
            }
            if let Some(value) = &self.members {
                next.members = value.clone();
            }
            if let Some(value) = &self.bridge_sls {
                next.bridge_sls = value.clone();
            }
            if let Some(value) = &self.effects {
                next.effects = value.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(annex);
        take!(project_id);
        take!(consequence_class);
        take!(reliability_class);
        take!(design_working_life_category);
        take!(design_working_life_years);
        take!(reference_period_years);
        take!(supervision_level);
        take!(inspection_level);
        take!(beta_computed);
        take!(permanents);
        take!(variables);
        take!(accidentals);
        take!(seismics);
        take!(members);
        take!(effects);
    }

}
//#endregion 🔖️Apply
