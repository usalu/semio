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
