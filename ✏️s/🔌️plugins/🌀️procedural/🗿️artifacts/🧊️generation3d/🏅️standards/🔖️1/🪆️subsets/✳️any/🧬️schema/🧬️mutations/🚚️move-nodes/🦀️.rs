//! 🚚️ `move-nodes` payload — one node-graph drag as intent (the node-graph gesture record of design §13.3): the canvas
//! offset every addressed widget moves by from its BASE position, so editing the drag in history replays it on any base.

use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use crate::generation3d_label_items;
use crate::generation3d_label_number;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

//#region 🔖️MoveNodes
/// 🚚️ Moves every addressed widget's canvas position by `(dx, dy)`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MoveNodes {
    pub ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_nodes<Op: From<MoveNodes>>(ids: Vec<String>, dx: f64, dy: f64) -> Op {
    Op::from(MoveNodes { ids, dx, dy })
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for MoveNodes
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<crate::move_widget::MoveWidget>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "nodes", kind: "move-nodes", record: "MovedNodes" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let [(x_en, x_de), (y_en, y_de)] = [self.dx, self.dy].map(generation3d_label_number);
        let (en, de) = generation3d_label_items(self.ids.len(), "node(s)", "Knoten");
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move {en} by ({x_en}, {y_en})"), &format!("{de} um ({x_de}; {y_de}) verschieben"))
    }

    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}

impl<Op> protocol::ColdMutationKind<Generation3dSnapshot, Op> for MoveNodes
where
    Op: protocol::ColdMutation<Generation3dSnapshot, ReferenceDiff = Generation3dDiff> + From<crate::move_widget::MoveWidget>,
{
    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::move_nodes::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::move_nodes::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️MoveNodes
