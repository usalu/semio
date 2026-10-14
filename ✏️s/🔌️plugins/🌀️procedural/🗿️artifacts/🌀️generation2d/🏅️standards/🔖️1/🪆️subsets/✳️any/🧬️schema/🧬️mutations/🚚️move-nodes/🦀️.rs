//! 🚚️ Generation2d mutation — `MoveNodes`: one node-graph drag as intent (the node-graph gesture record of design §13.3):
//! the canvas offset every addressed widget moves by from its BASE position, so editing the drag in history replays it on
//! any base.

use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use crate::generation2d_label_number;

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
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

impl<Op> MutationKind<Generation2dSnapshot, Op> for MoveNodes
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<crate::move_widget::MoveWidget>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "nodes", kind: "move-nodes", record: "MovedNodes" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::move_nodes::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::move_nodes::inverse::inverse(self, base)?
    
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
