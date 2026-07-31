#!/usr/bin/env python3
"""Transform flow/core/lib.rs to flow.document/v1 canonical model."""
import re
from pathlib import Path

SRC = Path("/tmp/flow_core_lib_orig.rs")
DST = Path("/Users/ueli/Documents/semio/flow/core/lib.rs")

content = SRC.read_text()

# --- 1. Replace Widget region through tree_from_fixture with Document region ---
old_widget_region = content[content.index("// #region 🔖️Widget"):content.index("fn tree_signature")]

NEW_DOCUMENT_REGION = r'''// #region 🔖️Document
/// 📏️ One named numeric field inside a stepper input chrome.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepperFieldSpec {
    pub key: String,
    #[serde(default)]
    pub value: f64,
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

fn default_stepper_schema() -> String {
    "vector".into()
}

fn default_stepper_step() -> f64 {
    0.1
}

fn default_stepper_fields_for_schema(schema: &str) -> Vec<StepperFieldSpec> {
    match schema {
        "vector" | "point" => vec![
            StepperFieldSpec { key: "x".into(), value: 0.0 },
            StepperFieldSpec { key: "y".into(), value: 0.0 },
            StepperFieldSpec { key: "z".into(), value: 0.0 },
        ],
        _ => vec![],
    }
}

fn effective_stepper_fields<'a>(schema: &str, fields: &'a [StepperFieldSpec]) -> Vec<StepperFieldSpec> {
    if fields.is_empty() {
        default_stepper_fields_for_schema(schema)
    } else {
        fields.to_vec()
    }
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

/// 📍️ Persisted node position on the canvas.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetLayout {
    pub x: f64,
    pub y: f64,
}

/// 🧾️ Serializable flow document with authoritative neural tree and strippable UI.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowDocumentV1 {
    pub schema: String,
    pub tree: Tree,
    pub ui: FlowUiV1,
}

/// 🖼️ GUI-only flow data that can be removed without destroying logic.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowUiV1 {
    pub camera: CameraJson,
    pub nodes: BTreeMap<String, FlowNodeGui>,
    #[serde(default)]
    pub previews: Vec<FlowPreviewGui>,
}

/// 🧩️ GUI-only node presentation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowNodeGui {
    pub layout: WidgetLayout,
    pub chrome: NodeChrome,
}

/// 🪟️ GUI-only node chrome.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum NodeChrome {
    Plain { #[serde(default = "default_neuron_preview")] preview: bool },
    Slider { min: f64, max: f64, step: f64, #[serde(default = "default_slider_value")] value: f64 },
    Stepper { schema: String, fields: Vec<StepperFieldSpec>, step: f64 },
    Note { #[serde(default)] text: String },
    Image { #[serde(default)] src: String },
    Variable { name: String, schema: String },
}

/// 👁️ GUI-only preview binding.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowPreviewGui {
    pub id: String,
    pub source: Option<FlowChannelRefV1>,
    pub mode: String,
    #[serde(default)] pub preview: Dictionary,
    #[serde(default)] pub expanded: BTreeSet<String>,
    #[serde(default)] pub layout: Option<WidgetLayout>,
}

/// 📡️ Serializable channel reference.
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

const FLOW_INPUT_PORTS_KEY: &str = "_flowInputPorts";
const FLOW_OUTPUT_PORTS_KEY: &str = "_flowOutputPorts";

impl Default for FlowDocumentV1 {
    fn default() -> Self {
        Self {
            schema: "flow.document/v1".into(),
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
            ui: FlowUiV1 {
                camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
                nodes: BTreeMap::from([
                    ("slider".into(), FlowNodeGui { layout: WidgetLayout { x: 0.0, y: 0.0 }, chrome: NodeChrome::Slider { min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP, value: 3.0 } }),
                    ("add".into(), FlowNodeGui { layout: WidgetLayout { x: 200.0, y: 0.0 }, chrome: NodeChrome::Plain { preview: true } }),
                    ("out_sum".into(), FlowNodeGui { layout: WidgetLayout { x: 400.0, y: 0.0 }, chrome: NodeChrome::Plain { preview: false } }),
                ]),
                previews: vec![FlowPreviewGui { id: "preview".into(), source: Some(FlowChannelRefV1 { neuron: "add".into(), channel: "sum".into() }), mode: "text".into(), preview: Dictionary::new(), expanded: BTreeSet::new(), layout: Some(WidgetLayout { x: 400.0, y: 0.0 }) }],
            },
        }
    }
}

fn document_synapse_specs(tree: &Tree) -> Vec<SynapseSpec> {
    tree.synapses.iter().map(|s| SynapseSpec { id: s.id.clone(), from: s.from.clone(), to: s.to.clone(), from_port: s.from_port.clone(), to_port: s.to_port.clone() }).collect()
}

fn find_neuron<'a>(tree: &'a Tree, id: &str) -> Option<&'a Neuron> {
    tree.neurons.iter().find(|n| n.id == id)
}

fn find_neuron_mut<'a>(tree: &'a mut Tree, id: &str) -> Option<&'a mut Neuron> {
    tree.neurons.iter_mut().find(|n| n.id == id)
}

fn boundary_channel(neuron: &Neuron) -> String {
    neuron.params.get("channel").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap_or(neuron.id.as_str()).to_string()
}

fn stored_port_ids(params: &Dictionary, key: &str) -> Vec<String> {
    let Some(dict) = params.get(key).and_then(|v| v.as_dictionary()) else {
        return vec![];
    };
    let mut ids: Vec<_> = dict.keys().cloned().collect();
    ids.sort();
    ids
}

fn set_stored_port_ids(params: Dictionary, key: &str, ports: &[String]) -> Dictionary {
    let mut list = Dictionary::new();
    for port in ports {
        list = list.insert(port, NeuralValue::Atom(Atom::String(port.clone())));
    }
    params.insert(key, NeuralValue::Dictionary(list))
}

fn neuron_input_port_ids(neuron: &Neuron, kind_infos: &HashMap<String, OperatorInfo>) -> Vec<String> {
    let stored = stored_port_ids(&neuron.params, FLOW_INPUT_PORTS_KEY);
    if !stored.is_empty() {
        return stored;
    }
    default_neuron_input_ports(&neuron.kind, &[], kind_infos)
}

fn neuron_output_port_ids(neuron: &Neuron, kind_infos: &HashMap<String, OperatorInfo>) -> Vec<String> {
    let stored = stored_port_ids(&neuron.params, FLOW_OUTPUT_PORTS_KEY);
    if !stored.is_empty() {
        return stored;
    }
    default_neuron_output_ports(&neuron.kind, &[], kind_infos)
}

fn neuron_preview_enabled(document: &FlowDocumentV1, neuron_id: &str) -> bool {
    document.ui.nodes.get(neuron_id).map(|node| match &node.chrome {
        NodeChrome::Plain { preview } => *preview,
        _ => true,
    }).unwrap_or(true)
}

fn chrome_for_neuron(neuron: &Neuron) -> NodeChrome {
    match neuron.kind.as_str() {
        INPUT_KIND => {
            let channel = boundary_channel(neuron);
            match channel.as_str() {
                "number" => NodeChrome::Slider { min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP, value: 3.0 },
                "text" => NodeChrome::Note { text: String::new() },
                "image" => NodeChrome::Image { src: String::new() },
                other if other == "vector" || other == "point" => NodeChrome::Stepper { schema: other.into(), fields: default_stepper_fields_for_schema(other), step: default_stepper_step() },
                name => NodeChrome::Variable { name: name.into(), schema: boundary_schema_from_params(&neuron.params) },
            }
        }
        OUTPUT_KIND => NodeChrome::Plain { preview: false },
        CLUSTER_KIND => NodeChrome::Plain { preview: true },
        _ => NodeChrome::Plain { preview: true },
    }
}

fn ensure_ui_node(document: &mut FlowDocumentV1, neuron_id: &str) {
    if document.ui.nodes.contains_key(neuron_id) {
        return;
    }
    let layout = WidgetLayout { x: 0.0, y: 0.0 };
    let chrome = find_neuron(&document.tree, neuron_id).map(chrome_for_neuron).unwrap_or(NodeChrome::Plain { preview: true });
    document.ui.nodes.insert(neuron_id.to_string(), FlowNodeGui { layout, chrome });
}

fn dedupe_document_neurons(document: &mut FlowDocumentV1) {
    let mut seen = BTreeSet::new();
    document.tree.neurons.retain(|neuron| seen.insert(neuron.id.clone()));
}

'''

content = content.replace(old_widget_region, NEW_DOCUMENT_REGION)

# Remove duplicate FlowFixtureV1 / FlowGuiV1 / old FlowDocumentV1 if still present in widget_label section
# widget_label starts after tree_signature - keep widget_* functions for now, rename in step 2

# --- 2. Global renames ---
renames = [
    ("FlowGuiV1", "FlowUiV1"),
    ("FlowFixtureV1", "FLOW_FIXTURE_PLACEHOLDER"),  # temp to avoid double replace
    ("FLOW_FIXTURE_PLACEHOLDER", "FlowDocumentV1"),
    ("from_fixture", "from_document"),
    ("replace_fixture", "replace_document"),
    ("parse_fixture_json", "parse_document_json"),
    ("pub fn fixture_json", "pub fn document_json"),
    ("dedupe_fixture_widgets", "dedupe_document_neurons"),
    ("self.fixture", "self.document"),
    ("host.fixture", "host.document"),
    ("&self.fixture", "&self.document"),
    ("&mut self.fixture", "&mut self.document"),
    ("tree_from_fixture", "REMOVE_tree_from_fixture"),
    ("WidgetDescriptor", "NodeDescriptor"),
    ("widget_from_descriptor", "node_from_descriptor"),
    ("widget_to_dag_node", "neuron_to_dag_node"),
    ("widget_io_ports", "neuron_io_ports"),
    ("widget_node_size", "neuron_node_size"),
    ("widget_display_meta", "neuron_display_meta"),
    ("widget_label", "neuron_label"),
    ("widget_operator_info", "neuron_operator_info"),
    ("widget_kind_info", "neuron_kind_info"),
    ("widget_has_output", "node_has_output"),
    ("widget_has_input", "node_has_input"),
    ("widget_id_for", "node_id_for"),
    ("sync_dag_display_from_widgets", "sync_dag_display_from_document"),
    ("flow.fixture/v1", "flow.document/v1"),
    ("loadFixtureJson", "loadDocumentJson"),
    ("fixtureJson", "documentJson"),
]

for old, new in renames:
    content = content.replace(old, new)

# Remove document() getter that conflicts - rename to exported_document
content = content.replace(
    "    pub fn document(&self) -> FlowDocumentV1 {\n        self.document.to_document()\n    }",
    "",
)

# Fix FlowHost struct field
content = re.sub(
    r"pub struct FlowHost \{\n    pub fixture: FlowDocumentV1,",
    "pub struct FlowHost {\n    pub document: FlowDocumentV1,",
    content,
)

# Fix FlowHistory
content = content.replace(
    "struct FlowHistory {\n    past: Vec<FlowDocumentV1>,\n    future: Vec<FlowDocumentV1>,\n    pending: Option<FlowDocumentV1>,\n}",
    "struct FlowHistory {\n    past: Vec<FlowDocumentV1>,\n    future: Vec<FlowDocumentV1>,\n    pending: Option<FlowDocumentV1>,\n}",
)

content = content.replace(
    "fn content_changed(a: &FlowDocumentV1, b: &FlowDocumentV1) -> bool {\n        a.widgets != b.widgets || a.synapses != b.synapses || a.layout != b.layout\n    }",
    "fn content_changed(a: &FlowDocumentV1, b: &FlowDocumentV1) -> bool {\n        a.tree != b.tree || a.ui.nodes != b.ui.nodes || a.ui.previews != b.ui.previews\n    }",
)

# Fix camera access
content = content.replace("self.document.camera", "self.document.ui.camera")
content = content.replace("host.document.camera", "host.document.ui.camera")
content = content.replace("a.document.camera", "a.ui.camera")  # might not exist
content = content.replace("pre.document.camera", "pre.ui.camera")
content = content.replace("prev.document.camera", "prev.ui.camera")
content = content.replace("next.document.camera", "next.ui.camera")
content = content.replace("camera_before = host.document.camera", "camera_before = host.document.ui.camera")
content = content.replace("self.document.camera = camera", "self.document.ui.camera = camera")
content = content.replace("prev.document.camera = camera", "prev.ui.camera = camera")
content = content.replace("next.document.camera = camera", "next.ui.camera = camera")

# Fix layout access
content = content.replace("self.document.layout", "self.document.ui.nodes")  # wrong - need manual fix later
content = content.replace("host.document.layout", "host.document.ui.nodes")

DST.write_text(content)
print("Phase 1 written, lines:", len(content.splitlines()))
