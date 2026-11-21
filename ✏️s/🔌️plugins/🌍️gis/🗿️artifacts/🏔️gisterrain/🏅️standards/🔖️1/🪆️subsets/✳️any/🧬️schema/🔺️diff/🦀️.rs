//! 🧬️ GIS terrain diff schema — sparse field delta over the artifact.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::schema::snapshot::ElevationSource;

//#region 🔹Diff
/// 🔺️ Sparse field delta for the GIS terrain artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.gis.gisterrain")]
pub struct GisTerrainDiff {
    #[state(artifact)]
    pub exaggeration: Option<f64>,
    #[state(artifact)]
    pub imported_map: Option<ImportedMapChange>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub elevation_source: Option<ElevationSourceChange>,
}
/// 🔄️ A present replacement can explicitly clear the optional imported map owner.
#[derive(Clone,Debug,Default,PartialEq,ToValue,FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ImportedMapChange{
    #[value(default,skip_serializing_if="Option::is_none")]
    pub value:Option<crate::schema::imported_map::ImportedMap>,
}

/// 🏔️ A present replacement may clear the authored source.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(deny_unknown_fields)]
pub struct ElevationSourceChange {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<ElevationSource>,
}
//#endregion 🔹Diff

use crate::GisTerrainSnapshot;
use protocol::MutationDiff;

impl MutationDiff<GisTerrainSnapshot> for GisTerrainDiff {}

impl protocol::ColdMutationDiff<GisTerrainSnapshot> for GisTerrainDiff {
    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(exaggeration);
        take!(imported_map);
        take!(elevation_source);
    }

    fn apply(&self, unpublished: &mut GisTerrainSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = self.exaggeration {
            unpublished.exaggeration = value;
        }
        if let Some(value) = &self.imported_map {
            unpublished.imported_map = value.value.clone();
        }
        if let Some(value) = &self.elevation_source {
            unpublished.elevation_source = value.value.clone();
        }
        Ok(())
    }
}

impl protocol::DiffAlgebra<GisTerrainSnapshot> for GisTerrainDiff {
    fn is_empty(&self) -> bool {
        self.exaggeration.is_none() && self.imported_map.is_none() && self.elevation_source.is_none()
    }
}

impl protocol::ColdDiffAlgebra<GisTerrainSnapshot> for GisTerrainDiff {
    fn inverse(&self, base: &GisTerrainSnapshot) -> Self {
        Self {
            exaggeration: self.exaggeration.map(|_| base.exaggeration),
            imported_map: self.imported_map.as_ref().map(|_| ImportedMapChange { value: base.imported_map.clone() }),
            elevation_source: self.elevation_source.as_ref().map(|_| ElevationSourceChange { value: base.elevation_source.clone() }),
        }
    }
}

/// ⚡️ Diff helpers used by mutations.
pub fn diff_exaggeration(exaggeration: f64) -> GisTerrainDiff {
    GisTerrainDiff { exaggeration: Some(exaggeration), ..Default::default() }
}

pub fn diff_imported_map(value: Option<crate::schema::imported_map::ImportedMap>) -> GisTerrainDiff {
    GisTerrainDiff { imported_map: Some(ImportedMapChange{value}), ..Default::default() }
}


