//! 🔎️ `analyseModel`: brings the instance's inference session up to the document as a job. Every window, gesture and export reads its derived values from that session; a big edit, an undo or a load
//! leaves it far behind, and the read that finds it so would block. The job does the same work in bounded steps instead: a progress stage per step, a cancel that keeps every node finished so far,
//! and a settled session afterwards, so the next read answers from memory. [`AnalyseWork`] is its retained-command shell, which honours `StepContext::is_cancelled`. The dispatch path [`handle`]
//! settles the session in one go for callers that do not run jobs.

use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use crate::editor::bim::{BimCommand, BimModelApp};
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::Analysis;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_plugin::{ArtifactView, ConfigView, EditorApp, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

/// ⏩️ Graph nodes one job step may finish.
pub const NODES_PER_STEP: usize = 256;
/// 🗣️ The progress text of a step, English and German.
pub const PREVIEW: &[u8] = br#"{"en":"Analysing the model","de":"Modell wird analysiert"}"#;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "analyse-model")]
pub struct AnalyseModel {
    /// The state a toggle that triggers the analysis reports; the analysis ignores it.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pressed: Option<bool>,
}

fn ended(error: protocol::InferenceError) -> Fault {
    fault("bim.analyse.inference", format!("the model could not be analysed: {error}"))
}

/// 🧵️ The retained-command shell of an [`Analysis`]: one step per call, the framework yields between steps and tells the work when the user cancelled.
pub struct AnalyseWork {
    tool_id: &'static str,
    owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    analysis: Option<Analysis>,
    failure:Option<protocol::InferenceWorkFailure>,
    read:Option<store::os_store::snapshot_read::SnapshotRead<ModelSnapshot>>,
}

impl AnalyseWork {
    /// 🏗️ The work of tool `tool_id`.
    pub fn new(tool_id: &'static str, owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, owner, analysis: None,failure:None,read:None }
    }
}

impl ArtifactCommandWork<EditorApp<BimModelApp>> for AnalyseWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn terminal_is_empty(&self)->bool{self.read.is_none()&&self.analysis.is_none()&&self.failure.is_none()}

    fn extent(&self, command: &BimCommand, _snapshot: &ModelSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BimModelApp>>>) -> Option<usize> {
        (self.analysis.is_none() && command.command_id() == self.tool_id).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BimModelApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BimModelApp>>, semio_framework_job::OriginalTurnFailure> {
        if let Some(original)=protocol::admit_inference_failure(&mut self.failure,cx)?{return Err(original)}
        if self.failure.is_some(){return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-analyse",preview:PREVIEW})}
        if !matches!(input.command,BimCommand::AnalyseModel(_)){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original BIM analysis command changed").into())}
        if cx.should_yield(){return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-analyse",preview:PREVIEW})}
        if self.read.is_none(){self.read=input.snapshot_read.take_current(input.operation.generation,input.operation.canonical_base_revision,cx)?;return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-analyse",preview:PREVIEW})}
        if self.analysis.is_none(){let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-analyse",preview:PREVIEW})}self.analysis=Some(Analysis::new(Some(&self.owner)));cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-analyse",preview:PREVIEW})}
        let snapshot=self.read.as_ref().unwrap().get();let analysis=self.analysis.as_mut().unwrap();if cx.is_cancelled(){analysis.cancel(snapshot)}
        match analysis.advance(snapshot,cx){
            Ok(progress)if progress.done=>Ok(ArtifactCommandWorkStep::Complete(Emit::default())),
            Ok(_)=>{cx.set_stage("bim-analyse");Ok(ArtifactCommandWorkStep::Progress{stage:"bim-analyse",preview:PREVIEW})},
            Err(original)=>{self.failure=Some(original);Ok(ArtifactCommandWorkStep::Progress{stage:"bim-analyse",preview:PREVIEW})},
        }
    }
}

/// 🔎️ The synchronous path: the session settled at the document in one go.
pub fn handle(_payload: &AnalyseModel, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::try_with_inference(ctx.gestures.as_ref(),doc.snapshot,|_|()).map_err(ended)?;
    Ok(Emit::default())
}


