mod semio_s_artifact_procedural_3d_model { pub use super::graph_ports::*; }
use std::collections::BTreeMap;
mod semio_framework_artifact_flow_flow {
    pub type SynapseSpec = Synapse;
    pub struct FlowHostSnapshot { pub synapses: Vec<Synapse> }
    pub struct Synapse { pub id: String, pub from: String, pub from_port: String, pub to: String, pub to_port: String }
}
mod semio_os_infinite_directed_presentation {
    pub use super::dag_wire::parse_dag_endpoint;
    pub struct DagHostSnapshot { pub nodes: Vec<Node>, pub edges: Vec<Edge> }
    pub struct Node {
        pub id: String, pub name: String, pub x: f64, pub y: f64, pub width: f64, pub height: f64,
        pub inputs: Vec<Port>, pub outputs: Vec<Port>,
    }
    impl Node { pub fn inputs(&self) -> &[Port] { &self.inputs } pub fn outputs(&self) -> &[Port] { &self.outputs } }
    pub struct Port { pub id: String, pub label: String, pub value_type: Option<String>, pub visible: bool }
    pub struct Edge { pub id: String, pub source: String, pub target: String }
}
mod semio_ui_declaration { pub mod ui {
    #[derive(Default)]
    pub struct NodeGraphPortRecord {
        pub id: String, pub code: Option<String>, pub label: Option<String>, pub value_type: Option<String>,
        pub abbreviation: Option<String>, pub full_name: Option<String>, pub artifact_kind: Option<String>,
    }
    #[derive(Default)]
    pub struct NodeGraphNodeRecord {
        pub id: String, pub label: Option<String>, pub x: f64, pub y: f64, pub width: f64, pub height: f64,
        pub inputs: Vec<NodeGraphPortRecord>, pub outputs: Vec<NodeGraphPortRecord>,
    }
    pub struct NodeGraphEdgeRecord {
        pub id: String, pub source_node_id: String, pub target_node_id: String,
        pub source_port_id: String, pub target_port_id: String, pub label: Option<String>,
    }
} }
use semio_ui_declaration::ui::{NodeGraphNodeRecord, NodeGraphPortRecord};
type GraphPortRecord = NodeGraphPortRecord;
#[derive(Default)]
struct IoPortSpec { id: String, label: String, code: String, abbreviation: String, full_name: String, artifact_kind: Option<String>, value_type: Option<String> }
impl IoPortSpec { fn simple(id: String, label: String) -> Self { Self { id, label, ..Default::default() } } }
struct Label(&'static str);
impl Label { fn as_str(&self) -> &'static str { self.0 } }
struct Generation3dLabels { graph_input_port: Label, graph_output_port: Label }
const CASES: &[(&str, &str, bool, &str, &str)] = &[CODEC_FIXTURE];
const INSTANCE_CASES: &[(&str, &str, bool, usize, &str)] = &[INSTANCE_FIXTURE];
const WIRE_CASES: &[(&str, &str, &str, &str, &str, &str, &str, &str)] = &[WIRE_FIXTURE];
#[test]
fn actual_directional_port_codec_round_trips_all_fixture_channels() {
    use graph_ports::*;
    let mut ids = std::collections::BTreeSet::new();
    for &(widget, channel, input, code, target) in CASES {
        assert_eq!(graph_port_code(channel, input), code);
        assert_eq!(graph_port_channel(code), channel);
        assert_eq!(graph_port_id(widget, channel, input), target);
        let parsed = parse_graph_port_id(target).unwrap();
        assert_eq!((parsed.widget, parsed.channel.as_ref(), parsed.input), (widget, channel, input));
        assert!(ids.insert(target));
    }
}
#[test]
fn actual_flow_and_wgpu_projections_keep_same_named_ports_and_wire_targets_separate() {
    use semio_os_infinite_directed_presentation::{DagHostSnapshot, Node, Port, Edge};
    use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Synapse};
    use graph_ports::graph_port_channel;
    let port = || Port { id: "vector".into(), label: "Vector".into(), value_type: Some("vector".into()), visible: true };
    let node = |id: &str| Node { id: id.into(), name: id.into(), x: 0., y: 0., width: 10., height: 10., inputs: vec![port()], outputs: vec![port()] };
    let dag = DagHostSnapshot { nodes: vec![node("source"), node("axis")], edges: vec![Edge { id: "wire".into(), source: "source@vector".into(), target: "axis@vector".into() }] };
    let flow = FlowHostSnapshot { synapses: vec![Synapse { id: "wire".into(), from: "source".into(), from_port: "vector".into(), to: "axis".into(), to_port: "vector".into() }] };
    let (nodes, edges) = dag_host_snapshot_to_workflow(&dag, &flow.synapses);
    let axis = &nodes[1];
    let input = &axis.inputs[0];
    let output = &axis.outputs[0];
    assert_eq!((&input.id[..], &output.id[..]), ("axis@!ivector", "axis@vector"));
    assert_eq!(input.label, output.label);
    assert_eq!(input.code, output.code);
    let edge = &edges[0];
    let wgpu_input = port_to_io(input);
    let wgpu_output = port_to_io(output);
    assert_eq!((&wgpu_input.id[..], &wgpu_output.id[..]), ("!ivector", "vector"));
    assert_eq!(edge.target_port_id, wgpu_input.id);
    assert_eq!(edge.source_port_id, wgpu_output.id);
    assert_eq!(graph_port_channel(&edge.target_port_id), "vector");
    assert_eq!(wgpu_input.code, "vector");
    assert_eq!(wgpu_input.label, wgpu_output.label);
    let labels = Generation3dLabels { graph_input_port: Label("Eingang"), graph_output_port: Label("Ausgang") };
    let outline: Vec<_> = node_ports(axis, &labels).into_iter().map(|(port, text)| (port.id.as_str(), text)).collect();
    assert_eq!(outline, [("axis@!ivector", "Eingang"), ("axis@vector", "Ausgang")]);
    use graph_ports::GraphTargetKind::{Edge as WireTarget, Handle, Node as WidgetTarget};
    let input_ids = [Handle.target(&input.id)];
    let output_ids = [Handle.target(&output.id)];
    let wire_ids = [WireTarget.target("wire")];
    let node_ids = [WidgetTarget.target("axis")];
    let inlet_marks = PreviewGraphTargets::from_ids(&input_ids);
    let outlet_marks = PreviewGraphTargets::from_ids(&output_ids);
    let wire_marks = PreviewGraphTargets::from_ids(&wire_ids);
    let node_marks = PreviewGraphTargets::from_ids(&node_ids);
    let inlet = preview_wire_hover(&flow, &inlet_marks);
    assert!(inlet.matches("source", "source@vector", false));
    assert!(!inlet.matches("axis", &input.id, true));
    let output_hover = preview_wire_hover(&flow, &outlet_marks);
    assert!(!output_hover.matches("source", "source@vector", false));
    let link = preview_wire_hover(&flow, &wire_marks);
    assert!(link.matches("source", "source@vector", false));
    assert!(link.matches("axis", &input.id, true));
    assert!(!link.matches("axis", &output.id, false));
    assert!(!link.matches("source", "source@!ivector", true));
    let incoming = PreviewChannelAddress::new("axis", "vector", 0, true);
    let outgoing = PreviewChannelAddress::new("axis", "vector", 0, false);
    assert_eq!((incoming.port.as_str(), outgoing.port.as_str()), (input.id.as_str(), output.id.as_str()));
    assert!(incoming.marked(&inlet_marks) && !outgoing.marked(&inlet_marks));
    assert!(outgoing.marked(&outlet_marks) && !incoming.marked(&outlet_marks));
    assert!(incoming.marked(&node_marks) && outgoing.marked(&node_marks));
    assert_eq!(incoming.label(), "← axis@vector");
}

#[test]
fn whole_dictionary_wire_hover_respects_both_endpoint_directions() {
    use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Synapse};
    let flow = FlowHostSnapshot { synapses: vec![Synapse { id: "wire".into(), from: "source".into(), from_port: "" .into(), to: "axis".into(), to_port: "".into() }] };
    let wire_ids = [graph_ports::GraphTargetKind::Edge.target("wire")];
    let hover = preview_wire_hover(&flow, &PreviewGraphTargets::from_ids(&wire_ids));
    assert!(hover.matches("source", "source@first", false));
    assert!(hover.matches("source", "source@second", false));
    assert!(hover.matches("axis", "axis@!ifirst", true));
    assert!(!hover.matches("axis", "axis@first", false));
    assert!(!hover.matches("source", "source@!ifirst", true));
    assert!(!hover.matches("third", "third@first", false));
    let inlet_ids = [graph_ports::GraphTargetKind::Handle.target("axis@!i")];
    let input = preview_wire_hover(&flow, &PreviewGraphTargets::from_ids(&inlet_ids));
    assert!(input.matches("source", "source@first", false));
    assert!(!input.matches("axis", "axis@!ifirst", true));
}

#[test]
fn actual_instance_address_recovers_delimiters_and_direction() {
    use graph_ports::*;
    for &(widget, channel, input, index, instance) in INSTANCE_CASES {
        let address = PreviewChannelAddress::new(widget, channel, index, input);
        assert_eq!(address.instance, instance);
        let parsed = parse_graph_instance_id(instance).unwrap();
        assert_eq!((parsed.port.widget, parsed.port.channel.as_ref(), parsed.port.input, parsed.index), (widget, channel, input, index));
    }
    for invalid in ["axis@!iport#0", "axis@port#in-0", "axis@port#01", "axis@hello%7e#0"] {
        assert!(parse_graph_instance_id(invalid).is_none(), "{invalid}");
    }
}

#[test]
fn resolved_wire_ports_use_the_authored_widget_scopes() {
    use semio_os_infinite_directed_presentation::{DagHostSnapshot, Edge};
    use semio_framework_artifact_flow_flow::Synapse;
    for &(source, raw_source, channel_source, code_source, target, raw_target, channel_target, code_target) in WIRE_CASES {
        assert_eq!(dag_wire::dag_endpoint(source, channel_source), raw_source);
        assert_eq!(dag_wire::dag_endpoint(target, channel_target), raw_target);
        let authored = Synapse { id: "wire".into(), from: source.into(), from_port: channel_source.into(), to: target.into(), to_port: channel_target.into() };
        let dag = DagHostSnapshot { nodes: vec![], edges: vec![Edge { id: "wire".into(), source: raw_source.into(), target: raw_target.into() }] };
        let (_, edges) = dag_host_snapshot_to_workflow(&dag, &[authored]);
        assert_eq!(edges.len(), 1);
        let edge = &edges[0];
        assert_eq!((&*edge.source_node_id, &*edge.source_port_id), (source, code_source));
        assert_eq!((&*edge.target_node_id, &*edge.target_port_id), (target, code_target));
    }
}

#[test]
fn exact_same_string_targets_remain_disjoint() {
    use graph_ports::GraphTargetKind;
    let rows = [COLLISION_FIXTURE];
    let raw = COLLISION_RAW;
    let mut unique = std::collections::BTreeSet::new();
    for (granularity, expected) in rows {
        let kind = GraphTargetKind::for_granularity(granularity).unwrap();
        assert_eq!(kind.target(raw), expected);
        assert_eq!(GraphTargetKind::parse(expected), Some((kind, raw)));
        assert!(unique.insert(expected));
    }
}
