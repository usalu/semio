//! 🕸️ `change-column-rebar-f-yk-pa` mutation leaf.

use semio_s_artifact_norm_en1994_model::En1994Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeColumnRebarFYkPa {
    pub id: String,
    pub new_reinforcement_f_yk_pa: f64,
}

impl<Op> protocol::MutationKind<En1994Snapshot, Op> for ChangeColumnRebarFYkPa where Op: protocol::Mutation<En1994Snapshot, Diff = semio_s_artifact_norm_en1994_diff::En1994Diff> + From<crate::change_column_rebar_f_yk_pa::ChangeColumnRebarFYkPa> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "column-rebar-f-yk-pa", kind: "change-column-rebar-f-yk-pa", record: "ChangedColumnRebarFYkPa" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change column reinforcement strength f_yk", "Bewehrungsfestigkeit f_yk der Stütze ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1994Snapshot, Op> for ChangeColumnRebarFYkPa where Op: protocol::ColdMutation<En1994Snapshot, ReferenceDiff = semio_s_artifact_norm_en1994_diff::En1994Diff> + From<crate::change_column_rebar_f_yk_pa::ChangeColumnRebarFYkPa> {
    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1994Snapshot>>::ReferenceDiff> {
        crate::change_column_rebar_f_yk_pa::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_column_rebar_f_yk_pa::inverse::inverse(self, base)
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
