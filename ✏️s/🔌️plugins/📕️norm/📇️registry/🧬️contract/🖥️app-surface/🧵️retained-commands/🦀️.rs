//! 🧵️ Retained commands of the norm app surface: the publication contracts, the retained editor trait, and the tool job builder. See [`super`].

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
pub const NORM_RETAINED_TOOL_IDS: &[&str] = &["setSelectedCheckIndex", "setField", "insertItem", "removeItem", "applyRemedy"];

/// 🧬️ The payload schema id every norm retained command job is admitted under.
pub const NORM_RETAINED_PAYLOAD_SCHEMA: &str = "norm.tool-command.v1";

/// 🎒️ Wire ceiling for one norm tool dispatch: the largest payload is an inserted row, a few dozen scalar quantities — kilobytes, never megabytes.
pub const NORM_RETAINED_RAW_BYTES: usize = 524_288;

/// 🚦️ Per-tool publication lanes, read straight off the command bodies: `set-field`/`insert-item`/`remove-item`/`apply-remedy` commit
/// artifact mutations, and
/// `selected-check` writes persisted-local state through the exact Results-window config lane.
pub const NORM_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setSelectedCheckIndex", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setField", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "insertItem", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeItem", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "applyRemedy", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

/// 🫧️ Publication lanes of the live-editing tools: `setFields` commits artifact mutations in one batch, the preview and progress tools only ever touch the transient lane.
pub const NORM_LIVE_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setFields", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "previewRemedy", lanes: &[ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "clearPreview", lanes: &[ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "cancelEvaluation", lanes: &[ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "evaluateUnits", lanes: &[ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "viewportAct", lanes: &[ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "copyEntities", lanes: &[ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "duplicateEntities", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "pasteEntities", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "moveEntity", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "deleteEntities", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

/// 🚦️ The publication contracts of [`NORM_EDITOR_TOOL_IDS`], in the same order.
pub const NORM_EDITOR_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setSelectedCheckIndex", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setField", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "insertItem", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeItem", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "applyRemedy", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setFields", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Transient, ArtifactToolPublicationLane::Presence] },
    ArtifactToolPublicationContract { tool_id: "previewRemedy", lanes: &[ArtifactToolPublicationLane::Transient, ArtifactToolPublicationLane::Presence] },
    ArtifactToolPublicationContract { tool_id: "clearPreview", lanes: &[ArtifactToolPublicationLane::Transient, ArtifactToolPublicationLane::Presence] },
    ArtifactToolPublicationContract { tool_id: "cancelEvaluation", lanes: &[ArtifactToolPublicationLane::Transient, ArtifactToolPublicationLane::Presence] },
    ArtifactToolPublicationContract { tool_id: "evaluateUnits", lanes: &[ArtifactToolPublicationLane::Transient, ArtifactToolPublicationLane::Presence] },
    ArtifactToolPublicationContract { tool_id: "viewportAct", lanes: &[ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "copyEntities", lanes: &[ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "duplicateEntities", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "pasteEntities", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "moveEntity", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "deleteEntities", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

/// 📏️ Work items of a live command: a `setFields` bundle is one bounded step however many leaves it writes (one batched edit), as is every other live tool.
pub const fn norm_live_extent(_command: &LiveCommand) -> usize {
    1
}

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
    const TOOL_IDS: &'static [&'static str] = NORM_RETAINED_TOOL_IDS;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = NORM_PUBLICATION_CONTRACTS;
    type Family: NormFamily<Document = Self::Snapshot, Mutation = Self::Mutation>;
    type ResultsWindowConfigOwner: WindowConfigOwner<State = semio_s_plugin_norm_results_config::schema::NormResultsWindowConfig, Mutation = semio_s_plugin_norm_results_config::mutations::NormResultsWindowConfigMutation>;

    /// 🪺️ The exact native edit vocabulary used by inspector form preparation.
    fn inspector_rules() -> &'static NormEditRules;

    /// 🧭️ The family resolver retains its typed semantic admission and special handlers.
    fn inspector_resolve(document: &Self::Snapshot, edit: &NormEdit) -> Result<Vec<Self::Mutation>, String>;

    fn selected_check_window_mutation(command: &Self::Command) -> Option<semio_s_plugin_norm_results_config::mutations::NormResultsWindowConfigMutation>;

    /// 🔗️ The view effects of a click on a check row (the selection of its owner entity, see [`live_check_focus_effects`]); none for apps without a viewport.
    fn check_focus_effects(_command: &Self::Command, _document: &Self::Snapshot) -> Vec<semio_framework_plugin::Effect> {
        Vec::new()
    }

    /// 🎹️ Interaction effects of an ephemeral view command, resolved from its captured transient and interaction roots.
    fn view_focus_effects(_command: &Self::Command, _document: &Self::Snapshot, _transient: &Self::Transient, _interaction: &protocol::InteractionState) -> Vec<semio_framework_plugin::Effect> {
        Vec::new()
    }

    fn dispatch_retained(command: &Self::Command, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>) -> NormRetainedCommandResult<Self::Mutation>;
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
        effects: A::check_focus_effects(command, doc.snapshot),
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
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<A>>>,
    operation: &semio_framework_plugin::AppOperationContext,
) -> NormRetainedCommandResult<A::Mutation> {
    if !A::TOOL_IDS.contains(&A::command_id(command)) {
        return Err(Fault::from("norm-command-retained-route-rejected"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let window = context.and_then(|context| context.window_config.as_ref());
    let mut emit = dispatch_norm_command::<A>(command, &doc, &ConfigView { snapshot: config, window }, context.and_then(|context| context.view_state.as_deref()))?;
    if let Some(context) = context {
        emit.effects.extend(A::view_focus_effects(command, snapshot, context.transient.get(), interaction));
    }
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
        Self { keys: A::TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect(), owner: std::marker::PhantomData }
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
    const TOOL_IDS: &'static [&'static str] = A::TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = <A as semio_framework_plugin::ArtifactEditor>::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = A::PUBLICATION_CONTRACTS;
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
    let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "norm-command-retained-producer-request-absent"))?;
    if !A::TOOL_IDS.contains(&original.tool_id.as_str()) {
        return Ok(false);
    }
    let tool_id = A::command_id(&original.command);
    if tool_id != original.tool_id {
        return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "norm-command-tool-mismatch").into());
    }
    semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, A::command_id, NORM_RETAINED_RAW_BYTES, 1, true, |request| -> Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<A>>> { Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, norm_retained_reduce::<A>, norm_bounded_extent::<A>)) })
}
//#endregion 🔌️EditorOverrides

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod publication_tests;
