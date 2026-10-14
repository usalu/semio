//! 📏️ `change-storey-drift-x-m` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeStoreyDriftXM {
    pub building_index: usize,
    pub storey_index: usize,
    pub new_drift_x_m: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeStoreyDriftXM where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_storey_drift_xm::ChangeStoreyDriftXM> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "storey-drift-x-m",
        kind: "change-storey-drift-xm",
        record: "ChangeStoreyDriftXM",
    };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<En1998Snapshot>>::Diff> {
        crate::change_storey_drift_xm::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_storey_drift_xm::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change interstorey drift", "Stockwerksverschiebung ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-storey-drift-x-m".into()]
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
