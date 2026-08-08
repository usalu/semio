//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


pub use super::schema::*;

use crate::artifacts::gisterrain::schema::GisTerrainArtifact;
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use protocol::MutationDiff;

//#region 🔹Apply
impl GisTerrainDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &GisTerrainArtifact) -> GisTerrainArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(value) = self.exaggeration {
            next.exaggeration = value;
        }
        if let Some(value) = &self.imported_features_json {
            next.imported_features_json = value.clone();
        }
        if let Some(list) = &self.selected_ids {
            next.selected_ids = list.values.clone();
        }
        if let Some(value) = &self.camera_json {
            next.camera_json = value.clone();
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        next
    }
}

impl MutationDiff<GisTerrainSnapshot> for GisTerrainDiff {
    fn apply(&self, snapshot: &GisTerrainSnapshot) -> GisTerrainSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(value) = self.exaggeration {
            next.exaggeration = value;
        }
        if let Some(value) = &self.imported_features_json {
            next.imported_features_json = value.clone();
        }
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
        take!(exaggeration);
        take!(imported_features_json);
        take!(selected_ids);
        take!(camera_json);
        take!(locale);
    }
}
//#endregion 🔹Apply

//#region 🔹Helpers
/// ⚡️ Diff helpers used by mutations.
pub fn diff_exaggeration(exaggeration: f64) -> GisTerrainDiff {
    GisTerrainDiff { exaggeration: Some(exaggeration), ..Default::default() }
}

pub fn diff_imported_features_json(features_json: String) -> GisTerrainDiff {
    GisTerrainDiff { imported_features_json: Some(features_json), ..Default::default() }
}

pub fn diff_set_snapshot(snapshot: &GisTerrainSnapshot) -> GisTerrainDiff {
    GisTerrainDiff {
        artifact: Some(Box::new(GisTerrainArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔹Helpers

//#region 🔹Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_diffs_absorb_last_writer_wins_and_apply_onto_the_snapshot() {
        let base = GisTerrainSnapshot { exaggeration: 1.0, imported_features_json: String::new() };
        let mut diff = GisTerrainDiff { exaggeration: Some(2.0), ..Default::default() };
        diff.absorb(GisTerrainDiff { exaggeration: Some(3.0), imported_features_json: Some("null".into()), ..Default::default() });
        let next = diff.apply(&base);
        assert_eq!(next.exaggeration, 3.0);
        assert_eq!(next.imported_features_json, "null");
    }

    #[test]
    fn a_whole_artifact_diff_wins_over_every_field_diff() {
        let base = GisTerrainSnapshot { exaggeration: 1.0, imported_features_json: String::new() };
        let replacement = GisTerrainSnapshot { exaggeration: 9.0, imported_features_json: "{}".into() };
        let mut diff = GisTerrainDiff { exaggeration: Some(2.0), ..Default::default() };
        diff.absorb(diff_set_snapshot(&replacement));
        assert_eq!(diff.apply(&base), replacement);
    }
}
//#endregion 🔹Tests
