//! 🧬️ Transparent playground semantic mutation aggregate.

use crate::standards::v1::subsets::any::schema::diff::PlaygroundDiff;
use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;

pub use super::change_schema::{ChangeSchema, KINDS};

//#region 🔖️Aggregate
/// 🧬️ Closed semantic mutation vocabulary for a playground document.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = PlaygroundSnapshot, diff = PlaygroundDiff, schema = "s.demonstrator.playground", reference = PlaygroundSnapshot, reference_diff = PlaygroundDiff)]
pub enum PlaygroundMutation {
    ChangeSchema(ChangeSchema),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence

//#endregion 🧪️StructuralCorrespondence
