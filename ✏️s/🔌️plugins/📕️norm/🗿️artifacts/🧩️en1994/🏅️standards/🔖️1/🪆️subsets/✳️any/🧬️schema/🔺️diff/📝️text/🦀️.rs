//! 🔺️ En1994 artifact — sparse field diff runtime.

use crate::artifact_schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifact_schema::En1994Artifact;
use crate::En1994Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
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
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &En1994Snapshot) -> En1994Diff {
    En1994Diff { artifact: Some(Box::new(En1994Artifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🚚️Carrier
pub type En1994DiffText = String;
//#endregion 🚚️Carrier
