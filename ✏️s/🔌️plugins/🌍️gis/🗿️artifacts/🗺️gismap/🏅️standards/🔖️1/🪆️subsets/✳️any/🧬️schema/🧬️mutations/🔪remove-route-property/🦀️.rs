//! 🔪 `remove-route-property` mutation payload — removes one property from a route feature's payload object.
//! The property edit is addressed by key inside the route's opaque payload object; the diff carries that one property
//! only, never the whole payload (`replace-route-data` stays the whole-payload gesture).

use crate::diff::GisMapDiff;
use crate::GisMapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🔪 Removes `key` from the payload of the `routes` entry addressed by `feature`. Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-route-property")]
pub struct RemoveRouteProperty {
    pub feature: String,
    pub key: String,
}

impl<Op> MutationKind<GisMapSnapshot, Op> for RemoveRouteProperty
where
    Op: protocol::Mutation<GisMapSnapshot, Diff = GisMapDiff> + From<RemoveRouteProperty> + From<crate::standards::v1::subsets::any::schema::mutations::set_route_property::SetRouteProperty>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "route-property", kind: "remove-route-property", record: "RoutePropertyRemoved" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove property \"{}\" of route \"{}\"", self.key, self.feature), &format!("Eigenschaft \"{}\" von Route \"{}\" entfernen", self.key, self.feature))
    }
}

impl<Op> protocol::ColdMutationKind<GisMapSnapshot, Op> for RemoveRouteProperty
where
    Op: protocol::ColdMutation<GisMapSnapshot, ReferenceDiff = GisMapDiff> + From<RemoveRouteProperty> + From<crate::standards::v1::subsets::any::schema::mutations::set_route_property::SetRouteProperty>,
{
    fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &GisMapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
//#endregion 🔹Payload
