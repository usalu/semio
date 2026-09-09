//! 🏔️ GIS terrain semantic mutation aggregate.

use crate::diff::GisTerrainDiff;
use crate::GisTerrainSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

pub use super::change_exaggeration::ChangeExaggeration;
pub use super::change_imported_features::ChangeImportedFeatures;
pub use crate::schema::operations::*;

//#region 🔖️Aggregate
/// 🗺️ Typed terrain mutation vocabulary backed by direct semantic owners.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum, dsl::Mutations, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutations(snapshot = GisTerrainSnapshot, diff = GisTerrainDiff, schema = "gis.gisterrain")]
pub enum GisTerrainMutation {
    ChangeExaggeration(ChangeExaggeration),
    ChangeImportedFeatures(ChangeImportedFeatures),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
