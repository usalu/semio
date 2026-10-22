//! 🔧 `change-cellar-ventilation`.
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeCellarVentilation {
    pub new_cellar_ventilation_m3_h: f64,
}
impl<Op> protocol::MutationKind<Din16798Snapshot, Op> for ChangeCellarVentilation where Op: protocol::Mutation<Din16798Snapshot, Diff = semio_s_artifact_norm_din16798_diff::Din16798Diff> + From<crate::change_cellar_ventilation::ChangeCellarVentilation> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "cellar-ventilation", kind: "change-cellar-ventilation", record: "ChangeCellarVentilation" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change cellar airflow", "Kellerluftvolumenstrom ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Din16798Snapshot, Op> for ChangeCellarVentilation where Op: protocol::ColdMutation<Din16798Snapshot, ReferenceDiff = semio_s_artifact_norm_din16798_diff::Din16798Diff> + From<crate::change_cellar_ventilation::ChangeCellarVentilation> {
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din16798Snapshot>>::ReferenceDiff> { crate::change_cellar_ventilation::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::change_cellar_ventilation::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
