# E5 — FILL feature deep dive: 3d reference, 2d port comparison, 5d port design

Scope: the FILL tool/feature across 🧊️3d (reference), ◻️2d (already-ported, compared), 🖐️5d (port design). All paths
below are relative to the repo root `/Users/ueli/Documents/semio`, quoted because of emoji. Read-only audit; no
source files were touched, only this report was written.

Vocabulary reminder: Node=Object=Part, Handle=Vortex=Grip, Edge=Attraction=Fastener.

---

## 1. 🧊️3d — the reference FILL implementation

Editor root: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`.

### 1.1 `⏳️precompute/🪣️fill/🦀️.rs` (3,293 lines) — the bounded fill planner

The **planner** is `FillBuilder` (declared ~line 392 in the "prepare" region), a resumable step-machine over
fixed-capacity owners (`FixedFixtureOwner`:267, `FixedCatalogOwner`:281, `RetainedBrushKindWeights`:481), a
`CollisionSpatialIndex`, and Fenwick-tree-backed weighted target/candidate pools. It never allocates unbounded
memory: every document-scale collection is a `FixedOwnerVec/Map/Set` capped by `DOCUMENT_*_SLOTS` constants from
`⏳️precompute/📐️geometry/🦀️.rs`, and a resumable "close ladder" (`retire_one_close_owner`, ~1136) lets
`terminal_is_empty` answer honestly one bounded unit at a time.

**Stage machine** — `FillJobStage` (~208): `RetractTail → PrepareFixture → PrepareCatalogs → PrepareMeshes →
PrepareEntries → PrepareSpatial → PrepareLookup → PrepareConfiguration → PrepareTargets → SelectTarget →
PrepareCandidates → SelectCandidate → ConstructPreview → QueryBroadPhase → TestCollision → AcceptCandidate →
Complete(FillPlanEnd)`. A plan cannot end silently: `FillPlanEnd` (60) is `Reached | Stalled(FillStall)`, and
`FillStall` (32) enumerates exactly `NoOpenVortex | NoCompatibleKind | NoFreePlacement | ArtifactCapacity` — every
stall is a declared, localized, visible step, never a silent clamp.

**`FillBuilder::advance()`** (~2252-2340) is the one bounded transition function, reused by both the retained
testing lane and `FillRunJob` (via a `FillRunTransitionContext`, 2445, whose fuel is the run's own candidate
budget, not collision-sample count). It checks cancellation, stale operation/generation, then dispatches on
`self.stage`.

**Measures computed:**
- **Count** — `requested_count()`/`set_requested_count()` (~1309-1341): raising keeps the existing sequence as an
  exact prefix and continues; lowering walks `RetractTail → discard_tail_one` one unapplied placement at a time.
- **Object/vortex-kind distribution weights** — `RetainedBrushKindWeights{object_weights, vortex_weights}` (481),
  loaded during `PrepareConfiguration` from `SceneConfig.weights.{object,vortex}_weights`. A composite key
  `"{object_kind}{JOINT_SEP}{vortex_kind}"` is looked up first (a per-pair joint weight), else the marginals are
  multiplied (`puzzle3d_joint_vortex_weight`, editor root `🦀️.rs:2289`: `object_weight * vortex_weight`). A weight
  of exactly `0.0` removes the entry from the Fenwick pool entirely.
- **Target volumes** — `FixedFixtureOwner.target_volumes` copied from `scene.fixture.target_volumes`; every
  constructed candidate's AABB must be contained by `world_volumes_contain_aabb` (~1783) or the candidate is
  rejected `outside-target-volume` — this is the only spatial placement *constraint* fill has (as opposed to a
  collision *rejection*).

**Settings reads:** entirely through `FillPreparationRoots{scene: Arc<SceneConfig>, meshes: Arc<HashMap<String,
CollisionBody>>}` (~255-264), consumed field-by-field one bounded unit per stage; `contact_tolerance` and host
rules copy wholesale once `PrepareConfiguration` finishes.

**Cancellation/progress/capacity:** `FillStepContext` trait (71) forwards `operation()/generation()/set_stage()/
fault_payload()`; a `PreparationCapacityBranch`/`PreparationCapacityRefusal` (322-390) preflights every
document-scale owner against its fixed slot ceiling *before* any placement — too large a document faults instead
of silently truncating; a mid-plan fixed-page refusal stalls the plan visibly (`ArtifactCapacity`) instead of
faulting.

**`FillRunJob`** (~2507) wraps `FillBuilder` + a `ToolRunTickWriter`. `start()` (~2559) picks one of three paths:
**replay** (checkpoint `inputs` match — deterministically re-derive the sequence with no visible ticks), **restart**
(provisional ops already exist, or generation advanced), or fresh **new**. `observe()` (~2739) folds
`FillRunEvent`s (`Constructed | Refused(reason) | VortexMarked | Accepted | Abandoned | Discarded`, ~101) into
trace ticks: one candidate verdict = one unit of run fuel, published as `testing` → `danger` (collision) / `warning`
(rule refusal) / `success` (placed). On `Accepted` the run appends `create_object` + `connect_vortices`
(`FILL_RUN_OPS_PER_PLACEMENT = 2`) as provisional ops, capped at `TOOL_RUN_PROVISIONAL_OPS_MAX`. `settle()`
(~2854) writes the terminal step from `FillPlanEnd` — an undeclared end faults `fill-run-end-undeclared`.
`checkpoint()` (~2644) encodes `FillRunCheckpoint{requested, placements, provisional_ops, tested, next_key,
inputs}` so a rebuilt job (after the framework closed its predecessor) reaches the same point by silent replay
(`FillRunJob::replaying`).

**`FillRevalidateJob`** (~3025) — runs at **finalize**, re-tests every provisional placement against the current
**head** document (not the base the run started from): rebuilds head collision entries/vortex owners
(`prepare_head_one`, ~3088), tests each placement's mesh against every head body (`test_placement_unit`, ~3113),
retracts to the first conflict and re-appends every later survivor (`finish`, ~3186), publishing one `danger` step
carrying `TOOL_RUN_REASON_CONFLICT` plus the conflict count. This is the concrete mechanism behind the framework's
generic **"Artifact changed, re-applying provisional result"** rebasing step — that message itself is
framework-generic, not fill-specific: it is defined in `🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs:2207`
(`Self::RebasingStep => ("rebasingStep", "Artifact changed, re-applying provisional result", "Artefakt geändert,
vorläufiges Ergebnis wird neu angewendet")`), driven by `ToolRunRebasePolicy::Revalidate` (declared by the fill
tool, §1.2) whenever the base document moves while a run is live.

**Publication lanes:** `JobPayloadStream::Preview` (tick pages via the `ToolRunTickWriter`), `::CheckpointState`/
`applied_progress`, `::Fault`, and on completion `::CommitState`/`::CommitOutput` (left empty — the framework tool
run ledger, never this job, owns publishing the actual document edit at finalize).

**Fixture** `⏳️precompute/🪣️fill/🧫️fixtures/🎞️fill-run.json`: language-neutral oracle keyed by `documentCapacities`
(fixed-slot ceilings + a Nakagin-scale profile: 180 objects / 358 vortices / 12 kinds), `opsPerPlacement: 2`, a
`parryOracle` section (real Parry3d collision cross-check — `parry3d = "0.17"` is a **test-only** dependency in
`🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml:65-66`, commented "Third-party collision oracle for the fill run
job verdicts"), `ownMesh` (a body must collide with its own mesh first, else its kind's), `delivery` (large-run
byte-budget law: seed 7, requested 1,000,000, candidates 5,000, `deltaBudgetBytes: 65536`), and `interactive`
(turn/µs budget law for Nakagin: seed 1, requested 100, turns 771, `budgetUs: 2000`, `coldRuns: 5`).

**Unit tests** `⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` (1,776 lines): fixed-collection cap/+1 refusal laws,
cancellation/staleness, watchdog/deadline bounds (`empty_fill_transition_stays_below_watchdog_ceiling`,
`adversarial_broad_phase_fill_is_end_to_end_resumable_below_eight_ms`), a `FillRunMirror` harness replaying ticks
against the fixture (`fill_run_job_matches_the_language_neutral_fill_run_fixture`), a real Parry3d oracle
cross-check (`fill_run_job_collision_verdicts_agree_with_the_parry3d_oracle`), raise/lower requested-count behavior,
checkpoint replay, revalidation-conflict, and visible-end-for-every-stall-reason tests.

### 1.2 `⏳️precompute/🦀️.rs` (module root, 1,387 lines)

This module owns the collision-mesh decode/registration session and brush-suggestion dispatch — **it does not
register or dispatch fill jobs itself**; the module doc states the fill planner "is owned by the framework tool
run ledger, never by this session." There is no plugin-local `JobKind` enum: job identity is the framework's
generic `semio_framework_tool_run::JobKindId` string. Module wiring is a plain Rust `pub mod` tree declared in the
artifact root `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🦀️.rs:~1692-1703`: `precompute { brush; fill; geometry; }`.

### 1.3 `🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` (312 lines) — the `ToolRunDefinition`

```
pub const TOOL_ID: &str = "fill";
pub const RUN_JOB_KIND: &str = "s.puzzle.puzzle3d.fill.run";
pub const REVALIDATE_JOB_KIND: &str = "s.puzzle.puzzle3d.fill.revalidate";
pub const RUN_SETTINGS_CONFIG: [&str; 4] = ["/fillCount", "/contactTolerance", "/objectKindWeights", "/vortexKindWeights"];
```

`run_definition()` (41-56): `ToolRunDefinition{ mutating: true, rebase: ToolRunRebasePolicy::Revalidate,
reconfigure: ToolRunReconfigurePolicy::Resume, trace: ToolRunTraceKind::Instance3d, run_job, revalidate_job:
Some(...), settings: ToolRunSettingsReads{config: RUN_SETTINGS_CONFIG,...}, windows: Vec::new() }`. There is no
plugin-local UI-state enum for "provisional results" — Start/Pause/Step/Abort/Finalize/Dismiss and the
provisional/rebase machinery are entirely framework-injected (module doc line 3), driven by this declaration's
`rebase`/`reconfigure` policies plus the stage/counter/reason vocabulary from `🗣️terminology/🦀️.rs:156-216`
(`puzzle3d_fill_run_unit/_stages/_counters/_reasons`).

`count_measure()` (62-76): `WindowMeasure::Number{ id:"puzzle3d-fill-count", value: fill_count, min:0.0, max:None,
step:1.0, loading: live_fill_run(tool_run).map(|_| true), on_change: puzzle3d_action("setFillCount", None) }` —
`max: None` is a deliberate product decision: the planner plans toward whatever the user typed and reports a
capacity it cannot reach as a visible stall, never a silent clamp. `measures()` (80-82) = `[count_measure,
puzzle3d_distribution_group(...)]`.

`abort_action()` (90-93) builds a `toolRunAbort` action bound to the live run's `run`/`generation` identity.

**Job build** (`build_run_job`, 100-117) reads `Puzzle3dConfig` into a `Puzzle3dRuntime`, builds `SceneConfig`,
computes the mesh lane, branches `ToolRunJobPurpose::Run` (checkpoint + provisional count) vs `::Revalidate`
(rebuilds `Vec<FillRunPlacement>` from provisional ops via `fill_run_placements`).

`Puzzle3dFillToolRunJob` (169) wraps `FillToolRunPhase{Preparing(...)|Run(FillRunJob)|Revalidate(FillRevalidateJob)
|Closed}`. **Preparation** (188-219) installs real collision meshes from the shared derived-mesh store (or a
scaled-box fallback) one mesh identity per unit, folding url+source digests into the run's `inputs` digest, then
transitions into `Run`/`Revalidate`. It implements `ToolRunRetargetableJob<Puzzle3dConfig>` (`rebind`/`reconfigure`,
225-256): a **count-only** reconfigure retargets the live job in place (`job.retarget(...)`); any other config
change (weights/tolerance) returns `false` so the framework rebuilds the whole job from scratch.

**Camera behavior:** no fill-specific camera code exists anywhere in the tool or `🪟️windows/🧊️main` files. The
comment at fill tool `🦀️.rs:28` is explicit: "A camera move or any other publication leaves [the run inputs]
unchanged and never reconfigures a run" — i.e. **fill deliberately does not move the camera**, and moving the
camera never disturbs a live fill run. The only camera-adjacent engagement verb is `"zoom"`
(`apply_puzzle3d_focus_selection`), unrelated to fill.

**Tool runs panel plumbing:** `🪟️windows/🧊️main/🦀️.rs:~329-345` marks each fixture object `placed` for rendering
by checking `ArtifactView::tool_run().provisional_entities` against `fill_run_entity(object.id)` (a digest hash,
precompute `🦀️.rs:2379`), so provisional fill instances render live in the world body — the "provisional results"
UX. `🪟️windows/🧊️main/🦀️.rs:~916-947` wires the engagement input's Escape/abort to `fill_tool::abort_action` while
a fill run is live, so Escape sends `toolRunAbort` instead of merely disarming the engagement bar.

### 1.4 `🎮️commands/🧮️set-fill-count/🦀️.rs` (19 lines) — the engagement bar `fill <n>` wiring

```rust
pub fn request(count: u32) -> Effect {
    Effect::DispatchAction { req: ..., action: "setFillCount".into(), args: ...{ "value": count }, delay_ms: 0 }
}
pub(crate) fn parse_count(args: Option<&Value>) -> u32 { /* reads count-or-value, rounds/clamps to u32 */ }
```

Doc comment: "the count is shared configuration only... this command never touches the document" — a fill run
reads it only when its job is built; the framework tool run driver reconfigures a *live* run when it changes.

The actual `fill <n>` verb is parsed in `🎮️commands/📨️engagement-submit/🦀️.rs:15-26`:
```rust
pub const PUZZLE3D_ENGAGEMENT_VERBS: &[&str] = &["brush", "fill <n>", "zoom", "clear", "pick", "rectangle", "lasso"];
```
`strip_engagement_prefix(raw, "fill")` arms the Fill utility, parses `<n>` (falling back to the current count),
pushes `set_fill_count::request(count)`, then dispatches `TOOL_RUN_START_ACTION_ID` with `{tool_id: "fill"}` — one
`fill <n>` submit both **sets the count and starts the run**. `🎮️commands/🔂️engagement-repeat-last/🦀️.rs`
implements "run one more": `fill_count.saturating_add(1)` then `set_fill_count::request` — the `Resume` reconfigure
policy continues the same run's deterministic sequence for exactly one more object.

### 1.5 The fill slider / distribution tree ("add/remove")

Editor root `🦀️.rs:2319-2365`: `puzzle3d_distribution_children`/`puzzle3d_distribution_group` build a nested
`WindowMeasure::Group` — one slider per object kind (`setObjectKindWeight`) with nested per-vortex-kind **joint**
sliders underneath (`puzzle3d_joint_vortex_measures`, 2296-2317, `setVortexKindWeight`): the displayed value is the
joint probability for that object/vortex pair; siblings under the same object header redistribute when one moves,
and the header disables at zero. There is no literal "add/remove slider" command — a slider row exists per
catalog kind and appears/disappears implicitly via `🎮️commands/🌱️add-object-kind` (catalogue add) rather than a
dedicated add/remove-weight-row verb. `🎮️commands/⚖️set-kind-weight/🦀️.rs` applies the redistribution.

Target-volume commands (constrain *where* fill may place, consumed via `FillBuilder.base.target_volumes` /
`world_volumes_contain_aabb`): `➕️add-target-volume/🦀️.rs` (grid-snapped, sized by voxel dims, aborts with a
notice if no usable `origin`), `🪦️delete-target-volume/🦀️.rs`, `🚚️relocate-target-volume/🦀️.rs` (skips locked
volumes), `🚩️set-target-volume-flag/🦀️.rs` (`hidden`/`locked`).

Action-table wiring: editor root `🦀️.rs:2375-2377` (`puzzle3d_fill_options_scope`) — a fill/distribution-slider
gesture repaints only the world body + fill-tool measures, **never full shell chrome**, a deliberate perf
decision (a full-chrome repaint measured 0.7s idle / seconds under load on the live `:6013` shell, per the same
file's comment on `puzzle3d_window_option_scope`).

### 1.6 Plugin-root and artifact-level test batteries

`✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-{p4e,run-job,trace}/🟦️.ts` are **TypeScript mutation
self-tests**: each asserts that a static-source-pattern "law checker" (`interactivityPuzzleFill*Failures`, from
`📜️script.ts`) correctly *rejects* a planted regression against a small embedded fixture of Rust/TS source
snippets (anchored find/replace edits), not literal integration tests of the real files.

- **`🔬️interactivity-puzzle-fill-p4e/🟦️.ts`** (237 lines): "P4e" laws (bounded-page/fixed-capacity, cooperative
  step, resumable spatial index, danger-before-fault) surviving the tool-run conversion. ~26 cases, e.g.
  `missing-cooperative-unit`, `dynamic-fixture-object-owner`/`dynamic-mesh-owner` (fixed→dynamic collection
  regression), `missing-plus-one-law`, `fault-before-danger-step`, `direct-spatial-mutation`.
- **`🔬️interactivity-puzzle-fill-run-job/🟦️.ts`** (221 lines): exports the shared `PUZZLE_FILL_TOOL_RUN_FIXTURE`
  (embedded `precompute`/`fill`/`geometry`/`editor`/`tool`/`terminology`/`schema`/`transport`/`renderer` snippets,
  **plus `puzzle5dPrecompute` and `puzzle5dWindow` snippets** — see §3.1, this fixture already encodes the
  intended 5d delegation shape) and `puzzleFillToolRunFixtureWith(name, edits)`. ~28 cases assert the tool-run
  contract itself: `restart-on-rebase`/`restart-on-reconfigure` (must be `Revalidate`/`Resume`, never `Restart`),
  `batched-fuel` (fuel must be consumed one unit at a time), `committed-placements` (must go through the tick
  writer, never a direct document apply), `silent-stall` (a stall step must be `Warning`, not `Info`).
- **`🔬️interactivity-puzzle-fill-trace/🟦️.ts`** (29 lines): reuses the run-job fixture; 12 cases assert fill must
  render through the **generic** `<ToolRunTraceLayer lane={lane} />`, forbidding any fill-specific diagnostic
  overlay/type (`diagnostic-overlay`, `fill-data-attributes`, `reveal-cutoffs`, `unmounted-trace-layer`,
  `legacy-preview-fixture`) — including two cases explicitly about **5d** (`diagnostic-5d-window`,
  `diagnostic-5d-precompute`), confirming the trace layer must stay generic across all three artifacts.

### 1.7 Key types/functions for a future port to know (fully qualified)

- `crate::editor::puzzle3d::precompute::fill::{FillBuilder, FillJobStage, FillStall, FillPlanEnd, FillRunEvent, FillStepContext, FillPreparationRoots, FillRunJob, FillRevalidateJob, FillRunPlacement, fill_run_placements, fill_run_entity, fill_run_ops}`
- `crate::standards::v1::subsets::any::schema::{FillRunStage, FillRunCounter, FillRunReason, FillRunCheckpoint}`
- `crate::editor::puzzle3d::modes::edit::tools::fill::{TOOL_ID, RUN_JOB_KIND, REVALIDATE_JOB_KIND, RUN_SETTINGS_CONFIG, definition, run_definition, count_measure, measures, abort_action, build_run_job, Puzzle3dFillToolRunJob, FillRunInputs, FillToolRunTarget}`
- `crate::editor::puzzle3d::commands::{set_fill_count::{request, parse_count}, engagement_submit::{engagement_submit, PUZZLE3D_ENGAGEMENT_VERBS}, engagement_repeat_last}`
- `crate::editor::puzzle3d::{puzzle3d_distribution_group, puzzle3d_distribution_children, puzzle3d_joint_vortex_measures, puzzle3d_fill_options_scope}`
- `semio_framework_tool_run::{ToolRunDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunTickWriter, ToolRunTraceSubject, ToolRunVerdict, JobKindId}`
- `semio_framework_plugin::{ToolRunJobRequest, ToolRunJobPurpose, ToolRunRetargetableJob, ToolRunView, WindowMeasure}`
- `semio_framework_job::{InteractiveJob, StepContext, StepOutcome, Operation, RetainedJobPayload}`
- Framework-generic rebase message: `🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs:2207` (`RebasingStep`).

---

## 2. ◻️2d — how FILL was ported, what differs, what's missing

Editor root: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`.

**Structural fact first:** unlike 5d, **2d's crate has no `Cargo.toml` dependency on `semio-s-artifact-puzzle-3d`**
(verified: `🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml` lists no `puzzle3d`/`puzzle2d` cross-path). 2d fill is
a **fully independent reimplementation** against 2d's own "board engine" (`BoardFillJob`), not a delegation to 3d's
planner — this is the opposite pattern from 5d (§3), and matters for the port plan: 2d proves the "mirror the shape,
reimplement the mechanics" path is viable and shipped; 5d instead proves the "delegate to 3d's real planner and
translate" path is viable and already shipped for the run job itself.

### 2.1 `⏳️precompute/🪣️fill/🦀️.rs` (1,621 lines) — `Puzzle2dFillRunJob`/`Puzzle2dFillRevalidateJob`

Not "`Puzzle2dFillSessionWork`" (that name does not exist in current code — the old bespoke session-lifecycle
commands, `brushFillSession{Begin,Step,Cancel,...}`, were fully retired when 2d fill converted to the same generic
`ToolRunDefinition`/`InteractiveJob` machinery 3d uses; they now survive only in a historical migration-inventory
ticket file, `📜️script.ts:9761`, never in live source).

Module doc: "the run job streams the document into the board engine's fill ingress, drives its `BoardFillJob`
search and reports every candidate the search decides as a `placement2d` trace record." Key types: `FillRunStage`
(`Capture, Search, Test, Place, Retract` — 5 stages, one more "Capture" than a literal 3d mirror since 2d must
first stream the whole document into the engine's ingress), `FillRunCounter` (4: `Tested, Accepted, Collisions,
Rejected` — no `Marked`, since 2d has no vortex-marking search strategy), `FillRunReason` (11 variants, versus
3d's ~23 — 2d has no mesh/broad-phase/spatial-index failure modes at all, since its collision test is a
synchronous AABB overlap over JSON fields, not streamed real geometry).

### 2.2 `🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` (99 lines) — the ported `ToolRunDefinition`

```
pub const PUZZLE2D_FILL_RUN_JOB: &str = "puzzle2d.fill.run";
pub const PUZZLE2D_FILL_REVALIDATE_JOB: &str = "puzzle2d.fill.revalidate";
pub const RUN_SETTINGS_CONFIG: [&str; 3] = ["/fillCount", "/nodeKindWeights", "/handleKindWeights"];
```
`run_definition()` is the same contract shape as 3d's: `mutating: true, rebase: Revalidate, reconfigure: Resume,
trace: ToolRunTraceKind::Placement2d`. `count_measure()` mirrors 3d's `WindowMeasure::Number` **except** it
hardcodes `ready/loading/waiting: None` (a 2d unit test explicitly asserts this triple is always `None`, "run
progress lives in the ToolRun panel") — 2d's count measure never reflects run-liveness the way 3d's
`loading: live_fill_run(tool_run).map(|_| true)` does. `measures()` appends `puzzle2d_distribution_measures`
(node/handle-kind weight trees) instead of 3d's object/vortex distribution group — **no target-volume / spatial
zone concept exists in 2d at all**.

### 2.3 `brushFillSession*` / `inferred_node_kind_rows` search

`brushFillSession*` verbs: **zero live hits** in the 2d source tree — confirmed retired (§2.1). `inferred_node_kind_rows` is defined **once**, 2d editor root `🦀️.rs:396-432`, called from `inferred_kind_entries` (:367)
and from fill's own `fill_kind_rows` (⏳️precompute/🪣️fill/🦀️.rs:283-293). It has **no 3d counterpart** — 3d
always requires an explicit `VortexKindCatalog`/`ObjectKind` catalog. What it does: for a document carrying no
`meta.kindCatalogs` and whose engine manifest also carries no catalog rows, it derives one synthetic node-kind row
per distinct `nodeKind` string actually present in the document (shape/size/icon from the first matching node's
own fields). `fill_kind_rows` tries, in priority order: (1) `meta.kindCatalogs.nodes`, (2) the engine manifest's
catalog rows *only if* they carry non-empty handle templates, (3) this inference fallback — letting fill work on
manifest-less documents (e.g. "Concrete Forest"-style fixtures) that 3d's stricter catalog requirement would
reject.

### 2.4 `set-fill-count` and engagement bar

2d's `🎮️commands/🧮️set-fill-count/🦀️.rs` is structurally identical to 3d's (count-or-value parse, clamp,
write shared config, no document touch, unit tests present) — no gap here.

### 2.5 Diff summary (2d vs 3d)

**MATCHES:** the `ToolRunDefinition` contract shape (`mutating`/`rebase: Revalidate`/`reconfigure: Resume`/
run+revalidate job pair/settings reads); the run/revalidate job split and checkpoint-driven resume-on-raise /
retract-on-lower behavior; `set-fill-count`'s shape; the per-candidate `testing→danger/warning/success` trace
model backed by schema-validated stage/counter/reason enums; the language-neutral oracle-fixture testing pattern
(`🧫️fixtures/🎞️fill-run.json`); both artifacts have fully retired their old bespoke session-lifecycle command
verbs in favor of the generic ToolRun framework.

**DIFFERS BY DESIGN (not a defect):** no vortex/target-volume distribution or spatial zone concept (2d fill only
ever places against the document's own nodes/handles); no mesh-collision preparation phase (2d's `FillCapture`
document-streaming stage replaces 3d's real-geometry mesh-preparation phase); 2d achieves 3d's
`ToolRunRetargetableJob::reconfigure` in-place-retarget behavior by **rebuilding from checkpoint and silently
replaying** the deterministic search instead — 2d never implements `ToolRunRetargetableJob` at all; a much smaller
reason/counter taxonomy (11 reasons / 4 counters vs 3d's ~23 / 5) reflecting the absence of spatial-index/mesh
failure modes.

**MISSING vs 3d (worth flagging, not necessarily wrong):** no plugin-local `abort_action` convenience helper in
2d's tool module (the framework action still fires — organizational gap only); no `loading`-tied count measure
(deliberate, per 2d's own test, but a UI affordance 3d has that 2d explicitly does not); no
`ToolRunRetargetableJob` mechanism-parity (outcome parity is achieved differently, see above — flag only if
mechanism parity itself matters to the audit, not just product behavior).

---

## 3. 🖐️5d — what exists today, and the concrete port plan

Editor root: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`. **Note the
precompute directory uses a different emoji than 3d/2d**: `🧠️precompute` (brain), not `⏳️precompute` (hourglass).

### 3.1 The central finding: 5d's fill *content* is already implemented by delegation to 3d

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/Cargo.toml` depends directly on
`semio-s-artifact-puzzle-3d` (line ~34), with an explicit comment: *"⏯️ Tool run contract: the fill run job
translates the 3d planner's `ToolRunTick` pages into 5d ones."* This is **not** aspirational — it is live code:

`🧠️precompute/🪣️fill/🦀️.rs` (69 lines) — `build_run_job` calls
`semio_s_artifact_puzzle_3d::editor::puzzle3d::modes::edit::tools::fill::build_run_job(...)` directly (line 31),
wrapping a `Puzzle5dPlannerBoard` and returning a `Puzzle5dPlannerToolRunJob` that:

1. Maps 5d's `Puzzle5dPart`/`Puzzle5dFastener` document into 3d's `Puzzle3dObject`/`AttractionProps` graph
   (`puzzle3d_ops`, precompute root `🦀️.rs:158-176`, `~187`).
2. Runs the **real** 3d fill planner (`FillBuilder`, real collision meshes, real `contact_tolerance`, real
   object/vortex-kind weights — everything from §1) entirely in **3d/spatial space**.
3. Translates each `ToolRunTick` back into 5d ops via `Puzzle5dPlannerBoard::translate()` (precompute root
   `🦀️.rs:354-395`): decodes the 3d `create_object`/`connect_vortices` pair, calls `adopt()` to build a matching
   `Puzzle5dPart`/`Puzzle5dFastener`, re-encodes as `create_part`/`connect_grips`, and **pairs every 3d
   `instance3d` trace record with a twin `placement2d` trace record** (same key with a "twin" bit set,
   `Self::twin`, 397-402) so both windows (2d board, 3d world) see the placement live in the same tick.
4. The 2d flat position of a newly placed part is **not independently searched** — it comes from
   `Self::candidate_center`/the part's own `part_2d.{x,y}` set at `adopt()` time (~306-311, `self.beside(host,
   PUZZLE5D_DEFAULT_PART_RADIUS)`, a simple "beside its host grip" placement in the 2d board's own point grid,
   `Puzzle5dPointGrid`), independent of whatever spatial position 3d chose.

`🧠️precompute/🖌️brush/🦀️.rs` (56 lines) follows the identical bridge pattern for brush suggestions.

A test fixture confirms this is the *intended*, law-checked shape, not incidental: 3d's own mutation-self-test
fixture (`✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-run-job/🟦️.ts:149-158`) embeds:
```rust
// puzzle5dPrecompute:
pub fn fill_run_job(inner: &Puzzle3dPrecomputeSession, scene: Arc<SceneConfig>, operation: FillOperation) -> FillRunJob {
    inner.fill_run_job(scene, operation)
}
// puzzle5dWindow:
pub fn world_scene_body(precompute: &Puzzle5dPrecomputeSession) -> WorldScene {
    WorldScene::from_session(precompute)
}
```
i.e. the law-checker itself expects and protects a 5d precompute session that thinly wraps 3d's, matching what
actually exists.

### 3.2 Dual-pose Part/Grip model — already solved for FILL's purposes

`Puzzle5dPart` (5d editor root `🦀️.rs:277-290`) carries **both poses simultaneously**:
- `part_2d: Puzzle5dPart2d` (`🦀️.rs:236-259`): `x, y, shape, radius, width/height, text, icon_kind, hidden, locked`
  — flat/plan pose + 2d-only display fields.
- `part_3d: Puzzle5dPart3d` (`🦀️.rs:261-274`): `origin: [f64;3], mesh_url, orientation: Option<[f64;4]>
  (quaternion), scale, label` — spatial pose.

`Puzzle5dGrip` (`🦀️.rs:190-200`) is dual-pose the same way: `grip_2d: {angle, grip_kind, radius}` +
`grip_3d: {position: [f64;3], direction, radius, label}`. `Puzzle5dFastener` (`🦀️.rs:212-234`) is **not** split —
a single flat struct mixing 3d-style params (`gap, shift, rise, rotation, turn, tilt`) with 2d-style diagram
offsets (`x, y`).

**How the dual pose is reconciled today:** fill runs entirely in 3d space (§3.1); the 2d pose comes from a cheap
local "place beside the host grip in the 2d point grid" heuristic at `adopt()` time, **not** from an independent
2d search or an independent 3d↔2d collision-consistency check. Separately, a general-purpose flatten solver exists
— `🧬️schema/💡️inferences/🎛️flat-position/🦀️.rs` calls `semio_s_artifact_puzzle_3d::flatten_objects` to derive
`flatPositions` for the *whole graph* as a deterministic post-hoc projection of the 3d structure. Fill's own
`adopt()`-time 2d placement is a separate, cheaper mechanism than this general flattener; whether they should be
unified is a design question outside this audit's scope but worth flagging for the port plan below.

**Collision/placement rules:** 5d has **zero independent collision code**. The only "collision" vocabulary present
is already-localized fill-reason strings in `🗣️terminology/🦀️.rs` (`fill_reason_solid_overlap`: "Collides with a
placed part", `fill_stage_test`: "Testing collision", `fill_counter_collisions`: "Collisions") — the actual
verdicts come entirely from 3d's real planner via the bridge in §3.1. `rstar = "0.12"` is a 5d **dev-dependency**
only ("spatial index oracle for the grip grid radius and nearest queries. Test-only.") — not production collision
code.

### 3.3 The actual gap: FILL is a Utility fallback, not a first-class Tool

`✏️editor/🦀️.rs:8195-8201` (verified directly): the manifest builder calls `.utility(...)` **seven** times and
**zero** `.tool(...)` times:
```rust
.utility(board2d::utilities::select::definition(...))
.utility(world3d::utilities::transform::move_definition())
.utility(world3d::utilities::transform::rotate_definition())
.utility(world3d::utilities::transform::scale_definition())
.utility(board2d::utilities::brush::definition(...))
.utility(board2d::utilities::fill::definition(...))
.utility(world3d::utilities::world_relocate::definition())
```
There is **no `🛠️tools/` directory anywhere under the 5d editor** — only `☑️options/{🖌️brush,🪣️fill}` (mode-level
Utility Options groups, shared verbatim by both windows per the doc comment at `☑️options/🪣️fill/🦀️.rs:5-7`) and
`🪟️windows/◻️2d/🪛️utilities/{🖌️brush,🪣️fill}` (the canonical utility definitions, `UTILITY_ID = "fill"` +
`ToolRunDefinition`, doc'd as "SHARED BY BOTH WINDOWS — declared ONCE here"). `🪟️windows/🧊️3d/🦀️.rs:~36-53` binds
the same `board2d::utilities::fill::UTILITY_ID` rather than defining a separate 3d-window fill.

Because fill is only a Utility, `build_tool_run_job` (editor root `🦀️.rs:~7608-7613`) reaches it only as the
**default fallback arm** after checking `brush::UTILITY_ID` explicitly — it has no dedicated Tool activate control
and, per the corroborating same-day report `📓️E3-5d-gap-vs-3d.md` (§7 "Fill tool", row 12: *"PARTIAL — setFillCount
Migrated and precompute wired, but no real `ToolDefinition`/activate control exists — reached only via the Fill
utility rail, no ToolRun panel chrome"*), it therefore has **no `#tool.fill` activation control and no standard
ToolRun panel Start/Pause/Step/Abort/Finalize/Dismiss chrome** — even though the underlying run/revalidate job is
the real, fully-working 3d planner. This matches the E3 report's independent finding at `🦀️.rs:3863-3879`
(`PUZZLE5D_RETAINED_TOOL_IDS` has no fill/tool-activate id at all).

Brush is the second real gap, but for a *different* reason: brush candidates/preview already work, but placement
commands (`🎮️commands/🖌️add-brush-part`, `📋️register-brush-mesh`, `🎣️target-brush-suggestions`,
`🚧️set-brush-placement-contact-tolerance`) are classified `BatchOnlyPendingRewrite` (dead) in the action-dispatch
table per E3, despite complete command bodies — this is a registry/classification gap, not a missing-code gap.

Target volumes: **entirely absent from 5d** — no `add`/`delete`/`relocate`-target-volume command or utility exists
(confirmed by `find -iname "*volume*"` returning nothing under the 5d editor, and independently by E3 §"prioritized
work" row on target volumes).

### 3.4 PORT PLAN — files to create/change, with sources to port from

**(a) Precompute fill job — already ported, no work needed.**
`🧠️precompute/🪣️fill/🦀️.rs` (69 lines) and `🧠️precompute/🖌️brush/🦀️.rs` (56 lines) already bridge to 3d's real
planner correctly (§3.1). Treat these as done; do not re-derive them from 3d's `⏳️precompute/🪣️fill/🦀️.rs` —
that would duplicate collision/planning logic 5d already gets for free via the crate dependency.

**(b) FILL tool definition/UI states — CREATE, near-copy adaptation.**
- **CREATE** `🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`, adapted from 3d's
  `🗿️artifacts/🧊️3d/.../✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` (312 lines, §1.3) — but the mechanical
  precedent to copy the *shape* from is actually 2d's already-ported
  `🗿️artifacts/◻️2d/.../✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` (98 lines, §2.2), since 2d's version proves
  what a per-artifact port of this exact file looks like once trimmed to that artifact's own settings/vocabulary.
  For 5d specifically: `TOOL_ID = "fill"` (already used as `UTILITY_ID` today — reuse the same string so no
  document/config field renames are needed), `run_definition()` should keep the settings-reads list 5d's existing
  `☑️options/🪣️fill/🦀️.rs`/`🪛️utilities/🪣️fill/🦀️.rs` already imply (`/fillCount`, `/contactTolerance`,
  `/objectKindWeights`, `/vortexKindWeights` — identical to 3d's since the planner *is* 3d's), `trace:
  ToolRunTraceKind` should stay whatever the existing utility declares today (verify against
  `🪟️windows/◻️2d/🪛️utilities/🪣️fill/🦀️.rs` before assuming `Instance3d` vs `Placement2d` vs a 5d-specific dual
  kind — the twin-trace-key mechanism in §3.1 step 3 suggests both trace kinds are already emitted per tick, so the
  tool declaration likely needs to reference whatever `ToolRunTraceKind` the framework uses for dual/paired
  traces, if one exists, else keep the existing utility's kind).
- **CHANGE** `✏️editor/🦀️.rs:8195-8201` — add one `.tool(...)` registration call for the new fill tool definition,
  alongside (not replacing) the existing `.utility(board2d::utilities::fill::definition(...))` call if the Utility
  Options group is kept feeding the ToolRun panel's settings UI (see below).
- **CHANGE** `✏️editor/🦀️.rs:~7608-7613` (`build_tool_run_job`) — branch explicitly on the new fill `TOOL_ID`
  instead of relying on the "everything that isn't brush" default fallthrough.
- **CHANGE** `✏️editor/🦀️.rs:3863-3879` (`PUZZLE5D_RETAINED_TOOL_IDS`) — add the fill tool id so `#tool.fill`
  activation and the standard ToolRun panel chrome resolve.
- **KEEP, minor wiring only:** `☑️options/🪣️fill/🦀️.rs` (56 lines) — the mode-level Utility Options group (count
  measure with `loading: fill::live_fill_run(tool_run).map(|_| true)` — note this file **already has** the
  `loading`-tied wiring 2d is missing, §2.2 — reuse it as-is for the new Tool's measures, just repoint its
  `fill::live_fill_run` import if the underlying utility module path changes).

**(c) Commands.**
- `🎮️commands/🧮️set-fill-count/🦀️.rs` — done, no change.
- `🎮️commands/{🖌️add-brush-part,📋️register-brush-mesh,🎣️target-brush-suggestions,
  🚧️set-brush-placement-contact-tolerance,🔁️cycle-brush-candidate}/🦀️.rs` — bodies exist; **CHANGE** their
  classification from `BatchOnlyPendingRewrite` to `Migrated` in the action-dispatch/registry table (same
  `✏️editor/🦀️.rs` region as the retained-tool list) — this is a brush gap adjacent to fill, not fill itself, but
  blocks a complete "brush suggests, fill places" story if left dead.
- Target volumes — **entirely missing, CREATE new command modules** ported from 3d's
  `🎮️commands/{➕️add-target-volume,🪦️delete-target-volume,🚚️relocate-target-volume,🚩️set-target-volume-flag}/🦀️.rs`
  (§1.5). This is the one piece of fill-adjacent 3d functionality 5d has no analog of at all. Flag for product
  confirmation before porting mechanically: 5d's brush/fill already places by "beside the host grip" (§3.1 step 4)
  rather than by spatial voxel volumes, so a literal 3d-style target-volume port may not fit 5d's board-relative
  placement model as cleanly as it fits 3d's free-space world — this needs a design decision, not just a copy.
- Engagement bar `fill <n>` wiring — verify `engagement-submit`/`engagement-repeat-last` equivalents exist and are
  `Migrated`; port from 3d's `🎮️commands/📨️engagement-submit/🦀️.rs`/`🔂️engagement-repeat-last/🦀️.rs` (§1.4) if
  missing or still dead in the dispatch table.

**(d) Schema.** No new fields needed for the dual-pose model — `Puzzle5dPart{part_2d,part_3d}`/`Puzzle5dGrip
{grip_2d,grip_3d}` already carry both poses, and `💡️inferences/🎛️flat-position/🦀️.rs` already derives a
whole-graph 2d projection via 3d's `flatten_objects` (§3.2). **CHANGE** (verify, may already be correct):
`🧠️precompute/🪣️fill/🦀️.rs:52-60`'s `puzzle3d_ops` only handles `Puzzle5dMutation::CreatePart`/`::ConnectGrips` —
any new command class added in (c) (e.g. target-volume placement) needs a matching arm here or the bridge errors
`puzzle5d-fill-run-provisional`.

**(e) Tests.** Mirror `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-{p4e,trace,run-job}/🟦️.ts`
(§1.6) — the run-job/trace fixtures are **already partially 5d-aware** (`puzzle5dPrecompute`/`puzzle5dWindow`
snippets, `diagnostic-5d-window`/`diagnostic-5d-precompute` cases, §3.1), so extend these in place with a new
diagnostic anchor for the new `🛠️tools/🪣️fill/🦀️.rs` module rather than writing a new suite. **CREATE**
`🎭️modes/✏️edit/🛠️tools/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` mirroring 3d/2d's own tool-definition unit tests (shape of
`run_definition()`, count measure bounds, schema-table equality). **CHANGE**
`🧠️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` to add coverage for explicit tool-id dispatch once `build_tool_run_job`
branches on the fill tool id instead of the default-arm fallback.

**Sources most load-bearing for this plan:** 5d `✏️editor/🦀️.rs` (action registry ~8131-8201, retained-tool list
3863-3879, `build_tool_run_job` ~7608-7613 — spot-verified the `.tool`/`.utility` count directly), 5d
`🧠️precompute/🪣️fill/🦀️.rs` + precompute root `🦀️.rs`, 5d `☑️options/🪣️fill/🦀️.rs` and
`🪟️windows/◻️2d/🪛️utilities/🪣️fill/🦀️.rs`, 5d `🧬️schema/💡️inferences/🎛️flat-position/🦀️.rs`, 3d
`🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`, 2d `🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`, and the same-ticket prior report
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/📓️E3-5d-gap-vs-3d.md` (310 lines,
written earlier the same day, whose fill-specific findings corroborate this section's independently — line numbers
attributed to it above were not re-verified live except where explicitly noted as spot-checked).

---

## Summary

- **3d** has the complete, canonical FILL implementation: a bounded, resumable, fixed-capacity spatial planner
  (`FillBuilder`) driving two framework `InteractiveJob`s (`FillRunJob`/`FillRevalidateJob`) behind a
  `ToolRunDefinition`, with real collision meshes, target-volume constraints, joint object/vortex-kind weighted
  distribution, and an extensive test/fixture apparatus.
- **2d** independently reimplements the same `ToolRunDefinition`/job-pair *shape* against its own synchronous
  board engine (no 3d dependency), achieving outcome parity (raise/lower, revalidate-at-finalize, checkpoint
  replay) through a rebuild-and-replay mechanism rather than 3d's in-place `ToolRunRetargetableJob::reconfigure`.
  Its gaps versus 3d (no target volumes/vortex distribution, no `loading`-tied count measure, no plugin-local
  `abort_action`) are mostly deliberate, dimension-appropriate omissions, not oversights.
- **5d** takes a third approach — delegate the actual planner to 3d wholesale via a crate dependency and a
  tick-translation bridge (`Puzzle5dPlannerBoard`/`Puzzle5dPlannerToolRunJob`) — and this delegation is **already
  built and working** for the run/revalidate job, the dual-pose Part/Grip schema, and the 3d↔2d trace-key twinning.
  The real, narrow gap is that FILL was never promoted from a mode-level Utility to a first-class framework Tool,
  so it has no ToolRun panel chrome or `#tool.fill` activation control — a wiring/registration fix (§3.4b), not a
  planner-logic port. Target volumes are the one 3d fill-adjacent feature 5d has no analog of at all, and need a
  product decision (not just a mechanical port) given 5d's grip-relative placement model.
