//! 🪄️ The non-neuron widgets: sources that hand a document value to the wiring, and sinks that pass or swallow one.

use super::value::{outputs, GeometryValue, WidgetEvaluation, WidgetFault, FAULT_PREFIX};
use crate::standards::v1::subsets::any::schema::catalogue::Quality;
use semio_framework_artifact_flow_flow::{SynapseSpec, Widget};
use std::sync::Arc;

/// 🚫️ The refusal code of a widget kind that has no geometry meaning.
pub const WIDGET_UNSUPPORTED: &str = "generation3d.geometry.widget-unsupported";

fn pass_through(wires: &[&SynapseSpec], parent: &dyn Fn(&str) -> Option<Arc<WidgetEvaluation>>) -> WidgetEvaluation {
    let mut merged = super::value::Outputs::new();
    for wire in wires {
        let Some(source) = parent(&wire.from) else {
            let from = wire.from.clone();
            return WidgetEvaluation::faulted(WidgetFault::new(format!("{FAULT_PREFIX}input-missing"), format!("The preview is wired to widget \u{201c}{from}\u{201d}, which does not exist."), format!("Die Vorschau ist mit dem Widget \u{201c}{from}\u{201d} verbunden, das nicht existiert.")), Quality::ExactAnalytic);
        };
        if let Some(fault) = &source.fault {
            return WidgetEvaluation::faulted(fault.clone(), source.quality);
        }
        if wire.from_port.is_empty() {
            merged.extend(source.outputs.iter().map(|(port, value)| (port.clone(), value.clone())));
        } else if let Some(value) = source.outputs.get(&wire.from_port) {
            merged.insert(wire.from_port.clone(), value.clone());
        } else {
            let (from, from_port) = (wire.from.clone(), wire.from_port.clone());
            return WidgetEvaluation::faulted(WidgetFault::new(format!("{FAULT_PREFIX}input-missing"), format!("Widget \u{201c}{from}\u{201d} has no output \u{201c}{from_port}\u{201d}."), format!("Widget \u{201c}{from}\u{201d} hat keinen Ausgang \u{201c}{from_port}\u{201d}.")), source.quality);
        }
    }
    WidgetEvaluation::ok(merged, Quality::ExactAnalytic)
}

/// 🧮️ Evaluates a widget that is not an operator: sliders, notes, images and variables provide their document value, a preview passes its source through, actions and exports consume without output, a cluster is refused.
/// `wires` are the synapses ending at the widget; `parent` answers the evaluation of a source widget. An operator widget has no evaluation here.
pub fn evaluate(widget: &Widget, wires: &[&SynapseSpec], parent: &dyn Fn(&str) -> Option<Arc<WidgetEvaluation>>) -> Option<WidgetEvaluation> {
    let provided = |port: &str, value: GeometryValue| WidgetEvaluation::ok(outputs([(port, value)]), Quality::ExactAnalytic);
    Some(match widget {
        Widget::Neuron { .. } => return None,
        Widget::InputSlider { value, .. } => provided("number", GeometryValue::Number(*value)),
        Widget::InputNote { text, .. } => provided("text", GeometryValue::Text(text.clone())),
        Widget::InputImage { src, .. } => provided("image", GeometryValue::Text(src.clone())),
        Widget::Variable { name, .. } => provided("value", GeometryValue::Text(name.clone())),
        Widget::OutputPreview { .. } => pass_through(wires, parent),
        Widget::OutputAction { .. } | Widget::OutputExport { .. } => WidgetEvaluation::ok(Default::default(), Quality::ExactAnalytic),
        Widget::Cluster { name, .. } => WidgetEvaluation::faulted(
            WidgetFault::new(WIDGET_UNSUPPORTED, format!("The cluster \u{201c}{name}\u{201d} has no geometry meaning; wire its contents directly."), format!("Das Cluster \u{201c}{name}\u{201d} hat keine Geometriebedeutung; die Inhalte direkt verbinden.")),
            Quality::ExactAnalytic,
        ),
    })
}
