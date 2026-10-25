//! 🧵️ Retained commands of the norm app surface: the publication contracts, the retained editor trait, the evaluate work and the tool job builder. See [`super`].

use super::*;

//#region 🔖️Views
/// 📄️ Reads the document out of a `ArtifactView` — spelled once so every app's `render`/`handle` reads
/// it the same way.
pub fn snapshot<'a, D>(doc: &'a ArtifactView<'_, D>) -> &'a D {
    doc.snapshot
}
//#endregion 🔖️Views

//#region 🧵️RetainedCommands
/// 🧾️ Every norm tool id, in `app_commands!` row order. All fifteen apps declare exactly this set, so
/// the list, [`NORM_PUBLICATION_CONTRACTS`], every factory key set and every `bounded_first_step_tool_proofs!`
/// block are driven from this one constant.
pub const NORM_RETAINED_TOOL_IDS: &[&str] = &["evaluate", "setSelectedCheckIndex", "setField", "insertItem", "removeItem", "applyRemedy"];

/// 🧬️ The payload schema id every norm retained command job is admitted under.
pub const NORM_RETAINED_PAYLOAD_SCHEMA: &str = "norm.tool-command.v1";

/// 🎒️ Wire ceiling for one norm tool dispatch: the largest payload is an inserted row, a few dozen scalar quantities — kilobytes, never megabytes.
pub const NORM_RETAINED_RAW_BYTES: usize = 524_288;

/// 🚦️ Per-tool publication lanes, read straight off the command bodies: `set-field`/`insert-item`/`remove-item`/`apply-remedy` commit
/// artifact mutations, `evaluate` emits nothing at all (the report is derived on every read), and
/// `selected-check` writes persisted-local state through the exact Results-window config lane.
pub const NORM_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "evaluate", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "setSelectedCheckIndex", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setField", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "insertItem", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeItem", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "applyRemedy", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

/// 🛣️ Stable language-neutral identifier for one live publication lane.
pub const fn publication_lane_id(lane: ArtifactToolPublicationLane) -> &'static str {
    match lane {
        ArtifactToolPublicationLane::HostOnly => "host-only",
        ArtifactToolPublicationLane::Artifact => "artifact",
        ArtifactToolPublicationLane::Config => "config",
        ArtifactToolPublicationLane::Draft => "draft",
        ArtifactToolPublicationLane::Presence => "presence",
        ArtifactToolPublicationLane::Transient => "transient",
        ArtifactToolPublicationLane::WindowConfig => "window-config",
        ArtifactToolPublicationLane::WindowTransient => "window-transient",
        ArtifactToolPublicationLane::Child => "child",
        ArtifactToolPublicationLane::Interaction => "interaction",
    }
}

/// ⏱️ The one bounded-first-step contract all forty-five norm tool identities share.
pub fn norm_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(NORM_RETAINED_RAW_BYTES, 64, 64, 65_536, 7_999)
}

/// 🧵️ The per-app half of the shared factory: an editor states only how its own aggregated command enum
/// dispatches, and inherits every retained-command constant, reducer, factory and store preparation
/// below. `dispatch_retained` MUST route into the app's `🎮️commands/*` bodies, which stay the sole
/// authority for what a norm command does.
pub trait NormRetainedEditor: semio_framework_plugin::ArtifactEditor<Config = NoConfig, ConfigMutation = NoConfigMutation, DraftMutation = semio_framework_plugin::NoDraftMutation> {
    type Family: NormFamily<Document = Self::Snapshot, Mutation = Self::Mutation>;
    type ResultsWindowConfigOwner: WindowConfigOwner<State = semio_s_plugin_norm_results_config::schema::NormResultsWindowConfig, Mutation = semio_s_plugin_norm_results_config::mutations::NormResultsWindowConfigMutation>;

    fn selected_check_window_mutation(command: &Self::Command) -> Option<semio_s_plugin_norm_results_config::mutations::NormResultsWindowConfigMutation>;

    fn dispatch_retained(command: &Self::Command, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>) -> NormRetainedCommandResult<Self::Mutation>;
}

/// 🧮️ Cooperative evaluate-job state (progress phases + cancel + checkpoint bytes).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NormEvaluateWorkState {
    pub phase: u8,
    pub cancelled: bool,
}

impl NormEvaluateWorkState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn checkpoint_byte(&self,index:usize)->Option<u8>{(index==0).then_some(self.phase)}

    pub fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        self.phase = checkpoint.first().copied().unwrap_or(0);
        self.cancelled = false;
        Ok(())
    }

    pub fn begin_close(&mut self) {
        self.cancelled = true;
    }

    pub fn emit_progress_stage(&mut self) -> Option<&'static str> {
        match self.phase {
            0 => {
                self.phase = 1;
                Some("norm-evaluate-prepare")
            }
            1 => {
                self.phase = 2;
                Some("norm-evaluate-run")
            }
            _ => None,
        }
    }

    pub fn ready_to_evaluate(&self) -> bool {
        self.phase >= 2 && !self.cancelled
    }
}

/// 🧮️ Multi-step retained `evaluate` work: progress events, cooperative cancel, checkpoint/restore, revision-keyed report cache.
pub struct NormEvaluateCommandWork<A: NormRetainedEditor> {
    pub(super) tool_id: &'static str,
    pub(super) state: NormEvaluateWorkState,
    _owner: std::marker::PhantomData<fn() -> A>,
}

impl<A: NormRetainedEditor> NormEvaluateCommandWork<A> {
    pub fn new(tool_id: &'static str) -> Self {
        Self { tool_id, state: NormEvaluateWorkState::new(), _owner: std::marker::PhantomData }
    }
}

impl<A: NormRetainedEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<A>> for NormEvaluateCommandWork<A> {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        _command: &A::Command,
        _snapshot: &A::Snapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<A>>>,
    ) -> Option<usize> {
        Some(3)
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<A>>,
    _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.state.cancelled {
            return Ok(ArtifactCommandWorkStep::Complete(Emit::default()));
        }
        if let Some(stage) = self.state.emit_progress_stage() {
            let preview: &[u8] = match stage {
                "norm-evaluate-prepare" => br#"{"en":"Preparing evaluation","de":"Auswertung wird vorbereitet"}"#.as_slice(),
                _ => br#"{"en":"Evaluating checks","de":"Nachweise werden ausgewertet"}"#.as_slice(),
            };
            return Ok(ArtifactCommandWorkStep::Progress { stage, preview });
        }
        invalidate_cached_report_for::<A::Family>(input.snapshot);
        let report = <A::Family as NormFamily>::evaluate(input.snapshot);
        store_cached_report_for::<A::Family>(input.snapshot, report);
        let _ = document_revision_key(input.snapshot);
        let emit = norm_retained_reduce::<A>(
            input.command,
            input.snapshot,
            input.config,
            input.history,
            input.interaction,
            input.hover,
            input.context,
            input.operation,
        )?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn checkpoint_byte(&self,index:usize)->Option<u8>{self.state.checkpoint_byte(index)}

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        self.state.restore(checkpoint)
    }

    fn begin_close(&mut self) {
        self.state.begin_close();
    }
}

/// 🎯️ Routes ordinary and retained commands through the same exact Results-window address.
pub fn dispatch_norm_command<A: NormRetainedEditor>(
    command: &A::Command,
    doc: &ArtifactView<'_, A::Snapshot>,
    cfg: &ConfigView<'_, NoConfig>,
    view: Option<&semio_framework_plugin::ViewModel>,
) -> NormRetainedCommandResult<A::Mutation> {
    let Some(mutation) = A::selected_check_window_mutation(command) else {
        return A::dispatch_retained(command, doc, cfg);
    };
    let view = view.ok_or_else(|| Fault::from("norm-results-window-view-required"))?;
    Ok(Emit {
        window_config_mutations: vec![semio_s_plugin_norm_results_config::addressed::<A::ResultsWindowConfigOwner>(view, mutation)?],
        ..Default::default()
    })
}

/// 🧵️ The retained reducer shared by all fifteen apps — no norm command reads selection or hover, so the
/// interaction owners are unused and the reduction is exactly the ordinary `handle` path.
///
/// 🔁️ `artifact_mutations` is handed back in authored order: the retained publication lane stages the
/// whole bundle front-to-back into ONE batched edit (`store::begin_apply_batch`), exactly as
/// the ordinary `Emit::mutations` dispatch applies it. `XMutation::from_snapshot` emits ordered
/// `remove-layer`/`insert-layer` runs, so the published document is identical either way — the LIFO
/// compensation the old one-mutation-per-turn drain needed is gone with that drain.
#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
pub fn norm_retained_reduce<A: NormRetainedEditor>(
    command: &A::Command,
    snapshot: &A::Snapshot,
    config: &A::Config,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<A>>>,
    operation: &semio_framework_plugin::AppOperationContext,
) -> NormRetainedCommandResult<A::Mutation> {
    if !NORM_RETAINED_TOOL_IDS.contains(&A::command_id(command)) {
        return Err(Fault::from("norm-command-retained-route-rejected"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let window = context.and_then(|context| context.window_config.as_ref());
    let emit = dispatch_norm_command::<A>(command, &doc, &ConfigView { snapshot: config, window }, context.and_then(|context| context.view_state.as_ref()))?;
    Ok(emit)
}

/// 📏️ Every norm command is one bounded step; none of the three walks a collection incrementally.
pub fn norm_bounded_extent<A: NormRetainedEditor>(_command: &A::Command, _snapshot: &A::Snapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// 🏭️ The one owned bounded tool-job factory serving all fifteen norm editors — generic over the app, so
/// the per-standard `Snapshot`/`Mutation`/`Command` types are the only thing that varies and the owner
/// witness stays each app's own concrete `EditorApp<A>`.
pub struct NormBoundedCommandJobFactory<A: NormRetainedEditor> {
    pub(super) keys: Vec<semio_framework::ToolFactoryKey>,
    owner: std::marker::PhantomData<fn() -> A>,
}

impl<A: NormRetainedEditor> NormBoundedCommandJobFactory<A> {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: NORM_RETAINED_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect(), owner: std::marker::PhantomData }
    }
}

impl<A: NormRetainedEditor> semio_framework::ToolJobFactory for NormBoundedCommandJobFactory<A> {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<A>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<A>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        NORM_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework_plugin::InteractiveJobClassification {
        semio_framework_plugin::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        norm_bounded_contract()
    }

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: NORM_RETAINED_RAW_BYTES, checkpoint_resume: true);

}

impl<A: NormRetainedEditor> semio_framework_plugin::ArtifactOwnedToolJobFactory for NormBoundedCommandJobFactory<A> {
    type Owner = semio_framework_plugin::EditorApp<A>;
    const TOOL_IDS: &'static [&'static str] = NORM_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = <A as semio_framework_plugin::ArtifactEditor>::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = NORM_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 🔌️EditorOverrides
/// 📬️ `ArtifactEditor::build_artifact_store_one_item_preparation_factory` for every norm editor —
/// the store's paged document-lane preparation, the same one trinity binds, whose fold footprint every leaf
/// derives from its schema-declared inverse rows (`ArtifactStoreOneItemFootprint::for_leaf`, design §20.5).
pub fn norm_artifact_store_preparation<A: NormRetainedEditor>() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Snapshot, A::Mutation>>>
where
    A::Snapshot: semio_framework_value::retained_clone::RetainedClone,
    A::Mutation: Sync + store::ArtifactCanonicalJsonTree,
{
    Some(store::cold_mutation_apply_preparation_factory::<A::Snapshot, A::Mutation>())
}

/// 🧵️ `ArtifactEditor::build_tool_job` for every norm editor.
pub fn build_norm_tool_job<A: NormRetainedEditor>(request: &mut Option<semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<A>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
    let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
    if !NORM_RETAINED_TOOL_IDS.contains(&original.tool_id.as_str()) {
        return Ok(false);
    }
    let tool_id = A::command_id(&original.command);
    if tool_id != original.tool_id {
        return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "norm-command-tool-mismatch").into());
    }
    let maximum_work_items = if tool_id == "evaluate" { 3 } else { 1 };
    semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, A::command_id, NORM_RETAINED_RAW_BYTES, maximum_work_items, true, |request| -> Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<A>>> { if tool_id == "evaluate" {
        Box::new(NormEvaluateCommandWork::<A>::new(tool_id))
    } else {
        Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, norm_retained_reduce::<A>, norm_bounded_extent::<A>))
    } })
}
//#endregion 🔌️EditorOverrides
