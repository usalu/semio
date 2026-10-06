//! 🧬️ GIS terrain diff schema — sparse field delta over the artifact.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔹Diff
/// 🔺️ Sparse field delta for the GIS terrain artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.gis.gisterrain")]
pub struct GisTerrainDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::GisTerrainArtifact>>,
    #[state(artifact)]
    pub exaggeration: Option<f64>,
    #[state(artifact)]
    pub imported_map: Option<ImportedMapChange>,
}
/// 🔄️ A present replacement can explicitly clear the optional imported map owner.
#[derive(Clone,Debug,Default,PartialEq,ToValue,FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ImportedMapChange{
    #[value(default,skip_serializing_if="Option::is_none")]
    pub value:Option<crate::schema::ImportedMap>,
}
//#endregion 🔹Diff

use crate::schema::GisTerrainArtifact;
use crate::GisTerrainSnapshot;
use protocol::MutationDiff;

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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
