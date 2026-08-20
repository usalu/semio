# Get `s` Working Again — Unified Program (Async Runtime × Semantic UI), All Plugins, End to End

## Context

`s` (the semio app: 33 plugins + 26 extension crates on 59 component-guest fleet crates, react + wgpu-web + wgpu-native renderers) does not run end to end. Two migration programs ran concurrently on this live tree and both are now this session's responsibility (the user: "You are the only one left to perfectly plan and coordinate everything"):

1. **MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME** (ticket `☀️17`): pooled-actor microkernel, universal async + zero first-party dyn (Gate 1 passed), ONE async WIT `world actor` (landed, artifact-verified `[async-lift]` on all 7 exports), native `AsyncPluginRuntime` rewritten+mounted, web shard runtime + jco GO-jspi bridges, rulings R1–R22 + E1–E5 binding in `📌️important.md`.
2. **SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY** (ticket `☀️20`): retained-mode UI — `semio-framework-ui-contract` (flat `UiNodeRecord` table, `BuiltNode` DSL, `ActionId`/`Trigger`/`UiIntent`, `PresenceUpdate`), `-ui-runtime` (`ComponentTree`/`TreeNode`, `SurfaceReconciler`→`UiPatchOp`, `PresenceHub`, `transact()`), `-ui-render` (+4 GPU backends), `-ui-host`. W0–W3 done-verified, 300 tests. React renderer already consumes the new contract. `sdk-flip` landed.

**Mission (user mandate):** everything collaborative, interactive, non-laggy, non-blocking (plugin IO included), plenty of plugins with plenty of extensions, handcrafted clean mechanisms (no ad-hoc patches), long-term decisions regardless of effort, end to end — except the legacy root `compose/` tree (O3). Maximize parallel agents.

**Workforce:** this plan = Fable 5 · coordination = main chat **Opus 5 High ("sol")** · execution = parallel **Sonnet 5 High ("terra")** · read-only exploration = parallel **Haiku 4.5 ("luna")**. Same live tree, auto-commit bot, no branches/worktrees, no git-modifying commands ever.

## Verified current state (measured 2026-08-20 14:30–14:36 at HEAD `bd1ce10b9b`, not from stale reports)

| check | result |
|---|---|
| `semio-framework-plugin --lib` (SDK) | **EXIT 0** — the sdk-flip residue is FIXED; native SDK is GREEN |
| `semio-framework-plugin-host --lib` | **EXIT 0** — the earlier rustc ICE does NOT reproduce |
| os-kernel `--lib` + tests | EXIT 0 · **779/0**; kernel-db 424/0; framework 160; actor 70; graph 174; 3d 62; number 97; services 30; ui family 300 |
| `semio-s-plugin-note --lib` | aborts in **`semio-s-plugin-stdio`: 18,757 own errors** (92% async-shape: E0271 7233 · E0277 5179 · E0308 3894; only 352 UI-vocab lines). Concentrated in per-format `🔺️diff` schema modules under `🗿️artifacts/` (gltf 1797, dxf 769, docx 643, document 573, presentation 509, brep 436, root 407, xlsx 389). These are COMMITTED taxonomy leaves, not generated — no regeneration shortcut; the span-keyed fixpoint + hand-judgement path continues (44,102 → 18,757 so far) |
| `cargo check --workspace` | EXIT 101 at os-kernel: **133 errors under feature unification** — 127 in `🏪️store/🔄️sync/🦀️component.rs`, 6 in `📇️directory/🔌️client`. Cause: `sync` feature (tokio-tungstenite/notify/rusqlite) is non-default; NO current gate ever compiles that module |
| Not re-verified since the SDK last changed | SDK wasip2 `component-guest`/`component-extension-guest`, `--all-features`, dropped-future census (R17: newly-green ⇒ census owed) |

Structural facts that shape sequencing (verified):
- SDK `Cargo.toml` already declares `semio_framework_ui_contract` + `ui_runtime` path deps NEXT TO `ui_wgpu` (line ~47). The `ui_wgpu` dep cannot be deleted until the WindowKit region stops referencing it.
- **34 fleet Cargo.tomls depend on `semio-s-plugin-stdio`; ~25 crates do NOT** → those can go green in parallel with stdio.
- `ui-render` is NOT in the SDK dependency graph → render work parallelizes with SDK work under R19.
- vite `_shard` fix is in (`🧑️‍💻️dev/⚙️vite.config.ts:69`); ~207 materialized web bridges are ALL STALE (pre-async-world); bridge GENERATOR is updated and validated against a real jco probe; regeneration waits only on fleet wasm artifacts.
- Descriptors: 30/33 on disk, 13 ratcheted, 6 more plugins code-complete via `.declare_artifact`.
- NOT done from prior waves (no reports — treat as open): `KernelAsyncRuntime` (0 refs; `ParallelRuntime` live, 40 refs), winit block_on removal (~28), services http-hyper + TimerWheel Send root fix, pack-waker, extension activation host path, os-mcp feature mapping, run-through-kernel completion.
- Census: `block_on` 953 (framework 820 — mostly tests, fleet 133); `PluginWorkerClient`/TS `exchange` 0 live; UI-vocab hits in fleet ~2,680 across 803 files (stdio 890/352 files, norm 262/121).
- Fleet UI migration recipe exists: `☀️20/📓️recipe-plugin.md` + amendments in `📓️terra-sdk-helpers-report.md` (DslValue→UiValue fold, no optional-label constructors, WindowKit shared-trait warning).
- Bench budget 5 (interactive p95 ≤ 8 ms native under 40 cpu actors) genuinely fails at 140.9 ms — root cause diagnosed (see Mechanism 3).

All rulings **R1–R22, E1–E5, O1–O4** in `☀️17/📌️important.md` remain binding on every packet. Key ones for this phase: R9 (E1 transitivity), R10 (span-keyed only), R12/R17 (forced-rebuild dropped-future census the turn a crate goes green), R14 (name compiled targets; native-only acceptance invalid for cfg/guest code), R16/R20 (`insert-await.py` audit; never unattended), R19 (never dispatch an editing packet into a live packet's dependency graph), R21 (own-error counts invalid if upstream aborted), R22 (live peer work is never "fixed").

---

# PART 1 — Design of record: the six handcrafted mechanisms

These are the mechanisms that make `s` collaborative, interactive, non-laggy, non-blocking. Designed against the working tree at `bd1ce10b9b` (all path:line references verified). sol copies this whole plan into the ticket at kickoff as `📓️design-unified.md`; each mechanism decomposes into the packets named in Part 2.

## M1. UiIntent dispatch end to end (interactivity)

Current: renderer builds real intents (`🖱️ui/🖼️render/…/🦀️dispatch.rs:32-36` ListenerSet carries ActionBindings + addressing); host seam is a stub (`📺️renderer/…/🎯️targets/🧊️wgpu/🦀️kernel_seam.rs:14-24`, `default_intent_exchange` echoes back, no kernel round trip); WIT `ui-intent-event` exists (`📜️component.wit:662`); reactor decodes but only marks dirty (`⚛️reactor/🦀️component.rs:402-415`); intended pattern lives in `ui-runtime` `🦀️dispatch.rs` (`HandleIntent`, `is_stale_intent`, `DEFAULT_REVISION_TOLERANCE=1`) and `🦀️transaction.rs:276-305`.

Design (do NOT embed full `UiRuntime<S,D>` per actor — reuse its vocabulary at the reactor seam, funnel into the existing typed command dispatch):
- **Host router:** `SurfaceRouter { by_surface: HashMap<SurfaceId, instance> }` in kernel_runtime (surfaces are `"<instance>:<body-key>"` today — `⚛️reactor:539-544`); `kernel_intent_exchange` wraps the intent in `Envelope{ lane: Lane::Interactive, payload: Event::UiIntent }` → `Kernel::submit` → normal tick/grant path (exactly the lane budget 5 measures).
- **Reactor dispatch arm:** revision-guard at the reconciler that owns the revision (new accessor `PatchTracker::revision(surface) -> UiRevision`), drop stale intents per `is_stale_intent(..., DEFAULT_REVISION_TOLERANCE)`, batch per instance, then `plugin_runtime::plugin_dispatch_intents(instance, &intents)` next to the existing `plugin_exchange` batching (`:471-497`), route resulting frames through the SAME `route_app_frame` path.
- **SDK handler:** new `pub async fn plugin_dispatch_intents(...) -> PluginExchangeOutput` in `plugin_runtime` (`🔌️plugin/🦀️component.rs:15446` module): stamp `ActionMeta` (surface, node_key, seq, instance actor id from `INSTANCE_ACTORS:15484`); resolve typed command via new trait hook with compatible default: `ArtifactApp::command_from_intent(intent)` → default bridges to existing `command_from_action(&intent.action.name, merge_ui_values(args, input))` (`:9819-9825`); dispatch through existing `PluginApp::handle_command_frame` (`:10101-10106`) so view/shell action-kind discipline, command log, undo groups, AsyncTask spawning all apply unchanged.
- Args resolved ONLY in the guest (renderer echoes `ActionBinding::args` verbatim — already the contract, `🦀️action.rs:112-118`); version mismatch on `ActionId.version` → reject with Fault (decided; never silently dispatch). No new reply channel: the reply IS the next `UiPatch` revision bump + existing `PatchAck`; faults ride `AppFrame::Error`; `intent.seq` is the dedupe/log key. `ActionBinding{trigger, action: ActionId{scope,name,version}, args}` fully replaces old `ActionDescriptor`.

Acceptance (runtime): Activate-intent on a counter instance produces the mutation's UiPatch within the same poll turn + command-log entry; stale-by-2 intent produces NO patch and NO command; View-kind action returning artifact ops hard-faults; native click → `KernelSeam::submit_intents` → patch drained via `drain_outcomes` with a real waker (no Poll spin).

## M2. Presence / collaboration pipeline (the collaborative heart)

Current: SDK DROPS selection/hover/peer marks (`🔌️plugin/🦀️component.rs:12103-12130` threads `state` but `let _ = state;`; `PanelTreeBuilder:5539-5550` records but can't render). Contract+hub complete (`🦀️presence.rs:78-90` `PresenceUpdate{surface,node_key,own,peers,ttl_ms}`; `ui-runtime/🦀️presence.rs:63-121` `PresenceHub` record/expire/flush with coalescing). WIT `turn-result.presence` slot exists but is documented as replication `PresencePeer` while the reactor gap note says `PresenceHub` — contradiction. Roster plumbing exists on the document plane (`adopt_presence:10161-10169` sole peer ingress; `ephemeral_snapshot:12793` outbound). React `UiPresenceOverlayContext`/`usePresenceOverlayEntry` exists and must never write the document store.

Design — two planes, one derivation point:
- **Plane 1 (unchanged): collaboration truth.** Typed `A::Presence` + `InteractionState` replicate via the backbone roster (ephemeral_snapshot → hub → peers' `AppCommand::Presence` → `adopt_presence`).
- **Plane 2 (new): render-plane presence derived per turn.** `AppInstance.pending_presence: Vec<ui_contract::PresenceUpdate>` outbox; `stamp_and_cache_interaction_ui` (already walking every presented tree with `state` threaded — the exact spot its doc names as the future publisher) derives own selection/hover + `PeerMark`s from `peers_selecting`/`peers_hovering` (`:8336-8360`); `PanelTreeBuilder`'s recorded ids feed the same derivation. New `plugin_take_presence(instance)` drains the outbox. Reactor holds a per-actor `PresenceHub` thread-local next to `PATCHES`; after each dirty render: record into hub; once per poll: `hub.expire(now); result.presence = hub.flush()` (free burst coalescing).
- **Wire (DECIDED):** add `presence: Vec<ui_contract::PresenceUpdate>` to `kernel::TurnResult`; **repoint WIT `presence-update` to carry pack-encoded `ui_contract::PresenceUpdate`** (render-plane), NOT replication `PresencePeer` — the turn-result's consumer is the renderer (node_key-addressed, TTL-scoped); the roster already has its own channel and shipping it per turn is strictly worse. `PRESENCE_TTL_MS` default 4000.
- **Host fan-out:** native `KernelThreadState::apply_turn_result` → per-surface presence store in ui-render (mirrors React shape); web ShardClient turn results → provider owning `UiPresenceOverlayContext`, TTL timers. Neither ever touches the document store (no document revision per mouse-move — patch minimality preserved).

Acceptance (runtime): selecting in instance A yields `PresenceUpdate{own.selected}` with ZERO ui_patches when only presence changed; two instances over loopback backbone: B's selection appears as a PeerMark in A's turn result after one roster push, and ages out via ttl_ms after killing B (no goodbye message); N hover moves between polls coalesce to one update per (surface, node_key).

## M3. Interactive latency — bench budget 5 (p95 ≤ 8 ms native, currently 140.9 ms)

Root cause (diagnosed): with 2,550 actors, the interactive actor's shard also receives several of the 40 cpu grants in the same tick (`grants_per_tick: 64` covers all 41); `ShardLoop::pump` executes grants FIFO (`🧵️shard/🦀️component.rs:193-228`); nothing preempts a running wasm turn; `ShardFrame::Grant` carries no lane. Head-of-line blocking WITHIN a shard — DRR grant-ordering cannot fix it; pin-time saturation avoidance only helps at activation.

Design — three independently measurable pieces, in order:
1. **Lane-priority execution inside the shard** (smallest change, biggest win): `ShardFrame::Grant` gains `lane: Lane`; `ShardLoop` keeps two queues `[interactive+user-visible, background+maintenance]`; pump drains interactive exhaustively before any background grant and re-checks the transport between background turns.
2. **Epoch preemption bounds a single background turn:** `execute_turn` arms wasmtime epoch deadlines from `budget.wall_ms` (Grant already carries the DRR budget; bridge `turn_budget_from_grant:126-128`); background turn hitting its epoch yields `TurnStatus::MoreWork`, re-granted next tick. Worst-case interactive delay = one epoch tick (1 ms ticker), not one unbounded turn.
3. **`KernelAsyncRuntime` with reserved interactive workers** replaces `ParallelRuntime` behind the same facade (`activate`/`submit`/`tick_and_dispatch`/`complete`/`unregister`), as `🎯️targets/🧊️wgpu/🚄️async_runtime.rs`: unchanged `🎭️actor` DRR kernel; `interactive_rt` (2 tokio workers, nothing else runs there) + `bulk_rt` (N−2, cpu turns + job steps); each actor an owned async task (GuestInstance moved in — pinned-never-shared preserved as task affinity); one outcomes mpsc (deletes the forwarder threads). Decision staged: measure pieces 1+2 first (they remove the FIFO head-of-line term and should collapse p95 by an order of magnitude); adopt piece 3 for residual variance + as the home for job stepping. `🎭️actor` stays pure (no tokio/clock/transport in it).

Acceptance (runtime): `budget_4_and_5` p95 ≤ 8.0 ms with roundFaults 0 and all 41 outcomes per round (no cpu starvation); existing interactive-isolation + backpressure tests still pass; a background turn over budget observably yields MoreWork; a mid-burst interactive grant completes < 8 ms in a shard-level test.

## M4. Non-blocking plugin IO adoption (fleet-wide recipe)

Current: fleet block_on ≈ 133 (flow 61, cad 47, stdio 17, process 13, animate 3, block 3), dominantly `stdio::…::brep::schema::engine::block_on` inside sync reducers (e.g. `📐️cad/…/🚪️io/🦀️component.rs:356,457,496`). Sanctioned await surfaces exist: `AsyncTask` ("the ONLY way plugin code awaits", `:9000-9028`, keyed latest-wins, restartable), `TaskCtx.host.{http_fetch, storage_read, spawn_job}`, cold jobs runtime (`💼️jobs:141-369`: `register_job_kind`, `JobCtx::{tick, progress, checkpoint, budget}`, start/step/cancel, stall guard). `JobProgress` currently DROPPED at `⚛️reactor:446`. 41 `DownloadMediaExport` payloads built on the turn path.

Canonical recipe (per call-site class):
- **A — host IO in a reducer:** reducer stays pure, returns `Emit{ tasks: [AsyncTask::new(kind, |ctx| async { … ctx.host.http_fetch(…).await … Ok(TaskResolution::Command(..)) }).keyed(..).restartable(..) ] }`. Result re-enters `handle` as a typed command against current state.
- **B — CPU-heavy compute (tessellation, STEP, FEM, energy, video):** `register_job_kind` cold job; inside the JobFn the formerly-block_on'ed engine futures become REAL awaits interleaved with `ctx.progress(..)`, `ctx.checkpoint(..)`, `ctx.tick()` slice boundaries; reducer hops via `spawn_job(kind, input, JobPlacement::Isolated)` task. This is what deletes `engine::block_on` (stdio's exporter drops the export when the last consumer migrates).
- **C — progress + cancellation to the UI:** stop dropping `Event::JobProgress` — route into the owning instance's TRANSIENT lane via `plugin_note_job_progress(instance, job, bytes)` + dirty_render (progress bars/`Activity::Loading`; no document revision for progress numbers). Cancel = UI intent (M1) → typed command → task calling `ctx.host.cancel_job(job)`; supersession free via keyed tasks.
- **D — DownloadMediaExport (41 sites):** never build export bytes in `handle`; reducer → task → `spawn_job("semio.media-export/<kind>")` → on Done, follow-up command emits the effect. Exports > 4 MiB: job `storage_write`s the blob, effect carries the storage key (respects `max_patch_bytes`/`max_effects`).
- Enforcement: CI grep gate — `block_on` forbidden under `✏️s/🔌️plugins/**` with a shrinking allowlist. Migration order: **stdio (owns engine::block_on) → cad → flow(+ext) → process → animate → block**.

Acceptance (runtime): large STEP import completes via N>1 step_job slices with monotonic progress and exactly one document edit; mid-import cancel frees the slot, no mutation lands, progress entry clears; checkpoint→restore mid-job resumes from job checkpoint bytes; with a 100 MB export running, an interactive intent on the same instance still meets its lane budget; fleet block_on grep = 0.

## M5. WindowKit / scene resolution

Current: `WindowKit::render` is one shared trait method returning old `ui_wgpu::wgpu::UiNode` (`:13297-13303`); 3 kits (Text/Table/Mesh) build `ComponentScene` payloads; 15 product scene structs still live in the OLD wgpu target (`🖱️ui/📦️…/🎯️targets/🧊️wgpu/🦀️component.rs:3076-3880`); `🖱️ui/🎬️scene/🦀️component.rs` currently holds only generic 3D math; `SurfaceProps` in `🦀️surface.rs` is an admitted scaffold.

**Decision: land scene + SurfaceProps first, then flip the trait ONCE. Do NOT split the trait** (a split would freeze the retiring UiNode into four kits' API and touch all seven kits twice).
- **Scene crate:** move the 15 product scene structs into `🖱️ui/🎬️scene` (`semio-framework-ui-scene`, depends on ui-contract only, wasm32-safe); generic 3D math becomes a `math` region of the same crate. Each struct: `trait SceneDoc: Serialize+DeserializeOwned { const SCHEMA: &'static str }` ("world3d@1", …), pack-encoded via the standard wire-value path.
- **SurfaceProps contract (replace scaffold wholesale):** `{ kind: SurfaceKind, doc_schema: String, doc: SurfaceDoc /*opaque pack bytes*/, bindings: Vec<ActionBinding> }` + `encode<T: SceneDoc>`/`decode<T>` helpers living in ui-scene. Rules: unknown doc_schema → placeholder surface + logged fault (never panic, never dropped patch); scenes above `max_patch_bytes` threshold reference retained GPU resources by handle; doc bytes diff as an opaque blob.
- **Trait flip:** `async fn render(view) -> BuiltNode`; Tree/Image/Document/Media translate to plain components; Text/Table/Mesh return `Component::Surface(SurfaceProps::encode(kind, &scene))`.
- **Consumers:** GPU — `AnySurface`/vtable registry in ui-render maps doc_schema → painter (port the existing wgpu scene renderers); React — `Interpreter`'s `resolveComponentSceneHost(kind)`, decoding `SurfaceDoc.bytes` via a thin wasm pack-decoder exposed from existing jco glue (single encoding; no dual JSON — decided).

Acceptance (runtime): all seven kits render in both wgpu host and React shell smoke; `doc_schema:"world3d@99"` renders placeholder + fault while the surrounding tree still patches; encode→wire→decode round-trip byte-equal native + web.

## M6. Extension activation (descriptor-driven cascade, kernel-owned)

Current: `ActorKind::Extension{plugin, extension_id}` exists with pack codec + kernel tests (`🎭️actor:337-367, 3095-3131`) but NO host constructs it — the only native activation site is `KernelThreadState::create_app` (`🎯️targets/🧊️wgpu/📦️glue.rs:348-373`), PluginApp only. Descriptors carry `role:"extension"`, `extends`, `executionMode`, `dependsOn` (`📇️registry/🤖️generated/🔣️plugins.json`); builder asserts `ExtensionManifest.extends == dependencies[0].plugin_id` (`:17251-17330`); `ActivationEvent::OnExtensionRequest{point}` and `MessageEndpoint::Extension{id}` already exist.

Design:
- **`ExtensionIndex`** built from the installed descriptor set (registry JSON natively, manifest store on web): `by_parent: HashMap<plugin_id, Vec<ExtensionRecord{extension_id, package, execution, capability_requests}>>`. `.sxt` install/uninstall mutates the index and hot-attaches/detaches if the parent is live (hot-attach while parent suspended: queue until resume — decided).
- **Kernel additions (in `🎭️actor`, so both platforms inherit):** `activate_pinned(..., shard: ShardId, ...)` — extensions pin to the parent's shard (parent↔extension messages never cross a transport; `Isolated` still means own store, not own shard); `link_extension(parent, child)` + cascade — deactivate/suspend/kill/checkpoint cascade leaves-first over the explicit edge table.
- **Native cascade:** `create_app` tail loops `index.extensions_of(plugin_id)`, compiles cached, `activate_pinned` with `ActorKind::Extension`, `Lane::Background` default (re-laned on SurfaceVisible), `ActivationEvent::OnExtensionRequest`, and **`scoped_grants` = intersection of parent's granted set with the extension's requests — an extension can never hold a capability its parent lacks**; broker revokes transitively on parent deactivation.
- **Web mirror:** `ActivationRegistry.activate` consults the same index shape, per-extension `shardClient.activate` with parent affinity, local link records, symmetric cascade (restore: parent first).

Acceptance (runtime): activating cad brings up its N extensions on the SAME shard (kernel metrics assert); bench budget-3's 50×50 fixture passes through the REAL cascade path; parent deactivation leaves zero orphans; a trapping extension is restored/killed without faulting the parent (and vice versa); an over-asking extension activates with the grant absent (observable broker denial).

Cross-cutting: M1+M2 share the reactor-embeds-hub/tracker pattern and land together; M3 pieces 1+2 land early (every intent rides the lane they fix); M4 is independent; M5 gates only the WindowKit flip (which gates cad/mesh fleet UI migration); M6 depends only on its kernel packet. One shared SSOT edit to coordinate: `kernel::TurnResult` gains `presence` in the same window as any `kernel_turn_result_to_wit` change, and the WIT presence-update repoint is a sol-registrar schema edit.

---

# PART 2 — Program mechanics

## Kickoff (sol, first turn — packet `absorb-ticket`)

1. `ticket_reopen` with EXPLICIT path `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`.
2. Liveness probe (`git log --date=iso` + mtimes on SDK/store/renderer files, checked twice) → formally dissolve the R22 exclusion zone in `📓️status.md` (sole-coordinator condition, per the user).
3. Copy this plan into the ticket as `📓️design-unified.md`. Copy `☀️20/📓️recipe-plugin.md` (+ the sdk-helpers amendments) into `☀️17` as `📓️recipe-plugin.md`.
4. Ratify into `📌️important.md`: **E6** (= ☀️20's U1: `ui-contract`/`ui-runtime`/`ui-render` frame construction + input dispatch are literal sync `fn` by decree; async only at boundaries; untagged violations in either direction are defects); the M2 wire decision (WIT presence-update = render-plane `ui_contract::PresenceUpdate`); the new slugs below (collision-audited); the R19 test-only ruling for `sdk-tests` (sol decides in writing whether test-only edits to a crate a live packet depends on are exempt — recommendation: NO exemption, sequence it).
5. Append ABSORBED banner to `☀️20/📋️master.md` + `📋️packets.md` pointing at `☀️17`; `ticket_close` `☀️20` with its explicit path (ASCII-first files array; `📌️important.md` there emptied first if non-empty).
6. `df -h` → build-concurrency ruling: default ONE cargo build at a time through sol's queue; ≤3 concurrent with per-slug scratchpad target dirs only if free disk ≥ ~150 GB. TS lanes exempt.
7. Re-measure the SDK's unverified gates (wasip2 ×2 features, `--all-features`, forced-rebuild dropped-future census per R17) — these are GATE S′ prerequisites and take minutes on the warm dir.

## Packet registry

Owner tiers: sol = coordinator/registrar/acceptance-builds · terra = Sonnet executor (code + cheap checks only, acceptance UNRUN) · luna = Haiku read-only. `PLUGIN/` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`, `UI/` = `🧰️framework/🔨️modules/🖱️ui`. All scopes exclusive; registrar files (root Cargo.toml/lock, `📜️script.ts`, launch.json, `🤖️generated/**`, vite config, Shell/ShellHost) via lease-request only.

| Packet | Owner | path_scope | Deps | Size |
|---|---|---|---|---|
| `absorb-ticket` | sol | tickets/registrar | — | S |
| `oskernel-sync-features` | terra | `💻️os/🔨️modules/🏪️store/🔄️sync/**` + `📇️directory/🔌️client` residue (133 errs) | — | M |
| `ui-w4-core` | terra | `UI/🧬️contract/**` + `UI/🧠️runtime/**`: field-targeted UiPatchOp setters (SetStyle/SetAccessibility/SetBindings/SetMenu), ImageBuilder typestate, PresenceHub polish | — | L |
| `scene-surface` | terra | `UI/🎬️scene/**` + `UI/🧬️contract/…/🦀️surface.rs` (M5 packets A+B: scene crate move, SceneDoc, SurfaceProps final) | — | L |
| `ui-render-w4` | terra | `UI/🖼️render/**`: DispatchTree carries parent/overlay/listener (lossy `From<Vec<Hitbox>>` dead), input replay conformance | ui-w4-core | M |
| `react-w4` | terra | the 16 TS files (react renderer, window-kit TS, barrels) — TS lane | — | M |
| `pack-waker` | terra | `🎒️pack/⏳️async` + `🎒️pack/🌐️http` wakers + R8 pack async_trait removal (8 sites) | — | S |
| `services-async` | terra | `🛎️services/**` + `🌎️hub/📇️directory/**`: TimerWheel arm/disarm/armed_count R9 root fix, hyper+rustls behind `AsyncHttpTransport`, ureq out, R8 directory half | oskernel-sync-features | M |
| `luna-probes` | luna ×2 | read-only: (a) exact non-stdio-dependent fleet crate list; (b) stdio error taxonomy refresh → worklist; (c) census baseline JSON (block_on prod/test split, vocab hits, dropped futures); (d) cargo-tree edges for every planned co-dispatch (R19 pre-check); (e) `Operator` dyn-tail check (~138 impls — confirm still open, size it) | — | S |
| `sdk-wire` | terra+sol | **ATOMIC quiet window** over `PLUGIN/**` (root, `⚛️reactor/**`, `🌐host`, `💼️jobs`, `🏗️builder`) + `🎭️actor` lane field + `🎠️kernel` TurnResult: M1 stages B+C, M2 SDK derivation + reactor hub + `TurnResult.presence` + WIT presence repoint (sol registrar edit), M4-C progress routing, M3 piece 1 (Grant.lane + two-queue pump, same-crate wire bump) | ui-w4-core, scene-surface (for types only) | XL |
| `windowkit-flip` | same terra, same window | M5 packet C inside `PLUGIN/` + then sol lease: DELETE `ui_wgpu` dep from SDK Cargo.toml | scene-surface, sdk-wire | M |
| `stdio-vocab-offline` | terra | `✏️s/🔌️plugins/🗄️stdio/**` — recipe codemods, ZERO compiles (runs inside the quiet window) | recipe | M |
| `fleet-vocab-offline` | terra | `✏️s/🔌️plugins/**` minus stdio — recipe codemods offline | recipe | L |
| `stdio-green` | terra ×1 driver + ≤2 residue helpers on disjoint file lists | `✏️s/🔌️plugins/🗄️stdio/**` — 18,757 → 0 (fixpoint + E-class judgement + R10 hand shapes; R16 audits; R17 census at green) | GATE S′, stdio-vocab-offline | XL |
| `fleet-green-early` | terra ×1–2 | the ~25 non-stdio-dependent crates | GATE S′, fleet-vocab-offline | L |
| `kernel-async-native` | terra | `🎯️targets/🧊️wgpu/🎠️runtime.rs` + `🚄️async_runtime.rs` (new) + epoch ticker: M3 pieces 2+3, ParallelRuntime deleted, budget-5 groundwork | GATE S′ | XL |
| `surface-router` | terra | `📺️renderer/…/🧊️wgpu/🦀️kernel_seam.rs` + kernel_runtime router (M1 stage A native) + presence fan-out (M2 host half) | sdk-wire | M |
| `mcp-features` | terra | os-mcp feature mapping | GATE S′ | S |
| `fleet-green-a..e` | terra ×≤5 | partitions of the 34 stdio-dependent crates (cad/flow heavies balanced; demonstrator last) | GATE D, fleet-vocab-offline, windowkit-flip | L each |
| `winit-unblock` | terra | `🧊️wgpu/📦️glue.rs` (~28 block_on, ureq:1577, sleep:2604), `ProgramBridge`; Shell via lease | kernel-async-native | M |
| `run-through-kernel` | terra | `🏃️run/**` + `💻️os/🖥️host/**` kernel activation facade | kernel-async-native | M |
| `extension-activation` | terra | M6: `🎭️actor` kernel packet (activate_pinned, link table, cascade) then native `ExtensionIndex`/create_app cascade/scoped_grants; web mirror after web-rematerialize | kernel-async-native; kernel half can start at GATE S′ | L |
| `descriptor-close` | terra + sol builds | describe runs, 7 missing via `.declare_artifact`, ratchet 13→33 (SDK constant = one-line lease), wasip2 rebuild all 59 artifacts — pipelined per green partition | stdio-green / fleet-green-* | L |
| `web-rematerialize` | sol (generated) + terra verify | ~207 bridges re-materialized fresh, hash-checked | descriptor-close | M |
| `sdk-tests` | terra | SDK `#[cfg(test)]`/`--all-targets` residue (~378–536) + named-set baseline | GATE D + sol's R19 ruling | L |
| `adopt-a..f` | terra ×≤5 | M4 per partition: fleet block_on→0, register_job_kind per CPU-heavy plugin, 41 dl_export→jobs, pending_effects→0 | fleet-green-* (pipelined) | M each |
| `framework-test-adoption` | terra | framework ~820 block_on (tests → `#[async_test]`), suites for newly green crates, R17 censuses | GATE C-u | L |
| `e2e-web-boot` | terra | dev boot: hub loads, turn round-trip worker→wasm poll→turn-result, http-fetch through shim (Chrome JSPI) | web-rematerialize | M |
| `e2e-native-smoke` | terra | 33/33 through 🏃️run THROUGH the kernel | run-through-kernel, descriptor-close | M |
| `collab-e2e` | terra | two-client presence scenario (M2 acceptance) | sdk-wire, e2e-web-boot | M |
| `checkpoint-e2e` | terra | edit → checkpoint → kill shard worker → FailurePolicy restore → state intact, web AND native | e2e-web-boot, e2e-native-smoke | M |
| `parity-rebaseline` | terra+luna | 58/58 react vs wgpu re-baseline (same architecture both sides, first time) | GATE W-u | M |
| `bench-ladder` | terra | 8 budgets × 3 renderers + `budget_50x50_activate` (through the REAL extension cascade) | GATE W-u | M |
| `budget5-interactive` | terra+sol | conditional: if bench still > 8 ms after M3 pieces 1–3, dedicated design packet; bar renegotiated only by explicit owner decision | bench-ladder | L |
| `census-zero-exit` | luna+sol | exit checklist, census JSON vs baseline, `[DEBUG]` sweep, launch.json regen (lease), `📌️important.md` emptied LAST, `ticket_close` explicit path | all | S |

## Wave DAG (maximum safe parallelism; ≤6 live executors, builds through sol's queue)

```
W0  absorb-ticket (sol)  + SDK gate re-measure
W1  oskernel-sync-features ∥ ui-w4-core ∥ scene-surface ∥ react-w4 (TS) ∥ pack-waker ∥ luna-probes ×2
      — pairwise-disjoint dep graphs; nothing edits the SDK yet; R19-clean
      [services-async backfills as slots free]
        ── GATE K ──
W2  ═══ SDK QUIET WINDOW (atomic, one owner + sol registrar) ═══
    sdk-wire → windowkit-flip (sequential, same owner)
      ∥ ui-render-w4 (ui-render ∉ SDK graph)  ∥ services-async (∉ SDK guest graph)
      ∥ stdio-vocab-offline ∥ fleet-vocab-offline (zero compiles)
      ∥ react-w4 finish (TS)
        ── GATE S′ ──
W3  stdio-green (1+≤2)  ∥ fleet-green-early (~25 crates)  ∥ kernel-async-native
      ∥ surface-router ∥ mcp-features ∥ extension-activation (kernel half)
        ── GATE D ──
W4  fleet-green-a..e (≤5 partitions)  ∥ winit-unblock ∥ run-through-kernel
      ∥ extension-activation (native half) ∥ sdk-tests (per sol's R19 ruling)
      [descriptor-close pipelines per green partition through sol's build queue]
        ── GATE C-u ──
W5  descriptor-close finish → web-rematerialize (sol)
      ∥ adopt-a..f (pipelined) ∥ framework-test-adoption ∥ R17 censuses (continuous)
        ── GATE R-u ──
W6  e2e-web-boot ∥ e2e-native-smoke → collab-e2e ∥ checkpoint-e2e
      ∥ extension-activation (web mirror) ∥ parity-rebaseline ∥ bench-ladder
      → [budget5-interactive if red]
        ── GATE W-u ── GATE F-u ──
W7  census-zero-exit → EXIT ladder → ticket_close
```

## Gates (concrete; `$T` = scratchpad CARGO_TARGET_DIR, 600000 ms timeouts, negatives re-verified with python over emoji paths, targets NAMED per R14)

- **GATE K:** workspace `--lib` check clears os-kernel (sync feature module compiles); os-kernel 779/0 unchanged; ui-contract+ui-runtime+ui-render suites ≥ 300, patch fuzz corpus byte-identical Rust↔TS; `tsc --noEmit` clean on react-w4 projects.
- **GATE S′** (end quiet window — the fleet unlock): SDK `--lib` EXIT 0 · `--all-features` EXIT 0 · wasip2 `component-guest` AND `component-extension-guest` EXIT 0; python vocab grep `UiNode|ActionDescriptor|UiPresence` over `PLUGIN/**` = 0; forced-rebuild dropped-future census = 0; `ui_wgpu` dep DELETED from SDK Cargo.toml; two-mock-client presence integration test green; stale-intent test green; os-kernel/kernel-db/plugin-host test ladder unchanged.
- **GATE D:** stdio `--lib` EXIT 0 native AND wasip2; vocab grep stdio = 0; the ~25 genuine E0382 move-bug residue itemized and closed; R17 census at green; os-kernel 779/0 held.
- **GATE C-u:** `cargo check --workspace --all-targets` EXIT 0; full test ladder (os-kernel 779 · kernel-db 424 · framework 160 · actor 70 · graph 174 · 3d 62 · number 97 · services 30 · plugin-host 125+ · ui family 300+ · SDK suite named-set baseline); dyn census (236 traits) = 0; async-literal census ≥ 98% with E-tags; repo-wide vocab grep 0 outside ticket folders.
- **GATE R-u:** describe emits for async 🗒️note; descriptors 33/33, ratchet 33; note executes a turn through the mounted async runtime; cancellation-drops-Store + S1b/S1c preemption green.
- **GATE W-u:** dev boot loads `s`; plugin turn round-trips worker→wasm poll→turn-result; http-fetch resolves through the shim; native smoke 33/33 THROUGH the kernel; extension install→activate→cascade round-trip native; all ~207 bridges hash-fresh; banned symbols still zero.
- **GATE F-u:** census vs W1 baseline: production block_on = 0 minus R4 allow-list (prod/test split per R4 clause 5); pending_effects 0; register_job_kind > 0 per CPU-heavy plugin; 41 dl_export in jobs; R17 = 0 on every green crate.
- **EXIT:** parity 58/58 · bench 8×3 incl. budget 5 p95 ≤ 8 ms + budget_50x50_activate through the real cascade · collab-e2e (two clients, presence cross-visible, TTL expiry) · checkpoint-e2e (web AND native) · zero warnings native+wasip2+wasm32-unknown · launch.json regenerated · `[DEBUG]` sweep · `📌️important.md` emptied last · `ticket_close` explicit path + file list (ASCII-first array).

## Critical path & how it's shortened

CP: absorb → GATE K prereqs (ui-w4-core/scene-surface) → **sdk-wire+windowkit-flip** (the quiet window, now M–L not XL since the SDK is already green) → **stdio-green** (XL, the boulder) → longest fleet partition → descriptor-close tail (build-serialized) → web-rematerialize → e2e → exit. Shorteners already encoded: offline vocab codemods hide inside the quiet window; stdio residue shards to 3 agents on disjoint files; ~25 non-stdio crates go green in stdio's shadow; kernel-async-native (the other XL) runs entirely off-CP; descriptor builds pipeline per partition instead of barriering; luna worklists mean executors start editing, not exploring; sol keeps one warm acceptance target dir.

## Workforce protocol (binding, distilled from R1–R22 + this session)

- Executors write code + cheap checks; **sol runs every acceptance build** foreground with explicit 600000 ms timeouts (subagent background builds die at turn boundaries). `run_in_background: false` for executor Agent calls when sol spawns several per message.
- Every brief carries: the standing-rules block, the packet's path_scope, the 779/424/125-test regression floor, "a false green is the worst possible outcome", the R16 post-`--apply` audit, the no-whole-file-scripts rule (16k-line incident) with the `wc -l` guard, and named repaired files (R20).
- Before every wave dispatch: luna cargo-tree probe of each co-dispatch against live packets' dependency graphs (R19). Atomic packets are never interrupted (rule 25).
- Baselines are NAMED SETS, never counts. A green compile is not evidence of behavior — every newly green crate gets a forced-rebuild dropped-future census (R17) plus the `let _ =` grep before it is called done.
- Liveness before touching any contested file: mtimes + `git log --date=iso`; live peer work is escalated, never repaired (R22); abandoned wreckage is fixed.
- Scratch files: `.txt`/`.md`/`.json` in the ticket folder only; CARGO_TARGET_DIR always in the session scratchpad (EPERM in ticket folder); prune incremental/ between fleet builds.

## Risk register (owned)

1. **stdio fixpoint stall** — recovery tools exist in the ticket folder; shard residue to 3 terras; any class > 2k surviving 3 rounds → named design packet.
2. **Budget 5** — M3 pieces 1+2 first (expected order-of-magnitude drop), piece 3 for residue, conditional `budget5-interactive` before exit; the 8 ms bar moves only by explicit owner decision.
3. **WIT presence repoint desyncs Rust↔TS** — sol-registrar schema edit inside the quiet window, ts-rs regen sol-only, shared fuzz corpus in both suites at GATE K and GATE S′.
4. **WindowKit scene payloads stall the window** — the flip is its own packet after scene-surface lands in W1; if Text/Table/Mesh painters stall, the flip still lands (placeholder painters render, faults logged) and painter ports move to W3.
5. **Feature-unification recurrences** (os-kernel sync, os-mcp, the old ICE) — the workspace check joins GATE K and stays in every later gate; ICE is unreproduced but if it returns, minimize + gate the feature combo; toolchain changes are sol-only.
6. **Hidden dep edges breaking R19 co-dispatch** — luna cargo-tree probe before every wave; surprises resequence, never co-dispatch.
7. **Disk** — `df -h` at kickoff + every wave; below ~150 GB free: strict single-build queue.
8. **jco/JSPI Firefox** — GO-jspi stands (Chrome-first); fallback F2 (hand-rolled callback-ABI driver in the generator) already specified.
9. **Live tree / auto-commit bot** — every packet re-reads before editing; provenance via `git log --date=iso` only (commit-message dates are fake).
10. **Presence/intent semantics regressions** — integration-tested at sdk-wire (two mock clients, stale-intent, kind-discipline) long before web e2e; collab-e2e is the final proof.

## Verification (how the plan is checked during execution)

After each gate: the commands above, outputs pasted into `📓️status.md` with exit codes (rule 7). After GATE S′: fleet fan-out re-scoped from measured per-crate own-error counts (R21: verify the compiler REACHED each crate). After GATE W-u: parity smoke note+cad both renderers. Exit: the full EXIT ladder, every command pasted, census JSON diffed against the W1 baseline artifact in the ticket.
