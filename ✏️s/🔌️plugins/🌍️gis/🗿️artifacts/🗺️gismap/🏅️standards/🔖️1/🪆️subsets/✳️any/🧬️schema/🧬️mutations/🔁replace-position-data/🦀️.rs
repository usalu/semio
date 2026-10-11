//! 🔁️ `replace-position-data` mutation payload — whole-value swaps a position
//! feature's opaque payload (`MapFeature::data` is deliberately untyped, so a partial `change`
//! isn't expressible — this is a `replace`, per the taxonomy's "large structured sub-payload" rule).

use crate::diff::GisMapDiff;
use crate::mutations::GisMapMutation;
use crate::GisMapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🔁️ Replaces the `data` payload of the `positions` entry addressed by `id`. Diff/inverse
/// delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-position-data")]
pub struct ReplacePositionData {
    pub id: String,
    pub new_data: semio_framework_value::DslValue,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for ReplacePositionData {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "position-data", kind: "replace-position-data", record: "ReplacedPositionData" };

    fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisMapSnapshot) -> Result<Vec<GisMapMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace position \"{}\" data", self.id), &format!("Daten von Position \"{}\" ersetzen", self.id))
    }
}
//#endregion 🔹Payload
