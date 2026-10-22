//! 🧬️ `insert-accidental` mutation for EN 1990.
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertAccidental { pub index: Option<usize>, pub item: semio_s_artifact_norm_en1990_model::AccidentalAction, }
impl<Op> protocol::MutationKind<En1990Snapshot, Op> for InsertAccidental where Op: protocol::Mutation<En1990Snapshot, Diff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::remove_accidental::RemoveAccidental> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "accidental", kind: "insert-accidental", record: "InsertedAccidental" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert accidental action", "Außergewöhnliche Einwirkung einfügen")
    }
}

impl<Op> protocol::ColdMutationKind<En1990Snapshot, Op> for InsertAccidental where Op: protocol::ColdMutation<En1990Snapshot, ReferenceDiff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::remove_accidental::RemoveAccidental> {
    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1990Snapshot>>::ReferenceDiff> { crate::insert_accidental::diff::diff(self, base) }
    fn inverse(&self, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::insert_accidental::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
