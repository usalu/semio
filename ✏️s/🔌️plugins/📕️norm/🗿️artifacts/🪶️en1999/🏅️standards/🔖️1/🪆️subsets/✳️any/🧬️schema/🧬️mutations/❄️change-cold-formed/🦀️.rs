//! 🦎 `change-cold-formed` mutation leaf.

use semio_s_artifact_norm_en1999_diff::En1999Diff;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_model::ColdFormedSheet;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeColdFormed {
    pub cold_formed: Vec<ColdFormedSheet>,
}

impl<Op> protocol::MutationKind<En1999Snapshot, Op> for ChangeColdFormed where Op: protocol::Mutation<En1999Snapshot, Diff = semio_s_artifact_norm_en1999_diff::En1999Diff> + From<crate::change_cold_formed::ChangeColdFormed> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "cold-formed",
        kind: "change-cold-formed",
        record: "ChangedColdFormedSheets",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change cold-formed members", "Kaltprofilierte Bauteile ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1999Snapshot, Op> for ChangeColdFormed where Op: protocol::ColdMutation<En1999Snapshot, ReferenceDiff = semio_s_artifact_norm_en1999_diff::En1999Diff> + From<crate::change_cold_formed::ChangeColdFormed> {
    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        crate::change_cold_formed::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_cold_formed::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
