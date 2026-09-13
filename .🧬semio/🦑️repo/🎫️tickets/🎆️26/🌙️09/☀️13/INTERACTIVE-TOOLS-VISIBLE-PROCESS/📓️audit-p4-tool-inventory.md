# Audit — Phase 4 Tool Inventory (repo-wide, deep)

Read-only re-audit for ticket `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`, phase 4. Supersedes
`📓️audit-tool-inventory.md` (phase 1), which was shallow for 9 plugins and covered only algorithmic tools.
This pass covers **every plugin** under `✏️s/🔌️plugins/*` (34 directories), the editor surfaces named in the
brief (`🎮️commands`, `🛠️tools`, `🪛️utilities`, `☑️options`, `🧵️*session*`, `⏳️precompute`/`🧠️precompute`,
`💡️inferences`, `⚙️engine`, jobs), and the framework-level tools in `🧰️framework`.

Method: nine parallel read-only sub-audits (puzzle; procedural; the 9 plugins phase-1 covered shallowly;
three batches of never-covered plugins; framework touchpoints), each grepping `Effect::SpawnJob`,
`InteractiveJob`, `StepOutcome`, `BoundedJob`, `ToolDefinition::new`, and the editor directory shapes above,
then reading the actual handlers. The puzzle and 3d-fill findings were additionally hand-verified directly
in this session (not just delegated) because they are the ticket's named target and the one place a prior
wave report's claim ("locking is committing") needed a document-mutation-level trace, not a summary.

## Contract recap (what every row is scored against)

1. **Interactive** — the user sees the algorithm's *process*: intermediate results rendered while it runs,
   not just the final result revealed at the end.
2. **Progress** — steps, percentage, status surfaced to the UI.
3. **Lifecycle** — explicit start, abort (any time while running), finalize (only when complete).
4. **Transaction** — if the tool mutates the artifact, mutations are provisional while running (visible but
   not committed); abort rolls them back; finalize commits them as one unit. A tool that writes straight into
   the live document per-tick/per-step with no rollback path fails this even if it never "faults."

Kind: `algorithmic-mutating` (search/solve/generate that writes the artifact) · `algorithmic-readonly`
(analysis/simulation/preview, no artifact write or only a result record) · `direct-manipulation`
(drag/transform/connect gesture) · `one-shot-cheap` (O(1)/sub-frame reducer — out of lifecycle scope, stated
explicitly, not silently skipped).

Effort class: `wire-only` (a stepping Work/Job/measure already exists, only UI/event wiring is missing) ·
`restructure` (one monolithic synchronous call, no stepping primitive at all) ·
`new-algorithm-visualization` (no per-step/candidate data exists yet — the algorithm itself needs redesigning
to expose intermediate state).

Anything not independently re-verified this pass is marked **UNVERIFIED** rather than asserted.

---

## 0. Framework-level shared tools & touchpoints (`🧰️framework`)

### 0.1 Framework-level user-facing tools

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| VCS/History (undo, redo, commitCheckpoint, createAlternative, switchAlternative, checkoutCheckpoint, revertToCommand) | `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:996-1002`; `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:14743` (`framework_reserved_job!`), `:14916-14921`, `:24232,24291,24385`; fold `…/🏪️store/🦀️.rs:15428` (`checkout_checkpoint_internal`); ledger `…/🌿️vcs/🦀️.rs` | real job, `InteractiveJobClassification::Migrated` | **partial/no** — the job's step loop only walks the tiny wire-envelope bytes; the actual domain fold (`checkout_checkpoint_internal`, own docstring: *"full raw-fold … rather than an incremental update"*) runs as ONE opaque async call after `StepOutcome::Complete` fires | **no** for the real work — checkpoint/progress cadence only covers the envelope phase | **partial** — cancel only covers the envelope phase; no cancel point once the domain fold starts | **unclear/likely no mid-flight** — `checkout_checkpoint_internal` writes the live store directly inside one non-interruptible call | trap for conversions | **`Migrated` classification ≠ the algorithm is interactive** — this is the framework's own flagship "already migrated" tool and it still only wraps admission, not the domain work. Every plugin conversion must check the *domain* step loop, not just the manifest flag. |
| Search — `🌉️mcp/🔎️search/🦀️.rs:22,142` | sync fn over an in-memory `Catalog` | pure sync | n/a | n/a | n/a | n/a | `one-shot-cheap` | Confirmed synchronous, no I/O; out of scope by the ticket's own carve-out. |
| Search — `📺️renderer/…/🔎️ShellSearch/🟦️.tsx:1-225` (command palette / in-doc find) | React component, `rankFuzzyItems` | pure sync | n/a | n/a | n/a | n/a | `one-shot-cheap` | Instant, bounded by an already-resident array; never scales independently. |
| Generic import — `"import-media"` framework-reserved route | `🔌️plugin/🦀️.rs:14930,15167` (`framework_reserved_job!(FrameworkImportMediaJob, …, 8_388_608, …)`) | same carrier as VCS row | same envelope-only limitation | same | same | same | resumable-shaped, domain work UNVERIFIED | Path correction: `🔌️plugin/🖥️host/📥️imports` is NOT this — that file is the wasm-guest host-async ABI bridge, unrelated to document import. |
| Generic export — Media Export submit/poll/cancel | `🔌️plugin/🦀️.rs:14604-14646` (`ArtifactMediaExportJobRequest/Handle/Poll`), `:23917,24039,24141` (submit/poll/cancel), plugin doors `:33825/33833/33841` | real job, plugin-supplied producer | likely yes (UNVERIFIED beyond poll shape) | **yes, real** — `ArtifactMediaExportPoll::Running { applied_progress, checkpoint_available }` distinct from `Complete`/`Cancelled`/`Failed` | **yes** — explicit submit/poll/cancel triad | UNVERIFIED whether the encode itself is chunked per plugin producer | resumable | **Best existing model in the framework** for points 2+3 — point conversions at this pattern, not the VCS one. No generic shared "export" route exists otherwise; export is deliberately plugin-owned via this protocol. |
| OS commands (`set-default-editor/viewer`, `clear-default-app`, `directory-*`, `open-artifact`, `open-artifact-with`) | `🧰️framework/🛍️products/💻️os/🎮️commands/*/🦀️.rs` | field/pointer setters | n/a | n/a | n/a | n/a | `one-shot-cheap` | `🗿️open-artifact/🦀️.rs:1-19` and `🗃️open-artifact-with/🦀️.rs:1-20` confirmed (full read) — just label/id dispatch, no format-conversion logic. Other 8 verified-by-pattern, not individually opened. |
| Generic "layout arrange" | searched `🧰️framework/🔨️modules/🖱️ui` | — | — | — | — | — | **not found** | Every "layout" hit is the CSS-flexbox-style constraint renderer — an implementation detail, not a user-triggered algorithm. No framework-level auto-arrange tool exists. |

### 0.2 Shared plumbing every plugin conversion depends on

- **`Effect::SpawnJob`** — `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:632-637`. The one door a plugin emits to start long-running work instead of doing it inline in a turn.
- **`JobPlacement`** — `…/🎠️kernel/🦀️.rs:712-717` — `Inline`/`Isolated`/`Exclusive`.
- **`InteractiveJob` trait** — `🧰️framework/🔨️modules/🧵️job/🦀️.rs:1163-1174` — `step(&mut self, cx: &mut StepContext) -> StepOutcome`, `begin_close`, `close_step`, `terminal_is_empty`. THE primitive every tool conversion implements. Bounded via `StepContext::should_yield` against an 8ms ceiling (`drive_step`'s watchdog), cancellable via `StepContext::is_cancelled`.
- **`StepOutcome`** — `…/🧵️job/🦀️.rs:1097-1103` — `Yield | PreviewReady(RetainedJobPayload) | CheckpointReady(Checkpoint) | Complete(CommitCandidate) | Cancelled | Fault(JobFault)`. `PreviewReady`/`CheckpointReady` are exactly contract point 1/2's mechanism — already defined, ready to use, and the puzzle-assembly WFC engine (§2 below) shows what happens when a tool computes this richness and nobody reads it.
- **`BoundedJob`** — `…/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs:122-127` — the wasm-guest-side mirror (`jobs.wit`'s `job-step` variant) for a plugin that ships as a wasm component, vs. `InteractiveJob` for a native host-side tool.
- **`WindowMeasure::{Number,Progress}`** — canonical def `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:1064-1150`; projection `…/🧬️schema/📽️projection/🦀️.rs:873`; React renderer `…/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx:157`. Confirmed landed together in commit `d8dce87ca0` (2026-09-13 14:41, `git log --date=iso`), matching the ticket's "wave F" claim. This is the UI vocabulary a converted tool's progress renders through, end-to-end wired — puzzle 2d/3d/5d fill and puzzle 3d brush already consume it (confirmed directly this pass, see §2).
- **`🎯️action-bus`** — `🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs:504,517,526,558` — `ActionBus` registry of `ToolJobFactory`s, **rejects any factory not classified `Migrated`** — the enforcement gate the whole per-plugin migration hinges on. `ToolExecutionContract` (`:217-297`) is the budget/cadence declaration (`max_step_micros < 8000`, `checkpoint_every_steps`, `progress_every_steps`, `ToolCancellationPolicy::PerOperation`, `ToolExecutionShape::{Resumable,BoundedFirstStep}`) every migrated tool factory states — a declaration, not proof the algorithm is actually chunked (see VCS row above).
- **`InteractiveJobClassification`** — `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:736-742` — `Unclassified | Migrated | BatchOnlyPendingRewrite | ForbiddenFromUi | Deleted`. Per prior project memory, `BatchOnlyPendingRewrite` is dispatch-dead in the live path — several plugin batches below (norm-adjacent plugins, note, space, reasoning, layout, cad) still carry it on their content-mutating commands, which is itself a contract-3 violation independent of any algorithm question.
- **`🔀️dispatch` naming collision** — `🧰️framework/🔨️modules/🔀️dispatch/🦀️.rs` is an unrelated proc-macro crate (dyn-enum boilerplate). The real per-action dispatch lives in `ActionBus` above and in `…/🌉️mcp/🔀️dispatch/🦀️.rs` (MCP gateway saga/undo wrapper, not opened in depth — UNVERIFIED).
- **`🔄️machine`** — `🧰️framework/🔨️modules/🔄️machine/🦀️.rs` — a generic hierarchical statechart runtime, **not** job-tick machinery despite the name; a different building block.
- **`🧵️job` tick/admission internals** — `🧰️framework/🔨️modules/🧵️job/🦀️.rs` (2500+ lines): lane wall-clock budgets (`INTERACTIVE_LANE_WALL_US=1000` etc.), `RetainedJobPayload`/`JobPayloadPageGrant` (bounded paged streaming buffers), `drive_step` (`:1186`, the watchdog-timed step call), `WorkerJobSession`/`BatchJobSession` (`:2011-2400`), `ProgressEvent`/`ProgressChannelKind` (`:1688-1862`). Most plugin authors only touch `InteractiveJob`/`StepContext`/`StepOutcome`, not this file directly.
- **Transaction primitive — `HostTransactionCoordinator`** — `…/🔌️plugin/🖥️host/🦀️.rs:6364-6420+`, `run_transaction` (`:6392`). A genuine two-phase prepare/commit/rollback: one `TransactionPrepare` per member, **any rejection rolls back every already-prepared member**, commit in reverse discovery order with `TransactionUndo` compensation on failure. **This already is contract point 4's primitive** — but scoped to cross-artifact sagas, itself one opaque async fn, and it is UNVERIFIED whether any single-artifact plugin tool (the common case) reuses it or needs a lighter same-artifact variant. This is the single most important existing asset a "restructure" wave should look at first before inventing a new provisional-mutation mechanism.
- **Pending/uncommitted event-sourcing support** — `…/🌿️vcs/🦀️.rs:1177-1199` (`PendingChangeRef`), durable ledger `ArtifactHistoryLedger<Change>`/`<Checkpoint>` (`:287-610`, capacity 64, `:186`). The store's mutable `applied_edit_ids`/`redo_edit_ids` (`🏪️store/🦀️.rs:15408-15419`) are discardable in-memory state until `commitCheckpoint` content-addresses them — the pieces for "apply during steps, checkpoint = finalize, never-checkpoint = implicit abort" already exist in the data model, but are **not packaged as a reusable plugin-facing begin/abort/finalize API**. A framework wave should wrap this once rather than every plugin hand-rolling it.

**Overall implication:** the load-bearing primitives all exist and are wired end-to-end somewhere. But wrapping a tool in `SpawnJob`/`InteractiveJob` and flipping its classification to `Migrated` is *necessary but not sufficient* — the framework's own VCS/undo tool proves that the domain algorithm can still run as one un-chunked call after admission. Every plugin-level "wire-only" verdict below should be read as "the scaffolding exists," not "the algorithm is already chunked."

---

## 1. Puzzle (`🧩️puzzle`) — the ticket's named primary target

Shared infra: `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs` — `PuzzleCommandWork<A>`/`RetainedPuzzleCommandJob<A>` (L57-79, L310+, L640) used by every 2d/3d/5d command. It is a real bounded/resumable/checkpointed `InteractiveJob` (`PuzzleCommandPhase::{WirePages,WireBytes,Decode,Preflight,Work,WorkProgress,Publish,Complete,Fault}`, L164-174) — **but atomicity is per single dispatch, not across a sequence of dispatches**. Puzzle-3d fill is driven by many separate `fillBuildTick` dispatches over time, each individually atomic — that gap is exactly what lets fill violate contract point 4 across a whole run while looking clean at the single-dispatch grain.

### 1.1 Puzzle 3d — fill (directly hand-verified, not just delegated — this is the report's central finding)

| Tool | Files | Kind | (1) Interactive | (2) Progress | (3) Lifecycle | (4) Transaction | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| **Fill build** (`fill-build-tick`, `set-fill-count`, `cancel-fill-build`, `engagement-abort`) | `E=✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`: `⏳️precompute/🪣️fill/🦀️.rs` (4711 lines, `FillBuilder`), `⏳️precompute/🦀️.rs` (`Puzzle3dPrecomputeSession`), `🎮️commands/🪣️fill-build-tick/🦀️.rs:20-58`, `🎮️commands/🧮️set-fill-count/🦀️.rs:45-77`, `🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`, `🧬️schema/🦀️.rs:668-785` (`FillCandidateVerdict`, `FILL_TRIED_RING=12`) | `algorithmic-mutating` | **YES** — every tried candidate streams: `FillBuilder::push_tried`/`record_verdict` (`⏳️precompute/🪣️fill/🦀️.rs:4393,4406`) resolve `Testing→Collision/Rejected/Free/Accepted`; wire-encoded (`:460`); TS-side `WorldFillTriedRecord`/`FillTriedGhosts` mounted at `🧰️framework/…/🌐️World3dHost/🟦️.tsx:355,3776,7136` — confirmed end-to-end, danger/highlighted mesh styles present (`🟦️.tsx:493,509,513,556-566`). **But capped at `FILL_TRIED_RING = 12`** (`🧬️schema/🦀️.rs:684`) — **directly contradicts phase-4 requirement 4** ("shows all tested meshes, not a ring of 12"). | **YES** — `progress_measure()` (`🛠️tools/🪣️fill/🦀️.rs:73-89, 100-110`) is a real `WindowMeasure::Progress`: localized `stage`, `completed`/`total`, `steps` (locked/collision/rejected/tested), `cancel` action carrying `(job,operation,generation)` identity. | **PARTIAL** — start is implicit (selecting the tool/typing a count auto-spawns via `enqueue_fill_job()`, `fill-build-tick/🦀️.rs:28`); abort exists via `cancelFillBuild` and `engagementAbort`/Escape (both → `cancel_fill_job_for`, `⏳️precompute/🦀️.rs:3717-3739`); **no "finalize" distinct from "let it keep running until done."** | **NO — confirmed, load-bearing violation.** `fillBuildTick` runs every ~120ms while the tool is active; each call does `take_locked_into_fixture` (`set-fill-count/🦀️.rs:61-77`), which directly mutates `ctx.scene.fixture.objects`/`.attractions` in place (`extend`/`retain`, L71-75) at up to `FILL_LOCK_PLACEMENTS_PER_TICK=8` per tick. `puzzle3d_action_document_intent` includes `"fillBuildTick"` so **every tick is its own committed `create_object`/`delete_object`/`connect_vortices` mutation**, coalesced under one undo-history row (`"fill-count"`) but each independently and immediately applied. `cancel_fill_job()` (`⏳️precompute/🦀️.rs:3724-3739`) only flips a cancellation token on the *search* — it never touches `ctx.scene.fixture` and never reverts committed objects. A user who locks 40 pieces then cancels keeps all 40 permanently. | `restructure` | Confirms `📓️status.md`'s flagged contradiction exactly, with the actual mutation-level trace. The stepping/checkpoint/verdict-stream machinery already exists — what's missing is a document-side staging layer (locked-but-uncommitted placements rendered as real instances without being in `ctx.scene.fixture` until explicit finalize) plus a real cancel-time rollback. Ring cap also needs lifting/replacing with a bounded-but-complete or paginated stream (`RetainedJobPayload`/`JobPayloadPageGrant` from §0.2 is the natural vehicle). |
| **`import-fixture`** | `E/🎮️commands/📥️import-fixture/🦀️.rs:393-423` | `algorithmic-mutating` | no | no | partial — chunked/retransmission-safe, but no explicit user cancel, only passive sweep on session-slot retirement (`editor.rs:2991-3000`) | **YES — the one clean transaction example in the whole plugin.** Every partial chunk returns early without touching `ctx.scene.fixture` (L393-396); only the closing chunk does `ctx.scene.fixture = fixture;` as one atomic replace (L423). | `wire-only` | This is the template to copy for fill's staging layer. |
| **`export-fixture`** | `E/🎮️commands/📤️export-fixture/🦀️.rs:55-66` | `algorithmic-readonly` | no | no | one-shot, no lifecycle | n/a | `wire-only` | Multi-chunk output built synchronously in one retained-job turn despite chunk-looking shape (`editor.rs:3872-3883`) — invisible to the user. |
| **Brush suggestions search** | `⏳️precompute/🦀️.rs` (`BrushSearchProgress`, `advance_brush_search`); `🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️.rs:67-121` | `algorithmic-readonly` | **YES** — live `tested`/`free`/`blocked`/`total_candidates` Progress measure, per-candidate ghost verdict painted | **YES** — same measure | partial — begins on hover, no explicit cancel action (`cancel: None`, `brush/🦀️.rs:79`, deliberate — closing the popup is the de-facto cancel) | n/a — search never touches the document; commit is a separate atomic `*ctx.scene = next;` swap | none needed | Bounded: 8 slices/tick, 2000µs wall budget (`BRUSH_SEARCH_*` constants) — genuinely bounded, not hidden. |
| **`world-relocate`** | `E/🎮️commands/🌍️world-relocate/🦀️.rs:16-68` | `direct-manipulation`, flagged for scale | no | no | single synchronous call, no cancel | n/a (direct `object.origin` write + new attractions, one-shot) | `wire-only`/perf-fix | Confirmed genuinely unbounded: nested `for other in &fixture.objects { for vortex in &other.vortices { … } }` (L37-63), O(objects × vortices), plus a whole-document `resolve_puzzle3d_attractions` call after. Runs synchronously on every object-drop gesture. Cost-scaling flag, not a lifecycle violation at today's document sizes. |
| **`volume-brush`, `transform`** | `🪛️utilities/🧊️volume-brush/🦀️.rs` (54L), `🪛️utilities/🔄️transform/🦀️.rs` (54L), full reads | `one-shot-cheap` | n/a | n/a | n/a | n/a | none | Pure O(1) steppers/toggles, confirmed no algorithm. |
| **`delete-selection`, `set-selection-flag`, `scale-selection`, `duplicate-selection`, `create-attraction`, `add-object-kind`, `patch-inspector`** | `E/🎮️commands/*` (full reads) | `direct-manipulation` | no | no | none | no — all write `ctx.scene.fixture.*` directly in place, several then call whole-document `resolve_puzzle3d_attractions`/rescans regardless of selection size (`editor.rs:1166-1175,1251`) | `wire-only`/perf-fix | Cost-scaling flag: several of these do a full document scan on one small gesture. |
| **`accept-suggestion`, `add-brush-object`** | `E/🎮️commands/*`, `editor.rs:1957-1972` | `direct-manipulation` | no | no | no distinct abort/finalize for the rederive step | placement itself: yes (one atomic `*ctx.scene = next;` swap); immediately followed by a hidden O(attractions×(attractions+objects)) synchronous `puzzle3d_rederive_all_attractions` pass with zero visibility | `wire-only` | |
| **~24 trivial setters** (spacing, sun, selectable-kind, manual/automatic, vortex-show, panel-page, depth-variable, voxel-dims, proximity-radius, camera, projection, gumball-flag, chunk-size, vortex-direction, snap-enabled, hover-suggestion, open-import-fixture, target-volume ops, delete-attraction, kind-weight, brush-overlap-budget) | `E/🎮️commands/*` (each individually read) | `one-shot-cheap` | n/a | n/a | n/a | n/a | none | `open-import-fixture/🦀️.rs:10` has a leftover `eprintln!("[DEBUG] …")` in production code — repo-hygiene flag. |

### 1.2 Puzzle 2d — fill (core hand-verified; rest UNVERIFIED this pass)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| **Fill tool** | `◻️2d/…/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` (142L, full read); `🎮️commands/{🏁️fill-session-begin,👣️fill-session-step,🧹️fill-session-clear}/🦀️.rs` | `algorithmic-mutating` | partial — aggregate accepted/tested counters only (phase-1 finding, not independently re-confirmed this pass for per-object visuals) | **YES** — `progress_measure` (L94-110) real `WindowMeasure::Progress`, localized stage, `completed`/`total`, counters, `cancel: brushFillSessionCancel` with generation guard (note: different action name than 3d/5d's `cancelFillBuild` — worth unifying) | richer-looking state machine than 3d: `Puzzle2dFillLifecycle::{Capturing,Queued,Running,CheckpointReady,Applying,AwaitingAdoption,Closing,Idle,Discarded,Faulted,Cancelled}` (L28-47, 96, 116) | **NO — confirmed directly this pass**, resolving the open question the sub-audit flagged. `🎮️commands/👣️fill-session-step/🦀️.rs:5-8` docstring: *"The placements a previous session accepted are already committed to the document"* — i.e. despite the richer-looking lifecycle labels, 2d commits per-step exactly like 3d does, with no provisional/staged layer distinct from the live document. `AwaitingAdoption`/`Applying` are lifecycle **labels**, not evidence of staged mutation. | `restructure` | `PUZZLE2D_DEFAULT_FILL_COUNT=100` (L16), `max: None` (L62) confirmed — unbounded default-100 landed as wave-D claims. Retry toggle on `Faulted`/`Cancelled` confirmed (L116-118). |
| Rest of 2d (other `🎮️commands`/`🛠️tools`/`🪟️windows`) | — | — | — | — | — | — | — | **UNVERIFIED this pass** — phase-1 classified the remaining surface as small D/E reducers; not independently re-read here. |

### 1.3 Puzzle 5d — fill (core hand-verified; rest UNVERIFIED this pass)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| **Fill option** | `🖐️5d/…/✏️editor/🎭️modes/✏️edit/☑️options/🪣️fill/🦀️.rs` (108L, full read) | `algorithmic-mutating` (delegated) | not independently re-confirmed | **YES** — `fill_count_measure`/`fill_progress_measure` (L30-45, 69-85) read straight off `Puzzle5dPrecomputeSession::fill_progress()`, a real `WindowMeasure::{Number,Progress}` pair, `cancel: cancelFillBuild`, `max: None` — confirmed landed, closes the phase-1-flagged gap where 5d's slider previously showed nothing. | not independently re-confirmed | **UNVERIFIED** — 5d's own `set-fill-count`/`🧠️precompute`/`cancel-fill-build` command bodies were not read this pass. Own doc comment (L9-11): "5d's fill IS that [3d] planner" — if 5d truly reuses 3d's `Puzzle3dPrecomputeSession::take_fill_locked_chunk` path, it very likely inherits the same NO verdict as 3d; if it instead commits synchronously-to-completion in one `setFillCount` dispatch (as a wave-D report claims, not independently trusted here), that's a different mechanism but plausibly still a violation (still no rollback-on-cancel). **Flagged as the report's top "verify next" item alongside 2d's session handlers.** | `restructure` (pending verification) | |
| Rest of 5d (25+ commands: retarget, patch-grip, proximity-connect, etc.) | — | — | — | — | — | — | — | **UNVERIFIED this pass** — phase-1 classified as small D/E reducers. |

---

## 2. Procedural (`🌀️procedural`)

Headline correction to phase-1: `🧵️preview-eval` is **NOT** shared identically by generation2d and generation3d — generation2d has no such module at all and implements a materially weaker, unaddressed, uncancellable loop of its own.

### 2.1 generation3d

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| **Flow preview eval/tessellate chain** (`flowEvalTick`→`flowEvalResolve`→`flowTessellateResolve`/`CancelResolve`, `cancelPreviewEval`) | `🧵️preview-eval/🦀️.rs` (structs L33-106; fns `evaluate_tick` 525-596, `resolve_eval` 604-645, `cancel_preview_eval_for` 683-698, `preview_progress_status_json_for` 823-945); editor `🎮️commands/{⏱️flow-eval-tick,✅️flow-eval-resolve,🔺️flow-tessellate-resolve,🧯️flow-tessellate-cancel-resolve,🛑️cancel-preview-eval}`; render `🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:79-101` | `algorithmic-readonly` | **YES** — republished into the addressed window's retained transient every tick, rendered live | **YES, real** — `phase`/bilingual `phaseLabel`/`progress{unitsDone,unitsTotal,facesDone,facesTotal,…}`/`cancellable`; consumed by `🌐️World3dHost/🟦️.tsx:3860-3884` (`phaseText`/`percent`/`data-compute-*`) | **YES** — explicit arm/rearm (L127-129,586-590), explicit cancel with dual evaluate/tessellate-cancel retirement, terminal detection resolving to idle | n/a — readonly preview, only window-scoped transient | none — already exemplary | **Reference implementation** — shared verbatim by edit-preview, generate-preview, and the standalone viewer's preview. |
| **`import-document`** | `🎮️commands/📥️import-document/🦀️.rs:132-301` | `algorithmic-mutating` | partial — chunks stream but nothing renders until the last chunk | partial — `Generation3dImportStaging::open_runs` progress state exists (L249-250) but **no confirmed UI reader** (grep for consumers empty) | partial — implicit start/finalize (atomic replace on last chunk), **no abort/cancel method found** | **YES** — provisional staging until the final chunk, one atomic mutation batch | wire-only (if `open_runs` UI is confirmed missing) | UNVERIFIED: whether `open_runs` is rendered anywhere. |
| `🗺️reorganize` (auto-layout) | `🗺️reorganize/🦀️.rs:11-24` | `algorithmic-mutating` | no | no | no | no — unconditional commit | one-shot-cheap in practice / `restructure` if it scales | Fails all 4 by construction; only "cheap" because flow graphs are small today. |
| `📤️export-document` | `📤️export-document/🦀️.rs` (74L) | `algorithmic-readonly` | not deep-dived | — | — | — | `wire-only` if slow (UNVERIFIED size cost) | Single-shot synchronous serialize. |
| `🧬️generation`/`➕️add`/`🗑️remove`/`🏷️rename`/`🎚️update-values`/`🎯️select` | `🧬️generation/🦀️.rs:18-70` | direct-manipulation/one-shot-cheap | n/a | n/a | n/a | n/a | n/a | Form-field CRUD on saved parameter sets, not generative search. |
| ~20 gesture/config commands (translate/rotate/scale-selection, delete-selection, add/remove-widget, node-graph-edit, navigate-graph, camera/sun/show-mode, set-contributions, etc.) | one file each, 18-156 lines | direct-manipulation/one-shot-cheap | out of scope | — | — | — | — | **Caveat**: `✏️editor/🦀️.rs:2299-2337` tags EVERY action incl. these trivial ones `InteractiveJobClassification::Migrated` — app-level migration bookkeeping, not evidence of real per-tool lifecycle wiring. Don't mistake the tag for compliance. |
| `👁️preview` (generate mode), `🗂️generations`, `📝️form` windows | `🎭️modes/🧬️generate/🪟️windows/*` | — | — | — | — | — | — | Confirmed to read the same `flowEvalTick`/preview-eval status host — no re-derivation found. |

### 2.2 generation2d

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| **Flow node-graph evaluation** (`flowEvalTick`→`flowEvalResolve`) | `🎮️commands/⏱️flow-eval-tick/🦀️.rs:11-38`, `✅️flow-eval-resolve/🦀️.rs:10-24`; host loop `🧬️schema/🦀️.rs:217`; render `🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:41-50` | `algorithmic-readonly` | partial/yes — each tick resolves one more node, preview re-renders `session.eval_json()` | **NO** — repo-wide grep for `status_json`/`preview_progress`/`phase\b` in this artifact's editor: zero hits, confirmed absent | **partial** — implicit auto-arm, **no cancel command exists at all** in this artifact (confirmed by zero Rust `cancel` hits) | n/a — readonly | `restructure` | **Biggest concrete gap in the plugin**: a materially inferior, unaddressed, uncancellable sibling of gen3d's fully-built chain, solving the same problem class. |
| `🗺️reorganize`, generation CRUD family | `🎮️commands/*` 16-76L | direct-manipulation/one-shot-cheap/algorithmic-mutating (reorganize) | out of scope (reorganize: same as gen3d's) | — | — | — | — | |
| ~9 canvas/graph-edit commands | `🎮️commands/*` 16-73L | direct-manipulation | out of scope | — | — | — | — | Pointer/wheel handlers, 16 lines each. |

### 2.3 assembly (WFC solve) — highest-value single finding in this plugin

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| **`WfcJob`** (inner solver) | `🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/🦀️.rs`: `WfcStage` (L54-82, 10 named stages), `WfcPreview` (L85-101, rich per-step state), `preview()` L544 | `algorithmic-mutating` (capability) | **engine: YES the primitive exists; UI: NO** — repo-wide grep confirms `WfcPreview`/`WfcStage` referenced ONLY in this file and its own unit test | **NO for the same reason** — rich data computed and discarded | partial at engine level (proper resumable/checkpointable job) but **no** dispatch/step/cancel from any user gesture | n/a — never reachable | **wire-only** (visualization data already exists) — but gated on a correctness fix first (below) | Intermediate-state design is excellent, exactly what the contract wants — just never connected. |
| `AssemblyInferenceJob` (outer job driving `WfcJob` as `child`) | `🧬️schema/💡️inferences/🦀️.rs:100-131,207-222,396-518` | `algorithmic-mutating` | **NO** — `emit_preview` (L207-222) builds only a fixed 25-byte `sequence`/`completed`/`total`/`stage-as-u8` payload; `child.preview()` is **never called** (confirmed via grep of `child.` usages) | no exposed UI | proper `InteractiveJob` impl (Yield/PreviewReady/Complete/Cancelled/Fault) but no dispatch entry point | n/a | | |
| Solve entry point | `register_assembly_inference_factory` (`🦀️.rs:633`), registered at plugin boot (`🌀️procedural/🦀️.rs:83`) | — | **no command/window/UI action calls it anywhere in the repo** (confirmed by grep of `AssemblySolve`/`s.assembly.solve`) | — | — | — | — | **The live solver is documented as currently broken**: `📚️examples/🧪️tests/🧩️outcome/🦀️.rs:1-11` — `AssemblyInferenceJob::step` fails a retained-page handback assertion and **aborts the process**. This is a pre-existing defect that blocks any UI-wiring work, not something introduced by this audit. |
| `🌳️structure` window (problem-spec editor) | `✏️editor/🎭️modes/✏️edit/🪟️windows/🌳️structure/🦀️.rs:1-89` | direct-manipulation | n/a — by design never renders the solved assignment (doc comment L1-10) | n/a | `Migrated`-tagged synchronous tree mutations (same blanket-tag caveat as gen3d) | n/a | one-shot-cheap | Confirms: the ONLY assembly UI edits the problem definition; no UI path to *run* a solve exists at all. |

**Bottom line for assembly**: fully-designed interactive-preview protocol, zero UI consumers, plus a documented process-abort defect on the live-solve path — highest-value/lowest-effort target once the abort is fixed (the hard part, exposing intermediate state, is already built).

---

## 3. The 9 plugins phase-1 covered shallowly — deepened this pass

### 3.1 Energy (`🔋️energy`, artifact `🔋️model` — only artifact, confirmed)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| 4-tier energy simulation | `🧵️simulation-session/🦀️.rs` (2382L): `EnergyTierProjection` L132-147, `install_preview` L1071-1098, `EnergySimulationStatus` L118-129 (Idle/Admitting/Queued/Running/Cancelled/Faulted/FinalReady/Adopted/Closing); UI `✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs` | `algorithmic-mutating` | **YES** — 4 tiers stream live, highest-published tier shown | **YES** — full status enum drives UI | **YES** — explicit Start/Cancel/Retry/Discard/Adopt (L100-116); cancel L1938; adopt only sets a flag L1946-1957 | **YES, strong** — commit gated on `adopt_requested` (L1262-1285, "commit ACK rejected"/"stale commit retained"); nothing writes to the document until Adopt; Cancel/Discard never sets the flag | none | **Gold standard, reconfirmed with mutation-level evidence.** `Effect::SpawnJob{ENERGY_SIMULATION_JOB_KIND, Isolated}` at L2060. |

### 3.2 FEM (`🏗️fem`, 2d AND 3d — phase-1 only read 2d)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| FEM 2d mesh/graph admission | `◻️2d/…/✏️editor/🧵️session/🦀️.rs` | algorithmic-readonly | partial | **YES** — named progress tags (`fem2d.graph-admitted`, `.domain`, `.mesh-preview`, etc.) | YES (real `InteractiveJob`) | n/a | wire-only | |
| **FEM 2d Results window** (static/modal/buckling) | `◻️2d/…/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs:122-281` | algorithmic-readonly | **NO** | **NO** | **NO** | n/a | `restructure` | **Worse than phase-1 found**: `render_modal` (242-257)/`render_buckling` (263-281) call `fem2d_modal_mode_values`/`fem2d_buckling_mode_values` synchronously too, not just the static solve — all 3 display modes, on every render, in production (no `#[cfg(test)]` gate). |
| **FEM 3d mesh/graph + numeric solve** | `🧊️3d/…/✏️editor/🧵️session/🦀️.rs` (3888L, `crate::live_visual`) | algorithmic-readonly | **YES** | **YES, real** — `Fem3dVisualJobStage` (19 named stages, L120,2449-2691) wraps `Fem3dNumericalStage` (~60 sub-stages incl. a real PCG iterative solver publishing `residual_norm`/`tolerance`/`completed`/`total`, L247,1713-1717,3695-3706) | **YES** — auto-started on doc-revision change (`reconcile()` L3582-3660, `Effect::SpawnJob{FEM3D_MOUNTED_VISUAL_JOB_KIND}`, superseded jobs cancelled via `Effect::CancelJob`); explicit `cancel()` L3450 | n/a (immutable `Fem3dPageVisualLease` snapshot, not a document write) | none — already exemplary | **Phase-1 completely missed this** — arguably the best-instrumented FEM surface in the repo. Live `[DEBUG] eprintln!` at L3654-3659 not yet removed. |
| **FEM 3d Results window** | `🧊️3d/…/📊️results/🦀️.rs:79-172` | algorithmic-readonly | n/a | n/a | n/a | n/a | none | **Already fixed relative to 2d**: synchronous `render_static/modal/buckling` are `#[cfg(test)]`-gated dead code; production path `render_with_progress` (L90-96) reads the precomputed `Fem3dPageVisualLease`, zero compute during render (own doc comment L89). |

**Priority correction**: fem2d's Results window is the real, still-live offender across all 3 display modes; fem3d's equivalent has already been fixed and is the template.

### 3.3 Remodel (`📸️remodel`)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| Reconstruction pipeline (ingest→…→commit) | `🎮️commands/🏗️run-reconstruction/🦀️.rs` (1031L), `⏩️advance-reconstruction`, `🛑️cancel-reconstruction`, `🔁️retry-stage`; panel `📌️panels/🗿️artifact/🦀️.rs` | algorithmic-mutating | **YES** — sparse-cloud/camera preview streamed every tick | **YES** — `progress_0_1` + 10 `RequestedStage` + 7 `TerminalPhase` | **YES** — self-re-arming advance, explicit cancel, retry | **YES, confirmed and stronger than phase-1 realized** — see below | wire-only (one soft gap) | |

Transaction detail (traced end-to-end): `advance_terminal` pushes `create_asset(...)` every tick, but `CreateAsset::diff` (`🧷create-asset/🔺️diff/🦀️.rs:17-27`) detects the internal staging-key prefix and returns an **empty diff**, instead writing bytes into a process-local `Mutex<BTreeMap<...>>` (referenced `📸️remodeling/🦀️.rs:431-433`) never part of the persisted document. Only `TerminalPhase::Commit` (`run-reconstruction/🦀️.rs:871-887`) assembles one real `commit_reconstruction` mutation. Cancel (`413-432`) and cancel-requested advance (`897-899`) call `discard_staged_remodeling_{asset,mesh}` to purge the side table. **Net: the document is genuinely untouched while running; abort discards ephemeral staging; finalize commits atomically.** Real transaction semantics — the best-verified positive example in the repo alongside energy. Soft gap re-confirmed: a very-fast run never visibly shows "running" (panel comment L26).

### 3.4 Lowpoly (`💠️lowpoly`)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| 12 mesh-edit ops (extrude/inset/bevel/loopCut/subdivide/triangulate/mirror/flipFaces/merge/dissolve/snap/toggleSmooth) | `🎮️commands/🔷️mesh-edit/🦀️.rs:1-327` | direct-manipulation | no | no | no | no — writes through `mesh_edit`→`sync_meshes_to_snapshot()` synchronously | n/a | Shared synchronous helper for all 13. |
| **`decimate`** | same command file; kernel `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs:1273-1320` | **algorithmic-mutating** | no | no | no | no | `new-algorithm-visualization` | **Confirmed genuinely quadratic-class hazard, not hypothetical**: outer `while live_verts > target_verts` (up to O(V) iterations); each iteration does a full linear scan of every halfedge (`for he_id in 0..halfedges.len()`, L1285) to find the shortest edge, then `polygon_soup()`+`rebuild_from_polygon_soup()` (L1301,1316) — **rebuilds the entire half-edge topology from scratch per vertex collapsed.** Entirely synchronous, zero progress, zero cancel. |

### 3.5 Raster (`🖨️raster`)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| Layer/brush CRUD (18 commands) | `🎮️commands/*` | direct-manipulation | n/a | n/a | n/a | n/a | n/a | **Path correction to phase-1**: commands exist (phase-1 checked the wrong nested path). `🩹️patch-layer/🦀️.rs:1-89` verified pure field-write. |
| **Composite render pipeline** (the `⚙️engine`-equivalent phase-1 didn't read) | Viewer `👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️composite/🦀️.rs:36-56`; editor `✏️editor/🦀️.rs:885-886`; host `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:2041-2052` | algorithmic-readonly | no | no | no | n/a | `restructure` if documents grow | `render()` calls `raster_document_json_to_svg`→`rasterize_svg_to_png_base64`→`canonicalize_png_bytes` **synchronously on every paint**, both viewer composite and editor export. `resvg::render` cost scales with layer/vector count and resolution, no caching visible, no progress/job. Same "solve on render" anti-pattern as fem2d (see cross-plugin pattern below). |
| Masks panel | `📌️panels/🎭️masks/🦀️.rs:1-52` | one-shot-cheap | n/a | n/a | n/a | n/a | n/a | Pure list render. |

### 3.6 Animate (`🎞️animate`, artifact `🎬️presentation`)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| Tile-grid commands (18) | `🎮️commands/*` | direct-manipulation | n/a | n/a | n/a | n/a | n/a | Simple synchronous grid/selection edits, confirms phase-1. |
| **`export-video-from-deck`** (`⚙️engine/🎥️video`, unread by phase-1) | `🎮️commands/🎥️export-video-from-deck/🦀️.rs:14-49`; `⚙️engine/🦀️.rs:58-73`; `⚙️engine/🎥️video/🦀️.rs:330-…` | algorithmic-readonly | **NO** | **NO** | **NO** | n/a | `new-algorithm-visualization` | **Confirmed with code, not speculation**: `handle_async` (28-45) is one opaque async fn frame-rendering AND encoding the entire video inline (`VelloRenderer`+`SceneFileWriter` loop, MP4/GIF/PNG-sequence). Zero `SpawnJob`/`InteractiveJob`/progress/cancel anywhere in `⚙️engine`. UI blocks or shows nothing until the whole render+encode finishes. |

### 3.7 VCS plugin (`🌿️vcs` — NOT the framework module)

| Tool | Files | Kind | (1)-(4) | Effort | Notes |
|---|---|---|---|---|---|
| rename/change-counter/notes/status, add/remove-tag | `🔺️diff/🦀️.rs:1-49` (`VcsDiff`), `⚙️operations/🦀️.rs:1-61` | one-shot-cheap | n/a | out of scope | **Definitively resolved**: the mutation type is literally named `VcsDemoMutation` (`⚙️operations/🦀️.rs:3,16-25`), fixture dir `🌿️mutate-vcs-1` — this is a schema-conformance **demo fixture**, not a real version-control system. O(1) struct-field diff, nothing expensive. Phase-1's "likely D/E" now confirmed. |

### 3.8 Sourcing (`🪵️sourcing`, artifact `🗂️curation`)

| Tool | Files | Kind | (1)-(4) | Effort | Notes |
|---|---|---|---|---|---|
| stockFromCatalogue, curationAdd/SetCount/Remove, filters, sortTable | `✏️editor/🦀️.rs:1137-1149` | direct-manipulation | n/a | n/a | **Resolved, not just flagged**: `AGENTS.md` confirms sourcing curates a modular parts catalogue, not a crawl/fetch pipeline. `import_media` (`🦀️.rs:935-945`) only decodes a local base64 pack — no network I/O anywhere (`grep "fn crawl\|fn fetch\|fn scrape\|fn ingest"` → zero hits repo-wide in this plugin). Phase-1's "ingestion pipeline elsewhere" is resolved: there is no such pipeline. |

### 3.9 Process (`🏭️process`, artifact `🧊️process3d`)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| step-list editing | `🎮️commands/🪜️step/🦀️.rs:1-183` | direct-manipulation | n/a | n/a | n/a | n/a | n/a | Zero `SpawnJob` hits confirmed; `InteractiveJob` hits are all `InteractiveJobClassification`/`InteractiveJobCloseStep` framework boilerplate. |
| **Brep replay** (cut/drill/attach → B-Rep boolean chain) | `🧬️schema/💡️inferences/🦀️.rs:194-245` (`replay_process`); called from `🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️.rs:91,154` and `🚪️io/🦀️.rs:224-241` | algorithmic-readonly | **NO** | **NO** | partial — `resolved_up_to` cursor + `ChangeCursor` mutation give a document-level "how far replayed" concept, no per-op progress | n/a (derives a display mesh; document stores only the step list + cursor) | `new-algorithm-visualization` | **New finding phase-1 missed entirely.** Real `kernel.cut`/`kernel.fuse` B-Rep booleans, one per enabled step, with a memoized-by-signature cache — **but `processed_mesh` (L239-241) constructs a fresh `ProcessKernelReplay::new()` every call**, so the memo never survives across renders, and `render()` (`🪚️workpiece/🦀️.rs:91`) re-runs the ENTIRE boolean chain from stock, synchronously, on every paint. Same "solve on render" anti-pattern as fem2d/raster, for CSG — typically the most expensive of the three. |

**Cross-plugin pattern (recurs 4×, concrete evidence each time)**: "solve/render/rasterize synchronously inside a window's `render()`" — **fem2d** (`fem2d_solve_all`+modal+buckling), **raster** (`rasterize_svg_to_png_base64`), **process3d** (`replay_process`/CSG booleans) — all called fresh on every paint, versus **fem3d**'s already-fixed pattern (job produces an immutable lease, `render()` just reads it). fem3d's `live_visual` session is the best template for converting the other three.

Repo-hygiene note (not the ticket's core ask, flagged in passing per CLAUDE.md's `[DEBUG]` convention): live `eprintln!("[DEBUG] …")` remain in FEM 3D `reconcile()` (`🧵️session/🦀️.rs:3654-3659`), FEM 3D results `render_with_progress` (`📊️results/🦀️.rs:94`), and puzzle 3d's `open-import-fixture/🦀️.rs:10`.

---

## 4. Never-audited plugins — batch 1 (writer, mathematical, flow, gis, shooting, demonstrator, sequence, architect)

### 4.1 Writer (`✒️writer`)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| `format-document` | `🎮️commands/🧹️format-document/🦀️.rs:13-22`; formatter `🧬️schema/🦀️.rs:256` | algorithmic-mutating | no | no | no | no — one `EditText` replaces the whole document text directly | `restructure` | Real AST-based formatter (`semio_s_artifact_trinity_jack::core::format`) for `language_id=="jack"`; cost scales with document length. |
| `lint-document` | `🎮️commands/🔍️lint-document/🦀️.rs:12-14` | n/a — stub | — | — | — | — | — | Unconditionally returns a Fault; not implemented yet. |
| `request-completions` | `🎮️commands/✨️request-completions/🦀️.rs`; `🧬️schema/🦀️.rs:244-253` | algorithmic-readonly | no | no | no | n/a | `one-shot-cheap` | Bounded parser-driven autocomplete. |
| Field/config setters, binary import/export mutations | `🎮️commands/*`, `🚪️io/🧬️mutations/💾️binary/🦀️.rs` | direct-manipulation | n/a | n/a | n/a | n/a | n/a | Binary io codec (1670L) flagged only by a grep hit, not confirmed stepped — follow-up if in scope. |

### 4.2 Mathematical (`➗️mathematical`, artifact `➗️equation`)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| `roots` inference | `🧬️schema/💡️inferences/🌱roots/🦀️.rs:145-189` | algorithmic-readonly | no | no | n/a | n/a | `new-algorithm-visualization` if made interactive | Sturm-sequence root isolation + bisection to 1e-9; cost scales with polynomial degree, currently cheap for integer-polynomial scope. |
| `topology` inference | `🧬️schema/💡️inferences/🦀️.rs:36-37` | algorithmic-readonly | no | no | n/a | n/a | n/a | Cheap structural walk. |
| CAS library (`🌿️cas-internals/🦀️.rs`, 5045L: simplify/factor/solve/limit/taylor/diff/gradient) | same file | **dormant** | — | — | — | — | — | Confirmed via grep: nothing outside its own tests calls `crate::cas::…` — a full headless CAS exists but is wired to no editor command, window, or inference. Nothing to make interactive yet. |
| 6 field/graph setters | `🎮️commands/*` | direct-manipulation | n/a | n/a | n/a | n/a | n/a | Trivial; the editor command surface is essentially trivial. |

### 4.3 Flow (`🌊️flow` plugin — 29 commands, largest surface in this batch)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| `evaluate`/`flow-eval-tick`/`flow-eval-resolve` | `🎮️commands/🧮️evaluate/🦀️.rs:26-33`, `⏱️flow-eval-tick/🦀️.rs:49-64`, `🏁️flow-eval-resolve/🦀️.rs:51-54` | algorithmic-readonly | **YES** — incremental per-node preview | **YES** — `FlowHost::set_computing_progress` (framework module, out of plugin scope) surfaces active/stale nodes | **PARTIAL** — real start + budgeted self-re-arming ticks, **no plugin-level Cancel/Stop command in the 29-command list** | n/a — ephemeral, nothing to roll back | wire-only (missing cancel command) | Closest-to-conformant tool found in this batch — one gap: no user-facing cancel. |
| `reorganize` | `🎮️commands/🗺️reorganize/🦀️.rs:19-21` | algorithmic-mutating | no | no | no | no — writes mutations directly | `restructure` | One synchronous auto-layout pass. |
| ~27 widget CRUD / view-state commands | `🎮️commands/*` | direct-manipulation | n/a | n/a | n/a | n/a | n/a | Bounded by selection/edit-batch size, not document size. |

### 4.4 Gis (`🌍️gis`, artifacts `🏔️gisterrain`, `🗺️gismap`)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| `bounds` inference (gismap) | `🧬️schema/💡️inferences/📦bounds/🦀️.rs:22-60+` | algorithmic-readonly | no | no | n/a | n/a | n/a | Self-acknowledged O(n) over all features, "feature counts small" per its own docstring — cost-scale flag. |
| **`propose-bounds-region`** (gismap) | `🎮️commands/💡️inference/🦀️.rs:14-27` | algorithmic-readonly (proposal) | already conformant — positive example | already conformant | already conformant | already conformant | none | **The one place across 8 plugins that fully defers to a host-owned lifecycle**: pure `Effect::RequestInferenceProposal` dispatch; scope/idempotency-key/lease/progress-cursor/Cancel/Approve are all framework-owned (framework internals not audited here, out of scope, but the *pattern* is worth reusing elsewhere). |
| Route/feature CRUD | `🎮️commands/🗺️features/🦀️.rs:14-52` | direct-manipulation | n/a | n/a | n/a | n/a | n/a | O(n) filter/diff over routes, one-shot-cheap at typical scale. |

### 4.5 Shooting (`🎥️shooting`)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| `topology` inference | `🧬️schema/💡️inferences/🧭topology/🦀️.rs:27-45` | algorithmic-readonly | no | no | n/a | n/a | n/a | Trivial O(n) shot→camera walk. |
| `export-shots` (`all: bool`) | `🎮️commands/🖨️export/🦀️.rs:27-44` | algorithmic-mutating-adjacent (side-effect only) | no | **no** — a batch of N shots dispatched as one `Effect::IconRenderExport`, no per-item progress at this layer | no | n/a | `wire-only` (if render backend streams) / `restructure` otherwise | Actual render happens in an out-of-scope effect handler. |
| selection/scene/camera/document/gumball, `import-asset` | `🎮️commands/*` | direct-manipulation | n/a | n/a | n/a | n/a | n/a | Docstrings confirm CONFIG-only, no document mutation. |

### 4.6 Demonstrator (`🎪️demonstrator`, artifact `🎪️playground`)

Nothing to audit. Entire command surface is one command, `change-schema` (`🎮️commands/🔧️change-schema/🦀️.rs:20-22`), swapping one opaque string field. No `⚙️engine`, no `💡️inferences`, no other algorithmic surface anywhere — confirmed by directory listing and grep. Stated plainly rather than padded.

### 4.7 Sequence (`🎬️sequence`)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| **`run`/`stop`** | `🎮️commands/🏃️run/🦀️.rs:15-42`; executor `✏️editor/🦀️.rs:826-828` | algorithmic-readonly | no — whole compiled program executed synchronously to completion | no | **NO — `Stop` exists in the command catalog but its handler is a literal no-op** (`pub fn handle(...) { Ok(Emit::default()) }`, L40) | n/a | `restructure` | **Strongest single defect in batch 1**: the app already offers a Stop affordance that does nothing — cannot actually interrupt a running sequence. |
| `reorganize` | `🎮️commands/🔄️layout/🦀️.rs:30-36` | algorithmic-mutating | no | no | no | no — writes mutations directly | `restructure` | Same one-shot-DAG-layout pattern as flow's. |
| `topology` inference | `🧬️schema/💡️inferences/🦀️.rs:14-16` | algorithmic-readonly | no | no | n/a | n/a | n/a | Real Kahn's-algorithm topo sort, O(V+E), cheap today. |
| step CRUD, node-graph-edit, connection/viewport | `🎮️commands/*` | direct-manipulation | n/a | n/a | n/a | n/a | n/a | Bounded by selection/edit-batch. |

### 4.8 Architect (`🏛️architect`, artifact `🏛️program`) — richest analytical surface in this batch

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| `run-analysis` (19 kinds: Gap/Conflict/Dependency/Capacity/…/RelationshipAnalysis) | `🎮️commands/🔬️analysis/🦀️.rs:24-51`; `🧬️schema/💡️inferences/🦀️.rs:1580-1631+` (2189L file) | algorithmic-mutating | no — computed whole, then one atomic `CreateAnalysisRecord` | no | no | no — single atomic append, no provisional state while computing | `restructure` (most) / `wire-only` (cheap subset) | `analyze_conflict`→`detect_adjacency_conflicts` (`🦀️.rs:171-196+`) confirmed **O(n²) nested loop over all adjacency-register pairs**. |
| `run-report`, `run-validation`, `search` (whole-register keyword scan) | `🎮️commands/🔬️analysis/🦀️.rs:4-91`; `🎮️commands/🔍️search/🦀️.rs:22-31` | algorithmic-mutating/readonly | no | no | no/n-a | n-a (config-only for search/validation) | `restructure`/`one-shot-cheap` now | Search cost scales with total entity count. |
| `import-program`/`export-program`/CSV exchange | `🎮️commands/📤️exchange/🦀️.rs:1-111` | algorithmic-mutating (import)/readonly (export) | no — whole-document reset on import, no incremental parse feedback | no | no | no — one atomic reset | `restructure` | Same "parse + whole-document replace" shape as writer's `format-document`. |
| adjacency/register/element/template CRUD | `🎮️commands/*` | direct-manipulation | n/a | n/a | n/a | n/a | n/a | |
| `topology` inference | `🧬️schema/💡️inferences/🧭topology/🦀️.rs:48-67` | algorithmic-readonly | no | no | n/a | n/a | n/a | 3-color DFS + BFS, O(n), cheap. |

**Batch-1 cross-cutting**: best conformant pattern = flow's `evaluate` chain + gis's `propose-bounds-region`. Sharpest concrete bug = sequence's dead `Stop`. Richest un-converted surface = architect's analysis family. Nothing in this batch uses the framework's exact `Effect::SpawnJob`/`InteractiveJob`/`StepOutcome`/`BoundedJob` types — flow's evaluation loop self-chains via `Effect::DispatchAction` + its own `EvalStepBudget` instead.

---

## 5. Never-audited plugins — batch 2 (reasoning, forms, layout, cad, norm, playbook, imperative, trinity, dag)

Method note: `InteractiveJobClassification::Migrated` = live-dispatchable; `BatchOnlyPendingRewrite` = per project memory, dispatch-dead in the live path — treated here as its own contract-3 violation, separate from whether the underlying algorithm is chunked.

### 5.1 Reasoning (`💡️reasoning`, artifact `🔌️wires`)

| Tool | Files | Kind | (1)-(4) | Effort | Notes |
|---|---|---|---|---|---|
| `forceLayout`, `reorganize` | `🎮️commands/⚛️force-layout/🦀️.rs:12-41`; `🧬️schema/🦀️.rs:236` | algorithmic-mutating | no/no/`BatchOnlyPendingRewrite` (`✏️editor/🦀️.rs:659-660`)/no | `restructure` | Physics sim runs to convergence inside one call, only final positions emitted; mutations committed straight. |
| CRUD (addNode/addRelationship/deleteSelection/setActiveExample, pointer/viewport) | `✏️editor/🦀️.rs:654-658,661-663` | direct-manipulation/one-shot-cheap | n/a | wire-only (reclassify) | |

### 5.2 Forms (`📋️forms`)

No algorithmic engine found. `submit` (`🎮️commands/✅️submit/🦀️.rs:15-17`) is a **literal no-op stub** — no scoring/validation logic implemented yet. `setTryValue`/`setTryValueStep` (`🎯️set-try-value/🦀️.rs:332-475`) has real chunked-transport machinery (start/sessions/cancel) but it's for input reassembly, not compute. Rest is CRUD.

### 5.3 Layout (`📏️layout`) — best `InteractiveJob` example found in this batch

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| `exportPng/Svg/Pdf/Package` | stub handlers reject with `"layout-export-job-only"` (`🎮️commands/🌄️export-png/🦀️.rs:15-17`, siblings identical); real job `⚙️engine/📤️export/🦀️.rs` (~4300L) | algorithmic-mutating/readonly | **partial** — byte-budgeted chunked stepping keeps it responsive, but no partial-rendered-page preview reaches the UI before `Complete` (verified `🦀️.rs:2196,2375-2446,2527-2555,3866-3955`) | partial — resumable checkpoint/cursor state exists; whether a % reaches the UI **UNVERIFIED** | **YES** — real `InteractiveJobCloseStep::{Pending,Complete,Blocked}` (`:2466-2503,2567-2570,3943-3955`) | **YES** — `CommitCandidate`/`Complete` pattern (`:2212,2430-2446,4297`) | none — already compliant infra | Only real gap is point 1 — no incremental output preview. |
| addFrame/Page, patchPage/Frame, canvas pointer | `✏️editor/🦀️.rs:782-788` | direct-manipulation | n/a | n/a | `BatchOnlyPendingRewrite` | n/a | wire-only | Not deep-dived beyond classification. |

### 5.4 Cad (`📐️cad`)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| `importCadFile` (STEP/OBJ/STL/GLB) | `🎮️commands/📥️io/🦀️.rs:25-50` | algorithmic-mutating | no | no | `BatchOnlyPendingRewrite` (`✏️editor/🦀️.rs:2094`) | no — direct document reset | `restructure` | Whole-payload parse + reset in one call; cost scales with file/scene size. |
| `saveSelected/InPlay/Current` (STEP/OBJ/STL export) | `🎮️commands/📥️io/🦀️.rs:62-114` | algorithmic-readonly | no | no | `BatchOnlyPendingRewrite` (`:2115-2117`) | n/a | `restructure` | Synchronous tessellation, cost plausibly scales with scene. |
| `translateSelection/rotateSelection/scaleSelection/applyTransformation` | `🎮️commands/🔄️transform/🦀️.rs:24-60` | direct-manipulation | — | — | — | — | — | **Documented dead code** — "drag-objects retired… no child-dispatch seam exists yet." Not functional currently. |
| pointer/camera/lighting | `✏️editor/🦀️.rs:2088-2114` | direct-manipulation | n/a | n/a | mixed Migrated/BatchOnlyPendingRewrite | n/a | wire-only | |

### 5.5 Norm (`📕️norm`, 16 near-identical artifacts: en1990/91/92/93/94/95/96/98/99, din4108/16798/18599, vdi3805, iso16757) — flagged high-value by phase-1, confirmed template-identical

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| `evaluate` (compliance check, all 16 families) | registry e.g. `en1990/…/✏️editor/🦀️.rs:215,218`; algorithm `en1990/…/🧬️schema/💡️inferences/🦀️.rs:85-96`; shared factory `📕️norm/🖥️app-surface/🦀️.rs` | algorithmic-readonly | no — whole `CheckReport` computed in one call | **no, by architectural design** — `norm_bounded_extent` (`🖥️app-surface/🦀️.rs:490-493`) returns `Some(1)` unconditionally, comment: *"Every norm command is one bounded step; none of the three walks a collection incrementally."* | `Migrated` at the plumbing level (`NormBoundedCommandJobFactory`, `:495-552`), but extent architecturally fixed at 1 — abort mid-evaluation isn't meaningful | n/a — `evaluate` is read-only; `setSnapshot` (below) has real transaction machinery | `restructure` if per-clause progress is ever wanted | **No norm family can get incremental progress without changing the shared `norm_bounded_extent` contract** — one framework-level fix would apply to all 16. 8 of 16 read directly (en1990/91/92/93/94, din4108, vdi3805, iso16757); remaining 8 share identical 236/237-line file size — **not individually re-verified, flagged as a pattern assumption**. |
| `setSnapshot` | same registry | one-shot-cheap (bounded ≤4KB/1 item) | n/a | n/a | `Migrated`, real prepare/commit/cancel (`:680-743`) | **YES** — genuine prepare→commit with cancel/close_step | n/a | Actually well-built transactionally; just not algorithmic. |

### 5.6 Playbook (`📖️playbook`)

Nothing algorithmic. Registry (`✏️editor/🦀️.rs:530-537`): addStep/removeStep/moveStep/addBlock/removeBlock/moveBlock/updatePlaybook (all `BatchOnlyPendingRewrite`) + setContributions (`Migrated`). The only `⚙️engine` file (53L) is pure I/O plumbing. Viewer commands directory is empty. Pure document CRUD — stated plainly.

### 5.7 Imperative (`📜️imperative`, artifact `📜️procedure`) — sharpest single violation in batch 2

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| **`run`** (executes the whole procedure) | `🎮️commands/🏃️run/🦀️.rs:16-21`; `ImperativeHost::run()` `⚙️engine/🦀️.rs:210-212` → external `imperative_engine::Executor` | algorithmic-mutating | **NO** — every step executes synchronously to completion, only final `result.scope` surfaced | **NO** | **NO** — `BatchOnlyPendingRewrite` (`✏️editor/🦀️.rs:245`), **no way to cancel a long-running or stuck procedure** | no — no prepare/rollback around run's mutations | `new-algorithm-visualization` | A plugin whose entire purpose is executing a step sequence gives the UI zero visibility, no per-step results, no cancel, no partial-completion. |
| step CRUD, setContributions | `✏️editor/🦀️.rs:236-244` | one-shot-cheap | n/a | n/a | `BatchOnlyPendingRewrite` | n/a | wire-only | Procedure-definition editing, not execution. |

### 5.8 Trinity (`🔱️trinity`, artifacts `♻️rewriting`, `🔌️jack`)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| Rewrite-rule application (rewriting) | `apply_rule`/`apply_rule_json` `🧬️schema/🦀️.rs:279-291`; invoked from `✏️editor/🦀️.rs:194-212` | algorithmic-readonly | no — computed whole per edit | no | n/a — pure fn called inline during render | n/a | `new-algorithm-visualization` | Pattern-match/rewrite cost scales with "before" graph size, recomputed on every parameter/rule edit. |
| `reorganize` (rewriting) | `✏️editor/🦀️.rs:821` | algorithmic-mutating | not opened directly — inferred by cross-reference to wires/dag/jack's identical pattern | — | — | — | `restructure` | **UNVERIFIED** — flagged as inference, not direct read. |
| **`runQuery`** (jack) — best example in this batch | `🎮️commands/▶️run-query/🧵️job/🦀️.rs:175-262,296-346` | algorithmic-readonly | partial — staged localized stage text ("Parsing"→"Preparing"→"Evaluating", en/de) but result rows not streamed, only final `ReplaceQueryResult` | **YES** | **YES** — real `close_step`/`CloseStep::{Pending,Complete,Blocked}`, checkpoint/replay | yes-ish — result committed only on `CompleteWithEphemeral`; query itself never mutates the document | none — already compliant except point-1 detail | Best Progress+Lifecycle example found in this batch. |
| `reorganize` (jack) | `🎮️commands/🧭️reorganize/🦀️.rs:8-55` | algorithmic-mutating | no — 120 physics iterations run synchronously inside one call | no | `BatchOnlyPendingRewrite` (`✏️editor/🦀️.rs:886`) | no | `restructure` | Third occurrence of the repeated force-layout pattern. |
| formatDocument, text/graph edit, patch/CRUD | `✏️editor/🦀️.rs:880-889` | one-shot-cheap/direct-manipulation | n/a | n/a | mixed | n/a | n/a | Not individually deep-dived beyond classification. |

### 5.9 Dag (`🕸️dag`)

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| `reorganize` | `🎮️commands/🗂️reorganize/🦀️.rs:18-36` | algorithmic-mutating | no | no | `BatchOnlyPendingRewrite` (`✏️editor/🦀️.rs:696`) | no | `restructure` | Third confirmed occurrence of the force-layout-to-completion pattern (after reasoning, trinity/jack). |
| topological-sort inference | `🧬️schema/💡️inferences/🧭topology/🦀️.rs:2,9` | algorithmic-readonly | n/a — derived view, not a user tool | — | — | — | n/a | Own comment: "cheap" today; cost-scale flag if graphs grow. |
| node/edge CRUD, pointer | `✏️editor/🦀️.rs:688-699` | direct-manipulation | n/a | n/a | mostly `BatchOnlyPendingRewrite` | n/a | wire-only | |

**Batch-2 cross-cutting finding**: a single root-cause algorithm — **synchronous force-directed layout run to completion inside one handler, positions revealed only at the end** — recurs independently in **at least 3 confirmed plugins** (reasoning/wires `forceLayout`+`reorganize`, dag `reorganize`, trinity/jack `reorganize`) plus flow's and sequence's `reorganize` from batch 1 (5 total occurrences, same fix pattern) — the single largest "restructure" cluster in the whole audit, and a strong candidate for one shared framework-level fix (a real stepped force-layout primitive with intermediate-frame preview) rather than 5 separate plugin fixes. `📜️imperative`'s `run` is the sharpest individual defect in this batch. `📕️norm`'s `evaluate` is architecturally capped at one step across all 16 families by a shared framework touchpoint (`norm_bounded_extent`), not an oversight — fixing it once fixes all 16.

---

## 6. Never-audited plugins — batch 3 (draw, note, block, space)

Cross-cutting framework finding for this batch: generic `.action_interactive_job("id", classification)` wrapping (`DrawingGestureOperationJob`, `🖍️draw/🗿️artifacts/🖍️drawing/…/✏️editor/🦀️.rs:570-727`, read in full) decodes the wire command, calls the handler once, returns `StepOutcome::Complete` immediately — "Yield" here means wire-byte chunking, not algorithm progress. No command's *domain logic* in this batch was found decomposed into visible incremental steps.

### 6.1 Draw (`🖍️draw`, artifact `🖍️drawing`) — all 24 actions confirmed `Migrated`

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| `combineBoolean` | `🎮️commands/🔀️combine-boolean/🦀️.rs` | algorithmic-mutating | no — builds a declarative "boolean layer" node only, actual CSG runs downstream (unverified where, likely renderer) | no | generic wrapper only | n/a — single mutation append | wire-only for this command; downstream CSG UNVERIFIED | |
| Canvas gesture pipeline (pointer/click/commit/escape/engagement) | `🎮️commands/*`; `DrawingGestureOperationJob` `✏️editor/🦀️.rs:557-727` | direct-manipulation | live client-side stroke feedback, no document-side intermediate state | no % surfaced | generic close_step only, not user-facing cancel-mid-gesture | each gesture commits individually, no whole-drag batching | wire-only | |
| `updateLayerTraceParams` (bitmap trace/vectorize) | mutation only, `🔀️transform/🧬️schema/🧬️mutations/🔍️update-layer-trace-params/` | algorithmic-mutating (params only) | n/a | n/a | n/a | n/a | wire-only if a real tracer is added | Actual trace algorithm not found in this plugin — likely renderer-side, UNVERIFIED. |
| Layer CRUD (16 commands) | `🎮️commands/*` | one-shot-cheap | n/a | n/a | n/a | n/a | n/a | |

### 6.2 Note (`🗒️note`) — worst dispatch-classification hygiene in this batch

Most content-mutating commands are `BatchOnlyPendingRewrite` (dispatch-dead): `addBlock`, `moveBlock`, `deleteBlock`, `deleteSelection`, `duplicateBlock/Selection`, `patchBlocks`, **`inkApplyEvents`**, 8× `nudgeSelection*`, grid/snap/pencil/eraser setters, `saveDownload`. Only `setGridVisible/Spacing`, `engagementSubmit`, `setCamera(Zoom)`, `engagementInput`(×2), `loadRequest` are `Migrated`.

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| **`inkApplyEvents`** (pencil/ink stroke commit) | `🎮️commands/🖊️ink-apply-events/🦀️.rs:1-100` | algorithmic-mutating, cost scales with block+batch count | client-side yes, document-side no | no | **`BatchOnlyPendingRewrite` — not reachable through the live interactive-job path at all** | no — whole batch diffed and emitted as one mutation set, no partial-commit/abort | `restructure` | `note_ops_from_canvas_events()` clones the entire document, then loops the whole event batch once, all inside one synchronous call. |

### 6.3 Block (`🧱️block`, artifacts `◻️2d`, `🖐️5d`, `🧊️3d`) — cleanest classification hygiene, but no algorithmic surface

All actions in all 3 artifacts confirmed `Migrated` (full grep across all three editors, plus each has a unit test asserting the classification). No `⚙️engine`/`🏗️model`/`⚙️operations` directory exists anywhere in this plugin. `worldSurfacePlace`/`placeVortex` (3d brush "stamp") confirmed single-point O(1) insert, no loop over scene. Every other command sampled or grouped by name is O(1) node/kind CRUD. **No interactive/progress/lifecycle/transaction concerns exist in this plugin's editor layer** — stated plainly; real geometry/voxel compute if any lives outside this plugin's tree (unverified where).

### 6.4 Space (`🪐️space`, artifacts `🏠️home`, `🪐️space`, plus engine `⚙️engine/🪐️space`) — most genuine cost-scaling findings in this batch

| Tool | Files | Kind | (1) | (2) | (3) | (4) | Effort | Notes |
|---|---|---|---|---|---|---|---|---|
| `exportStudioPack` | `⚙️engine/🪐️space/🎮️commands/📦️export-studio-pack/🦀️.rs:1-29` | algorithmic-mutating (file output), cost scales with whole studio | no | no | `BatchOnlyPendingRewrite`; `handle()` runs `export_os_space_pack(&document)` fully synchronously | no — all-or-nothing single `Emit` | `restructure` | Confirmed no chunking/streaming. |
| `exportMedia` | `⚙️engine/🪐️space/🎮️commands/📤️export-media/🦀️.rs` | algorithmic-mutating, scales with node/document size | no | no | `BatchOnlyPendingRewrite` | no | `restructure` | Single synchronous call chain, whole result returned via one `Effect::DownloadMediaExport`. |
| `importMedia`, `importMediaPayload`, `exportStudioDsl` | `⚙️engine/🪐️space/🎮️commands/*` | plausibly same shape | **UNVERIFIED** — grouped by strong naming/classification analogy, not opened | — | `BatchOnlyPendingRewrite` | — | `restructure` (pending verification) | |
| `reorganizeWorkflow` | `⚙️engine/🪐️space/🎮️commands/🗂️reorganize-workflow/🦀️.rs` | algorithmic-mutating, O(n) nodes | no | no | `BatchOnlyPendingRewrite` | no — one `Emit::mutations` batch | `wire-only` (algorithm itself is trivial) | Confirmed **not** a real force-directed/topological layout — a naive grid placement (`col = index % 4`). Flagged per the cost-scaling rule even though visualization would be over-engineering here. |
| `compiledDagEngagementSubmit`/`workflowEngagementSubmit`/`nodeGraphEdit` | `⚙️engine/🪐️space/🎮️commands/*` | unclear | — | — | — | — | — | `compiledDagEngagementSubmit`'s handler is a stub no-op. The real DAG-compile algorithm was **not found inside this plugin's tree** — likely lives in `semio_framework_os::workflow`, outside `🔌️plugins` scope; flagged for whoever owns that framework crate rather than guessed at. |
| `applyDirectoryEventPage`/`foldDirectoryEvents` (home) | `🏠️home/…/✏️editor/🎮️commands/*` | plausibly scales with event-log size | **UNVERIFIED** — not opened | — | `BatchOnlyPendingRewrite` (home); note `foldDirectoryEvents` is `Migrated` in the *space*-artifact editor but `BatchOnlyPendingRewrite` in *home* — same action name, different migration state per surface | — | `restructure` (pending) | Named after event-log folding, plausible scale concern by domain, not confirmed. |
| ~25 navigation/membership commands (openSpace, createArtifact, inviteMember, etc.) | various | one-shot-cheap | n/a | n/a | `Migrated` (reachable) | n/a | n/a | The plugin's healthily-dispatched surface. |

Note: `🎥️shooting` was deliberately NOT re-audited by this batch (already covered in §4.5, batch 1) — confirmed no overlap/gap between agents.

---

## 7. Not covered this pass — flagged, not guessed

- **`🗄️stdio`** — ~40 file-format artifacts (csv/json/xml/pdf/docx/xlsx/step/ifc/gltf/png/svg/…, each with its own `✏️editor`), plus a shared `🧿️semio` internal-format artifact with 19 subset editors. This is the encode/decode/oracle-testing surface, not a document-editing "tool" in the ticket's sense for most formats — but was not opened this pass. Flag for a dedicated follow-up if any format's import/export is algorithmically expensive (e.g. IFC/STEP parsing, glTF/mesh import) rather than a straight codec.
- **`🗟️artifacts`** — a minimal plugin: exactly one artifact (`◻️2d`) with only a `🧵️session` under its editor, no `🎮️commands` found at the depth checked. Likely a shared test/template artifact base, not user-facing. Not opened further — flag rather than guess.

---

## 8. Prioritized conversion list — parallelizable waves, disjoint file ownership

Waves are ordered by (ticket-declared priority × user-facing frequency × existing-plumbing cheapness). Within a wave, each row's file set is disjoint from every other row's in that wave, so separate agents can run them concurrently without touching the same files. A later wave may depend on an earlier wave's shared-framework work (noted under "needs").

### Wave 0 — shared framework prerequisites (blocks several later waves; do first, single owner)

1. **Same-artifact provisional-mutation primitive.** Package the event-sourcing "apply into `applied_edit_ids` during steps, checkpoint = finalize, never-checkpoint = implicit abort" pattern (`🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:1177-1199,287-610`; `…/🏪️store/🦀️.rs:15408-15419`) as a reusable plugin-facing begin/abort/finalize API, OR document a single-artifact usage pattern for `HostTransactionCoordinator` (`…/🔌️plugin/🖥️host/🦀️.rs:6364-6420+`). **Every "restructure" row marked contract-4-no below depends on this landing first** — without it each plugin would hand-roll its own staging layer.
2. **Shared force-layout primitive.** One stepped, cancellable, intermediate-frame-emitting auto-layout `InteractiveJob`, replacing the 5 confirmed independent occurrences of "run physics/layout to completion synchronously": `reasoning/wires` `forceLayout`+`reorganize`, `dag` `reorganize`, `trinity/jack` `reorganize`, `flow` `reorganize`, `sequence`'s layout command (§4.3, §4.7, §5.1, §5.8, §5.9). Fixing this once in the framework and re-pointing 5 call sites is far cheaper than 5 separate restructures.
3. **`norm_bounded_extent` incremental-step contract** (`📕️norm/🖥️app-surface/🦀️.rs:490-552`) — lifting the hard-coded `Some(1)` cap fixes progress for `evaluate` across all 16 norm families at once.

### Wave 1 — wire-only, high priority, disjoint files (run in parallel)

| Owner scope | Files | What |
|---|---|---|
| Puzzle 3d fill ring cap | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/🧬️schema/🦀️.rs` (`FILL_TRIED_RING`), `…/⏳️precompute/🪣️fill/🦀️.rs` | Replace the fixed ring-of-12 with a paginated/bounded-but-complete tried-candidate stream (use `RetainedJobPayload`/`JobPayloadPageGrant` from §0.2) — satisfies phase-4 requirement 4 literally. |
| Puzzle brush cancel | `…/🧊️3d/…/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️.rs` | Add a real cancel action (currently deliberately `None`). |
| Puzzle 5d fill verification | `…/🖐️5d/…/🎮️commands/🧮️set-fill-count/🦀️.rs`, `…/🧠️precompute/🦀️.rs`, `…/🎮️commands/🛑️cancel-fill-build/🦀️.rs` | Read and confirm/fix the commit mechanism (§1.3's top open question). |
| Assembly WFC UI wiring | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/{🧬️schema/💡️inferences/🦀️.rs, ✏️editor/…}` | Thread `WfcJob::preview()` through `AssemblyInferenceJob::emit_preview`, build a command/window — **but only after** the documented process-abort defect (`📚️examples/🧪️tests/🧩️outcome/🦀️.rs:1-11`) is fixed; treat the fix as a hard prerequisite, not part of this wave's scope creep. |
| Generation3d import progress | `…/🧊️generation3d/…/✏️editor/🎮️commands/📥️import-document/🦀️.rs` | Confirm/wire a UI reader for `Generation3dImportStaging::open_runs`; add cancel. |
| Trinity/jack result streaming | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/…/🎮️commands/▶️run-query/🧵️job/🦀️.rs` | Stream result rows incrementally instead of one final replace. |
| Layout export preview | `📏️layout/…/⚙️engine/📤️export/🦀️.rs` | Surface partial-rendered-page preview before `Complete`; confirm the % actually reaches `WindowMeasure::Progress`. |
| Note dispatch-classification fix | `🗒️note/…/✏️editor/🦀️.rs` | Flip `addBlock`/`moveBlock`/`deleteBlock`/`deleteSelection`/`duplicateBlock`/`Selection`/`patchBlocks`/`nudgeSelection*`/`saveDownload` from `BatchOnlyPendingRewrite` to `Migrated` (pure dispatch-liveness fix, no algorithm change). |

### Wave 2 — restructure, disjoint files (run in parallel, some depend on Wave 0.1)

| Owner scope | Files | What |
|---|---|---|
| **Puzzle 3d fill transaction** (needs Wave 0.1) | `…/🧊️3d/…/🎮️commands/🪣️fill-build-tick/🦀️.rs`, `…/🎮️commands/🧮️set-fill-count/🦀️.rs`, `…/⏳️precompute/🦀️.rs` | The report's central finding: stage locked-but-uncommitted placements without writing `ctx.scene.fixture` until explicit finalize; make cancel actually discard staged placements. Use `import-fixture`'s clean chunk-then-atomic-replace pattern as the template. |
| **Puzzle 2d fill transaction** (needs Wave 0.1) | `◻️2d/…/🎮️commands/{🏁️fill-session-begin,👣️fill-session-step,🧹️fill-session-clear}/🦀️.rs` | Same fix, mirrored — currently also commits per-step directly despite richer-looking lifecycle labels. |
| FEM 2d results window | `🏗️fem/…/◻️2d/…/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs` | Route static/modal/buckling solve through `FemJobStage::Solve` (mirror fem3d's already-fixed `live_visual`/lease pattern — same plugin, no cross-plugin dependency). |
| Raster composite render | `🖨️raster/…/👁️viewer/…/🖼️composite/🦀️.rs`, `…/✏️editor/🦀️.rs:885-886`, host `rasterize_svg_to_png_base64` | Same "solve on render" fix, cache/job the raster path. |
| Process3d brep replay | `🏭️process/…/🧊️process3d/…/🧬️schema/💡️inferences/🦀️.rs:194-245`, `…/🪟️windows/🪚️workpiece/🦀️.rs` | Fix the memo cache lifetime bug (fresh `ProcessKernelReplay::new()` per call) first — cheapest fix — then stage the boolean chain. |
| Imperative `run` | `📜️imperative/…/🎮️commands/🏃️run/🦀️.rs`, `⚙️engine/🦀️.rs` | Needs a real per-step executor loop (depends on the external `imperative_engine` crate's own stepping support — verify first). |
| Sequence `run`/`stop` | `🎬️sequence/…/🎮️commands/🏃️run/🦀️.rs` | Make `Stop` actually interrupt; needs the same per-step executor treatment as imperative — coordinate on interface, not files. |
| Animate video export | `🎞️animate/…/⚙️engine/🎥️video/🦀️.rs`, `…/🎮️commands/🎥️export-video-from-deck/🦀️.rs` | Wrap frame-render+encode loop in a real job with per-frame progress. |
| Architect analysis family | `🏛️architect/…/🎮️commands/🔬️analysis/🦀️.rs`, `…/🧬️schema/💡️inferences/🦀️.rs` | Stage the 19 analysis kinds behind a real job, starting with the confirmed O(n²) `detect_adjacency_conflicts`. |
| Writer/architect document-replace commands | `✒️writer/…/🎮️commands/🧹️format-document/🦀️.rs`, `🏛️architect/…/🎮️commands/📤️exchange/🦀️.rs` | Both "parse + whole-document replace" shape — could share one restructuring approach even though separate plugins/files. |
| Note ink pipeline | `🗒️note/…/🎮️commands/🖊️ink-apply-events/🦀️.rs` | Chunk the batch instead of cloning+diffing the whole document per call, once reclassified live in Wave 1. |
| Space exports | `🪐️space/…/⚙️engine/🪐️space/🎮️commands/{📦️export-studio-pack,📤️export-media}/🦀️.rs` | Chunk/job the synchronous whole-studio/whole-node export. |

### Wave 3 — new-algorithm-visualization (needs design work, not just wiring; run in parallel per plugin)

- Lowpoly `decimate` (`💠️lowpoly/…/🎮️commands/🔷️mesh-edit/🦀️.rs`, kernel `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs:1273-1320`) — needs a genuinely incremental collapse loop (no full-rebuild-per-vertex), then a per-step preview design.
- Mathematical CAS exposure (`➗️mathematical/…/🌿️cas-internals/🦀️.rs`) — not urgent (nothing calls it yet), but design its future "Simplify"/"Solve" command's step model before wiring it up, so it doesn't launch already violating the contract.
- Trinity/rewriting rule application (`🔱️trinity/…/♻️rewriting/…/🧬️schema/🦀️.rs:279-291`) — needs a match-trace visualization design, not just chunking.
- Draw `combineBoolean` CSG (wherever the downstream CSG solve actually lives — locate it first, currently unverified/out of plugin tree).

### Cost-scaling flags — no lifecycle violation today, monitor / add a step budget defensively

`world-relocate` and several selection/attraction commands in puzzle 3d (§1.1), gis `bounds` inference, architect `topology`, sequence `topology`, dag `topology`, sourcing/writer's binary io (UNVERIFIED cost) — none need a progress UI today, but each does an unbounded document/feature-count scan with zero step budget; recommend a regression test asserting they stay sub-frame at realistic max document size, per phase-1's own recommendation (still open).

## 9. Shared framework touchpoints every conversion needs (cross-reference to §0.2)

Every wave-1/2/3 row above, regardless of plugin, will touch one or more of: `Effect::SpawnJob`/`JobPlacement` (start), `InteractiveJob`/`StepOutcome` (the step loop itself), `ActionBus`/`ToolExecutionContract`/`InteractiveJobClassification` (registration + the `Migrated` gate), `WindowMeasure::{Number,Progress}` (the UI surface, already wired end-to-end per commit `d8dce87ca0`), and — for every row marked contract-4-no — whichever provisional-mutation primitive Wave 0.1 lands. Conversions should NOT each invent their own transaction mechanism; that duplication is exactly what produced the puzzle 2d/3d divergence found in this audit (two different-looking lifecycle label sets, §1.2/§1.1, both failing point 4 identically underneath).
