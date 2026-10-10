//! ➕ `create-widget` payload — brings a new id-keyed [`Widget`] into existence at an insertion
//! index (FINAL-state, per `📓️derivation-rules.md` rule 3's addressing convention).

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::Widget;
//#region 🔖️CreateWidget
/// ➕ Full initial payload for a new widget, placed at `index` if no widget with the same id
/// already exists (upsert-by-id, matching the widgets delta's own dedupe rule).
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateWidget {
    pub index: usize,
    pub widget: Widget,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for CreateWidget {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "widget", kind: "create-widget", record: "CreatedWidget" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_widget::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::create_widget::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        use crate::terminology::{generation3d_catalogue_name, Generation3dLabels};
        let id = crate::widget_id(&self.widget);
        match &self.widget {
            Widget::Neuron { neuron_kind, .. } => {
                let name = |labels: &'static Generation3dLabels| generation3d_catalogue_name(labels, neuron_kind, neuron_kind).to_string();
                semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert \"{}\" ({id})", name(&Generation3dLabels::NATIVE_EN)), &format!("\"{}\" ({id}) einfügen", name(&Generation3dLabels::NATIVE_DE)))
            }
            _ => semio_framework_ui_locale::LocalizedLabel::native(&format!("Create widget \"{id}\""), &format!("Widget \"{id}\" erstellen")),
        }
    }

    fn target(&self) -> Vec<String> {
        vec![crate::widget_id(&self.widget).to_string()]
    }
}
//#endregion 🔖️CreateWidget
