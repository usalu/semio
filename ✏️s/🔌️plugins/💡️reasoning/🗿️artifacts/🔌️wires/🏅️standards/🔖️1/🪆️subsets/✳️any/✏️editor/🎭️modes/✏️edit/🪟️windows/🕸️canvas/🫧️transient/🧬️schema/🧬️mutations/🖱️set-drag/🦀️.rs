//! 🖱️ Sets the wires drag target and pointer coordinates.
use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-drag")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetDrag {
    pub node_id: Option<String>,
    pub last_x: f64,
    pub last_y: f64,
}

impl protocol::MutationKind<WiresCanvasTransient, WiresCanvasTransientMutation> for SetDrag {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "drag", kind: "set-drag", record: "SetDrag" };
    fn diff(&self, base: &WiresCanvasTransient) -> protocol::MutationOutcome<WiresCanvasTransient> {
        let mut next = base.clone();
        next.drag_node_id = self.node_id.clone();
        next.drag_last_x = self.last_x;
        next.drag_last_y = self.last_y;
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &WiresCanvasTransient) -> Vec<WiresCanvasTransientMutation> {
        vec![WiresCanvasTransientMutation::SetDrag(Self { node_id: base.drag_node_id.clone(), last_x: base.drag_last_x, last_y: base.drag_last_y })]
    }
    fn label(&self) -> String {
        "Set Drag".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["drag".into()]
    }
}
