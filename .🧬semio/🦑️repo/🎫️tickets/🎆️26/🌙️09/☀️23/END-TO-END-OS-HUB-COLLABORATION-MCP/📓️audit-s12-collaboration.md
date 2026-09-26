# S12 Audit — Outcome 3: Collaboration Between Users Over The Hub (React + wgpu native + wgpu wasm32)

Read-only auditor, session 12 (Sonnet 5), 2026-09-25 ~23:0x. No process is running (session 12 preamble: everything
died at ~19:30; confirmed here — `lsof -iTCP:7800` answers nothing). This report is a state-of-the-tree audit: it
reconciles the session-11 auditor's findings (`📓️audit-s11-collaboration.md`) against the session-11 slice reports
that landed AFTER that audit was written (`📓️wp-c10.md`, `📓️wp-wg7.md`, `📓️wp-wg8.md`, `📓️wp-g10.md`, `📓️wp-h9.md`,
all continuing to 15:4x–17:4x on 09-25), and re-verifies a sample of load-bearing claims directly against the
current source tree (grep/read only, no build). Where a claim could not be independently re-verified (no cargo/nx/bun,
no servers per this session's hard rules), it is marked **UNVERIFIED** and cites the wp report it rests on.

**Source re-verification performed this session (all read-only):**
- `WasmActor::connect` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:3946`) is a real admission→dial→hello
  body (not the no-op stub the s11 audit quoted at line 3677-3680) — **confirms WG7's K1-K7 landing**. The s11 audit's
  "wasm32 browser transport is an intentional no-op today" finding is **STALE**, superseded by this session's landed code.
- `grep -rl "checkpoint-publications" --include='*.rs' .` (whole repo, excluding node_modules): **0 hits**. Confirms H9
  item 1 — the route is fully deleted, not just unwired. The s11 audit's "still present, unchanged" flag (§5.2) is **STALE**.
- `🌎️hub/🗄️stores/🦀️.rs:61` still reads `type Documents = NoDocumentAuthority;` with the "by design" doc-comment —
  confirms the s11 audit's "won't do, by design" read (§5.1) is current and correct; no change needed.
- `suspendDocumentBrowserActorLink` / `resumeDocumentSocketAuthority` / `DOCUMENT_LINK_SHORTAGE_BOUND_MS` present in
  `🏪️store/👷️worker/🟦️.ts` — confirms C10's short-connection-shortage fix is landed in source (React).
- `RunView`, `VcsStoreCell`, `artifact_state_retirement_maintenance_step` present in `🛢️db/⚙️engine/🦀️.rs` and
  `🛢️db/🗿️artifact/🦀️.rs` — confirms H9 Item g's three-bounds document-growth fix is landed in source.
- `hub_access_permits` (`🌎️hub/🔐️auth/🛡️access-policy/🦀️.rs:137`) and `DocumentCheckInJob`
  (`🌎️hub/🗿️artifact-authority/📌️check-in/🦀️.rs:158`) present — confirms H9 items 1 and 3 landed in source.
- `grep -rn "CRDT|crdt"` under `🌎️hub` and `🏪️store`: **0 hits** — no CRDT-shaped merge machinery in the collaboration path.
- No TS-side `component_document_genesis`/`ComponentDocumentCodec` call found under `🏪️store` — confirms the React
  "fresh door artifact genesis" gap WG8 flagged to C10 is still **open, unaddressed in source**, not merely unproven live.
- No process is listening on 7800 or any hub port; every wp report's "live" claims are frozen at their last-measured
  timestamp (09-25, mostly 11:xx–17:xx) and have not been re-run today.

---

## 1. User-journey matrix

Legend: **LIVE-PROVEN** = a named run against a real hub, cited; **LAW-ONLY** = fixture/unit/nextest law green, no live
two-party run; **BROKEN** = attempted live and failed, root cause named; **MISSING** = no code path exists; **UNVERIFIED**
= claimed but this audit could not confirm (source absent, or claim rests only on a deleted capture).

| Journey | React↔React | React↔wgpu-native | React↔wgpu-wasm32 | wgpu-native↔wgpu-native | human↔agent |
|---|---|---|---|---|---|
| Create/open shared doc | **LIVE-PROVEN** — gismap on hub 7800/catalog A, `c10gis1/7` (`wp-c10.md` gismap scenario); note also opened | **MISSING** — no cross-shell run attempted this fleet; each shell only proven against itself or agents | **BROKEN** — wgpu-wasm32 attach reaches lease, refused `document-execution-target.schema-mismatch`/`component-mismatch` every run (`wp-wg7.md` runs c3/r5/c3); root cause (empty `io.artifactSchema` on declaration-tree surfaces) fixed 15:2x but the fix has never reached a live re-run before session 11 ended | **LIVE-PROVEN** — WG8 gate step 6 "Live" 3.4 s, both native shells, `run-collab-live.sh` run 18 (`wp-wg8.md` §3) | **LIVE-PROVEN (agent side)** — G10 MCP agent commits to a hub doc, `head_seq` advances (`wp-g10.md` item 4b, durability §P0-2); **BROKEN (human side)** — the human's `s`/note lane never matched the catalog the agent used, "document target changed" (`wp-g10.md` 4b log 12:5x) |
| Concurrent edit (both author, additive) | **LIVE-PROVEN** — `c10gis7`: A and B both `addFeature`, 0 faults, each ledger gains the other's row (`wp-c10.md`) | **MISSING** (blocked upstream — no cross-shell doc even opens) | **MISSING** (blocked upstream — attach never reaches Live) | **LIVE-PROVEN** — gate steps 8-10: A authors 3.9 s, B ingests, B authors, A ingests (`wp-wg8.md` §3 run 18) | **LIVE-PROVEN** — durability gate: 2 agents + 1 human, distinct mutation ids, relay, `head_seq` advances 0→1→2→3 (`wp-g10.md` item 6, 21/21) |
| Same-field conflict, visible outcome | **PARTIAL, LIVE-PROVEN for the notice, not for a real field collision** — C10's G-P2-3 fix delivers a localized `ui.conflict.hubRejected`/`hubTransformed` notice and a refused-edit correction back to the actor (unit-proven: "returns a refused or transformed batch's correction to the browser actor"); no live run stages two users editing the *same field* — every live run so far is additive (`wp-c10.md` item 6) | MISSING | MISSING | UNVERIFIED — WG8's gate proves ordering/ingest, not a same-field collision specifically | MISSING — no run stages an agent and a human colliding on the same field |
| Undo/redo (own vs other's) | **LAW-ONLY** — `durable_collaborative_redo_fixture_survives_reload_and_hub_restart` fixture PASS (s11 audit §1 row5, carried forward, not re-run this fleet); C10's own-actor undo/redo (`addFeature→undo→redo`, 0 faults, `c10gis7`) is **LIVE-PROVEN** but only for the author's own edit, not cross-peer undo | MISSING (blocked upstream) | MISSING | **LIVE-PROVEN (own-edit)** — gate step 11 "per-actor undo propagates" (`apply` false on B) (`wp-wg8.md` §3 run 18); native journey laws separately prove undo/redo/select/copy/paste hub-less (§1.4, 4/4 green ×5 runs) | UNVERIFIED — no run drives undo of an agent's edit by a human or vice versa |
| Presence roster | **LIVE-PROVEN** — symmetry, distinct colours, lease expiry (live socket keeps row), leave (507 ms), re-join replay all PASS on hub 7800 (`wp-c10.md` item 2, `c10pres3`) | MISSING (blocked upstream) | MISSING | **LIVE-PROVEN** — gate step 7 "both rosters online" (`wp-wg8.md` §3) | **LIVE-PROVEN** — durability row 5p: both agents show in the human's roster as `:agent` principals, hub-stamped `principal_kind` (`wp-g10.md` item 9); React `PresenceBar` badge unit-proven, live browser proof still blocked by 4b's lane skew |
| Peer cursors/selections (in-canvas) | **LAW-ONLY** — unit/fixture green (writer/draw/puzzle3d `CanvasPresenceOverlayV1`, s11 audit §1 row4); **no STEP-14 live two-browser run was ever completed against a catalog that actually publishes writer/draw/puzzle** — catalog A only opens gismap+note; catalog B publish was still in flight when session 11 ended | MISSING | MISSING | MISSING — WG8's gate proves roster/edit convergence, not in-canvas cursor rendering specifically | MISSING |
| Permissions: viewer read-only | **BROKEN** — `c10perm1`: a seeded `spectator`'s open-plan answers 503 `ComponentUnavailable` because catalog A has editor-only open targets; the document is unchanged, no viewer UI is ever reached (`wp-c10.md` "Permissions" table) | UNVERIFIED | UNVERIFIED | UNVERIFIED — not part of WG8's gate | UNVERIFIED |
| Permissions: removal drops socket | **LIVE-PROVEN** — `c10perm2`: `remove-member` → both document AND scoped-directory sockets close in 2.5 s, later open-plan 401, and (after the task-3 fix) a localized `access-revoked` notice fires (`wp-c10.md`) | UNVERIFIED | UNVERIFIED | UNVERIFIED | **LIVE-PROVEN (protocol layer only)** — H9 item 3: declared-authorization law shows a removed member gets 403 on a live job and `authority-changed` on a running Check In (`wp-h9.md` item 3); no agent-specific removal run |
| Connection shortage 5-20s (no freeze, queue, reconcile) | **PARTIAL, LIVE-PROVEN for no-freeze + queue, BROKEN for reconcile** — `c10out5/6`: hard 15 s cut, no UI freeze, offline edit applies locally + queues in the outbox, resumes with `Welcome:Tail, suspended, outbox flushed`; the **reconcile half fails live** on a hub-side bug (`Ack Rejected: DB I/O aggregate admission exhausted`, `wp-c10.md` task 3) — but this exact hub bug is the one H9 Item g fixed in source (RunView/VcsStoreCell/retirement drain, confirmed landed this session) and was **never re-run against the fix** before session 11 ended | MISSING | MISSING | **LIVE-PROVEN** — gate step 12: TCP-level sever/heal of B's socket while A is untouched; local dispatch 12.9 µs, 3.005 s frame-pump window with no stall, `Stale→Ready` on heal (`wp-wg8.md` §3, and s11 audit §1 row8 native half) | UNVERIFIED |
| Long-offline → refused (not silent loss) | **LIVE-PROVEN** — C10's design landed as `link-expired` after `DOCUMENT_LINK_SHORTAGE_BOUND_MS` (2× `HUB_RECONNECT_MAX_MS` = 60 s), en+de localized (`wp-c10.md` "Task 3 fix"); this is a source-confirmed mechanism, cited live from the shorter 15 s cut runs but the >60 s refusal path itself is **LAW-ONLY** in this fleet's evidence (no run held a cut past the bound) | MISSING | MISSING | UNVERIFIED — not exercised by WG8's gate | MISSING |
| Reload/resume (late-join catch-up) | **LAW-ONLY, one live proof cited but not re-verified this session** — `wp-c1.md` PR1 join-replay TS runner done, live `HUB_E2E` run queued-not-confirmed per s11 audit §1 row10; `wp-c4c.md` "late joiner" cited PASS on the framework document WS (not re-checked here) | MISSING | MISSING | **LIVE-PROVEN** — gate steps 9-10 ingest-after-resume, step 12 relive after a sever (`wp-wg8.md` §3) | **LIVE-PROVEN** — durability gate: "a late joiner's catch-up carries all 3 [mutations]" after hub restart, `head_seq` kept 3→3 (`wp-g10.md` item 6) |
| Fresh door artifact genesis | **MISSING in source** — WG8 fixed native (`open_document` loads `codec.genesis` before the actor exists, `wp-wg8.md` §3 run 5-8 fix); WG8 explicitly asked C10/React to check the same gap and this audit confirms **no TS-side call to any genesis/ComponentDocumentCodec equivalent exists** (`grep` under `🏪️store`, 0 hits) — open, unowned as of session 11's end | MISSING (native fixed, React unfixed — a cross-shell run would still fail on React's side) | UNVERIFIED whether wasm32's `open_document` (target-neutral per WG7) already benefits — WG8 says "it now seeds there too once a component codec is registered", which is a conditional, not a proof | **LIVE-PROVEN (native only)** — gate runs 5-8 fixed this exact defect (`wp-wg8.md` §3) | UNVERIFIED |

---

## 2. Architecture conformance (AGENTS.md: event-sourced CQRS, no CRUD, no CRDT, short shortages don't freeze)

No violation found in the areas this audit could reach; two things worth flagging as design decisions rather than bugs:

1. **No CRDT-shaped merge code.** `grep -rn "CRDT|crdt"` under `🌎️hub` and `🏪️store` returned 0 hits. Conflict handling is
   server-linearized: `MergePolicy` + `Ack`/`ApplyOutcome` on the hub, `commandOutcome` routed to a localized UI notice on
   rejection/transform (`wp-c10.md` item 6) — consistent with CQRS + event sourcing, not CRDT merge.
2. **No CRUD overwrite found in the write path.** Document mutation is envelope/checkpoint-based throughout: `DocumentCheckInJob`
   replays envelopes onto a pair via `TrustedArtifactReplayCodec::replay_envelopes` (`🌎️hub/🗿️artifact-authority/📌️check-in/🦀️.rs`),
   never a blind field overwrite; `ComponentDocumentCodec::genesis` mints identity, never patches state directly.
3. **`HubInstance::Documents = NoDocumentAuthority` is an intentional non-generic dead slot**, confirmed still true in source
   (`🌎️hub/🗄️stores/🦀️.rs:61`) with its doc-comment explaining hub's own router (not the generic `ServerInstance::Documents`
   trait) owns `/scopes/{scope}/document/ws`. This is documented as deliberate architecture (s11 audit §5.1), not a violation —
   flagging only so a future session doesn't "fix" it by wiring the trait slot.
4. **Short-shortage-doesn't-freeze is source-real, not just a claim.** `suspendDocumentBrowserActorLink` keeps a MOUNTED child's
   lease alive across a socket drop (actions keep applying locally, effects queue in `state.outbox`) instead of tearing down
   and cold-rebootstrapping on every reconnect — this is the load-bearing mechanism behind the "no freeze" live proofs (React
   `c10out4/5/6`, wgpu-native gate step 12). Both shells implement the *same shape* (suspend-and-resume, not retire-and-reopen)
   independently (TS worker vs. Rust `🐚️Shell/🎯️targets/🧊️wgpu`), which is consistent but is two independent implementations
   of one concept with no shared kernel primitive — worth a P2 note for a future consolidation, not a correctness bug.
5. **One real architecture gap: the hub's document-growth bounds were CRUD-shaped in effect even though the write path is
   event-sourced.** H9 Item g root-caused three *storage-engine* bounds (index-run-per-operation credit, a 64-edit in-memory
   VCS ledger, and un-retired replaced state values) that made a long-lived, append-only document become effectively
   **write-locked after ~20-125 edits** — not a CRUD/CRDT violation in the strict sense, but a place where the storage engine's
   fixed-size internal bookkeeping silently defeated the event-sourced model's promise of unbounded append. Fixed in source
   (confirmed: `RunView`, `VcsStoreCell`, `artifact_state_retirement_maintenance_step` all present), but the live proof
   (`wp-c10.md` task-3 reconcile failure) predates the fix and was never re-run against it.

No freeze-on-disconnect, no field-overwrite, and no CRDT reconciliation code were found anywhere this audit's targeted
greps reached. This is a narrower sweep than a full repo audit (time-boxed, read-only); treat absence of a hit as
"not found by this search," not as an exhaustive guarantee.

---

## 3. Ranked gaps (P0/P1/P2), owned vs unowned

### P0

- **P0-1 — [UNOWNED] Re-run the reconcile-after-shortage failure against H9's landed document-growth fix.** C10 measured
  a live BROKEN reconcile (`Ack Rejected: DB I/O aggregate admission exhausted`) that is almost certainly the exact bug
  H9's Item g fixed in source (confirmed landed: `RunView`/`VcsStoreCell`/retirement drain). Nobody re-ran C10's
  `c10out6`-style scenario against an H9-fixed hub before session 11 ended. **Acceptance:** re-run the short-shortage
  live scenario end-to-end (offline edit → reconnect → both peers converge) on a hub built from the current tree.
  **Slice:** C10 (scenario owner) + H9 (fix owner) — was unowned at session-11 close, needs explicit pickup.
- **P0-2 — [OWNED, WG7] Re-run the wgpu-wasm32 attach after the `io.artifactSchema` fix.** WG7 root-caused and landed
  the fix (`editor_surface`/`viewer_surface` stamp the document schema) at 15:2x but every attempt to attach afterward
  was blocked on catalog/lane skew, not the fix itself (`wp-wg7.md` 15:34 onward — still assembling a matching catalog
  when the log ends at 17:11). **Acceptance:** `wg7-browser-collab.mjs` reaches the document socket `Live` on wasm32,
  matching WG8's native step 6. **Slice:** WG7 (already its own open item).
- **P0-3 — [UNOWNED] Fresh-door-artifact genesis for React.** WG8 fixed and live-proved this for native
  (`open_document` loads `codec.genesis` before the actor mounts); this audit confirms **no equivalent exists on the
  TS/React side** (0 hits searching for a genesis call under `🏪️store`). Without it, a React user opening a
  never-before-loaded hub-created document will author under the wrong `documentId` and every edit will be refused
  as `document backbone scope mismatch` — the exact bug WG8 found natively. C10's own report ends with WG8's open
  question to it, unanswered. **Acceptance:** React's open-plan flow loads the component's genesis (or equivalent)
  before its first outbound `Commands` envelope on a fresh door artifact; a live run proves it. **Slice:** C10 (WG8
  flagged it explicitly to C10 at the end of `wp-c10.md`) — currently unowned/unacknowledged in C10's own status table.

### P1

- **P1-1 — [OWNED, C10] Live STEP-14 peer-cursor proof against a catalog that actually carries writer/draw/puzzle.**
  Unit/fixture-level `CanvasPresenceOverlayV1` proofs exist but no live two-browser run has ever reached in-canvas
  cursor rendering, across two full sessions, because every catalog available so far only opened gismap+note (catalog
  A) or was still being assembled (catalog B). **Acceptance:** two browsers see each other's live cursor/selection in
  at least one of writer/draw/puzzle3d, hub-served. **Slice:** C10 (its own item 1/7, carried from session 11).
- **P1-2 — [OWNED, C10] Same-field conflict scenario (not just additive-edit convergence).** Every live convergence
  proof so far (React and native) has both users adding *different* things; no run stages an actual field collision.
  The localized-notice mechanism (`ui.conflict.hubRejected`/`hubTransformed`) is unit-proven and ready to receive such
  a run. **Acceptance:** A and B both mutate the same field; the loser sees the visible notice, not silent data loss,
  live on a real hub. **Slice:** C10 (G-P2-3, its own item 6, "same-field live scenario waits for a writable 7800").
- **P1-3 — [OWNED, C10] Live viewer-role proof.** `c10perm1` shows the viewer path is currently BROKEN at the catalog
  level (spectator's open-plan gets a 503 because catalog A only has editor open-targets), not proven working — this is
  a real live-tested failure, worth tracking distinctly from "MISSING." **Acceptance:** a seated viewer opens the
  document read-only (no edit affordance) against a catalog that actually publishes a viewer surface. **Slice:** C10.
- **P1-4 — [UNOWNED] Cross-shell (React↔wgpu, either variant) live collaboration has literally never been attempted.**
  Every P0/live proof in this fleet is same-shell-pair (React↔React, native↔native, agent↔MCP). No wp report describes
  a run with one React browser and one wgpu shell on the same document. Given both React and wgpu-native independently
  reach "Live" on a hub document, the missing piece may be small, but it is untested and unowned. **Acceptance:** one
  React user and one wgpu-native (or wasm32, once P0-2 lands) user co-edit one hub document, live. **Slice:** none
  currently named — recommend C10+WG8 jointly, or a new coordinator-assigned slice.
- **P1-5 — [OWNED, H9, blocked on catalog] Live pg/neo4j document-growth and restart-under-1s proofs.** H9's Item g and
  Item b fixes are code-complete and unit/nextest-proven on sqlite; the live two-client e2e growth run and the
  hub-level pg/neo4j restart probe both need "a current-tree catalog," which was still not stably available (catalog B
  publish failed 3 times during session 11) when the log ends. **Acceptance:** the two-client e2e's 300+30-edit growth
  run and the SIGTERM-restart probe both complete on postgres and neo4j backends, not just sqlite. **Slice:** H9 (its
  own items a/b/g, explicitly still open).

### P2

- **P2-1 — [OWNED, H9→W2/coordinator] Warn every hub owner: 7800's binary as of session 11's end (`1a5cf10…`) still has
  the pre-fix ledger CHECK (64 KiB) and the pre-stack-budget crash.** G10 measured that a gis `inference_approve` on
  that exact binary **aborts the entire hub process** (tokio stack overflow) and is explicit that it ran zero
  approvals against 7800 for this reason (`wp-g10.md`, "Warning for W2/coordinator"). This is a P0-grade live landmine
  for session 12 if nobody rebuilds 7800 before the next approval-flow run. **Acceptance:** session 12's hub 7800 (or
  successor) is rebuilt from a tree containing G10's ledger-bound and stack-budget fixes before any inference-approval
  scenario runs against it, and its `inference/gis-map-jobs.sqlite3` is fresh (old tables keep the stale CHECK).
  **Slice:** whoever restarts the canonical hub in session 12 (W2 in session 11's numbering).
- **P2-2 — [UNOWNED] Consolidate the two independent "suspend-not-retire" link-shortage implementations.** React's
  `suspendDocumentBrowserActorLink` (TS, worker) and wgpu-native's equivalent frame-pump suspend logic solve the same
  problem with no shared kernel type. Not a bug, but drifted twice already (see architecture §2.4) and worth one
  kernel-level primitive before a third shell (wasm32) reimplements it a third way.
- **P2-3 — [UNOWNED] "Which plugin kinds can collaborate" is still largely UNKNOWN, not MISSING.** Only gis and note
  (and block, natively) have any live collaboration proof. cad, procedural, fem, space/home and everything else have
  never been driven through a hub-collaboration scenario in two sessions — absence of evidence, not evidence of
  absence, but worth a deliberate sweep rather than continuing to assume it via silence.
- **P2-4 — [OWNED, C10, still TODO] Delete the dead `checkpoint-publications` upload path if any non-Rust caller
  remains.** This audit re-confirms the Rust-side route and types are fully gone (0 grep hits repo-wide); this item
  should be closed as done unless a TS/JS caller is later found — worth one more grep pass by whoever owns it next,
  purely to close the loop, not because live evidence suggests it is still there.

---

## 4. What this audit could not verify

- Every "live" number cited above is a **snapshot from session 11's last measurement**, not re-run tonight (session 12
  has not started substantive work; no hub is up). Treat every LIVE-PROVEN cell as "was true as of the run's own
  timestamp on a tree that has since kept moving" — the usual staleness risk in this fleet (catalogs, lanes and
  binaries drifted under session 11's own feet multiple times per the wp reports themselves).
- This audit's own source greps were scoped (targeted paths, not a full-repo sweep) because full-repo `grep`/`rg` calls
  against this tree took >120 s and were backgrounded; two whole-repo `/usr/bin/grep` passes for `checkpoint-publications`
  did complete (0 hits, both confirmed), but a broader "find every CRUD/CRDT-shaped pattern anywhere" sweep was not
  attempted — only the specific paths named in the brief (kernel sync/store, hub document authority, worker).
- Cross-shell (React↔wgpu) collaboration status is inferred as "never attempted" from the absence of any such run in
  five detailed wp reports plus the prior session's audit — this is a strong absence-of-evidence signal but not a
  direct negative proof (no report explicitly says "we tried and it failed").
