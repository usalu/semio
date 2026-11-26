//! 🧪️ Service laws: the executable contextual inference answers the published payload contract, spends its budget in widget steps, resumes from its own result, cancels, honours the cache modes and reports faults in both languages.

use super::*;
use crate::GeometryHost as NativeGeometryHost;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshotRead;
use semio_framework_artifact_flow_flow::neural::{Atom, Dictionary, Value as NeuralValue};
use semio_framework_artifact_flow_flow::{CameraJson, FlowHostSnapshot, OrderedMap, OrderedSet, SynapseSpec, Widget};
use semio_plugin::{WireArtifactInferenceBudget, WireArtifactInferenceCacheMode};
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
use semio_framework_value::{RetainedCloneGrant,retirement::{RetireOwned,controlled::ControlledRetirement}};
use std::{rc::Rc,cell::RefCell,mem::ManuallyDrop};

fn original_clock()->Option<u64>{Some(1)}
fn original_stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<28,maximum_copy_bytes:1<<28,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,maximum_depth:256}}
struct FixtureCaller{caller:Option<OriginalJobFixtureCaller>}
impl Drop for FixtureCaller {
    fn drop(&mut self) {
        if let Some(caller) = self.caller.take() {
            caller.retire(original_stock());
        }
    }
}
struct OriginalOracle<T:RetireOwned>{value:ManuallyDrop<Option<T>>,caller:Rc<RefCell<FixtureCaller>>}
impl<T:RetireOwned>std::ops::Deref for OriginalOracle<T>{type Target=T;fn deref(&self)->&T{self.value.as_ref().unwrap()}}
impl<T:RetireOwned+std::fmt::Debug>std::fmt::Debug for OriginalOracle<T>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{std::fmt::Debug::fmt(std::ops::Deref::deref(self),f)}}
impl<T:RetireOwned>Drop for OriginalOracle<T>{fn drop(&mut self){let mut owner=ControlledRetirement::new(self.value.take().unwrap()).ok().unwrap();let mut source=self.caller.borrow_mut();close_owner(&mut owner,source.caller.as_mut().unwrap());}}
fn close_owner<T: RetireOwned>(owner: &mut ControlledRetirement<T>, caller: &mut OriginalJobFixtureCaller) {
    for _ in 0..100_000 {
        if owner.terminal_is_empty() { return; }
        caller.turn(original_stock(), |cx| cx.with_original_retirement(|cx| {
            let body = cx.retained_grant().maximum_copy_bytes;
            let demand = owner.selected_demand(body)?;
            semio_framework_job::close_original_effect_step(cx, demand, |effect| owner.step(body, effect))?;
            Ok(())
        })).unwrap();
    }
    assert!(owner.terminal_is_empty());
}
struct GeometryHost{host:NativeGeometryHost,caller:Rc<RefCell<FixtureCaller>>}
impl Default for GeometryHost {
    fn default() -> Self {
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(17), Generation(9), original_stock(), original_clock, 1_000_000_000, 64_000_000);
        caller.retain_dispatch().expect("geometry service retains the initiating dispatch across pending turns");
        Self { host: NativeGeometryHost::default(), caller: Rc::new(RefCell::new(FixtureCaller { caller: Some(caller) })) }
    }
}
impl std::ops::Deref for GeometryHost{type Target=NativeGeometryHost;fn deref(&self)->&Self::Target{&self.host}}
impl GeometryHost{
 fn infer(&self,request:&ArtifactInferenceExecutionRequest<'_>,context:Option<&dyn std::any::Any>)->Result<OriginalOracle<ArtifactInferenceExecution>,OriginalOracle<ArtifactInferenceExecutionError>>{let mut source=self.caller.borrow_mut();let caller=source.caller.as_mut().unwrap();for _ in 0..100000{let mut result=None;caller.refreshed_turn(request.retained,|cx|cx.with_original_work_loan(request.budgets.work_units,cx.deadline_us(),|cx|{let before=cx.retained_progress();let outcome=match context{Some(context)=>geometry_inference_service().infer_with_context(request,context,cx),None=>geometry_inference_service().infer(request,cx)};if let Ok(step)=&outcome{assert_eq!(step.retained_progress,receipt_since(before,cx.retained_progress()),"service reports the actual paid effects");}result=Some(outcome);Ok(())})).unwrap();match result.unwrap(){Ok(step)=>{if let Some(execution)=step.execution{return Ok(OriginalOracle{value:ManuallyDrop::new(Some(execution)),caller:Rc::clone(&self.caller)})}},Err(error)=>return Err(OriginalOracle{value:ManuallyDrop::new(Some(error)),caller:Rc::clone(&self.caller)})}}panic!("actual geometry service did not produce an execution or refusal")}
 fn close(&mut self) {
    let mut source = self.caller.borrow_mut();
    let caller = source.caller.as_mut().unwrap();
    for _ in 0..100_000 {
        if self.host.terminal_is_empty() { return; }
        let demand = self.host.retirement_demands(1 << 20).expect("geometry quotes its next owner");
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let failure = caller.refreshed_turn(grant, |cx| cx.with_original_retirement(|cx| {
            let before = cx.retained_progress();
            match self.host.close_original_step(cx) {
                Ok(step) => {
                    assert_eq!(step.progress(), Some(receipt_since(before,cx.retained_progress())));
                    Ok(None)
                }
                Err(failure) => Ok(Some(semio_framework_job::OriginalTurnFailure::from(failure))),
            }
        })).unwrap();
        if let Some(original) = failure {
            let mut owner = ControlledRetirement::new(original).ok().unwrap();
            close_owner(&mut owner,caller);
            panic!("geometry lifecycle refused its initiating caller");
        }
    }
    assert!(self.host.terminal_is_empty(), "geometry close exhausted its quoted turns");
 }
}
impl Drop for GeometryHost{fn drop(&mut self){self.close();}}

//#region 🧸️Fixtures
const FIXTURE: &str = include_str!("../🧫️fixtures/🔣️.json");

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

fn call(host: &GeometryHost, call: Call<'_>) -> Result<OriginalOracle<ArtifactInferenceExecution>, OriginalOracle<ArtifactInferenceExecutionError>> {
    let budgets = WireArtifactInferenceBudget { allocation_bytes: 1 << 24, work_units: call.work_units, recursion_depth: 8 };
    let request = ArtifactInferenceExecutionRequest { operation:17,generation:9,cancelled:false, retained:semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128}, policy: &[], budgets: &budgets, cancellation_id: "geometry-test", previous_state: call.previous, requested_cache_mode: call.mode, canonical_payload: call.payload.as_bytes(), dependencies: &[] };
    host.infer(&request,Some(&host.host))
}

fn once(host: &GeometryHost, payload: &str, work_units: u64) -> OriginalOracle<ArtifactInferenceExecution> {
    call(host, Call { payload, work_units, previous: None, mode: WireArtifactInferenceCacheMode::Incremental }).expect("the service answers")
}

fn completed_document(host: &GeometryHost, payload: &str, work_units: u64) -> (serde_json::Value, OriginalOracle<ArtifactInferenceExecution>) {
    let mut previous = None;
    let mut first = None;
    for _ in 0..100 {
        let execution = call(host, Call { payload, work_units, previous: previous.as_deref(), mode: WireArtifactInferenceCacheMode::Incremental }).expect("the service publishes a funded checkpoint");
        first.get_or_insert_with(|| json(&execution));
        if execution.complete { return (first.unwrap(), execution); }
        previous = execution.canonical_payload.clone();
    }
    panic!("the geometry service never published a terminal result");
}

fn run_updates(host: &GeometryHost, payload: &str, units: u64, mode: WireArtifactInferenceCacheMode) -> ((u32,u32), OriginalOracle<ArtifactInferenceExecution>) {
    let mut previous = None;
    let mut counts = (0,0);
    let mut completed = 0;
    for _ in 0..100 {
        let step = call(host, Call { payload, work_units: units, previous: previous.as_deref(), mode: mode.clone() }).expect("a funded geometry step");
        let result = typed(&step);
        counts.0 += result.computed;
        counts.1 += result.cache_hits;
        assert!(result.progress.completed >= completed);
        completed = result.progress.completed;
        if step.complete { return (counts,step); }
        assert!(result.cursor.is_some());
        previous = step.canonical_payload.clone();
    }
    panic!("geometry checkpoints never reached the last widget");
}

fn json(execution: &ArtifactInferenceExecution) -> serde_json::Value {
    serde_json::from_slice(execution.canonical_payload.as_deref().expect("the execution carries its payload")).expect("the result payload is JSON")
}

fn typed(execution: &ArtifactInferenceExecution) -> GeometryResult {
    semio_framework_pack_json::from_json_str(std::str::from_utf8(execution.canonical_payload.as_deref().expect("the execution carries its payload")).expect("UTF-8"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the result decodes through its typed schema")
}
//#endregion 🧸️Fixtures

#[test]
fn scalar_geometry_publishes_its_actual_widget_and_closes_the_owner() {
    let host = GeometryHost::default();
    let mut snapshot = Generation3dSnapshot::default();
    let host_snapshot = FlowHostSnapshot {
        schema: "flow.host_snapshot".into(), camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![Widget::InputSlider { id: "height".into(), label: "Height".into(), value: 6.0, min: 0.0, max: 10.0, step: 1.0 }],
        synapses: Vec::new(), layout: OrderedMap::new(),
    };
    std::mem::replace(&mut snapshot.host_snapshot, host_snapshot).retire_cold();
    let payload = request_text(Generation3dSnapshotRead::new(snapshot), None);
    let budgets = WireArtifactInferenceBudget { allocation_bytes: 1 << 24, work_units: 1000, recursion_depth: 8 };
    let request = ArtifactInferenceExecutionRequest {
        operation: 17, generation: 9, cancelled: false,
        retained: RetainedCloneGrant { maximum_items: 4096, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 128 },
        policy: &[], budgets: &budgets, cancellation_id: "scalar", previous_state: None,
        requested_cache_mode: WireArtifactInferenceCacheMode::Bypass, canonical_payload: payload.as_bytes(), dependencies: &[],
    };
    let mut previous = None;
    for _ in 0..3 {
        let call = ArtifactInferenceExecutionRequest { previous_state: previous.as_deref(), requested_cache_mode: WireArtifactInferenceCacheMode::Bypass, ..request };
        let execution = host.infer(&call, Some(&host.host)).expect("the scalar request runs");
        let result = typed(&execution);
        assert_eq!((result.progress.completed, result.progress.total), (1, 1));
        if result.complete {
            assert_eq!(result.widgets.len(), 1);
            assert_eq!(result.widgets[0].id, "height");
            assert!(result.widgets[0].fault.is_none());
            assert_eq!(result.widgets[0].outputs.len(), 1);
            return;
        }
        previous = execution.canonical_payload.clone();
    }
    panic!("the scalar widget never published its terminal result");
}

#[test]
fn projected_preview_preserves_connection_order_when_sources_publish_the_same_port() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🚀️original-field/🧪️tests/👁️preview/🧫️.json")).unwrap();
    let mut widgets: Vec<_> = law["widgets"].as_array().unwrap().iter().map(|row| Widget::InputSlider {
        id: row["id"].as_str().unwrap().into(), label: row["id"].as_str().unwrap().into(), value: row["number"].as_f64().unwrap(), min: 0.0, max: 10.0, step: 1.0,
    }).collect();
    widgets.push(Widget::OutputPreview { id: "preview".into(), preview: Dictionary::new(), expanded: OrderedSet::new() });
    let synapses = law["connections"].as_array().unwrap().iter().enumerate().map(|(ordinal, row)| SynapseSpec {
        id: ordinal.to_string(), from: row["from"].as_str().unwrap().into(), to: "preview".into(), from_port: row["port"].as_str().unwrap().into(), to_port: String::new(),
    }).collect();
    let mut snapshot = Generation3dSnapshot::default();
    let graph = FlowHostSnapshot { schema: "flow.host_snapshot".into(), camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, widgets, synapses, layout: OrderedMap::new() };
    std::mem::replace(&mut snapshot.host_snapshot, graph).retire_cold();
    let payload = request_text(Generation3dSnapshotRead::new(snapshot), None);
    let host = GeometryHost::default();
    let budgets = WireArtifactInferenceBudget { allocation_bytes: 1 << 24, work_units: 1000, recursion_depth: 8 };
    let mut previous = None;
    for _ in 0..10 {
        let request = ArtifactInferenceExecutionRequest {
            operation: 17, generation: 9, cancelled: false, policy: &[], budgets: &budgets, cancellation_id: "projected",
            retained: RetainedCloneGrant { maximum_items: 4096, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 128 },
            previous_state: previous.as_deref(), requested_cache_mode: WireArtifactInferenceCacheMode::Bypass, canonical_payload: payload.as_bytes(), dependencies: &[],
        };
        let execution = host.infer(&request, Some(&host.host)).expect("the actual connected preview runs");
        if execution.complete {
            let result = typed(&execution);
            let preview = result.widgets.iter().find(|widget| widget.id == "preview").unwrap();
            assert!(preview.fault.is_none());
            assert_eq!(preview.outputs.len(), 1);
            assert_eq!(preview.outputs[0].port, "number");
            assert_eq!(preview.outputs[0].detail, law["expected"]["number"].as_str().unwrap());
            return;
        }
        previous = execution.canonical_payload.clone();
    }
    panic!("the preview never reached its terminal result");
}

//#region 🧪️Contract
#[test]
fn the_registered_metadata_publishes_the_widget_step_contract_and_both_schemas_parse() {
    let metadata = geometry_inference_service().metadata();
    assert_eq!((metadata.owner, metadata.artifact_kind, metadata.inference_schema), ("procedural", "s.procedural.generation3d", "s.procedural.generation3d.inference"));
    let contract = metadata.payload.expect("the service publishes a payload contract");
    assert_eq!(contract.progress_unit, "widget-step");
    for schema in [contract.input_schema, contract.output_schema] {
        let parsed: serde_json::Value = serde_json::from_str(schema).expect("the schema is JSON");
        assert_eq!(parsed["type"], "object");
    }
    assert!(contract.artifact_binding.is_some_and(|binding| binding.field == "document" && !binding.required));
    semio_plugin::register_artifact_inference_service(geometry_inference_service()).expect("the service registers without conflict");
    assert!(semio_plugin::artifact_inference_service("s.procedural.generation3d", "s.procedural.generation3d.inference").expect("registry reads").is_some());
}

#[test]
fn an_idle_failure_check_leaves_the_geometry_host_empty_and_the_callers_grant_unspent() {
    let host = GeometryHost::default();
    let mut source = host.caller.borrow_mut();
    let caller = source.caller.as_mut().unwrap();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 128 };
    caller.refreshed_turn(grant, |cx| {
        let before = cx.retained_progress();
        assert!(host.with(|engine| engine.admit_failure(cx)).unwrap()?.is_none());
        assert_eq!(cx.retained_progress(), before);
        assert!(host.terminal_is_empty());
        Ok(())
    }).unwrap();
}

#[test]
fn a_service_without_its_geometry_host_refuses_by_name() {
    let budgets = WireArtifactInferenceBudget { allocation_bytes: 1 << 20, work_units: 10, recursion_depth: 1 };
    let request = ArtifactInferenceExecutionRequest { operation:17,generation:9,cancelled:false, retained:semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128}, policy: &[], budgets: &budgets, cancellation_id: "x", previous_state: None, requested_cache_mode: WireArtifactInferenceCacheMode::Incremental, canonical_payload: b"{}", dependencies: &[] };
    let host=GeometryHost::default();assert_eq!(host.infer(&request,None).err().expect("the service refuses").code, "artifact-inference.context-required");
    assert_eq!(host.infer(&request,Some(&0u8)).err().expect("the service refuses").code, "generation3d.geometry.context");
}

#[test]
fn the_request_and_result_fixtures_hold_in_rust_and_the_result_decodes_through_its_typed_schema() {
    let fixture = fixture();
    let host = GeometryHost::default();
    let payload = request_text(document(2.0, 1.5), None);
    let (stepped, execution) = completed_document(&host, &payload, 1000);
    let (_, faulted) = completed_document(&GeometryHost::default(), &request_text(document(2.0, -1.0), None), 1000);
    let observed = serde_json::json!({
        "request": serde_json::from_str::<serde_json::Value>(&payload).expect("the request is JSON"),
        "complete": json(&execution),
        "stepped": stepped,
        "faulted": json(&faulted),
    });
    assert_eq!(observed, fixture, "the service must reproduce all contract cases: {observed}");
    let result = typed(&execution);
    assert!(result.complete && result.cursor.is_none() && result.progress.completed == result.progress.total);
    assert_eq!(semio_framework_pack_json::to_json_string(&result), String::from_utf8(execution.canonical_payload.clone().expect("the execution carries its payload")).expect("UTF-8"), "the typed schema re-encodes byte for byte");
    assert_eq!((execution.complete, execution.validity.as_str(), execution.quality.as_str()), (true, "valid", "exact-analytic"));
    assert!(execution.diagnostics.is_empty());
}
//#endregion 🧪️Contract

//#region 🧪️Steps
#[test]
fn a_run_resumes_from_its_own_result_and_equals_the_complete_run_for_every_work_unit_grant() {
    let payload = request_text(document(2.0, 1.5), None);
    let (_, reference) = completed_document(&GeometryHost::default(), &payload, 1000);
    let expected = typed(&reference);
    for units in [256, 512, 1024, 2048] {
        let host = GeometryHost::default();
        let (total, execution) = run_updates(&host, &payload, units, WireArtifactInferenceCacheMode::Incremental);
        assert_eq!(typed(&execution).widgets, expected.widgets, "{units} physical work units per call");
        assert_eq!(total, (4, 0));
    }
}

#[test]
fn stepped_results_match_the_committed_fixture_and_a_rebuilt_owner_restarts_instead_of_resuming() {
    let payload = request_text(document(2.0, 1.5), None);
    let host = GeometryHost::default();
    let first = once(&host, &payload, 1000);
    assert_eq!(json(&first), fixture()["stepped"]);
    let rebuilt = GeometryHost::default();
    let replay = call(&rebuilt, Call { payload: &payload, work_units: 1000, previous: first.canonical_payload.as_deref(), mode: WireArtifactInferenceCacheMode::Incremental }).expect("rebuilt owner restarts");
    assert_eq!(json(&replay),json(&first));
    let expected = completed_document(&GeometryHost::default(), &payload, 1000).1;
    let mut current = replay;
    let mut computed = typed(&current).computed;
    for _ in 0..100 {
        if current.complete { break; }
        current = call(&rebuilt, Call { payload: &payload, work_units: 1000, previous: current.canonical_payload.as_deref(), mode: WireArtifactInferenceCacheMode::Incremental }).unwrap();
        computed += typed(&current).computed;
    }
    assert!(current.complete);
    assert_eq!(typed(&current).widgets, typed(&expected).widgets);
    assert_eq!(computed, 4);
}

#[test]
fn cancelling_stops_the_run_with_a_named_error_and_a_new_call_starts_over_from_the_cache() {
    let payload = request_text(document(2.0, 1.5), None);
    let host = GeometryHost::default();
    let first = once(&host, &payload, 1000);
    let completed = typed(&first).progress.completed;
    assert!(completed > 0 && !first.complete);
    assert!(!host.cancel_run("another-id").unwrap());
    assert!(host.cancel_run("geometry-test").unwrap());
    let refused = call(&host, Call { payload: &payload, work_units: 1000, previous: first.canonical_payload.as_deref(), mode: WireArtifactInferenceCacheMode::Incremental }).err().expect("cancelled geometry refuses continuation");
    assert_eq!(refused.code, "generation3d.geometry.cancelled");
    let (counts, _) = run_updates(&host,&payload,1000,WireArtifactInferenceCacheMode::Incremental);
    assert_eq!(counts,(4-completed,completed));
}
//#endregion 🧪️Steps

//#region 🧪️Cache
#[test]
fn the_cache_modes_are_incremental_cold_and_bypass() {
    let payload = request_text(document(2.0, 1.5), None);
    let host = GeometryHost::default();
    let run = |payload: &str, mode| run_updates(&host,payload,1000,mode);
    assert_eq!(run(&payload,WireArtifactInferenceCacheMode::Incremental).0,(4,0));
    assert_eq!(run(&payload,WireArtifactInferenceCacheMode::Incremental).0,(0,4));
    let edited = request_text(document(3.0, 1.5), None);
    assert_eq!(run(&edited,WireArtifactInferenceCacheMode::Incremental).0,(2,2));
    assert_eq!(run(&edited,WireArtifactInferenceCacheMode::Cold).0,(4,0));
    let (counts, bypass) = run(&edited,WireArtifactInferenceCacheMode::Bypass);
    assert_eq!(bypass.actual_cache_mode,WireArtifactInferenceCacheMode::Bypass);
    assert_eq!(counts,(4,0));
}
//#endregion 🧪️Cache

//#region 🧪️Refusals
#[test]
fn faults_are_reported_as_diagnostics_in_both_languages_and_make_the_run_invalid() {
    let payload = request_text(document(2.0, -1.0), None);
    let (_, execution) = completed_document(&GeometryHost::default(), &payload, 1000);
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
    let refusal = |payload: &str, units: u64| call(&host, Call { payload, work_units: units, previous: None, mode: WireArtifactInferenceCacheMode::Incremental }).err().expect("the service refuses");
    assert_eq!(refusal(&payload, 0).code, "generation3d.geometry.invalid-request");
    let minimum = host.caller.borrow().caller.as_ref().unwrap().source_parent().loan_admission_work_demand().unwrap();
    assert!(minimum > 1);
    assert_eq!(refusal(&payload, minimum - 1).code, "generation3d.geometry.invalid-request");
    assert!(host.terminal_is_empty());
    assert_eq!(refusal("{}", 10).code, "generation3d.geometry.invalid-request");
    assert_eq!(refusal("{\"snapshot\": 3}", 10).code, "generation3d.geometry.invalid-request");
    assert_eq!(refusal("not json", 10).code, "generation3d.geometry.invalid-request");
    let tight = WireArtifactInferenceBudget { allocation_bytes: 8, work_units: 10, recursion_depth: 1 };
    let request = ArtifactInferenceExecutionRequest { operation:17,generation:9,cancelled:false, retained:semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128}, policy: &[], budgets: &tight, cancellation_id: "x", previous_state: None, requested_cache_mode: WireArtifactInferenceCacheMode::Incremental, canonical_payload: payload.as_bytes(), dependencies: &[] };
    assert_eq!(host.infer(&request,Some(&host.host)).err().expect("the service refuses").code, "generation3d.geometry.invalid-request");
    let garbage = ArtifactInferenceExecutionRequest { previous_state: Some(b"nope"), budgets: &WireArtifactInferenceBudget { allocation_bytes: 1 << 24, work_units: 10, recursion_depth: 1 }, ..request };
    assert_eq!(host.infer(&garbage,Some(&host.host)).err().expect("the service refuses").code, "generation3d.geometry.invalid-request");
    let foreign = serde_json::json!({
        "complete": false, "progress": {"completed": 0, "total": 4, "fraction": 0},
        "computed": 0, "cacheHits": 0, "fuelUsed": 0, "faulted": 0, "widgets": [],
        "cursor": {"digest": "0".repeat(64), "next": 0}
    }).to_string();
    let error = call(&host, Call { payload: &payload, work_units: 1000, previous: Some(foreign.as_bytes()), mode: WireArtifactInferenceCacheMode::Incremental }).err().expect("geometry inference must reject this request");
    assert_eq!(error.code, "generation3d.geometry.invalid-request");
    assert!(host.terminal_is_empty());
}

#[test]
fn a_selected_generation_overrides_the_slider_values_it_names() {
    let mut request = fixture()["request"].clone();
    request["snapshot"]["generation"]["generations"] = serde_json::json!([{"id":"tall","name":"Tall","values":{"size":4.0}}]);
    let evaluate = |selected: Option<&str>| {
        let mut request = request.clone();
        if let Some(selected) = selected { request["generation"] = selected.into(); }
        let (_, execution) = completed_document(&GeometryHost::default(), &request.to_string(), 1000);
        typed(&execution).widgets
    };
    let plain = evaluate(None);
    assert_eq!(plain, evaluate(Some("missing")));
    let tall = evaluate(Some("tall"));
    let output = |widgets: &[GeometryWidgetResult], id: &str| widgets.iter().find(|widget| widget.id == id).unwrap().outputs[0].detail.clone();
    assert_eq!(output(&plain, "size"), "2");
    assert_eq!(output(&tall, "size"), "4");
    assert_eq!(output(&plain, "ball"), output(&tall, "ball"));
    assert_ne!(output(&plain, "box"), output(&tall, "box"));
    assert_eq!(output(&tall, "view"), output(&tall, "box"));
}
//#endregion 🧪️Refusals
