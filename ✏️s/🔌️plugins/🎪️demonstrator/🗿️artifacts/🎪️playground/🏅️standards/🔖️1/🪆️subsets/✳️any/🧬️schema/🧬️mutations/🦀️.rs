//! 🧬️ Transparent playground semantic mutation aggregate.

use crate::standards::v1::subsets::any::schema::diff::PlaygroundDiff;
use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;

pub use super::change_schema::{apply_playground_mutation_json, round_trip_playground_dsl, undo_playground_mutation_json, ChangeSchema, KINDS};

//#region 🔖️Aggregate
/// 🧬️ Closed semantic mutation vocabulary for a playground document.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = PlaygroundSnapshot, diff = PlaygroundDiff, schema = "s.demonstrator.playground")]
pub enum PlaygroundMutation {
    ChangeSchema(ChangeSchema),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
