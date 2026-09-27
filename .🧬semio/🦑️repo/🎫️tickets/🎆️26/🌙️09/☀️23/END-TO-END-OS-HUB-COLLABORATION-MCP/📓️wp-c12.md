# WP-C12 — Collaboration Between Users Over The Hub, React Shell

Session 14 slice C12 (successor of C11: handover [📓️wp-c11.md](📓️wp-c11.md), [📓️wp-ld.md](📓️wp-ld.md),
[📓️audit-s13-collaboration.md](📓️audit-s13-collaboration.md), [📓️acceptance-s13.md](📓️acceptance-s13.md) §3). Ports: hubs 8020–8029,
serves 6520–6529 (C11's hub 8022 inherited). Scripts `wp-c12/`; expendable captures `wp-c12/generated/`; durable data and logs
`.🧬semio/🌐hub/s14-c12-*`. Rules: `📓️session-14-preamble.md` (+ 13/12).

## Session 14

| # | Item | Status |
|---|---|---|
| 1 | React collab-e2e on hub 7800 (B3; ALL after W4 moves it): every step PASS or root cause | baseline `c12collab-1` **7/14** (1,2,3,4,5,7,12 PASS; 9/10 SKIP external). 6 = hub check-in `codec-refused` → H13; 8 = harness typed into the wrong textarea (fixed); 11/13 = whole-text SET (item 2); 14 = writer + draw PASS, puzzle3d opening `hydration was rejected: Identity` (open) |
| 2 | Writer concurrent typing: range-text operations design + prepared guest patch + laws (window 3) | design below; TS twin laws **33/33** (`wp-c12/splice/`, jsdiff oracle + 400 seeded workloads); Rust twin law queued (native lane); prepared set `wp-c12/splice/patch/c12-splice-patch.py` dry run **84 planned / 0 problems** (host splice mode, oracle catalog vectors, python second implementation, fixture quintet still to write) |
| 3 | Dialog closed by the post-sign-in Home re-bootstrap (S16 → S18; collaboration flow) | **S18 owns the product fix** (coordinator 18:4x); C12 keeps the harness wait; evidence relayed (socket `since` = preference cursor) |
| 4 | Connection shortage 5 s / 15 s / 60 s during typing on both clients: no freeze, convergence, en + de link state | run `c12short3` (7800/B3, A en → proxy 8023, B de → proxy 8024): **no freeze** 3/3 (worst frame 17–101 ms), **link state localized** 3/3 (en `reconnecting…`/`live`, de `verbinde erneut…`/`verbunden`), 60 s cut relinked at the bound (no expiry); **RED: every keystroke typed during the cut is lost** — the reconnect flush is ONE batch, 7800 refuses it `unavailable: DB I/O aggregate admission exhausted`, the worker rolls back + rebootstraps (hub head 24→24 over the 15 s and 60 s rounds) → H13 (hub admission) + C12 client half (resend) |
| 5 | Late joiner + presence (cursors/selection) for note, writer, draw, puzzle (React) | pending |
| 6 | React collaboration acceptance as a permanent harness (📜️script.ts verb + nx target + generated launch row; R10) | pending |

## Item 2 — Concurrent Typing: Range-Text Operations (design, window 3)

**Defect (measured, C11 `c11typing2`, C12 `c12collab-1` STEPs 11/13).** The host's typing verb `textEdit {text}` ships the WHOLE
text; writer maps it to `EditText {text}`, a whole-body SET (`target = ["text"]`). Two humans typing at once: every replica and the
hub fold SETs in their order, so the last SET wins and the other human's keystrokes vanish (A lost 3, B lost 7 chars; STEP 13 kept
one marker only). Sequential typing within one relay latency loses too (STEP 11: user2's run lost after its first key). Per-user
undo cannot work either: EditText's inverse restores the whole old body, erasing the peer's text typed after it.

**Design (event-sourced CQRS, no CRDT: operations are events in the hub's total order; no per-character ids, no tombstones).**

1. **One range-text operation, schema-first, language-neutral** — `semio.ui.scene.text-splice.v1`:
   `TextSplice { start, deleted, insert, before, after }`, positions/lengths in Unicode scalar values; `before`/`after` = up to 32
   scalars the author saw around the replaced run. Twins: Rust `semio-framework-ui-scene::text_splice` (guest-linked, wasm-safe
   crate every text host and guest already links) and TS `🎬️scene/✂️text-splice/🟦️.ts`; one JSON schema + one fixture
   (`🧬️schema/✂️text-splice`, `🧫️fixtures/✂️text-splice`).
2. **Stable positions = context anchoring.** `locate(text, splice)` is deterministic: the run with two-sided context
   (k = 32 → 4 scalars per side), then one-sided context (k = 32 → 1; both sides compete), then the run alone; every step takes the
   match nearest the author's `start` (ties: lowest). A run that is GONE deletes nothing (`clamped`, reported as the outcome
   warning `mutation.clamped`) — a deletion never destroys text its author did not see; its insert still lands. `apply` answers
   the new text and the exact inverse splice (the removed run back, context of the new text) — per-author undo relocates the
   same way, so undoing MY run never removes YOUR run typed next to it (STEP 11).
3. **Rebase on the actor/store path = the store's existing replay, no new kernel code.** `ArtifactStore::ingest_remote` already
   HLC-merges remote edits, rewinds to the fork point and replays the divergent suffix (`replay_suffix_partitioned`): replaying a
   `SpliceText` re-runs its `diff` against the new base, i.e. relocates it — that IS the rebase. The hub applies the same leaf
   through the same codec in commit order. The React worker's backstop (`reorder` → `requireArtifactRebootstrap` when a remote op
   folded over pending local ones) keeps convergence when the actor's HLC order and the hub's commit order disagree; with splices
   the re-seeded pair already holds both authors' text, so the rebuild no longer loses characters (follow-up, perf: rebuild only
   when the orders actually differ — measure the rebuild rate under two-human typing after the landing).
4. **Writer** — new direct leaf `✂️splice-text` (`SpliceText`, binary tag 4, text opcode `splice-text`, target `["text"]`,
   `diff` = relocate + `diff_set_text(new body)`, `inverse` = the located inverse splice, label en "Edit a text range" / de
   "Textbereich bearbeiten") + fixture quintet; new command `textSplice` (`start, deleted, insert, before, after, seq, anchor,
   caret`) → `Emit::amend([SpliceText], "writer-text-edit")` (same coalescing key: a typing run stays one undo step; LD's
   `announce_from` sends only the appended ops) PLUS the window-transient `SetEditorSelection {start: anchor, end: caret,
   splice: seq}` in the same turn; `textSelect` carries the host's current `splice` too. `WriterEditorSelection` gains
   `splice: u64` (transient only, never ledgered). `textEdit {text}` stays the whole-text verb (agents, `setText`).
5. **Scene contract** — the writer text scene's `settingsJson` gains `typing: {mode: "splice"}` (schema
   `TextEditorTypingV1` in the same schema file) and its `selectionJson` echoes `splice` = the last host splice the actor applied
   for THIS window. Kinds without a range leaf keep `typing` absent → the host types with `textEdit` as today (capability, not a
   fallback: vcs/jack/stdio text kinds are whole-text kinds until each gets its own range leaf — follow-ups per owner).
6. **Host (React `TextEditor`, splice mode)** — the Jupiter client model: every flushed run is ONE splice
   `fromEdit(lastSent, local)` with a monotonically increasing `seq` (the coalescing dispatcher keeps one delivery in flight);
   unacknowledged splices are kept in order; a scene with `selection.splice = s` drops every splice ≤ s and the editor shows
   `rebase(remote, unacknowledged + undelivered, local, selection)` (fold the unapplied splices onto the guest's text; caret and
   anchor travel as zero-width marker splices). No text-equality echo matching in splice mode. Selection offsets stay UTF-8
   bytes on the wire (session unit), scalars inside the splice.
7. **Laws** — (a) fixture laws, TS + Rust replay the SAME vectors (edits, applications + inverses, concurrent folds in hub order,
   host rebases, UTF-8 ↔ scalar); (b) third-party oracle: jsdiff `diffChars` derives the same changed run for every edit;
   (c) property: 400 seeded two-author insert-only workloads lose no scalar; (d) two-actor interleaving law (writer, Rust): two
   registered writer instances on a `MemoryBackbone` type concurrent runs (same point + different points), exchange in both
   orders → equal text, both runs present and contiguous, the hub-order fold (TS reference) predicts the text; per-author undo
   removes only the undoer's run on both replicas; (e) host law (TS, mounted): two simulated hosts + a simulated hub order →
   both editors converge, no keystroke lost, caret stays at the end of the own run.
8. **Vigilant hubs** — `SpliceText` declares `target = ["text"]`; LD's grading treats concurrent same-target writes as conflicts
   on a vigilant hub (refused). Splices are self-rebasing, so the follow-up is a leaf-declared `commutes_with_same_target`
   (or a range-precise target) — owner LD lineage/H13; normal-policy hubs (7800) accept and report.

### Infra (pids I started or inherited)

| What | pid | Port | Notes |
|---|---|---|---|
| (inherited from C11) serves `s` dev → 7800 | 8832 / 8838 (vite 8852 / 8846) | 6523 / 6524 | logs `s13-c11-logs/serve-652{3,4}-b3.txt` |
| (inherited from C11) vigilant hub 8022 (B3 clone) | hold 45592 / hub 45599 | 8022 | root `s13-c11-hub-8022-vigilant` |
| (inherited from C11) serves `s` dev → 8022 | 61785 / 61791 (vite 61827 / 61829) | 6525 / 6526 | logs `s13-c11-logs/serve-652{5,6}-8022.txt` |

### Log (session 14)

- 18:23 read preambles 14/13/12, AGENTS.md, `📓️fleet-14-agents.md` (no "CHAIN LAUNCHED" yet → guest freeze not started),
  `📓️wp-c11.md`, `📓️wp-ld.md`, audit s13 collaboration, acceptance s13 §3, fleet-13 log from 14:00. Reconcile: C11's tree edits
  (TextEditor one-dispatcher fix, collaboration harness, worker, ShellHost presence beat, canvas/text presence overlays) are all
  in the tree (staged since the 11:30 auto-commit), 0 `[DEBUG] c11` lines; C11's in-flight window-3 work = ticket-local draft
  `wp-c11/splice/` (context-anchored splice reference + fixture draft + bun law with jsdiff), no tree edit. Live: 7800 READY on B3
  (runId `fd90596d…`), C11's hub 8022 + serves 6523–6526 alive (all C11's pids, now mine). Load 9, swap 7.5/9 GiB, 141 GiB free.
- 18:3x stopped C11's vigilant hub 8022 (hub 45599 → hold 45592 exited) and serves 6525/6526 (pgids 61785/61791) and 6523/6524 (8832/8838): not
  needed this hour (memory, rule 6). Ports 8022/6523–6526 free.
- 18:37–18:53 **collab-e2e baseline `c12collab-1`** on 7800/B3 (`zsh wp-c12/run-collab.sh c12collab-1 http://127.0.0.1:7800 <7800 admin file>`,
  log `.🧬semio/🌐hub/s14-c12-logs/c12collab-1-run.txt`): **7/14 PASS** (1, 2, 3, 4, 5, 7, 12), 9/10 SKIP (external hub), FAIL 6, 8, 11, 13, 14.
  STEP 12 (4 s offline, local echo 843 ms, offline edit reaches user1) PASS for the first time. STEP 14: writer caret PASS (x 74→396),
  **draw peer cursor PASS** ((432,301)→(1003,624)), puzzle3d FAIL — the creator's open failed: `AppChannelClient.loadDocumentArchive
  (s.puzzle.puzzle3d@1/*#editor): document archive parent Pack and SPR hydration was rejected: Identity` (run log l.2435).
- 18:4x **harness leaked credentials into run logs**: `attachBrowserDiagnostics` printed every `/auth/` POST body (the sign-in password) and
  the full bearer capability. Fixed in the harness (auth scheme only; `/auth/` bodies `<redacted>`). Coordinator: scrub existing logs →
  `wp-c12/scrub-credentials.py --write` over `.🧬semio/🌐hub/s13-c11-logs/`: **1048 occurrences in 5 files scrubbed in place**
  (`c11collab-1-run.txt` 420, `c11collab-b3-1-run.txt` 233, `-b3-2` 76, `-b3-3` 59, `-b3-4` 260); re-scan 0; `wp-c11/generated` had none.
  (`c12collab-1-run.txt` ran before the fix → scrubbed below.) Probe scripts take credentials from env only (`wp-c12/env.sh`).
- 18:4x item 3: coordinator — S18 takes the product fix; relayed C11's evidence (the socket opened with `since=132` = the preference-page
  cursor while events ended at 82, closed at once → `event-page?after=0` re-bootstrap).
- 18:5x **STEP 8 root cause = harness**: probe `wp-c12/probe-c12-reopen.mjs` (`c12reopen1`, 6523/6524 → 7800; out `wp-c12/generated/
  c12reopen1-out.txt`): rounds base / re-open from the Space index / refused Check In / base → **4/4 relayed** (settled 135–275 ms, hub head
  3→17). The e2e typed into `page.locator('textarea, [contenteditable="true"]').first()`, which after STEP 6's reload is not the editor
  (the History panel's check-in field comes first). Fix: `collabTextEditor(page)` = `.semio-text-editor-host textarea` for STEPs 4, 8,
  10–14. Observed on the way: re-opening an ALREADY open document from the Space index opens a SECOND document socket for the same
  document in the same page (both stay open) — follow-up in item 5.
- 18:5x STEP 6 routed to H13 (relay): writer Check In `codec-refused` ("Check-in refused: the document could not be rebuilt"), reproduced
  in `c12reopen1` round 3 (artifact `artifact-0d45de16…`, head 14); 7800's hold `capture.txt` keeps only the FIRST 1 MB of hub output, so
  no span of today's refusals is on disk.
- 19:0x os `tsc --noEmit -p tsconfig.json` (`wp-c12/generated/tsc-os-1.txt`): **1 error, a peer's** (`📏️layout …/🔺️diff/🟦️.ts(81,3)`
  TS2739), 0 in the collaboration harness.
- 19:1x item 4 probe `wp-c12/probe-c12-shortage.mjs` written: A (en-US) → proxy 8023 (control 8025) → 7800, B (de-DE) → proxy 8024
  (control 8026) → 7800, serves 6525 → 8023 and 6526 → 8024; both links cut at once for 5 s / 15 s / 60 s while both type; frame
  (rAF) + DOM-read samples every 250 ms, hub badge + pill + notices per language, convergence, markers kept, hub head.
- 19:0x–19:3x live runs blocked twice by peers' host TS (F3 `TEXT_EDITOR_SCENE_LANES` module-cycle ReferenceError 19:14–19:27;
  S18 `NamedLayoutStore` export gap ~19:30) → relayed, both fixed; `c12short1`/`c12short2` = boot failures (not product results).
- 19:1x–19:4x **item 2 laws (TS twin)**: `wp-c12/splice/{text-splice.ts,schema.json,fixture.json,make-fixture.ts,text-splice.test.ts}`;
  `bun test ./text-splice.test.ts` → **33 pass / 0 fail, 1393 expects** (`wp-c12/generated/splice-law-ts-1.txt`): Ajv admits the fixture;
  8 edit vectors (jsdiff `diffChars` derives the same changed run for every one), 8 applications + inverses, 10 concurrent folds in
  hub order (same point, different points, double delete clamped, delete around a concurrent insert clamped, astral scalars, sync
  mid-run, typo corrected while the other types at the same point, replace a word the other changed, repeated context), 4 host
  rebases, UTF-8 ↔ scalar offsets, and 400 seeded two-author insert-only workloads with no scalar lost. Rust twin
  `wp-c12/splice/text_splice.rs` + law `text_splice_law.rs` in the standalone crate `wp-c12/splice/rs-check/` (no repo deps, private
  target + build dir), queued in the native lane (`zsh wp-c12/splice/rs-check.sh`, capture `wp-c12/generated/splice-law-rs-1.txt`).
- 19:4x–20:0x **item 4 run `c12short3`** (`bun wp-c12/probe-c12-shortage.mjs c12short3 http://127.0.0.1:6525 http://127.0.0.1:6526
  http://127.0.0.1:8025 http://127.0.0.1:8026 <space> 5000,15000,60000`, out/console/report `wp-c12/generated/c12short3-*`):
  - no freeze: worst rAF 17/43 ms (5 s), 91/101 ms (15 s), 50/60 ms (60 s); worst DOM read 0.9 / 1.2 / 0.7 s.
  - link state: A (en) badge `Hub connection: reconnecting…` → `live · 2 peers`, B (de) `Hub-Verbindung: verbinde erneut…` →
    `verbunden · 2 Mitwirkende`; the sync pill still says `Remote: backoff` (en) — jargon, not wrong language (noted for S18).
  - 60 s cut: the link retried at the policy bound (60 s) and relinked — no `link-expired` (bound 60 s, ceiling 90 s).
  - **RED — typing during a cut is lost on both clients.** After each restore the worker flushes the whole outbox as ONE
    `Commands` batch; 7800 answers `Rejected {reason: "unavailable: DB I/O aggregate admission exhausted", messages: []}` (4 times:
    both humans, 15 s + 60 s rounds, and B in the 5 s round) → the worker's correction path rolls back and rebootstraps ("The
    server requires a fresh authoritative restore. Stale document UI was discarded…" en/de) → every marker typed during the cut is
    gone from both editors and the hub (head 24 → 24 across the 15 s and 60 s rounds; 5 s round: only A's markers survived).
    Root cause = db per-operation IO credit (`🛢️db/🗄️storage` `db_io_operation_add`: 64 pages / 16 controls) refusing a batch the
    wire declares legal (`DOCUMENT_BACKBONE_BATCH_MAXIMUM_*` = 8192 envelopes / 256 KiB) + the client discarding a transient refusal.
    Relayed to H13 (hub admits any declared-legal batch in bounded steps; transient refusals machine-readable, proposed code
    `hub.unavailable`); client half (resend on a transient refusal, never discard) = C12, waiting for the code.
  - probe defect found: "convergence" also matched two EMPTY editors (rebuild in progress) — fixed in the permanent harness version.
- 19:1x H13 root-caused STEP 6 (guest `print_mirror` dropped a populated pair → vcs Drop panic → `codec-refused`), landed 19:15
  (guest-linked); STEP 6 re-run needs 7800 on ALL or a restaged guest.
