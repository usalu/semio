# Explore: Copyable action-migration recipe (`BatchOnlyPendingRewrite` → `Migrated`)

Generated: 2026-09-05, read-only Sonnet explorer. Every claim below was verified against current
source in this working tree (not the committed `🔣️.json` descriptors, which drift — see §2.5) via
`grep`/`sed`/`python3 -m json.tool` through Bash. No edits were made anywhere.

---

## 1. The framework contract — where everything lives

All of the following live in **one file**:
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (39,084 lines). Line numbers below are from
this session's read of current source; the ticket brief's `~11915` for the dispatch gate is stale by
~130 lines (the file has been edited since) — use the anchors below, not that number.

### 1.1 `InteractiveJobClassification` enum
Four variants used across the repo: `Migrated`, `BatchOnlyPendingRewrite`, `ForbiddenFromUi`,
`Unclassified` (plus `Deleted`, seen in test code at `🦀️.rs:7308`, `:36937`). Declared in
`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (per the prior puzzle3d recipe, `:829-968`; not re-verified
byte-for-byte this session, but the enum's four call sites above were).

### 1.2 Declaring/flipping classification on an app builder
- `AppBuilder::action_interactive_job(action_id, classification)` — `🦀️.rs:5197-5208`. Searches bare
  actions, window-kind actions, bare commands and mode commands by id; a miss leaves the id
  `Unclassified`.
- `AppBuilder::interactive_jobs(classification)` — `🦀️.rs:5211-5219`. Blanket-sets every declared
  action/command (used by lowpoly, `sequence`, etc., not per-id).

### 1.3 The dispatch gate — `validate_ui_dispatch_classification`
`🦀️.rs:12041-12047`:
```rust
fn validate_ui_dispatch_classification(owner: &str, id: &str, classification: semio_framework::InteractiveJobClassification) -> Result<(), Fault> {
    if classification == semio_framework::InteractiveJobClassification::Migrated { Ok(()) }
    else { Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.not-ui-safe"), format!("UI dispatch rejected {owner}:{id} with interactive-job classification {classification:?}"))) }
}
```
Called from four call sites, **before command construction**, so a rejected action never even
reaches the app's reducer:
- `dispatch_action` — `🦀️.rs:22368` (string-verb UI actions)
- `dispatch_command` — `🦀️.rs:22415` (manifest app/mode commands)
- two typed-command wire/JSON admission paths — `🦀️.rs:19407`, `:19438`

Fault code emitted on rejection: **`interactive-job.not-ui-safe`**, `FaultOrigin::Framework`. This is
the fault every dead action in the per-plugin blocker table produces when clicked in the UI.

### 1.4 `bounded_first_step_tool_proofs!` macro
`🦀️.rs:12667-12719` (two arms — with/without `factory_type:`). Generates
`fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof>`, one `ArtifactBoundedFirstStepProof::new::<$owner>(...)` row per `tools: { "id" => contract, ... }` entry, each optionally
qualified `.with_factory_type::<$owner, $factory_type>()` (`:12589-12593`).

### 1.5 `factory_type` and the owned tool-job factory trait
- `semio_framework::ToolJobFactory` — the base trait (declared in
  `🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs`, not re-read this session; methods used everywhere below:
  `keys()`, `payload_schema_id()`, `classification()`, `execution_contract()`, `create_job(...)`,
  `create_job_from_wire_pages_with_payload(...)`).
- `semio_framework_plugin::ArtifactOwnedToolJobFactory: ToolJobFactory` — `🦀️.rs:12742-12760`. Adds
  `type Owner: ArtifactApp`, `const TOOL_IDS: &'static [&'static str]`, `const DOCUMENT_SCHEMA: &'static str`, `const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract]`.
- `ArtifactToolPublicationLane` enum — `🦀️.rs:12722-12730`: `HostOnly | Artifact | Config | Draft | Presence | Transient | Child`.
- `ArtifactToolFactoryRegistry<'a, A: ArtifactApp>::register<F>` — `🦀️.rs:12848-12889`. What
  `ArtifactEditor::register_tool_job_factories` calls. Enforces, in order, all fail with named fault
  codes:
  1. `F::DOCUMENT_SCHEMA == A::DOCUMENT_SCHEMA` and non-empty payload schema id, else
     `interactive-job.owner-schema`.
  2. `F::TOOL_IDS` is an **exact bijection** with `factory.keys()`'s tool ids, and every key's
     `controller_id` matches this registry's controller, else `interactive-job.owner-key`.
  3. `PUBLICATION_CONTRACTS` covers exactly the same id set, every lane list non-empty, and `HostOnly`
     (if present) is the *sole* lane for that tool, else `interactive-job.publication-contract`.
  4. `factory.classification() == Migrated`, else `interactive-job.owner-classification`.
  5. No duplicate tool id across factories on this registry, else `interactive-job.owner-duplicate`.

### 1.6 The build-time completeness gate (separate from the dispatch gate)
`AppActionRegistry::validate_tool_job_rows` (via `tool_job_registration`), `🦀️.rs:12177-12243` (line
numbers in this session's read; the prior puzzle3d recipe cited `:12059-12137` under slightly older
line numbers — same function). Computes `migrated = ids classified Migrated` and requires
`A::bounded_first_step_tool_proofs()` to contain **exactly one row per migrated id**, each either a
bare "generic" row (`factory == BOUNDED_FIRST_STEP_FACTORY`, no factory type — routes through the
shared `TypedCommandFullOperationJob`) or an "exact" row matching a live
`ArtifactOwnedToolJobFactory` registration (factory type id/name, owner witness, controller, contract
all cross-checked). Mismatch: `interactive-job.catalog-authority` (names the specific mismatched
field). Missing proof for a migrated id: `interactive-job.catalog-incomplete`. **This runs at app
construction, not at dispatch** — flipping a classification without a matching proof row makes the
whole app fail to build, not just that one action fail at click-time.

### 1.7 Viewer read-only enforcement — a *separate* mechanism from classification
- `VIEWER_REJECTED_ACTION_IDS: [&str; 7]` — `🦀️.rs:19160`: `["undo","redo","commitCheckpoint","createAlternative",REVERT_TO_COMMAND_ACTION_ID,"cut","paste"]`. Checked in `dispatch_action`
  (`🦀️.rs:22363-22364`) before the classification gate even runs, for any `A::ROLE == AppRole::Viewer`.
  Fault: `viewer.read-only` (`FaultOrigin::Framework`), `🦀️.rs:19169-19171`.
- **Structural** backstop in `dispatch_emit` — `🦀️.rs:20978-20980`: `if A::ROLE == AppRole::Viewer && !artifact_mutations.is_empty() { return Err(viewer_read_only_fault(verb)); }`. Belt-and-suspenders for a
  hand-written `ArtifactApp` impl; unreachable through the real `ViewerApp<V>` adapter because...
- `ArtifactViewer::handle` returns `ViewEmit<ConfigMutation>` (`🦀️.rs:27046`), a struct
  (`🦀️.rs:27131-27135`) with exactly three fields: `config_mutations: Vec<ConfigMutation>`,
  `effects: Vec<Effect>`, `ui_dirty: UiDirtyScope`. **No `artifact_mutations` field exists** — a viewer
  cannot emit one even if it tried; this is a type-level guarantee, not a runtime check
  (`🦀️.rs:27129-27130` doc comment says so explicitly).
- `ViewerApp<V>` (`🦀️.rs:27419-27436`) is the sole runtime adapter converting `ViewEmit` into `Emit`
  with `artifact_mutations`/`draft_mutations` always empty by construction.

These two things — the classification gate and the viewer-role guard — are independent. A viewer's
own declared actions still need `Migrated` + a valid proof row to be dispatchable from the UI at all;
being a viewer doesn't exempt them from §1.3-1.6, it only additionally forbids them from ever carrying
an artifact/draft mutation.

---

## 2. Step-by-step recipe (exact shapes from lowpoly + generation3d)

Reference files read in full for this section:
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
  (48/48 Migrated; factory at lines 979-1057, proofs macro at 1607-1633)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
  (recently migrated from 23/6 to 29/0; factory at lines 165-317, `ArtifactEditor` overrides at
  685-736, proofs macro at 737-767)

### 2.1 Declare the retained tool ids (one `const`)
```rust
const GENERATION3D_RETAINED_TOOL_IDS: &[&str] = &["setActiveExample", "nodeGraphEdit", /* …29 total… */];
```
(`🦀️.rs:165-193` in generation3d's editor.) This single list drives: the factory's `ToolFactoryKey`
set, `ArtifactOwnedToolJobFactory::TOOL_IDS`, the `build_tool_job` gate, and the `tools: {...}` block
inside `bounded_first_step_tool_proofs!` — keep all four in sync (a mismatch fails closed at
app-construction per §1.6).

### 2.2 Write the reducer function
A plain function `fn app_retained_reduce(command, snapshot, config, history, interaction, hover, operation) -> Result<Emit<Mutation, ConfigMutation, DraftMutation>, Fault>` that dispatches on
`command` and calls the SAME handler code the ordinary `ArtifactEditor::handle` path already calls —
migration should not require rewriting business logic (generation3d's `generation3d_retained_reduce`,
`🦀️.rs:216-238`; puzzle3d's `puzzle3d_retained_reduce`, `🦀️.rs:2553-2591`, both fall through to
`command.dispatch(...)`/`app.handle_action_impl(...)` for the default case).

### 2.3 Write the factory struct
```rust
struct FooBoundedCommandJobFactory { keys: Vec<ToolFactoryKey> }
impl FooBoundedCommandJobFactory {
    fn new(controller_id: &str) -> Self { Self { keys: FOO_RETAINED_TOOL_IDS.iter().map(|id| ToolFactoryKey::new(controller_id, *id)).collect() } }
}
impl semio_framework::ToolJobFactory for FooBoundedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<FooPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<FooPlayApp>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { FOO_RETAINED_PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { foo_bounded_contract() }
    fn create_job(&mut self, _op, payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(&mut self, _op, payload, input, checkpoint) -> Result<..> { /* size-check then ArtifactRetainedCommandJob::from_wire[_with_checkpoint] */ }
}
impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FooBoundedCommandJobFactory {
    type Owner = EditorApp<FooPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FOO_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FOO_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "someArtifactEdit", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "worldPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        // …one row per tool id, lanes = exactly what that handler's Emit actually populates.
    ];
}
```
Verbatim shape from `LowpolyCommandJobFactory` (`🦀️.rs:979-1057`) and
`Generation3dBoundedCommandJobFactory` (`🦀️.rs:248-317`). **Lane choice is not a style choice** — it
is read directly off each handler's `Emit` construction (see generation3d's per-tool lane table,
reproduced in the implementation doc
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️03/PROCEDURAL-3D-END-TO-END/📓️implementation-app-owned-factory.md`)
and mismatches fault at publication time (`🦦️.rs:22905-22907`, confirmed live by puzzle3d's own
`relocateTargetVolume` finding, §4 below) — over-declaring a lane is harmless, under-declaring one that
is actually emitted is not.

**View-state actions (camera, selection, locale, toggles) route through the `Config` lane**, never
`Artifact` — this is the answer to "how does a camera/selection/locale action get migrated": it is a
completely ordinary `Migrated` action whose handler emits only `config_mutations`. Every scalar-toggle
action in lowpoly (`setCamera`, `toggleSun`, `setSunAzimuth`, `toggleShowEdges`, …,
`🦀️.rs:1057-1076`) and generation3d (`setCamera`, `setLodMode`, `toggleSun`, `setLocale`,
`selectGeneration`, `🦀️.rs:289-315`) does exactly this.

### 2.4 Wire the four `ArtifactEditor` overrides, immediately before the proofs macro
```rust
fn build_artifact_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
    Some(Arc::new(FooArtifactStorePreparationFactory))
}
fn build_config_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
    Some(Arc::new(FooConfigPreparationFactory))
}
fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
    let controller = registry.controller_id().to_string();
    registry.register(FooBoundedCommandJobFactory::new(&controller))
}
fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
    if !FOO_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) { return Ok(None); }
    // construct BoundedArtifactCommandWork::new(tool_id, foo_retained_reduce, foo_bounded_extent), wrap in
    // ArtifactRetainedCommandPayload::try_new_with_context(...), return Some(ToolOperationSpec::new(...))
}
```
`ArtifactStoreOneItemPreparationFactory<P, Mutation>` trait itself is at
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13244`; its trait-default is `None`
(`🦦️.rs:11181-11199`, i.e. `ArtifactApp`'s own defaults) — **this is the single most common real
blocker**, not the classification label itself (§4).

Generation3d's preparation factories are **generic over any `Mutation: protocol::Mutation<Snapshot>`**
via `mutation.diff()`/`.inverse()`/`MutationDiff::apply()` (`🦦️.rs:322-450`) rather than hand-matching
every mutation variant — this is the shape to copy whenever the app's `Mutation` enum already derives
a real `protocol::Mutation`/`MutationDiff` impl (true for every `#[derive(dsl::Mutations)]` type in
this repo).

### 2.5 The proofs macro invocation + classification flip + descriptor regen (in order)
```rust
semio_framework_plugin::bounded_first_step_tool_proofs! {
    owner: semio_framework_plugin::EditorApp<FooPlayApp>,
    owner_file: "✏️s/🔌️plugins/…/✏️editor/🦀️.rs",
    controller: "s.foo.foo@1/*#editor",
    document_schema: "foo.schema.id",
    factory: "FooBoundedCommandJobFactory",
    factory_type: FooBoundedCommandJobFactory,
    tools: { "someArtifactEdit" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500), /* … */ }
}
```
Then flip every migrated id's classification: `.action_interactive_job("someArtifactEdit", InteractiveJobClassification::Migrated)`.

**Descriptor regeneration is a required, separate, LAST step — not automatic:**
```
cd ✏️s/🔌️plugins/<plugin>/📦️packages/🦀️rust && bun ./📜️script.ts describe
```
which calls `describePluginComponent` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:404` — note: this is `🖨️describe`, not `📇️describe` as an earlier
ticket note misnamed it) after a fresh `wasm32-wasip2` build. **Verified drift trap, live right now**:
`✏️s/🔌️plugins/🌀️procedural/🔣️.json`'s `s.procedural.generation3d@1/*#editor` app still lists
`nodeGraphEdit`, `addGeneration`, `removeGeneration`, `renameGeneration`, `updateGenerationValues`,
`selectGeneration` as `"interactiveJob":"batchOnlyPendingRewrite"` (checked this session via
`python3 -m json.load` — 30 stale `batchOnlyPendingRewrite` rows, 6 unique ids × 5 duplicate window
mounts) even though the Rust source at
`🗿️artifacts/🧊️generation3d/…/✏️editor/🦀️.rs` has classified all 29 ids `Migrated` since the
`PROCEDURAL-3D-END-TO-END` ticket landed. **The Rust flip alone does not make an action usable if
anything downstream (registry `check`, deployed catalog, etc.) reads the committed JSON instead of a
fresh build** — always regenerate before declaring a migration complete.

### 2.6 Tests the framework testkit expects
- `semio_framework_plugin::testkit::assert_viewer_never_mutates::<V>()` — `🦦️.rs:7055-7068`. Requires
  `V::Command: Default`; dispatches the default command and asserts the document/draft store
  generation and edit-log length are unchanged. Only meaningful for `ArtifactViewer` impls.
- `assert_editor_and_viewer_share_dialect::<E, V>()` — `🦦️.rs:7074-7076`. One-line `E::DIALECT ==
  V::DIALECT` assertion — cheap, always run it for a subset with both surfaces.
- Real functional test through the job path (not string-matching): construct the app via
  `app_with_registry()` + `bind_instance_id(n)` (a bare `testkit::app()` fails closed with
  `interactive-job.catalog-authority` because its `migrated_tool_ids()` is always empty — confirmed by
  the puzzle3d recipe's own trace), dispatch through `dispatch_typed`/the real wire path, then drain
  via repeated `app.maintenance_step(...)` calls (the same call a host issues every actor tick) until
  the expected document change is observed. See puzzle3d's
  `set_active_example_dispatches_through_the_tool_job_path_and_swaps_the_document` for the pattern.
- `bounded_first_step_tool_proofs().len()` / `PUBLICATION_CONTRACTS.len()` / retained-id-list length —
  three-way equality assertion, catches drift immediately (lowpoly `🦦️.rs:2140-2141`, generation3d
  `🦦️.rs:2169-2178`).

---

## 3. Viewer-specific guidance — the dead-viewer-action premise does not hold in current source

The task brief (and `📓️explore-per-plugin-blockers.md`) states cad has 17, fem 16+16, gis 13, flow 15,
imperative 10, dag 11, reasoning 8, note 27, space home 3/space 6 dead **viewer** actions. **This
session could not reproduce any of those counts against current Rust source.** Checked by recursively
grepping every file under each app's `👁️viewer/` directory tree (not just the top-level
`👁️viewer/🦀️.rs`, which is what the per-plugin-blockers doc's own stated methodology used and which
misses actions declared in `👁️viewer/🎭️modes/*/🪟️windows/*/🦀️.rs` submodules):

| App | `BatchOnlyPendingRewrite` under `👁️viewer/**` | `Migrated` under `👁️viewer/**` |
|---|---|---|
| cad | 0 | 0 (see below — literally zero declared actions) |
| gis gismap | 0 | 0 |
| fem 3d | 0 | 0 |
| fem 2d | 0 | 0 |
| flow | 0 | 0 |
| imperative procedure | 0 | 0 |
| dag | 0 | 0 |
| reasoning-mindmap wires | 0 | 0 |
| note | 0 | 0 |
| space home | 0 | 0 |
| space space | 0 | 0 |

Reading `CadViewer` (`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16-45`) explains why: its `Command` enum has exactly one variant, `Noop`
(`:18-22`), and `handle` always returns `ViewEmit::default()` (`:68`). The window body render function
(`…/👁️viewer/🎭️modes/👁️view/🪟️windows/📐️shape/🦀️.rs:37`) declares `actions: Vec::new()`. Cross-checked
against the committed descriptor: `s.cad.cad@1/*#viewer`'s single window kind lists 13 actions total,
**all** framework-injected (undo/redo/checkpoint/alternative/clipboard/tutorial/history-filter/
noteShellCommand) and **all already `migrated`**. The same "zero own actions, `Command::Noop`,
framework rows only" shape was confirmed for gismap, dag, note, and every other app in the table above
via direct source read. This looks like the intended end-state of ticket
`26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` (referenced in a `🚧️ SDK GAP` comment in both cad's
editor and viewer files) having already landed for these apps.

Repo totals were re-measured for sanity: `grep -rc BatchOnlyPendingRewrite`/`Migrated` across all of
`✏️s/🔌️plugins` today give **461 BatchOnly / 522 Migrated** occurrences (vs. the brief's 414/427) —
consistent with ongoing work (e.g. puzzle3d alone moved from 6→59 Migrated between the two counts), so
the aggregate totals are plausible; it is specifically the **per-app viewer dead-action figures** that
this session could not confirm and believes are stale (possibly read from an old committed descriptor
snapshot predating the viewer-purification ticket, or from a different revision).

**Practical guidance for a viewer action that genuinely needs to exist** (camera orbit, "jump to
pane", a future view-only action — flagged as a real future case in `CadViewer::handle`'s own
docstring, `🦦️.rs:60-62`): it is migrated exactly like any Config-lane editor action (§2.3) — declare
it, give it `InteractiveJobClassification::Migrated`, a `bounded_first_step_tool_proofs!` row, and a
factory/proof whose `ArtifactOwnedToolJobFactory::Owner = ViewerApp<V>`. It is *structurally*
incapable of the `Artifact`/`Draft` lanes (no such field on `ViewEmit`), so its
`PUBLICATION_CONTRACTS` entry must be `&[ArtifactToolPublicationLane::Config]` (or `HostOnly` for a
pure host effect like `openDialog`) — never `Artifact`. `assert_viewer_never_mutates::<V>()` (§2.6) is
the regression test that keeps this honest.

---

## 4. Blocking framework gaps

### 4.1 `PuzzleCommandWork::step` — the "app-instance parameter" premise is STALE, corrected same week

The task brief's framing (also stated in this ticket's own `📓️explore-per-plugin-blockers.md` and in
`PUZZLE-3D-END-TO-END/📓️status.md`, dated 2026-09-03 20:33) is:

> `PuzzleCommandWork::step` (`🧩️puzzle/…/🎮️commands/🧵️retained/🦀️.rs:42-49`) receives
> command/snapshot/config/interaction/hover and never the app, so `fillBuildTick`'s Work cannot reach
> the live precompute session — needs a signature change across puzzle 2d/3d/5d.

**`PUZZLE-3D-END-TO-END/📓️plan-2026-09-05.md:15-43` (same ticket, two days later) explicitly retracts
this**, and the retraction is now implemented in source (verified this session):

> "That is wrong... **there is no live app instance in this design.** `with_puzzle3d_app_for`
> constructs a fresh `Puzzle3dPlayApp::default()` on every call and restores session state from
> `config.fill_checkpoint`... the checkpoint in `Config` IS the session... `PuzzleCommandWork::step`
> already receives `config: &A::Config`. Therefore the Work arm can call
> `with_puzzle3d_app_for(config, |app| fill_build_tick_cached(app, config))`... **No trait change. No
> 2d/5d churn.**"

Confirmed in current source: `puzzle3d_retained_reduce`
(`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2578-2582`)
already has `if command.action_id() == "fillBuildTick" { if let Some(emit) =
fill_build_tick::fill_build_tick_cached(app, config) { return emit; } }` inside a
`with_puzzle3d_app_for(config, |app| ...)` closure, and a passing unit test
(`fill_build_tick_work_spawns_the_isolated_planner_and_persists_the_checkpoint`, `:8117-8134`) drives
`Puzzle3dPrecomputeCommandWork::step` directly and asserts a real `Effect::SpawnJob` plus one
persisted config mutation come out — proving the fix works without any `PuzzleCommandWork` signature
change. **`PuzzleCommandWork::step` does not need an app-instance parameter, and no other plugin needs
one either** — the puzzle-family's whole design deliberately keeps `ArtifactApp` methods as
`&self`-less associated functions with all session state living in `Config`, so this pattern
generalizes.

**What is still actually incomplete for puzzle3d** (current source, 59 Migrated / 8 BatchOnly,
verified by `grep -c` this session — not the brief's stale "10/69"):
- `fillBuildTick`, `suggestionsTick`, `registerBrushMesh`: real completions now exist and are
  unit-tested, but the ids are **not yet** added to `PUZZLE3D_RETAINED_TOOL_IDS` /
  `PUBLICATION_CONTRACTS` / the proofs macro / flipped to `Migrated` — purely mechanical, per §2, no
  further design work.
- `transformBegin`, `transformEnd`: still route through `NoopPuzzleCommandWork`
  (`🦦️.rs:6811`) — genuinely still stubs, need real reducer work first.
- `engagementRepeatLast`: has a dedicated `Puzzle3dEngagementRepeatWork` (`🦦️.rs:6772`) — not
  independently verified this session whether it's real or still a stub.
- `setFillCountStep`: **would silently no-op if migrated mechanically** — no arm in `build_tool_job`,
  falls to the generic reducer whose default arm is `_ => {}` (`PUZZLE-3D-END-TO-END/📓️findings-2026-09-05.md` §3). Needs an explicit arm before flipping.
- `setFixtureJson`: payload can be up to 128,755 bytes (Nakagin fixture) against the shared
  `PUZZLE_COMMAND_RAW_BYTES = 8_192` wire cap (`🎮️commands/🧵️retained/🦀️.rs:10`) — a structural
  transport-size blocker, not a classification/wiring problem; needs a chunked/resumable wire path
  before it can honestly migrate (`📓️findings-2026-09-05.md` §5).

### 4.2 Do other plugins share a `PuzzleCommandWork`-shaped limitation?
No plugin outside the puzzle family (2d/3d/5d, which literally share
`🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs`) uses `PuzzleCommandWork` at all — lowpoly and generation3d use
the framework's own `BoundedArtifactCommandWork<A>`
(`🧰️framework/…/🔌️plugin/🧵️retained-command/🦀️.rs:124-159`), whose `step` signature **already**
includes `operation: &AppOperationContext` (`:144`, carrying `app_instance_id`) — i.e. the framework's
generic retained-work trait was never missing this; only puzzle's own bespoke
`PuzzleCommandWork<A>::step` (`🎮️commands/🧵️retained/🦀️.rs:37-48`) omits it, and per §4.1 that
omission turned out not to matter once the checkpoint-in-Config design was understood correctly.

### 4.3 The 15 norm apps — one shared shape, confirmed structurally identical

Enumerated all 15 (`din4108, en1990, en1991, en1992, en1993, en1994, en1995, en1996, en1997, en1999,
en1998, din16798, din18599, iso16757, vdi3805`) under
`✏️s/🔌️plugins/📕️norm/🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`. **Every single
one** declares the exact same 3-action set, in the exact same order, all `BatchOnlyPendingRewrite`,
confirmed by scripted grep across all 15 files this session:

| action id | kind | lane it would need | handler behavior (identical shape across all 15, e.g. din4108
`…/✏️editor/🎮️commands/{📤️set-snapshot,🧮️evaluate,☑️selected-check}/🦀️.rs`) |
|---|---|---|---|
| `setSnapshot` | Mutation | `Artifact` | `crate::app_surface::commit_snapshot_fields(FooMutation::from_snapshot(doc.snapshot, &payload.snapshot), "setSnapshot")` |
| `evaluate` | View | none (`Emit::default()`) | compliance report is derived on every read, never persisted — genuinely a no-op mutation, `HostOnly` lane |
| `setSelectedCheckIndex` | View | `Config` | `crate::app_surface::commit_selected_check_index::<FooMutation>(payload.index)` → `NormConfigMutation::ChangeSelectedCheckIndex` |

No differences found across the 15 beyond the per-app `Snapshot`/`Mutation` type names. The plugin's
own `🖥️app-surface/🦀️.rs:1-12` doc comment says this explicitly: "the fifteen norm apps are
structurally identical by construction... and differ only in their per-standard `Document` type, ids
and labels... every entry point is either a plain constructor or generic over the artifact's
`Document`/`NormFamily`." All 15 share one `NormConfig`/`NormConfigMutation` (from `crate::config`) and
one `app_surface` module already generic over `NormFamily`.

**This means one shared, generic `NormBoundedCommandJobFactory<Family: NormFamily>`-style factory
(generic over Snapshot/Mutation the same way generation3d's preparation factories are, §2.4) can
realistically serve all 15** — write it once against a `NormFamily`-parameterized owner type (or one
factory type instantiated per app, sharing 100% of its body via a generic), then apply the §2
mechanical steps per app. Given `evaluate` truly emits nothing, its `PUBLICATION_CONTRACTS` lane is
`HostOnly`; `setSnapshot` is `Artifact`; `setSelectedCheckIndex` is `Config`. No app-specific business
logic needs to change — same "wiring, not rewriting" situation as writer (§ recipe precedent).

---

## 5. Dependency-ordered dispatch table

Ordered so each row's prerequisite is satisfied by an earlier row wherever possible.

| Order | App family | Actions to migrate | Prerequisite | Files to touch |
|---|---|---|---|---|
| 1 | procedural generation3d | 0 remaining (source-complete) | **Descriptor regen only** | `cd ✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust && bun ./📜️script.ts describe`; diff `✏️s/🔌️plugins/🌀️procedural/🔣️.json` against `git show HEAD:'✏️s/🔌️plugins/🌀️procedural/🔣️.json'` for the 6 stale `nodeGraphEdit`/`addGeneration`/`removeGeneration`/`renameGeneration`/`updateGenerationValues`/`selectGeneration` rows |
| 2 | norm (15 apps) | 3 × 15 = 45 actions, all `BatchOnlyPendingRewrite` today | none — self-contained, no framework change, one generic factory design | one new generic factory (design once, e.g. in `📕️norm/🖥️app-surface/🦀️.rs` or a new `🖥️app-surface/🧵️retained/🦀️.rs`), then per-app: `.action_interactive_job` flips + `bounded_first_step_tool_proofs!` block in each `🗿️artifacts/<app>/…/✏️editor/🦀️.rs` (15 files) |
| 3 | puzzle3d mechanical wave | `fillBuildTick`, `suggestionsTick`, `registerBrushMesh` (3) | none — real completions already exist and are unit-tested (§4.1) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`: `PUZZLE3D_RETAINED_TOOL_IDS` (~:2530 per plan doc), `PUBLICATION_CONTRACTS`, `bounded_first_step_tool_proofs!`, 3 classification flips (currently lines 7366/7372/7407) |
| 4 | puzzle3d semantic wave | `transformBegin`, `transformEnd`, `engagementRepeatLast`, `setFillCountStep` (4) | real reducer/Work logic per action (transformBegin/End still Noop; setFillCountStep needs a dedicated `build_tool_job` arm) | same file, `Work` `step` bodies (~lines 6772-6811 region) |
| 5 | puzzle3d transport-blocked | `setFixtureJson` (1) | **framework change**: chunked/resumable wire path, or a per-tool wire-byte override above the shared `PUZZLE_COMMAND_RAW_BYTES = 8_192` | `🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:10` (shared const) plus the dispatch-side wire admission path (`🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs:643`, `RawWireLimit`) |
| 6 | forms | 28 tools (`FormsBoundedCommandJobFactory`, landed but not compile-verified per this repo's other ticket) | **verify only** — a green `cargo check`/`test` for `semio-s-plugin-forms`, then descriptor regen | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️03/GIVE-FORMS-APP-AN-OWNED-TOOL-JOB-FACTORY/` for exact anchors |
| 7 | sourcing curation | remaining actions per its own ticket | **stdio compiling** (2196 `ToValue`/`FromValue` derive errors) + grid-window >32 KiB payload fix | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END/📓️status.md` |
| 8 | puzzle2d / puzzle5d | 34 / 41 remaining (per brief's snapshot — re-verify counts before starting, given how fast puzzle3d moved) | same puzzle-family recipe as puzzle3d §4.1; **do not re-derive an app-instance-param "blocker"** — it doesn't exist | `🗿️artifacts/◻️2d/…/✏️editor/🦀️.rs`, `🗿️artifacts/🖐️5d/…/✏️editor/🦀️.rs` |
| 9 | cad / gis / fem / flow / imperative / dag / reasoning / note / space viewers | **0 confirmed** — §3 found no dead viewer actions in current source for any of these | re-run `explore-per-plugin-blockers.md`'s viewer count methodology against current source (recursive, not top-level-file-only) before scheduling any work here | n/a until counts are reconfirmed |
| 10 | remaining plugins with 0 actions or non-compiling crates (block, playbook, trinity, animate, remodel, architect, energy, raster) | varies | plugin must compile first (see `📓️explore-per-plugin-blockers.md` §1 "Concrete blocker" column, mostly `#[path]`/mutation-module E0433/E0599 errors from the `✳️base`→`🧱️base` rename in flight) | per-plugin, see that table |

---

## Files referenced (all read, zero edits made)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5197-5219, 12041-12047, 12177-12243, 12667-12760, 12820-12889, 19160-19171, 20978-20980, 22363-22415, 27044-27160, 27419-27436`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:124-260`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:17-18, 404`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:979-1057, 1607-1633, 2012-2058, 2140-2141`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:165-317, 685-767, 2169-2180`
- `✏️s/🔌️plugins/🌀️procedural/🔣️.json` (30 stale `batchOnlyPendingRewrite` rows, verified via `python3 -m json.load` this session)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2553-2591, 6772-6811, 7364-7410, 8117-8134` (59 Migrated / 8 BatchOnly, verified this session)
- `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:1-90`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:16-84` and its `🎭️modes/👁️view/🪟️windows/📐️shape/🦀️.rs:37`
- `✏️s/🔌️plugins/📐️cad/🔣️.json` (`s.cad.cad@1/*#viewer`, 13 framework-injected actions, all `migrated`)
- `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs:1-50`; all 15 apps' `…/✏️editor/🦀️.rs` and `…/🎮️commands/{evaluate,selected-check,set-snapshot}/🦀️.rs` (din4108 read in full as the representative)
- `.🧬semio/🦑️репо/…` ticket docs: `PROCEDURAL-3D-END-TO-END/📓️implementation-app-owned-factory.md`;
  `PUZZLE-3D-END-TO-END/{📓️interactive-job-migration-recipe.md, 📓️findings-2026-09-05.md, 📓️plan-2026-09-05.md, 📓️status.md, 📓️wave-E-report.md}`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/S-END-TO-END/📓️explore-per-plugin-blockers.md` (this ticket's own prior audit — cross-checked, and corrected in §3/§4.1 above)

## Corrections to the task brief's premises, summarized

1. **`PuzzleCommandWork::step` does not need an app-instance parameter.** A 2026-09-03 analysis said
   it did; a 2026-09-05 analysis in the same ticket retracted that and implemented the real fix
   (config-embedded checkpoint, no trait change) — confirmed present in current source and covered by
   a passing unit test. See §4.1.
2. **The viewer dead-action counts (cad 17, fem 16+16, gis 13, flow 15, imperative 10, dag 11,
   reasoning 8, note 27, space 3/6) could not be reproduced against current source** — every sampled
   viewer directory has zero `BatchOnlyPendingRewrite` actions anywhere in its tree; several (cad, gis,
   dag, note, space) declare literally zero authored actions at all. See §3.
3. **puzzle3d is at 59 Migrated / 8 BatchOnly today**, not the brief's cited "10/69" — a fast-moving
   target; re-verify counts immediately before starting any puzzle-family work.
