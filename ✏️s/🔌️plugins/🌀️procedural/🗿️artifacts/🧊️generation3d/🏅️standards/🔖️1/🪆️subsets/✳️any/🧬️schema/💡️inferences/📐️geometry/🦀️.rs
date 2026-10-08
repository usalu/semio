//! 📐️ `geometry` — the named inference that evaluates every widget of a generation3d snapshot to typed geometry values.
//!
//! It is an `InferredField` keyed by widget id: the plan is the topological order over synapses, a widget's dependency
//! input is its kind, stored literals and incoming wiring, and its compute resolves typed inputs from the parents'
//! evaluations and starts the catalogue kind's compute as a stepped job. The heavy values live in the instance-owned
//! [`engine::GeometryEngine`]; the inference record carries per-widget summaries.

#[path = "⚙️compute/🦀️.rs"]
pub mod compute;
#[path = "🚂️engine/🦀️.rs"]
pub mod engine;
#[path = "🔌️inputs/🦀️.rs"]
pub mod inputs;
#[path = "🗃️registry/🦀️.rs"]
pub mod registry;
#[path = "🛰️service/🦀️.rs"]
pub mod service;
#[path = "💎️value/🦀️.rs"]
pub mod value;
#[path = "🪄️widgets/🦀️.rs"]
pub mod widgets;

use crate::standards::v1::subsets::any::schema::catalogue::{Catalogue, Kind, Port, PortType, Quality};
use crate::{widget_id, Generation3dSnapshot};
use compute::{WidgetJob, WidgetStep};
use inputs::resolve_inputs;
use protocol::{ComputeStep, InferenceFault, InferencePending, InferenceStep, InferredField};
use semio_framework_artifact_flow_flow::{FlowHostSnapshot, SynapseSpec, Widget};
use semio_framework_value::{DslValue, ToValue};
use semio_framework_value_derive::{FromValue, ToValue};
use std::any::Any;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::Arc;
use value::{GeometryValue, WidgetEvaluation, WidgetFault, FAULT_PREFIX};

/// 🧰️ What a compute module imports: the value, input, job and fault vocabulary of a widget compute.
pub mod prelude {
    pub use super::compute::{failed, finish, finish_with_quality, ComputeEntry, KernelSession, StartFn, WidgetJob, WidgetStep};
    pub use super::inputs::WidgetInputs;
    pub use super::value::{kernel_fault, outputs, GeometryValue, Outputs, PlaneValue, SelectionKind, SelectionValue, WidgetEvaluation, WidgetFault};
    pub use crate::standards::v1::subsets::any::schema::catalogue::{Kind, Port, PortType, Quality};
}

//#region 🔖️Graph
/// 🔗️ The distinct existing widgets wired into `key`, in synapse order — the parents of its plan step.
fn parent_ids(host: &FlowHostSnapshot, key: &str) -> Vec<String> {
    let mut parents: Vec<String> = Vec::new();
    for synapse in host.synapses.iter().filter(|synapse| synapse.to == key) {
        if !parents.contains(&synapse.from) && host.widgets.iter().any(|widget| widget_id(widget) == synapse.from) {
            parents.push(synapse.from.clone());
        }
    }
    parents
}

/// 🧭️ The evaluation order: Kahn over the distinct parents in document order; widgets on or downstream of a cycle trail as parentless steps.
fn plan_of(host: &FlowHostSnapshot) -> Vec<InferenceStep<String>> {
    let mut seen = BTreeSet::new();
    let ids: Vec<&str> = host.widgets.iter().map(widget_id).filter(|id| seen.insert(*id)).collect();
    let mut incoming: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for synapse in &host.synapses {
        incoming.entry(synapse.to.as_str()).or_default().push(synapse.from.as_str());
    }
    let mut parents: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for id in &ids {
        let mut list: Vec<&str> = Vec::new();
        for from in incoming.get(id).into_iter().flatten() {
            if seen.contains(from) && !list.contains(from) {
                list.push(from);
            }
        }
        parents.insert(id, list);
    }
    let mut waiting: BTreeMap<&str, usize> = ids.iter().map(|id| (*id, parents[id].len())).collect();
    let mut children: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for id in &ids {
        for parent in &parents[id] {
            children.entry(parent).or_default().push(id);
        }
    }
    let mut queue: VecDeque<&str> = ids.iter().copied().filter(|id| waiting[id] == 0).collect();
    let mut order: Vec<&str> = Vec::new();
    while let Some(id) = queue.pop_front() {
        order.push(id);
        for child in children.get(id).into_iter().flatten() {
            if let Some(count) = waiting.get_mut(child) {
                *count -= 1;
                if *count == 0 {
                    queue.push_back(child);
                }
            }
        }
    }
    let ordered: BTreeSet<&str> = order.iter().copied().collect();
    let mut plan: Vec<InferenceStep<String>> = order.iter().map(|id| InferenceStep { key: id.to_string(), parents: parents[id].iter().map(|parent| parent.to_string()).collect() }).collect();
    plan.extend(ids.iter().filter(|id| !ordered.contains(*id)).map(|id| InferenceStep { key: id.to_string(), parents: Vec::new() }));
    plan
}
//#endregion 🔖️Graph

//#region 🔖️Dependency
/// 🔌️ One wire into a widget, as far as its evaluation reads it.
#[derive(Clone, Debug, PartialEq, ToValue)]
#[value(rename_all = "camelCase")]
pub struct GeometryWire {
    pub from: String,
    pub from_port: String,
    pub to_port: String,
}

/// 🔑️ Everything a widget's evaluation reads besides its parents' values: the widget variant and kind, the catalogue's own content hash of the kind, the stored literals and the incoming wiring.
#[derive(Clone, Debug, PartialEq, ToValue)]
#[value(rename_all = "camelCase")]
pub struct GeometryDependency {
    pub variant: String,
    pub kind: String,
    pub kind_definition: Option<Kind>,
    pub literal: DslValue,
    pub wiring: Vec<GeometryWire>,
}

fn dependency_of(host: &FlowHostSnapshot, catalogue: &Catalogue, key: &str) -> GeometryDependency {
    let wiring = host.synapses.iter().filter(|synapse| synapse.to == key).map(|synapse| GeometryWire { from: synapse.from.clone(), from_port: synapse.from_port.clone(), to_port: synapse.to_port.clone() }).collect();
    let number = |value: f64| DslValue::Object(vec![("value".to_string(), DslValue::float(value))]);
    let text = |value: &str| DslValue::Object(vec![("value".to_string(), DslValue::String(value.to_string()))]);
    let (variant, kind, literal) = match host.widgets.iter().find(|widget| widget_id(widget) == key) {
        Some(Widget::Neuron { neuron_kind, params, .. }) => ("neuron", neuron_kind.clone(), params.to_value()),
        Some(Widget::InputSlider { value, .. }) => ("input-slider", String::new(), number(*value)),
        Some(Widget::InputNote { text: note, .. }) => ("input-note", String::new(), text(note)),
        Some(Widget::InputImage { src, .. }) => ("input-image", String::new(), text(src)),
        Some(Widget::Variable { name, .. }) => ("variable", String::new(), text(name)),
        Some(Widget::OutputPreview { .. }) => ("output-preview", String::new(), DslValue::Null),
        Some(Widget::OutputAction { .. }) => ("output-action", String::new(), DslValue::Null),
        Some(Widget::OutputExport { .. }) => ("output-export", String::new(), DslValue::Null),
        Some(Widget::Cluster { name, .. }) => ("cluster", String::new(), text(name)),
        None => ("missing", String::new(), DslValue::Null),
    };
    GeometryDependency { variant: variant.to_string(), kind_definition: catalogue.kind(&kind).cloned(), kind, literal, wiring }
}
//#endregion 🔖️Dependency

//#region 🔖️Start
enum Started {
    Ready(WidgetEvaluation),
    Job(Box<dyn WidgetJob>, String),
}

fn faulted(code: &str, en: String, de: String, quality: Quality) -> Started {
    Started::Ready(WidgetEvaluation::faulted(WidgetFault::new(format!("{FAULT_PREFIX}{code}"), en, de), quality))
}

fn begin(snapshot: &GeometryInput<'_>, key: &str, parents: &[Arc<WidgetEvaluation>]) -> Started {
    let host = &snapshot.snapshot.host_snapshot;
    let Some(widget) = host.widgets.iter().find(|widget| widget_id(widget) == key) else {
        return faulted("widget-missing", format!("Widget \u{201c}{key}\u{201d} does not exist."), format!("Widget \u{201c}{key}\u{201d} existiert nicht."), Quality::ExactAnalytic);
    };
    let expected = parent_ids(host, key);
    let kind = match widget {
        Widget::Neuron { neuron_kind, .. } => snapshot.catalogue.kind(neuron_kind),
        _ => None,
    };
    let quality = kind.map_or(Quality::ExactAnalytic, |kind| kind.quality);
    if parents.is_empty() && !expected.is_empty() {
        return faulted("cycle", format!("Widget \u{201c}{key}\u{201d} is part of a wiring cycle or depends on one."), format!("Widget \u{201c}{key}\u{201d} ist Teil eines Verbindungszyklus oder hängt von einem ab."), quality);
    }
    let wires: Vec<&SynapseSpec> = host.synapses.iter().filter(|synapse| synapse.to == key).collect();
    let by_id: BTreeMap<&str, &Arc<WidgetEvaluation>> = expected.iter().map(String::as_str).zip(parents.iter()).collect();
    let parent = |id: &str| by_id.get(id).map(|evaluation| Arc::clone(evaluation));
    if let Some(evaluation) = widgets::evaluate(widget, &wires, &parent) {
        return Started::Ready(evaluation);
    }
    let Widget::Neuron { neuron_kind, params, .. } = widget else {
        return faulted("widget-unsupported", format!("Widget \u{201c}{key}\u{201d} has no geometry meaning."), format!("Widget \u{201c}{key}\u{201d} hat keine Geometriebedeutung."), quality);
    };
    let Some(kind) = kind else {
        return faulted("kind-unknown", format!("The widget kind \u{201c}{neuron_kind}\u{201d} is not in the catalogue."), format!("Der Widget-Typ \u{201c}{neuron_kind}\u{201d} ist nicht im Katalog."), quality);
    };
    match resolve_inputs(kind, key, params, &wires, &parent) {
        Ok(inputs) => Started::Job(registry::start(kind, inputs), kind.id.clone()),
        Err(fault) => Started::Ready(WidgetEvaluation::faulted(fault, kind.quality)),
    }
}

fn matches_port(port: &Port, found: &GeometryValue) -> bool {
    let element = |found: &GeometryValue| match (port.port_type, found) {
        (PortType::Number | PortType::Length | PortType::Angle, GeometryValue::Number(_)) => true,
        (PortType::Integer, GeometryValue::Integer(_)) => true,
        (PortType::Boolean, GeometryValue::Boolean(_)) => true,
        (PortType::Text | PortType::Enum, GeometryValue::Text(_)) => true,
        (PortType::Vector, GeometryValue::Vector(_)) => true,
        (PortType::Point, GeometryValue::Point(_)) => true,
        (PortType::Plane, GeometryValue::Plane(_)) => true,
        (PortType::Shape | PortType::Shapes, GeometryValue::Shape(shape)) => port.shape_kinds.as_deref().is_none_or(|kinds| kinds.contains(&value::shape_kind(shape.kind()))),
        (PortType::Mesh, GeometryValue::Mesh(_)) => true,
        (PortType::Selection, GeometryValue::Selection(_)) => true,
        (PortType::Any, _) => true,
        _ => false,
    };
    if port.list || port.port_type == PortType::Shapes {
        return matches!(found, GeometryValue::List(items) if items.iter().all(element));
    }
    element(found)
}

/// ⚖️ Enforces the output contract of a finished job: a faulted evaluation has no outputs, a successful one has exactly the catalogue's output ports with matching value variants.
pub(crate) fn contract_checked(kind: &Kind, evaluation: WidgetEvaluation) -> WidgetEvaluation {
    if let Some(fault) = evaluation.fault {
        return WidgetEvaluation::faulted(fault, evaluation.quality);
    }
    let violation = kind.outputs.iter().find(|port| evaluation.outputs.get(&port.name).is_none_or(|found| !matches_port(port, found))).map(|port| port.name.clone()).or_else(|| evaluation.outputs.keys().find(|name| kind.output(name).is_none()).cloned());
    match violation {
        None => evaluation,
        Some(port) => WidgetEvaluation::faulted(
            WidgetFault::new(format!("{FAULT_PREFIX}output-contract"), format!("The widget \u{201c}{}\u{201d} produced no valid value for output \u{201c}{port}\u{201d}.", kind.label.en), format!("Das Widget \u{201c}{}\u{201d} hat keinen gültigen Wert für den Ausgang \u{201c}{port}\u{201d} erzeugt.", kind.label.de)).at(port),
            evaluation.quality,
        ),
    }
}
//#endregion 🔖️Start

//#region 🔖️Field
/// ⏳ The widget job parked in the driver's cursor between calls.
struct GeometryPending {
    job: Box<dyn WidgetJob>,
    kind: String,
    catalogue: Arc<Catalogue>,
}

impl InferencePending for GeometryPending {
    fn cancel(&mut self) {
        self.job.cancel();
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// 📐️ The `geometry` field: widget id to the shared evaluation of that widget.
pub struct Generation3dGeometry;

/// 🧩️ The admitted semantic inputs of one inference run.
pub struct GeometryInput<'a> {
    pub snapshot: &'a Generation3dSnapshot,
    pub catalogue: Arc<Catalogue>,
}

impl<'a> GeometryInput<'a> {
    /// 🏗️ Binds a snapshot to an explicitly admitted immutable catalogue.
    pub fn new(snapshot: &'a Generation3dSnapshot, catalogue: Arc<Catalogue>) -> Self { Self { snapshot, catalogue } }
}

impl<'a> InferredField<GeometryInput<'a>> for Generation3dGeometry {
    type Key = String;
    type Value = Arc<WidgetEvaluation>;
    type Dependency = GeometryDependency;

    const FIELD_ID: &'static str = "s.procedural.generation3d.inference.geometry";
    const SCHEMA_VERSION: u32 = 2;

    fn reads() -> &'static [&'static str] {
        &["hostSnapshot/widgets", "hostSnapshot/synapses"]
    }

    fn plan(snapshot: &GeometryInput<'_>) -> Vec<InferenceStep<Self::Key>> {
        plan_of(&snapshot.snapshot.host_snapshot)
    }

    fn dep_input(snapshot: &GeometryInput<'_>, key: &Self::Key, _parents: &[Self::Key]) -> Self::Dependency {
        dependency_of(&snapshot.snapshot.host_snapshot, &snapshot.catalogue, key)
    }

    fn compute(snapshot: &GeometryInput<'_>, key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
        let mut pending = None;
        loop {
            match Self::compute_step(snapshot, key, parents, &mut pending, usize::MAX) {
                Ok(ComputeStep::Done { value, .. }) => return value,
                Ok(ComputeStep::Working { .. }) => continue,
                Err(fault) => return Arc::new(WidgetEvaluation::faulted(WidgetFault::new(fault.code, fault.message.clone(), fault.message), Quality::ExactAnalytic)),
            }
        }
    }

    fn value_bytes(value: &Self::Value) -> usize {
        value.byte_len()
    }

    fn compute_step(snapshot: &GeometryInput<'_>, key: &Self::Key, parents: &[Self::Value], pending: &mut Option<Box<dyn InferencePending>>, fuel: usize) -> Result<ComputeStep<Self::Value>, InferenceFault> {
        if pending.is_none() {
            match begin(snapshot, key, parents) {
                Started::Ready(evaluation) => return Ok(ComputeStep::Done { value: Arc::new(evaluation), fuel_used: 1 }),
                Started::Job(job, kind) => *pending = Some(Box::new(GeometryPending { job, kind, catalogue: Arc::clone(&snapshot.catalogue) })),
            }
        }
        let parked = pending.as_mut().and_then(|parked| parked.as_any_mut().downcast_mut::<GeometryPending>()).ok_or_else(|| InferenceFault::new(format!("{FAULT_PREFIX}internal"), "the pending slot does not hold a geometry job"))?;
        let grant = fuel.max(1);
        match parked.job.step(grant) {
            WidgetStep::Working { progress } => Ok(ComputeStep::Working { fuel_used: grant, progress }),
            WidgetStep::Done(evaluation) => {
                let checked = contract_checked(parked.catalogue.kind(&parked.kind).ok_or_else(|| InferenceFault::new(format!("{FAULT_PREFIX}internal"), "the admitted kind is missing"))?, evaluation);
                *pending = None;
                Ok(ComputeStep::Done { value: Arc::new(checked), fuel_used: 1 })
            }
        }
    }
}
//#endregion 🔖️Field

//#region 🔖️Record
/// 🚫️ A fault as the inference record states it.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dFaultRecord {
    pub code: String,
    pub en: String,
    pub de: String,
    pub port: Option<String>,
}

/// 📇️ One output of a widget as the inference record states it: the port, the value kind and a one-line detail.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dOutputRecord {
    pub port: String,
    pub kind: String,
    pub detail: String,
}

/// 📇️ One widget's evaluation as the inference record states it.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dWidgetRecord {
    pub quality: Quality,
    pub fault: Option<Generation3dFaultRecord>,
    pub outputs: Vec<Generation3dOutputRecord>,
}

/// 📐️ `geometry` — the evaluation of every widget, summarised: quality, fault and the kind and detail of each output.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dGeometryRecord {
    pub widgets: BTreeMap<String, Generation3dWidgetRecord>,
    pub faulted: u32,
}

impl Generation3dWidgetRecord {
    /// 📇️ The record of one evaluation.
    pub fn of(evaluation: &WidgetEvaluation) -> Self {
        Self {
            quality: evaluation.quality,
            fault: evaluation.fault.as_ref().map(|fault| Generation3dFaultRecord { code: fault.code.clone(), en: fault.message.en.clone(), de: fault.message.de.clone(), port: fault.port.clone() }),
            outputs: evaluation.outputs.iter().map(|(port, value)| {
                let summary = value.summary();
                Generation3dOutputRecord { port: port.clone(), kind: summary.kind, detail: summary.detail }
            }).collect(),
        }
    }
}

impl Generation3dGeometryRecord {
    /// 📇️ The record of a full evaluation.
    pub fn of(evaluations: &BTreeMap<String, Arc<WidgetEvaluation>>) -> Self {
        Self { widgets: evaluations.iter().map(|(id, evaluation)| (id.clone(), Generation3dWidgetRecord::of(evaluation))).collect(), faulted: evaluations.values().filter(|evaluation| evaluation.fault.is_some()).count() as u32 }
    }
}

/// 📐️ Evaluates a snapshot completely and cold, the semantic source every cached or stepped run equals.
pub fn infer_geometry(snapshot: &GeometryInput<'_>) -> BTreeMap<String, Arc<WidgetEvaluation>> {
    protocol::infer_field::<GeometryInput<'_>, Generation3dGeometry>(snapshot, None)
}
//#endregion 🔖️Record

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
