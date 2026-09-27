# WP-WG10 — Collaboration in the Native wgpu Shell (Session 13)

Slice WG10 (session 13, 2026-09-26 19:0x), successor of WG8 (handover [`📓️wp-wg8.md`](📓️wp-wg8.md), captures
`.🧬semio/🌐hub/s12-wg8-captures`). Ports: hubs 8090–8099, serves 6590–6599 (keeps React serve 6590, pid 35215 → vite 35814).
Private cargo target: `.tmp-ticket/wp-wg10/target`. Captures: `wp-wg10/generated/` (expendable), durable
`.🧬semio/🌐hub/s13-wg10-*`. Hub 7800 = W3 (client use only).

## Session 13

| # | Item | Status |
|---|------|--------|
| 1 | S12-6 genesis-on-open parity native ↔ React ↔ wasm32 (one kernel primitive, fixture law, live) | **LANDED + LIVE (14:20, 7800 B3)**: both native shells of the gate opened a fresh door artifact (`artifact-e66fe6df…`) through the canonical-pair seed (the only seed left: a failed pair fetch fails the open, and an unseeded guest authors under its app id, which the hub refuses) — Live in 15 ms, A's/B's edits accepted, per-actor undo. Kernel laws 11/11, TS 26/26, wasm32 green |
| 1b | Landing: WG8's `kernel-patch-transport-deadline.py` (coordinator, WG10-1) | **LANDED**: native + wasm32 green, laws **2/2** |
| 2 | Full cross-shell journey native ↔ React `s` on one hub document on 7800, en + de, measured — incl. the live CURSOR leg on a board-canvas kind (audit s13 P1-5) | **edits leg 8/8 PASS on 7800 B3 (15:09, en)**; **cursor leg RED (15:26, puzzle2d)**: door `ready` after 649 s, both shells opened the board but neither socket reached Live (native `Detached`, React `live:false`, sockets cycling every ~30 s, no hub error event) → cursors not published (A saw no React cursor). Hypothesis (not measured): hub fan-out lag → `RebootstrapRequired` + close 1013 → the actor's dead "pair inside the Welcome" rule (LD 15:2x) — the item-6 patch. de run: pending |
| 3 | `hub-live-collaboration-check` 12/12 on 7800 after the all-package catalog + more kinds (board canvases, text) | **12/12 EXIT 0 on 7800 B3 (14:20, block)** (`s13-wg10-captures/collab-live-b3-2.txt`); run 1 aborted with a test-thread stack overflow in step 8 (2 MiB default thread, debug) → run 2 with `RUST_MIN_STACK=8388608` (macOS main-thread size). ALL catalog + other kinds: after the final publish |
| 4 | Native accessibility (AccessKit tree for chrome + keyboard traversal), measured, root fixes | **LANDED: native + wasm32 green, laws green**: AccessKit bridge behind `♿️native-accessibility` (chrome + every visible document window; AT actions → shell accessibility events), keyboard ring (Tab/Shift+Tab over the chrome's focusable publication then windows, Enter/Space activate, painted focus ring), Cargo.lock +20 packages. Renderer laws **29/29**, TS **3/3**. Measured gap (source): natively Tab only cycled dock windows, no chrome control was keyboard-reachable, nothing painted keyboard focus, no platform accessibility tree existed. Live macOS VoiceOver/AX measurement: pending (needs a native app run) |
| 5 | Presence/ephemeral wire as ONE kernel primitive + TS twin + fixture law (audit s13 P2-2); re-verify a native late-join shows full history after WG9's echo-suppression set | pending (after 1–4) |
| 6 | Coordinator (15:3x): native/wgpu shell rebootstrap from the pair route (LD item 3 Rust twin) — prepared patch for window 3, law red→green, live repro plan, wasm32 half with WG9 | **PREPARED, dry run clean (15:34, not applied)**: `wp-wg10/patch-rebootstrap-reseed.py` (11 files). Compiles: unverified until window 3 (tree frozen). WG9 told (wasm32 live check) |

### Log (session 13)

- 19:07 start; read AGENTS.md, preambles 13/12, `📓️wp-wg8.md`. Load 24, 111 GiB free, 4 rustc. 7800 ready (B2),
  6590 answers 200 (pid 35215, vite 35814), 8050/6552/6553 are WG9's. WG8's last unreported work (18:12–18:35, scripts
  `wp-wg8/edit-kernel-fair-turns*.py`, `edit-kernel-compile-gap.py`): fair kernel turns (a preempted actor yields its slice,
  only an explicit cancel ends it) and the component parse off the kernel loop are in the tree; its `[DEBUG] wg8` timing
  (`edit-debug-create-app-timing.py`) is not (0 lines).
- 19:1x **Item 1 design (coordinator-approved 19:2x).** Three shells, two mechanisms: React seeds a hub open from the hub's
  canonical checkpoint pair (`GET /spaces/{s}/documents/{d}/active-checkpoint/pair`, `seedColdPairFromCanonicalCheckpoint`,
  C10 item 1); native (owned interpreter) and wasm32 (jco, WG7) minted `codec.genesis` locally
  (`store::component_document_genesis`). The local mint is only right while the document's active checkpoint IS its genesis:
  after a Check In the hub's active checkpoint moves, and a cold native client that says Hello with no frontier is tailed from 0
  (or, once the db floor moves, refused as a database-private Snapshot). One kernel primitive = the pair: Rust decoder + admission
  beside the TS twin in `📇️directory/🧬️schema`, one directory-client fetch on every transport, and the document actor starts at the
  pair's baseline. There were already THREE Rust decoders of this wire (hub `lag_rebootstrap`, MCP `🧩️pair`, none in the kernel);
  the kernel one is the shells' — folding the hub's and the MCP's copies onto it is a follow-up (noted, not done).
- 19:2x–19:5x **Item 1 source** (all compile-atomic intent; the convoy delayed the checks, see below):
  - schema-first contract: schema `💻️os/🧬️schema/🪢️canonical-checkpoint-pair-v1/🔣️.json`, fixture
    `💻️os/🧫️fixtures/📇️directory/🪢️canonical-checkpoint-pair-v1.json` (2 pairs incl. a 3-record check-in pair, 15 refusals at
    the decode or digest stage, 7 admissions) generated by an independent Python encoder + hashlib
    (`wp-wg10/gen-canonical-pair-fixture.py`);
  - kernel `📇️directory/🧬️schema/🪢️canonical-checkpoint-pair-v1/🦀️.rs` (new): `decode_canonical_checkpoint_pair_v1` (framing +
    pack/SPR/aggregate digests), `CanonicalCheckpointPairV1::admit(scope, &DocumentOpenCheckpointV1)`, refusal codes identical to
    the TS names; law `🧪️tests/🔬️unit` (4 tests over the fixture);
  - kernel client `📇️directory/🔌️client/🪢️canonical-checkpoint-pair/🦀️.rs` (new): `document_canonical_checkpoint_pair(ctx, scope,
    expected)` — exact route + `Accept`, transient statuses (React's list) asked again ≤ 3, bounded, decoded, admitted;
    `DirectoryTransport::get_accepting` (new trait method; native pool, kernel browser `fetch`, the wgpu door, 3 test doubles);
    client law `the_canonical_pair_is_fetched_verified_and_admitted_as_the_authorized_checkpoint`;
  - kernel store sync: `ArtifactHost::set_document_seed(key, pair)` (consumed by the next `open`, like the lease); native actor
    `seed_hub_document` (history known, folder persisted, `server_frontier` = the pair's baseline so `SocketHelloV1` asks only for
    what followed it; setup no longer overwrites a seed with a stale folder archive); wasm actor starts with the same frontier;
  - wgpu Shell (both targets): the open's execution-target lease `checkpoint` (from the hub resolution, else read in
    `bind_document_execution_target`) + the signed-in client make a `ShellHubDocumentSeed`; `seed_hub_document` fetches the pair
    detached (cancellable with the open), loads it into the guest, and the actor gets the same pair before it opens;
  - wasm32 door: `directory-http` gains `accept` and a `bodyBase64` answer (Rust `📇️directory-door` + page `🚪️host-io/🟦️.ts`) + 3 door
    laws; the pair route needs a bearer, which the wasm32 shell has after its hub sign-in (WG7 run s12a);
  - TS twin: `admitCanonicalCheckpointPairV1` beside the decoder; the decoder now refuses an empty SPR (the hub producer and the MCP
    decoder already did); React's worker admits through it (all five checkpoint fields, was scope + checkpoint id);
  - **retired** (only hub opens used it; local/folder documents never called it — coordinator's local-first constraint holds):
    `ComponentDocumentCodec::genesis`, `ComponentDocumentGenesis`, `component_document_genesis` + its law, the native host
    adapter's and the wasm32 jco codec's `genesis`, the JS bridge `codecGenesis`/`codecPair` (`🐚️plugin-bridge/🟦️.ts`; the
    generated frame worker still carries the old line until the next `generate-frame-worker` — **overlap with WG9's area, noted**).
    The guest's WIT `codec.genesis` export stays: the hub's trusted catalog mints every artifact's genesis through it.
  - TS law `💻️os/🧪️tests/🪢️canonical-checkpoint-pair/🟦️.ts`: **26/26** (Ajv schema, constants, 2 pairs, 15 refusals, 7 admissions;
    `node:crypto` digest oracle) — `wp-wg10/generated/ts-pair-1.txt`.
- 19:39 kernel `--lib --tests --features sync,ureq` check rc=0 (`check-kernel-1.txt`) — **void per coordinator rule 24** (fingerprints
  pointed into a scratch clone before 20:14); re-running.
- 20:0x rule 22: stopped the inherited idle React serve 6590 (pid 35215 + its vite 35814); restart later with
  `wp-wg8/serve-react.sh 6590 <hub>`.
- 20:1x native check 2: red only in peers' in-flight edits (`📡️spr` import of a replication fn being added, `🔲️pixels` png encoder);
  check 3 queued 27 min without a rustc child in the fleet's cargo convoy (19 cargos) → stopped (rule 25). Check 4 detached 20:4x (RC=101: kernel lib + plugin host green; kernel tests red only in WG9's new `🔬️document-echo-suppression` test — missing `observed`/`target`; renderer blocked by a peer's `semio-framework-surface` Affine edit). Check 5/6 killed by the convoy (rc 143). Rule 26: my cargo now runs on `build-fleet-b` (cold).
- Found: a WG8-era wasm mutex wrapper **pid 90450** (`fleet-mutex.sh wasm wg8`, started ~11:15, 9 h 26 m) still runs its `zsh -c`
  wasm32 checks outside the lock (W3 holds `/tmp/semio-wasm-build.lock` since 20:03). Not WG10's to kill (rule 8) — reported to main.
- 20:5x **WG10-1 landed in source**: `wp-wg8/kernel-patch-transport-deadline.py` dry run (write-suppressed replay) clean on the current tree
  → applied as `wp-wg10/kernel-patch-transport-deadline.py` (copy): native ureq agent bounds only the connect; each request's overall
  timeout = what the caller's `OperationContext` deadline leaves (runtime clock), else `UREQ_HTTP_UNBOUNDED_REQUEST_MS` (120 s);
  fixture `🪪️runtime/🧫️fixtures/⏱️request-budget.json` + 2 laws. Compile pending (same check as item 1).
- 21:0x **Item 1 seed law**: `a_seeded_hub_actor_says_hello_at_the_canonical_pair_baseline` (sync unit tests; the loopback socket
  helpers `connect`/`receive_frame` lifted to module level, shared with the epoch law).
- 21:1x **Item 4 (native AccessKit) source**:
  - deps (renderer crate only; native targets): `accesskit 0.25.1`, `accesskit_winit 0.34.1` (`rwh_06`; Linux `accesskit_unix` +
    `async-io`), dev `accesskit_consumer 0.39.1` (oracle). `cargo metadata` re-resolved `Cargo.lock`: +20 packages, zbus 5.17→5.19,
    zbus_macros, zbus_names 4.3.3→4.3.4, zvariant 5.13→5.15 (+ derive/utils) — only `ashpd` (Linux) uses them. Before-copy:
    `wp-wg10/generated/Cargo.lock.before-accesskit`. Main told.
  - `🎯️targets/🧊️wgpu/♿️native-accessibility/🦀️.rs` (new, native): `NativeAccessibilityPublication` (title + windows, published per
    presented frame, versioned only on change), `native_accessibility_tree` (Window root named by the brand title, one Group/Pane per
    window, nesting rebuilt from `depth`, roles mapped like Chromium maps ARIA, names/description/shortcut/state/numeric/live/bounds,
    Focus/Click/SetValue actions from the node's flags, FNV-derived stable ids), `action_dispatch` (Focus/Blur/Click/SetValue →
    `DispatchEvent::Accessibility` on window + generation + node id + key), `NativeAccessibilityBridge` (AccessKit adapter with direct
    handlers: initial tree answered synchronously, actions queued + event-loop wake).
  - Interpreter: `build_accessibility_dump` (+ its structs, `dump_window_ids`) no longer test/wasm32-only; `native_accessibility_windows()`.
  - Shell: `acknowledge_presented_input` publishes the native tree after the chrome publication; one `crate::boot_window_title()`
    (the window title and the platform root's name; the winit window used a private copy of the same rule).
  - winit app: the native window is created hidden, the bridge attached, then shown; every window event goes to the adapter first;
    `about_to_wait` refreshes the platform tree and dispatches queued AT actions through `OsHost::handle_event`.
  - fixture `🧑‍🎨engine/🧫️fixtures/♿️native-accessibility-tree/🔣️.json` + schema `🧬️schema/♿️native-accessibility-tree` + law
    `🧪️tests/♿️native-accessibility` (5 laws: consumer-tree roles/names/state/actions/focus, the role table, actions → shell events,
    id stability, publication versioning).

- 21:1x (before the cut) item 4 laws: Rust `🧪️tests/♿️native-accessibility` gained `only_the_bridge_module_names_accesskit` (coordinator:
  AccessKit approved behind the one module; a walk over every renderer `.rs` asserts no other file names the crate); TS runner
  `🧪️tests/♿️native-accessibility/🟦️.ts` **3/3** (`ts-a11y-1.txt`: Ajv schema; every role `uiAccessibilityRoleV1` or the chrome can
  announce has a platform role in the table). Z3 told about the Linux deps (acknowledged).
- ~21:30 cut by the usage limit (coordinator rule 28: every process died overnight). Nothing of mine was running a hub/serve.
- 04:58 resumed. Reconciled: every item-1 / transport-deadline / item-4 edit is on disk (`git diff` + greps), Cargo.lock carries
  the 8 accesskit packages; the keyboard-ring edit had NOT been applied (its last anchor had moved) → anchor fixed, applied
  (`wp-wg10/edit-shell-keyboard-ring.py`): Tab/Shift+Tab walk the chrome's focusable/enabled/visible accessibility publication then
  the dock windows (`keyboard_ring`, `advance_keyboard_ring`), Enter/Space press the ring's control through
  `handle_accessibility_event(Activate)`, overlay phase 10 paints the focus ring (`theme.focus_ring`) around its hit rect; the dead
  `cycle_active_window` deleted. Law `🐚️Shell/🧪️tests/⌨️native-keyboard-ring` over fixture `🧫️fixtures/⌨️native-keyboard-ring`.
  Measured before the fix (source reading, 21:2x): natively Tab only cycled dock windows; no chrome control was keyboard-reachable
  and no keyboard focus was painted; the chrome accessibility projection existed only for the browser mirror.
- 05:0x native check 9 (kernel `sync,ureq` + plugin host + renderer + MCP, lib + tests, `build-fleet-b`, cold) running.
- 05:26 check 9 deadlocked 23 min in build-fleet-b (`sample`: `prebuild_lock_exclusive` → `flock`, 0 rustc child) → killed; rerun on a
  private check-only build-dir `.🧬semio/🦑️repo/⚡️cache/cargo/build-wg10` (coordinator deletes it at REBUILD START).
- 05:47 **native check GREEN** (`check-native-10.txt`, 20 min cold): os-kernel `sync,ureq` + plugin-host + renderer-wgpu + os-mcp,
  `--lib --tests` (warnings = type-checked; my 4 new warnings fixed: 2 in the runtime law, 2 in the keyboard-ring law).
- 05:59 **kernel laws 11/11** (`laws-kernel-1.txt`): canonical pair limits / 2 pairs / 15 refusals / 7 admissions, client fetch
  (route, `Accept`, bearer, 503 asked again, 401/404/503 named), `a_seeded_hub_actor_says_hello_at_the_canonical_pair_baseline`,
  codec registry laws without `genesis`, the transport-deadline laws, the epoch law (helpers lifted).
- 06:0x landing rows (3) in `📓️landing.md`, `wp-w3/requests/wg10.txt` written. wasm32 checks queued 5th in the mutex
  (`check-wasm-1.txt`); rule 29: native + laws suffice for the row, wasm32 is REBUILD's fast gate — leave the queue at 06:50.
  Renderer laws building (`laws-renderer-1.txt`).
- 06:0x–06:34 renderer laws: run 1 (parallel) 26/29 — the two Shell seal laws (mine + a peer's pre-existing
  `chrome_accessibility_dispatch_validates_current_identity_and_activates_once`) refused `retained presented input candidate could not be
  sealed` (process-wide seal state across parallel tests, WG7's known case) and the door source law
  `the_directory_command_queue_compiles_on_the_browser_target` was red: WG8's native-only `SHELL_DIRECTORY_NETWORK_BYTES_PER_MINUTE`
  sat inside the shared `🎮️DirectoryCommandQueue` region → moved beside `shell_directory_transport`. Runs 2–3 `--test-threads=1`: ring law
  harness fixes (retire the previous frame's hits like `InputFrame` does; Enter on the search toggle opens the palette and moves focus
  into its query field, as React does → fixture step says so). **Run 4: 29/29** (`laws-renderer-4.txt`).
- 06:28–06:34 **wasm32 GREEN ×3** (`check-wasm-1.txt`, mutex hold): kernel `--target wasm32-wasip2`, kernel `--features sync --target
  wasm32-unknown-unknown`, renderer `--target wasm32-unknown-unknown` — covers item 1, the transport patch and item 4's lib code.
- 06:20 hub 8091 (C11 recipe: B2 clone + 19:07 current-tree `os-hub`, `wp-wg10/wg10-hub.sh`, hold 51503 / hub 51505, ready ~06:30) and
  React serve 6590 → 8091 (pid 59118, detached via `wp-w2/w2-detach.py`) started for a cursor-leg run; cut ~06:35 before it ran.
- 09:55 resumed (rule 30: REBUILD START 09:53, build-wg10 deleted). Landing rows updated with wasm32 + law evidence; requests file
  `wp-w3/requests/wg10.txt` stands. Rule 22: stopped hub 8091 (hold 51503 → hub exited) and serve 6590 (59118 + vite 59161) —
  idle until 7800 is on B3. Waiting for the coordinator; on call for chain errors in kernel/renderer.
- 13:58 resumed (rule 31: 7800 READY on B3). Renderer test binary rebuilt twice on build-fleet-b (the 2nd picks up G11's 14:03 kernel
  socket-grant empty-body fix), durable copy `.🧬semio/🌐hub/s13-wg10-bin/renderer-tests`; cargo-free runner `wp-wg10/run-live-law.sh`.
  React serve 6590 → 7800 started (`s13-wg10-logs/serve-6590-b3.txt`, w2-detach).
- 14:13 gate run 1 on 7800 B3 (`collab-live-b3-1.txt`): 1–7 PASS (door `accepted → ready` in 160 s, both opens 9.0–9.7 s, Live 8 ms,
  presence both ways) then **`has overflowed its stack` → SIGABRT in step 8** (A's `addHandleKind` dispatch → render). The law thread
  is libtest's 2 MiB default; debug async frames of the dispatch/render path now exceed it (WG8's runs passed on the same law). The app's
  shell runs on the 8 MiB main thread. Finding, not root-caused (which frame grew is unmeasured) — watch item.
- 14:16–14:20 **gate run 2 (`RUST_MIN_STACK=8388608`): 12/12 PASS, EXIT 0** (`collab-live-b3-2.txt`, 231 s): 4a door `accepted →
  preparing → ready`, both native opens seeded by the canonical pair, 6 Live 15 ms, 7 presence both, 8 A authors 1.9 s, 9 B ingests,
  10 B authors 2.0 s + A ingests, 11 per-actor undo (A's own `addHandleKind` false, B's `apply` untouched), 12 offline edit 1.9 s
  (≤ 2× online), pump 3.0 s, stale → ready, relive 4.2 s, A ingests the offline edit. = item 1 live + item 3 (block) on B3.
- 14:18 renderer laws 29/30: the new `a_painted_navbar_hands_the_platform_a_named_tree` measured census {3 nodes, 1 focusable, 0
  unnamed, 0 unknown roles} but its `nodes ≤ painted controls` bound was wrong (the chrome also announces status nodes) → law now
  paints 3 panel tabs and asserts each reaches the platform tree, focusable, named, known role (rerun `laws-renderer-6.txt`).
- 14:20–14:50 usage cut (whole fleet). 14:56 resumed.
- 14:57 React serve 6590 restarted (the 13:59 one predated the 14:1x taxonomy break/restore), `serve-6590-b3-2.txt`.
- 14:57–15:09 **CROSS-SHELL (edits, block2d) on 7800 B3: 8/8 PASS, law exit 0** (`s13-wg10-captures/cross-shell-b3e1/`, 684 s; runner
  `wp-wg10/run-cross-shell.sh`, durable binary, ONE headless browser): 1 A (native) signs in; 2 A creates a space + a block2d artifact
  through its door and opens it (seeded by the canonical pair), Live at once; 3 B (React `s`) opens it, Live 4.2 s, `hub=live`;
  4 presence both ways (A sees `User Two` 3.3 s, B sees A 3 ms after the handshake, one wire `peer:hub.v1.…`); 5 A edits 4.3 s → React
  6→7 Handle Kinds 2 ms after; 6 React edits → A's ledger ingests; 7 each undoes own (A's undo reached React, React's own undo 34 ms,
  A sees B's undo 337 ms); 8 React reload → Live 4.9 s, converged, A's next edit reached it 10 ms after.
- 15:00 renderer laws run 6: blocked by a peer's in-flight plugin-host edit (`codec_replay_envelopes_observed` missing its new
  `&GuestCallCancellation` argument, E0061, landing window 2) — not mine; rerun once it compiles.
- 15:09 cross-shell CURSORS (puzzle2d board) run `b3c1` launched.
- 15:09–15:26 **cross-shell CURSORS (puzzle2d, `b3c1`): 3 of 5 red.** 1 A signs in ✓; 2 A's door followed the puzzle creation to `ready`
  (649 s) and the open settled `Opened` (49 s, frames ≤ 0.92 s), but the document socket never reached Live (`remote=Detached`);
  3 React opened the board, `hub=live`, socket received 18 frames, never Live (console: `The current app has no registered board
  session factory` ×3 — React side, routed); 4 presence both ways ✓ (the roster is the space-wide stream); 5 A published views for
  `2d-detail`/`2d-overview`/`2d-selection` with a pointer but saw no React cursor. Hub log for the document: 11 socket upgrades, each
  `cancelled … closed` after ~30 s, presence join/leave, no error. Hub source: a socket whose fan-out receiver lags gets
  `RebootstrapRequired` + close 1013 (`🌎️hub/🏗️bootstrap` `RecvError::Lagged` arm) — with a busy board this matches LD's 15:2x finding
  (the native actor then waits for a pair inside a Welcome the hub never sends). Not measured yet: whether this run's sockets saw
  `RebootstrapRequired` (the native law does not log actor status) — the item-6 law + repro plan measure it.
- 15:2x LD + coordinator: item 6 assigned (prepared patch; tree frozen after 15:45).
- 15:3x **item 6 prepared** — `wp-wg10/patch-rebootstrap-reseed.py` (dry run: every anchor exactly once on the 15:34 tree, incl. WG9's
  15:18 lost-Ack edits in `🔄️sync`; `--apply` writes + regenerates the fixture). Design (one mechanism with item 1's open seed; the
  actor cannot fetch the pair — its grant source is a sync/`!Send` seam — the shell's concrete client can):
  - kernel `🪢️canonical-checkpoint-pair-v1`: `CanonicalCheckpointPairV1::admit_rebootstrap(&RebootstrapRequired)` (scope, checkpoint id,
    descriptor digest, baseline; the control carries no aggregate — the decoder proved the digests); client
    `rebootstrap_canonical_checkpoint_pair(ctx, control)` over one shared `fetch_canonical_checkpoint_pair` (the open's
    `document_canonical_checkpoint_pair` now uses it too);
  - kernel `🔄️sync`, both actors: `RebootstrapRequired` → drop projection + socket, keep unacked work, emit
    `ArtifactEvent::RebootstrapRequired { control }`, no dial while waiting (`start_connect_hub`/`connect` gated), no outage backoff;
    the dead "pair inside the Welcome" refusals in the None/Tail arms deleted; `ArtifactActorMsg::Reseed { pack, spr, baseline }`
    (+ mailbox byte bound) → native: `install_hub_seed` (shared with the open's `seed_hub_document`), unacked ops the pair lacks go back
    to the guest as `RemoteMutations`, dial at once (Hello at the baseline); wasm: Hello frontier = baseline, ops back, dial next turn;
  - Shell (both targets): `ShellSyncReseed` + `advance_sync_reseed` in the frame pump — detached pair fetch, verify, `load_app_document_pack`,
    `Reseed` to the actor; transient refusal asked again after 1 s; closing the document drops it;
  - laws: fixture `rebootstrapAdmissions` (6 rows, generator extended; schema updated) → Rust schema law + TS twin
    `admitCanonicalCheckpointPairForRebootstrapV1` in the TS runner; kernel client law `a_rebootstrap_pair_is_admitted_only_as_the_controls_checkpoint`;
    native actor law `a_rebootstrap_waits_for_the_hosts_reseed_and_says_hello_at_its_baseline` (red on today's tree: no event is
    emitted and a later None Welcome is refused); backbone-parity event kind `rebootstrapRequired`.
  - **Live repro plan** (after window 3 + rebuild): (1) cross-shell CURSORS on puzzle2d (`CROSS_MODE=cursors run-cross-shell.sh`) —
    today's `b3c1` never reached Live on either shell; expect Live + cursors both ways; (2) forced rebootstrap: two native shells on one
    block2d document, flood B's socket (ephemeral cursor stream at high rate) until the hub's fan-out lags → `RebootstrapRequired` + close
    1013; expect B back Live ≤ 2 s without backoff, its unacked edit kept, both ledgers converged (gate step 12's severable relay can host
    the flood); (3) GIS approval checkpoint (the other producer) via the semio MCP `inference_approve` on a gismap with a native viewer open.

