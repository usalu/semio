//! 🔎️ `analyseModel`: brings the instance's inference session up to the document as a job. Every window, gesture and export reads its derived values from that session; a big edit, an undo or a load
//! leaves it far behind, and the read that finds it so would block. The job does the same work in bounded steps instead: a progress stage per step, a cancel that keeps every node finished so far,
//! and a settled session afterwards, so the next read answers from memory. [`AnalyseWork`] is its retained-command shell, which honours `StepContext::is_cancelled`. The dispatch path [`handle`]
//! settles the session in one go for callers that do not run jobs.

use crate::editor::bim::kit::fault;
use crate::editor::bim::{BimCommand, BimDispatchCtx, BimModelApp};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::Analysis;
use crate::{ModelMutation, ModelSnapshot};
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
}

impl AnalyseWork {
    /// 🏗️ The work of tool `tool_id`.
    pub fn new(tool_id: &'static str, owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, owner, analysis: None }
    }
}

impl ArtifactCommandWork<EditorApp<BimModelApp>> for AnalyseWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &BimCommand, _snapshot: &ModelSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BimModelApp>>>) -> Option<usize> {
        (self.analysis.is_none() && command.command_id() == self.tool_id).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BimModelApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BimModelApp>>, Fault> {
        if !matches!(input.command, BimCommand::AnalyseModel(_)) {
            return Err(Fault::from("bim-analyse-work-mismatch"));
        }
        let analysis = self.analysis.get_or_insert_with(|| Analysis::new(Some(&self.owner), NODES_PER_STEP));
        if cx.is_cancelled() {
            analysis.cancel(input.snapshot);
            return Err(fault("bim.analyse.cancelled", "the analysis was cancelled"));
        }
        cx.consume_fuel(1);
        if analysis.advance(input.snapshot).map_err(ended)?.done {
            return Ok(ArtifactCommandWorkStep::Complete(Emit::default()));
        }
        cx.set_stage("bim-analyse");
        Ok(ArtifactCommandWorkStep::Progress { stage: "bim-analyse", preview: PREVIEW })
    }
}

/// 🔎️ The synchronous path: the session settled at the document in one go.
pub fn handle(_payload: &AnalyseModel, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let mut analysis = Analysis::new(ctx.gestures.as_ref(), usize::MAX);
    while !analysis.is_done() {
        analysis.advance(doc.snapshot).map_err(ended)?;
    }
    Ok(Emit::default())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
