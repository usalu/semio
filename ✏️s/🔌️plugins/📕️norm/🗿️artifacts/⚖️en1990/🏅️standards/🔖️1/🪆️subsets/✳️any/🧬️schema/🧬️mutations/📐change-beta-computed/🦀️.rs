//! 🧬️ `change-beta-computed` mutation for EN 1990.

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeBetaComputed {
    pub new_beta_computed: f64,
}

impl<Op> protocol::MutationKind<En1990Snapshot, Op> for ChangeBetaComputed where Op: protocol::Mutation<En1990Snapshot, Diff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::change_beta_computed::ChangeBetaComputed> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "beta-computed",
        kind: "change-beta-computed",
        record: "ChangedBetaComputed",
    };

    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<En1990Snapshot>>::Diff> {
        crate::change_beta_computed::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_beta_computed::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change computed reliability index β", "Berechneten Zuverlässigkeitsindex β ändern")
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
