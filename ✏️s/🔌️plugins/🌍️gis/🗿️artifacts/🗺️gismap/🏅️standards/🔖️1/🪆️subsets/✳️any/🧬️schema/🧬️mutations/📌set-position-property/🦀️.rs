//! 📌 `set-position-property` mutation payload — sets one property of a position feature's payload object.
//! The property edit is addressed by key inside the position's opaque payload object; the diff carries that one property
//! only, never the whole payload (`replace-position-data` stays the whole-payload gesture).

use crate::diff::GisMapDiff;
use crate::mutations::GisMapMutation;
use crate::GisMapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 📌 Sets `key` to `value` in the payload of the `positions` entry addressed by `feature` (in place when the key exists, otherwise inserted before `before`, appended without one). Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-position-property")]
pub struct SetPositionProperty {
    pub feature: String,
    pub key: String,
    pub value: semio_framework_value::DslValue,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub before: Option<String>,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for SetPositionProperty {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "position-property", kind: "set-position-property", record: "PositionPropertySet" };

    fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisMapSnapshot) -> Result<Vec<GisMapMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set property \"{}\" of position \"{}\"", self.key, self.feature), &format!("Eigenschaft \"{}\" von Position \"{}\" setzen", self.key, self.feature))
    }
}
//#endregion 🔹Payload
