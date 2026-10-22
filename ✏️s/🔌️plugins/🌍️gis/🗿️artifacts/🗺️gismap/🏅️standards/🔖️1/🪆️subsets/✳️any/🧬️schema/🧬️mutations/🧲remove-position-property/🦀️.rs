//! 🧲 `remove-position-property` mutation payload — removes one property from a position feature's payload object.
//! The property edit is addressed by key inside the position's opaque payload object; the diff carries that one property
//! only, never the whole payload (`replace-position-data` stays the whole-payload gesture).

use crate::diff::GisMapDiff;
use crate::GisMapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🧲 Removes `key` from the payload of the `positions` entry addressed by `feature`. Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-position-property")]
pub struct RemovePositionProperty {
    pub feature: String,
    pub key: String,
}

impl<Op> MutationKind<GisMapSnapshot, Op> for RemovePositionProperty
where
    Op: protocol::Mutation<GisMapSnapshot, Diff = GisMapDiff> + From<RemovePositionProperty> + From<crate::standards::v1::subsets::any::schema::mutations::set_position_property::SetPositionProperty>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "position-property", kind: "remove-position-property", record: "PositionPropertyRemoved" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove property \"{}\" of position \"{}\"", self.key, self.feature), &format!("Eigenschaft \"{}\" von Position \"{}\" entfernen", self.key, self.feature))
    }
}

impl<Op> protocol::ColdMutationKind<GisMapSnapshot, Op> for RemovePositionProperty
where
    Op: protocol::ColdMutation<GisMapSnapshot, ReferenceDiff = GisMapDiff> + From<RemovePositionProperty> + From<crate::standards::v1::subsets::any::schema::mutations::set_position_property::SetPositionProperty>,
{
    fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &GisMapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
//#endregion 🔹Payload
