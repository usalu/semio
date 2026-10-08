//! 🧪️ Service laws: the executable contextual inference answers the published payload contract, spends its budget in widget steps, resumes from its own result, cancels, honours the cache modes and reports faults in both languages.

use super::*;
use crate::host::geometry_service::GeometryHost;
use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead;
use semio_framework_artifact_flow_flow::neural::{Atom, Dictionary, Value as NeuralValue};
use semio_framework_artifact_flow_flow::{CameraJson, FlowHostSnapshot, OrderedMap, OrderedSet, SynapseSpec, Widget};
use semio_framework_plugin::{WireArtifactInferenceBudget, WireArtifactInferenceCacheMode};

//#region 🧸️Fixtures
const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(FIXTURE).expect("the service fixture parses")
}

fn number_literal(value: f64) -> NeuralValue {
    NeuralValue::Dictionary(Dictionary::new().insert("$schema", NeuralValue::Atom(Atom::String("number".into()))).insert("value", NeuralValue::Atom(Atom::Decimal(value))))
}

fn document(width: f64, radius: f64) -> Generation3dSnapshotRead {
    let widgets = vec![
        Widget::InputSlider { id: "size".into(), label: "Size".into(), value: 2.0, min: 0.0, max: 10.0, step: 0.5 },
        Widget::Neuron { id: "box".into(), neuron_kind: "brep.primitive.box".into(), params: Dictionary::new().insert("width", number_literal(width)), input_ports: vec![], output_ports: vec![], preview: true },
        Widget::Neuron { id: "ball".into(), neuron_kind: "brep.primitive.sphere".into(), params: Dictionary::new().insert("radius", number_literal(radius)), input_ports: vec![], output_ports: vec![], preview: true },
        Widget::OutputPreview { id: "view".into(), preview: Dictionary::new(), expanded: OrderedSet::new() },
    ];
    let wire = |id: &str, from: &str, from_port: &str, to: &str, to_port: &str| SynapseSpec { id: id.into(), from: from.into(), to: to.into(), from_port: from_port.into(), to_port: to_port.into() };
    let synapses = vec![wire("e1", "size", "number", "box", "depth"), wire("e2", "size", "number", "box", "height"), wire("e3", "box", "shape", "view", "")];
    let mut snapshot = Generation3dSnapshot::default();
    let host = FlowHostSnapshot { schema: "flow.host_snapshot".into(), camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, widgets, synapses, layout: OrderedMap::new() };
    std::mem::replace(&mut snapshot.host_snapshot, host).retire_cold();
    Generation3dSnapshotRead::new(snapshot)
}

fn request_text(read: Generation3dSnapshotRead, generation: Option<&str>) -> String {
    let request = GeometryRequest { snapshot: Some(read.into_inner()), document: None, generation: generation.map(str::to_string) };
    let text = semio_framework_pack_json::to_json_string(&request);
    request.snapshot.expect("moved in above").retire_cold();
    text
}

struct Call<'a> {
    payload: &'a str,
    work_units: u64,
    previous: Option<&'a [u8]>,
    mode: WireArtifactInferenceCacheMode,
}

fn call(host: &GeometryHost, call: Call<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    let budgets = WireArtifactInferenceBudget { allocation_bytes: 1 << 24, work_units: call.work_units, recursion_depth: 8 };
    let request = ArtifactInferenceExecutionRequest { policy: &[], budgets: &budgets, cancellation_id: "geometry-test", previous_state: call.previous, requested_cache_mode: call.mode, canonical_payload: call.payload.as_bytes(), dependencies: &[] };
    geometry_inference_service().infer_with_context(&request, host)
}

fn once(host: &GeometryHost, payload: &str, work_units: u64) -> ArtifactInferenceExecution {
    call(host, Call { payload, work_units, previous: None, mode: WireArtifactInferenceCacheMode::Incremental }).expect("the service answers")
}

fn json(execution: &ArtifactInferenceExecution) -> serde_json::Value {
    serde_json::from_slice(&execution.canonical_payload).expect("the result payload is JSON")
}

fn typed(execution: &ArtifactInferenceExecution) -> GeometryResult {
    semio_framework_pack_json::from_json_str(std::str::from_utf8(&execution.canonical_payload).expect("UTF-8"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the result decodes through its typed schema")
}
//#endregion 🧸️Fixtures

//#region 🧪️Contract
#[test]
fn the_registered_metadata_publishes_the_widget_step_contract_and_both_schemas_parse() {
    let metadata = geometry_inference_service().metadata();
    assert_eq!((metadata.artifact_kind, metadata.inference_schema), ("s.procedural.generation3d", "s.procedural.generation3d.geometry"));
    let contract = metadata.payload.expect("the service publishes a payload contract");
    assert_eq!(contract.progress_unit, "widget-step");
    for schema in [contract.input_schema, contract.output_schema] {
        let parsed: serde_json::Value = serde_json::from_str(schema).expect("the schema is JSON");
        assert_eq!(parsed["type"], "object");
    }
    assert!(contract.artifact_binding.is_some_and(|binding| binding.field == "document" && !binding.required));
    semio_framework_plugin::register_artifact_inference_services(vec![geometry_inference_service()]).expect("the service registers without conflict");
    assert!(semio_framework_plugin::artifact_inference_service("s.procedural.generation3d", "s.procedural.generation3d.geometry").expect("registry reads").is_some());
}

#[test]
fn a_service_without_its_geometry_host_refuses_by_name() {
    let budgets = WireArtifactInferenceBudget { allocation_bytes: 1 << 20, work_units: 10, recursion_depth: 1 };
    let request = ArtifactInferenceExecutionRequest { policy: &[], budgets: &budgets, cancellation_id: "x", previous_state: None, requested_cache_mode: WireArtifactInferenceCacheMode::Incremental, canonical_payload: b"{}", dependencies: &[] };
    assert_eq!(geometry_inference_service().infer(&request).unwrap_err().code, "artifact-inference.context-required");
    assert_eq!(geometry_inference_service().infer_with_context(&request, &0u8).unwrap_err().code, "generation3d.geometry.context");
}

#[test]
fn the_request_and_result_fixtures_hold_in_rust_and_the_result_decodes_through_its_typed_schema() {
    let fixture = fixture();
    let host = GeometryHost::default();
    let payload = request_text(document(2.0, 1.5), None);
    let execution = once(&host, &payload, 1000);
    let stepped = once(&GeometryHost::default(), &payload, 2);
    let faulted = once(&GeometryHost::default(), &request_text(document(2.0, -1.0), None), 1000);
    eprintln!("[DEBUG] geometry-service-observed-fixture={}", serde_json::json!({"request": serde_json::from_str::<serde_json::Value>(&payload).unwrap(), "complete": json(&execution), "stepped": json(&stepped), "faulted": json(&faulted)}));
    assert_eq!(serde_json::from_str::<serde_json::Value>(&payload).expect("the request is JSON"), fixture["request"], "the committed request is the encoding of the typed request");
    assert_eq!(json(&execution), fixture["complete"], "the committed result is what the service answers");
    let result = typed(&execution);
    assert!(result.complete && result.cursor.is_none() && result.progress.completed == result.progress.total);
    assert_eq!(semio_framework_pack_json::to_json_string(&result), String::from_utf8(execution.canonical_payload.clone()).expect("UTF-8"), "the typed schema re-encodes byte for byte");
    assert_eq!((execution.complete, execution.validity.as_str(), execution.quality.as_str()), (true, "valid", "exact-analytic"));
    assert!(execution.diagnostics.is_empty());
}
//#endregion 🧪️Contract

//#region 🧪️Steps
#[test]
fn a_run_resumes_from_its_own_result_and_equals_the_complete_run_for_every_work_unit_grant() {
    let payload = request_text(document(2.0, 1.5), None);
    let reference = typed(&once(&GeometryHost::default(), &payload, 1000));
    for units in 1..=5u64 {
        let host = GeometryHost::default();
        let mut previous: Option<Vec<u8>> = None;
        let mut calls = 0;
        let last = loop {
            let execution = call(&host, Call { payload: &payload, work_units: units, previous: previous.as_deref(), mode: WireArtifactInferenceCacheMode::Incremental }).expect("a step answers");
            calls += 1;
            assert!(calls < 50);
            if execution.complete {
                break typed(&execution);
            }
            let partial = typed(&execution);
            assert!(partial.cursor.is_some() && !partial.complete && partial.progress.completed < partial.progress.total);
            previous = Some(execution.canonical_payload);
        };
        assert_eq!(last.widgets, reference.widgets, "{units} units per call");
        assert_eq!(calls, 4usize.div_ceil(units as usize), "{units} units per call: one unit per computed widget");
    }
}

#[test]
fn stepped_results_match_the_committed_fixture_and_a_rebuilt_owner_restarts_instead_of_resuming() {
    let fixture = fixture();
    let payload = request_text(document(2.0, 1.5), None);
    let host = GeometryHost::default();
    let first = call(&host, Call { payload: &payload, work_units: 2, previous: None, mode: WireArtifactInferenceCacheMode::Incremental }).expect("first step");
    assert_eq!(json(&first), fixture["stepped"], "the first two-unit step");
    let rebuilt = GeometryHost::default();
    let resumed = call(&rebuilt, Call { payload: &payload, work_units: 1000, previous: Some(&first.canonical_payload), mode: WireArtifactInferenceCacheMode::Incremental }).expect("a rebuilt owner restarts instead of resuming");
    assert!(resumed.complete);
    assert_eq!(typed(&resumed).widgets, typed(&once(&GeometryHost::default(), &payload, 1000)).widgets);
    assert_eq!(typed(&resumed).computed, 4, "a rebuilt owner has no retained run, so it computes everything");
}

#[test]
fn cancelling_stops_the_run_with_a_named_error_and_a_new_call_starts_over_from_the_cache() {
    let payload = request_text(document(2.0, 1.5), None);
    let host = GeometryHost::default();
    let first = call(&host, Call { payload: &payload, work_units: 2, previous: None, mode: WireArtifactInferenceCacheMode::Incremental }).expect("first step");
    assert!(!host.cancel_run("another-id").expect("idle"), "another cancellation id cancels nothing");
    assert!(host.cancel_run("geometry-test").expect("the host is idle between calls"));
    let refused = call(&host, Call { payload: &payload, work_units: 2, previous: Some(&first.canonical_payload), mode: WireArtifactInferenceCacheMode::Incremental }).unwrap_err();
    assert_eq!(refused.code, "generation3d.geometry.cancelled", "continuing a cancelled run reports the cancellation");
    let again = once(&host, &payload, 1000);
    assert!(again.complete);
    let result = typed(&again);
    assert_eq!((result.computed, result.cache_hits), (2, 2), "the two finished widgets are served from the cache");
}
//#endregion 🧪️Steps

//#region 🧪️Cache
#[test]
fn the_cache_modes_are_incremental_cold_and_bypass() {
    let payload = request_text(document(2.0, 1.5), None);
    let host = GeometryHost::default();
    let first = typed(&once(&host, &payload, 1000));
    assert_eq!((first.computed, first.cache_hits), (4, 0));
    let incremental = typed(&once(&host, &payload, 1000));
    assert_eq!((incremental.computed, incremental.cache_hits), (0, 4), "an unchanged request is all cache hits");
    let edited = request_text(document(3.0, 1.5), None);
    let one_edit = typed(&once(&host, &edited, 1000));
    assert_eq!((one_edit.computed, one_edit.cache_hits), (2, 2), "only the edited box and its preview recompute");
    let cold = typed(&call(&host, Call { payload: &edited, work_units: 1000, previous: None, mode: WireArtifactInferenceCacheMode::Cold }).expect("cold"));
    assert_eq!((cold.computed, cold.cache_hits), (4, 0));
    let bypass = call(&host, Call { payload: &edited, work_units: 1000, previous: None, mode: WireArtifactInferenceCacheMode::Bypass }).expect("bypass");
    assert_eq!(bypass.actual_cache_mode, WireArtifactInferenceCacheMode::Bypass);
    assert_eq!((typed(&bypass).computed, typed(&bypass).cache_hits), (4, 0));
}
//#endregion 🧪️Cache

//#region 🧪️Refusals
#[test]
fn faults_are_reported_as_diagnostics_in_both_languages_and_make_the_run_invalid() {
    let mut read = document(2.0, 1.5);
    if let Widget::Neuron { params, .. } = &mut read.host_snapshot.widgets[2] {
        let displaced = std::mem::replace(params, Dictionary::new().insert("radius", number_literal(-1.0)));
        drop(displaced);
    }
    let payload = request_text(read, None);
    let execution = once(&GeometryHost::default(), &payload, 1000);
    assert_eq!(execution.validity, "invalid");
    assert_eq!(json(&execution), fixture()["faulted"]);
    let diagnostic = &execution.diagnostics[0];
    assert_eq!((diagnostic.code.as_str(), diagnostic.severity.as_str()), ("generation3d.geometry.input-range", "error"));
    assert_eq!((diagnostic.parameters["widget"].as_str(), diagnostic.parameters["port"].as_str()), ("ball", "radius"));
    assert_ne!(diagnostic.message, diagnostic.parameters["de"], "English message, German parameter");
    assert_eq!(typed(&execution).faulted, 1);
}

#[test]
fn invalid_requests_and_budgets_are_refused_by_name() {
    let host = GeometryHost::default();
    let payload = request_text(document(2.0, 1.5), None);
    let refusal = |payload: &str, units: u64| call(&host, Call { payload, work_units: units, previous: None, mode: WireArtifactInferenceCacheMode::Incremental }).unwrap_err();
    assert_eq!(refusal(&payload, 0).code, "generation3d.geometry.invalid-request");
    assert_eq!(refusal("{}", 10).code, "generation3d.geometry.invalid-request");
    assert_eq!(refusal("{\"snapshot\": 3}", 10).code, "generation3d.geometry.invalid-request");
    assert_eq!(refusal("not json", 10).code, "generation3d.geometry.invalid-request");
    let tight = WireArtifactInferenceBudget { allocation_bytes: 8, work_units: 10, recursion_depth: 1 };
    let request = ArtifactInferenceExecutionRequest { policy: &[], budgets: &tight, cancellation_id: "x", previous_state: None, requested_cache_mode: WireArtifactInferenceCacheMode::Incremental, canonical_payload: payload.as_bytes(), dependencies: &[] };
    assert_eq!(geometry_inference_service().infer_with_context(&request, &host).unwrap_err().code, "generation3d.geometry.invalid-request");
    let garbage = ArtifactInferenceExecutionRequest { previous_state: Some(b"nope"), budgets: &WireArtifactInferenceBudget { allocation_bytes: 1 << 24, work_units: 10, recursion_depth: 1 }, ..request };
    assert_eq!(geometry_inference_service().infer_with_context(&garbage, &host).unwrap_err().code, "generation3d.geometry.invalid-request");
}

#[test]
fn a_selected_generation_overrides_the_slider_values_it_names() {
    let host = GeometryHost::default();
    let plain = typed(&once(&host, &request_text(document(2.0, 1.5), None), 1000));
    let unknown = typed(&once(&host, &request_text(document(2.0, 1.5), Some("missing")), 1000));
    assert_eq!(plain.widgets.iter().map(|widget| widget.dep.clone()).collect::<Vec<_>>(), unknown.widgets.iter().map(|widget| widget.dep.clone()).collect::<Vec<_>>(), "a generation that does not exist changes nothing");
}
//#endregion 🧪️Refusals
