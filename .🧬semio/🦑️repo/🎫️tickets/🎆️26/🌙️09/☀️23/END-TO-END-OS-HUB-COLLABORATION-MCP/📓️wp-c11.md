# WP-C11 — Collaboration Between Users In The React `s` Shell

Session 13 slice C11 (successor of C10: handover [📓️wp-c10.md](📓️wp-c10.md), audit [📓️audit-s12-collaboration.md](📓️audit-s12-collaboration.md)).
Ports: hubs 8020–8029, serves 6520–6529. Scripts `wp-c11/`; expendable captures `wp-c11/generated/`; durable data and logs
`.🧬semio/🌐hub/s13-c11-*` (logs `s13-c11-logs/<tag>`). Builds/tests prefixed `nice -n 10`.

## Current-Tree Hub Recipe (C11, 19:4x — for any live-testing slice)

Hub 8021 = catalog B2 (9 packages, 16 creatable kinds) served by a CURRENT-TREE `os-hub` (19:07 build, includes H9 Qd:
directory list / event pages fast). Boot 11 min at load 80.

| what | value |
|---|---|
| binary | `.🧬semio/🌐hub/s13-c11-bin/os-hub-hub-8021` — signed copy of `.🧬semio/🦑️repo/⚡️cache/cargo/target/debug/os-hub` as of 19:07 (source sha256 `e2686559b4c7e2e74f141735a4fd4bb951b5289154217c69124683e7ed9b09d8`). **Copy this file; do not rebuild** — landing slices change hub-native codec crates, a later build may refuse B2 |
| rebuild (only if you must) | `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=<your target> nice -n 10 cargo build -p semio-hub --bin os-hub` |
| catalog | clone of `.🧬semio/🌐hub/w2-catalog-b2/trusted-catalog` (generation `f485bf7e…e2e1`): copy `generations/<gen>` (`cp -c -Rp`) + `current.json` into `<fresh data root>/trusted-catalog/` (dirs `0700`) |
| launcher | `zsh .tmp-ticket/wp-c11/c11-hub.sh <port> "/Users/ueli/Documents/semio/.🧬semio/🌐hub/w2-catalog-b2" <os-hub binary> <name>` → data `.🧬semio/🌐hub/s13-c11-<name>`, state `…-state` (`hold.txt`, `status.txt`, `ready.json`, `admin-capability.json`; `touch …-state/admin-request` → fresh admin capability in ≤ 5 s); hold = `c11-hub-hold.ts` detached (setsid). Copy the script and change `NAME`/paths for your slice prefix |
| users | provisioned before boot with `printf '%s' <password> \| OS_HUB_DATA=<root> <binary> credential set --email <e> --display-name <n>`: `user1@semio.dev` / `gm1-local-dev-pass-1`, `user2@semio.dev` / `gm1-local-dev-pass-2`, `user3@semio.dev` / `gm1-local-dev-pass-3` |
| env | none required (credential sign-in on; `OS_HUB_MERGE_POLICY=vigilant` optional) |
| serves | `zsh .tmp-ticket/wp-c11/serve.sh s <port> http://127.0.0.1:<hub port> dev` (join-only, `nice -n 10`) |

## Session 13

| # | Item | Status |
|---|---|---|
| 1 | collab-e2e 10/10 + STEP 14 (writer, draw, puzzle3d peer cursors + selections), presence symmetry, join/leave/expiry on 7800 | **7800/B3 run b3-4: 6/14** (1, 2, 3, 4, 5, 7 PASS; 9/10 SKIP: external hub). STEP 14 writer caret leg PASS (x 74→382); draw/puzzle3d legs failed on harness row-open after the saga (fixed); 6 = Check In `codec-refused` (H11); 8 = typing after the refused check-in never relayed (under investigation); 11/13 = concurrent writer typing is whole-text last-writer-wins (guest design prepared); 12 = harness read local echo too early (fixed). Root-fixed on the way: author's last keys lost (TextEditor dispatcher), STEP 2 (`access-changed`) |
| 2 | Opening flakes of session-12 two-browser runs (S12-3g, "document closed", "target changed", session never switches) | **not reproduced**: 8021 (10 opens) + 7800/B3 (all runs today: creation saga + Space-index row opens, 0 "document closed"/"target changed", `/spaces/<id>` misses 0). New: a dialog opened < ~8 s after sign-in is closed by Home's re-bootstrap → S16 |
| 3 | G-P1-4: B's inspector reflects A's remote edit ≤ ~2 s, live | pending |
| 4 | Durable collaborative undo/redo by two users + viewer read-only + removed member, live | pending |
| 5 | Two-browser matrix over every hub-creatable kind on 7800 (A creates, B opens via Space index, both edit, own undo, presence, reload converges), en + de | 7800/B3 **en running** (V1's permanent `two-human`, 19 kinds; with C11's per-human args + set-aware undo); 2d.block 9/9 checks PASS so far |
| 6 | Zero-touch clean-state `dev s` / ▶️start with a local hub, timed (after the all-package publish) | waits for W3's `--packages all` publish |
| 7 | Coordinator (audit s13 P1-4): an edit staged during a cut reaches the worker ~5 s after reconnect → verify LD's fix live, drop the outage probe's "stamp relative to the cut" workaround | waits for LD item 3 + restage |
| 8 | Coordinator (audit s13 P2-3): one human's undo would erase a DIFFERENT peer's still-pending edit — define + prove the correct outcome live (law exists since session 11) | pending |
| 9 | Coordinator (audit s13 P1-2): same-field conflict live, viewer role live, peer cursors live | cursors: writer caret live PASS on 7800 (collab b3-4); draw (Canvas2dHost presence, new) + puzzle3d pending re-run; vigilant hub 8022 READY for the conflict leg; viewer leg pending |

### Infra (pids I started)

| What | pid | Port | Notes |
|---|---|---|---|
| **09-27 (live wave)** serves `s` dev → 7800 | 8832 / 8838 (w2-detach) | 6523 / 6524 | logs `s13-c11-logs/serve-652{3,4}-b3.txt` |
| vigilant hub 8022 (B3 clone, 7800 binary, `OS_HUB_MERGE_POLICY=vigilant`) | hold 45592 / hub 45599 | 8022 | root `s13-c11-hub-8022-vigilant`, state `…-state` |
| serves `s` dev → 8022 | 61785 / 61791 (w2-detach) | 6525 / 6526 | logs `s13-c11-logs/serve-652{5,6}-8022.txt` |
| (09-26 entries below: all died in the overnight loss) | | | |
| serve `s` dev → 7800 | 15754 | 6523 | `wp-c11/serve.sh`, log `s13-c11-logs/serve-6523.txt` |
| serve `s` dev → 7800 | 15755 | 6524 | log `s13-c11-logs/serve-6524.txt` |
| hub 8021 (B2 catalog clone + 19:07 current-tree `os-hub`, fresh root, user1/2/3 = 7800 passwords + `gm1-local-dev-pass-3`) | hold 27066 / hub 27070 | 8021 | `wp-c11/c11-hub.sh`; data `s13-c11-hub-8021`, state `…-state` (admin capability, `admin-request`); binary `s13-c11-bin/os-hub-hub-8021` (source sha256 `e2686559…`); ready 11 min at load 80 |
| serve `s` dev → 8021 | 34902 | 6525 | log `s13-c11-logs/serve-6525.txt` |
| serve `s` dev → 8021 | 34909 | 6526 | log `s13-c11-logs/serve-6526.txt` |

### Log (session 13)

- 19:07 read AGENTS.md, preambles 13 + 12, `📓️wp-c10.md`, `📓️audit-s12-collaboration.md`, S15 S12-3g, W2 Hub Handoff. Hub 7800 READY
  on B2 (hold 28673 / hub 54029, runId `8d0ec7a5…`). Nothing of C10's is running; ports 8020–8029 / 6520–6529 free. Load 23, 1 rustc,
  111 GiB free.
- 19:09 serves 6523/6524 (dev lane) → 7800. The dev lane is stale against the tree (`[stale] space, stdio, trinity, wfc, writer …
  source-changed`) — expected while the landing runs; hub documents load their module from the hub catalog anyway.
- 19:13 matrix `c11mx1` (7800, note + writer, en): FAIL before any kind — user1's Home never listed the new space. Measured
  (`probe-c11-home.mjs`, `c11-hub-http.ts`): 7800's B2 binary answers user1 (member of ~53 spaces) `GET /directory/spaces` in
  **84 s** and `GET /directory/event-page/v1?after=0` in **17.7 s** (64 KiB, 121 events, `hasMore`); the hub process idles at
  0.3 % CPU. Home shows "Retrying directory update through sequence 0" and lists only the local Demo Studio; the link pill
  flips online ↔ reconnecting every ~10 s. user2 (6 spaces) gets its rows. Root = H9 Qd (whole-log fold per list, per-event
  membership reads, quadratic sealing) — fixed in the tree, not in 7800's 02:31 binary → **every 7800 run with user1 waits
  for W3's restart on a current-tree `os-hub`** (main told 19:4x). Harness: the matrix now finds a new space by NAME in a
  windowed table (`waitNamedRow`, pages every `table-window-scroll`).
- 19:21–19:33 own hub **8021** (see Infra): fresh root, B2 catalog clone, 19:07 current-tree `os-hub` (includes H9 Qd), users
  user1/user2/user3; ready in 11 min at load 80–90 (catalog guest-codec interpretation at ~13 % CPU). Serves 6525/6526 → 8021.
  Home on 8021: `event-page?after=0` answers at once, socket `since=3`, rows in < 2 s.
- 19:4x coordinator notes (audit s13): items 7–9 added to the table.
- 19:36–19:53 matrix `c11mx2` on 8021 (en, note + writer): note — create/open (saga) PASS, B opens via Space index PASS, presence
  2/2 PASS, every window takes focus/commands PASS (0 faults), A→B PASS (hub head 0→2), **B→A FAIL**: B's own view did not
  change (A saw it, head 3); B undo/redo + reload converge PASS. writer — both humans `setText` the SAME pinned text, so B's edit
  was invisible (harness defect); faults `commitCheckpoint refused … action-state-unconfirmed`, `undo refused … action-state-
  unconfirmed`, `redo refused … action-owner-mismatch`. Also measured: every hub note check-in is refused `codec-refused` after
  101–112 s (hub log `server.document.check-in`, both humans; the span drops the underlying `AuthorityError`) and the worker
  polls its status every 100 ms meanwhile (`DOCUMENT_CHECK_IN_POLL_MS`, 245 GETs in 55 s from one human) → routed.
- 20:0x **FOUND — an author sees its own hub-document edit only ~20 s late.** `probe-c11-selfpaint.mjs` (8021, note, ABBA): every
  edit reaches the PEER in 1.6–5.0 s but repaints the AUTHOR's own windows only after 21.5–24.8 s. `[DEBUG] c11` worker trace
  (`c11self4`): the addBlock command (`act seq=5`, 1 mutation, relayed at +0.1 s) is followed by patches for the artifact /
  inspection / history PANELS only — no window patch; the windows (`note-composite@2`, `note-navigator@2`) are patched only in the
  NEXT action's turn (`seq=6`, the shell's check-in `commitCheckpoint` ~20 s later). The guest's command turn does not re-render
  the author's window; a remote delivery does (why the peer is fast). Session 12 masked it: the author then ingested the hub's
  echo of its own batch (a "remote" delivery → window render); since the echo is dropped by identity (WG7) nothing repaints.
  **Root fix (worker, TS host):** `refreshDocumentSurfaces(child, assertCurrent, windows)` replaces `refreshPanelSurfaces` — after
  an app command or any mutating action the actor re-projects every verified WINDOW (own window context, `windowVisibleEvents`,
  shared with `renderSurface`) plus every panel; remote deliveries keep panels only (measured: they re-render windows).
  Unchanged surfaces answer no patch. Law: the actor cold-pair law (`browser document actor transfers one verified cold pair…`)
  now asserts one window re-projection after the mutating early command and one per later app command, none after the
  zero-mutation UI intent. **Run blocked**: the suite dies earlier on LD's in-flight envelope change (test literals lack
  `observed`/`target`, 20:28) → routed via main; live re-proof after the publish.
- 20:2x **FOUND — every hub-document edit now fails** (`c11self5`, writer `setText`): `act error … DocumentBackboneBatchError:
  document backbone batch: observed-flag` at `readDocumentBackboneEnvelopeBatchAtExact` → `action-state-unconfirmed` → actor
  closed → later actions `action-owner-mismatch`. Cause: LD's landing added `observed`/`target` to the envelope wire (host TS
  now), B2 guests emit the old wire. Coordinator: by design (no compat); live EDIT testing waits for W3's rebuild + publish +
  fresh hub roots. Continuing with non-edit legs + host fixes; edit battery scripted for right after the publish.
- 20:0x rule 22 (memory): stopped serves 6523/6524 (pids 15754/15755 + vites 15973/15988); hub 8021 + serves 6525/6526 kept (in use).
- 20:31 collab-e2e `c11collab-1` on 8021 (serves 6521/6522 by the harness; my 6525/6526 stopped for memory): STEP 1, 3, 5, 7 PASS;
  STEP 4/8/11/12/13 FAIL (every edit refused, envelope wire change); **STEP 2 FAIL**: user2's Home was not told live about the
  share. Root cause (`probe-c11-membership-page.ts` over HTTP): a human ADDED to a space created before their directory cursor
  receives only `member.upserted` (page after=47 → `[48 member.upserted]`; the full replay has `[44 space.created, 45, 48]`) and
  `authorizationGeneration` stays 1 → Home cannot build the row until a reload. Also: the Home guest refused an after=0 page under
  the same authority (`frontier-race`), so the worker's existing `rebootstrap-required → wake(true)` path could never succeed.
  (A space created after the reader's cursor was fine — why `c11mx2` passed.) Coordinator: my pushback on bumping
  `authorizationGeneration` accepted (it is the auth-session revocation generation; the worker retires the directory AND every
  open hub document on a change) → schema-first `access-changed`.
- 20:5x–21:2x **ROOT FIX — access-changed (schema-first, hub-emitted, event-derived):**
  - kernel schema `📇️directory/🧬️schema` (Rust `DirectoryStreamMessage::AccessChanged { space_id, change: DirectoryAccessChange }`,
    re-exported; TS twin; JSON schema `oneOf` row, `change ∈ {granted, revoked}`, `spaceId` non-empty).
  - hub `🌎️hub/🏗️bootstrap/🦀️.rs` (H11 informed, their 3 signatures untouched): the GLOBAL directory socket's replay and live loops
    follow every event with the frame `directory_access_change_for_reader` owes the socket's own reader — a grant/redemption naming
    it for a space whose `space.created` this socket never delivered (`created_on_socket`), every removal naming it (the removal
    event itself is skipped for the removed reader, so it was never told); the frame borrows no space authority (space-less,
    admitted like the reader's own events), never goes on the shared bus (`directory_message_visible` → false).
  - consumers: worker `directoryStreamWakeV1` (new, `📇️directory/🟦️.ts`) → `origin` for `access-changed`/`rebootstrap-required`;
    wgpu Shell → `rebootstrap = true`; MCP remote → revoke on its space's `revoked`, descriptor refresh on `granted`; kernel
    directory client `track` ignores it.
  - Home guest (`✏️s/🔌️plugins/🪐️space/…/🏠️home/…/🎚️config/🦀️.rs`, S16 agreed, one hunk): a page with `after_seq_exclusive == 0`
    rebuilds the projection under any authority; an incremental page must continue the held frontier (`frontier-race`).
  - Laws: shared fixture `🌎️hub/🧫️fixtures/🔑️directory-access-changed-v1` (9 cases, 5 wakes). TS `🔑️directory-access-changed`
    (Ajv admits every message/frame + 2 hostiles refused; independent reference of the reader rule; worker wake per row) **3/3 PASS**
    (`generated/law-access-changed-ts-1.txt`). Rust (written): hub bin-unit `a_membership_event_naming_the_reader_owes_it_exactly_
    the_fixture_access_change` (pure core over the fixture + wire shape), socket law `a_reader_added_to_an_older_space_is_told_its_
    access_changed_and_its_own_new_space_is_not` (sqlite; grant → frame after the event; own space → none; removal → revoked), and
    the existing global-revocation law now expects `access-changed {A, revoked}` before B's event. Home guest law (apply-directory-
    event-page unit): the old "same-authority after=0 is stale" assertion became after=3 → `frontier-race` + an origin-replay rebuild
    case. Checks: space-home native `--lib --tests` rc=0 (124 warnings) and wasm32-wasip2 `--lib` rc=0 (before the kernel variant);
    kernel + MCP + hub + wgpu `--lib --tests` on build-fleet-b running.
- 20:1x–20:3x (before the cut) **peer cursors, measured** (`probe-c11-cursor.mjs`, 8021, writer + draw): writer — B paints A's cursor
  but it never moves (`cursorX [5,5,5]`): `TextEditorHost` publishes `pointer = [caret, 0, 0]` on a fake canvas `{0,0,1}` / size
  `[1,1]`, so the "cursor" is the caret index mapped to a pixel, not the pointer; the harness' STEP 14 writer "PASS" in `c11collab-1`
  was a false pass (moved to `(0,0)` = the marker vanished) → harness fixed (a move only counts while the marker is painted).
  draw — B paints nothing: draw renders through `Canvas2dHost`, which published no presence view and mounted no overlay. Also:
  every peer cursor rides the 5 s interval heartbeat only (`subscribeLocalPresenceWindowViewsV1` had no subscriber).
- 21:3x–04:5x usage cut + overnight process loss (rule 28): hub 8021 hold/hub and serves 6525/6526 died; `c11-collab-1`'s serves
  6521/6522 had been stopped by the harness.
- 04:58 **resume + reconcile**: every in-flight edit present (worker, os test, Home guest, kernel schema Rust/TS/JSON, re-export,
  MCP remote, directory client, wgpu shell, hub bootstrap + bin-unit laws, fixture, TS law + registration, collab harness);
  the 8 temporary `[DEBUG] c11` worker lines REMOVED (0 left).
- 05:1x–06:0x **checks** (rules 26–29): os tsc 0 errors in my files (64 peer errors); TS laws `DirectoryAccessChanged` +
  `DirectoryEventPageBootstrapV1` + directory-page correlation **7/7**; worker suite **127/128** — the red one is LD's hand-written
  old-wire hex batch (`preserves a server Commands batch with a maximum-u64 HLC…`), told LD; my actor-window re-projection
  assertions pass. wasm32-wasip2 `cargo check -p semio-framework-os-kernel -p semio-s-artifact-space-home --features
  …/component-app-assembly --lib` **rc=0** (05:37, build-fleet-b, mutex; run 1 at 05:34 hit a peer's half-landed
  `AppFactory.codec` field in semio-framework-plugin, fixed by its owner 05:34:59). Native `--lib --bins --tests` of kernel,
  os-mcp, hub, renderer-wgpu: run 3 stopped after 15 min with no rustc child (convoy, rule 25); run 4 started 05:58 (load 117).
  W3 request `wp-w3/requests/c11.txt` (kernel, space-home).
- 05:4x–06:0x **live peer cursors (TS host only, no rebuild impact):**
  - `ShellHost` beats a document's presence as soon as a local window view changes (pointer, camera, caret), spaced by
    `PRESENCE_LIVE_VIEW_BEAT_INTERVAL_MS` = 100 (`presenceLiveViewBeatDelayMsV1`), instead of waiting for the 5 s beat. Pinned in the
    shared fixture `📡️replication/🧫️fixtures/💓️presence-liveness-v1` (`liveViewBeatIntervalMs` + law `a-moving-pointer-beats-live`,
    5 vectors); law `👥️scoped-presence` **10/10**.
  - `Canvas2dHost` (draw and every canvas-2d kind) publishes its camera + the pointer in WORLD coordinates (throttled 50 ms,
    cleared on leave/unmount) and mounts `CanvasPresenceOverlayV1` with its live camera + observed size. Law: fixture
    `📐️Canvas2dHost/🧫️fixtures/👕️peer-presence` (5 cameras incl. pan/zoom in/out, world = (client − size/2)/zoom + camera) →
    mounted suite `👕️peer-presence` **2/2** (published pointer = fixture world; a peer at that world point paints under the author's
    screen point for the same camera and at `canvasPointToScreen` for another camera). Registered in the react test config.
  - Noted for H11: every presence beat that changes a peer's bytes republishes the space's directory `presence` projection to
    every member socket, although that projection (actor, user, surface, color) is unchanged by a pointer move — with live beats
    that is up to 10 redundant directory frames/s per moving human (each costs two authority reads per member socket).
- 06:0x–06:3x (before the second cut): text peer carets — `TextPeerCaretsOverlayV1` (new, `👕️canvas-presence`): a text window's
  peers publish their caret in the editor's WORLD coordinates on `space: "text"` (`caretWorldJson` + camera + size, from
  `emitSelection` and after a wheel scroll); the viewer projects each through its own editor camera (`worldToScreenJson`) and
  paints one caret per other actor (label, colour, localized `aria-label` en/de); the old fake publish (`pointer = [caret,0,0]` on a
  `{0,0,1}`/`[1,1]` canvas) and its overlay are removed from `TextEditorHost`. Law: fixture `👕️canvas-presence/🧫️fixtures/✏️text-carets`
  → suite `✏️text-carets` **3/3** (en + de + no projection), with `🎭️react-overlay` + `👕️peer-presence` 7/7 total. os tsc: 0 errors in my
  files. H11 took the directory-presence republish note and landed "publish the directory projection only when it changes".
- 06:15 landing native check moved to build-landing (coordinator stopped my build-fleet-b run 37461 after 16 min idle); run 5 was
  still compiling when the fleet was cut ~06:35 → **no C11 landing row was written**. The set reached the tree anyway and W3's chain
  b3 compiled it into the restaged guests and the 7800 binary (coordinator 14:0x: "restaged `s` guests include your access-changed
  + Home after=0 fix"); the Rust laws I wrote (hub bin-unit ×2 + the revocation law change, Home guest unit law) have NOT been run.
- 13:58 **resume (rule 31): 7800 READY on catalog B3** (current tree, channel 18, new envelope wire, fresh root, runId `fd90596d…`).
  Live verification wave started: collab-e2e `c11collab-b3-1` (7800, harness serves 6521/6522, one headless Chromium).
- 13:59–14:04 **collab-e2e `c11collab-b3-1` on 7800/B3: 6/14 PASS, 2 SKIP (9/10: external hub), 6 FAIL.** PASS: 1, **2 (added member told
  live — the `access-changed` fix, red in `c11collab-1`)**, 3, **4 (typing reaches the peer; the fresh door artifact's first 2
  envelopes name the artifact id)**, 5, 7. FAIL: 6 (check-in history entry not shown in 15 s), 8 (a later keystroke never reached
  user2), 11 (the shared text lost characters of both markers), 12 (no local echo while `setOffline`), 13 (writers diverged; user2
  empty), 14 (writer surface selector: the textarea is now a hidden input sink).
- 14:0x–14:2x **FOUND + ROOT-FIXED — an author's last keystrokes vanished** (`probe-c11-typing.mjs`, two humans, one writer on 7800,
  rounds of typed markers, per-round wire capture (every Commands/Ack decoded) + temporary `[DEBUG] c11` echo/deliver/sendEdit logs):
  round 1 lost the author's last 1–2 chars in both views while every sent batch was Accepted. The trace shows the guest taking an
  OLDER text after a newer one (`deliver "a0x3290"` at +0 ms, then `deliver "a0x329"` at +183 ms; earlier `"a0"` after `"a0x1"`)
  — two coalescing dispatchers alive at once: `WasmEditorSurface`'s `deliver` was a `useMemo` over `[controllerId, explicitDraft,
  onAction, onDraftChange, surfaceId]`, so every new `onAction` identity built a NEW dispatcher while the old one still had a send
  in flight and a pending text; the new one sent the newest text at once, then the old one flushed its stale pending text last.
  Fix (TS host, `✏️TextEditor`): one dispatcher per surface (`[]`), reading its owner through `deliveryOwnerRef`. Law: delivery
  fixture `📮️delivery` + schema gain `typedAfterNewOwner` and the case `a-new-action-owner-mid-flight-keeps-one-delivery-order`
  (the mounted React law re-renders the host with a new `onAction` mid-flight) → **5/5**; mutant (deps `[onAction]`) → **red** on
  that case. Live after the fix (`c11typing4`, 5 alternating rounds A,B,A,B,A): **5/5 lossless, both views equal, settled 12–535 ms**.
- **Still RED (not host-fixable): two humans typing into the same writer at the same time lose each other's characters**
  (`c11typing2` round AB: A lost 3, B lost 7 chars; both views equal afterwards). The editor's `textEdit` carries the WHOLE text
  ("Replaces the entire text of the document", `TextWindowKit`/writer), and the guest diffs it against ITS current buffer: when the
  other human's operation was folded into the actor before this human's queued full text runs, the diff deletes the other human's
  insertion (trace: B's `…a1b1` is followed by A's full text `…a1x2`, which removes `b1`). A host-side rebase cannot close it (the
  stale full text is already queued in the actor). Needs a guest change → prepared as a design (below), guest edits frozen.
- Removed all temporary `[DEBUG] c11` lines (worker 8 earlier, TextEditor 4 now); 0 left in the tree.
- 14:56 collab-e2e `c11collab-b3-2` (7800, with the dispatcher fix) launched.
- 14:56–15:16 collab-e2e `b3-2` / `b3-3`: 0/14 — STEP 1's Create Space dialog opened and was closed again by the Home
  re-bootstrap that follows sign-in (user1 fetches `event-page/v1?after=0` a second time ~5–8 s after sign-in with a new socket
  grant); the harness acted 2 s after sign-in. Probe `probe-c11-createspace.mjs` (5 s after sign-in): dialog opens, fields
  `name/kind/visibility`. Routed the product side to S16; harness now waits for 8 s of directory quiet after sign-in
  (`collabSettleAfterLoad(…, "s-home-create-space")`) and re-presses the toolbar button when no dialog opened.
- 15:1x harness STEP 14 writer leg rewritten: a text peer's marker is its CARET (`collabAssertPeerCaretMoves`: the mover presses
  Home then End in its editor; the observer's `[data-peer-caret]` must move right), both editors re-mounted before the leg.
- 15:1x **routed to H11:** every writer Check In on 7800/B3 is refused `codec-refused` (4/4 today, 2.7–7.9 s; note on 8021
  yesterday); the span drops the underlying `AuthorityError` → blocks collab STEP 6.
- 15:1x V1 ported my matrix fixes (per-human args, set-aware undo) into the permanent `two-human` harness; tsc 0.
- 15:20 collab-e2e `b3-4` launched; vigilant hub **8022** booting for the same-field conflict leg (B3 catalog clone, the 7800
  binary `s13-w3-bin/s13-w3-hub-7800-b3/os-hub` sha256 `962ba372…`, `OS_HUB_MERGE_POLICY=vigilant`, fresh root
  `s13-c11-hub-8022-vigilant`, hold pid 45592, users 1–3).
- **Guest design (prepared, frozen until ALL PUBLISHED) — concurrent writer typing.** Writer's `textEdit` → `EditText { text }`
  is a whole-document SET (op payload = the full text, target field `text`), so two humans typing at once is last-writer-wins on
  the entire text (measured: `c11typing2` round AB lost 3 + 7 chars). Correct shape (event-sourced, no CRDT): the host already
  knows its base (the last text it sent or saw acknowledged); `textEdit` carries `{ text, base }` (or the host sends a splice
  `{ start, delete, insert, baseLength }`), the writer command computes the author's splice from `base → text`, rebases it through
  the guest's current buffer (`base → current`, prefix/suffix splice; overlapping regions: the later author's insert after the
  earlier one) and emits a `SpliceText { start, delete, insert }` mutation (inverse = the reverse splice), coalescing under the
  existing `writer-text-edit` key. Field-precise vigilant conflicts then apply only to overlapping splices. Owner: writer + the
  SDK text kit (T13/LB lineage) → coordinator.
