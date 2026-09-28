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

### Session 14b

Successor agent (2026-09-28 12:1x, after the usage cut + app restart). Rules: `📓️session-14-preamble.md` incl. "Session 14b".

| # | Item | Status |
|---|---|---|
| 0 | Reconcile predecessor's in-flight worker edit (`hub.unavailable` resend) | **found complete in HEAD** (`5bcb2da`, 20:36): state field, init, cleanup, bounded drain, transient resend + law; TS twin `hubTransientApplyRefusalV1` + law in `💻️os/🟦️.ts`. **Re-worked (bug found):** the drain still pipelined live batches, so a transiently refused batch could be overtaken by a later one (the hub refuses that one for good: `dependency names an unknown edit`), and a dropped socket re-queued its in-flight batch BEHIND the queue. Now ack-clocked (one `Commands` batch in flight), bounded by 16 envelopes AND half the declared batch bytes (128 KiB, as Rust `announce_history`), in-flight batches return to the FRONT. tsc 0 own errors, boot to Home 0 pageerrors (×5). Serve boots: the ShellHost `documentProgramFocused` break is gone (12:14 boot OK) |
| 1 | Client half of the P1 data-loss fix + live proof (`probe-c12-shortage.mjs`) | **code landed, live proof BLOCKED**: runs `c12short4/5` (7800/B3, A types alone): **no rebuild** in every round, no freeze (worst frame 50 ms) — the whole-outbox DB refusal is gone — but the markers never reached the hub: this exposed a **hidden worker bug** (1b, fixed). Since G12's channel 18→19 bump (12:58) no document opens against 7800/B3 (`document-open.unsupported-execution-protocol`, run `c12short6`) → re-run when 7800 is on ALL (coordinator: ~2–3 h) |
| 1b | NEW — reconnect after a 5 s cut leaves the document "verifying" forever, every edit refused `owner-mismatch`, 24 edits stuck `Pending` | **root-caused + fixed (TS; worker laws written, run queued in the native lane)**: (a) the resume check demanded the suspended lease's `revalidation.directoryRevision` verbatim, but the hub fills it with the GLOBAL directory head (`🌎️hub/🏗️bootstrap/🦀️.rs:3505`), which any directory event on the shared hub moves → every resume refused → full reopen; (b) the reopen dropped lease + pair but kept `currentPack`/`frontier`/resume token → `seedColdPairFromCanonicalCheckpoint` skipped (pack present) → the new child was never seeded (hidden until today because the DB refusal forced a full rebuild, which cleared the pack). Fix: `sameExecutionTargetV1` (📇️directory schema TS; checkpoint + revalidation adopted forward-only, every other field full-field) + `DocumentExecutionTargetLease.refresh`; a dropped lease forgets the mounted document (`forgetMountedDocument`: pack, spr, frontier, resume token, ingested ids — unacknowledged ids stay noted); a mounted child replaced while the document stays open tells the Shell (`retireMountedDocumentChild` → `artifact-rebootstrap-required`). Fixture `🧫️fixtures/📇️directory/⏯️execution-target-resume-v1.json` (15 cases) + fast-deep-equal oracle. Rust twin of `sameExecutionTargetV1` (`📇️directory/🧬️schema/🦀️.rs`, frozen) = prepared patch — pending |
| 1c | NEW — TS worker lacked the Rust `settle_committed_envelopes` (parity scenario `lost-ack-committed-op-settles`, added 27 21:54, red on the TS twin): a committed op whose Ack died with the socket was re-sent re-stamped → the hub refuses it as a replay → rollback + rebuild = another keystroke-loss path after a cut | **fixed (TS)**: `settleCommittedEnvelopes` in the `Commands` branch (outbox + pending batches, retention released, pendingMutations trimmed, drain continues) (`wp-c12/c12-settle-edit.py`); tsc 0 own errors (`tsc-os-14b-7.txt`), boot OK (`boot-14b-6.txt`); worker vitest re-queued (`vitest-worker-14b-2.txt`) |
| 2 | collab-e2e on 7800 | **blocked** (channel 19 vs B3, as above); S18's post-sign-in fix is live-proven → **8 s quiet wait dropped** from the harness (`🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts`) |
| 3 | Writer concurrent typing (window 3) | predecessor's Rust twin law **6/6 PASS** (`generated/splice-law-rs-1.txt`, 27 21:11); prepared patch `wp-c12/splice/patch/c12-splice-patch.py` dry run on the live tree (after the overnight peer edits) **84 planned / 0 problems** (13:2x); still to write before window 3: host splice mode, oracle catalog vectors, python second implementation, fixture quintet |
| 4 | Late joiner + presence (note/writer/draw/puzzle) | predecessor run `c12late1` (27 20:12–20:30, recorded now): writer late-join + roster + selection + B→A edit PASS, caret-move FAIL; draw late-join + roster + pointer PASS, selection + B→A edit FAIL; puzzle3d render error `no registered board session factory` (edits not applied); puzzle2d no artifact row in 240 s; note late-join + roster PASS, journey FAIL (boundingBox timeout). Re-run after 7800 is on ALL |
| 5 | Harness per rule 17 | existing `verify collab` (os-dev collab-e2e) extended, no new directory: each shell reaches the hub through its own in-process TCP relay; NEW STEP 15 = real link cuts 5/15/60 s (`S_COLLAB_LINK_CUTS_MS`) while user1 types — no freeze (rAF < 1 s), no rebuild notice, every marker in both shells (`wp-c12/c12-harness-shortage-edit.py`); tsc 0 own errors (`tsc-os-14b-5.txt`); not run live (7800 blocked). R10 registered `react-collaboration-e2e-en|de` → `verify collab [--hub <url>] [--locale en|de]` now runs under `withAcceptanceRecord` and publishes an en + de record (pass = all 15 steps recorded, none FAIL; `blocked` for an unreachable hub / missing credentials), both browser contexts in the locale, STEP 1/2 option labels from the Space app's own en/de labels (`wp-c12/c12-harness-acceptance-edit.py`); tsc 0 own errors (`tsc-os-14b-6.txt`); blocked path exercised: `verify collab --hub http://127.0.0.1:1 --locale de` without credentials → schema-valid `blocked` record (`generated/collab-acceptance-blocked.json`) |

**Session 14b log**

- 12:1x read preamble (14b), AGENTS.md, fleet tail, this report. The predecessor's worker edit = HEAD `5bcb2da` (no working-tree diff). C13's `[DEBUG] c13 …` lines in the worker are C13's (left alone).
- 12:14 serve `s` react dev 6520 → 7800 (`wp-c12/serve.sh`, pgid 52381): boot to Home 17 s, 0 pageerrors (`generated/boot-14b-1.txt`).
- 12:2x worker drain rewrite (`wp-c12/c12-outbox-edit.py`) + law rewrite (`wp-c12/c12-outbox-law-edit.py`: ack-clocked, fake-timer backoff 187.5/375 ms, byte bound with 3 × 100 kB edits, socket drop re-queues AHEAD). tsc os (`generated/tsc-os-14b-1.txt`): 85 errors, 0 in touched files (peers': `📦️packages/🦀️rust/📜️script.ts` ×53, engine tests, layout diff). Boot OK (`boot-14b-2.txt`). Worker vitest queued in the native lane (`generated/vitest-worker-14b-1.txt`, pid 59539).
- 12:3x run `c12short4` (proxies 8023/8024 → 7800, serves 6520/6521, A en / B de, A types alone, cuts 5/15/60 s): no freeze, en/de link state, **no rebuild** in any round; the markers never reached the hub (head 2→3), pill `Pending (24)`, `commitCheckpoint`/`textEdit refused: owner-mismatch` after the first reconnect.
- 12:4x temporary `[DEBUG] c12` worker probes (`wp-c12/c12-debug-edit.py`) + worker console capture in the probe: `c12short5` → at the cut `suspendLink` OK; after the restore the resume's plan authority failed (`sameProjected:false`), fresh lease, `activate {suspended:false, pair:false, pack:true, outbox:25}`, `seed {pack:true}` → no `active-checkpoint/pair` GET → the child was never seeded. Debug removed (0 `[DEBUG] c12` left).
- 12:5x `c12short6` failed at the first open: `plan-parse document-open.unsupported-execution-protocol` = G12's channel 19 vs B3's 18 → relayed to main (confirmed expected; live runs wait for 7800 on ALL).
- 13:0x fixes 1b (a) + (b) (`wp-c12/c12-reseed-edit.py`, `c12-reseed-law-edit.py`), resume fixture + law. tsc os (`tsc-os-14b-4.txt`): 84 errors, 0 in touched files. Boot OK (`boot-14b-5.txt`). Serves 6520/6521 + proxies 8023/8024 stopped (idle until 7800 is on ALL).
- 14:16 worker vitest (native lane, `generated/vitest-worker-14b-1.txt`, whole worker file): **132 pass / 2 fail** — the drain/transient law and the retire-and-reseed law PASS; FAIL (a) resume law: fixture case "a checkpoint of another descriptor" is no valid lease (`stale-checkpoint` at parse) → case removed (14 cases); FAIL (b) parity `lost-ack-committed-op-settles` → TS twin of `settle_committed_envelopes` was missing (item 1c) → added. Re-run queued.

### Session 14c

Successor agent (2026-09-28 17:0x, after the 14:37 usage cut + app restart). Rules: `📓️session-14-preamble.md` incl. "Session 14c".

| # | Item | Status |
|---|---|---|
| 0a | Reconcile: collab harness acceptance wrapper + `--hub/--locale` | **found complete in HEAD** (`dfe2687`, 16:29): `runCollabE2eCli` publishes `react-collaboration-e2e-${locale}` = exactly R10's registered ids (`🎯️acceptance/🎚️config/🔣️.json` l.78–79, target `collab-e2e` → `verify collab`, args forwarded); no working-tree diff |
| 0b | Reconcile: worker laws (bounded drain, `hub.unavailable` resend, cold-pair reseed, settle) | **134/134 GREEN** (`wp-c12/generated/vitest-worker-14c-2.txt`, native lane, 17:22). Red 1 (u64 Commands law, `state.outbox` undefined) = the law's hand-built state predated `settleCommittedEnvelopes` → NEW single constructor `newArtifactState` (worker), used by `openArtifact`, that law AND the parity `seedState` (its 52-line literal had already drifted: 5 fields missing) (`wp-c12/c12-state-ctor-edit.py`). Red 2 (resume law, `invalid-surface`) = 2 fixture cases no lease can express (write grant on a viewer surface; another component without its closed browser actor's source digest) → "the editor role granted since" + browser actor follows (`wp-c12/c12-resume-fixture-edit.py`; every case checked against the live parser + relation, `c12-resume-fixture-check.ts`). Run 1 (17:03, before the ctor): 133/134, the only red = retention-bytes law timed out under load 65 (11.5 s vs 5 s; passes at normal load: run 2). tsc os 84 errors, all peers', 0 in touched files (`tsc-os-14c-1.txt`). Boot `serve s react dev` 6520 via S18's `ensureDevServe` (`wp-c12/c12-boot.ts`) → **Home in 16.3 s, 0 pageerrors** (`boot-14c-2.txt`; attempt 1 died on a peer's half-landed `hub-document-sweep` edit — duplicate `edits` — which the peer fixed at 17:23) |
| 0c | C13 relay: explicit per-test timeout for the retention-bytes law | `}, 30_000)` on that one law (1 MiB of messages; 11.5 s at load 65), no global loosening; worker laws **134/134** (`vitest-worker-14c-3.txt`, 17:27) |
| 1 | Writer concurrent typing — prepared window-3 set completed | `wp-c12/splice/patch/c12-splice-patch.py` dry run on the live tree **111 planned / 0 problems** (17:4x; was 83). Added: (a) **host splice mode** — TS twin host state machine (`textEditorSpliceHostV1`, `sendTextEditorSpliceV1`, `receiveTextEditorSceneV1`, `refuseTextEditorSpliceV1`: Jupiter client model, refused run disappears, caret collapses where it was) + 5 React `✏️TextEditor` hunks (numbered `textSplice` per run, `textSelect` carries `splice`, scenes rebase unapplied + unsent typing, pack without the echoed buffer); (b) fixture `hostTyping` (6 vectors, schema-validated) + **two-host/two-replica simulation law** (300 seeded sessions: host→replica lag, hub order, per-host applied seq; every typed scalar always visible to its author, both converge on the hub-order fold) — TS twin laws **41/41** (`bun test`, tree), mutant "ignore unapplied" caught by 4 laws; (c) **conformance**: splice-text fixture quintet `⚠️warns-that-an-already-removed-run-leaves-the-brief-unchanged` + its Rust leaf test, oracle catalog vector + manifest entry, `mutate-writer-1` Rust subject (KINDS, guard vector, handle check), feature rows, **Python second implementation** of locate/apply — agrees with the TS reference on all 22 fixture vectors (`py-splice-differential.py`) and on **2 703 seeded random vectors (331 clamped), 0 disagreements** (`random-vectors.ts`). Not typecheckable before window 3 (imports scene exports the set adds; scene TS stays frozen with the browser bundle) |
| 2 | Live runs at 7800-on-ALL (STEP 6 check-in, STEP 15 link cuts, late join) | ready to fire: `zsh wp-c12/run-collab.sh <tag> http://127.0.0.1:7800 <7800 admin capability file> en|de` (now `verify collab --hub --locale`, credentials from `env.sh` only — the old argv/default-password path removed); late join `probe-c12-latejoin.mjs`; BLOCKED until the chain moves 7800 onto ALL |

| 3 | Live collaboration on 7800/p24 (21:10 READY, channel 19): collab-e2e en + de, records | **en `c12collab-p24-en` 7/15** (1,2,3,4,5,7,12 PASS; 9/10 SKIP external), record `react-collaboration-e2e-en` published (fail). STEP 6: the hub Check In **works** ("Checked in", checkpoint published seq 53) — the harness waited for the message in History, which the shell never shows → harness fixed (status + checkpoint id + rows). STEP 8: **P1** after the check-in, user1's reload shows "Document restore failed … document-archive-replacement.initializer-failed … duplicate mutation id" → keystrokes never leave → relayed to H13 (space 01a0e96e-50e3-76e1-9fda-d0f0b50de003, doc artifact-6e93e2212c8aefbd8b48555f146b8572); plain base/check-in/reopen sequences do NOT reproduce it (`c12reopen-p24b/c/d`). 11/13 = whole-text SET (item 2 set, L1 train). 14: writer caret + draw pointer PASS, puzzle3d canvas never attached (300 s). 15: harness defect — STEP 14 left both humans on draw/puzzle3d → fixed (re-open the writer first). **de `c12collab-p24-de` 0/15**, record published (fail): STEP 1 — **P1 Home table shows only rows 1–11 of 29** (en + de; wheel/PageDown/End never window further, `c12home-rows-2.txt`), so the created space never appears → relayed S18/LB2; re-run after the fix |
| 4 | Late joiner + presence (7800/p24, API-created shared space, one serve) | `c12late-p24b`: **writer 6/6 PASS** (late join sees the whole history, roster 2/2 both ways, A's caret appears + moves in B, marker in B, B's edit reaches A — caret-move was FAIL on 27th); draw: late join + roster + pointer PASS, selection FAIL, B→A not proven (the probe's `addLayer` verb did not apply); puzzle3d: same pattern (pointer PASS; `addNode` not applied); puzzle2d: render error "no registered board session factory" (open since 27th); note: late join + roster PASS, cursor leg found no canvas (probe) |
| 5 | Reopen data-loss race (found while isolating STEP 8) | `c12reopen-p24c` round 2: after A's hard reload the editor mounted EMPTY before the tail caught up; A's first keystroke (whole-text SET) wiped the earlier text for both (m0 lost). Not reproduced in `-p24d` (caught up at mount, 39 ms). Root cause class = typing accepted before catch-up + whole-text SET; the splice set (item 2) removes the data loss; the host should also hold typing until the document caught up (follow-up) |

| 6 | STEP 8 root cause (H13 capture + code) → prepared set for L1's T3 | a replica folds every hub-tail operation as a remote edit NAMED AFTER ITS MUTATION ID (`store::edit_from_operation_envelope`); writer's store initializer (SeedHistory) seeds the entry id AND each forward's id → the same id twice → `duplicate mutation id`, document unloadable. Same copy in 7 more plugin initializers (gismap, jack wire-runtime, draw owned, raster, generation2d, generation3d, process3d) + the SDK bounded initializer; only the framework's two hydrations had the guard. Set `wp-c12/seed/c12-seed-history-patch.py` (**dry run 15 planned / 0 problems**, 22:49): ONE rule `ArtifactStoreInitializationRuntime::seed_edit_operation(entry_id, id)` (an operation named like its own entry is that node; cross-entry repeats still refuse) used by all 9 initializers + both hydrations + the composition test; law `a_document_folded_from_the_hub_tail_initializes_again` replays H13's 17-envelope capture (fixture `✳️any/🧫️fixtures/🔁️hub-tail-after-check-in/🔣️.json`, wire fields verbatim; the first Commit has no parent — decoded) → fold → pack → writer initializer → candidate == fold. **Written, not run** (cannot pass without the fix; L1's lanes prove it). The fixture directory is new → R10 taxonomy check |
| 7 | 8010 (H13, declared-batch fix) runs | `c12collab-8010-en` 22:19: **7/15** (1,2,3,4,5,7,12). STEP 6: after the STEP 6 hard reload user1's document never reached the hub (`execution-target/browser-actor` request ERR_ABORTED on the reload, "Check In (1)" stays pending, no check-in status) — same class as STEP 8 (the reload's browser-actor child never came up); STEP 15: the 5 s cut passed; in the 15 s cut user2's browser actor child faulted on a resumed hub frame (`[backbone-worker] malformed hub frame … invocation rejected: invoke reactor/poll: Error: [object Object] (see error.payload)` — the guest error payload is lost in `childRejectionText`, follow-up) and user2's editor stayed empty, so the keystrokes never showed there. The machine rebooted 22:42 (every process died) during `c12collab-8010-de`; re-runs from 22:51 |

| 8 | STEP 15 on 8010 (H13 binary 2155, declared-batch fix) | full harness `c12collab-8010-en2` (22:51): **7/15** again (1,2,3,4,5,7,12); STEP 15 "the 5000 ms cut rebuilt the document" — contaminated by STEP 13's concurrent whole-text SETs (the worker's reorder backstop rebuilds). Standalone probe `c12short-8010` (`wp-c12/c12-shortage-run.sh`: proxies 8023/8024 → 8010, serves 6525/6526 via `ensureDevServe`, A types alone, en + de): **5 s cut: all PASS** (no freeze — worst frame 25/32 ms, en/de link state, convergence, every keystroke in both editors, **no rebuild**); **15 s / 60 s: FAIL** — no freeze and localized states hold, but both clients rebuild ("fresh authoritative restore", A 3/7×, B 3/4×), A's pill "Pending (64)" drains, user2's browser actor child faults on a resumed hub frame (`malformed hub frame … invocation rejected: invoke reactor/poll: Error: [object Object] (see error.payload)`), the editors converge EMPTY (15 s) or B stays empty (60 s); user1 also hit a pageerror `SyntaxError … "A " is not valid JSON` after a 500 from its dev serve (URL not captured). Next: read the child's payload (row 9 patch), then root-cause the ≥ 15 s rebuild in the worker (resume refused → reopen path) |
| 9 | Guest fault payload lost in the child | prepared `wp-c12/child-payload/c12-child-payload-patch.py` (browser bundle → L1's train): `childRejectionReason` carries a component-model error's typed `payload` (JSON; bigints decimal, bytes as length; bounded) instead of "[object Object] (see error.payload)"; law case added; dry run 3 hunks / 0 problems; projection checked on the law's own input (`child-payload/check.ts`). SeedHistory law amended per H13: a local edit before AND after the tail (STEP 8's mixed shape: 15 remote + 2 local); re-dry-run 15/0 |
| 10 | collab de | `c12collab-8010-de2` (23:16, load 63): **1/15** — STEP 2's share dialog timed out (30 s), everything after it skipped/failed; under that load not a product verdict. de on 7800 (Home fix) and a de re-run on 8010 wait for load < 40 (coordinator rule) |

**Session 14c log**

- 17:0x read preamble (14c), AGENTS.md, fleet tail, this report. Harness wrapper + worker edits all in HEAD `dfe2687`; no own working-tree diff.
- 17:03 worker vitest run 1 (lane, `vitest-worker-14c-1.txt`) after the resume fixture + u64 law state fix: 133/134 (load-timeout only).
- 17:0x C13 relay (via main): red 1 is mine; prefer the worker's own state constructor → `newArtifactState` extracted; relayed back (landed).
- 17:05–17:10 tsc os (306 s under load 65): 84 errors, all peers'. 17:22 worker vitest run 2: **134/134**. 17:23 boot OK. Serve 6520 stopped by the fixture (port free).
- 17:2x C13 relay → retention law 30 s timeout; run 3 134/134 (17:27); relayed back.
- 17:3x–17:5x prepared set: host splice mode + host vectors + simulation law (first version passed the "ignore unapplied" mutant because the replica applied the host's splice synchronously → modelled the host→replica lag; now caught), conformance quintet/catalog/subject/feature/Python twin; dry run 111/0. `run-collab.sh` credentials moved to env.

- 21:10 coordinator: 7800 READY on p24. 21:11–21:27 collab en (`run-collab.sh c12collab-p24-en … en`, two harness serves 6521/6522 + 1 browser/2 contexts, stopped by the harness). 21:3x reopen probes (one serve 6520 via `c12-with-serve.ts` = S18's `ensureDevServe`). 21:4x harness fix `wp-c12/c12-harness-p24-edit.py` (STEP 6 success signal = `#s-checkin-status` localized ready text + `data-history-json.currentCheckpointId` moved; STEP 14/15 share `collabReopenWriter`); tsc os **0 errors** (`generated/tsc-os-14c-2.txt`, 193 s). 21:51 collab de → STEP 1 Home window bug. 22:0x Home probe, late-join probe with an API-created shared space (`createSharedSpace` in `c12-lib.mjs`, capability read in-page only). Relays: S18/LB2 (Home), H13 (STEP 8 ids; STEP 15 on 8010 requested).

- 22:42 machine reboot (every process died, incl. `c12collab-8010-de`). Torn-write check: every C12 script dry-runs clean, every edited tree file parses (`bun build --no-bundle`), JSON fixtures parse — nothing torn.
- 22:4x SeedHistory set (`wp-c12/seed/`), relayed; H13 approved + amendment (mixed local/remote shape) applied. 22:51 `c12collab-8010-en2`; 23:05 `c12short-8010`; 23:16 `c12collab-8010-de2`. Serves/proxies/browsers of every run stopped by their wrappers (ports 6520–6526, 8023–8026 free at 23:30).

**Window-3 runbook (C12 set)** — after "WINDOW 3 OPEN": `python3 wp-c12/splice/patch/c12-splice-patch.py` (dry run) → `--apply`; native lane: `cargo test -p semio-framework-ui-scene` (text_splice law) + writer crate `--lib --tests` (leaf quintet, command, window-state, concurrent-typing law); `bun test` scene `🧪️tests/✂️text-splice`; the writer `mutate-writer-1` case (Rust subject + Python oracle); `tsc` os + boot to Home (rule 20); wasm32 writer via the window-3 wasm lane; then live: collab-e2e en + de on 7800/ALL (STEPs 11/13 = two humans typing at once) + `c12typing` probe.

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
