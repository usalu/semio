//! 🧵️ `change-selection-series` — sets the optional product-series scalar of the active selection.

use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSelectionSeries {
    pub new_series_id: Option<String>,
}

impl<Op> protocol::MutationKind<Iso16757Snapshot, Op> for ChangeSelectionSeries where Op: protocol::Mutation<Iso16757Snapshot, Diff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::change_selection_series::mutation::ChangeSelectionSeries> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "selection-series", kind: "change-selection-series", record: "ChangedSelectionSeries" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        crate::change_selection_series::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_selection_series::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&{
        match &self.new_series_id {
            Some(id) => format!("Change selection series to \"{id}\""),
            None => "Clear selection series".to_string(),
        }
        }, &{
        match &self.new_series_id {
            Some(id) => format!("Auswahlreihe auf \"{id}\" ändern"),
            None => "Auswahlreihe leeren".to_string(),
        }
        })
    }
}
//#endregion 🔖️Payload
