//! 🏠️ Native geometry service admission and instance engine ownership.
use crate::standards::v1::subsets::any::schema::inferences::geometry::engine::{CacheMode, EngineError, EngineStep, GeometryEngine};
use crate::standards::v1::subsets::any::schema::inferences::geometry::Generation3dWidgetRecord;
use crate::standards::v1::subsets::any::schema::inferences::geometry::service::*;
use crate::standards::v1::subsets::any::schema::inferences::geometry::engine::GEOMETRY_CACHE_BUDGET_BYTES;
use crate::standards::v1::subsets::any::io::text::snapshot::catalogue::catalogue;
use std::cell::RefCell;
use std::sync::Arc;
use crate::standards::v1::subsets::any::schema::catalogue::Quality;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::{generation_host_snapshot_for, Generation3dSnapshot};
use protocol::{DepHash, InferenceError};
use semio_framework_plugin::{ArtifactDocumentPayload, ArtifactInferenceExecution, ArtifactInferenceExecutionError, ArtifactInferenceExecutionRequest, ArtifactInferencePayloadContract, ArtifactInferenceService, ArtifactInferenceServiceMetadata, WireArtifactInferenceCacheMode, WireArtifactInferenceDiagnostic};
use semio_framework_value::ToValue;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;


/// 🏠️ The shareable cell a contextual inference service receives: the engine behind interior mutability, because the service gets a shared reference to its context.
pub struct GeometryHost {
    engine: RefCell<GeometryEngine>,
}

impl Default for GeometryHost {
    fn default() -> Self {
        Self::new(GEOMETRY_CACHE_BUDGET_BYTES)
    }
}

impl GeometryHost {
    pub fn new(budget_bytes: usize) -> Self {
        Self { engine: RefCell::new(GeometryEngine::new(Arc::clone(catalogue()), budget_bytes)) }
    }

    /// 🔓️ Runs `body` with the engine; a re-entrant call is [`EngineError::Busy`], never a panic.
    pub fn with<R>(&self, body: impl FnOnce(&mut GeometryEngine) -> R) -> Result<R, EngineError> {
        let mut engine = self.engine.try_borrow_mut().map_err(|_| EngineError::Busy)?;
        Ok(body(&mut engine))
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
            owner: "procedural-generation3d",
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
    )
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
        .iter()
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
        EngineError::Inference(InferenceError::Cancelled) => ArtifactInferenceExecutionError::new("generation3d.geometry.cancelled", "the geometry run was cancelled"),
        EngineError::Inference(other) => ArtifactInferenceExecutionError::new("generation3d.geometry.internal", other.to_string()),
    }
}

fn cache_mode(requested: &WireArtifactInferenceCacheMode) -> CacheMode {
    match requested {
        WireArtifactInferenceCacheMode::Cold => CacheMode::Cold,
        WireArtifactInferenceCacheMode::Incremental => CacheMode::Incremental,
        WireArtifactInferenceCacheMode::Bypass => CacheMode::Bypass,
    }
}

fn infer_geometry(request: &ArtifactInferenceExecutionRequest<'_>, context: &dyn std::any::Any) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    let host = context.downcast_ref::<GeometryHost>().ok_or_else(|| ArtifactInferenceExecutionError::new("generation3d.geometry.context", "geometry inference requires its instance's geometry host"))?;
    if request.budgets.work_units == 0 || request.canonical_payload.len() as u64 > request.budgets.allocation_bytes {
        return Err(invalid("geometry inference exceeds its execution budget"));
    }
    let digest = digest_of(request.canonical_payload);
    let mode = cache_mode(&request.requested_cache_mode);
    let fuel = usize::try_from(request.budgets.work_units).unwrap_or(usize::MAX);
    let checkpoint = match request.previous_state {
        Some(bytes) => {
            let text = std::str::from_utf8(bytes).map_err(|error| invalid(error.to_string()))?;
            let previous: GeometryResult = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| invalid(error.to_string()))?;
            previous.cursor
        }
        None => None,
    };
    let (step, result) = host
        .with(|engine| -> Result<(EngineStep, GeometryResult), ArtifactInferenceExecutionError> {
            let resumes = checkpoint.as_ref().is_some_and(|cursor| cursor.digest == digest && cursor.next as usize == engine.progress().0) && engine.is_running(&digest);
            if !resumes {
                if engine.holds(&digest) {
                    engine.restart(mode, request.cancellation_id);
                } else {
                    let text = std::str::from_utf8(request.canonical_payload).map_err(|error| invalid(error.to_string()))?;
                    let parsed: GeometryRequest = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| invalid(error.to_string()))?;
                    let snapshot = parsed.into_snapshot().map_err(invalid)?;
                    engine.start(digest.clone(), snapshot, mode, request.cancellation_id);
                }
            }
            let step = engine.step(fuel).map_err(engine_failure)?;
            let result = result_of(engine, &step, &digest);
            Ok((step, result))
        })
        .map_err(engine_failure)??;
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
        canonical_payload: semio_framework_pack_json::to_json_string(&result).into_bytes(),
        validity: if diagnostics.is_empty() { "valid" } else { "invalid" }.to_string(),
        diagnostics,
        quality,
        complete: step.done,
        actual_cache_mode: request.requested_cache_mode.clone(),
    })
}
//#endregion 🔖️Service

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
