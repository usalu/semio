//! 🔺️ En1999 artifact — sparse field diff runtime.

use crate::artifact_schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifact_schema::En1999Artifact;
use crate::En1999Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
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
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &En1999Snapshot) -> En1999Diff {
    En1999Diff { artifact: Some(Box::new(En1999Artifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion ️Tests

pub type En1999DiffText = String;
