//! 🧵️ The viewer's bounded configuration-only contribution installation.
use super::*;
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactory};
use semio_plugin::{ArtifactOwnedToolJobFactory, ArtifactToolPublicationContract, ArtifactToolPublicationLane, InteractiveJobClassification, ViewerApp};
use semio_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};

const MAXIMUM_BYTES: usize = 16_384;

fn contract() -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(MAXIMUM_BYTES, 64, 1, 4_096, 7_500) }

fn extent(_command: &FormsViewCommand, _snapshot: &FormsSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> { Some(1) }

fn reduce(
    command: &FormsViewCommand,
    snapshot: &FormsSnapshot,
    config: &FormsConfig,
    history: &semio_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_plugin::app::InteractionHoverState,
    context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<ViewerApp<FormsViewer>>>,
    operation: &semio_plugin::AppOperationContext,
) -> Result<semio_plugin::Emit<FormMutation, FormsConfigMutation>, Fault> {
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: context.and_then(|value| value.window_config.as_ref()) };
    let emit = command.dispatch(&doc, &cfg)?;
    Ok(semio_plugin::Emit { config_mutations: emit.config_mutations, effects: emit.effects, ui_scope: emit.ui_dirty, ..Default::default() })
}

pub(super) struct FormsViewContributionFactory { keys: Vec<ToolFactoryKey> }

impl FormsViewContributionFactory {
    pub(super) fn new(controller: &str) -> Self { Self { keys: vec![ToolFactoryKey::new(controller, "setContributions")] } }
}

impl ToolJobFactory for FormsViewContributionFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<FormsViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<FormsViewer>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { "forms.view.contributions.v1" }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { contract() }
    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: MAXIMUM_BYTES, checkpoint_resume: false);
}

impl ArtifactOwnedToolJobFactory for FormsViewContributionFactory {
    type Owner = ViewerApp<FormsViewer>;
    const TOOL_IDS: &'static [&'static str] = &["setContributions"];
    const DOCUMENT_SCHEMA: &'static str = FORMS_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::Config] }];
}

pub(super) fn build(
    request: &mut Option<semio_plugin::ArtifactOwnedToolJobRequest<ViewerApp<FormsViewer>>>,
    output: &mut Option<semio_plugin::ErasedToolJob>,
    cx: &mut semio_framework_job::StepContext<'_>,
) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    let original = request.as_ref().ok_or_else(|| semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "original Forms viewer command is absent"))?;
    if original.tool_id != "setContributions" { return Ok(false); }
    if original.command.command_id() != original.tool_id { return Err(semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "Forms viewer command does not match its tool").into()); }
    semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, FormsViewCommand::command_id, MAXIMUM_BYTES, 1, false, |_| BoundedArtifactCommandWork::new("setContributions", reduce, extent))
}

struct FormsViewContributionProofs;

impl FormsViewContributionProofs {
    semio_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<FormsViewer>,
        owner_file: "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.forms.forms@1/*#viewer",
        artifact_schema: "forms.form",
        factory: "FormsViewContributionFactory",
        factory_type: FormsViewContributionFactory,
        contract: contract(),
        tools: ["setContributions"]
    }
}

pub(super) fn proofs() -> Vec<semio_plugin::ArtifactBoundedFirstStepProof> { FormsViewContributionProofs::bounded_first_step_tool_proofs() }
