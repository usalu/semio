//! 🔄 `insert-fatigue-spectrum` mutation leaf: inserts one fatigue spectrum row.

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertFatigueSpectrum {
    pub index: Option<usize>,
    pub item: semio_s_artifact_norm_en1990_model::FatigueSpectrum,
}

impl<Op> protocol::MutationKind<En1990Snapshot, Op> for InsertFatigueSpectrum where Op: protocol::Mutation<En1990Snapshot, Diff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::remove_fatigue_spectrum::RemoveFatigueSpectrum> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "fatigue-spectrum",
        kind: "insert-fatigue-spectrum",
        record: "InsertedFatigueSpectrum",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Insert fatigue spectrum {}", self.item.id), &format!("Ermüdungskollektiv {} einfügen", self.item.id))
    }
}

impl<Op> protocol::ColdMutationKind<En1990Snapshot, Op> for InsertFatigueSpectrum where Op: protocol::ColdMutation<En1990Snapshot, ReferenceDiff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::remove_fatigue_spectrum::RemoveFatigueSpectrum> {
    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1990_diff::En1990Diff> {
        crate::insert_fatigue_spectrum::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::insert_fatigue_spectrum::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
