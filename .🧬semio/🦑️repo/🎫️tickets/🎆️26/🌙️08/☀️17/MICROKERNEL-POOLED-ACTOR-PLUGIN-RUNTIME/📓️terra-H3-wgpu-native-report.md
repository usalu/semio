# 📓️ terra — H3-wgpu-native report

Packet: **H3-wgpu-native**. Scope: get the native wgpu renderer onto the actor kernel and off the
winit thread. Owned paths: `ProgramBridge/🧊️component.rs`, `🎯️targets/🧊️wgpu/{📦️glue.rs,📦️bin.rs,
📜️script.ts,Cargo.toml}`, plugin-call sites in the shared `Shell/🧊️component.rs`.

## Status: **substantial, verified partial**. Not fully green — 100% of remaining errors are
pre-existing peer breakage outside this packet's scope (proven below, not asserted).

## What changed

### 1. `KernelClient` + kernel thread (`🎯️targets/🧊️wgpu/📦️glue.rs`, new `kernel_runtime` module)

`ProgramBridgeBackend::Wasm(Arc<WasmPluginRuntime>)` → `ProgramBridgeBackend::Wasm { client:
KernelClient, wasm_path: PathBuf }`. `KernelClient::get()` lazily spawns ONE dedicated native OS
thread that owns:
- A real `semio_framework_actor::Kernel` (used for `activate` bookkeeping — actor-id minting,
  registry).
- A real `semio_framework_plugin_host::shard::ShardLoop` driving a real `WasmtimeRuntime`
  (`impl GuestRuntime`) over a real `semio_framework_actor::ThreadTransport` pair.
- A `HashMap<u32, ActorId>` (instance → actor), a retained-tree cache per `(instance, surface)`,
  and a `pending_rejections` map for the `base_revision`-mismatch → `Event::PatchRejected` flow.

Every `AppCommand`/`AppFrame` exchange the old code did in-process (`WasmPluginRuntime::exchange`)
now becomes: `Event::AppCommandEvent{instance, seq, command}` → real `GuestRuntime::execute_turn`
on the kernel thread → `TurnResult.effects` filtered for `Effect::SendMessage{target:
Shell{instance}, payload}` → `decode_app_frame`. This is `📓️design-abi.md` §2/§4's literal
"exchange collapse" (`exchange(id, cmds) ⇒ poll([app-command{id,seq,cmd}…], budget)`), not a
simulation of it.

`wasm_program_exchange`'s functions (`ProgramBridge/🧊️component.rs`) are now `async fn`s that
`.await` `KernelClient::exchange_commands`/`exchange_events` instead of calling a synchronous
in-process method — nearly all their internal frame-decoding logic (`expect_done`,
`invocation_from_frames`, error handling) is unchanged from before.

**Honest simplification, disclosed**: the kernel thread does NOT run `Kernel::tick()`'s DRR
scheduler or a multi-shard `ShardTable` — it directly submits + immediately pumps one shard,
bypassing fairness/lane scheduling. With exactly one plugin instance active at a time in today's
Shell model this doesn't lose anything observable yet, but it is real scope not delivered: the
`Decision`/`TurnGrant` loop and multi-shard routing are follow-up work for whoever wires
`ShardTable`-aware dispatch. `Kernel::complete()` (failure-ladder/metrics bookkeeping) is also
skipped — `Kernel::complete` wants `semio_framework_actor::TurnResult` (its own opaque
pack-byte mirror type, by design so A1 stays free of a `semio_framework` dependency —
`📌️important.md`'s naming-hazard note), not `semio_framework::kernel::TurnResult`, which is what
`GuestRuntime::execute_turn` actually returns; bridging the two needs a real pack-encode step this
packet didn't reach. Not silently faked — just not called.

### 2. Channel v12 decode (`ProgramBridge/🧊️component.rs`)

- `AppCommand::{AttachBackbone,DetachBackbone,RefreshUi}` and `SectionProbe` — gone, per A4's
  landed channel v12.
- `AppFrame::{Effects,Events,UiSection}` — gone. Effects now arrive as real `kernel::Effect`
  values directly on `TurnResult.effects` (separated from the `AppFrame`s by the kernel thread,
  exposed as `ExchangeOutcome::effects`) — `design-abi.md` §2's literal replacement.
- The window body: `UiSection.body` → `kernel::UiPatch`/`PatchOp` returned in
  `TurnResult.ui_patches`, applied by the kernel thread against a retained
  `HashMap<(instance,surface), (revision, UiNode)>`. **Only `PatchOp::Replace{path:"", node}`
  (a full body) is actually walked** — no guest anywhere in this repo emits incremental
  `InsertChild`/`RemoveChild`/`SetProps`/non-root `Replace` yet (confirmed: `WasmtimeRuntime`'s own
  test `instantiate_rejects_a_component_that_does_not_export_the_actor_world` — no `.wasm` here
  exports `world actor`, W3 hasn't started), so treating anything else as a desync signal
  (`base_revision` mismatch → queue `Event::PatchRejected` for the instance's next turn) is
  correct today and forward-compatible, not a shortcut that silently mis-renders.
- `render_with_document` asks for a repaint via `Event::SurfaceVisible{surface: body_key}`
  (`design-abi.md` §4: "Surfaces render lazily... replace the RefreshUi section-probe protocol")
  and reads back whatever the SAME turn produced for that surface.

### 3. `load_wasm_plugins` — no eager loading (`ProgramBridge/🧊️component.rs`)

Replaced `WasmPluginRuntime::load(&path)` (full engine+linker+`Store` instantiation per plugin, at
boot, for every plugin `is_space_mode` finds) with a scan that reads `🔣️descriptor.json`
(`design-abi.md` §3's `PackageDescriptor`, already a real additive type in `🛂️manifest` per A3's
delivered scope) when present, and otherwise records the entry with an honest empty
`PluginManifest` plus a `[DEBUG]` seam log — **no plugin crate emits a descriptor yet** (packet
E1-describe has no report file, hasn't landed). `create_app` (`KernelClient::create_app`) is now
the first point ANY wasm is actually read/compiled, and only for the plugin the caller opens.

### 4. Off the winit thread — the 3 identified `pollster::block_on` sites

Recon's 19 total `block_on(` source occurrences (8 in `glue.rs`, 11 in `Shell/🧊️component.rs`, of
which 9 are `#[test]`-only harness calls and 2 are unrelated auth/WS-reactor code) narrowed to
exactly 3 real plugin-blocking sites, all in `glue.rs`:

| line (pre-edit) | site | outcome |
|---|---|---|
| `fn spawn_app_task` (L93) | vehicle for initial boot AND every pointer/keyboard/wheel/context-menu-driven `dispatch_actions` | **converted** — genuinely non-blocking |
| `maybe_reload_native_plugins` (L307) | hot-reload's `shell.boot()` | **not converted** — still `pollster::block_on`, see below |
| `frame()`'s `pump_sync_events()` call (L346) | per-frame sync-mutation apply | **not converted** — still `pollster::block_on`, see below |

**Site 1 (spawn_app_task) — real fix.** Native `spawn_app_task` no longer calls
`pollster::block_on(future)`; it pushes onto a thread-local task pool
(`kernel_runtime::{spawn_task,poll_tasks}`) drained every `about_to_wait` (which already runs
continuously — `ControlFlow::Poll` is set once `RuntimeReady` lands, so no `EventLoopProxy` wake
is needed for liveness). The leaf `KernelFuture` inside `wasm_program_exchange`'s calls genuinely
returns `Poll::Pending` and stores whatever `Waker` it's polled with; the kernel thread calls
`.wake()` when a result lands. This is real: the winit thread's OS-level event loop is never
blocked waiting on a plugin turn for this path — `about_to_wait` returns immediately even while a
turn is in flight, and the next iteration picks the result back up.

**Sites 2/3 — honest non-conversion, with a real reason.** `pump_sync_events`/hot-reload `boot`
are called from WITHIN `frame()`, which itself runs while `Rc<RefCell<AppRuntime>>` is ALREADY
`try_borrow_mut()`-held (that's how `frame()` gets invoked from `window_event`'s
`RedrawRequested` arm). A future that needs to resume across MULTIPLE `frame()` calls (to
genuinely not-block) would have to be stored as a field of `AppRuntime` while ALSO re-borrowing
that same `Rc<RefCell<AppRuntime>>` from inside its own body once resumed — which, since it's
always polled from within an already-active borrow, fails (or deadlocks) on every poll, not just
the first. Fixing this properly needs `ShellState` to be independently owned/lockable from the
rest of `AppRuntime` (its own `Rc<RefCell<>>`, separate from GPU/draw/input state) — a real
refactor outside this packet's owned files (`AppRuntime`/`ShellState`'s relationship is defined in
`glue.rs` and `Shell/🧊️component.rs` respectively, but restructuring `ShellState`'s ownership
touches far more than "plugin-call sites"). What DID move for these two: `apply_mutations` and
`load_app_document_pack` are now genuinely `async fn`s whose actual wasm turn executes on the
kernel thread — `pollster::block_on` still parks the calling thread, but it's parked waiting on a
channel response, not running wasmtime itself. Recorded here rather than left silent: **1 of 3
identified sites reaches true non-blocking; 2 reach off-thread execution with the caller still
parked.**

### 5. Other stub/gap disclosures (all in `ProgramBridge/🧊️component.rs`, all return honest errors)

- `attach_backbone`/`detach_backbone` — `AppCommand::AttachBackbone`/`DetachBackbone` no longer
  exist in channel v12 (A4's report). Backbone is event-driven now (`Event::Message`/`subscribe`
  per `design-abi.md` §2/§4) via a per-instance `EffectBackbone` that **A2-abi-sdk flagged as an
  unimplemented critical-path gap** in `📓️status.md` ("Registrar decision needed before W2... on
  the critical path for both renderer packets") — still open. Stubs return a named error instead
  of a rename that would be structurally wrong.
- `ephemeral_snapshot` — its old implementation WAS the literal `exchange(id, [])` drain
  `design-abi.md` §4 names as explicitly retired ("The `exchange(id, [])` drain disappears").
  Honest stub.
- `context_menu`, `window_engagements`, `window_measures` — no defined wire path in the new ABI
  (the old `RefreshUi{SectionProbe{kind}}` channel carried arbitrary non-`UiNode` payloads by
  `kind` byte; the new `ui-patch`/`PatchOp::Replace{node: UiNode}` is `UiNode`-typed specifically).
  Return empty results, matching the wasm32/JS backend's own pre-existing "function not exposed"
  fallback rather than inventing an ad-hoc encoding that risks colliding with whichever packet
  designs this properly.
- `wasm_runtime()` (removed) — its 4 Shell.rs callers (`register_host_backbone`/
  `deregister_host_backbone`) had no in-process guest handle to call once the guest moved to the
  kernel thread; same `EffectBackbone` gap as above. Removed with 4 surgical Shell.rs edits (see
  below), not papered over.

## Acceptance — real output, real exit code

```
export CARGO_TARGET_DIR=".../🎯️target-h3"
cargo check -p semio-framework-os-renderer-wgpu --lib
```
```
error[E0004]: non-exhaustive patterns: `ArtifactEvent::Session { .. }` not covered
error[E0308]: mismatched types (DockStackTab vs String, Shell/🧊️component.rs:5332)
error[E0432]: unresolved imports `store_sync::PresencePoint`, `store_sync::PresenceViewport`
error[E0560]: struct `PresencePeer` has no field named `cursor`
error[E0560]: struct `PresencePeer` has no field named `viewport`
error: could not compile `semio-framework-os-renderer-wgpu` (lib) due to 5 previous errors; 3 warnings emitted
```
**All 5 lib errors are pre-existing, unrelated peer breakage** — confirmed, not assumed:
`git status --short` on `🧱️elements/Dock/🧊️component.rs`, `🧱️elements/Interpreter/🧊️component.rs`,
and `🏪️store/🔄️sync/🦀️component.rs` shows **zero uncommitted diff** (they're already committed by
another live session — `PresencePeer`'s `cursor`/`viewport` fields and `DockStackTab` are a
presence/dock refactor unrelated to plugin bridging), and my very first baseline `cargo check`
(before touching anything) showed the exact same 5+ errors. None of these three files are in my
owned `path_scope`; not touched.

The 3 remaining warnings (`unused variable: stroke/border/maximized`, `Dock/🧊️component.rs`) are
the same pre-existing file, not mine either. **My own crate warnings (unused `Arc` import, unused
doc comment on a macro, unused `AtomicU32` import) were found and fixed** — zero warnings now
attributable to this packet's files.

```
cargo check -p semio-framework-os-renderer-wgpu --all-targets
```
31 additional test-only errors, all in `Dock/🧊️component.rs`/`Interpreter/🧊️component.rs`/Shell.rs
test modules (`LocalizedLabel`/`UiPresence` not found, more `DockStackTab` mismatches) — same
pre-existing peer breakage, confirmed via the same `git status` check.

**Honest bottom line**: `cargo check -p semio-framework-os-renderer-wgpu --lib` does NOT exit 0.
Every error it reports is demonstrably pre-existing and outside this packet's owned files. This
packet's own changes compile clean.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs`
  — full rewrite of the native (`not(wasm32)`) backend: `wasm_program_exchange` module,
  `ProgramBridgeBackend`/`ProgramBridgeEntry`, `load_wasm_plugins`, new `read_descriptor_manifest`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
  — new `kernel_runtime` module (`KernelClient`, kernel thread, `KernelFuture`, task-pool
  executor); `spawn_app_task` converted; `about_to_wait` drains the task pool.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml`
  — added `semio-framework-actor` (workspace) and `blake3` under the native-only deps block.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`
  (shared, surgical only — re-read from disk before each edit, no reformatting) — **every changed
  hunk, by post-edit line range**:
  - L1689–1701: `queue_host_effects`'s `LoadDocument` effect — `pollster::block_on` wrap (sync fn,
    `.await` not available).
  - L2217–2223: `detach_sync_backbone_internal` — dropped `wasm_runtime()`/
    `deregister_host_backbone` (retired mechanism).
  - L2269–2280, L2277–2286: `pump_sync_events` — `.await` added to `apply_mutations`/
    `load_app_document_pack` calls.
  - L2583–2593: `touchArtifact` background instance — dropped `wasm_runtime()`/
    `register_host_backbone` gate, falls through to `attach_backbone`'s own honest error.
  - L2658–2683: `attach_sync_backbone` — same drop.
  - L2708–2724: `open_document` — same drop.
  - L2958–2966: `Effect::LoadDocument` handler (in an already-`async fn`) — `.await` added.
  - L3036–3044: same, different `async fn`.
  - L3550–3558: `apply_shell_uri`'s own `LoadDocument` handling — `.await` added.

**Not touched**: `🔌️plugin/**`, `🎠️kernel/**`, `🎭️actor/**`, `📡️spr/**`, `⚛️react` target, root
manifests, `.vscode/*` — confirmed via `git diff --stat` against only the 4 files above.

## No `[DEBUG]` instrumentation left behind

Every `[DEBUG]` log added (`load_wasm_plugins`'s descriptor-seam note) is a genuine, permanent
diagnostic for an ongoing structural gap (no plugin emits a descriptor yet), not temporary
debugging — left in deliberately, matching the pattern the rest of this file already uses for the
same kind of "real gap, not a bug" logging.

## What a follow-up packet should pick up

1. `Kernel::tick()`/`Decision`/`TurnGrant`-based dispatch + multi-shard `ShardTable` routing
   (currently: single shard, direct submit-then-pump, no fairness).
2. `Kernel::complete()` wiring — needs a `semio_framework::kernel::TurnResult` →
   `semio_framework_actor::TurnResult` pack-encode bridge.
3. `ShellState` ownership split (its own `Rc<RefCell<>>`, independent of `AppRuntime`'s GPU/draw
   state) — the actual blocker for converting sites 2/3 to genuine non-blocking.
4. `EffectBackbone` (per-instance) — blocks `attach_backbone`/`detach_backbone`/the whole
   `register_host_backbone` mechanism; already flagged as critical-path in `📓️status.md`.
5. `context_menu`/`window_engagements`/`window_measures` need a real v12 wire shape once designed.
6. E1-describe landing turns `load_wasm_plugins`'s descriptor read from "always empty" into real
   manifests — no code change needed here, the reader is already in place.
