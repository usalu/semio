# 🔌️ Wave W0-H: tool-run integration seams

Lane W0-H of `📋️tool-run-contract.md` (§3.2 delivery and cursor, §3.3, §3.4, §4.1). **Status: landed. Owned suites green; the renderer-wgpu checks are blocked upstream by a peer (§3).**

Abbreviations:
- `R` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs`
- `P` = `🔌️plugin/🦀️.rs`
- `M` = `🧰️framework/🔨️modules/⏯️tool-run/`
- `W` = `♾️infinite/🌍️world/🦀️.rs`
- `E` = `…/📺️renderer/🧑‍🎨engine/🧱️elements/`

Logs are in `T/🗑️generated/W0-H/`.

## 1. What changed

### 1.1 Trace delivery end to end (§3.2)

**Echo carrier** (`🛂️manifest`, foreign)
- `ViewModel.tool_run_trace_cursor_by_window_id: HashMap<String, ToolRunTraceCursor>` (`🛂️manifest/🦀️.rs:4490`), wire name `toolRunTraceCursorByWindowId`. It is keyed by window instance id, like `activeUtilityByWindowId`. See deviation 1.
- Schema `🪟️view-context/🧬️schema/🔣️.json`: the property (maxProperties 64) plus `$defs/ToolRunTraceCursor` (`run` ≤ 2^53−1, `generation`/`page` u32).
- TS: `PluginViewState.toolRunTraceCursorByWindowId` and `ViewToolRunTraceCursor` (`🛂️manifest/🟦️.ts:869-873`); `parseResolvedPluginViewState` admits and validates the field (`:960,987`).
- Capacity: `VIEW_CONTEXT_TRACE_CURSOR_ENTRIES`, `VIEW_CONTEXT_TRACE_CURSOR_FIELDS`, and `MAX_SURFACE_VIEW_CONTEXT_BYTES` grown to cover the field (`:4570`).
- Fixture `🪟️resolved-host-context/🔣️.json`: a valid cursor plus 4 refused rows (negative page, unknown field, inexact run, empty window id).

**Cursor type** (`M/🦀️.rs:812`, additive)
- `ToolRunTraceCursor` now derives `Default`, serde and `ToValue`/`FromValue` (camelCase, `deny_unknown_fields`).

**Ledger** (`R`)

| Item | Line | What it does |
|---|---|---|
| `trace_delta` | `:546` | With no run (dismissed or retired), a cursor with `page > 0` gets one empty `clear` delta. `page == 0` means an empty layer and gets nothing. Encoding now uses the first-party `base64_codec::base64_url_encode`. |
| `trace_lane(window, body, cursor, budget)` | `:560` | Answers one window. It keeps per-window bookkeeping in `trace_windows: BTreeMap<String, ToolRunTraceWindow>` (`:414`). |
| Backlog rule | — | Pages left over after the byte budget mark the ledger UI-dirty. This repeats until the echoed cursor has stood still for `TOOL_RUN_TRACE_STALL_REFRESHES = 4` refreshes (`:36`), so a renderer that never echoes cannot cause a refresh loop. |
| `retain_trace_windows` | `:581` | Drops windows that are no longer live. |

**Injection** (`R:908`, called from `P` `render` at `:28339`)
- Scope: only renders that carry a `window_id`.
- It finds the first `world-3d@1` / `canvas-2d@1` surface depth first (`R:144`).
- The lane is answered for that window's echoed cursor.
- `inject_tool_run_trace_lane_into` (`R:155`) re-encodes the spine so its `lanes` gain `SceneLaneRef { toolRunTrace, bytes, scene_lane_hash }`, then appends the `paged_text_carrier`. The result is byte-identical to what `split_lanes` + `scene_surface` would emit. Surface bindings are kept.
- A body with no scene surface forgets its window record. Panels and override renders carry nothing.
- Dependency: `base64_codec` (`semio-framework-io-base64`, first-party) added to the plugin `Cargo.toml`.

**React echo** (W0-E module `E/🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx`)
- New functions:
  - `publishToolRunTraceCursor` (`:275`);
  - `toolRunTraceCursorViewState(liveWindowIds)` (`:282`). It skips a run id above 2^53−1; the guest then resends from page 0.
  - `useToolRunTraceCursorEcho(windowId)` (`:293`). It gives a stable `onCursor` and forgets the cursor on unmount.
- Foreign one-liners:
  - `WH:5128`: `useToolRunTraceStore(scene?.toolRunTrace, useToolRunTraceCursorEcho(windowInstanceId))`.
  - `🏛️ShellHost/🟦️.tsx:419` import, and `:4754` `toolRunTraceCursorByWindowId` in the refresh view state.

**wgpu echo**
- `World3dState.tool_run_trace_window_id` (`W:1422`) is set when `🎞️Scenes` paints the world into a window (`Scenes:1348`).
- `world3d_tool_run_trace_cursors(states)` (`W:10096`) collects the echoes.
- `🐚️Shell` `live_view_state` fills the map (`:4114`).
- The 6 full `ViewModel` literals in the Shell gained the field.

### 1.2 Presence (§3.4)

- **Guest.**
  - `EphemeralSnapshot.tool_run` (`P:11785`) is filled from the ledger (`P:27975`).
  - `AppFrame::Ephemeral` gains a trailing `tool_run: Vec<u8>` holding `encode_presence_tool_run` bytes, empty without a run (`📡️spr/🧵️channel/🦀️.rs:2291,3215`). `P:35861` emits it on every exchange.
  - TS mirror: `🛍️products/💻️os/🟦️.ts` type, encoder and decoder. Golden hex is updated in both the Rust and TS channel tests.
- **Codec** (replication, foreign):
  - `pub fn encode_presence_tool_run` and new `pub fn decode_presence_tool_run` (peer limits, no trailing bytes) (`📡️wire/🦀️.rs:1675,1866`);
  - TS `encodePresenceToolRun` / `decodePresenceToolRun` (`📡️replication/🟦️.ts:638,645`).
- **Presence assembly** (`🏪️store/🔄️sync/🦀️.rs`, owned):
  - `presence_tool_run_clamped` (`:1082`);
  - `PresenceHeartbeatProducer.observe_tool_run` (`:1114`). Once a summary is observed, it replaces the renderer's value in every offer (observed `None` clears a stale one). Before any observation, the renderer's value is still clamped so `completed ≤ total`.
  - `ArtifactHost::observe_presence_tool_run_key` (`:1347`).

### 1.3 Tick writer offset (additive, `M/🦀️.rs`)

- `ToolRunTickWriter::with_provisional_base(identity, provisional_len)` (`:1135`); `new` delegates to it.
- The writer tags each entity with the provisional length at append time (`entity_marks`, `:1122`). `retract_to(len)` drops pending entities tagged above `len`.
- The existing API is unchanged.
- In the ledger (`R:333`), job steps are renumbered by the ledger, the same way trace pages already were. This means a resumed job's writer, which restarts at step 0, cannot collide with `framework.toolRun.step.<seq>` ids.
- W0-D's toy run and revalidate jobs now write through `with_provisional_base`. They previously built `ToolRunTick` literals.

### 1.4 Dirty scope

- `ToolRunLedger::dirty_scope()` (`R:587`) returns `Partial { window_bodies: <body keys of the scene windows the lane rides in>, panel_bodies: ["framework.body.toolRun"] }`.
- Ticks and panel-only transitions call `mark_tool_run_ui_dirty` (`R:880`).
- Transitions that swap the document every window reads call `mark_tool_run_document_dirty` → `Full` (`R:887`). These are abort complete, fault discard, job complete, refold done, conflict return, finalized, and the `restart` discard.
- `flush_tool_run_ui_dirty` (`R:894`) keeps the flag set until the typed UI outbox has room, so a queued unrelated scope never swallows a run change. Each driver turn flushes first.
- Action results: `toolRunStart` → `Full`, because the scene windows are not known yet. Every other applied action → `dirty_scope()`.

### 1.5 Provisional entity set (§4.1 layer 1): the instance-record flag

- **Producer.** The scene producer stamps `"provisional": true` on each `instancesJson` record whose entity is in `ArtifactView::tool_run()?.provisional_entities`. The toy app in the suite is the reference producer.
- **React.** It already honours `WorldInstanceRecord.provisional` (W0-E/W1-C).
- **wgpu.**
  - `World3dSceneInstanceEntry.provisional` (`W:9555`) fills `World3dState.provisional_instance_ids` at bridge publication (`W:9878`).
  - The draw pass already repaints those instances with the provisional token.
- **Path for W1-B/W1-C.**
  - Stamp the flag wherever the fill scene serialises instances, including `instancesDeltaJson.changed[]` records.
  - React honours both. wgpu reads the flag only from full `instancesJson` through the scene bridge; the wgpu `instancesDeltaJson` and typed-snapshot paths do not carry it yet (open item 6).

### 1.6 wasm32-unknown-unknown `Send` fix (coordinator addition)

- `ToolRunJob = Box<dyn InteractiveJob + Send>` (`R:52`).
- On the browser target, `InteractiveJob` drops its `JobThreadTransfer: Send` bound, but the ledger lives in a `PluginApp: Send`.

## 2. Public API as landed

```rust
// semio-framework-tool-run (additive)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
pub struct ToolRunTraceCursor { pub run: u64, pub generation: u32, pub page: u32 }
impl ToolRunTickWriter { pub fn with_provisional_base(identity: ToolRunIdentity, provisional_len: u32) -> Self; }
// semio-framework (manifest)
pub struct ViewModel { …, pub tool_run_trace_cursor_by_window_id: HashMap<String, ToolRunTraceCursor> }
pub const VIEW_CONTEXT_TRACE_CURSOR_ENTRIES: usize = 64; pub const VIEW_CONTEXT_TRACE_CURSOR_FIELDS: usize = 3;
// semio-framework-plugin
pub const TOOL_RUN_TRACE_STALL_REFRESHES: u8 = 4;
pub type ToolRunJob = Box<dyn semio_framework_job::InteractiveJob + Send>;
impl<A: ArtifactApp> ToolRunLedger<A> {
    pub fn trace_delta(&self, cursor: Option<ToolRunTraceCursor>, byte_budget: usize) -> Option<String>;
    pub fn trace_lane(&mut self, window_id: &str, body_key: &str, cursor: Option<ToolRunTraceCursor>, byte_budget: usize) -> Option<String>;
    pub fn retain_trace_windows(&mut self, live: impl Fn(&str) -> bool);
    pub fn dirty_scope(&self) -> UiDirtyScope;
    pub fn is_ui_dirty(&self) -> bool;
}
impl VcsArtifactApp<A, M> {
    pub(crate) fn flush_tool_run_ui_dirty(&mut self);
    pub(crate) fn inject_tool_run_trace_lane(&mut self, tree: &mut ComponentTree, body_key: &str, view_state: &ViewModel) -> Result<(), Fault>;
}
pub struct EphemeralSnapshot { …, pub tool_run: Option<protocol::PresenceToolRun> }
// os kernel
AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction, tool_run: Vec<u8> }
pub fn presence_tool_run_clamped(tool_run: PresenceToolRun) -> PresenceToolRun;
impl PresenceHeartbeatProducer { pub fn observe_tool_run(&mut self, tool_run: Option<PresenceToolRun>); }
impl ArtifactHost { pub fn observe_presence_tool_run_key(&self, document_key: &ArtifactDocumentKey, tool_run: Option<PresenceToolRun>) -> bool; }
// replication
pub fn encode_presence_tool_run(tool_run: &PresenceToolRun, out: &mut Vec<u8>);
pub fn decode_presence_tool_run(bytes: &[u8]) -> Result<PresenceToolRun, ProtocolError>;
// os-infinite
pub struct World3dState { …, pub tool_run_trace_window_id: Option<String> }
pub fn world3d_tool_run_trace_cursors<'a>(states: impl IntoIterator<Item = &'a World3dState>) -> HashMap<String, ToolRunTraceCursor>;
```

```ts
export type ViewToolRunTraceCursor = { readonly run: number; readonly generation: number; readonly page: number }; // PluginViewState.toolRunTraceCursorByWindowId?
export function encodePresenceToolRun(toolRun: ArtifactPresenceToolRun): number[];
export function decodePresenceToolRun(bytes: Uint8Array): ArtifactPresenceToolRun;
export function publishToolRunTraceCursor(windowId: string, cursor: ToolRunTraceCursor | null): void;
export function toolRunTraceCursorViewState(liveWindowIds: readonly string[]): Record<string, ViewToolRunTraceCursor>;
export function useToolRunTraceCursorEcho(windowId: string | null | undefined): (cursor: ToolRunTraceCursor) => void;
```

## 3. Tests run (foreground)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-plugin --features artifact-app-testing -- tool_run` | **15 passed, 0 failed** (`plugin-test-final.txt`): W0-D's 9 laws, W0-B's builder test and 5 new laws. No warnings in `R` or the suite. |
| `cargo test -p semio-framework-tool-run` | **23 passed** (`tool-run-test-1.txt`); 2 new: `writerResume` fixture law, cursor serde/value round trip |
| `bun test ./🧪️tests/🧩️conformance/🟦️.ts` (in `M`) | 18 pass; ajv validates the new `writerResume` fixture section (`tool-run-ts-1.txt`) |
| `cargo check -p semio-framework-plugin --features component-guest --target wasm32-wasip2` | Finished; the plugin lib reached 28 warnings (`plugin-wasip2-1.txt`, `-2.txt`) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` | `semio-framework-plugin` **now compiles** (26 warnings; the 7 `Send` errors are gone), and so does `os-infinite`. The crate itself stops on peer errors in `semio-s-artifact-puzzle-3d` (`FixtureObject.reveal_index`, `AtomicU64` import, the removed `FillBuildProgress`/`FILL_TRIED_RING`; W1-A/W1-B in flight). See `renderer-wasm32-unknown-1.txt`. A retry got past puzzle-3d (110 warnings) and stopped in `semio-s-artifact-puzzle-5d` on the removed `Puzzle3dPrecomputeSession::fill_*` API (`renderer-wasm32-unknown-2.txt`). |
| `cargo check -p semio-framework-os-renderer-wgpu --lib` (native) | Same puzzle-3d peer block. Both runs happened after my edits (`renderer-native-check-1.txt`, `-2.txt`). **Consequence: the `🐚️Shell`/`🎞️Scenes` edits (§1.1 wgpu echo) are not compiler-verified.** |
| `cargo test -p semio-framework --lib -- view_context resolved_host_context window_view_context` | 9 passed (`manifest-test-1.txt`) |
| `bun -e 'testResolvedHostContext()'` | 17 cases (ajv + TS parser), exit 0 (`view-context-ts-1.txt`) |
| `cargo test -p semio-framework-replication -- presence` | 17 passed, including the new standalone-body law over the peer corpus (`replication-test-2.txt`) |
| `bun T/🐍️w0h-presence-tool-run-body-probe.ts` | 9 corpus bodies, 0 failures: TS codec agrees with Rust (`presence-body-probe-2.txt`) |
| `cargo test -p semio-framework-os-kernel --features sync --lib -- presence frame golden` | 69 passed, 1 failed. The failure is the known `presence_retirement…mounted_worker…` red W0-F reported as pre-existing. Passing tests include the new `artifact_host_presence_heartbeat_stamps_the_observed_tool_run_summary` and the channel golden corpus (`os-kernel-test-1.txt`). |
| `bun ./📜️script.ts test long "../../🟦️.ts"` (os TS package) | 224 passed, including Ephemeral round trips and golden hex (`os-ts-test-3.txt`) |
| `cargo test -p semio-framework-os-infinite --lib -- scene_bridge world_surfaces_echo tool_run` | 10 passed; 2 new: provisional flag through the real bridge, cursor echo map (`infinite-test-2.txt`) |
| `bun ./📜️script.ts test long "tool-run-trace"` (react package) | 8 passed; 1 new: echo is ajv-valid against the view-context schema, admitted by `parseResolvedPluginViewState`, and caught up for the TS ledger (`react-trace-test-1.txt`) |
| `bun ./📜️script.ts test long "🌐️World3dHost/🧪️tests"` | 1 passed (W1-C mount test with the echo hook) |
| `bunx tsc --noEmit -p tsconfig.json` (react package) | 0 errors mention tool-run, the echo or the view-context field. The remaining errors are pre-existing peer errors (`react-tsc-1.txt`). |

**The five new plugin laws**
- `…scene_render_carries_the_trace_lane_and_honours_the_echoed_cursor`: lane present and matching its ref; no cursor → clear plus the full log; caught-up cursor → nothing; resume → pages from the echoed page; generation mismatch → clear from page 0; dismiss → empty clear; `page 0` → nothing.
- `…trace_backlog_keeps_the_scene_dirty_until_the_echoed_cursor_stalls`: a one-byte budget delivers one page per refresh, and a stalled cursor costs exactly 4 dirty refreshes.
- `…tick_dirty_scope_is_the_panel_plus_the_scene_windows_and_excludes_unrelated_windows`.
- `…presence_and_the_ephemeral_snapshot_follow_the_run_with_completed_never_above_total`.
- `…provisional_entities_ride_the_instance_records_the_producer_stamps`.

**Mutation checks** (every file restored and compared byte-identical with `cmp`)
- Injection answering `None` instead of the echoed cursor → the cursor law failed.
- `flush` pushing `Full` → the dirty-scope law failed (`plugin-test-mutation-1.txt`).
- Dropping the writer's entity truncate → the `writerResume` law failed (`tool-run-test-mutation-1.txt`).

**Full plugin lib run** (`plugin-lib-full-1.txt`)
- About 96 reds. The run aborts on the peer `a_long_command_stream_never_pins_the_retained_ingress_authority` destructor panic.
- Sampled reds are unrelated and pre-existing:
  - `interactive-job-classification`;
  - `plugin-assembly.package-id`;
  - `interactive-job.catalog-authority`;
  - W0-D's `testkit-txn` transaction fixtures.
- W0-D's 771-tick bench also went red under full-suite parallel load (preemption budget); it passes in the targeted run.

## 4. Commands to register in launch.json

- `cargo test -p semio-framework-plugin --features artifact-app-testing -- tool_run` (same as W0-D; still needs a filtered `📜️script.ts` target)
- `bun nx run @semio-tech/framework-tool-run-rs:test`
- `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown`
- `bun ./📜️script.ts test long "tool-run-trace"` (react package)

## 5. Deviations from the contract, with reasons

1. **Where the cursor rides.** It is a view-state map keyed by window instance id, not a field inside `windowInstances[]`. Reasons:
   - This is the repo's established pattern for host-owned per-window state (`activeUtilityByWindowId`).
   - It keeps `ViewWindowInstance` a pure roster entry, which 144 literal sites across concurrently edited plugins construct.
2. **Per-refresh budget and backlog.** The contract only says "at most a per-refresh byte budget". The ledger additionally re-dirties scene windows while a backlog exists, bounded by `TOOL_RUN_TRACE_STALL_REFRESHES`. Without that, a completed run with more pages than one budget would stall until the next unrelated refresh.
3. **One lane per window body.** Only the first scene surface of a window body carries the lane, because one echoed cursor exists per window.
4. **Dirty scope split.** Tick-sized changes use the minimal scope. Document swaps and `toolRunStart` still use `Full`: a non-scene window reading the overlay must see an abort or finalize, and before the first render no scene window is known.
5. **Presence transport.** The summary rides `AppFrame::Ephemeral` (plus `EphemeralSnapshot`) in the peer's own tool run encoding. The store sync assembly stamps it into every heartbeat.
6. **Entity marks at tick granularity.** No `ToolRunTick` field was added: W1-A constructs `ToolRunTick` literals, and the brief requires additive changes. The writer is exact inside a tick. The ledger stays conservative across ticks (W0-D deviation 6).
7. **The ledger renumbers job steps.** This mirrors page renumbering; otherwise resumed writers would duplicate panel step ids.

## 6. Foreign edits

| Area | Files |
|---|---|
| Manifest (W0-B) | `🛂️manifest/🦀️.rs` (ViewModel field, capacity), `🛂️manifest/🟦️.ts`, `🪟️view-context/🧬️schema/🔣️.json`, `🪟️view-context/🧫️fixtures/🪟️resolved-host-context/🔣️.json`, `🧪️tests/🔬️view-context-capacity/🦀️.rs`, `🧪️tests/🪟️resolved-host-context/🦀️.rs` |
| Replication (W0-F) | `📡️wire/🦀️.rs` (pub encode, new decode), `🟦️.ts` (2 exports), `📡️wire/🧪️tests/🔬️presence-codec/🦀️.rs` (1 law) |
| Channel | `📡️spr/🧵️channel/🦀️.rs` and its unit tests (field, golden hex); `🛍️products/💻️os/🟦️.ts`; `🧪️tests/🧪️backbone-envelope-io/🟦️.ts` |
| Plugin `P` outside the tool-run regions | `EphemeralSnapshot` field (`:11785`), `ephemeral_snapshot` (`:27975`), frame emission (`:35861`), render hook (`:28339`); `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:2975` and `🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:471` (new field); `📦️packages/🦀️rust/Cargo.toml` (`base64_codec`) |
| wgpu | `W` (entry flag, window id, cursors fn); `W/🧪️tests/🔬️unit/🦀️.rs` (2 laws); `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1348` (window id); `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`live_view_state` plus 6 literals) |
| React | `E/🌐️World3dHost/🟦️.tsx` (import, `:5128`); `E/🏛️ShellHost/🟦️.tsx` (`:419`, `:4754`) |

Ticket input script: `T/🐍️w0h-presence-tool-run-body-probe.ts`.

## 7. Open items

1. **Coordinator.** Rerun `cargo check -p semio-framework-os-renderer-wgpu --lib` for both native and `--target wasm32-unknown-unknown` once the puzzle crates compile again (the last block was `semio-s-artifact-puzzle-5d`). This is the only compiler proof still owed for the Shell and Scenes edits.
2. **Host consumers of `AppFrame::Ephemeral.tool_run`.** No host consumes Ephemeral frames today: presence pack and interaction are unconsumed too, and `PluginRuntime.ephemeralSnapshot` is `undefined`.
   - React: cache the latest frame per instance in `🔌️PluginRuntime`, decode with `decodePresenceToolRun`, and set `toolRun` in `🏛️ShellHost`'s `presenceHeartbeat` peer (`:7234`).
   - Native: observe frames in the `🌉️ProgramBridge` exchange, then call `ArtifactHost::observe_presence_tool_run_key` before `🐚️Shell::advance_presence_preview_step`.
3. **W1-B.** Stamp `"provisional": true` on fill instance records from `doc.tool_run()`. Use `ToolRunTickWriter::with_provisional_base` for resumed and revalidate jobs.
4. **W2-A.** When mounting `ToolRunTrace2dLayer`, pass `useToolRunTraceCursorEcho(windowInstanceId)` as its cursor callback.
6. **wgpu provisional on delta/snapshot paths.** `provisional_instance_ids` is refreshed only by the `instancesJson` bridge; a producer that publishes `instancesDeltaJson` or a typed `World3dSnapshotLease` needs the same flag carried there (W1-C/W0-E owners).
5. **Stale projection.** The `🧬️schema/📽️projection/🦀️.rs` `ViewModel` TypeScript entry was already stale: it lacks `focusedWindowId` and `sessionIdentity`. It was not extended here; whoever owns the projection should regenerate it.
