//! 🔧 `insert-assessment` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertAssessment {
    pub index: Option<usize>,
    pub assessment: semio_s_artifact_norm_en1998_model::En1998Assessment,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for InsertAssessment where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::remove_assessment::RemoveAssessment> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "assessment",
        kind: "insert-assessment",
        record: "InsertAssessment",
    };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<En1998Snapshot>>::Diff> {
        crate::insert_assessment::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::insert_assessment::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert assessed element", "Bewertetes Bauteil einfügen")
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-assessment".into()]
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
