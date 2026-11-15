# R41: restart-to-first-inference of the graph read (PREPARED, NOT LANDED)

Scripts: `T/🧰️scripts/R41/` (T = FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION): `edits.py` (4 anchored edits), `new/…/✂️cutoff/🧪️tests/🌑️epoch/🟦️.ts` (8 tests), `land.py`, `rehearse.py`, `overlay.py`, `suite.py`, `bench.ts`. Scratch: `/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R41/` (`ov` overlay, `suite-before.txt`, `suite-after.txt`, `bench-8129.txt`). No graph read was started; only traces, logs, the node store (read only) and library code in an overlay were used.

## 1. What the two phases verify, and why they are redone

R31 did not skip any verification: it only defers a graph-server restart while a serve runs. The skip the question remembers is R28 (manifest fast path + vouches, landed), and R28 itself says "the cursor-less epoch is unchanged".

- **Membership** (`Verifying workspace membership: x of 127201 entries`, `🕰️host/🟦️.ts` `membershipByStat`, `🧭️frontier` `frontierHoldsV1`): one stat per directory and per member (mode) of the stored frontier, 64 parallel. Without a journal cursor it is the full scan. Measured from the traces: 8 s (98727, 12:14:39 to 12:14:47), 2 to 6 s in 81001, 63529, 54225. It is NOT the 13 to 15 minutes; it is repeated once per plan round (3 times in 63529). I changed nothing here: every member stat is needed to see a mode change, and a persisted stamp cannot replace a stat. On NFS workers it is probably longer (not measured).
- **Input closures** (`Verifying input closures: x of 8129`, `closure-kinds/🟨️.mjs` `runOnce`): 8 129 suspect closures (every closure whose support has a stale fact; one stale shared file makes all of them suspects). With a dirty summary the manifest path answers every closure the dirty set does not reach with no load, and a closure it reaches is compared by its vouch. **Without a dirty summary (`summary === undefined`) every closure takes the slow path** (`graph.get`, part and meta loads, `invalidate`, a second `ask`, about 1 700 parts per closure, about 100 ms each) and **no vouch is ever created or sealed** (`createVouchV1` only exists with a summary), so a read that is killed leaves nothing resumable. The summary comes only from the change journal (`plan` passes `journalDirty` only when the journal answers). The log line `graph: No verified change journal cursor is recorded for generation N; verifying the tree once.` (52 times for generation 389 alone in the worker logs) is that case: a new generation, a cursor lost by a read that died before `factsVerified/published/commit`, a journal epoch change. Every restart of a read in that case starts at zero.
- Per-phase times (traces `98727`, `81001`, `63529`, `54225` in `<tmp>/semio-footprint/`; group server): membership 2 to 8 s, doubt+facts 30 to 45 s (stamp-vouched already), first closure pass 3.5 min (81001), 6.6 min (98727), 10.6 min (63529), then the next plan round repeats membership 8 s, facts 42 s, and the closure pass starts at 0 again (98727 12:23:08, 81001 11:47:10).
- Unique rows are cheap: the 28 146 parts of the live store hold 1 206 600 rows but only 112 447 distinct ones (84 781 paths, 1 bare resolution in 8 665 resolves). Per-row check cost measured with plain fs on real rows: lstat 35 us, fileExists 7 us, directoryExists 4 us, resolve 88 us, analysis stat 54 us, typescriptText 147 us: one pass over the distinct rows is about 5 s. So per-row stamps would save little; the time is the per-closure framework path, which only the manifest path avoids. Scripts `kinds.mjs`, `cost.mjs` in the scratch.

## 2. What was built

The dirty set of a read without a journal is the set of fact rows the plan has JUST verified against the disk and found stale (`stale` of `staleFactRowsV1`, complete: it bisects down to single rows). A closure whose support holds no stale row cannot be moved; a closure that does is compared by its vouch (stamps of exactly the stale files it reads) or derived. No new persisted format: the R28 manifest, part metas and vouches are the content-addressed stamps (vouch = sha256 of file bytes / listing / link target / absent, keyed by the closure's record digest through its entry and by the leaves it reads; never trusted for anything that is not content). Edits:
1. `🧭️row-paths/🟨️.mjs`: new `staleSummaryV1(rows, relative, volatile)`: paths, probed directories, bare ancestors, configuration and install flags of the stale rows, built with the same `rowPathsV1` the part metas use; a row above the checkout adds nothing (the settler verifies those rows once per pass), an unknown kind adds nothing (its parts are volatile and always asked again).
2. `closure-kinds/🟨️.mjs` (+ `.d.mts`): `settle({staleFacts})`; the summary is the journal dirty set when given, else the stale-fact summary, else none; an empty list is no summary (old behaviour).
3. `🕰️generations/🟨️.mjs` `plan`: passes `staleFacts` exactly when `journalDirty` is absent.
Consequence: a read without journal now creates and seals vouches per chunk, saves the manifest every 15 s (R28), and a restart over the same edits derives 0 and checks 0 rows.

Not done, with the reason: a persisted membership stamp (see section 1: no sound substitute for the stat), per-row verdict stamps (about 5 s of work to save), a cross-machine shared stamp (vouches are already content-keyed, so a record from another machine is only used if the bytes are equal; the manifest store itself is per machine).

## 3. Proof

Tests first: `🌑️epoch/🟦️.ts`, 8 tests, 819 expectations; red before = the file does not load (`staleSummaryV1` missing), green on the overlay. Laws: restart over the same stale rows derives 0, checks 0 rows, vouches the 3 (then 8 129 in the bench); changed leaf, leaf back at an older content, shared file reaching all 40 closures are re-derived; removed leaf derived once, vouched while removed, derived again when it returns with other bytes; changed record (the generation holds another fact) is reported changed, not answered; forged vouch is not trusted (rows are checked, an edit under it is derived); torn shard reads as no manifest and the answer stays correct; two concurrent reads over one store agree and the next start derives 0; empty stale list keeps the old behaviour; property law (200 random books, brute-force oracle): every part that holds a stale row is reached by the summary. Mutation check: removing the directory line of `staleSummaryV1` turns the property law red (and it did not before I added unresolved resolves to the generator).

Regression suite (live tree = before, overlay = after; `suite.py`): identical reds before and after, no new red:
| file | before | after |
|---|---|---|
| derived-nodes | 7 pass 7 fail | 7 pass 7 fail (same 7) |
| closure-kinds manifest | 1/0 | 1/0 |
| generations | 30/0 | 30/0 |
| generations/🔌️load | 99/0 | 99/0 (25 settle calls inside it now take the new path) |
| cutoff | 159/0 | 159/0 |
| cutoff/vouch | 8/0 | 8/0 |
| doubt | 8/0 | 8/0 |
| graph-release | 10 pass 8 fail | same 8 |
| reset-closure, plan-stages, carried-closures, repository-walks | 3, 3, 3, 8 /0 | same |
| generation-protocol | 45 pass 2 fail | same 2 |
| epoch (new) | does not load | 8/0 |

Bench at the live count (`bench.ts`, 8 129 closures, 60 shared parts each, 100 edited files + 1 shared file, in-memory-size store on disk, RSS 543 to 605 MB; `bench-8129.txt`): restart over the same edits: **3 964 ms and 8 189 rows checked (restart 2), 5 723 ms (restart 3) before; 1 196 ms, 0 rows, 8 129 vouched (restart 2), 834 ms (restart 3) after**. The first start after a change still derives everything the change reaches (21 s after, 18.6 s before; same work). 1 500 closures: 742 ms to 30 ms.

## 4. Landing (`land.py`)

`--dry-run` (clean on the live tree: 4 edits, 1 new file, 0 refused), `--land [--merge] [--no-tests] [--lock-held]`, `--rollback <dir>`. Takes `T/landing.lock`, backs up to `T/🗑️generated/R41-landing-*`, appends `landings.md`, runs 9 test directories. `--merge`: each file is re-read right before the replace; the anchored edits are applied to that text (R36 and others change the same modules), the result must differ from the live text only by this package's pure insertions, a file that changed between plan and replace is re-planned or, if an anchor is gone or twice, the whole landing is refused and what was replaced is restored. Rehearsal `rehearse.py` in a scratch root: dry run writes nothing, land, second dry run all `same`, lock released, rollback byte for byte (new test removed), land on top of a peer's unrelated edit, anchor removed by a peer REFUSED and nothing written: all ok.

Landing cost: 3 code files of the graph read plus one declaration file change, so the graph server restarts like after any edit of its closure (R31: deferred until the serve ends, at most 60 min). It does NOT change a persisted format (R28 had to), so manifests and vouches already on disk stay valid; no cold start of the manifest. Whether the graph goes stale: `🕰️generations/` is declared `none` in the inference protocol (`📥️inference/🔖️stamps/🧬️schema/🔣️.json:217`) and `🧮️derived-nodes` is in no inferrer's `sources`, which is why R28 landed the same files without a re-stale; I did not run the stamp law on the overlay, so this is read from the schema, not proven.

## 5. Not verified
- The 60 s target for both phases on the real tree. Expected: membership 8 s + facts 45 s + a closure pass that is the manifest path (about 1 s per 8 000 closures in the synthetic bench; with the real 1 700 parts per closure the load of metas for touched parts is not measured) is about 55 to 60 s with the first start after a change still deriving what the change reaches. The synthetic bench does not reproduce the 7 minutes of the live slow path (4 s for the same count), so the saving of the live slow path is argued from code and the traces, not measured.
- First start after landing: vouches exist only for closures sealed under a journal run, so the first epoch read derives what a read derives today, and then keeps its vouches.
- The facts verification is trusted as complete (journal-free doubt stamps of the directory table); a wrong racy stamp there would be as wrong for the projects as for the closures.
- `🧭️invocation/🕸️graph/🧠️memo/🧮️derive` and `🎯️task-state` call the settler without `staleFacts` (unchanged). Linux and Windows not run; `tsc` not run.
