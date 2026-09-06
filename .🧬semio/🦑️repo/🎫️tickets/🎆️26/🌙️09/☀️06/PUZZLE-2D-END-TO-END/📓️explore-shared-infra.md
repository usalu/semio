# Explore: Shared Infra — puzzle2d vs 3d/5d, and the framework plugin host

Read-only exploration. All paths below are relative to repo root `/Users/ueli/Documents/semio` unless absolute. This report is being assembled from firsthand reads plus a fleet of parallel read-only research agents; sections are filled in as evidence lands.

---

## 1. Crate-root module map — 2d/3d/5d mounting, and `🎮️commands/🧵️retained`

### Crate root wiring

`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/🦀️.rs` (2,998 lines) is WIRING ONLY — every `mod` is a `#[path]` pointing at exactly one taxonomy component file (Shape V2, ticket `26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST`). Grouping modules use `#[path = "."]` so their name isn't spliced into the base directory.

- Line 30: `pub mod retained_command;` → `#[path = "../../🎮️commands/🧵️retained/🦀️.rs"]` — the shared retained-command session (see below), mounted once at crate root, generic over all three artifacts.
- Line 35: `pub mod artifacts { ... }` region (`//#region 🗿️Artifacts`) mounts, as siblings:
  - `pub mod puzzle2d` (line 37) → `../../🗿️artifacts/◻️2d/🦀️.rs`, with nested `standards::v1::subsets::any::{schema, ...}` all under `◻️2d/🏅️standards/🔖️1/…`.
  - `pub mod puzzle5d` (line 712) → `../../🗿️artifacts/🖐️5d/🦀️.rs`.
  - `pub mod puzzle3d` (line 1395) → `../../🗿️artifacts/🧊️3d/🦀️.rs`.
- `pub mod editor { ... }` region (line 2210, `//#region ✏️Editor`) mounts, as siblings, each artifact's editor tree:
  - `pub mod puzzle2d` (line 2212) → editor component `../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, plus nested `config` (+ `config::schema`), `presence` (+ `presence::schema`), `terminology`, `wasm` (all mounted the same shape as 3d/5d), and an **`engine`** submodule (line 2242) unique to 2d in name: `board_host`, `brush`, `icons`, `layout`, `linking` — i.e. 2d's board-rendering/layout machinery lives under `⚙️engine/`.
  - `pub mod puzzle3d` (line 2391) → same shape (`config`, `presence`, `terminology`, `wasm`), but instead of `engine` it has a **`precompute`** submodule (`⏳️precompute/{brush,fill,geometry}`) — 3d's GPU/mesh precompute pipeline, structurally analogous in position but functionally different in content from 2d's `engine`.
  - `pub mod puzzle5d` (line 2615) → same shape (`config`, `presence`, `terminology`, `wasm`).
- `pub mod viewer { ... }` region (line ~2839) mounts `puzzle3d`/`puzzle5d` viewer trees (`👁️viewer/…`, with `modes::view::windows::{main, world3d}`) — **puzzle2d has no `viewer` submodule** at crate root; 2d's window/rendering entry point lives entirely under `editor::puzzle2d::engine::board_host`, not a separate `viewer` tree.
- `//#region 🔖️Plugin` (line ~2921): `mod plugin;` → `#[path = "../../🦀️.rs"]` (the plugin root one level up), `pub use plugin::PuzzleApps;`, and `semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::PuzzleApps)` gated on `feature = "plugin-entry"` — this is where the plugin's WASM component entry point is registered, one shared component for 2d+3d+5d.
- `//#region 📚️Examples` (line ~2932): `pub mod examples { pub mod puzzle2d { concrete_forest, nakagin_capsule_tower } pub mod puzzle3d { concrete_forest, nakagin_capsule_tower } pub mod puzzle5d { capsule_dream, concrete_forest, nakagin_capsule_tower } }` — **puzzle2d does have its own native `nakagin-capsule-tower` example**, not a projection/import of the 3d one (see §3).

### `🎮️commands/🧵️retained/🦀️.rs` (1,029 lines) — fully generic, not 3d-specific

Everything in this file is generic over `A: ArtifactApp` (a type parameter), and the file's own doc comment (line 1) reads: *"Fixed-capacity retained command session shared by Puzzle 2d, 3d, and 5d."* There is **no 3d-specific code in this file** — all 3d-specific constants (e.g. `PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT`) live in the 3d editor file itself, not here.

- **Limits** (`//#region 🔖️Limits`, lines 9-20), all shared constants:
  ```rust
  pub const PUZZLE_COMMAND_RAW_BYTES: usize = 8_192;
  pub const PUZZLE_COMMAND_DECODED_ITEMS: usize = 512;
  pub const PUZZLE_COMMAND_WORK_ITEMS: usize = 4_096;
  pub const PUZZLE_COMMAND_OUTPUT_BYTES: usize = 262_144;
  pub const PUZZLE_COMMAND_STEP_MICROS: u32 = 7_500;
  pub const PUZZLE_COMMAND_CHECKPOINT_BYTES: usize = 112;
  ```
  (lines 10-15) plus `puzzle_command_contract()` (line 17) building a `ToolExecutionContract::resumable(...)` from them.

- **`PuzzleCommandWork<A: ArtifactApp>` trait** (line 38): `tool_id() -> &'static str`, `bind_operation(&mut self, Operation)` (default no-op), `extent(&self, command, snapshot, interaction) -> Option<usize>` (the per-action budget function every artifact-specific `Work` struct implements), `step(...) -> Result<PuzzleCommandWorkStep<A>, Fault>`, `begin_close`, `close_step` (default `Complete`), `terminal_is_empty` (default `true`). This is the trait 2d's `Puzzle2dActiveExampleWork`/`Puzzle2dForceLayoutWork` and 3d's per-tool work structs all implement — fully generic, artifact-specific logic lives in the impls, not the trait.

- **`BoundedFirstStepCommandWork<A>`** (line 59): a generic single-step work wrapper taking a `PuzzleCommandReducer<A>` fn pointer and a `PuzzleCommandExtent<A>` fn pointer (type aliases at lines 23/31) — used by both 2d (`addNode`, `applyBoardEvents`, line 2038-2039 of the 2d editor) and 3d for tools whose extent/reduce logic is a plain function rather than a stateful multi-step struct.

- **`NoopPuzzleCommandWork<A>`** (line 98) and **`RetainedPuzzleCommandJob<A>`** (line 287, the actual `InteractiveJob` impl driving the wire→decode→preflight→work→publish state machine, lines 502-598) are likewise fully generic; the `PuzzleCommandPhase` enum (line 148) and its checkpoint codec (`PuzzleCommandCheckpointState`, line 161, `MAGIC = b"PZCP"`, `VERSION = 1`) are shared wire/resume format for all three artifacts.

- **Test-time cross-artifact proof** (`#[cfg(test)] mod tests`, line 753): the fixture-oracle test `every_puzzle_factory_validates_and_adopts_the_exact_checkpoint_owner` (line 859) asserts, by `include_str!`-ing the *source* of all three artifacts' editor files, that puzzle3d and puzzle5d both call `RetainedPuzzleCommandJob::validate_wire_checkpoint(...)` / `::from_validated_wire_checkpoint(...)`, while **puzzle2d explicitly does NOT** implement a `BoundedFirstStepCommandJobFactory`-style `ArtifactOwnedToolJobFactory` the same way (lines 868-870):
  ```rust
  let puzzle2d = include_str!("../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
  assert!(!puzzle2d.contains("impl semio_framework_plugin::ArtifactOwnedToolJobFactory for BoundedFirstStepCommandJobFactory"));
  assert!(!puzzle2d.contains("registry.register(BoundedFirstStepCommandJobFactory"));
  ```
  i.e. 2d's retained factory (`Puzzle2dRetainedCommandJobFactory`) is its own named type, not the shared generic `BoundedFirstStepCommandJobFactory` type 3d/5d use — a structural divergence worth flagging (2d rolled its own factory wrapper name even though the underlying `RetainedPuzzleCommandJob<A>`/`PuzzleCommandWork<A>` machinery is shared).

- **Retained tool-id gates found per artifact** (grep, not this file):
  - `PUZZLE2D_RETAINED_TOOL_IDS` — `◻️2d/…/✏️editor/🦀️.rs:1072`: `&["setActiveExample", "forceLayout", "addNode", "applyBoardEvents"]` (4 ids — matches ticket status.md's "gate `PUZZLE2D_RETAINED_TOOL_IDS` = 4 ids").
  - `PUZZLE3D_RETAINED_TOOL_IDS` — `🧊️3d/…/✏️editor/🦀️.rs:2530` (longer list, 3d has many more `Migrated` tool ids already).
  - Both are asserted against the same `retained-jobs/🔣️.json` fixture oracle format in `🧵️retained/🦀️.rs:996-1010` (`expected("puzzle2d", "puzzle.2d.fixture", PUZZLE2D_RETAINED_TOOL_IDS)` etc.) — i.e. the fixture-oracle test infrastructure itself is fully shared/generic across all three artifacts, just parameterized by owner name, document schema, and tool-id slice per artifact.

---

## 2. `ArtifactEditor` / `ArtifactApp` / `EditorApp<E>` contract — Puzzle2dPlayApp vs Puzzle3dPlayApp

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (39,371 lines — referred to as FRAMEWORK below).

### `ArtifactApp` trait (FRAMEWORK:11120-~11760)

```rust
pub trait ArtifactApp: Default + Send + 'static {
    const DIALECT: Dialect;
    const APP_ID: &'static str;
    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = false;
    const DOCUMENT_SCHEMA: &'static str;
    const ROLE: AppRole = AppRole::Editor;
    type Snapshot: Clone + PartialEq + protocol::ToValue + protocol::FromValue + Send + Sync + store::ArtifactDsl + ArtifactPack + 'static;
    type Mutation: ::protocol::Mutation<Self::Snapshot> + PartialEq + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Config: Clone + Default + PartialEq + protocol::ToValue + protocol::FromValue + Send + Sync + store::ConfigRecord + ArtifactPack + 'static;
    type ConfigMutation: ...; type Draft: ...; type DraftMutation: ...;
    type Presence: ...; type PresenceMutation: ...; type Transient: ...; type TransientMutation: ...;
    type Command: ::protocol::OpBinary + Send + Sync + 'static;
    ...
}
```
line 11120. The **one method with a receiver** (`&self`) in the whole trait is `async fn instance_id(&self) -> &str` (~11128-11135) — doc comment: *"the real, runtime-resolved app id an ownership check must compare against… `EditorApp<E>`/`ViewerApp<V>` override it to return their derived `surface_app_id`"* — this is where "the session"/runtime instance identity is resolved, distinct from the compile-time `APP_ID` const. There is **no separate "Session" associated type** — the closest analogs are `Self::Config` (persisted config store) plus `Self::Presence`/`Self::Transient` (ephemeral shared/local state) and the `ConfigView`/`ArtifactView` parameters `handle` receives.

Everything else is an **associated function without `&self`**:
- `async fn initial_snapshot() -> Self::Snapshot;` — required, no default.
- `async fn handle(command: &Self::Command, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, interaction: &InteractionView<'_>, draft: &DraftView<'_, Self::Draft>, engines: &EngineHandles) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault>;` — **line 11368**, required, no default. Doc: *"the pure heart of the app — a total, side-effect-free function from (command, document, config, draft, engines) to an Emit. No `&mut self`."*
- `async fn build_tool_job(_request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault>` — default `Ok(None)` (~11175).
- `fn build_reserved_tool_job(_request: ArtifactReservedToolJobRequest<Self>) -> Result<Option<ArtifactReservedToolJob>, Fault>` — default `Ok(None)` (~11179).
- `fn register_tool_job_factories(_registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault>` — default `Ok(())` (~11164).
- `fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>>` — default `None` (~11202).
- `build_artifact_store_one_item_preparation_factory` / `build_config_store_one_item_preparation_factory` / `build_draft_store_one_item_preparation_factory` / `build_presence_store_one_item_preparation_factory` / `build_transient_store_one_item_preparation_factory` — all `Option<Arc<dyn …PreparationFactory<...>>>`, all default `None` (~11205-11235).
- **`routes` does not exist anywhere in `ArtifactApp` or `ArtifactEditor`** (confirmed by grepping `\broutes\b`, 59 hits, none in this contract — it is not part of the contract the ticket brief assumed it was).
- **`close_step` is not a method of `ArtifactApp`/`ArtifactEditor`** — it belongs to `InteractiveJob::close_step` (job crate, see below), `ArtifactInstanceOperationOwner::close_step` (FRAMEWORK:12798), and dozens of per-job/store-disposer impls — job/close-lifecycle mechanics, not the editor-authoring contract. In puzzle specifically, `close_step` lives on the per-command `Work` structs via `PuzzleCommandWork::close_step` (§1 above), never on the `ArtifactEditor` impl itself.

### `ArtifactEditor` trait (FRAMEWORK:26618-27038)

Doc comment (26612-26617): *"Authoring trait for a mutation-capable surface — a structural copy of `ArtifactApp`'s members plus `ROLE`/`DIALECT`; `APP_ID` is intentionally absent, the runtime id being derived from `DIALECT`+`ROLE` via `surface_app_id` instead of hand-written. `EditorApp<E>` below is the sole `ArtifactApp` implementor for any `E: ArtifactEditor`."*

Structural differences from `ArtifactApp`: no `APP_ID` const, no `instance_id` method (that's `EditorApp<E>`-only, since `E` is a stateless/ZST author type), and almost every method is **synchronous** (`fn`, not `async fn`) — e.g. `fn handle(...)` at **line 26818** vs `ArtifactApp::handle`'s `async fn` at 11368 (exceptions still `async fn`: `command_from_intent`, `media_ports`). `fn ephemeral(...) -> (Vec<Self::PresenceMutation>, Vec<Self::TransientMutation>)` (~26791) returns a **plain tuple**, not `EphemeralEmit<Self>` — doc comment: *"`EphemeralEmit` is bound to the RUNTIME `ArtifactApp` trait, which `Self` here does not implement; `EditorApp<E>::ephemeral` wraps this tuple into a real `EphemeralEmit` at the adapter boundary."* `register_tool_job_factories`/`build_tool_job`/`build_reserved_tool_job` are parameterized by `EditorApp<Self>` rather than `Self` for the same reason. `build_document_store_owners` and the `build_*_store_one_item_preparation_factory` set mirror `ArtifactApp`'s exactly (all default `None`).

### `EditorApp<E: ArtifactEditor>` (FRAMEWORK:27205-27459) — confirmed "sole `ArtifactApp` implementor"

```rust
pub struct EditorApp<E: ArtifactEditor> { id: String, _editor: std::marker::PhantomData<E> }
impl<E: ArtifactEditor> Default for EditorApp<E> {
    fn default() -> Self { Self { id: surface_app_id(&E::DIALECT.into(), E::ROLE), _editor: std::marker::PhantomData } }
}
impl<E: ArtifactEditor> ArtifactApp for EditorApp<E> {
    const APP_ID: &'static str = "surface";  // fixed placeholder
    async fn instance_id(&self) -> &str { self.surface_id().await }  // real runtime id
    async fn handle(...) -> Result<...> { E::handle(command, doc, cfg, interaction, draft, engines) }  // 27367-27375
    // every other member is pure `E::method(...)` forwarding
}
```
Pure 1:1 adapter; the only non-trivial logic is `id` derivation and the `instance_id`/`surface_id` override. `ViewerApp<V: ArtifactViewer>` (~27462) is the read-only sibling, structurally identical but forces `Draft = NoDraft` unconditionally (no viewer ever originates content).

### `ArtifactOwnedToolJobFactory` (FRAMEWORK:12779-12791)

```rust
pub trait ArtifactOwnedToolJobFactory: semio_framework::ToolJobFactory {
    type Owner: ArtifactApp;
    const TOOL_IDS: &'static [&'static str];
    const DOCUMENT_SCHEMA: &'static str;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[];
    fn latest_wins_target(_command: &<Self::Owner as ArtifactApp>::Command) -> Option<&str> { None }
    fn build_latest_wins_command_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<<Self::Owner as ArtifactApp>::Command>>> { None }
}
```
Doc (12777-12778): *"A plugin factory's concrete owner is part of its Rust type, so a copied `ArtifactApp` cannot reuse the factory by copying controller/schema/tool constants."* — `Owner: ArtifactApp` binds each factory to exactly one concrete app type (always `EditorApp<SomePlayApp>` in practice).

### `bounded_first_step_tool_proofs!` macro (FRAMEWORK:12704-12756)

`#[macro_export] macro_rules!` with a list form (`tools: [tool1, tool2, ...]`, one shared contract) and a map form (`tools: { tool => contract, ... }`, per-tool contracts). Both expand into a `bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof>` that declares — in the owning plugin file itself — exactly which tool ids a given `owner` (`ArtifactApp`) may run through a specific bounded-first-step `factory` under a given `controller`/`document_schema`, each with a publication contract. An optional `.with_factory_type::<owner, factory_type>()` call gates every action (even framework built-ins) behind a declared factory type; per a standing memory note in this repo, omitting it leaves every tool call dead with `interactive-job.missing-owned-reducer`. `ArtifactApp`/`ArtifactEditor`'s own `bounded_first_step_tool_proofs()` default to `Vec::new()` (fail-closed).

### `InteractiveJob` — lives in the job crate, not this framework file

`🧰️framework/🔨️modules/🧵️job/🦀️.rs:1349`:
```rust
pub trait InteractiveJob: Send {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome;
    fn begin_close(&mut self);
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep;
    fn register_close_wake(&self, _waker: &Waker) -> bool { false }
    fn terminal_is_empty(&self) -> bool;
}
```
(`InteractiveJobCloseStep`: `Pending{released_items, released_bytes} | Blocked | Complete`, lines 1338-1342.) The plugin-host file only references it, e.g. `pub trait ArtifactReservedJob: semio_framework_job::InteractiveJob` (FRAMEWORK:14210).

### `PUZZLE_COMMAND_RAW_BYTES`/`PUZZLE_COMMAND_WORK_ITEMS` — confirmed NOT in the framework file (cross-checks §1)

They live in `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:10-15` as documented in §1 — the framework file has no knowledge of puzzle-specific capacity constants.

### `Effect::SpawnJob` — lives in the kernel module, not this framework file

`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:359` (`pub enum Effect { ... }`), `SpawnJob` variant at **line 625**:
```rust
SpawnJob { job: u64, kind: String, input: Vec<u8>, placement: JobPlacement },
CancelJob { job: u64 },
```
Used in actor/kernel/reactor/host/renderer modules, and in puzzle **only by 3D** (editor root + `🎮️commands/🪣️fill-build-tick/🦀️.rs`) and in `fem`/`energy`/`remodel` plugins — **absent from the 2D editor tree entirely**.

### Publication-lane gate at FRAMEWORK:22904-22948

After a tool job's completion (`ArtifactToolCompletionValue`) resolves, this gate cross-checks which store lanes the result actually *touched* (artifact/config/draft/presence/transient/child mutations) against `mounted.publication_lanes` — the declared `ArtifactToolPublicationLane` set from that factory's `PUBLICATION_CONTRACTS` for the tool id that ran:
```rust
ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral) => {
    if (!emit.artifact_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Artifact))
        || (!emit.config_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Config))
        || (!emit.draft_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Draft))
        || (!ephemeral.presence.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Presence))
        || (!ephemeral.transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Transient))
        || (!emit.child_emits.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Child))
    { return Err(plugin_sdk_fault("typed-operation emitted a store lane absent from its exact factory publication contract")); }
```
If a job writes to any lane it didn't declare, the whole completion is hard-rejected rather than silently applied. This is exactly the mechanism puzzle2d's `PUBLICATION_CONTRACTS` (editor `🦀️.rs:1135-1139`, see §1/§3) exist to satisfy — and the mechanism behind the code/audit mismatch flagged at the end of §3.

### Puzzle2dPlayApp vs Puzzle3dPlayApp — method-by-method

2D editor file: 2,980 lines; 3D editor file: 10,440 lines (~3.5x larger). `impl ArtifactEditor for Puzzle2dPlayApp` begins at **2D:1847**; `impl ArtifactEditor for Puzzle3dPlayApp` begins at **3D:6701**.

| Trait method | Puzzle2dPlayApp | Puzzle3dPlayApp | Verdict |
|---|---|---|---|
| associated types | 2D:1848-1860, `Draft=NoDraft`, `Transient=NoTransient` | 3D:6702-6714, same pattern | identical shape |
| `build_document_store_owners` | **not overridden** → `None` | **3D:6716-6718**: `Some(bounded_document_store_owners::<Snapshot, Mutation>())` | **DIVERGE** |
| `build_config_store_owners` | not overridden → `None` | **3D:6720-6722**: `Some(bounded_config_store_owners::<Config, ConfigMutation>())` | **DIVERGE** |
| `build_config_store_one_item_preparation_factory` | not overridden → `None` | **3D:6724-6726**: `Some(Arc::new(Puzzle3dConfigStorePreparationFactory))` (type defined 3D:6478) | **DIVERGE** |
| `build_artifact_store_one_item_preparation_factory` | not overridden → `None` | **3D:6728-6730**: `Some(Arc::new(Puzzle3dArtifactStorePreparationFactory))` (type defined 3D:6576) | **DIVERGE** |
| `build_document_store_disposer` / `build_config_store_disposer` | not overridden | **3D:6732-6736**: both `Some(bounded_*_store_disposer::<...>())` | **DIVERGE** |
| `app_schema` | **2D:1867-1869** | **3D:~6839-6841** | mirrored |
| `initial_snapshot` | **2D:1871-1874**: `set_active_example::warm_examples(); Puzzle2dPlaySnapshot(...)` | **3D:~6843** | same shape, different builders |
| `command_id` / `command_from_action` | **2D:1877-1885** | **3D:~6856-6862** | identical pattern |
| `mounted_job_prepare_snapshot_read` | **2D:1887-1889**: delegates to `set_fill_count::prepare_snapshot_read` | **not overridden** (3D handles fill inline in `handle` via `precompute.restore_persisted_fill`) | **DIVERGE** |
| `pending_effects` | **2D:1891-1893**: delegates to `set_fill_count::reconcile_snapshot_read` | not overridden → `Vec::new()` | **DIVERGE** |
| `handle` | **2D:1897-1975+**, dispatches via a local `match action { ... }` + free functions in `🎮️commands/*` | **3D:6870-6895**, delegates through a cached wrapper (`with_puzzle3d_app_for(...).handle_action_impl(...)`) with special-cased `fillBuildTick`/`setFillCount` fast paths | **DIVERGE in architecture, identical signature shape** — see verbatim below |
| `register_tool_job_factories` | **2D:2023-2026**: registers ONE `Puzzle2dRetainedCommandJobFactory` | **3D:6755-6758**: registers ONE `Puzzle3dRetainedCommandJobFactory` | identical pattern |
| `build_tool_job` | **2D:2028-2050**: `match action_id { "setActiveExample"/"forceLayout" => dedicated Work, "addNode"/"applyBoardEvents" => BoundedFirstStepCommandWork, _ => Err }` — 4 tool ids, 2 dedicated Work structs | **3D:6759-6825**: same shape over **~60+ tool ids**, ~15 dedicated `Puzzle3dXxxWork` structs, grouped scalar/config branches, a `Noop` branch, catch-all fallback | same mechanism, vastly larger 3D surface |
| `interaction_topology` | not overridden (empty default) | **3D:~6905+**: real object/vortex/target-volume forest topology | **DIVERGE** |
| `io`/`import_media`, `render`/`window_engagements`/`window_measures`/`tool_measures`/`context_menu` | all overridden, 2D:2054-2153-ish | all overridden, 3D:~6936-~7078 | mirrored pattern both sides |
| `Effect::SpawnJob` usage | absent | present (editor root + `🪣️fill-build-tick`) | **DIVERGE** |

**`handle` verbatim signatures** — identical shape, divergent body:
```rust
// 2D — ◻️2d/…/✏️editor/🦀️.rs:1897-1903
fn handle(
    command: &Puzzle2dCommand, doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>,
    interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles,
) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation, Self::DraftMutation>, Fault> {

// 3D — 🧊️3d/…/✏️editor/🦀️.rs:6870-6876
fn handle(
    command: &Puzzle3dCommand, doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>,
    interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles,
) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation, Self::DraftMutation>, Fault> {
```
2D's doc comment explains the pattern both share: *"ArtifactApp::handle is pure (no `&self`) — rebuild a fresh BoardHost from the document each call."* 2D builds a fresh `RefCell<BoardHost>` + `Puzzle2dActionCtx` and dispatches inline; 3D wraps a heavier cached `app` object (with its own `precompute` field) and delegates to `handle_action_impl` — a materially different architecture driven by 3D's much larger interactive (fill/vortex/suggestion precompute) surface.

### Shared base vs duplicated code

**Genuinely shared** (not reinvented per artifact): the `PuzzleCommandWork<A: ArtifactApp>` trait and `BoundedFirstStepCommandWork<A>` (§1); the capacity constants and `puzzle_command_contract()`; the `ToolJobFactory` + `ArtifactOwnedToolJobFactory` shape each dimension's `Puzzle{2,3}dRetainedCommandJobFactory` implements identically (`type Owner = EditorApp<Puzzle{2,3}dPlayApp>`, `TOOL_IDS`, `DOCUMENT_SCHEMA`, `PUBLICATION_CONTRACTS`, oversized-wire rejection against `PUZZLE_COMMAND_RAW_BYTES`).

**Duplicated per artifact** (not templated): the entire `handle` dispatch body, the `Puzzle{2,3}dActionCtx` context struct, and every `🎮️commands/*` action module.

**A real functional gap flagged by this comparison**: 2D has **no** `build_document_store_owners`/`build_config_store_owners`/preparation-factory overrides at all — it relies entirely on `ArtifactEditor`'s fail-closed `None` defaults, whereas 3D explicitly wires all of these (`bounded_document_store_owners`, `bounded_config_store_owners`, `Puzzle3dConfigStorePreparationFactory`, `Puzzle3dArtifactStorePreparationFactory`, plus both store disposers). Without these overrides, `EditorApp<Puzzle2dPlayApp>::build_document_store_owners()` returns `None`, meaning the framework's generic atomic-replacement/close code paths (FRAMEWORK ~11202, ~11274-11290) have no document/config-store owner authority declared for Puzzle 2D through this trait at all — this may be an intentional simplification (2D's simpler board may not need atomic-replacement authority) or a real gap; worth a direct question to whoever owns Puzzle 2D persistence behavior, since it's a clean, unambiguous asymmetry versus 3D.

---

## 3. Extent/budget system — does 2d have a nakagin-scale trap analogous to 3d's ×64 object trap?

**Yes, but structurally different: no multiplier formula overflows the shared 4,096 budget; instead `forceLayout` fails a tighter, artifact-local node-count gate (64) that Nakagin's real size (180 nodes) was never checked against.**

### 2d Nakagin Capsule Tower fixture is native, not a 3d projection — corrected node count is 180 (not 121)

Puzzle2d ships its own real DSL fixture (not derived from 3d): `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio` (387 lines), loaded via `include_str!` in `…/🏗️nakagin-capsule-tower/🦀️.rs:19` and parsed with `crate::artifacts::puzzle2d::dsl::parse_dsl` (line 31 of that file). `flat-position` (`◻️2d/…/🧬️schema/💡️inferences/🎛️flat-position/🦀️.rs:36-41`) is a pure in-document BFS layout inference (resolves each node's `(x,y)` from `Fixed`/`Derived` anchors) — confirmed to have **no relationship to 3d** and no derivation path from the 3d document; each of 2d/3d/5d hand-authors its own independent `nakagin-capsule-tower` fixture.

Counted directly from the fixture (verified twice, first pass mis-bounded and undercounted at 121 — corrected by counting the raw `nodes [...] { … }` block span, lines 26-205, 180 indented rows):
- **180 nodes**, **179 edges**, **358 total handle entries**, **14 kind-compatibility rules**. This N=180/H=358 matches *exactly* the 3d Nakagin fixture's own corrected count from the sibling ticket's `📓️extent-tightening-report.md` (the 3d `📓️extent-budget-audit.md`'s "121 objects" figure was itself superseded/corrected there) — strong circumstantial evidence the 2d and 3d Nakagin fixtures were kept in numeric sync by hand, even though no code performs that sync automatically.
- concrete-forest 2d fixture (`…/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio`) is tiny: **1 node, 0 edges, 11 handles, 17 kind-compatibility rules** — the low end of the size spectrum.

### 2d editor's `extent()` implementations — only 4 retained tool ids total, all in `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

2d-specific budget constants live entirely in this editor file (none in the shared `🎮️commands/🧵️retained/🦀️.rs`, confirming §1's "no 3d/2d-specific code in the shared file" finding): `PUZZLE2D_BOARD_EVENT_BATCH_LIMIT: usize = 256` (line 1147), `PUZZLE2D_FORCE_MAX_NODES: usize = 64` / `PUZZLE2D_FORCE_MAX_EDGES: usize = 512` / `PUZZLE2D_FORCE_MAX_HANDLES: usize = 512` (lines 1408-1410), `PUZZLE2D_FORCE_ITERATIONS: u32 = 420` (line 1411). **There is no analog of `PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT`** — no per-object multiplier feeds any 2d `extent()` formula; the `PUZZLE2D_FORCE_MAX_*` constants are used as hard pre-gates returning `None` outright, not multiplied terms that could overflow 4,096.

| Tool id | Work type | `extent()` formula | file:line | Nakagin (N=180,E=179,C=14) value | Concrete Forest (N=1,E=0,C=17) value | Verdict |
|---|---|---|---|---|---|---|
| `setActiveExample` | `Puzzle2dActiveExampleWork` | `source_N+source_E+source_C + target.N+target.E+target.C + 2`, capped at 4,096 | 1272-1285 | from empty: **375**; Nakagin⇄Nakagin worst case: **748** | **20** | **SAFE** (linear in source+target size, no multiplier; far under 4,096 even doubled) |
| `forceLayout` | `Puzzle2dForceLayoutWork` | hard gate `nodes ≤ PUZZLE2D_FORCE_MAX_NODES(64) ∧ edges ≤ PUZZLE2D_FORCE_MAX_EDGES(512)`, else `None`; else `nodes+edges` (min 1) | 1559-1566 | nodes = **180 > 64** → **`None`**, regardless of edges (179 ≤ 512, moot) | gate passes → **1** | **BLOCKED on Nakagin** — same preflight fault as 3d's dead actions, `"puzzle command exceeds fixed semantic work capacity"` (🧵️retained/🦀️.rs:545), but for a **document-size gate 64× smaller than the 4,096 shared budget**, not an arithmetic overflow of it. **SAFE on Concrete Forest.** |
| `addNode` | `BoundedFirstStepCommandWork` / `puzzle2d_retained_extent` | `matches!(action_id, "addNode").then_some(1)` — constant | 1208-1210 | 1 | 1 | **SAFE**, document-size-independent |
| `applyBoardEvents` | `BoundedFirstStepCommandWork` / `puzzle2d_board_events_extent` | parses `eventsJson` batch, extent = event count, capped ≤ `PUZZLE2D_BOARD_EVENT_BATCH_LIMIT`(256) | 1149-1157 | depends only on browser flush-batch size, not document size | same | **SAFE by construction**, contingent on the board-session client never coalescing >256 events into one call — a client-side assumption, not something the 180-node document itself threatens |

### Two additional soundness gaps surfaced while tracing the runtime (beyond the headline `forceLayout` block)

1. **`forceLayout`'s declared `extent()` undercounts real work by ~1,470× at its own permitted maximum, but this doesn't hard-fault because of how `work_extent` is consumed.** Tracing `🧵️retained/🦀️.rs`: `work_extent` is only spent down by the `Preflight` phase (lines 539-553, one checkpoint per unit up to `work_extent`); the subsequent `Work` phase (555-574) calls `work.step()` in a loop with **no runtime check that `work_cursor` stays ≤ `work_extent`**. `forceLayout`'s `step()` (editor `🦀️.rs:1719-1794`) runs `PUZZLE2D_FORCE_ITERATIONS=420` iterations of an O(n²) all-pairs repulsion pass over nodes plus a springs pass over edges — at the maximum *permitted* 64 nodes that's `64·63/2=2,016` pairs × 420 ≈ **846,720** real `step()` calls versus a declared extent of at most `64+512=576` (nodes+edges cap sum). This is a latent performance/checkpoint-count soundness issue (each `step()` call is one checkpoint boundary) parallel in kind to 3d's `patchInspector` multiplier bug, but neither current fixture exercises it (Nakagin is blocked outright by the 64-node gate; Concrete Forest has 1 node).
2. **`PUZZLE2D_FORCE_MAX_HANDLES=512` is not checked in `extent()` at all** — it's only enforced as a hard runtime fault deep inside `step()`'s `Handles` stage (`🦀️.rs:1641-1643`, `Fault::from("puzzle2d-force-handle-capacity")`) if live handle count reaches 512. A hypothetical document with ≤64 nodes but >512 total handles would pass the `extent()` preflight and then fault mid-run — a real gap structurally analogous to 3d's `patchInspector` design error, currently unexercised by either fixture (Nakagin's 358 handles never reaches this code path since it's already blocked at the node gate).

### Net comparison to 3d

3d's three dead actions (`worldRelocate`, `createAttraction`, `acceptSuggestion`) and one design-error action (`patchInspector`) all come from formulas that **multiply** `objects.len()` by a large per-object constant (×64 vortices, ×512 decoded items) until the *shared* 4,096-item budget is arithmetically exceeded. 2d has no such multiplier anywhere, and no 2d action arithmetically overflows 4,096 on the 180-node Nakagin fixture. Instead, 2d's one dead action (`forceLayout`) fails a **narrower, artifact-local, non-multiplied design gate** (`nodes ≤ 64`) that sits 64× below the shared ceiling — Nakagin's 180 nodes are ~2.8× over that 64-node ceiling regardless of what the 4,096 shared cap is set to; raising `PUZZLE_COMMAND_WORK_ITEMS` (3d's proposed fix) would not help 2d's `forceLayout` at all, since it never reaches that shared cap — `PUZZLE2D_FORCE_MAX_NODES` itself would need raising.

Given `PUZZLE2D_RETAINED_TOOL_IDS` is only 4 of 40 puzzle2d editor actions (ticket status.md), this is the complete set of retained/resumable-job actions with budget exposure today — the other ~36 actions are still `BatchOnlyPendingRewrite` and not yet wired to `PuzzleCommandWork`/`extent()` at all.

### Also found: a code/audit mismatch on classification, worth flagging alongside item 2

The publication-authority audit `✏️s/🔌️plugins/🧩️puzzle/🔏️publication-authority/🔣️.json` lists `Puzzle2dPlayApp`'s only group as `"status": "BatchOnlyPendingRewrite"`, `"routes": ["addNode", "forceLayout", "setActiveExample"]` (no `applyBoardEvents` entry at all) — but the editor code itself (`◻️2d/…/✏️editor/🦀️.rs:2267-2270`) classifies all 4 of these as `InteractiveJobClassification::Migrated`, and `PUBLICATION_CONTRACTS` (lines 1135-1139) declares `ArtifactToolPublicationContract` entries with `Artifact` lane for exactly these 4 tool ids. This is a real staleness between the `🔏️publication-authority/🔣️.json` audit artifact and the current editor source — directly relevant to DoD item 3's requirement that "`publication-authority-audit Puzzle2dPlayApp` admit them," and worth reconciling as part of that work.

Separately, a parallel **non-retained** `setActiveExample` implementation still exists at `◻️2d/…/✏️editor/🎮️commands/🛍️set-active-example/🦀️.rs` (`begin_active_example`/`step_active_example`, self-chained via `Effect::DispatchAction("setActiveExampleStep")`, classified `BatchOnlyPendingRewrite` separately at line 2295) and is wired into `ArtifactApp::handle()` (`🦀️.rs:1907-1912`). Since the user-facing `"setActiveExample"` action is classified `Migrated`, production dispatch should route through the extent-budgeted `Puzzle2dActiveExampleWork` path instead, leaving this older `handle()` branch reachable only from test dispatch (its only non-definition call sites are in `#[cfg(test)]` blocks, editor `🦀️.rs:2408/2602/2612/2618`) — a plausible pre-migration leftover worth confirming as dead and removing per CLAUDE.md's no-legacy-code rule, though this was not traced deeply enough to assert with full certainty.

---

## 4. Raw wire cap (`PUZZLE_COMMAND_RAW_BYTES = 8192`) vs 2d payloads

### Definition and double enforcement

`PUZZLE_COMMAND_RAW_BYTES = 8_192` — `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:10`, backing the on-stack buffer `raw: [u8; PUZZLE_COMMAND_RAW_BYTES]` (lines 299/387) and `puzzle_command_contract()` (line 17-19), which every dimension uses identically: `ToolExecutionContract::resumable(PUZZLE_COMMAND_RAW_BYTES, PUZZLE_COMMAND_DECODED_ITEMS, 1, PUZZLE_COMMAND_OUTPUT_BYTES, PUZZLE_COMMAND_STEP_MICROS, 1, 1)`.

Enforced **twice**:
1. Generically, by the framework `ActionBus` — `🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs`: `RawWireLimit { controller_id, tool_id, actual, maximum }` (defined line 419), raised in `begin_exact_wire` (line 643-644, paged path) and `dispatch_wire` (line 786-787, non-paged path) whenever `declared_bytes > contract.max_raw_wire_bytes`.
2. Redundantly, inside each dimension's own `create_job_from_wire_pages_with_payload`: 2d at `◻️2d/…/✏️editor/🦀️.rs:1116`, 3d at `🧊️3d/…/✏️editor/🦀️.rs:6223`, 5d at `🖐️5d/…/✏️editor/🦀️.rs:8512` (and `:7356` gating a bounded-first-step path) — same-shaped check against the literal `PUZZLE_COMMAND_RAW_BYTES` constant.

The paging mechanism (`RetainedToolWireInput`, `TOOL_WIRE_PAGE_BYTES = 4_096`, same framework file lines 78/109-175) streams an already-bounded payload in fixed increments — `try_new` (line 119-121) rejects `declared_bytes > maximum_bytes` up front, and `admit_page` (128-140) refuses any page that would push admitted bytes past that declared total. **No chunked/streaming path exists anywhere in the puzzle plugin that lets a caller assemble a wire payload bigger than 8,192 bytes** — confirmed by grepping `chunk` case-insensitively across the whole `✏️s/🔌️plugins/🧩️puzzle` tree: every hit is either an unrelated 3d LOD config value (`chunk_size`/`setChunkSize`), in-process geometry array chunking (`.chunks_exact(2)`), or a resumable-work "chunk of mutations per step" (server-side incremental *application*, not wire transport — see `setActiveExample` below).

The framework *does* already support larger per-tool raw-wire contracts elsewhere — `FrameworkConfigurationBinaryJobFactory` (`🧰️framework/…/🔌️plugin/🦀️.rs:14620-14654`) uses `ToolExecutionContract::resumable(65_536, 1_024, 4_096, 65_536, 7_500, 1, 1)`, an 8x larger cap for its `"configuration-binary"` tool — so a per-tool override is a puzzle-plugin decision to make, not a missing framework capability.

### Puzzle2d's 4 retained tools against this cap

All four `PUZZLE2D_RETAINED_TOOL_IDS` (`setActiveExample`, `forceLayout`, `addNode`, `applyBoardEvents`) share the same 8,192-byte contract (re-confirmed at editor `🦀️.rs:2020` via the `bounded_first_step_tool_proofs!` invocation).

- **`applyBoardEvents` is the one genuine risk.** Wire argument is `{"eventsJson": "<JSON array of events>"}`. The item-count cap (`PUZZLE2D_BOARD_EVENT_BATCH_LIMIT = 256`) is checked only *after* raw-byte admission, so a batch well within 256 events can still overflow 8,192 raw bytes if individual events carry coordinate/geometry payloads (`brushPlace`/`edgeCreate`). The design assumption (doc-comment near editor `🦀️.rs:1159`) is that "the browser flushes a handful of events per interaction" — a deliberate bulk replay of 100+ events in one call is plausible to trip `RawWireLimit`.
- **`setActiveExample` is NOT a risk — and structurally sidesteps the whole problem**, unlike 3d/5d. Its wire argument is only `{"exampleId": "nakagin"}`; the ~129KB fixture JSON itself is embedded in the wasm binary (`crate::examples::puzzle2d::nakagin_capsule_tower::SOURCE`) and applied **server-side**, one mutation at a time, via the `Puzzle2dExampleStage` state machine (`ClearEdges → ClearNodes → Manifest → ClearCompatibility → AddCompatibility → Catalogs → Nodes → Edges → Complete`, editor `🦀️.rs:1230-1240` + step logic through ~1440). Nothing resembling the whole fixture ever crosses the wire.
- **`addNode`/`forceLayout`** — small fixed-shape arguments, no bulk-JSON risk.
- **`replaceKindCatalogs`** exists only as an internal *mutation* (pushed by `setActiveExample`'s stage machine, and by the `📚replace-kind-catalogs` mutation module) — never a directly wire-dispatched tool in 2d, so it is not subject to `PUZZLE_COMMAND_RAW_BYTES` at all.
- **`setFixtureJson` does not exist in puzzle2d** (0 hits for `set_fixture_json`/`setFixtureJson` under the 2d tree) — it exists only in 3d/5d (`🎮️commands/🧪️set-fixture-json/🦀️.rs`), where the repo's own sibling-ticket notes already flag it as blocked by `RawWireLimit` for the ~129KB Nakagin fixture (`.🧬semio/…/PUZZLE-3D-END-TO-END/📓️status.md:890`; `✏️s/🔌️plugins/🧩️puzzle/🔏️publication-authority/🔣️.json:51` explicitly records: *"argument is a whole fixture (~129KB for the Nakagin example) against the shared 8,192-byte PUZZLE_COMMAND_RAW_BYTES retained-job wire cap; needs a chunked or resumable wire path, not a wiring change"*).

### `ArtifactReservedToolInput::Media` — a real but unused bypass for 2d

Puzzle2d has **zero** usage of `ArtifactReservedToolInput`, `ArtifactReservedJob`, `ArtifactReservedToolJob`, or `ArtifactOwnedToolJobFactory`-for-reserved-routes anywhere in its editor/command files, and does not override `build_reserved_tool_job` (the framework default at `🧰️framework/…/🔌️plugin/🦀️.rs:11182` returns `Ok(None)` unconditionally) — consistent with §6's finding of no reserved-route implementation. It does declare one inbound media port, `kit:in` (editor `🦀️.rs:2044-2052`), but `import_media` is a hardcoded stub returning `Err(MediaError::NotImplemented)` (editor `🦀️.rs:2085-2087`, doc-commented as intentional: 2d's kind-catalog vocabulary is structurally unrelated to `kit.catalog`'s shape).

Architecturally significant: `Media`-variant reserved-tool-job requests are built with `raw_wire: Vec::new()` (framework `build_artifact_reserved_media_job`, `🦀️.rs:21847-21867`) — the payload travels as an already-decoded `Media` value, **never through `RetainedToolWireInput`**, so it is not subject to `PUZZLE_COMMAND_RAW_BYTES` at all. Separately, the framework gives every `ArtifactApp` a default `document:in`/`document:out` media path (framework `🦀️.rs:11561-11599`) that base64-encodes/decodes the *whole document pack* with no byte ceiling, gated only on the app implementing `whole_document_operation` — which none of puzzle2d/3d/5d do. So the framework-level bypass mechanism exists and is unused by puzzle entirely: 2d avoids the large-payload problem only by construction (fixtures baked into the binary, applied via a stepped mutation state machine), not by routing through Media/reserved jobs; 3d/5d's `setFixtureJson` is the one tool actually blocked by the 8,192-byte cap today.

---

## 5. Board session / wasm bridge (`✏️editor/🌉️wasm`, `@semio-tech/puzzle-wasm`, react vs wgpu)

**Both a React (wasm-bindgen) renderer and a native wgpu renderer exist and both drive the same `BoardHost` engine — but only React goes through `BoardSession`/the wasm bridge; wgpu links `BoardHost` natively and bypasses it entirely.**

### `✏️editor/🌉️wasm` module

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/` — `🦀️.rs` (475 lines), `🟦️.ts` (21 lines), plus test fixtures. Gated `#![cfg(all(target_arch = "wasm32", not(target_env = "p2")))]` — browser-only. Per its own header docstring, it exists specifically so puzzle2d's `⚙️engine` slot (`BoardHost`) stays free of `wasm-bindgen`/`web-sys`/`wgpu` dependencies, so a headless workflow runner can drive the same engine with no browser deps — this module is the *only* place those browser deps enter puzzle2d.

Exposes stateless geometry/DSL helper fns (`boardComputeEdgeBezier`, `boardRedrawLayoutFixtureJson`, `puzzle2dParseDslJson`, etc., lines 34-80) plus the stateful **`BoardSession`**:
```rust
#[wasm_bindgen]
pub struct BoardSession { state: Rc<RefCell<BoardSessionInner>> }   // 🦀️.rs:102-105
```
`BoardSessionInner` (84-87) wraps exactly `host: BoardHost` (the pure engine, from `crate::editor::puzzle2d::engine`) and `gpu: canvas::gpu_session::CanvasGpuSession` (the WebGPU presentation surface). Key methods: `new()`/`newNormal()` (109-118, build via `puzzle_board_host()`/`_normal()`), `attach_canvas` (136-158, binds WebGPU to an `HtmlCanvasElement`, returns a `Promise` via `future_to_promise` so `&mut BoardSession` is never held across `await`), `setSize`/`setCamera(Silent)`/`renderFrame` (160-169, 228-237, 469-472 — `renderFrame` → `host.build_vector_scene()` then `gpu.render_frame(&scene, clear)`), pointer/wheel interaction forwarders (239-272), and **`drainEventsJson`** (284-287, the event-out channel: `crate::editor::puzzle2d::drain_board_events_json(&mut host)`), plus document/fixture sync methods (`syncDescriptorJson`, `setNodePositionsJson`, `parseFixtureJson`, `setKindCatalogsJson`, etc.).

TS side (`🌉️wasm/🟦️.ts:1-21`) imports the wasm-pack build output by **relative path**, not the npm package name:
```ts
const loadModule = createWasmModuleLoader<PuzzleSessionModule>(async () => {
  const module = await import("../../../../../../../../📦️packages/🦀️rust/pkg/semio_puzzle.js");
  await module.default();
  return module;
});
export async function createPuzzleBoardSession(): Promise<Board2dWasmSession> { ... return new module.BoardSession(); }
export const PUZZLE_BOARD_SESSION_FACTORIES: readonly AppSurfaceSessionFactory[] = [
  { kind: "board-2d", pluginId: "puzzle", appId: "s.puzzle.puzzle2d@1/*#editor", create: createPuzzleBoardSession },
  { kind: "board-2d", pluginId: "puzzle", appId: "s.puzzle.puzzle2d@1/*#viewer", create: createPuzzleBoardSession },
];
```
Re-exported at `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🟦️.ts:35`. `BoardSession` is puzzle-plugin-only (no other plugin defines one). The generic contract it must satisfy (`Board2dWasmSession`) is defined framework-side: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx:277-321`.

### `applyBoardEvents` and `forceLayout` — TS↔Rust flow

**`applyBoardEvents`** is a command/action-id string (not a function name) — the single write-path verb the browser board session commits mutations through:
- React canvas emission — `🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx:443-448`: a buffer filled by `session.drainEventsJson()` on every pointer move/up (429-441) is coalesced and dispatched: `if (eventsJson) dispatch("applyBoardEvents", { eventsJson })`.
- Rust dispatch (plugin side) — `◻️2d/…/✏️editor/🦀️.rs:1988`: `"applyBoardEvents" => apply_board_events::apply_board_events(ctx, args)`, one of the 4 `PUZZLE2D_RETAINED_TOOL_IDS`.
- The **wgpu-native renderer emits the identical action-id string** (not through `BoardSession`) — `🧰️framework/…/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:3734` etc.: `board_action(controller_id, "applyBoardEvents", json!({ "eventsJson": events_json }))`. A framework constant `board2d_actions::APPLY_BOARD_EVENTS` exists (`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧩️component.rs:3746-3749`) but isn't actually referenced — both sides pass the literal string.

**`forceLayout`** has **no TS/wasm-bindgen call site at all** — it is dispatched generically as a menu/UI action id through the same `dispatch(action_id, args)` mechanism, with no payload:
- Rust dispatch — `◻️2d/…/✏️editor/🦀️.rs:1968`: `"forceLayout" | "reorganize" => force_layout::force_layout(ctx)`.
- Implementation — `.../✏️editor/🎮️commands/⚛️force-layout/🦀️.rs:6-13` — calls `crate::editor::puzzle2d::engine::apply_force_graph_layout_to_fixture_v1_json(&scene.fixture, r#"{"mode":"force-graph"}"#)` and replaces `ctx.scene.fixture` with the result.
- It runs purely server-side (plugin-component document mutation), never inside `BoardSession`/wasm-bindgen — confirming §3's finding that `forceLayout`'s node-count gate (`PUZZLE2D_FORCE_MAX_NODES=64`) is a server-side capacity gate unrelated to the browser rendering bridge.

### The force-layout algorithm — a real Fruchterman-Reingold-style spring embedder, shared with another plugin

`apply_force_graph_layout_to_fixture_v1_json` is **not implemented in the puzzle plugin at all** — it's a shared framework module also reused by the `reasoning`/`wires` mindmap plugin (comment at `✏️s/🔌️plugins/💡️reasoning/…/🧬️schema/🦀️.rs:259`: *"same shared mechanism `puzzle/2d`'s `forceLayout`/`reorganize` uses"*). Resolution chain: `🧰️framework/…/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1165-1168` → `🧰️framework/…/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:222-317` (builds positions/radii/edge-pairs from the fixture JSON, supports `"puzzle.2d.fixture"`/`"reasoning.mindmap.fixture"`/`"trinity.graph"` schemas) → core physics at `🧰️framework/🔨️modules/🕸️graph/🖊️drawing/🦀️.rs:77-124`, `run_force_layout`. It's an iterative spring-embedder with cooling: pairwise repulsion + Hookean springs toward an ideal edge length + optional gravity + damped velocity integration. Defaults: `iterations=420` (matches `PUZZLE2D_FORCE_ITERATIONS` from §3), `ideal_edge_length=140.0`, `repulsion_strength=6500.0`, `spring_strength=0.028`, `gravity=0.0`, `time_step=0.85`, `velocity_damping=0.88`, `max_speed=48.0`, deterministic seed `0x5eedfaced0`. **Notable finding**: `barnes_hut_theta`/`pairwise_repulsion_max_bodies` options exist but are read into unused local variables (`🦀️.rs:90-91`) — `add_pairwise_repulsion` is a plain O(n²) double loop, not an actual Barnes-Hut quadtree approximation; this corroborates the extent-budget agent's independent finding (§3) that a max-size `forceLayout` run is O(n²) per iteration and would need ~846,720 real step calls at the 64-node ceiling.

### `runWasmPackWebBuild` and the `@semio-tech/puzzle-wasm` package

Call site: `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts:8-17`:
```ts
runWasmPackWebBuild({
  rsDir: this.root, skipEnvVar: "PUZZLE_BOARD_SKIP_WASM_BUILD", logPrefix: "puzzle/board", wasmBaseName: "semio_puzzle", shipProfile: "wasm-release", noDefaultFeatures: true,
  pkg: { name: "@semio-tech/puzzle-wasm", files: ["semio_puzzle_bg.wasm", "semio_puzzle.js", "semio_puzzle.d.ts", "semio_puzzle_bg.wasm.d.ts"], main: "semio_puzzle.js", module: "semio_puzzle.js", types: "semio_puzzle.d.ts" },
});
```
Definition: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2864-2954`. Runs `wasm-pack build --target web --out-dir pkg --out-name semio_puzzle --no-pack [--release|--profile wasm-release] --no-default-features`, output to `pkg/` (matching the wasm bridge's relative import path), skip flag `PUZZLE_BOARD_SKIP_WASM_BUILD=1`, then synthesizes `pkg/package.json`. This `--target web` wasm-pack build is confirmed **separate** from the plugin's own `wasm32-wasip2` component build (`EXTENSION_COMPONENT_WASM_TARGET`, same file) used for headless/OS-shell plugin execution — i.e. puzzle2d ships two distinct wasm artifacts for two distinct purposes. No file under `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript` imports the bare `@semio-tech/puzzle-wasm` specifier — all consumption goes through the relative `pkg/` path; the only other consumer of the package name found repo-wide is `.storybook/stories/puzzle/3d/World.stories.tsx` (puzzle**3d**, not 2d, reusing the same wasm-pack build for `puzzle3dParseDslJson`).

### Which renderer(s) use the bridge — confirmed both, with a clean split

- **React canvas renderer** (uses `BoardSession`/the wasm bridge): `🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx` (1,044 lines) — holds `sessionRef.current: Board2dWasmSession`, calls `createPuzzleBoardSession()` → `attach_canvas`/`renderFrame`/`drainEventsJson()` on the real `BoardSession`, dispatches `applyBoardEvents`. Note: puzzle2d has **no dedicated `.tsx` component of its own** — the only `.tsx` file in the whole puzzle plugin tree is `📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/🟦️.tsx`, and that one is a pure data-composition module (flattens 5d parts for sketchpad topology), not a UI/JSX renderer. The actual React rendering surface for puzzle2d's canvas lives entirely in the **shared framework** `Board2dHost` component, not inside the puzzle plugin.
- **wgpu-native renderer** (bypasses `BoardSession` entirely): `🧰️framework/…/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` (~4,000+ lines) holds `board_host: Option<ManuallyDrop<puzzle::editor::puzzle2d::engine::BoardHost>>` (line 75/363) as a **plain native Rust struct field**, no wasm-bindgen. `paint_puzzle_board` (3542-3572) calls `host.build_vector_scene()` directly and renders via a vello-offscreen-texture wgpu pipeline (`render_vello_scene`), not `CanvasGpuSession`. Pointer/wheel forwarders (3944-4097) call the identical `BoardHost` methods `BoardSession` thinly wraps — one shared engine, two independent bridges. First-party confirmation from repo history: ticket `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️11/WGPU-RENDERER-FULL-PARITY/🎫️ticket.json:6` explicitly records *"used `puzzle_2d::BoardHost` directly as a plain Rust type (no wasm-bindgen wrapper)... `SEMIO_RENDERER=wgpu`"*.

This is a deliberate architecture per the wasm bridge module's own docstring: `BoardHost` is kept wasm-agnostic specifically so it can be embedded either behind a browser wasm-bindgen boundary (React/`BoardSession` path) or compiled straight into a native/wasm32-unknown-unknown renderer (wgpu path) — directly relevant to DoD item 5's "react renderer, then wgpu wasm" boot sequence, which exercises these two independent bridges over the same underlying engine in turn.

---

## 6. Reserved tool routes — 2d vs 5d, and the root `📜️script.ts` policy

**Confirmed: puzzle2d has no reserved-tool-route implementation at all, and the root policy's importer-cohort inventory explicitly requires one (`import-media`) that is currently missing/fail-closed.**

### `puzzle5d_reserved_factory!` — the production pattern 2d lacks

Defined in `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:122` (a `macro_rules!`), paired with a `puzzle5d_reserved_publication!` macro (~line 108) that supplies each factory's `PUBLICATION_CONTRACTS` (`copy` → `HostOnly` lane; `cut`/`paste`/`import-media` → `Artifact` lane). Invoked 4 times at lines 169-172:
```rust
puzzle5d_reserved_factory!(Puzzle5dCopyJobFactory, "copy", "puzzle.5d.reserved.copy.v1");
puzzle5d_reserved_factory!(Puzzle5dCutJobFactory, "cut", "puzzle.5d.reserved.cut.v1");
puzzle5d_reserved_factory!(Puzzle5dPasteJobFactory, "paste", "puzzle.5d.reserved.paste.v1");
puzzle5d_reserved_factory!(Puzzle5dImportJobFactory, "import-media", "puzzle.5d.reserved.import-media.v1");
```
Each expands into a struct implementing `ToolJobFactory` (fixed resumable contract, `Migrated` classification, a `payload_schema_id` literal) and `ArtifactOwnedToolJobFactory` (`Owner = EditorApp<Puzzle5dPlayApp>`, `DOCUMENT_SCHEMA = PUZZLE5D_SCHEMA`). Note puzzle5d's file header describes `Puzzle5dPlayApp` as "the plugin's unified 2d+3d play app" — architecturally distinct from the standalone `Puzzle2dPlayApp` artifact.

### Puzzle2d: zero hits for every reserved-route marker

Greps against `◻️2d/…/✏️editor/🦀️.rs` all returned 0 matches: `ArtifactReservedToolInput`, `build_reserved_tool_job`, `reserved_factory`, `"import-media"`/`"import_media"`. The one `ArtifactOwnedToolJobFactory` impl in the file (line 1131, `Puzzle2dRetainedCommandJobFactory`) has `TOOL_IDS = PUZZLE2D_RETAINED_TOOL_IDS` (`setActiveExample`, `forceLayout`, `addNode`, `applyBoardEvents` — none of copy/cut/paste/import-media). The only `"copy"`/`"cut"`/`"paste"` string hits (lines 2375-2377) are inside a `#[cfg(test)] mod testkit` dispatch helper that forwards to `app.handle_action(...)` for test purposes only — not a production registration, and `import-media` doesn't appear there either.

### Root `📜️script.ts` policy — two different gates, one of which does bind 2d

- `TOOL_JOB_FRAMEWORK_RESERVED_IDS` includes `copy`/`cut`/`paste` (generic, framework-level, all artifacts) at `📜️script.ts:1220-1240`; `TOOL_JOB_PLUGIN_RESERVED_IDS = ["copy", "cut", "paste", "import-media"]` — only `import-media` is genuinely plugin/artifact-owned (each artifact decodes its own media).
- `toolJobPuzzleReservedRoutesExact` (`📜️script.ts:4849`, invoked `:10317`) is **puzzle5d-only** — it hardcodes the path to `🖐️5d/…/✏️editor/🦀️.rs` and checks for `puzzle5d_reserved_factory!(...)`, `impl ArtifactEditor for Puzzle5dPlayApp`, `impl InteractiveJob for Puzzle5d{Copy,Cut,Paste,Import}Job`, etc. It never reads any puzzle2d or puzzle3d file, so it cannot be the mechanism requiring 2d to have these routes.
- **`toolJobReservedImporterOwners`** (`📜️script.ts:5199-5228`, invoked near `:10318`) is the gate that *does* reach 2d: it reads a JSON inventory at `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📊️p8yj-importer-cohorts.json`, which (lines ~546-560) **explicitly lists `Puzzle2dPlayApp` as an owner required to implement the `import-media` reserved route**, `cohort: "C"`, `"currentMonolith": true` (i.e. per that ticket's own bookkeeping, still unmigrated — `Puzzle5dPlayApp`'s entry in the same file has `"currentMonolith": false`, already done). The inventory's recorded `file` path for the 2d owner (`…/✏️editor/🦀️component.rs`) does not exist under today's taxonomy (`find … -iname "*component*.rs"` → 0 hits under puzzle2d editor), so `policyReadFileSafe` would read it as empty regardless — the check fails closed for `Puzzle2dPlayApp` either way.

### Net gap

`import-media` **is** required of puzzle2d by the root policy's importer-cohort gate, and no implementation exists — this drives (or should drive) a failure line of the shape `"N app-owned import-media route(s) remain fail-closed pending explicit resumable factories"` in the `verify` aggregation (`📜️script.ts` ~10380-10382). `copy`/`cut`/`paste` are framework-reserved and not specifically gated per-artifact by the puzzle-only exactness check, so their absence in 2d is not itself flagged by `toolJobPuzzleReservedRoutesExact` (which only inspects puzzle5d) — but is presumably covered (or not) by whatever generic framework-reserved-route check applies repo-wide to every `ArtifactApp`, which was out of scope for this puzzle-focused grep and was not separately verified.

---

## 7. Terminology/i18n (`✏️editor/🗣️terminology`) — English + German, no default language

**The 30 terminology entries themselves are 100% EN/DE-complete, but the "no default language" rule is violated (English is a silent fallback), unlike puzzle3d's fail-closed sibling in the same plugin.**

### Files and schema

Only one file: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs` (74 lines). Structural sibling `🧊️3d/…/✏️editor/🗣️terminology/🦀️.rs` (163 lines, 99 label fields vs 2d's 30) uses the identical macro schema: `semio_framework_plugin::app_labels! { pub struct <X>Labels { field: native_en "...", native_de "...", reuse_en "...", reuse_de "..."; ... } }` — a 2×2 **locale × terminology** matrix per field (`Locale::{En,De}` × `Terminology::{Native,Reuse}`), not a plain locale map. Example (2d, line 18): `nodes: native_en "Nodes", native_de "Knoten", reuse_en "Building components", reuse_de "Baukomponenten";`.

### Term completeness: 30/30 complete

All 30 label fields (lines 18-54: `nodes, handles, edges, none, window_overview, window_detail, window_selection, schema, extension, id, node_kind, x, y, automatic, lod, suggestion, offset, node_weights, handle_weights, select, brush, fill, count, placement, fill_progress, fill_cancel, fill_retry, fill_fault, fill_result, example_concrete_forest`) have all four cells (`native_en`/`native_de`/`reuse_en`/`reuse_de`) populated with non-empty text, verified line-by-line. **No missing English or German value anywhere in the module.**

### "No default language" rule — VIOLATED in puzzle2d, honored in puzzle3d

Framework's own stated intent: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:2454` — *"🌐️ Explicit locale identity. There is intentionally no default locale."*

**Puzzle3d honors this (fail-closed):** `puzzle3d_locale` (`🧊️3d/…/🗣️terminology/🦀️.rs:104-110`) matches only explicit `"en"|"en-US"` → `Locale::En` and `"de"|"de-DE"` → `Locale::De`, else `None`; `puzzle3d_labels` (lines 114-118) returns `Option<&'static Puzzle3dLabels>`, propagating `None` via `?` for any unsupported locale/terminology; a dedicated test `label_resolution_has_no_locale_or_terminology_default` (lines 146-161) asserts `"fr"` and `"de-AT"` return `None`, and an unsupported terminology (`"legacy"`) returns `None`.

**Puzzle2d violates it.** `puzzle2d_labels` (`◻️2d/…/🗣️terminology/🦀️.rs:61-65`):
```rust
pub fn puzzle2d_labels(config: &Puzzle2dConfig) -> &'static Puzzle2dLabels {
    let locale = if is_de_locale(config) { Locale::De } else { Locale::En };
    let terminology = if config.terminology.as_str() == "reuse" { Terminology::Reuse } else { Terminology::Native };
    Puzzle2dLabels::labels(locale, terminology)
}
```
with `is_de_locale` (line 71-73): `config.locale.starts_with("de")`. This is infallible (`&'static Puzzle2dLabels`, not `Option`) — any locale that doesn't start with `"de"` (empty string, `"fr"`, a typo, an unset config default) **silently falls back to English**, and any terminology other than exactly `"reuse"` silently falls back to `Native`. There is no way to signal "unsupported/unset locale," and **no test guards against this** (contrast with 3d's explicit test). This directly contradicts CLAUDE.md's "no default language" rule, and puzzle3d's own precedent inside the same plugin shows the fail-closed pattern was known and achievable.

### Action ids are structurally decoupled from the terminology module, and two have no label at all

Editor action ids: `puzzle2d_command_variants!` enum (`◻️2d/…/✏️editor/🦀️.rs:853-897`, 36 command ids). Spot-check against the terminology module:

- **26 of 36 action-palette commands bypass the terminology module entirely**, carrying inline `LocalizedLabel::native(en, de)` pairs at their registration call site instead (e.g. `addNode` line 2213, `setActiveExample` line 2214, `deleteSelection` line 2217, `forceLayout` line 2220, and `duplicateSelection`, `focusSelection`, `selectSameKind`, `setSelectionFlag`, `patchInspectorNodes`, `redrawHandles`, `reorganize`, `applyBoardEvents`, `setFillCount`, all `brushFillSession*`/`brush*` commands, `setCamera`, `engagement*`, `setLodModeForPane`, `setGridSnapEnabled`, `setGridFactor`, `setBrushKindWeights`, `setBrushNodeSize`, `setSuggestionOffset`, `lodScaleJson`). Both EN and DE text is present in each `native(...)` call (no missing translation), but these never route through `Puzzle2dLabels`/the `reuse` terminology axis — unlike puzzle3d, which exposes a dedicated `puzzle3d_localized_phrase` helper (`🧊️3d/…/🗣️terminology/🦀️.rs:129-137`) specifically so action/phrase labels can adapt under `reuse` terminology; **puzzle2d has no equivalent `_phrase` helper**, only the plain `puzzle2d_localized` (`◻️2d/…/✏️editor/🦀️.rs:2180-2181`).
- Only **4 of ~36** action-adjacent labels actually consume the terminology struct: `select` (utility, line 2307, via `puzzle2d_localized(|l| l.select)`), `brush` (line 2308, `l.brush`), `fill` (line 2310, `l.fill`), and the example-picker's `example_concrete_forest` option label (line 2263).
- **`setLocale` and `setTerminology` (enum lines 896-897) have no `ActionDefinition`/`LocalizedLabel` registration anywhere in the file** — each appears only 2 times total in the whole file (enum declaration + dispatch match arm at lines 1989-1990), versus ≥4 occurrences for every other of the 36 command ids (enum + dispatch + at least one label registration). No framework-level fallback label exists either (`"setLocale"`/`"setTerminology"` grep across the framework plugin-host file returns nothing). These two actions are genuinely unlabeled in the UI-facing action surface — a missing terminology entry, not merely a missing translation.

### Net

The terminology *content* that exists is complete and bilingual; the *access path* to it has a default-language violation (English fallback) that puzzle3d already solved correctly in the same crate, and the *coverage* of the terminology module across puzzle2d's 36 actions is thin (4/36), with 2 actions (`setLocale`, `setTerminology`) having no label at all.
