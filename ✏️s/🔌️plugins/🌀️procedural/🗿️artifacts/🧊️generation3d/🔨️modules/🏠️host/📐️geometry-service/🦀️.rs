//! 🏠️ Native geometry service admission and instance engine ownership.
#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as protocol;
use semio_s_artifact_procedural_3d_geometry::engine::CacheMode;
use semio_s_artifact_procedural_3d_geometry::engine::EngineError;
use semio_s_artifact_procedural_3d_geometry::engine::EngineStep;
use semio_s_artifact_procedural_3d_geometry::engine::GeometryEngine;
use semio_s_artifact_procedural_3d_geometry::engine::EngineAdmission;
use semio_s_artifact_procedural_3d_geometry::Generation3dWidgetRecord;
use semio_s_artifact_procedural_3d_geometry::service::*;
use semio_s_artifact_procedural_3d_geometry::engine::GEOMETRY_CACHE_BUDGET_BYTES;
use semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue;
use std::cell::RefCell;
use std::sync::Arc;
use semio_s_artifact_procedural_3d_model::catalogue::Quality;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::generation_host_snapshot_for;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use protocol::DepHash;
use semio_framework_plugin::{ArtifactDocumentPayload, ArtifactInferenceExecution, ArtifactInferenceExecutionStep, ArtifactInferenceExecutionError, ArtifactInferenceExecutionRequest, ArtifactInferencePayloadContract, ArtifactInferenceService, ArtifactInferenceServiceMetadata, WireArtifactInferenceCacheMode, WireArtifactInferenceDiagnostic};
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::{RetirementDemand, ToValue as _, ValueError};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;


/// 🏠️ The shareable cell a contextual inference service receives: the engine behind interior mutability, because the service gets a shared reference to its context.
pub struct GeometryHost {
    engine: RefCell<GeometryEngine>,
    admission:RefCell<std::mem::ManuallyDrop<Option<EngineAdmission>>>,
}

impl Drop for GeometryHost{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"geometry host requires original engine and request donor closure");}}

impl Default for GeometryHost {
    fn default() -> Self {
        Self::new(GEOMETRY_CACHE_BUDGET_BYTES)
    }
}

impl GeometryHost {
    pub fn new(budget_bytes: usize) -> Self {
        Self { engine: RefCell::new(GeometryEngine::new(Arc::clone(catalogue()), semio_s_artifact_procedural_3d_io_geometry::context(), budget_bytes)),admission:RefCell::new(std::mem::ManuallyDrop::new(None)) }
    }

    /// 🔓️ Runs `body` with the engine; a re-entrant call is [`EngineError::Busy`], never a panic.
    pub fn with<R>(&self, body: impl FnOnce(&mut GeometryEngine) -> R) -> Result<R, EngineError> {
        let mut engine = self.engine.try_borrow_mut().map_err(|_| EngineError::Busy)?;
        Ok(body(&mut engine))
    }

    /// 🧹️ Closes every owner the engine holds; a re-entrant call is [`EngineError::Busy`] and leaves it open.
    pub fn retire_cold(&self) -> Result<(), EngineError> {
        self.with(GeometryEngine::retire_cold)
    }

    /// 🧾️ Whether the engine holds nothing left to close; a busy engine is not terminal.
    pub fn terminal_is_empty(&self) -> bool {
        self.with(|engine|engine.terminal_is_empty()).unwrap_or(false)&&self.admission.try_borrow().is_ok_and(|admission|admission.is_none())
    }

    /// 🛑️ Cancels the run in flight.
    pub fn cancel(&self) -> Result<(), EngineError> {
        self.with(GeometryEngine::cancel)
    }

    /// 🛑️ Cancels the run in flight when it carries `cancellation_id`.
    pub fn cancel_run(&self, cancellation_id: &str) -> Result<bool, EngineError> {
        self.with(|engine| engine.cancel_run(cancellation_id))
    }
}

/// 📥️ A geometry request: the snapshot stated in full, or the artifact document the gateway binds; optionally the selected generation whose values override the sliders.
#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GeometryRequest {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Generation3dSnapshot>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<ArtifactDocumentPayload>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
}

impl GeometryRequest {
    /// 📸️ The snapshot this request evaluates, with the selected generation applied; ownership moves to the caller.
    pub fn into_snapshot(self) -> Result<Generation3dSnapshot, String> {
        let mut snapshot = match (self.snapshot, self.document) {
            (Some(snapshot), None) => snapshot,
            (None, Some(document)) => document.settled_snapshot::<Generation3dSnapshot, Generation3dMutation>()?,
            (Some(snapshot), Some(_)) => {
                snapshot.retire_cold();
                return Err("state exactly one of `snapshot` and `document`".into());
            }
            (None, None) => return Err("state `snapshot`, or name the artifact so `document` is bound".into()),
        };
        if let Some(selected) = self.generation.as_deref() {
            let patched = generation_host_snapshot_for(&snapshot.host_snapshot, snapshot.generation.as_state(), Some(selected));
            std::mem::replace(&mut snapshot.host_snapshot, patched).retire_cold();
        }
        Ok(snapshot)
    }
}


//#region 🔖️Service
pub const GEOMETRY_INFERENCE_CONTRACT: ArtifactInferencePayloadContract = ArtifactInferencePayloadContract {
    payload_schema_id: GEOMETRY_PAYLOAD_SCHEMA,
    input_schema: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🛰️service/📥️request.json"),
    output_schema: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🛰️service/📤️result.json"),
    progress_unit: GEOMETRY_PROGRESS_UNIT,
    artifact_binding: Some(semio_framework_plugin::ArtifactInferenceDocumentBinding { field: "document", encoding: semio_framework::INFERENCE_ARTIFACT_PACK_BASE64, required: false }),
    commit: None,
};

/// 🪪️ The geometry service: contextual, so it runs against the engine its caller already owns.
pub const fn geometry_inference_service() -> ArtifactInferenceService {
    ArtifactInferenceService::new_contextual(
        ArtifactInferenceServiceMetadata {
            owner: "procedural",
            artifact_kind: GEOMETRY_ARTIFACT_KIND,
            artifact_schema: GEOMETRY_ARTIFACT_KIND,
            artifact_schema_version: 1,
            inference_schema: GEOMETRY_INFERENCE_SCHEMA,
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
            payload: Some(GEOMETRY_INFERENCE_CONTRACT),
        },
        infer_geometry,
        geometry_inference_demands,
    )
}

/// ♻️ A geometry call ends terminal inside its own turn and keeps no child between turns, so there is nothing to quote.
fn geometry_inference_demands(_request: &ArtifactInferenceExecutionRequest<'_>, context: &dyn std::any::Any, _copy: usize) -> Result<RetirementDemand, ValueError> {
    context.downcast_ref::<GeometryHost>().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "geometry demand requires its instance's geometry host"))?;
    Ok(RetirementDemand::default())
}

fn invalid(message: impl Into<String>) -> ArtifactInferenceExecutionError {
    ArtifactInferenceExecutionError::new("generation3d.geometry.invalid-request", message)
}

fn digest_of(payload: &[u8]) -> String {
    DepHash::root(GEOMETRY_INFERENCE_SCHEMA, 1, payload).0.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn quality_name(quality: Quality) -> String {
    quality.to_value().as_str().unwrap_or("approximate").to_string()
}

const QUALITY_ORDER: [Quality; 6] = [Quality::ExactAnalytic, Quality::ExactNumerical, Quality::PolygonMesh, Quality::TessellatedMesh, Quality::MeshDerivedBrep, Quality::Approximate];

fn weakest(qualities: impl Iterator<Item = Quality>) -> Quality {
    qualities.max_by_key(|quality| QUALITY_ORDER.iter().position(|ordered| ordered == quality)).unwrap_or(Quality::ExactAnalytic)
}

fn result_of(engine: &GeometryEngine, step: &EngineStep, digest: &str) -> GeometryResult {
    let widgets: Vec<GeometryWidgetResult> = engine
        .evaluations()
        .map(|(id, evaluation)| {
            let record = Generation3dWidgetRecord::of(evaluation);
            GeometryWidgetResult { id: id.clone(), dep: engine.dependency_hash(id).unwrap_or_default(), quality: record.quality, fault: record.fault, outputs: record.outputs }
        })
        .collect();
    GeometryResult {
        complete: step.done,
        progress: GeometryProgress { completed: step.completed as u32, total: step.total as u32, fraction: f64::from(step.fraction) },
        computed: step.computed as u32,
        cache_hits: step.hits as u32,
        fuel_used: step.fuel_used as u32,
        faulted: widgets.iter().filter(|widget| widget.fault.is_some()).count() as u32,
        cursor: (!step.done).then(|| GeometryCursor { digest: digest.to_string(), next: step.completed as u32 }),
        widgets,
    }
}

fn engine_failure(error: EngineError) -> ArtifactInferenceExecutionError {
    match error {
        EngineError::Busy => ArtifactInferenceExecutionError::new("generation3d.geometry.busy", "the geometry engine is serving another call"),
        EngineError::NotStarted => ArtifactInferenceExecutionError::new("generation3d.geometry.not-started", "the geometry engine has no run"),
    }
}

fn cache_mode(requested: &WireArtifactInferenceCacheMode) -> CacheMode {
    match requested {
        WireArtifactInferenceCacheMode::Cold => CacheMode::Cold,
        WireArtifactInferenceCacheMode::Incremental => CacheMode::Incremental,
        WireArtifactInferenceCacheMode::Bypass => CacheMode::Bypass,
    }
}

fn receipt_since(before:RetainedCloneProgress,after:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:after.copied_items.checked_sub(before.copied_items).expect("original geometry receipt items"),copied_bytes:after.copied_bytes.checked_sub(before.copied_bytes).expect("original geometry receipt body"),retained_capacity_bytes:after.retained_capacity_bytes.checked_sub(before.retained_capacity_bytes).expect("original geometry receipt capacity"),released_bytes:after.released_bytes.checked_sub(before.released_bytes).expect("original geometry receipt release")}}

fn infer_geometry(request: &ArtifactInferenceExecutionRequest<'_>, context: &dyn std::any::Any, cx:&mut semio_framework_plugin::ArtifactInferenceStepContext<'_>) -> Result<ArtifactInferenceExecutionStep, ArtifactInferenceExecutionError> {
    let host = context.downcast_ref::<GeometryHost>().ok_or_else(|| ArtifactInferenceExecutionError::new("generation3d.geometry.context", "geometry inference requires its instance's geometry host"))?;
    let admitted=host.with(|engine|engine.admit_failure(cx)).map_err(engine_failure)?.map_err(|original|ArtifactInferenceExecutionError::original(original,false))?;
    if let Some(original)=admitted{return Err(ArtifactInferenceExecutionError::original(original,false))}
    if host.with(GeometryEngine::has_failure).map_err(engine_failure)?{return Ok(ArtifactInferenceExecutionStep{execution:None,retained_progress:Default::default(),terminal:false})}
    let before=cx.retained_progress();
    if request.cancelled {
        host.cancel().map_err(engine_failure)?;
        let mut admission=host.admission.try_borrow_mut().map_err(|_|engine_failure(EngineError::Busy))?;
        if let Some(owner)=admission.as_mut(){match owner.close_step(cx){Ok(true)=>{admission.take();},Ok(false)=>return Ok(ArtifactInferenceExecutionStep{execution:None,retained_progress:receipt_since(before,cx.retained_progress()),terminal:false}),Err(original)=>{host.with(|engine|engine.hold_failure(original)).map_err(engine_failure)?;return Ok(ArtifactInferenceExecutionStep{execution:None,retained_progress:receipt_since(before,cx.retained_progress()),terminal:false})}}}
        let terminal=match host.with(|engine|engine.close_original_step(cx)).map_err(engine_failure)?{Ok(terminal)=>terminal,Err(original)=>{host.with(|engine|engine.hold_failure(original)).map_err(engine_failure)?;false}};
        return Ok(ArtifactInferenceExecutionStep { execution: None, retained_progress:receipt_since(before,cx.retained_progress()), terminal });
    }
    if request.retained.maximum_items == 0 || request.retained.maximum_depth == 0 {
        return Ok(ArtifactInferenceExecutionStep { execution: None, retained_progress: RetainedCloneProgress::default(), terminal: false });
    }
    if request.budgets.work_units == 0 || request.canonical_payload.len() as u64 > request.budgets.allocation_bytes {
        return Err(invalid("geometry inference exceeds its execution budget"));
    }
    let digest = digest_of(request.canonical_payload);
    let mode = cache_mode(&request.requested_cache_mode);
    let _checkpoint = match request.previous_state {
        Some(bytes) => {
            let text = std::str::from_utf8(bytes).map_err(|error| invalid(error.to_string()))?;
            let previous: GeometryResult = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| invalid(error.to_string()))?;
            previous.cursor
        }
        None => None,
    };
    let outcome = host
        .with(|engine| -> Result<Option<(EngineStep, GeometryResult)>, ArtifactInferenceExecutionError> {
            let resumes=engine.holds(&digest);
            if !resumes {
                let mut admission=host.admission.try_borrow_mut().map_err(|_|engine_failure(EngineError::Busy))?;
                if admission.is_none()&&!engine.terminal_is_empty(){match engine.close_original_step(cx){Ok(true)=>{},Ok(false)=>return Ok(None),Err(original)=>{engine.hold_failure(original);return Ok(None)}}}
                if admission.is_none(){
                    let text=std::str::from_utf8(request.canonical_payload).map_err(|error|invalid(error.to_string()))?;
                    let parsed:GeometryRequest=semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|invalid(error.to_string()))?;
                    let snapshot=parsed.into_snapshot().map_err(invalid)?;
                    **admission=Some(EngineAdmission::new(digest.clone(),snapshot,mode,request.cancellation_id.to_string()));
                }
                if admission.as_ref().is_some_and(|owner|owner.digest()!=Some(digest.as_str())){return Err(invalid("geometry request changed while its original admission remains open"))}
                match engine.start_original_step(&mut admission,cx){Ok(true)=>{},Ok(false)=>return Ok(None),Err(original)=>{engine.hold_failure(original);return Ok(None)}}
            }
            let step=match engine.step(cx){Ok(step)=>step,Err(original)=>{engine.hold_failure(original);return Ok(None)}};
            let result = result_of(engine, &step, &digest);
            Ok(Some((step, result)))
        })
        .map_err(engine_failure)??;
    let Some((step,result))=outcome else{
        let delta=receipt_since(before,cx.retained_progress());
        return Ok(ArtifactInferenceExecutionStep{execution:None,retained_progress:delta,terminal:false});
    };
    let diagnostics: Vec<WireArtifactInferenceDiagnostic> = result
        .widgets
        .iter()
        .filter_map(|widget| {
            let fault = widget.fault.as_ref()?;
            let mut parameters = BTreeMap::from([("widget".to_string(), widget.id.clone()), ("de".to_string(), fault.de.clone())]);
            if let Some(port) = &fault.port {
                parameters.insert("port".to_string(), port.clone());
            }
            Some(WireArtifactInferenceDiagnostic { code: fault.code.clone(), message: fault.en.clone(), severity: "error".to_string(), parameters })
        })
        .collect();
    let quality = quality_name(weakest(result.widgets.iter().map(|widget| widget.quality)));
    Ok(ArtifactInferenceExecution {
        retirement_progress: RetainedCloneProgress { copied_items: 1, ..Default::default() },
        canonical_payload: Some(semio_framework_pack_json::to_json_string(&result).into_bytes()),
        validity: if diagnostics.is_empty() { "valid" } else { "invalid" }.to_string(),
        diagnostics,
        quality,
        complete: step.done,
        actual_cache_mode: WireArtifactInferenceCacheMode::Bypass,
    }.into_step(true))
}
//#endregion 🔖️Service

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
