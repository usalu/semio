# S13 Audit — Outcome 3: Collaboration Between Users Over The Hub (React + wgpu native + wgpu wasm32)

Read-only auditor A13-collab, session 13 (Sonnet 5), 2026-09-26 ~19:2x–20:0x. No builds/servers started by this audit;
hub 7800 (pid 54029), a hub on 8040 (pid 17050) and a hub on 8050 (pid 79010) were already listening when this audit
ran (other session-13 slices are live). This is a fresh gap list built from (a) the session-12 audit
(`📓️audit-s12-collaboration.md`), (b) the session-12 sections of `📓️wp-c10.md`, `📓️wp-wg7.md`, `📓️wp-wg8.md`,
`📓️wp-g10.md`, `📓️wp-h9.md` (full) and `📓️wp-t12.md` (partial, targeted), and (c) direct read-only re-verification
against the CURRENT source tree (`git grep`/`Read`, no build) — this audit trusts the source over any wp report where
they disagree, and says so.

**Source re-verification performed this session (all read-only, current tree):**

- **CONFIRMED STILL BROKEN, unowned:** `flush_apply_outbound` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19169-19195`,
  target-neutral guest store, compiled into every plugin document actor on every shell) re-encodes and re-sends
  `self.envelope.vcs.edits.last()` on every call, and when `self.dag.seed_applied(mutation_id)` answers
  `Err(Duplicate)` the arm is `{}` (swallowed) and the function **still sends that envelope on the backbone anyway**
  (line 19182-19186). This is the exact shape of C10's 09:4x live finding ("the second keystroke's guest backbone
  message re-carries the first, already Accepted operation byte-for-byte … the hub rejects the batch"). C10 routed
  this to T12 at 09:4x; `git`-searching `📓️wp-t12.md`'s whole session-12 log for `outbound`, `keystroke`,
  `flush_apply`, `duplicate.*op` returns **0 hits** — T12 never picked it up (most likely reason: rule 20's hard
  guest freeze from 05:45 landed before T12 reached it, and T12's own log ends without returning to it). **This is
  still live in source today** — see P0-1.
- **CONFIRMED TWIN DRIFT, unowned:** the echo-suppression-by-operation-identity fix WG7 designed at 17:1x
  (`admitRemoteEnvelopes`/`noteAuthoredEnvelopeIds`) is landed **only on the React TS side**
  (`git grep` hits in `🏪️store/👷️worker/🟦️.ts`, `🛍️products/💻️os/🟦️.ts`,
  `🧪️tests/🔁️document-echo-suppression/🟦️.ts`). The Rust kernel twin (`admit_remote_envelopes`,
  `note_authored_envelopes`, `applied_op_ids` in `🏪️store/🔄️sync/🦀️.rs`, shared by native wgpu **and** wasm32 wgpu)
  and the hub twin (`HUB_CATCH_UP_ORIGIN` in `🌎️hub/🏗️bootstrap/🦀️.rs`) return **0 `git grep` hits anywhere in the
  tree** — both are still only `wp-wg7/s12-echo-suppression-{kernel,hub}-patch.py`, dry-run-verified but never
  applied. Native wgpu and wasm32 wgpu shells therefore still show a late joiner / reopener **0 of N existing
  blocks** (WG7's own measured symptom, `s12i-reopen`), while React does not. See P0-2.
- **CONFIRMED EMPTY:** `.tmp-ticket/📓️landing.md` `# Session 13 Landing Window` (opened 19:0x by the coordinator) has
  a header row and **zero landed rows** as of this audit. Every session-12 prepared-but-frozen patch set (WG7 echo
  suppression kernel+hub, WG7 link-expiry, WG8 transport-deadline, H9 opaque-concurrency, H9 per-kind labels, H9
  creation-progress, T12's stdio kind-id post-publish set) is still sitting as an unapplied `.py`/`.diff` in its
  slice's ticket folder. Hub 7800 is live right now on the OLD pre-landing binary.
- **PARTIALLY CONTRADICTS the wp-wg7 report (good news):** `DOCUMENT_LINK_SHORTAGE_POLICY` / `DocumentLink::apply` /
  `retry_at` / `ceiling_at` (`🏪️store/🔄️sync/🦀️.rs:1100-1260`) already implement the bound-capped backoff
  (`retry_at = min(now+backoff, since+bound)`, `ceiling_at = since+bound+reconnect_max`) that
  `wp-wg7/s12-link-expiry-patch.py` was written to add — the current source is **not** the buggy shape WG7's 10:1x
  log describes. Either this patch landed by another path than the log records, or the live bug WG7 measured was
  against a stale binary. Net effect: the connection-shortage/expiry mechanism reads as source-correct today; not
  independently re-run live by this audit.
- **CONFIRMED NOT LANDED:** `ArtifactKindSpec` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:5593`) has no `label` field
  (still `pub name: String`) — H9's per-kind localized-label patch (item L) is unapplied.
- **CONFIRMED NOT LANDED:** no `causal_head` symbol anywhere in the tree (`git grep`, 0 hits) — H9's Qa
  same-field-opaque-write store half (`ArtifactStore::causal_head()`) is unapplied; a vigilant hub still cannot
  distinguish an aware concurrent write from a blind one.
- `grep -rn "CRDT|crdt"` under `🌎️hub` and `🏪️store`: not re-run this session (no reason to expect it changed since
  s12's confirmed 0 hits); still trusted from the s12 audit.

---

## 1. Session-12 audit items — closed vs open

| s12 item | Verdict now | Evidence |
|---|---|---|
| P0-1 re-run reconcile-after-shortage against H9's DB-fix | **CLOSED for the DB-side cause, OPEN on a new cause** | H9's `g14`–`g18` growth/admission fixes are landed and no longer reproduce `DB I/O aggregate admission exhausted` live (`wp-h9.md` g15/g17/tc18). But C10's own later outage runs (`wp-c10.md` 10:0x–10:1x) found reconcile still fails for a *different* reason: B's staged edit during a cut is not submitted to the worker until ~5 s after reconnect (a hold between the staged form and the submit, never located). Not re-verified against a rebuilt hub this session (landing window empty). New home: **P1-4 below**. |
| P0-2 re-run wgpu-wasm32 attach after `io.artifactSchema` fix | **CLOSED, then reopened one level up** | WG7 proved wasm32↔wasm32 collab live 3 times (`s12c` 10/13, `s12f` 20/20, `s12i` 13/13) — the schema-mismatch item itself is done. But the late-joiner echo defect (P0-2 below) now blocks the *next* thing on the same path. |
| P0-3 fresh-door-artifact genesis for React | **CLOSED, live evidence** | `wp-c10.md` session-12 item 1: `seedColdPairFromCanonicalCheckpoint` hardened + retried, hub accepted a fresh door note's commands (head 0→6); collab-e2e STEP 4 asserts the first outbound envelope names the artifact id. Not independently re-run by this audit against the current binary. |
| P1-1 live peer-cursor proof (writer/draw/puzzle3d, React) | **STILL OPEN** | Every session-12 attempt to reach STEP 14 (peer cursors) was blocked upstream by the guest re-announce defect (writer typing never got past STEP 4/8/11/12/13, `wp-c10.md` "Found 09:4x" onward). Directly downstream of **P0-1** below. |
| P1-2 same-field conflict scenario (live, not just additive) | **STILL OPEN, mechanism ready** | `ui.conflict.hubConcurrentEdit`/`hubConcurrentInvariant` localized notices landed (`wp-c10.md` 23:5x) but no live run staged an actual same-field collision this session either; the Qa opaque-concurrency fix that would let a *vigilant* hub grade it correctly is unapplied (confirmed above). |
| P1-3 live viewer-role proof | **STILL OPEN, not retried** | No session-12 wp report re-ran `c10perm1` against a catalog carrying a real viewer surface. |
| P1-4 cross-shell (React↔wgpu) collaboration | **HALF CLOSED** | WG8's cross-shell run 12 (`wp-wg8.md` 17:47–18:01): **native wgpu ↔ React `s`, 8/8 PASS** — sign-in, create+door, presence both ways, edits both ways, undo both ways, reload-converge, all live on 7800 B2. wasm32↔React and wasm32↔native are still **never attempted** (no wp report describes it). Cursor leg of the SAME native↔React pair is still blocked (block2d has no board; the retry law needs a canvas kind). |
| P1-5 live pg/neo4j growth + restart proofs | **CLOSED** | `wp-h9.md` `tc19`/`tc23`/`tc24`: postgres and neo4j both pass 300+30-edit growth, SIGTERM/SIGKILL restart, agent revocation, execution-target-after-commit, on both backends. One residual: SIGKILL→reopen of a grown pathmap document exceeds the 30 s fixture bound under load (`tc23`/`tc24`, open perf item, H11/DB1). |
| P2-1 warn hub owners: 7800 stack-overflow / 64 KiB ledger CHECK binary | **MOOT** | 7800 has been rebuilt multiple times since (catalog B2, current binary as of session end); the specific stale binary G10 warned about is gone. |
| P2-2 consolidate the two suspend-not-retire link-shortage impls | **CLOSED** | `wp-wg7.md` S12-1b: one kernel region `DocumentLinkShortage` now drives native actor, wasm32 actor **and** the wgpu Shell's `sync_link`; React has its own TS twin of the same policy. Three independent implementations → one kernel + one TS twin. Confirmed present in source (see re-verification above). |
| P2-3 "which plugin kinds can collaborate" still unknown | **MOSTLY CLOSED for MCP/agent side, open for live human co-edit** | G10's plugin-coverage harness (`wp-g10.md` S3, run 4) exercised `artifact_create`/`action_invoke` for all 34 packages headlessly. Live **human** co-edit is still proven only for note (React), block2d (native cross-shell), and note/gismap (React↔React) — everything else remains untested for live two-human collaboration. |
| P2-4 delete dead `checkpoint-publications` route if any caller remains | **CLOSED** | `wp-c10.md`: re-confirmed 0 callers repo-wide. |

---

## 2. Fresh user-journey matrix (session 13, as of now)

| Journey | React↔React | React↔wgpu-native | wgpu-native↔wgpu-native | wgpu-wasm32↔wgpu-wasm32 | any↔wasm32-other | human↔agent |
|---|---|---|---|---|---|---|
| Co-edit, additive (any kind) | LIVE-PROVEN (note, `wp-c10.md` c10mx11) | **LIVE-PROVEN** (block2d, `wp-wg8.md` run 12) | LIVE-PROVEN (`wp-wg8.md` gate run 25) | LIVE-PROVEN (note, `wp-wg7.md` s12c/s12f) | **MISSING**, never attempted | **LIVE-PROVEN** (`wp-g10.md` S1, en+de) |
| Co-edit, TEXT/typing specifically | **BROKEN** — guest re-announce defect (P0-1) rejects the batch after keystroke 2, every actor-relayed document | untested (block2d has no text field) | untested this session | untested this session (note's typing not separately isolated from the block-add flow) | MISSING | untested (agent commits structural ops, not typed text, in these runs) |
| Late join / reopen sees full history | LIVE-PROVEN (`wp-c10.md` 04:5x root fix + law + live) | not exercised | not exercised | **BROKEN** — 0 of N blocks shown (P0-2) | MISSING | LIVE-PROVEN (`wp-g10.md` durability gate, catch-up 3/3) |
| Reconnect after 5–20 s shortage, no freeze | LIVE-PROVEN (no freeze; reconcile has a new open defect, see P1-4) | not isolated in run 12 | LIVE-PROVEN (`wp-wg8.md` step 12) | **LIVE-PROVEN** (`wp-wg7.md` S12-1b, 15 s/20 s cuts) | MISSING | not exercised |
| Long offline → refused, not silent loss | LIVE-PROVEN (`link-expired`) | not isolated | not exercised this session | LIVE-PROVEN (long cut expires, never relinks) | MISSING | not exercised |
| Conflicting same-field edit, visible outcome | Notice mechanism landed, **no live same-field run** | MISSING | MISSING | MISSING | MISSING | MISSING |
| Durable collaborative undo/redo (own edit) | LIVE-PROVEN | LIVE-PROVEN (run 12 step 7) | LIVE-PROVEN (gate step 11) | not isolated this session | MISSING | not exercised |
| Undo/redo of **another peer's** edit | LAW-ONLY, unchanged since s11 | MISSING | MISSING | MISSING | MISSING | MISSING |
| Presence roster | LIVE-PROVEN | LIVE-PROVEN (run 12 step 4) | LIVE-PROVEN | LIVE-PROVEN (`wp-wg7.md` roster parity) | MISSING | LIVE-PROVEN (`wp-g10.md` S1, agent badge en/de) |
| Peer cursors/selections in-canvas | **STILL BLOCKED** by the typing defect (STEP 14 never reached) | BLOCKED — no board on block2d; law exists (`a_native_and_a_react_user_see_each_others_cursor_on_one_hub_board`), never run to green | law-level only (native canvas-presence unit/law proofs, no live run) | not exercised | MISSING | MISSING |
| Viewer read-only role | **BROKEN**, unretried since s11 (`c10perm1`) | UNVERIFIED | UNVERIFIED | UNVERIFIED | MISSING | UNVERIFIED |
| Removed member drops socket | LIVE-PROVEN (document + directory sockets) | UNVERIFIED | UNVERIFIED | UNVERIFIED | MISSING | LIVE-PROVEN protocol-level (`wp-h9.md` law), live delegation-revocation race still open (see P1-3) |
| Agent edit seen live by human | LIVE-PROVEN (`wp-g10.md` S1, en+de, roster badge, canvas update) | not exercised | not exercised | **BLOCKED** by the same late-joiner/echo defect (P0-2) | MISSING | (same cell) |

---

## 3. Protocol/store code paths and twin drift

The three shells share ONE target-neutral guest-linked module, `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`
(compiled into every plugin document actor: native wgpu, wasm32-wasip2 guest, and indirectly the wasm32-browser
actor), plus one target-split sync layer `🏪️store/🔄️sync/🦀️.rs` (native actor, wasm32 browser actor, both target-neutral
`DocumentSocketConnect`/`DocumentLinkShortage` regions), and one independent React TS reimplementation
`🏪️store/👷️worker/🟦️.ts` + `🛍️products/💻️os/🟦️.ts` (the "worker" that plays the same role as the Rust actor, but is a
separate hand-written codebase, not a compiled target of the same source).

Divergence points found this session:

1. **Echo/re-announce suppression** — React's worker: fixed (id-based, landed). Rust kernel (both native and wasm32
   actors read the same `🔄️sync/🦀️.rs`): unfixed. Hub (`🏗️bootstrap/🦀️.rs`): unfixed (still stamps a socket's own
   actor onto its catch-up tail). **Three lanes, one already-designed fix, two-thirds unapplied.**
2. **Outbound re-send guard** — `flush_apply_outbound` (kernel store, shared by every actor-relayed document
   regardless of shell) has no "only send what's new since the last flush" invariant; it re-derives from
   `vcs.edits.last()` every call and only *detects* (does not *prevent*) resending an already-seeded mutation id.
   React's worker has its own, separately-implemented `admitLocalMutations`/outbound path in `🏪️store/👷️worker/🟦️.ts`
   that appears NOT to share this shape (C10's finding was reproduced specifically through the actor/guest lane, not
   the worker's own local-edit path) — i.e. this is a kernel-only defect, not one React independently reimplemented
   and also has, but it means **the kernel's shared code has a live typing-breaking bug that the React-only path
   does not**, which is itself evidence the two implementations have drifted rather than sharing one guarded
   primitive.
3. **Link shortage** — now genuinely unified (`DocumentLinkShortage` kernel region drives native actor, wasm32
   actor, and the wgpu Shell's own `sync_link`; React keeps a separate but faithful TS twin). This is the one
   axis where session 12 successfully removed a three-way drift down to a clean kernel-primitive + TS-twin split.
4. **Presence/ephemeral wire** — was three shapes at the start of session 12 (React had none, native had none,
   wasm32 had none in the browser-lane sense); by session 12's end all three publish `views`/`interaction`/
   presence pack over one conceptual wire, but through three separately-written call sites (React `stampSession`,
   native `ProgramBridge`'s `AppFrame::Ephemeral` cache, wasm32 `browser_ephemeral`). No shared kernel type unifies
   them the way `DocumentLinkShortage` does — a repeat of the audit's earlier P2 "two independent implementations of
   one concept" note, now three.

---

## 4. Ranked gaps

### P0 (3)

- **P0-1 — [UNOWNED, was routed C10→T12, never landed] Guest store re-sends an already-accepted edit on the next
  local commit, breaking text-editing collaboration on every hub-creatable kind.** Confirmed live in current source:
  `flush_apply_outbound` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19169-19195`) re-encodes
  `vcs.edits.last()` on every flush and, when `dag.seed_applied` reports `Duplicate` (line 19180-19184), still sends
  the envelope on the backbone. This is the exact live symptom C10 measured 09:4x (`wp-c10.md`): the second keystroke
  re-carries the first Accepted operation byte-for-byte, the hub rejects the batch as a replay conflict, and the
  document rebootstraps — collab STEPs 4/8/11/12/13 (writer typing) have been red since. Nothing in `wp-t12.md`'s
  session-12 log (0 grep hits for `outbound`/`keystroke`/`flush_apply`) shows this was ever picked up.
  **Acceptance:** two consecutive local edits on an actor-relayed document never resend a seeded mutation id; a
  language-agnostic law over the store's outbound-flush path plus a live typing run (writer or note) reaches STEP 14
  (peer cursors) on React. **Slice: LD (store landing) to fix the kernel primitive; C11 to re-run the collab matrix
  once fixed.**

- **P0-2 — [OWNED by design (WG7), unapplied in 2 of 3 lanes] Echo-suppression-by-identity is landed only in
  React; native and wasm32 shells still show a late joiner/reopener 0 of N existing edits.** WG7 root-caused this
  17:0x (`wp-wg7.md`) and designed the fix on all three lanes, but `git grep` confirms only the React TS side
  (`admitRemoteEnvelopes`/`noteAuthoredEnvelopeIds`) is in the tree; the Rust kernel twin
  (`admit_remote_envelopes`/`note_authored_envelopes`) and the hub twin (`HUB_CATCH_UP_ORIGIN`) are not — both are
  still `wp-wg7/s12-echo-suppression-{kernel,hub}-patch.py`, dry-run-verified only. This directly blocks the human's
  view of an agent's edit on wasm32 (WG7 S12-3: "the human's view of the agent's block is blocked by the
  late-joiner/echo defect") and blocks any reopen/late-join on native or wasm32. **Acceptance:** `s12i-reopen`-style
  run shows the full existing history (not 0) on a fresh attach, on native wgpu and wasm32 wgpu, matching React's
  already-fixed behaviour. **Slice: LD (kernel `🔄️sync/🦀️.rs` region) + H11 (hub bootstrap origin stamp) — land as
  one set (they were designed together); WG9/WG10 re-verify live after landing.**

- **P0-3 — [ALL LANDING SLICES, coordinator-owned] The session-13 landing window is open but empty.**
  `.tmp-ticket/📓️landing.md`'s "Session 13 Landing Window" table has a header and zero rows; every prepared
  session-12 patch set (P0-1's peer fixes aside — this is the broader set: WG8's transport-deadline patch, H9's
  opaque-concurrency Qa store-half + per-kind-label patch + creation-progress patch, T12's stdio-kind-id
  post-publish set) is sitting unapplied while hub 7800 is live on the pre-landing binary right now (confirmed
  listening on 7800, pid 54029). Every "fixed in source, not yet live" item in this report and in the wp reports it
  is built on stays that way until this window closes. **Acceptance:** every prepared set in the preamble's list
  lands compile-atomic per rule 2, each with a `📓️landing.md` row. **Slice: LA/LB/LC/LD/LE per the preamble's landing
  sequence; this is the single highest-leverage item in the whole report.**

### P1 (5)

- **P1-1 — [UNOWNED] Same-field concurrent-write correctness on a vigilant hub is still unmeasurable.** H9's Qa
  design (stamp a mutation's causal head at authoring time in the store; grade an opaque write against the
  document's frontier head in the db) is fully designed and dry-run-verified (`wp-h9.md` 17:0x) but `causal_head` is
  not in the tree (0 `git grep` hits) — a vigilant hub still cannot tell an aware concurrent write from a blind one
  for any plugin whose diffs are opaque to the db (i.e. almost every plugin). **Slice: LD (store half) + H11 (db
  half) — the two halves must land together (H9's own note: landing the db half alone would make a vigilant hub
  refuse every write from today's clients).**

- **P1-2 — [OWNED, C11] Live same-field conflict scenario, live viewer-role proof, live peer-cursor proof —
  three carried-over session-11/12 items, still open.** All three need the same precondition (P0-1 fixed, so typing
  reaches a hub document) plus, for the conflict scenario, P1-1. **Slice: C11**, once P0-1 lands.

- **P1-3 — [OWNED, H9→H11] A revoked agent delegation's already-admitted edit can still land after the
  revocation in one ordering.** H9 found and designed the fix 16:5x (`wp-h9.md`: hold every revoked session's
  binding exclusively, bounded by the frame deadline, before invalidating grants) but it is "written, compile
  after publish 4" — not yet compiled or run. **Acceptance:** `a_withdrawn_delegation_admits_no_agent_edit_after_it_in_either_order`
  compiles and passes on the post-landing binary; G10's S4 en/de asymmetry (one locale refused a post-revocation
  edit, the other still applied it) does not reproduce. **Slice: H11.**

- **P1-4 — [reassigned from s12 P0-1, UNOWNED] The short-shortage reconcile has a second, unlocated root cause on
  React.** H9's DB-side fix is confirmed landed and no longer reproduces the original `DB I/O aggregate admission
  exhausted` error, but C10's later outage runs (`wp-c10.md` 10:0x–10:1x) found a *different* stall: an offline
  edit staged during a cut is not submitted to the worker until ~5 s after reconnect, so it always lands after
  reconnection rather than during the cut as the actor itself does. Never located this session (rule-20 freeze,
  then usage cuts). **Acceptance:** the outage probe's edit-vs-cut-window timestamp assertion passes without the
  "stamps every edit relative to the cut" workaround the probe currently needs. **Slice: C11.**

- **P1-5 — [UNOWNED] Cross-shell wasm32↔React and wasm32↔native have never been attempted; the one proven
  cross-shell pair (native↔React) still has no live cursor leg.** WG8 closed native↔React for edits/presence/undo
  (`wp-wg8.md` run 12, 8/8) but its cursor law needs a canvas-kind document (block2d has none) and was never run to
  green; wasm32 was never paired with either other shell in a live two-human run. **Slice: WG9 + WG10 + C11**
  (whichever pairing the coordinator prioritises next); recommend native↔React cursors first since the law already
  exists (`a_native_and_a_react_user_see_each_others_cursor_on_one_hub_board`).

### P2 (3)

- **P2-1 — [OWNED, H11] Per-kind localized labels (en/de) are designed and dry-run-verified but unapplied**
  (`ArtifactKindSpec` still has no `label` field, confirmed above) — every hub-creatable-kind picker still shows the
  hub's generic "Editor" label (S15 finding c), which is why C10's harness had to switch off label-based kind
  selection this session. Cosmetic for the collaboration outcome itself but blocks any test harness that still
  keys off labels.
- **P2-2 — [UNOWNED] Presence/ephemeral wire is three separately-written call sites, not one kernel primitive** —
  same shape as the now-resolved link-shortage drift (§3.4); worth doing next given link-shortage's consolidation
  already set the pattern.
- **P2-3 — [UNOWNED] Undo/redo of another peer's edit remains untested (law-only since session 11)** — every live
  proof this session and last is still "each peer undoes only their own edit"; no run has one human undo a
  document to a state that erases a DIFFERENT peer's still-pending edit.

---

## 5. What this audit could not verify

- No build or server was started; every "landed in source" claim above is a direct read of the current file, but
  no cargo/nx/bun check was run to confirm it *compiles* — several of the "prepared" patches this report and the wp
  reports describe are explicitly not compile-verified pending the landing window.
- `wp-t12.md` was read for its Status table and roughly the first half of its session-12 log (item S15 guest
  defects, editor/kind declarations, contract/inventory work); the back half (S15 guest defects §S, T10/T11
  follow-ups, remodeling laws, pdf kinds — lines ~423–716) was not read, since targeted greps across the whole file
  found no mention of the C10-routed re-announce defect anywhere, which was this audit's main reason to read T12 at
  all. If T12 picked up unrelated collaboration-adjacent work in that unread portion, it is not reflected here.
- The rest of `wp-wg7.md`'s log (lines ~400–733, session-11 K1–K7/R1–R4 landing detail, already summarized in its
  own Status table and in the s12 audit) was not re-read; nothing in the Status table suggested session-12 content
  lived past line ~400.
- Live process state (hub 7800 / 8040 / 8050 listening) is a snapshot at audit time; other session-13 agents are
  actively working concurrently and may change source, land patches, or restart hubs during or immediately after
  this audit.
