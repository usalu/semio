//! 🚚️ Generation2d mutation — `MoveNodes`: one node-graph drag as intent (the node-graph gesture record of design §13.3):
//! the canvas offset every addressed widget moves by from its BASE position, so editing the drag in history replays it on
//! any base.

use crate::standards::v1::subsets::any::schema::diff::Generation2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{generation2d_label_number,Generation2dMutation};

use crate::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️MoveNodes
/// 🚚️ Moves every addressed widget's canvas position by `(dx, dy)`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MoveNodes {
    pub ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_nodes(ids: Vec<String>, dx: f64, dy: f64) -> Generation2dMutation {
    Generation2dMutation::MoveNodes(MoveNodes { ids, dx, dy })
}

impl MutationKind<Generation2dSnapshot, Generation2dMutation> for MoveNodes {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "nodes", kind: "move-nodes", record: "MovedNodes" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Generation2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let [(x_en, x_de), (y_en, y_de)] = [self.dx, self.dy].map(generation2d_label_number);
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move {} node(s) by ({x_en}, {y_en})", self.ids.len()), &format!("{} Knoten um ({x_de}; {y_de}) verschieben", self.ids.len()))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
//#endregion 🔖️MoveNodes
