//! 🌊 Flow core: widgets, neural evaluation, and DAG canvas host.

pub use infinite_board_port_directed_dag as dag;
pub use infinite_cavas as cavas;
pub use neural_engine as neural;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::OnceLock;

use dag::{
    computation_node_height, computation_node_width, dag_fixture_execution_rows, dag_fixture_to_wire_literal, fit_node_size, image_widget_size, io_widget_height, io_widget_width, normalize_node_display, note_widget_size, preview_widget_size,
    slider_widget_height, slider_widget_width, would_create_cycle, DagFixture, DagFixtureEdge, DagHost, DagLayoutOptions, DagNodeKind, DagNodeSpec, DagPreviewContent, EdgeRouteStyle,
    IoPortSpec,
};
use mathematical_graph_manifest::{PropertyBag, PropertyValue};
use neural::{
    channel_output, cluster_operator_info, compute_dirty_set, Atom, BudgetedEval, ChannelSpec, Dictionary, EvalChannels, EvalError, Evaluator, NeuralCache, Neuron, OperatorInfo, Synapse, Tree, TreeSnapshot, Value as NeuralValue, CLUSTER_KIND,
    INPUT_KIND, OUTPUT_KIND,
};
use serde::{Deserialize, Serialize};

// #region 🔖Document
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

fn default_variable_name() -> String {
    "value".into()
}

fn default_variable_schema() -> String {
    "dictionary".into()
}

fn default_export_format() -> String {
    "svg".into()
}

fn export_widget_display_meta(format: &str) -> (String, String, String) {
    let normalized = format.trim();
    let format_key = if normalized.is_empty() { "svg" } else { normalized };
    let upper = format_key.to_uppercase();
    let name = format!("Export {upper}");
    (name, upper, "emoji:📤".into())
}

/// 📍 Persisted node position on the canvas.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct WidgetLayout {
    pub x: f64,
    pub y: f64,
}

/// 🧾 Serializable flow document with authoritative neural tree and strippable UI.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowDocument {
    pub schema: String,
    pub tree: Tree,
    pub ui: FlowUi,
}

/// 🖼️ GUI-only flow data that can be removed without destroying logic.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowUi {
    pub camera: CameraJson,
    pub nodes: BTreeMap<String, FlowNodeGui>,
    #[serde(default)]
    pub previews: Vec<FlowPreviewGui>,
}

/// 🖼️ Alias retained for cluster widget serde compatibility.
pub type FlowGui = FlowUi;

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
    Plain {
        #[serde(default = "default_neuron_preview")]
        preview: bool,
    },
    Slider {
        min: f64,
        max: f64,
        step: f64,
        #[serde(default = "default_slider_value")]
        value: f64,
    },
    Note {
        #[serde(default)]
        text: String,
    },
    Image {
        #[serde(default)]
        src: String,
    },
    Variable {
        name: String,
        schema: String,
    },
}

/// 👁️ GUI-only preview binding.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowPreviewGui {
    pub id: String,
    pub source: Option<FlowChannelRef>,
    pub mode: String,
    #[serde(default)]
    pub preview: Dictionary,
    #[serde(default)]
    pub expanded: BTreeSet<String>,
    #[serde(default)]
    pub layout: Option<WidgetLayout>,
}

/// 📡 Serializable channel reference.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowChannelRef {
    pub neuron: String,
    pub channel: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct CameraJson {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

fn default_from_port() -> String {
    String::new()
}

fn default_to_port() -> String {
    String::new()
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

/// 🎛️ Flow widget discriminant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Widget {
    Neuron {
        id: String,
        #[serde(rename = "neuronKind")]
        neuron_kind: String,
        #[serde(default)]
        params: Dictionary,
        #[serde(default, alias = "input_ports")]
        input_ports: Vec<String>,
        #[serde(default, alias = "output_ports")]
        output_ports: Vec<String>,
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
    InputNote {
        id: String,
        #[serde(default)]
        text: String,
    },
    InputImage {
        id: String,
        #[serde(default)]
        src: String,
    },
    Variable {
        id: String,
        #[serde(default = "default_variable_name")]
        name: String,
        #[serde(default = "default_variable_schema")]
        schema: String,
    },
    OutputPreview {
        id: String,
        #[serde(default)]
        preview: Dictionary,
        #[serde(default)]
        expanded: BTreeSet<String>,
    },
    OutputAction {
        id: String,
        #[serde(default)]
        action: String,
    },
    OutputExport {
        id: String,
        #[serde(default = "default_export_format")]
        format: String,
    },
    Cluster {
        id: String,
        #[serde(default)]
        name: String,
        tree: Tree,
        #[serde(default)]
        flow: FlowGui,
    },
}

/// 🧩 Legacy fixture format still used by {@link FlowHost} retained state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowFixture {
    pub schema: String,
    pub camera: CameraJson,
    pub widgets: Vec<Widget>,
    pub synapses: Vec<SynapseSpec>,
    #[serde(default)]
    pub layout: BTreeMap<String, WidgetLayout>,
}

impl Default for FlowFixture {
    fn default() -> Self {
        Self {
            schema: "flow.fixture".into(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            widgets: vec![
                Widget::InputSlider { id: "slider".into(), value: 3.0, min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP },
                Widget::Neuron { id: "add".into(), neuron_kind: "math.add".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: true },
                Widget::OutputPreview { id: "preview".into(), preview: Dictionary::new(), expanded: BTreeSet::new() },
            ],
            synapses: vec![
                SynapseSpec { id: "s1".into(), from: "slider".into(), to: "add".into(), from_port: "number".into(), to_port: "a".into() },
                SynapseSpec { id: "s2".into(), from: "add".into(), to: "preview".into(), from_port: "sum".into(), to_port: String::new() },
            ],
            layout: BTreeMap::new(),
        }
    }
}

impl FlowFixture {
    pub fn to_document(&self) -> FlowDocument {
        let mut nodes = BTreeMap::new();
        let mut previews = Vec::new();
        for widget in &self.widgets {
            let id = widget_id_for(widget).to_string();
            nodes.insert(id.clone(), FlowNodeGui { layout: self.layout.get(id.as_str()).cloned().unwrap_or(WidgetLayout { x: 0.0, y: 0.0 }), chrome: widget_chrome(widget) });
            if let Widget::OutputPreview { id, preview, expanded } = widget {
                let source = self.synapses.iter().find(|synapse| synapse.to == *id).map(|synapse| FlowChannelRef { neuron: synapse.from.clone(), channel: synapse.from_port.clone() });
                previews.push(FlowPreviewGui { id: id.clone(), source, mode: "text".into(), preview: preview.clone(), expanded: expanded.clone(), layout: self.layout.get(id).cloned() });
            }
        }
        FlowDocument { schema: "flow.document".into(), tree: tree_from_fixture(self, &HashMap::new()), ui: FlowUi { camera: self.camera.clone(), nodes, previews } }
    }
}

fn widget_chrome(widget: &Widget) -> NodeChrome {
    match widget {
        Widget::InputSlider { value, min, max, step, .. } => NodeChrome::Slider { min: *min, max: *max, step: *step, value: *value },
        Widget::InputNote { text, .. } => NodeChrome::Note { text: text.clone() },
        Widget::InputImage { src, .. } => NodeChrome::Image { src: src.clone() },
        Widget::Variable { name, schema, .. } => NodeChrome::Variable { name: name.clone(), schema: schema.clone() },
        Widget::Neuron { preview, .. } => NodeChrome::Plain { preview: *preview },
        Widget::Cluster { .. } => NodeChrome::Plain { preview: true },
        Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. } => NodeChrome::Plain { preview: false },
    }
}

fn tree_from_fixture(fixture: &FlowFixture, kind_infos: &HashMap<String, OperatorInfo>) -> Tree {
    let neurons = fixture
        .widgets
        .iter()
        .filter_map(|w| match w {
            Widget::Neuron { id, neuron_kind, params, output_ports, .. } => {
                let mut params = params.clone();
                if !output_ports.is_empty() {
                    params = params.insert("count", NeuralValue::Atom(Atom::Decimal(output_ports.len() as f64)));
                } else if let Some(spec) = kind_infos.get(neuron_kind).and_then(|info| info.variadic_output.as_ref()) {
                    params = params.insert("count", NeuralValue::Atom(Atom::Decimal(spec.min as f64)));
                }
                Some(Neuron { id: id.clone(), kind: neuron_kind.clone(), params, tree: None })
            }
            Widget::InputSlider { id, value, .. } => Some(Neuron { id: id.clone(), kind: "core.number".into(), params: Dictionary::new().insert("value", NeuralValue::Atom(Atom::Decimal(*value))), tree: None }),
            Widget::InputNote { id, text } => Some(Neuron { id: id.clone(), kind: "core.text".into(), params: Dictionary::new().insert("value", NeuralValue::Atom(Atom::String(text.clone()))), tree: None }),
            Widget::InputImage { id, src } => Some(Neuron { id: id.clone(), kind: "core.image".into(), params: Dictionary::new().insert("dataUrl", NeuralValue::Atom(Atom::String(src.clone()))), tree: None }),
            Widget::Variable { id, name, schema } => {
                Some(Neuron { id: id.clone(), kind: "core.variable".into(), params: Dictionary::new().insert("name", NeuralValue::Atom(Atom::String(name.clone()))).insert("schema", NeuralValue::Atom(Atom::String(schema.clone()))), tree: None })
            }
            Widget::Cluster { id, name, tree, .. } => Some(Neuron { id: id.clone(), kind: CLUSTER_KIND.into(), params: Dictionary::new().insert("name", NeuralValue::Atom(Atom::String(name.clone()))), tree: Some(Box::new(tree.clone())) }),
            _ => None,
        })
        .collect();
    let synapses = fixture.synapses.iter().map(|s| Synapse { id: s.id.clone(), from: s.from.clone(), to: s.to.clone(), from_port: s.from_port.clone(), to_port: s.to_port.clone() }).collect();
    Tree { neurons, synapses }
}

impl Default for FlowDocument {
    fn default() -> Self {
        Self {
            schema: "flow.document".into(),
            tree: Tree {
                neurons: vec![
                    Neuron::with_kind("slider", INPUT_KIND, Dictionary::new().insert("channel", NeuralValue::Atom(Atom::String("number".into())))),
                    Neuron::with_kind("add", "math.add", Dictionary::new()),
                    Neuron::with_kind("out_sum", OUTPUT_KIND, Dictionary::new().insert("channel", NeuralValue::Atom(Atom::String("sum".into())))),
                ],
                synapses: vec![
                    Synapse { id: "s1".into(), from: "slider".into(), to: "add".into(), from_port: String::new(), to_port: "a".into() },
                    Synapse { id: "s2".into(), from: "add".into(), to: "out_sum".into(), from_port: "sum".into(), to_port: String::new() },
                ],
            },
            ui: FlowUi {
                camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
                nodes: BTreeMap::from([
                    ("slider".into(), FlowNodeGui { layout: WidgetLayout { x: 0.0, y: 0.0 }, chrome: NodeChrome::Slider { min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP, value: 3.0 } }),
                    ("add".into(), FlowNodeGui { layout: WidgetLayout { x: 200.0, y: 0.0 }, chrome: NodeChrome::Plain { preview: true } }),
                    ("out_sum".into(), FlowNodeGui { layout: WidgetLayout { x: 400.0, y: 0.0 }, chrome: NodeChrome::Plain { preview: false } }),
                ]),
                previews: vec![FlowPreviewGui {
                    id: "preview".into(),
                    source: Some(FlowChannelRef { neuron: "add".into(), channel: "sum".into() }),
                    mode: "text".into(),
                    preview: Dictionary::new(),
                    expanded: BTreeSet::new(),
                    layout: Some(WidgetLayout { x: 400.0, y: 0.0 }),
                }],
            },
        }
    }
}

fn widget_label(widget: &Widget) -> String {
    match widget {
        Widget::Neuron { neuron_kind, .. } => neuron_kind.clone(),
        Widget::InputSlider { .. } => "Slider".into(),
        Widget::InputNote { .. } => "Note".into(),
        Widget::InputImage { .. } => "Image".into(),
        Widget::Variable { name, .. } => name.clone(),
        Widget::OutputPreview { .. } => "Preview".into(),
        Widget::OutputAction { action, .. } => action.clone(),
        Widget::OutputExport { format, .. } => format.to_uppercase(),
        Widget::Cluster { name, .. } => {
            if name.is_empty() {
                "Cluster".into()
            } else {
                name.clone()
            }
        }
    }
}

fn widget_display_meta(widget: &Widget, kind_infos: &HashMap<String, OperatorInfo>) -> (String, String, String) {
    match widget {
        Widget::Neuron { neuron_kind, .. } => kind_infos.get(neuron_kind).map(|info| (info.name.clone(), info.abbreviation.clone(), info.icon.clone())).unwrap_or_else(|| {
            let (name, abbreviation) = dag::normalize_node_display(neuron_kind, neuron_kind);
            (name, abbreviation, String::new())
        }),
        Widget::InputSlider { .. } => ("Slider".into(), "Slider".into(), "emoji:🎚️".into()),
        Widget::InputNote { .. } => ("Note".into(), "Note".into(), "emoji:📝".into()),
        Widget::InputImage { .. } => ("Image".into(), "Image".into(), "emoji:🖼️".into()),
        Widget::Variable { name, .. } => (name.clone(), name.chars().take(3).collect::<String>(), "emoji:🔣".into()),
        Widget::OutputPreview { .. } => ("Preview".into(), "Preview".into(), "emoji:👁️".into()),
        Widget::OutputAction { action, .. } => {
            let title = if action.is_empty() { "Action" } else { action.as_str() };
            let (name, abbreviation) = dag::normalize_node_display(title, title);
            (name, abbreviation, "emoji:⚡".into())
        }
        Widget::OutputExport { format, .. } => export_widget_display_meta(format),
        Widget::Cluster { name, .. } => {
            let title = if name.is_empty() { "Cluster" } else { name.as_str() };
            let (display_name, abbreviation) = dag::normalize_node_display(title, title);
            (display_name, abbreviation, "emoji:🧩".into())
        }
    }
}

fn variable_port_label(name: &str) -> (String, String, String) {
    let code: String = name.chars().take(1).collect::<String>().to_uppercase();
    let abbrev: String = if name.len() <= 3 { name.to_string() } else { name.chars().take(3).collect() };
    let code = if code.is_empty() { "V".into() } else { code };
    let abbrev = if abbrev.is_empty() { "val".into() } else { abbrev };
    (code, abbrev, name.to_string())
}

fn variable_io_ports(name: &str, schema: &str) -> (Vec<IoPortSpec>, Vec<IoPortSpec>) {
    let (code, abbrev, full_name) = variable_port_label(name);
    let mut input = IoPortSpec::named(&code, &abbrev, name, &full_name);
    input.value_type = Some(schema.to_string());
    let mut output = IoPortSpec::named(&code, &abbrev, name, &full_name);
    output.value_type = Some(schema.to_string());
    (vec![input], vec![output])
}

fn cluster_io_layout(cluster_id: &str, name: &str, tree: &Tree, synapses: &[SynapseSpec]) -> (Vec<IoPortSpec>, Vec<IoPortSpec>) {
    let info = cluster_operator_info(cluster_id, name, tree);
    let inputs = info.inputs.iter().map(|spec| input_spec_to_port(spec, &Dictionary::new(), is_port_connected(synapses, cluster_id, &spec.name))).collect();
    let outputs = info.outputs.iter().map(channel_spec_to_output_port).collect();
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

fn channel_spec_to_output_port(spec: &ChannelSpec) -> IoPortSpec {
    let mut port = IoPortSpec::named(&spec.code, &spec.abbreviation, &spec.name, &spec.full_name);
    port.label = spec.label.clone().unwrap_or_else(|| spec.code.clone());
    port.value_type = channel_spec_value_type(spec);
    port.default = spec.default.as_ref().map(neural_value_to_json_value);
    port.cardinality = spec.cardinality.symbol();
    port
}

fn input_spec_to_port(spec: &ChannelSpec, params: &Dictionary, connected: bool) -> IoPortSpec {
    let value = params.get(&spec.name).or(spec.default.as_ref()).map(neural_value_to_json_value);
    let mut port = IoPortSpec::named(&spec.code, &spec.abbreviation, &spec.name, &spec.full_name);
    port.label = spec.label.clone().unwrap_or_else(|| spec.code.clone());
    port.value_type = channel_spec_value_type(spec);
    port.default = spec.default.as_ref().map(neural_value_to_json_value);
    port.value = value;
    port.connected = Some(connected);
    port.cardinality = spec.cardinality.symbol();
    port
}

fn default_neuron_input_ports(kind: &str, input_ports: &[String], kind_infos: &HashMap<String, OperatorInfo>) -> Vec<String> {
    if !input_ports.is_empty() {
        return input_ports.to_vec();
    }
    if let Some(spec) = kind_infos.get(kind).and_then(|info| info.variadic_input.as_ref()) {
        return (0..spec.min).map(|index| index.to_string()).collect();
    }
    if let Some(info) = kind_infos.get(kind) {
        if !info.inputs.is_empty() && info.inputs[0].name != "*" {
            return info.inputs.iter().map(|entry| entry.name.clone()).collect();
        }
    }
    vec![]
}

fn variadic_output_label(index: usize) -> String {
    if index == 0 {
        "i".into()
    } else {
        format!("i+{index}")
    }
}

fn default_neuron_output_ports(kind: &str, output_ports: &[String], kind_infos: &HashMap<String, OperatorInfo>) -> Vec<String> {
    if !output_ports.is_empty() {
        return output_ports.to_vec();
    }
    if let Some(spec) = kind_infos.get(kind).and_then(|info| info.variadic_output.as_ref()) {
        return (0..spec.min).map(|index| index.to_string()).collect();
    }
    vec![]
}

fn build_variadic_output_ports(neuron_kind: &str, output_ports: &[String], kind_infos: &HashMap<String, OperatorInfo>) -> Vec<IoPortSpec> {
    let ports = default_neuron_output_ports(neuron_kind, output_ports, kind_infos);
    let output_spec = kind_infos.get(neuron_kind).and_then(|info| info.outputs.first());
    ports
        .iter()
        .enumerate()
        .map(|(index, port_id)| {
            let label = variadic_output_label(index);
            let mut port = if let Some(spec) = output_spec {
                let mut entry = channel_spec_to_output_port(spec);
                entry.id = port_id.clone();
                entry.label = label.clone();
                entry.code = label.clone();
                entry.abbreviation = label.clone();
                entry.full_name = if index == 0 { "IndexValue".into() } else { format!("IndexValuePlus{index}") };
                entry
            } else {
                IoPortSpec::simple(&label, &label)
            };
            port.id = port_id.clone();
            port
        })
        .collect()
}

fn neuron_output_ports(neuron_kind: &str, output_ports: &[String], kind_infos: &HashMap<String, OperatorInfo>) -> (Vec<IoPortSpec>, bool) {
    let info = kind_infos.get(neuron_kind);
    let has_variadic_output = info.and_then(|entry| entry.variadic_output.as_ref()).is_some();
    let outputs = if has_variadic_output { build_variadic_output_ports(neuron_kind, output_ports, kind_infos) } else { info.map(|entry| entry.outputs.iter().map(channel_spec_to_output_port).collect::<Vec<_>>()).unwrap_or_default() };
    (outputs, has_variadic_output)
}

fn neuron_io_layout(neuron_id: &str, neuron_kind: &str, input_ports: &[String], output_ports: &[String], params: &Dictionary, synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> (Vec<IoPortSpec>, Vec<IoPortSpec>, bool, bool) {
    let info = kind_infos.get(neuron_kind);
    let (outputs, has_variadic_output) = neuron_output_ports(neuron_kind, output_ports, kind_infos);
    if let Some(_spec) = info.and_then(|entry| entry.variadic_input.as_ref()) {
        let ports = default_neuron_input_ports(neuron_kind, input_ports, kind_infos);
        let inputs = ports
            .iter()
            .map(|port_id| {
                let connected = is_port_connected(synapses, neuron_id, port_id);
                IoPortSpec { id: port_id.clone(), label: port_id.clone(), connected: Some(connected), ..Default::default() }
            })
            .collect();
        return (inputs, outputs, true, has_variadic_output);
    }
    if let Some(entry) = info {
        if !entry.inputs.is_empty() && entry.inputs[0].name != "*" {
            let inputs = entry.inputs.iter().map(|spec| input_spec_to_port(spec, params, is_port_connected(synapses, neuron_id, &spec.name))).collect();
            return (inputs, outputs, false, has_variadic_output);
        }
    }
    if !input_ports.is_empty() {
        let inputs = input_ports
            .iter()
            .map(|port_id| {
                let connected = is_port_connected(synapses, neuron_id, port_id);
                if let Some(spec) = info.and_then(|entry| entry.inputs.iter().find(|channel| channel.name == *port_id)) {
                    return input_spec_to_port(spec, params, connected);
                }
                let mut port = IoPortSpec::simple(port_id, port_id);
                port.connected = Some(connected);
                port
            })
            .collect();
        return (inputs, outputs, false, has_variadic_output);
    }
    (vec![], outputs, false, has_variadic_output)
}

fn widget_io_ports(widget: &Widget, synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> (Vec<IoPortSpec>, Vec<IoPortSpec>, bool, bool) {
    match widget {
        Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, .. } => neuron_io_layout(id, neuron_kind, input_ports, output_ports, params, synapses, kind_infos),
        Widget::InputSlider { .. } => (vec![], vec![IoPortSpec::named("N", "Num", "number", "Number")], false, false),
        Widget::InputNote { .. } => (vec![], vec![IoPortSpec::named("T", "Txt", "text", "Text")], false, false),
        Widget::InputImage { .. } => (vec![], vec![IoPortSpec::named("I", "Img", "image", "Image")], false, false),
        Widget::Variable { name, schema, .. } => {
            let (inputs, outputs) = variable_io_ports(name, schema);
            (inputs, outputs, false, false)
        }
        Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. } => (vec![], vec![], false, false),
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
            let output = IoPortSpec::named("N", "Num", "number", "Number");
            (slider_widget_width(&label, &output), slider_widget_height())
        }
        Widget::InputNote { text, .. } => note_widget_size(text),
        Widget::OutputAction { .. } | Widget::OutputExport { .. } => (io_widget_width(&label), io_widget_height(&label)),
        Widget::InputImage { src, .. } => image_widget_size(src),
        Widget::Variable { name, schema, .. } => {
            let (inputs, outputs) = variable_io_ports(name, schema);
            let (display_name, abbreviation, _) = widget_display_meta(widget, kind_infos);
            let (normalized_name, _) = normalize_node_display(&display_name, &abbreviation);
            (computation_node_width(&normalized_name, &inputs, &outputs), computation_node_height(1, 1, false, false))
        }
        Widget::OutputPreview { preview, expanded, .. } => preview_widget_size(&dag_preview_content_from_dict(preview), expanded),
        Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, .. } => {
            let (inputs, outputs, variadic_inputs, variadic_outputs) = neuron_io_layout(id, neuron_kind, input_ports, output_ports, params, synapses, kind_infos);
            let (display_name, abbreviation, _) = widget_display_meta(widget, kind_infos);
            let (normalized_name, _) = normalize_node_display(&display_name, &abbreviation);
            (computation_node_width(&normalized_name, &inputs, &outputs), computation_node_height(inputs.len(), outputs.len(), variadic_inputs, variadic_outputs))
        }
        Widget::Cluster { id, name, tree, .. } => {
            let (inputs, outputs) = cluster_io_layout(id, name, tree, synapses);
            let (display_name, abbreviation, _) = widget_display_meta(widget, kind_infos);
            let (normalized_name, _) = normalize_node_display(&display_name, &abbreviation);
            (computation_node_width(&normalized_name, &inputs, &outputs), computation_node_height(inputs.len(), outputs.len(), false, false))
        }
    }
}

fn property_bag_from_dictionary(dict: &Dictionary) -> PropertyBag {
    serde_json::from_value(serde_json::to_value(dict).unwrap_or(serde_json::Value::Null)).unwrap_or_default()
}

fn widget_operator_kind(widget: &Widget) -> Option<String> {
    match widget {
        Widget::Neuron { neuron_kind, .. } => Some(neuron_kind.clone()),
        Widget::InputSlider { .. } => Some("core.number".into()),
        Widget::InputNote { .. } => Some("core.text".into()),
        Widget::InputImage { .. } => Some("core.image".into()),
        Widget::Variable { .. } => Some("core.variable".into()),
        Widget::Cluster { .. } => Some(CLUSTER_KIND.into()),
        _ => None,
    }
}

fn widget_properties(widget: &Widget, kind_infos: &HashMap<String, OperatorInfo>) -> PropertyBag {
    match widget {
        Widget::Neuron { neuron_kind, params, output_ports, .. } => {
            let mut bag = property_bag_from_dictionary(params);
            if !output_ports.is_empty() {
                bag.insert("count".into(), PropertyValue::Number(output_ports.len() as f64));
            } else if let Some(spec) = kind_infos.get(neuron_kind).and_then(|info| info.variadic_output.as_ref()) {
                bag.insert("count".into(), PropertyValue::Number(spec.min as f64));
            }
            bag
        }
        Widget::InputSlider { value, .. } => {
            let mut bag = PropertyBag::new();
            bag.insert("value".into(), PropertyValue::Number(*value));
            bag
        }
        Widget::InputNote { text, .. } => {
            let mut bag = PropertyBag::new();
            bag.insert("value".into(), PropertyValue::String(text.clone()));
            bag
        }
        Widget::InputImage { src, .. } => {
            let mut bag = PropertyBag::new();
            bag.insert("dataUrl".into(), PropertyValue::String(src.clone()));
            bag
        }
        Widget::Variable { name, schema, .. } => {
            let mut bag = PropertyBag::new();
            bag.insert("name".into(), PropertyValue::String(name.clone()));
            bag.insert("schema".into(), PropertyValue::String(schema.clone()));
            bag
        }
        Widget::Cluster { name, tree, .. } => {
            let mut bag = PropertyBag::new();
            bag.insert("name".into(), PropertyValue::String(name.clone()));
            if let Ok(json) = serde_json::to_string(tree) {
                bag.insert("clusterTree".into(), PropertyValue::String(json));
            }
            bag
        }
        _ => PropertyBag::new(),
    }
}

fn widget_to_dag_node(widget: &Widget, index: usize, layout: &BTreeMap<String, WidgetLayout>, synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> DagNodeSpec {
    let id = match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id.clone(),
    };
    let (width, height) = widget_node_size(widget, synapses, kind_infos);
    let (x, y) = layout.get(&id).map(|p| (p.x, p.y)).unwrap_or(((index as f64) * 200.0, 0.0));
    let (name, abbreviation, icon) = widget_display_meta(widget, kind_infos);
    let mut node = match widget {
        Widget::Neuron { id: neuron_id, neuron_kind, params, input_ports, output_ports, .. } => {
            let (inputs, outputs, variadic_inputs, variadic_outputs) = neuron_io_layout(neuron_id, neuron_kind, input_ports, output_ports, params, synapses, kind_infos);
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
            operator_kind: None,
            properties: PropertyBag::new(),
            kind: DagNodeKind::Slider { min: *min, max: *max, step: *step, value: *value, output: IoPortSpec::named("N", "Num", "number", "Number") },
        },
        Widget::InputNote { text, .. } => {
            DagNodeSpec { id, name, abbreviation, icon, x, y, width, height, operator_kind: None, properties: PropertyBag::new(), kind: DagNodeKind::Note { text: text.clone(), output: IoPortSpec::named("T", "Txt", "text", "Text") } }
        }
        Widget::InputImage { src, .. } => {
            DagNodeSpec { id, name, abbreviation, icon, x, y, width, height, operator_kind: None, properties: PropertyBag::new(), kind: DagNodeKind::Image { src: src.clone(), output: IoPortSpec::named("I", "Img", "image", "Image") } }
        }
        Widget::OutputPreview { preview, expanded, .. } => DagNodeSpec {
            id,
            name,
            abbreviation,
            icon,
            x,
            y,
            width,
            height,
            operator_kind: None,
            properties: PropertyBag::new(),
            kind: DagNodeKind::Preview { content: dag_preview_content_from_dict(preview), expanded: expanded.clone(), input: IoPortSpec::named("", "", "", "PreviewInput") },
        },
        Widget::OutputAction { action, .. } => {
            DagNodeSpec { id, name, abbreviation, icon, x, y, width, height, operator_kind: None, properties: PropertyBag::new(), kind: DagNodeKind::Action { label: action.clone(), input: IoPortSpec::named("", "", "", "ActionInput") } }
        }
        Widget::OutputExport { format, .. } => {
            let (display_name, abbreviation, _) = export_widget_display_meta(format);
            DagNodeSpec {
                id,
                name: display_name,
                abbreviation,
                icon,
                x,
                y,
                width,
                height,
                operator_kind: None,
                properties: PropertyBag::new(),
                kind: DagNodeKind::Export { label: format.to_uppercase(), format: format.clone(), input: IoPortSpec::named("", "", "", "ExportInput") },
            }
        }
        Widget::Variable { name, schema, .. } => {
            let (inputs, outputs) = variable_io_ports(name, schema);
            DagNodeSpec::computation(id, name.clone(), abbreviation, icon, inputs, outputs, false, false, x, y, width, height)
        }
        Widget::Cluster { id: cluster_id, name: cluster_name, tree, .. } => {
            let (inputs, outputs) = cluster_io_layout(cluster_id, cluster_name, tree, synapses);
            DagNodeSpec::cluster(id, name, abbreviation, icon, inputs, outputs, x, y, width, height)
        }
    };
    node.operator_kind = widget_operator_kind(widget);
    node.properties = widget_properties(widget, kind_infos);
    node
}

fn parse_port_endpoint(endpoint: &str, default_port: &str) -> (String, String) {
    if let Some((widget_id, port_id)) = endpoint.split_once('@') {
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
    if places == 0 {
        1.0
    } else {
        10f64.powi(-(places as i32))
    }
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

/// 🗂️ Dictionary schema tags with dedicated preview rendering (anything else falls back to key-sniffing / a raw tree dump).
enum DictSchemaKind {
    Number,
    Text,
    Image,
}

impl DictSchemaKind {
    fn parse(schema: &str) -> Option<Self> {
        match schema {
            "number" => Some(Self::Number),
            "text" => Some(Self::Text),
            "image" => Some(Self::Image),
            _ => None,
        }
    }
}

fn dag_preview_content_from_dict(dict: &Dictionary) -> DagPreviewContent {
    match dict.schema().and_then(DictSchemaKind::parse) {
        Some(DictSchemaKind::Number) => {
            if let Some(n) = dict.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()) {
                return DagPreviewContent::Scalar { text: format_preview_number(n) };
            }
        }
        Some(DictSchemaKind::Text) => {
            if let Some(t) = dict.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) {
                return DagPreviewContent::Scalar { text: t.to_string() };
            }
        }
        Some(DictSchemaKind::Image) => {
            if let Some(src) = dict.get("dataUrl").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) {
                return DagPreviewContent::Image { src: src.to_string() };
            }
        }
        None => {}
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
    serde_json::to_value(dict).ok().map(|json| DagPreviewContent::Tree { json }).unwrap_or(DagPreviewContent::Empty)
}

fn preview_content_summary(content: &DagPreviewContent) -> String {
    match content {
        DagPreviewContent::Empty => "—".into(),
        DagPreviewContent::Scalar { text } => {
            if text.is_empty() {
                "—".into()
            } else {
                text.clone()
            }
        }
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
        #[serde(rename = "neuronKind")]
        neuron_kind: String,
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
    OutputExport {
        #[serde(default)]
        id: Option<String>,
        #[serde(default)]
        format: String,
    },
    Variable {
        #[serde(default)]
        id: Option<String>,
        #[serde(default)]
        name: Option<String>,
        #[serde(default)]
        schema: Option<String>,
    },
}

fn descriptor_explicit_id(descriptor: &WidgetDescriptor) -> Option<String> {
    let id = match descriptor {
        WidgetDescriptor::Neuron { id, .. }
        | WidgetDescriptor::InputSlider { id, .. }
        | WidgetDescriptor::InputNote { id, .. }
        | WidgetDescriptor::InputImage { id }
        | WidgetDescriptor::OutputPreview { id }
        | WidgetDescriptor::OutputAction { id, .. }
        | WidgetDescriptor::OutputExport { id, .. }
        | WidgetDescriptor::Variable { id, .. } => id.clone(),
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
        WidgetDescriptor::Neuron { neuron_kind, .. } => Widget::Neuron {
            id,
            neuron_kind: neuron_kind.clone(),
            params: Dictionary::new(),
            input_ports: default_neuron_input_ports(neuron_kind, &[], kind_infos),
            output_ports: default_neuron_output_ports(neuron_kind, &[], kind_infos),
            preview: true,
        },
        WidgetDescriptor::InputSlider { value, min, max, step, .. } => {
            let (value, min, max, step) = resolve_input_slider_fields(*value, *min, *max, *step);
            Widget::InputSlider { id, value, min, max, step }
        }
        WidgetDescriptor::InputNote { text, .. } => Widget::InputNote { id, text: text.clone().unwrap_or_default() },
        WidgetDescriptor::InputImage { .. } => Widget::InputImage { id, src: String::new() },
        WidgetDescriptor::OutputPreview { .. } => Widget::OutputPreview { id, preview: Dictionary::new(), expanded: BTreeSet::new() },
        WidgetDescriptor::OutputAction { action, .. } => Widget::OutputAction { id, action: if action.is_empty() { "log".into() } else { action.clone() } },
        WidgetDescriptor::OutputExport { format, .. } => Widget::OutputExport { id, format: if format.is_empty() { default_export_format() } else { format.clone() } },
        WidgetDescriptor::Variable { name, schema, .. } => {
            Widget::Variable { id, name: name.clone().filter(|value| !value.trim().is_empty()).unwrap_or_else(default_variable_name), schema: schema.clone().filter(|value| !value.trim().is_empty()).unwrap_or_else(default_variable_schema) }
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
    pub neuron_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
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
                CatalogueItem { kind: "inputSlider".into(), neuron_kind: None, action: None, format: None, name: "Slider".into(), abbreviation: "Slider".into(), icon: "emoji:🎚️".into(), summary: "Number input".into() },
                CatalogueItem { kind: "inputNote".into(), neuron_kind: None, action: None, format: None, name: "Note".into(), abbreviation: "Note".into(), icon: "emoji:📝".into(), summary: "Text input".into() },
                CatalogueItem { kind: "inputImage".into(), neuron_kind: None, action: None, format: None, name: "Image".into(), abbreviation: "Image".into(), icon: "emoji:🖼️".into(), summary: "Image input".into() },
                CatalogueItem { kind: "variable".into(), neuron_kind: None, action: None, format: None, name: "Variable".into(), abbreviation: "Variable".into(), icon: "emoji:🔣".into(), summary: "Named typed dictionary".into() },
            ],
        },
        CatalogueSection {
            id: "outputs".into(),
            title: "Outputs".into(),
            groups: vec![],
            items: vec![
                CatalogueItem { kind: "outputPreview".into(), neuron_kind: None, action: None, format: None, name: "Preview".into(), abbreviation: "Preview".into(), icon: "emoji:👁️".into(), summary: "Preview dictionary".into() },
                CatalogueItem { kind: "outputAction".into(), neuron_kind: None, action: Some("log".into()), format: None, name: "Action".into(), abbreviation: "Action".into(), icon: "emoji:⚡".into(), summary: "Side-effect action".into() },
                CatalogueItem {
                    kind: "outputExport".into(),
                    neuron_kind: None,
                    action: None,
                    format: Some("svg".into()),
                    name: "Export SVG".into(),
                    abbreviation: "SVG".into(),
                    icon: "emoji:📤".into(),
                    summary: "Export connected value as SVG".into(),
                },
                CatalogueItem {
                    kind: "outputExport".into(),
                    neuron_kind: None,
                    action: None,
                    format: Some("png".into()),
                    name: "Export PNG".into(),
                    abbreviation: "PNG".into(),
                    icon: "emoji:📤".into(),
                    summary: "Export connected value as PNG".into(),
                },
                CatalogueItem {
                    kind: "outputExport".into(),
                    neuron_kind: None,
                    action: None,
                    format: Some("obj".into()),
                    name: "Export OBJ".into(),
                    abbreviation: "OBJ".into(),
                    icon: "emoji:📤".into(),
                    summary: "Export connected value as OBJ".into(),
                },
                CatalogueItem {
                    kind: "outputExport".into(),
                    neuron_kind: None,
                    action: None,
                    format: Some("glb".into()),
                    name: "Export GLB".into(),
                    abbreviation: "GLB".into(),
                    icon: "emoji:📤".into(),
                    summary: "Export connected value as GLB".into(),
                },
            ],
        },
        CatalogueSection {
            id: "contract".into(),
            title: "Contract".into(),
            groups: vec![],
            items: vec![
                CatalogueItem { kind: "neuron".into(), neuron_kind: Some(INPUT_KIND.into()), action: None, format: None, name: "Input".into(), abbreviation: "In".into(), icon: "emoji:📥".into(), summary: "Cluster input contract channel".into() },
                CatalogueItem {
                    kind: "neuron".into(), neuron_kind: Some(OUTPUT_KIND.into()), action: None, format: None, name: "Output".into(), abbreviation: "Out".into(), icon: "emoji:📤".into(), summary: "Cluster output contract channel".into()
                },
            ],
        },
    ]
}

fn merge_catalogue_sections(host_json: &str) -> Result<Vec<CatalogueSection>, FlowCoreError> {
    let mut sections: Vec<CatalogueSection> = if host_json.trim().is_empty() { vec![] } else { serde_json::from_str(host_json)? };
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

/// 📚 Serializes module-grouped operator catalogue sections for host catalogue seeding.
pub fn flow_operator_catalogue_json() -> String {
    use std::collections::BTreeMap;
    let operators = flow_registry().operator_catalogue();
    let mut by_module: BTreeMap<String, Vec<neural::OperatorInfo>> = BTreeMap::new();
    for info in operators {
        by_module.entry(info.module.clone()).or_default().push(info);
    }
    let sections: Vec<CatalogueSection> = by_module
        .into_iter()
        .map(|(module, items)| CatalogueSection {
            id: module.clone(),
            title: titleize_module(&module),
            groups: vec![],
            items: items.into_iter().map(|info| CatalogueItem { kind: "neuron".into(), neuron_kind: Some(info.id), action: None, format: None, name: info.name, abbreviation: info.abbreviation, icon: info.icon, summary: info.summary }).collect(),
        })
        .collect();
    serde_json::to_string(&sections).unwrap_or_else(|_| "[]".into())
}

/// 🧠 Serializes operator catalogue entries for neuron port layout seeding.
pub fn flow_neuron_kind_infos_json() -> String {
    serde_json::to_string(&flow_registry().operator_catalogue()).unwrap_or_else(|_| "[]".into())
}

/// 🌊 Default LOD mode id for automatic camera-driven detail.
pub const FLOW_LOD_MODE_AUTOMATIC: &str = "automatic";

/// 🌊 Flow-backed NodeGraphScene fields required for wgpu FlowHost sync.
#[derive(Clone, Debug)]
pub struct FlowBackedNodeGraphExtras {
    pub fixture_json: Option<String>,
    pub operators_json: Option<String>,
    pub capabilities_json: Option<String>,
    pub lod_json: Option<String>,
    pub eval_json: Option<String>,
    pub computing_json: Option<String>,
}

/// 🌊 Builds shared NodeGraphScene fields for flow-backed plugins. `driver`, when set, contributes
/// `eval_json`/`computing_json` from an off-main-thread `flowEvalTick` chain (see [`FlowEvalDriver`]).
pub fn flow_backed_node_graph_extras(
    fixture: &FlowFixture,
    lod_mode: &str,
    proximity_distance: f64,
    grid_visible: bool,
    grid_snap_enabled: bool,
    grid_factor: f64,
    driver: Option<&FlowEvalDriver>,
) -> FlowBackedNodeGraphExtras {
    let automatic = lod_mode.is_empty() || lod_mode == FLOW_LOD_MODE_AUTOMATIC;
    FlowBackedNodeGraphExtras {
        fixture_json: serde_json::to_string(fixture).ok(),
        operators_json: Some(flow_neuron_kind_infos_json()),
        capabilities_json: Some(r#"{"engine":"flow","spotlight":true,"noteEdit":true,"clusters":true,"previewToggle":true}"#.into()),
        lod_json: Some(
            serde_json::json!({
                "automatic": automatic,
                "forcedLabel": if automatic { serde_json::Value::Null } else { serde_json::json!(lod_mode) },
                "proximityDistance": proximity_distance,
                "gridVisible": grid_visible,
                "gridSnapEnabled": grid_snap_enabled,
                "gridFactor": grid_factor,
            })
            .to_string(),
        ),
        eval_json: driver.map(|driver| driver.eval_json().to_string()),
        computing_json: driver.and_then(|driver| driver.computing_json().map(str::to_string)),
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
        flow_module_draw::register(&mut registry);
        flow_module_bim::register(&mut registry);
        registry
    })
}
// #endregion 🔖ModuleRegistry

// #region 🔖EvalBridge
#[cfg(target_arch = "wasm32")]
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
        let result = self.cb.call2(&JsValue::NULL, &JsValue::from_str(kind_id), &JsValue::from_str(&input_json)).map_err(|_| EvalError::InvalidInput("bridge call failed".into()))?;
        let result_json = result.as_string().ok_or_else(|| EvalError::InvalidInput("bridge did not return string".into()))?;
        parse_bridge_dictionary_json(&result_json)
    }
}

/// 🔌 Native eval-bridge callback: operator kind id + input dictionary in, evaluated dictionary or `EvalError` out.
#[cfg(not(target_arch = "wasm32"))]
type EvalBridgeFn = dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Send;

#[cfg(not(target_arch = "wasm32"))]
struct EvalBridge {
    cb: Box<EvalBridgeFn>,
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
    dict.keys().map(|key| (key.clone(), neural_value_to_json(dict.get(key).expect("key came from dict.keys(), so get(key) cannot miss")))).collect()
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
            if port.name == "*" {
                continue;
            }
            if let Some(value) = dict.get(&port.name) {
                ports.insert(port.name.clone(), neural_value_to_json(value));
            }
        }
        return ports;
    }
    dictionary_to_json_object(dict)
}

fn output_ports_json(dict: &Dictionary) -> serde_json::Map<String, serde_json::Value> {
    let mut ports = serde_json::Map::new();
    for key in dict.keys() {
        if let Some(value) = dict.get(key) {
            ports.insert(key.clone(), serde_json::to_value(value).unwrap_or(serde_json::Value::Null));
        }
    }
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
        let mut dict = Dictionary::new();
        for (key, val) in out_ports {
            if let Ok(value) = serde_json::from_value::<NeuralValue>(val.clone()) {
                dict = dict.insert(key.clone(), value);
            }
        }
        if !dict.is_empty() {
            outputs.insert(widget_id, dict);
        }
    }
    outputs
}

fn inputs_from_channel_eval_json(json: &str) -> HashMap<String, Dictionary> {
    let Ok(parsed) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(json) else {
        return HashMap::new();
    };
    let mut inputs = HashMap::new();
    for (widget_id, entry) in parsed {
        let Some(in_ports) = entry.get("in").and_then(|value| value.as_object()) else {
            continue;
        };
        let mut dict = Dictionary::new();
        for (key, val) in in_ports {
            if let Ok(value) = serde_json::from_value::<NeuralValue>(val.clone()) {
                dict = dict.insert(key.clone(), value);
            }
        }
        if !dict.is_empty() {
            inputs.insert(widget_id, dict);
        }
    }
    inputs
}

fn preview_dict_from_connection(src: &Dictionary, from_port: &str, to_port: &str) -> Dictionary {
    let payload = if from_port.is_empty() {
        src.clone()
    } else if let Some(dict) = src.get(from_port).and_then(|value| value.as_dictionary()) {
        dict.clone()
    } else if let Some(value) = src.get(from_port) {
        Dictionary::new().insert(from_port, value.clone())
    } else {
        Dictionary::new()
    };
    if to_port.is_empty() {
        payload
    } else {
        Dictionary::new().insert(to_port, NeuralValue::Dictionary(payload))
    }
}

fn widget_operator_info(widget: &Widget, kind_infos: &HashMap<String, OperatorInfo>) -> Option<OperatorInfo> {
    match widget {
        Widget::Neuron { neuron_kind, .. } => kind_infos.get(neuron_kind).cloned(),
        Widget::Variable { name, schema, .. } => {
            let (inputs, outputs) = variable_io_ports(name, schema);
            let info = OperatorInfo {
                id: "core.variable".into(),
                module: "core".into(),
                name: name.clone(),
                abbreviation: name.chars().take(3).collect(),
                icon: "emoji:🔣".into(),
                summary: "Named typed dictionary".into(),
                inputs: inputs.iter().map(|port| ChannelSpec::requires(&port.id, &[schema.as_str()])).collect(),
                outputs: outputs.iter().map(|port| ChannelSpec::provides(&port.id, vec![schema.clone()])).collect(),
                ..Default::default()
            };
            Some(info)
        }
        Widget::Cluster { id, name, tree, .. } => Some(cluster_operator_info(id, if name.is_empty() { "Cluster" } else { name }, tree)),
        _ => None,
    }
}

fn widget_to_inner_neuron(widget: &Widget) -> Option<Neuron> {
    match widget {
        Widget::Neuron { id, neuron_kind, params, .. } => Some(Neuron::with_kind(id, neuron_kind, params.clone())),
        Widget::InputSlider { id, value, .. } => Some(Neuron::with_kind(id, "core.number", Dictionary::new().insert("value", NeuralValue::Atom(Atom::Decimal(*value))))),
        Widget::InputNote { id, text } => Some(Neuron::with_kind(id, "core.text", Dictionary::new().insert("value", NeuralValue::Atom(Atom::String(text.clone()))))),
        Widget::InputImage { id, src } => Some(Neuron::with_kind(id, "core.image", Dictionary::new().insert("dataUrl", NeuralValue::Atom(Atom::String(src.clone()))))),
        Widget::Variable { id, name, schema } => Some(Neuron::with_kind(id, "core.variable", Dictionary::new().insert("name", NeuralValue::Atom(Atom::String(name.clone()))).insert("schema", NeuralValue::Atom(Atom::String(schema.clone()))))),
        _ => None,
    }
}

fn contract_boundary_params(channel: &str, schema: &str) -> Dictionary {
    Dictionary::new().insert("channel", NeuralValue::Atom(Atom::String(channel.into()))).insert("operators", NeuralValue::Atom(Atom::String(schema.into())))
}

fn boundary_schema_from_params(params: &Dictionary) -> String {
    params.get("operators").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or("dictionary").to_string()
}

fn variable_widget_meta(widgets: &[Widget], id: &str) -> Option<(String, String)> {
    widgets.iter().find_map(|widget| match widget {
        Widget::Variable { id: widget_id, name, schema } if widget_id == id => Some((name.clone(), schema.clone())),
        _ => None,
    })
}

fn is_variable_widget(widgets: &[Widget], id: &str) -> bool {
    widgets.iter().any(|widget| widget_id_for(widget) == id && matches!(widget, Widget::Variable { .. }))
}

fn boundary_variable_widget_ids(selected: &BTreeSet<String>, crossing: &[SynapseSpec], widgets: &[Widget]) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for synapse in crossing {
        let from_selected = selected.contains(&synapse.from);
        let to_selected = selected.contains(&synapse.to);
        if to_selected && !from_selected && is_variable_widget(widgets, &synapse.to) {
            ids.insert(synapse.to.clone());
        }
        if from_selected && !to_selected && is_variable_widget(widgets, &synapse.from) {
            ids.insert(synapse.from.clone());
        }
    }
    ids
}

fn unique_generated_boundary_name(prefix: &str, serial: &mut usize, used: &BTreeSet<String>) -> String {
    loop {
        *serial += 1;
        let candidate = format!("{prefix}{serial}");
        if !used.contains(&candidate) {
            return candidate;
        }
    }
}

fn dictionary_schema_at_port(output: &Dictionary, port: &str) -> Option<String> {
    let dict = if port.is_empty() { output.clone() } else { output.get(port).and_then(|value| value.as_dictionary()).cloned()? };
    dict.schema().map(str::to_string)
}

fn infer_port_schema(outputs: &HashMap<String, Dictionary>, kind_infos: &HashMap<String, OperatorInfo>, widgets: &[Widget], synapses: &[SynapseSpec], widget_id: &str, port: &str) -> String {
    if let Some(schema) = outputs.get(widget_id).and_then(|out| dictionary_schema_at_port(out, port)) {
        return schema;
    }
    if let Some(widget) = widgets.iter().find(|entry| widget_id_for(entry) == widget_id) {
        if let Widget::Variable { schema, .. } = widget {
            if !schema.is_empty() {
                return schema.clone();
            }
        }
        let (input_ports, output_ports, _, _) = widget_io_ports(widget, synapses, kind_infos);
        let port_spec = output_ports.iter().find(|entry| entry.id == port).or_else(|| input_ports.iter().find(|entry| entry.id == port));
        if let Some(spec) = port_spec {
            if let Some(value_type) = &spec.value_type {
                if let Some(first) = value_type.split(',').next() {
                    if !first.is_empty() && first != "value" {
                        return first.to_string();
                    }
                }
            }
        }
    }
    "dictionary".into()
}

/// 🧩 Built-in "core.*" neuron kinds recognized when reconstructing a widget from an evaluated neuron.
enum CoreNeuronKind {
    Number,
    Text,
    Image,
    Variable,
}

impl CoreNeuronKind {
    fn parse(kind: &str) -> Option<Self> {
        match kind {
            "core.number" => Some(Self::Number),
            "core.text" => Some(Self::Text),
            "core.image" => Some(Self::Image),
            "core.variable" => Some(Self::Variable),
            _ => None,
        }
    }
}

fn neuron_to_exploded_widget(neuron: &Neuron) -> Widget {
    match neuron.kind.as_str() {
        INPUT_KIND => {
            Widget::Variable { id: neuron.id.clone(), name: neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or(neuron.id.as_str()).into(), schema: boundary_schema_from_params(&neuron.params) }
        }
        OUTPUT_KIND => {
            Widget::Variable { id: neuron.id.clone(), name: neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or(neuron.id.as_str()).into(), schema: boundary_schema_from_params(&neuron.params) }
        }
        kind => match CoreNeuronKind::parse(kind) {
            Some(CoreNeuronKind::Number) => {
                Widget::InputSlider { id: neuron.id.clone(), value: neuron.params.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()).unwrap_or(3.0), min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP }
            }
            Some(CoreNeuronKind::Text) => Widget::InputNote { id: neuron.id.clone(), text: neuron.params.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or("").into() },
            Some(CoreNeuronKind::Image) => Widget::InputImage { id: neuron.id.clone(), src: neuron.params.get("dataUrl").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or("").into() },
            Some(CoreNeuronKind::Variable) => Widget::Variable {
                id: neuron.id.clone(),
                name: neuron.params.get("name").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or("value").into(),
                schema: neuron.params.get("schema").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or("dictionary").into(),
            },
            None => Widget::Neuron { id: neuron.id.clone(), neuron_kind: neuron.kind.clone(), params: neuron.params.clone(), input_ports: vec![], output_ports: vec![], preview: neuron.kind != INPUT_KIND && neuron.kind != OUTPUT_KIND },
        },
    }
}

fn build_channel_eval_json(fixture: &FlowFixture, channels: &EvalChannels, kind_infos: &HashMap<String, OperatorInfo>) -> String {
    let mut widgets = serde_json::Map::new();
    for widget in &fixture.widgets {
        let id = widget_id_for(widget);
        let operator_info = widget_operator_info(widget, kind_infos);
        let kind_info = operator_info.as_ref();
        let input_dict = match widget {
            Widget::Neuron { params, .. } => channels.inputs.get(id).cloned().unwrap_or_default().merge(params),
            _ => channels.inputs.get(id).cloned().unwrap_or_default(),
        };
        let output_dict = channels.outputs.get(id);
        let mut entry = serde_json::Map::new();
        entry.insert("in".into(), serde_json::Value::Object(input_ports_json(&input_dict, kind_info)));
        entry.insert("out".into(), serde_json::Value::Object(output_dict.map(output_ports_json).unwrap_or_default()));
        if let Some(output) = output_dict {
            if let Some(error) = output.get("error").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()) {
                entry.insert("error".into(), serde_json::Value::String(error.to_string()));
            }
        }
        widgets.insert(id.to_string(), serde_json::Value::Object(entry));
    }
    serde_json::to_string(&widgets).unwrap_or_else(|_| "{}".into())
}

fn is_brep_geometry_handle(handle: &str) -> bool {
    ["vertex-", "edge-", "wire-", "face-", "shell-", "solid-", "compound-", "curve-", "surface-"].iter().any(|prefix| handle.starts_with(prefix))
}

fn collect_geometry_handles_from_value(value: &NeuralValue, handles: &mut Vec<String>) {
    if let Some(dict) = value.as_dictionary() {
        collect_geometry_handles_from_dictionary(dict, handles);
    }
}

fn collect_geometry_handles_from_dictionary(dict: &Dictionary, handles: &mut Vec<String>) {
    if let Some(handle) = dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()) {
        if is_brep_geometry_handle(handle) {
            handles.push(handle.to_string());
        }
    }
    for key in dict.keys() {
        if let Some(value) = dict.get(key) {
            collect_geometry_handles_from_value(value, handles);
        }
    }
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

fn collect_drawing_handles_from_value(value: &NeuralValue, handles: &mut Vec<String>) {
    if let Some(dict) = value.as_dictionary() {
        collect_drawing_handles_from_dictionary(dict, handles);
    }
}

fn collect_drawing_handles_from_dictionary(dict: &Dictionary, handles: &mut Vec<String>) {
    if let Some(handle) = dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()) {
        if handle.starts_with("drawing-") {
            handles.push(handle.to_string());
        }
    }
    for key in dict.keys() {
        if let Some(value) = dict.get(key) {
            collect_drawing_handles_from_value(value, handles);
        }
    }
}

fn collect_live_drawing_handles_from_channels(channels: &EvalChannels) -> Vec<String> {
    let mut handles = Vec::new();
    for dict in channels.outputs.values().chain(channels.inputs.values()) {
        collect_drawing_handles_from_dictionary(dict, &mut handles);
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

// #region ⚠️ Errors
/// 🧯 `FlowHost`'s error type — wraps JSON codec failures, the `dag` crate's own `DagError`, and
/// this crate's own graph-editing validation failures. Every variant's Display text is byte-for-byte
/// identical to the `String` it replaces, so downstream `.to_string()` call sites (wasm_bindgen
/// `JsValue` bridging, JSON error envelopes) are unaffected.
#[derive(Debug, thiserror::Error)]
pub enum FlowCoreError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Dag(#[from] dag::DagError),
    #[error("widget id already exists: {0}")]
    WidgetIdExists(String),
    #[error("unknown widget: {0}")]
    UnknownWidget(String),
    #[error("unknown neuron widget: {0}")]
    UnknownNeuronWidget(String),
    #[error("{0} is not variadic")]
    NotVariadicInput(String),
    #[error("{0} is not variadic output")]
    NotVariadicOutput(String),
    #[error("{0} is not a neuron")]
    NotNeuron(String),
    #[error("widget is not a neuron: {0}")]
    WidgetNotNeuron(String),
    #[error("{0} reached max input ports")]
    MaxInputPortsReached(String),
    #[error("{0} reached max output ports")]
    MaxOutputPortsReached(String),
    #[error("unknown input port: {0}")]
    UnknownInputPort(String),
    #[error("unknown output port: {0}")]
    UnknownOutputPort(String),
    #[error("{widget} requires at least {min} inputs")]
    MinInputPorts { widget: String, min: usize },
    #[error("{widget} requires at least {min} outputs")]
    MinOutputPorts { widget: String, min: usize },
    #[error("{0} has no output port")]
    NoOutputPort(String),
    #[error("{0} has no input port")]
    NoInputPort(String),
    #[error("cannot connect widget to itself")]
    SelfConnection,
    #[error("cannot insert widget between itself")]
    SelfInsertion,
    #[error("connection would create cycle")]
    CycleWouldBeCreated,
    #[error("connection already exists")]
    ConnectionAlreadyExists,
    #[error("unknown synapse: {0}")]
    UnknownSynapse(String),
    #[error("unknown widget layout: {0}")]
    UnknownWidgetLayout(String),
    #[error("select at least two widgets to collapse")]
    CollapseNeedsTwoWidgets,
    #[error("selection contains unknown widgets")]
    CollapseUnknownWidgets,
    #[error("cannot collapse clusters")]
    CollapseContainsClusters,
    #[error("unknown cluster: {0}")]
    UnknownCluster(String),
    #[error("widget is not a cluster: {0}")]
    WidgetNotCluster(String),
}
// #endregion ⚠️ Errors

// #region 🔖FlowHost
/// 🏠 Retained flow host: fixture, dag scene, evaluation cache.
pub struct FlowHost {
    pub fixture: FlowFixture,
    pub dag: DagHost,
    pub outputs: HashMap<String, Dictionary>,
    export_payloads: HashMap<String, Dictionary>,
    pub last_eval_json: String,
    eval_bridge: Option<EvalBridge>,
    host_catalogue_json: String,
    kind_infos: HashMap<String, OperatorInfo>,
    neural_cache: std::sync::Arc<NeuralCache>,
    previous_snapshot: Option<TreeSnapshot>,
    previous_channels: Option<EvalChannels>,
    next_widget_serial: u64,
    next_synapse_serial: u64,
    viewport_w: u32,
    viewport_h: u32,
    viewport_dpr: f64,
    pan_anchor: Option<(f64, f64, f64, f64)>,
    ghost_node: Option<dag::DagNodeSpec>,
    /// ↩️ Undo/redo, backed by the standard `store::DocumentStore<FlowFixture, FlowOperation>`
    /// mechanism (see the `impl FlowHost`'s `🔖History` region) instead of a hand-rolled snapshot stack.
    history_store: FlowStore,
    /// 🚩 Armed by `begin_change`/`begin_gesture` for a discrete mutation not yet flushed into
    /// `history_store` — lets `can_undo` reflect it immediately, mirroring how the old snapshot stack's
    /// `begin_change` pushed synchronously instead of lazily.
    pending_change: bool,
    /// 🖐️ `true` while a coalescing gesture (drag, inline note edit) is in progress — guards
    /// `begin_change` from checkpointing mid-gesture; see `begin_gesture`/`commit_gesture_history`.
    gesture_active: bool,
}

impl Default for FlowHost {
    fn default() -> Self {
        Self::from_fixture(FlowFixture::default())
    }
}

impl FlowHost {
    pub fn from_fixture(fixture: FlowFixture) -> Self {
        Self::from_fixture_with_cache(fixture, std::sync::Arc::new(NeuralCache::new()))
    }

    /// 🧠 Builds a host sharing an existing [`NeuralCache`] — lets a long-lived caller (e.g. a
    /// stateless request/response program boundary that reconstructs `FlowHost` on every call)
    /// keep per-node memoization alive across those reconstructions instead of discarding it.
    pub fn from_fixture_with_cache(mut fixture: FlowFixture, neural_cache: std::sync::Arc<NeuralCache>) -> Self {
        dedupe_fixture_widgets(&mut fixture);
        // 🌱 A throwaway placeholder, same as `dag` below — `rebuild_dag` (via `sync_from_dag`)
        // settles auto-computed layout onto `self.fixture` before the real undo/redo baseline is
        // captured, so a fresh host never starts with a spurious undoable step.
        let history_store = FlowStore::new(create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow-host", FlowFixture::default(), None));
        let mut host = Self {
            fixture,
            dag: DagHost::from_fixture(DagFixture { schema: "dag.fixture".into(), camera: dag::DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] }),
            outputs: HashMap::new(),
            export_payloads: HashMap::new(),
            last_eval_json: String::new(),
            eval_bridge: None,
            host_catalogue_json: String::new(),
            kind_infos: HashMap::new(),
            neural_cache,
            previous_snapshot: None,
            previous_channels: None,
            next_widget_serial: 1,
            next_synapse_serial: 100,
            viewport_w: 1,
            viewport_h: 1,
            viewport_dpr: 1.0,
            pan_anchor: None,
            ghost_node: None,
            history_store,
            pending_change: false,
            gesture_active: false,
        };
        host.rebuild_dag();
        host.history_store = FlowStore::new(create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow-host", host.fixture.clone(), None));
        host
    }

    /// 📥 Replaces fixture content while keeping catalogue, operator metadata, eval bridge, and the live camera.
    pub fn replace_fixture(&mut self, fixture: FlowFixture) {
        self.apply_fixture(fixture, true, false);
    }

    /// 📥 Scene resync: reloads fixture layout/content without discarding eval baseline or cached outputs.
    pub fn resync_fixture_from_scene(&mut self, fixture: FlowFixture) {
        self.apply_fixture(fixture, false, true);
    }

    /// 📥 Replaces fixture content without clearing undo/redo history.
    pub fn set_fixture_preserving_history(&mut self, fixture: FlowFixture) {
        self.apply_fixture(fixture, false, false);
    }

    fn apply_fixture(&mut self, mut fixture: FlowFixture, reset_history: bool, preserve_eval: bool) {
        dedupe_fixture_widgets(&mut fixture);
        // 🎥 Camera is ephemeral view state (same as undo/redo) — never snap the live pan/zoom when a
        // scene resync reloads fixture content (hover, eval tick, remote operations, …).
        let camera = self.fixture.camera.clone();
        fixture.camera = camera;
        self.fixture = fixture;
        if !preserve_eval {
            self.outputs.clear();
            self.export_payloads.clear();
            self.last_eval_json.clear();
            self.previous_snapshot = None;
            self.previous_channels = None;
        }
        self.pan_anchor = None;
        self.ghost_node = None;
        self.rebuild_dag();
        if reset_history {
            // 🌱 Captured AFTER `rebuild_dag` (see `from_fixture_with_cache`'s matching comment) so the
            // new undo/redo baseline is the settled, auto-laid-out fixture, not the raw input.
            self.history_store = FlowStore::new(create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow-host", self.fixture.clone(), None));
            self.pending_change = false;
            self.gesture_active = false;
        }
    }

    pub fn parse_fixture_json(json: &str) -> Result<FlowFixture, FlowCoreError> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn fixture_json(&self) -> Result<String, FlowCoreError> {
        Ok(serde_json::to_string(&self.fixture)?)
    }

    pub fn document(&self) -> FlowDocument {
        self.fixture.to_document()
    }

    pub fn catalogue_json(&self) -> Result<String, FlowCoreError> {
        let sections = merge_catalogue_sections(&self.host_catalogue_json)?;
        Ok(serde_json::to_string(&sections)?)
    }

    pub fn set_host_catalogue_json(&mut self, json: &str) {
        self.host_catalogue_json = json.to_string();
    }

    pub fn set_neuron_kind_infos_json(&mut self, json: &str) {
        self.kind_infos = if json.trim().is_empty() { HashMap::new() } else { serde_json::from_str::<Vec<OperatorInfo>>(json).map(|items| items.into_iter().map(|info| (info.id.clone(), info)).collect()).unwrap_or_default() };
        self.rebuild_dag();
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_eval_bridge_fn(&mut self, cb: Box<EvalBridgeFn>) {
        self.eval_bridge = Some(EvalBridge { cb });
    }

    pub fn evaluate(&mut self) -> Result<String, FlowCoreError> {
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
        let inputs = inputs_from_channel_eval_json(json);
        self.outputs = outputs.clone();
        self.apply_preview_outputs(&outputs);
        self.apply_export_outputs(&outputs);
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        self.previous_snapshot = Some(TreeSnapshot::capture(&tree, &seeds));
        self.previous_channels = Some(EvalChannels { outputs, inputs });
        self.dag.clear_computing();
    }

    /// 🧵 Installs a durable eval baseline from an off-thread driver onto this ephemeral host.
    pub fn install_eval_baseline(&mut self, snapshot: Option<TreeSnapshot>, channels: Option<EvalChannels>) {
        self.previous_snapshot = snapshot;
        self.previous_channels = channels;
    }

    /// 🧵 Captures this host's eval baseline for persistence on a durable driver.
    pub fn eval_baseline(&self) -> (Option<TreeSnapshot>, Option<EvalChannels>) {
        (self.previous_snapshot.clone(), self.previous_channels.clone())
    }

    /// ⚙️ Probes pending nodes and paints active/stale computing chrome on the DAG canvas.
    pub fn refresh_computing_chrome_from_pending(&mut self) {
        let remaining = self.pending_eval_widget_ids();
        if remaining.is_empty() {
            self.dag.clear_computing();
            return;
        }
        let active = remaining.first().map(|id| id.as_str());
        let stale = remaining.get(1..).unwrap_or(&[]).to_vec();
        self.dag.set_computing_progress(active, &stale);
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
        self.fixture.camera = CameraJson { x, y, zoom: zoom.clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX) };
        self.dag.set_camera(x, y, self.fixture.camera.zoom);
    }

    pub fn wheel_zoom_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        let before = self.screen_to_world_point(sx, sy);
        let factor = if delta_y < 0.0 { ui_styling::metrics::camera::WHEEL_ZOOM_IN_FACTOR } else { ui_styling::metrics::camera::WHEEL_ZOOM_OUT_FACTOR };
        let zoom = (self.fixture.camera.zoom * factor).clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX);
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

    pub fn set_ghost_widget(&mut self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<(), FlowCoreError> {
        let descriptor: WidgetDescriptor = serde_json::from_str(descriptor_json)?;
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

    pub fn add_widget(&mut self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<String, FlowCoreError> {
        self.begin_change();
        self.clear_ghost_widget();
        let descriptor: WidgetDescriptor = serde_json::from_str(descriptor_json)?;
        let id = descriptor_explicit_id(&descriptor).unwrap_or_else(|| self.next_widget_id(&descriptor));
        if self.fixture.widgets.iter().any(|widget| widget_id_for(widget) == id) {
            return Err(FlowCoreError::WidgetIdExists(id));
        }
        let widget = widget_from_descriptor(&descriptor, id.clone(), &self.kind_infos);
        self.fixture.widgets.push(widget);
        self.fixture.layout.insert(id.clone(), WidgetLayout { x: world_x, y: world_y });
        self.rebuild_dag();
        Ok(id)
    }

    pub fn remove_widget(&mut self, widget_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let before = self.fixture.widgets.len();
        self.fixture.widgets.retain(|w| widget_id_for(w) != widget_id);
        if self.fixture.widgets.len() == before {
            return Err(FlowCoreError::UnknownWidget(widget_id.to_string()));
        }
        self.fixture.layout.remove(widget_id);
        self.fixture.synapses.retain(|s| s.from != widget_id && s.to != widget_id);
        self.rebuild_dag();
        Ok(())
    }

    pub fn move_widget(&mut self, widget_id: &str, x: f64, y: f64) -> Result<(), FlowCoreError> {
        if !self.fixture.widgets.iter().any(|w| widget_id_for(w) == widget_id) {
            return Err(FlowCoreError::UnknownWidget(widget_id.to_string()));
        }
        self.fixture.layout.insert(widget_id.to_string(), WidgetLayout { x, y });
        self.dag.set_widget_position(widget_id, x, y)?;
        Ok(())
    }

    pub fn connect(&mut self, from_id: &str, to_id: &str) -> Result<String, FlowCoreError> {
        let from_port = first_output_port(from_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos);
        let to_port = first_input_port(to_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos);
        self.connect_ports(from_id, &from_port, to_id, &to_port)
    }

    pub fn connect_ports(&mut self, from_id: &str, from_port: &str, to_id: &str, to_port: &str) -> Result<String, FlowCoreError> {
        self.begin_change();
        if from_id == to_id {
            return Err(FlowCoreError::SelfConnection);
        }
        if !widget_has_output(from_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoOutputPort(from_id.to_string()));
        }
        if !widget_has_input(to_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoInputPort(to_id.to_string()));
        }
        let existing: Vec<(String, String)> = self.fixture.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        if would_create_cycle(&existing, from_id, to_id) {
            return Err(FlowCoreError::CycleWouldBeCreated);
        }
        if self.fixture.synapses.iter().any(|s| s.from == from_id && s.from_port == from_port && s.to == to_id && s.to_port == to_port) {
            return Err(FlowCoreError::ConnectionAlreadyExists);
        }
        self.fixture.synapses.retain(|s| !(s.to == to_id && s.to_port == to_port));
        self.next_synapse_serial += 1;
        let synapse_id = format!("s{}", self.next_synapse_serial);
        self.fixture.synapses.push(SynapseSpec { id: synapse_id.clone(), from: from_id.to_string(), to: to_id.to_string(), from_port: from_port.to_string(), to_port: to_port.to_string() });
        self.rebuild_dag();
        Ok(synapse_id)
    }

    pub fn add_input_port(&mut self, widget_id: &str, index: usize) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_input.clone()).ok_or_else(|| FlowCoreError::NotVariadicInput(widget_id.to_string()))?;
        let widget = self.fixture.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { input_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let mut ports = default_neuron_input_ports(&neuron_kind, input_ports, &self.kind_infos);
        if let Some(max) = spec.max {
            if ports.len() >= max {
                return Err(FlowCoreError::MaxInputPortsReached(widget_id.to_string()));
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
        Ok(())
    }

    pub fn remove_input_port(&mut self, widget_id: &str, port_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_input.clone()).ok_or_else(|| FlowCoreError::NotVariadicInput(widget_id.to_string()))?;
        let widget = self.fixture.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { input_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let ports = default_neuron_input_ports(&neuron_kind, input_ports, &self.kind_infos);
        if ports.len() <= spec.min {
            return Err(FlowCoreError::MinInputPorts { widget: widget_id.to_string(), min: spec.min });
        }
        let Some(remove_index) = ports.iter().position(|port| port == port_id) else {
            return Err(FlowCoreError::UnknownInputPort(port_id.to_string()));
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
        Ok(())
    }

    pub fn add_output_port(&mut self, widget_id: &str, index: usize) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_output.clone()).ok_or_else(|| FlowCoreError::NotVariadicOutput(widget_id.to_string()))?;
        let widget = self.fixture.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { output_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let mut ports = default_neuron_output_ports(&neuron_kind, output_ports, &self.kind_infos);
        if let Some(max) = spec.max {
            if ports.len() >= max {
                return Err(FlowCoreError::MaxOutputPortsReached(widget_id.to_string()));
            }
        }
        let insert_at = index.min(ports.len());
        ports.insert(insert_at, insert_at.to_string());
        for synapse in &mut self.fixture.synapses {
            if synapse.from != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.from_port.parse::<usize>() {
                if old_index >= insert_at {
                    synapse.from_port = (old_index + 1).to_string();
                }
            }
        }
        *output_ports = (0..ports.len()).map(|slot| slot.to_string()).collect();
        self.rebuild_dag();
        Ok(())
    }

    pub fn remove_output_port(&mut self, widget_id: &str, port_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_output.clone()).ok_or_else(|| FlowCoreError::NotVariadicOutput(widget_id.to_string()))?;
        let widget = self.fixture.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { output_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let ports = default_neuron_output_ports(&neuron_kind, output_ports, &self.kind_infos);
        if ports.len() <= spec.min {
            return Err(FlowCoreError::MinOutputPorts { widget: widget_id.to_string(), min: spec.min });
        }
        let Some(remove_index) = ports.iter().position(|port| port == port_id) else {
            return Err(FlowCoreError::UnknownOutputPort(port_id.to_string()));
        };
        self.fixture.synapses.retain(|synapse| !(synapse.from == widget_id && synapse.from_port == port_id));
        for synapse in &mut self.fixture.synapses {
            if synapse.from != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.from_port.parse::<usize>() {
                if old_index > remove_index {
                    synapse.from_port = (old_index - 1).to_string();
                }
            }
        }
        let mut next_ports = ports;
        next_ports.remove(remove_index);
        *output_ports = (0..next_ports.len()).map(|slot| slot.to_string()).collect();
        self.rebuild_dag();
        Ok(())
    }

    pub fn disconnect(&mut self, synapse_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let before = self.fixture.synapses.len();
        self.fixture.synapses.retain(|s| s.id != synapse_id);
        if self.fixture.synapses.len() == before {
            return Err(FlowCoreError::UnknownSynapse(synapse_id.to_string()));
        }
        self.rebuild_dag();
        Ok(())
    }

    // #region GumballEditing
    /// 🔀 Splices `mid_id` between `anchor_id` and its downstream consumers on `anchor_out_port`.
    pub fn insert_between(&mut self, anchor_id: &str, anchor_out_port: &str, mid_id: &str, mid_in_port: &str, mid_out_port: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        if !self.fixture.widgets.iter().any(|widget| widget_id_for(widget) == anchor_id) {
            return Err(FlowCoreError::UnknownWidget(anchor_id.to_string()));
        }
        if !self.fixture.widgets.iter().any(|widget| widget_id_for(widget) == mid_id) {
            return Err(FlowCoreError::UnknownWidget(mid_id.to_string()));
        }
        if anchor_id == mid_id {
            return Err(FlowCoreError::SelfInsertion);
        }
        if !widget_has_output(anchor_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoOutputPort(anchor_id.to_string()));
        }
        if !widget_has_input(mid_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoInputPort(mid_id.to_string()));
        }
        if !widget_has_output(mid_id, &self.fixture.widgets, &self.fixture.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoOutputPort(mid_id.to_string()));
        }
        let existing: Vec<(String, String)> = self.fixture.synapses.iter().map(|synapse| (synapse.from.clone(), synapse.to.clone())).collect();
        if would_create_cycle(&existing, anchor_id, mid_id) {
            return Err(FlowCoreError::CycleWouldBeCreated);
        }
        let mid_has_input = self.fixture.synapses.iter().any(|synapse| synapse.to == mid_id);
        if !mid_has_input {
            for synapse in &mut self.fixture.synapses {
                if synapse.from == anchor_id && synapse.from_port == anchor_out_port {
                    synapse.from = mid_id.to_string();
                    synapse.from_port = mid_out_port.to_string();
                }
            }
        }
        if self.fixture.synapses.iter().any(|synapse| synapse.from == anchor_id && synapse.from_port == anchor_out_port && synapse.to == mid_id && synapse.to_port == mid_in_port) {
            self.rebuild_dag();
            return Ok(());
        }
        self.next_synapse_serial += 1;
        let synapse_id = format!("s{}", self.next_synapse_serial);
        self.fixture.synapses.push(SynapseSpec { id: synapse_id, from: anchor_id.to_string(), to: mid_id.to_string(), from_port: anchor_out_port.to_string(), to_port: mid_in_port.to_string() });
        self.rebuild_dag();
        Ok(())
    }

    /// ↔️ Shifts widgets to the right of `anchor_id` to open layout space for inserted nodes.
    pub fn make_space(&mut self, anchor_id: &str, dx: f64, dy: f64) -> Result<(), FlowCoreError> {
        self.begin_change();
        let anchor_x = self.fixture.layout.get(anchor_id).map(|layout| layout.x).ok_or_else(|| FlowCoreError::UnknownWidgetLayout(anchor_id.to_string()))?;
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
    pub fn set_neuron_params(&mut self, widget_id: &str, params_json: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let patch: Dictionary = serde_json::from_str(params_json)?;
        let widget = self.fixture.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { params, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        *params = params.merge(&patch);
        self.sync_dag_display_from_widgets();
        Ok(())
    }
    // #endregion GumballEditing

    /// 🌳 Recomputes widget positions from the current graph using layered tree layout.
    pub fn reorganize(&mut self, opts_json: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let opts: DagLayoutOptions = if opts_json.trim().is_empty() { DagLayoutOptions::default() } else { serde_json::from_str(opts_json)? };
        let theme = self.dag.canvas_theme;
        self.dag = DagHost::from_fixture_without_layout(self.build_dag_fixture_v1());
        self.dag.canvas_theme = theme;
        self.dag.reorganize(&opts)?;
        self.sync_from_dag();
        Ok(())
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "pointer-event handler mirroring this file's other screen-space input methods (pointer_move_screen/pointer_up_screen/wheel_screen) — position + button + modifier-key flags is the natural shape for this UI event, not a bundling candidate on its own without also restructuring its siblings"
    )]
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        if pan {
            self.pan_anchor = Some((sx, sy, self.fixture.camera.x, self.fixture.camera.y));
            return;
        }
        self.clear_ghost_widget();
        self.dag.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.begin_gesture();
        self.dag.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta, alt, false);
        if let Some((side, widget_id, index)) = self.dag.take_pending_port_insert() {
            match side {
                dag::DagPortSide::Input => {
                    let _ = self.add_input_port(&widget_id, index);
                }
                dag::DagPortSide::Output => {
                    let _ = self.add_output_port(&widget_id, index);
                }
            }
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

    pub fn selection_preview_method(&self) -> &str {
        self.dag.selection_preview_method()
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

    pub fn delete_selection(&mut self) -> Result<(), FlowCoreError> {
        if !self.dag.has_selection() {
            return Ok(());
        }
        self.begin_change();
        self.dag.delete_selected();
        self.sync_from_dag();
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
        self.evaluate_step(usize::MAX);
    }

    /// ⏳🧵 Evaluates at most `budget` cache-missed (dirty) nodes and returns the not-yet-computed
    /// widget ids in topo order — `remaining[0]` is the node currently blocking, `remaining[1..]`
    /// are downstream widgets waiting behind it. An off-main-thread caller (a program worker) resumes
    /// with another `evaluate_step` call until `remaining` is empty; a single `evaluate_step(usize::MAX)`
    /// call (via [`FlowHost::evaluate`]/`evaluate_internal`) still evaluates everything synchronously
    /// in one shot for callers that don't need to spread the work across ticks (tests, explicit
    /// worker-side `evaluate` actions that already run off the caller's main thread).
    ///
    /// `begin_epoch`/`sweep` bracket the *whole run* (every tick up to and including the completing
    /// one), not each tick: `begin_epoch` is cheap to call repeatedly (just bumps a counter), while
    /// `sweep` evicts anything not touched since — calling it before the run completes would discard
    /// earlier ticks' results. A run interleaved with another unrelated evaluation sharing the same
    /// [`NeuralCache`] (e.g. a generation-preview eval firing mid-chain) may have its in-progress
    /// entries swept early by that other call's completion; the next tick simply recomputes them —
    /// extra work, never a wrong result.
    pub fn evaluate_step(&mut self, budget: usize) -> Vec<String> {
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let snapshot = TreeSnapshot::capture(&tree, &seeds);
        let dirty = compute_dirty_set(self.previous_snapshot.as_ref(), &snapshot);
        if dirty.is_empty() && self.previous_channels.is_some() && !self.outputs.is_empty() {
            return Vec::new();
        }
        let registry = flow_registry();
        let evaluator = Evaluator::new(registry);
        self.neural_cache.begin_epoch();
        let previous = self.previous_channels.as_ref();
        let budgeted = if let Some(bridge) = self.eval_bridge.as_ref() {
            let mut dispatch = |kind: &str, input: &Dictionary| bridge.evaluate(kind, input);
            evaluator.evaluate_channels_budgeted(&tree, &seeds, &self.kind_infos, &mut dispatch, &self.neural_cache, &dirty, previous, budget)
        } else {
            let mut dispatch = |kind: &str, input: &Dictionary| registry.dispatch(kind, input);
            evaluator.evaluate_channels_budgeted(&tree, &seeds, &self.kind_infos, &mut dispatch, &self.neural_cache, &dirty, previous, budget)
        };
        match budgeted {
            Ok(BudgetedEval { channels, remaining }) => {
                self.outputs = channels.outputs.clone();
                self.apply_preview_outputs(&channels.outputs);
                self.apply_export_outputs(&channels.outputs);
                self.last_eval_json = build_channel_eval_json(&self.fixture, &channels, &self.kind_infos);
                if !remaining.is_empty() {
                    return remaining;
                }
                self.neural_cache.sweep();
                let live_handles = collect_live_geometry_handles_from_channels(&channels);
                flow_module_brep::retain_geometry_handles(&live_handles);
                let live_drawing_handles = collect_live_drawing_handles_from_channels(&channels);
                flow_module_draw::retain_drawing_handles(&live_drawing_handles);
                // 🔒 Only advance the snapshot/channels pair together, and only on success — a
                // failed evaluation keeps diffing against the last known-good state next time,
                // which is always a safe (never under-dirty) baseline.
                self.previous_snapshot = Some(snapshot);
                self.previous_channels = Some(channels);
                Vec::new()
            }
            Err(err) => {
                self.neural_cache.sweep();
                if self.last_eval_json.is_empty() || is_global_eval_error_json(&self.last_eval_json) {
                    self.last_eval_json = serde_json::json!({ "error": err.to_string() }).to_string();
                }
                Vec::new()
            }
        }
    }

    /// 👀 Probes which widget ids still need evaluation without computing anything (`budget = 0`) —
    /// used to decide whether a tick chain must be (re)armed and what to mark as computing/stale.
    pub fn pending_eval_widget_ids(&self) -> Vec<String> {
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let snapshot = TreeSnapshot::capture(&tree, &seeds);
        let dirty = compute_dirty_set(self.previous_snapshot.as_ref(), &snapshot);
        if dirty.is_empty() && self.previous_channels.is_some() && !self.outputs.is_empty() {
            return Vec::new();
        }
        let registry = flow_registry();
        let evaluator = Evaluator::new(registry);
        let previous = self.previous_channels.as_ref();
        let mut probe_never_dispatches = |kind: &str, _: &Dictionary| -> Result<Dictionary, EvalError> { Err(EvalError::InvalidInput(format!("pending_eval_widget_ids probed a dispatch for {kind}"))) };
        match evaluator.evaluate_channels_budgeted(&tree, &seeds, &self.kind_infos, &mut probe_never_dispatches, &self.neural_cache, &dirty, previous, 0) {
            Ok(BudgetedEval { remaining, .. }) => remaining,
            Err(_) => Vec::new(),
        }
    }


    fn build_tree(&self) -> Tree {
        let fixture = self.build_dag_fixture_v1();
        let (nodes, edges) = dag_fixture_execution_rows(&fixture);
        neural_dag::tree_from_dag(&nodes, &edges)
    }

    /// 📝 Renders the compiled DAG fixture as wire-literal text.
    pub fn compiled_wire_literal(&self) -> String {
        dag_fixture_to_wire_literal(&self.build_dag_fixture_v1())
    }

    fn build_seeds(&self) -> HashMap<String, Dictionary> {
        let mut seeds = HashMap::new();
        for widget in &self.fixture.widgets {
            match widget {
                Widget::InputSlider { id, value, .. } => {
                    seeds.insert(id.clone(), channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(*value)))));
                }
                Widget::InputNote { id, text } => {
                    seeds.insert(id.clone(), channel_output("text", Dictionary::with_schema("text").insert("value", NeuralValue::Atom(Atom::String(text.clone())))));
                }
                Widget::InputImage { id, src } => {
                    seeds.insert(id.clone(), channel_output("image", Dictionary::with_schema("image").insert("dataUrl", NeuralValue::Atom(Atom::String(src.clone())))));
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
                        *preview = preview_dict_from_connection(src, &syn.from_port, &syn.to_port);
                    }
                }
            }
        }
        self.sync_dag_display_from_widgets();
        self.dag.fit_preview_sizes();
    }

    fn apply_export_outputs(&mut self, outputs: &HashMap<String, Dictionary>) {
        for widget in &self.fixture.widgets {
            if let Widget::OutputExport { id, .. } = widget {
                if let Some(out) = outputs.get(id) {
                    self.export_payloads.insert(id.clone(), out.clone());
                } else if let Some(syn) = self.fixture.synapses.iter().find(|s| s.to == *id) {
                    if let Some(src) = outputs.get(&syn.from) {
                        let payload = preview_dict_from_connection(src, &syn.from_port, &syn.to_port);
                        self.export_payloads.insert(id.clone(), payload);
                    }
                }
            }
        }
    }

    pub fn export_payload_json(&self, widget_id: &str) -> Result<String, FlowCoreError> {
        let payload = self.export_payloads.get(widget_id).cloned().unwrap_or_default();
        Ok(serde_json::to_string(&payload)?)
    }

    /// 📤 Returns and clears a pending export control click from the last pointer hit.
    pub fn take_pending_export_click(&mut self) -> Option<String> {
        self.dag.take_pending_export_click()
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
                (Widget::OutputExport { format, .. }, DagNodeKind::Export { label, format: dag_format, .. }) => {
                    *label = format.to_uppercase();
                    *dag_format = format.clone();
                }
                _ => {}
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn sync_dag_ghost(&mut self) {
        self.dag.set_ghost_node(self.ghost_node.clone());
    }

    fn rebuild_dag(&mut self) {
        let fixture = self.build_dag_fixture_v1();
        let theme = self.dag.canvas_theme;
        let automatic_lod = self.dag.automatic_lod();
        let forced_draw_lod = self.dag.forced_draw_lod_label().map(str::to_string);
        let ghost = self.ghost_node.clone();
        self.dag = DagHost::from_fixture_without_layout(fixture);
        self.dag.canvas_theme = theme;
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

    /// @emoji 🎯 All pick targets under a screen point as JSON for DOM disambiguation menus.
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        self.dag.pick_targets_at_screen_json(sx, sy)
    }

    /// @emoji 🎯 Screen-space geometry for a live entity (`domain`/`id` in the pick-target grammar) —
    /// see `DagHost::entity_screen_json`. Powers introduction-demonstration semantic targeting.
    pub fn entity_screen_json(&self, domain: &str, id: &str) -> String {
        self.dag.entity_screen_json(domain, id)
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
    pub fn align_selection(&mut self, mode: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        self.dag.align_selection(mode)?;
        self.sync_from_dag();
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
    pub fn toggle_preview(&mut self, widget_id: &str) -> Result<(), FlowCoreError> {
        let Some(widget) = self.fixture.widgets.iter_mut().find(|w| widget_id_for(w) == widget_id) else {
            return Err(FlowCoreError::UnknownWidget(widget_id.to_string()));
        };
        let Widget::Neuron { preview, .. } = widget else {
            return Err(FlowCoreError::WidgetNotNeuron(widget_id.to_string()));
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
                (Widget::OutputExport { format, .. }, DagNodeKind::Export { format: dag_format, .. }) => {
                    *format = dag_format.clone();
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
                let (from, from_port) = parse_port_endpoint(&edge.source, "");
                let (to, to_port) = parse_port_endpoint(&edge.target, "");
                SynapseSpec { id: edge.id.clone(), from, to, from_port, to_port }
            })
            .collect();
        self.fixture.camera = CameraJson { x: self.dag.fixture.camera.x, y: self.dag.fixture.camera.y, zoom: self.dag.fixture.camera.zoom };
    }

    fn build_dag_fixture_v1(&self) -> DagFixture {
        let mut seen = BTreeSet::new();
        let nodes: Vec<DagNodeSpec> =
            self.fixture.widgets.iter().enumerate().filter(|(_, widget)| seen.insert(widget_id_for(widget).to_string())).map(|(i, w)| widget_to_dag_node(w, i, &self.fixture.layout, &self.fixture.synapses, &self.kind_infos)).collect();
        let existing: Vec<(String, String)> = self.fixture.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        let edges: Vec<DagFixtureEdge> = self
            .fixture
            .synapses
            .iter()
            .filter(|syn| !would_create_cycle(&existing.iter().filter(|(a, b)| !(a == &syn.from && b == &syn.to)).cloned().collect::<Vec<_>>(), &syn.from, &syn.to))
            .map(|syn| DagFixtureEdge { id: syn.id.clone(), source: format!("{}@{}", syn.from, syn.from_port), target: format!("{}@{}", syn.to, syn.to_port), route_style: EdgeRouteStyle::default(), properties: PropertyBag::new() })
            .collect();
        DagFixture { schema: "dag.fixture".into(), camera: dag::DagCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom }, nodes, edges }
    }

    fn screen_to_world_point(&self, sx: f64, sy: f64) -> cavas::Point {
        use cavas::camera::{screen_to_world, Camera, Viewport};
        use cavas::Point;
        let cam = Camera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.viewport_w, height: self.viewport_h, dpr: self.viewport_dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    fn next_widget_id(&mut self, descriptor: &WidgetDescriptor) -> String {
        self.next_widget_serial += 1;
        let prefix = match descriptor {
            WidgetDescriptor::Neuron { neuron_kind, .. } => neuron_kind.replace('.', "_"),
            WidgetDescriptor::InputSlider { .. } => "slider".into(),
            WidgetDescriptor::InputNote { .. } => "note".into(),
            WidgetDescriptor::InputImage { .. } => "image".into(),
            WidgetDescriptor::Variable { .. } => "variable".into(),
            WidgetDescriptor::OutputPreview { .. } => "preview".into(),
            WidgetDescriptor::OutputAction { .. } => "action".into(),
            WidgetDescriptor::OutputExport { .. } => "export".into(),
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
        self.refresh_computing_chrome_from_pending();
    }

    pub fn slider_overlay_state_json(&self) -> Result<String, FlowCoreError> {
        Ok(self.dag.slider_overlay_state_json()?)
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
        self.refresh_computing_chrome_from_pending();
    }

    /// ✏️ Begins inline note editing for a widget at a world-space click.
    pub fn begin_note_edit(&mut self, widget_id: &str, world_x: f64, world_y: f64) {
        self.begin_gesture();
        self.dag.begin_note_edit(widget_id, world_x, world_y);
    }

    /// ✏️ Inserts text into the active note editor.
    pub fn note_insert_text(&mut self, chunk: &str) {
        if !self.dag.note_insert_text(chunk) {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Backspaces in the active note editor.
    pub fn note_backspace(&mut self) {
        if !self.dag.note_backspace() {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Deletes forward in the active note editor.
    pub fn note_delete_forward(&mut self) {
        if !self.dag.note_delete_forward() {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Moves the active note caret.
    pub fn note_move_caret(&mut self, direction: &str, extend: bool) {
        if !self.dag.note_move_caret(direction, extend) {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Commits inline note editing into fixture history.
    pub fn note_commit_edit(&mut self) {
        self.dag.note_commit_edit();
        self.sync_from_dag();
        self.commit_gesture_history();
    }

    /// ✏️ Toggles native caret visibility while editing a note.
    pub fn set_note_caret_visible(&mut self, visible: bool) {
        self.dag.set_note_caret_visible(visible);
    }

    pub fn schemas_json(&self) -> Result<String, FlowCoreError> {
        let refs = flow_registry().schema_refs();
        Ok(serde_json::to_string(&refs)?)
    }

    pub fn set_variable_name(&mut self, widget_id: &str, name: &str) {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return;
        }
        self.begin_change();
        for widget in &mut self.fixture.widgets {
            if let Widget::Variable { id, name: variable_name, .. } = widget {
                if id == widget_id {
                    *variable_name = trimmed.to_string();
                }
            }
        }
        self.rebuild_dag();
    }

    pub fn set_variable_schema(&mut self, widget_id: &str, schema: &str) {
        let trimmed = schema.trim();
        if trimmed.is_empty() {
            return;
        }
        self.begin_change();
        for widget in &mut self.fixture.widgets {
            if let Widget::Variable { id, schema: variable_schema, .. } = widget {
                if id == widget_id {
                    *variable_schema = trimmed.to_string();
                }
            }
        }
        self.rebuild_dag();
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
        self.refresh_computing_chrome_from_pending();
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

    pub fn set_canvas_theme_from_json(&mut self, json: &str) -> Result<(), FlowCoreError> {
        Ok(self.dag.set_canvas_theme_from_json(json)?)
    }

    pub fn set_canvas_theme_dark(&mut self, dark: bool) {
        self.dag.canvas_theme = dag::CanvasPalette::from_board_palette(if dark { &ui_styling::BOARD_DARK } else { &ui_styling::BOARD_LIGHT });
    }

    pub fn paint_scene(&self, scene: &mut cavas::Scene, width: u32, height: u32, dpr: f64) {
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

    pub fn set_grid_visible(&mut self, visible: bool) {
        self.dag.set_grid_visible(visible);
    }

    pub fn set_grid_snap_enabled(&mut self, enabled: bool) {
        self.dag.set_grid_snap_enabled(enabled);
    }

    pub fn set_grid_factor(&mut self, factor: f64) -> Result<(), FlowCoreError> {
        self.dag.set_grid_factor(factor)?;
        Ok(())
    }

    pub fn focus_selection_camera(&self, pad: f64) -> Option<CameraJson> {
        self.dag.focus_selection_camera(pad).map(|camera| CameraJson { x: camera.x, y: camera.y, zoom: camera.zoom })
    }

    pub fn draw_lod_label(&self) -> &'static str {
        self.dag.draw_lod_label()
    }

    pub fn label_overlay_paint_state_json(&self) -> Result<String, FlowCoreError> {
        Ok(self.dag.label_overlay_paint_state_json()?)
    }

    /// 💥 Returns and clears a pending cluster explode target from the last pointer hit.
    pub fn take_pending_cluster_explode(&mut self) -> Option<String> {
        self.dag.take_pending_cluster_explode()
    }

    /// 🧩 Collapses the selected widgets into one cluster neuron.
    pub fn collapse_selection(&mut self, selected_ids: &[String]) -> Result<String, FlowCoreError> {
        if selected_ids.len() < 2 {
            return Err(FlowCoreError::CollapseNeedsTwoWidgets);
        }
        let selected: BTreeSet<String> = selected_ids.iter().cloned().collect();
        if !selected.iter().all(|id| self.fixture.widgets.iter().any(|widget| widget_id_for(widget) == id)) {
            return Err(FlowCoreError::CollapseUnknownWidgets);
        }
        if selected.iter().any(|id| self.fixture.widgets.iter().any(|widget| widget_id_for(widget) == id && matches!(widget, Widget::Cluster { .. }))) {
            return Err(FlowCoreError::CollapseContainsClusters);
        }
        self.begin_change();
        let mut crossing_external = Vec::new();
        for synapse in &self.fixture.synapses {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if (from_selected || to_selected) && !(from_selected && to_selected) {
                crossing_external.push(synapse.clone());
            }
        }
        let boundary_variables = boundary_variable_widget_ids(&selected, &crossing_external, &self.fixture.widgets);
        let mut inner_neurons = Vec::new();
        let mut inner_layout = BTreeMap::new();
        for widget in &self.fixture.widgets {
            let id = widget_id_for(widget).to_string();
            if !selected.contains(&id) {
                continue;
            }
            if boundary_variables.contains(&id) {
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
        let mut retained_external = Vec::new();
        for synapse in &self.fixture.synapses {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if from_selected && to_selected {
                if boundary_variables.contains(&synapse.from) || boundary_variables.contains(&synapse.to) {
                    continue;
                }
                inner_synapses.push(Synapse { id: synapse.id.clone(), from: synapse.from.clone(), to: synapse.to.clone(), from_port: synapse.from_port.clone(), to_port: synapse.to_port.clone() });
            } else if from_selected || to_selected {
            } else {
                retained_external.push(synapse.clone());
            }
        }
        let mut used_channels = BTreeSet::new();
        let mut input_serial = 0usize;
        let mut output_serial = 0usize;
        let mut boundary_index = 0usize;
        let mut cluster_external = Vec::new();
        let outputs = self.outputs.clone();
        let kind_infos = self.kind_infos.clone();
        let widgets = self.fixture.widgets.clone();
        let synapses_snapshot = self.fixture.synapses.clone();
        for synapse in crossing_external {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if to_selected && !from_selected {
                let inner_target = if boundary_variables.contains(&synapse.to) {
                    self.fixture.synapses.iter().find(|entry| entry.from == synapse.to && selected.contains(&entry.to)).map(|entry| (entry.to.clone(), entry.to_port.clone())).unwrap_or_else(|| (synapse.to.clone(), synapse.to_port.clone()))
                } else {
                    (synapse.to.clone(), synapse.to_port.clone())
                };
                let (channel, schema) = if let Some((name, schema)) = variable_widget_meta(&widgets, &synapse.to) {
                    let schema = if schema.is_empty() { infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &synapse.from, &synapse.from_port) } else { schema };
                    (name, schema)
                } else {
                    let channel = unique_generated_boundary_name("input", &mut input_serial, &used_channels);
                    let schema = infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &synapse.from, &synapse.from_port);
                    (channel, schema)
                };
                used_channels.insert(channel.clone());
                boundary_index += 1;
                let boundary_id = format!("__in_{boundary_index}");
                inner_neurons.push(Neuron::with_kind(&boundary_id, INPUT_KIND, contract_boundary_params(&channel, &schema)));
                inner_synapses.push(Synapse { id: format!("{boundary_id}_link"), from: boundary_id, to: inner_target.0, from_port: String::new(), to_port: inner_target.1 });
                cluster_external.push(SynapseSpec { id: synapse.id.clone(), from: synapse.from.clone(), to: String::new(), from_port: synapse.from_port.clone(), to_port: channel });
            } else if from_selected && !to_selected {
                let inner_source = if boundary_variables.contains(&synapse.from) {
                    self.fixture.synapses.iter().find(|entry| entry.to == synapse.from && selected.contains(&entry.from)).map(|entry| (entry.from.clone(), entry.from_port.clone())).unwrap_or_else(|| (synapse.from.clone(), synapse.from_port.clone()))
                } else {
                    (synapse.from.clone(), synapse.from_port.clone())
                };
                let (channel, schema) = if let Some((name, schema)) = variable_widget_meta(&widgets, &synapse.from) {
                    let schema = if schema.is_empty() { infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &inner_source.0, &inner_source.1) } else { schema };
                    (name, schema)
                } else {
                    let channel = unique_generated_boundary_name("output", &mut output_serial, &used_channels);
                    let schema = infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &inner_source.0, &inner_source.1);
                    (channel, schema)
                };
                used_channels.insert(channel.clone());
                boundary_index += 1;
                let boundary_id = format!("__out_{boundary_index}");
                inner_neurons.push(Neuron::with_kind(&boundary_id, OUTPUT_KIND, contract_boundary_params(&channel, &schema)));
                inner_synapses.push(Synapse { id: format!("{boundary_id}_link"), from: inner_source.0, to: boundary_id, from_port: inner_source.1, to_port: String::new() });
                cluster_external.push(SynapseSpec { id: synapse.id.clone(), from: String::new(), to: synapse.to.clone(), from_port: channel, to_port: synapse.to_port.clone() });
            }
        }
        let (sum_x, sum_y, layout_count) = selected.iter().filter_map(|id| self.fixture.layout.get(id)).fold((0.0, 0.0, 0usize), |(sx, sy, count), layout| (sx + layout.x, sy + layout.y, count + 1));
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
            flow: FlowGui { camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: inner_layout.into_iter().map(|(id, layout)| (id, FlowNodeGui { layout, chrome: NodeChrome::Plain { preview: true } })).collect(), previews: vec![] },
        };
        self.fixture.widgets.retain(|widget| !selected.contains(widget_id_for(widget)));
        self.fixture.widgets.push(cluster);
        for id in &selected {
            self.fixture.layout.remove(id);
        }
        self.fixture.layout.insert(cluster_id.clone(), WidgetLayout { x: cluster_x, y: cluster_y });
        self.fixture.synapses = retained_external;
        for synapse in cluster_external {
            if synapse.to.is_empty() {
                self.fixture.synapses.push(SynapseSpec { id: synapse.id, from: synapse.from, to: cluster_id.clone(), from_port: synapse.from_port, to_port: synapse.to_port });
            } else {
                self.fixture.synapses.push(SynapseSpec { id: synapse.id, from: cluster_id.clone(), to: synapse.to, from_port: synapse.from_port, to_port: synapse.to_port });
            }
        }
        self.rebuild_dag();
        Ok(cluster_id)
    }

    /// 💥 Explodes a cluster back into its inner widgets.
    pub fn explode_cluster(&mut self, cluster_id: &str) -> Result<(), FlowCoreError> {
        let cluster_index = self.fixture.widgets.iter().position(|widget| matches!(widget, Widget::Cluster { id, .. } if id == cluster_id)).ok_or_else(|| FlowCoreError::UnknownCluster(cluster_id.to_string()))?;
        let Widget::Cluster { tree, flow, .. } = self.fixture.widgets[cluster_index].clone() else {
            return Err(FlowCoreError::WidgetNotCluster(cluster_id.to_string()));
        };
        let cluster_layout = self.fixture.layout.get(cluster_id).cloned().unwrap_or(WidgetLayout { x: 0.0, y: 0.0 });
        self.begin_change();
        let mut boundary_channels: HashMap<String, (String, String)> = HashMap::new();
        for neuron in &tree.neurons {
            if neuron.kind == INPUT_KIND {
                let channel = neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or(neuron.id.as_str()).to_string();
                boundary_channels.insert(channel.clone(), (format!("{cluster_id}/{}", neuron.id), channel));
            } else if neuron.kind == OUTPUT_KIND {
                let channel = neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or(neuron.id.as_str()).to_string();
                boundary_channels.insert(channel.clone(), (format!("{cluster_id}/{}", neuron.id), channel));
            }
        }
        let mut restored_widgets = Vec::new();
        for neuron in &tree.neurons {
            let namespaced_id = format!("{cluster_id}/{}", neuron.id);
            if neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND {
                let widget = neuron_to_exploded_widget(neuron);
                let widget = match widget {
                    Widget::Variable { name, schema, .. } => Widget::Variable { id: namespaced_id.clone(), name, schema },
                    other => other,
                };
                let layout = flow.nodes.get(&neuron.id).map(|node| node.layout.clone()).unwrap_or(WidgetLayout { x: 0.0, y: 0.0 });
                self.fixture.layout.insert(namespaced_id.clone(), WidgetLayout { x: cluster_layout.x + layout.x, y: cluster_layout.y + layout.y });
                restored_widgets.push((namespaced_id, neuron.id.clone(), widget));
                continue;
            }
            let mut widget = neuron_to_exploded_widget(neuron);
            match &mut widget {
                Widget::Neuron { id, .. } | Widget::InputSlider { id, .. } | Widget::InputNote { id, .. } | Widget::InputImage { id, .. } | Widget::Variable { id, .. } => *id = namespaced_id.clone(),
                _ => {}
            }
            let layout = flow.nodes.get(&neuron.id).map(|node| node.layout.clone()).unwrap_or(WidgetLayout { x: 0.0, y: 0.0 });
            self.fixture.layout.insert(namespaced_id.clone(), WidgetLayout { x: cluster_layout.x + layout.x, y: cluster_layout.y + layout.y });
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
                if let Some((variable_id, variable_port)) = boundary_channels.get(&synapse.to_port) {
                    next_synapses.push(SynapseSpec { id: synapse.id.clone(), from: synapse.from.clone(), to: variable_id.clone(), from_port: synapse.from_port.clone(), to_port: variable_port.clone() });
                    continue;
                }
            } else if synapse.from == cluster_id {
                if let Some((variable_id, variable_port)) = boundary_channels.get(&synapse.from_port) {
                    next_synapses.push(SynapseSpec { id: synapse.id.clone(), from: variable_id.clone(), to: synapse.to.clone(), from_port: variable_port.clone(), to_port: synapse.to_port.clone() });
                    continue;
                }
            } else {
                next_synapses.push(synapse.clone());
            }
        }
        for synapse in &tree.synapses {
            let Some(from) = id_map.get(&synapse.from) else { continue };
            let Some(to) = id_map.get(&synapse.to) else { continue };
            let from_port = tree
                .neurons
                .iter()
                .find(|neuron| neuron.id == synapse.from && neuron.kind == INPUT_KIND)
                .and_then(|neuron| neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()))
                .map(str::to_string)
                .unwrap_or_else(|| synapse.from_port.clone());
            let to_port = tree
                .neurons
                .iter()
                .find(|neuron| neuron.id == synapse.to && neuron.kind == OUTPUT_KIND)
                .and_then(|neuron| neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()))
                .map(str::to_string)
                .unwrap_or_else(|| synapse.to_port.clone());
            self.next_synapse_serial += 1;
            next_synapses.push(SynapseSpec { id: format!("s{}", self.next_synapse_serial), from: from.clone(), to: to.clone(), from_port, to_port });
        }
        self.fixture.synapses = next_synapses;
        self.rebuild_dag();
        Ok(())
    }

    // #region History
    fn content_changed(a: &FlowFixture, b: &FlowFixture) -> bool {
        a.widgets != b.widgets || a.synapses != b.synapses || a.layout != b.layout
    }

    /// 🧾 Flushes an armed-but-not-yet-recorded discrete mutation into `history_store` as one
    /// invertible `FlowOperation::SetFixture` edit — the standard `store::DocumentStore`/`Operation`/
    /// `OperationDiff` mechanism (see `🔖Operations`) driving undo/redo here instead of the old
    /// hand-rolled `Vec<FlowFixture>` snapshot stack. Unconditional once armed (no `content_changed`
    /// gate), mirroring the old stack's unconditional `past.push` on a discrete `begin_change` — only
    /// the gesture-coalescing path (`commit_gesture_history`) skips a no-op edit.
    fn flush_pending_change(&mut self) {
        if self.pending_change {
            self.pending_change = false;
            let _ = self.history_store.dispatch(DocumentCommand::Apply { operations: vec![FlowOperation::SetFixture { fixture: self.fixture.clone() }], description: None });
        }
    }

    /// ↩️ Arms a checkpoint for the mutation about to happen, unless a gesture (`begin_gesture`) is
    /// currently coalescing several mutations into one.
    pub fn begin_change(&mut self) {
        if !self.gesture_active {
            self.flush_pending_change();
            self.pending_change = true;
        }
    }

    /// 🖐️ Starts a coalescing gesture (drag, inline note edit): flushes anything already armed first,
    /// then suppresses further `begin_change` checkpoints until `commit_gesture_history`.
    fn begin_gesture(&mut self) {
        self.flush_pending_change();
        self.gesture_active = true;
    }

    fn commit_gesture_history(&mut self) {
        if self.gesture_active {
            self.gesture_active = false;
            let committed = self.history_store.projection().unwrap_or_else(|_| self.fixture.clone());
            if Self::content_changed(&committed, &self.fixture) {
                let _ = self.history_store.dispatch(DocumentCommand::Apply { operations: vec![FlowOperation::SetFixture { fixture: self.fixture.clone() }], description: None });
            }
        }
    }

    /// ↩️ Restores the previous fixture content snapshot, keeping the current camera.
    pub fn undo(&mut self) -> bool {
        self.flush_pending_change();
        let camera = self.fixture.camera.clone();
        if self.history_store.dispatch(DocumentCommand::Undo).is_err() {
            return false;
        }
        let Ok(mut restored) = self.history_store.projection() else {
            return false;
        };
        restored.camera = camera;
        self.fixture = restored;
        self.rebuild_dag();
        true
    }

    /// ↪️ Re-applies a fixture content snapshot undone earlier, keeping the current camera.
    pub fn redo(&mut self) -> bool {
        let camera = self.fixture.camera.clone();
        if self.history_store.dispatch(DocumentCommand::Redo).is_err() {
            return false;
        }
        let Ok(mut restored) = self.history_store.projection() else {
            return false;
        };
        restored.camera = camera;
        self.fixture = restored;
        self.rebuild_dag();
        true
    }

    /// ↩️ Whether a content undo step is available.
    pub fn can_undo(&self) -> bool {
        self.pending_change || !self.history_store.applied_edit_ids().is_empty()
    }

    /// ↪️ Whether a content redo step is available.
    pub fn can_redo(&self) -> bool {
        !self.history_store.redo_edit_ids().is_empty()
    }
    // #endregion History
}

// #region 🔖EvalDriver
/// 🧵 Off-main-thread evaluation state a flow-backed program runtime embeds across its per-action
/// `FlowHost` reconstructions (the [`NeuralCache`] persists via [`FlowHost::from_fixture_with_cache`]'s
/// shared `Arc`, but everything else on `FlowHost` is rebuilt every call — this is the part that
/// needs to survive between ticks). Drives a chain of single-node ticks via `HostEffect::DispatchAction`
/// instead of one synchronous full evaluate: `sync` decides whether a tick chain must be (re)armed,
/// `tick` performs one budgeted step.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowEvalDriver {
    #[serde(default)]
    eval_json: String,
    #[serde(default)]
    computing_json: Option<String>,
    #[serde(skip)]
    previous_snapshot: Option<TreeSnapshot>,
    #[serde(skip)]
    previous_channels: Option<EvalChannels>,
    /// Guards against `sync` re-arming a chain that's already ticking — never persisted, an
    /// in-progress chain doesn't survive a process restart anyway.
    #[serde(skip)]
    tick_scheduled: bool,
}

fn flow_eval_computing_progress_json(remaining: &[String]) -> String {
    serde_json::json!({ "active": remaining.first(), "stale": remaining.get(1..).unwrap_or(&[]) }).to_string()
}

impl FlowEvalDriver {
    /// 🧵 Restores the last converged eval baseline onto a freshly built ephemeral host.
    pub fn install_baseline_into(&self, host: &mut FlowHost) {
        host.install_eval_baseline(self.previous_snapshot.clone(), self.previous_channels.clone());
    }

    /// 🧵 Persists the eval baseline from a host after a tick or convergence.
    pub fn capture_baseline_from(&mut self, host: &FlowHost) {
        let (snapshot, channels) = host.eval_baseline();
        self.previous_snapshot = snapshot;
        self.previous_channels = channels;
    }

    /// 🔁 Probes `host` for pending work (cheap — reuses `FlowHost`'s own dirty-set diffing) and
    /// reports whether a `flowEvalTick` chain needs to be (re)armed. Safe to call on every
    /// mutation/refresh; a no-operation when nothing changed or a chain is already running.
    pub fn sync(&mut self, host: &FlowHost) -> bool {
        let remaining = host.pending_eval_widget_ids();
        if remaining.is_empty() {
            self.computing_json = None;
            return false;
        }
        self.computing_json = Some(flow_eval_computing_progress_json(&remaining));
        if self.tick_scheduled {
            return false;
        }
        self.tick_scheduled = true;
        true
    }

    /// ⏱️ Runs one budgeted evaluation step on `host` and updates the driver's view of the result.
    /// Returns whether another tick is still needed.
    pub fn tick(&mut self, host: &mut FlowHost) -> bool {
        let remaining = host.evaluate_step(1);
        self.eval_json = host.last_eval_json.clone();
        self.computing_json = if remaining.is_empty() { None } else { Some(flow_eval_computing_progress_json(&remaining)) };
        if remaining.is_empty() {
            self.capture_baseline_from(host);
        }
        self.tick_scheduled = !remaining.is_empty();
        self.tick_scheduled
    }

    pub fn eval_json(&self) -> &str {
        &self.eval_json
    }

    /// ✍️ Overwrites the cached eval JSON directly — for callers that computed it out-of-band (a
    /// synchronous generation-preview eval, or an explicit "push these outputs" action) rather than
    /// via `tick`. Clears any in-progress chain since the picture it was converging toward is moot.
    pub fn set_eval_json(&mut self, eval_json: String) {
        self.eval_json = eval_json;
        self.computing_json = None;
        self.tick_scheduled = false;
        self.previous_snapshot = None;
        self.previous_channels = None;
    }

    pub fn computing_json(&self) -> Option<&str> {
        self.computing_json.as_deref()
    }

    /// ⏳ Whether a tick chain is currently arming/running.
    pub fn pending(&self) -> bool {
        self.tick_scheduled
    }
}
// #endregion 🔖EvalDriver

fn dedupe_fixture_widgets(fixture: &mut FlowFixture) {
    let mut seen = BTreeSet::new();
    fixture.widgets.retain(|widget| seen.insert(widget_id_for(widget).to_string()));
}

fn widget_id_for(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}

fn widget_has_output(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> bool {
    widgets.iter().any(|w| widget_id_for(w) == widget_id && !widget_io_ports(w, synapses, kind_infos).1.is_empty())
}

fn first_output_port(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> String {
    widgets.iter().find(|w| widget_id_for(w) == widget_id).and_then(|w| widget_io_ports(w, synapses, kind_infos).1.first().map(|port| port.id.clone())).unwrap_or_default()
}

fn first_input_port(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> String {
    widgets
        .iter()
        .find(|w| widget_id_for(w) == widget_id)
        .map(|w| match w {
            Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. } => String::new(),
            _ => widget_io_ports(w, synapses, kind_infos).0.first().map(|port| port.id.clone()).unwrap_or_default(),
        })
        .unwrap_or_default()
}

fn widget_has_input(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> bool {
    widgets.iter().any(|w| {
        if widget_id_for(w) != widget_id {
            return false;
        }
        matches!(w, Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. }) || !widget_io_ports(w, synapses, kind_infos).0.is_empty()
    })
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
        let mut scene = cavas::Scene::new();
        let clear = self.host.dag.canvas_theme.raster_clear;
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
        let fixture = FlowHost::parse_fixture_json(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let mut inner = self.state.borrow_mut();
        inner.host.replace_fixture(fixture);
        Ok(())
    }

    #[wasm_bindgen(js_name = resyncFixtureJson)]
    pub fn resync_fixture_json(&self, json: &str) -> Result<(), JsValue> {
        let fixture = FlowHost::parse_fixture_json(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let mut inner = self.state.borrow_mut();
        inner.host.resync_fixture_from_scene(fixture);
        Ok(())
    }

    #[wasm_bindgen(js_name = fixtureJson)]
    pub fn fixture_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.fixture_json().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = catalogueJson)]
    pub fn catalogue_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.catalogue_json().map_err(|e| JsValue::from_str(&e.to_string()))
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
        self.state.borrow_mut().host.add_input_port(widget_id, index as usize).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = removeInputPort)]
    pub fn remove_input_port(&self, widget_id: &str, port_id: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.remove_input_port(widget_id, port_id).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = addOutputPort)]
    pub fn add_output_port(&self, widget_id: &str, index: u32) -> Result<(), JsValue> {
        self.state.borrow_mut().host.add_output_port(widget_id, index as usize).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = removeOutputPort)]
    pub fn remove_output_port(&self, widget_id: &str, port_id: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.remove_output_port(widget_id, port_id).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = connectPorts)]
    pub fn connect_ports(&self, from_id: &str, from_port: &str, to_id: &str, to_port: &str) -> Result<String, JsValue> {
        self.state.borrow_mut().host.connect_ports(from_id, from_port, to_id, to_port).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = compiledWireLiteral)]
    pub fn compiled_wire_literal(&self) -> String {
        self.state.borrow().host.compiled_wire_literal()
    }

    #[wasm_bindgen(js_name = applyEvalOutputsJson)]
    pub fn apply_eval_outputs_json(&self, json: &str) {
        self.state.borrow_mut().host.apply_eval_outputs_json(json);
    }

    #[wasm_bindgen(js_name = setComputingProgress)]
    pub fn set_computing_progress(&self, json: &str) {
        let payload: serde_json::Value = serde_json::from_str(json).unwrap_or(serde_json::Value::Null);
        let active = payload.get("active").and_then(|value| value.as_str()).map(str::to_string);
        let stale: Vec<String> = payload.get("stale").and_then(|value| value.as_array()).map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect()).unwrap_or_default();
        self.state.borrow_mut().host.set_computing_progress(active.as_deref(), &stale);
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
        self.state.borrow_mut().host.set_hover_channel(widget_id.as_deref(), port.as_deref());
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
        self.state.borrow_mut().host.toggle_preview(widget_id).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = collapseSelection)]
    pub fn collapse_selection(&self, ids_json: &str) -> Result<String, JsValue> {
        let ids: Vec<String> = serde_json::from_str(ids_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
        self.state.borrow_mut().host.collapse_selection(&ids).map_err(|err| JsValue::from_str(&err.to_string()))
    }

    #[wasm_bindgen(js_name = explodeCluster)]
    pub fn explode_cluster(&self, cluster_id: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.explode_cluster(cluster_id).map_err(|err| JsValue::from_str(&err.to_string()))
    }

    #[wasm_bindgen(js_name = takePendingExportClick)]
    pub fn take_pending_export_click(&self) -> Option<String> {
        self.state.borrow_mut().host.take_pending_export_click()
    }

    #[wasm_bindgen(js_name = exportPayloadJson)]
    pub fn export_payload_json(&self, widget_id: &str) -> Result<String, JsValue> {
        self.state.borrow().host.export_payload_json(widget_id).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = takePendingClusterExplode)]
    pub fn take_pending_cluster_explode(&self) -> Option<String> {
        self.state.borrow_mut().host.take_pending_cluster_explode()
    }

    #[wasm_bindgen(js_name = setSliderValue)]
    pub fn set_slider_value(&self, widget_id: &str, value: f64) {
        self.state.borrow_mut().host.set_slider_value(widget_id, value);
    }

    #[wasm_bindgen(js_name = sliderOverlayStateJson)]
    pub fn slider_overlay_state_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.slider_overlay_state_json().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setNoteText)]
    pub fn set_note_text(&self, widget_id: &str, text: &str) {
        self.state.borrow_mut().host.set_note_text(widget_id, text);
    }

    #[wasm_bindgen(js_name = beginNoteEdit)]
    pub fn begin_note_edit(&self, widget_id: &str, world_x: f64, world_y: f64) {
        self.state.borrow_mut().host.begin_note_edit(widget_id, world_x, world_y);
    }

    #[wasm_bindgen(js_name = noteInsertText)]
    pub fn note_insert_text(&self, chunk: &str) {
        self.state.borrow_mut().host.note_insert_text(chunk);
    }

    #[wasm_bindgen(js_name = noteBackspace)]
    pub fn note_backspace(&self) {
        self.state.borrow_mut().host.note_backspace();
    }

    #[wasm_bindgen(js_name = noteDeleteForward)]
    pub fn note_delete_forward(&self) {
        self.state.borrow_mut().host.note_delete_forward();
    }

    #[wasm_bindgen(js_name = noteMoveCaret)]
    pub fn note_move_caret(&self, direction: &str, extend: bool) {
        self.state.borrow_mut().host.note_move_caret(direction, extend);
    }

    #[wasm_bindgen(js_name = noteCommitEdit)]
    pub fn note_commit_edit(&self) {
        self.state.borrow_mut().host.note_commit_edit();
    }

    #[wasm_bindgen(js_name = setNoteCaretVisible)]
    pub fn set_note_caret_visible(&self, visible: bool) {
        self.state.borrow_mut().host.set_note_caret_visible(visible);
    }

    #[wasm_bindgen(js_name = setImageSrc)]
    pub fn set_image_src(&self, widget_id: &str, src: &str) {
        self.state.borrow_mut().host.set_image_src(widget_id, src);
    }

    #[wasm_bindgen(js_name = schemasJson)]
    pub fn schemas_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.schemas_json().map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = setVariableName)]
    pub fn set_variable_name(&self, widget_id: &str, name: &str) {
        self.state.borrow_mut().host.set_variable_name(widget_id, name);
    }

    #[wasm_bindgen(js_name = setVariableSchema)]
    pub fn set_variable_schema(&self, widget_id: &str, schema: &str) {
        self.state.borrow_mut().host.set_variable_schema(widget_id, schema);
    }

    #[wasm_bindgen(js_name = addWidget)]
    pub fn add_widget(&self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<String, JsValue> {
        self.state.borrow_mut().host.add_widget(descriptor_json, world_x, world_y).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setGhostWidget)]
    pub fn set_ghost_widget(&self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_ghost_widget(descriptor_json, world_x, world_y).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = clearGhostWidget)]
    pub fn clear_ghost_widget(&self) {
        self.state.borrow_mut().host.clear_ghost_widget();
    }

    #[wasm_bindgen(js_name = removeWidget)]
    pub fn remove_widget(&self, widget_id: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.remove_widget(widget_id).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = moveWidget)]
    pub fn move_widget(&self, widget_id: &str, x: f64, y: f64) -> Result<(), JsValue> {
        self.state.borrow_mut().host.move_widget(widget_id, x, y).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = insertBetween)]
    pub fn insert_between(&self, anchor_id: &str, anchor_out_port: &str, mid_id: &str, mid_in_port: &str, mid_out_port: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.insert_between(anchor_id, anchor_out_port, mid_id, mid_in_port, mid_out_port).map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = makeSpace)]
    pub fn make_space(&self, anchor_id: &str, dx: f64, dy: f64) -> Result<(), JsValue> {
        self.state.borrow_mut().host.make_space(anchor_id, dx, dy).map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = setNeuronParams)]
    pub fn set_neuron_params(&self, widget_id: &str, params_json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_neuron_params(widget_id, params_json).map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = connect)]
    pub fn connect(&self, from_id: &str, to_id: &str) -> Result<String, JsValue> {
        self.state.borrow_mut().host.connect(from_id, to_id).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = disconnect)]
    pub fn disconnect(&self, synapse_id: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.disconnect(synapse_id).map_err(|e| JsValue::from_str(&e.to_string()))
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

    #[wasm_bindgen(js_name = cameraJson)]
    pub fn camera_json(&self) -> String {
        serde_json::to_string(&self.state.borrow().host.fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into())
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
        self.state.borrow().host.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e.to_string()))
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
            let (render_ctx, renderer, surface) = cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
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

    #[wasm_bindgen(js_name = setCanvasThemeJson)]
    pub fn set_canvas_theme_json(&mut self, json: &str) {
        let _ = self.state.borrow_mut().host.set_canvas_theme_from_json(json);
    }

    #[wasm_bindgen(js_name = reorganize)]
    pub fn reorganize(&self, options_json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.reorganize(options_json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&mut self) -> Result<(), JsValue> {
        self.state.borrow_mut().render_frame_gpu()
    }

    #[wasm_bindgen(js_name = pointerDownScreen)]
    #[allow(clippy::too_many_arguments, reason = "thin wasm_bindgen forwarder for FlowHost::pointer_down_screen's own justified-allow shape")]
    pub fn pointer_down_screen(&self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        self.state.borrow_mut().host.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta, alt, pan);
    }

    #[wasm_bindgen(js_name = pointerMoveScreen)]
    pub fn pointer_move_screen(&self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.state.borrow_mut().host.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt);
    }

    #[wasm_bindgen(js_name = pickTargetsAtScreenJson)]
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)
    }

    #[wasm_bindgen(js_name = entityScreenJson)]
    pub fn entity_screen_json(&self, domain: &str, id: &str) -> String {
        self.state.borrow().host.entity_screen_json(domain, id)
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

    #[wasm_bindgen(js_name = selectionPreviewMethod)]
    pub fn selection_preview_method(&self) -> String {
        self.state.borrow().host.selection_preview_method().to_string()
    }

    #[wasm_bindgen(js_name = selectionUnionBoundsScreenJson)]
    pub fn selection_union_bounds_screen_json(&self) -> String {
        self.state.borrow().host.selection_union_bounds_screen_json()
    }

    #[wasm_bindgen(js_name = alignSelection)]
    pub fn align_selection(&self, mode: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.align_selection(mode).map_err(|e| JsValue::from_str(&e.to_string()))
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
        self.state.borrow_mut().host.delete_selection().map_err(|e| JsValue::from_str(&e.to_string()))
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
pub fn render_drawing_scene(handle: &str) -> String {
    flow_module_draw::render_scene_json(handle)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn export_drawing_svg(handle: &str) -> String {
    flow_module_draw::export_svg_json(handle)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn export_drawing_pdf(handle: &str) -> String {
    flow_module_draw::export_pdf_json(handle)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn export_drawing_dwg(handle: &str) -> String {
    flow_module_draw::export_dwg_json(handle)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn import_drawing_dwg(data_base64: &str) -> String {
    flow_module_draw::import_dwg_json(data_base64)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn dispose_drawing(handle: &str) {
    flow_module_draw::dispose_drawing(handle);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn trace_drawing_bitmap(width: u32, height: u32, mask: &[u8], threshold: f64, simplify_epsilon: f64) -> String {
    flow_module_draw::trace_bitmap_json(width, height, mask, threshold, simplify_epsilon)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn boolean_drawing_segments(a_json: &str, b_json: &str, operation: &str) -> String {
    flow_module_draw::boolean_segments_json(a_json, b_json, operation)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn dispose(handle: &str) {
    flow_module_brep::dispose_geometry(handle);
}

/// 📐 Encodes a `MeshData` JSON payload as base64 DWG bytes, for JS consumers holding a mesh but no drawing/geometry handle.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn dwg_encode_mesh_json(mesh_json: &str) -> String {
    let Ok(mesh) = serde_json::from_str::<semio_framework_core::MeshData>(mesh_json) else {
        return serde_json::json!({ "error": "invalid mesh json" }).to_string();
    };
    let drawing = semio_framework_core::mesh_to_dwg_drawing(&mesh);
    match semio_framework_core::dwg_to_bytes(&drawing) {
        Ok(bytes) => {
            use base64::Engine;
            serde_json::json!({ "dwg": base64::engine::general_purpose::STANDARD.encode(bytes) }).to_string()
        }
        Err(error) => serde_json::json!({ "error": error }).to_string(),
    }
}

/// 📐 Decodes base64 DWG bytes into a `MeshData` JSON payload.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn dwg_decode_mesh_json(data_base64: &str) -> String {
    use base64::Engine;
    let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(data_base64) else {
        return serde_json::json!({ "error": "invalid base64 dwg payload" }).to_string();
    };
    match semio_framework_core::dwg_from_bytes(&bytes) {
        Ok(drawing) => {
            let mesh = semio_framework_core::dwg_drawing_to_mesh(&drawing);
            serde_json::to_string(&mesh).unwrap_or_else(|_| serde_json::json!({ "error": "failed to serialize mesh" }).to_string())
        }
        Err(error) => serde_json::json!({ "error": error }).to_string(),
    }
}
// #endregion 🔖WasmSession

// #region 🔖DocumentVcs
// 🧾 `create_document_envelope`/`DocumentCommand` are unconditional (not test/wasm-only)
// because `FlowHost`'s own undo/redo (see `impl FlowHost`'s `🔖History` region) dispatches through
// them in every build.
use store::create_document_envelope;
use store::DocumentCommand;
use store::{DocumentEnvelope, DocumentStore};
use protocol::{collection_diff_from_operation, invert_collection_operation, CollectionDiff, CollectionOperation, Identified, Operation, OperationDiff, Patchable};

pub const FLOW_DOCUMENT_SCHEMA: &str = "flow.fixture";

//#region 🔖CollectionSupport
impl Identified<String> for Widget {
    fn id(&self) -> &String {
        match self {
            Widget::Neuron { id, .. }
            | Widget::InputSlider { id, .. }
            | Widget::InputNote { id, .. }
            | Widget::InputImage { id, .. }
            | Widget::Variable { id, .. }
            | Widget::OutputPreview { id, .. }
            | Widget::OutputAction { id, .. }
            | Widget::OutputExport { id, .. }
            | Widget::Cluster { id, .. } => id,
        }
    }
}

/// 🩹 Whole-value replacement patch — flow widgets are heterogeneous enum variants, so a granular
/// per-field patch buys nothing; `Patch { patch: Widget }` LWW-replaces and `diff_patch` inverts to
/// the prior widget unconditionally (never `None`, matching `invert_collection_operation`'s
/// no-panic contract for a `Patchable` whose `apply_patch` can be a genuine no-op).
impl Patchable<Widget> for Widget {
    fn apply_patch(&mut self, patch: &Widget) {
        *self = patch.clone();
    }

    fn diff_patch(&self, other: &Self) -> Option<Widget> {
        Some(other.clone())
    }
}

impl Identified<String> for SynapseSpec {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Patchable<SynapseSpec> for SynapseSpec {
    fn apply_patch(&mut self, patch: &SynapseSpec) {
        *self = patch.clone();
    }

    fn diff_patch(&self, other: &Self) -> Option<SynapseSpec> {
        Some(other.clone())
    }
}

/// ▶️ Applies a `CollectionDiff` (removed → modified → added) to an owned `Vec`.
fn apply_flow_collection_diff<TId, TItem, TPatch>(items: &mut Vec<TItem>, diff: &CollectionDiff<TId, TPatch, TItem>)
where
    TId: PartialEq,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    for id in &diff.removed {
        items.retain(|item| item.id() != id);
    }
    for patch in &diff.modified {
        if let Some(item) = items.iter_mut().find(|item| item.id() == &patch.id) {
            item.apply_patch(&patch.patch);
        }
    }
    for added in &diff.added {
        items.push(added.clone());
    }
}

/// ➕ Merges an incoming `CollectionDiff` into an existing one (coalescing two edits' diffs).
fn absorb_flow_collection_diff<TId: Clone, TItem: Clone, TPatch: Clone>(target: &mut Option<CollectionDiff<TId, TPatch, TItem>>, incoming: Option<CollectionDiff<TId, TPatch, TItem>>) {
    if let Some(next) = incoming {
        match target {
            Some(existing) => {
                existing.removed.extend(next.removed);
                existing.modified.extend(next.modified);
                existing.added.extend(next.added);
            }
            None => *target = Some(next),
        }
    }
}
//#endregion 🔖CollectionSupport

//#region 🔖Operations
/// 📍 One node-layout assignment inside a `SetLayout` operation; `None` removes the entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FlowLayoutEntry {
    pub id: String,
    #[dsl(block)]
    pub layout: Option<WidgetLayout>,
}

/// 🌊 Typed, invertible flow-document operation. `Widgets`/`Synapses` are id-keyed collection operations for
/// granular convergence; `SetLayout` moves nodes; `SetFixture` replaces the whole fixture (import/reset).
/// The camera is ephemeral view state (program runtime), never a document operation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum FlowOperation {
    Widgets(CollectionOperation<String, Widget, Widget>),
    Synapses(CollectionOperation<String, SynapseSpec, SynapseSpec>),
    SetLayout { entries: Vec<FlowLayoutEntry> },
    SetFixture { fixture: FlowFixture },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowDiff {
    pub fixture: Option<FlowFixture>,
    pub widgets: Option<CollectionDiff<String, Widget, Widget>>,
    pub synapses: Option<CollectionDiff<String, SynapseSpec, SynapseSpec>>,
    pub layout: Option<Vec<FlowLayoutEntry>>,
}

impl OperationDiff<FlowFixture> for FlowDiff {
    fn apply(&self, projection: &FlowFixture) -> FlowFixture {
        if let Some(fixture) = &self.fixture {
            return fixture.clone();
        }
        let mut next = projection.clone();
        if let Some(diff) = &self.widgets {
            apply_flow_collection_diff(&mut next.widgets, diff);
        }
        if let Some(diff) = &self.synapses {
            apply_flow_collection_diff(&mut next.synapses, diff);
        }
        if let Some(entries) = &self.layout {
            for entry in entries {
                match &entry.layout {
                    Some(layout) => {
                        next.layout.insert(entry.id.clone(), layout.clone());
                    }
                    None => {
                        next.layout.remove(&entry.id);
                    }
                }
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.fixture.is_some() {
            *self = FlowDiff { fixture: other.fixture, ..Default::default() };
            return;
        }
        absorb_flow_collection_diff(&mut self.widgets, other.widgets);
        absorb_flow_collection_diff(&mut self.synapses, other.synapses);
        if let Some(mut entries) = other.layout {
            self.layout.get_or_insert_with(Vec::new).append(&mut entries);
        }
    }
}

impl Operation<FlowFixture> for FlowOperation {
    type Diff = FlowDiff;

    fn diff(&self, projection: &FlowFixture) -> FlowDiff {
        match self {
            FlowOperation::Widgets(operation) => FlowDiff { widgets: Some(collection_diff_from_operation(&projection.widgets, operation)), ..Default::default() },
            FlowOperation::Synapses(operation) => FlowDiff { synapses: Some(collection_diff_from_operation(&projection.synapses, operation)), ..Default::default() },
            FlowOperation::SetLayout { entries } => FlowDiff { layout: Some(entries.clone()), ..Default::default() },
            FlowOperation::SetFixture { fixture } => FlowDiff { fixture: Some(fixture.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &FlowFixture) -> Vec<Self> {
        match self {
            FlowOperation::Widgets(operation) => vec![FlowOperation::Widgets(invert_collection_operation(&projection.widgets, operation))],
            FlowOperation::Synapses(operation) => vec![FlowOperation::Synapses(invert_collection_operation(&projection.synapses, operation))],
            FlowOperation::SetLayout { entries } => vec![FlowOperation::SetLayout { entries: entries.iter().map(|entry| FlowLayoutEntry { id: entry.id.clone(), layout: projection.layout.get(&entry.id).cloned() }).collect() }],
            FlowOperation::SetFixture { .. } => vec![FlowOperation::SetFixture { fixture: projection.clone() }],
        }
    }
}

/// 🌉 Host-mutation → granular-operations bridge: diffs a `FlowFixture` before/after a `FlowHost` mutation into
/// the minimal set of `FlowOperation`s, so the rich stateful engine keeps owning mutation logic (port wiring,
/// cycle checks, cluster collapse) while the document store still records convergent, invertible operations.
/// The camera is intentionally excluded (it is program runtime state).
pub fn flow_fixture_operations(before: &FlowFixture, after: &FlowFixture) -> Vec<FlowOperation> {
    let mut operations = Vec::new();
    let after_widget_ids: std::collections::BTreeSet<&str> = after.widgets.iter().map(widget_id_for).collect();
    for widget in &before.widgets {
        let id = widget_id_for(widget);
        if !after_widget_ids.contains(id) {
            operations.push(FlowOperation::Widgets(CollectionOperation::Remove { id: id.to_string() }));
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        let id = widget_id_for(widget);
        match before.widgets.iter().find(|entry| widget_id_for(entry) == id) {
            None => operations.push(FlowOperation::Widgets(CollectionOperation::Add { id: id.to_string(), item: widget.clone(), at: index })),
            Some(prev) if prev != widget => operations.push(FlowOperation::Widgets(CollectionOperation::Patch { id: id.to_string(), patch: widget.clone() })),
            Some(_) => {}
        }
    }
    let after_synapse_ids: std::collections::BTreeSet<&str> = after.synapses.iter().map(|synapse| synapse.id.as_str()).collect();
    for synapse in &before.synapses {
        if !after_synapse_ids.contains(synapse.id.as_str()) {
            operations.push(FlowOperation::Synapses(CollectionOperation::Remove { id: synapse.id.clone() }));
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        match before.synapses.iter().find(|entry| entry.id == synapse.id) {
            None => operations.push(FlowOperation::Synapses(CollectionOperation::Add { id: synapse.id.clone(), item: synapse.clone(), at: index })),
            Some(prev) if *prev != *synapse => operations.push(FlowOperation::Synapses(CollectionOperation::Patch { id: synapse.id.clone(), patch: synapse.clone() })),
            Some(_) => {}
        }
    }
    let mut entries = Vec::new();
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            entries.push(FlowLayoutEntry { id: id.clone(), layout: Some(layout.clone()) });
        }
    }
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            entries.push(FlowLayoutEntry { id: id.clone(), layout: None });
        }
    }
    if !entries.is_empty() {
        operations.push(FlowOperation::SetLayout { entries });
    }
    operations
}
//#endregion 🔖Operations

//#region 🔖Dsl
/// 🌱 `Value`/`Atom`/`Dictionary`/`Tree`/`Neuron`/`Synapse` are all defined in `neural_engine`
/// (a foreign crate out of scope for this conversion), so none of them can carry a
/// `#[derive(dsl::Dsl...)]` themselves — Rust's orphan rule requires the impl target type to live in
/// the crate that also owns the trait or the type, and neither is true here. `ValueDsl`/`TreeDsl`/
/// `NeuronNodeDsl` below are local structural twins that the real types convert to/from right at the
/// `parse_dsl`/`print_dsl`/`parse_op`/`print_op` boundary — mirroring `imperative_core::ValueDsl`'s
/// identical fix for the same foreign-`Dictionary`/`Value`/`Atom` problem one-for-one (same crate,
/// same shapes).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct ValueDsl {
    /// 🕳️ Presence-only flag (the payload is never inspected) — `Atom::Null`'s tag.
    null: Option<bool>,
    #[dsl(key = "bool")]
    boolean: Option<bool>,
    #[dsl(key = "int")]
    integer: Option<i64>,
    decimal: Option<f64>,
    text: Option<String>,
    #[dsl(key = "dict")]
    dictionary: Option<BTreeMap<String, ValueDsl>>,
}

fn value_to_value_dsl(value: &NeuralValue) -> ValueDsl {
    let mut dsl_value = ValueDsl { null: None, boolean: None, integer: None, decimal: None, text: None, dictionary: None };
    match value {
        NeuralValue::Atom(Atom::Null) => dsl_value.null = Some(true),
        NeuralValue::Atom(Atom::Boolean(b)) => dsl_value.boolean = Some(*b),
        NeuralValue::Atom(Atom::Integer(i)) => dsl_value.integer = Some(*i),
        NeuralValue::Atom(Atom::Decimal(d)) => dsl_value.decimal = Some(*d),
        NeuralValue::Atom(Atom::String(s)) => dsl_value.text = Some(s.clone()),
        NeuralValue::Dictionary(dict) => dsl_value.dictionary = Some(dictionary_to_value_dsl_map(dict)),
    }
    dsl_value
}

fn value_dsl_to_value(dsl_value: &ValueDsl) -> NeuralValue {
    if dsl_value.null.is_some() {
        return NeuralValue::Atom(Atom::Null);
    }
    if let Some(b) = dsl_value.boolean {
        return NeuralValue::Atom(Atom::Boolean(b));
    }
    if let Some(i) = dsl_value.integer {
        return NeuralValue::Atom(Atom::Integer(i));
    }
    if let Some(d) = dsl_value.decimal {
        return NeuralValue::Atom(Atom::Decimal(d));
    }
    if let Some(s) = &dsl_value.text {
        return NeuralValue::Atom(Atom::String(s.clone()));
    }
    match &dsl_value.dictionary {
        Some(entries) => NeuralValue::Dictionary(value_dsl_map_to_dictionary(entries)),
        None => NeuralValue::Atom(Atom::Null),
    }
}

fn dictionary_to_value_dsl_map(dict: &Dictionary) -> BTreeMap<String, ValueDsl> {
    dict.keys().map(|key| (key.clone(), value_to_value_dsl(dict.get(key).expect("key came from dict.keys()")))).collect()
}

fn value_dsl_map_to_dictionary(entries: &BTreeMap<String, ValueDsl>) -> Dictionary {
    entries.iter().fold(Dictionary::new(), |dict, (key, value)| dict.insert(key.clone(), value_dsl_to_value(value)))
}

/// 📦 `None` when `dict` is empty, mirroring `imperative_core`'s identical printer convention —
/// omits an empty dictionary section rather than printing empty braces.
fn dictionary_to_option_dsl_map(dict: &Dictionary) -> Option<BTreeMap<String, ValueDsl>> {
    (!dict.is_empty()).then(|| dictionary_to_value_dsl_map(dict))
}

fn option_dsl_map_to_dictionary(entries: Option<BTreeMap<String, ValueDsl>>) -> Dictionary {
    entries.map(|entries| value_dsl_map_to_dictionary(&entries)).unwrap_or_default()
}

/// 🔢 `BTreeSet<String>` has no blanket `dsl::DslField` impl (only `Vec`/`BTreeMap`/arrays do) — a
/// sorted `Vec<String>` is a lossless, order-independent stand-in at the DSL-text boundary since the
/// real field is reconstructed as a set on the way back in.
fn btree_set_to_vec(set: &BTreeSet<String>) -> Vec<String> {
    set.iter().cloned().collect()
}

fn vec_to_btree_set(items: Vec<String>) -> BTreeSet<String> {
    items.into_iter().collect()
}

/// 🌳 Local twin of `neural::Tree` — mutually recursive with `NeuronNodeDsl` exactly like
/// `imperative_core::PathDsl`/`StepNodeDsl`, so `neurons` goes through `NeuronNodeDsl`'s
/// `dsl::DslVariants` lazy `fn() -> RecordSpec` pointer instead of `TreeDsl` and `NeuronNodeDsl`
/// eagerly recursing into each other just to construct the schema.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct TreeDsl {
    #[dsl(statements, block)]
    neurons: Vec<NeuronNodeDsl>,
    #[dsl(table)]
    synapses: Vec<SynapseDsl>,
}

/// 🔵 Local twin of `neural::Neuron` — a one-variant `dsl::DslEnum` (not a plain `DslRecord`) purely
/// for the mutual-recursion reason documented on `TreeDsl`.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum NeuronNodeDsl {
    Neuron {
        id: String,
        kind: String,
        params: Option<BTreeMap<String, ValueDsl>>,
        #[dsl(block)]
        tree: Option<TreeDsl>,
    },
}

/// 🔌 DSL-only mirror of `SynapseSpec` (and of `neural::Synapse`, its foreign twin embedded in
/// `Tree`) — models the `from`/`fromPort` -> `to`/`toPort` connection as a single unified
/// `dsl::Wire` literal (`from@fromPort->to@toPort`) instead of four separate string fields, per
/// the unified syntax law for graph edges/connections. Converts at the `store::DocumentDsl`/
/// `store::OpText` boundary only (`flow_fixture_to_dsl`/`flow_operation_to_dsl` and their inverses,
/// plus `tree_to_tree_dsl`/`tree_dsl_to_tree` for the nested neural-tree case); `SynapseSpec`
/// itself (JSON shape, `tree_from_fixture`, `flow_fixture_operations`, every other consumer
/// matching on its `from`/`to`/`from_port`/`to_port` fields) is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct SynapseDsl {
    id: String,
    link: dsl::Wire,
}

fn synapse_to_dsl(synapse: &SynapseSpec) -> SynapseDsl {
    let from = dsl::WireNode { id: synapse.from.clone(), kind: None, port: (!synapse.from_port.is_empty()).then(|| synapse.from_port.clone()) };
    let to = dsl::WireNode { id: synapse.to.clone(), kind: None, port: (!synapse.to_port.is_empty()).then(|| synapse.to_port.clone()) };
    SynapseDsl { id: synapse.id.clone(), link: dsl::Wire(dsl::WireValue { from, edge: Some((true, to)), properties: dsl::DslValue::Object(Vec::new()) }) }
}

fn synapse_from_dsl(synapse: SynapseDsl) -> Result<SynapseSpec, String> {
    let dsl::WireValue { from, edge, .. } = synapse.link.0;
    let (directed, to) = edge.ok_or_else(|| "synapse wire literal must have a target".to_string())?;
    if !directed {
        return Err("synapse wire literal must be directed".into());
    }
    Ok(SynapseSpec { id: synapse.id, from: from.id, to: to.id, from_port: from.port.unwrap_or_default(), to_port: to.port.unwrap_or_default() })
}

fn tree_to_tree_dsl(tree: &Tree) -> TreeDsl {
    TreeDsl {
        neurons: tree.neurons.iter().map(neuron_to_neuron_node_dsl).collect(),
        synapses: tree.synapses.iter().map(|synapse| synapse_to_dsl(&SynapseSpec { id: synapse.id.clone(), from: synapse.from.clone(), to: synapse.to.clone(), from_port: synapse.from_port.clone(), to_port: synapse.to_port.clone() })).collect(),
    }
}

fn tree_dsl_to_tree(tree: TreeDsl) -> Result<Tree, String> {
    Ok(Tree {
        neurons: tree.neurons.into_iter().map(neuron_node_dsl_to_neuron).collect::<Result<Vec<_>, _>>()?,
        synapses: tree.synapses.into_iter().map(|dsl_synapse| synapse_from_dsl(dsl_synapse).map(|spec| Synapse { id: spec.id, from: spec.from, to: spec.to, from_port: spec.from_port, to_port: spec.to_port })).collect::<Result<Vec<_>, _>>()?,
    })
}

fn neuron_to_neuron_node_dsl(neuron: &Neuron) -> NeuronNodeDsl {
    NeuronNodeDsl::Neuron { id: neuron.id.clone(), kind: neuron.kind.clone(), params: dictionary_to_option_dsl_map(&neuron.params), tree: neuron.tree.as_deref().map(tree_to_tree_dsl) }
}

fn neuron_node_dsl_to_neuron(node: NeuronNodeDsl) -> Result<Neuron, String> {
    let NeuronNodeDsl::Neuron { id, kind, params, tree } = node;
    let tree = match tree {
        Some(tree) => Some(Box::new(tree_dsl_to_tree(tree)?)),
        None => None,
    };
    Ok(Neuron { id, kind, params: option_dsl_map_to_dictionary(params), tree })
}

/// 🎛️ Local twin of `Widget` — a tagged `dsl::DslEnum` mirroring its serde `kind` tags one-for-one.
/// `Cluster`'s `flow: FlowGui` is deliberately printed via the engine's `serde_json::Value` escape
/// hatch (untyped but byte-for-byte round-tripping JSON), not its own nested DSL grammar: `FlowGui`/
/// `FlowNodeGui`/`NodeChrome`/`FlowPreviewGui` are GUI-only view state (see each type's own doc
/// comment) that never feeds neural evaluation — `tree_from_fixture`'s `Cluster` handling reads only
/// `tree`, never `flow` — the same "derived read-view, not a DSL-typed field" reasoning `FlowDocument`
/// itself gets relative to `FlowFixture`, just one level further in.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum WidgetDsl {
    Neuron {
        id: String,
        neuron_kind: String,
        params: Option<BTreeMap<String, ValueDsl>>,
        input_ports: Vec<String>,
        output_ports: Vec<String>,
        preview: bool,
    },
    InputSlider { id: String, value: f64, min: f64, max: f64, step: f64 },
    InputNote { id: String, text: String },
    InputImage { id: String, src: String },
    Variable { id: String, name: String, schema: String },
    OutputPreview {
        id: String,
        preview: Option<BTreeMap<String, ValueDsl>>,
        expanded: Vec<String>,
    },
    OutputAction { id: String, action: String },
    OutputExport { id: String, format: String },
    Cluster {
        id: String,
        name: String,
        #[dsl(block)]
        tree: TreeDsl,
        flow: serde_json::Value,
    },
}

/// 🌉 `#[derive(dsl::DslEnum)]` only gives `WidgetDsl` a `dsl::DslVariants` binding, not
/// `dsl::DslField` — so it can't sit directly in a plain (non-`Vec`) field on its own.
/// `FlowOperationDsl`'s `WidgetsAdd.item`/`WidgetsPatch.patch` are REQUIRED, never-collection single
/// values; this hand impl reuses the exact same "exactly one tagged statement" idiom
/// `process_3d::SolidSpec` uses for the identical shape, so those fields stay a bare `WidgetDsl`
/// rather than a `Box<WidgetDsl>`.
impl dsl::DslField for WidgetDsl {
    fn shape() -> dsl::Shape {
        dsl::Shape::Statements(<WidgetDsl as dsl::DslVariants>::variants())
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Statements(vec![<WidgetDsl as dsl::DslVariants>::to_named_record(self)])
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Statements(items) if items.len() == 1 => <WidgetDsl as dsl::DslVariants>::from_named_record(&items[0].0, &items[0].1).map_err(|e| e.message),
            other => Err(format!("expected exactly 1 tagged widget value, found {other:?}")),
        }
    }
}

fn widget_to_widget_dsl(widget: &Widget) -> WidgetDsl {
    match widget {
        Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => {
            WidgetDsl::Neuron { id: id.clone(), neuron_kind: neuron_kind.clone(), params: dictionary_to_option_dsl_map(params), input_ports: input_ports.clone(), output_ports: output_ports.clone(), preview: *preview }
        }
        Widget::InputSlider { id, value, min, max, step } => WidgetDsl::InputSlider { id: id.clone(), value: *value, min: *min, max: *max, step: *step },
        Widget::InputNote { id, text } => WidgetDsl::InputNote { id: id.clone(), text: text.clone() },
        Widget::InputImage { id, src } => WidgetDsl::InputImage { id: id.clone(), src: src.clone() },
        Widget::Variable { id, name, schema } => WidgetDsl::Variable { id: id.clone(), name: name.clone(), schema: schema.clone() },
        Widget::OutputPreview { id, preview, expanded } => WidgetDsl::OutputPreview { id: id.clone(), preview: dictionary_to_option_dsl_map(preview), expanded: btree_set_to_vec(expanded) },
        Widget::OutputAction { id, action } => WidgetDsl::OutputAction { id: id.clone(), action: action.clone() },
        Widget::OutputExport { id, format } => WidgetDsl::OutputExport { id: id.clone(), format: format.clone() },
        Widget::Cluster { id, name, tree, flow } => WidgetDsl::Cluster { id: id.clone(), name: name.clone(), tree: tree_to_tree_dsl(tree), flow: serde_json::to_value(flow).unwrap_or(serde_json::Value::Null) },
    }
}

fn widget_dsl_to_widget(widget: WidgetDsl) -> Result<Widget, String> {
    Ok(match widget {
        WidgetDsl::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => {
            Widget::Neuron { id, neuron_kind, params: option_dsl_map_to_dictionary(params), input_ports, output_ports, preview }
        }
        WidgetDsl::InputSlider { id, value, min, max, step } => Widget::InputSlider { id, value, min, max, step },
        WidgetDsl::InputNote { id, text } => Widget::InputNote { id, text },
        WidgetDsl::InputImage { id, src } => Widget::InputImage { id, src },
        WidgetDsl::Variable { id, name, schema } => Widget::Variable { id, name, schema },
        WidgetDsl::OutputPreview { id, preview, expanded } => Widget::OutputPreview { id, preview: option_dsl_map_to_dictionary(preview), expanded: vec_to_btree_set(expanded) },
        WidgetDsl::OutputAction { id, action } => Widget::OutputAction { id, action },
        WidgetDsl::OutputExport { id, format } => Widget::OutputExport { id, format },
        WidgetDsl::Cluster { id, name, tree, flow } => Widget::Cluster { id, name, tree: tree_dsl_to_tree(tree)?, flow: serde_json::from_value(flow).unwrap_or_default() },
    })
}

/// 📄 Local mirror of `FlowFixture` — see this region's opening doc comment for why `widgets:
/// Vec<Widget>` (which embeds foreign `Dictionary`/`Tree` types) can't stay as-is under a direct
/// `#[derive(dsl::DslDocument)]`. `FlowDocument` (the derived read-view built by
/// `FlowFixture::to_document()`) deliberately does NOT get this treatment — it's a computed
/// projection for rendering, never itself round-tripped through DSL text.
#[derive(Clone, Debug, PartialEq, dsl::DslDocument)]
#[dsl(extension = "flow")]
#[dsl(layout = "lines")]
struct FlowFixtureDsl {
    schema: String,
    #[dsl(block)]
    camera: CameraJson,
    #[dsl(statements, block)]
    widgets: Vec<WidgetDsl>,
    #[dsl(table)]
    synapses: Vec<SynapseDsl>,
    layout: BTreeMap<String, WidgetLayout>,
}

fn flow_fixture_to_dsl(fixture: &FlowFixture) -> FlowFixtureDsl {
    FlowFixtureDsl { schema: fixture.schema.clone(), camera: fixture.camera.clone(), widgets: fixture.widgets.iter().map(widget_to_widget_dsl).collect(), synapses: fixture.synapses.iter().map(synapse_to_dsl).collect(), layout: fixture.layout.clone() }
}

fn flow_fixture_dsl_to_fixture(fixture: FlowFixtureDsl) -> Result<FlowFixture, String> {
    Ok(FlowFixture {
        schema: fixture.schema,
        camera: fixture.camera,
        widgets: fixture.widgets.into_iter().map(widget_dsl_to_widget).collect::<Result<Vec<_>, _>>()?,
        synapses: fixture.synapses.into_iter().map(synapse_from_dsl).collect::<Result<Vec<_>, _>>()?,
        layout: fixture.layout,
    })
}

impl store::DocumentDsl for FlowFixture {
    const EXTENSION: &'static str = "flow";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let dsl_fixture = <FlowFixtureDsl as store::DocumentDsl>::parse_dsl(text)?;
        flow_fixture_dsl_to_fixture(dsl_fixture).map_err(|message| store::TextError::new(message, store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        <FlowFixtureDsl as store::DocumentDsl>::print_dsl(&flow_fixture_to_dsl(self))
    }
}

/// 🗜️ `FlowFixture` has no `#[derive(dsl::DslDocument)]` of its own (see `FlowFixtureDsl`'s doc
/// comment above), so it doesn't automatically gain `store::DocumentPack` the way every derived type
/// does — this hand-written twin of the `store::DocumentDsl` impl just above delegates through the
/// same `flow_fixture_to_dsl`/`flow_fixture_dsl_to_fixture` mirror instead of `__dsl_to_record`/
/// `__dsl_from_record`.
impl store::DocumentPack for FlowFixture {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <FlowFixtureDsl as store::DocumentPack>::encode_pack_with(&flow_fixture_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let dsl_fixture = <FlowFixtureDsl as store::DocumentPack>::decode_pack_with(bytes, options)?;
        flow_fixture_dsl_to_fixture(dsl_fixture).map_err(|message| store::text_error_to_pack_error(store::TextError::new(message, store::TextSpan::at(1, 1))))
    }
}
//#endregion 🔖Dsl

//#region 🔖OpText
/// ✂️ Local DSL-only mirror of `FlowOperation` — `protocol::CollectionOperation<K,V,P>` is declared
/// in the `protocol` crate (foreign type), so it cannot itself gain a `dsl::DslField`/
/// `dsl::DslVariants` binding here (orphan rule). This twin flattens the `Widgets`/
/// `Synapses { collection }` wrappers into their own keyworded variants — mirroring
/// `imperative_core::ImperativeOperationDsl`'s/`process_3d::Process3dOperationDsl`'s identical fix
/// for the same foreign-`CollectionOperation` problem — and converts at the `protocol::OpText`
/// boundary only; `FlowOperation` itself, and every consumer matching on it
/// (`flow_fixture_operations`, `flow/plugin`), is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum FlowOperationDsl {
    WidgetsAdd {
        index: usize,
        #[dsl(block)]
        item: WidgetDsl,
    },
    WidgetsRemove { id: String },
    WidgetsMove {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    WidgetsPatch {
        id: String,
        #[dsl(block)]
        patch: WidgetDsl,
    },
    SynapsesAdd {
        index: usize,
        #[dsl(block)]
        item: SynapseDsl,
    },
    SynapsesRemove { id: String },
    SynapsesMove {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    SynapsesPatch {
        id: String,
        #[dsl(block)]
        patch: SynapseDsl,
    },
    #[dsl(key = "layout")]
    SetLayout { entries: Vec<FlowLayoutEntry> },
    #[dsl(key = "fixture")]
    SetFixture {
        #[dsl(block)]
        fixture: FlowFixtureDsl,
    },
}

fn flow_operation_to_dsl(operation: &FlowOperation) -> FlowOperationDsl {
    match operation {
        FlowOperation::Widgets(CollectionOperation::Add { id: _id, item, at }) => FlowOperationDsl::WidgetsAdd { index: *at, item: widget_to_widget_dsl(item) },
        FlowOperation::Widgets(CollectionOperation::Remove { id }) => FlowOperationDsl::WidgetsRemove { id: id.clone() },
        FlowOperation::Widgets(CollectionOperation::Move { id, to }) => FlowOperationDsl::WidgetsMove { id: id.clone(), to_index: *to },
        FlowOperation::Widgets(CollectionOperation::Patch { id, patch }) => FlowOperationDsl::WidgetsPatch { id: id.clone(), patch: widget_to_widget_dsl(patch) },
        FlowOperation::Synapses(CollectionOperation::Add { id: _id, item, at }) => FlowOperationDsl::SynapsesAdd { index: *at, item: synapse_to_dsl(item) },
        FlowOperation::Synapses(CollectionOperation::Remove { id }) => FlowOperationDsl::SynapsesRemove { id: id.clone() },
        FlowOperation::Synapses(CollectionOperation::Move { id, to }) => FlowOperationDsl::SynapsesMove { id: id.clone(), to_index: *to },
        FlowOperation::Synapses(CollectionOperation::Patch { id, patch }) => FlowOperationDsl::SynapsesPatch { id: id.clone(), patch: synapse_to_dsl(patch) },
        FlowOperation::SetLayout { entries } => FlowOperationDsl::SetLayout { entries: entries.clone() },
        FlowOperation::SetFixture { fixture } => FlowOperationDsl::SetFixture { fixture: flow_fixture_to_dsl(fixture) },
    }
}

fn flow_operation_from_dsl(operation: FlowOperationDsl) -> Result<FlowOperation, String> {
    Ok(match operation {
        FlowOperationDsl::WidgetsAdd { index, item } => {
            let item = widget_dsl_to_widget(item)?;
            FlowOperation::Widgets(CollectionOperation::Add { id: widget_id_for(&item).to_string(), item, at: index })
        }
        FlowOperationDsl::WidgetsRemove { id } => FlowOperation::Widgets(CollectionOperation::Remove { id }),
        FlowOperationDsl::WidgetsMove { id, to_index } => FlowOperation::Widgets(CollectionOperation::Move { id, to: to_index }),
        FlowOperationDsl::WidgetsPatch { id, patch } => FlowOperation::Widgets(CollectionOperation::Patch { id, patch: widget_dsl_to_widget(patch)? }),
        FlowOperationDsl::SynapsesAdd { index, item } => {
            let item = synapse_from_dsl(item)?;
            FlowOperation::Synapses(CollectionOperation::Add { id: item.id.clone(), item, at: index })
        }
        FlowOperationDsl::SynapsesRemove { id } => FlowOperation::Synapses(CollectionOperation::Remove { id }),
        FlowOperationDsl::SynapsesMove { id, to_index } => FlowOperation::Synapses(CollectionOperation::Move { id, to: to_index }),
        FlowOperationDsl::SynapsesPatch { id, patch } => FlowOperation::Synapses(CollectionOperation::Patch { id, patch: synapse_from_dsl(patch)? }),
        FlowOperationDsl::SetLayout { entries } => FlowOperation::SetLayout { entries },
        FlowOperationDsl::SetFixture { fixture } => FlowOperation::SetFixture { fixture: flow_fixture_dsl_to_fixture(fixture)? },
    })
}

impl protocol::OpText for FlowOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let dsl_operation = <FlowOperationDsl as protocol::OpText>::parse_op(line)?;
        flow_operation_from_dsl(dsl_operation).map_err(|message| store::TextError::new(message, store::TextSpan::at(1, 1)))
    }

    fn print_op(&self) -> String {
        <FlowOperationDsl as protocol::OpText>::print_op(&flow_operation_to_dsl(self))
    }
}

/// ⚡ Binary mirror of the `OpText` impl above — `FlowOperationDsl` already derives `OpBinary`
/// via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for FlowOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        flow_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let dsl_operation = FlowOperationDsl::decode_op(bytes)?;
        flow_operation_from_dsl(dsl_operation).map_err(|message| protocol::ProtocolError::Malformed { what: "flow operation", offset: 0, detail: message })
    }
}
//#endregion 🔖OpText

pub type FlowEnvelope = DocumentEnvelope<FlowFixture, FlowOperation>;
pub type FlowStore = DocumentStore<FlowFixture, FlowOperation>;

pub fn empty_flow_projection() -> FlowFixture {
    FlowFixture::default()
}

#[cfg(target_arch = "wasm32")]
mod flow_vcs_wasm {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct FlowDocumentVcs {
        store: RefCell<FlowStore>,
    }

    #[wasm_bindgen]
    impl FlowDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<FlowDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: FlowEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    FlowStore::new(envelope)
                }
                None => FlowStore::new(create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow", empty_flow_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}

// #region 🔖FormsBridge
pub mod forms_bridge {
    use super::{FlowFixture, Widget};
    use playbook::{PlaybookBlock, PlaybookBlockOption, PlaybookSpec, PlaybookStep, PLAYBOOK_DOCUMENT_SCHEMA};

    fn humanize_widget_label(id: &str) -> String {
        let mut words = Vec::new();
        let mut current = String::new();
        for ch in id.chars() {
            if ch == '_' || ch == '-' || ch == ' ' {
                if !current.is_empty() {
                    words.push(current.clone());
                    current.clear();
                }
                continue;
            }
            if ch.is_uppercase() && !current.is_empty() {
                words.push(current.clone());
                current.clear();
            }
            current.push(if ch.is_uppercase() { ch } else { ch.to_ascii_uppercase() });
        }
        if !current.is_empty() {
            words.push(current);
        }
        if words.is_empty() {
            return id.to_string();
        }
        words.join(" ")
    }

    /// 🔀 Schema aliases treated as a single-choice question when generating a playbook block (anything else is free text).
    enum SchemaQuestionFamily {
        Choice,
    }

    impl SchemaQuestionFamily {
        fn parse(schema: &str) -> Option<Self> {
            match schema.trim().to_ascii_lowercase().as_str() {
                "enum" | "single" | "select" | "choice" => Some(Self::Choice),
                _ => None,
            }
        }
    }

    fn variable_question_kind(schema: &str) -> &'static str {
        match SchemaQuestionFamily::parse(schema) {
            Some(SchemaQuestionFamily::Choice) => "single",
            None => "text",
        }
    }

    fn widget_to_playbook_block(widget: &Widget) -> Option<PlaybookBlock> {
        match widget {
            Widget::InputSlider { id, value, min, max, step, .. } => Some(PlaybookBlock {
                id: id.clone(),
                label: humanize_widget_label(id),
                kind: "slider".into(),
                description: None,
                required: None,
                placeholder: None,
                default: Some(serde_json::json!(*value)),
                min: Some(*min),
                max: Some(*max),
                step: Some(*step),
                unit: None,
                text: None,
                options: None,
                fields: None,
                schema: None,
                src: None,
                accept: None,
                fixture_slug: None,
                params: None,
                condition: None,
            }),
            Widget::InputNote { id, text, .. } => Some(PlaybookBlock {
                id: id.clone(),
                label: humanize_widget_label(id),
                kind: "note".into(),
                description: None,
                required: None,
                placeholder: None,
                default: None,
                min: None,
                max: None,
                step: None,
                unit: None,
                text: Some(text.clone()),
                options: None,
                fields: None,
                schema: None,
                src: None,
                accept: None,
                fixture_slug: None,
                params: None,
                condition: None,
            }),
            Widget::InputImage { id, src, .. } => Some(PlaybookBlock {
                id: id.clone(),
                label: humanize_widget_label(id),
                kind: "image".into(),
                description: None,
                required: None,
                placeholder: None,
                default: None,
                min: None,
                max: None,
                step: None,
                unit: None,
                text: None,
                options: None,
                fields: None,
                schema: None,
                src: Some(src.clone()),
                accept: None,
                fixture_slug: None,
                params: None,
                condition: None,
            }),
            Widget::Variable { id, name, schema, .. } => {
                let kind = variable_question_kind(schema);
                let options = if kind == "single" { Some(vec![PlaybookBlockOption { value: schema.clone(), label: humanize_widget_label(schema) }]) } else { None };
                Some(PlaybookBlock {
                    id: id.clone(),
                    label: humanize_widget_label(name),
                    kind: kind.into(),
                    description: None,
                    required: None,
                    placeholder: None,
                    default: Some(serde_json::Value::String(name.clone())),
                    min: None,
                    max: None,
                    step: None,
                    unit: None,
                    text: None,
                    options,
                    fields: None,
                    schema: Some(schema.clone()),
                    src: None,
                    accept: None,
                    fixture_slug: None,
                    params: None,
                    condition: None,
                })
            }
            _ => None,
        }
    }

    pub fn flow_fixture_to_form_spec(fixture: &FlowFixture) -> PlaybookSpec {
        let blocks: Vec<PlaybookBlock> = fixture.widgets.iter().filter_map(widget_to_playbook_block).collect();
        PlaybookSpec { schema: PLAYBOOK_DOCUMENT_SCHEMA.into(), id: "flow-generate".into(), version: "1".into(), title: Some("Generate".into()), steps: vec![PlaybookStep { id: "inputs".into(), title: "Inputs".into(), description: None, blocks }] }
    }

    /// 🏷️ Widget "kind" tags recognized when patching a single generation value into a raw fixture-JSON widget.
    enum WidgetPatchKind {
        InputSlider,
        InputNote,
        InputImage,
        Variable,
    }

    impl WidgetPatchKind {
        fn parse(kind: &str) -> Option<Self> {
            match kind {
                "inputSlider" => Some(Self::InputSlider),
                "inputNote" => Some(Self::InputNote),
                "inputImage" => Some(Self::InputImage),
                "variable" => Some(Self::Variable),
                _ => None,
            }
        }
    }

    pub fn apply_generation_values_to_fixture(fixture_json: &str, values: &serde_json::Map<String, serde_json::Value>) -> String {
        let mut root: serde_json::Value = serde_json::from_str(fixture_json).unwrap_or(serde_json::json!({}));
        let Some(widgets) = root.get_mut("widgets").and_then(|entry| entry.as_array_mut()) else {
            return fixture_json.to_string();
        };
        for widget in widgets.iter_mut() {
            let Some(id) = widget.get("id").and_then(|entry| entry.as_str()) else {
                continue;
            };
            let Some(value) = values.get(id) else {
                continue;
            };
            let kind = widget.get("kind").and_then(|entry| entry.as_str()).unwrap_or_default();
            match WidgetPatchKind::parse(kind) {
                Some(WidgetPatchKind::InputSlider) => {
                    if let Some(number) = value.as_f64() {
                        widget["value"] = serde_json::json!(number);
                    }
                }
                Some(WidgetPatchKind::InputNote) => {
                    if let Some(text) = value.as_str() {
                        widget["text"] = serde_json::json!(text);
                    }
                }
                Some(WidgetPatchKind::InputImage) => {
                    if let Some(src) = value.as_str() {
                        widget["src"] = serde_json::json!(src);
                    }
                }
                Some(WidgetPatchKind::Variable) => {
                    if let Some(text) = value.as_str() {
                        widget["name"] = serde_json::json!(text);
                    }
                }
                None => {}
            }
        }
        serde_json::to_string(&root).unwrap_or_else(|_| fixture_json.to_string())
    }
}
// #endregion 🔖FormsBridge

#[cfg(test)]
mod flow_vcs_tests {
    use super::*;

    fn sample_widget(id: &str) -> Widget {
        Widget::InputNote { id: id.into(), text: format!("note {id}") }
    }

    fn round_trip(fixture: &FlowFixture, operation: &FlowOperation) -> FlowFixture {
        let forward = vcs::apply_operation(fixture, operation);
        let backwards = operation.backwards(fixture);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = vcs::apply_operation(&restored, back);
        }
        assert_eq!(&restored, fixture, "backwards() must exactly restore the pre-operation fixture");
        forward
    }

    #[test]
    fn widget_add_patch_remove_round_trip() {
        let fixture = FlowFixture { widgets: Vec::new(), synapses: Vec::new(), ..FlowFixture::default() };
        let add = FlowOperation::Widgets(CollectionOperation::Add { id: "w1".into(), item: sample_widget("w1"), at: 0 });
        let with_widget = round_trip(&fixture, &add);
        assert_eq!(with_widget.widgets.len(), 1);

        let patch = FlowOperation::Widgets(CollectionOperation::Patch { id: "w1".into(), patch: Widget::InputNote { id: "w1".into(), text: "renamed".into() } });
        let patched = round_trip(&with_widget, &patch);
        assert!(matches!(&patched.widgets[0], Widget::InputNote { text, .. } if text == "renamed"));

        let remove = FlowOperation::Widgets(CollectionOperation::Remove { id: "w1".into() });
        let removed = round_trip(&patched, &remove);
        assert!(removed.widgets.is_empty());
    }

    #[test]
    fn set_layout_round_trip() {
        let fixture = FlowFixture::default();
        let operation = FlowOperation::SetLayout { entries: vec![FlowLayoutEntry { id: "slider".into(), layout: Some(WidgetLayout { x: 12.0, y: 34.0 }) }] };
        let next = round_trip(&fixture, &operation);
        assert_eq!(next.layout.get("slider"), Some(&WidgetLayout { x: 12.0, y: 34.0 }));
    }

    #[test]
    fn flow_fixture_ops_diffs_widgets_synapses_layout() {
        let before = FlowFixture { widgets: vec![sample_widget("a"), sample_widget("b")], synapses: Vec::new(), ..FlowFixture::default() };
        let mut after = before.clone();
        after.widgets.retain(|widget| Identified::id(widget) != "a");
        after.widgets.push(sample_widget("c"));
        after.layout.insert("c".into(), WidgetLayout { x: 1.0, y: 2.0 });
        let operations = flow_fixture_operations(&before, &after);
        let materialized = operations.iter().fold(before.clone(), |acc, operation| vcs::apply_operation(&acc, operation));
        assert_eq!(materialized.widgets.len(), 2);
        assert!(materialized.widgets.iter().any(|widget| Identified::id(widget) == "c"));
        assert!(materialized.widgets.iter().all(|widget| Identified::id(widget) != "a"));
        assert_eq!(materialized.layout.get("c"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
    }

    #[test]
    fn coalesced_layout_drag_produces_one_edit() {
        let mut store = FlowStore::new(create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow", empty_flow_projection(), None));
        for y in [10.0, 20.0, 30.0] {
            store
                .dispatch(DocumentCommand::AmendLast { operations: vec![FlowOperation::SetLayout { entries: vec![FlowLayoutEntry { id: "slider".into(), layout: Some(WidgetLayout { x: 0.0, y }) }] }], coalesce_key: Some("move-slider".into()) })
                .expect("drag tick");
        }
        assert_eq!(store.envelope().vcs.edits.len(), 1, "coalesced drag must produce exactly one edit");
        assert_eq!(store.projection().expect("projection").layout.get("slider"), Some(&WidgetLayout { x: 0.0, y: 30.0 }));
    }

    /// 📜 Exercises every `Widget` variant (including `Cluster`'s nested `Tree`/`flow` payload,
    /// `Dictionary`-bearing `params`/`preview`, and `BTreeSet` `expanded`) through the `dsl::` derive
    /// layer — the ground-truth proof for the `🔖Dsl` region built on top of `FlowFixture`.
    #[test]
    fn flow_fixture_dsl_round_trips_including_cluster_widget() {
        let mut fixture = FlowFixture::default();
        fixture.widgets.push(Widget::Cluster {
            id: "cluster-1".into(),
            name: "Cluster One".into(),
            tree: Tree {
                neurons: vec![
                    Neuron { id: "inner-in".into(), kind: "core.number".into(), params: Dictionary::new().insert("value", NeuralValue::Atom(Atom::Decimal(1.0))), tree: None },
                    Neuron {
                        id: "inner-add".into(),
                        kind: "math.add".into(),
                        params: Dictionary::new().insert("count", NeuralValue::Atom(Atom::Integer(2))),
                        tree: Some(Box::new(Tree { neurons: vec![Neuron::with_kind("nested", "core.text", Dictionary::new().insert("value", NeuralValue::Atom(Atom::String("deep".into()))))], synapses: vec![] })),
                    },
                ],
                synapses: vec![Synapse { id: "inner-s1".into(), from: "inner-in".into(), to: "inner-add".into(), from_port: "number".into(), to_port: "a".into() }],
            },
            flow: FlowGui { camera: CameraJson { x: 1.0, y: 2.0, zoom: 1.5 }, nodes: BTreeMap::new(), previews: Vec::new() },
        });
        fixture.widgets.push(Widget::OutputPreview {
            id: "preview2".into(),
            preview: Dictionary::new().insert("value", NeuralValue::Atom(Atom::Decimal(3.5))),
            expanded: BTreeSet::from(["a".to_string(), "b".to_string()]),
        });
        store::test_support::assert_dsl_round_trip(&fixture);
        store::test_support::assert_dsl_pack_equivalence(&fixture);
    }

    /// 📜 Exercises `store::OpText` for every `FlowOperation` variant — the ground-truth proof for the
    /// `🔖OpText` region's `FlowOperationDsl` twin.
    #[test]
    fn flow_operation_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&FlowOperation::Widgets(CollectionOperation::Add { id: "w1".into(), item: sample_widget("w1"), at: 0 }));
        store::test_support::assert_op_line_round_trip(&FlowOperation::Widgets(CollectionOperation::Remove { id: "w1".into() }));
        store::test_support::assert_op_line_round_trip(&FlowOperation::Widgets(CollectionOperation::Move { id: "w1".into(), to: 2 }));
        store::test_support::assert_op_line_round_trip(&FlowOperation::Widgets(CollectionOperation::Patch { id: "w1".into(), patch: sample_widget("w1") }));
        let synapse = SynapseSpec { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "y".into() };
        store::test_support::assert_op_line_round_trip(&FlowOperation::Synapses(CollectionOperation::Add { id: "s1".into(), item: synapse.clone(), at: 0 }));
        store::test_support::assert_op_line_round_trip(&FlowOperation::Synapses(CollectionOperation::Remove { id: "s1".into() }));
        store::test_support::assert_op_line_round_trip(&FlowOperation::Synapses(CollectionOperation::Move { id: "s1".into(), to: 1 }));
        store::test_support::assert_op_line_round_trip(&FlowOperation::Synapses(CollectionOperation::Patch { id: "s1".into(), patch: synapse }));
        store::test_support::assert_op_line_round_trip(&FlowOperation::SetLayout { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: Some(WidgetLayout { x: 1.0, y: 2.0 }) }] });
        store::test_support::assert_op_line_round_trip(&FlowOperation::SetLayout { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: None }] });
        store::test_support::assert_op_line_round_trip(&FlowOperation::SetFixture { fixture: FlowFixture::default() });
    }

    /// 📜 `store::test_support::assert_store_roundtrip` over a real `DocumentStore<FlowFixture,
    /// FlowOperation>` — proves the `Operation`/`OperationDiff` (`🔖Operations`) and `OpText`
    /// (`🔖OpText`) layers compose correctly end to end, matching every other converted crate's test.
    #[test]
    fn flow_fixture_satisfies_vcs_test_support_store_roundtrip() {
        let document = FlowFixture::default();
        let operation = FlowOperation::Widgets(CollectionOperation::Add { id: "w1".into(), item: sample_widget("w1"), at: 0 });
        store::test_support::assert_store_roundtrip(document, operation);
    }

    /// 📜 `flow/example/default.flow` is the handcrafted `.flow` DSL-text migration of what used to
    /// be `default.flow.json` (see this crate's ticket history) — this is the permanent proof that
    /// the checked-in fixture still parses and round trips, not a one-time migration script.
    #[test]
    fn default_flow_example_dsl_round_trips() {
        let text = include_str!("../../example/default.flow");
        let fixture = <FlowFixture as store::DocumentDsl>::parse_dsl(text).expect("default.flow must parse");
        store::test_support::assert_dsl_round_trip(&fixture);
        store::test_support::assert_dsl_pack_equivalence(&fixture);
    }
}
// #endregion 🔖DocumentVcs

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use cavas::camera::{world_to_screen, Camera, Viewport};
    use cavas::Point;
    use dag::HandleRole;
    use neural::{ChannelSpec as InputSpec, OperatorInfo as NeuronKindInfo, Registry};
    use std::sync::{Mutex, OnceLock};

    const NUMBER_OPS: &[&str] = &["core.number"];
    static RECTANGLE_EXTRUDE_FIXTURE_TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn test_math_bridge(kind: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        if kind == "core.number" {
            let value = input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
            return Ok(channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(value)))));
        }
        if kind == "core.text" {
            let value = input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap_or_default();
            return Ok(channel_output("text", Dictionary::with_schema("text").insert("value", NeuralValue::Atom(Atom::String(value.into())))));
        }
        if kind == "core.image" {
            let value = input.get("dataUrl").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap_or_default();
            return Ok(channel_output("image", Dictionary::with_schema("image").insert("dataUrl", NeuralValue::Atom(Atom::String(value.into())))));
        }
        if kind == "math.add" {
            let a = input.get("a").or_else(|| input.get("number")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("a".into()))?;
            let b = input.get("b").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
            return Ok(channel_output("sum", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(a + b)))));
        }
        if kind == "math.passThrough" {
            let n = input.get("number").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("number".into()))?;
            return Ok(channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(n)))));
        }
        if kind == "core.variable" {
            let name = input.get("name").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).ok_or_else(|| EvalError::MissingInput("name".into()))?;
            let payload = input.get(name).and_then(|v| v.as_dictionary()).cloned().ok_or_else(|| EvalError::MissingInput(name.into()))?;
            return Ok(channel_output(name, payload));
        }
        Err(EvalError::UnknownKind(kind.into()))
    }

    fn fixture_kind_infos_json() -> String {
        let mut registry = Registry::new();
        flow_module_core::register(&mut registry);
        flow_module_math::register(&mut registry);
        flow_module_brep::register(&mut registry);
        serde_json::to_string(&registry.operator_catalogue()).unwrap_or_else(|_| "[]".into())
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
                outputs: vec![InputSpec::named("S", "Sum", "sum", "Sum")],
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
                outputs: vec![InputSpec::named("N", "Num", "number", "Number")],
                ..Default::default()
            },
        ])
        .unwrap()
    }

    fn host_with_test_bridge() -> FlowHost {
        let mut host = FlowHost::default();
        host.set_eval_bridge_fn(Box::new(test_math_bridge));
        host.set_neuron_kind_infos_json(&test_kind_infos_json());
        host.set_host_catalogue_json(
            &serde_json::to_string(&[CatalogueSection {
                id: "math".into(),
                title: "Math".into(),
                groups: vec![],
                items: vec![
                    CatalogueItem { kind: "neuron".into(), neuron_kind: Some("math.add".into()), action: None, format: None, name: "Add".into(), abbreviation: "Add".into(), icon: "emoji:➕".into(), summary: "Sums two numbers".into() },
                    CatalogueItem {
                        kind: "neuron".into(),
                        neuron_kind: Some("math.passThrough".into()),
                        action: None,
                        format: None,
                        name: "PassThrough".into(),
                        abbreviation: "Pass".into(),
                        icon: "emoji:➡️".into(),
                        summary: "Forwards a number".into(),
                    },
                ],
            }])
            .unwrap(),
        );
        host.evaluate_internal();
        host
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
        assert_eq!(slider.width, add.width, "all components should share one width");
        assert_eq!(slider.width, computation_node_width(&slider.name, &[], &[]));
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
        // 🧵 Mutating a widget never auto-evaluates anymore (see `evaluate_step`'s doc comment) — an
        // off-main-thread ticker outside `flow_core` is responsible for that; this simulates one tick
        // with a direct `evaluate_internal` call.
        let mut host = host_with_test_bridge();
        host.set_slider_value("slider", 5.0);
        host.evaluate_internal();
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
    fn pending_eval_widget_ids_reports_without_computing() {
        let mut host = host_with_test_bridge();
        let before = host.preview_text();
        host.set_slider_value("slider", 9.0);
        let pending = host.pending_eval_widget_ids();
        assert!(pending.contains(&"add".to_string()), "the widget downstream of the changed slider is pending");
        assert!(!pending.contains(&"slider".to_string()), "the seed slider itself is not a pending neuron");
        assert_eq!(host.preview_text(), before, "a probe must never actually compute anything");
    }

    #[test]
    fn set_slider_value_marks_downstream_computing_chrome() {
        let mut host = host_with_test_bridge();
        host.set_slider_value("slider", 7.0);
        let pending = host.pending_eval_widget_ids();
        assert!(!pending.is_empty(), "slider change must flag downstream nodes as pending");
        host.refresh_computing_chrome_from_pending();
        let remaining = host.pending_eval_widget_ids();
        assert_eq!(remaining.first().map(String::as_str), pending.first().map(String::as_str));
    }

    #[test]
    fn apply_eval_outputs_json_establishes_baseline_for_dirty_probe() {
        let mut host = host_with_test_bridge();
        let eval_json = host.last_eval_json.clone();
        let mut fresh = FlowHost::default();
        fresh.set_eval_bridge_fn(Box::new(test_math_bridge));
        fresh.set_neuron_kind_infos_json(&test_kind_infos_json());
        fresh.apply_eval_outputs_json(&eval_json);
        fresh.set_slider_value("slider", 4.0);
        let pending = fresh.pending_eval_widget_ids();
        assert!(pending.contains(&"add".to_string()));
        assert!(!pending.contains(&"slider".to_string()));
    }

    #[test]
    fn eval_driver_round_trips_baseline_across_ephemeral_hosts() {
        let mut driver = FlowEvalDriver::default();
        let mut host = host_with_test_bridge();
        driver.capture_baseline_from(&host);
        host.set_slider_value("slider", 8.0);
        let mut replay = FlowHost::default();
        replay.set_eval_bridge_fn(Box::new(test_math_bridge));
        replay.set_neuron_kind_infos_json(&test_kind_infos_json());
        replay.replace_fixture(host.fixture.clone());
        driver.install_baseline_into(&mut replay);
        let pending = replay.pending_eval_widget_ids();
        assert!(pending.contains(&"add".to_string()));
        assert!(!pending.contains(&"slider".to_string()));
    }

    /// 🧵 Builds a two-computable-node chain (`add` -> `pass`, replacing `add`'s direct link to
    /// `preview`) on top of the default fixture, for tests that need more than one node to step
    /// through with a budgeted `evaluate_step`.
    fn host_with_two_node_chain() -> (FlowHost, String) {
        let mut host = host_with_test_bridge();
        let pass_id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough","params":{},"input_ports":[],"preview":false}"#, 240.0, 0.0).unwrap();
        host.connect_ports("add", "sum", &pass_id, "number").unwrap();
        host.connect_ports(&pass_id, "number", "preview", "").unwrap();
        let stale_link = host.fixture.synapses.iter().find(|s| s.from == "add" && s.to == "preview").map(|s| s.id.clone());
        if let Some(id) = stale_link {
            host.disconnect(&id).unwrap();
        }
        host.evaluate_internal();
        (host, pass_id)
    }

    #[test]
    fn evaluate_step_budget_one_converges_over_multiple_calls() {
        let (mut host, _pass_id) = host_with_two_node_chain();
        assert_eq!(host.preview_text(), "3", "chain settles to the same value as the direct add->preview link");
        host.set_slider_value("slider", 6.0);
        // ⏳ Nothing evaluates until stepped — mirrors a mutation with no tick chain run yet.
        assert_eq!(host.preview_text(), "3");
        // ⏱️ Tick 1: budget for one cache-missed node — computes "add" for free-riding boundary nodes
        // plus that one dispatch, then stops right before the next miss ("pass"). `remaining[0]` is
        // the blocking node; anything after it (here, "preview") is just downstream-and-untouched.
        let remaining_after_tick1 = host.evaluate_step(1);
        assert_eq!(remaining_after_tick1.first(), Some(&"pass".to_string()), "pass is the next node blocking completion");
        assert_eq!(host.preview_text(), "3", "the chain hasn't reached \"pass\" (and thus \"preview\") yet");
        // ⏱️ Tick 2: "add" is now cached, so this reaches and computes "pass".
        let remaining_after_tick2 = host.evaluate_step(1);
        assert!(remaining_after_tick2.is_empty(), "the walk reached the end of the topo order");
        assert_eq!(host.preview_text(), "6", "converged to the dragged value after both ticks");
    }

    #[test]
    fn flow_eval_driver_sync_and_tick_state_machine() {
        let (mut host, _pass_id) = host_with_two_node_chain();
        let mut driver = FlowEvalDriver::default();
        assert!(!driver.pending());
        // 🔁 Nothing changed yet — sync must not arm a chain.
        assert!(!driver.sync(&host));
        assert!(!driver.pending());
        host.set_slider_value("slider", 12.0);
        assert!(driver.sync(&host), "a changed slider arms the chain");
        assert!(driver.pending());
        assert!(driver.computing_json().is_some_and(|json| json.contains("add")), "the immediate dependent is reported as active");
        // 🔁 A `pending_effects`-style resync while a chain is already scheduled must not re-arm it.
        assert!(!driver.sync(&host));
        assert!(driver.tick(&mut host), "one more tick (\"pass\") is still needed");
        assert!(driver.pending());
        assert!(!driver.tick(&mut host), "the chain has converged");
        assert!(!driver.pending());
        assert_eq!(host.preview_text(), "12");
        // 🔀 Mid-chain fixture change: arm, tick once, then supersede with a newer value before the
        // chain finishes. `sync` correctly declines to arm a second chain (one is already scheduled —
        // `tick_scheduled` guards exactly this) but the in-flight chain's own ticks always re-derive
        // from the live fixture, so it still converges on the LATEST value, not the superseded one.
        host.set_slider_value("slider", 20.0);
        assert!(driver.sync(&host));
        assert!(driver.tick(&mut host)); // computes "add" for 20
        host.set_slider_value("slider", 30.0);
        assert!(!driver.sync(&host), "a chain is already scheduled — sync must not arm a redundant second one");
        assert!(driver.pending(), "the in-flight chain is still the one that will pick up 30");
        while driver.tick(&mut host) {}
        assert_eq!(host.preview_text(), "30", "converges on the latest value, not the superseded intermediate one");
    }

    #[test]
    fn connect_ports_allows_fan_out_from_same_output() {
        let mut host = host_with_test_bridge();
        let pass_id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough","params":{},"input_ports":[],"preview":false}"#, 120.0, 120.0).unwrap();
        host.connect_ports("add", "sum", &pass_id, "number").unwrap();
        let fan_out: Vec<_> = host.fixture.synapses.iter().filter(|s| s.from == "add" && s.from_port == "sum").collect();
        assert_eq!(fan_out.len(), 2);
        assert!(fan_out.iter().any(|s| s.to == "preview"));
        assert!(fan_out.iter().any(|s| s.to == pass_id));
    }

    #[test]
    fn connect_ports_replaces_existing_incoming_on_same_input() {
        let mut host = host_with_test_bridge();
        assert!(host.fixture.synapses.iter().any(|s| s.from == "slider" && s.to == "add" && s.to_port == "a"));
        let note_id = host.add_widget(r#"{"kind":"inputNote","id":"note","text":"2"}"#, -120.0, 0.0).unwrap();
        host.connect_ports(&note_id, "text", "add", "a").unwrap();
        let incoming_a: Vec<_> = host.fixture.synapses.iter().filter(|s| s.to == "add" && s.to_port == "a").collect();
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
        host.connect_ports("slider", "number", "add", "b").unwrap();
        host.evaluate_internal();
        assert!(calls.load(Ordering::Relaxed) > after_slider);
    }

    #[test]
    fn dirty_propagation_only_dispatches_affected_branch() {
        use std::sync::Arc;
        use std::sync::Mutex as StdMutex;
        // Branch A (default fixture): slider -> add -> preview. Branch B (added here): a second,
        // disconnected slider -> passThrough, sharing no synapse with branch A.
        let calls: Arc<StdMutex<Vec<String>>> = Arc::new(StdMutex::new(Vec::new()));
        let calls_for_bridge = calls.clone();
        let mut host = FlowHost::default();
        host.set_eval_bridge_fn(Box::new(move |kind, input| {
            calls_for_bridge.lock().unwrap().push(kind.to_string());
            test_math_bridge(kind, input)
        }));
        host.set_neuron_kind_infos_json(&test_kind_infos_json());
        let slider_b_id = host.add_widget(r#"{"kind":"inputSlider","value":1.0}"#, 400.0, 0.0).unwrap();
        let pass_id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough","params":{},"input_ports":[],"preview":false}"#, 600.0, 0.0).unwrap();
        host.connect_ports(&slider_b_id, "number", &pass_id, "number").unwrap();
        host.evaluate_internal();
        calls.lock().unwrap().clear();

        host.set_slider_value("slider", 5.0);
        host.evaluate_internal();

        let dispatched = calls.lock().unwrap().clone();
        assert!(dispatched.iter().any(|kind| kind == "math.add"), "branch A (add) should re-dispatch after its slider changed");
        assert!(!dispatched.iter().any(|kind| kind == "math.passThrough"), "branch B (pass) must stay clean when only branch A changed");
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
        let baseline = calls.load(Ordering::Relaxed);
        assert!(baseline > 0, "first evaluation is a cache miss and must dispatch to the bridge");
        host.evaluate_internal();
        assert_eq!(calls.load(Ordering::Relaxed), baseline, "an unchanged tree must be served entirely from the cache");
        host.set_slider_value("slider", 4.0);
        host.evaluate_internal();
        assert_eq!(calls.load(Ordering::Relaxed), baseline + 1, "only the node downstream of the changed slider should re-dispatch");
    }

    #[test]
    fn collect_live_geometry_handles_includes_input_channels() {
        let mut outputs = HashMap::new();
        outputs.insert("box".into(), Dictionary::with_schema("geometry").insert("handle", NeuralValue::Atom(Atom::String("solid-box".into()))).insert("kind", NeuralValue::Atom(Atom::String("solid".into()))));
        outputs.insert("volume".into(), Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(12.0))));
        let mut inputs = HashMap::new();
        inputs.insert("volume".into(), Dictionary::new().insert("geometry", NeuralValue::Dictionary(Dictionary::with_schema("geometry").insert("handle", NeuralValue::Atom(Atom::String("solid-box".into()))))));
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

    fn collect_live_geometry_handles(outputs: &HashMap<String, Dictionary>) -> Vec<String> {
        let mut handles = Vec::new();
        for dict in outputs.values() {
            collect_geometry_handles_from_dictionary(dict, &mut handles);
        }
        handles.sort();
        handles.dedup();
        handles
    }

    #[test]
    fn collect_live_geometry_handles_traverses_nested_dictionaries() {
        let mut outputs = HashMap::new();
        outputs.insert("box".into(), Dictionary::with_schema("geometry").insert("handle", NeuralValue::Atom(Atom::String("solid-1".into()))).insert("kind", NeuralValue::Atom(Atom::String("solid".into()))));
        outputs.insert("nested".into(), Dictionary::new().insert("child", NeuralValue::Dictionary(Dictionary::with_schema("face").insert("handle", NeuralValue::Atom(Atom::String("face-2".into()))))));
        let handles = collect_live_geometry_handles(&outputs);
        assert_eq!(handles, vec![String::from("face-2"), String::from("solid-1")]);
    }

    #[test]
    fn collect_live_drawing_handles_traverses_list_values() {
        let mut outputs = HashMap::new();
        outputs.insert(
            "get".into(),
            Dictionary::new()
                .insert("value", NeuralValue::Dictionary(Dictionary::with_schema("list").insert("0", NeuralValue::Dictionary(Dictionary::with_schema("draw.drawing").insert("handle", NeuralValue::Atom(Atom::String("drawing-2".into()))))))),
        );
        let channels = EvalChannels { outputs, inputs: HashMap::new() };
        assert_eq!(collect_live_drawing_handles_from_channels(&channels), vec![String::from("drawing-2")]);
    }

    #[test]
    fn evaluate_emits_channel_structured_json() {
        let host = host_with_test_bridge();
        let parsed: serde_json::Value = serde_json::from_str(&host.last_eval_json).expect("json");
        let add = parsed.get("add").and_then(|value| value.as_object()).expect("add channels");
        assert!(add.get("in").and_then(|value| value.as_object()).is_some());
        let out = add.get("out").and_then(|value| value.as_object()).expect("add out");
        assert!(out.get("sum").is_some());
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
        assert_eq!(seeds.get("image").and_then(|d| d.get("image")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("dataUrl")).and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some(png));
    }

    #[test]
    fn slider_drag_does_not_evaluate_until_explicit_evaluate() {
        // 🧵 A live drag firing many pointer-move ticks used to re-evaluate the whole graph on every
        // one of them (fine for cheap graphs, a repeated multi-second stall for a heavy one, e.g. a
        // brep boolean). Dragging alone must never evaluate now — the off-main-thread ticker (outside
        // `flow_core`) picks up the changed slider value at its own pace; an explicit `evaluate`
        // (simulated here) still updates the preview once it runs.
        let mut host = host_with_test_bridge();
        host.set_viewport(800, 600, 1.0);
        let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
        assert_eq!(host.preview_text(), "3");
        host.pointer_down_screen(sx, sy, 0, false, false, false, false);
        host.pointer_move_screen(sx + 80.0, sy, false, false, false);
        assert_eq!(host.preview_text(), "3", "a live drag must not synchronously re-evaluate the graph");
        host.evaluate_internal();
        assert_ne!(host.preview_text(), "3", "an explicit evaluate still picks up the dragged value");
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
        assert_eq!(parsed.schema, "flow.fixture");
    }

    #[test]
    fn flow_document_tree_is_shakable() {
        let host = host_with_test_bridge();
        let document = host.document();
        assert_eq!(document.schema, "flow.document");
        assert!(!document.tree.neurons.is_empty());
        let registry = neural::Registry::new();
        let evaluator = Evaluator::new(&registry);
        let dispatch = |kind: &str, input: &Dictionary| test_math_bridge(kind, input);
        let mut seeds = HashMap::new();
        seeds.insert("slider".into(), channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(3.0)))));
        let channels = evaluator.evaluate_channels_with(&document.tree, &seeds, &host.kind_infos, &dispatch).unwrap();
        assert_eq!(channels.outputs.get("add").and_then(|d| d.get("sum")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    }

    #[test]
    fn rebuild_dag_preserves_canvas_theme() {
        use cavas::Color;
        let mut host = FlowHost::default();
        host.dag.canvas_theme.node_fill = Color::from_rgba8(12, 34, 56, 255);
        host.rebuild_dag();
        assert_eq!(host.dag.canvas_theme.node_fill.to_rgba8(), Color::from_rgba8(12, 34, 56, 255).to_rgba8());
    }

    #[test]
    fn set_canvas_theme_dark_applies_board_dark_strokes() {
        let mut host = FlowHost::default();
        host.set_canvas_theme_dark(true);
        let stroke = host.dag.canvas_theme.node_stroke.to_rgba8();
        assert!(stroke.r > 80 || stroke.g > 80);
        host.set_canvas_theme_dark(false);
        let light_stroke = host.dag.canvas_theme.node_stroke.to_rgba8();
        assert!(light_stroke.r < 80);
    }

    #[test]
    fn paint_scene_dark_theme_paints_edges_and_nodes() {
        let mut host = host_with_test_bridge();
        host.set_viewport(1280, 800, 1.0);
        host.set_canvas_theme_dark(true);
        let mut scene = cavas::Scene::new();
        host.paint_scene(&mut scene, 1280, 800, 1.0);
        assert!(scene.path_count() > 8, "populated fixture should paint edges, handles, and node bodies under dark board theme");
    }

    #[test]
    fn replace_fixture_preserves_kind_infos_and_named_input_ports() {
        let mut host = host_with_test_bridge();
        host.replace_fixture(FlowFixture {
            schema: "flow.fixture".into(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            widgets: vec![Widget::Neuron { id: "add".into(), neuron_kind: "math.add".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: true }],
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
                    neuron_kind: Some("brep.prim3d.box".into()),
                    action: None,
                    format: None,
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
        assert_eq!(prim3d.items[0].neuron_kind.as_deref(), Some("brep.prim3d.box"));
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
    fn flow_backed_node_graph_extras_include_fixture_and_flow_engine() {
        let host = host_with_test_bridge();
        let extras = flow_backed_node_graph_extras(&host.fixture, FLOW_LOD_MODE_AUTOMATIC, 0.0, true, false, ui_styling::metrics::board::GRID_FACTOR_DEFAULT, None);
        assert!(extras.fixture_json.as_ref().is_some_and(|json| json.contains("flow.fixture")));
        assert!(extras.operators_json.as_ref().is_some_and(|json| json.contains("math.add")));
        assert!(extras.capabilities_json.as_ref().is_some_and(|json| json.contains(r#""engine":"flow""#)));
        assert!(extras.lod_json.as_ref().is_some_and(|json| json.contains(r#""automatic":true"#)));
    }

    #[test]
    fn flow_fixture_with_synapses_builds_dag_edges_and_ports() {
        let mut host = host_with_test_bridge();
        host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
        host.replace_fixture(<FlowFixture as store::DocumentDsl>::parse_dsl(include_str!("../../example/default.flow")).expect("fixture"));
        assert!(!host.dag.fixture.edges.is_empty(), "synapses should become dag edges");
        let add = host.dag.fixture.nodes.iter().find(|node| node.id == "add").expect("add node");
        assert_eq!(add.inputs().len(), 2);
        assert_eq!(add.outputs().len(), 1);
        let mut scene = cavas::Scene::new();
        host.set_canvas_theme_dark(true);
        host.paint_scene(&mut scene, 1280, 800, 1.0);
        assert!(scene.path_count() > 8, "rich flow graph should paint edges and handles");
    }

    #[test]
    fn add_widget_and_connect() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"neuron","neuronKind":"math.passThrough"}"#, 100.0, 50.0).unwrap();
        host.connect_ports("slider", "number", &id, "number").unwrap();
        host.connect_ports(&id, "number", "preview", "").unwrap();
        host.set_slider_value("slider", 4.0);
        host.evaluate_internal();
        assert_eq!(host.preview_text(), "4");
    }

    #[test]
    fn output_export_widget_catalogue_descriptor_and_payload() {
        let mut host = host_with_test_bridge();
        let sections = merge_catalogue_sections("").unwrap();
        let exports: Vec<_> = sections.iter().flat_map(|section| section.items.iter()).filter(|item| item.kind == "outputExport").collect();
        assert_eq!(exports.len(), 4);
        assert!(exports.iter().any(|item| item.format.as_deref() == Some("svg")));
        let id = host.add_widget(r#"{"kind":"outputExport","format":"png"}"#, 120.0, 80.0).unwrap();
        host.connect_ports("add", "sum", &id, "").unwrap();
        host.set_slider_value("slider", 4.0);
        host.evaluate_internal();
        let payload_json = host.export_payload_json(&id).expect("export payload");
        assert_ne!(payload_json, "{}");
        assert!(payload_json.contains("4") || payload_json.contains("value") || payload_json.contains("sum"));
        let node = host.dag.fixture.nodes.iter().find(|node| node.id == id).expect("export node");
        assert!(matches!(node.kind, DagNodeKind::Export { .. }));
    }

    /// ↩️ Exercises the standard `store::DocumentStore<FlowFixture, FlowOperation>` undo/redo
    /// mechanism directly (the same one `FlowHost::undo`/`redo` are built on) — add a widget, undo,
    /// confirm it's gone, redo, confirm it's back — in place of the old test's direct assertions on a
    /// hand-rolled `Vec<FlowFixture>` snapshot stack.
    #[test]
    fn undo_redo_add_widget() {
        let mut host = host_with_test_bridge();
        let fixture_before = host.fixture.clone();
        let count_before = fixture_before.widgets.len();
        let id = host.add_widget(r#"{"kind":"inputNote","text":"undo me"}"#, 42.0, 42.0).unwrap();
        assert_eq!(host.fixture.widgets.len(), count_before + 1);

        let operations = flow_fixture_operations(&fixture_before, &host.fixture);
        assert!(!operations.is_empty(), "add_widget must diff into vcs operations");

        let envelope: FlowEnvelope = create_document_envelope(FLOW_DOCUMENT_SCHEMA, "test", fixture_before, None);
        let mut store = FlowStore::new(envelope);
        store.dispatch(DocumentCommand::Apply { operations, description: None }).expect("apply add-widget operations");
        assert_eq!(store.projection().expect("projection").widgets.len(), count_before + 1);

        store.dispatch(DocumentCommand::Undo).expect("undo");
        let after_undo = store.projection().expect("projection");
        assert_eq!(after_undo.widgets.len(), count_before);
        assert!(!after_undo.widgets.iter().any(|w| widget_id_for(w) == id));

        store.dispatch(DocumentCommand::Redo).expect("redo");
        let after_redo = store.projection().expect("projection");
        assert!(after_redo.widgets.iter().any(|w| widget_id_for(w) == id));
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

    #[test]
    fn replace_fixture_preserves_live_camera() {
        let mut host = host_with_test_bridge();
        host.set_camera(120.0, -45.0, 1.75);
        host.replace_fixture(FlowFixture {
            schema: "flow.fixture".into(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            widgets: vec![Widget::InputNote { id: "note".into(), text: "hello".into() }],
            synapses: vec![],
            layout: BTreeMap::new(),
        });
        assert_eq!(host.fixture.camera.x, 120.0);
        assert_eq!(host.fixture.camera.y, -45.0);
        assert!((host.fixture.camera.zoom - 1.75).abs() < 1e-9);
        assert!(host.fixture.widgets.iter().any(|w| widget_id_for(w) == "note"));
    }

    fn test_dictionary_merge_bridge(kind: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        if kind == "core.number" {
            let value = input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
            return Ok(Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(value))));
        }
        if kind != "dictionary.merge" {
            return Err(EvalError::UnknownKind(kind.into()));
        }
        let items = input.get("items").and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput("items".into()))?;
        let mut indices: Vec<usize> = items.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
        indices.sort_unstable();
        if indices.len() < 2 {
            return Err(EvalError::MissingInput("items".into()));
        }
        let mut merged = Dictionary::with_schema("dictionary");
        for index in indices {
            let slot = items.get(&index.to_string()).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(index.to_string()))?;
            merged = merged.merge(slot);
        }
        Ok(channel_output("dictionary", merged))
    }

    #[test]
    fn variadic_merge_evaluates_port_routed_inputs() {
        let mut host = FlowHost::from_fixture(FlowFixture {
            schema: "flow.fixture".into(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            widgets: vec![
                Widget::InputSlider { id: "a".into(), value: 1.0, min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP },
                Widget::InputSlider { id: "b".into(), value: 2.0, min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP },
                Widget::Neuron { id: "merge".into(), neuron_kind: "dictionary.merge".into(), params: Dictionary::new(), input_ports: vec!["0".into(), "1".into()], output_ports: vec![], preview: true },
                Widget::OutputPreview { id: "preview".into(), preview: Dictionary::new(), expanded: BTreeSet::new() },
            ],
            synapses: vec![
                SynapseSpec { id: "s1".into(), from: "a".into(), to: "merge".into(), from_port: "number".into(), to_port: "0".into() },
                SynapseSpec { id: "s2".into(), from: "b".into(), to: "merge".into(), from_port: "number".into(), to_port: "1".into() },
                SynapseSpec { id: "s3".into(), from: "merge".into(), to: "preview".into(), from_port: "dictionary".into(), to_port: String::new() },
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
                outputs: vec![InputSpec::named("D", "Dic", "dictionary", "MergedDictionary")],
                variadic_input: Some(neural::VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
                ..Default::default()
            }])
            .unwrap(),
        );
        host.previous_snapshot = None;
        host.outputs.clear();
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
        assert_eq!(preview.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(2.0));
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
        let id = host.add_widget(r#"{"kind":"inputSlider","value":10.2,"min":10.2,"max":15.0,"step":0.1}"#, 0.0, 0.0).unwrap();
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
        assert!(node.width >= 40.0);
        assert_eq!(node.height, dag::DAG_CHANNEL_ROW_HEIGHT);
    }

    #[test]
    fn begin_note_edit_groups_undo_into_single_gesture() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputNote","text":"hi"}"#, 0.0, 0.0).unwrap();
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
        let origin_x = node.x - node.width * 0.5 + 4.0;
        host.begin_note_edit(&id, origin_x + 40.0, node.y);
        host.note_insert_text("!");
        host.note_commit_edit();
        let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
        let Widget::InputNote { text, .. } = widget else {
            panic!("expected note widget");
        };
        assert_eq!(text, "hi!");
        assert!(host.undo());
        let Widget::InputNote { text: restored, .. } = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget") else {
            panic!("expected note widget");
        };
        assert_eq!(restored, "hi");
    }

    #[test]
    fn wheel_screen_zoom_gesture_changes_zoom() {
        let mut host = host_with_test_bridge();
        let z0 = host.fixture.camera.zoom;
        host.wheel_screen(400.0, 300.0, 0.0, -10.0, true);
        assert_ne!(host.fixture.camera.zoom, z0);
    }

    #[test]
    fn set_note_text_keeps_uniform_component_width() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputNote","text":"hi"}"#, 0.0, 0.0).unwrap();
        let short_w = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node").width;
        host.set_note_text(&id, "a much longer note string");
        let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
        let DagNodeKind::Note { text, .. } = &node.kind else {
            panic!("expected note node");
        };
        assert_eq!(text, "a much longer note string");
        assert_eq!(node.width, short_w);
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
                outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
                ..Default::default()
            }])
            .unwrap(),
        );
        let descriptor = r#"{"kind":"neuron","neuronKind":"brep.sketch2d.circle"}"#;
        host.set_ghost_widget(descriptor, 40.0, 40.0).unwrap();
        let ghost_width = host.ghost_node.as_ref().expect("ghost").width;
        let placed_id = host.add_widget(descriptor, 80.0, 80.0).unwrap();
        let placed_width = host.dag.fixture.nodes.iter().find(|node| node.id == placed_id).expect("placed").width;
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
                outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
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
        let overlay_ghost_rows: Vec<_> = overlay["labels"].as_array().unwrap().iter().filter(|row| row["ghost"] == true).collect();
        assert_eq!(overlay_ghost_rows.len(), 3);
        let placed_node = {
            let widget = widget_from_descriptor(&serde_json::from_str::<WidgetDescriptor>(descriptor).unwrap(), "placed".into(), &host.kind_infos);
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
        let mut scene = cavas::Scene::new();
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
                outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
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
        let ghost_rows: Vec<_> = overlay["labels"].as_array().unwrap().iter().filter(|row| row["ghost"] == true).collect();
        assert_eq!(ghost_rows.len(), 3);
    }

    #[test]
    fn ghost_widget_paint_scene_smoke() {
        let mut host = host_with_test_bridge();
        host.set_viewport(800, 600, 1.0);
        host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"math.add"}"#, 10.0, 20.0).unwrap();
        let mut scene = cavas::Scene::new();
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
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[NeuronKindInfo {
                id: "dictionary.merge".into(),
                module: "dictionary".into(),
                name: "Merge".into(),
                abbreviation: "Merge".into(),
                icon: "emoji:🔀".into(),
                summary: "Merge".into(),
                inputs: vec![],
                outputs: vec![InputSpec::named("D", "Dic", "dictionary", "MergedDictionary")],
                variadic_input: Some(neural::VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
                ..Default::default()
            }])
            .unwrap(),
        );
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
        use cavas::Point;
        let mut host = FlowHost::default();
        host.set_viewport(1280, 800, 1.0);
        host.fixture.widgets = vec![
            Widget::Neuron { id: "sphere".into(), neuron_kind: "brep.prim3d.sphere".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: false },
            Widget::Neuron { id: "torus".into(), neuron_kind: "brep.prim3d.torus".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: false },
            Widget::Neuron { id: "cut".into(), neuron_kind: "brep.bool.cut".into(), params: Dictionary::new(), input_ports: vec!["a".into(), "b".into()], output_ports: vec![], preview: true },
        ];
        host.fixture.synapses = vec![
            SynapseSpec { id: "e1".into(), from: "sphere".into(), to: "cut".into(), from_port: "solid".into(), to_port: "a".into() },
            SynapseSpec { id: "e2".into(), from: "torus".into(), to: "cut".into(), from_port: "solid".into(), to_port: "b".into() },
        ];
        host.fixture.layout.insert("sphere".into(), WidgetLayout { x: 0.0, y: -60.0 });
        host.fixture.layout.insert("torus".into(), WidgetLayout { x: 0.0, y: 60.0 });
        host.fixture.layout.insert("cut".into(), WidgetLayout { x: 240.0, y: 0.0 });
        let solid_out = vec![InputSpec::named("S", "Sld", "solid", "Solid")];
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[
                NeuronKindInfo {
                    id: "brep.prim3d.sphere".into(),
                    module: "brep".into(),
                    name: "Sphere".into(),
                    abbreviation: "Sphere".into(),
                    icon: "emoji:⚪".into(),
                    summary: "Sphere".into(),
                    inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
                    outputs: solid_out.clone(),
                    ..Default::default()
                },
                NeuronKindInfo {
                    id: "brep.prim3d.torus".into(),
                    module: "brep".into(),
                    name: "Torus".into(),
                    abbreviation: "Torus".into(),
                    icon: "emoji:🛢️".into(),
                    summary: "Torus".into(),
                    inputs: vec![InputSpec::number_default("major", 2.0, NUMBER_OPS), InputSpec::number_default("minor", 0.5, NUMBER_OPS)],
                    outputs: solid_out.clone(),
                    ..Default::default()
                },
                NeuronKindInfo {
                    id: "brep.bool.cut".into(),
                    module: "brep".into(),
                    name: "Cut".into(),
                    abbreviation: "Cut".into(),
                    icon: "emoji:🔗".into(),
                    summary: "Cut".into(),
                    inputs: vec![InputSpec::requires("a", &["geometry"]), InputSpec::requires("b", &["geometry"])],
                    outputs: solid_out,
                    ..Default::default()
                },
            ])
            .unwrap(),
        );
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
        assert!(host.dag.engine.render_snapshot().pending_edge.is_none(), "dragging wired cut near sources must not preview proximity edges");
        host.pointer_up_screen(screen.x - 180.0, screen.y, false, false, false);
        assert_eq!(host.dag.engine.edges.len(), 2);
        assert_eq!(host.fixture.synapses.len(), 2);
    }

    #[test]
    fn dag_bridge_keeps_same_named_brep_input_and_output_distinct() {
        let mut host = FlowHost::default();
        host.fixture.widgets = vec![
            Widget::Neuron { id: "extrude".into(), neuron_kind: "brep.solid.extrude".into(), params: Dictionary::new(), input_ports: vec!["wire".into(), "vector".into()], output_ports: vec![], preview: true },
            Widget::Neuron { id: "brep".into(), neuron_kind: "brep.brep".into(), params: Dictionary::new(), input_ports: vec!["brep".into(), "vertex".into(), "edge".into(), "face".into()], output_ports: vec![], preview: true },
            Widget::Neuron { id: "get".into(), neuron_kind: "list.get".into(), params: Dictionary::new(), input_ports: vec!["list".into(), "index".into(), "wrap".into()], output_ports: vec!["0".into()], preview: true },
        ];
        host.fixture.synapses = vec![
            SynapseSpec { id: "e112".into(), from: "extrude".into(), to: "brep".into(), from_port: "solid".into(), to_port: "brep".into() },
            SynapseSpec { id: "e113".into(), from: "brep".into(), to: "get".into(), from_port: "brep".into(), to_port: "list".into() },
        ];
        host.fixture.layout.insert("extrude".into(), WidgetLayout { x: 0.0, y: 0.0 });
        host.fixture.layout.insert("brep".into(), WidgetLayout { x: 200.0, y: 0.0 });
        host.fixture.layout.insert("get".into(), WidgetLayout { x: 400.0, y: 0.0 });
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[
                NeuronKindInfo {
                    id: "brep.solid.extrude".into(),
                    module: "brep".into(),
                    name: "Extrude".into(),
                    abbreviation: "Extr".into(),
                    icon: "emoji:⬆️".into(),
                    summary: "Extrude".into(),
                    inputs: vec![InputSpec::requires("wire", &["geometry"]), InputSpec::requires("vector", &["vector"])],
                    outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
                    ..Default::default()
                },
                NeuronKindInfo {
                    id: "brep.brep".into(),
                    module: "brep".into(),
                    name: "Brep".into(),
                    abbreviation: "Brep".into(),
                    icon: "emoji:🧊".into(),
                    summary: "Brep".into(),
                    inputs: vec![InputSpec::requires("brep", &["brep.brep"]), InputSpec::list("vertex", &["brep.brep"]), InputSpec::list("edge", &["brep.brep"]), InputSpec::list("face", &["brep.brep"])],
                    outputs: vec![InputSpec::named("B", "Brp", "brep", "Brep")],
                    ..Default::default()
                },
                NeuronKindInfo {
                    id: "list.get".into(),
                    module: "list".into(),
                    name: "Get".into(),
                    abbreviation: "Get".into(),
                    icon: "emoji:📋".into(),
                    summary: "Get".into(),
                    inputs: vec![InputSpec::list("list", &["list.get"]), InputSpec::number_default("index", 0.0, &["list.get"]), InputSpec::boolean_default("wrap", false, &["list.get"])],
                    outputs: vec![InputSpec::named("V", "Val", "value", "ListValue")],
                    ..Default::default()
                },
            ])
            .unwrap(),
        );
        host.rebuild_dag();
        let incoming = host.dag.engine.edges.get(&112).expect("incoming brep edge");
        let outgoing = host.dag.engine.edges.get(&113).expect("outgoing brep edge");
        let incoming_target = host.dag.engine.handles.get(&incoming.target).expect("incoming target");
        let outgoing_source = host.dag.engine.handles.get(&outgoing.source).expect("outgoing source");
        assert_eq!(incoming_target.role, HandleRole::Target);
        assert_eq!(outgoing_source.role, HandleRole::Source);
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
        assert!(host.fixture.layout.contains_key("slider"));
        assert!(host.fixture.layout.contains_key("add"));
    }

    #[test]
    fn add_input_port_inserts_variadic_slot() {
        let mut host = host_with_test_bridge();
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[NeuronKindInfo {
                id: "dictionary.merge".into(),
                module: "dictionary".into(),
                name: "Merge".into(),
                abbreviation: "Merge".into(),
                icon: "emoji:🔀".into(),
                summary: "Merge".into(),
                inputs: vec![],
                outputs: vec![InputSpec::named("D", "Dic", "dictionary", "MergedDictionary")],
                variadic_input: Some(neural::VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
                ..Default::default()
            }])
            .unwrap(),
        );
        let merge_id = host.add_widget(r#"{"kind":"neuron","neuronKind":"dictionary.merge"}"#, 0.0, 0.0).unwrap();
        host.add_input_port(&merge_id, 1).unwrap();
        let widget = host.fixture.widgets.iter().find(|widget| widget_id_for(widget) == merge_id).expect("merge");
        let Widget::Neuron { input_ports, .. } = widget else { panic!("neuron") };
        assert_eq!(input_ports.len(), 3);
    }

    #[test]
    fn add_output_port_inserts_variadic_get_slot() {
        let mut host = host_with_test_bridge();
        host.set_neuron_kind_infos_json(
            &serde_json::to_string(&[NeuronKindInfo {
                id: "list.get".into(),
                module: "list".into(),
                name: "Get".into(),
                abbreviation: "Get".into(),
                icon: "emoji:📋".into(),
                summary: "Reads consecutive values by index".into(),
                inputs: vec![InputSpec::list("list", &["list.get"]), InputSpec::number_default("index", 0.0, &["list.get"]), InputSpec::boolean_default("wrap", false, &["list.get"])],
                outputs: vec![InputSpec::named("V", "Val", "value", "ListValue")],
                variadic_output: Some(neural::VariadicSpec { slot_key: "value".into(), min: 1, max: None }),
                ..Default::default()
            }])
            .unwrap(),
        );
        let get_id = host.add_widget(r#"{"kind":"neuron","neuronKind":"list.get"}"#, 0.0, 0.0).unwrap();
        let node = host.dag.fixture.nodes.iter().find(|node| node.id == get_id).expect("get");
        let labels: Vec<&str> = node.outputs().iter().map(|port| port.label.as_str()).collect();
        assert_eq!(labels, vec!["i"]);
        host.add_output_port(&get_id, 1).unwrap();
        let widget = host.fixture.widgets.iter().find(|widget| widget_id_for(widget) == get_id).expect("get");
        let Widget::Neuron { output_ports, .. } = widget else { panic!("neuron") };
        assert_eq!(output_ports.len(), 2);
        let node = host.dag.fixture.nodes.iter().find(|node| node.id == get_id).expect("get");
        let labels: Vec<&str> = node.outputs().iter().map(|port| port.label.as_str()).collect();
        assert_eq!(labels, vec!["i", "i+1"]);
    }

    #[test]
    fn add_widget_with_explicit_id() {
        let mut host = host_with_test_bridge();
        let id = host.add_widget(r#"{"kind":"inputSlider","id":"custom_slider","value":2.0}"#, 0.0, 0.0).unwrap();
        assert_eq!(id, "custom_slider");
    }

    #[test]
    fn insert_between_rewires_downstream_and_connects_anchor() {
        let mut host = host_with_test_bridge();
        let mid = host.add_widget(r#"{"kind":"neuron","id":"mid","neuronKind":"math.passThrough"}"#, 120.0, 0.0).unwrap();
        host.insert_between("slider", "number", &mid, "number", "number").unwrap();
        assert!(host.fixture.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == "mid"));
        assert!(host.fixture.synapses.iter().any(|synapse| synapse.from == "mid" && synapse.to == "add"));
        assert!(host.fixture.synapses.iter().any(|synapse| synapse.from == "add" && synapse.to == "preview"));
        assert!(!host.fixture.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == "add"));
    }

    #[test]
    fn insert_between_preserves_existing_mid_inputs() {
        let mut host = host_with_test_bridge();
        let variable_id = host.add_widget(r#"{"kind":"variable","name":"width","schema":"number"}"#, 120.0, 0.0).unwrap();
        host.connect_ports("slider", "number", &variable_id, "width").unwrap();
        host.insert_between("slider", "number", &variable_id, "width", "width").unwrap();
        assert!(host.fixture.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == variable_id && synapse.to_port == "width"));
        assert!(!host.fixture.synapses.iter().any(|synapse| synapse.from == variable_id && synapse.to == variable_id));
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
        let preview_synapse = host.fixture.synapses.iter().find(|synapse| synapse.from == "add" && synapse.to == "preview").map(|synapse| synapse.id.clone()).expect("preview synapse");
        host.disconnect(&preview_synapse).unwrap();
        let id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough"}"#, 100.0, 0.0).unwrap();
        host.connect_ports(&id, "number", "preview", "").unwrap();
        host.set_neuron_params(&id, r#"{"number":{"$schema":"number","value":7.5}}"#).unwrap();
        host.evaluate_internal();
        assert_eq!(host.preview_text(), "7.5");
    }

    #[test]
    fn cluster_ports_from_contract() {
        let inner = Tree {
            neurons: vec![
                Neuron::with_kind("in_a", INPUT_KIND, Dictionary::new().insert("channel", NeuralValue::Atom(Atom::String("a".into()))).insert("operators", NeuralValue::Atom(Atom::String("core.number".into())))),
                Neuron::with_kind("out_sum", OUTPUT_KIND, Dictionary::new().insert("channel", NeuralValue::Atom(Atom::String("sum".into()))).insert("operators", NeuralValue::Atom(Atom::String("core.number".into())))),
            ],
            synapses: vec![],
        };
        let widget = Widget::Cluster { id: "cluster".into(), name: "Add cluster".into(), tree: inner, flow: FlowGui::default() };
        let (inputs, outputs, _, _) = widget_io_ports(&widget, &[], &HashMap::new());
        assert_eq!(inputs.len(), 1);
        assert_eq!(outputs.len(), 1);
        assert_eq!(inputs[0].id, "a");
        assert_eq!(outputs[0].id, "sum");
    }

    #[test]
    fn variable_relay_evaluates_through_flow_host() {
        let mut host = host_with_test_bridge();
        let variable_id = host.add_widget(r#"{"kind":"variable","name":"width","schema":"number"}"#, 0.0, 0.0).unwrap();
        let slider_id = host.add_widget(r#"{"kind":"inputSlider","value":4.0}"#, -200.0, 0.0).unwrap();
        host.connect_ports(&slider_id, "number", &variable_id, "width").unwrap();
        let eval_json = host.evaluate().expect("evaluate");
        let parsed: serde_json::Value = serde_json::from_str(&eval_json).expect("eval json");
        let width = parsed.get(&variable_id).and_then(|entry| entry.get("out")).and_then(|out| out.get("width")).expect("variable width output");
        assert_eq!(width.get("$schema").and_then(|value| value.as_str()), Some("number"));
    }

    #[test]
    fn collapse_uses_variable_name_as_cluster_input_port() {
        let mut host = host_with_test_bridge();
        host.fixture.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
        let variable_id = host.add_widget(r#"{"kind":"variable","name":"width","schema":"number"}"#, 100.0, 0.0).unwrap();
        host.fixture.layout.insert("add".into(), WidgetLayout { x: 200.0, y: 0.0 });
        host.fixture.synapses.retain(|synapse| synapse.from != "slider" || synapse.to != "add");
        host.connect_ports("slider", "number", &variable_id, "width").unwrap();
        host.connect_ports(&variable_id, "width", "add", "a").unwrap();
        host.rebuild_dag();
        let cluster_id = host.collapse_selection(&[variable_id.clone(), "add".into()]).unwrap();
        let cluster = host
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Cluster { id, tree, .. } if id == &cluster_id => Some(tree.clone()),
                _ => None,
            })
            .expect("cluster");
        let (inputs, _) = cluster.contract();
        assert!(inputs.iter().any(|port| port.name == "width"));
        host.explode_cluster(&cluster_id).unwrap();
        assert!(host.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Variable { name, .. } if name == "width")));
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

    #[test]
    fn rectangle_extrude_fixture_port_labels_follow_draw_lod() {
        let _guard = RECTANGLE_EXTRUDE_FIXTURE_TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|error| error.into_inner());
        // 🩹 Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
        // handcrafted DSL (`store::DocumentDsl`) — inlined the same flow-fixture JSON this test actually
        // parses (`FlowHost::parse_fixture_json`), decoupled from procedural's document format.
        let json = r#"{
  "schema": "flow.fixture",
  "camera": { "x": 140, "y": -60, "zoom": 2.2 },
  "widgets": [
    { "kind": "inputSlider", "id": "width", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "height", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "distance", "value": 3, "min": 0.1, "max": 10, "step": 0.1 },
    {
      "kind": "neuron",
      "id": "rect",
      "neuronKind": "brep.curve.rectangle",
      "params": {},
      "input_ports": ["width", "height"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "vector",
      "neuronKind": "math.vector",
      "params": {},
      "input_ports": ["x", "y", "z"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "extrude",
      "neuronKind": "brep.solid.extrude",
      "params": {},
      "input_ports": ["wire", "vector"],
      "preview": true
    },
    {
      "kind": "neuron",
      "id": "volume",
      "neuronKind": "brep.measure.volume",
      "params": {},
      "input_ports": ["geometry"],
      "preview": false
    }
  ],
  "synapses": [
    { "id": "e1", "from": "width", "to": "rect", "fromPort": "number", "toPort": "width" },
    { "id": "e2", "from": "height", "to": "rect", "fromPort": "number", "toPort": "height" },
    { "id": "e3", "from": "rect", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e4", "from": "distance", "to": "vector", "fromPort": "number", "toPort": "z" },
    { "id": "e5", "from": "vector", "to": "extrude", "fromPort": "vector", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "volume", "fromPort": "solid", "toPort": "geometry" }
  ],
  "layout": {
    "rect": { "x": 120, "y": -40 },
    "vector": { "x": 200, "y": 20 },
    "extrude": { "x": 280, "y": -40 },
    "volume": { "x": 360, "y": -40 },
    "width": { "x": 40, "y": -60 },
    "height": { "x": 40, "y": -20 },
    "distance": { "x": 120, "y": 20 }
  }
}
"#;
        let fixture = FlowHost::parse_fixture_json(json).expect("fixture json");
        let mut host = FlowHost::from_fixture(fixture);
        host.set_neuron_kind_infos_json(&fixture_kind_infos_json());
        host.set_viewport(1280, 800, 1.0);
        host.fixture.camera.zoom = 1.0;
        host.rebuild_dag();
        let mut port_texts = |lod: &str| -> Vec<String> {
            host.dag.set_automatic_lod(false);
            host.dag.set_forced_draw_lod_label(lod);
            let raw: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
            raw["labels"].as_array().expect("labels").iter().filter(|row| row["kind"] == "port").filter_map(|row| row["text"].as_str().map(str::to_string)).collect()
        };
        let normal = port_texts("normal");
        assert!(normal.iter().any(|text| text.ends_with("wid")), "normal ports: {normal:?}");
        assert!(normal.iter().any(|text| text.ends_with("wir")), "normal ports: {normal:?}");
        let detail = port_texts("detail");
        assert!(detail.iter().any(|text| text.ends_with("width")), "detail ports: {detail:?}");
        assert!(detail.iter().any(|text| text.ends_with("wire")), "detail ports: {detail:?}");
        let micro = port_texts("micro");
        assert!(micro.iter().any(|text| text.ends_with("RectangleWire")), "micro ports: {micro:?}");
        assert!(micro.iter().any(|text| text.ends_with("ExtrudedSolid")), "micro ports: {micro:?}");
    }

    #[test]
    fn rectangle_extrude_fixture_evaluates_solid_output() {
        let _guard = RECTANGLE_EXTRUDE_FIXTURE_TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|error| error.into_inner());
        // 🩹 Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
        // handcrafted DSL (`store::DocumentDsl`) — inlined the same flow-fixture JSON this test actually
        // parses (`FlowHost::parse_fixture_json`), decoupled from procedural's document format.
        let json = r#"{
  "schema": "flow.fixture",
  "camera": { "x": 140, "y": -60, "zoom": 2.2 },
  "widgets": [
    { "kind": "inputSlider", "id": "width", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "height", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "distance", "value": 3, "min": 0.1, "max": 10, "step": 0.1 },
    {
      "kind": "neuron",
      "id": "rect",
      "neuronKind": "brep.curve.rectangle",
      "params": {},
      "input_ports": ["width", "height"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "vector",
      "neuronKind": "math.vector",
      "params": {},
      "input_ports": ["x", "y", "z"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "extrude",
      "neuronKind": "brep.solid.extrude",
      "params": {},
      "input_ports": ["wire", "vector"],
      "preview": true
    },
    {
      "kind": "neuron",
      "id": "volume",
      "neuronKind": "brep.measure.volume",
      "params": {},
      "input_ports": ["geometry"],
      "preview": false
    }
  ],
  "synapses": [
    { "id": "e1", "from": "width", "to": "rect", "fromPort": "number", "toPort": "width" },
    { "id": "e2", "from": "height", "to": "rect", "fromPort": "number", "toPort": "height" },
    { "id": "e3", "from": "rect", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e4", "from": "distance", "to": "vector", "fromPort": "number", "toPort": "z" },
    { "id": "e5", "from": "vector", "to": "extrude", "fromPort": "vector", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "volume", "fromPort": "solid", "toPort": "geometry" }
  ],
  "layout": {
    "rect": { "x": 120, "y": -40 },
    "vector": { "x": 200, "y": 20 },
    "extrude": { "x": 280, "y": -40 },
    "volume": { "x": 360, "y": -40 },
    "width": { "x": 40, "y": -60 },
    "height": { "x": 40, "y": -20 },
    "distance": { "x": 120, "y": 20 }
  }
}
"#;
        let fixture = FlowHost::parse_fixture_json(json).expect("fixture json");
        let mut host = FlowHost::from_fixture(fixture);
        host.set_neuron_kind_infos_json(&fixture_kind_infos_json());
        let eval_json = host.evaluate().expect("evaluate");
        let parsed: serde_json::Value = serde_json::from_str(&eval_json).expect("eval json");
        let solid = parsed.get("extrude").and_then(|entry| entry.get("out")).and_then(|out| out.get("solid").or_else(|| out.get("S"))).expect("extrude solid output");
        assert_eq!(solid.get("$schema").and_then(|v| v.as_str()), Some("geometry"));
        assert_eq!(solid.get("kind").and_then(|v| v.as_str()), Some("solid"));
    }

    #[test]
    fn hexagonal_mushroom_fixture_reports_extruded_solid_output() {
        // 🩹 Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
        // handcrafted DSL (`store::DocumentDsl`) — inlined the same flow-fixture JSON this test actually
        // parses (`FlowHost::parse_fixture_json`), decoupled from procedural's document format.
        let json = r#"{
  "schema": "flow.fixture",
  "camera": { "x": 94.75581571737445, "y": -97.50833134679668, "zoom": 1.7844325616011099 },
  "widgets": [
    { "kind": "inputSlider", "id": "height", "label": "Column Height", "value": 6.0, "min": 0.0, "max": 10.0, "step": 0.5, "unit": "m" },
    { "kind": "inputSlider", "id": "radius", "label": "Profile Radius", "value": 0.5, "min": 0.1, "max": 2.0, "step": 0.05, "unit": "m" },
    { "kind": "inputSlider", "id": "sides", "label": "Side Count", "value": 6.0, "min": 3.0, "max": 12.0, "step": 1.0 },
    { "kind": "neuron", "id": "profile", "neuronKind": "brep.curve.polygon", "params": {}, "input_ports": ["radius", "sides"], "preview": false },
    { "kind": "neuron", "id": "extrusion-axis", "neuronKind": "math.vector", "params": {}, "input_ports": ["x", "y", "z"], "preview": false },
    { "kind": "neuron", "id": "extrude", "neuronKind": "brep.solid.extrude", "params": {}, "input_ports": ["wire", "vector"], "preview": true },
    { "kind": "outputPreview", "id": "column-preview", "preview": {}, "expanded": [] }
  ],
  "synapses": [
    { "id": "e1", "from": "height", "to": "extrusion-axis", "fromPort": "number", "toPort": "z" },
    { "id": "e2", "from": "radius", "to": "profile", "fromPort": "number", "toPort": "radius" },
    { "id": "e3", "from": "sides", "to": "profile", "fromPort": "number", "toPort": "sides" },
    { "id": "e4", "from": "profile", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e5", "from": "extrusion-axis", "to": "extrude", "fromPort": "vector", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "column-preview", "fromPort": "solid", "toPort": "" }
  ],
  "layout": {
    "height": { "x": -197.1913555449187, "y": -102.70789997839545 },
    "radius": { "x": -156.03796288966, "y": -177.3373596163105 },
    "sides": { "x": -156.43467044109153, "y": -155.28679730672846 },
    "profile": { "x": -64.49671116929301, "y": -163.40310309861746 },
    "extrusion-axis": { "x": -65.26327021036892, "y": -116.45687403531778 },
    "extrude": { "x": 34.842068675720895, "y": -154.18083645790136 },
    "column-preview": { "x": 237.4197774877085, "y": -103.14518978933415 }
  }
}
"#;
        let fixture = FlowHost::parse_fixture_json(json).expect("fixture json");
        let mut host = FlowHost::from_fixture(fixture);
        host.set_neuron_kind_infos_json(&fixture_kind_infos_json());
        let eval_json = host.evaluate().expect("evaluate");
        let parsed: serde_json::Value = serde_json::from_str(&eval_json).expect("eval json");
        let solid = parsed.get("extrude").and_then(|entry| entry.get("out")).and_then(|out| out.get("solid").or_else(|| out.get("S"))).expect("extrude solid output");
        assert_eq!(solid.get("$schema").and_then(serde_json::Value::as_str), Some("geometry"));
        assert_eq!(solid.get("kind").and_then(serde_json::Value::as_str), Some("solid"));
        let handle = solid.get("handle").and_then(serde_json::Value::as_str).expect("solid handle");
        assert!(handle.starts_with("solid-"));
        let mesh: serde_json::Value = serde_json::from_str(&flow_module_brep::tessellate_geometry_json(handle, 0.05)).expect("solid mesh json");
        assert!(mesh.get("error").is_none(), "solid tessellation: {mesh}");
        assert!(mesh.get("position").and_then(serde_json::Value::as_array).is_some_and(|positions| !positions.is_empty()));
    }

    #[test]
    fn compiled_wire_literal_includes_operator_kinds() {
        let host = host_with_test_bridge();
        let text = host.compiled_wire_literal();
        assert!(text.contains("core.number"));
        assert!(text.contains("math.add"));
    }

    #[test]
    fn flow_fixture_to_form_spec_maps_input_widgets() {
        use crate::forms_bridge::flow_fixture_to_form_spec;
        let fixture = FlowFixture::default();
        let spec = flow_fixture_to_form_spec(&fixture);
        let kinds: Vec<&str> = spec.steps[0].blocks.iter().map(|question| question.kind.as_str()).collect();
        assert!(kinds.contains(&"slider"));
    }

    #[test]
    fn apply_generation_values_to_fixture_patches_slider_value() {
        use crate::forms_bridge::{apply_generation_values_to_fixture, flow_fixture_to_form_spec};
        let fixture = FlowFixture::default();
        let spec = flow_fixture_to_form_spec(&fixture);
        let slider_id = spec.steps[0].blocks.iter().find(|question| question.kind == "slider").map(|question| question.id.clone()).expect("slider question");
        let fixture_json = serde_json::to_string(&fixture).expect("fixture json");
        let mut values = serde_json::Map::new();
        values.insert(slider_id.clone(), serde_json::json!(8.0));
        let patched = apply_generation_values_to_fixture(&fixture_json, &values);
        let reparsed: serde_json::Value = serde_json::from_str(&patched).expect("patched json");
        let slider = reparsed.get("widgets").and_then(|widgets| widgets.as_array()).and_then(|widgets| widgets.iter().find(|widget| widget.get("id").and_then(|id| id.as_str()) == Some(slider_id.as_str()))).expect("slider widget");
        assert_eq!(slider.get("value").and_then(|value| value.as_f64()), Some(8.0));
    }
}
// #endregion 🔖Tests
