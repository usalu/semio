//! 🧨 `remove-region-property` mutation payload — removes one property from a region feature's payload object.
//! The property edit is addressed by key inside the region's opaque payload object; the diff carries that one property
//! only, never the whole payload (`replace-region-data` stays the whole-payload gesture).

use crate::diff::GisMapDiff;
use crate::mutations::GisMapMutation;
use crate::GisMapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🧨 Removes `key` from the payload of the `regions` entry addressed by `feature`. Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-region-property")]
pub struct RemoveRegionProperty {
    pub feature: String,
    pub key: String,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for RemoveRegionProperty {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "region-property", kind: "remove-region-property", record: "RegionPropertyRemoved" };

    fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisMapSnapshot) -> Result<Vec<GisMapMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove property \"{}\" of region \"{}\"", self.key, self.feature), &format!("Eigenschaft \"{}\" von Region \"{}\" entfernen", self.key, self.feature))
    }
}
//#endregion 🔹Payload
