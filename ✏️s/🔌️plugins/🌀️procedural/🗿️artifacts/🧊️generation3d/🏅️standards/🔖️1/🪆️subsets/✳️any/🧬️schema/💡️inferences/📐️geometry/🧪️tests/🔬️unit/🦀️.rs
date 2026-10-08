use crate::standards::v1::subsets::any::io::text::snapshot::catalogue::catalogue;
//! 🧪️ Geometry inference laws: the engine evaluates documents to typed values, recomputes only what a change reaches,
//! localizes every refusal, and a stepped run equals the unbounded one.

use super::compute::{ReadyJob, WidgetJob, WidgetStep};
use super::engine::{CacheMode, GeometryEngine};
use super::inputs::WidgetInputs;
use super::registry::probe;
use super::value::{outputs, GeometryValue, WidgetEvaluation};
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead;
use semio_framework_artifact_flow_flow::neural::{Atom, Dictionary, Value as NeuralValue};
use semio_framework_artifact_flow_flow::{CameraJson, OrderedMap, OrderedSet};
use std::cell::Cell;

//#region 🧸️Fixtures
fn text_atom(text: &str) -> NeuralValue {
    NeuralValue::Atom(Atom::String(text.to_string()))
}

fn number_literal(value: f64) -> NeuralValue {
    NeuralValue::Dictionary(Dictionary::new().insert("$schema", text_atom("number")).insert("value", NeuralValue::Atom(Atom::Decimal(value))))
}

fn text_literal(value: &str) -> NeuralValue {
    NeuralValue::Dictionary(Dictionary::new().insert("$schema", text_atom("text")).insert("value", text_atom(value)))
}

fn boolean_literal(value: bool) -> NeuralValue {
    NeuralValue::Dictionary(Dictionary::new().insert("$schema", text_atom("boolean")).insert("value", NeuralValue::Atom(Atom::Boolean(value))))
}

fn neuron(id: &str, kind: &str, params: Vec<(&str, NeuralValue)>) -> Widget {
    Widget::Neuron { id: id.into(), neuron_kind: kind.into(), params: params.into_iter().fold(Dictionary::new(), |dictionary, (port, value)| dictionary.insert(port, value)), input_ports: vec![], output_ports: vec![], preview: true }
}

fn slider(id: &str, value: f64) -> Widget {
    Widget::InputSlider { id: id.into(), label: id.into(), value, min: 0.0, max: 100.0, step: 0.1 }
}

fn preview(id: &str) -> Widget {
    Widget::OutputPreview { id: id.into(), preview: Dictionary::new(), expanded: OrderedSet::new() }
}

fn wire(id: &str, from: &str, from_port: &str, to: &str, to_port: &str) -> SynapseSpec {
    SynapseSpec { id: id.into(), from: from.into(), to: to.into(), from_port: from_port.into(), to_port: to_port.into() }
}

fn document(widgets: Vec<Widget>, synapses: Vec<SynapseSpec>) -> Generation3dSnapshotRead {
    let mut snapshot = Generation3dSnapshot::default();
    let host = FlowHostSnapshot { schema: "flow.host_snapshot".into(), camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, widgets, synapses, layout: OrderedMap::new() };
    std::mem::replace(&mut snapshot.host_snapshot, host).retire_cold();
    Generation3dSnapshotRead::new(snapshot)
}

fn evaluate(read: Generation3dSnapshotRead) -> (GeometryEngine, BTreeMap<String, Arc<WidgetEvaluation>>) {
    let mut engine = GeometryEngine::new(std::sync::Arc::clone(crate::standards::v1::subsets::any::io::text::snapshot::catalogue::catalogue()), 1 << 24);
    engine.start("test".into(), read.into_inner(), CacheMode::Incremental, "test");
    assert!(engine.step(usize::MAX).expect("the run steps").done);
    let values = engine.evaluations().clone();
    (engine, values)
}

fn rerun(engine: &mut GeometryEngine, read: Generation3dSnapshotRead, digest: &str) -> BTreeMap<String, Arc<WidgetEvaluation>> {
    engine.start(digest.into(), read.into_inner(), CacheMode::Incremental, "test");
    assert!(engine.step(usize::MAX).expect("the run steps").done);
    engine.evaluations().clone()
}

fn number_of(values: &BTreeMap<String, Arc<WidgetEvaluation>>, widget: &str, port: &str) -> f64 {
    match values[widget].outputs.get(port) {
        Some(GeometryValue::Number(value)) => *value,
        other => panic!("{widget}.{port} is {other:?}"),
    }
}

fn box_chain(size: f64) -> Generation3dSnapshotRead {
    document(
        vec![slider("size", size), neuron("box", "brep.primitive.box", vec![]), preview("view")],
        vec![wire("e1", "size", "number", "box", "width"), wire("e2", "size", "number", "box", "depth"), wire("e3", "size", "number", "box", "height"), wire("e4", "box", "shape", "view", "")],
    )
}

fn solid_volume(shape: &semio_framework_3d::brep::engine::ShapeValue) -> (f64, f64) {
    let mesh = shape.tessellate(0.05).expect("a box tessellates");
    let corner = |index: u32| -> [f64; 3] { let at = index as usize * 3; [f64::from(mesh.positions[at]), f64::from(mesh.positions[at + 1]), f64::from(mesh.positions[at + 2])] };
    let signed: f64 = mesh.indices.chunks(3).map(|triangle| {
        let (a, b, c) = (corner(triangle[0]), corner(triangle[1]), corner(triangle[2]));
        (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0])) / 6.0
    }).sum();
    let points: Vec<parry3d::math::Point<parry3d::math::Real>> = mesh.positions.chunks(3).map(|at| parry3d::math::Point::new(at[0], at[1], at[2])).collect();
    let triangles: Vec<[u32; 3]> = mesh.indices.chunks(3).map(|triangle| [triangle[0], triangle[1], triangle[2]]).collect();
    (signed, f64::from(parry3d::mass_properties::MassProperties::from_trimesh(1.0, &points, &triangles).mass()))
}
//#endregion 🧸️Fixtures

//#region 🧪️Evaluation
#[test]
fn a_slider_driven_box_chain_evaluates_to_the_exact_solid_and_equals_the_cold_inference() {
    let read = box_chain(2.5);
    let cold = infer_geometry(&crate::test_serial::geometry_input(&read));
    let (engine, values) = evaluate(box_chain(2.5));
    assert_eq!(values, cold, "the engine run is the inference, value for value");
    assert_eq!(number_of(&values, "size", "number"), 2.5);
    let boxed = &values["box"];
    assert_eq!((boxed.fault.as_ref(), boxed.quality), (None, Quality::ExactAnalytic));
    let GeometryValue::Shape(shape) = &boxed.outputs["shape"] else { panic!("the box output is a shape") };
    let (signed, oracle) = solid_volume(shape);
    assert!((signed - 15.625).abs() < 1e-6 && (oracle - 15.625).abs() < 1e-4, "volume {signed} / parry3d {oracle} must equal 2.5^3");
    assert_eq!(values["view"].outputs, values["box"].outputs, "the preview passes the wired output through");
    assert_eq!(engine.totals().computed, 3, "slider, box and preview are the three widgets computed");
}

#[test]
fn the_plan_is_topological_in_document_order_and_cycle_members_trail_as_parentless_steps() {
    let read = document(
        vec![preview("view"), neuron("a", "math.add", vec![]), slider("s", 1.0), neuron("loop1", "math.add", vec![]), neuron("loop2", "math.add", vec![])],
        vec![wire("e1", "a", "result", "view", ""), wire("e2", "s", "number", "a", "a"), wire("e3", "loop1", "result", "loop2", "a"), wire("e4", "loop2", "result", "loop1", "a"), wire("e5", "ghost", "x", "a", "b")],
    );
    let plan = Generation3dGeometry::plan(&crate::test_serial::geometry_input(&read));
    let keys: Vec<&str> = plan.iter().map(|step| step.key.as_str()).collect();
    assert_eq!(keys, ["s", "a", "view", "loop1", "loop2"]);
    assert_eq!(plan[1].parents, ["s"], "a wire from a widget that does not exist is no parent");
    assert!(plan[3].parents.is_empty() && plan[4].parents.is_empty());
}

#[test]
fn a_wiring_cycle_faults_every_widget_on_or_downstream_of_it_in_both_languages() {
    let (_, values) = evaluate(document(
        vec![slider("s", 4.0), neuron("a", "math.add", vec![]), neuron("b", "math.add", vec![]), preview("view"), neuron("free", "math.add", vec![])],
        vec![wire("e1", "a", "result", "b", "a"), wire("e2", "b", "result", "a", "a"), wire("e3", "b", "result", "view", ""), wire("e4", "s", "number", "free", "a")],
    ));
    for id in ["a", "b"] {
        let fault = values[id].fault.as_ref().unwrap_or_else(|| panic!("{id} is on the cycle"));
        assert_eq!(fault.code, "generation3d.geometry.cycle");
        assert!(!fault.message.en.is_empty() && !fault.message.de.is_empty() && fault.message.en != fault.message.de);
        assert!(values[id].outputs.is_empty());
    }
    assert_eq!(values["view"].fault.as_ref().map(|fault| fault.code.as_str()), Some("generation3d.geometry.cycle"), "a preview of a faulted widget carries its fault");
    assert_eq!(number_of(&values, "free", "result"), 4.0, "a = 4, b defaults to 0");
}

#[test]
fn a_self_loop_is_a_cycle() {
    let (_, values) = evaluate(document(vec![neuron("a", "math.add", vec![])], vec![wire("e1", "a", "result", "a", "a")]));
    assert_eq!(values["a"].fault.as_ref().map(|fault| fault.code.as_str()), Some("generation3d.geometry.cycle"));
}
//#endregion 🧪️Evaluation

//#region 🧪️Incrementality
#[test]
fn changing_one_widget_recomputes_only_it_and_its_descendants() {
    let build = |size: f64, radius: f64| {
        document(
            vec![slider("size", size), slider("radius", radius), neuron("box", "brep.primitive.box", vec![]), neuron("ball", "brep.primitive.sphere", vec![]), neuron("sum", "math.add", vec![]), preview("box-view"), preview("ball-view")],
            vec![
                wire("e1", "size", "number", "box", "width"),
                wire("e2", "size", "number", "box", "depth"),
                wire("e3", "size", "number", "box", "height"),
                wire("e4", "radius", "number", "ball", "radius"),
                wire("e5", "size", "number", "sum", "a"),
                wire("e6", "radius", "number", "sum", "b"),
                wire("e7", "box", "shape", "box-view", ""),
                wire("e8", "ball", "shape", "ball-view", ""),
            ],
        )
    };
    let (mut engine, base) = evaluate(build(2.0, 1.0));
    assert_eq!((engine.totals().computed, engine.totals().hits), (7, 0));

    let again = rerun(&mut engine, build(2.0, 1.0), "same");
    assert_eq!(again, base);
    assert_eq!((engine.totals().computed, engine.totals().hits), (7, 7), "an unchanged document computes nothing");

    let resized = rerun(&mut engine, build(3.0, 1.0), "size");
    assert_eq!(engine.totals().computed - 7, 4, "size, box, sum and the box preview");
    assert_eq!(engine.totals().hits - 7, 3, "radius, ball and the ball preview are served from the cache");
    assert_eq!(number_of(&resized, "sum", "result"), 4.0);
    assert!(Arc::ptr_eq(&base["ball"], &resized["ball"]), "an untouched widget keeps its evaluation allocation");

    let before = engine.totals();
    let rounder = rerun(&mut engine, build(3.0, 1.5), "radius");
    assert_eq!(engine.totals().computed - before.computed, 4, "radius, ball, sum and the ball preview");
    assert_eq!(engine.totals().hits - before.hits, 3);
    assert_eq!(number_of(&rounder, "sum", "result"), 4.5);
}

#[test]
fn a_stored_literal_change_recomputes_its_widget_and_descendants_while_layout_and_camera_do_not() {
    let build = |width: f64| document(vec![neuron("box", "brep.primitive.box", vec![("width", number_literal(width))]), preview("view")], vec![wire("e1", "box", "shape", "view", "")]);
    let (mut engine, _) = evaluate(build(2.0));
    rerun(&mut engine, build(2.0), "layout");
    assert_eq!((engine.totals().computed, engine.totals().hits), (2, 2));
    rerun(&mut engine, build(5.0), "width");
    assert_eq!(engine.totals().computed, 4, "the box and its preview recompute");

    let read = build(5.0);
    let mut moved = (*read).clone();
    moved.host_snapshot.camera.zoom = 9.0;
    moved.host_snapshot.layout.insert("box".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 40.0, y: 7.0 });
    for step in Generation3dGeometry::plan(&crate::test_serial::geometry_input(&read)) {
        let calm = semio_framework_pack_json::to_json_string(&Generation3dGeometry::dep_input(&crate::test_serial::geometry_input(&read), &step.key, &step.parents));
        let busy = semio_framework_pack_json::to_json_string(&Generation3dGeometry::dep_input(&crate::test_serial::geometry_input(&moved), &step.key, &step.parents));
        assert_eq!(calm, busy, "{}: layout and camera are not dependencies", step.key);
    }
    moved.retire_cold();
    let (a, b) = (dependency_of(&build(2.0).host_snapshot, catalogue(), "box"), dependency_of(&build(5.0).host_snapshot, catalogue(), "box"));
    assert_ne!(semio_framework_pack_json::to_json_string(&a), semio_framework_pack_json::to_json_string(&b), "a stored literal is a dependency");
    assert!(a.kind_definition.is_some(), "the admitted typed definition is part of the dependency");
}

#[test]
fn bypass_neither_reads_nor_writes_the_cache_and_cold_clears_it_first() {
    let (mut engine, base) = evaluate(box_chain(2.0));
    let warm = engine.cache_bytes();
    assert!(warm > 0);
    engine.start("bypass".into(), box_chain(2.0).into_inner(), CacheMode::Bypass, "test");
    assert!(engine.step(usize::MAX).unwrap().done);
    assert_eq!((engine.totals().computed, engine.totals().hits, engine.cache_bytes()), (6, 0, warm), "bypass computes everything and leaves the cache as it was");
    assert_eq!(engine.evaluations(), &base);
    engine.start("cold".into(), box_chain(2.0).into_inner(), CacheMode::Cold, "test");
    assert!(engine.step(usize::MAX).unwrap().done);
    assert_eq!((engine.totals().computed, engine.totals().hits), (9, 0), "a cold run recomputes everything");
}

#[test]
fn a_zero_budget_engine_still_evaluates_correctly_without_caching() {
    let mut engine = GeometryEngine::new(std::sync::Arc::clone(crate::standards::v1::subsets::any::io::text::snapshot::catalogue::catalogue()), 0);
    engine.start("x".into(), box_chain(2.0).into_inner(), CacheMode::Incremental, "test");
    assert!(engine.step(usize::MAX).unwrap().done);
    assert_eq!(engine.cache_bytes(), 0);
    assert_eq!(engine.evaluations(), &infer_geometry(&crate::test_serial::geometry_input(&box_chain(2.0))));
}
//#endregion 🧪️Incrementality

//#region 🧪️Faults
#[test]
fn a_connection_of_the_wrong_type_is_a_localized_fault_that_stops_every_consumer() {
    let (_, values) = evaluate(document(
        vec![neuron("flag", "math.boolean", vec![("value", boolean_literal(true))]), neuron("box", "brep.primitive.box", vec![]), preview("view"), neuron("after", "math.add", vec![])],
        vec![wire("e1", "flag", "value", "box", "width"), wire("e2", "box", "shape", "view", ""), wire("e3", "box", "shape", "after", "a")],
    ));
    let kind = catalogue().kind("brep.primitive.box").expect("box is catalogued");
    let fault = values["box"].fault.as_ref().expect("a boolean is no length");
    assert_eq!((fault.code.as_str(), fault.port.as_deref()), ("generation3d.geometry.input-type", Some("width")));
    assert!(fault.message.en.contains(&kind.input("width").unwrap().label.en) && fault.message.de.contains(&kind.input("width").unwrap().label.de));
    assert_ne!(fault.message.en, fault.message.de);
    assert_eq!(values["view"].fault.as_ref(), Some(fault), "the preview shows the fault of its source");
    let upstream = values["after"].fault.as_ref().expect("a consumer of a faulted widget");
    assert_eq!((upstream.code.as_str(), upstream.port.as_deref()), ("generation3d.geometry.upstream", Some("a")), "a faulted source blocks its consumer before any type check");
}

#[test]
fn a_consumer_of_a_faulted_widget_is_blocked_with_an_upstream_fault_naming_the_source() {
    let (_, values) = evaluate(document(
        vec![neuron("bad", "brep.primitive.box", vec![("width", number_literal(-1.0))]), neuron("also", "brep.primitive.sphere", vec![])],
        vec![wire("e1", "bad", "shape", "also", "radius")],
    ));
    assert_eq!(values["bad"].fault.as_ref().map(|fault| fault.code.as_str()), Some("generation3d.geometry.input-range"));
    let blocked = values["also"].fault.as_ref().expect("blocked");
    assert_eq!((blocked.code.as_str(), blocked.port.as_deref()), ("generation3d.geometry.upstream", Some("radius")));
    assert!(blocked.message.en.contains("bad") && blocked.message.de.contains("bad"));
    assert_ne!(blocked.message.en, blocked.message.de);
}

#[test]
fn literals_defaults_ranges_options_and_unknown_wires_are_checked_in_both_languages() {
    let (_, values) = evaluate(document(
        vec![
            neuron("zero", "brep.primitive.sphere", vec![("radius", number_literal(0.0))]),
            neuron("wrong", "brep.primitive.sphere", vec![("radius", text_atom("three"))]),
            neuron("fine", "brep.primitive.sphere", vec![("radius", number_literal(2.0))]),
            neuron("default", "brep.primitive.sphere", vec![]),
            neuron("dangling", "brep.primitive.sphere", vec![]),
            neuron("unknown", "no.such.kind", vec![]),
            neuron("round", "math.round", vec![("mode", text_literal("sideways"))]),
        ],
        vec![wire("e1", "nobody", "x", "dangling", "radius")],
    ));
    let code = |id: &str| values[id].fault.as_ref().map(|fault| fault.code.clone());
    assert_eq!(code("zero").as_deref(), Some("generation3d.geometry.input-range"), "the radius is exclusive of zero");
    assert_eq!(code("wrong").as_deref(), Some("generation3d.geometry.input-literal"));
    assert_eq!(code("fine"), None);
    assert_eq!(code("default"), None, "the catalogue default radius is used");
    assert_eq!(code("dangling").as_deref(), Some("generation3d.geometry.input-missing"));
    assert_eq!(code("unknown").as_deref(), Some("generation3d.geometry.kind-unknown"));
    assert_eq!(code("round").as_deref(), Some("generation3d.geometry.input-option"));
    for (id, fault) in values.iter().filter_map(|(id, evaluation)| evaluation.fault.as_ref().map(|fault| (id, fault))) {
        assert!(!fault.message.en.trim().is_empty() && !fault.message.de.trim().is_empty() && fault.message.en != fault.message.de, "{id}: {fault}");
        assert!(fault.code.starts_with(FAULT_PREFIX), "{id}: {}", fault.code);
    }
}

#[test]
fn widgets_of_every_other_variant_provide_pass_through_or_refuse() {
    let (_, values) = evaluate(document(
        vec![
            slider("slide", 7.5),
            Widget::InputNote { id: "note".into(), text: "hello".into() },
            Widget::InputImage { id: "image".into(), src: "data:image/png;base64,AA==".into() },
            Widget::Variable { id: "var".into(), name: "width".into(), schema: "dictionary".into() },
            preview("view"),
            Widget::OutputAction { id: "act".into(), action: "go".into() },
            Widget::OutputExport { id: "out".into(), format: "svg".into() },
        ],
        vec![wire("e1", "slide", "number", "view", "")],
    ));
    assert_eq!(values["slide"].outputs["number"], GeometryValue::Number(7.5));
    assert_eq!(values["note"].outputs["text"], GeometryValue::Text("hello".into()));
    assert_eq!(values["image"].outputs["image"], GeometryValue::Text("data:image/png;base64,AA==".into()));
    assert_eq!(values["var"].outputs["value"], GeometryValue::Text("width".into()));
    assert_eq!(values["view"].outputs["number"], GeometryValue::Number(7.5));
    assert!(values["act"].outputs.is_empty() && values["act"].fault.is_none() && values["out"].outputs.is_empty());
}
//#endregion 🧪️Faults

//#region 🧪️Steps
thread_local! {
    static CANCELLED: Cell<usize> = const { Cell::new(0) };
}

struct Slow {
    remaining: usize,
    done: WidgetEvaluation,
}

impl WidgetJob for Slow {
    fn step(&mut self, fuel: usize) -> WidgetStep {
        self.remaining -= fuel.min(self.remaining);
        if self.remaining > 0 {
            return WidgetStep::Working { progress: 1.0 - self.remaining as f32 / 3.0 };
        }
        WidgetStep::Done(self.done.clone())
    }

    fn cancel(&mut self) {
        CANCELLED.with(|count| count.set(count.get() + 1));
    }
}

fn slow_number(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    match inputs.number("value") {
        Ok(value) => Box::new(Slow { remaining: 3, done: WidgetEvaluation::ok(outputs([("value", GeometryValue::Number(value))]), kind.quality) }),
        Err(fault) => Box::new(ReadyJob::new(WidgetEvaluation::faulted(fault, kind.quality))),
    }
}

fn slow_chain() -> Generation3dSnapshotRead {
    document(
        vec![slider("s", 5.0), neuron("n1", "math.number", vec![]), neuron("n2", "math.number", vec![]), neuron("n3", "math.number", vec![])],
        vec![wire("e1", "s", "number", "n1", "value"), wire("e2", "n1", "value", "n2", "value"), wire("e3", "n2", "value", "n3", "value")],
    )
}

#[test]
fn a_stepped_run_equals_the_unbounded_run_for_every_fuel_and_charges_each_widget_job_slice() {
    let _slow = probe::install("math.number", slow_number);
    let reference = infer_geometry(&crate::test_serial::geometry_input(&slow_chain()));
    assert_eq!(number_of(&reference, "n3", "value"), 5.0);
    for fuel in 1..=14 {
        let mut engine = GeometryEngine::new(std::sync::Arc::clone(crate::standards::v1::subsets::any::io::text::snapshot::catalogue::catalogue()), 1 << 20);
        engine.start("slow".into(), slow_chain().into_inner(), CacheMode::Incremental, "test");
        let (mut calls, mut spent) = (0, 0);
        loop {
            let step = engine.step(fuel).expect("a stepped run advances");
            calls += 1;
            spent += step.fuel_used;
            assert!(calls < 100);
            if step.done {
                break;
            }
        }
        assert_eq!(engine.evaluations(), &reference, "fuel {fuel}");
        if fuel == 1 {
            assert_eq!((calls, spent), (10, 10), "one unit for the slider, three one-unit slices for each job");
        }
    }
}

#[test]
fn cancelling_mid_job_cancels_the_widget_job_keeps_finished_widgets_and_a_new_run_resumes_from_the_cache() {
    let _slow = probe::install("math.number", slow_number);
    CANCELLED.with(|count| count.set(0));
    let mut engine = GeometryEngine::new(std::sync::Arc::clone(crate::standards::v1::subsets::any::io::text::snapshot::catalogue::catalogue()), 1 << 20);
    engine.start("slow".into(), slow_chain().into_inner(), CacheMode::Incremental, "test");
    for _ in 0..3 {
        assert!(!engine.step(1).unwrap().done);
    }
    assert_eq!(engine.progress(), (1, 4), "the slider is finished, n1 has run two of its three slices");
    assert!(!engine.cancel_run("someone-else"), "a cancellation id that does not name the run cancels nothing");
    assert!(engine.cancel_run("test"));
    assert_eq!(CANCELLED.with(Cell::get), 1, "the in-flight job is cancelled once");
    assert!(matches!(engine.step(1), Err(super::engine::EngineError::Inference(protocol::InferenceError::Cancelled))));
    assert_eq!(engine.evaluations().keys().collect::<Vec<_>>(), ["s"]);
    let computed = engine.totals().computed;
    engine.restart(CacheMode::Incremental, "test");
    assert!(engine.step(usize::MAX).unwrap().done);
    assert_eq!(engine.totals().computed - computed, 3, "only the three unfinished widgets compute again");
    assert_eq!(engine.evaluations(), &infer_geometry(&crate::test_serial::geometry_input(&slow_chain())));
}

fn wrong_outputs(kind: &Kind, _inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    Box::new(ReadyJob::new(WidgetEvaluation::ok(outputs([("value", GeometryValue::Boolean(true)), ("extra", GeometryValue::Number(1.0))]), kind.quality)))
}

#[test]
fn a_job_that_breaks_the_output_contract_is_turned_into_a_localized_fault() {
    let _wrong = probe::install("math.number", wrong_outputs);
    let (_, values) = evaluate(document(vec![neuron("n", "math.number", vec![])], vec![]));
    let fault = values["n"].fault.as_ref().expect("the contract is enforced");
    assert_eq!((fault.code.as_str(), fault.port.as_deref()), ("generation3d.geometry.output-contract", Some("value")));
    assert!(values["n"].outputs.is_empty() && fault.message.en != fault.message.de);
}
//#endregion 🧪️Steps

//#region 🧪️Record
#[test]
fn the_inference_record_summarises_every_widget_and_matches_the_engine() {
    use crate::standards::v1::subsets::any::schema::inferences::Generation3dInference;
    use protocol::Inference;
    let read = document(
        vec![slider("size", 2.0), neuron("box", "brep.primitive.box", vec![]), neuron("bad", "brep.primitive.sphere", vec![("radius", number_literal(-3.0))])],
        vec![wire("e1", "size", "number", "box", "width")],
    );
    let inferred = Generation3dInference::infer(&crate::test_serial::geometry_input(&read)).expect("the inference runs");
    assert_eq!(inferred.geometry.faulted, 1);
    assert_eq!(inferred.geometry.widgets["size"].outputs, [Generation3dOutputRecord { port: "number".into(), kind: "number".into(), detail: "2".into() }]);
    assert_eq!(inferred.geometry.widgets["box"].outputs[0].kind, "shape");
    assert_eq!(inferred.geometry.widgets["bad"].fault.as_ref().map(|fault| fault.code.as_str()), Some("generation3d.geometry.input-range"));
    assert_eq!(inferred.geometry, Generation3dGeometryRecord::of(&infer_geometry(&crate::test_serial::geometry_input(&read))));
    assert_eq!(inferred.topology.node_count, 3);
}
//#endregion 🧪️Record

#[test]
fn inference_reads_the_explicit_admitted_catalogue() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🗂️catalogue/🧫️fixtures/🧩️admitted-context/🔣️.json")).expect("admitted-context fixture");
    let kind_id = fixture["kind"].as_str().expect("kind identity");
    let mut definitions = catalogue().categories().iter().map(|file| (file.category.id.clone(), file.clone())).collect::<Vec<_>>();
    let kind = definitions.iter_mut().flat_map(|(_, file)| file.kinds.iter_mut()).find(|kind| kind.id == kind_id).expect("fixture kind");
    kind.quality = Quality::Approximate;
    let admitted = std::sync::Arc::new(crate::standards::v1::subsets::any::schema::catalogue::Catalogue::from_categories(definitions).expect("typed catalogue"));
    let snapshot = box_chain(2.0);
    let context = GeometryInput::new(&snapshot, std::sync::Arc::clone(&admitted));
    let custom = infer_geometry(&context);
    let builtin = infer_geometry(&crate::test_serial::geometry_input(&snapshot));
    assert_eq!(custom["box"].quality.to_value().as_str(), fixture["quality"].as_str());
    assert_eq!(builtin["box"].quality.to_value().as_str(), fixture["builtinQuality"].as_str());
    let mut engine = GeometryEngine::new(admitted, 1 << 20);
    engine.start("custom-catalogue".into(), snapshot, CacheMode::Incremental, "custom-catalogue");
    while !engine.step(1).expect("typed catalogue engine").done {}
    assert_eq!(engine.evaluations(), &custom);
    println!("[DEBUG] Inference and one-unit engine read the explicit admitted catalogue independently of the bundled JSON owner");
}
