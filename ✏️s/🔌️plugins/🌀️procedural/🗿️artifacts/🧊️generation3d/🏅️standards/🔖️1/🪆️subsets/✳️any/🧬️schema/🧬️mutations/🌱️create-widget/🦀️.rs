//! ➕ `create-widget` payload — brings a new id-keyed [`Widget`] into existence at an insertion
//! index (FINAL-state, per `📓️derivation-rules.md` rule 3's addressing convention).

use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::Widget;
//#region 🔖️CreateWidget
/// ➕ Full initial payload for a new widget, placed at `index` if no widget with the same id
/// already exists (upsert-by-id, matching the widgets delta's own dedupe rule).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateWidget {
    pub index: usize,
    pub widget: Widget,
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for CreateWidget
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<crate::delete_widget::DeleteWidget>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "widget", kind: "create-widget", record: "CreatedWidget" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        use semio_s_artifact_procedural_3d_terminology::generation3d_catalogue_name;
        use semio_s_artifact_procedural_3d_terminology::Generation3dLabels;
        let id = semio_s_artifact_procedural_3d_model::widget_id(&self.widget);
        match &self.widget {
            Widget::Neuron { neuron_kind, .. } => {
                let name = |labels: &'static Generation3dLabels| generation3d_catalogue_name(labels, neuron_kind, neuron_kind).to_string();
                semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert \"{}\" ({id})", name(&Generation3dLabels::NATIVE_EN)), &format!("\"{}\" ({id}) einfügen", name(&Generation3dLabels::NATIVE_DE)))
            }
            _ => semio_framework_ui_locale::LocalizedLabel::native(&format!("Create widget \"{id}\""), &format!("Widget \"{id}\" erstellen")),
        }
    }

    fn target(&self) -> Vec<String> {
        vec![semio_s_artifact_procedural_3d_model::widget_id(&self.widget).to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<Generation3dSnapshot, Op> for CreateWidget
where
    Op: protocol::ColdMutation<Generation3dSnapshot, ReferenceDiff = Generation3dDiff> + From<crate::delete_widget::DeleteWidget>,
{
    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::create_widget::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::create_widget::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️CreateWidget
