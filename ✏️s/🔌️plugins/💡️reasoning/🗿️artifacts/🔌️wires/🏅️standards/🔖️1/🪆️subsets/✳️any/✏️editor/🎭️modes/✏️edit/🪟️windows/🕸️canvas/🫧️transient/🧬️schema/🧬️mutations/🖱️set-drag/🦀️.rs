//! 🖱️ Sets the wires drag target and pointer coordinates.
use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-drag")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetDrag {
    pub node_id: Option<String>,
    pub start_x: f64,
    pub start_y: f64,
    pub last_x: f64,
    pub last_y: f64,
    pub zoom: f64,
}

impl protocol::MutationKind<WiresCanvasTransient, WiresCanvasTransientMutation> for SetDrag {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "drag", kind: "set-drag", record: "SetDrag" };
    fn diff(&self, base: &WiresCanvasTransient) -> protocol::MutationOutcome<WiresCanvasTransientDiff> {
        protocol::MutationOutcome::new(WiresCanvasTransientDiff {
            drag_node_id: (base.drag_node_id != self.node_id).then(|| WiresCanvasOptionalNode { value: self.node_id.clone() }),
            drag_start_x: (base.drag_start_x != self.start_x).then_some(self.start_x),
            drag_start_y: (base.drag_start_y != self.start_y).then_some(self.start_y),
            drag_last_x: (base.drag_last_x != self.last_x).then_some(self.last_x),
            drag_last_y: (base.drag_last_y != self.last_y).then_some(self.last_y),
            drag_zoom: (base.drag_zoom != self.zoom).then_some(self.zoom),
        })
    }
    fn inverse(&self, base: &WiresCanvasTransient) -> Result<Vec<WiresCanvasTransientMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![WiresCanvasTransientMutation::SetDrag(Self { node_id: base.drag_node_id.clone(), start_x: base.drag_start_x, start_y: base.drag_start_y, last_x: base.drag_last_x, last_y: base.drag_last_y, zoom: base.drag_zoom })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Drag", "Ziehvorgang setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["drag".into()]
    }
}
