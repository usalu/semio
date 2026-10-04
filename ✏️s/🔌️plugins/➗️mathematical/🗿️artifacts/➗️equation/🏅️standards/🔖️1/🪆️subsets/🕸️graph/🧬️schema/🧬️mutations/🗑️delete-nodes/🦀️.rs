//! 🗑️ `delete-nodes` — plural/bulk delete, the real multi-select gesture behind the node-graph
//! canvas's `delete` row of the shared node-graph vocabulary (`✏️editor/🎮️commands/🕸️node-graph-edit`) —
//! a separate mutation per taxonomy's "Bulk/plural mutations" rule, never a bare `Vec` bolted onto
//! the singular `delete-node`.

use crate::{EquationMutation, EquationSnapshot};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteNodes {
    pub ids: Vec<String>,
}

impl protocol::MutationKind<EquationSnapshot, EquationMutation> for DeleteNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "nodes", kind: "delete-nodes", record: "DeletedNodes" };

    fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &EquationSnapshot) -> Result<Vec<EquationMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete {} nodes", self.ids.len()), &format!("{} Knoten löschen", self.ids.len()))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
//#endregion 🔖️Payload
