//! 🚚️ `move-nodes` payload — one node-graph drag as intent (the node-graph gesture record of design §13.3): the canvas
//! offset every addressed widget moves by from its BASE position, so editing the drag in history replays it on any base.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use crate::generation3d_label_items;
use crate::generation3d_label_number;
use crate::Generation3dMutation;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

//#region 🔖️MoveNodes
/// 🚚️ Moves every addressed widget's canvas position by `(dx, dy)`.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveNodes {
    pub ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_nodes(ids: Vec<String>, dx: f64, dy: f64) -> Generation3dMutation {
    Generation3dMutation::MoveNodes(MoveNodes { ids, dx, dy })
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for MoveNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "nodes", kind: "move-nodes", record: "MovedNodes" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::move_nodes::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
        crate::move_nodes::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let [(x_en, x_de), (y_en, y_de)] = [self.dx, self.dy].map(generation3d_label_number);
        let (en, de) = generation3d_label_items(self.ids.len(), "node(s)", "Knoten");
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move {en} by ({x_en}, {y_en})"), &format!("{de} um ({x_de}; {y_de}) verschieben"))
    }

    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
//#endregion 🔖️MoveNodes
