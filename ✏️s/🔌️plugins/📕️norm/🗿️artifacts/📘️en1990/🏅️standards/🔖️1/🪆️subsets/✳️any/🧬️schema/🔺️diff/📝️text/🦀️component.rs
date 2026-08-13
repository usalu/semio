//! 🔺️ En1990 artifact — sparse field diff runtime.

use crate::artifacts::en1990::schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1990::schema::En1990Artifact;
use crate::artifacts::en1990::En1990Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl En1990Diff {
    pub fn apply_to_artifact(&self, artifact: &En1990Artifact) -> En1990Artifact {
        let mut next = artifact.clone();
        if let Some(value) = &self.g_k { next.g_k = value.clone(); }
        if let Some(child) = &self.q_k { next.q_k = child.clone(); }
        if let Some(value) = &self.resistance_kn { next.resistance_kn = value.clone(); }
        if let Some(value) = &self.consequence_class { next.consequence_class = value.clone(); }
        if let Some(value) = &self.annex { next.annex = value.clone(); }
        if let Some(value) = &self.seismic_a_ed_kn { next.seismic_a_ed_kn = value.clone(); }
        if let Some(value) = &self.selected_check_index {
            next.selected_check_index = *value;
        }
        next
    }
}

impl MutationDiff<En1990Snapshot> for En1990Diff {
    fn apply(&self, snapshot: &En1990Snapshot) -> En1990Snapshot {
        let mut next = snapshot.clone();
        if let Some(value) = &self.g_k { next.g_k = value.clone(); }
        if let Some(child) = &self.q_k { next.q_k = child.clone(); }
        if let Some(value) = &self.resistance_kn { next.resistance_kn = value.clone(); }
        if let Some(value) = &self.consequence_class { next.consequence_class = value.clone(); }
        if let Some(value) = &self.annex { next.annex = value.clone(); }
        if let Some(value) = &self.seismic_a_ed_kn { next.seismic_a_ed_kn = value.clone(); }
        next
    }

    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(g_k);
        take!(q_k);
        take!(resistance_kn);
        take!(consequence_class);
        take!(annex);
        take!(seismic_a_ed_kn);
        take!(selected_check_index);
    }
}
//#endregion 🔖️Apply
