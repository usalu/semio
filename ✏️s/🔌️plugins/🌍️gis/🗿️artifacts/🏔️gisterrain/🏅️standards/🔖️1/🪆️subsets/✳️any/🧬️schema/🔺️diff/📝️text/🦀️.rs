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
    /// 🧬️ Applies sparse document changes to the artifact.
    pub fn apply_to_artifact(&self, artifact: &GisTerrainArtifact) -> protocol::MutationApplyResult<GisTerrainArtifact> {
        self.apply(&artifact.to_snapshot()).map(GisTerrainArtifact::from_snapshot)
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
            if let Some(value) = &self.imported_map {
                next.imported_map = value.value.clone();
            }
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
        take!(imported_map);
    }
}
//#endregion 🔹Apply

//#region 🔹Helpers
/// ⚡️ Diff helpers used by mutations.
pub fn diff_exaggeration(exaggeration: f64) -> GisTerrainDiff {
    GisTerrainDiff { exaggeration: Some(exaggeration), ..Default::default() }
}

pub fn diff_imported_map(value: Option<crate::schema::ImportedMap>) -> GisTerrainDiff {
    GisTerrainDiff { imported_map: Some(ImportedMapChange{value}), ..Default::default() }
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

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type GisTerrainDiffText = String;
//#endregion 🚚️Carrier
