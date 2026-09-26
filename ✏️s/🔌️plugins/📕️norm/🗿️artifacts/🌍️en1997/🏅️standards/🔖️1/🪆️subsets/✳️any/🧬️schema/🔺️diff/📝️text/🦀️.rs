//! 🔺️ En1997 artifact — sparse field diff runtime.

use crate::artifact_schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifact_schema::En1997Artifact;
use crate::En1997Snapshot;
use protocol::MutationDiff;

macro_rules! apply_scalar {
    ($self:ident, $next:ident, $($field:ident),+ $(,)?) => {
        $(
            if let Some(value) = &$self.$field {
                $next.$field = value.clone();
            }
        )+
    };
}

//#region 🔖️Apply
impl En1997Diff {
    pub fn apply_to_artifact(&self, artifact: &En1997Artifact) -> protocol::MutationApplyResult<En1997Artifact> {
        if let Some(replacement) = &self.artifact {
            return Ok((**replacement).clone());
        }
        let mut next = artifact.clone();
        apply_scalar!(self, next, structure_id, geotechnical_category, design_situation, design_approach, annex, groundwater_level, investigation_depth);
        if let Some(list) = &self.layers { next.layers = list.values.clone(); }
        if let Some(list) = &self.footings { next.footings = list.values.clone(); }
        if let Some(list) = &self.piles { next.piles = list.values.clone(); }
        if let Some(list) = &self.retaining_walls { next.retaining_walls = list.values.clone(); }
        if let Some(list) = &self.slopes { next.slopes = list.values.clone(); }
        if let Some(list) = &self.uplift_cases { next.uplift_cases = list.values.clone(); }
        Ok(next)
    }
}

impl MutationDiff<En1997Snapshot> for En1997Diff {
    fn apply(&self, snapshot: &En1997Snapshot) -> protocol::MutationApplyResult<En1997Snapshot> {
        if let Some(replacement) = &self.artifact {
            return Ok(replacement.to_snapshot());
        }
        let mut next = snapshot.clone();
        apply_scalar!(self, next, structure_id, geotechnical_category, design_situation, design_approach, annex, groundwater_level, investigation_depth);
        if let Some(list) = &self.layers { next.layers = list.values.clone(); }
        if let Some(list) = &self.footings { next.footings = list.values.clone(); }
        if let Some(list) = &self.piles { next.piles = list.values.clone(); }
        if let Some(list) = &self.retaining_walls { next.retaining_walls = list.values.clone(); }
        if let Some(list) = &self.slopes { next.slopes = list.values.clone(); }
        if let Some(list) = &self.uplift_cases { next.uplift_cases = list.values.clone(); }
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
        take!(structure_id);
        take!(geotechnical_category);
        take!(design_situation);
        take!(design_approach);
        take!(annex);
        take!(groundwater_level);
        take!(investigation_depth);
        take!(layers);
        take!(footings);
        take!(piles);
        take!(retaining_walls);
        take!(slopes);
        take!(uplift_cases);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &En1997Snapshot) -> En1997Diff {
    En1997Diff { artifact: Some(Box::new(En1997Artifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
pub type En1997DiffText = String;
//#endregion 🚚️Carrier
