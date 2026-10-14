//! 🔁️ `replace-part-number-rule` — whole-value swap of the part-number derivation rule
//! (`Literal`/`Table`/`Script` variants differ structurally, so this is a `replace`, not a `change`).

use semio_s_artifact_norm_iso16757_model::part_5::PartNumberRule;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ReplacePartNumberRule {
    pub new_rule: PartNumberRule,
}

impl<Op> protocol::MutationKind<Iso16757Snapshot, Op> for ReplacePartNumberRule where Op: protocol::Mutation<Iso16757Snapshot, Diff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::replace_part_number_rule::mutation::ReplacePartNumberRule> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "part-number-rule", kind: "replace-part-number-rule", record: "ReplacedPartNumberRule" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        crate::replace_part_number_rule::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::replace_part_number_rule::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace part-number rule", "Artikelnummer-Regel ersetzen")
    }
}
//#endregion 🔖️Payload
