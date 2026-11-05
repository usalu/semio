//! 🧵 `remove-prestress-steel` mutation leaf: removes one prestressing steel row.

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemovePrestressSteel {
    pub index: usize,
}

impl<Op> protocol::MutationKind<En1992Snapshot, Op> for RemovePrestressSteel where Op: protocol::Mutation<En1992Snapshot, Diff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::insert_prestress_steel::InsertPrestressSteel> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "remove",
        entity: "prestress-steel",
        kind: "remove-prestress-steel",
        record: "RemovedPrestressSteel",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Remove prestressing steel {}", self.index), &format!("Spannstahl {} entfernen", self.index))
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<En1992Snapshot, Op> for RemovePrestressSteel where Op: protocol::ColdMutation<En1992Snapshot, ReferenceDiff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::insert_prestress_steel::InsertPrestressSteel> {
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1992_diff::En1992Diff> {
        crate::remove_prestress_steel::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::remove_prestress_steel::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
