//! 🧬️ `remove-permanent` mutation for EN 1990.
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemovePermanent { pub index: usize, }
impl<Op> protocol::MutationKind<En1990Snapshot, Op> for RemovePermanent where Op: protocol::Mutation<En1990Snapshot, Diff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::insert_permanent::InsertPermanent> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "permanent", kind: "remove-permanent", record: "RemovedPermanent" };
    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<En1990Snapshot>>::Diff> { crate::remove_permanent::diff::diff(self, base) }
    fn inverse(&self, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::remove_permanent::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove permanent action", "Ständige Einwirkung entfernen")
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
