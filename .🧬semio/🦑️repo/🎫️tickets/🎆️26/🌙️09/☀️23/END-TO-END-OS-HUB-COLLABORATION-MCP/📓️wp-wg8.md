# WP-WG8 — wgpu Native Collaboration: Kernel Turn (B1), Guest-Owned Codec (B2), Live Check, Creation Door

Slice: WG8 (session 11). Ports: hubs 8090–8099, serves 6590–6599. Private cargo target: `.tmp-ticket/wp-wg8/target`.
Captures: `wp-wg8/generated/`. Inheritance: `.tmp-ticket-0918/📓️g7w-…md` (B1, B2, B3, §8), `📓️n2-…md` §6.4,
`📓️wg6-…md`, `📓️tc3b-catalog-genesis-landed.md`; sibling `📓️wp-wg7.md` (N2 relay, `ureq` move, wasm32 `connect`).

## Status

| # | Item | Status |
|---|------|--------|
| 1 | B1 — native kernel turn never returns after a real guest boots | **FIXED, law green (measured 05:39)**: `a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub ... ok` — open 14.6 s (debug interpreter), every surface admitted, backbone bind receipt accepted, `addHandleKind` once in 3.6 s |
| 1b | §1.4 — native reserved-tool jobs (undo, redo, selection, clipboard) spawned but never stepped | **FIXED, one mechanism with React + guest (07:3x)**: 4/4 native journey laws green in 5 consecutive runs (edit, undo, redo, select-all + copy + paste; `generated/journey-{17..21}.raw.txt`), new shard law, kernel fixture laws (Rust 7/7, TS) — §1.4 |
| 2 | B2 — native document actor resolves a guest-owned kind through the mounted component's `codec` interface | **LANDED, laws green** (store resolution law, sync `componentIdentity` fixture law, 190 kernel sync/channel/client laws); **live `Live` not run** — needs a hub catalog carrying block (§2, §6) |
| 3 | `hub-live-collaboration-check` steps 1–12 green (runner: **WG8**, per WG7's scope split) | **GREEN (17:0x), all 12 steps, test exits 0 in 209 s** on hub 7800 catalog B (runId `345ceda4…`, generation `e8167ce8…`): `two_live_wgpu_shells_collaborate_on_one_hub_document ... ok` (`.🧬semio/🌐hub/s11-wg8-captures/collab-live-18.raw.txt`). Five native defects fixed on the way, each measured first (§3) |
| 4 | G-P1-3 — wgpu artifact-creation door (schema-first, progress + cancel, en + de) | **DONE, live-proven (05:42)**: a native wgpu shell created a hub artifact from its own door on hub 7800 — catalog ready (gis, note), `accepted → preparing → ready` in 53.7 s, `artifact-db290b13…`; 4 fixture laws green (§4) |

## Session 12

Ports: hubs 8090–8099, serves 6590–6599; durable data `.🧬semio/🌐hub/s12-wg8-*`. Hub 7800 = W2 (catalog B2 pending).

| # | Item | Status |
|---|------|--------|
| S12-1 | Async native open (`os.open-artifact` a retained frame-pumped operation, progress + cancel) | **DONE, laws green**: open law + 4 journey laws 5/5 at load ~40 (`journey-32.raw.txt`, 04:0x): relay ≤ 2.9 ms, opening frames ≤ 0.64 s, post-open render frames ≤ 0.35 s (debug), cancel settles `Cancelled` and mounts nothing. Root fixes on the way: kernel request queue lost wakes; settle-lane refresh reads (bodies + reserved sections) detached; live-only reserved jobs no longer checkpoint at their end + a cut owned checkpoint cancels itself (native undo/redo trap) |
| S12-2 | Native app presence (cursor/selection both ways, one wire, live with a React peer) + coordinator add-on: full cross-shell journey (native ↔ React `s`, one hub document) | **native half landed, laws green (01:3x)**: schema-first contract `🧬️schema/👕️canvas-presence` + fixture (publish / paint / labels en+de); wgpu twin `🧱️elements/👕️canvas-presence/🎯️targets/🧊️wgpu` (board view = scene camera + world point under the pointer; peer cursors, viewports, marks via replication's `peers_for_window`); the shell's heartbeat now carries `views`, `active_tool`, the guest's `interaction` + presence pack (native `AppFrame::Ephemeral` cache, the retired stub replaced); the chrome paints peer cursor/viewport/name chip + mark chips over every board in the hub colour; hub-admitted self identity from `Session`. Laws: Rust 3/3 + Shell 2/2 (painted board end to end), TS 5/5 (React's `puzzle2dScreenToWorld` / `peersForWindow` / `PEER_OVERLAY_LABELS`, gl-matrix `mat2d` inverse as oracle). Live cross-shell run: waits on 7800 B2. React gaps routed (C10): no `interaction` on React's heartbeat (`ephemeralSnapshot: undefined`), Board2dHost marks domain hardcoded `layer` |
| S12-3 | B2 `Live` on catalog B2 (guest-owned block via component codec) | **PROVEN live (05:40)**: gate run 24 step 6 Live in 11 s on 7800 B2 with the hub-resolved catalog component aaf7ee82 (component codec from the mounted guest) |
| S12-4 | `hub-live-collaboration-check` 12/12 on 7800 B2 (+ after `--packages all`); kernel store/sync, renderer, shard suites | **GREEN on 7800 B2 (08:53): 12/12, exit 0** (`s12-wg8-captures/collab-live-25.raw.txt`, generation f485bf7e…): both native shells resolve block by the serving generation, step 6 Live 10.0 s, 7 both online 0.15 s, 8 A authors 7.8 s, 9 B ingests, 10 B authors + A ingests, 11 per-actor undo, 12 offline edit 5.2 s (online 8.5 s), pump 3.0 s, stale → ready, relive 6.7 s, A ingests it. After `--packages all`: pending W2. Suites green 00:4x (kernel 572/572, shard 69/69, renderer 92/92 + kernel_runtime 41/41) |
| S12-5 | wasm32 checks for every crate touched in s11 + s12 | **GREEN (16:11, `s12-wg8-captures/check-wasm-9.txt`, after every WG8 edit)**: wasip2 `semio-framework` + `semio-framework-os-kernel` + `semio-framework-plugin` rc=0, os-kernel `--features sync` wasm32-unknown-unknown rc=0, `semio-framework-os-renderer-wgpu` wasm32-unknown-unknown rc=0 (earlier: wasip2 02:17, unknown-unknown 05:23) |
| S12-6 | Genesis-on-open parity with WG7 / C10 (via coordinator) | not started |
| S12-8 | Coordinator decision: a hub document's component resolves by the SERVING catalog generation (local only on equal content hash), verified, stored content-addressed, progress + cancel | **landed, laws green (04:2x); live on 7800 B2 running**: kernel resolver `📇️directory/🔌️client/🧩️execution-target-module` (`resolve_execution_target_module`: lease → local if equal SHA-256 → verified store entry → hub `execution-target/{component, descriptor}` bytes verified against the lease, stored whole via rename; `ExecutionTargetModuleStore` per-user cache dir; steps + cancel); native shell open gains phase `Resolving` (band `1/3` with the step, en+de, cancel cancels the in-flight request) and mounts the hub-resolved program (`program_bridge::load_resolved_program`, descriptor must name the lease digest). Laws: schema `🧬️schema/🧩️execution-target-module-resolution-v1` + fixture (9 cases over the hub's lease corpus) — Rust 9/9, TS oracle (Ajv + `node:crypto`) 11/11; band law en/de 1/1 |
| S12-7 | R8 relay: hub-projection twin drift — React's fold carries U5's link axis, the Rust twin did not | **DONE (01:2x), schema-first**: schema `🧬️schema/🔗️hub-projection` gains `session` (`none`/`signedOut`/`signedIn`) + `link` (`verifying`/`reachable`/`unreachable`), states `local`/`online`, `offline` dropped (unreachable with a 3-valued link, also in React's fold); fixture 10 cases; Rust fold `hub_connection_summary(statuses, session, link)` + `HubLink`; the Shell's duplicate `ShellHubConnectionState`/`ShellHubAuthorityV1` deleted (one state enum), badge texts en/de `hub.local`/`hub.online`; laws: Rust `hub_connection:: hub_projection_workspace_tests` 42/42, TS runner (Ajv schema + independent fold + React `hubConnectionSummaryV1` as oracle) 6/6 |

### Log (session 12)

- 23:0x start; read preambles + report. Found session-11 async-open work applied but unreported: `ShellDocumentOpening`
  (phase `Instantiating → Seeding → Cancelled | Failed`), `ShellDetached` (pool-spawned request, frame pump takes the
  answer), `render_surfaces_detached` (settle lane renders guest bodies off the shell), open laws in
  `🔗️hub-projection-workspace` (`settle_document_opening`, `a_native_guest_open_keeps_the_frame_loop_painting_and_is_cancellable`).
  Journey run 25 (18:01, `s11-wg8-captures/journey-25.raw.txt`): every open stuck `Instantiating` for 180 s; the headless
  `frame_pump` did not pump renderer I/O then (fixed after run 25; the detached-refresh edit landed 18:13, never run).
- 23:2x renderer test binary builds; open law alone green (`open-1.raw.txt`): relay 1.5 ms, cancel → `Cancelled`, opening
  frames ≤ 254 ms; but journey run 26 (`journey-26.raw.txt`, 5/5 pass) showed the settle lane pending for the full 180 s after
  3 of 5 opens (`rendering: 2.7–4.9 M` frames): a `[DEBUG] wg8` probe (run 27) named the term — `settle_pump.rendering` stayed
  `Some` (the detached render never answered); a second probe (run 28) caught exactly one
  `kernel push contended producer=true` right before the hang.
- **Root cause (kernel request queue, `🧊️renderer/🦀️.rs` `KernelRequestQueue::try_push`):** a push that met the queue's
  lock held (`try_lock` failed) returned the request with NO wake arranged, so a pool-driven `KernelFuture` (the detached
  render, the detached `create_app`) was never polled again; `drive`-polled callers (tests, dispatch) hid it by busy-polling.
  Also a second producer waiting on a full queue overwrote the first one's single `producer_waker`. **Fix:** a contended
  push asks its producer to retry at once; a full queue keeps every distinct waiting producer's wake (bounded by the queue
  capacity, `will_wake` dedupe) and wakes them all when a request leaves. Law
  `kernel_request_admission_never_parks_without_a_wake` over the schema-first fixture `🧫️fixtures/🧵️kernel-pool-future`
  (`admissions`: held lock, full lane ×1, full lane ×3 producers) with tokio's `sync::Mutex` / bounded `mpsc` as the oracle
  under the same wake-only executor; queue + future laws 10/10 (`laws-queue-1.txt`).
- 00:2x journey run 29 (`journey-29.raw.txt`): **5/5**, every open's render phase settles in 17–30 s total (was 180 s ×3);
  relay ≤ 2.3 ms; opening frames ≤ 0.71 s; post-open render frames ≤ 2.2 s (sections still inline). Latency outliers remain
  (`selectAll` 39.7 s, one undo 32.9 s — the session-11 "~30 s" watch item). `[DEBUG] wg8` probes reverted (0 lines).
- 00:4x suites on the current tree (S12-4 row). wasm32 gates queued behind W2/R8 in the mutex.
- 01:0x–01:2x **S12-7 (R8 relay, hub-projection link axis)**: `🧬️schema/🔗️hub-projection` + fixture (10 cases: busiest live,
  signed-out masks live, no hub is local, live outranks an unreachable link, unreachable link outranks dialling, verifying is
  connecting, connecting outranks backoff, backoff outranks detached, reachable+detached / reachable+empty are online);
  `🔗️HubConnection` fold + `HubLink` (serde camelCase, the schema's spelling) + unit laws (`an_unreachable_link_is_a_shortage…`,
  `a_reachable_link_with_nothing_live_is_online…`, `a_shell_with_no_hub_at_all_is_local…`; the icon-uniqueness law now asserts
  distinct spellings — `live`/`online` share React's cloud icon, the text tells them apart); Shell: `hub_projection()` maps
  `verified_session_authority` → session, `identity_offline`/authority → link (the connection book always holds the local
  bootstrap hub, so the native shell is never `none`); `ShellHubConnectionState` + `ShellHubAuthorityV1` deleted; the live
  sign-in law now asserts `online` after sign-in. TS runner `🧪️tests/🔗️hub-projection/🟦️.ts` passes `session`/`link` to React's
  fold (it failed typecheck before, R8 23:13). Measured: Rust 42/42 (`laws-renderer-8.txt`), TS 6/6 (`ts-hub-projection-1.txt`,
  run through a one-file vitest config `wp-wg8/vitest-one.config.mts`). Note for C10: React's `offline` state is unreachable in
  `hubConnectionSummaryV1` (all three link values return earlier) — dead in React's type/icon/label maps.
- 01:3x **S12-2 native presence (one wire, one encoding)** — what React sends today (measured in source): `views` (Board2dHost
  camera + world pointer, `space: canvas`, window = the dock window instance), `activeTool`; NOT `interaction`/`presencePack`
  (`PluginRuntime.ephemeralSnapshot: undefined`, and nobody calls `pushPresence`), and its board overlay paints marks only for the
  hardcoded domain `layer` (block2d declares `handle`). The native heartbeat sent none of it. Landed:
  - schema `🧑‍🎨engine/🧬️schema/👕️canvas-presence` + fixture `🧫️fixtures/👕️canvas-presence` (5 publish cases, 1 roster paint case with
    self / on-window peer with pointer + tool / other-window peer / pointer-less peer, labels en+de);
  - `🧱️elements/👕️canvas-presence/🎯️targets/🧊️wgpu/🦀️.rs` (mounted `crate::canvas_presence`): `canvas_screen_to_point`,
    `board_presence_view`, `board_peer_overlays` (replication's `peers_for_window` / `canvas_point_to_screen` /
    `canvas_peer_viewport_rect` / `peer_overlay_path`), `peer_overlay_label` en+de; the renderer now depends on
    `semio-framework-replication` directly (Cargo.toml + one Cargo.lock line, swapped atomically, `cargo metadata --locked` = 0);
  - EngineCanvas `board2d_presence_state(host)` (the board scene's camera + active utility, React's `sceneRef.cameraJson`);
  - ProgramBridge (native): every command exchange keeps the guest's last `AppFrame::Ephemeral` decoded
    (`ProgramEphemeralSnapshot { presence, interaction, tool_run }`, forgotten on `destroy_app`) — replaces the retired stub;
  - Shell: `presence_self` from `ArtifactEvent::Session` (was ignored), `presence_pointer` from every pointer move, heartbeat
    `views` + `active_tool` + `interaction` + presence pack; overlay phase 8 of `render_overlay_step` paints each peer's viewport
    frame, cursor dot and name chip and every mark chip, clipped to its board, in `theme.presence_color`.
  - Laws: `canvas_presence::tests` 3/3; Shell `board_presence_tests` 2/2 — a real painted Board2d surface (camera from its
    scene) → the heartbeat view, and the roster → overlays, nothing before the hub named the local actor; TS runner
    `🧪️tests/👕️canvas-presence/🟦️.ts` 5/5. Both Shell painting laws and `engine_surface_retention` now hold the crate's
    `engine_surface_law_guard` (process-wide engine/retained registries; `engine_surface_retention` failed in parallel with them
    before). Parallel set `canvas_presence board_presence engine_surface_retention hub_projection hub_connection` 50/50
    (`laws-board-presence-7.txt`). Pre-existing, not mine: `shell_input_tests` 40/71 red alone and serial ("painted within its
    opportunity ceiling" — R8's ui-contract retirement-stall family, fix after `--packages all`).
- 01:4x post-open refresh: `[DEBUG] wg8` timing (reverted) showed the settle lane's refresh still spent ~0.75 s inline on the
  three reserved-section reads (catalogue 216 ms, engagements 342 ms, measures/tools 187 ms). Fix: the detached settle read now
  carries them too (`render_refresh_detached` → `ShellRenderedRefresh { surfaces, catalogue, engagements, measures, tools }`;
  `refresh_ui_rendered` applies whatever was read ahead for the same instance, reads the rest itself; the one-time catalogue
  claim moves to the step that spawns the read). Journey run 30: post-open render frames ≤ 0.33 s (were ≤ 2.2 s).
- 01:5x run 30 also surfaced a **native undo/redo trap under load** (4/5; redo `guest trapped: owned turn is mid-flight and
  cannot admit 1 more event(s)`); `[DEBUG] wg8` dispatch trace (run 31, reverted): the reserved job's terminal step reported
  `Job`, and the very next turn was `Fault(mid-flight)`. Root cause (plugin host): on `JobStep::Done` the shard checkpointed the
  guest for the commit candidate; the owned checkpoint ran on a 1 s wall and, cut, returned `Err` **with the checkpoint operation
  still pending** — the deferred `Event::JobCompleted` then met a mid-flight guest. Fixes: (1) `🧵️shard` — a live-only
  (`!replayable`, framework reserved) job commits no restore state, so its end takes no checkpoint (`candidate.state` empty);
  (2) `🖥️host` `OwnedRuntime::checkpoint` cancels its own cut operation before returning the fault (never leaves a guest
  mid-flight). Law extended: `a_framework_reserved_spawn_starts_live_…` asserts no checkpoint at the end and an empty
  candidate state; `shard::` 69/69 (`laws-plugin-host-6.txt`).
- 03:40 resumed after the usage cut. wasm32 gates (`check-wasm-6.txt`, held 02:17): framework+kernel+plugin wasip2 **rc=0**,
  renderer unknown-unknown **rc=0** (includes kernel `sync`); the standalone kernel `sync` unknown-unknown run failed on a
  vanished proc-macro dylib in the shared build-dir (`libsemio_framework_value_derive-….dylib` "does not exist" — a concurrent
  build-dir rewrite, not a code error); re-queued with the shard edits.
- 04:0x journey run 32 at load ~40: **5/5** (`journey-32.raw.txt`), undo/redo no longer trap.
- 04:0x **B2 component identity**: catalog B2's block `sourceComponentSha256` aaf7ee82 ≠ every local build (component-release
  3a5a14fd, component-dev 4146fa0a; the release descriptor still names B's 0d1a9bcd). Coordinator decision (04:1x): resolve by the
  serving generation, never materialize locally. Why the execution-target routes and not `/trusted-catalog/plugin-modules`: the
  plugin-module bundle is the BROWSER module (jco core wasm + JS shims) derived from the component; a native shell mounts the
  component itself, and `POST /spaces/{s}/documents/{d}/execution-target/{manifest,component,descriptor}` serve exactly the lease's
  component + descriptor for that document (the same chain the semio MCP remote workspace already walks). Both are the serving
  generation; React (S15) and native now agree on the rule and on the vocabulary (`local` / `store` / `hub`).
  **For WG7 (wasm32 wgpu):** the resolver's decision + verification (`resolve_execution_target_module`, `verify_execution_target_bytes`)
  are target-neutral except the on-disk store (`#[cfg(not(target_arch = "wasm32"))]`); a wasm32 shell mounts JS plugin modules, so
  its twin is S15's plugin-module route with its durable store — not this component store.
- 04:3x gate run 19 on 7800 B2 launched (detached pid 48965, capture `s12-wg8-captures/collab-live-19.raw.txt`, store
  `.🧬semio/🌐hub/s12-wg8-execution-targets`).
- 04:3x gate run 19 (7800 B2): steps 1–3 PASS; 4a: A's door reached `ready` but its open never settled — the law's creation loop
  only drove `pump_sync_events`, so the resolution's descriptor read (renderer I/O) was never pumped (law fixed: full `frame_pump`
  + `settle_document_opening`); B's resolution answered `http 503` with no reason (the client dropped refusal bodies). The routes
  themselves answer (probe `wp-wg8/probe-execution-target.ts`: manifest 200, descriptor 200 106 012 B, component 200 17 678 000 B
  in 8 ms warm). Fixed on the way: the hub's execution-target **descriptor is the canonical PACK** of `PackageDescriptor`, not
  JSON — `load_resolved_program` now decodes it with the MCP remote's admission (decode, exact canonical re-encode, version 1,
  plugin + digest), stored as `descriptors/<sha256>.pack`; the three execution-target fetches keep a bounded refusal body.
- 04:4x runs 20–21: every native **sign-in answered `Unreachable`** — curl measured the hub's credential mint at 6.8 s and 13.6 s
  under load ~40 (peer Docker build, playwright, cargo), past the native lane's 5 s directory-command deadline; React's sign-in has
  no request deadline. Fix: the sign-in turn (mint + `me`) runs under its own finite `HUB_SIGN_IN_DEADLINE_MS` (30 s), still
  cancellable. Run 22 launched.
- 05:0x gate run 22 (7800 B2): 1–3, 4a PASS, A Live + authors (8) + undoes (11 own); B's resolution `http 503
  {"code":"deadline-exceeded"}` (hub's 8 s selection deadline at load ~40); A had NOT resolved — the door's relay names no
  plugin/app, so it bypassed the resolver and mounted the stale local bytes. Also `Surface unavailable: framework.hub — panel
  exceeds 128 document nodes` (user1 holds dozens of spaces). Fixes: (1) the lease names the plugin and app
  (`lease.package.pluginId`, `lease.surface.appId`), so every hub relay resolves (`document_execution_target_lease` first,
  then `resolve_execution_target_files`); an explicit relay plugin that disagrees with the lease is refused out loud;
  (2) `asked_through_shortage`: a `503` is asked again, ≤ `EXECUTION_TARGET_UNAVAILABLE_ATTEMPTS` (3), cancellable — fixture
  +2 cases (Rust 11/11, TS oracle 13/13); (3) the hub workspace tree carries ≤ `HUB_WORKSPACE_VISIBLE_SPACE_ROWS` (8) rows + the
  open space + a "N more — narrow the search" line (en/de, `data-semio-hub-spaces-hidden`); law
  `many_spaces_fit_one_panel_document_with_the_open_space_and_a_count_of_the_rest` (64 rows → ≤ 128 nodes). Run 23 launched.
- 05:2x run 23: every resolution refused `os.directory-client exhausted its network_bytes_per_min budget` — the native directory
  transport's HTTP pool budget was 10 MB/min, smaller than one 17.7 MB component. Fix: `SHELL_DIRECTORY_NETWORK_BYTES_PER_MINUTE`
  = 2 × `DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES` (the largest bounded answer the client takes, twice).
- 05:40 **gate run 24 on 7800 B2: 11/12 PASS** (`collab-live-24.raw.txt`): both native shells resolved block by the serving
  generation — store now holds `components/aaf7ee82….wasm` + `descriptors/bcaa9827….pack` = the catalog's
  `sourceComponentSha256`/`sourceDescriptorByteSha256` — step 6 Live 11.0 s, 7 both rosters online, 8 A authors 4.4 s, 9 B ingests,
  10 B authors + A ingests, 11 per-actor undo. **B2 `Live` (S12-3) proven** with the catalog's own component. Step 12 red only on
  its freeze bound: B's offline edit 10.3 s vs online 4.3 s (pump 3.0 s, stale → ready, relive 84 ms, A ingests the offline edit);
  the capture shows the edit's refresh hit `plugin retained document for surface 'block2d-board' exceeded its bounded opportunity
  budget` — the same retained-surface spin behind the 30–40 s `selectAll` in every native journey. Investigating (run 33).
- 05:4x retained-surface spin hunt (runs 33–35, `[DEBUG] wg8` probes, reverted): at load ≤ 10 the whole journey is 5/5 with
  `selectAll` 4.7 s (was 30–40 s under load 40–55), undo/redo 4.0–4.8 s; the one sampled busy owner was a panel surface mid-patch
  (`framework.panel.artifact patch=true published=false`) — the opportunity budget is spent by load, no defect reproduced at low
  load. Watch item, not fixed.
- 05:45 coordinator rule 20: HARD guest freeze (kernel, plugin SDK, guest-linked framework crates, root Cargo files). All my
  kernel edits (execution-target resolver, refusal bodies, shortage retry, lease split) landed 04:1x–05:2x, before it; the
  Cargo.lock/renderer `replication` dependency landed 01:0x. Since the freeze: renderer/shell-only edits.
- 05:23 wasm32 gates (`check-wasm-7.txt`): kernel `sync` unknown-unknown **rc=0**, renderer unknown-unknown **rc=0** (every
  target-neutral edit up to then: resolver, hub rows, hub link axis, presence).
- 08:40 resumed after the usage cut; load 7; 7800 ready; React serve 6590 up (pid 79710).
- 08:53 **gate run 25 on 7800 B2: 12/12, EXIT 0** (load ~7): the step-12 red of run 24 was load (offline 5.2 s vs online 8.5 s now).

### Files (session 12)

Paths relative to `🧰️framework/🛍️products/💻️os/🔨️modules/` unless absolute.

- `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — `KernelRequestQueue::try_push` (contended push wakes, bounded
  producer-waker list), mounts `canvas_presence`; law `kernel_request_admission_never_parks_without_a_wake` in
  `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document`; fixture/schema `🧵️kernel-pool-future` (`admissions`).
- `📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — async open (`ShellDocumentOpening`, phases incl. `Resolving`,
  `begin_document_resolution`, `continue_resolved_open`, band text en/de), detached settle refresh (`render_refresh_detached`,
  `ShellRenderedRefresh`), hub projection (`HubLink`/session axes, `shell_hub_connection_text`, `hub.local`/`hub.online`),
  presence (`presence_self`, `presence_pointer`, heartbeat views/interaction/presence pack, `board_presence_views`,
  `board_peer_overlays`, overlay phase 8 painting), `HUB_SIGN_IN_DEADLINE_MS`, `SHELL_DIRECTORY_NETWORK_BYTES_PER_MINUTE`.
- `📺️renderer/🧑‍🎨engine/🧱️elements/👕️canvas-presence/🎯️targets/🧊️wgpu/🦀️.rs` (new) + `🧪️tests/🔬️wgpu-unit`; schema + fixture
  `📺️renderer/🧑‍🎨engine/{🧬️schema,🧫️fixtures}/👕️canvas-presence`; TS runner `📺️renderer/🧑‍🎨engine/🧪️tests/👕️canvas-presence/🟦️.ts`;
  Shell law `🐚️Shell/🧪️tests/👕️board-presence`.
- `📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` — `board2d_presence_state`.
- `📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` — native `AppFrame::Ephemeral` cache
  (`ProgramEphemeralSnapshot`), `load_resolved_program` (canonical pack descriptor admission).
- `📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs` — `HubLink`, `hub_connection_summary(.., link)`,
  `HUB_WORKSPACE_VISIBLE_SPACE_ROWS`; `🏘️SpaceBrowser` label `MoreRows`; laws in `🔗️HubConnection/🧪️tests/🔬️wgpu-unit`.
- `📺️renderer/🧑‍🎨engine/{🧬️schema,🧫️fixtures,🧪️tests}/🔗️hub-projection` — session + link axes.
- `📇️directory/🔌️client/🧩️execution-target-module/🦀️.rs` (new) + mount + refusal bodies in `📇️directory/🔌️client/🦀️.rs`; law in
  `📇️directory/🔌️client/🧪️tests/🔬️unit`; `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/{🧬️schema/🧩️execution-target-module-resolution-v1,
  🧫️fixtures/📇️directory/🧩️execution-target-module-resolution-v1.json, 🧪️tests/🧩️execution-target-module-resolution/🟦️.ts}`.
- `🔌️plugin/🖥️host/🧵️shard/🦀️.rs` (live-only jobs end without a checkpoint), `🔌️plugin/🖥️host/🦀️.rs` (cut checkpoint cancels itself);
  law extended in `🧵️shard/🧪️tests/🔬️unit`.
- `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml` + `/Users/ueli/Documents/semio/Cargo.lock` (one line) —
  direct `semio-framework-replication` dependency.
- `📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs` — `frame_pump` pumps renderer I/O + settle,
  open laws, cross-shell law `a_native_and_a_react_user_collaborate_on_one_hub_document` (+ `CrossShellHandshake`,
  `paint_session_windows`, `pump_until`), gate law pumps full frames in 4a, band law; `🧲️engine-surface-retention` holds the
  engine-surface law guard.
- Ticket-local (`wp-wg8/`): `run-cross-shell.sh`, `cross-shell.mjs`, `serve-react.sh`, `run-collab-live-b2.sh`, `wasm-checks-{3,4}.sh`,
  `probe-react-boot.mjs`, `probe-execution-target.ts`, `vitest-one.config.mts`, `edit-*.py` (incl. reverted `edit-debug-*`).
- 09:5x a peer's in-flight test edit (`🐚️Shell/🧪️tests/🔀️wgpu-document-relay/🦀️.rs:240`, 08:53, E0502 borrow of `shell` inside `tabs_mut(..).push(..)`) blocked every renderer test build for an hour; fixed with the one obvious line (the pill text computed first). My builds meanwhile queued ~1 h in `prebuild_lock_exclusive` behind the fleet's wasm/native builds.
- 09:5x–10:3x **cross-shell runs 1–3** (native wgpu user A = law `a_native_and_a_react_user_collaborate_on_one_hub_document`,
  React `s` user B = `wp-wg8/cross-shell.mjs` on serve 6590, both on 7800 B2, handshake files). Run 3 measured (`cross-shell-3/`):
  A signs in, creates a space, seats B, creates a block2d artifact through its door (hub-resolved component), Live 6.2 s;
  **B opens the native-created artifact from its Space index in React, socket Live 9.6 s, `live · 2 peers`; presence both ways
  PASS** (A sees B 3.6 s, B sees A at once; one wire: the React roster row is `peer:<hub actor>` = A's `Session` actor); **A's edit
  reached React** (React's board summary 6 → 7 Handle Kinds) — the driver's ledger witness was wrong (React's History lists
  only that session's own commands), now the board summary. Harness fixes for run 4: React's edit goes through the window's
  Actions pane (`#action.addHandleKind`, opened first), undo through `#action.undo`; `--enable-unsafe-webgpu` (the React board
  painted no canvas headless); the native paint helper reuses the shell input laws' painter (no board registered before);
  React's reload restores the document in place (no Home), handled; creation waits 300 s (door reached `ready` after 324 s at load).
- 10:3x **cross-shell run 4** (`cross-shell-4/`): 1 A signs in ✓, 2 A creates + opens (Live 4.5 s) ✓, 3 B opens in React (Live
  17 s) ✓, 4 presence both ways ✓, **6 A edits → React ingests (6 → 7 Handle Kinds, 4 ms after A's handshake) ✓**, 7 React edits
  (7 → 8, 20.5 s) and A's ledger later shows it as `apply` (the law read A's count only after the handshake wait had already pumped
  the ingest — law bug, fixed: ledger count taken before the wait, `pump_until_ledger` re-reads the history), **8 each undoes own:
  A's undo reached React (8 → 7), React's own undo (7 → 6), A's ledger shows React's edit unapplied (`apply` false) ✓ (law read a
  stale snapshot — fixed)**. 9 reload: React restored into the Space index (not Home, not the document); the driver now reopens
  from whichever of the two it lands on. **5 cursors — not exercisable with block2d**: its only window (`block2d-board`, "Node
  Kind") is by design a summary surface (two text lines, `SurfaceKind::Board2d` but no board canvas), so neither shell has a
  board to publish a pointer or paint an overlay (native `boards=[]`, React no canvas). The cursor leg needs a canvas kind
  (puzzle2d board / draw) — the native resolver now mounts any kind the hub leases, so it is a harness follow-up, not a runtime gap.
- 10:35–10:57 **cross-shell run 5** (`cross-shell-5/`, load ~30): 1 ✓, 2 A creates + opens (Live 11.7 s) ✓, 3 B opens in React
  (Live 10.9 s) ✓, 4 presence both ways ✓ (A sees B at once, B sees A 4 ms after A's handshake), 5 ✗ (block2d: no board on either
  shell), **6 A edits (8.4 s) → React 6 → 7 Handle Kinds 5 ms after the handshake ✓, 7 React edits (7 → 8, 20.4 s) → A's ledger
  `apply` ✓, 8 each undoes own ✓** (A's undo reached React 8 → 7, React's own undo 7 → 6 in 414 ms, A's ledger `("addHandleKind",
  false), ("apply", false)` after 14.1 s). 9 ✗ in the driver: after `page.reload` React restored into the Space index and the driver
  waited for a Home row — fixed (`openDocument` races the artifact row against the space row).
- 10:57–11:15 **cross-shell run 6** (load 52–63 on 10 cores): 1 ✓; 2 ✗ — the door took 300 s to `ready`, the open then sat in
  `Seeding` (the guest's genesis load in the debug interpreter, on the pool) for the whole 180 s settle budget; React (same load)
  never listed the space within 180 s. CPU starvation, no defect; run stopped (my pids 79755/79798/79831).
- 11:0x **cross-shell laws split** (`🐚️Shell/🧪️tests/🔗️hub-projection-workspace`): one shared `CrossShellMeeting::meet(schema)`
  (steps 1–4: sign-in, space, seat B, door create + hub-resolved open, paint, React opens, presence both ways) + `finish` (done
  handshake, sign-out, assert); law `a_native_and_a_react_user_collaborate_on_one_hub_document` (block2d: 5 A edits → B ingests,
  6 B edits → A ingests, 7 each undoes own, 8 reload converges) and new law
  `a_native_and_a_react_user_see_each_others_cursor_on_one_hub_board` on `CROSS_SHELL_BOARD_SCHEMA = "puzzle.2d.fixture"` (catalog
  B2's puzzle; its editor window `2d-overview` is `SurfaceKind::Board2d` on both shells, and both shells name the window by the
  guest's default layout, native `block2d-board` = React's `windowInstanceId`). Driver `CROSS_MODE=edits|cursors`
  (`cursors`: mounted witness = a painted canvas, then the cursor leg); runner `CROSS_MODE=… run-cross-shell.sh <tag>` picks the law.
  `cargo check --lib --profile test` rc=0 (warnings = proof; the two in my file fixed).
- 11:1x wasm32 final gates queued in the mutex (`wasm-checks-5.sh` → `check-wasm-8.txt`: wasip2 framework + kernel + plugin, kernel
  `sync` unknown-unknown, renderer unknown-unknown).
- 11:21 test binary with the split laws built (`--no-run` rc=0, 0 warnings in my file). **Cross-shell run 7** (`CROSS_MODE=cursors`,
  load 55–64): step 1 ✗ — both native sign-in attempts answered `Unreachable` inside the 30 s sign-in deadline (hub mint + `me`
  under load; React's sign-in, which has no request deadline, got through at the same time). Load, not a defect; to re-measure
  the mint time with curl once 7800 is back.
- ~14:59 app restart killed every process (my wasm check queue `check-wasm-8`, the React serve 6590). 15:01 7800 rebooting on B2
  (W2 now runs the `--packages all` publish in the wasm mutex). 15:03 React serve 6590 relaunched (`serve-react-6590-b.txt`),
  wasm gates re-queued (`wasm-checks-5.sh` → `check-wasm-9.txt`, behind W2's hold). Rules 20/21 (guest freeze; no edits to
  taxonomy/nx/project.json/root Cargo/`.cargo`/`📇️directory/🧬️schema`) observed: no such edits since 05:45.
- 15:3x an external sweep deleted `wp-wg8/🗑️generated` and my private target dir `wp-wg8/target` (all ticket files re-stamped 12:50);
  build/check logs now live in `.🧬semio/🌐hub/s12-wg8-captures/`. First relink `rc=101` (`semio-framework-os-flow` lib: `linking with
  cc failed`, shared build-dir race), retried.
- 15:4x rule 23 (7800 is W2's even unbound): nothing of mine listened on 7800 at ~15:41 — my only server is the React serve 6590
  (pid 35215 → vite 35814, `S_LOCAL_ONLY=1`, so `ensureDevLocalHub` returns before it could spawn a hub; its log has no
  `dev-local-hub` line); `hub-live-collaboration-check` boots its own hub only on `freeLoopbackPort()` (OS-chosen ephemeral port) and
  was not run. The 7800 client defaults are gone from my runners: `run-cross-shell.sh <tag> <hubOrigin> <reactUrl>` and
  `run-collab-live-b2.sh <hubOrigin>` refuse to start without an explicit origin; the Rust laws already require `SEMIO_HUB_LIVE_ORIGIN`.
- 15:58 test binary relinked (`build-cross-split-3.txt`, rc=0). **wasm32 gates GREEN (`check-wasm-9.txt`, hold 15:54–16:11, mutex
  released)**: wasip2 framework + kernel + plugin rc=0 (15:57), kernel `sync` unknown-unknown rc=0 (15:59), renderer unknown-unknown
  rc=0 (16:11) — every WG8 edit of s11 + s12 compiles for both wasm32 targets. Rule 24 (build-quiet until publish 4): no further
  wasm32 builds/checks from WG8.
- 16:15 7800 ready again (B2, runId 8d0ec7a5…). Cross-shell run 8 (`CROSS_MODE=cursors`, puzzle2d) launched.
- 16:22 **cross-shell run 8** (`cursors`): 1 ✓ only on the second attempt (first `Unreachable`), 2 ✗ — no space row after
  `CREATE_SPACE` (`space=`, catalog idle). Measured with curl on 7800 (load ~50): mint `POST /auth/sessions` **9–46 s** (hub log
  `server.auth.session.mint` 11–46 s), `GET /directory/spaces` **25–58 s** for user1 (82 spaces; user2: 47 spaces in 3.2 s), `me`
  2–4 ms, `create-space` itself 5–45 ms (hub log). Two causes: (a) hub-side latency (routed: list → H9; mint is 261 ms median on the
  current tree per H10, the post-publish binary carries it); (b) native: the kernel ureq agent has a fixed 15 s overall timeout
  (`UREQ_HTTP_READ_TIMEOUT_MS`, `📇️directory/🔌️client` native transport) that overrides the shell's 30 s sign-in deadline and cuts the
  list reload — sign-in answered `Unreachable`, the reload went `Stale`, and both shells rebuild the space list ONLY from that query
  after a command (state-driven). Run 8 stopped (my pids 46840/47487/47902). Coordinator messaged; agreed.
- 16:3x–16:51 **fix (renderer + shared contract, event-driven read-your-writes)**: a command receipt's directory events are folded into
  the space rows at once — `spaceRowsAfterEventsV1` (`📇️directory/🏘️spaces/🟦️.ts`) and its Rust twin
  `space_browser::space_rows_after_events` (`🏘️SpaceBrowser/🎯️targets/🧊️wgpu`): `space.created` by me + my `member.upserted` → my
  row (role → access: author → `author`, spectator → `member`; member count from the same receipt), rename / visibility / archive
  (author → spectator) restate a row, `space.deleted` drops it, leaving a private space drops it, leaving a public one keeps it as
  `public`, joining a public one counts me, a redeemed invite counts a member, `document.announced` counts a document; another user's
  `member.upserted` leaves the count to the next list (not derivable). Wired: native `flush_pending_directory_commands` folds every
  receipt and marks the rows `Ready` before the list reload (a failed reload leaves them `Stale`, still usable); React
  `useHubConnection.runCommand` folds the receipt into `rows` and keeps the phase `ready` (it used to fall to `loading` — rows unusable —
  for the whole list query). Schema-first: `🏘️spaces/🧬️.schema.json` gains `receiptFolds` + `definitions.spaceRow`/`directoryEvent`;
  fixture `🏘️spaces/🔣️.json` gains 11 cases incl. the golden directory log (`💻️os/🧫️fixtures/📇️directory/⚡️events.json`).
  Laws: Rust `space_browser::tests` **18/18** (2 new: the fixture table; agreement with the kernel read model `os_directory::fold` on
  the golden log), TS `hub-sign-in-spaces-check` **98/98** (2 new + Ajv strict over the extended schema; oracle: the TS read model
  `foldAll`) — `laws-space-browser-1.txt`, `laws-hub-spaces-2.txt`.
- 16:4x **prepared kernel patch** (rule 20, NOT applied, NOT compiled): `wp-wg8/kernel-patch-transport-deadline.py` — the native ureq
  transport bounds only the connect; each request's overall timeout = what the caller's `OperationContext` deadline leaves (runtime
  clock `run_io` uses), else a 120 s backstop (frees the blocking IO thread); body stall bound unchanged. Laws in the patch: shared
  fixture `🪪️runtime/🧫️fixtures/⏱️request-budget.json` (5 cases) + a slow local server (600 ms head: served inside a 5 s deadline,
  refused past a 200 ms one). Apply after W2's publish, then kernel native tests + wasm32 checks.
- 16:52 **cross-shell run 9** (`cursors`, load ~30): 1 ✓ (first attempt), 2: the created space is now listed at once (receipt fold,
  live) and the door's catalog is ready (`2d.puzzle`), but the creation trail went `accepted → indeterminate` at the native door's
  fixed 120 s deadline (`HUB_ARTIFACT_CREATION_DEADLINE_MS`) — the hub finished it anyway: the space page lists
  `artifact-557f1238…` (`2d.puzzle`), updated ~275 s after the space was created. The same defect S15 fixed on React (shared
  contract `🏪️store/👷️worker/🌱️creation-polling/🔣️.json`: follow a creation while the hub answers, backoff 100 ms → 2 s, `indeterminate`
  only after 60 s without an answer). Run 9 stopped (my pids).
- 17:0x **native creation door on the shared polling contract** (`edit-creation-polling.py`): `HubArtifactCreation` carries
  `last_answered_at_ms` + `polls` instead of `deadline_at_ms`; `hub_connection::space_artifact_creation_poll_delay_ms` /
  `space_artifact_creation_unreachable` read the shared contract file (the constants `HUB_ARTIFACT_CREATION_DEADLINE_MS`/`_POLL_MS`
  deleted); the Shell's pump marks every 2xx answer and backs off per poll. Law
  `the_creation_door_follows_the_shared_polling_contract` (delays 0..40 recomputed from the contract's raw numbers, the bound both
  sides). `hub_connection::` + `space_browser::` **58/58** (`laws-hub-door-1.txt`). Cross-shell creation wait 300 → 600 s.
  Run 10 (`cursors`) launched 17:04.
- 17:04–17:27 **cross-shell run 10** (`cursors`, load ~33): 1 ✓; 2: the door now follows the creation to the end — trail
  `accepted → preparing → ready` (**~10 min** on the hub for a 2d.puzzle at load 33; the polling fix works live) — then the native open
  of the hub-resolved puzzle component failed: `kernel: actor 0 stayed preempted past its 30s turn budget` (the new instance's
  `InstanceOpen` turn, resumed after every 100 ms preemption, ran > 30 s wall in the debug test host at load 33;
  `RUN_TURN_SETTLE_BUDGET` in `🧊️renderer/🦀️.rs`). React (B): the new space's row was never attached on Home within 180 s — Home
  now holds ~50 spaces for user2 and the driver waited for a row off-screen. Driver fix: after 20 s without either row it follows the
  space's deep link `/spaces/<id>` (the React shell's own hard-navigation route). Run 10 stopped (my pids).
  **Finding (native kernel, not fixed):** one `run_turn` request monopolises the single kernel request loop for up to 30 s and then
  reports a slow-but-progressing guest as wedged; a long `InstanceOpen` (puzzle2d: 28 MB component, debug host, loaded machine) fails
  the open instead of showing progress. Proposal: a preempted actor yields the loop (the request is re-queued kernel-side and resumed
  next iteration, other requests interleave), the open's band shows it, and cancellation (the caller dropping the reply) — not a
  fixed wall budget — ends it; a guest that never finishes is then the caller's cancel, as in React's worker. Wide blast radius
  (every native turn) → proposed to the coordinator rather than landed blind.
- 17:28 cross-shell run 11 (`edits`, block2d) launched to measure 1–8 with the receipt fold + creation polling fixes.
- 17:46 run 11: 1–2 ✓ (door followed the creation 613 s to `ready`, Live 5.1 s), React driver died on its own bug (the deep-link used
  the shadowed `URL` string as a constructor) — fixed (`globalThis.URL`), run stopped.
- 17:47–18:01 **CROSS-SHELL RUN 12 (edits, block2d, 7800 B2, load ~25–40): 8/8 PASS, law exit 0** (`cross-shell-12/`, 819 s):
  | step | result |
  |---|---|
  | 1 A (native wgpu) signs in | ✓ first attempt |
  | 2 A creates a space (listed at once via the receipt fold), seats B, creates a block2d artifact through its door and opens it | ✓ door `accepted → preparing → ready` in ~619 s (hub-side), open 19.1 s (frames ≤ 0.10 s opening / ≤ 0.21 s rendering), Live 5.8 s |
  | 3 B (React `s`, Playwright) opens it | ✓ via the `/spaces/<id>` deep link (Home holds 47 rows), Live 11.3 s, `hub=live` |
  | 4 presence both ways | ✓ A sees B after 1.8 s (`User Two`, colour 1), B sees A 4 ms after A's handshake (one wire: `peer:hub.v1.…`) |
  | 5 A edits → B ingests | ✓ A 7.2 s, React 6 → 7 Handle Kinds 2 ms after the handshake |
  | 6 B edits → A ingests | ✓ React 7 → 8 (20.2 s incl. opening the Actions pane), A's ledger shows `apply` |
  | 7 each undoes own | ✓ A's undo reached React (8 → 7), React's own undo 7 → 6 in 61 ms, A's ledger `(addHandleKind,false),(apply,false)` after 19.6 s |
  | 8 same-document reload converges | ✓ React reload → Home → deep link → reopened Live 9.9 s at 6 Handle Kinds (= A's state), A's next edit reached it 3 ms after (6 → 7) |
  Cursor leg (separate law, puzzle2d board): blocked by the native long-turn limit (finding above, coordinator-approved redesign in progress).
- 18:0x coordinator approved the long-turn redesign (fair round-robin slices, progress, only an explicit cancel ends a turn; laws: a 60 s
  turn completes while another actor keeps answering, cancel ends it, no starvation).

## Coordination (read me, WG7 / coordinator)

- **Runner of `hub-live-collaboration-check`: WG8** (WG7's scope split, `📓️wp-wg7.md`).
- **Kind identity — one mechanism.** New in the kernel store (`🏪️store/🦀️.rs`, region `CodecRegistry`):
  `ComponentDocumentCodec` (trait: `schema`, `pack_schema_hash`, `print_mirror`), `register_component_document_codec`,
  `DocumentKindCodec { Linked, Component }` and `document_kind_codec(schema)` — linked Rust codec first, else the
  codec of the mounted component that owns the kind (the hub's trusted-catalog order: linked `codec`, else `guest`).
  WG7's shared helper `document_pack_schema_hash(schema, lease)` (region `DocumentSocketConnect`) now asks
  `document_kind_codec` first, then the lease; the native actor's `start_connect_hub`, `finish_connect_hub` and
  bootstrap identity use that one helper; bootstrap validation and folder mirror persistence use
  `DocumentKindCodec::print_mirror`. The browser host can register a jco-backed component codec the same way.
- New fixture cases `componentIdentity` in WG7's `🏪️store/🧫️fixtures/document-socket-connect-v1/🔣️.json`
  (+ runner `a_mounted_component_codec_is_the_kind_identity_before_any_lease`); every case owns a distinct schema
  because the component registry is process-wide — WG7's `packIdentity` cases (schema `block.2d`) are untouched.

## 1. B1 — root cause and fix

Measured inheritance (G7w runs 10–13, `.tmp-ticket-0918/wp-g7w/generated/g7w-live-1{0..3}.txt`):

- run 10/11: the first post-boot turn was **cut on its 100 ms wall grant** (owned interpreter → shard
  `DeadlineExceeded` → reported as `MoreWork`); the next ordinary event reached a guest still mid-call →
  `owned turn is mid-flight` trap → failure ladder → `tick` grants nothing → "shard produced no outcome".
- run 12/13 (G7w's resumed settle loop: continue every `MoreWork` with `Event::Wake` until `Idle`): the open
  settles, but an authored edit spins **30 138 `MoreWork` turns** until the 30 s budget
  ("turn did not settle within 30s", steps 8/10/11/12).

Root cause (two conflated meanings of `MoreWork`):

1. A **preempted** owned turn (cut mid-call, owns the next call) and a guest's **completed** turn saying it has
   more work were the same outcome, so the host could not know when resuming was mandatory.
2. The settle loop swallowed the **command-ingress protocol**: `exchange_commands` drives one page per turn
   and must observe every `CommandIngressStatus`; the settle loop absorbed a `PageAccepted`/`CommandPending`
   into later `Idle`-ingress Wake turns (`ExchangeOutcome::absorb` keeps the LAST ingress), so the driver
   re-offered the page / never saw completion, and a guest waiting on a host round trip (command page,
   typed-operation ACK) answered `MoreWork` forever.

Fix (source landed 01:5x, `cargo check` green, see §5):

- `🔌️plugin/🖥️host/🧵️shard/🦀️.rs`: new `ShardOutcome::Preempted { actor }` (pack tag 6) — emitted when a turn
  faults `DeadlineExceeded | FuelExhausted` **and** the instance is still `turn_in_flight()` (owned
  interpreter). Wasmtime's non-resumable cut keeps its old `MoreWork` shape. `FuelExhausted` on a resumable
  instance was a latent trap (kernel fault ladder) — now a preemption too.
- `🧊️renderer/🦀️.rs` `run_turn_once`: any granted actor reported `Preempted` gets an `Event::Wake` resume
  envelope inside the same tick loop, bounded by `RUN_TURN_SETTLE_BUDGET` — a request never hands a mid-flight
  guest back to its caller.
- `run_turn`: the React host's settle rules (`settlePluginTurn`): lifecycle receipt → ack; completed `MoreWork`
  with **idle command and cold-pair ingress** → continue; non-idle ingress → return to the caller's page
  driver; stop after `run_turn_quiescent_continuations()` (= `max_patch_bytes / max_text_bytes × 8` = 128,
  the same derivation as React's `PLUGIN_UI_QUIESCENT_CONTINUATIONS`) continuations that carried nothing
  (`ExchangeOutcome::carries_nothing`, native twin of `shardTurnCarriesNothingV1`).
- `🏃️run/🦀️.rs` (`semio-framework-os-run`): the same `Preempted` outcome is resumed there (its own tick loop).

Laws: `a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub` (green) and
`a_native_guest_undoes_its_authored_edit_without_a_hub` (green since §1.4) (`🐚️Shell/🧪️tests/🔗️hub-projection-workspace`),
driven by the new language-agnostic fixture `🧫️fixtures/⏯️native-guest-journey/🔣️.json` (open relay args, verb,
undo, expected ledger deltas); verb `native-guest-journey-check` (renderer package, nx target of the same name,
launch row `⚖️gate🧊️wgpu⏯️native-guest-journey` after `⚖️gate🔐️hub-auth🧊️wgpu-live-collaboration` in both
launch files). Hub-less, so B1 can never hide behind a hub failure again. The temporary `[DEBUG] g7w` turn
logging (committed by G7w) is removed.
Run 1 (01:2x, pre-fix tree + pre-H9 guest): `component has no core module implementing the owned Semio actor ABI`
— the staged block component predates H9's 14th owned export (coordinator: not B1); block2d rebuild launched
through the wasm mutex (`wp-wg8/block-release.sh`, `generated/block-release-1.txt`).

### 1.2 Two more native-only defects found by the hub-less law (fixed)

Journey run 2 (01:5x, rebuilt post-H9 block2d): preemption now resumes (`outcome other` = `Preempted`, then the
turn completes), but every `SurfaceVisible` answered **no patch** → "retained surface … is not admitted".

- **Surface identity.** The kernel names an instance's surface `"<instance>:<surface>"` (the guest's
  `parse_surface_instance`; the wasmtime host builds the same from WIT `surface-ref`; `📥️ui-patch` names patches
  so). The native `ProgramBridge::render_with_document` sent a bare `"block2d-board"` — the owned interpreter
  hands the kernel event to the guest verbatim, the guest parsed no instance and mounted nothing — and looked the
  retained document up by the bare id. Fix: `kernel_surface_id(instance, surface)` for the event, the targeted
  advance and the lookup (`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`).
- **Actor 0 had no patch transport.** Run 3: the first reconcile turn with patches faulted `fixed turn patch
  transport admission refused the exact owner`. The shard's transport session is the granted actor id, and
  `ActorId(0)` is real (first app of the first plugin: ordinal, kind, index, generation all 0), but the kernel's
  `UiTurnPatchTransportArena::reserve` and `UiTurnPatchTransportLease::try_from_token` refused session 0 as a
  sentinel. Every lookup already keys on the slot state, so 0 needs no sentinel. Fix in `🎠️kernel/🦀️.rs`
  (`semio-framework`, target-neutral) + law `ui_turn_patch_transport_admits_the_zero_actor_as_its_session`
  (`🎠️kernel/🧪️tests/🔬️ui-turn-patch`). This is also why G7w's actor A never painted while B (actor 16384) only
  lacked the surface identity.

Journey run 5 (02:4x): open 46.4 s (debug interpreter), `error=None` (every surface admitted), `addHandleKind`
`Ok(())` in 1.36 s, ledger `[] → [("addHandleKind", true)]`; commands now answer `commandComplete` per page.

### 1.3 Three more native defects on the edit/undo path (03:1x–05:3x)

- **Frames were prefix-decoded.** `decode_app_frame` accepted any payload that merely STARTED like a frame; a
  document-backbone binding receipt (pack-value tag 3 = `AppFrame::Document`) was swallowed as a frame, so the
  open relay failed with "binding returned 0 shell receipts". Fix: `decode_app_frame` refuses trailing bytes
  (`📡️spr/🧵️channel/🦀️.rs`, kernel crate, target-neutral) + law `decode_app_frame_refuses_trailing_bytes`.
- **Typed-operation pages were never acknowledged in time.** Reserved tool jobs (undo, redo, checkpoint, copy…)
  publish a `semio.typed-operation-page.v1` page and wait for its ACK; the renderer parked the page in a
  process-wide exchange no native consumer ever read and queued the ACK behind the running request, so undo spun
  516 silent `MoreWork` turns and never applied. Fix (React's `settlePluginTurn` rule): `apply_turn_result` puts
  the pages on `ExchangeOutcome::typed_results`, `run_turn_once` owes one ACK event per page, `run_turn` delivers
  every owed event (lifecycle ACK, typed ACK) on a turn of its own inside the same settle. The dead
  `TypedOperationResultExchange`/`MountedTypedOperationResultExchange`, `KernelRequest::AcknowledgeTypedOperationResult`,
  `deliver_typed_operation_result_ack`, both `acknowledge_typed_operation_result` accessors, their test file and the
  fixed-slot budget row (`⏳️async/🧫️fixtures/🧱️boxed-fixed-slots`) are deleted; the page's wire-bound law moved to
  `🧪️tests/🗞️typed-result-page`.
- **`absorb` let an `Idle` ingress overwrite `CommandComplete`.** With ACK turns inside the settle, the page
  turn's `CommandComplete` was replaced by the ACK turn's `Idle`, so `exchange_commands` resent the page: run 9
  applied one `addHandleKind` 64 times; run 10 resent it 6 466 times in 83 min (coordinator-reported 99 % CPU; `sample`
  in `generated/journey-10-sample.txt`: test thread in `author_edit → dispatch_action → exchange_commands`; run
  killed, pids 88067/88069/88099). Fix: `absorb` keeps the latest NON-idle ingress.

### 1.4 Native reserved-tool jobs — root cause and fix (landed 07:3x)

Two faults, both measured on the native journey (block2d release, no hub):

1. **Nothing stepped the job** (runs 11–14): the undo page answers `commandComplete` with the admission `Invocation`
   plus `SpawnJob { kind: "framework.reserved.tool" }`. The shard admitted the spawn as a replay seed and started it,
   but steps a job only on a host `Payload::JobStep { turn }` carrying the exact shard-minted `JobTurn`, which the host
   never learned; the renderer sent `JobStep` only for product-replay entries. `JobCompleted` never reached the guest.
2. **The seed closed silently** (runs 15–16, once the host stepped): `ShardLoop::pump: job 71 has no independently
   admitted operation authority (retained shard failure: plugin: ShardLoop::replay: checkpoint page admission refused
   for actor 16384)`. Every spawn's seed checkpoints the whole guest before it starts the job (so a host can replay
   it on another worker); block2d's checkpoint outgrows the fixed checkpoint pages, the seed closed, and the executor
   retained the failure where no host ever read it.

Fix (one mechanism; the guest half is C8's `admit_reserved_spawned_job` + one commit unit per turn, unchanged):

- **Kernel contract** `semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND` (next to `SPAWNED_JOB_STEP_CEILING`), TS twin
  in `🎠️kernel/🟦️.ts`, both asserted from the fixture `🧫️fixtures/🧵️spawned-job-drive` (`reservedKind`). The plugin's
  `app::FRAMEWORK_RESERVED_JOB_KIND` is a re-export; React's `PluginRuntime` uses the TS constant (3 literals gone).
  Pure additions — no ABI, pack-schema or codec-hash change (soft freeze).
- **Shard** (`🔌️plugin/🖥️host/🧵️shard`): `ShardOutcome::Turn { jobs }` reports each admitted spawn's exact `JobTurn`;
  `ShardOutcome::actor()`; a reserved seed is live-only (`MountedReplaySeed.replayable == false`: capture kind and input,
  then start — no guest checkpoint) and `validate_replay_request` refuses it. Law
  `a_framework_reserved_spawn_starts_live_hands_its_turn_to_the_host_and_refuses_replay` (69/69 `shard::`).
- **Renderer kernel thread** (`kernel_runtime`): a fixed `ReservedToolJob` registry (64, like every job table there),
  filled by `admit_reserved_jobs` from each settled turn's reserved spawns and reported turns. `settle_reserved_jobs`
  runs after `exchange` and after `exchange_commands` completes (React: `driveReservedToolJob` serialized after the
  command's ingress): `run_reserved_job_once` grants one `Payload::JobStep` per turn, the step publication yields the next
  turn or ends the job, and the shard's deferred `JobCompleted` turn — an outcome no grant asked for — is counted as owed
  and awaited by `dispatch_turn` (per-outcome grant/owed tally), then settled by the same `settle_turn` rules (typed-page
  ACKs, `MoreWork`). Reserved spawns are host work, never `ExchangeOutcome.effects` (so never a product replay); every
  turn result of one dispatch is applied (a second used to overwrite the first). `ParallelRuntime::take_shard_failure`
  names a retained shard failure in the fault (that is how fault 2 was found).

Measured (runs 17–21, `generated/journey-{17..21}.raw.txt`): undo `Ok`, ledger `addHandleKind` → `false`, 3.8–4.3 s
(debug interpreter; the verb's own exchange 0.43 s, the rest is the shell's refresh exchanges); redo `Ok`, → `true`;
`selectAll`/`copy`/`paste` `Ok`, ledger unchanged (block2d has no clipboard producer, so the framework clipboard routes
answer empty — the fixture declares it). The reserved drive takes 2 steps per job (`Yield`, `Complete`). Two latency
outliers of ~30 s (run 18 undo, run 19 `selectAll`) did not recur in runs 20–21 under `[DEBUG]` timing (no kernel
request > 1.6 s, no outcome wait > 0.5 s, no foreign grant); the machine was running W2's release builds. Watch item.
Runs 22–23 (same timing, 07:5x–08:0x): no outlier; run 22's 4th open failed with `os.open-artifact could not switch to
block: plugin: wasm decode: byte range exceeds input` — W2's batch B was rewriting block's `component-release` under the
staged runtime at that moment (not a kernel fault; `run-collab-live.sh` republishes the runtime from W2's finished
component). One more guard from reading the drive: a submitted `JobStep` that no grant answered used to return an
empty `Ok` (the loop would resubmit the same turn); `dispatch_turn` now counts an empty reserved turn as settled only
when a step publication arrived, else it is the loud "shard produced no outcome".

### 1.5 Earlier notes (resolved)

- The undo spin (516 silent `MoreWork`) was the unacknowledged typed-operation page (§1.3, fixed); the undo job
  itself is §1.4.
- The backbone-bind "0 shell receipts" was the prefix frame decode (§1.3, fixed).

## 2. B2 — design and landing

- Kernel store: trait + registry + `document_kind_codec` (above).
- Plugin host: `🔌️plugin/🖥️host/🧬️component-codec/🦀️.rs` — `OwnedComponentDocumentCodec` (owned interpreter +
  compiled component + schema; `codec.pack-schema-hash` asked once and kept; `codec.print-mirror` per call) and
  `GUEST_CODEC_BUDGET` (4 G fuel, 30 s no-progress wall — the hub's values; the hub still carries its own copy,
  follow-up: import this one).
- Renderer: `KernelClient::create_app(.., artifact_schema)`; `ProgramBridgeEntry::create_app` passes
  `app_document_schema(app_id)` — the app's `io.artifact_schema`, or for a viewer (which declares none) its
  package's schema for the same dialect, so a spectator's mount registers the codec too; the kernel thread
  registers the compiled component as that kind's codec right after compile (`CreateAppRequestOwner` gained the
  field inside its bounded close; its shutdown law updated).
- Follow-up (not done): the hub's `GuestArtifactCodecBinding` could adopt `OwnedComponentDocumentCodec` once the
  kernel trait grows genesis/apply-ops with fuel-progress observation; it keeps its own copy of the budget today.
- Laws (written, not run yet): store `a_kind_resolves_to_its_linked_codec_before_its_mounted_component`; sync
  `a_mounted_component_codec_is_the_kind_identity_before_any_lease` (fixture `componentIdentity`).

## 3. The 12-step gate on catalog B — runs 1–18 and five native fixes

Runner `wp-wg8/run-collab-live.sh` (block2d release runtime staged from W2's catalog component `0d1a9bcd…`; the release
descriptor was rematerialized at 12:18, so the publish verb accepts it). Captures moved to the durable
`.🧬semio/🌐hub/s11-wg8-captures/` after the 12:19 cleanup deleted `generated/` (runs 1–4 lost; their findings are below).

| run | result | measured cause → fix |
|---|---|---|
| 1 | 1–5 ✓, 6 ✗ (sockets stay `Connecting`) | `there is no reactor running` panic on a pool worker at the hub dial (`🏪️store/🔄️sync`): the native document actor took whatever Tokio runtime its spawner was inside, and a wgpu shell spawns from a plain thread. **Fix:** `document_socket_io_reactor()` — one current-thread I/O+time reactor on its own `semio-document-io` thread, entered only while an actor is polled. Law `a_hub_dial_polled_off_any_runtime_is_driven_by_the_document_socket_reactor` (kernel `os_store::sync` 67/67) |
| 5–8 | 1–6 ✓ (**B2 `Live` proven**: native codec hash `1869126a…` = hub pin), 8 ✓, 11 ✓; 7, 9, 10, 12 ✗ | every authored edit refused by the document actor as `document backbone scope mismatch`: the guest's envelopes carried `document_id = s.block.block2d@1/*#editor` (its app id) — a fresh door artifact answers the socket `Bootstrap::None`, so nothing ever gave the guest the document's identity. **Fix:** the store contract gains `ComponentDocumentCodec::genesis` + `component_document_genesis(schema, id)` (zero-history check identical to the hub's `initial_pair`); the host adapter calls `codec.genesis`; `open_document` loads that genesis into the guest before its actor exists — the hub seeds the artifact from the same export. Law `a_document_opens_on_the_genesis_its_owning_component_mints_for_its_identity` |
| 13 | 1–6, 8, 9, 11 ✓; 7, 10, 12 ✗ | no presence heartbeat ever reached the hub: the native shell beats in the chrome walk's presence phase, which a headless law never runs, so the hub closed each idle socket after its presence lease (reconnect every 30 s) and B's edit waited out A's backoff. **Fix (law):** `frame_pump` = what one painted frame does for a document (sync pump + the presence phase → `advance_presence_preview_step`) |
| 14–15 | 1–11 ✓; 12 ✗ (pump 31 s / 7 s for a 3 s window) | `[DEBUG]` timing: every pump carrying a presence or status event called `refresh_ui(Full)` — every guest body re-rendered ten times a second per peer, 3.4–5.6 s per pump in debug. **Fix:** `pump_sync_events` refreshes guest bodies only when the guest's document changed (remote mutations, archive, backbone effects, terminal fault); status/bootstrap/conflict republish the host-owned Sync panel; presence is footer state painted next frame |
| 16 | 1–11 ✓; 12 ✗ (A's ledger read 0.8 s after relive) | the law read A before B's outbox flushed; now it pumps 10 s after relive, as steps 9–10 do. "Not frozen" is measured against the same shell's online edit (≤ 2×), not a 2 s constant a debug interpreter never meets |
| 5–17 | 8 of these runs never exited | sampled (run 17): the test thread parked for 23 min in `block_on(handle_hub_workspace_action)`; `hub_verb` now uses `drive` (pumps renderer I/O + worker retirements like the frame does). The hung test processes were mine; stopped by pid |
| **18** | **12/12 ✓, exit 0** | step 6 Live in 3.4 s, 7 both rosters online, 8 A authors (3.9 s), 9 B ingests, 10 B authors + A ingests, 11 per-actor undo propagates (`apply` false on B), 12 offline edit 4.6 s (online 6.4 s), pump 3.0 s, Stale → Ready, relive 0.1 s, A ingests the offline edit |

Also measured, not fixed here (routed): opening a document runs inside the frame pump (the door's `Ready` open held one
pump for ~20 s in debug: guest mount + genesis + manifest) — the native `os.open-artifact` relay should become a retained
operation like the creation itself; the native presence peer carries no app presence pack (no bounded ephemeral snapshot
API natively yet, the state React publishes when its snapshot misses the bound).

## 4. G-P1-3 — the wgpu artifact-creation door

- Directory client (kernel, target-neutral): `🔌️client/🌱️space-artifact-creation/🦀️.rs` —
  `space_artifact_creation_catalog`, `create_space_artifact`, `space_artifact_creation_status`,
  `cancel_space_artifact_creation` over the schema-owned `SpaceArtifactCreation*V1` types; exact routes
  (`SpaceArtifactCreationRoute`), bounded answers, canonical parse, space/request identity checked.
- Hub workspace (`🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs`): `HubArtifactCreationState` (catalog phase, catalog,
  chosen kind, name, pre-minted idempotency key, one creation), `hub_artifact_creation_intent`, the door section
  in the open space (catalog line + role, one choice per kind with the hub's en/de label, name, Create, the phase
  line + role, Cancel while cancellable, "Cancellation requested…", opening / failed + "Open artifact"); every text
  byte-identical to React's `ARTIFACT_CREATION_PROGRESS_TEXT_V1`.
- Shell: `hubSelectArtifactKind`, `hubSetArtifactName`, `hubCreateArtifact`, `hubCancelArtifactCreation`,
  `hubOpenCreatedArtifact`; `open_hub_artifact_creation` on open space; `pump_hub_artifact_creation` (one bounded
  hub request per frame: submit / cancel / poll ≥ 100 ms apart, 120 s → `Indeterminate`, `409` → catalog
  re-read) from both targets' `pump_directory_events`; `Ready` opens the artifact through the ordinary
  `os.open-artifact` relay (dialect coordinate, schema, space) and records `Opened`/`Failed`.
- Laws (read React's own language-agnostic fixtures `🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/{🔣️.json,
  🪪️catalog-authority/🔣️.json}`, so React's `ArtifactCreationProgressNotice` is the independent twin):
  `the_creation_door_speaks_the_fixture_texts_in_both_tongues`, `every_fixture_creation_phase_paints_its_role_and_controls`,
  `every_fixture_catalog_phase_paints_its_line_and_choices`, `the_door_seals_one_intent_for_a_chosen_kind_and_a_valid_name`.

Live (05:42, `generated/door-live-1.txt`): law `a_live_hub_artifact_is_created_through_the_wgpu_creation_door`
against W2's hub 7800 (catalog A, `user1@semio.dev`): catalog `ready` with kinds `s.gis.gismap`, `s.note.note`;
`s.gis.gismap` → trail `accepted → preparing → ready` in 53.7 s, ready `artifact-db290b13c9dd5c1b223e3d38e156edfd`
(`gis.map`); the door's tree reports phase `ready` in en and de; opening `Failed` as expected (no native gis guest
staged in this shell). Verb `hub-live-creation-check` (nx target + launch row `⚖️gate🔐️hub-auth🧊️wgpu-live-creation`
before `…-wgpu-live-collaboration`, both launch files). The two-user law now creates its block2d document through
the door (step `4a-create-through-the-door`) instead of inventing a document id.

Open: `os.create-space-artifact` (the Space-index guest's own create dialog, React's route) is not yet wired to
the same operation on wgpu — it needs the Rust twin of React's `encodeArtifactKindChoice`/dialog kind choices.

## 5. Checks

| when | command | result |
|---|---|---|
| 01:5x | `cargo check -p semio-framework-os-kernel --features sync,ureq -p semio-framework-plugin-host -p semio-framework-os-run --lib` | green (kernel 4 pre-existing warnings; my 4 `unused_qualifications` fixed) |
| 01:5x | same with bins | `semio-framework-os-run` **bin** red in a peer file (`🏗️bootstrap/🦀️.rs:43/50` `store::Media: Serialize` missing) — not mine |
| 02:0x | `cargo check -p semio-framework-os-renderer-wgpu --lib --tests` | green (1 own break in the create-request shutdown law fixed; 129 / 281 pre-existing warnings) |
| 02:2x | `cargo check -p semio-framework --lib --tests` (transport session-0 fix + law) | green |
| 02:3x | `cargo check -p semio-framework-os-kernel --lib --features sync,ureq` (creation client) | green |
| 02:4x | `cargo check -p semio-framework-os-renderer-wgpu --lib --tests` (door + surface identity) | green (1 own missing import fixed) |
| 03:08 | wasm32: `semio-framework` + kernel `--target wasm32-wasip2`, kernel `--features sync --target wasm32-unknown-unknown` (store codec, transport session 0, creation client) | green (`generated/check-wasm-2.txt`) |
| 04:03 | same, after the exact frame decode | green (`generated/check-wasm-3.txt`) |
| 05:39 | same + renderer `--lib --target wasm32-unknown-unknown` (door, shell, typed ACK, all renderer edits up to 05:37) | green (`generated/check-wasm-4.txt`) |
| 05:28 | laws: kernel `--features sync,ureq` — B2 store + sync fixture laws, `decode_app_frame_refuses_trailing_bytes`, WG7's connect laws | 16 / 16 passed (`generated/laws-kernel-1.txt`) |
| 05:29 | kernel suites `os_spr::channel`, `os_store::sync`, `os_directory::client` | 190 / 190 passed (`laws-kernel-2.txt`) |
| 05:30 | `semio-framework` `ui_turn_patch_transport` (incl. the actor-0 law) | 10 / 10 with `--test-threads=1`; 8 fail in parallel — the suite shares the process-wide arena under `try_lock` (pre-existing, not the session-0 change) (`laws-framework-{1,2}.txt`) |
| 05:30 | plugin host `shard::` (incl. `Preempted` codec round trip) | 68 / 68 passed (`laws-plugin-host-1.txt`) |
| 05:31 | renderer `hub_connection::` (4 door laws + the verb-closure law updated for the 5 door verbs), typed-page laws, create-request shutdown law, slot-table budget | 37 + 14 passed (`laws-renderer-{1,2}.txt`) |
| 05:39 | native journey (staged post-H9 block2d release) | edit law **ok**, undo law red (§1.4) (`generated/journey-14.raw.txt`) |
| 05:42 | live door law on hub 7800 | **1 passed** (`generated/door-live-1.txt`) |
| 06:5x | `cargo check -p semio-framework-os-renderer-wgpu -p semio-framework-plugin-host --lib --tests` (§1.4 host drive) | green (only pre-existing warnings) |
| 07:0x | native journey runs 15–16 | undo red: first no step, then fault 2 of §1.4 named by the new retained-failure report |
| 07:1x | `cargo check -p semio-framework -p semio-framework-plugin -p semio-framework-plugin-host -p semio-framework-os-renderer-wgpu --lib --tests` (kernel constant, live-only seed) | green |
| 07:1x | `cargo test -p semio-framework --lib -- spawned_job` + TS `testSpawnedJobDriveContract` (bun) | 7/7 + TS pass (`reservedKind` asserted on both) |
| 07:1x–07:3x | native journey runs 17–21 (edit, undo, redo, select-all/copy/paste) | **4/4 green, 5 runs in a row** |
| 07:3x | `cargo test -p semio-framework-plugin-host --lib -- shard::` | 69/69 (`generated/laws-plugin-host-3.txt`) |
| 07:3x | `cargo check -p semio-framework-os-run -p semio-framework-os-renderer-wgpu --lib --tests` | green |
| 07:3x | renderer `kernel_runtime program_bridge typed_result` laws | parallel: 8 product-replay laws abort on a poisoned process-wide registry (pre-existing: shared registries); serially `kernel_runtime::semantic_document_tests` **34/34** (`laws-renderer-{3,4}.txt`); program-bridge + typed-page laws green |
| 07:3x | React package `typecheck` (PluginRuntime uses the kernel constant) | exit 0 |
| 08:0x | native journey runs 22–23 (outlier hunt, `[DEBUG] wg8` timing, reverted after) | 23: 4/4; 22: 3/4, the 4th open hit W2 rewriting block's component (above) |
| 12:1x–17:1x | two-user gate runs 1–18 on 7800 catalog B (captures from run 5 in `.🧬semio/🌐hub/s11-wg8-captures/`) | **run 18: 12/12, exit 0** (§3) |
| 16:5x | `cargo test -p semio-framework-os-kernel --lib --features sync,ureq -- os_store::sync os_store::component` | 448/448 ×2 (one earlier run: `retained_readiness_wake_after_turn_release_is_observed_once` missed its 1 s deadline under load; alone and ×3 in the suite green) |
| 17:1x | native journey run 24 (genesis-seeded opens) | 4/4 (`s11-wg8-captures/journey-24.raw.txt`) |
| 17:1x | renderer `chrome_maintenance sync_card hub_projection_workspace_tests hub_connection:: sync_panel presence` | 71/71 (`laws-renderer-5.txt`) |
| 17:2x | plugin host `shard:: component` | `shard::` 69/69; `component::` 220/225 — the 5 failures sweep stale staged components on disk (`target-g8/…/semio_s_plugin_note.wasm`: pre-H9 owned ABI; `replay-envelopes` missing), not these edits (`laws-plugin-host-4.txt`) |
| 17:2x | wasm32 gates (`wasm-checks-2.sh`: framework+kernel wasip2, kernel `sync` unknown-unknown, renderer unknown-unknown) | queued in the wasm mutex behind W2's `warm rest` hold and WG7 (`s11-wg8-captures/check-wasm-5.txt`) |

## Files (WG8)

Paths relative to `🧰️framework/🛍️products/💻️os/🔨️modules/` unless absolute.

- `🔌️plugin/🖥️host/🧵️shard/🦀️.rs` — `ShardOutcome::Preempted` (+ codec round-trip law row in `🧪️tests/🔬️unit`);
  §1.4: `ShardOutcome::Turn { jobs }`, `ShardOutcome::actor()`, live-only reserved seeds (`replayable`), replay refusal;
  law `a_framework_reserved_spawn_starts_live_hands_its_turn_to_the_host_and_refuses_replay` in `🧪️tests/🔬️unit`.
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/{🦀️.rs,🟦️.ts}` — `FRAMEWORK_RESERVED_JOB_KIND` (Rust + TS twin);
  fixture `🧫️fixtures/🧵️spawned-job-drive/🔣️.json` (`reservedKind`) + both runners in `🧪️tests/🧵️spawned-job-drive`.
- `🔌️plugin/🦀️.rs` — `app::FRAMEWORK_RESERVED_JOB_KIND` re-exports the kernel constant.
- Gate (§3): `🏪️store/🔄️sync/🦀️.rs` — `document_socket_io_reactor` (+ law in `🧪️tests/🔬️native-actor-retained-turn-fixtures`);
  `🏪️store/🦀️.rs` — `ComponentDocumentCodec::genesis`, `ComponentDocumentGenesis`, `component_document_genesis` (+ law in
  `🧪️tests/🔬️unit`, `FixtureComponentCodec::genesis` in `🔄️sync/🧪️tests/🔬️document-socket-connect`); `🔌️plugin/🖥️host/🧬️component-codec`
  — `genesis`; `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `open_document` opens on the component genesis, `pump_sync_events`
  refreshes guest bodies only on document changes; `🐚️Shell/🧪️tests/🔗️hub-projection-workspace` — `frame_pump`, `hub_verb`
  via `drive`, step 12 ingest window and relative freeze bound.
- `📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` — the React reserved drive reads the kernel constant.
- `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎠️runtime/🦀️.rs` — `ParallelRuntime::take_shard_failure`.
- `🏃️run/🦀️.rs` — resumes a preempted actor.
- `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — `run_turn` settle rules, preemption resume, quiescence,
  `carries_nothing`, `absorb` keeps non-idle ingress, typed-operation pages on `ExchangeOutcome::typed_results` + in-settle
  ACKs (dead exchange/request/ACK plumbing deleted), `create_app(.., artifact_schema)` + component codec registration;
  G7w's `[DEBUG] g7w` logging removed; §1.4: `ReservedToolJob`, `settle_turn`, `settle_reserved_jobs`, `admit_reserved_jobs`,
  `run_reserved_job_once`, `dispatch_turn` (grant/owed tally, every turn result applied), reserved spawns out of `effects`.
- `📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` — `kernel_surface_id`, `create_app` passes the
  app's `io.artifact_schema`.
- `🎠️kernel/🦀️.rs` (`/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🦀️.rs`) — transport session 0 admitted;
  law in `🧪️tests/🔬️ui-turn-patch`.
- `📡️spr/🧵️channel/🦀️.rs` — exact `decode_app_frame`; law in `🧪️tests/🔬️unit`.
- `🏪️store/🦀️.rs` — `ComponentDocumentCodec`, `DocumentKindCodec`, `register_component_document_codec`,
  `document_kind_codec`; law in `🧪️tests/🔬️unit`.
- `🏪️store/🔄️sync/🦀️.rs` — `document_pack_schema_hash` asks `document_kind_codec`; native actor connect/finish/bootstrap
  identity + bootstrap validation + folder mirror persistence use it; fixture `🏪️store/🧫️fixtures/document-socket-connect-v1`
  (`componentIdentity`) + runner law.
- `🔌️plugin/🖥️host/🧬️component-codec/🦀️.rs` (new) + mount in `🔌️plugin/🖥️host/🦀️.rs` — `OwnedComponentDocumentCodec`,
  `GUEST_CODEC_BUDGET`.
- `📇️directory/🔌️client/🌱️space-artifact-creation/🦀️.rs` (new) + mount in `📇️directory/🔌️client/🦀️.rs`.
- `📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs` — creation door state, texts, UI; laws in
  `🔗️HubConnection/🧪️tests/🔬️wgpu-unit`.
- `📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — door verbs, catalog load, frame-pumped creation
  operation, opening through the relay.
- `📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs` — native journey laws (edit, undo, redo,
  select-all/copy/paste), live door law, two-user law creates through the door and undoes with the fixture's verb.
- `📺️renderer/🧑‍🎨engine/🧫️fixtures/⏯️native-guest-journey/🔣️.json` (new; `redo`, `clipboard`, expected ledgers added in §1.4).
- Tests adjusted: `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs` (create-request
  shutdown law, `typed_results`, budget rows), `🗞️typed-result-page/🦀️.rs` (page bound law moved in),
  `🧪️tests/🔬️wgpu-renderer-kernel-runtime-typed-operation-result-exchange/` (deleted with the exchange),
  `🧱️elements/🌉️ProgramBridge/🧪️tests/🕹️wgpu-reserved-verb-answer/🦀️.rs`; fixture row removed from
  `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json`.
- `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json}` — verbs
  `native-guest-journey-check`, `hub-live-creation-check`; `/Users/ueli/Documents/semio/.vscode/{launch.json,🧩️launch.seed.jsonc}`
  rows `⚖️gate🔐️hub-auth🧊️wgpu-live-creation`, `⚖️gate🧊️wgpu⏯️native-guest-journey`.
- `native-guest-journey-check` (renderer package `📜️script.ts`) runs the four journey laws.
- Ticket-local: `wp-wg8/{run-native-journey.sh,run-door-live.sh,block-release.sh,wasm-checks.sh,wasm-checks-2.sh,edit-*.py}`
  (`edit-debug-*.py` apply/revert the temporary `[DEBUG] wg8` timing; reverted, 0 lines left).

## Processes

- `75765` journey run 1, `24132`/`33624`/`44282`/`52569`/`62155` journey runs 2–6 (each exits by itself).
- `87049` block2d `materialize-release` (exited 01:55, rc 0; component sha256 `7d4bb1ed…`, native runtime republished).
- `44189`, `73318`, `9716` wasm32 checks (all exited green). `2629`, `3335`, `6859`, `9609` journey runs 11–14 (exited).
- Journey runs 15–24 (each exited by itself). Gate runs 5, 7, 8, 12, 13, 14, 16, 17 hung after their ledger (test harness
  `block_on`, §3); stopped by pid 17:0x (zsh + cargo + test binary of each). Run 18 exited by itself. My block
  `materialize-release` mutex waiter (11:52) was cancelled after the descriptor turned out rematerialized at 12:18.
- `wasm-checks-2.sh` queued in the wasm mutex (17:17, detached).
- No hub or serve started by WG8 (the live door law used W2's hub 7800 as a client).

## Log

- 00:5x start; read preambles, audit §3/§6, G7w, N2 §6.4, TC3b, WG7 skeleton.
- 01:05 guest request `wp-w1/requests/wg8.txt` (block in the full catalog + post-H9 release component).
- 01:2x journey run 1 → owned-ABI refusal (pre-H9 guest). 01:36 block2d rebuild launched (coordinator allowed).
- 01:4x–02:0x B1 + B2 source landed, native checks green.
- 05:2x resumed after the outage/usage cut; journey run 10 (spinning 83 min, coordinator notice) sampled and killed (my pids).
- 05:3x–05:4x typed ACK in settle, absorb ingress fix, frame decode, laws run, `[DEBUG] g7w` removed, B1 edit law green,
  live creation door green on hub 7800.
- 06:0x coordinator notice on note's `dist/component-dev` rebuilt outside the mutex at 05:39: **not WG8** — WG8's only guest
  build is block `materialize-release` through the wasm mutex (01:36–01:55, `generated/block-release-1.txt`); WG8's journey
  runs build only the renderer test binary. The native runtime needs the block **release** component (the 88 MB dev one
  exceeds the 64 MiB execution-target bound); WG8 republishes it from W2's `component-release` once W2's final pass lands it.
- 10:2x–17:2x gate: W2 catalog B publish failed twice (10:39 codec probe, 11:11 space-creation JSON), published 11:44, 7800
  ready 11:50, killed by the 12:19 low-disk cleanup, back 12:45 (runId `345ceda4…`); gate runs 1–18, five fixes, run 18 green.
- 06:4x–07:4x §1.4: shard reports admitted job turns, renderer drives reserved jobs (runs 15–16 red: seed checkpoint
  overflow found through the new retained-failure report), live-only reserved seeds + kernel constant, redo and
  select-all/copy/paste laws; runs 17–21 all green; 2 latency outliers timed, not recurring.

## 6. Next (for the coordinator)

1. **Done: the gate is green on catalog B** (§3). Was: **Catalog with block, early.** Ask W2 to publish a catalog that carries block as soon as block's `component-release`
   is built (e.g. `--packages stdio,gis,note,draw,writer,puzzle,block` into a copy served on WG8's hub 8090, or on
   7800) instead of after all 34. Then: `@semio-tech/block-plugin:materialize-release` through the wasm mutex (so the
   release descriptor matches W2's component) and `zsh .tmp-ticket/wp-wg8/run-collab-live.sh` (optionally
   `SEMIO_HUB_LIVE_ORIGIN=http://127.0.0.1:8090`). Expected: steps 1–12 measurable, 11 green since §1.4.
2. **§1.4 landed.** Left open by design: a product (non-reserved) spawn of a guest whose checkpoint outgrows the fixed
   replay checkpoint pages (block2d's does; other guests not measured) still closes its seed — the product-replay owners
   should size or page that checkpoint; the failure is now named in the fault instead of lost. Pre-existing: the renderer's product-replay
   laws share process-wide registries and abort in parallel (serially green); an unmounted stray copy of the shard unit
   tests lives at `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🦀️.rs` (compiled by no crate).
3. `semio-framework` `ui_turn_patch_transport` laws fail when run in parallel (shared process-wide arena under
   `try_lock`); they pass with `--test-threads=1`. Pre-existing; worth a serial guard.
4. `os.create-space-artifact` (the Space-index guest's create dialog) on wgpu → the same door operation (needs the Rust
   twin of React's kind-choice encoding).
5. `wp-wg8/target` is kept (coordinator); it is no longer needed for the gate.
6. **WG7 / React parity (not measured by WG8):** a fresh door artifact answers the document socket `Bootstrap::None`, so any
   host whose guest opens without the document's genesis authors under the wrong document id. Worth one check on the wasm32
   wgpu shell (WG7) — `open_document` is target-neutral, so it now seeds there too once a component codec is registered —
   and on React. Presence is timer-driven in React already. The native `os.open-artifact`
   open inside the frame pump (~20 s in debug) wants a retained operation (§3).

