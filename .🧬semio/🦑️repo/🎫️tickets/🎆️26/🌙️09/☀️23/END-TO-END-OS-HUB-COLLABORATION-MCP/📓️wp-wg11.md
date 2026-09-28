# WP-WG11 — wgpu Shells (wasm32 Browser + Native winit) Collaborate Over the Hub, Session 14

Slice WG11 (session 14, 2026-09-27 18:2x), successor of WG9 ([`📓️wp-wg9.md`](📓️wp-wg9.md), wasm32 browser shell) and WG10
([`📓️wp-wg10.md`](📓️wp-wg10.md), native winit shell + cross-shell with React). Coordinator = main chat. Rules:
[`📓️session-14-preamble.md`](📓️session-14-preamble.md). Ports: hubs 8050–8059, serves 6550–6559; 7800 = W4 (read-only live use).
Scripts `wp-wg11/`, captures `wp-wg11/generated/` (expendable), durable logs `.🧬semio/🌐hub/s14-wg11-logs/`. Private cargo
target `.tmp-ticket/wp-wg11/target`. Landing rows: [`📓️landing.md`](📓️landing.md) `# Session 14`.

## Session 14b

Successor agent (2026-09-28 12:0x, restart after the usage cut). Chain launched 12:02:46, GUEST FREEZE ON (window 3 closed).

| # | Item | Status |
|---|------|--------|
| 1 | Window-3 prepared patches dry-run on the live tree (overnight peer set ~1 870 files) | reseed **clean** (11 files); board-pointer **re-anchored** (renderer `handle_pointer_move` now takes `PointerInfo` → anchor `holder(pointer.id)`, law drives `shell_input_tests::mouse_pointer(1)`, `mouse_pointer` → `pub(super)`) then **clean** (3 files); a11y text-name **clean** (3 files); RUST_MIN_STACK root fix: in progress (static call-graph of the overflowing path) |
| 2 | WG9 lost-Ack kernel laws green on today's tree (native lane) | queued (hold 4) |
| 3 | Live on 7800 B3 | in progress |
| 4 | Row 3.8 cross-shell-TYPE pairings | wasm32 ↔ React 13/14 (s14 `wr-2`); wasm32 ↔ native running (`wn-1`) |
| 5 | Row 4.10 agent-reply pixel assertion | judgement fixed (see log); re-run pending |
| 6 | Permanent `hub-collaboration-acceptance` + RELAY spec to R10 | pending |

### RELAY spec (R10, window 3) — `hub-collaboration-acceptance`

- **Project** `@semio-tech/framework-renderer-wgpu` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json`).
- **Target** `hub-collaboration-acceptance`: `nx:run-commands`, `cache: false`, cwd = that package dir, command
  `bun ./📜️script.ts hub-collaboration-acceptance`; one nx **configuration per journey** appending `--journey <j>` (table). The
  gate appends `--hub <url> --locale en|de` and the serve flags. Verb + code land by WG11 in window 3: verb in the package's
  `📜️script.ts` (stages the block2d release native runtime exactly like `hub-live-collaboration-check` for native journeys), code
  in NEW dir `🎯️targets/🧊️wgpu/🧪️tests/🤝️hub-collaboration/🟦️.ts` (taxonomy registration = R10).
- **Args**: `--journey <j>` `--hub <url>` `[--serve <wgpu release serve url>]` `[--react-serve <React s dev serve url>]`
  `[--locale en|de]` `[--space <id> --document <id>]` `[--native-binary <renderer test binary>]` `[--native-modules <dir>]`
  `[--tag <t>]` `[--out <dir>]`. Exit ≠ 0 unless every step passes; missing hub/serve/credential/native runtime → `blocked` record.
- **Env** (never argv, never logged): `SEMIO_TWO_HUMAN_USER1_EMAIL`, `SEMIO_TWO_HUMAN_USER1_PASSWORD`, `SEMIO_TWO_HUMAN_USER2_EMAIL`,
  `SEMIO_TWO_HUMAN_USER2_PASSWORD` (the two-human matrix's variables). The agent journey mints its own delegation with human 1.
- **Hub precondition**: credential sign-in for both humans, catalog with `note`, `block`, `puzzle` (B3 has them), agent delegations
  (agent-pixels). No `hubAdmin`, no `backends`.

| Configuration (`--journey`) | Requires | Browsers | Check (`-<locale>`) | Criteria | en title | de title | Wall (load 50–90) |
|---|---|---|---|---|---|---|---|
| `wasm32` | hub, serve = wgpu release `note` | 1 chromium, 2 contexts (+2 in-process TCP relays) | `wgpu-collaboration-wasm32` | 3.2, 3.7, 3.9, 3.10 | wgpu wasm32 ↔ wasm32 collaboration over the hub | wgpu-wasm32 ↔ wasm32-Zusammenarbeit über den Hub | 8 min |
| `wasm32-react` | hub, serve = wgpu release `block2d`, localServe = React `s` dev | 1 / 2 | `wgpu-collaboration-wasm32-react` | 3.8 | wgpu wasm32 ↔ React collaboration | Zusammenarbeit wgpu-wasm32 ↔ React | 12 min |
| `wasm32-native` | hub, serve = wgpu release `block2d`, native block2d runtime + renderer test build | 1 / 1 + native | `wgpu-collaboration-wasm32-native` | 3.8 | wgpu wasm32 ↔ native wgpu collaboration | Zusammenarbeit wgpu-wasm32 ↔ natives wgpu | 13 min + build |
| `native-react` | hub, localServe = React `s` dev, native block2d runtime + renderer test build | 1 / 1 + native | `wgpu-collaboration-native-react` | 3.5 | native wgpu ↔ React collaboration | Zusammenarbeit natives wgpu ↔ React | 15 min + build |
| `native-react-cursors` | as `native-react` | 1 / 1 + native | `wgpu-peer-cursors-native-react` | 3.6 | native wgpu ↔ React peer cursors | Fremdcursor natives wgpu ↔ React | 15 min + build |
| `cursors` | hub, serve = wgpu release `puzzle2d` | 1 / 2 | `wgpu-peer-cursors` | 3.6 | wgpu wasm32 peer cursors on one board | wgpu-wasm32-Fremdcursor auf einem Brett | 8 min |
| `agent-pixels` | hub, serve = wgpu release `note`, os-mcp binary (`requireMcpBinary`) | 1 / 2 sequential | `wgpu-agent-reply-pixels` | 3.12, 4.10 | wgpu renders an AI agent's edit (pixel check) | wgpu zeichnet die Bearbeitung eines KI-Agenten (Pixelprüfung) | 35 min |

### Agreement WG11 ↔ LB2 (13:3x) — wgpu TableRow painter, window 3, both halves land together

Existing `TableRowProps` contract, no schema change. (1) Plain rows (`table_window_row`: Home, space, bcf, wav) have NO children after
LB2's pass — the SDK's `row-action-<i>` child buttons go; editable rows (`editable_table_window_row_at`: csv, tsv, xlsx, bcf, wav editors
+ xlsx viewer) keep exactly ONE child per materialised cell, positional to the column slice: key `cell-<logicalColumn>` (Input + Commit
binding, or Text) or a paged text-draft Surface `table-draft-<rowKey>-<col>` / read-only Surface `table-value-<rowKey>-<col>` → wgpu
paints cell i from child i when present, else `cells[i]` (as React). (2) `placement: Row` → trailing actions column headed
`Table.actions_label`; `Menu` → the row's context menu. (3) Row primary activation = the row record's `Trigger::Activate`. (4) The SDK's
`table_row_action` always sets `label` (LB2's law asserts it); wgpu uses it as accessible name + tooltip. (5) Row windowing =
`Table.window` (`TreeWindow`, `row_extent`) exactly like trees. `RowAction` stays the one representation (`row_actions`).

### Design — wgpu TableRow painter (window 3, with LB2)

Measured today in the tree (`🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs` ~1137–1170): `Table` → bare vertical Stack (no header, no
windowing); `TableRow` with children → bare horizontal Stack (cells = children; row actions only through the SDK's duplicate child
buttons); childless `TableRow` → ONE Button labelled `cells.join(" · ")` (no actions). The contract a11y projection names `Table` (grid)
and `TableRow` (row) but never projects `row_actions`, so the wgpu mirror cannot reach a row action by keyboard.
1. **Plain rows** (no children): `Table` → retained `UiNode::Tree` with ONE section (`window` = `Table.window`, header = the column labels
   + `actions_label`), each `TableRow` → a tree item with NEW `cells` (painted in the section's column grid, ellipsized; label = first
   cell for the row's name) + `actions` from `row_actions` (existing trailing-action paint + hit path; `Menu` placement → row menu) +
   `action` = the row's Activate; the `TableRow` records mount as the keyed identity rows the tree arm already uses.
2. **Editable rows** (one child per cell): the cell children stay real nodes; the row lays child i into column rect i (the tree row's
   single inline `control` generalises to per-cell controls) with the trailing actions reserved; Surface cells paint through the
   existing scene slot.
3. **A11y**: both contract projections (Rust + TS twin + shared fixture) project each `placement: Row` action as a `button` child of its
   row named by the action's label (keys `<rowKey>#row-action-<i>`; mirror activation dispatches the RowAction binding).
4. **Laws**: reconcile unit (plain + editable rows → cells/actions/window), paint pixel law (column text + trailing icons vs a reference
   draw list), hit law (icon → RowAction, row → Activate), projection fixture rows; live: Home in the wgpu shell shows cell text +
   actions (SH2's verify-home) after LB2's SDK change.

### Session 14b log

- 12:0x reconcile. Predecessor's only applied tree edit = item 1c echo pins (`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`, 27 18:37) — still in the
  tree. Unlogged predecessor work found: agent-pixels run `ap-1` (27 20:15–20:50, 9/11, `s14-wg11-logs/harness-ap-1.txt`), native
  hold 2 (27 21:23: hub check RED on peer `os-kernel-db` `PlannedEntries`/`apply_one`; **kernel lost-Ack laws 13/13 rc=0 21:24**),
  hold 3 (renderer test build RED on peer `semio-framework-os-infinite` missing `semio_framework_ui_viewport`), overlay proof never
  held. No `[DEBUG]` lines of WG11 in the tree; no half-applied hunk (the a11y patch script was untouched since 27 19:45).
- 12:09 **own slip, repaired at once**: a `--revert` flag on `wg11-echo-hub-law-pins.py` (meant as a no-op probe; the script's
  `--revert` WRITES) restored the pre-pin lines of `🔬️bin-unit/🦀️.rs`; re-applied with `--apply` within ~2 min; the dry run now
  refuses again (pins present) — net change zero (`generated/s14b/echo-pins-*.txt`).
- 12:10 dry runs on the live tree (`generated/s14b/*-dryrun*.txt`): WG10 reseed clean (11 files); a11y text-name clean (3 files);
  board-pointer RED (anchor `holder(pointer_id)` gone: the overnight set changed `handle_pointer_move` to take a `ui_render::
  PointerInfo`) → patch re-anchored on `holder(pointer.id)`, its law now drives `shell_input_tests::mouse_pointer(1)` (+ that helper
  → `pub(super)`) → **clean, 3 files** (`board-pointer-dryrun-2.txt`). WG9's original script stays red (superseded).
- 12:12 native hold 4 + 5 queued (`wg11-native-45.sh`, capture `s14-wg11-logs/native-4-5.txt`). Hold 4 (12:23–12:36): hub check
  **RED on peer** `semio-s-artifact-vcs-vcs` (`XlsxSnapshot.workbook` E0609/E0560, overnight set); kernel lib-test **does not compile**
  on the peer's overnight store set (missing fixture `🏪️store/🧬️retained-clone/🧪️tests/…/🧫️fixtures/📦️copy/🔣️.json`, E0618
  `RetainedCloneGrant` called as a fn) → item 2 BLOCKED on today's tree (last green: 27 21:24, 13/13). Kernel is frozen → no fix.
- 12:15–12:28 **row 3.8 wasm32 ↔ native — first run ever: 11/11 PASS, native law EXIT 0** (`harness-wn-1.txt`,
  `generated/hub-collaboration/wn-1/`; 7800 B3, block2d serve 6553 on the catalog-exact root `s14-wg11-b3-block`, WG10's 14:12 renderer
  test binary, `RUST_MIN_STACK=8388608` as a measurement condition): B signs in + attaches (`Persisted`), browser opens the native
  user's document live, presence both ways (`User Two · User One`, 6 ms), native edit → wasm32 6→7 (3 ms after the handshake),
  wasm32 edit → native ingests (7→8), each undoes only their own (8→7→6), wasm32 reload converges (live 30.7 s) and the next native
  edit arrives. Record `wgpu-collaboration-wasm32-native-en` PASS.
- 12:3x item 5 judgement fix (harness): ap-1's "changes the frame" was judged against 0.2 % of the body (2 970 px) while an empty
  text block paints ~1 200 px; its "equals reference" PASS was false — the reference session (same human, first session still open)
  never attached (`Remote: getrennt`) and so equalled the BEFORE frame. Now: bounds `agentEditMinPixels` 256 / `agentSameMaxPixels` 64
  (or 4× / 2× measured noise); the live session closes before the cold reference session boots; new step "a fresh session decodes the
  same committed state"; equality requires it.
- 12:30–12:40 **row 4.10 agent-pixels (en) 12/12 PASS** (`harness-ap-2.txt`, `generated/hub-collaboration/ap-2/`; note serve 6552 on
  `s13-wg9-b3-note`): agent = its own delegated principal, roster `wgpu acceptance agent ap-2 (AI agent) · User One`, MCP `addBlock`
  SUCCEEDED, block decoded 2.0 s later without reload, hub head 0→1, **frame changes 1 173 px, live frame == cold reference render
  0 px** (noise 0). Record `wgpu-agent-reply-pixels-en` PASS. Finding: the Artifact panel row of a text block paints its label
  `Text` and preview `text` overlapping (`generated/s14b/ap1-row-zoom.png`) — a wgpu outline-row layout defect (not fixed; frozen).
- 12:3x R10 relays: RELAY spec written above + sent; harness made self-provisioning (omitted `--serve`/`--react-serve` → own serve
  on a free port via `ensureDevServe`, stops only its own) + `native-react-cursors` split into its own journey/check; tsc 0 errors
  (`generated/s14b/tsc-harness-3.txt`). Live self-start of a wgpu serve: pending window 3 (no plugin `dist` in the tree now).
- 12:2x item 3 static call graph (`wg11-stack-paths.py`, graph of WG10's 14:12 binary): step 8's stack = test body 250 KB + `author_edit`
  56 KB + `drive` 56 KB + **`ShellState::dispatch_action` poll 841 KB** + `apply_mutations` 75 KB + `apply_ops_inner` 152 KB +
  `refresh_ui` subtree 359 KB ≈ 1.79 MB + libtest ≈ over 2 MiB. `dispatch_action`'s future is only ~55 KB (its `drive` frame): the
  841 KB are debug temporaries of the ~40 awaited child futures across its arms (≈25 `note_shell_setting_command` sites). Hold 5
  (`-Zprint-type-sizes`) measures the child future sizes for the fix.
- 12:42–12:50 **cursors (puzzle2d, de) 5/7** (`harness-cu-1.txt`; serve 6554 on `s13-wg9-b3-puzzle`): sign-in ×2, attach ×2
  (`Gespeichert`), presence both ways PASS; **peer cursors RED both ways (pixel deltas 0/0/0 over a control pass)** = the known
  frozen-presence-pointer defect the prepared board-pointer patch fixes (window 3) → re-run after it lands.
- 12:53–13:11 native ↔ React `nr-1`: **INVALID precondition** — the React serve (6555, current tree via `ensureDevServe`) speaks
  channel 19 since the 12:55 alignment; B3 is 18 → React `Remote: detached` (coordinator 12:55 broadcast); native law steps 1–5 PASS.
  Stopped by SIGTERM: the harness aborted, killed its native child and closed its browser (cancellation path proven). React serve stopped.
  Live React pairings wait for 7800 on ALL.
- 13:12–13:18 **wasm32 journey `w-2` (de, fresh space + note, 2 severable relays): 23/24** (`harness-w-2.txt`): painted, sign-in,
  attach, presence both ways, committed content, A→B 4.0 s, B→A 4.0 s, frames name the document, short cut never freezes A (10/6 ms),
  A relinks 3.0 s after heal + B receives the offline edit, medium cut spoken `reconnecting` after 5.1 s (German kept-edits line),
  relink + line clears, long cut expires B after 62.4 s in German, never relinks by itself, late joiner = hub head 3. Red: the kept-edits
  judgement read the card ONCE ~1 s after the offline edit (`Remote: erneuter Versuch`, no code yet) → judgement now polls ≤ 8 s
  (`keptLineMs`) for `pending`/`reconnecting` (the medium cut shows the line takes ~5 s); tsc 0.
- 13:2x coordinator: new window-3 item **wgpu TableRow painter** (with LB2). Contract proposal sent (`RELAY LB2`, existing
  `TableRowProps`, no schema change; SDK child-button duplication dropped by LB2).
- 13:3x item 3 **prepared** `wp-wg11/wg11-shell-turn-patch.py` (window 3; dry run clean, `generated/s14b/shell-turn-dryrun-2.txt`,
  2 files): 12 heavy shell turns (`dispatch_action`, `dispatch_command`, `apply_os_command`, `apply_mutations`, `apply_shell_uri`,
  `handle_hub_workspace_action`, `handle_sync_action`, `handle_checkin_action`, `execute_staged_action`, `handle_replay_shell_command`,
  `observe_invocation_history`, `set_extension_enabled`) return `ShellTurn<'a, R>` — their body boxed at the DEFINITION
  (`shell_turn(async move {…})`; `Send` on native because `spawn_dispatch_reserved` requires it, not on wasm32); the 9 call-site
  `Box::pin(self.<turn>(…))` E0733 wrappers and their two stale comments go. Child slot sizes measured from the disassembly of
  `dispatch_action`'s poll (`generated/s14b/dispatch-action-poll.s`, sret slot gaps): `apply_os_command` ~55 KB ×3, `dispatch_command`
  ~42 KB ×3, `apply_mutations` ~37 KB, `handle_hub_workspace_action` ~34 KB, `apply_shell_uri` ~33 KB, `handle_sync_action` ~30 KB,
  `execute_staged_action` ~59 KB, `handle_replay_shell_command` ~18 KB, `observe_invocation_history` ~16 KB (Σ slots ≈ 833 KB of the
  841 KB frame). Law `a_framework_setting_dispatch_completes_on_a_one_mebibyte_thread` (`⚙️settings-general-layout`): red before (two
  nested 841 KB frames), must pass after. Parse-checked (rustfmt on the patched copy: no parse error). Window-3 proof: native
  `--lib --tests` check + the law + frame table (`wg11-stack-frames.sh`) + live gate at the DEFAULT stack. Hold 5 (`-Zprint-type-sizes`)
  stays queued to confirm the set.
- 13:4x rule 23: no overlay build for WG11's sets (each < 20 files, renderer-only); `s14-wg11-overlay` has no build-dir (never held).
- 14:16–14:23 native hold 5: **renderer lib-test build of today's tree rc=0** (6 m 44 s, 450 warnings = type-checked;
  `generated/s14b/n5-build.txt`); durable binary `.🧬semio/🌐hub/s14-wg11-bin/renderer-tests-1423` (channel 19 — for live native runs on
  ALL). Today's `dispatch_action` poll frame = **844 KB** (prologue `0xce000 + 0x420`) — the overflow class is unchanged by the overnight
  set. The type-size capture came out empty (my awk kept the LAST `: n bytes` = the alignment) → filter fixed (`wg11-native-5.sh`, tested
  on a sample line); the sizes will be taken on the PATCHED build in window 3 (one hold: patch → `-Zprint-type-sizes` → frame table →
  law), no extra lane hold now.

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
