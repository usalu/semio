//! 🏔️ Replace the terrain document's authored elevation source.

use crate::diff::{ElevationSourceChange, GisTerrainDiff};
use crate::mutations::GisTerrainMutation;
use crate::schema::snapshot::ElevationSource;
use crate::GisTerrainSnapshot;
use protocol::{ColdMutationKind, MutationKind, MutationOutcome, SemanticDescriptor};

/// 📥️ Replace an authored source or clear it to restore the bundled fallback.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-elevation-source")]
pub struct ReplaceElevationSource {
    #[dsl(key = "new-elevation-source")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub new_elevation_source: Option<ElevationSource>,
}

impl MutationKind<GisTerrainSnapshot, GisTerrainMutation> for ReplaceElevationSource {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor {
        verb: "replace", entity: "elevation-source", kind: "replace-elevation-source", record: "ReplacedElevationSource",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Replace elevation source", "Höhenquelle ersetzen")
    }
}

impl ColdMutationKind<GisTerrainSnapshot, GisTerrainMutation> for ReplaceElevationSource {
    fn diff(&self, base: &GisTerrainSnapshot) -> MutationOutcome<GisTerrainDiff> {
        if self.new_elevation_source.as_ref().is_some_and(|source| !source.is_valid()) {
            return MutationOutcome::error("terrain.elevation-source.invalid", "Elevation requires ordered WGS84 bounds and exactly 33 rows of 33 samples.", ["elevationSource"]);
        }
        if self.new_elevation_source == base.elevation_source {
            return MutationOutcome::empty().warning("mutation.no-op", "The authored elevation source is already identical.");
        }
        MutationOutcome::new(GisTerrainDiff {
            elevation_source: Some(ElevationSourceChange { value: self.new_elevation_source.clone() }),
            ..Default::default()
        })
    }

    fn inverse(&self, base: &GisTerrainSnapshot) -> Result<Vec<GisTerrainMutation>, semio_framework_value::ValueError> {
        Ok(vec![GisTerrainMutation::ReplaceElevationSource(Self { new_elevation_source: base.elevation_source.clone() })])
    }
}
