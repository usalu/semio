//! 🕹️ `move-node` — absolute spatial reposition of a graph node (the node-graph canvas's `move`
//! edit op).

use crate::EquationSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MoveNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

impl<Op> protocol::MutationKind<EquationSnapshot, Op> for MoveNode
where
    Op: protocol::Mutation<EquationSnapshot, Diff = crate::EquationDiff> + From<MoveNode>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node", kind: "move-node", record: "MovedNode" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move node \"{}\"", self.id), &format!("Knoten \"{}\" verschieben", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<EquationSnapshot, Op> for MoveNode
where
    Op: protocol::ColdMutation<EquationSnapshot, ReferenceDiff = crate::EquationDiff> + From<MoveNode>,
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
