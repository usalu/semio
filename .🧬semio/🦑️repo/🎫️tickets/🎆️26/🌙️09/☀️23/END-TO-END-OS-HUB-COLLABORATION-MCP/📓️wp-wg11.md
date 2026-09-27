# WP-WG11 — wgpu Shells (wasm32 Browser + Native winit) Collaborate Over the Hub, Session 14

Slice WG11 (session 14, 2026-09-27 18:2x), successor of WG9 ([`📓️wp-wg9.md`](📓️wp-wg9.md), wasm32 browser shell) and WG10
([`📓️wp-wg10.md`](📓️wp-wg10.md), native winit shell + cross-shell with React). Coordinator = main chat. Rules:
[`📓️session-14-preamble.md`](📓️session-14-preamble.md). Ports: hubs 8050–8059, serves 6550–6559; 7800 = W4 (read-only live use).
Scripts `wp-wg11/`, captures `wp-wg11/generated/` (expendable), durable logs `.🧬semio/🌐hub/s14-wg11-logs/`. Private cargo
target `.tmp-ticket/wp-wg11/target`. Landing rows: [`📓️landing.md`](📓️landing.md) `# Session 14`.

## Session 14

| # | Item | Status |
|---|------|--------|
| 1a | WG10 item 6 `wp-wg10/patch-rebootstrap-reseed.py` (native/wgpu shell re-seeds from `active-checkpoint/pair` on an actor-bound rebuild) — window 3 | **dry run clean 18:40** (11 files, every anchor once; `wp-wg11/generated/reseed-dryrun.txt`); compile proof in an overlay: pending |
| 1b | WG9 `wp-wg9/s13-board-presence-pointer-patch.py` (renderer notes `presence_pointer` while a board claims the move) — window 3 | **dry run clean 18:40** (2 files; `wp-wg11/generated/board-pointer-dryrun.diff`); its law is a source-order text law → re-deriving with a behavioural Shell law (routing moved into `ShellState`) |
| 1c | WG9 hub bin-unit echo-suppression pins (hub test file only, open during the freeze) | **applied 18:37** (`wp-wg11/wg11-echo-hub-law-pins.py`, dry run clean); native check + law queued (`s14-wg11-logs/native-1.txt`) |
| 2 | WG9 lost-Ack kernel fix (`settle_committed_envelopes`, `rollback_envelope → Option`, 3 laws, parity `lost-ack-committed-op-settles`) green on the current tree | queued (same native hold) |
| 3 | RUST_MIN_STACK: root-fix the 2 MiB test-thread overflow of the live gate (debug), no env var | pending |
| 4 | Live on 7800 (B3 now, ALL later): wasm32 s13b suite (23), native `hub-live-collaboration-check`, native ↔ React cross-shell (8), puzzle2d peer cursors, a11y of the wgpu roster + link states; re-run after window 3 | pending |
| 5 | Permanent wgpu collaboration acceptance (`📜️script.ts` verb + nx target + generated launch row; with R10) | pending |
| 6 | (coordinator 18:3x, audit-s14-state row 3.8) cross-shell-TYPE pairings wasm32 ↔ React and wasm32 ↔ native on one hub document (attach, presence both ways, edits both ways, late joiner), live + in the permanent harness | pending |
| 7 | (coordinator 18:3x, audit-s14-state row 4.10) wgpu agent-reply PIXEL assertion: a semio-MCP agent's edit is rendered correctly (readback/screenshot diff vs a reference render), in the harness | pending |

### Session 14 log

- 18:2x start. Read AGENTS.md, preambles 14/13/12, `📓️fleet-14-agents.md` (no "CHAIN LAUNCHED" yet; W4 unblocking the chain),
  `📓️wp-wg9.md`, `📓️wp-wg10.md`, `📓️fleet-13-agents.md` from 14:00. 0 wasm32 lanes held; native lane: lb2 holding, w4 queued.
- 18:3x reconcile of the predecessors' tree edits:
  - WG9 lost-Ack fix (applied 15:18): kernel laws **13/13 rc=0 at 16:21** + wasm32 browser/wasip2 rc=0 15:31/15:32
    (`.🧬semio/🌐hub/s13-wg9-logs/settlement-checks-1.txt`) — the landing row still said "checks pending".
  - WG9 sign-in patch (applied 14:56): renderer native `--lib --tests` check **rc=0 16:27**, renderer laws **23/23 rc=0 16:29**
    (`s13-wg9-logs/signin-native-2.txt`) — landing row said "native check pending".
  - WG9 hub bootstrap `HUB_CATCH_UP_ORIGIN` ×2 in the tree (bootstrap lines 5081/5194/5382); its bin-unit pins were reverted 06:32.
  - Unreported by the predecessors: WG9 run `s13e` (16:01–16:06, fresh note on serve 6554, A en / B de): 11/15 — attach ×2,
    presence, frames ×2 green; 5–8 red `no-edit-control` (probe could not find the Add Text control) —
    `s13-wg9-logs/collab-s13e.txt`. WG10 cross-shell de run `b3e2de` (15:38–16:21): native rc=101, React side timed out 180 s
    waiting for `s-space-create-artifact` (React never opened the doc) → steps 3–8 red — `s13-wg10-captures/cross-shell-b3e2de/`.
- 18:37 item 1c applied: `wp-wg11/wg11-echo-hub-law-pins.py` (imports WG9's `LAW_EDITS`; dry run clean `generated/echo-pins-dryrun.diff`):
  reconnect + joiner catch-up asserts pin `origin == HUB_CATCH_UP_ORIGIN` (≠ the joiner's actor), fixture `hubCatchUpOrigin`
  equality, both bootstrap call sites counted. Native hold 1 queued (`wp-wg11/wg11-native-1.sh`, pid 44569): hub check → hub law →
  kernel settlement laws.
- 18:4x native hold 1 (18:48–18:52, `s14-wg11-logs/native-1.txt`): **hub check + hub law + kernel laws all RED on peers' in-flight
  guest-linked edits, not on item 1c** — `semio-framework-3d` `🥽️mesh` E0282, `semio-framework-ui-contract` `🛡️limits`/`📐️layout`
  E0308/E0277 (`&bool`, `&&[str]` — value derive/codec change), `semio-framework-actor`, os-kernel 55 errors: the Codex peer's
  18:35–18:44 value-derive/codec set (coordinator 18:5x). Re-queue when the kernel compiles again.
- 18:4x item 1b re-derived: `wp-wg11/wg11-board-presence-pointer-patch.py` (dry run clean, `generated/board-pointer-v2-dryrun.diff`,
  3 files) = WG9's one renderer line + a BEHAVIOURAL law instead of WG9's source-order text law: every `👕️canvas-presence`
  `publish` case with a pointer is driven through `AppInteractionState::handle_pointer_move` (the path winit and the browser worker
  drive) over a painted board (`painted_board_with_input`, `shell_input_tests::pointer_interaction` → `pub(super)`); the heartbeat's
  view must equal the fixture's expected view and ≥ 1 case must be claimed by the board surface (the frozen path). Supersedes
  WG9's script for window 3.
- 18:5x inherited React serve 6590 (WG10's, pid 24033 + vite 24162, AV2's range now) stopped. React `s` serve **6555** started through
  S18's `ensureDevServe` (`wp-wg11/wg11-react-serve.ts`, joined to 7800, log `s14-wg11-logs/react-serve-6555.log`); boot > 240 s at load 111.
- 18:55–19:01 **wasm32 suite `s14a` (7800 B3, fresh note `artifact-077a7975756477104325d9df8793bea7` in WG9's space
  `01a0e2bd-…`, serve 6552 = WG9's note serve (renderer wasm-release 13:49), A en-US / B de-DE, outage on): 21/23**
  (`s14-wg11-logs/collab-s14a.txt`, captures `wp-wg11/generated/collab-s14a-*`; harness `wp-wg11/wg11-browser-collab.mjs` = WG9's).
  PASS: painted ×2, sign-in ×2 (B's chrome German), attach ×2 (`Persisted`/`Gespeichert`), roster `User One · User Two`, 4b late
  content ×2, A→B edit (+4.2 s), B→A edit (+14.3 s, within the probe's poll), frames name the document ×2, 15 s cut never freezes
  A (worker answers 5/11 ms), offline edit admitted `Pending (1)`, **12: after the 15 s cut A relinks in 3.0 s and B receives the
  offline edit** (red in s13b = the lost-Ack class), 13 B's long cut expires (87 s) in German, 13d German line.
  FAIL 12b: the 20 s cut was never spoken as `reconnecting` — Playwright's `setOffline` does not close an open WebSocket, so the link
  never dropped (sync stayed `Persisted`) → harness limitation, not product: the permanent harness cuts through a severable TCP relay.
  FAIL 14: card empty when read (`Remote: getrennt` = detached, never relinked — the product fact holds; the read raced the card
  toggle) → permanent harness re-opens and re-reads the card.
- 19:02–19:08 **native gate run `gate-b3-1` (7800 B3, WG10's durable 14:12 renderer test binary, DEFAULT test-thread stack):
  steps 1–7 PASS, then `has overflowed its stack` → SIGABRT (exit 134) in step 8 (A's first `addHandleKind`)** —
  `s14-wg11-logs/gate-b3-1.txt`. Item 3 reproduced on B3 (same step as WG10's 14:13 run 1).
- 19:10–19:20 **native gate `gate-b3-2` (same binary, `RUST_MIN_STACK=8388608` as a measurement condition only): 12/12 PASS,
  EXIT 0, 553 s** (`s14-wg11-logs/gate-b3-2.txt`): door `accepted → ready`, both guests mounted, both bound to the hub actor,
  socket Live 49 ms, presence both 145 ms, A authors 8.3 s + B ingests, B authors 14.2 s + A ingests, per-actor undo, connection
  loss (20 connections severed, offline edit, stale → ready, relive 26.7 s). Latencies are load-bound (load avg 80–111).
- 19:1x catalog-exact block module root `s14-wg11-b3-block` (served wasmSha256 `b7c334ee…` == catalog B3; shard worker fresh)
  (`wp-wg11/wg11-b3-block.sh`, `s14-wg11-logs/b3-block.txt`); wasm32 block2d serve **6553** (pid 40822, WG9's serve recipe with
  variant `block2d`). Seeded block2d `artifact-342b6cb6d06c5138ccaea5fb8b7dc9df` in WG9's space.
- 19:0x–19:2x **permanent harness written (staged in the ticket, lands in window 3)**:
  `wp-wg11/harness/🤝️hub-collaboration/🟦️.ts` (target `🎯️targets/🧊️wgpu/🧪️tests/🤝️hub-collaboration/🟦️.ts`, NEW dir → R10
  told via RELAY 19:0x): journeys `wasm32`, `wasm32-react`, `wasm32-native`, `native-react` (+ `--mode cursors`), `cursors`,
  `agent-pixels`; severable TCP relay per wasm32 human (real cuts: `setOffline` never closes an open WebSocket); native peer =
  the renderer's live laws (`--native-binary` or `cargo test`) following its file handshake with a wasm32 OR React follower;
  records via `withAcceptanceRecord` + `publishAcceptanceCheckResult` (`<check>-<locale>`), `blocked` for missing hub/serve/
  credentials; humans only from `SEMIO_TWO_HUMAN_USER{1,2}_{EMAIL,PASSWORD}`. Scratch tsc (`wp-wg11/tsc/tsconfig-wg11.json`,
  extends the os tsconfig): 1 error in my file (duplicate key) → fixed; re-run pending. Ticket entry `wp-wg11/harness/wg11-verb.ts`,
  runner `wp-wg11/wg11-harness-run.sh`.
- 19:23–19:35 harness run `wr-1` (`--journey wasm32-react`, block2d `artifact-342b…`, wasm32 6553 / React 6555): 8/13 — sign-ins,
  attach, React open (Live 11.0 s), presence both ways, React undo, late React joiner, late wasm32 attach PASS; wasm32 edit/count
  red: the wasm32 board's counts paragraph carries NO accessible name and the actions live in the window's folded Actions pane.
- 19:3x exploration (`wp-wg11/wg11-explore-wasm32.ts`, `generated/explore-block-*`): `dumpStructure` paints `6 Handle Kinds, 11
  Handles`; the projection announces `block2d-play-board.summary`/`.counts` as unnamed `paragraph`s (**a11y defect, item 4e**);
  unfolding `framework.window.block2dBoard.engagement.toggle` exposes 16 named action treeitems incl. `action.addHandleKind`; the
  History panel exposes `framework.history.undo.run`. Harness: document count = painted text (`dumpStructure`, what a sighted user
  reads) + a SEPARATE a11y step (counts paragraph named = painted text); edits via the Actions pane; undo via History.
- 19:3x **a11y root fix prepared** `wp-wg11/wg11-text-accessible-name-patch.py` (dry run clean, `generated/text-name-dryrun.diff`,
  3 files): the shared projection (`accessibility_projection_node` + TS twin `uiAccessibilityProjectionNodeV1`) names a `Text`
  node by its `value` unless an explicit label wins; shared fixture row `#caption` (Rust contract law, wgpu target projection law,
  TS runner all answer it). Guest-linked (ui-contract) → window 3.
- 19:42–19:47 **harness run `wr-2`: wasm32 ↔ React (block2d, 7800 B3, en): 13/14 PASS — first cross-shell-TYPE pairing ever run**
  (`s14-wg11-logs/harness-wr-2.txt`, `wp-wg11/generated/hub-collaboration/wr-2/report.json`): presence both ways (`User One · User
  Two` / `UO UT`), wasm32 edit → React 6→7 (seen 5 ms after the edit settled), React edit → wasm32 7→8 in 2.0 s, wasm32 undoes
  its own (8→7, React 7), React undoes its own (7→6, wasm32 6), late React joiner converges (6), late wasm32 joiner converges (6).
  Only red: the a11y step (counts paragraph unnamed) → the prepared patch. Acceptance record published en + de (FAIL on that step).
- 19:4x scratch overlay `.🧬semio/🌐hub/s14-wg11-overlay` (APFS clones of 77 832 tracked + untracked files, `wp-wg11/wg11-overlay-sync.py`)
  for compile/law proofs of the window-3 patches (overlay lane, private build-dir).
- 19:56–20:03 **harness run `w-1` (`--journey wasm32`, 7800 B3, fresh space + note created by the harness, both humans through
  their own SEVERABLE RELAY, en): 22/24** (`s14-wg11-logs/harness-w-1.txt`, `generated/hub-collaboration/w-1/report.json`):
  painted ×2, sign-in ×2 (through relays), attach ×2 (`Persisted`), roster `User Two · User One` both ways, committed content ×2
  (head 0), A→B 4.1 s, B→A 4.1 s, frames name the document ×2, **a REAL 15 s cut (6 connections severed) never freezes A (worker
  answers 16/9 ms)**, A relinks 3.1 s after heal and B receives the offline edit, **medium cut spoken `reconnecting` after 2.1 s**
  (`Connection lost. Your edits are kept…`), relink + line clears (13.3 s, backoff), **long cut expires B after 63.5 s** in English,
  late joiner shows 3 = hub head 3. Red (judgement, not product): the offline edit's pill read `Remote: backoff` (the kept-edits
  line is the card's `reconnecting` text) and after heal + 15 s the expired card shows `Remote: detached` with the line gone —
  judgements now accept the card's kept-edits line / a detached state (the product facts are recorded as detail).
- 20:0x item 3: macOS `lldb` and `sample` both need developer-tools authorization (both hung on attach; my pids killed at once) →
  static measurement instead. `wp-wg11/wg11-stack-frames.sh` (objdump prologues incl. Rust's inline stack-probe `sub x9, sp, #…`)
  over WG10's debug renderer test binary: **largest single frames 4.9 MB `RuntimeApply::start_frame_deferred`, 4.2 MB
  `RuntimeMailbox::restore_presenter_interaction_step`, 2.69 MB `ShellState::handle_shell_hit` poll, 1.58 MB
  `handle_pointer_button_for` poll, 1.42 MB `spawn_dispatch_reserved`, 1.18 MB `handle_keyboard_async`, 1.0 MB
  `handle_accessibility_event`, 938 KB `Ui::frame_step`, 840 KB `ShellState::dispatch_action` poll**
  (`generated/stack-frames-wg10bin-2.txt`). By-value type sizes read off `Option<T>::take` frames: `AppInteractionState` ≈ 68 KB,
  `FrameDeferredExecutionOwner` ≈ 82 KB, `RetainedPaintFrame` ≈ 77 KB, `UiWindow` ≈ 80 KB. Step 8 = `drive(dispatch_action(…))` on
  libtest's 2 MiB thread: the future itself is < 133 KB (its `drive` frame is not in the top 400) — the POLL frames overflow (debug
  gives every awaited sub-future temporary and every by-value move its own slot). Full table running (`stack-frames-wg10bin-all.txt`).
- 20:1x lanes: native hold 2 (hub + kernel laws) + hold 3 (current-tree renderer test binary) queued (`native-2-3.txt`, 9 ahead);
  overlay proof of the three window-3 patches queued (`overlay-proof-1.txt`, 12 ahead; `wg11-overlay-proof.sh`,
  `wg11-overlay-apply.py` stages each patch with its repo paths rewritten to the overlay — tested: all three dry-run + apply clean in
  the overlay, tree files verified untouched). Swap 32.8/33.8 GB, load ~90, 18 rustc.
