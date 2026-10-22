//! 🔗️ `connect-nodes` — creates an edge relationship between two graph nodes (the node-graph
//! canvas's `connect` edit op).

use crate::EquationSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ConnectNodes {
    pub id: String,
    pub source: String,
    pub target: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl<Op> protocol::MutationKind<EquationSnapshot, Op> for ConnectNodes
where
    Op: protocol::Mutation<EquationSnapshot, Diff = crate::EquationDiff> + From<ConnectNodes> + From<crate::standards::v1::subsets::graph::schema::mutations::disconnect_nodes::DisconnectNodes>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "node", kind: "connect-nodes", record: "ConnectedNodes" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect \"{}\" to \"{}\"", self.source, self.target), &format!("\"{}\" mit \"{}\" verbinden", self.source, self.target))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<EquationSnapshot, Op> for ConnectNodes
where
    Op: protocol::ColdMutation<EquationSnapshot, ReferenceDiff = crate::EquationDiff> + From<ConnectNodes> + From<crate::standards::v1::subsets::graph::schema::mutations::disconnect_nodes::DisconnectNodes>,
{
    fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<EquationSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
