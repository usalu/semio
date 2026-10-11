//! 🗑️ `delete-widget` payload — removes an id-keyed [`Widget`] from the fixture. It does NOT cascade: the wires naming
//! the widget and its layout entry stay standing, and a command that deletes a widget spells that cascade as leaves of
//! its own (`generation3d_widget_removal`: `disconnect-synapse`, `delete-widget-position`, then this leaf).

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
//#region 🔖️DeleteWidget
/// 🗑️ Removes the widget with `id`; the diff/inverse leaves capture the full removed payload from
/// `base` so undo is a real `create-widget`, never a sentinel.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteWidget {
    pub id: String,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for DeleteWidget {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "widget", kind: "delete-widget", record: "DeletedWidget" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::standards::v1::subsets::any::schema::mutations::delete_widget::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::delete_widget::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete widget \"{}\"", self.id), &format!("Widget \"{}\" löschen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteWidget
