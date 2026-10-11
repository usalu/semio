//! 🔀️ `reorder-positions` mutation payload — repositions a position feature within `positions`
//! by id (id-keyed collection, so addressing is `id`+`to_index`, not a bare index pair).

use crate::diff::GisMapDiff;
use crate::mutations::GisMapMutation;
use crate::GisMapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🔀️ Moves the `positions` entry addressed by `id` to `to_index`. Diff/inverse delegate to the
/// sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "reorder-positions")]
pub struct ReorderPositions {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for ReorderPositions {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "positions", kind: "reorder-positions", record: "ReorderedPositions" };

    fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisMapSnapshot) -> Result<Vec<GisMapMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Reorder position \"{}\" to {}", self.id, self.to_index), &format!("Position \"{}\" an Stelle {} verschieben", self.id, self.to_index))
    }
}
//#endregion 🔹Payload
