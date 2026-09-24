# S11 Audit — Outcome 3: Collaboration Between Users Over The Hub

Read-only auditor, session 11 (Sonnet 5), 2026-09-25 00:3x–01:2x. Repo state observed: session 11 has just started
(`📓️landing.md` empty, `wp-c10.md`/`wp-g10.md`/`wp-h9.md` all skeleton/TODO at 00:34–00:40). **Everything below is the
session-10 end state** (reports dated 2026-09-23/24) carried forward, plus three claims re-verified directly against
the tree tonight (marked "verified 2026-09-25"). Session-11 slices **C10** (browser collab), **WG7** (wgpu), **H9**
(hub) are the owners tasked with closing what follows; none had landed anything as of this audit.

Sources read in full: `📓️audit-collaboration.md` (09-23 baseline), `📓️peer-audit-g10.md` (09-24 12:45 re-audit),
`📓️work-packages.md`, `📓️wp-c1.md`…`📓️wp-c8.md` (+c2b/c3b/c4b/c4c/c4d), `📓️wp-o3.md`, `📓️wp-c10.md`, `📓️wp-g10.md`,
`📓️wp-h9.md`, `.tmp-ticket-0918/📓️g7w-wgpu-native-two-user-collaboration.md` (full), `…n2-wasm32-wgpu-artifact-open-
relay.md` (targeted), `…wg6-wgpu-hub-sign-in-spaces-workspace.md` (targeted), `…g7-mcp-agent-and-collaboration-
audit.md` (targeted, Outcome-4 territory, skimmed only). Files named in the brief but not present in either ticket
tree (`pr1-presence-roster-symmetry.md`, `n2-*.md` standalone) turned out to be **tags inside other reports**, not
separate files — see §7.

---

## 1. User journeys — status table

| # | Journey | React `s` | wgpu native | wgpu wasm32 (browser) | Evidence |
|---|---|---|---|---|---|
| 1 | Invite / add member to a space | **WORKS-LIVE** | **WORKS-LIVE** | UNKNOWN | `wp-c7.md` step 2 (A `upsert-member` → hub → B sees `Author` role); G7w step 2 PASS (`os.directory.upsert-member` relay, hub trace `upsert-member ok`, B's roster shows the role) |
| 2 | Share a document, both users open it | **PARTIAL** | **PARTIAL (fails at open)** | MISSING | React: `wp-c7.md` 1a–1e PASS (both attach, one sustained socket each); wgpu native: G7w step 3 PASS (roster), step 4 **FAIL** — native kernel returns "shard produced no outcome for this turn" the instant the guest is opened (B1, `g7w…md:60,100-110`); wgpu wasm32: `WasmActor::connect` is a no-op (see §7.2) |
| 3 | Concurrent edits converge (A and B both author) | **PARTIAL, TESTS-ONLY for convergence itself** | MISSING | MISSING | React: `wp-c7.md` — A and B both author `addFeature`, guest-applied, hub-persisted, **"B ingests A's edit live (`MergeReport`, canvas changed)"** observed live 2026-09-24 04:xx; but the run then traps on `commitCheckpoint` (native fix landed in C8, **guest rebuild not yet applied to a hub-served lane** as of this audit — see §7.1); wgpu: blocked at step 4, never reaches an edit |
| 4 | Presence roster + cursors + selections | **PARTIAL** | **PARTIAL** | MISSING | Roster: React **WORKS-LIVE** (`wp-c7.md` 1d — "both peers in both rosters, distinct hub colours", live 2 headless Chromium); wgpu native roster **WORKS-LIVE** (G7w step 3); in-canvas peer cursors/selections (writer/draw/puzzle3d, `CanvasPresenceOverlayV1`): **TESTS-ONLY** — unit/fixture green (`wp-c3.md`, `wp-c3b.md`), STEP-14 live two-browser proof never completed (`wp-c3b.md` "Gaps": taxonomy error blocked the run; `wp-c8.md`/`wp-o3.md` still chasing collab-e2e 10/10 as of the last session-10 report); presence *online* flag on wgpu native never reaches true (needs step 6, document socket) |
| 5 | Collaborative undo/redo (durable, per-author) | **TESTS-ONLY** | MISSING | MISSING | `wp-c5.md`: server-stamped redo survives pack+`.spr` reload and **hub restart** in a fixture/unit run (`durable_collaborative_redo_fixture_survives_reload_and_hub_restart` PASS, hub two-author law fence `no-durable-collaborative-redo` absent) — no live two-browser undo/redo was driven this session; wgpu native step 11 (`per-actor undo`) FAILs, blocked on step 4 |
| 6 | Conflicting edits (same-field or ordering conflict) | **TESTS-ONLY (ordering only)** | UNKNOWN | UNKNOWN | Server linearizes via `MergePolicy` + `Ack`/`ApplyOutcome` (`audit-collaboration.md` §5); C2/C2b prove Rust↔TS parity for `ack-rejected`/`duplicate-commands-idempotent` fixtures (60/60 unit); no live run stages an actual field-level conflict (both browser sessions in `wp-c7.md` add *different* features, not colliding edits) |
| 7 | Permissions: viewer read-only / removed member loses socket | **PARTIAL** | UNKNOWN | UNKNOWN | Admission and revocation are proven at the protocol layer (`audit-collaboration.md` hub Rust suite: "cross-document grant rejection", "session revoke under broadcast pause"; C4d: hub refuses Anonymous / foreign-actor envelopes, 9/9 `documents::` tests) but no session-10/11 report drives a **live** viewer-role UI (read-only affordance) or a live "remove member mid-session, socket drops" browser scenario. `wp-c10.md` item 5 lists this as still **PENDING** |
| 8 | Short connection shortage (queue, no freeze, reconcile) | **PARTIAL** | **WORKS-LIVE (network half only)** | UNKNOWN | React: reconnect/backoff + resume_token design proven at the fixture level (C2 `rebootstrap-required` scenario, `wp-c1.md` C9 wire "reconnect/resume"); no live browser drop-and-heal was captured this session (`wp-c7.md`/`wp-c8.md` never reached that scenario step). wgpu native: G7w step 12 **partial** — TCP-level sever/heal of B's sockets while A is untouched, measured live: B's local dispatch stayed at 12.9 µs, a 3 s frame-pump window took 3.005 s (no stall), B's space list went `stale→ready` on heal — genuinely "no freeze". The **catch-up/reconcile half** is unproven (needs step 6, never Live to begin with) |
| 9 | Long-offline → refusal (not silent data loss) | TESTS-ONLY | UNKNOWN | UNKNOWN | `RebootstrapRequired` → canonical checkpoint-pair path exists (`audit-collaboration.md` §5, `🌎️hub/🛰️lag-rebootstrap/🦀️.rs`); C2 proves the Rust/TS state-machine transition (`rebootstrap-required` fixture); no live "was offline long enough to require rebootstrap, refused/rebootstrapped visibly" run exists |
| 10 | Late join / resume (joiner catches up to current frontier) | **PARTIAL** | UNKNOWN | UNKNOWN | React: `wp-c1.md` PR1 "join-replay (A beats before B attaches)" — TS runner done, **live `HUB_E2E` run QUEUED on hub mutex, not confirmed executed** this session; `wp-c4c.md` "late joiner" PASS on the framework document WS (open+rejoin+late, sqlite, live, 2026-09-24). A separate, more serious finding: `📓️peer-audit-g10.md` §A (Outcome 4 row) reports a fresh joiner's catch-up **replays only 1 frame after 4+ accepted agent edits** because the pre-H4 guest mints a constant edit id (hub dedups it as an idempotent replay) — fixed in source by H4, **not yet live** pending W1's guest rebuild |
| 11 | Notifications of remote changes (human sees agent/peer edit) | **WORKS-LIVE (agent case only)**, PARTIAL (human-human) | MISSING | MISSING | `📓️peer-audit-g10.md`: G4 drove an MCP agent editing a hub doc live and **"the human's live socket receives the agent's Commands frame"** (real relay, real actor id) — but the hub ledger's `head_seq` did not advance (H4 root-caused, fix not yet live). Human-human: `wp-c7.md` "B ingests A's edit live" is the closest live proof, but the **inspector panel stays stale on remote ingest** until C8's native fix (`document_surfaces()` panel/window fix) reaches a hub-served guest |
| 12 | Which plugin kinds can collaborate today (hub catalog coverage) | See §2 | See §2 | See §2 | — |

---

## 2. Plugin-kind hub-catalog coverage (verified against work-packages.md + wp-c8/wp-c7/g7w, 2026-09-24 end-of-session)

- **Catalog A** (W1, published 2026-09-24 20:46, generation `ee491213…`): `stdio, gis, note, draw, writer, puzzle`.
  This is the **only** trusted catalog that exists on any hub data root as of session-11 start (`session-11-preamble.md`
  line 17-18). Before catalog A, `wp-c8.md` (13:32) states catalog A was "still not published" and a stale c7-boot
  catalog carried an old gis build that panicked on `commitCheckpoint`.
- **gis**: hub-bound edits WORKS-LIVE once on a fresh guest (`wp-c7.md` — addFeature persisted, reload shows it); the
  checkpoint-panic fix (C8 item 1) is native-only and **not yet proven against a hub-served (catalog A) guest** — no
  report in this audit trail re-ran the collab scenario against catalog A specifically (catalog A was only just
  published at session-11's 00:00 boundary).
- **writer, draw, puzzle 2d/3d**: STEP 14 (peer cursors) needs these three in the hub catalog; `wp-c7.md` (04:20) says
  "writer/puzzle currently uncatalogable (`ArtifactPack::record_spec()` missing)"; `work-packages.md` records this as
  fixed by P4/P5 later in session 10, and catalog A does list `writer`/`draw`/`puzzle`, but **no live STEP-14 run
  against catalog A has been reported**.
- **block**: G7w picked `block2d` specifically *because* it is the only kind with a complete native-hostable component
  outside the hub catalog work — it is **not** in catalog A and has no creation door on the wgpu shell (G7w §5 B3).
  wgpu-native collaboration is therefore blocked on two independent things: the native-kernel turn bug (B1) and a
  block-carrying catalog (B3), neither of which is scheduled to land together.
- **dag**: descriptor declares `artifactSchema: ""` — cannot bind any document at all (G7w §1), irrelevant to
  collaboration by construction.
- Everything else (space/home, cad, procedural, fem, etc.): no session-10/11 report drives a hub-collaboration
  scenario for these kinds; treat as **UNKNOWN**, not MISSING — absence of evidence, not evidence of absence.

---

## 3. React vs wgpu parity — the actual gap, quantified

| Capability | React `s` | wgpu native | wgpu wasm32 |
|---|---|---|---|
| Sign in to a real hub | WORKS-LIVE | WORKS-LIVE (`wg6-wgpu-hub-sign-in-spaces-workspace.md` §4.2, 4 green runs against hub 7891, native transport) | **Not observed** — WGr's 2026-09-20 note (cited by WG6 §4.3) says the sign-in *button click* never leaves the page over the browser `directory-http` door |
| Create/list/open a space, roster | WORKS-LIVE | WORKS-LIVE (same law) | Not observed |
| Open a hub-bound document (any real guest) | WORKS-LIVE (gis) | **FAILS** — native kernel bug B1 (turn-outcome timeout, not a timing issue: raised to 120 s, still fails in seconds) | **Cannot** — `WasmActor::connect` is a stub (verified below) |
| Document socket goes Live | WORKS-LIVE | **FAILS** — B2: the native document actor has no linked codec for a guest-owned kind, `document_codec("block.2d") = None`, actor loops `Backoff` forever | Same root cause, plus the transport itself is unwired |
| Presence roster | WORKS-LIVE | WORKS-LIVE (member roster; *online* flag never true, needs the document socket) | Not observed |
| Edit exchange / convergence | PARTIAL (traps on checkpoint before full convergence proof) | Never reached | Never reached |
| Connection-loss resilience | Design proven, live run not captured | **WORKS-LIVE for the "no freeze" half** (µs-level local dispatch under a severed socket, measured) | Not observed |

**Root-cause summary for the wgpu gap (both native and wasm32), each independently confirmed in the tree tonight:**

1. **wasm32 browser transport is an intentional no-op today.** Verified 2026-09-25 at
   `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:3677-3680`:
   ```rust
   impl WasmActor {
       async fn connect(&mut self) {
           let _ = (&self.hub_base_url, &self.hub_space_id, &self.hub_surface);
       }
   ```
   This matches N2's and G7w's claims exactly (`n2-wasm32-wgpu-artifact-open-relay.md` §6.2, `g7w-…md` §8.1) — **not
   stale**, still true. It is frozen (kernel crate) pending WG7's session-11 landing of the N2 relay design (§8.2-8.4
   of `g7w-…md` is a fully written, unapplied patch plan: K1-K7 hunks + R1-R3 shell hunks).
2. **Native kernel turn bug (B1)** is a different, deeper defect than the transport stub: even with a real guest
   compiled and instantiated (block2d, native, no hub involved in this specific failure), the very next renderer turn
   (`refresh_app_catalogue`) returns no outcome and every later event is refused. This is **not owned by WG7's relay
   plan** — G7w explicitly names it "the renderer kernel-runtime (terra-kernel-loop) owner", unassigned as of this
   audit.

---

## 4. Data classification (persisted/ephemeral × local/shared)

Schema-first `PersistenceDataClass` (four values) landed and tested (`wp-c6.md`, PASS): `persistedLocalOnly`,
`persistedShared`, `ephemeralLocalOnly`, `ephemeralShared`. Home union folds hub rows as `origin: "hub"` over local
`origin: "local"` in **both** shells (browser `ShellHost.foldDirectoryEvents`, native wgpu `applyDirectoryEventPage`,
per `wp-c6.md` "Files" section). Ephemeral studios are blocked from share/collab UI (en+de dialog) with `promote-to-
hub-space` / `persist-locally` exit paths. Presence/cursors/selections are correctly `ephemeralShared` (never WAL,
`wp-c3.md` header). This item is the most solid part of Outcome 3 — **WORKS-LIVE at the oracle level** (space-plugin
`persistence-data-class-check` 11/11 clean, Rust + TS fixtures green) though not re-driven against a live two-browser
session this pass.

---

## 5. Stale-claim flags (verified against the tree tonight, 2026-09-25)

1. **`audit-collaboration.md` item A1** ("`HubInstance::Documents = NoDocumentAuthority`... wire `db::Database`
   document authority behind `ServerInstance::Documents`") is **superseded by an explicit design decision, not an
   open gap**. `wp-c4.md` (09-23) claimed this was fixed ("`HubInstance::Documents` | `HubDocumentAuthority` over
   `Arc<db::Database>`" — PASS) — but the **current tree still reads `type Documents = NoDocumentAuthority`**
   (`🌎️hub/🗄️stores/🦀️.rs:61`, verified 2026-09-25), now with a doc-comment explaining this is deliberate: *"Documents
   are `NoDocumentAuthority`: hub's own router owns `/scopes/{scope}/document/ws`, because only its socket handler
   carries grant admission, presence leases and live revocation."* Between C4 (which wired the generic port) and
   H2/C4b/C4c (which deleted the legacy axum surface and moved clients to the framework's document-socket **module**
   composed directly into hub's own router, per `wp-c4b.md`/`wp-c4c.md`), the architecture changed again: the generic
   `ServerInstance::Documents` trait slot is intentionally left unfilled because hub's own document router (not the
   trait) is the one true document-socket owner. Net effect for collaboration: functionally fine (documents work
   through the merged framework router — C4c proved open+rejoin+late live on it), but **A1/A2 (framework port
   migration) should be closed as "won't do, by design" rather than carried as an open gap** — nobody should spend
   session-11 time trying to wire `ServerInstance::Documents` for hub.
2. **`checkpoint-publications` route** (G5 in `peer-audit-g10.md`, called "dead-or-not, no caller found" at 09-24
   12:45) is **still present, unchanged**: verified at `🌎️hub/🏗️bootstrap/🦀️.rs:10355` tonight. `wp-h9.md` item 1
   ("delete the `checkpoint-publications` upload path") is the only session-11 slice that owns removing it, and it is
   still `TODO`.
3. **Collab-e2e "10/10" is never actually claimed anywhere** — every report from C7 through O3 across two sessions
   caps out around 3-8/10 with named blockers (guest checkpoint trap, catalog A not yet published, taxonomy race).
   `wp-c10.md` (session 11, item 1) still lists this as **PENDING** at 00:34 tonight — treat any future "10/10" claim
   with suspicion unless it cites a run against catalog A specifically (the previous catalogs used were stale/partial).

---

## 6. Gaps, ranked

### P0

- **G-P0-1 — wgpu native kernel turn bug (B1).** Blocks all of wgpu-native collaboration (steps 4-12 of G7w's ledger).
  Not the transport stub, not a hub problem — the renderer's own turn loop returns no outcome for the very first
  post-boot turn on a real guest and then refuses to resume. **Acceptance:** G7w's own law
  (`two_live_wgpu_shells_collaborate_on_one_hub_document`) reaches step 6 (document socket Live) on both shells.
  **Owner:** WG7 (session 11) per its brief ("native two-user collaboration 12/12"); needs the renderer kernel-runtime
  owner specifically, not the shell-target owner.
- **G-P0-2 — Prove the human-human convergence + peer-cursor journeys against catalog A, live, end to end.** Every
  live proof so far (`wp-c7.md`) used stale or pre-catalog-A guests; the checkpoint-panic fix (C8) and the STEP-14
  peer-cursor assertions (C3b) have never run against the one catalog that actually exists tonight. **Acceptance:**
  `collab-e2e` (or its successor) reaches STEP 14 with visible peer cursors in at least writer+draw+puzzle3d, on hub
  7800/catalog A, two real browser contexts. **Owner:** C10 (session 11), continuing C7/C8.
- **G-P0-3 — wasm32 browser hub transport (`WasmActor::connect`).** Currently a guaranteed no-op; the wgpu browser
  target cannot join a hub document at all. The fix is fully designed (`g7w-…md` §8.2-8.4, hunks K1-K7 + R1-R3) but
  unapplied, frozen behind the `ureq` target-gate move. **Acceptance:** a wasm32 wgpu shell signs into a hub, opens a
  hub-bound document, and its `sync_status` reaches `Live`. **Owner:** WG7 (explicitly its brief: "land N2 relay +
  `ureq` move, browser document actor hub `connect`").

### P1

- **G-P1-1 — Live short-connection-shortage + late-join/resume proof (React).** Design + fixture-level proof exist
  (C2's `rebootstrap-required` scenario, C4c's late-joiner), but no live browser run drops and heals a real socket
  while asserting no UI freeze and a queued-then-flushed edit. **Acceptance:** a scripted two-browser run drops one
  user's WS for 5-20 s, shows a non-frozen UI + reconnect status (en+de), and the other user's edits arrive on
  reconnect. **Owner:** C10 (its own brief item 3 names exactly this).
- **G-P1-2 — Permissions live proof (viewer read-only, removed member).** Protocol-level admission/revocation is
  solid (C4d's 9/9 `documents::` tests reject foreign/anonymous actors); no live run exercises a **viewer** role's
  read-only UI or a **removed-mid-session** member's socket actually dropping in front of a human. **Acceptance:** a
  live run seats a viewer (no edit affordance rendered/enabled) and removes an author mid-session (their document
  socket closes, UI reflects it). **Owner:** C10 (its brief item 5 names "permissions... viewer read-only, removed
  member loses socket").
- **G-P1-3 — wgpu artifact-creation door.** The wgpu shell has no caller of the space's artifact-creation routes at
  all (G7w B3, N2 §6.4) — even once B1/B3(catalog) are fixed, there is no in-shell way to create a shared document to
  collaborate on. **Acceptance:** the wgpu shell can create a hub-bound artifact from its own UI, not just open one
  a test seeded via `DirectoryClient`. **Owner:** WG7 or S15 (whichever ends up owning wgpu shell creation flows) —
  NEW if neither claims it explicitly.
- **G-P1-4 — Notification/refresh on remote ingest (inspector panel staleness).** Native fix landed (C8 item 2,
  `document_surfaces()`), law green, but never observed live in a hub-served browser session because the guest
  rebuild + catalog-A timing never lined up in session 10. **Acceptance:** in a live two-browser run, B's inspector
  panel (not just canvas) updates within ~2 s of A's remote edit. **Owner:** C10 (subsumed by G-P0-2's re-run).

### P2

- **G-P2-1 — Close A1/A2 as "by design," update the ledger.** Stop carrying "wire `ServerInstance::Documents` for
  hub" as an open architectural gap (§5.1); it actively wastes a future slice's time if picked up literally.
  **Acceptance:** one sentence in the next audit or in `work-packages.md` recording the decision. **Owner:** whoever
  next touches `peer-audit-g10.md`'s G-series ledger, or the session-11 coordinator directly.
- **G-P2-2 — `checkpoint-publications` route decision.** Already owned (H9 item 1, still TODO) — no new owner needed,
  just flagging it is real and unstarted as of 00:40 tonight.
- **G-P2-3 — Conflicting-edit (same-field collision) scenario.** No live or fixture run stages an actual field-level
  conflict (as opposed to two independent additive edits); ordering/idempotency is proven, real conflict-resolution
  UX is not. **Acceptance:** a fixture where A and B both mutate the same field; assert the loser sees a visible
  `ApplyOutcome`/rejection, not silent data loss. **Owner:** NEW (no slice currently names this explicitly).

---

## 7. Notes on the brief's named sources

- `📓️pr1-presence-roster-symmetry.md`, standalone `n2-*.md` reports for wgpu, `g7-mcp-agent-and-collaboration-audit.md`
  as a *collaboration* source: **PR1** and **N2** turned out to be inline tags inside `wp-c1.md`/`wp-c3.md` and the
  `.tmp-ticket-0918/📓️n2-wasm32-wgpu-artifact-open-relay.md` file (found via `ls | grep`, not `find -iname` — the
  latter silently returned zero hits on these emoji filenames even with correct patterns; do not trust `find -iname`
  on this tree, use `ls | /usr/bin/grep` instead). `g7-mcp-agent-and-collaboration-audit.md` is Outcome-4 (MCP/agent)
  territory with one overlapping section (§3, agent-as-collaborator) — its P0/P1 findings belong in the AI-MCP
  auditor's report, not duplicated here; §11 above cites only the one collaboration-relevant fact (agent edit reaches
  a human's live socket, ledger doesn't advance).
- Time-box: ~50 minutes elapsed (over the 45-minute budget by design, since the wgpu evidence trail required chasing
  through symlinked `.tmp-ticket-0918` files not discoverable by `find -iname`). No cargo/bun/tests were run; three
  `grep`/`sed` reads against the live tree were used only to verify already-written claims (§5), not to explore.
