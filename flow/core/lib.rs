//! 🌊 Flow core: widgets, neural evaluation, and DAG canvas host.

pub use infinite_cavas as cavas;
pub use mathematical_graph_port_directed_dag as dag;
pub use neural_engine as neural;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::hash::{Hash, Hasher};
use std::sync::OnceLock;

use dag::{
    computation_node_height, computation_node_width, fit_node_size, image_widget_size, io_widget_height, io_widget_width, note_widget_size, preview_widget_size, slider_widget_height, slider_widget_width,
    normalize_node_display, would_create_cycle, DagFixtureEdgeV1, DagFixtureV1, DagHost, DagLayoutOptions, DagNodeKind, DagNodeSpec, DagPreviewContent, IoPortSpec,
};
use neural::{
    cluster_operator_info, Atom, ChannelSpec, Dictionary, EvalChannels, EvalError, Evaluator, NeuralCache, Neuron, OperatorInfo, Synapse, Tree, Value as NeuralValue, CLUSTER_KIND,
    INPUT_KIND, OUTPUT_KIND,
};
use serde::{Deserialize, Serialize};

// #region 🔖Widget
/// 🎛️ Flow widget discriminant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Widget {
    Neuron {
        id: String,
        neuronKind: String,
        #[serde(default)]
        params: Dictionary,
        #[serde(default, alias = "input_ports")]
        input_ports: Vec<String>,
        #[serde(default = "default_neuron_preview")]
        preview: bool,
    },
    InputSlider {
        id: String,
        #[serde(default = "default_slider_value")]
        value: f64,
        #[serde(default = "default_slider_min")]
        min: f64,
        #[serde(default = "default_slider_max")]
        max: f64,
        #[serde(default = "default_slider_step")]
        step: f64,
    },
    InputNote { id: String, #[serde(default)] text: String },
    InputImage { id: String, #[serde(default)] src: String },
    OutputPreview { id: String, #[serde(default)] preview: Dictionary, #[serde(default)] expanded: BTreeSet<String> },
    OutputAction { id: String, #[serde(default)] action: String },
    Cluster {
        id: String,
        #[serde(default)]
        name: String,
        tree: Tree,
        #[serde(default)]
        flow: FlowGuiV1,
    },
}

fn default_slider_value() -> f64 {
    3.0
}

fn default_slider_min() -> f64 {
    FLOW_SLIDER_MIN
}

fn default_slider_max() -> f64 {
    FLOW_SLIDER_MAX
}

fn default_slider_step() -> f64 {
    FLOW_SLIDER_STEP
}

fn default_neuron_preview() -> bool {
    true
}

/// 📍 Persisted widget position on the canvas.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetLayout {
    pub x: f64,
    pub y: f64,
}

/// 🧩 Serializable flow document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowFixtureV1 {
    pub schema: String,
    pub camera: CameraJson,
    pub widgets: Vec<Widget>,
    pub synapses: Vec<SynapseSpec>,
    #[serde(default)]
    pub layout: BTreeMap<String, WidgetLayout>,
}

/// 🧾 Serializable flow document with strippable GUI data and authoritative neural tree.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowDocumentV1 {
    pub schema: String,
    pub flow: FlowGuiV1,
    pub tree: Tree,
}

/// 🖼️ GUI-only flow data that can be removed without destroying logic.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowGuiV1 {
    pub camera: CameraJson,
    pub nodes: BTreeMap<String, FlowNodeGui>,
    #[serde(default)]
    pub previews: Vec<FlowPreviewGui>,
}

/// 🧩 GUI-only node presentation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowNodeGui {
    pub layout: WidgetLayout,
    pub chrome: NodeChrome,
}

/// 🪟 GUI-only node chrome.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum NodeChrome {
    Plain,
    Slider { min: f64, max: f64, step: f64 },
    Note,
    Image,
}

/// 👁️ GUI-only preview binding.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowPreviewGui {
    pub id: String,
    pub source: Option<FlowChannelRefV1>,
    pub mode: String,
}

/// 📡 Serializable channel reference.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowChannelRefV1 {
    pub neuron: String,
    pub channel: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CameraJson {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

fn default_from_port() -> String {
    "out".into()
}

fn default_to_port() -> String {
    "in".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SynapseSpec {
    pub id: String,
    pub from: String,
    pub to: String,
    #[serde(default = "default_from_port")]
    pub from_port: String,
    #[serde(default = "default_to_port")]
    pub to_port: String,
}

impl Default for FlowFixtureV1 {
    fn default() -> Self {
        Self {
            schema: "flow.fixture/v1".into(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            widgets: vec![
                Widget::InputSlider { id: "slider".into(), value: 3.0, min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP },
                Widget::Neuron {
                    id: "add".into(),
                    neuronKind: "math.add".into(),
                    params: Dictionary::new(),
                    input_ports: vec![],
                    preview: true,
                },
                Widget::OutputPreview { id: "preview".into(), preview: Dictionary::new(), expanded: BTreeSet::new() },
            ],
            synapses: vec![
                SynapseSpec {
                    id: "s1".into(),
                    from: "slider".into(),
                    to: "add".into(),
                    from_port: "out".into(),
                    to_port: "a".into(),
                },
                SynapseSpec {
                    id: "s2".into(),
                    from: "add".into(),
                    to: "preview".into(),
                    from_port: "out".into(),
                    to_port: "in".into(),
                },
            ],
            layout: BTreeMap::new(),
        }
    }
}

impl FlowFixtureV1 {
    pub fn to_document(&self) -> FlowDocumentV1 {
        let mut nodes = BTreeMap::new();
        let mut previews = Vec::new();
        for widget in &self.widgets {
            let id = widget_id_for(widget).to_string();
            nodes.insert(
                id.clone(),
                FlowNodeGui {
                    layout: self.layout.get(id.as_str()).cloned().unwrap_or(WidgetLayout { x: 0.0, y: 0.0 }),
                    chrome: widget_chrome(widget),
                },
            );
            if let Widget::OutputPreview { id, .. } = widget {
                let source = self
                    .synapses
                    .iter()
                    .find(|synapse| synapse.to == *id)
                    .map(|synapse| FlowChannelRefV1 { neuron: synapse.from.clone(), channel: synapse.from_port.clone() });
                previews.push(FlowPreviewGui { id: id.clone(), source, mode: "text".into() });
            }
        }
        FlowDocumentV1 {
            schema: "flow.document/v1".into(),
            flow: FlowGuiV1 { camera: self.camera.clone(), nodes, previews },
            tree: tree_from_fixture(self),
        }
    }
}

fn widget_chrome(widget: &Widget) -> NodeChrome {
    match widget {
        Widget::InputSlider { min, max, step, .. } => NodeChrome::Slider { min: *min, max: *max, step: *step },
        Widget::InputNote { .. } => NodeChrome::Note,
        Widget::InputImage { .. } => NodeChrome::Image,
        Widget::Cluster { .. } => NodeChrome::Plain,
        _ => NodeChrome::Plain,
    }
}

fn tree_from_fixture(fixture: &FlowFixtureV1) -> Tree {
    let neurons = fixture
        .widgets
        .iter()
        .filter_map(|w| match w {
            Widget::Neuron { id, neuronKind, params, .. } => Some(Neuron {
                id: id.clone(),
                kind: neuronKind.clone(),
                params: params.clone(),
                tree: None,
            }),
            Widget::InputSlider { id, value, .. } => Some(Neuron {
                id: id.clone(),
                kind: "core.number".into(),
                params: Dictionary::new().insert("value", NeuralValue::Atom(Atom::Decimal(*value))),
                tree: None,
            }),
            Widget::InputNote { id, text } => Some(Neuron {
                id: id.clone(),
                kind: "core.text".into(),
                params: Dictionary::new().insert("value", NeuralValue::Atom(Atom::String(text.clone()))),
                tree: None,
            }),
            Widget::InputImage { id, src } => Some(Neuron {
                id: id.clone(),
                kind: "core.image".into(),
                params: Dictionary::new().insert("dataUrl", NeuralValue::Atom(Atom::String(src.clone()))),
                tree: None,
            }),
            Widget::Cluster { id, name, tree, .. } => Some(Neuron {
                id: id.clone(),
                kind: CLUSTER_KIND.into(),
                params: Dictionary::new().insert("name", NeuralValue::Atom(Atom::String(name.clone()))),
                tree: Some(Box::new(tree.clone())),
            }),
            _ => None,
        })
        .collect();
    let synapses = fixture
        .synapses
        .iter()
        .map(|s| Synapse {
            id: s.id.clone(),
            from: s.from.clone(),
            to: s.to.clone(),
            from_port: s.from_port.clone(),
            to_port: s.to_port.clone(),
        })
        .collect();
    Tree { neurons, synapses }
}

fn tree_signature(tree: &Tree, seeds: &HashMap<String, Dictionary>) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let mut neurons: Vec<_> = tree
        .neurons
        .iter()
        .map(|neuron| (neuron.id.as_str(), neuron.kind.as_str(), &neuron.params, &neuron.tree))
        .collect();
    neurons.sort_by(|left, right| left.0.cmp(right.0));
    for (id, kind, params, subtree) in neurons {
        id.hash(&mut hasher);
        kind.hash(&mut hasher);
        if let Ok(json) = serde_json::to_string(params) {
            json.hash(&mut hasher);
        }
        if let Some(subtree) = subtree {
            if let Ok(json) = serde_json::to_string(subtree) {
                json.hash(&mut hasher);
            }
        }
    }
    let mut synapses: Vec<_> = tree
        .synapses
        .iter()
        .map(|synapse| (synapse.from.as_str(), synapse.to.as_str(), synapse.from_port.as_str(), synapse.to_port.as_str()))
        .collect();
    synapses.sort_by(|left, right| (left.0, left.1, left.2, left.3).cmp(&(right.0, right.1, right.2, right.3)));
    for (from, to, from_port, to_port) in synapses {
        from.hash(&mut hasher);
        to.hash(&mut hasher);
        from_port.hash(&mut hasher);
        to_port.hash(&mut hasher);
    }
    let mut seed_keys: Vec<_> = seeds.keys().collect();
    seed_keys.sort();
    for key in seed_keys {
        key.hash(&mut hasher);
        if let Ok(json) = serde_json::to_string(seeds.get(key).expect("key")) {
            json.hash(&mut hasher);
        }
    }
    hasher.finish()
}

fn widget_label(widget: &Widget) -> String {
    match widget {
        Widget::Neuron { neuronKind, .. } => neuronKind.clone(),
        Widget::InputSlider { .. } => "Slider".into(),
        Widget::InputNote { .. } => "Note".into(),
        Widget::InputImage { .. } => "Image".into(),
        Widget::OutputPreview { .. } => "Preview".into(),
        Widget::OutputAction { action, .. } => action.clone(),
        Widget::Cluster { name, .. } => if name.is_empty() { "Cluster".into() } else { name.clone() },
    }
}

fn widget_display_meta(widget: &Widget, kind_infos: &HashMap<String, OperatorInfo>) -> (String, String, String) {
    match widget {
        Widget::Neuron { neuronKind, .. } => kind_infos.get(neuronKind).map(|info| (info.name.clone(), info.abbreviation.clone(), info.icon.clone())).unwrap_or_else(|| {
            let (name, abbreviation) = dag::normalize_node_display(neuronKind, neuronKind);
            (name, abbreviation, String::new())
        }),
        Widget::InputSlider { .. } => ("Slider".into(), "Slider".into(), "emoji:🎚️".into()),
        Widget::InputNote { .. } => ("Note".into(), "Note".into(), "emoji:📝".into()),
        Widget::InputImage { .. } => ("Image".into(), "Image".into(), "emoji:🖼️".into()),
        Widget::OutputPreview { .. } => ("Preview".into(), "Preview".into(), "emoji:👁️".into()),
        Widget::OutputAction { action, .. } => {
            let title = if action.is_empty() { "Action" } else { action.as_str() };
            let (name, abbreviation) = dag::normalize_node_display(title, title);
            (name, abbreviation, "emoji:⚡".into())
        }
        Widget::Cluster { name, .. } => {
            let title = if name.is_empty() { "Cluster" } else { name.as_str() };
            let (display_name, abbreviation) = dag::normalize_node_display(title, title);
            (display_name, abbreviation, "emoji:🧩".into())
        }
    }
}

fn cluster_io_layout(cluster_id: &str, name: &str, tree: &Tree, synapses: &[SynapseSpec]) -> (Vec<IoPortSpec>, Vec<IoPortSpec>) {
    let info = cluster_operator_info(cluster_id, name, tree);
    let inputs = info
        .inputs
        .iter()
        .map(|spec| input_spec_to_port(spec, &Dictionary::new(), is_port_connected(synapses, cluster_id, &spec.id)))
        .collect();
    let outputs = info
        .outputs
        .iter()
        .map(|spec| IoPortSpec {
            id: spec.id.clone(),
            label: spec.label.clone().unwrap_or_else(|| spec.id.clone()),
            value_type: channel_spec_value_type(spec),
            default: spec.default.as_ref().map(neural_value_to_json_value),
            value: None,
            connected: None,
        })
        .collect();
    (inputs, outputs)
}

fn neural_value_to_json_value(value: &NeuralValue) -> serde_json::Value {
    serde_json::to_value(value).unwrap_or(serde_json::Value::Null)
}

fn channel_spec_value_type(spec: &ChannelSpec) -> Option<String> {
    if spec.operators.is_empty() {
        Some("value".into())
    } else {
        Some(spec.operators.join(","))
    }
}

fn is_port_connected(synapses: &[SynapseSpec], neuron_id: &str, port_id: &str) -> bool {
    synapses.iter().any(|syn| syn.to == neuron_id && syn.to_port == port_id)
}

fn input_spec_to_port(spec: &ChannelSpec, params: &Dictionary, connected: bool) -> IoPortSpec {
    let label = spec.label.clone().unwrap_or_else(|| spec.id.clone());
    let value = params.get(&spec.id).or(spec.default.as_ref()).map(neural_value_to_json_value);
    IoPortSpec {
        id: spec.id.clone(),
        label,
        value_type: channel_spec_value_type(spec),
        default: spec.default.as_ref().map(neural_value_to_json_value),
        value,
        connected: Some(connected),
    }
}

fn default_neuron_input_ports(kind: &str, input_ports: &[String], kind_infos: &HashMap<String, OperatorInfo>) -> Vec<String> {
    if !input_ports.is_empty() {
        return input_ports.to_vec();
    }
    if let Some(spec) = kind_infos.get(kind).and_then(|info| info.variadic_input.as_ref()) {
        return (0..spec.min).map(|index| index.to_string()).collect();
    }
    if let Some(info) = kind_infos.get(kind) {
        if !info.inputs.is_empty() && info.inputs[0].id != "*" {
            return info.inputs.iter().map(|entry| entry.id.clone()).collect();
        }
    }
    vec!["in".into()]
}

fn neuron_io_layout(
    neuron_id: &str,
    neuron_kind: &str,
    input_ports: &[String],
    params: &Dictionary,
    synapses: &[SynapseSpec],
    kind_infos: &HashMap<String, OperatorInfo>,
) -> (Vec<IoPortSpec>, Vec<IoPortSpec>, bool, bool) {
    let info = kind_infos.get(neuron_kind);
    let outputs = info
        .map(|entry| {
            entry
                .outputs
                .iter()
                .map(|spec| IoPortSpec {
                    id: spec.id.clone(),
                    label: spec.label.clone().unwrap_or_else(|| spec.id.clone()),
                    value_type: channel_spec_value_type(spec),
                    default: spec.default.as_ref().map(neural_value_to_json_value),
                    value: None,
                    connected: None,
                })
                .collect::<Vec<_>>()
        })
        .filter(|outputs| !outputs.is_empty())
        .unwrap_or_else(|| vec![IoPortSpec::simple("out", "out")]);
    if let Some(_spec) = info.and_then(|entry| entry.variadic_input.as_ref()) {
        let ports = default_neuron_input_ports(neuron_kind, input_ports, kind_infos);
        let inputs = ports
            .iter()
            .map(|port_id| {
                let connected = is_port_connected(synapses, neuron_id, port_id);
                IoPortSpec {
                    id: port_id.clone(),
                    label: port_id.clone(),
                    connected: Some(connected),
                    ..Default::default()
                }
            })
            .collect();
        let variadic_outputs = info.and_then(|entry| entry.variadic_output.as_ref()).is_some();
        return (inputs, outputs, true, variadic_outputs);
    }
    if let Some(entry) = info {
        if !entry.inputs.is_empty() && entry.inputs[0].id != "*" {
            let inputs = entry
                .inputs
                .iter()
                .map(|spec| input_spec_to_port(spec, params, is_port_connected(synapses, neuron_id, &spec.id)))
                .collect();
            return (inputs, outputs, false, false);
        }
    }
    if !input_ports.is_empty() {
        let inputs = input_ports
            .iter()
            .map(|port_id| {
                let connected = is_port_connected(synapses, neuron_id, port_id);
                IoPortSpec {
                    id: port_id.clone(),
                    label: port_id.clone(),
                    connected: Some(connected),
                    ..Default::default()
                }
            })
            .collect();
        return (inputs, outputs, false, false);
    }
    (
        vec![IoPortSpec::simple("in", "in")],
        outputs,
        false,
        false,
    )
}

fn widget_io_ports(widget: &Widget, synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> (Vec<IoPortSpec>, Vec<IoPortSpec>, bool, bool) {
    match widget {
        Widget::Neuron { id, neuronKind, params, input_ports, .. } => {
            neuron_io_layout(id, neuronKind, input_ports, params, synapses, kind_infos)
        }
        Widget::InputSlider { .. } | Widget::InputNote { .. } | Widget::InputImage { .. } => (
            vec![],
            vec![IoPortSpec::simple("out", "out")],
            false,
            false,
        ),
        Widget::OutputPreview { .. } | Widget::OutputAction { .. } => (
            vec![IoPortSpec::simple("in", "in")],
            vec![],
            false,
            false,
        ),
        Widget::Cluster { id, name, tree, .. } => {
            let (inputs, outputs) = cluster_io_layout(id, name, tree, synapses);
            (inputs, outputs, false, false)
        }
    }
}

fn widget_node_size(widget: &Widget, synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> (f64, f64) {
    let label = widget_label(widget);
    match widget {
        Widget::InputSlider { .. } => {
            let output = IoPortSpec::simple("out", "out");
            (slider_widget_width(&label, &output), slider_widget_height())
        }
        Widget::InputNote { text, .. } => note_widget_size(text),
        Widget::OutputAction { .. } => (io_widget_width(&label), io_widget_height(&label)),
        Widget::InputImage { src, .. } => image_widget_size(src),
        Widget::OutputPreview { preview, expanded, .. } => preview_widget_size(&dag_preview_content_from_dict(preview), expanded),
        Widget::Neuron { id, neuronKind, params, input_ports, .. } => {
            let (inputs, outputs, variadic_inputs, variadic_outputs) =
                neuron_io_layout(id, neuronKind, input_ports, params, synapses, kind_infos);
            let (display_name, abbreviation, _) = widget_display_meta(widget, kind_infos);
            let (normalized_name, _) = normalize_node_display(&display_name, &abbreviation);
            (
                computation_node_width(&normalized_name, &inputs, &outputs),
                computation_node_height(inputs.len(), outputs.len(), variadic_inputs, variadic_outputs),
            )
        }
        Widget::Cluster { id, name, tree, .. } => {
            let (inputs, outputs) = cluster_io_layout(id, name, tree, synapses);
            let (display_name, abbreviation, _) = widget_display_meta(widget, kind_infos);
            let (normalized_name, _) = normalize_node_display(&display_name, &abbreviation);
            (computation_node_width(&normalized_name, &inputs, &outputs), computation_node_height(inputs.len(), outputs.len(), false, false))
        }
    }
}

fn widget_to_dag_node(
    widget: &Widget,
    index: usize,
    layout: &BTreeMap<String, WidgetLayout>,
    synapses: &[SynapseSpec],
    kind_infos: &HashMap<String, OperatorInfo>,
) -> DagNodeSpec {
    let id = match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::Cluster { id, .. } => id.clone(),
    };
    let (width, height) = widget_node_size(widget, synapses, kind_infos);
    let (x, y) = layout.get(&id).map(|p| (p.x, p.y)).unwrap_or(((index as f64) * 200.0, 0.0));
    let (name, abbreviation, icon) = widget_display_meta(widget, kind_infos);
    match widget {
        Widget::Neuron { id: neuron_id, neuronKind, params, input_ports, .. } => {
            let (inputs, outputs, variadic_inputs, variadic_outputs) =
                neuron_io_layout(neuron_id, neuronKind, input_ports, params, synapses, kind_infos);
            DagNodeSpec::computation(id, name, abbreviation, icon, inputs, outputs, variadic_inputs, variadic_outputs, x, y, width, height)
        }
        Widget::InputSlider { value, min, max, step, .. } => DagNodeSpec {
            id,
            name,
            abbreviation,
            icon,
            x,
            y,
            width,
            height,
            kind: DagNodeKind::Slider {
                min: *min,
                max: *max,
                step: *step,
                value: *value,
                output: IoPortSpec::simple("out", "out"),
            },
        },
        Widget::InputNote { text, .. } => DagNodeSpec {
            id,
            name,
            abbreviation,
            icon,
            x,
            y,
            width,
            height,
            kind: DagNodeKind::Note {
                text: text.clone(),
                output: IoPortSpec::simple("out", "out"),
            },
        },
        Widget::InputImage { src, .. } => DagNodeSpec {
            id,
            name,
            abbreviation,
            icon,
            x,
            y,
            width,
            height,
            kind: DagNodeKind::Image {
                src: src.clone(),
                output: IoPortSpec::simple("out", "out"),
            },
        },
        Widget::OutputPreview { preview, expanded, .. } => DagNodeSpec {
            id,
            name,
            abbreviation,
            icon,
            x,
            y,
            width,
            height,
            kind: DagNodeKind::Preview {
                content: dag_preview_content_from_dict(preview),
                expanded: expanded.clone(),
                input: IoPortSpec::simple("in", "in"),
            },
        },
        Widget::OutputAction { action, .. } => DagNodeSpec {
            id,
            name,
            abbreviation,
            icon,
            x,
            y,
            width,
            height,
            kind: DagNodeKind::Action {
                label: action.clone(),
                input: IoPortSpec::simple("in", "in"),
            },
        },
        Widget::Cluster { id: cluster_id, name: cluster_name, tree, .. } => {
            let (inputs, outputs) = cluster_io_layout(cluster_id, cluster_name, tree, synapses);
            DagNodeSpec::cluster(id, name, abbreviation, icon, inputs, outputs, x, y, width, height)
        }
    }
}

fn parse_port_endpoint(endpoint: &str, default_port: &str) -> (String, String) {
    if let Some((widget_id, port_id)) = endpoint.split_once(':') {
        return (widget_id.to_string(), port_id.to_string());
    }
    (endpoint.to_string(), default_port.to_string())
}

const FLOW_SLIDER_MIN: f64 = 0.0;
const FLOW_SLIDER_MAX: f64 = 10.0;
const FLOW_SLIDER_STEP: f64 = 0.1;

fn sensible_slider_max(value: f64) -> f64 {
    let v = value.abs();
    if v <= 1.0 {
        return 1.0;
    }
    if v <= 10.0 {
        return 10.0;
    }
    let magnitude = 10f64.powi(v.log10().floor() as i32);
    let normalized = v / magnitude;
    let nice = if normalized <= 1.0 {
        1.0
    } else if normalized <= 2.0 {
        2.0
    } else if normalized <= 5.0 {
        5.0
    } else {
        10.0
    };
    (nice * magnitude).max(v)
}

fn decimal_places_from_f64(value: f64) -> u32 {
    if (value - value.round()).abs() < 1e-9 {
        return 0;
    }
    for places in 1..=12 {
        let step = 10f64.powi(-(places as i32));
        if ((value / step).round() * step - value).abs() < 1e-9 {
            return places;
        }
    }
    1
}

fn slider_step_from_decimal_places(places: u32) -> f64 {
    if places == 0 { 1.0 } else { 10f64.powi(-(places as i32)) }
}

fn sensible_slider_range(value: f64) -> (f64, f64, f64) {
    let step = slider_step_from_decimal_places(decimal_places_from_f64(value));
    if value < 0.0 {
        let bound = sensible_slider_max(value);
        return (-bound, bound, step);
    }
    (0.0, sensible_slider_max(value), step)
}

fn resolve_input_slider_fields(value: Option<f64>, min: Option<f64>, max: Option<f64>, step: Option<f64>) -> (f64, f64, f64, f64) {
    if let (Some(value), Some(min), Some(max), Some(step)) = (value, min, max, step) {
        let (min, max) = if min <= max { (min, max) } else { (max, min) };
        return (value.clamp(min, max), min, max, step.max(1e-9));
    }
    if let Some(value) = value {
        let (min, max, step) = sensible_slider_range(value);
        return (value.clamp(min, max), min, max, step);
    }
    (default_slider_value(), FLOW_SLIDER_MIN, FLOW_SLIDER_MAX, FLOW_SLIDER_STEP)
}

fn format_preview_number(n: f64) -> String {
    if (n - n.round()).abs() < 0.05 {
        format!("{}", n.round() as i64)
    } else {
        format!("{n:.1}")
    }
}

fn dag_preview_content_from_dict(dict: &Dictionary) -> DagPreviewContent {
    if dict.schema() == Some("number") {
        if let Some(n) = dict.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()) {
            return DagPreviewContent::Scalar { text: format_preview_number(n) };
        }
    }
    if dict.schema() == Some("text") {
        if let Some(t) = dict.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) {
            return DagPreviewContent::Scalar { text: t.to_string() };
        }
    }
    if dict.schema() == Some("image") {
        if let Some(src) = dict.get("dataUrl").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) {
            return DagPreviewContent::Image { src: src.to_string() };
        }
    }
    if let Some(n) = dict.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()) {
        return DagPreviewContent::Scalar { text: format_preview_number(n) };
    }
    if let Some(t) = dict.get("text").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) {
        return DagPreviewContent::Scalar { text: t.to_string() };
    }
    if let Some(src) = dict.get("image").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) {
        return DagPreviewContent::Image { src: src.to_string() };
    }
    if dict.is_empty() {
        return DagPreviewContent::Empty;
    }
    serde_json::to_value(dict)
        .ok()
        .map(|json| DagPreviewContent::Tree { json })
        .unwrap_or(DagPreviewContent::Empty)
}

fn preview_content_summary(content: &DagPreviewContent) -> String {
    match content {
        DagPreviewContent::Empty => "—".into(),
        DagPreviewContent::Scalar { text } => if text.is_empty() { "—".into() } else { text.clone() },
        DagPreviewContent::Image { .. } => "image".into(),
        DagPreviewContent::Tree { json } => preview_tree_collapsed_summary(json),
    }
}

fn preview_tree_collapsed_summary(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Object(map) => format!("{{{} keys}}", map.len()),
        serde_json::Value::Array(arr) => format!("[{} items]", arr.len()),
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => "null".into(),
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum WidgetDescriptor {
    Neuron {
        neuronKind: String,
        #[serde(default)]
        id: Option<String>,
    },
    InputSlider {
        #[serde(default)]
        id: Option<String>,
        #[serde(default)]
        value: Option<f64>,
        #[serde(default)]
        min: Option<f64>,
        #[serde(default)]
        max: Option<f64>,
        #[serde(default)]
        step: Option<f64>,
    },
    InputNote {
        #[serde(default)]
        id: Option<String>,
        #[serde(default)]
        text: Option<String>,
    },
    InputImage {
        #[serde(default)]
        id: Option<String>,
    },
    OutputPreview {
        #[serde(default)]
        id: Option<String>,
    },
    OutputAction {
        #[serde(default)]
        id: Option<String>,
        #[serde(default)]
        action: String,
    },
}

fn descriptor_explicit_id(descriptor: &WidgetDescriptor) -> Option<String> {
    let id = match descriptor {
        WidgetDescriptor::Neuron { id, .. }
        | WidgetDescriptor::InputSlider { id, .. }
        | WidgetDescriptor::InputNote { id, .. }
        | WidgetDescriptor::InputImage { id }
        | WidgetDescriptor::OutputPreview { id }
        | WidgetDescriptor::OutputAction { id, .. } => id.clone(),
    }?;
    let trimmed = id.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn widget_from_descriptor(descriptor: &WidgetDescriptor, id: String, kind_infos: &HashMap<String, OperatorInfo>) -> Widget {
    match descriptor {
        WidgetDescriptor::Neuron { neuronKind, .. } => Widget::Neuron {
            id,
            neuronKind: neuronKind.clone(),
            params: Dictionary::new(),
            input_ports: default_neuron_input_ports(neuronKind, &[], kind_infos),
            preview: true,
        },
        WidgetDescriptor::InputSlider { value, min, max, step, .. } => {
            let (value, min, max, step) = resolve_input_slider_fields(*value, *min, *max, *step);
            Widget::InputSlider { id, value, min, max, step }
        }
        WidgetDescriptor::InputNote { text, .. } => Widget::InputNote { id, text: text.clone().unwrap_or_default() },
        WidgetDescriptor::InputImage { .. } => Widget::InputImage { id, src: String::new() },
        WidgetDescriptor::OutputPreview { .. } => Widget::OutputPreview { id, preview: Dictionary::new(), expanded: BTreeSet::new() },
        WidgetDescriptor::OutputAction { action, .. } => {
            Widget::OutputAction { id, action: if action.is_empty() { "log".into() } else { action.clone() } }
        }
    }
}
// #endregion 🔖Widget

// #region 🔖Catalogue
/// 🌿 Nested catalogue group authored by neuron-kind module authors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogueGroup {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CatalogueItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<CatalogueGroup>,
}

/// 📚 Catalogue section for drag-and-drop palette.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogueSection {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CatalogueItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<CatalogueGroup>,
}

/// 🧷 Draggable catalogue entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogueItem {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neuronKind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    pub name: String,
    pub abbreviation: String,
    pub icon: String,
    pub summary: String,
}

fn static_catalogue_sections() -> Vec<CatalogueSection> {
    vec![
        CatalogueSection {
            id: "inputs".into(),
            title: "Inputs".into(),
            groups: vec![],
            items: vec![
                CatalogueItem { kind: "inputSlider".into(), neuronKind: None, action: None, name: "Slider".into(), abbreviation: "Slider".into(), icon: "emoji:🎚️".into(), summary: "Number input".into() },
                CatalogueItem { kind: "inputNote".into(), neuronKind: None, action: None, name: "Note".into(), abbreviation: "Note".into(), icon: "emoji:📝".into(), summary: "Text input".into() },
                CatalogueItem { kind: "inputImage".into(), neuronKind: None, action: None, name: "Image".into(), abbreviation: "Image".into(), icon: "emoji:🖼️".into(), summary: "Image input".into() },
            ],
        },
        CatalogueSection {
            id: "outputs".into(),
            title: "Outputs".into(),
            groups: vec![],
            items: vec![
                CatalogueItem { kind: "outputPreview".into(), neuronKind: None, action: None, name: "Preview".into(), abbreviation: "Preview".into(), icon: "emoji:👁️".into(), summary: "Preview dictionary".into() },
                CatalogueItem { kind: "outputAction".into(), neuronKind: None, action: Some("log".into()), name: "Action".into(), abbreviation: "Action".into(), icon: "emoji:⚡".into(), summary: "Side-effect action".into() },
            ],
        },
        CatalogueSection {
            id: "contract".into(),
            title: "Contract".into(),
            groups: vec![],
            items: vec![
                CatalogueItem {
                    kind: "neuron".into(),
                    neuronKind: Some(INPUT_KIND.into()),
                    action: None,
                    name: "Input".into(),
                    abbreviation: "In".into(),
                    icon: "emoji:📥".into(),
                    summary: "Cluster input contract channel".into(),
                },
                CatalogueItem {
                    kind: "neuron".into(),
                    neuronKind: Some(OUTPUT_KIND.into()),
                    action: None,
                    name: "Output".into(),
                    abbreviation: "Out".into(),
                    icon: "emoji:📤".into(),
                    summary: "Cluster output contract channel".into(),
                },
            ],
        },
    ]
}

fn merge_catalogue_sections(host_json: &str) -> Result<Vec<CatalogueSection>, String> {
    let mut sections: Vec<CatalogueSection> = if host_json.trim().is_empty() {
        vec![]
    } else {
        serde_json::from_str(host_json).map_err(|e| e.to_string())?
    };
    sections.extend(static_catalogue_sections());
    Ok(sections)
}

fn titleize_module(module: &str) -> String {
    let mut chars = module.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
// #endregion 🔖Catalogue

// #region 🔖ModuleRegistry
fn flow_registry() -> &'static neural::Registry {
    static REGISTRY: OnceLock<neural::Registry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        let mut registry = neural::Registry::new();
        flow_module_core::register(&mut registry);
        flow_module_math::register(&mut registry);
        flow_module_text::register(&mut registry);
        flow_module_logic::register(&mut registry);
        flow_module_dictionary::register(&mut registry);
        flow_module_list::register(&mut registry);
        flow_module_brep::register(&mut registry);
        flow_module_bim::register(&mut registry);
        registry
    })
}
// #endregion 🔖ModuleRegistry

// #region 🔖EvalBridge
fn parse_bridge_dictionary_json(result_json: &str) -> Result<Dictionary, EvalError> {
    if let Ok(dict) = serde_json::from_str::<Dictionary>(result_json) {
        return Ok(dict);
    }
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(result_json) {
        if let Some(err) = value.get("error").and_then(|v| v.as_str()) {
            return Err(EvalError::InvalidInput(err.into()));
        }
    }
    Err(EvalError::InvalidInput("invalid bridge response".into()))
}

#[cfg(target_arch = "wasm32")]
struct EvalBridge {
    cb: js_sys::Function,
}

#[cfg(target_arch = "wasm32")]
impl EvalBridge {
    fn evaluate(&self, kind_id: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        use wasm_bindgen::JsValue;
        let input_json = serde_json::to_string(input).map_err(|e| EvalError::InvalidInput(e.to_string()))?;
        let result = self
            .cb
            .call2(&JsValue::NULL, &JsValue::from_str(kind_id), &JsValue::from_str(&input_json))
            .map_err(|_| EvalError::InvalidInput("bridge call failed".into()))?;
        let result_json = result.as_string().ok_or_else(|| EvalError::InvalidInput("bridge did not return string".into()))?;
        parse_bridge_dictionary_json(&result_json)
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct EvalBridge {
    cb: Box<dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl EvalBridge {
    fn evaluate(&self, kind_id: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        (self.cb)(kind_id, input)
    }
}
// #endregion 🔖EvalBridge

// #region 🔖ChannelEval
fn neural_value_to_json(value: &NeuralValue) -> serde_json::Value {
    serde_json::to_value(value).unwrap_or(serde_json::Value::Null)
}

fn dictionary_to_json_object(dict: &Dictionary) -> serde_json::Map<String, serde_json::Value> {
    dict.keys()
        .map(|key| (key.clone(), neural_value_to_json(dict.get(key).expect("key"))))
        .collect()
}

fn input_ports_json(dict: &Dictionary, kind_info: Option<&OperatorInfo>) -> serde_json::Map<String, serde_json::Value> {
    let mut ports = serde_json::Map::new();
    if let Some(info) = kind_info {
        if let Some(variadic) = &info.variadic_input {
            if let Some(slots) = dict.get(&variadic.slot_key).and_then(|value| value.as_dictionary()) {
                for key in slots.keys() {
                    if let Some(value) = slots.get(key) {
                        ports.insert(key.clone(), neural_value_to_json(value));
                    }
                }
            }
        }
        for port in &info.inputs {
            if port.id == "*" {
                continue;
            }
            if let Some(value) = dict.get(&port.id) {
                ports.insert(port.id.clone(), neural_value_to_json(value));
            }
        }
        return ports;
    }
    dictionary_to_json_object(dict)
}

fn output_ports_json(dict: &Dictionary) -> serde_json::Map<String, serde_json::Value> {
    let mut ports = serde_json::Map::new();
    ports.insert("out".into(), serde_json::to_value(dict).unwrap_or(serde_json::Value::Null));
    ports
}

fn outputs_from_channel_eval_json(json: &str) -> HashMap<String, Dictionary> {
    let Ok(parsed) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(json) else {
        return HashMap::new();
    };
    let mut outputs = HashMap::new();
    for (widget_id, entry) in parsed {
        let Some(out_ports) = entry.get("out").and_then(|value| value.as_object()) else {
            continue;
        };
        let Some(out_val) = out_ports.get("out") else {
            continue;
        };
        if let Ok(dict) = serde_json::from_value::<Dictionary>(out_val.clone()) {
            outputs.insert(widget_id, dict);
        }
    }
    outputs
}

fn widget_kind_info<'a>(widget: &Widget, kind_infos: &'a HashMap<String, OperatorInfo>) -> Option<&'a OperatorInfo> {
    match widget {
        Widget::Neuron { neuronKind, .. } => kind_infos.get(neuronKind),
        _ => None,
    }
}

fn widget_operator_info(widget: &Widget, kind_infos: &HashMap<String, OperatorInfo>) -> Option<OperatorInfo> {
    match widget {
        Widget::Neuron { neuronKind, .. } => kind_infos.get(neuronKind).cloned(),
        Widget::Cluster { id, name, tree, .. } => Some(cluster_operator_info(id, if name.is_empty() { "Cluster" } else { name }, tree)),
        _ => None,
    }
}

fn widget_to_inner_neuron(widget: &Widget) -> Option<Neuron> {
    match widget {
        Widget::Neuron { id, neuronKind, params, .. } => Some(Neuron::with_kind(id, neuronKind, params.clone())),
        Widget::InputSlider { id, value, .. } => Some(Neuron::with_kind(
            id,
            "core.number",
            Dictionary::new().insert("value", NeuralValue::Atom(Atom::Decimal(*value))),
        )),
        Widget::InputNote { id, text } => Some(Neuron::with_kind(
            id,
            "core.text",
            Dictionary::new().insert("value", NeuralValue::Atom(Atom::String(text.clone()))),
        )),
        Widget::InputImage { id, src } => Some(Neuron::with_kind(
            id,
            "core.image",
            Dictionary::new().insert("dataUrl", NeuralValue::Atom(Atom::String(src.clone()))),
        )),
        _ => None,
    }
}

fn contract_boundary_params(channel: &str) -> Dictionary {
    Dictionary::new()
        .insert("channel", NeuralValue::Atom(Atom::String(channel.into())))
        .insert("operators", NeuralValue::Atom(Atom::String("core.number".into())))
}

fn neuron_to_exploded_widget(neuron: &Neuron) -> Widget {
    match neuron.kind.as_str() {
        "core.number" => Widget::InputSlider {
            id: neuron.id.clone(),
            value: neuron
                .params
                .get("value")
                .and_then(|value| value.as_atom())
                .and_then(|atom| atom.as_f64())
                .unwrap_or(3.0),
            min: FLOW_SLIDER_MIN,
            max: FLOW_SLIDER_MAX,
            step: FLOW_SLIDER_STEP,
        },
        "core.text" => Widget::InputNote {
            id: neuron.id.clone(),
            text: neuron
                .params
                .get("value")
                .and_then(|value| value.as_atom())
                .and_then(|atom| atom.as_str())
                .unwrap_or("")
                .into(),
        },
        "core.image" => Widget::InputImage {
            id: neuron.id.clone(),
            src: neuron
                .params
                .get("dataUrl")
                .and_then(|value| value.as_atom())
                .and_then(|atom| atom.as_str())
                .unwrap_or("")
                .into(),
        },
        _ => Widget::Neuron {
            id: neuron.id.clone(),
            neuronKind: neuron.kind.clone(),
            params: neuron.params.clone(),
            input_ports: vec![],
            preview: neuron.kind != INPUT_KIND && neuron.kind != OUTPUT_KIND,
        },
    }
}

fn build_channel_eval_json(fixture: &FlowFixtureV1, channels: &EvalChannels, kind_infos: &HashMap<String, OperatorInfo>) -> String {
    let mut widgets = serde_json::Map::new();
    for widget in &fixture.widgets {
        let id = widget_id_for(widget);
        let operator_info = widget_operator_info(widget, kind_infos);
        let kind_info = operator_info.as_ref();
        let input_dict = match widget {
            Widget::Neuron { params, .. } => channels
                .inputs
                .get(id)
                .cloned()
                .unwrap_or_default()
                .merge(params),
            _ => channels.inputs.get(id).cloned().unwrap_or_default(),
        };
        let output_dict = channels.outputs.get(id);
        let mut entry = serde_json::Map::new();
        entry.insert("in".into(), serde_json::Value::Object(input_ports_json(&input_dict, kind_info)));
        entry.insert(
            "out".into(),
            serde_json::Value::Object(output_dict.map(output_ports_json).unwrap_or_default()),
        );
        if let Some(output) = output_dict {
            if let Some(error) = output.get("error").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()) {
                entry.insert("error".into(), serde_json::Value::String(error.to_string()));
            }
        }
        widgets.insert(id.to_string(), serde_json::Value::Object(entry));
    }
    serde_json::to_string(&widgets).unwrap_or_else(|_| "{}".into())
}

fn collect_geometry_handles_from_value(value: &NeuralValue, handles: &mut Vec<String>) {
    if let Some(dict) = value.as_dictionary() {
        collect_geometry_handles_from_dictionary(dict, handles);
    }
}

fn collect_geometry_handles_from_dictionary(dict: &Dictionary, handles: &mut Vec<String>) {
    if dict.schema() == Some("geometry") {
        if let Some(handle) = dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()) {
            handles.push(handle.to_string());
            return;
        }
    }
    for key in dict.keys() {
        if let Some(value) = dict.get(key) {
            collect_geometry_handles_from_value(value, handles);
        }
    }
}

fn collect_live_geometry_handles(outputs: &HashMap<String, Dictionary>) -> Vec<String> {
    let mut handles = Vec::new();
    for dict in outputs.values() {
        collect_geometry_handles_from_dictionary(dict, &mut handles);
    }
    handles.sort();
    handles.dedup();
    handles
}

fn collect_live_geometry_handles_from_channels(channels: &EvalChannels) -> Vec<String> {
    let mut handles = Vec::new();
    for dict in channels.outputs.values().chain(channels.inputs.values()) {
        collect_geometry_handles_from_dictionary(dict, &mut handles);
    }
    handles.sort();
    handles.dedup();
    handles
}

fn is_global_eval_error_json(json: &str) -> bool {
    let Ok(parsed) = serde_json::from_str::<serde_json::Value>(json) else {
        return true;
    };
    let Some(object) = parsed.as_object() else {
        return true;
    };
    object.len() == 1 && object.contains_key("error")
}
// #endregion 🔖ChannelEval

// #region History
#[derive(Default)]
struct FlowHistory {
    past: Vec<FlowFixtureV1>,
    future: Vec<FlowFixtureV1>,
    pending: Option<FlowFixtureV1>,
}
// #endregion History

// #region 🔖FlowHost
/// 🏠 Retained flow host: fixture, dag scene, evaluation cache.
pub struct FlowHost {
    pub fixture: FlowFixtureV1,
    pub dag: DagHost,
    pub outputs: HashMap<String, Dictionary>,
    pub last_eval_json: String,
    eval_bridge: Option<EvalBridge>,
    host_catalogue_json: String,
    kind_infos: HashMap<String, OperatorInfo>,
    neural_cache: NeuralCache,
    last_tree_signature: Option<u64>,
    next_widget_serial: u64,
    next_synapse_serial: u64,
    viewport_w: u32,
    viewport_h: u32,
    viewport_dpr: f64,
    pan_anchor: Option<(f64, f64, f64, f64)>,
    ghost_node: Option<dag::DagNodeSpec>,
    history: FlowHistory,
}

impl Default for FlowHost {
    fn default() -> Self {
        Self::from_fixture(FlowFixtureV1::default())
    }
}

impl FlowHost {
    pub fn from_fixture(mut fixture: FlowFixtureV1) -> Self {
        dedupe_fixture_widgets(&mut fixture);
        let mut host = Self {
            fixture,
            dag: DagHost::from_fixture(DagFixtureV1 { schema: "dag.fixture/v1".into(), camera: dag::DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] }),
            outputs: HashMap::new(),
            last_eval_json: String::new(),
            eval_bridge: None,
            host_catalogue_json: String::new(),
            kind_infos: HashMap::new(),
            neural_cache: NeuralCache::new(),
            last_tree_signature: None,
            next_widget_serial: 1,
            next_synapse_serial: 100,
            viewport_w: 1,
            viewport_h: 1,
            viewport_dpr: 1.0,
            pan_anchor: None,
            ghost_node: None,
            history: FlowHistory::default(),
        };
        host.rebuild_dag();
        host.touch_channel_eval();
        host
    }

    /// 📥 Replaces fixture content while keeping catalogue, operator metadata, and eval bridge.
    pub fn replace_fixture(&mut self, mut fixture: FlowFixtureV1) {
        dedupe_fixture_widgets(&mut fixture);
        self.fixture = fixture;
        self.outputs.clear();
        self.last_eval_json.clear();
        self.last_tree_signature = None;
        self.pan_anchor = None;
        self.ghost_node = None;
        self.history = FlowHistory::default();
        self.rebuild_dag();
        self.touch_channel_eval();
    }

    pub fn parse_fixture_json(json: &str) -> Result<FlowFixtureV1, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }

    pub fn fixture_json(&self) -> Result<String, String> {
        serde_json::to_string(&self.fixture).map_err(|e| e.to_string())
    }

    pub fn document(&self) -> FlowDocumentV1 {
        self.fixture.to_document()
    }

    pub fn catalogue_json(&self) -> Result<String, String> {
        let sections = merge_catalogue_sections(&self.host_catalogue_json)?;
        serde_json::to_string(&sections).map_err(|e| e.to_string())
    }

    pub fn set_host_catalogue_json(&mut self, json: &str) {
        self.host_catalogue_json = json.to_string();
    }

    pub fn set_neuron_kind_infos_json(&mut self, json: &str) {
        self.kind_infos = if json.trim().is_empty() {
            HashMap::new()
        } else {
            serde_json::from_str::<Vec<OperatorInfo>>(json)
                .map(|items| items.into_iter().map(|info| (info.id.clone(), info)).collect())
                .unwrap_or_default()
        };
        self.rebuild_dag();
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_eval_bridge_fn(&mut self, cb: Box<dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError>>) {
        self.eval_bridge = Some(EvalBridge { cb });
    }

    #[cfg(target_arch = "wasm32")]
    pub fn set_eval_bridge_js(&mut self, cb: js_sys::Function) {
        self.eval_bridge = Some(EvalBridge { cb });
    }

    pub fn evaluate(&mut self) -> Result<String, String> {
        self.evaluate_internal();
        Ok(self.last_eval_json.clone())
    }

    /// 📥 Applies channel-structured eval JSON from an off-thread worker without re-running operators.
    pub fn apply_eval_outputs_json(&mut self, json: &str) {
        if is_global_eval_error_json(json) {
            self.dag.clear_computing();
            return;
        }
        self.last_eval_json = json.to_string();
        let outputs = outputs_from_channel_eval_json(json);
        self.outputs = outputs.clone();
        self.apply_preview_outputs(&outputs);
        self.dag.clear_computing();
    }

    /// ⚙️ Marks one actively computing widget and downstream widgets as stale.
    pub fn set_computing_progress(&mut self, active_widget_id: Option<&str>, stale_widget_ids: &[String]) {
        self.dag.set_computing_progress(active_widget_id, stale_widget_ids);
    }

    /// ✅ Clears computing chrome from all widgets.
    pub fn clear_computing_widget_ids(&mut self) {
        self.dag.clear_computing();
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.viewport_w = width.max(1);
        self.viewport_h = height.max(1);
        self.viewport_dpr = dpr.max(1.0);
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
    }

    pub fn world_from_screen(&self, sx: f64, sy: f64) -> (f64, f64) {
        let p = self.screen_to_world_point(sx, sy);
        (p.x, p.y)
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.fixture.camera = CameraJson { x, y, zoom: zoom.clamp(0.05, 8.0) };
        self.dag.set_camera(x, y, self.fixture.camera.zoom);
    }

    pub fn wheel_zoom_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        let before = self.screen_to_world_point(sx, sy);
        let factor = if delta_y < 0.0 { 1.1 } else { 0.9 };
        let zoom = (self.fixture.camera.zoom * factor).clamp(0.05, 8.0);
        self.fixture.camera.zoom = zoom;
        self.dag.set_camera(self.fixture.camera.x, self.fixture.camera.y, zoom);
        let after = self.screen_to_world_point(sx, sy);
        self.fixture.camera.x += before.x - after.x;
        self.fixture.camera.y += before.y - after.y;
        self.dag.set_camera(self.fixture.camera.x, self.fixture.camera.y, zoom);
    }

    pub fn wheel_pan_screen(&mut self, delta_x: f64, delta_y: f64) {
        let zoom = self.fixture.camera.zoom;
        let x = self.fixture.camera.x - delta_x / zoom;
        let y = self.fixture.camera.y - delta_y / zoom;
        self.set_camera(x, y, zoom);
    }

    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_x: f64, delta_y: f64, zoom_gesture: bool) {
        if zoom_gesture {
            self.wheel_zoom_screen(sx, sy, delta_y);
        } else {
            self.wheel_pan_screen(delta_x, delta_y);
        }
    }

    pub fn set_ghost_widget(&mut self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<(), String> {
        let descriptor: WidgetDescriptor = serde_json::from_str(descriptor_json).map_err(|e| e.to_string())?;
        let id: String = "__ghost__".into();
        let widget = widget_from_descriptor(&descriptor, id.clone(), &self.kind_infos);
        let mut layout = BTreeMap::new();
        layout.insert(id, WidgetLayout { x: world_x, y: world_y });
        let mut node = widget_to_dag_node(&widget, 0, &layout, &[], &self.kind_infos);
        fit_node_size(&mut node);
        self.ghost_node = Some(node.clone());
        self.dag.set_ghost_node(Some(node));
        Ok(())
    }

    pub fn clear_ghost_widget(&mut self) {
        self.ghost_node = None;
        self.dag.set_ghost_node(None);
    }

    pub fn add_widget(&mut self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<String, String> {
        self.begin_change();
        self.clear_ghost_widget();
        let descriptor: WidgetDescriptor = serde_json::from_str(descriptor_json).map_err(|e| e.to_string())?;
        let id = descriptor_explicit_id(&descriptor).unwrap_or_else(|| self.next_widget_id(&descriptor));
        if self.fixture.widgets.iter().any(|widget| widget_id_for(widget) == id) {
            return Err(format!("widget id already exists: {id}"));
        }
        let widget = widget_from_descriptor(&descriptor, id.clone(), &self.kind_infos);
        self.fixture.widgets.push(widget);
        self.fixture.layout.insert(id.clone(), WidgetLayout { x: world_x, y: world_y });
        self.rebuild_dag();
        self.touch_channel_eval();
        Ok(id)
    }

    pub fn remove_widget(&mut self, widget_id: &str) -> Result<(), String> {
        self.begin_change();
        let before = self.fixture.widgets.len();
        self.fixture.widgets.retain(|w| widget_id_for(w) != widget_id);
        if self.fixture.widgets.len() == before {
            return Err(format!("unknown widget: {widget_id}"));
        }
        self.fixture.layout.remove(widget_id);
        self.fixture.synapses.retain(|s| s.from != widget_id && s.to != widget_id);
        self.rebuild_dag();
        self.touch_channel_eval();
        Ok(())
    }

    pub fn move_widget(&mut self, widget_id: &str, x: f64, y: f64) -> Result<(), String> {
        if !self.fixture.widgets.iter().any(|w| widget_id_for(w) == widget_id) {
            return Err(format!("unknown widget: {widget_id}"));
        }
        self.fixture.layout.insert(widget_id.to_string(), WidgetLayout { x, y });
        self.dag.set_widget_position(widget_id, x, y)?;
        Ok(())
    }

    pub fn connect(&mut self, from_id: &str, to_id: &str) -> Result<String, String> {
        self.connect_ports(from_id, "out", to_id, "in")
    }

    pub fn connect_ports(&mut self, from_id: &str, from_port: &str, to_id: &str, to_port: &str) -> Result<String, String> {
        self.begin_change();
        if from_id == to_id {
            return Err("cannot connect widget to itself".into());
        }
        if !widget_has_output(from_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos) {
            return Err(format!("{from_id} has no output port"));
        }
        if !widget_has_input(to_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos) {
            return Err(format!("{to_id} has no input port"));
        }
        let existing: Vec<(String, String)> = self.fixture.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        if would_create_cycle(&existing, from_id, to_id) {
            return Err("connection would create cycle".into());
        }
        if self
            .fixture
            .synapses
            .iter()
            .any(|s| s.from == from_id && s.from_port == from_port && s.to == to_id && s.to_port == to_port)
        {
            return Err("connection already exists".into());
        }
        self.fixture
            .synapses
            .retain(|s| !(s.to == to_id && s.to_port == to_port));
        self.next_synapse_serial += 1;
        let synapse_id = format!("s{}", self.next_synapse_serial);
        self.fixture.synapses.push(SynapseSpec {
            id: synapse_id.clone(),
            from: from_id.to_string(),
            to: to_id.to_string(),
            from_port: from_port.to_string(),
            to_port: to_port.to_string(),
        });
        self.rebuild_dag();
        self.touch_channel_eval();
        Ok(synapse_id)
    }

    pub fn add_input_port(&mut self, widget_id: &str, index: usize) -> Result<(), String> {
        self.begin_change();
        let neuron_kind = self
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuronKind, .. } if id == widget_id => Some(neuronKind.clone()),
                _ => None,
            })
            .ok_or_else(|| format!("unknown neuron widget: {widget_id}"))?;
        let spec = self
            .kind_infos
            .get(&neuron_kind)
            .and_then(|info| info.variadic_input.clone())
            .ok_or_else(|| format!("{widget_id} is not variadic"))?;
        let widget = self.fixture.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| format!("unknown widget: {widget_id}"))?;
        let Widget::Neuron { input_ports, .. } = widget else {
            return Err(format!("{widget_id} is not a neuron"));
        };
        let mut ports = default_neuron_input_ports(&neuron_kind, input_ports, &self.kind_infos);
        if let Some(max) = spec.max {
            if ports.len() >= max {
                return Err(format!("{widget_id} reached max input ports"));
            }
        }
        let insert_at = index.min(ports.len());
        ports.insert(insert_at, insert_at.to_string());
        for synapse in &mut self.fixture.synapses {
            if synapse.to != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.to_port.parse::<usize>() {
                if old_index >= insert_at {
                    synapse.to_port = (old_index + 1).to_string();
                }
            }
        }
        *input_ports = (0..ports.len()).map(|slot| slot.to_string()).collect();
        self.rebuild_dag();
        self.touch_channel_eval();
        Ok(())
    }

    pub fn remove_input_port(&mut self, widget_id: &str, port_id: &str) -> Result<(), String> {
        self.begin_change();
        let neuron_kind = self
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuronKind, .. } if id == widget_id => Some(neuronKind.clone()),
                _ => None,
            })
            .ok_or_else(|| format!("unknown neuron widget: {widget_id}"))?;
        let spec = self
            .kind_infos
            .get(&neuron_kind)
            .and_then(|info| info.variadic_input.clone())
            .ok_or_else(|| format!("{widget_id} is not variadic"))?;
        let widget = self.fixture.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| format!("unknown widget: {widget_id}"))?;
        let Widget::Neuron { input_ports, .. } = widget else {
            return Err(format!("{widget_id} is not a neuron"));
        };
        let ports = default_neuron_input_ports(&neuron_kind, input_ports, &self.kind_infos);
        if ports.len() <= spec.min {
            return Err(format!("{widget_id} requires at least {} inputs", spec.min));
        }
        let Some(remove_index) = ports.iter().position(|port| port == port_id) else {
            return Err(format!("unknown input port: {port_id}"));
        };
        self.fixture.synapses.retain(|synapse| !(synapse.to == widget_id && synapse.to_port == port_id));
        for synapse in &mut self.fixture.synapses {
            if synapse.to != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.to_port.parse::<usize>() {
                if old_index > remove_index {
                    synapse.to_port = (old_index - 1).to_string();
                }
            }
        }
        let mut next_ports = ports;
        next_ports.remove(remove_index);
        *input_ports = (0..next_ports.len()).map(|slot| slot.to_string()).collect();
        self.rebuild_dag();
        self.touch_channel_eval();
        Ok(())
    }

    pub fn disconnect(&mut self, synapse_id: &str) -> Result<(), String> {
        self.begin_change();
        let before = self.fixture.synapses.len();
        self.fixture.synapses.retain(|s| s.id != synapse_id);
        if self.fixture.synapses.len() == before {
            return Err(format!("unknown synapse: {synapse_id}"));
        }
        self.rebuild_dag();
        self.touch_channel_eval();
        Ok(())
    }

    // #region GumballEditing
    /// 🔀 Splices `mid_id` between `anchor_id` and its downstream consumers on `anchor_out_port`.
    pub fn insert_between(
        &mut self,
        anchor_id: &str,
        anchor_out_port: &str,
        mid_id: &str,
        mid_in_port: &str,
        mid_out_port: &str,
    ) -> Result<(), String> {
        self.begin_change();
        if !self.fixture.widgets.iter().any(|widget| widget_id_for(widget) == anchor_id) {
            return Err(format!("unknown widget: {anchor_id}"));
        }
        if !self.fixture.widgets.iter().any(|widget| widget_id_for(widget) == mid_id) {
            return Err(format!("unknown widget: {mid_id}"));
        }
        if anchor_id == mid_id {
            return Err("cannot insert widget between itself".into());
        }
        if !widget_has_output(anchor_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos) {
            return Err(format!("{anchor_id} has no output port"));
        }
        if !widget_has_input(mid_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos) {
            return Err(format!("{mid_id} has no input port"));
        }
        if !widget_has_output(mid_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos) {
            return Err(format!("{mid_id} has no output port"));
        }
        for synapse in &mut self.fixture.synapses {
            if synapse.from == anchor_id && synapse.from_port == anchor_out_port {
                synapse.from = mid_id.to_string();
                synapse.from_port = mid_out_port.to_string();
            }
        }
        let existing: Vec<(String, String)> = self.fixture.synapses.iter().map(|synapse| (synapse.from.clone(), synapse.to.clone())).collect();
        if would_create_cycle(&existing, anchor_id, mid_id) {
            return Err("connection would create cycle".into());
        }
        if self.fixture.synapses.iter().any(|synapse| {
            synapse.from == anchor_id && synapse.from_port == anchor_out_port && synapse.to == mid_id && synapse.to_port == mid_in_port
        }) {
            self.rebuild_dag();
            self.touch_channel_eval();
            return Ok(());
        }
        self.next_synapse_serial += 1;
        let synapse_id = format!("s{}", self.next_synapse_serial);
        self.fixture.synapses.push(SynapseSpec {
            id: synapse_id,
            from: anchor_id.to_string(),
            to: mid_id.to_string(),
            from_port: anchor_out_port.to_string(),
            to_port: mid_in_port.to_string(),
        });
        self.rebuild_dag();
        self.touch_channel_eval();
        Ok(())
    }

    /// ↔️ Shifts widgets to the right of `anchor_id` to open layout space for inserted nodes.
    pub fn make_space(&mut self, anchor_id: &str, dx: f64, dy: f64) -> Result<(), String> {
        self.begin_change();
        let anchor_x = self
            .fixture
            .layout
            .get(anchor_id)
            .map(|layout| layout.x)
            .ok_or_else(|| format!("unknown widget layout: {anchor_id}"))?;
        for (widget_id, layout) in &mut self.fixture.layout {
            if layout.x > anchor_x {
                layout.x += dx;
                layout.y += dy;
            }
            let _ = self.dag.set_widget_position(widget_id, layout.x, layout.y);
        }
        Ok(())
    }

    /// 🧬 Merges JSON params into a neuron widget for compact transform values.
    pub fn set_neuron_params(&mut self, widget_id: &str, params_json: &str) -> Result<(), String> {
        self.begin_change();
        let patch: Dictionary = serde_json::from_str(params_json).map_err(|error| error.to_string())?;
        let widget = self
            .fixture
            .widgets
            .iter_mut()
            .find(|widget| widget_id_for(widget) == widget_id)
            .ok_or_else(|| format!("unknown widget: {widget_id}"))?;
        let Widget::Neuron { params, .. } = widget else {
            return Err(format!("{widget_id} is not a neuron"));
        };
        *params = params.merge(&patch);
        self.sync_dag_display_from_widgets();
        self.touch_channel_eval();
        Ok(())
    }
    // #endregion GumballEditing

    /// 🌳 Recomputes widget positions from the current graph using layered tree layout.
    pub fn reorganize(&mut self, opts_json: &str) -> Result<(), String> {
        self.begin_change();
        let opts: DagLayoutOptions = if opts_json.trim().is_empty() {
            DagLayoutOptions::default()
        } else {
            serde_json::from_str(opts_json).map_err(|e| e.to_string())?
        };
        let theme = self.dag.vello_theme;
        self.dag = DagHost::from_fixture_without_layout(self.build_dag_fixture_v1());
        self.dag.vello_theme = theme;
        self.dag.reorganize(&opts)?;
        self.sync_from_dag();
        Ok(())
    }

    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        if pan {
            self.pan_anchor = Some((sx, sy, self.fixture.camera.x, self.fixture.camera.y));
            return;
        }
        self.clear_ghost_widget();
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.history.pending = Some(self.fixture.clone());
        self.dag.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta, alt);
        if let Some((widget_id, index)) = self.dag.take_pending_port_insert() {
            let _ = self.add_input_port(&widget_id, index);
            return;
        }
        self.sync_from_dag();
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        if let Some((start_sx, start_sy, cam_x, cam_y)) = self.pan_anchor {
            let zoom = self.fixture.camera.zoom;
            let dx = (sx - start_sx) / zoom;
            let dy = (sy - start_sy) / zoom;
            self.set_camera(cam_x - dx, cam_y - dy, zoom);
            return;
        }
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.dag.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt);
        self.sync_from_dag();
        if self.dag.widget_drag_active() {
            self.touch_channel_eval();
        }
    }

    pub fn widget_drag_active(&self) -> bool {
        self.dag.widget_drag_active()
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.pan_anchor = None;
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.dag.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt);
        self.sync_from_dag();
        self.commit_gesture_history();
        self.touch_channel_eval();
    }

    pub fn set_selection_options(&mut self, method: &str, mode: &str) {
        self.dag.set_selection_options(method, mode, true, false, false);
    }

    pub fn selection_preview_points_json(&self) -> String {
        self.dag.selection_preview_points_json()
    }

    pub fn selection_preview_crossing(&self) -> bool {
        self.dag.selection_preview_crossing()
    }

    pub fn preselect_widget_ids_json(&self) -> String {
        serde_json::json!({
            "ids": self.dag.preselect_widget_ids(),
            "removedIds": self.dag.preselect_removed_widget_ids(),
        })
        .to_string()
    }

    pub fn cancel_area_select(&mut self) -> bool {
        let cancelled = self.dag.cancel_area_select();
        if cancelled {
            self.sync_from_dag();
        }
        cancelled
    }

    pub fn delete_selection(&mut self) -> Result<(), String> {
        if !self.dag.has_selection() {
            return Ok(());
        }
        self.begin_change();
        self.dag.delete_selected();
        self.sync_from_dag();
        self.touch_channel_eval();
        Ok(())
    }

    /// ✅ Whether the canvas has any committed node, edge, or handle selection.
    pub fn has_selection(&self) -> bool {
        self.dag.has_selection()
    }

    pub fn select_all(&mut self) {
        self.dag.select_all();
        self.sync_from_dag();
    }

    fn evaluate_internal(&mut self) {
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let signature = tree_signature(&tree, &seeds);
        if self.last_tree_signature == Some(signature) && !self.outputs.is_empty() {
            return;
        }
        self.last_tree_signature = Some(signature);
        let registry = flow_registry();
        let evaluator = Evaluator::new(registry);
        self.neural_cache.begin_epoch();
        let result = if let Some(bridge) = self.eval_bridge.as_ref() {
            let mut dispatch = |kind: &str, input: &Dictionary| bridge.evaluate(kind, input);
            evaluator.evaluate_channels_sequential_cached(&tree, &seeds, &self.kind_infos, &mut dispatch, &self.neural_cache)
        } else {
            let dispatch = |kind: &str, input: &Dictionary| registry.dispatch(kind, input);
            evaluator.evaluate_channels_cached(&tree, &seeds, &self.kind_infos, &dispatch, &self.neural_cache)
        };
        self.neural_cache.sweep();
        match result {
            Ok(channels) => {
                self.outputs = channels.outputs.clone();
                self.apply_preview_outputs(&channels.outputs);
                self.last_eval_json = build_channel_eval_json(&self.fixture, &channels, &self.kind_infos);
                let live_handles = collect_live_geometry_handles_from_channels(&channels);
                flow_module_brep::retain_geometry_handles(&live_handles);
            }
            Err(err) => {
                if self.last_eval_json.is_empty() || is_global_eval_error_json(&self.last_eval_json) {
                    self.last_eval_json = serde_json::json!({ "error": err.to_string() }).to_string();
                }
            }
        }
    }

    /// 🧵 Runs channel eval on native hosts; wasm UI defers to the orchestrator worker.
    fn touch_channel_eval(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        self.evaluate_internal();
    }

    fn build_tree(&self) -> Tree {
        tree_from_fixture(&self.fixture)
    }

    fn build_seeds(&self) -> HashMap<String, Dictionary> {
        let mut seeds = HashMap::new();
        for widget in &self.fixture.widgets {
            match widget {
                Widget::InputSlider { id, value, .. } => {
                    seeds.insert(id.clone(), Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(*value))));
                }
                Widget::InputNote { id, text } => {
                    seeds.insert(id.clone(), Dictionary::with_schema("text").insert("value", NeuralValue::Atom(Atom::String(text.clone()))));
                }
                Widget::InputImage { id, src } => {
                    seeds.insert(id.clone(), Dictionary::with_schema("image").insert("dataUrl", NeuralValue::Atom(Atom::String(src.clone()))));
                }
                _ => {}
            }
        }
        seeds
    }

    fn apply_preview_outputs(&mut self, outputs: &HashMap<String, Dictionary>) {
        for widget in &mut self.fixture.widgets {
            if let Widget::OutputPreview { id, preview, .. } = widget {
                if let Some(out) = outputs.get(id) {
                    *preview = out.clone();
                } else if let Some(syn) = self.fixture.synapses.iter().find(|s| s.to == *id) {
                    if let Some(src) = outputs.get(&syn.from) {
                        *preview = src.clone();
                    }
                }
            }
        }
        self.sync_dag_display_from_widgets();
        self.dag.fit_preview_sizes();
    }

    fn sync_dag_display_from_widgets(&mut self) {
        for widget in &self.fixture.widgets {
            let id = widget_id_for(widget);
            let Some(node) = self.dag.fixture.nodes.iter_mut().find(|n| n.id == *id) else {
                continue;
            };
            match (widget, &mut node.kind) {
                (Widget::InputSlider { value, .. }, DagNodeKind::Slider { value: dag_value, .. }) => {
                    *dag_value = *value;
                }
                (Widget::InputNote { text, .. }, DagNodeKind::Note { text: dag_text, .. }) => {
                    *dag_text = text.clone();
                }
                (Widget::InputImage { src, .. }, DagNodeKind::Image { src: dag_src, .. }) => {
                    *dag_src = src.clone();
                }
                (Widget::OutputPreview { preview, expanded, .. }, DagNodeKind::Preview { content, expanded: dag_expanded, .. }) => {
                    *content = dag_preview_content_from_dict(preview);
                    *dag_expanded = expanded.clone();
                }
                (Widget::OutputAction { action, .. }, DagNodeKind::Action { label, .. }) => {
                    *label = action.clone();
                }
                _ => {}
            }
        }
    }

    fn sync_dag_ghost(&mut self) {
        self.dag.set_ghost_node(self.ghost_node.clone());
    }

    fn rebuild_dag(&mut self) {
        let fixture = self.build_dag_fixture_v1();
        let theme = self.dag.vello_theme;
        let automatic_lod = self.dag.automatic_lod();
        let forced_draw_lod = self.dag.forced_draw_lod_label().map(str::to_string);
        let ghost = self.ghost_node.clone();
        self.dag = DagHost::from_fixture_without_layout(fixture);
        self.dag.vello_theme = theme;
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.dag.set_automatic_lod(automatic_lod);
        if let Some(label) = forced_draw_lod {
            self.dag.set_forced_draw_lod_label(&label);
        }
        self.dag.set_ghost_node(ghost);
        self.sync_preview_dimmed();
        self.sync_from_dag();
    }

    fn sync_preview_dimmed(&mut self) {
        let off = self.preview_off_widget_ids();
        self.dag.set_dimmed(&off);
    }

    /// 🎯 Selected widget ids as JSON array.
    pub fn selected_widget_ids_json(&self) -> String {
        serde_json::to_string(&self.dag.selected_node_ids()).unwrap_or_else(|_| "[]".into())
    }

    /// 🖱️ Hovered widget id when the pointer is over a node or port handle.
    pub fn hovered_widget_id(&self) -> Option<String> {
        self.dag.hovered_node_id()
    }

    /// 🔌 Hovered widget channel when the pointer is over a port row or handle.
    pub fn hovered_channel_json(&self) -> String {
        self.dag.hovered_channel_json()
    }

    /// 🔌 Selected widget channels from handle picks.
    pub fn selected_channels_json(&self) -> String {
        self.dag.selected_channels_json()
    }

    /// ✅ Replaces selection from a JSON array of widget ids.
    pub fn set_selection_json(&mut self, json: &str) {
        let ids: Vec<String> = serde_json::from_str(json).unwrap_or_default();
        self.dag.set_selection(&ids);
    }

    /// 📦 Screen-space union bounds of the current selection for DOM overlays.
    pub fn selection_union_bounds_screen_json(&self) -> String {
        self.dag.selection_union_bounds_screen_json()
    }

    /// 📐 Aligns or distributes the current multi-node selection.
    pub fn align_selection(&mut self, mode: &str) -> Result<(), String> {
        self.begin_change();
        self.dag.align_selection(mode)?;
        self.sync_from_dag();
        self.touch_channel_eval();
        Ok(())
    }

    /// 🖱️ Sets hover to a widget id, or clears hover.
    pub fn set_hover(&mut self, widget_id: Option<&str>) {
        self.dag.set_hover(widget_id);
    }

    /// 🔌 Sets hover to a widget channel, or clears hover.
    pub fn set_hover_channel(&mut self, widget_id: Option<&str>, port_id: Option<&str>) {
        self.dag.set_hover_channel(widget_id, port_id);
    }

    /// 🔌 Replaces channel selection from JSON.
    pub fn set_selected_channels_json(&mut self, json: &str) {
        self.dag.set_selected_channels_json(json);
    }

    /// 🌫️ Widget ids with preview disabled.
    pub fn preview_off_widget_ids(&self) -> Vec<String> {
        self.fixture
            .widgets
            .iter()
            .filter_map(|widget| match widget {
                Widget::Neuron { id, preview: false, .. } => Some(id.clone()),
                _ => None,
            })
            .collect()
    }

    /// 🌫️ Sets preview-off neurons from a JSON array of widget ids.
    pub fn set_preview_off_json(&mut self, json: &str) {
        let ids: Vec<String> = serde_json::from_str(json).unwrap_or_default();
        for widget in &mut self.fixture.widgets {
            if let Widget::Neuron { id, preview, .. } = widget {
                *preview = !ids.contains(id);
            }
        }
        self.sync_preview_dimmed();
    }

    /// 👁️ Toggles preview on a neuron widget.
    pub fn toggle_preview(&mut self, widget_id: &str) -> Result<(), String> {
        let Some(widget) = self.fixture.widgets.iter_mut().find(|w| widget_id_for(w) == widget_id) else {
            return Err(format!("unknown widget: {widget_id}"));
        };
        let Widget::Neuron { preview, .. } = widget else {
            return Err(format!("widget is not a neuron: {widget_id}"));
        };
        *preview = !*preview;
        self.sync_preview_dimmed();
        Ok(())
    }

    fn sync_from_dag(&mut self) {
        let dag_ids: BTreeSet<String> = self.dag.fixture.nodes.iter().map(|node| node.id.clone()).collect();
        self.fixture.widgets.retain(|widget| dag_ids.contains(widget_id_for(widget)));
        for node in &self.dag.fixture.nodes {
            self.fixture.layout.insert(node.id.clone(), WidgetLayout { x: node.x, y: node.y });
        }
        for widget in &mut self.fixture.widgets {
            let id = widget_id_for(widget);
            let Some(node) = self.dag.fixture.nodes.iter().find(|n| n.id == *id) else {
                continue;
            };
            match (widget, &node.kind) {
                (Widget::InputSlider { value, .. }, DagNodeKind::Slider { value: dag_value, .. }) => {
                    *value = *dag_value;
                }
                (Widget::InputNote { text, .. }, DagNodeKind::Note { text: dag_text, .. }) => {
                    *text = dag_text.clone();
                }
                (Widget::InputImage { src, .. }, DagNodeKind::Image { src: dag_src, .. }) => {
                    *src = dag_src.clone();
                }
                (Widget::OutputPreview { expanded, .. }, DagNodeKind::Preview { expanded: dag_expanded, .. }) => {
                    *expanded = dag_expanded.clone();
                }
                (Widget::OutputAction { action, .. }, DagNodeKind::Action { label, .. }) => {
                    *action = label.clone();
                }
                _ => {}
            }
        }
        self.fixture.synapses = self
            .dag
            .fixture
            .edges
            .iter()
            .map(|edge| {
                let (from, from_port) = parse_port_endpoint(&edge.source, "out");
                let (to, to_port) = parse_port_endpoint(&edge.target, "in");
                SynapseSpec {
                    id: edge.id.clone(),
                    from,
                    to,
                    from_port,
                    to_port,
                }
            })
            .collect();
        self.fixture.camera = CameraJson {
            x: self.dag.fixture.camera.x,
            y: self.dag.fixture.camera.y,
            zoom: self.dag.fixture.camera.zoom,
        };
    }

    fn build_dag_fixture_v1(&self) -> DagFixtureV1 {
        let mut seen = BTreeSet::new();
        let nodes: Vec<DagNodeSpec> = self
            .fixture
            .widgets
            .iter()
            .enumerate()
            .filter(|(_, widget)| seen.insert(widget_id_for(widget).to_string()))
            .map(|(i, w)| widget_to_dag_node(w, i, &self.fixture.layout, &self.fixture.synapses, &self.kind_infos))
            .collect();
        let existing: Vec<(String, String)> = self.fixture.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        let edges: Vec<DagFixtureEdgeV1> = self
            .fixture
            .synapses
            .iter()
            .filter(|syn| !would_create_cycle(&existing.iter().filter(|(a, b)| !(a == &syn.from && b == &syn.to)).cloned().collect::<Vec<_>>(), &syn.from, &syn.to))
            .map(|syn| DagFixtureEdgeV1 {
                id: syn.id.clone(),
                source: format!("{}:{}", syn.from, syn.from_port),
                target: format!("{}:{}", syn.to, syn.to_port),
            })
            .collect();
        DagFixtureV1 { schema: "dag.fixture/v1".into(), camera: dag::DagCameraV1 { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom }, nodes, edges }
    }

    fn screen_to_world_point(&self, sx: f64, sy: f64) -> cavas::vello::kurbo::Point {
        use cavas::camera::{screen_to_world, Camera, Viewport};
        use cavas::vello::kurbo::Point;
        let cam = Camera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.viewport_w, height: self.viewport_h, dpr: self.viewport_dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    fn next_widget_id(&mut self, descriptor: &WidgetDescriptor) -> String {
        self.next_widget_serial += 1;
        let prefix = match descriptor {
            WidgetDescriptor::Neuron { neuronKind, .. } => neuronKind.replace('.', "_"),
            WidgetDescriptor::InputSlider { .. } => "slider".into(),
            WidgetDescriptor::InputNote { .. } => "note".into(),
            WidgetDescriptor::InputImage { .. } => "image".into(),
            WidgetDescriptor::OutputPreview { .. } => "preview".into(),
            WidgetDescriptor::OutputAction { .. } => "action".into(),
        };
        format!("{prefix}_{}", self.next_widget_serial)
    }

    pub fn set_slider_value(&mut self, widget_id: &str, value: f64) {
        self.begin_change();
        for widget in &mut self.fixture.widgets {
            if let Widget::InputSlider { id, value: v, min, max, step, .. } = widget {
                if id == widget_id {
                    if value < *min || value > *max {
                        let (new_min, new_max, new_step) = sensible_slider_range(value);
                        *min = new_min;
                        *max = new_max;
                        *step = new_step;
                    }
                    *v = value.clamp(*min, *max);
                }
            }
        }
        self.sync_dag_display_from_widgets();
        let _ = self.evaluate();
    }

    pub fn set_note_text(&mut self, widget_id: &str, text: &str) {
        self.begin_change();
        for widget in &mut self.fixture.widgets {
            if let Widget::InputNote { id, text: note } = widget {
                if id == widget_id {
                    *note = text.to_string();
                }
            }
        }
        self.sync_dag_display_from_widgets();
        self.dag.fit_note_sizes();
        let _ = self.evaluate();
    }

    pub fn set_image_src(&mut self, widget_id: &str, src: &str) {
        self.begin_change();
        for widget in &mut self.fixture.widgets {
            if let Widget::InputImage { id, src: image } = widget {
                if id == widget_id {
                    *image = src.to_string();
                }
            }
        }
        self.sync_dag_display_from_widgets();
        self.dag.fit_preview_sizes();
        let _ = self.evaluate();
    }

    pub fn preview_text(&self) -> String {
        self.fixture
            .widgets
            .iter()
            .find_map(|w| match w {
                Widget::OutputPreview { preview, .. } => Some(preview_content_summary(&dag_preview_content_from_dict(preview))),
                _ => None,
            })
            .unwrap_or_else(|| "—".into())
    }

    pub fn set_vello_theme_from_json(&mut self, json: &str) -> Result<(), String> {
        self.dag.set_vello_theme_from_json(json)
    }

    pub fn paint_scene(&self, scene: &mut cavas::vello::Scene, width: u32, height: u32, dpr: f64) {
        self.dag.paint_scene(scene, width, height, dpr);
    }

    pub fn set_automatic_lod(&mut self, enabled: bool) {
        self.dag.set_automatic_lod(enabled);
    }

    pub fn set_proximity_distance(&mut self, world: f64) {
        self.dag.set_proximity_distance(world);
    }

    pub fn set_forced_draw_lod_label(&mut self, label: &str) {
        self.dag.set_forced_draw_lod_label(label);
    }

    pub fn draw_lod_label(&self) -> &'static str {
        self.dag.draw_lod_label()
    }

    pub fn label_overlay_paint_state_json(&self) -> Result<String, String> {
        self.dag.label_overlay_paint_state_json()
    }

    pub fn param_overlay_paint_state_json(&self) -> Result<String, String> {
        self.dag.param_overlay_paint_state_json()
    }

    /// 💥 Returns and clears a pending cluster explode target from the last pointer hit.
    pub fn take_pending_cluster_explode(&mut self) -> Option<String> {
        self.dag.take_pending_cluster_explode()
    }

    /// 🧩 Collapses the selected widgets into one cluster neuron.
    pub fn collapse_selection(&mut self, selected_ids: &[String]) -> Result<String, String> {
        if selected_ids.len() < 2 {
            return Err("select at least two widgets to collapse".into());
        }
        let selected: BTreeSet<String> = selected_ids.iter().cloned().collect();
        if !selected.iter().all(|id| self.fixture.widgets.iter().any(|widget| widget_id_for(widget) == id)) {
            return Err("selection contains unknown widgets".into());
        }
        if selected.iter().any(|id| {
            self.fixture
                .widgets
                .iter()
                .any(|widget| widget_id_for(widget) == id && matches!(widget, Widget::Cluster { .. }))
        }) {
            return Err("cannot collapse clusters".into());
        }
        self.begin_change();
        let mut inner_neurons = Vec::new();
        let mut inner_layout = BTreeMap::new();
        for widget in &self.fixture.widgets {
            let id = widget_id_for(widget).to_string();
            if !selected.contains(&id) {
                continue;
            }
            if let Some(neuron) = widget_to_inner_neuron(widget) {
                inner_neurons.push(neuron);
            }
            if let Some(layout) = self.fixture.layout.get(&id) {
                inner_layout.insert(id, layout.clone());
            }
        }
        let mut inner_synapses = Vec::new();
        let mut crossing_external = Vec::new();
        let mut retained_external = Vec::new();
        for synapse in &self.fixture.synapses {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if from_selected && to_selected {
                inner_synapses.push(Synapse {
                    id: synapse.id.clone(),
                    from: synapse.from.clone(),
                    to: synapse.to.clone(),
                    from_port: synapse.from_port.clone(),
                    to_port: synapse.to_port.clone(),
                });
            } else if from_selected || to_selected {
                crossing_external.push(synapse.clone());
            } else {
                retained_external.push(synapse.clone());
            }
        }
        let mut boundary_index = 0usize;
        let mut cluster_external = Vec::new();
        for synapse in crossing_external {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if to_selected && !from_selected {
                boundary_index += 1;
                let boundary_id = format!("__in_{boundary_index}");
                let channel = if synapse.to_port.is_empty() || synapse.to_port == "in" {
                    synapse.to.clone()
                } else {
                    synapse.to_port.clone()
                };
                inner_neurons.push(Neuron::with_kind(&boundary_id, INPUT_KIND, contract_boundary_params(&channel)));
                inner_synapses.push(Synapse {
                    id: format!("{boundary_id}_link"),
                    from: boundary_id.clone(),
                    to: synapse.to.clone(),
                    from_port: "out".into(),
                    to_port: synapse.to_port.clone(),
                });
                cluster_external.push(SynapseSpec {
                    id: synapse.id.clone(),
                    from: synapse.from.clone(),
                    to: String::new(),
                    from_port: synapse.from_port.clone(),
                    to_port: channel,
                });
            } else if from_selected && !to_selected {
                boundary_index += 1;
                let boundary_id = format!("__out_{boundary_index}");
                let channel = if synapse.from_port.is_empty() || synapse.from_port == "out" {
                    synapse.from.clone()
                } else {
                    synapse.from_port.clone()
                };
                inner_neurons.push(Neuron::with_kind(&boundary_id, OUTPUT_KIND, contract_boundary_params(&channel)));
                inner_synapses.push(Synapse {
                    id: format!("{boundary_id}_link"),
                    from: synapse.from.clone(),
                    to: boundary_id.clone(),
                    from_port: synapse.from_port.clone(),
                    to_port: "in".into(),
                });
                cluster_external.push(SynapseSpec {
                    id: synapse.id.clone(),
                    from: String::new(),
                    to: synapse.to.clone(),
                    from_port: channel,
                    to_port: synapse.to_port.clone(),
                });
            }
        }
        let (sum_x, sum_y, layout_count) = selected
            .iter()
            .filter_map(|id| self.fixture.layout.get(id))
            .fold((0.0, 0.0, 0usize), |(sx, sy, count), layout| (sx + layout.x, sy + layout.y, count + 1));
        let count = layout_count.max(1) as f64;
        let cluster_x = sum_x / count;
        let cluster_y = sum_y / count;
        self.next_widget_serial += 1;
        let cluster_id = format!("cluster_{}", self.next_widget_serial);
        let inner_tree = Tree { neurons: inner_neurons, synapses: inner_synapses };
        let cluster = Widget::Cluster {
            id: cluster_id.clone(),
            name: "Cluster".into(),
            tree: inner_tree,
            flow: FlowGuiV1 {
                camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
                nodes: inner_layout
                    .into_iter()
                    .map(|(id, layout)| (id, FlowNodeGui { layout, chrome: NodeChrome::Plain }))
                    .collect(),
                previews: vec![],
            },
        };
        self.fixture.widgets.retain(|widget| !selected.contains(&widget_id_for(widget).to_string()));
        self.fixture.widgets.push(cluster);
        for id in &selected {
            self.fixture.layout.remove(id);
        }
        self.fixture.layout.insert(cluster_id.clone(), WidgetLayout { x: cluster_x, y: cluster_y });
        self.fixture.synapses = retained_external;
        for synapse in cluster_external {
            if synapse.to.is_empty() {
                self.fixture.synapses.push(SynapseSpec {
                    id: synapse.id,
                    from: synapse.from,
                    to: cluster_id.clone(),
                    from_port: synapse.from_port,
                    to_port: synapse.to_port,
                });
            } else {
                self.fixture.synapses.push(SynapseSpec {
                    id: synapse.id,
                    from: cluster_id.clone(),
                    to: synapse.to,
                    from_port: synapse.from_port,
                    to_port: synapse.to_port,
                });
            }
        }
        self.rebuild_dag();
        self.touch_channel_eval();
        Ok(cluster_id)
    }

    /// 💥 Explodes a cluster back into its inner widgets.
    pub fn explode_cluster(&mut self, cluster_id: &str) -> Result<(), String> {
        let cluster_index = self
            .fixture
            .widgets
            .iter()
            .position(|widget| matches!(widget, Widget::Cluster { id, .. } if id == cluster_id))
            .ok_or_else(|| format!("unknown cluster: {cluster_id}"))?;
        let Widget::Cluster { tree, flow, .. } = self.fixture.widgets[cluster_index].clone() else {
            return Err(format!("widget is not a cluster: {cluster_id}"));
        };
        let cluster_layout = self.fixture.layout.get(cluster_id).cloned().unwrap_or(WidgetLayout { x: 0.0, y: 0.0 });
        self.begin_change();
        let mut boundary_channels: HashMap<String, (String, String)> = HashMap::new();
        for neuron in &tree.neurons {
            if neuron.kind == INPUT_KIND {
                let channel = neuron
                    .params
                    .get("channel")
                    .and_then(|value| value.as_atom())
                    .and_then(|atom| atom.as_str())
                    .unwrap_or(neuron.id.as_str())
                    .to_string();
                if let Some(synapse) = tree.synapses.iter().find(|syn| syn.from == neuron.id) {
                    boundary_channels.insert(channel, (synapse.to.clone(), synapse.to_port.clone()));
                }
            } else if neuron.kind == OUTPUT_KIND {
                let channel = neuron
                    .params
                    .get("channel")
                    .and_then(|value| value.as_atom())
                    .and_then(|atom| atom.as_str())
                    .unwrap_or(neuron.id.as_str())
                    .to_string();
                if let Some(synapse) = tree.synapses.iter().find(|syn| syn.to == neuron.id) {
                    boundary_channels.insert(channel, (synapse.from.clone(), synapse.from_port.clone()));
                }
            }
        }
        let mut restored_widgets = Vec::new();
        for neuron in &tree.neurons {
            if neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND {
                continue;
            }
            let namespaced_id = format!("{cluster_id}/{}", neuron.id);
            let mut widget = neuron_to_exploded_widget(neuron);
            match &mut widget {
                Widget::Neuron { id, .. }
                | Widget::InputSlider { id, .. }
                | Widget::InputNote { id, .. }
                | Widget::InputImage { id, .. } => *id = namespaced_id.clone(),
                _ => {}
            }
            let layout = flow
                .nodes
                .get(&neuron.id)
                .map(|node| node.layout.clone())
                .unwrap_or(WidgetLayout { x: 0.0, y: 0.0 });
            self.fixture.layout.insert(
                namespaced_id.clone(),
                WidgetLayout {
                    x: cluster_layout.x + layout.x,
                    y: cluster_layout.y + layout.y,
                },
            );
            restored_widgets.push((namespaced_id, neuron.id.clone(), widget));
        }
        let id_map: HashMap<String, String> = restored_widgets.iter().map(|(namespaced, original, _)| (original.clone(), namespaced.clone())).collect();
        self.fixture.widgets.remove(cluster_index);
        self.fixture.layout.remove(cluster_id);
        for (_, _, widget) in restored_widgets {
            self.fixture.widgets.push(widget);
        }
        let mut next_synapses = Vec::new();
        for synapse in &self.fixture.synapses {
            if synapse.to == cluster_id {
                if let Some((inner_to, inner_to_port)) = boundary_channels.get(&synapse.to_port) {
                    if let Some(mapped_to) = id_map.get(inner_to) {
                        next_synapses.push(SynapseSpec {
                            id: synapse.id.clone(),
                            from: synapse.from.clone(),
                            to: mapped_to.clone(),
                            from_port: synapse.from_port.clone(),
                            to_port: inner_to_port.clone(),
                        });
                        continue;
                    }
                }
            } else if synapse.from == cluster_id {
                if let Some((inner_from, inner_from_port)) = boundary_channels.get(&synapse.from_port) {
                    if let Some(mapped_from) = id_map.get(inner_from) {
                        next_synapses.push(SynapseSpec {
                            id: synapse.id.clone(),
                            from: mapped_from.clone(),
                            to: synapse.to.clone(),
                            from_port: inner_from_port.clone(),
                            to_port: synapse.to_port.clone(),
                        });
                        continue;
                    }
                }
            } else {
                next_synapses.push(synapse.clone());
            }
        }
        for synapse in &tree.synapses {
            if tree.neurons.iter().any(|neuron| neuron.id == synapse.from && (neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND)) {
                continue;
            }
            if tree.neurons.iter().any(|neuron| neuron.id == synapse.to && (neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND)) {
                continue;
            }
            let Some(from) = id_map.get(&synapse.from) else { continue };
            let Some(to) = id_map.get(&synapse.to) else { continue };
            self.next_synapse_serial += 1;
            next_synapses.push(SynapseSpec {
                id: format!("s{}", self.next_synapse_serial),
                from: from.clone(),
                to: to.clone(),
                from_port: synapse.from_port.clone(),
                to_port: synapse.to_port.clone(),
            });
        }
        self.fixture.synapses = next_synapses;
        self.rebuild_dag();
        self.touch_channel_eval();
        Ok(())
    }

    // #region History
    fn content_changed(a: &FlowFixtureV1, b: &FlowFixtureV1) -> bool {
        a.widgets != b.widgets || a.synapses != b.synapses || a.layout != b.layout
    }

    fn begin_change(&mut self) {
        if self.history.pending.is_none() {
            self.history.past.push(self.fixture.clone());
            self.history.future.clear();
        }
    }

    fn commit_gesture_history(&mut self) {
        if let Some(pre) = self.history.pending.take() {
            if Self::content_changed(&pre, &self.fixture) {
                self.history.past.push(pre);
                self.history.future.clear();
            }
        }
    }

    /// ↩️ Restores the previous fixture content snapshot, keeping the current camera.
    pub fn undo(&mut self) -> bool {
        let Some(prev) = self.history.past.pop() else {
            return false;
        };
        let camera = self.fixture.camera.clone();
        self.history.future.push(self.fixture.clone());
        self.fixture = prev;
        self.fixture.camera = camera;
        self.rebuild_dag();
        self.touch_channel_eval();
        true
    }

    /// ↪️ Re-applies a fixture content snapshot undone earlier, keeping the current camera.
    pub fn redo(&mut self) -> bool {
        let Some(next) = self.history.future.pop() else {
            return false;
        };
        let camera = self.fixture.camera.clone();
        self.history.past.push(self.fixture.clone());
        self.fixture = next;
        self.fixture.camera = camera;
        self.rebuild_dag();
        self.touch_channel_eval();
        true
    }

    /// ↩️ Whether a content undo step is available.
    pub fn can_undo(&self) -> bool {
        !self.history.past.is_empty()
    }

    /// ↪️ Whether a content redo step is available.
    pub fn can_redo(&self) -> bool {
        !self.history.future.is_empty()
    }
    // #endregion History
}

fn dedupe_fixture_widgets(fixture: &mut FlowFixtureV1) {
    let mut seen = BTreeSet::new();
    fixture.widgets.retain(|widget| seen.insert(widget_id_for(widget).to_string()));
}

fn widget_id_for(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}

fn widget_has_output(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> bool {
    widgets.iter().any(|w| widget_id_for(w) == widget_id && !widget_io_ports(w, synapses, kind_infos).1.is_empty())
}

fn widget_has_input(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> bool {
    widgets.iter().any(|w| widget_id_for(w) == widget_id && !widget_io_ports(w, synapses, kind_infos).0.is_empty())
}
// #endregion 🔖FlowHost

// #region 🔖WasmSession
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::future_to_promise;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

#[cfg(target_arch = "wasm32")]
struct FlowSessionInner {
    host: FlowHost,
    gpu: cavas::gpu_session::CanvasGpuSession,
    width: u32,
    height: u32,
    dpr: f64,
}

#[cfg(target_arch = "wasm32")]
impl FlowSessionInner {
    fn set_logical_size_and_maybe_resize_surface(&mut self, lw: u32, lh: u32, dpr: f64, pw: u32, ph: u32) {
        self.width = lw;
        self.height = lh;
        self.dpr = dpr;
        self.host.set_viewport(lw, lh, dpr);
        self.gpu.resize_surface(pw, ph);
    }

    fn render_frame_gpu(&mut self) -> Result<(), JsValue> {
        self.host.sync_dag_ghost();
        let mut scene = cavas::vello::Scene::new();
        let clear = self.host.dag.vello_theme.raster_clear;
        self.host.paint_scene(&mut scene, self.width, self.height, self.dpr);
        let scene = cavas::render::scale_scene_for_device_pixel_ratio(scene, self.dpr);
        self.gpu.render_frame(&scene, clear)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct FlowSession {
    state: Rc<RefCell<FlowSessionInner>>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl FlowSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { state: Rc::new(RefCell::new(FlowSessionInner { host: FlowHost::default(), gpu: cavas::gpu_session::CanvasGpuSession::default(), width: 1, height: 1, dpr: 1.0 })) }
    }

    #[wasm_bindgen(js_name = loadFixtureJson)]
    pub fn load_fixture_json(&self, json: &str) -> Result<(), JsValue> {
        let fixture = FlowHost::parse_fixture_json(json).map_err(|e| JsValue::from_str(&e))?;
        let mut inner = self.state.borrow_mut();
        inner.host.replace_fixture(fixture);
        Ok(())
    }

    #[wasm_bindgen(js_name = fixtureJson)]
    pub fn fixture_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.fixture_json().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = catalogueJson)]
    pub fn catalogue_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.catalogue_json().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = setEvalBridge)]
    pub fn set_eval_bridge(&self, cb: js_sys::Function) {
        self.state.borrow_mut().host.set_eval_bridge_js(cb);
    }

    #[wasm_bindgen(js_name = setCatalogueJson)]
    pub fn set_catalogue_json(&self, json: &str) {
        self.state.borrow_mut().host.set_host_catalogue_json(json);
    }

    #[wasm_bindgen(js_name = setNeuronKindInfosJson)]
    pub fn set_neuron_kind_infos_json(&self, json: &str) {
        self.state.borrow_mut().host.set_neuron_kind_infos_json(json);
    }

    #[wasm_bindgen(js_name = addInputPort)]
    pub fn add_input_port(&self, widget_id: &str, index: u32) -> Result<(), JsValue> {
        self.state
            .borrow_mut()
            .host
            .add_input_port(widget_id, index as usize)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = removeInputPort)]
    pub fn remove_input_port(&self, widget_id: &str, port_id: &str) -> Result<(), JsValue> {
        self.state
            .borrow_mut()
            .host
            .remove_input_port(widget_id, port_id)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = connectPorts)]
    pub fn connect_ports(&self, from_id: &str, from_port: &str, to_id: &str, to_port: &str) -> Result<String, JsValue> {
        self.state
            .borrow_mut()
            .host
            .connect_ports(from_id, from_port, to_id, to_port)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = evaluate)]
    pub fn evaluate(&self) -> js_sys::Promise {
        let state = self.state.clone();
        future_to_promise(async move {
            let json = {
                let mut inner = state.borrow_mut();
                inner.host.evaluate_internal();
                inner.host.last_eval_json.clone()
            };
            Ok(JsValue::from_str(&json))
        })
    }

    #[wasm_bindgen(js_name = evaluateSync)]
    pub fn evaluate_sync(&self) -> Result<String, JsValue> {
        self.state.borrow_mut().host.evaluate().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = applyEvalOutputsJson)]
    pub fn apply_eval_outputs_json(&self, json: &str) {
        self.state.borrow_mut().host.apply_eval_outputs_json(json);
    }

    #[wasm_bindgen(js_name = setComputingProgress)]
    pub fn set_computing_progress(&self, json: &str) {
        let payload: serde_json::Value = serde_json::from_str(json).unwrap_or(serde_json::Value::Null);
        let active = payload.get("active").and_then(|value| value.as_str()).map(str::to_string);
        let stale: Vec<String> = payload
            .get("stale")
            .and_then(|value| value.as_array())
            .map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect())
            .unwrap_or_default();
        self.state
            .borrow_mut()
            .host
            .set_computing_progress(active.as_deref(), &stale);
    }

    #[wasm_bindgen(js_name = clearComputingWidgetIds)]
    pub fn clear_computing_widget_ids(&self) {
        self.state.borrow_mut().host.clear_computing_widget_ids();
    }

    #[wasm_bindgen(js_name = previewText)]
    pub fn preview_text(&self) -> String {
        self.state.borrow().host.preview_text()
    }

    #[wasm_bindgen(js_name = selectedWidgetIds)]
    pub fn selected_widget_ids(&self) -> String {
        self.state.borrow().host.selected_widget_ids_json()
    }

    #[wasm_bindgen(js_name = hoveredWidgetId)]
    pub fn hovered_widget_id(&self) -> Option<String> {
        self.state.borrow().host.hovered_widget_id()
    }

    #[wasm_bindgen(js_name = hoveredChannelJson)]
    pub fn hovered_channel_json(&self) -> String {
        self.state.borrow().host.hovered_channel_json()
    }

    #[wasm_bindgen(js_name = selectedChannelsJson)]
    pub fn selected_channels_json(&self) -> String {
        self.state.borrow().host.selected_channels_json()
    }

    #[wasm_bindgen(js_name = previewOffWidgetIds)]
    pub fn preview_off_widget_ids(&self) -> String {
        serde_json::to_string(&self.state.borrow().host.preview_off_widget_ids()).unwrap_or_else(|_| "[]".into())
    }

    #[wasm_bindgen(js_name = setSelection)]
    pub fn set_selection(&self, json: &str) {
        self.state.borrow_mut().host.set_selection_json(json);
    }

    #[wasm_bindgen(js_name = setHover)]
    pub fn set_hover(&self, widget_id: Option<String>) {
        self.state.borrow_mut().host.set_hover(widget_id.as_deref());
    }

    #[wasm_bindgen(js_name = setHoverChannel)]
    pub fn set_hover_channel(&self, widget_id: Option<String>, port: Option<String>) {
        self.state
            .borrow_mut()
            .host
            .set_hover_channel(widget_id.as_deref(), port.as_deref());
    }

    #[wasm_bindgen(js_name = setSelectedChannels)]
    pub fn set_selected_channels(&self, json: &str) {
        self.state.borrow_mut().host.set_selected_channels_json(json);
    }

    #[wasm_bindgen(js_name = setPreviewOff)]
    pub fn set_preview_off(&self, json: &str) {
        self.state.borrow_mut().host.set_preview_off_json(json);
    }

    #[wasm_bindgen(js_name = togglePreview)]
    pub fn toggle_preview(&self, widget_id: &str) -> Result<(), JsValue> {
        self.state
            .borrow_mut()
            .host
            .toggle_preview(widget_id)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = collapseSelection)]
    pub fn collapse_selection(&self, ids_json: &str) -> Result<String, JsValue> {
        let ids: Vec<String> = serde_json::from_str(ids_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
        self.state
            .borrow_mut()
            .host
            .collapse_selection(&ids)
            .map_err(|err| JsValue::from_str(&err))
    }

    #[wasm_bindgen(js_name = explodeCluster)]
    pub fn explode_cluster(&self, cluster_id: &str) -> Result<(), JsValue> {
        self.state
            .borrow_mut()
            .host
            .explode_cluster(cluster_id)
            .map_err(|err| JsValue::from_str(&err))
    }

    #[wasm_bindgen(js_name = takePendingClusterExplode)]
    pub fn take_pending_cluster_explode(&self) -> Option<String> {
        self.state.borrow_mut().host.take_pending_cluster_explode()
    }

    #[wasm_bindgen(js_name = setSliderValue)]
    pub fn set_slider_value(&self, widget_id: &str, value: f64) {
        self.state.borrow_mut().host.set_slider_value(widget_id, value);
    }

    #[wasm_bindgen(js_name = setNoteText)]
    pub fn set_note_text(&self, widget_id: &str, text: &str) {
        self.state.borrow_mut().host.set_note_text(widget_id, text);
    }

    #[wasm_bindgen(js_name = setImageSrc)]
    pub fn set_image_src(&self, widget_id: &str, src: &str) {
        self.state.borrow_mut().host.set_image_src(widget_id, src);
    }

    #[wasm_bindgen(js_name = addWidget)]
    pub fn add_widget(&self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<String, JsValue> {
        self.state.borrow_mut().host.add_widget(descriptor_json, world_x, world_y).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = setGhostWidget)]
    pub fn set_ghost_widget(&self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<(), JsValue> {
        self.state
            .borrow_mut()
            .host
            .set_ghost_widget(descriptor_json, world_x, world_y)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = clearGhostWidget)]
    pub fn clear_ghost_widget(&self) {
        self.state.borrow_mut().host.clear_ghost_widget();
    }

    #[wasm_bindgen(js_name = removeWidget)]
    pub fn remove_widget(&self, widget_id: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.remove_widget(widget_id).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = moveWidget)]
    pub fn move_widget(&self, widget_id: &str, x: f64, y: f64) -> Result<(), JsValue> {
        self.state.borrow_mut().host.move_widget(widget_id, x, y).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = insertBetween)]
    pub fn insert_between(
        &self,
        anchor_id: &str,
        anchor_out_port: &str,
        mid_id: &str,
        mid_in_port: &str,
        mid_out_port: &str,
    ) -> Result<(), JsValue> {
        self.state
            .borrow_mut()
            .host
            .insert_between(anchor_id, anchor_out_port, mid_id, mid_in_port, mid_out_port)
            .map_err(|error| JsValue::from_str(&error))
    }

    #[wasm_bindgen(js_name = makeSpace)]
    pub fn make_space(&self, anchor_id: &str, dx: f64, dy: f64) -> Result<(), JsValue> {
        self.state
            .borrow_mut()
            .host
            .make_space(anchor_id, dx, dy)
            .map_err(|error| JsValue::from_str(&error))
    }

    #[wasm_bindgen(js_name = setNeuronParams)]
    pub fn set_neuron_params(&self, widget_id: &str, params_json: &str) -> Result<(), JsValue> {
        self.state
            .borrow_mut()
            .host
            .set_neuron_params(widget_id, params_json)
            .map_err(|error| JsValue::from_str(&error))
    }

    #[wasm_bindgen(js_name = connect)]
    pub fn connect(&self, from_id: &str, to_id: &str) -> Result<String, JsValue> {
        self.state.borrow_mut().host.connect(from_id, to_id).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = disconnect)]
    pub fn disconnect(&self, synapse_id: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.disconnect(synapse_id).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = undo)]
    pub fn undo(&self) -> bool {
        self.state.borrow_mut().host.undo()
    }

    #[wasm_bindgen(js_name = redo)]
    pub fn redo(&self) -> bool {
        self.state.borrow_mut().host.redo()
    }

    #[wasm_bindgen(js_name = canUndo)]
    pub fn can_undo(&self) -> bool {
        self.state.borrow().host.can_undo()
    }

    #[wasm_bindgen(js_name = canRedo)]
    pub fn can_redo(&self) -> bool {
        self.state.borrow().host.can_redo()
    }

    #[wasm_bindgen(js_name = worldFromScreen)]
    pub fn world_from_screen(&self, sx: f64, sy: f64) -> String {
        let (x, y) = self.state.borrow().host.world_from_screen(sx, sy);
        serde_json::json!({ "x": x, "y": y }).to_string()
    }

    #[wasm_bindgen(js_name = setCamera)]
    pub fn set_camera(&self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.set_camera(x, y, zoom);
    }

    #[wasm_bindgen(js_name = wheelScreen)]
    pub fn wheel_screen(&self, sx: f64, sy: f64, delta_x: f64, delta_y: f64, zoom_gesture: bool) {
        self.state.borrow_mut().host.wheel_screen(sx, sy, delta_x, delta_y, zoom_gesture);
    }

    #[wasm_bindgen(js_name = setWheelZoomActive)]
    pub fn set_wheel_zoom_active(&self, active: bool) {
        self.state.borrow_mut().host.dag.set_wheel_zoom_active(active);
    }

    #[wasm_bindgen(js_name = lodScaleJson)]
    pub fn lod_scale_json(&self) -> String {
        dag::dag_lod_scale_json()
    }

    #[wasm_bindgen(js_name = setAutomaticLod)]
    pub fn set_automatic_lod(&self, enabled: bool) {
        self.state.borrow_mut().host.set_automatic_lod(enabled);
    }

    #[wasm_bindgen(js_name = setProximityDistance)]
    pub fn set_proximity_distance(&self, world: f64) {
        self.state.borrow_mut().host.set_proximity_distance(world);
    }

    #[wasm_bindgen(js_name = setForcedDrawLodLabel)]
    pub fn set_forced_draw_lod_label(&self, label: &str) {
        self.state.borrow_mut().host.set_forced_draw_lod_label(label);
    }

    #[wasm_bindgen(js_name = drawLodLabel)]
    pub fn draw_lod_label(&self) -> String {
        self.state.borrow().host.draw_lod_label().to_string()
    }

    #[wasm_bindgen(js_name = labelOverlayPaintStateJson)]
    pub fn label_overlay_paint_state_json(&self) -> Result<String, JsValue> {
        self.state
            .borrow()
            .host
            .label_overlay_paint_state_json()
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = paramOverlayPaintStateJson)]
    pub fn param_overlay_paint_state_json(&self) -> Result<String, JsValue> {
        self.state
            .borrow()
            .host
            .param_overlay_paint_state_json()
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = attachCanvas)]
    pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> js_sys::Promise {
        let inner = self.state.clone();
        if inner.borrow().gpu.gpu_ready() {
            return future_to_promise(async move { Err(JsValue::from_str("canvas surface already attached")) });
        }
        let lw = logical_w.max(1);
        let lh = logical_h.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        let canvas = canvas.clone();
        future_to_promise(async move {
            let (render_ctx, renderer, surface) = cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph)
                .await
                .map_err(|err| JsValue::from_str(&err))?;
            let mut g = inner.borrow_mut();
            if g.gpu.gpu_ready() {
                return Err(JsValue::from_str("canvas surface already attached"));
            }
            g.set_logical_size_and_maybe_resize_surface(lw, lh, dpr, pw, ph);
            g.gpu.finish_attach(canvas, render_ctx, renderer, surface);
            Ok(JsValue::UNDEFINED)
        })
    }

    #[wasm_bindgen(js_name = gpuReady)]
    pub fn gpu_ready(&self) -> bool {
        self.state.borrow().gpu.gpu_ready()
    }

    #[wasm_bindgen(js_name = setSize)]
    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        let lw = width.max(1);
        let lh = height.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        let mut inner = self.state.borrow_mut();
        inner.set_logical_size_and_maybe_resize_surface(lw, lh, dpr, pw, ph);
    }

    #[wasm_bindgen(js_name = setVelloThemeJson)]
    pub fn set_vello_theme_json(&mut self, json: &str) {
        let _ = self.state.borrow_mut().host.set_vello_theme_from_json(json);
    }

    #[wasm_bindgen(js_name = reorganize)]
    pub fn reorganize(&self, options_json: &str) -> Result<(), JsValue> {
        self.state
            .borrow_mut()
            .host
            .reorganize(options_json)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&mut self) -> Result<(), JsValue> {
        self.state.borrow_mut().render_frame_gpu()
    }

    #[wasm_bindgen(js_name = pointerDownScreen)]
    pub fn pointer_down_screen(&self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        self.state.borrow_mut().host.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta, alt, pan);
    }

    #[wasm_bindgen(js_name = pointerMoveScreen)]
    pub fn pointer_move_screen(&self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.state.borrow_mut().host.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt);
    }

    #[wasm_bindgen(js_name = widgetDragActive)]
    pub fn widget_drag_active(&self) -> bool {
        self.state.borrow().host.widget_drag_active()
    }

    #[wasm_bindgen(js_name = pointerUpScreen)]
    pub fn pointer_up_screen(&self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.state.borrow_mut().host.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt);
    }

    #[wasm_bindgen(js_name = setSelectionOptions)]
    pub fn set_selection_options(&self, method: &str, mode: &str) {
        self.state.borrow_mut().host.set_selection_options(method, mode);
    }

    #[wasm_bindgen(js_name = selectionPreviewPointsJson)]
    pub fn selection_preview_points_json(&self) -> String {
        self.state.borrow().host.selection_preview_points_json()
    }

    #[wasm_bindgen(js_name = selectionPreviewCrossing)]
    pub fn selection_preview_crossing(&self) -> bool {
        self.state.borrow().host.selection_preview_crossing()
    }

    #[wasm_bindgen(js_name = selectionUnionBoundsScreenJson)]
    pub fn selection_union_bounds_screen_json(&self) -> String {
        self.state.borrow().host.selection_union_bounds_screen_json()
    }

    #[wasm_bindgen(js_name = alignSelection)]
    pub fn align_selection(&self, mode: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.align_selection(mode).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = preselectWidgetIdsJson)]
    pub fn preselect_widget_ids_json(&self) -> String {
        self.state.borrow().host.preselect_widget_ids_json()
    }

    #[wasm_bindgen(js_name = cancelAreaSelect)]
    pub fn cancel_area_select(&self) -> bool {
        self.state.borrow_mut().host.cancel_area_select()
    }

    #[wasm_bindgen(js_name = deleteSelection)]
    pub fn delete_selection(&self) -> Result<(), JsValue> {
        self.state.borrow_mut().host.delete_selection().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = hasSelection)]
    pub fn has_selection(&self) -> bool {
        self.state.borrow().host.has_selection()
    }

    #[wasm_bindgen(js_name = selectAll)]
    pub fn select_all(&self) {
        self.state.borrow_mut().host.select_all();
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn tessellate(handle: &str, tolerance: f64) -> String {
    flow_module_brep::tessellate_geometry_json(handle, tolerance)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn dispose(handle: &str) {
    flow_module_brep::dispose_geometry(handle);
}
// #endregion 🔖WasmSession

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use cavas::camera::{world_to_screen, Camera, Viewport};
    use cavas::vello::kurbo::Point;
    use neural::{ChannelSpec as InputSpec, OperatorInfo as NeuronKindInfo};

    const NUMBER_OPS: &[&str] = &["core.number"];

    fn test_math_bridge(kind: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        if kind == "core.number" {
            let value = input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
            return Ok(Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(value))));
        }
        if kind == "core.text" {
            let value = input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap_or_default();
            return Ok(Dictionary::with_schema("text").insert("value", NeuralValue::Atom(Atom::String(value.into()))));
        }
        if kind == "core.image" {
            let value = input.get("dataUrl").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap_or_default();
            return Ok(Dictionary::with_schema("image").insert("dataUrl", NeuralValue::Atom(Atom::String(value.into()))));
        }
        if kind == "math.add" {
            let a = input
                .get("a")
                .or_else(|| input.get("number"))
                .and_then(|v| v.as_dictionary())
                .and_then(|d| d.get("value"))
                .and_then(|v| v.as_atom())
                .and_then(|a| a.as_f64())
                .ok_or_else(|| EvalError::MissingInput("a".into()))?;
            let b = input.get("b").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
            return Ok(Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(a + b))));
        }
        if kind == "math.passThrough" {
            let n = input
                .get("number")
                .and_then(|v| v.as_dictionary())
                .and_then(|d| d.get("value"))
                .and_then(|v| v.as_atom())
                .and_then(|a| a.as_f64())
                .ok_or_else(|| EvalError::MissingInput("number".into()))?;
            return Ok(Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(n))));
        }
        Err(EvalError::UnknownKind(kind.into()))
    }

    fn test_kind_infos_json() -> String {
        serde_json::to_string(&[
            NeuronKindInfo {
                id: "math.add".into(),
                module: "math".into(),
                name: "Add".into(),
                abbreviation: "Add".into(),
                icon: "emoji:➕".into(),
                summary: "Sums two numbers".into(),
                inputs: vec![InputSpec::number("a", NUMBER_OPS), InputSpec::number_default("b", 0.0, NUMBER_OPS)],
                outputs: vec![InputSpec::provides("out", vec!["core.number".into()])],
                ..Default::default()
            },
            NeuronKindInfo {
                id: "math.passThrough".into(),
                module: "math".into(),
                name: "PassThrough".into(),
                abbreviation: "Pass".into(),
                icon: "emoji:➡️".into(),
                summary: "Forwards a number".into(),
                inputs: vec![InputSpec::number_default("number", 0.0, NUMBER_OPS)],
                outputs: vec![InputSpec::provides("out", vec!["core.number".into()])],
                ..Default::default()
            },
        ])
        .unwrap()
    }

    fn host_with_test_bridge() -> FlowHost {
        let mut host = FlowHost::default();
        host.set_eval_bridge_fn(Box::new(test_math_bridge));
        host.set_neuron_kind_infos_json(&test_kind_infos_json());
        host.set_host_catalogue_json(&serde_json::to_string(&[CatalogueSection {
            id: "math".into(),
            title: "Math".into(),
            groups: vec![],
            items: vec![
                CatalogueItem {
                    kind: "neuron".into(),
                    neuronKind: Some("math.add".into()),
                    action: None,
                    name: "Add".into(),
                    abbreviation: "Add".into(),
                    icon: "emoji:➕".into(),
                    summary: "Sums two numbers".into(),
                },
                CatalogueItem {
                    kind: "neuron".into(),
                    neuronKind: Some("math.passThrough".into()),
                    action: None,
                    name: "PassThrough".into(),
                    abbreviation: "Pass".into(),
                    icon: "emoji:➡️".into(),
                    summary: "Forwards a number".into(),
                },
            ],
        }]).unwrap());
        host.evaluate_internal();
        host
    }

    fn widget_screen_point(host: &FlowHost, widget_id: &str) -> (f64, f64) {
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == widget_id).expect("node");
        let cam = Camera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
        let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
        let screen = world_to_screen(&cam, &viewport, Point::new(node.x, node.y));
        (screen.x, screen.y)
    }

    fn widget_slider_track_screen_point(host: &FlowHost, widget_id: &str) -> (f64, f64) {
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == widget_id).expect("node");
        let (wx, wy) = dag::slider_track_center(node).expect("slider track");
        let cam = Camera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
        let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
        let screen = world_to_screen(&cam, &viewport, Point::new(wx, wy));
        (screen.x, screen.y)
    }

    #[test]
    fn default_fixture_maps_widgets_to_native_dag_kinds() {
        let host = host_with_test_bridge();
        let slider = host.dag.fixture.nodes.iter().find(|n| n.id == "slider").expect("slider");
        assert!(matches!(slider.kind, DagNodeKind::Slider { .. }));
        assert_eq!(slider.height, slider_widget_height());
        let add = host.dag.fixture.nodes.iter().find(|n| n.id == "add").expect("add");
        assert!(matches!(add.kind, DagNodeKind::Computation { .. }));
        assert!(slider.width >= 50.0, "slider should use function IO column width");
        assert!(slider.width <= add.width, "slider width should follow function sizing");
        let preview = host.dag.fixture.nodes.iter().find(|n| n.id == "preview").expect("preview");
        assert!(matches!(preview.kind, DagNodeKind::Preview { .. }));
    }

    #[test]
    fn default_fixture_evaluates_add_preview() {
        let host = host_with_test_bridge();
        assert_eq!(host.preview_text(), "3");
    }

    #[test]
    fn slider_updates_preview() {
        let mut host = host_with_test_bridge();
        host.set_slider_value("slider", 5.0);
        assert_eq!(host.preview_text(), "5");
    }

    #[test]
    fn evaluate_skips_unchanged_tree_after_move_widget() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_for_bridge = calls.clone();
        let mut host = FlowHost::default();
        host.set_eval_bridge_fn(Box::new(move |kind, input| {
            calls_for_bridge.fetch_add(1, Ordering::Relaxed);
            test_math_bridge(kind, input)
        }));
        host.set_neuron_kind_infos_json(&test_kind_infos_json());
        host.evaluate_internal();
        let baseline = calls.load(Ordering::Relaxed);
        host.move_widget("slider", -120.0, 20.0).unwrap();
        host.evaluate_internal();
        assert_eq!(calls.load(Ordering::Relaxed), baseline);
    }

    #[test]
    fn connect_ports_replaces_existing_incoming_on_same_input() {
        let mut host = host_with_test_bridge();
        assert!(host.fixture.synapses.iter().any(|s| s.from == "slider" && s.to == "add" && s.to_port == "a"));
        let note_id = host
            .add_widget(r#"{"kind":"inputNote","id":"note","text":"2"}"#, -120.0, 0.0)
            .unwrap();
        host.connect_ports(&note_id, "out", "add", "a").unwrap();
        let incoming_a: Vec<_> = host
            .fixture
            .synapses
            .iter()
            .filter(|s| s.to == "add" && s.to_port == "a")
            .collect();
        assert_eq!(incoming_a.len(), 1);
        assert_eq!(incoming_a[0].from, note_id);
        assert!(!host.fixture.synapses.iter().any(|s| s.from == "slider" && s.to == "add" && s.to_port == "a"));
    }

    #[test]
    fn evaluate_runs_after_tree_change() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_for_bridge = calls.clone();
        let mut host = FlowHost::default();
        host.set_eval_bridge_fn(Box::new(move |kind, input| {
            calls_for_bridge.fetch_add(1, Ordering::Relaxed);
            test_math_bridge(kind, input)
        }));
        host.set_neuron_kind_infos_json(&test_kind_infos_json());
        host.evaluate_internal();
        let baseline = calls.load(Ordering::Relaxed);
        host.set_slider_value("slider", 5.0);
        host.evaluate_internal();
        let after_slider = calls.load(Ordering::Relaxed);
        assert!(after_slider > baseline);
        host.disconnect("s1").unwrap();
        host.connect_ports("slider", "out", "add", "b").unwrap();
        host.evaluate_internal();
        assert!(calls.load(Ordering::Relaxed) > after_slider);
    }

    #[test]
    fn neural_cache_persists_across_evaluations() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_for_bridge = calls.clone();
        let mut host = FlowHost::default();
        host.set_eval_bridge_fn(Box::new(move |kind, input| {
            calls_for_bridge.fetch_add(1, Ordering::Relaxed);
            test_math_bridge(kind, input)
        }));
        host.set_neuron_kind_infos_json(&test_kind_infos_json());
        host.evaluate_internal();
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        host.evaluate_internal();
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        host.set_slider_value("slider", 4.0);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn collect_live_geometry_handles_includes_input_channels() {
        let mut outputs = HashMap::new();
        outputs.insert(
            "box".into(),
            Dictionary::with_schema("geometry")
                .insert("handle", NeuralValue::Atom(Atom::String("solid-box".into())))
                .insert("kind", NeuralValue::Atom(Atom::String("solid".into()))),
        );
        outputs.insert(
            "volume".into(),
            Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(12.0))),
        );
        let mut inputs = HashMap::new();
        inputs.insert(
            "volume".into(),
            Dictionary::new().insert(
                "geometry",
                NeuralValue::Dictionary(
                    Dictionary::with_schema("geometry").insert("handle", NeuralValue::Atom(Atom::String("solid-box".into()))),
                ),
            ),
        );
        let channels = EvalChannels { outputs, inputs };
        let handles = collect_live_geometry_handles_from_channels(&channels);
        assert_eq!(handles, vec![String::from("solid-box")]);
    }

    #[test]
    fn apply_eval_outputs_json_preserves_state_on_global_error() {
        let mut host = host_with_test_bridge();
        let good = host.last_eval_json.clone();
        host.apply_eval_outputs_json(r#"{"error":"missing input: geometry"}"#);
        assert_eq!(host.last_eval_json, good);
        assert!(!host.outputs.is_empty());
    }

    #[test]
    fn collect_live_geometry_handles_traverses_nested_dictionaries() {
        let mut outputs = HashMap::new();
        outputs.insert(
            "box".into(),
            Dictionary::with_schema("geometry")
                .insert("handle", NeuralValue::Atom(Atom::String("solid-1".into())))
                .insert("kind", NeuralValue::Atom(Atom::String("solid".into()))),
        );
        outputs.insert(
            "nested".into(),
            Dictionary::new().insert(
                "child",
                NeuralValue::Dictionary(
                    Dictionary::with_schema("geometry").insert("handle", NeuralValue::Atom(Atom::String("solid-2".into()))),
                ),
            ),
        );
        let handles = collect_live_geometry_handles(&outputs);
        assert_eq!(handles, vec![String::from("solid-1"), String::from("solid-2")]);
    }

    #[test]
    fn evaluate_emits_channel_structured_json() {
        let host = host_with_test_bridge();
        let parsed: serde_json::Value = serde_json::from_str(&host.last_eval_json).expect("json");
        let add = parsed.get("add").and_then(|value| value.as_object()).expect("add channels");
        assert!(add.get("in").and_then(|value| value.as_object()).is_some());
        let out = add.get("out").and_then(|value| value.as_object()).expect("add out");
        assert!(out.get("out").is_some());
    }

    #[test]
    fn preview_text_formats_geometry_as_tree_summary() {
        let dict = Dictionary::new().insert("geometry", NeuralValue::Atom(Atom::String("solid-3".into())));
        let content = dag_preview_content_from_dict(&dict);
        assert!(matches!(content, DagPreviewContent::Tree { .. }));
        assert_eq!(preview_content_summary(&content), "{1 keys}");
    }

    #[test]
    fn preview_scalar_content_from_number_dict() {
        let dict = Dictionary::new().insert("number", NeuralValue::Atom(Atom::Decimal(3.0)));
        assert!(matches!(
            dag_preview_content_from_dict(&dict),
            DagPreviewContent::Scalar { text } if text == "3"
        ));
    }

    #[test]
    fn image_input_seed_and_preview_content() {
        let mut host = host_with_test_bridge();
        let png = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";
        host.fixture.widgets.push(Widget::InputImage { id: "image".into(), src: png.into() });
        host.rebuild_dag();
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == "image").expect("image node");
        assert!(matches!(node.kind, DagNodeKind::Image { .. }));
        let seeds = host.build_seeds();
        assert_eq!(seeds.get("image").and_then(|d| d.get("dataUrl")).and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some(png));
    }

    #[test]
    fn slider_drag_evaluates_preview_before_release() {
        let mut host = host_with_test_bridge();
        host.set_viewport(800, 600, 1.0);
        let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
        assert_eq!(host.preview_text(), "3");
        host.pointer_down_screen(sx, sy, 0, false, false, false, false);
        host.pointer_move_screen(sx + 80.0, sy, false, false, false);
        assert_ne!(host.preview_text(), "3");
        host.pointer_up_screen(sx + 80.0, sy, false, false, false);
    }

    #[test]
    fn dag_slider_drag_syncs_fixture_value() {
        let mut host = host_with_test_bridge();
        host.set_viewport(800, 600, 1.0);
        let slider_node = host.dag.fixture.nodes.iter().find(|n| n.id == "slider").expect("slider").clone();
        let DagNodeKind::Slider { .. } = slider_node.kind else {
            panic!("expected slider kind");
        };
        let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
        host.pointer_down_screen(sx, sy, 0, false, false, false, false);
        host.pointer_move_screen(sx + 80.0, sy, false, false, false);
        host.pointer_up_screen(sx + 80.0, sy, false, false, false);
        let value = host
            .fixture
            .widgets
            .iter()
            .find_map(|w| match w {
                Widget::InputSlider { id, value, .. } if id == "slider" => Some(*value),
                _ => None,
            })
            .unwrap();
        assert!(value > 3.0);
    }

    #[test]
    fn default_fixture_does_not_auto_layout() {
        let host = host_with_test_bridge();
        let slider = host.fixture.layout.get("slider").expect("slider");
        let add = host.fixture.layout.get("add").expect("add");
        let preview = host.fixture.layout.get("preview").expect("preview");
        assert_eq!(slider.x, 0.0);
        assert_eq!(add.x, 200.0);
        assert_eq!(preview.x, 400.0);
    }

    #[test]
    fn canvas_slider_hit_adjusts_value_playground_viewport() {
        let mut host = host_with_test_bridge();
        host.set_viewport(1259, 706, 1.0);
        let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
        host.pointer_down_screen(sx, sy, 0, false, false, false, false);
        host.pointer_move_screen(sx + 90.0, sy, false, false, false);
        host.pointer_up_screen(sx + 90.0, sy, false, false, false);
        let slider = host
            .fixture
            .widgets
            .iter()
            .find_map(|w| match w {
                Widget::InputSlider { id, value, .. } if id == "slider" => Some(*value),
                _ => None,
            })
            .unwrap();
        assert!(slider > 3.0);
    }

    #[test]
    fn canvas_slider_hit_adjusts_value() {
        let mut host = host_with_test_bridge();
        host.set_viewport(800, 600, 1.0);
        let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
        host.pointer_down_screen(sx, sy, 0, false, false, false, false);
        host.pointer_move_screen(sx + 80.0, sy, false, false, false);
        host.pointer_up_screen(sx + 80.0, sy, false, false, false);
        let slider = host
            .fixture
            .widgets
            .iter()
            .find_map(|w| match w {
                Widget::InputSlider { id, value, .. } if id == "slider" => Some(*value),
                _ => None,
            })
            .unwrap();
        assert!(slider > 3.0);
    }

    #[test]
    fn reorganize_overwrites_saved_layout_left_to_right() {
        let mut host = host_with_test_bridge();
        host.fixture.layout.insert("slider".into(), WidgetLayout { x: -900.0, y: -900.0 });
        host.fixture.layout.insert("add".into(), WidgetLayout { x: -900.0, y: -900.0 });
        host.fixture.layout.insert("preview".into(), WidgetLayout { x: -900.0, y: -900.0 });
        host.rebuild_dag();
        host.reorganize("").unwrap();
        let slider = host.fixture.layout.get("slider").expect("slider layout");
        let add = host.fixture.layout.get("add").expect("add layout");
        let preview = host.fixture.layout.get("preview").expect("preview layout");
        assert!(add.x > slider.x);
        assert!(preview.x > add.x);
    }

    #[test]
    fn fixture_json_round_trip() {
        let host = FlowHost::default();
        let json = host.fixture_json().unwrap();
        let parsed = FlowHost::parse_fixture_json(&json).unwrap();
        assert_eq!(parsed.schema, "flow.fixture/v1");
    }

    #[test]
    fn flow_document_tree_is_shakable() {
        let host = host_with_test_bridge();
        let document = host.document();
        assert_eq!(document.schema, "flow.document/v1");
        assert!(!document.flow.nodes.is_empty());
        let registry = neural::Registry::new();
        let evaluator = Evaluator::new(&registry);
        let mut dispatch = |kind: &str, input: &Dictionary| test_math_bridge(kind, input);
        let channels = evaluator.evaluate_channels_with(&document.tree, &HashMap::new(), &host.kind_infos, &mut dispatch).unwrap();
        assert_eq!(
            channels.outputs.get("add").and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()),
            Some(3.0)
        );
    }

    #[test]
    fn rebuild_dag_preserves_vello_theme() {
        use cavas::vello::peniko::Color;
        let mut host = FlowHost::default();
        host.dag.vello_theme.node_fill = Color::from_rgba8(12, 34, 56, 255);
        host.rebuild_dag();
        assert_eq!(host.dag.vello_theme.node_fill.to_rgba8(), Color::from_rgba8(12, 34, 56, 255).to_rgba8());
    }

    #[test]
    fn replace_fixture_preserves_kind_infos_and_named_input_ports() {
        let mut host = host_with_test_bridge();
        host.replace_fixture(FlowFixtureV1 {
            schema: "flow.fixture/v1".into(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            widgets: vec![Widget::Neuron {
                id: "add".into(),
                neuronKind: "math.add".into(),
                params: Dictionary::new(),
                input_ports: vec![],
                preview: true,
            }],
            synapses: vec![],
            layout: BTreeMap::new(),
        });
        let node = host.dag.fixture.nodes.iter().find(|node| node.id == "add").expect("add node");
        let input_ids: Vec<&str> = node.inputs().iter().map(|port| port.id.as_str()).collect();
        assert_eq!(input_ids, vec!["a", "b"]);
    }

    #[test]
    fn catalogue_nested_groups_round_trip() {
        let host_json = serde_json::to_string(&[CatalogueSection {
            id: "brep".into(),
            title: "Brep".into(),
            items: vec![],
            groups: vec![CatalogueGroup {
                id: "brep.primitives-3d".into(),
                title: "Primitives 3D".into(),
                items: vec![CatalogueItem {
                    kind: "neuron".into(),
                    neuronKind: Some("brep.prim3d.box".into()),
                    action: None,
                    name: "Box".into(),
                    abbreviation: "Box".into(),
                    icon: "emoji:📦".into(),
                    summary: "Axis-aligned box".into(),
                }],
                groups: vec![],
            }],
        }])
        .unwrap();
        let sections = merge_catalogue_sections(&host_json).unwrap();
        let brep = sections.iter().find(|section| section.id == "brep").expect("brep section");
        let prim3d = brep.groups.iter().find(|group| group.title == "Primitives 3D").expect("prim3d group");
        assert_eq!(prim3d.items[0].neuronKind.as_deref(), Some("brep.prim3d.box"));
    }

    #[test]
    fn catalogue_has_module_sections() {
        let host = host_with_test_bridge();
        let json = host.catalogue_json().unwrap();
        assert!(json.contains("math"));
        assert!(json.contains("math.add"));
        assert!(json.contains("Inputs"));
        assert!(json.contains("Outputs"));
    }

    #[test]
    fn add_widget_and_connect() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"neuron","neuronKind":"math.passThrough"}"#, 100.0, 50.0).unwrap();
        host.connect_ports("slider", "out", &id, "number").unwrap();
        host.connect_ports(&id, "out", "preview", "in").unwrap();
        host.set_slider_value("slider", 4.0);
        assert_eq!(host.preview_text(), "4");
    }

    #[test]
    fn undo_redo_add_widget() {
        let mut host = host_with_test_bridge();
        let count_before = host.fixture.widgets.len();
        let id = host.add_widget(r#"{"kind":"inputNote","text":"undo me"}"#, 42.0, 42.0).unwrap();
        assert_eq!(host.fixture.widgets.len(), count_before + 1);
        assert!(host.can_undo());
        assert!(host.undo());
        assert_eq!(host.fixture.widgets.len(), count_before);
        assert!(!host.fixture.widgets.iter().any(|w| widget_id_for(w) == id));
        assert!(host.can_redo());
        assert!(host.redo());
        assert!(host.fixture.widgets.iter().any(|w| widget_id_for(w) == id));
    }

    #[test]
    fn camera_change_does_not_create_undo_step() {
        let mut host = host_with_test_bridge();
        let camera_before = host.fixture.camera.clone();
        host.set_camera(camera_before.x + 50.0, camera_before.y - 30.0, camera_before.zoom * 1.5);
        assert!(!host.can_undo());
        let id = host.add_widget(r#"{"kind":"inputNote","text":"x"}"#, 0.0, 0.0).unwrap();
        assert!(host.can_undo());
        assert!(host.undo());
        assert_eq!(host.fixture.camera.x, camera_before.x + 50.0);
        assert_eq!(host.fixture.camera.y, camera_before.y - 30.0);
        assert!((host.fixture.camera.zoom - camera_before.zoom * 1.5).abs() < 1e-9);
        assert!(!host.fixture.widgets.iter().any(|w| widget_id_for(w) == id));
    }

    fn test_dictionary_merge_bridge(kind: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        if kind == "core.number" {
            let value = input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
            return Ok(Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(value))));
        }
        if kind != "dictionary.merge" {
            return Err(EvalError::UnknownKind(kind.into()));
        }
        let items = input
            .get("items")
            .and_then(|value| value.as_dictionary())
            .ok_or_else(|| EvalError::MissingInput("items".into()))?;
        let mut indices: Vec<usize> = items.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
        indices.sort_unstable();
        if indices.len() < 2 {
            return Err(EvalError::MissingInput("items".into()));
        }
        let mut merged = Dictionary::with_schema("dictionary");
        for index in indices {
            let slot = items
                .get(&index.to_string())
                .and_then(|value| value.as_dictionary())
                .ok_or_else(|| EvalError::MissingInput(index.to_string()))?;
            merged = merged.merge(slot);
        }
        Ok(merged)
    }

    #[test]
    fn variadic_merge_evaluates_port_routed_inputs() {
        let mut host = FlowHost::from_fixture(FlowFixtureV1 {
            schema: "flow.fixture/v1".into(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            widgets: vec![
                Widget::InputSlider { id: "a".into(), value: 1.0, min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP },
                Widget::InputSlider { id: "b".into(), value: 2.0, min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP },
                Widget::Neuron {
                    id: "merge".into(),
                    neuronKind: "dictionary.merge".into(),
                    params: Dictionary::new(),
                    input_ports: vec!["0".into(), "1".into()],
                    preview: true,
                },
                Widget::OutputPreview { id: "preview".into(), preview: Dictionary::new(), expanded: BTreeSet::new() },
            ],
            synapses: vec![
                SynapseSpec {
                    id: "s1".into(),
                    from: "a".into(),
                    to: "merge".into(),
                    from_port: "out".into(),
                    to_port: "0".into(),
                },
                SynapseSpec {
                    id: "s2".into(),
                    from: "b".into(),
                    to: "merge".into(),
                    from_port: "out".into(),
                    to_port: "1".into(),
                },
                SynapseSpec {
                    id: "s3".into(),
                    from: "merge".into(),
                    to: "preview".into(),
                    from_port: "out".into(),
                    to_port: "in".into(),
                },
            ],
            layout: BTreeMap::new(),
        });
        host.set_eval_bridge_fn(Box::new(test_dictionary_merge_bridge));
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[NeuronKindInfo {
                id: "dictionary.merge".into(),
                module: "dictionary".into(),
                name: "Merge".into(),
                abbreviation: "Merge".into(),
                icon: "emoji:🔀".into(),
                summary: "Merge".into(),
                inputs: vec![],
                outputs: vec![InputSpec::provides("out", vec!["dictionary".into()])],
                variadic_input: Some(neural::VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
                ..Default::default()
            }])
            .unwrap(),
        );
        host.evaluate_internal();
        let preview = host
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::OutputPreview { preview, .. } => Some(preview),
                _ => None,
            })
            .expect("preview");
        assert_eq!(
            preview.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()),
            Some(2.0)
        );
    }

    #[test]
    fn widget_to_dag_node_carries_display_meta() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"neuron","neuronKind":"math.add"}"#, 0.0, 0.0).unwrap();
        let node = host.dag.fixture.nodes.iter().find(|node| node.id == id).expect("node");
        assert_eq!(node.name, "Add");
        assert_eq!(node.abbreviation, "Add");
        assert_eq!(node.icon, "emoji:➕");
    }

    #[test]
    fn add_slider_widget_with_explicit_range() {
        let mut host = host_with_test_bridge();
        let id = host
            .add_widget(r#"{"kind":"inputSlider","value":10.2,"min":10.2,"max":15.0,"step":0.1}"#, 0.0, 0.0)
            .unwrap();
        let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
        let Widget::InputSlider { value, min, max, step, .. } = widget else {
            panic!("expected slider widget");
        };
        assert!((value - 10.2).abs() < 1e-6);
        assert!((min - 10.2).abs() < 1e-6);
        assert!((max - 15.0).abs() < 1e-6);
        assert!((step - 0.1).abs() < 1e-6);
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
        let DagNodeKind::Slider { min: dag_min, max: dag_max, step: dag_step, value: dag_value, .. } = &node.kind else {
            panic!("expected slider node");
        };
        assert!((dag_min - 10.2).abs() < 1e-6);
        assert!((dag_max - 15.0).abs() < 1e-6);
        assert!((dag_step - 0.1).abs() < 1e-6);
        assert!((dag_value - 10.2).abs() < 1e-6);
    }

    #[test]
    fn add_note_widget_with_text() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputNote","text":"some text"}"#, 0.0, 0.0).unwrap();
        let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
        let Widget::InputNote { text, .. } = widget else {
            panic!("expected note widget");
        };
        assert_eq!(text, "some text");
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
        let DagNodeKind::Note { text: dag_text, .. } = &node.kind else {
            panic!("expected note node");
        };
        assert_eq!(dag_text, "some text");
        assert!(node.width > 40.0);
        assert!(node.height > 20.0);
    }

    #[test]
    fn set_note_text_resizes_node() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputNote","text":"hi"}"#, 0.0, 0.0).unwrap();
        let short_w = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node").width;
        host.set_note_text(&id, "a much longer note string");
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
        let DagNodeKind::Note { text, .. } = &node.kind else {
            panic!("expected note node");
        };
        assert_eq!(text, "a much longer note string");
        assert!(node.width > short_w);
    }

    #[test]
    fn add_slider_widget_with_single_value_uses_sensible_range() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputSlider","value":5.0}"#, 0.0, 0.0).unwrap();
        let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
        let Widget::InputSlider { value, min, max, step, .. } = widget else {
            panic!("expected slider widget");
        };
        assert!((value - 5.0).abs() < 1e-6);
        assert!((min - 0.0).abs() < 1e-6);
        assert!((max - 10.0).abs() < 1e-6);
        assert!((step - 1.0).abs() < 1e-6);
    }

    #[test]
    fn add_slider_widget_with_decimal_value_uses_matching_step() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputSlider","value":1.3}"#, 0.0, 0.0).unwrap();
        let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
        let Widget::InputSlider { value, min, max, step, .. } = widget else {
            panic!("expected slider widget");
        };
        assert!((value - 1.3).abs() < 1e-6);
        assert!((min - 0.0).abs() < 1e-6);
        assert!((max - 10.0).abs() < 1e-6);
        assert!((step - 0.1).abs() < 1e-6);
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
        let DagNodeKind::Slider { min: dag_min, max: dag_max, step: dag_step, value: dag_value, .. } = &node.kind else {
            panic!("expected slider node");
        };
        assert!((dag_min - 0.0).abs() < 1e-6);
        assert!((dag_max - 10.0).abs() < 1e-6);
        assert!((dag_step - 0.1).abs() < 1e-6);
        assert!((dag_value - 1.3).abs() < 1e-6);
    }

    #[test]
    fn add_slider_widget_with_two_decimal_places_uses_finer_step() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputSlider","value":1.25}"#, 0.0, 0.0).unwrap();
        let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
        let Widget::InputSlider { step, .. } = widget else {
            panic!("expected slider widget");
        };
        assert!((step - 0.01).abs() < 1e-6);
    }

    #[test]
    fn set_slider_value_expands_bounds_when_out_of_range() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputSlider","value":3.0,"min":0.0,"max":10.0,"step":1.0}"#, 0.0, 0.0).unwrap();
        host.set_slider_value(&id, 12.0);
        let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
        let Widget::InputSlider { value, min, max, .. } = widget else {
            panic!("expected slider widget");
        };
        assert!((value - 12.0).abs() < 1e-6);
        assert!((min - 0.0).abs() < 1e-6);
        assert!((max - 20.0).abs() < 1e-6);
    }

    #[test]
    fn ghost_widget_matches_placed_neuron_size() {
        let mut host = host_with_test_bridge();
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[NeuronKindInfo {
                id: "brep.sketch2d.circle".into(),
                module: "brep".into(),
                name: "Sketch Circle".into(),
                abbreviation: "Circle".into(),
                icon: "emoji:⚪".into(),
                summary: "Sketched circle profile".into(),
                inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
                outputs: vec![InputSpec::provides("out", vec!["geometry".into()])],
                ..Default::default()
            }])
            .unwrap(),
        );
        let descriptor = r#"{"kind":"neuron","neuronKind":"brep.sketch2d.circle"}"#;
        host.set_ghost_widget(descriptor, 40.0, 40.0).unwrap();
        let ghost_width = host.ghost_node.as_ref().expect("ghost").width;
        let placed_id = host.add_widget(descriptor, 80.0, 80.0).unwrap();
        let placed_width = host
            .dag
            .fixture
            .nodes
            .iter()
            .find(|node| node.id == placed_id)
            .expect("placed")
            .width;
        assert!((ghost_width - placed_width).abs() < 1e-6, "ghost width {ghost_width} != placed {placed_width}");
    }

    #[test]
    fn ghost_widget_preview_and_clear() {
        let mut host = host_with_test_bridge();
        host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"math.add"}"#, 42.0, 24.0).unwrap();
        let ghost = host.ghost_node.as_ref().expect("ghost");
        assert!((ghost.x - 42.0).abs() < 1e-6);
        assert!((ghost.y - 24.0).abs() < 1e-6);
        assert_eq!(ghost.name, "Add");
        assert_eq!(ghost.abbreviation, "Add");
        assert_eq!(ghost.icon, "emoji:➕");
        host.clear_ghost_widget();
        assert!(host.ghost_node.is_none());
    }

    #[test]
    fn ghost_widget_label_overlay_matches_placed_at_micro() {
        let mut host = host_with_test_bridge();
        host.set_viewport(1280, 800, 1.0);
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[NeuronKindInfo {
                id: "brep.sketch2d.circle".into(),
                module: "brep".into(),
                name: "Sketch Circle".into(),
                abbreviation: "Circle".into(),
                icon: "emoji:⚪".into(),
                summary: "Sketched circle profile".into(),
                inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
                outputs: vec![InputSpec::provides("out", vec!["geometry".into()])],
                ..Default::default()
            }])
            .unwrap(),
        );
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label("micro");
        let descriptor = r#"{"kind":"neuron","neuronKind":"brep.sketch2d.circle"}"#;
        host.set_ghost_widget(descriptor, 40.0, 40.0).unwrap();
        let ghost = host.dag.ghost_node().expect("ghost");
        assert_eq!(host.draw_lod_label(), "micro");
        assert_eq!(dag::DagDrawLod::Micro.node_label(), dag::DagNodeLabel::Name);
        assert!(dag::DagDrawLod::Micro.shows_port_labels());
        assert!(dag::DagDrawLod::Micro.shows_handles());
        let ghost_overlay_rows = host.dag.label_overlay_rows_for_node_spec(ghost, true);
        assert_eq!(ghost_overlay_rows.len(), 3);
        let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
        let overlay_ghost_rows: Vec<_> = overlay["labels"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| row["ghost"] == true)
            .collect();
        assert_eq!(overlay_ghost_rows.len(), 3);
        let placed_node = {
            let widget = widget_from_descriptor(
                &serde_json::from_str::<WidgetDescriptor>(descriptor).unwrap(),
                "placed".into(),
                &host.kind_infos,
            );
            let mut layout = BTreeMap::new();
            layout.insert("placed".into(), WidgetLayout { x: 80.0, y: 80.0 });
            let mut node = widget_to_dag_node(&widget, 0, &layout, &[], &host.kind_infos);
            fit_node_size(&mut node);
            node
        };
        let placed_rows = host.dag.label_overlay_rows_for_node_spec(&placed_node, false);
        assert_eq!(ghost_overlay_rows.len(), placed_rows.len());
        for (ghost_row, placed_row) in ghost_overlay_rows.iter().zip(placed_rows.iter()) {
            assert_eq!(ghost_row["text"], placed_row["text"]);
            assert_eq!(ghost_row["layout"], placed_row["layout"]);
            assert_eq!(ghost_row["align"], placed_row["align"]);
        }
        let mut scene = cavas::vello::Scene::new();
        host.paint_scene(&mut scene, 1280, 800, 1.0);
    }

    #[test]
    fn rebuild_dag_preserves_ghost_overlay_at_micro() {
        let mut host = host_with_test_bridge();
        host.set_viewport(1280, 800, 1.0);
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[NeuronKindInfo {
                id: "brep.sketch2d.circle".into(),
                module: "brep".into(),
                name: "Sketch Circle".into(),
                abbreviation: "Circle".into(),
                icon: "emoji:⚪".into(),
                summary: "Sketched circle profile".into(),
                inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
                outputs: vec![InputSpec::provides("out", vec!["geometry".into()])],
                ..Default::default()
            }])
            .unwrap(),
        );
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label("micro");
        host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"brep.sketch2d.circle"}"#, 12.0, 18.0).unwrap();
        host.rebuild_dag();
        assert!(host.dag.ghost_node().is_some());
        assert_eq!(host.draw_lod_label(), "micro");
        let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
        let ghost_rows: Vec<_> = overlay["labels"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| row["ghost"] == true)
            .collect();
        assert_eq!(ghost_rows.len(), 3);
    }

    #[test]
    fn ghost_widget_paint_scene_smoke() {
        let mut host = host_with_test_bridge();
        host.set_viewport(800, 600, 1.0);
        host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"math.add"}"#, 10.0, 20.0).unwrap();
        let mut scene = cavas::vello::Scene::new();
        host.paint_scene(&mut scene, 800, 600, 1.0);
    }

    #[test]
    fn selection_and_preview_state_round_trip() {
        let mut host = FlowHost::default();
        host.set_selection_json(r#"["slider","add"]"#);
        let selected: Vec<String> = serde_json::from_str(&host.selected_widget_ids_json()).unwrap();
        assert_eq!(selected, vec!["slider", "add"]);
        host.set_hover(Some("add"));
        assert_eq!(host.hovered_widget_id().as_deref(), Some("add"));
        host.set_preview_off_json(r#"["add"]"#);
        assert_eq!(host.preview_off_widget_ids(), vec!["add"]);
        host.toggle_preview("add").unwrap();
        assert!(host.preview_off_widget_ids().is_empty());
    }

    #[test]
    fn channel_hover_and_selection_round_trip_at_detail_lod() {
        let mut host = host_with_test_bridge();
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label("detail");
        host.set_hover_channel(Some("add"), Some("a"));
        let hovered: dag::DagChannelRef = serde_json::from_str(&host.hovered_channel_json()).unwrap();
        assert_eq!(hovered.widget_id, "add");
        assert_eq!(hovered.port, "a");
        assert_eq!(hovered.direction, "in");
        host.set_selected_channels_json(r#"[{"widgetId":"add","port":"a","direction":"in"}]"#);
        let selected: Vec<dag::DagChannelRef> = serde_json::from_str(&host.selected_channels_json()).unwrap();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].widget_id, "add");
        assert_eq!(selected[0].port, "a");
    }

    #[test]
    fn drag_merge_node_preserves_single_fixture_widget() {
        let mut host = host_with_test_bridge();
        host.set_neuron_kind_infos_json(&serde_json::to_string(&[NeuronKindInfo {
            id: "dictionary.merge".into(),
            module: "dictionary".into(),
            name: "Merge".into(),
            abbreviation: "Merge".into(),
            icon: "emoji:🔀".into(),
            summary: "Merge".into(),
            inputs: vec![],
            outputs: vec![InputSpec::provides("out", vec!["dictionary".into()])],
            variadic_input: Some(neural::VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
            ..Default::default()
        }]).unwrap());
        let merge_id = host.add_widget(r#"{"kind":"neuron","neuronKind":"dictionary.merge"}"#, 120.0, 80.0).unwrap();
        host.set_viewport(800, 600, 1.0);
        let merge = host.dag.fixture.nodes.iter().find(|n| n.id == merge_id).expect("merge").clone();
        let grab = Point::new(merge.x, merge.y);
        let cam = Camera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
        let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
        let screen = world_to_screen(&cam, &viewport, grab);
        host.pointer_down_screen(screen.x, screen.y, 0, false, false, false, false);
        host.pointer_move_screen(screen.x + 80.0, screen.y + 40.0, false, false, false);
        host.pointer_up_screen(screen.x + 80.0, screen.y + 40.0, false, false, false);
        assert_eq!(host.fixture.widgets.iter().filter(|w| widget_id_for(w) == merge_id).count(), 1);
        assert_eq!(host.dag.fixture.nodes.iter().filter(|n| n.id == merge_id).count(), 1);
        let moved = host.fixture.layout.get(&merge_id).expect("merge layout");
        assert!((moved.x - merge.x).abs() > 1.0);
    }

    #[test]
    fn ghost_widget_cleared_on_pointer_down_and_add_widget() {
        let mut host = host_with_test_bridge();
        host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"dictionary.merge"}"#, 12.0, 18.0).unwrap();
        host.set_viewport(800, 600, 1.0);
        host.pointer_down_screen(120.0, 120.0, 0, false, false, false, false);
        assert!(host.ghost_node.is_none());
        host.set_ghost_widget(r#"{"kind":"inputSlider"}"#, 0.0, 0.0).unwrap();
        let _ = host.add_widget(r#"{"kind":"inputSlider"}"#, 40.0, 40.0).unwrap();
        assert!(host.ghost_node.is_none());
        assert_eq!(host.fixture.widgets.iter().filter(|w| widget_id_for(w).starts_with("slider")).count(), 2);
        assert_eq!(host.dag.fixture.nodes.iter().filter(|n| n.id == "slider").count(), 1);
    }

    #[test]
    fn delete_selection_removes_widget_from_fixture() {
        let mut host = host_with_test_bridge();
        host.dag.set_selection(&["slider".into()]);
        host.delete_selection().unwrap();
        assert!(host.fixture.widgets.iter().all(|w| widget_id_for(w) != "slider"));
        assert!(host.dag.fixture.nodes.iter().all(|n| n.id != "slider"));
    }

    #[test]
    fn node_drag_proximity_skips_wired_cut_inputs_in_flow() {
        use cavas::camera::{world_to_screen, Camera, Viewport};
        use cavas::vello::kurbo::Point;
        let mut host = FlowHost::default();
        host.set_viewport(1280, 800, 1.0);
        host.fixture.widgets = vec![
            Widget::Neuron {
                id: "sphere".into(),
                neuronKind: "brep.prim3d.sphere".into(),
                params: Dictionary::new(),
                input_ports: vec![],
                preview: false,
            },
            Widget::Neuron {
                id: "torus".into(),
                neuronKind: "brep.prim3d.torus".into(),
                params: Dictionary::new(),
                input_ports: vec![],
                preview: false,
            },
            Widget::Neuron {
                id: "cut".into(),
                neuronKind: "brep.bool.cut".into(),
                params: Dictionary::new(),
                input_ports: vec!["a".into(), "b".into()],
                preview: true,
            },
        ];
        host.fixture.synapses = vec![
            SynapseSpec { id: "e1".into(), from: "sphere".into(), to: "cut".into(), from_port: "out".into(), to_port: "a".into() },
            SynapseSpec { id: "e2".into(), from: "torus".into(), to: "cut".into(), from_port: "out".into(), to_port: "b".into() },
        ];
        host.fixture.layout.insert("sphere".into(), WidgetLayout { x: 0.0, y: -60.0 });
        host.fixture.layout.insert("torus".into(), WidgetLayout { x: 0.0, y: 60.0 });
        host.fixture.layout.insert("cut".into(), WidgetLayout { x: 240.0, y: 0.0 });
        host.rebuild_dag();
        host.dag.set_proximity_distance(160.0);
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label("normal");
        assert_eq!(host.dag.engine.edges.len(), 2, "synapses should load as engine edges");
        let cut = host.dag.fixture.nodes.iter().find(|node| node.id == "cut").expect("cut");
        let grab = Point::new(cut.x, cut.y);
        let cam = Camera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
        let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
        let screen = world_to_screen(&cam, &viewport, grab);
        host.pointer_down_screen(screen.x, screen.y, 0, false, false, false, false);
        host.pointer_move_screen(screen.x - 180.0, screen.y, false, false, false);
        assert!(
            host.dag.engine.render_snapshot().pending_edge.is_none(),
            "dragging wired cut near sources must not preview proximity edges"
        );
        host.pointer_up_screen(screen.x - 180.0, screen.y, false, false, false);
        assert_eq!(host.dag.engine.edges.len(), 2);
        assert_eq!(host.fixture.synapses.len(), 2);
    }

    #[test]
    fn delete_selection_removes_selected_edge_from_fixture() {
        let mut host = host_with_test_bridge();
        let synapse_count_before = host.fixture.synapses.len();
        assert!(synapse_count_before > 0);
        let edge_id = *host.dag.engine.edges.keys().next().expect("edge");
        host.dag.engine.selection.edge_ids.insert(edge_id);
        assert!(host.has_selection());
        host.delete_selection().unwrap();
        assert!(host.fixture.synapses.len() < synapse_count_before);
        assert!(!host.has_selection());
    }

    #[test]
    fn align_selection_left_aligns_selected_widget_layout() {
        let mut host = host_with_test_bridge();
        host.move_widget("slider", -120.0, 20.0).unwrap();
        host.move_widget("add", 180.0, -40.0).unwrap();
        host.dag.set_selection(&["slider".into(), "add".into()]);
        host.align_selection("alignLeft").unwrap();
        let slider = host.dag.fixture.nodes.iter().find(|node| node.id == "slider").expect("slider");
        let add = host.dag.fixture.nodes.iter().find(|node| node.id == "add").expect("add");
        let slider_left = slider.x - slider.width * 0.5;
        let add_left = add.x - add.width * 0.5;
        assert!((slider_left - add_left).abs() < 1e-6, "left edges should match after alignLeft");
        assert!(host.fixture.layout.get("slider").is_some());
        assert!(host.fixture.layout.get("add").is_some());
    }

    #[test]
    fn add_input_port_inserts_variadic_slot() {
        let mut host = host_with_test_bridge();
        host.set_neuron_kind_infos_json(&serde_json::to_string(&[NeuronKindInfo {
            id: "dictionary.merge".into(),
            module: "dictionary".into(),
            name: "Merge".into(),
            abbreviation: "Merge".into(),
            icon: "emoji:🔀".into(),
            summary: "Merge".into(),
            inputs: vec![],
            outputs: vec![InputSpec::provides("out", vec!["dictionary".into()])],
            variadic_input: Some(neural::VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
            ..Default::default()
        }]).unwrap());
        let merge_id = host.add_widget(r#"{"kind":"neuron","neuronKind":"dictionary.merge"}"#, 0.0, 0.0).unwrap();
        host.add_input_port(&merge_id, 1).unwrap();
        let widget = host.fixture.widgets.iter().find(|widget| widget_id_for(widget) == merge_id).expect("merge");
        let Widget::Neuron { input_ports, .. } = widget else { panic!("neuron") };
        assert_eq!(input_ports.len(), 3);
    }

    #[test]
    fn add_widget_with_explicit_id() {
        let mut host = host_with_test_bridge();
        let id = host
            .add_widget(r#"{"kind":"inputSlider","id":"custom_slider","value":2.0}"#, 0.0, 0.0)
            .unwrap();
        assert_eq!(id, "custom_slider");
    }

    #[test]
    fn insert_between_rewires_downstream_and_connects_anchor() {
        let mut host = host_with_test_bridge();
        let mid = host
            .add_widget(r#"{"kind":"neuron","id":"mid","neuronKind":"math.passThrough"}"#, 120.0, 0.0)
            .unwrap();
        host.insert_between("slider", "out", &mid, "number", "out").unwrap();
        assert!(host.fixture.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == "mid"));
        assert!(host.fixture.synapses.iter().any(|synapse| synapse.from == "mid" && synapse.to == "add"));
        assert!(host.fixture.synapses.iter().any(|synapse| synapse.from == "add" && synapse.to == "preview"));
        assert!(!host.fixture.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == "add"));
    }

    #[test]
    fn make_space_shifts_widgets_right_of_anchor() {
        let mut host = host_with_test_bridge();
        host.fixture.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
        host.fixture.layout.insert("add".into(), WidgetLayout { x: 200.0, y: 0.0 });
        host.fixture.layout.insert("preview".into(), WidgetLayout { x: 400.0, y: 0.0 });
        host.rebuild_dag();
        host.make_space("slider", 100.0, 0.0).unwrap();
        assert!((host.fixture.layout.get("slider").expect("slider").x - 0.0).abs() < 1e-6);
        assert!((host.fixture.layout.get("add").expect("add").x - 300.0).abs() < 1e-6);
        assert!((host.fixture.layout.get("preview").expect("preview").x - 500.0).abs() < 1e-6);
    }

    #[test]
    fn set_neuron_params_merges_into_eval_input() {
        let mut host = host_with_test_bridge();
        let preview_synapse = host
            .fixture
            .synapses
            .iter()
            .find(|synapse| synapse.from == "add" && synapse.to == "preview")
            .map(|synapse| synapse.id.clone())
            .expect("preview synapse");
        host.disconnect(&preview_synapse).unwrap();
        let id = host
            .add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough"}"#, 100.0, 0.0)
            .unwrap();
        host.connect_ports(&id, "out", "preview", "in").unwrap();
        host.set_neuron_params(&id, r#"{"number":{"$schema":"number","value":7.5}}"#).unwrap();
        assert_eq!(host.preview_text(), "7.5");
    }

    #[test]
    fn cluster_ports_from_contract() {
        let inner = Tree {
            neurons: vec![
                Neuron::with_kind(
                    "in_a",
                    INPUT_KIND,
                    Dictionary::new()
                        .insert("channel", NeuralValue::Atom(Atom::String("a".into())))
                        .insert("operators", NeuralValue::Atom(Atom::String("core.number".into()))),
                ),
                Neuron::with_kind(
                    "out_sum",
                    OUTPUT_KIND,
                    Dictionary::new()
                        .insert("channel", NeuralValue::Atom(Atom::String("sum".into())))
                        .insert("operators", NeuralValue::Atom(Atom::String("core.number".into()))),
                ),
            ],
            synapses: vec![],
        };
        let widget = Widget::Cluster {
            id: "cluster".into(),
            name: "Add cluster".into(),
            tree: inner,
            flow: FlowGuiV1::default(),
        };
        let (inputs, outputs, _, _) = widget_io_ports(&widget, &[], &HashMap::new());
        assert_eq!(inputs.len(), 1);
        assert_eq!(outputs.len(), 1);
        assert_eq!(inputs[0].id, "a");
        assert_eq!(outputs[0].id, "sum");
    }

    #[test]
    fn collapse_then_explode_round_trips() {
        let mut host = host_with_test_bridge();
        host.fixture.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
        host.fixture.layout.insert("add".into(), WidgetLayout { x: 200.0, y: 0.0 });
        host.rebuild_dag();
        let cluster_id = host.collapse_selection(&["slider".into(), "add".into()]).unwrap();
        assert!(host.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Cluster { id, .. } if id == &cluster_id)));
        host.explode_cluster(&cluster_id).unwrap();
        assert!(host.fixture.widgets.iter().any(|widget| widget_id_for(widget).starts_with(&format!("{cluster_id}/"))));
        assert!(!host.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Cluster { .. })));
    }
}
// #endregion 🔖Tests
