//! 🔀️ `change-graph-directed` — flips the graph playground's directed/undirected toggle.

use crate::{EquationMutation, EquationSnapshot};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeGraphDirected {
    pub new_directed: bool,
}

impl protocol::MutationKind<EquationSnapshot, EquationMutation> for ChangeGraphDirected {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "graph", kind: "change-graph-directed", record: "ChangedGraphDirected" };

    fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &EquationSnapshot) -> Result<Vec<EquationMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set graph direction to {}", if self.new_directed { "directed" } else { "undirected" }), &format!("Graphrichtung auf {} setzen", if self.new_directed { "gerichtet" } else { "ungerichtet" }))
    }
    fn target(&self) -> Vec<String> {
        vec!["graph".into()]
    }
}
//#endregion 🔖️Payload
