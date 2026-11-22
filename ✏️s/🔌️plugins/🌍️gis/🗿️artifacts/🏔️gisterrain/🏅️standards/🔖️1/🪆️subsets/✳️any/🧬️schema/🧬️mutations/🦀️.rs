//! 🏔️ GIS terrain semantic mutation aggregate.

use crate::diff::GisTerrainDiff;
use crate::GisTerrainSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

pub use super::change_exaggeration::ChangeExaggeration;
pub use super::change_imported_features::ChangeImportedFeatures;
pub use super::replace_elevation_source::ReplaceElevationSource;
pub use crate::schema::operations::*;

#[path = "🎯️history-target/🦀️.rs"]
mod history_target;

//#region 🔖️Aggregate
/// 🗺️ Typed terrain mutation vocabulary backed by direct semantic owners.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = GisTerrainSnapshot, diff = GisTerrainDiff, schema = "gis.gisterrain", reference = GisTerrainSnapshot, reference_diff = GisTerrainDiff)]
pub enum GisTerrainMutation {
    ChangeExaggeration(ChangeExaggeration),
    ChangeImportedFeatures(ChangeImportedFeatures),
    ReplaceElevationSource(ReplaceElevationSource),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
