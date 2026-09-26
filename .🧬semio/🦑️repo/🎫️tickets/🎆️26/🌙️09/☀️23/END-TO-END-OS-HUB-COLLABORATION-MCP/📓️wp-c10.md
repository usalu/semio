# WP-C10 — Browser Collaboration Live On Hub 7800

Session 11 slice C10 (continues C7/C8). Ports: hubs 8020–8029, serves 6520–6529. Captures `wp-c10/generated/`.

## Status

| Item | Status |
|---|---|
| 1. collab-e2e 10/10 + STEP 14 (writer, draw, puzzle3d peer cursors) on hub 7800, two browsers | BLOCKED on catalog B by peer-owned state. `c10collab-b1` (12:0x, 0/13): user1 held 10 spaces and the `s` Home faults at ≥ 9 rows (`s-home-main nodes: 129, max_nodes: 128`, U5's landed fix not in the staged `space` guest). `c10collab-b2` (13:0x, restarted 7800, release lane, 0/13): user1 sees its new space, user2's Home never lists it, STEP 2/3 dialogs time out — the release-lane guests are a Sep 24 build that does not match catalog B. Needs W2's `s` restage from the catalog-B tree. Harness updated for today's DOM (rows by `data-ui-node-key`, row actions by `aria-label`, History tab by `data-slot="panel-tab-button"`, lane via `S_COLLAB_LANE`) |
| 2. Presence live: roster symmetry, cursors, selections, join/leave, lease expiry | roster symmetry, lease expiry (live socket keeps row), leave (507 ms), re-join replay: **PASS live on 7800**; in-canvas cursors/selections wait for W2's catalog |
| 3. Short connection shortage 5–20 s: no freeze, queue, reconcile, status en+de; long offline refused | FIXED + live-proven: no freeze, status en+de, offline edits apply + queue + resume (Welcome Tail, outbox flush), long offline → `link-expired`; end-to-end reconcile blocked by hub `DB I/O aggregate admission exhausted` (hub bug, routed) |
| 4. Zero-touch clean state, timed (launch rows `dev s` / ▶️start / hub) | Z1–Z4 FIXED (stale binary; one owner + session broker for every launch row; one dev catalog list; wait instead of give-up); broker live-proven (two users auto-signed-in). Timed clean-root catalog publish (phase A, the exact `ensureTrustedCatalog` command): run 2 (10:39, 884 s) died at `browser actor artifact: closed byte bound` (gis actor > 64 MiB — W2 root-fixed it 10:2x with deflate-raw cores); run 3 (10:4x, 984 s) passed that and the GIS cold-map laws, then died in the candidate proof on an EMPTY artifact-creation answer that the script parsed before checking its status (`JSON Parse error: Unexpected EOF`) → the probe now names `HTTP <status> <body>`; run 4 queued behind W2 |
| 5. Durable collaborative undo/redo + permissions (viewer read-only, removed member loses socket) live | removed member loses socket in 2.5 s PASS (+ `access-revoked` notice); undo/redo on a hub document: the S15 `action-state-unconfirmed` → `action-owner-mismatch` chain is F9 (fixed), and actor-lane `addFeature` → undo → redo ran with 0 faults on gismap (c10gis7 on 7800/catalog A; c10gp14e on 8024); the NOTE-on-hub proof and the viewer leg wait for a serve lane that matches the hub catalog (viewer targets come with W2's full publish) |
| 6. G-P2-3 same-field conflicting edit: fixture + live; loser sees a visible, localized outcome (never silent loss) | PARTIAL: (1) hub-rejected/transformed batches were SILENT rollbacks → localized notice (en/de); (2) NEW 10:4x: in the actor lane the hub's correction (rollback / transformed replacement) went to the Shell's local instance, so the AUTHOR's actor kept a refused edit on screen (phantom edit) → now delivered to the actor (`deliverAckCorrection`, test "returns a refused or transformed batch's correction to the browser actor"). Same-field live scenario waits for a writable 7800 |
| 7. G-P1-4 B's inspector panel reflects A's remote edit within ~2 s (asserted live) | IMPLEMENTED + unit-proven, live assertion still OPEN. Code: actor-rendered panels (per-surface offers/verdicts, verified panel surfaces, panel stores + intents), panel re-projection after every action/remote delivery (the guest re-renders only its window on a document change), F10 (WIT `list<u64>` crossing the child boundary). Live: panels mount with the actor and remote edits reach the peer (`c10gp14e` on hub 8024: steps 2/3/4 PASS, 0 faults), but every later attempt hit a peer-owned blocker — hub DB I/O exhaustion (8024, then 7800), the 12:2x disk cleanup, and now a serve lane that does not match catalog B (`The document target changed`, local `release` gis = Sep 24 build, `dev` gis core pruned). Needs: the `gis2d`/`s` lanes restaged from the catalog-B tree |

## Session 12

Started 22:50 (all processes died ~19:30; staged `s` guests = W2 restage4 18:56 = current tree; 7800 down until W2's catalog B2).

| # | Item | Status |
|---|---|---|
| 1 | React genesis-on-open for a fresh door artifact (first `Commands` carries `documentId = artifact-…`) | **hardened + live evidence**: the actor is seeded from the hub's genesis pair (`active-checkpoint/pair`); busy → retried under the lease, refused → integrity-failed + reopen (session law +2 rows); live: the hub ACCEPTED a fresh door note's commands on its own document socket (head 0→6, the hub refuses a foreign `documentId`); collab-e2e STEP 4 decodes user1's first outbound `Commands` (7800 run pending) |
| 2 | Second-window commands of an actor-bound document (`note-navigator` → `noteShellCommand` = `command owner mismatch`) | **ROOT-FIXED + laws + LIVE PASS** (8021 `c10mx10`/`c10mx11`: every window of both humans takes focus + commands, 0 faults) |
| 3 | collab-e2e 10/10 + STEP 14 (writer, draw, puzzle3d cursors + selections), presence symmetry, join/leave/expiry on 7800 | run 9 **4/14**: STEP 1, 2, 3, 5 PASS (presence symmetric); writer typing steps blocked by T12's guest re-announce defect after C10's scene-host root fix |
| 4 | G-P1-4 B's inspector reflects A's remote edit ≤ ~2 s, live | not reached this session (two-browser runs blocked by opening flakes + load) |
| 5 | G-P2-3 same-field conflict: loser sees localized outcome, author never keeps a refused edit | localized refusal notice (en/de) + law 2/2; **divergence ROOT-FIXED** (rebuild after remote-over-local fold, worker law); vigilant hub accepts the concurrent write → **H9**; live re-check blocked by opening flakes (S15/U5) |
| 6 | Short connection shortage 5–20 s end-to-end reconcile live (H9's DB I/O fix) | 8021 (de): no freeze, localized link state, hub holds both edits, both converge; **RED**: B's staged submit is held until the link returns (actor itself applies during the cut) — open |
| 7 | Durable collaborative undo/redo by two users + viewer read-only + removed member | not reached this session; writer path blocked by T12's re-announce defect |
| 8 | Zero-touch clean-state `dev s` / ▶️start with local hub, timed | waits for W2's `--packages all` |
| 9 | Coordinator add-on (audit s12 P2-3): two-browser collaboration matrix over EVERY hub-creatable kind on 7800 (B2, then `--packages all`): A creates, B opens via Space index, both edit, each undoes own edit, presence, reload converges; en + de; reds root-fixed or routed | not run on 7800 this session (blocked as item 4); note row 9/9 PASS on 8021 (01:5x) |
| — | Audit s12 P2-4 (`checkpoint-publications` callers) | CLOSED: `git grep` finds no caller in TS/JS/Rust/JSON; the only hit is the hub law that keeps the route deleted (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:15082`) |
| — | Found 04:5x: a reopened / late-joining hub document showed only its active checkpoint (tail `Commands` before `Session` went to the Shell's hidden local instance) | **ROOT-FIXED + law + LIVE** (worker 127/127; 8021 both humans `#105` after reload, was `#93`) |
| — | Found 05:3x: a `/spaces/<id>` hard load showed `actor-activation.revoked` / `no actor for instance 1` instead of the Space app (~half of full scenarios) — a retired Home instance's refresh failure raised a window fault | **ROOT-FIXED + law** (`🩺️window-fault` 18/18); live re-run in flight |
| — | Found 09:0x: a hub document opened from a space closed ~1.4 s after it mounted (the route effect re-applied the unchanged `/spaces/<id>`) | **ROOT-FIXED + law** (`🧭️route-ledger` 3/3, XState oracle); live re-run in flight |
| — | Found 09:1x: a `/spaces/<id>` hard load mounts the Space app twice (identity restore re-establishes) | **ROUTED to U5**; harnesses settle after a load |
| — | Found 09:3x: every scene-host action (typing, canvas, board, 3D) of a hub document was dropped by a no-op `onAction` | **ROOT-FIXED + law** (`🎭️actor-window-actions` 2/2); live first keystroke reaches the hub |
| — | Found 09:4x: the guest store re-announces an accepted operation on the next keystroke → hub replay conflict | **ROUTED to T12** (guest freeze); blocks collab STEP 4/8/11/12/13 |
| — | Found 10:1x: two humans diverged for good after a short link loss (remote folded over pending local) | **ROOT-FIXED + law** (worker 127/129) |
| — | Found 10:2x: freshly opened hub documents die in two-human runs (stale / document closed / never shown) | **ROUTED to S15/U5** |
| — | Found 00:3x (F11): share / rename / invite were dead in the React shell (pack-sorted directory commands refused as non-canonical) | **ROOT-FIXED + law + LIVE** |

### Log (session 12)

- 22:54 read preambles, audit, WG7/WG8 genesis notes, S15 §13 (b). Nothing of mine is running.
- 23:1x **Item 1 analysis (audit s12 P0-3).** The audit's "0 genesis hits under `🏪️store`" is a name search; React does
  not mint the genesis locally because it RECEIVES it: a door-created artifact's first active checkpoint IS the genesis pair
  the hub minted with the owning component's `initial_pair` (`🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs`
  `materialize_selected_genesis`: baseline frontier `{document_id, 0, "", 0, 0³²}`), and the worker fetches it through
  `GET /spaces/{s}/documents/{d}/active-checkpoint/pair` (`seedColdPairFromCanonicalCheckpoint`) and installs it into the
  actor child BEFORE `activate` — the same bytes WG8's native `codec.genesis` produces, verified against the lease checkpoint.
  The React gap that remains is the silent fallback: any non-OK or mistyped pair answer (`503` DB I/O, `504`, `404`) returned
  `false` and the actor activated WITHOUT a document, so every action was then refused `action-owner-mismatch`
  (`dispatchAction` requires `coldApplied`) — the React twin of WG8's identity-less guest. Fix + law below.
- 23:2x **Item 2 root fix (S15 finding b).** Root cause, measured in the code: an actor-bound document's actor rendered
  only the lease window (`fields.surface.windowKindId`, the hub's `app.window_kinds.first()`); every OTHER window of the
  same document (note opens `note-composite` + `note-navigator`) rendered from the Shell's LOCAL instance, which never sees
  the live document, while every action raised there (and every shell `noteShellCommand` while it is focused) went to the
  actor through `directBrowserActorForSession` and was refused by `browserActorAppCommandBytes`
  (`address.windowKindId !== lease window` → `command owner mismatch`). Fix (TS host only, no ABI change):
  - worker: `DocumentRenderSurfacesV1.windows` = every window kind of the VERIFIED app (strict: id + bodyKey, unique,
    panel keys never collide with a window, total ≤ one patch offer); `renderSurface` makes every window visible in its own
    `windowViewContext`; `browserActorAppCommandV1` admits a command in any verified window (base instance, view bound to
    that window; a command by id runs in the view's active window or the lease window) and returns the window whose painted
    revision the command is judged against; the mount identity names the lease window (`windowKindId` on
    `browser-actor-ui-mounted`, wire decode + test).
  - Shell: retained actor UI keeps `windows: Map` + `panels: Map`; `applyBrowserActorUiPatchesV1(patches, windowKeys,
    panelKeys, …)` (first offer must paint A window); every base window of an actor-bound session renders from its actor
    store and a not-yet-painted window shows the pending body (never the local one); intents go to the actor under the
    window's own surface; a command is stamped with the revision of the window it was issued in.
  - Laws: new neutral corpus `💻️os/🧫️fixtures/📇️directory/🪟️browser-actor-command-windows-v1.json` (11 rows: lease/second
    window, windowless view, by-id command, undeclared window, split instance, view bound elsewhere, foreign app) →
    `browser actor command windows` law; `🎭️browser-actor-panels` corpus now two windows + panels (9 steps, Ajv verdicts);
    the cold-pair actor law now declares a second window and asserts both windows become visible in their own context.
  - Checks: os `typecheck` 0 errors in C10 files (3 peer errors: `🔗️hub-projection`, `🧩️package-integration`,
    `🏘️SpaceBrowser` test); os `test long` **460/460** (`generated/os-test-s12-1.txt`); panels **2/2**.
- 23:3x **Item 1 hardening.** `seedColdPairFromCanonicalCheckpoint` no longer returns `false` for a pair the hub did not hand
  over: it throws `CanonicalCheckpointPairUnavailableV1(status)`. `activateDocumentBrowserActorAfterSession`: a transient
  refusal (408/425/429/5xx, `hub session unreachable`) activates the child and starts `retryCanonicalCheckpointPair`
  (jittered backoff 0.5–30 s, bounded by the lease's retirement and the socket; the pair then installs through the
  post-activation `installColdPair`, which now awaits an in-flight activation instead of dropping a pair that arrives during
  `openGuest`); any other refusal (404, wrong media type, 401/403) → `integrity-failed` naming the status + socket 1008 →
  the ordinary reopen. Laws: `🧵️browser-actor-session-v1.json` +2 rows — `canonical-pair-busy` (503: child activates, the
  pair is re-requested, lease live, no integrity failure) and `canonical-pair-refused` (404: no body/load/activation,
  `integrity-failed` "canonical checkpoint pair: unavailable (status 404)"); every other row now meets a busy hub instead of
  the old silent JSON answer. Worker suite **124/124** (`generated/worker-test-s12-4.txt`); os typecheck: only the 2 peer
  errors. Renderer long run hit its 900 s budget under load 81 (W2 release builds) — targeted engine suites re-run below.

- 23:4x **Item 1 live witness wired into collab-e2e** (`🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts`): `collabRecordSentCommands`
  decodes every `ClientFrame::Commands` user1 SENDS on its document sockets (`decodeClientFrame`, undecodable frames recorded,
  never skipped); STEP 4 now also asserts that the door-created writer's FIRST outbound envelope names `documentId = <artifact>`
  and every envelope matches its socket's document. os typecheck clean in C10 files.
- 23:4x harness: kinds are picked by the hub creation catalog's kind id (`[role=option][data-value=<kindId>]`: writer
  `text.document`, draw `2d.drawing`, puzzle3d `3d.puzzle`) — the hub labels every kind "Editor" (S15 finding c), so the old
  label regexes could never pick writer. New runner `wp-c10/run-collab-s12.sh` (durable logs `.🧬semio/🌐hub/s12-c10-logs/<tag>`).
- 23:4x **run `c10s12-own1` launched** (pid 16930, nohup): collab-e2e, own hub 8020 booted by the harness from a clone of
  catalog B (`e8167ce8…`) + H9's 23:36 current-tree `os-hub` (copy `s12-c10-bin/os-hub-c10s12-own1`), data root
  `s12-c10-hub-8020-c10s12-own1`, serves 6521/6522 on the dev lane (restage4). Pre-B2 dry run of the whole harness incl. the
  item-1 witness and STEP 9/10 (own hub restart).
- 23:5x **Item 5 (G-P2-3) — the loser's notice is localized.** Measured in the code: a hub refusal showed
  `Change refused by the hub: <hub reason>` — the hub's ENGLISH engine text (`command c-2 touches region(s) also touched by
  concurrent command c-1`) even in a German shell. Hub facts (read): the outcome step grades a region another human's
  concurrent command touched as `mutation.clamped` (Warning) and a violated constraint as `mutation.invariant` (Fatal)
  (`🛢️db/🗿️artifact` `grade_conflict_record`, frozen code set); `OS_HUB_MERGE_POLICY` decides: `normal` (default, 7800)
  ACCEPTS a same-field concurrent write (last writer wins, both edits in history), `vigilant` REFUSES it. Fix:
  `hubCommandRejectionReasonKeyV1` (ShellHelpers) reads the refusal's MutationMessage codes → `ui.conflict.hubConcurrentEdit`
  (en "Someone else changed the same part at the same time" / de "Jemand anderes hat gleichzeitig dieselbe Stelle geändert")
  or `ui.conflict.hubConcurrentInvariant` (en "Conflicts with a simultaneous change" / de "Widerspricht einer gleichzeitigen
  Änderung"), else the bare localized "Change refused by the hub"; the English reason goes to a named `console.warn` only.
  Schema `📚️I18n` + both bundles (normal + beginner). Law: neutral corpus `🛠️ShellHelpers/🧫️fixtures/⚔️hub-command-rejection`
  (5 rows × en/de exact notice text, 3 hostile payloads) → suite `⚔️hub-command-rejection` **2/2**; os typecheck clean in
  C10 files. Live plan: 7800 (normal) = convergence to the later writer, both rows in both ledgers; own hub with
  `OS_HUB_MERGE_POLICY=vigilant` = the second writer is refused, sees the localized notice, its actor rebootstraps to the
  winner's value (no phantom).
- 23:5x **run `c10s12-own2`** (own hub 8020 via the harness, catalog B clone + H9 23:36 binary, dev lane): hub ready after
  ~6 min of catalog loading; both humans signed in; **0/13** — STEP 1 user2's Home never lists user1's new PUBLIC space
  (user2 is not a member; the directory socket delivered frames), STEP 2 the row's `share` button click timed out (pointer
  click; screenshot shows the row with open/rename/share/delete/member icons), everything after cascades. Harness-side
  questions, not product faults yet → switched to a long-lived own hub to iterate step by step.
- 23:5x **own long-lived hub 8021** (`wp-c10/c10-hub.sh`: catalog B clone, H9 23:36 binary copy, users user1/user2 with
  7800's passwords, hold `c10-hub-hold.ts` detached pid **27395**, data `s12-c10-hub-8021`, state `s12-c10-hub-8021-state`)
  + serves `s` dev lane **6523** (pid 27450) / **6524** (pid 27451) → 8021 (logs `s12-c10-logs/serve-652{3,4}.txt`).
- 00:3x **F11 FIXED — sharing a space (and rename, share-link, revoke-invite) was dead in the React shell.** Measured on
  hub 8021 (`probe-s12-share2.mjs`, decoding every worker request the Shell posted): the Share dialog produced
  `{"email","kind","role","spaceId"}` — the worker wire is pack-encoded and pack maps are key-ASCENDING, so the command
  arrived in alphabetical order, `sealDirectoryCommandRequestV1` → `parseDirectoryCommandV1` refused it as non-canonical and
  the worker answered `directory-command-failed … invalid` (console `[os-shell] directory command failed <id> invalid`),
  no POST ever left. Only kinds whose declaration order happens to be ascending worked (`create-space`, `set-visibility`,
  `remove-member`, `delete-space`); `upsert-member`, `rename-space`, `create-invite`, `revoke-invite` never could.
  Root fix: `decodeBackboneWorkerRequest` decodes `directory-command` strictly (exact fields, request id) and restores the
  command's declaration order with `canonicalDirectoryCommandV1` — the same thing `directory-administration-submit` already
  did. Law: `carries every directory command across the worker wire back into its declaration order, so the worker can seal
  it` (os root suite) over the hub's own command corpus mirror `🧾️command-receipt-v1.json` (4 requests, 3 reordered kinds;
  each decoded command seals to the corpus' exact canonical request JSON; zero request id and an extra field refused) PASS.
  Live after the fix: the decoded command is `{kind,spaceId,email,role}`, no failure, directory bootstrap acks through seq 10.
- 00:3x Also measured (U5/shell, routed in the summary): Create Space pressed ~1 s after sign-in is refused
  `s.home.session-identity-required` (console only, no notice to the human); harnesses wait 5 s after sign-in.
  Harness fixes: STEP 1/2 now match the hub's membership listing (a human's Home lists spaces they are a MEMBER of —
  `directory_space_access_decision`; user2's row can only appear after the share), row actions are addressed by their
  named labels (`share: <space>`, `open: <artifact>`), a new space is found by its unique NAME (a fresh Home streams old
  rows in over several pages).
- 01:1x **U5 fixes re-measured on restarted serves 6523/6524 (pids 84343/84344, no_bg_nice)**: `probe-s12-reload.mjs` —
  Home after 3 reloads lists **9/9** of user1's hub spaces each time (was 1); `probe-s12-reopen.mjs` — `/spaces/<id>` hard
  load mounts the Space app **3/3** (4.7–9.5 s; was 2/3). One console error remains during a load (not user-visible):
  `[os-shell] tree window refresh failed … plugin-ui.intake-rejected:intake:actor-activation.revoked` right after
  `shell.surface-switch.sealed-instance dropped host effects for space#1` (`wp-c10/generated/probe-reopen-s12-2.txt`).
- 01:0x matrix `c10mx8`/`c10mx9` on 8021 (note): A creates + opens (windows `note-composite` + `note-navigator`), B opens via
  the Space index, **presence 2/2** PASS; the ledger is not an edit witness (it lists chrome/view verbs) → the matrix now
  witnesses the document TEXT of every window body plus the hub's own `headSeq` (admin API). Hub truth for mx8's note:
  `headSeq 6` after A's and B's edits + undos — the fresh door artifact's commands were ACCEPTED (the hub refuses
  `envelope document does not match this socket` otherwise): item 1 live evidence, React authors under `artifact-…`.
  mx9: B's open showed "The document target changed. Reopen the document." (flake, under investigation).
- 01:2x **WG8 relay — three React presence defects root-fixed (feed STEP 14 + WG8's cross-shell run).**
  1. *React's heartbeat never carried `interaction`/`presencePack`.* Local lane: `AppChannelClient` keeps the last
     `AppFrame::Ephemeral` of its instance from ANY outcome (`ephemeral()`), and `PluginRuntime.ephemeralSnapshot` serves it
     (was `undefined`) — the native twin is `ProgramBridge::observe_ephemeral`. Actor lane (every hub document): the browser
     actor's publication decoders now carry the Ephemeral contents (`BrowserActorEphemeralSnapshotV1`) instead of dropping
     them; the reservation keeps the last one (`ephemeralSnapshot`); `stampSession` — the one place a peer is stamped —
     publishes an actor-bound document's pack + interaction from its ACTOR (the Shell's local instance never sees the live
     document) and drops the Shell's local values.
  2. *Board2dHost painted no peer selection for block2d:* the overlay got the literal `domain="layer"`; it now gets the
     board scene's declared interaction domain (`scene.domainId`, as World3dHost already did).
  3. *`hubConnectionSummaryV1`'s `offline` was unreachable* (the three link states each return earlier): state removed from
     the type, the fold's dead tail, icon/tone/label maps — the indicator's states now equal the shared hub-projection
     schema's `summary.state` enum.
  Laws (all against WG8's shared fixtures): new `🧱️elements/👕️canvas-presence/🧪️tests/🎭️react-overlay` (registered) renders
  `CanvasPresenceOverlayV1` from `🧫️fixtures/👕️canvas-presence` — declared domain paints exactly the fixture's 3 marks + 1
  cursor, `layer`/none paints no mark — **2/2**; ShellSync `folds every session, link and document mix into exactly the
  shared hub projection's states` (reached set == schema enum) — suite **19/19**; os `stampSession publishes an actor-bound
  document's own presence pack and interaction…` (fixture peer-a's `handle` interaction round-trips; unpublished → none;
  local lane keeps the Shell's) + `keeps each instance's last published Ephemeral frame…` (AppChannelClient, other instance
  ignored) + publication decoders' Ephemeral expectation — os **466/466**; os typecheck clean in C10 files (1 new peer error
  `🔄️shell-utility-leaves`).
- 01:3x **Item 2 LIVE PASS (note on hub 8021, run `c10mx10`)**: A and B open a hub note (windows `note-composite` +
  `note-navigator`, both actor-rendered); clicking every window of both humans (window activation → the shell's
  `noteShellCommand`, S15's refused case) raises **0 faults** (no `command owner mismatch`); presence 2/2. Hub truth: A's
  addBlock → head 0→2, B's → 4, A's undo → 5, B's undo → 6 (edits AND undos reach the hub from both humans). The window-body
  TEXT witness could not see them (note draws blocks as SVG; its status line keeps saying "0 blocks") →
  `probe-s12-note-edit.mjs`: the author's own window re-renders (nodes 427→518, SVG 115→136, check-in "(1)", ledger
  "Add Block↶", head 6→7). Witness now = text + element/SVG/canvas counts per window body (`c10-journey.mjs`, helpers shared
  by matrix + probes). Hub note check-ins are refused `codec-refused` (hub log `server.document.check-in`, both humans) and
  one open got `503` on `execution-target/component` — hub-side, recorded, re-checked on 7800/B2.
- 01:5x **Matrix row `s.note.note` (en) 9/9 PASS, 0 faults** (`c10mx11`, hub 8021 = catalog B clone + H9 23:36 binary, serves
  6523/6524 dev lane): A creates from the Space app → opens; B opens from the Space index; presence 2/2; every window takes
  focus + commands; A's addBlock reaches B (#93→#97), B's reaches A (#101), A's undo removes only A's block (#97 both),
  B's undo returns both to the start (#93), both reload and converge; hub head 0→2→4→5→6. Full matrix over every creatable
  kind launched: run `c10mx-all-en1` (pid in the next entry's capture `s12-c10-logs/c10mx-all-en1-run.txt`).
- 03:4x resumed after the usage-limit cut (coordinator: B2 published 03:18, 7800 coming up). Overnight full-kind matrix
  `c10mx-all-en1` on 8021 (catalog B): 0/12 — the first kind (2d.block) opened but my mount detector (`[data-surface-id]`)
  never saw it, and every later kind failed with "the Space app never mounted" (A's shell stuck after the block2d board) —
  to be re-run on 7800/B2 with a board-aware detector. Outage probe `c10out-s12-2` (de, 15 s real cut): B's edit during the
  cut applied locally; reconnect after the cut (serve proxy answered 500 while 8021 was unreachable, then open-plan 200 →
  socket-grants 200); hub head 2→4 = only ONE of the two cut-time addBlocks (A's) reached the hub; B's queued edit did not
  → item 6 reconcile still RED; instrumenting the resume path (temporary `[DEBUG] c10 …` lines in the worker: suspend /
  resume / welcome / flush / ack — to be removed). Two instrumented runs failed earlier: A's creation-opened note answered
  `hub program unavailable … plugin-modules/…/semio_s_plugin_note_component.core.wasm answered HTTP 500` (a 14.9 MB module
  fetched through the serve proxy; the same URL answers 200 on retry) → the opening fails with no retry (routed: S15's
  module-store install lane should treat 5xx as transient).
- 03:5x **7800 ready on B2** (runId `6e9ecb80…`, fresh root; user1/user2 sign in, 0 spaces). **collab-e2e `c10s12-7800-1`
  launched** (external, dev lane serves 6521/6522 started by the harness; pid 39414; log `s12-c10-logs/c10s12-7800-1/run.txt`).
- 04:0x–04:18 **collab-e2e on 7800/B2**: run `c10s12-7800-1` 1/13 — STEP 2/3 the Space app window showed
  `plugin-ui.intake-rejected:intake:actor-activation.revoked` on a hard load (both humans); run `c10s12-7800-2` (harness now
  loads `/spaces/<id>` through `collabOpenSpace`, which records every load that does not mount the Space app — **0 misses**
  with U5's session-lane fix) **2/14**: STEP 1+2 PASS (create, share as author, user2's Home lists it, Space app mounts);
  STEP 3: the writer row replicates but the harness looked for the editor while the creation saga still showed "Loading
  plugin writer" (hub module install, ~29 MB) → harness now waits up to 240 s (`collabWaitForEditor`); STEP 4/8/12/13 row
  `open:` pointer clicks timed out (the ACTIONS column sits past the window edge) → keyboard activation
  (`collabRowAction`); STEP 7 401 (stale admin capability) → the runner touches `admin-request` first. Also noticed: the
  "Loading plugin writer · Cancel" band overlaps the navbar's app/role chips (cosmetic, U5). Run `c10s12-7800-3` launched.
- 04:2x–04:5x **collab-e2e runs 3–6 on 7800/B2** (`s12-c10-logs/c10s12-7800-{3..6}`): run 3 died in vite (`504 Outdated
  Optimize Dep`, dev-lane dep re-optimisation), run 4 **2/14** (STEP 1+2), run 5 **1/14** (STEP 2: user2's Home was never told
  about the shared space — the routed hub defect below), run 6 **3/14**: STEP 1–3 PASS (space created + shared, writer created
  by the door, row replicated to user2, user1's editor mounted after the hub module install); STEP 4+ failed on the harness
  locator `button[aria-label^="open:"]` — the row action is now labelled `Open: <artifact id>` (U5's labels are title-case;
  `probe-s12-artrow.mjs` dumped it) → `collabRowAction` matches case-insensitively (`[aria-label^="open:" i]`).
- 04:2x–04:5x **item 6 outage probe on 8021 (de)**, runs `c10out-s12-5..10`: 5 = "Das Dokumentziel wurde geändert" on B's
  open (the tail defect below), 6/8 = Home/Space-app route misses (U5/hub, not the cut), 7/9/10 = no freeze (worst frame
  9–77 ms over 39 samples), link state in German ("Remote: erneuter Versuch | verbinde erneut…" → "Gespeichert"), hub head
  2→6 holds BOTH humans' cut-time edits, and both humans end on the same document (`#105/38/0 ¦ #93/28/0`); the probe's
  verdicts were wrong, not the product: B's resume took ~19 s (15 s cut + backoff), so A's "during the cut" edit really ran
  after the link returned. The probe now stamps every edit relative to the cut and judges convergence against both
  humans' own post-edit texts.
- 04:5x **ROOT FIX — a reopened (or late-joining) hub document showed only its checkpoint.** Measured with
  `probe-s12-doctext.mjs` (8021): after both humans reload, each shows the active checkpoint's text (`#93`, 0 blocks)
  although the hub head holds 6 commands. Cause (worker): the hub's socket order is `Welcome` → tail `Commands` →
  `Session`; the reservation that routes backbone into the browser actor only exists after `Session`, so
  `handleHubFrame` emitted every tail `Commands` frame as `documentBackbone` to the Shell's LOCAL instance, which is never
  shown for an actor-bound document — the actor started from the checkpoint and never saw the tail. Fix: while a live
  lease names a `closed-browser-actor` target and no reservation exists yet, `Commands` frames are retained
  (`browserActorBackboneBeforeReservation`, bounded by `DOCUMENT_BACKBONE_RETENTION_LIMITS`, overflow is a named error,
  cleared with the lease) and handed to the actor in order inside `reserveDocumentBrowserActorChild` before `reserve()`;
  guest ingest receipts published during that drain are admitted (`backboneDraining`, the port is otherwise still not
  live → `actor-document-port.not-live`). Law: `browser actor catch-up tail` (os worker suite — tail frames before the
  Session reach the actor, none reach the Shell). Worker **127/127** (`generated/worker-test-s12-7.txt`); os typecheck: 0
  errors in C10 files (3 peer errors: renderer `🔬️artifact-creation-ready-opening`, `🗄️plugin-module-store` ×2). Live
  (8021, de): both humans reload → both show `#105/38/0/8786b5c7 ¦ #93/28/0/9da5354c`, equal, 3 blocks (was `#93`, 0).
- 05:2x removed the five temporary `[DEBUG] c10 …` worker lines. Launched collab-e2e `c10s12-7800-7` (pid 74148) and
  outage `c10out-s12-11` (pid 74770, 8021, de).
- 05:3x collab-e2e `c10s12-7800-7` **1/14** and outage `c10out-s12-11` both died on user2's `/spaces/<id>` hard load, 3 of 3
  loads each: the Space app's window showed `plugin-ui.intake-rejected:intake:actor-activation.revoked` (7800) or
  `[DEBUG] program space: no actor for instance 1 (createApp not called, or already destroyed)` (8021) instead of the
  index, while the program already ran as instance 2. Isolated loads never miss (`probe-s12-reopen.mjs` 6/6 on 6523/6525;
  new `probe-s12-spacerace.mjs`, both humans in one browser, 8/8 incl. 90 s idle), full scenarios miss ~half the time
  (collab runs 5 + 7, outage runs 6, 8, 11). Coordinator relay (H9) at the same time: 7800's intermittent directory fanout
  miss is the hub's exclusive authority-gate mutex, fixed in the tree, reaches 7800 with W2's `--packages all` restart
  (building a current-tree hub for 8021 would contend with W2's hub-mutex build; not done).
- 05:4x **ROOT FIX — a retired instance's failure painted over its successor's window.** Temporary `[DEBUG] c10
  window-fault` logs at every unguarded fault site, outage run `c10out-s12-12` caught it live (user1):
  `window-fault triple space 1 retired=true … no actor for instance 1` and `window-fault conflicts space 1 retired=true
  app-channel.disposed`. The `/spaces/<id>` load switches Home (instance 1) → Space index (instance 2); the session-triple
  refresh effect and the conflict read still addressed instance 1, failed with a RETIREMENT, and raised `SET_ERROR` /
  `SET_INSTANCE_FAULT` anyway — the fault then stayed on the Space app's screen. Only `reportRefreshFault` knew the
  retirement rule. Fix (TS host): `liveInstanceWindowFaultV1(error, supervisor, retired)` in `🏛️ShellHost/🩺️fault` — the one
  answer to "does this failure raise a window fault": `null` for a retired instance (`isPluginInstanceRetiredV1`), else the
  classified fault; `reportRefreshFault`, the session-triple refresh (now through `reportRefreshFault`), the spawned-window
  refresh and the conflict read all use it. Law: neutral corpus `🏛️ShellHost/🧫️fixtures/🪦️retired-instance-fault` (8 rows:
  marked gate refusal, unmarked one = still a fault, revoked activation, disposed/closed channel, hot-swapped handle, wire
  ABI mismatch, unpublished surfaces) + strict Ajv shape oracle + the shell's call-site contract → `🩺️window-fault`
  **18/18** (`generated/window-fault-s12-1.txt`); os typecheck: 0 errors in C10 files (1 peer error
  `⌨️text-input-oracle`). Debug lines removed. Live re-run: outage `c10out-s12-13`.
- 05:5x–08:40 paused (usage limit). 08:4x resumed (coordinator: 7800 still B2, W2 re-runs `--packages all`, rule 20 hard
  guest freeze — C10's work since is TS host + harness only). All C10 processes survived. `c10out-s12-13`: the Space app
  mounted cleanly (no painted fault any more), then A's create dialog did not open (see 09:1x).
- 08:5x collab-e2e `c10s12-7800-8` **0/14**, STEP 1: user1's new space never appeared in Home. The hub had it (user1 is a
  member of 52 spaces, `GET /directory/spaces`), but user1's Home table is now WINDOWED ("Rows 1–47 of 53",
  `probe-s12-hometable.mjs`; scrolling to the bottom shows "Rows 7–53 of 53"), so the new row was never in the DOM.
  Harness defect: `collabFindRow` pages every `[data-slot="table-window-scroll"]` top to bottom as a human scrolls and backs
  `collabWaitForNamedRow` / `collabWaitForRow`; the probes' `waitRow` (`c10-lib.mjs`) does the same.
- 09:0x **ROOT FIX — a hub document opened from a space closed ~1.4 s after it mounted.** Outage runs 12/14 and the new
  single-user `probe-s12-stay.mjs` (8021): A creates a note, it mounts, then every socket closes about 1.4 s after
  `execution-target/browser-actor` and A is back on the Space index (directory re-bootstrap `after=0`, a new space-index
  instance, no page load, no identity change: temporary `[DEBUG] c10` capability/identity/human-change logs stayed
  silent). Cause: routes carry no document (`/spaces/<id>` stays the URI while a hub document of that space is open), and
  the route effect re-applied the unchanged URI whenever `applyShellUri`'s identity or `loadedPlugins.length` changed (a hub
  program loading, a session change). Since U5's session lane (01:48) re-applications queue instead of being dropped, so
  each one switched the human back to the index. Fix (TS host): `🏛️ShellHost/🧭️route-ledger` (`createShellRouteLedgerV1`),
  the route in effect. The route effect applies only a URI that is not in effect; the lane records an application that
  took effect (`applyShellUri` now answers whether it did: `false` = not ready: no session, plugin or space app yet);
  `establishPrimarySession` (boot, worker loss, hot swap, changed human) and `navigateShellUri` (every guest/human
  navigation: guest `navigate` effect, instance route, not-found Home, breadcrumb back to the space, Hub overlay "open
  space"; closing the overlay stays a plain history step) invalidate it; a session replaced while a route was applying
  leaves that route unapplied (generation). Law: neutral corpus `🏛️ShellHost/🧫️fixtures/🧭️route-ledger` (6 scenarios incl.
  the measured one) replayed against the ledger AND an independent XState machine, strict Ajv shape oracle, shell
  call-site contract → `🧭️route-ledger` **3/3** (registered); `🔀️surface-switch` 3/3, `🩺️window-fault` 18/18; os typecheck:
  only the peer `⌨️text-input-oracle` error. Debug lines removed.
- 09:1x **Routed to U5 (SendMessage main):** a hard load of `/spaces/<id>` mounts the Space app, then the stored identity
  resolves ~5–7 s later, the human-change effect re-establishes Home and the route re-opens the space ("space index opening
  failed: document closed"). A dialog or document opened in between is lost; this was the create dialog without its kind
  picker in `c10out-s12-13` / `c10stay-2` (`probe-s12-createdialog.mjs`: the dialog itself is fine). Harness side: after a
  hard load both harnesses wait until the page has been quiet for 12 s (no re-opening, no directory re-bootstrap) and the
  Space app is mounted again (`settleAfterLoad`, `collabSettleAfterLoad`).
- 09:2x launched collab-e2e `c10s12-7800-9` and `c10stay-3` (8021).
- 09:2x **`c10stay-3` PASS** (route fix live): a hub note created from the Space index stays open for the whole 150 s watch,
  `note-composite` + `note-navigator` in every sample, its document socket never closes (was: closed ~1.4 s after mount).
- 09:2x **collab-e2e `c10s12-7800-9` 4/14** (7800/B2): **STEP 1, 2, 3, 5 PASS**: the space is created and found in the windowed
  Home, shared live, Space app mounted; the writer is door-created, its row replicates, the editor mounts; presence roster
  2/2 in distinct hub session colours on both shells. STEP 4/8/11/12/13 red: user1's typing never reached the hub (head 0).
- 09:3x **ROOT FIX — every scene-host action of a hub document was dropped.** `probe-s12-type.mjs` (8021): typing into the
  writer sends only heartbeats; the keys reach the editor's input sink, `sendEdit` runs, and no `textEdit` ever reaches the
  shell's input funnel. Cause: since 26/09/09 an actor-rendered window/panel handed its component scene hosts `onAction =
  refuseBrowserActorActionDescriptor`, originally a `[DEBUG]` "requires the complete UI intent" line, later reduced to an
  empty callback. Every action of the text editor, 2D/3D canvases, boards and node graphs of a hub document vanished
  (only action-pane verbs, which travel as UI intents, worked; that is what the matrix exercised). Since item 2 the actor
  admits app commands, so the fix hands actor windows and panels the shell's input funnel (`onActionStable`), whose actor
  branch (`directBrowserActorForSession` → `dispatchDirectBrowserActorCommand`) forwards them. The no-op is deleted. Law:
  `🎭️actor-window-actions` **2/2** (registered). Live: `textEdit` now routes ACTOR and the hub head moves 0→1.
- 09:4x **Routed to T12 (guest store, rule-20 frozen):** the second keystroke's guest backbone message re-carries the first,
  already Accepted operation byte-for-byte (`[edit-A#0, edit-B#0]`, diff/inverse digests equal, `generated/c10type-7/8/10`).
  The hub rejects the batch ("conflict: replayed operation 'edit-A#0'"), the document rebootstraps, later edits are
  refused. Measured: the worker drops the hub's echo of its own batch correctly (`origin == state.actor`) and the duplicate
  is in ONE guest message. Suspect: `🏪️store/🦀️.rs` `flush_apply_outbound` never drains `pending_report.outbound`. No host-side
  dedupe added (it would be a compatibility layer and breaks the per-message retention charge). Collab STEPs 4/8/11/12/13
  (writer typing) stay red until T12 lands it. Temporary `[DEBUG] c10` lines removed from worker, ShellHost and TextEditor.
- 09:5x harness: collab STEP 7 now matches each human's hub user id (the hub's connection report names users only by
  `authenticatedUserId`; ids read from each page's remembered capability, `collabHubUserId`). STEP 6 opens the writer
  document first and unfolds the History panel's closed "History" tree section before looking for `#s-checkin`
  (`collabOpenCheckin`). The probe witness `docText` no longer counts the canvas presence overlay (a peer's "User Two: edit"
  label made equal documents look different).
- 10:0x **item 6 (outage, 8021, de), runs `c10out-s12-15..21`:** no freeze (worst frame 47–80 ms over 38–39 samples), the link
  state is shown in German, the hub holds both humans' cut-time edits (head 2→6), and both end on the same document. Measured
  with temporary timing logs: during the cut the suspended actor APPLIES B's actions within 50–250 ms (`suspended=true`), but
  B's staged verb submit entered the worker only ~5 s after the reconnect, so B's edit showed at +23 s instead of during the
  cut. The Shell's patch verdicts, mailbox and `dispatchDirectBrowserActorCommand` carry no link gate. The hold sits between
  the staged form and the submit; not yet located. **Item 6 stays RED on "applied locally during the cut".**
- 10:1x **ROOT FIX — two humans diverged for good after a short link loss.** `probe-s12-conflict.mjs` gained
  `S_CONFLICT_CONTROL` (a link proxy in front of B, 8026 → vigilant hub 8025, control 8027, serve 6529 through it): B's link is
  cut while A and B set the same writer field. Run `c10conf-vig-2`: after the link returns, A shows B's text and B shows A's,
  for good. The hub ordered A's command first and B's after the reconnect; B had folded its own edit first and A's tail on
  top. Rule: another human's operations folded in while this shell's own operations are unacknowledged mean the local fold
  order differs from the hub's. The worker marks it (`remoteFoldedOverLocal`) when fresh remote Commands are delivered with
  `pendingMutations` non-empty, and once the last local operation is Accepted an actor-bound document is rebuilt from the
  hub's authoritative pair (`requireArtifactRebootstrap`, the path Transformed/Rejected already take). Any rebootstrap clears
  the mark. Law: worker "remote operations folded over pending local ones" (interleaved → one rebootstrap after the accepted
  outcome; not interleaved → none). Worker **127/129** (the 2 failures are a peer's Ajv `x-semio-note` keyword in space
  administration); os typecheck: only the peer `⌨️text-input-oracle` error. Live re-check `c10conf-vig-3/4` could not reach
  the edit step (see the opening flakes below).
- 10:1x **Routed to H9:** the vigilant hub (8025, `OS_HUB_MERGE_POLICY=vigilant` confirmed in its environment) ACCEPTS the true
  same-field concurrent write (head 0→4, no refusal). B's envelopes carry `dependencies: []`.
- 10:2x **Routed to S15/U5 — freshly opened hub documents die in two-human runs** (5 of the last 7 runs, hubs 8021 and 8025):
  (a) "created artifact remains ready after opening failed: document closed" 8–36 s after the open-plan, before any
  browser-actor reservation; (b) "The document target changed" (stale) right after mount; (c) the actor mounts and paints
  (patch verdicts acknowledged) but the session never switches to it. Twice it came 0.3–1.6 s after the new 60 s
  `trusted-catalog/plugin-modules` refresh (ShellHost `hubCatalog`): a hypothesis, not proven. Single-human runs never
  showed it (`c10stay-3/4/5` PASS in en and de). Load was 57–62 during these runs (W2 publish plus builds), so the
  2-browser runs were paused. All temporary `[DEBUG] c10` lines are removed.

**Evidence loss 12:2x:** `wp-c10/generated/` (captures, serve/hub logs, catalog-A seed, the pre-H9 hub binary, phase-A data) was
deleted by something outside this slice between 12:21 and 12:30, and at the same time every serve of mine (6520/6523/6524/6525) and the
link proxy died without an exit line. Only wp-c10 lost its folder; the one thing it held that others did not was a private
`CARGO_TARGET_DIR` (`generated/data/target`, phase A), so a stray-cargo-target sweep is the likely cause — phase A now uses the
shared target. Captures cited before 12:21 below are gone; the runs after it are re-captured.
Coordinator 12:4x: it was an external low-disk cleanup (disk at 96 %), which also wiped hub 7800's data root (W2 restarted it on
catalog B, fresh root, runId `345ceda4…`) and pruned 15 plugins' dev-lane `.core.wasm` (gis, puzzle, stdio, vcs, wfc, …): a `dev`
serve now boots `data-semio-os-error=gis2d`. The RELEASE lane is intact but OLD (gis core `43b911c1…`, staged Sep 24 06:03; catalog B's gis
is `8460f752…`), so a release serve boots and then every hub document refuses with `The document target changed. Reopen the document.`
(run `c10gp14n`, 13:0x). `serve.sh` and the collab harness (`S_COLLAB_LANE`) can select the lane; neither lane matches catalog B until W2
restages `s` / `gis2d` from the catalog-B tree. Durable C10 data now lives under
`.🧬semio/🌐hub/s11-c10-*` (logs: `s11-c10-logs/`); `generated/` is expendable.

**Blocker for every live write proof (routed, not C10-owned):** hub DB I/O credit exhaustion. Every `Commands` batch is
answered `Ack Rejected: "unavailable: DB I/O aggregate admission exhausted"` (`🛢️db/🗄️storage/🦀️.rs` `db_io_operation_add`)
for a document that has grown: 7800's gismap after ~5 h; my hub 8024's gismap after ~20 edits (run `c10gp14h`, 11:3x) — and a
RESTARTED 8024 refused the very first edit on that same document (`c10gp14i`), so it is not a slow process leak but a per-write
admission that the document's size outgrows (a write that must read/copy the whole grown pair exceeds the per-operation page/byte
credit). Every long-lived hub document becomes read-only; fresh documents work until they grow.

Audit ref: `📓️audit-s11-collaboration.md` §6 — this slice owns G-P0-2, G-P1-1, G-P1-2, G-P1-4, G-P2-3. Every pass claimed
here cites a run against hub 7800 / catalog A (or states the hub and catalog it ran on).

## Infra (pids I started)

**Session 12 — ALL STOPPED (10:3x):** hubs 8021 (hold 27395 / os-hub 27402) and 8025 vigilant (hold 75402 / os-hub 75404),
serves 6523/6525/6526/6527/6528/6529, link proxies 8022→8021 and 8026→8025. Durable data stays under `.🧬semio/🌐hub/s12-c10-*`
(hub roots `s12-c10-hub-8021`, `s12-c10-hub-8025-vigilant`, logs `s12-c10-logs/`). Restart: `OS_HUB_MERGE_POLICY=<normal|vigilant>
zsh wp-c10/c10-hub.sh <port> <catalog root> <os-hub binary> <name>` (fresh root) or the hold on an existing root; serves
`setopt no_bg_nice; nohup zsh wp-c10/serve.sh s <port> <hubUrl> dev & disown`; proxy `bun wp-c10/c10-link-proxy.ts <listen> <upstream> <control>`.

**Session 12 (history):** hub **8021** hold `c10-hub-hold.ts` pid 27395 → os-hub 27402 (nice 0; catalog B clone + H9 23:36
binary; data `.🧬semio/🌐hub/s12-c10-hub-8021`, state `…-state`, admin capability `…-state/admin-capability.json`, touch
`…-state/admin-request` to re-issue); serves `s` dev lane **6523** pid 64257 / **6524** pid 64258 → 8021 (restarted 00:4x with
`setopt no_bg_nice`, logs `s12-c10-logs/serve-652{3,4}.txt`; serves restarted 01:1x as pids **84343 / 84344**); link proxy
**8022 → 8021** (control 8023) pid **8322** + serve **6525** (user2's lane through the proxy). Current (05:2x): serve
**6523** pid 51374, **6525** (→ proxy 8022) pid 51375, serves **6526 / 6527** → 7800 pids 40753 / 40754 (matrix lanes),
proxy 8322, hub 8021 hold 27395 / os-hub 27402; 6524 stopped. collab-e2e starts and stops its own 6521/6522. Stop all when done.

**Session 11 (historical):**

All C10 serves, proxies and hubs are STOPPED (13:1x). Serve recipe: `zsh wp-c10/serve.sh <variant> <port> <hubUrl> [dev|release]`
(nohup + disown, log under `.🧬semio/🌐hub/s11-c10-logs/`). Queued: zero-touch phase A run 4 (wasm mutex ticket `…131150-52439-c10`,
data `.🧬semio/🌐hub/s11-c10-zt-hub-data`, log `.🧬semio/🌐hub/s11-c10-logs/zt-phase-a-run4.txt`).

| What (history) | pid | Port | Notes |
|---|---|---|---|
| serves `s` → 7800 | 18154 / 18156 | 6520 / 6523 | killed by the 12:2x cleanup |
| serve `gis2d` → 7800 | 31943→98660→29809 | 6524 | last one on the release lane; stopped 13:1x (lane ≠ catalog B) |
| link proxy 8021 → 7800 / serve gis2d → 8021 | 2396 / 2426 | 8021 / 6525 | killed by the 12:2x cleanup |
| hub 8024 (catalog A clone, pre-H9 binary) + serve gis2d → 8024 | 19654→55616 / 19709 | 8024 / 6526 | stopped 12:0x; data was under `generated/` (deleted) |

## Log


### Task 4 — zero-touch analysis (00:45–01:10, before hub 7800)

Launch chain read end to end (no run yet):
- `🛠️dev🪐️space⚛️react` = `bun nx run workspace:dev -- s` → root `DevScript` → `nx run @semio-tech/framework-os-dev:dev -- s`, which the
  root nx wrapper rewrites to `dev-s-react-dev` → `activate-s-react-dev` (Nx-cached closure) → `serve s react dev` →
  `ensureDevLocalHub` (8787, data `.🧬semio/🌐hub/hub-dev`) → `ensureTrustedCatalog` (`os-hub:trusted-catalog-bootstrap --packages
  stdio,gis,note,writer,draw,puzzle` when `current.json` is absent) → `hubDevBinaryPath` → `startLocalHub` → react-relay session →
  `/_semio/dev/local-session` → shell auto-signed-in.
- `▶️start` = `workspace:start` → spawns `os-hub:dev` detached with `stdio: ignore` on 8787/hub-dev, whose `DevScript` publishes
  only `stdio,gis` (`TRUSTED_BOOTSTRAP_LINKED_PACKAGES`) when the root has no catalog.

Manual steps / hazards found:
- **Z1 stale hub binary** (coordinator note): `hubDevBinaryPath` returned any existing `dist/build-dev/os-hub` without staging, so
  `dev s` could boot a binary from an older tree. FIXED (below).
- **Z2 two catalogs for one data root:** `▶️start`'s hub publishes `stdio,gis` into hub-dev, `dev s` publishes 6 packages; whoever runs
  first wins and both only test `current.json` existence → after `▶️start`, `dev s` reuses a hub with no writer/draw/puzzle/note.
- **Z3 no credential when `dev s` reuses a hub it does not own** (`▶️start` or `🛠️dev🗄️os-hub` first, or the compound rows): token ""
  → the shell shows sign-in, but a clean data root has no credential → manual `os-hub credential set`.
- **Z4 race:** `ensureDevLocalHub` gives up immediately ("continuing local-first") when 8787 is bound but not yet ready (the compound
  rows start both at once) → the shell runs without a hub until restarted.

| Fix | Files | Check | Result |
|---|---|---|---|
| Z1 `hubDevBinaryPath`/`hubDevPostgresBinaryPath` stage through `os-hub:build-dev(-postgres)` on every launch (Nx source hash decides freshness; failed staging with a present file throws) | `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts`, law in `🌎️hub/📦️packages/🦀️rust/📜️script.ts` `LocalBootstrapLaunchCheckScript` (5 cases) | `bun ./📜️script.ts local-bootstrap-launch-check` | PASS `staging-cases=5` (`generated/launch-check-1.txt`) |

### Live on hub 7800 (catalog A) — `s` shell entry flow, 01:00–02:20

Scenario scripts (ticket folder): `wp-c10/c10-lib.mjs` (two Playwright contexts, one per human), `c10-collab.mjs` (Home →
space → share → artifact → both open), `c10-seed.ts` (the product's own sealed directory commands + the hub artifact-creation
route), probes `probe-*.mjs`. Coordinator scope change (01:5x): catalog A opens only **gismap** and **note**; writer/draw/puzzle
are codec-only there, so STEP 14 on those kinds waits for W2's new catalog.

Measured findings, in the order a human hits them (all on serves 6520/6523 → hub 7800):

| # | Finding | Evidence | Status |
|---|---|---|---|
| F1 | Home's **Create Space** button is covered by the window's folded `Actions` chip: `elementFromPoint` at the button centre answers the chip (`SPAN inline-label "Actions"`), so a pointer cannot reach it; the button sits in the body's first line, which the dead-line scroll host leaves under the chrome when the content does not overflow | `generated/probe-hit` output (C10 log 01:20), screenshots `probe-boot-1.png`, `probe-dom-signed-in.png` | reported (chrome owner S15/U5); keyboard activation works, the harness uses it |
| F2 | Home's directory page never settled: every `applyDirectoryEventPage` completion threw `typed-operation returned more than one terminal output` → the owner stayed "Updating directory through sequence N" forever and Home listed no hub space | `generated/c10j-console.txt`; `[DEBUG]` capture showed the SAME receipt twice: `pending-output` parked by the admitting command turn + `terminal-output` on the completion | **FIXED** (below), measured: pages ack, directory socket opens, Home lists hub spaces (`probe-home.png`) |
| F3 | Live directory events are folded into Home with `foldDirectoryEvents`, which Home classifies `BatchOnlyPendingRewrite` → every live fold is refused (`dispatch-failed … interactive-job classification BatchOnlyPendingRewrite`), so a space created after sign-in only appears after the next page | `generated/c10e-console.txt` | open (Home guest classification; U5 6b) |
| F4 | The artifact-kind picker shows **"Editor"** for every kind: the hub's creation catalog labels a kind with its surface app's label (`🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` `artifact_creation_catalog` → `app.label`), not the kind's name | `probe-kinds.mjs` output: two options `Editor`, `data-value` kindIds `s.gis.gismap`, `s.note.note` | open (hub; needs a localized kind label source) |
| F5 | The Space app's table stays empty: its scoped directory stream asks `POST /directory/spaces/{space}/documents/index/socket-grants`, which the hub answers **404** (`issue_scoped_directory_socket_grant` requires an announced descriptor; the space index `index` is never announced) | `generated/c10h-console.txt` (16× 404), `c10k-error-user1.png` (headers, no rows, while the space holds 3 documents) | open |
| F6 | Artifact creation is slow: `accepted` for ~280 s before `preparing → ready` (one gismap), i.e. a human waits minutes after Create | `c10-seed.ts artifact` run 02:0x | open (hub creation service) |
| F7 | Harness drift: dialogs are `[role=dialog][data-slot=dialog-content]` (not `dialog-box`); app nodes are addressed by `data-ui-node-key` (DOM ids are window-scoped `window:<w>/<key>`); table rows are UI nodes keyed `space:<id>`/`artifact:<id>` (no `data-row-id`); the kind field is `kindChoice` (not `kindId`); an empty Home renders its empty state, never a table host | harness runs c10a–c10d | **FIXED** in `🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts` (typecheck 0 errors) |

Fix F2 — `🔌️PluginRuntime/🟦️.tsx` `typedOperationTerminalOutputV1`: one publication seen on both carriers is ONE output
(structural equality, `sameTerminalValueV1`); two different values still throw. The source-text law in the react target
script (`typedOperationTerminalOutputV1([...completionLeftover, ...carried])`) is unchanged.

### gismap on hub 7800 — C8 ten-step scenario (`wp-c10/c10-gis-scenario.mjs`, serve gis2d 6524 → 7800), 02:20–03:10

Space `01a0d5bc-cdb7-776b-83f0-b895b8fd0fd0`, gismap `artifact-e61a3b3ee0640b77bbd65637ab7d4fb3` (created through the hub's
artifact-creation route, `c10-seed.ts artifact`).

| Run | Result | Root cause found / fixed |
|---|---|---|
| c10gis1 | 1a–1e, 5 PASS (rosters symmetric, distinct hub colours); 2/3 FAIL; 10 faults: `commitCheckpoint refused: action-state-unconfirmed`, then every action `action-owner-mismatch` | the auto check-in closed the document child (below) |
| c10gis2/3 | `[DEBUG]` capture: `browser-actor-publication: unsupported host effect publish-event` | **F8 FIXED**: the actor lane's `routeTurnEffects` treated the guest's own pub/sub `publish-event` (a committed checkpoint publishes one) as a foreign host effect and closed the child; the local shard lane drops it too (`wireEffectToFriendly` has no case). Now skipped (`🏪️store/👷️worker/🟦️.ts`) |
| c10gis5/6 | `action-publication-unprojected` on the checkpoint turn; the command sent one `semio.history.transition` envelope while no invocation projects it | **F9 FIXED**: an app command's projection covers OPERATION envelopes only; history transitions (undo/redo/checkpoint) are framework records → filtered by the new TS twin `HISTORY_TRANSITION_DIFF_SCHEMA` (`📡️replication/🟦️.ts`, pinned against the neutral schema's `diffSchema` const in `🧪️history-transition` test) |
| c10gis7 | **0 faults**; A and B both author; B's ledger gains A's `create-position…` row and B's canvas changes (and vice versa); inspector witness still FAIL | inspector panel never changes in the hub lane, not even for the author's OWN edit (A: Positions 152 after its add) → G-P1-4 is a hub-lane panel gap, see next |
| c10gis8 (6,7,8) | 6 FAIL (inspector witness), 7 PASS (5+5 edits applied, inspectors equal — vacuous while panels are stale), 8 PASS (reload re-attach), 0 faults | — |

Checks after the outage (03:2x): os `tsc` — 0 errors in C10's files (2 errors in peers' files: `🔗️hub-projection` test arity,
`resolvemcpbinarypath` test type; not touched); `@semio-tech/framework-replication` tests 12/12 (`generated/replication-test-1.txt`).
Serves 6520/6523/6524 survived the outage.

### Short connection shortage (task 3) — gismap on hub 7800, 03:30–05:35

Playwright `setOffline` is NOT a link cut: runs c10out1–3 show 0 `ws-closed` events and the pill stays `Gespeichert`; bytes are
dropped silently (a black-holed socket). The proof therefore uses a real transport cut: `wp-c10/c10-link-proxy.ts` (TCP relay
8021 → 7800, control 8022: `mode=close` destroys every relayed connection and refuses new ones for N ms; `mode=stall` pauses
bytes). user2 runs gis2d serve 6525 with `S_HUB_URL=http://127.0.0.1:8021`, attaches `127.0.0.1:8021/<space>/<doc>`, in
**German** (context locale `de-DE`).

| Run | Check | Result | Evidence |
|---|---|---|---|
| c10out4 (close 15 s) | no UI freeze | **PASS**: 62 samples, worst rAF frame 0 ms, worst DOM read 15 ms | `generated/c10out4-collab-scenario.json` `outage.samples` |
| c10out4 | connection status shown (de) | **PASS**: pill `Remote: erneuter Versuch` during the cut, `Gespeichert` after restore (en twin `Remote: backoff`, c10gis runs) | same |
| c10out4 | local edit continues while cut | **FAIL**: `addFeature refused: owner-mismatch — action-owner-mismatch` | console `user2 … addFeature refused` |
| c10out4 | reconcile both ways after restore | **FAIL** (witness: History rows naming the peer's feature) | `outage.bSawA/aSawB` |

Root cause (read + measured): `connectHubOnce` `socket.onclose` calls `dropDocumentExecutionTargetLease` → the browser actor
child (the only thing that can apply a gismap edit in the hub lane) is closed with the socket, and every reconnect runs
open-plan → a NEW lease → a new child → a cold bootstrap. So the hub lane has no offline window at all: actions during the
cut are refused, and reconnection is a reopen, not a resume (the worker's outbox/resume-token path only serves the local
lane). The hub's session actor is stable across grants (`authenticate_document_socket_subject`: "a session's actor is bound
to the session … so per-actor undo spans reconnects"), so the fix is to keep the live child and its lease across a short
cut: mark the reservation link-down (actions admitted, backbone effects queued in `state.outbox`), and on reconnect reuse the
lease when the new plan names the same verified target (`sameLeaseFieldsV1`), re-admit the same actor's new grant, hello
with `resume_token`/frontier, ingest the missed tail, flush the outbox; refuse edits once the cut exceeds the short-shortage
bound (long offline refused, AGENTS.md). Status: designed, not yet implemented (see Status table).

### Permissions (task 5) — gismap on hub 7800

| Run | Check | Result | Evidence |
|---|---|---|---|
| c10perm1 (user2 seeded `spectator`) | viewer opens read-only | **BLOCKED on catalog**: the spectator's `open-plan` answers **503** (`ComponentUnavailable`: catalog A has editor open targets only), shell shows "The document target changed. Reopen the document."; the hub document stays unchanged (user1 sees no user2 row) | `generated/c10perm1-run.txt` |
| c10perm2 (user2 author, attached) | removed member loses the live socket | **PASS**: after user1's `remove-member` (HTTP 202), user2's document socket AND scoped directory socket closed **2.5 s** later, no reconnect, pill `Remote: detached`; later open-plan answers 401 | `generated/c10perm2-run.txt` |
| c10perm2 | the human is told why | gap: only `Remote: detached`, no localized "removed from this space" notice | same |

### Task 3 fix — a mounted hub document rides out a short link loss (05:40–06:20)

Implemented in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts` (TS host, no guest change):
- `socket.onclose` → `suspendDocumentBrowserActorLink`: a MOUNTED child (cold pair applied, surface painted, backbone bound)
  and its applied `VerifiedColdDocumentPair` are suspended instead of retired (socket identity released, lease kept); actions
  keep applying and their backbone effects queue in the outbox. A child still activating retires as before.
- Reconnect (`requestDocumentSocketAuthority`): the attempt is captured UNPUBLISHED beside a suspended child (a failed attempt
  never retires it); a plan naming the same verified target → `resumeDocumentSocketAuthority` exchanges it for a grant and
  `DocumentExecutionTargetLease.readmitBrowserActor` re-admits the SAME hub actor (session-bound, refused if the actor differs);
  a changed target → the ordinary reopen. `Session` → `resumeLink`, then the ordinary resume path (`Welcome` None/Tail, tail
  `Commands`, outbox flush).
- Bound: `DOCUMENT_LINK_SHORTAGE_BOUND_MS` = 2 × `HUB_RECONNECT_MAX_MS` (60 s); past it the child retires with the new
  localized status **`link-expired`** (en "The connection was lost for too long. Reconnect to keep editing this document." /
  de "Die Verbindung war zu lange unterbrochen. …") — long offline is refused. An open-plan answering 401/403/404/410 beside
  a suspended child retires it at once with the new status **`access-revoked`** (en "Your access to this document was removed."
  / de "Ihr Zugriff auf dieses Dokument wurde entfernt.") — closes the "removed member is not told why" gap.
- Status vocabulary schema-first: `DocumentExecutionTargetStatusCodeV1` + texts (`📇️directory/🧬️schema/🟦️.ts`), the neutral
  corpus `🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json` (status + roles), and the hub corpus
  oracle `proveExecutionTargetLeaseCorpus` inventory (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`).
- G-P2-3 (visible outcome, never silent loss): a hub `Ack` that REJECTS or TRANSFORMS a human's batch was a silent rollback
  (`commandOutcome` had no Shell consumer). The Shell now raises a localized notice: `ui.conflict.hubRejected` (en "Change
  refused by the hub" / de "Änderung vom Hub abgelehnt") with the hub's reason, `ui.conflict.hubTransformed` (en "Change
  adjusted" / de "Änderung angepasst") — schema `📚️I18n/🟦️.tsx`, bundles `🎯️targets/⚛️react/🟦️.tsx`, branch `🏛️ShellHost/🟦️.tsx`.

| Check | Result | Evidence |
|---|---|---|
| os `tsc` | 0 errors in C10 files (2 pre-existing peer errors, see above) | `generated/tsc-os-6.txt` |
| `@semio-tech/framework-os` vitest (all in-source, long) | **379/379** | `generated/os-test-full-1.txt` |
| hub `execution-target-lease-check` (corpus oracle + source laws, TS) | PASS `status=7` | `generated/lease-check-1.txt` |
| live c10out5 (hard cut 15 s, user2 de) | no freeze PASS; pill `Remote: erneuter Versuch` → `Remote: verbindet` → `Gespeichert`; **offline edit PASS** (applied locally during the cut, `editsAfter 1`) | `generated/c10out5-*` |
| live c10out6 (same, `[DEBUG]` capture) | resume path exercised: user2's reconnect `Welcome: Tail, suspended true, outbox 2` (offline edit + its checkpoint transition queued and flushed); hub `head_seq` 41 → 42 (user1's interim) → 45 after restore | `generated/c10out6-*` |
| live c10out6 reconcile | **FAIL — hub side**: every command batch in this window was answered `Ack Rejected: "unavailable: DB I/O aggregate admission exhausted"` (user1's AND user2's) — the hub's db I/O credit ledger (`🛢️db/🗄️storage/🦀️.rs` `db_io_operation_add`) refuses writes after ~5 h uptime; the same edits propagated at 02:50 (c10gis7). Routed to the coordinator (H9 / db owner). | console `[DEBUG] c10 ack … Rejected` |

`[DEBUG]` lines: worker `c10 welcome` / `c10 ack` REMOVED 10:4x (the `ack` one would have thrown on a `Transformed` Ack — `JSON.stringify` of its bigint timestamps; the new routing test caught it).
Note: `📓️` coordinator 05:58 — my serves did NOT materialize note: serve logs 6520/6523 only print `[stale] note …` (read-only
freshness report); 6524/6525 are gis2d.

### Presence lifecycle live (task 2) — gismap on hub 7800, run c10pres3 (06:3x)

| Check | Result |
|---|---|
| rosters symmetric, distinct hub colours, same colour per peer across rosters | PASS (every run since c10gis1; c10pres3 1d/5) |
| lease expiry with a live socket: user2's main thread blocked 22 s (no beats, socket open) | PASS: user1's roster stays 2 during and after (a live socket is always a row, PR1 contract) |
| leave: user2 reloads (document socket closes) | PASS: user1's roster → 1 after **507 ms** |
| re-join + join replay: user2 re-attaches | PASS: user1 sees 2 after 4 ms of the attach settling, user2 sees 2 (replayed roster) |

Evidence `generated/c10pres3-{run.txt,collab-scenario.json}`. In-canvas cursors/selections wait for W2's catalog (writer/draw/puzzle).

### Task 4 — zero-touch fixes Z2–Z4 (06:40–07:10)

Design (clean, one owner): **one detached local hub owner per data root** holds the hub's local-bootstrap pipe and runs a
**session broker**; every `s` serve — single-user rows AND both two-user rows — signs in through it. Which launch row reaches
a clean `hub-dev` root first no longer matters, stopping one UI never takes the hub from another, and the 15-minute local
session can be re-minted on demand.

| Change | Where |
|---|---|
| Session broker (`startLocalSessionBroker`, `requestLocalBrokerSession`, exact parsers for record/request/session): loopback-only, bearer secret in a `0600` record inside the `0700` data root, mints only the run's declared react-relay profiles through the authenticated pipe (serialized, contiguous sequences) | `🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts` |
| Schema-first: `LocalSessionBrokerRecordV1`, `LocalSessionRequestV1`, `LocalSessionV1` | `🌎️hub/🚀️local-bootstrap/🧬️schema/🔣️.json` |
| Language-agnostic cases (14, accepted + hostile) | `🌎️hub/🚀️local-bootstrap/🧫️fixtures/🎫️session-broker-v1/🔣️.json` |
| One development catalog list + profiles (`developer`, `user-1`, `user-2`) for every dev hub (Z2) | `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts` `LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES`, `LOCAL_HUB_DEVELOPMENT_PROFILES` |
| Owner `local-hub [hubUrl] [dataDir]` (catalog → staged binary → hub → broker → hold; a second owner for a live root exits) and `ensureDevLocalHub` as its consumer: spawns the owner detached when no hub, waits on owner-log/readiness progress (300 s stall bound, no wall budget) when the port is bound but not ready (Z4), then proves a broker session; a hub without a broker for the root (someone else's) is joined with manual sign-in | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts`, router `📦️packages/🟦️typescript/📜️script.ts`, Nx target `@semio-tech/framework-os-dev:local-hub` |
| `/_semio/dev/local-session` asks the broker per request (fresh session, per-serve profile) instead of serving a launch-time token | `🧑‍💻dev/🔌️vite-plugins/🟦️.ts`, `♻️activation/🌐️serve/🟦️.ts` |
| `▶️start` launches the owner (`framework-os-dev:local-hub`) instead of a bare `os-hub:dev` | root `📜️script.ts` `StartScript` |
| `os-hub:dev` publishes the same development catalog and runs the broker too (Z2/Z3) | `🌎️hub/📦️packages/🦀️rust/📜️script.ts` `DevScript` |
| Launch rows: new `🛠️dev🗄️os-hub🎫️local` (seed + launch.json, order 387.01); `👤️1`/`👤️2` react rows carry `SEMIO_DEV_LOCAL_HUB_PROFILE` `user-1`/`user-2` | `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json` |

| Check | Result | Evidence |
|---|---|---|
| hub `local-bootstrap-launch-check` (staging 5 cases + broker 14 cases, Ajv and TS parsers agree case by case) | PASS | `generated/launch-check-2.txt` |
| hub TS `tsc` / quick tests | 0 errors in C10 files / 19 passed, 2 skipped | `generated/tsc-hub-2.txt`, `hub-ts-test-1.txt` |
| os `tsc` | 0 errors in C10 files | `generated/tsc-os-7.txt` |
| live broker (hub 8023 on a catalog-A clone booted with the development profiles + `startLocalSessionBroker`; two serves 6526/6527 with profiles `user-1`/`user-2`, NO `S_LOCAL_ONLY`) | PASS: serves log `[dev-local-hub] ready … as 01a0d6ec-6dae… (user-1)` / `… 01a0d6ec-6f84… (user-2)` (two distinct hub users); `/_semio/dev/local-session` answers fresh sessions; fresh browser contexts on both serves end **signed in with no manual step** (8.2 s / 10.6 s to ready+signed-in); broker record removed on stop | `generated/broker-hold-8023.txt`, `serve-6526.txt`, `serve-6527.txt`, probe output in log |
| owner spawn path + timed clean-root run | queued: phase A (catalog publish into a clean root, the exact command `ensureTrustedCatalog` runs) waits on the wasm mutex behind W2 (ticket `…-c10` since 06:32); the owner path needs a hub binary from the current tree, which only pairs with the NEW catalog (H9 ABI) | `generated/zt-phase-a.txt` |

Open (Z5): after a lapsed local session the shell does not re-claim on its own (`hubSessionRefused` keeps the old capability;
the local-session effect runs only when the capability is `null`). The broker makes a re-claim possible; wiring the shell
to re-claim on refusal is the remaining step.

### Coordinator items 07:2x–07:4x

- **Data hygiene (rule 15):** stopped my phase-A tree (pids 35029/48556/…, it held the wasm mutex 07:01–07:31 and had finished
  `derive 8/8`), moved `wp-c10/catalog-a-seed` and `wp-c10/bin` to `wp-c10/generated/data/` (gitignored, verified with
  `git check-ignore`), deleted the recreatable `wp-c10/hub-8023`, `wp-c10/zt-hub-data`, `wp-c10/target`; scripts repointed
  (`run-collab.sh`, `zt-phase-a.sh` → `generated/data/…`). Phase A re-queued 07:31 (`generated/zt-phase-a.txt`).
- **S15 `hub5` (note undo/redo refused, then `action-owner-mismatch`):** S15's run was 02:32–02:36, before F8/F9 landed
  (~03:05). F9 is exactly this chain: undo/redo send a `semio.history.transition` envelope that no invocation projects →
  `action-publication-unprojected` → the child was closed → every later action `action-owner-mismatch`. After F8/F9 the
  gismap run c10gis7 on hub 7800 ran `addFeature` → undo → redo with **0 faults** (undo/redo rows applied). A note re-proof on
  hub 7800 is blocked by catalog skew, not by the action lane: the note serve built from the post-landing tree (W2's
  05:58 restage) opens the catalog-A note with a different pack schema hash → `documentOpenPlanAuthority` refuses →
  "The document target changed. Reopen the document." (c10note2, open-plan/manifest/component/descriptor all 200, no grant).
  Re-run on W2's catalog B.

### G-P1-4 — actor-rendered panels (08:00–08:30)

Root cause: in the hub (browser-actor) lane the verified child rendered ONLY the window (`renderSurface` made one
`surface-visible`, `captureBrowserActorUiPatchV1` kept only the window patch). Panels came from the Shell's local instance,
which never sees the live document, so the inspector showed the cold snapshot forever — even for the author's own edit.

Fix (schema-first, no compatibility path):
- `🩹️patch-handoff` (schema, fixture, TS, test): an offer carries `patches[]` (1–64, distinct surfaces) under ONE guest
  receipt; the result carries one `verdicts[]` entry per offered surface (`acknowledged` | `rejected` + reason), in offer
  order — exactly the guest's per-surface `patch-ack` / `patch-rejected` events. Owner match now also requires the verdict
  surfaces to equal the offered surfaces.
- worker: the panel surfaces come from the VERIFIED package descriptor (`verifiedPanelSurfacesV1`: every bodied panel-tab
  leaf, keyed by `panelTabKindId`, depth ≤ 8, unique, ≤ 63), never from the Shell's own manifest. `renderSurface` sends the
  window (window context) plus every panel (panel context) as `surface-visible`; per-surface acknowledged revisions; a UI
  intent is admitted against its own surface's painted revision (`browserActorUiIntentV1` names the surface).
- live bug found and fixed in the same change: one encoded panel view shared by several events tripped the child
  boundary's `value alias` refusal (`integrity-failed: browser actor child: value alias`, run `c10gp14b`) → encoded per event;
  the worker test now renders TWO panels through the real child client (which runs that admission).
- Shell: `applyBrowserActorUiPatchesV1` (ShellHelpers) applies per surface; a rejected surface is RESET to the empty
  document (`UiDocumentStore.reset`) — the guest's full resend after `patch-rejected` starts from a fresh reconciler at
  revision 0, so keeping the stale store (as before) could only loop to `patch feedback limit`. The first offer of an opening
  must paint the window. Panels of an actor-bound document render from the actor's stores (`BrowserActorPanelHostV1`, a tab
  without a store shows the pending body, never the local one) and their intents go to the actor under the panel surface.
- Tests: `🛠️ShellHelpers/🧪️tests/🎭️browser-actor-panels` (neutral corpus `🧫️fixtures/🎭️browser-actor-panels`; Ajv checks
  every verdict against the handoff schema), patch-handoff corpus (hostile offers/results), worker test with a two-panel
  descriptor (exact event lists), reservation test now mints its lease with the real descriptor.
- Live: `c10gp14c` (serve 6524 → 7800): both actors mount with panels, 0 faults; A's edit is answered `Rejected: unavailable:
  DB I/O aggregate admission exhausted` by the hub, so neither inspector can move — the assertion waits for 7800's restart.
- Own hub 8024 (catalog A clone, writable) run `c10gp14d`: B's actor died on A's first remote edit — `malformed hub frame … browser
  actor child: invocation rejected: invoke reactor/poll: … record required`, then B's socket closed. Root cause (**F10**): a panel
  update emits a `set-children` op whose `children: list<node-id>` (`list<u64>`) jco lifts as a **`BigUint64Array`**
  (`_liftFlatList … typedArray: BigUint64Array` in the closed actor), and the child boundary's `measureChildValue` admitted only
  `Uint8Array` among typed arrays. Windows never hit it (gismap's window re-renders by `upsert`), panels do. Fix: the boundary admits
  exactly the jco WIT-numeric-list lifts (`isWitNumericList`, one exclusive transferable buffer each; `DataView`/`Uint8ClampedArray`
  refused by name — corpus `valueAdmission` in the containment fixture + schema, test `🧪️browser-actor-child-admits-wit-numeric-lists`),
  and both `set-children` decoders read `list<node-id>` through one `nodeIdList` (patch-handoff; fixture `wireSetChildren`). The LOCAL
  lane's decoder had the same blind spot silently: `Array.isArray(children) ? … : []` turned a lifted `BigUint64Array` into NO
  children; it now uses `nodeIdList` and refuses any other shape.

## From WG8 (17:4x) — does React need genesis-on-open?

Native wgpu measured (WG8 gate runs 5–12): a fresh door-created artifact answers the document socket `Welcome { bootstrap: None }`,
so a guest that never loaded the document authors envelopes under its **app id** and every edit is refused as `document backbone
scope mismatch`. WG8 fixed the native shell by loading the owning component's `codec.genesis(document_id)` (the hub's own creation
baseline, `store_sync::os_store::component_document_genesis`) into the guest before its actor binds. **Please check React with a
fresh door artifact** (not a pre-edited one): the editor's first outbound `Commands` envelope must carry `documentId = artifact-…`.
If React's editor gets its identity from the open-plan / closed browser actor instead, nothing is needed; if not, the same genesis
load applies.
