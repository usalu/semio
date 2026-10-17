//! 🎞️ Sequence reserved media import job.

use neural_engine::Dictionary;
use semio_framework_plugin::{DslValue, Emit, Fault, Media, MediaPayload, NoConfigMutation};
use crate::mutations::SequenceMutation;
use crate::{SequenceSnapshot, SequenceStep, SequenceWorkingScene, StepParams};
use crate::editor::sequence::edit_rules;
use crate::editor::sequence::retained::SEQUENCE_STORE_MAXIMUM_BYTES;
use crate::editor::sequence::app::SequencePlayApp;
use crate::editor::sequence::host::max_serial_in_snapshot;
use crate::editor::sequence::base::{sequence_child_leaves_emit, sequence_host_snapshot_from_children};
use crate::editor::sequence::persistent::sequence_close_step;

//#region 🎞️ReservedImport
/// 🎞️ The reserved tool id the framework registers for every app's media import.
pub const SEQUENCE_IMPORT_TOOL_ID: &str = "import-media";

/// 🎞️ The only media port Sequence declares as an importer (`sequence_io`).
const SEQUENCE_IMPORT_PORT: &str = "steps:in";

/// 🎞️ The ONE concrete resumable importer this app owns. `VcsArtifactApp::dispatch_import_media`
/// builds its emit EXCLUSIVELY from the resumable job — `A::import_media` is never reached on a
/// mounted app — so while this returned the trait default `None` every `steps:in` delivery died
/// `interactive-job.missing-reserved-builder` although the synchronous importer below was correct
/// (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, media §8, S10-E). Original admitted media
/// remains owned through native parsing/projection, typed binding and the same composed child
/// publication. Every source candidate drains through its existing retirement authority.
pub struct SequenceImportJob {
    pub port: Option<String>,
    pub media: Option<Media>,
    parser: Option<semio_framework_pack_json::JsonParseCursor>,
    projection: Option<semio_framework_pack_json::JsonValueProjection>,
    decode_receipt: Option<semio_framework_value::native_decoding::NativeDecodeContinuation>,
    pub input: Option<neural_engine::retirement::RetainedDictionaryInput>,
    pub params: Option<Dictionary>,
    pub retirement: Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    snapshot: Option<std::sync::Arc<SequenceSnapshot>>,
    children: Option<semio_framework_plugin::app::ChildContentView>,
    emit: Option<Emit<SequenceMutation, NoConfigMutation>>,
    checkpoint_publication: semio_framework_job_publication::RetainedJobPublication,
    checkpoint_delivered: bool,
    fault_publication: semio_framework_job_publication::RetainedJobPublication,
    fault_detail: Option<Vec<u8>>,
    decoded: bool,
    completed: bool,
    closing: bool,
    completion: Option<semio_framework_plugin::ArtifactToolCompletion<semio_framework_plugin::EditorApp<SequencePlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<semio_framework_plugin::EditorApp<SequencePlayApp>>>,
}

type SequenceJobTurn<'a> = Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError>;

enum SequenceDecodeTurn {
    Cancelled,
    Yield,
    Done,
}

/// 📦️ Takes the original parameter value into its canonical dictionary shape.
fn sequence_import_parameter_value(value: DslValue) -> DslValue {
    if matches!(value, DslValue::Object(_)) { value } else { DslValue::Object(vec![("value".into(), value)]) }
}

/// 🎟️ The exact one-turn grant a quoted demand needs.
fn sequence_demand_grant(demand: semio_framework_value::RetirementDemand) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }
}

/// 🧾️ Projects one JSON grammar refusal onto the shared value refusal.
fn sequence_json_error(error: semio_framework_pack_json::JsonError) -> semio_framework_value::ValueError {
    match error {
        semio_framework_pack_json::JsonError::Native(error) => error,
        error => semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()),
    }
}

/// 🎟️ Quotes the grant for the next typed input binding turn.
fn sequence_input_grant(input: &neural_engine::retirement::RetainedDictionaryInput) -> Result<semio_framework_value::retained_clone::RetainedCloneGrant, semio_framework_value::ValueError> {
    let copy_bytes = input.next_copy_byte_demand()?;
    Ok(sequence_demand_grant(semio_framework_value::RetirementDemand { copy_bytes, capacity_bytes: input.next_capacity_byte_demand(copy_bytes)?, release_bytes: input.next_release_byte_demand()?, depth: input.next_depth_demand()?.max(1) }))
}

impl SequenceImportJob {
    pub fn new(request: semio_framework_plugin::ArtifactReservedToolJobRequest<semio_framework_plugin::EditorApp<SequencePlayApp>>, port: String, media: Media) -> Self {
        Self {
            port: Some(port),
            media: Some(media),
            parser: None,
            projection: None,
            decode_receipt: None,
            input: None,
            params: None,
            retirement: None,
            snapshot: Some(request.snapshot),
            children: Some(request.children),
            emit: None,
            checkpoint_publication: semio_framework_job_publication::RetainedJobPublication::new(),
            checkpoint_delivered: false,
            fault_publication: semio_framework_job_publication::RetainedJobPublication::new(),
            fault_detail: None,
            decoded: false,
            completed: false,
            closing: false,
            completion: Some(request.completion),
            pending_completion_rejection: None,
        }
    }

    pub fn fault<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>, detail: &str) -> SequenceJobTurn<'a> {
        if self.fault_detail.is_none() {
            let bytes = detail.as_bytes();
            self.fault_detail = Some(bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)].to_vec());
        }
        let bytes = self.fault_detail.as_deref().unwrap_or_default();
        self.fault_publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Fault, bytes, cx)
    }

    fn decode(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<SequenceDecodeTurn, String> {
        if self.port.as_deref() != Some(SEQUENCE_IMPORT_PORT) {
            return Err("sequence import only implements steps:in".into());
        }
        if self.params.is_none() {
            if self.input.is_none() && self.parser.is_none() && self.projection.is_none() {
                match self.media.as_mut().map(|media| &mut media.payload) {
                    Some(MediaPayload::Intrinsic { value, .. }) => {
                        let value = std::mem::replace(value, DslValue::Null);
                        self.input = Some(neural_engine::retirement::RetainedDictionaryInput::new(sequence_import_parameter_value(value)));
                    },
                    Some(MediaPayload::Structured { .. }) => self.parser = Some(semio_framework_pack_json::JsonParseCursor::new(semio_framework_pack_json::JsonMemberPolicy::Reject)),
                    Some(MediaPayload::Binary { .. }) | None => return Err("sequence steps:in requires an intrinsic or structured parameter value".into()),
                }
                cx.consume_fuel(1);
            }
            while !cx.should_yield() {
                if cx.is_cancelled() { return Ok(SequenceDecodeTurn::Cancelled); }
                if let Some(input) = self.input.as_mut() {
                    cx.set_stage(input.progress().2);
                    let grant = sequence_input_grant(input).map_err(|error| error.message.to_string())?;
                    match input.step(grant) {
                        Ok(step) => {
                            if let Some(dictionary) = step.dictionary {
                                self.params = Some(dictionary);
                                cx.consume_fuel(1);
                                break;
                            }
                        },
                        Err(error) if error.kind == semio_framework_value::ValueRefusalKind::Canceled => return Ok(SequenceDecodeTurn::Cancelled),
                        Err(error) => return Err(error.message.to_string()),
                    }
                } else {
                    let result = {
                        let mut accept = |_| !cx.is_cancelled();
                        let control = match self.decode_receipt.take() { Some(receipt) => semio_framework_value::NativeDecodeControl::resume(receipt, &mut accept), None => Ok(semio_framework_value::NativeDecodeControl::new(SEQUENCE_STORE_MAXIMUM_BYTES, &mut accept)) };
                        match control {
                            Ok(mut control) => {
                                let result = if let Some(projection) = self.projection.as_mut() {
                                    match projection.normal_step_demands() {
                                        Ok(demand) => projection.step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, &mut control, sequence_demand_grant(demand)),
                                        Err(error) => Err(error),
                                    }
                                } else if let (Some(parser), Some(MediaPayload::Structured { json, .. })) = (self.parser.as_mut(), self.media.as_ref().map(|media| &media.payload)) {
                                    match parser.normal_step_demands(json.as_str()).and_then(|demand| parser.step(json.as_str(), 1, &mut control, sequence_demand_grant(demand))) {
                                        Ok(Some(value)) => { self.projection = Some(semio_framework_pack_json::JsonValueProjection::new(value)); Ok(None) },
                                        Ok(None) => Ok(None),
                                        Err(error) => Err(sequence_json_error(error)),
                                    }
                                } else { Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "sequence import source disappeared")) };
                                match control.pause() { Ok(receipt) => { self.decode_receipt = Some(receipt); result }, Err(error) => Err(error) }
                            },
                            Err(error) => Err(error),
                        }
                    };
                    match result {
                        Ok(Some(value)) => self.input = Some(neural_engine::retirement::RetainedDictionaryInput::new(sequence_import_parameter_value(value))),
                        Ok(None) => {},
                        Err(error) if error.kind == semio_framework_value::ValueRefusalKind::Canceled => return Ok(SequenceDecodeTurn::Cancelled),
                        Err(error) => return Err(error.message.to_string()),
                    }
                    cx.set_stage(if self.projection.is_some() { "sequence-import-project" } else { "sequence-import-parse" });
                }
                cx.consume_fuel(1);
            }
            if self.params.is_none() {
                return Ok(SequenceDecodeTurn::Yield);
            }
        }
        let (Some(snapshot), Some(children)) = (self.snapshot.as_ref(), self.children.as_ref()) else {
            return Err("sequence import lost its snapshot authority".into());
        };
        let Ok(live) = sequence_host_snapshot_from_children(snapshot.as_ref(), children) else {
            return Err("sequence import could not read its composed flow child".into());
        };
        let id = format!("step-{}", max_serial_in_snapshot(&live).max(100) + 1);
        let x = live.steps.iter().map(|step| step.x).fold(0.0_f64, f64::max) + if live.steps.is_empty() { 0.0 } else { 280.0 };
        let mut edit = edit_rules::SceneEdit::new(SequenceWorkingScene { steps: live.steps.clone(), edges: live.edges.clone() });
        let params = StepParams(self.params.take().expect("retained parameter binding completed"));
        edit.insert_step(SequenceStep { id, kind: "computation.import".into(), params, x, y: 0.0, slot: None, collapsed: false });
        self.emit = Some(sequence_child_leaves_emit(snapshot.as_ref(), edit.leaves));
        self.children = None;
        self.decoded = true;
        Ok(SequenceDecodeTurn::Done)
    }

    fn nested(mut demand: semio_framework_value::RetirementDemand) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "sequence import close depth overflow"))?;
        Ok(demand)
    }

    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if let Some(rejected) = self.pending_completion_rejection.as_ref() {
            if let Ok(emit) = rejected.emit.as_ref() {
                if let Some(demand) = emit.child_close_demands(body)? {
                    return Ok(demand);
                }
            }
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        if let Some(emit) = self.emit.as_ref() {
            if let Some(demand) = emit.child_close_demands(body)? {
                return Ok(demand);
            }
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        if let Some(active) = self.retirement.as_ref() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?);
        }
        if self.children.is_some() {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        if let Some(input) = self.input.as_ref() {
            let copy_bytes = input.next_close_copy_byte_demand()?;
            return Ok(semio_framework_value::RetirementDemand { copy_bytes, capacity_bytes: input.next_close_capacity_byte_demand(copy_bytes)?, release_bytes: input.next_close_release_byte_demand()?, depth: input.next_close_depth_demand()?.max(1) });
        }
        if self.params.is_some() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.params)?);
        }
        if self.media.is_some() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.media)?);
        }
        if self.parser.is_some() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.parser)?);
        }
        if self.projection.is_some() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.projection)?);
        }
        if self.port.is_some() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.port)?);
        }
        if self.completion.is_some() {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        if self.snapshot.is_some() {
            return Ok(semio_framework_value::RetirementDemand { capacity_bytes: semio_framework_value::shared_retirement_birth_bytes::<SequenceSnapshot>(), depth: 2, ..Default::default() });
        }
        if !self.fault_publication.terminal_is_empty() {
            return Self::nested(self.fault_publication.retirement_demands()?);
        }
        if !self.checkpoint_publication.terminal_is_empty() {
            return Self::nested(self.checkpoint_publication.retirement_demands()?);
        }
        Ok(Default::default())
    }

    fn close_original(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let unit = || Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        if semio_framework_job::InteractiveJob::terminal_is_empty(self) {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let child = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        let fault = |fault: Fault| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, fault.message);
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            if let Ok(emit) = rejected.emit.as_mut() {
                if let Some(step) = emit.close_child_one(grant).map_err(fault)? {
                    return Ok(RetainedCloneStep::Progress(step.progress().unwrap_or_default()));
                }
            }
            self.pending_completion_rejection = None;
            return unit();
        }
        if let Some(emit) = self.emit.as_mut() {
            if let Some(step) = emit.close_child_one(grant).map_err(fault)? {
                return Ok(RetainedCloneStep::Progress(step.progress().unwrap_or_default()));
            }
            self.emit = None;
            return unit();
        }
        if self.retirement.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.children.take().is_some() {
            return unit();
        }
        if let Some(input) = self.input.as_mut() {
            input.cancel();
            let step = input.close_step(grant)?;
            if input.terminal_is_empty() { self.input = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.params.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.params, &mut self.retirement, child);
        }
        if self.media.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.media, &mut self.retirement, child);
        }
        if self.parser.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.parser, &mut self.retirement, child);
        }
        if self.projection.is_some() {
            self.decode_receipt = None;
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.projection, &mut self.retirement, child);
        }
        if self.port.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.port, &mut self.retirement, child);
        }
        if self.completion.take().is_some() {
            return unit();
        }
        if let Some(snapshot) = self.snapshot.take() {
            return match semio_framework_value::admit_shared_retirement(snapshot, child, true) {
                Ok((owner, progress)) => {
                    self.retirement = Some(owner);
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, snapshot)) => {
                    self.snapshot = Some(snapshot);
                    Err(error)
                }
            };
        }
        if !self.fault_publication.terminal_is_empty() {
            return self.fault_publication.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if !self.checkpoint_publication.terminal_is_empty() {
            return self.checkpoint_publication.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
}

impl semio_framework_job::InteractiveJob for SequenceImportJob {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> SequenceJobTurn<'a> {
        if cx.is_cancelled() {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx);
        }
        if self.fault_detail.is_some() {
            return self.fault(cx, "");
        }
        if self.pending_completion_rejection.is_some() {
            return self.fault(cx, "sequence import completion remains rejected");
        }
        if !self.decoded {
            cx.set_stage("sequence-import-decode");
            match self.decode(cx) {
                Err(detail) => return self.fault(cx, &detail),
                Ok(SequenceDecodeTurn::Cancelled) => return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx),
                Ok(SequenceDecodeTurn::Yield) => return semio_framework_job::JobOutcomeBorrow::admit_yield(cx),
                Ok(SequenceDecodeTurn::Done) => {
                    cx.consume_fuel(1);
                }
            }
        }
        if !self.checkpoint_delivered {
            let outcome = self.checkpoint_publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Checkpoint { applied_progress: 1 }, &[1], cx)?;
            if outcome.is_some() {
                self.checkpoint_delivered = true;
            }
            return Ok(outcome);
        }
        cx.set_stage("sequence-import-publish");
        if let Some(emit) = self.emit.as_mut() {
            let demand = emit.child_preparation_demands(0)?;
            match emit.prepare_child_one(sequence_demand_grant(demand)) {
                Ok(semio_framework_plugin::app::ChildEmitPreparationStep::Ready(_)) => {}
                Ok(semio_framework_plugin::app::ChildEmitPreparationStep::Pending(_)) => {
                    cx.consume_fuel(1);
                    return semio_framework_job::JobOutcomeBorrow::admit_yield(cx);
                }
                Ok(semio_framework_plugin::app::ChildEmitPreparationStep::Refused(fault, _)) | Err(fault) => return self.fault(cx, &fault.message),
            }
        }
        if !self.completed {
            let Some(emit) = self.emit.take() else {
                return self.fault(cx, "sequence import lost its decoded child publication");
            };
            let Some(completion) = self.completion.as_ref() else {
                self.emit = Some(emit);
                return self.fault(cx, "sequence import lost its completion authority");
            };
            if !completion.has_mounted_consumer() {
                self.emit = Some(emit);
                return self.fault(cx, "sequence import completion consumer is absent");
            }
            if let Err(rejected) = completion.complete(Ok(emit), semio_framework_plugin::EphemeralEmit::default()) {
                let message = rejected.fault.message.clone();
                self.pending_completion_rejection = Some(rejected);
                return self.fault(cx, &message);
            }
            self.completed = true;
        }
        semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(None, None),
            semio_framework_job::JobOutcomeKind::CheckpointReady { .. } => self.checkpoint_publication.borrow_outcome(descriptor),
            semio_framework_job::JobOutcomeKind::Fault => self.fault_publication.borrow_outcome(descriptor),
            semio_framework_job::JobOutcomeKind::PreviewReady => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "sequence import publishes no preview")),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.closing = true;
        sequence_close_step(self.close_original(grant))
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.port.is_none()
            && self.media.is_none() && self.parser.is_none() && self.projection.is_none() && self.input.is_none() && self.params.is_none() && self.retirement.is_none()
            && self.children.is_none()
            && self.snapshot.is_none()
            && self.emit.is_none()
            && self.completion.is_none()
            && self.pending_completion_rejection.is_none()
            && self.fault_publication.terminal_is_empty()
            && self.checkpoint_publication.terminal_is_empty()
    }
}

impl semio_framework_plugin::ArtifactReservedJob for SequenceImportJob {}
//#endregion 🎞️ReservedImport
