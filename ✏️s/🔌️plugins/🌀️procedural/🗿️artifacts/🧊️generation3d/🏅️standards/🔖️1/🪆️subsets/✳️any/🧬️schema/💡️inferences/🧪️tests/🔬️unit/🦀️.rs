use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use super::*;
use protocol::Inference;
use semio_framework_artifact_flow_flow::{FlowHostSnapshot, SynapseSpec, Widget};

//#region 🧸️Fixtures
/// 🧩️ Binds a fixture snapshot to the bundled catalogue and the native compute context.
fn geometry_input(snapshot: &Generation3dSnapshot) -> semio_s_artifact_procedural_3d_geometry::GeometryInput<'_> {
    semio_s_artifact_procedural_3d_geometry::GeometryInput::new(snapshot, semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue(), &semio_s_artifact_procedural_3d_io_geometry::NativeGeometryComputeContext)
}

fn number_literal(value: f64) -> semio_framework_artifact_flow_flow::neural::Value {
    use semio_framework_artifact_flow_flow::neural::{Atom, Dictionary, Value};
    Value::Dictionary(Dictionary::new().insert("$schema", Value::Atom(Atom::String("number".into()))).insert("value", Value::Atom(Atom::Decimal(value))))
}

fn neuron(id: &str, kind: &str, params: Vec<(&str, semio_framework_artifact_flow_flow::neural::Value)>) -> Widget {
    Widget::Neuron { id: id.into(), neuron_kind: kind.into(), params: params.into_iter().fold(semio_framework_artifact_flow_flow::neural::Dictionary::new(), |dictionary, (port, value)| dictionary.insert(port, value)), input_ports: vec![], output_ports: vec![], preview: true }
}

fn slider(id: &str, value: f64) -> Widget {
    Widget::InputSlider { id: id.into(), label: id.into(), value, min: 0.0, max: 100.0, step: 0.1 }
}

fn wire(id: &str, from: &str, from_port: &str, to: &str, to_port: &str) -> SynapseSpec {
    SynapseSpec { id: id.into(), from: from.into(), to: to.into(), from_port: from_port.into(), to_port: to_port.into() }
}

fn document(widgets: Vec<Widget>, synapses: Vec<SynapseSpec>) -> semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshotRead {
    let mut snapshot = Generation3dSnapshot::default();
    let host = FlowHostSnapshot { schema: "flow.host_snapshot".into(), camera: semio_framework_artifact_flow_flow::CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, widgets, synapses, layout: semio_framework_artifact_flow_flow::OrderedMap::new() };
    std::mem::replace(&mut snapshot.host_snapshot, host).retire_cold();
    semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshotRead::new(snapshot)
}

fn sample_snapshot() -> Generation3dSnapshot {
    let mut snapshot = Generation3dSnapshot::default();
    snapshot.host_snapshot = FlowHostSnapshot {
        schema: "flow.host_snapshot".into(),
        camera: semio_framework_artifact_flow_flow::CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![
            Widget::InputSlider { id: "a".into(), label: "A".into(), value: 1.0, min: 0.0, max: 10.0, step: 1.0 },
            Widget::Neuron { id: "b".into(), neuron_kind: "math.add".into(), params: Default::default(), input_ports: vec![], output_ports: vec![], preview: false },
            Widget::OutputPreview { id: "c".into(), preview: Default::default(), expanded: Default::default() },
        ],
        synapses: vec![
            SynapseSpec { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "value".into(), to_port: "a".into() },
            SynapseSpec { id: "s2".into(), from: "b".into(), to: "c".into(), from_port: "sum".into(), to_port: String::new() },
        ],
        layout: Default::default(),
    };
    snapshot
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    let snapshot = sample_snapshot();
    assert_eq!(Generation3dInference::infer(&geometry_input(&snapshot)).expect("valid materialized inference fixture"), Generation3dInference::infer(&geometry_input(&snapshot)).expect("valid materialized inference fixture"));
}

/// 💡️ LAW (totality on the identity element): the EMPTY document infers an empty topology — no
/// nodes, no edges, an empty order, zero depth, and trivially cycle-free.
///
/// This is deliberately not `infer(Default::default()) == Inference::default()`: neither side of
/// that equation holds. `Generation3dSnapshot::default()` is the three-widget demo graph
/// (`default_generation3d_snapshot`'s docstring), and `Generation3dTopology::default()` is the
/// DERIVED zero value, whose `cycle_free: false` contradicts the empty graph's real semantics
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn inference_of_the_empty_document_is_empty_and_trivially_cycle_free() {
    let empty = semio_s_artifact_procedural_3d_model::empty_generation3d_snapshot();
    assert_eq!(
        Generation3dInference::infer(&geometry_input(&empty)).expect("valid materialized inference fixture"),
        Generation3dInference { topology: Generation3dTopology { node_count: 0, edge_count: 0, topo_order: Vec::new(), depth: 0, cycle_free: true }, geometry: Default::default() }
    );
    empty.retire_cold();
}

#[test]
fn topology_matches_the_linear_chain() {
    let snapshot = sample_snapshot();
    let inferred = Generation3dInference::infer(&geometry_input(&snapshot)).expect("valid materialized inference fixture");
    assert_eq!(inferred.topology.node_count, 3);
    assert_eq!(inferred.topology.edge_count, 2);
    assert!(inferred.topology.cycle_free);
    assert_eq!(inferred.topology.depth, 2);
    assert_eq!(inferred.topology.topo_order, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
}
//#endregion 🧪️InferenceLaws

//#region 🧪️Record
#[test]
fn the_inference_record_summarises_every_widget_and_matches_the_engine() {
    let read = document(
        vec![slider("size", 2.0), neuron("box", "brep.primitive.box", vec![]), neuron("bad", "brep.primitive.sphere", vec![("radius", number_literal(-3.0))])],
        vec![wire("e1", "size", "number", "box", "width")],
    );
    let inferred = Generation3dInference::infer(&geometry_input(&read)).expect("the inference runs");
    assert_eq!(inferred.geometry.faulted, 1);
    assert_eq!(inferred.geometry.widgets["size"].outputs, [semio_s_artifact_procedural_3d_geometry::Generation3dOutputRecord { port: "number".into(), kind: "number".into(), detail: "2".into() }]);
    assert_eq!(inferred.geometry.widgets["box"].outputs[0].kind, "shape");
    assert_eq!(inferred.geometry.widgets["bad"].fault.as_ref().map(|fault| fault.code.as_str()), Some("generation3d.geometry.input-range"));
    assert_eq!(inferred.geometry, semio_s_artifact_procedural_3d_geometry::Generation3dGeometryRecord::of(&semio_s_artifact_procedural_3d_geometry::infer_geometry(&geometry_input(&read))));
    assert_eq!(inferred.topology.node_count, 3);
}
//#endregion 🧪️Record
