//! 🔺️ EN 1995 artifact — sparse field diff runtime.

use crate::artifact_schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifact_schema::En1995Artifact;
use crate::En1995Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
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
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
