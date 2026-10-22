//! 🟢️ `create-node` — brings a new id-keyed graph node into existence.

use crate::EquationSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateNode {
    pub id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl<Op> protocol::MutationKind<EquationSnapshot, Op> for CreateNode
where
    Op: protocol::Mutation<EquationSnapshot, Diff = crate::EquationDiff> + From<CreateNode> + From<crate::standards::v1::subsets::graph::schema::mutations::delete_node::DeleteNode>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create node \"{}\"", self.label), &format!("Knoten \"{}\" erstellen", self.label))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<EquationSnapshot, Op> for CreateNode
where
    Op: protocol::ColdMutation<EquationSnapshot, ReferenceDiff = crate::EquationDiff> + From<CreateNode> + From<crate::standards::v1::subsets::graph::schema::mutations::delete_node::DeleteNode>,
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
