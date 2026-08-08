//! 🔺️ Iso16757 artifact — sparse field diff runtime.

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

use crate::artifacts::iso16757::schema::Iso16757Artifact;
use crate::artifacts::iso16757::Iso16757Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl Iso16757Diff {
    pub fn apply_to_artifact(&self, artifact: &Iso16757Artifact) -> Iso16757Artifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(value) = self.catalogue { next.catalogue = value; }
        if let Some(value) = self.dictionary { next.dictionary = value; }
        if let Some(value) = self.geometry { next.geometry = value; }
        if let Some(value) = self.selection { next.selection = value; }
        if let Some(value) = self.part_number_rule { next.part_number_rule = value; }
        if let Some(value) = self.part_number_inputs { next.part_number_inputs = value; }
        if let Some(value) = self.script_limits { next.script_limits = value; }
        if let Some(value) = self.exchange_process { next.exchange_process = value; }
        if let Some(value) = &self.selected_check_index {
            next.selected_check_index = *value;
        }
        next
    }
}

impl MutationDiff<Iso16757Snapshot> for Iso16757Diff {
    fn apply(&self, snapshot: &Iso16757Snapshot) -> Iso16757Snapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(value) = self.catalogue { next.catalogue = value; }
        if let Some(value) = self.dictionary { next.dictionary = value; }
        if let Some(value) = self.geometry { next.geometry = value; }
        if let Some(value) = self.selection { next.selection = value; }
        if let Some(value) = self.part_number_rule { next.part_number_rule = value; }
        if let Some(value) = self.part_number_inputs { next.part_number_inputs = value; }
        if let Some(value) = self.script_limits { next.script_limits = value; }
        if let Some(value) = self.exchange_process { next.exchange_process = value; }
        next
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
        take!(catalogue);
        take!(dictionary);
        take!(geometry);
        take!(selection);
        take!(part_number_rule);
        take!(part_number_inputs);
        take!(script_limits);
        take!(exchange_process);
        take!(selected_check_index);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &Iso16757Snapshot) -> Iso16757Diff {
    Iso16757Diff {
        artifact: Some(Box::new(Iso16757Artifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔖️Helpers
