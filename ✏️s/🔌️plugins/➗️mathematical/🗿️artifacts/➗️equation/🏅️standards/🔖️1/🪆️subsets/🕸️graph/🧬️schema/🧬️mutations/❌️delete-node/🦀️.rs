//! ❌️ `delete-node` — removes an id-keyed graph node, cascading to every edge incident on it.

use crate::EquationSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteNode {
    pub id: String,
}

impl<Op> protocol::MutationKind<EquationSnapshot, Op> for DeleteNode
where
    Op: protocol::Mutation<EquationSnapshot, Diff = crate::EquationDiff> + From<DeleteNode> + From<crate::standards::v1::subsets::graph::schema::mutations::connect_nodes::ConnectNodes> + From<crate::standards::v1::subsets::graph::schema::mutations::create_node::CreateNode>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "node", kind: "delete-node", record: "DeletedNode" };

    fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete node \"{}\"", self.id), &format!("Knoten \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
