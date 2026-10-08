//! 📐️ Geometry computation owned by the named flow artifact inference provider.

use super::{module_registry, SessionCapture};
use semio_framework_plugin::{ArtifactInferenceExecution, ArtifactInferenceExecutionError, ArtifactInferenceExecutionRequest, ArtifactInferencePayloadContract, ArtifactInferenceService, ArtifactInferenceServiceMetadata, WireArtifactInferenceCacheMode};

pub const GEOMETRY_INFERENCE_SCHEMA: &str = "s.flow-extension-brep.geometry";
pub const GEOMETRY_ARTIFACT_KIND: &str = "s.flow.flow";

/// 🎟️ Keeps one original geometry execution source and its exact registry/session close authority.
pub struct GeometryInferenceContext {session:SessionCapture,evaluation:flow_extension_sdk::ExtensionEvaluationResources}
impl GeometryInferenceContext {
    pub fn new(session:SessionCapture)->Self {let evaluation=flow_extension_sdk::ExtensionEvaluationResources::new(module_registry(&session));Self {session,evaluation}}
    pub fn registry(&self)->&neural_engine::SharedRegistry {self.evaluation.registry()}
    pub fn session(&self)->&SessionCapture {&self.session}
    pub fn next_close_byte_demand(&self)->usize {use semio_framework_plugin::ExtensionResourceOwner;self.evaluation.next_close_byte_demand().max(self.session.shell_byte_requirement()).max(self.session.next_close_byte_demand()).max(1)}
    pub fn begin_close(&mut self) {semio_framework_plugin::ExtensionResourceOwner::begin_close(&mut self.evaluation);}
    pub fn close_step(&mut self,items:usize,bytes:usize)->Result<semio_framework_plugin::PluginCloseStep,semio_framework::Fault> {
        use semio_framework_plugin::{ExtensionResourceOwner,PluginCloseStep as Step};
        if !self.evaluation.terminal_is_empty(){return self.evaluation.close_step(items,bytes);}
        self.session.close_step(items,bytes).map(|step|match step {neural_engine::ValueRetirementStep::Pending {released_items,released_bytes}=>Step::Pending {released_items,released_bytes},neural_engine::ValueRetirementStep::Blocked=>Step::Blocked {reason:"BREP resource family is paused"},neural_engine::ValueRetirementStep::Complete=>Step::Complete}).map_err(|message|semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin,semio_framework::FaultCode::new("extension.brep-close"),message))
    }
    pub fn terminal_is_empty(&self)->bool {semio_framework_plugin::ExtensionResourceOwner::terminal_is_empty(&self.evaluation)&&self.session.terminal_is_empty()}
}

pub const GEOMETRY_INFERENCE_CONTRACT: ArtifactInferencePayloadContract = ArtifactInferencePayloadContract {
    payload_schema_id: "s.flow-extension-brep.geometry.payload",
    input_schema: include_str!("📥️request.json"),
    output_schema: include_str!("📤️result.json"),
    progress_unit: "operator-step",
    artifact_binding: None,
    commit: None,
};

/// 🪪️ Registers the computation owner without capturing or duplicating instance resources.
pub const fn geometry_inference_service() -> ArtifactInferenceService {
    ArtifactInferenceService::new_contextual(ArtifactInferenceServiceMetadata {
        owner: "flow-extension-brep",
        artifact_kind: GEOMETRY_ARTIFACT_KIND,
        artifact_schema: "s.flow.flow",
        artifact_schema_version: 1,
        inference_schema: GEOMETRY_INFERENCE_SCHEMA,
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        payload: Some(GEOMETRY_INFERENCE_CONTRACT),
    }, infer_geometry)
}

#[cfg(test)]
pub(crate) static GEOMETRY_INFERENCE_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🎯️ Shares each registered operator's owned fidelity contract with named inference results.
pub(crate) fn geometry_operator_quality(registry: &neural_engine::Registry, operator: &str) -> String {
    let Some(info) = registry.operator_info(operator) else { return "Unsupported".into(); };
    if let Some(operation) = operator.strip_prefix("brep.mesh.") { return super::mesh::operation_quality(operation).into(); }
    if !super::NODE_KERNEL_METHOD.iter().any(|(id, _)| *id == operator) && info.group.iter().any(|group| group == "Schemas") && operator.rsplit('.').next().is_some_and(|schema| registry.schema(schema).is_some()) { return "SchemaValue".into(); }
    super::NODE_KERNEL_METHOD.iter().find(|(id, _)| *id == operator).map(|(_, method)| format!("{:?}", super::operation_quality(method))).unwrap_or_else(|| "Unsupported".into())
}

fn infer_geometry(request: &ArtifactInferenceExecutionRequest<'_>, context: &dyn std::any::Any) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    let error = |message| ArtifactInferenceExecutionError::new("geometry-inference.invalid-request", message);
    let context = context.downcast_ref::<GeometryInferenceContext>().ok_or_else(|| ArtifactInferenceExecutionError::new("geometry-inference.context", "geometry inference requires its original Brep execution owner"))?;
    let registry=context.registry();
    if request.budgets.work_units == 0 || request.canonical_payload.len() as u64 > request.budgets.allocation_bytes {
        return Err(error("geometry inference exceeds its execution budget".to_string()));
    }
    let text = std::str::from_utf8(request.canonical_payload).map_err(|failure| error(failure.to_string()))?;
    let mut payload: flow_extension_sdk::EvaluateRequest = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|failure| error(failure.to_string()))?;
    if !payload.operator_id.starts_with("brep.") {
        return Err(error("geometry inference requires a Brep operator".to_string()));
    }

    if payload.resume && (!request.dependencies.is_empty()||!request.policy.is_empty()) {return Err(error("compact geometry resume cannot replace dependency or policy payloads".to_string()));}
    if !payload.resume {
        let input = semio_framework_pack_json::parse(&payload.input_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|failure| error(failure.to_string()))?;
        let mut pending = vec![&input];
        let mut referenced_geometry = false;
        while let Some(value) = pending.pop() {
            match value {
                semio_framework_pack_json::Value::Object(object) => { referenced_geometry |= object.get("handle").and_then(semio_framework_pack_json::Value::as_str).is_some(); pending.extend(object.iter().map(|(_, value)| value)); }
                semio_framework_pack_json::Value::Array(array) => pending.extend(array),
                _ => {},
            }
        }
        if referenced_geometry && payload.dependency_json.is_empty() && request.dependencies.is_empty() {
            return Err(ArtifactInferenceExecutionError::new("geometry-inference.dependencies-required", "referenced geometry requires canonical source and wiring dependencies"));
        }
    }
    if !request.dependencies.is_empty() {
        let source = semio_framework_pack_json::parse(&payload.dependency_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| semio_framework_pack_json::Value::from(payload.dependency_json.clone()));
        let dependencies = semio_framework_pack_json::array(request.dependencies.iter().map(|(owner, bytes)| semio_framework_pack_json::object([
            ("owner".into(), semio_framework_pack_json::Value::from(owner.clone())),
            ("payload".into(), semio_framework_pack_json::array(bytes.iter().map(|byte| semio_framework_pack_json::Value::from(u64::from(*byte))))),
        ])));
        payload.dependency_json = semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("source".into(), source), ("dependencies".into(), dependencies),
        ]));
    }

    if !request.policy.is_empty() {
        let source = semio_framework_pack_json::parse(&payload.dependency_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| semio_framework_pack_json::Value::from(payload.dependency_json.clone()));
        payload.dependency_json = semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("source".into(), source),
            ("policy".into(), semio_framework_pack_json::array(request.policy.iter().map(|byte| semio_framework_pack_json::Value::from(u64::from(*byte))))),
        ]));
    }
    payload.budget = request.budgets.work_units;
    payload.round_units = request.budgets.work_units;
    payload.cancellation_id = request.cancellation_id.into();
    if request.requested_cache_mode == WireArtifactInferenceCacheMode::Cold { flow_extension_sdk::cancel_evaluation(registry,&payload.operator_id, payload.node_hash); }
    if request.requested_cache_mode == WireArtifactInferenceCacheMode::Bypass { payload.node_hash = 0; }
    #[cfg(test)]
    GEOMETRY_INFERENCE_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let quality = geometry_operator_quality(registry, &payload.operator_id);
    let reply = flow_extension_sdk::evaluate_step_envelope(registry, payload);
    Ok(ArtifactInferenceExecution {
        canonical_payload: reply.wire.into_bytes(),
        diagnostics: Vec::new(),
        validity: if reply.faulted { "invalid" } else { "valid" }.into(),
        quality,
        complete:reply.complete,
        actual_cache_mode: request.requested_cache_mode.clone(),
    })
}
