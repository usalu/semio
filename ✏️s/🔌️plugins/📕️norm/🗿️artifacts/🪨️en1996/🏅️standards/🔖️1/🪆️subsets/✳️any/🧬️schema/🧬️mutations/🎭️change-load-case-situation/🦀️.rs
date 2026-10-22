//! 🎭️change-load-case-situation
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeLoadCaseSituation {
    pub wall_index: usize,
    pub load_case_index: usize,
    pub new_design_situation: String,
}

impl<Op> protocol::MutationKind<En1996Snapshot, Op> for ChangeLoadCaseSituation where Op: protocol::Mutation<En1996Snapshot, Diff = semio_s_artifact_norm_en1996_diff::En1996Diff> + From<crate::change_load_case_situation::ChangeLoadCaseSituation> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "load-case-situation",
        kind: "change-load-case-situation",
        record: "ChangedLoadCaseSituation",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change load-case design situation", "Lastfall-Bemessungssituation ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1996Snapshot, Op> for ChangeLoadCaseSituation where Op: protocol::ColdMutation<En1996Snapshot, ReferenceDiff = semio_s_artifact_norm_en1996_diff::En1996Diff> + From<crate::change_load_case_situation::ChangeLoadCaseSituation> {
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1996Snapshot>>::ReferenceDiff> {
        crate::change_load_case_situation::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_load_case_situation::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
