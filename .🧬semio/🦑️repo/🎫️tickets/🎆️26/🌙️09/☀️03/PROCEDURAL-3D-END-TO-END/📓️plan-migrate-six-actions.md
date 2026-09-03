# Plan — migrate generation3d's six `BatchOnlyPendingRewrite` actions to honest `Migrated`

## 0. Source-note correction

The prompt for this plan says to read
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️03/PROCEDURAL-3D-END-TO-END/📓️gap-six-blocked-actions.md` first.
**That file does not exist** — `ls` of the ticket directory shows only `📓️status.md`,
`📓️explore-action-classifications*.md`, `📓️explore-app-infrastructure.md`, `📓️explore-descriptor-generation.md`,
`📓️explore-lowpoly-recipe.md`, `📓️explore-tests-inventory.md`, `🎫️ticket.json`, `📌️important.md` and `🗑️generated/`.
`📓️status.md` references `gap-six-blocked-actions.md` as "See" but it was apparently never written. Everything
below is independently re-derived from source, per the instruction to verify rather than trust the missing audit.

## 1. Confirmed facts (read directly from source)

1. **The six declaration sites** — `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`,
   inside `create_generation3d_app()`:
   - L600 `.action_interactive_job("nodeGraphEdit", InteractiveJobClassification::BatchOnlyPendingRewrite)`
   - L610 `.action_interactive_job("addGeneration", InteractiveJobClassification::BatchOnlyPendingRewrite)`
   - L611 `.action_interactive_job("removeGeneration", InteractiveJobClassification::BatchOnlyPendingRewrite)`
   - L612 `.action_interactive_job("renameGeneration", InteractiveJobClassification::BatchOnlyPendingRewrite)`
   - L613 `.action_interactive_job("updateGenerationValues", InteractiveJobClassification::BatchOnlyPendingRewrite)`
   - L624 `.action_interactive_job("selectGeneration", InteractiveJobClassification::BatchOnlyPendingRewrite)`

2. **The gate.** `validate_ui_dispatch_classification` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11915`)
   only checks `classification == Migrated`; trivial. But two callers matter, both inside `VcsArtifactApp`:
   `dispatch_action` (`:19025`) and `dispatch_command` (`:19056`) call it, then fall through to
   `self.admit_command_json(...)` → `self.qualified_tool_proof(verb)` (`:18983`) →
   `self.require_complete_tool_operation_pipeline(&admission)` (`:22398`, called from
   `dispatch_typed_command_inner` at `:22412`). **`require_complete_tool_operation_pipeline` accepts only
   `QualifiedToolProof::AppOwned`; `FrameworkOwned` and `Bounded` are both hard-rejected** with fault code
   `interactive-job.missing-owned-reducer` (`:22400-22404`). `qualified_tool_proof` itself already refuses to
   hand out a `Bounded` proof for the normal command path — it returns the SAME fault first if the verb is
   only present in `self.bounded_tool_proofs` (`:18993-18995`). `Bounded` proofs are only ever handed out by
   `qualified_host_configuration_tool_proof` (`:18999-19012`), which is reached exclusively through
   `A::host_configuration_mutation` — gen3d never overrides that (default `Ok(None)`,
   `🧰️framework/…/🔌️plugin/🦀️.rs:11199` / `:26073`) — so it never applies to gen3d's app commands.

   **Consequence:** a command classified `Migrated` whose ONLY proof is a
   `bounded_first_step_tool_proofs!` entry (the framework's generic/"unowned" reducer path) is not actually
   dispatchable. Flipping the classification without also giving the tool an app-owned
   `ArtifactOwnedToolJobFactory` registration produces a command that passes
   `validate_ui_dispatch_classification` and the startup catalog check, then fails every real dispatch with
   `interactive-job.missing-owned-reducer`. This is exactly the "not by flipping the label" failure mode the
   task warned about, and it is NOT what the (missing) audit summary in the task prompt described (contract
   arm + "disposition"/lanes + reducer arms) — see §2.

3. **gen3d has ZERO app-owned tool-job-factory machinery today**, for ANY of its 29 commands, not just the
   six. Repo-wide grep of the whole `procedural` plugin for `ArtifactOwnedToolJobFactory`,
   `register_tool_job_factories`, `ArtifactRetainedCommandJob`, `PUBLICATION_CONTRACTS`,
   `BoundedArtifactCommandWork` returns exactly one file: generation3d's SIBLING app,
   `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`.
   generation3d's own editor file has none of these symbols (checked with `rg` — zero hits) and its
   `impl ArtifactEditor for Generation3dPlayApp` never overrides `register_tool_job_factories`,
   `build_tool_job`, `build_artifact_store_one_item_preparation_factory`, or
   `build_config_store_one_item_preparation_factory` (defaults all apply: empty registry, `None`, `None`,
   `None` — `🧰️framework/…/🔌️plugin/🦀️.rs:25931`, `:25976-25987`).

4. **generation2d is the real precedent, not lowpoly**, and it is instructive that it has NOT migrated its
   own copy of these same six actions either. `Generation2dCommand` declares
   `nodeGraphEdit`/`addGeneration`/`removeGeneration`/`renameGeneration`/`updateGenerationValues`/
   `selectGeneration` (same ids) and ALL SIX are still `BatchOnlyPendingRewrite` in generation2d too
   (`generation2d editor 🦀️.rs:738,744-747,756`). generation2d's ONLY app-owned factory,
   `Generation2dBoundedCommandJobFactory`, covers exactly `GENERATION2D_BOUNDED_TOOL_IDS = ["nodeGraphViewport",
   "setShowMode", "generate", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasWheel"]`
   (`:115`) — 7 tools, none of which touch the Artifact lane; its `PUBLICATION_CONTRACTS` uses only `Config`
   and `HostOnly` lanes (`:195-203`), and consistent with that, generation2d implements
   `build_config_store_one_item_preparation_factory` (`:401`) but has **no** Artifact-lane preparation
   factory at all — because nothing it has migrated yet needs one. This is strong, independent corroboration
   that the Artifact-lane preparation factory is a genuinely-missing prerequisite for this whole family of
   commands (add/remove/rename/updateValues generation + nodeGraphEdit), in both 2D and 3D.

5. **Command-enum entries and handlers already exist and are correct** for all six (the one part of the
   original 3-item audit claim that is fully right):
   - `nodeGraphEdit` → `node_graph_edit::NodeGraphEdit` (editor `🦀️.rs:63`); handler file
     `…/✏️editor/🎮️commands/🕸️node-graph-edit/🦀️.rs` has BOTH `pub fn handle(payload, doc, cfg, session)`
     (satisfies the `app_commands!`-generated `dispatch`) and `pub fn apply(payload, doc, cfg, interaction,
     session)` (used explicitly by `Generation3dPlayApp::handle`'s match arm at editor `🦀️.rs:418`, because
     `nodeGraphEdit`'s `deleteSelection` sub-operation needs live `graph` selection). Its own doc comment
     (`:53-57`) already documents that reaching it through the generic `dispatch()` path — rather than
     `apply()` — degrades `deleteSelection` sub-ops to "selection is empty"; this is accepted, pre-existing,
     documented behavior, not something this migration introduces.
   - `addGeneration`/`removeGeneration`/`renameGeneration`/`updateGenerationValues` all share one
     `handle_generation(action, args, projection, cfg)` helper (`…/🧬️add-generation/🦀️.rs:17-35`, duplicated
     verbatim into the sibling `🧬️remove-generation`, `🧬️rename-generation`, `🧬️update-generation-values`
     files) that emits `Emit { artifact_mutations: […], config_mutations: vec![Generation3dConfigMutation::
     SetGeneration { selected_generation_id, generation_preview_text }], .. }`. Each file's own
     `pub fn handle(payload, doc, cfg, session)` matches the exact 4-arg shape `app_commands!` needs.
   - `selectGeneration` → `…/🧬️select-generation/🦀️.rs:18-24`, `pub fn handle(...)` emits
     `Emit::config(vec![Generation3dConfigMutation::SetGeneration { .. }])` only — no artifact mutation.
   - **`app_commands!`'s generated `dispatch()` already has a match arm for every declared command**
     (`🧰️framework/…/🔌️plugin/🦀️.rs:10574-10601`: `$(Self::$Payload(payload) => $module::handle(payload, doc,
     cfg, ctx),)*`), so the "explicit reducer arms… (currently a fallthrough)" claim in the task's summary of
     the (missing) audit is **wrong**: `Generation3dPlayApp::handle`'s `_ => command.dispatch(doc, cfg, &mut
     session)` fallthrough (editor `🦀️.rs:422`) already correctly routes all five generation commands to
     their `handle()` functions. No new reducer-arm code is needed for those five. `nodeGraphEdit` is already
     explicitly matched too (`:418`). **Nothing to add here** beyond what the new retained-command reducer
     (§4) needs to route to.

## 2. What's actually missing (supersedes the 3-item summary in the task)

The task's relayed audit said the gap was "(a) a `ToolExecutionContract` arm, (b) publication-lane
declaration / disposition classifier, (c) explicit reducer arms." Per §1: (a) is real, (c) is not needed
(already covered by the macro), and (b) as stated is lowpoly-shaped and doesn't fit gen3d's actual
mechanism (gen3d has no "disposition" function of its own — that's a lowpoly-only pattern). The real,
citation-backed gap is:

- **(a) contract arm** — real: `bounded_first_step_tool_proofs!` (editor `🦀️.rs:210-241`) has 23 entries,
  missing exactly `nodeGraphEdit`, `addGeneration`, `removeGeneration`, `renameGeneration`,
  `updateGenerationValues`, `selectGeneration`.
- **(b′) a NEW app-owned `ArtifactOwnedToolJobFactory`** — gen3d has none (§1.3). Without one, `(a)` alone
  gets you a `Bounded`-only proof, which `require_complete_tool_operation_pipeline` rejects (§1.2). This is
  the real "wiring the disposition promises" gap — an app-owned factory, not a disposition enum.
- **(b″) NEW Artifact- and Config-lane `store::ArtifactStoreOneItemPreparationFactory` impls** — gen3d
  overrides neither `build_artifact_store_one_item_preparation_factory` nor
  `build_config_store_one_item_preparation_factory` (both default to `None`, §1.3). An app-owned factory's
  `PUBLICATION_CONTRACTS` is validated against these at construction
  (`ArtifactToolFactoryRegistry::register`, `🧰️framework/…/🔌️plugin/🦀️.rs:12736-12742`: a declared `Artifact`
  lane requires `Self::build_artifact_store_one_item_preparation_factory()` to be `Some`, else the lane is
  marked "unsupported" and every tool that declares it is rejected at dispatch
  (`unsupported_publication_contracts`, `:19119-19137`, checked first thing inside `qualified_tool_proof`,
  `:18984-18986`). `addGeneration`/`removeGeneration`/`renameGeneration`/`updateGenerationValues`/
  `nodeGraphEdit` all need the `Artifact` lane; all five generation commands need the `Config` lane too.
- **(c) reducer arms** — NOT needed as a separate item; the new factory's retained-reduce function (§4)
  supplies the routing, reusing existing `handle()`/`apply()` functions verbatim.
- **No new `Transient` type is needed.** None of the six emit/require mid-gesture ephemeral state; gen3d's
  `type Transient = semio_framework_plugin::NoTransient` is unaffected.

### An important caveat that must be verified empirically before implementing (I could not run cargo)

The same missing-app-owned-factory defect in §1.3 appears to apply to gen3d's other 23 currently-`Migrated`
actions too (`setActiveExample`, `addWidget`, …) — none of them have an app-owned factory either, by the same
grep. If that is true, those 23 are ALSO not actually dispatchable today (dispatch/typed-command tests using
`app_with_registry()`/`dispatch_typed`/`handle_action` would fail with `interactive-job.missing-owned-reducer`),
which would make this a pre-existing, wider defect, separate from and larger than this ticket's six-action
scope. I was told not to run cargo (build-directory lock), so this is NOT empirically confirmed — it is a
structural reading of `qualified_tool_proof`/`require_complete_tool_operation_pipeline`/
`ArtifactToolFactoryRegistry::register`, cross-checked against generation2d's parallel, still-partial state.
**Run the verification command in §6 BEFORE starting**, to establish current ground truth for the 23, so a
regression in this area is never attributed to this patch.

## 3. Lane table for the six (from each handler's actual `Emit`)

| Action | Emits | Lanes | Citation |
|---|---|---|---|
| `nodeGraphEdit` | `Emit { artifact_mutations: operations, .. }` (via `apply_operations`) | **Artifact** only | `🕸️node-graph-edit/🦀️.rs:50` |
| `addGeneration` | `artifact_mutations` (`CreateGeneration`) + `config_mutations` (`SetGeneration`) | **Artifact + Config** | `🧬️add-generation/🦀️.rs:17-35,57-59` |
| `removeGeneration` | `artifact_mutations` (`DeleteGeneration`) + `config_mutations` (`SetGeneration`) | **Artifact + Config** | `🧬️remove-generation/🦀️.rs:17-35,59-60` |
| `renameGeneration` | `artifact_mutations` (`RenameGeneration`) + `config_mutations` (`SetGeneration`) | **Artifact + Config** | `🧬️rename-generation/🦀️.rs:17-35,60+` |
| `updateGenerationValues` | `artifact_mutations` (`ChangeGenerationValue`) + `config_mutations` (`SetGeneration`) | **Artifact + Config** | `🧬️update-generation-values/🦀️.rs:17-35,60+` |
| `selectGeneration` | `Emit::config(vec![SetGeneration{..}])` only, no artifact mutation | **Config** only | `🧬️select-generation/🦀️.rs:24` |

No action among the six needs `Transient` or is `HostOnly` (all persist through the document/config event log,
none use `Effect::LoadDocument`-style host bypass).

## 4. New app-owned types — definitive answer: YES, three are needed

Nothing existing covers these; all three are brand new in generation3d's editor file. Templates adapted
1:1 from `Generation2dBoundedCommandJobFactory`/`Generation2dConfigPreparationFactory`
(`…/generation2d/…/✏️editor/🦀️.rs:113-203,208-260`) and lowpoly's `LowpolyArtifactStorePreparationFactory`
(`…/lowpoly/…/✏️editor/🦀️.rs:1244-1383`, for the Artifact-lane shape generation2d doesn't have an example of).

### 4.1 `Generation3dArtifactStorePreparationFactory` (NEW — Artifact lane)

Generic over `Generation3dMutation` (uses the `protocol::Mutation` trait's `diff`/`inverse`, exactly like
lowpoly, rather than hand-matching every one of gen3d's ~14 mutation kinds — `nodeGraphEdit`'s fixture diff
alone can emit up to 10 different kinds via `generation3d_fixture_operations`, so hand-matching is the wrong
shape here; lowpoly's generic pattern is the right template, not generation2d's hand-matched one).

Insert as a new region right before `impl ArtifactEditor for Generation3dPlayApp {` (i.e. immediately after
the `//#endregion 🔖️Generation3dPlayApp`… actually: insert immediately **before** line 158
`impl ArtifactEditor for Generation3dPlayApp {`, i.e. after the closing `}` of `generation3d_render_body`
at line 156):

```rust
//#region 🧵️RetainedCommands
const GENERATION3D_RETAINED_TOOL_IDS: &[&str] = &["nodeGraphEdit", "addGeneration", "removeGeneration", "renameGeneration", "updateGenerationValues", "selectGeneration"];
const GENERATION3D_RETAINED_PAYLOAD_SCHEMA: &str = "generation.3d.tool-command.v1";
const GENERATION3D_RETAINED_RAW_BYTES: usize = 8_192;
const GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 65_536;
const GENERATION3D_CONFIG_TEXT_MAXIMUM_BYTES: usize = 4_096;
const GENERATION3D_CONFIG_PUBLICATION_MAXIMUM_BYTES: usize = 8_192;

fn generation3d_bounded_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::bounded_first_step(GENERATION3D_RETAINED_RAW_BYTES, 32, 32, 16_384, 7_500)
}

fn generation3d_bounded_extent(_command: &Generation3dCommand, _snapshot: &Generation3dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// 🕹️ `nodeGraphEdit` reads real `graph` selection directly off `protocol::InteractionState` (the raw,
/// crate-public half of what `app::InteractionView` wraps) — plugin code cannot construct an
/// `InteractionView` itself (`state`/`hover`/`peers` are `pub(crate)` to `semio_framework_plugin`), so this
/// is the only way to preserve real-selection `deleteSelection` sub-op behavior for a retained-command-job
/// reducer, instead of falling back to `node_graph_edit::handle`'s documented "treat selection as empty".
fn generation3d_retained_reduce(
    command: &Generation3dCommand,
    snapshot: &Generation3dSnapshot,
    config: &Generation3dConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &semio_framework_plugin::AppOperationContext,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation, NoDraftMutation>, Fault> {
    if !GENERATION3D_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::from("generation3d-command-retained-route-rejected"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config };
    let mut session = FlowEvalSession::new();
    if let Generation3dCommand::NodeGraphEdit(payload) = command {
        let selected = interaction.selection.get("graph").map(|selection| selection.ids.clone()).unwrap_or_default();
        return Ok(node_graph_edit::apply_selected(payload, &doc, &selected));
    }
    command.dispatch(&doc, &cfg, &mut session)
}

struct Generation3dBoundedCommandJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl Generation3dBoundedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_RETAINED_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Generation3dBoundedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<Generation3dPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<Generation3dPlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        generation3d_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > GENERATION3D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((semio_framework::ToolJobFactoryError::new("Generation3d retained command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Generation3dBoundedCommandJobFactory {
    type Owner = EditorApp<Generation3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addGeneration", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeGeneration", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "renameGeneration", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "updateGenerationValues", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "selectGeneration", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    ];
}
//#endregion 🧵️RetainedCommands
```

(`GENERATION_3D_SCHEMA` is already imported at the top of the file, `use crate::artifacts::generation3d::{artifact_kind, Generation3dSnapshot, GENERATION_3D_SCHEMA};` — line 9.)

### 4.2 `Generation3dArtifactStorePreparationFactory` + `Generation3dArtifactStorePreparation` (NEW — Artifact lane, generic)

Insert immediately after the block in §4.1 (still before `impl ArtifactEditor for Generation3dPlayApp {`):

```rust
//#region 📬️ArtifactStorePreparation
fn generation3d_artifact_mutation_retained_bytes(mutation: &Generation3dMutation) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "generation3d-artifact-mutation-encode-failed".to_string())
}

fn admit_generation3d_artifact_mutation(mutation: &Generation3dMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = generation3d_artifact_mutation_retained_bytes(mutation)?;
    if retained_bytes > GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("generation3d-artifact-mutation-envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn prepare_generation3d_artifact(base: &Generation3dSnapshot, mutation: Generation3dMutation) -> Result<(Generation3dSnapshot, Vec<Generation3dMutation>, Generation3dMutation), String> {
    admit_generation3d_artifact_mutation(&mutation)?;
    let inverse = protocol::Mutation::inverse(&mutation, base);
    let diff = protocol::Mutation::diff(&mutation, base).into_parts().0;
    let post = protocol::MutationDiff::apply(&diff, base).map_err(|_| "generation3d-artifact-diff-apply-failed".to_string())?;
    Ok((post, inverse, mutation))
}

fn generation3d_store_edit<M>(prefix: &str, forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
    let id = format!("{prefix}-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

struct Generation3dArtifactStorePreparationFactory;

struct Generation3dArtifactStorePreparation {
    base: Option<store::SnapshotRead<Generation3dSnapshot>>,
    mutation: Option<Generation3dMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Generation3dSnapshot, Generation3dMutation> for Generation3dArtifactStorePreparationFactory {
    fn preflight(&self, mutation: &Generation3dMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("generation3d-artifact-lane-or-description-envelope".into());
        }
        admit_generation3d_artifact_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Generation3dSnapshot, Generation3dMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Generation3dSnapshot, Generation3dMutation>>, store::ArtifactStoreOneItemPreparationRequest<Generation3dSnapshot, Generation3dMutation>> {
        let retained_bytes = generation3d_artifact_mutation_retained_bytes(&request.mutation).unwrap_or(GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Generation3dArtifactStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Generation3dSnapshot, Generation3dMutation> for Generation3dArtifactStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "generation3d-artifact-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "generation3d-artifact-mutation-owner-missing".to_string())?;
        let (post, inverse, forward) = prepare_generation3d_artifact(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "generation3d-artifact-authority-missing".to_string())?;
        let edit = generation3d_store_edit("generation3d-artifact-retained", forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("generation3d-artifact-base-retirement-rejected".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️ArtifactStorePreparation
```

**Verify before applying:** confirm `Generation3dMutation` actually implements `protocol::Mutation<Generation3dSnapshot>` with public-enough `diff`/`inverse` (it must, to satisfy `ArtifactEditor::Mutation: ::protocol::Mutation<Self::Snapshot> + … + OpText + OpBinary`, the same bound lowpoly's `LowpolyMutation` satisfies) and that `protocol::MutationDiff::apply` returns a `Result` (lowpoly's usage — `🦀️.rs:1160` — confirms this shape for `LowpolyMutation::Diff`).

### 4.3 `Generation3dConfigStorePreparationFactory` + `Generation3dConfigStorePreparation` (NEW — Config lane, hand-matched)

Only one `Generation3dConfigMutation` variant is ever emitted by the six: `SetGeneration { selected_generation_id: Option<String>, generation_preview_text: Option<String> }` (confirmed in `🎚️config/🦀️.rs:199-200`, and it's the only variant any of the five generation handlers construct). Hand-match it exactly like `Generation2dConfigPreparationFactory` does for its own two variants (`…/generation2d/…/✏️editor/🦀️.rs:208-260`) rather than going generic — simpler, and it's the only variant that needs to be supported right now.

Insert immediately after §4.2's block:

```rust
//#region 📬️ConfigStorePreparation
fn generation3d_config_text_bytes(config: &Generation3dConfig) -> usize {
    [
        config.lod_mode.len(),
        config.show_mode.len(),
        config.sun_json.len(),
        config.selected_generation_id.as_ref().map_or(0, String::len),
        config.generation_preview_text.as_ref().map_or(0, String::len),
        config.active_utility_id.len(),
        config.locale.len(),
        config.preview_eval_text.as_ref().map_or(0, String::len),
    ]
    .into_iter()
    .fold(0usize, usize::saturating_add)
}

fn generation3d_config_publication_bytes(mutation: &Generation3dConfigMutation) -> Result<usize, String> {
    let bytes = match mutation {
        Generation3dConfigMutation::SetGeneration { selected_generation_id, generation_preview_text } => {
            selected_generation_id.as_ref().map_or(0, String::len).saturating_add(generation_preview_text.as_ref().map_or(0, String::len))
        }
        _ => return Err("generation3d-config-unsupported-mutation".into()),
    };
    if bytes > GENERATION3D_CONFIG_TEXT_MAXIMUM_BYTES {
        return Err("generation3d-config-text-envelope".into());
    }
    Ok(GENERATION3D_CONFIG_PUBLICATION_MAXIMUM_BYTES)
}

struct Generation3dConfigPreparationFactory;

struct Generation3dConfigPreparation {
    base: Option<store::SnapshotRead<Generation3dConfig>>,
    mutation: Option<Generation3dConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Generation3dConfig, Generation3dConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Generation3dConfig, Generation3dConfigMutation> for Generation3dConfigPreparationFactory {
    fn preflight(&self, mutation: &Generation3dConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > 64) {
            return Err("generation3d-config-lane-or-description-envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: generation3d_config_publication_bytes(mutation)? })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Generation3dConfig, Generation3dConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Generation3dConfig, Generation3dConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<Generation3dConfig, Generation3dConfigMutation>> {
        if request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > 64
            || self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err()
            || generation3d_config_text_bytes(request.base.get()) > GENERATION3D_CONFIG_TEXT_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Generation3dConfigPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Generation3dConfig, Generation3dConfigMutation> for Generation3dConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || grant.maximum_bytes < GENERATION3D_CONFIG_PUBLICATION_MAXIMUM_BYTES || self.cancelled || self.closing {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.checkpoint.cursor != 0 {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "generation3d-config-base-owner-missing".to_string())?;
        let mutation = self.mutation.as_ref().ok_or_else(|| "generation3d-config-mutation-owner-missing".to_string())?;
        let mut next = base.get().clone();
        let inverse = match mutation {
            Generation3dConfigMutation::SetGeneration { selected_generation_id, generation_preview_text } => {
                let previous = Generation3dConfigMutation::SetGeneration { selected_generation_id: base.get().selected_generation_id.clone(), generation_preview_text: base.get().generation_preview_text.clone() };
                next.selected_generation_id = selected_generation_id.clone();
                next.generation_preview_text = generation_preview_text.clone();
                previous
            }
            _ => return Err("generation3d-config-unsupported-mutation".into()),
        };
        if generation3d_config_text_bytes(&next) > GENERATION3D_CONFIG_TEXT_MAXIMUM_BYTES {
            return Err("generation3d-config-post-text-envelope".into());
        }
        let authority = self.authority.as_ref().ok_or_else(|| "generation3d-config-authority-missing".to_string())?;
        let id = format!("generation3d-config-{}", authority.next_sequence_number());
        let edit = protocol::Edit {
            id: id.clone(),
            actor: Some(authority.actor().to_string()),
            forwards: vec![mutation.clone()],
            inverse: vec![inverse],
            mutation_meta: vec![protocol::MutationMeta {
                mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
                dependencies: Vec::new(),
                base_version: authority.base_applied_edit_count() as u64,
                author_id: Some(protocol::ActorId(authority.actor().to_string())),
                timestamp: authority.next_clock(),
                undo_policy: protocol::UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: None,
                origin: Default::default(),
            }],
            description: self.description.clone(),
            coalesce_key: None,
            sequence_number: authority.next_sequence_number(),
            started_at: String::new(),
            finished_at: None,
        };
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(next))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: GENERATION3D_CONFIG_PUBLICATION_MAXIMUM_BYTES as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Generation3dConfig, Generation3dConfigMutation>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Generation3dConfig, Generation3dConfigMutation>> {
        self.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 || grant.maximum_bytes < GENERATION3D_CONFIG_PUBLICATION_MAXIMUM_BYTES {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: GENERATION3D_CONFIG_PUBLICATION_MAXIMUM_BYTES });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("generation3d-config-base-retirement-rejected".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️ConfigStorePreparation
```

## 5. `node_graph_edit.rs` — one small addition (needed by §4.1's reducer)

File: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph-edit/🦀️.rs`.

Add a new `pub(crate)` wrapper right after the existing `pub fn apply(...)` (after line 75), reusing the
already-private `apply_operations`:

```rust
/// 🕹️ Retained-command-job entry point (`generation3d_retained_reduce`, editor `🦀️.rs`) — same real-selection
/// behavior as `apply` above, but callable without an `app::InteractionView` (which plugin code cannot
/// construct; its fields are `pub(crate)` to the framework crate). `selected` is read straight off
/// `protocol::InteractionState` by the caller.
pub(crate) fn apply_selected(payload: &NodeGraphEdit, doc: &ArtifactView<'_, Generation3dSnapshot>, selected: &[String]) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
    let sub_operations = parse_sub_operations(&payload.operations_json);
    apply_operations(&doc.snapshot.fixture, &sub_operations, selected)
}
```

If minimizing file-touch surface is preferred over correctness, this step can be skipped and
`generation3d_retained_reduce` (§4.1) can call `command.dispatch(&doc, &cfg, &mut session)` uniformly for all
six, including `nodeGraphEdit` — its own doc comment already documents this exact fallback ("`deleteSelection`
sub-operations degrade to treating the selection as empty") as pre-existing, accepted behavior, not a
regression. Recommend doing the small `apply_selected` addition instead, since it costs one function and
preserves real behavior.

## 6. Registration overrides inside `impl ArtifactEditor for Generation3dPlayApp`

Insert immediately before `semio_framework_plugin::bounded_first_step_tool_proofs! {` (i.e. right after line
209, the blank line following `const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;`):

```rust
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Generation3dArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Generation3dConfigPreparationFactory))
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Generation3dBoundedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !GENERATION3D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("generation3d-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, generation3d_retained_reduce, generation3d_bounded_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new_with_context(
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            request.context,
            operation_context,
            request.completion,
            Generation3dCommand::command_id,
            GENERATION3D_RETAINED_RAW_BYTES,
            1,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

```

Needs two new imports added to the `use semio_framework_plugin::{...}` block at editor `🦀️.rs:26-30`:
`ArtifactOwnedToolJobRequest`, `ArtifactToolFactoryRegistry`, `ArtifactToolPublicationContract`,
`ArtifactToolPublicationLane`, `AppOperationContext` (generation2d imports the same five at its own
`🦀️.rs:26-27`). `semio_framework::{ToolFactoryKey, ToolJobFactoryError}` can stay fully qualified
(`semio_framework::ToolFactoryKey::new(...)` etc., as written above) to minimize the import diff, or be added
as bare imports to match generation2d's style — either is fine.

## 7. The tool-proof block (editor `🦀️.rs:210-241`) — six new entries + `factory_type`

**Mechanical constraint discovered while designing this patch:** `bounded_first_step_tool_proofs!`'s
`tools: { … }` form applies ONE shared, optional `factory_type: $ty` to every tool listed in that single
macro invocation (`🧰️framework/…/🔌️plugin/🦀️.rs:12602-12622`) — it cannot mix "generic, no factory" rows with
"owned by factory X" rows in one call, and a `fn bounded_first_step_tool_proofs()` can only be defined once
per `impl`. Since the six all need `factory_type: Generation3dBoundedCommandJobFactory` while the existing 23
currently have none, the existing block must change shape: add `factory_type: Generation3dBoundedCommandJobFactory,`
to the macro invocation and change `factory: "BoundedFirstStepCommandJobFactory"` to
`factory: "Generation3dBoundedCommandJobFactory"` — but ONLY for entries actually owned by that factory. Given
the macro cannot mix ownership within one call, and per §2's caveat the other 23 may share the exact same
"no owner" defect today, the mechanically simplest, lowest-risk, ticket-scoped change is to leave the existing
23 rows and their `factory: "BoundedFirstStepCommandJobFactory"` byte-for-byte untouched (preserving
whatever their current — possibly already-broken, see §2 caveat — behavior is, with zero regression risk from
this patch), and add the six new rows as a SECOND, separate literal `Vec` extend rather than through the macro,
since the macro's generated function name (`bounded_first_step_tool_proofs`) can only be produced once. Concretely,
replace the macro invocation at editor `🦀️.rs:210-241` with a hand-written function that starts by calling what
the macro used to generate (a local closure keeps the existing 23 lines almost character-identical) and appends
the six owned rows:

Replace lines 210-241 with:

```rust
    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        const OWNER_FILE: &str = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs";
        const CONTROLLER: &str = "s.procedural.generation3d@1/*#editor";
        const DOCUMENT_SCHEMA: &str = "generation.3d";
        let mut proofs = vec![
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "setActiveExample", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "deleteSelection", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "removeWidget", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "moveMediaNode", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "addWidget", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "patchFlowWidgets", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "reorganize", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "translateSelection", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "rotateSelection", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "scaleSelection", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "nodeGraphViewport", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "worldPointerDown", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "graphPointerDown", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "setLodMode", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "setShowMode", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "toggleSun", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "setSunAzimuth", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "setSunElevation", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "setSunIntensity", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "setCamera", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "setActiveUtility", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "setLocale", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "BoundedFirstStepCommandJobFactory", "flowEvalTick", DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)),
        ];
        for tool_id in GENERATION3D_RETAINED_TOOL_IDS {
            proofs.push(
                semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Generation3dPlayApp>>(OWNER_FILE, CONTROLLER, "Generation3dBoundedCommandJobFactory", tool_id, DOCUMENT_SCHEMA, generation3d_bounded_contract())
                    .with_factory_type::<EditorApp<Generation3dPlayApp>, Generation3dBoundedCommandJobFactory>(),
            );
        }
        proofs
    }
```

This preserves the exact 23 existing rows byte-for-byte (same `factory` sentinel, same contract numbers,
same order) and appends the six new owned rows built from `GENERATION3D_RETAINED_TOOL_IDS` (§4.1), so there
is exactly one source of truth for which six tool ids are owned.

**If, per §2's caveat, verification shows the 23 are equally non-dispatchable today** and the team wants the
complete fix rather than a scoped one: fold all 29 into `GENERATION3D_RETAINED_TOOL_IDS`/`PUBLICATION_CONTRACTS`
(§4.1) and go back to the simple one-shot macro form (`bounded_first_step_tool_proofs! { …
factory_type: Generation3dBoundedCommandJobFactory, tools: { 29 entries } }`), matching lowpoly's/
generation2d's own all-one-factory shape exactly. That is a strictly bigger change (every one of the other 22
handlers' `Emit` shapes needs auditing for its own `PUBLICATION_CONTRACTS` lanes, plus `deleteSelection`/
`translateSelection`/`rotateSelection`/`scaleSelection` need the same real-interaction-selection treatment as
§5 gives `nodeGraphEdit`) and is intentionally NOT included as literal code here — it is out of this ticket's
stated six-action scope and needs its own audit pass.

## 8. The six classification flips (editor `🦀️.rs:599-627`)

Change exactly these six lines from `InteractiveJobClassification::BatchOnlyPendingRewrite` to
`InteractiveJobClassification::Migrated` (no other text on the line changes):

```
L600  .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::Migrated)
L610  .action_interactive_job("addGeneration", InteractiveJobClassification::Migrated)
L611  .action_interactive_job("removeGeneration", InteractiveJobClassification::Migrated)
L612  .action_interactive_job("renameGeneration", InteractiveJobClassification::Migrated)
L613  .action_interactive_job("updateGenerationValues", InteractiveJobClassification::Migrated)
L624  .action_interactive_job("selectGeneration", InteractiveJobClassification::Migrated)
```

## 9. Order of edits

1. §5 — add `apply_selected` to `node_graph_edit.rs` (isolated, no dependents yet, compiles standalone).
2. §4.2, §4.3 — add the two preparation-factory blocks (new free-standing types; no dependents yet).
3. §4.1 — add the retained-command/job-factory block (depends on §5's `apply_selected` and needs
   `Generation3dConfig`/`Generation3dMutation` etc. already in scope — they are, via existing imports).
4. §6 — add the four `ArtifactEditor` overrides (depend on all three new factories existing).
5. §7 — replace the `bounded_first_step_tool_proofs!` block with the hand-written function (depends on
   `Generation3dBoundedCommandJobFactory` and `generation3d_bounded_contract()` from step 3).
6. §8 — flip the six classification lines last (this is what actually starts exercising the new path — do
   it after everything it depends on already compiles, so a compile failure never lands with the six already
   claiming Migrated).
7. Add the five new imports to the `use semio_framework_plugin::{...}` block (§6's note) — do this together
   with step 4, since that's the first point they're needed.

## 10. Verification

Per the task's constraint, do NOT run cargo. When cargo is available again (build-directory lock released),
run, in this order:

```bash
# 0. BEFORE any edit — establish ground truth for the §2 caveat (do the 23 already-Migrated actions
#    actually dispatch today, or do they already fail with interactive-job.missing-owned-reducer?):
cargo test --package semio-s-plugin-procedural --lib generation3d:: -- --nocapture

# 1. AFTER the edits in this plan — full native check + the plugin's own test suite:
cargo check --package semio-s-plugin-procedural
cargo test --package semio-s-plugin-procedural --lib generation3d:: -- --nocapture

# 2. wasm target (the app's actual runtime target, per ticket status.md):
cargo check --package semio-s-plugin-procedural --target wasm32-wasip2
```

## 11. Existing tests likely needing updates

- `…/✏️editor/🎮️commands/🧬️add-generation/🦀️.rs` tests `add_generation_records_an_undoable_generation_operation`,
  `select_generation_does_not_mutate_the_document` — use `app()` (registry-less) + `dispatch()`/
  `assert_undo_redo_round_trip`, dispatching `AddGeneration`/`SelectGeneration` directly. Per §2's caveat these
  may currently already fail (or may currently succeed via a mechanism this plan hasn't fully traced through
  registry-less construction) — re-run per §10 step 0 to know their current baseline, then re-run after the
  patch and confirm they still pass (they exercise `handle()` directly-ish via `dispatch_typed`, which after
  this patch resolves through the new `Generation3dBoundedCommandJobFactory` instead of failing).
- `…/✏️editor/🦀️.rs` inline `#[cfg(test)] mod tests` (around line 1440+) — no change expected, but re-run
  since it shares the same `app()`/`app_with_registry()` testkit helpers.
- No test currently asserts on the classification value itself (`test_set_action_classification` is a
  framework testkit helper for OTHER apps' tests, not present in gen3d's own test module) — nothing to update
  there.
- New unit tests worth adding (not required for the framework gate, but for coverage parity with
  generation2d's `retained_route_dispositions_are_exact_and_exhaustive` test,
  `…/generation2d/…/✏️editor/🦀️.rs:1033-1044`): an analogous
  `generation3d_retained_route_dispositions_are_exact_and_exhaustive` asserting
  `GENERATION3D_RETAINED_TOOL_IDS.len() == 6`, `Generation3dPlayApp::bounded_first_step_tool_proofs().len() ==
  29`, `Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.len() == 6`, and that every retained tool
  id has a matching publication contract.

## 12. Generated artifacts that must be regenerated afterwards

- `✏️s/🔌️plugins/🌀️procedural/🔣️.json` is committed and generated. Confirmed by direct inspection: it currently
  embeds `"interactiveJob": "batchOnlyPendingRewrite"` for `nodeGraphEdit` five times (once per window:
  `procedural-main`, `procedural-preview`, `generation3d-generations`, `generation3d-generate-form`,
  `generation3d-generate-preview`) and the same shape for the other four generation actions and
  `selectGeneration`. After step 8, regenerate it:
  ```bash
  cd ✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust && bun ./📜️script.ts describe
  ```
  then confirm the descriptor now reads `"interactiveJob": "migrated"` for all six action ids and diff-review
  the rest of the file is otherwise unchanged (no other action's `interactiveJob` value should move).
- Gate it: `bun nx run @semio-tech/plugin-registry:check` (per the task's own note) — run after regenerating,
  before committing.

## 13. Answers to the deliverable's specific questions

1. **Lanes** — see §3 table; justified from each handler's literal `Emit` construction, not guessed.
2. **Exact code / anchors** — §4 (three new types), §5 (one new fn in `node_graph_edit.rs`), §6 (four
   `ArtifactEditor` overrides), §7 (tool-proof block replacement), §8 (six classification flips).
3. **New app-owned types needed** — **YES, definitively three**: `Generation3dBoundedCommandJobFactory`
   (job factory), `Generation3dArtifactStorePreparationFactory` (Artifact-lane preparation factory),
   `Generation3dConfigStorePreparationFactory` (Config-lane preparation factory). gen3d has NONE of the
   lowpoly/generation2d-style machinery today (§1.3, zero grep hits in the whole `procedural` plugin outside
   generation2d). **No new `Transient` type is needed** (none of the six touch ephemeral/mid-gesture state).
4. **Order + verification** — §9, §10.
5. **Tests / generated artifacts** — §11, §12.

## 14. Critical Files for Implementation

- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph-edit/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs (template/reference only, not edited)
- ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs (template/reference only, not edited)
- ✏️s/🔌️plugins/🌀️procedural/🔣️.json (generated, must be regenerated, not hand-edited)
