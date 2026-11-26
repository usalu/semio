//! 🔎️ `analyseModel` confirms the completed inference of the exact mounted document revision.
//! The reference handler computes inference when used outside a mounted view.

use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use crate::editor::bim::{BimCommand, BimModelApp};
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::{Analysis, MountedAnalysisAccess};
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_plugin::{ArtifactView, ConfigView, EditorApp, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

/// ⏩️ Graph nodes one job step may finish.
pub const NODES_PER_STEP: usize = 256;
/// 🗣️ The progress text of a step, English and German.
pub const PREVIEW: &[u8] = br#"{"en":"Analysing the model","de":"Modell wird analysiert"}"#;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "analyse-model")]
pub struct AnalyseModel {
    /// 🔘️ The state a toggle that triggers the analysis reports; the analysis ignores it.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pressed: Option<bool>,
}

fn ended(error: protocol::InferenceError) -> Fault {
    fault("bim.analyse.inference", format!("the model could not be analysed: {error}"))
}

/// 🧵️ The mounted analysis command borrows the current rendered revision under its own operation.
pub struct AnalyseWork {
    tool_id: &'static str,
    owner: semio_plugin::ArtifactInstanceOperationOwnerHandle,
    access: MountedAnalysisAccess,
    complete: bool,
}

impl AnalyseWork {
    /// 🏗️ The work of tool `tool_id`.
    pub fn new(tool_id: &'static str, owner: semio_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, owner, access: Default::default(), complete: false }
    }
}

impl ArtifactCommandWork<EditorApp<BimModelApp>> for AnalyseWork {
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(::core::mem::size_of::<Self>())
    }

    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn terminal_is_empty(&self) -> bool {
        self.access.terminal_is_empty()
    }

    fn extent(&self, command: &BimCommand, _snapshot: &ModelSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BimModelApp>>>) -> Option<usize> {
        (!self.complete && command.command_id() == self.tool_id).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BimModelApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BimModelApp>>, semio_framework_job::OriginalTurnFailure> {
        if self.complete || !matches!(input.command, BimCommand::AnalyseModel(_)) {
            return Err(semio_framework_value::ValueError::invariant("original BIM analysis command changed or completed").into());
        }
        if cx.is_cancelled() {
            return Err(Fault::from("bim.analyse.cancelled").into());
        }
        let ready = Analysis::read_rendered(&self.owner, &mut self.access, input.operation.canonical_base_revision, cx, |_, cx| cx.charge_work_unit())?;
        if ready == Some(true) {
            self.complete = true;
            return Ok(ArtifactCommandWorkStep::Complete(Emit::default()));
        }
        cx.set_stage("bim-analyse");
        Ok(ArtifactCommandWorkStep::Progress { stage: "bim-analyse", preview: PREVIEW })
    }
}

/// 🔎️ The synchronous path: the session settled at the document in one go.
pub fn handle(_payload: &AnalyseModel, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::try_with_inference(ctx.gestures.as_ref(),doc.snapshot,|_|()).map_err(ended)?;
    Ok(Emit::default())
}

