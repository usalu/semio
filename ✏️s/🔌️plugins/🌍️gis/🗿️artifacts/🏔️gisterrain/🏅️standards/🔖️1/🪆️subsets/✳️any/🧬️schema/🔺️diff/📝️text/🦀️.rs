use crate::schema::diff::*;
//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::GisTerrainArtifact;
use crate::GisTerrainSnapshot;
use protocol::MutationDiff;

//#region 🔹Apply
impl GisTerrainDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &GisTerrainArtifact) -> protocol::MutationApplyResult<GisTerrainArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(value) = self.exaggeration {
                next.exaggeration = value;
            }
            if let Some(value) = &self.imported_features_json {
                next.imported_features_json = value.clone();
            }
            if let Some(value) = &self.camera_json {
                next.camera_json = value.clone();
            }
            if let Some(value) = &self.locale {
                next.locale = value.clone();
            }
            next
        })
    }
}

impl MutationDiff<GisTerrainSnapshot> for GisTerrainDiff {
    fn apply(&self, snapshot: &GisTerrainSnapshot) -> protocol::MutationApplyResult<GisTerrainSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(value) = self.exaggeration {
                next.exaggeration = value;
            }
            if let Some(value) = &self.imported_features_json {
                next.imported_features_json = value.clone();
            }
            // 🕸️ Keep `mesh` a pure function of the two fields above — mirrors
            // `apply_gis_terrain_mutation`'s identical re-derivation (see `GisTerrainSnapshot.mesh`'s doc).
            next = crate::gis_terrain_snapshot_with_derived_mesh(next);
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
        take!(exaggeration);
        take!(imported_features_json);
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
    GisTerrainDiff { artifact: Some(Box::new(GisTerrainArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
//#endregion 🔹Helpers

//#region 🔹Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔹Tests
