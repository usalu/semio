# result-verify2-g (wave 1026g, executor verify2-g): journal-backed dependency verification, a journal service that survives the live tree, the plan-reuse leak

`C` = this ticket. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `B` = `L/🏗️build`, `N` = `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes`. Tools `C/verify2-g-tools/` (`edits/*.py` = the single source of every change, `work/` = edited copies, `staged/<batch>/` = apply/check/revert + verify + base hashes, `restage.sh`, `selftest.sh`, `mir.sh` = own mirror `v2g` under `$TMPDIR/v2g-sm`, `bench-verify.ts`, `profile-dep*.txt`, `bench-final.txt`). Nothing of the library/process/launcher is edited live; no official graph run was made (only `repo dependencies ensure`, `repo leases status`).

FOR gate-g: two batches, in this order, both selftested (apply, idempotent re-apply, revert, drift refusal exit 2) and `tsc --strict` clean on every touched file:
1. `C/verify2-g-tools/staged/journal-fix` (6 files; `sh .../apply.sh|check.sh|revert.sh`, laws `sh .../verify.sh`).
2. `C/verify2-g-tools/staged/vouch` (10 files; apply AFTER journal-fix; laws `sh .../verify.sh`). Real-run proof owed: `SEMIO_REPO_PROFILE=1 bun repo dependencies ensure` twice; the second must show the mark `dependencies.vouch` = `journal` and `dependencies.verify` + `dependencies.recheck` well under 1 s (today 3.7-6.7 s + 5.4-7.7 s at load 33).
FOR graphlat-g: section 5 (journal-backed graph verification, hook points in `generationHostV1`, no overlap with your `plan-once` edit of `🕰️generations/🟨️.mjs`).
FOR toolsetup-g: the 4 + 1 service leak pairs listed in section 4 are the ones your note described; the oracle fix is in `journal-fix`.
FOR coordinator: the change-journal service never survived on the live tree (section 1); every journal consumer so far (plan-reuse cursor, watch-g's world check) silently walked. The 6 leaked `reuse-*` pairs below are not mine; they run old code and idle out after 2 h.

## 0. Where the time goes (measured today, `SEMIO_REPO_PROFILE=1 bun repo dependencies ensure`, load 33-42, `C/verify2-g-tools/profile-dep{1,2,3}.txt`)
| step | run 1 (memo MISS, load 42) | run 2 (hit, load 33) | run 3 (hit, load 32) |
|---|---|---|---|
| `source.capture` in the door | 18.1 s (peers edited the 1,456-file closure) | 0.6 s | 0.7 s |
| child boot (loader of the 17 MB snapshot, before `leases.open`) | 11.7 s | 8.2 s | 6.2 s |
| `leases.open` | 0.9 s | 1.7 s | 1.1 s |
| `dependencies.verify` (complete pass) | 5.6 s | 6.7 s | 3.7 s |
| `dependencies.witness` | 0.8 s | 0.3 s | 0.6 s |
| `dependencies.recheck` (second pass) | 7.7 s | 5.4 s | 5.9 s |
| wall of the command | 49 s | 27 s | 22 s |
So the "admission" of the bench classes is NOT the capture on a hit (0.6 s) but the executor boot behind it; the capture only costs when a peer edited the closure (18 s at load 42). The "double verify" is not two call sites: it is ONE function, `ensureBuildEnvironmentV1` -> `inspectIn` (`B/🚀️bootstrap/🟦️.ts`), which runs `verifyJavascriptDependencyGenerationReceiptV1` (complete) and, after the parser witness, the same function with `recheck:true` (all tiers but the 2,988 provider rows). Both walk the same 79,005 frontier rows (24 MB evidence), 5,605 input sources and 327 manifest texts. The two passes differ by 2,988 of ~88,000 rows, so the second is no cheaper than the first (5.4-7.7 s vs 3.7-6.7 s).

## 1. Finding: the change-journal service dies with "memory" within seconds on the live tree (fix staged: `journal-fix`)
Reproduced three times on `/Users/ueli/Documents/semio` (state.json `reason: memory`, `ready: false`, 237 KB of log in 2 s). `maximumResidentBytes` was 50 MiB, a fresh service is 33-37 MB, and JSC does not return pages after a burst (forced `Bun.gc(true)` leaves RSS unchanged: probe 57.7 -> 57.8 MB), so one burst of a few thousand rows (20 agents editing) ends the service; the next client restarts it, it dies again. Controlled evidence (`scratchpad flood.ts`, fake watcher): 150,000 rows peak 51.5 MB, 1,000,000 rows peak 62-68 MB. The `reuse-*` test services survived only because test trees are quiet. Cure: cap 128 MiB (the resident build service has 512 MiB). With the fix four services of my own bench stayed alive 8-12 minutes on the live tree at 38-42 MB RSS.
Also in `journal-fix` (all with laws in `📜️journal/🧪️tests/🟦️.ts`, 7 tests incl. the new ones; neutral changes suite 9/9 unchanged):
- `root-gone`: a service whose workspace vanished stops itself and removes its journal (temp checkouts of tests);
- `directory-gone`: a service whose journal directory vanished stops (4 services of my bench kept running at 5-12 % CPU and ignored SIGTERM after their private directory was removed; needed `kill -9`, done by pid);
- a watcher `stop()` that never returns can no longer keep the process alive (`stopWaitMilliseconds` 3000, plus a hard exit 9 s after SIGTERM), law with a hanging fake supervisor;
- law `🌊️burst` (child process, 1,000,000 rows): alive under the declared bound, and DEAD under the former 50 MiB bound (negative control proves the law bites).

## 2. Finding: the plan-reuse oracle leaks one journal service per run (fix staged: `journal-fix`)
`B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts` "decision cycle" calls `prepareBazelPlanReuseV1(tempCheckout)` -> `takeBazelPlanCursorV1` -> `connectChangeJournalV1(root).cursor()`, which starts a detached service (bun ~33 MB + `semio-repo-watch` ~5 MB, ppid 1, 2 h idle shutdown) for a checkout the test deletes seconds later. Attributed proof: I ran the unfixed test once (pids 91256 service + 91771 watcher appeared, root `T/reuse-AonApo/checkout`, started 19:39:49) and killed exactly that pair; the fixed test (`SEMIO_REPO_CHANGES=off` around the cycle, restored in `finally`, plus `expect(!existsSync(changeJournalDirectoryV1(root)))`) starts no process (polled `ps` for 26 s). Together with `root-gone` a future leak ends within a second of the checkout's deletion.
Not mine, not killed (old code, roots are deleted temp checkouts, idle shutdown after 2 h): 15575/15772, 25051/25872, 32662/32734, 37825/38007, 39046/39124, 40409/40492 (service/watcher). A coordinator may `kill` the six service pids (SIGTERM is enough for old code that is idle).

## 3. Delivered: `vouch` (the journal narrows the dependency verification; both passes become O(changes))
Design (schema-first, EN/DE texts not needed: the vouch is silent, it only narrows):
- NEW neutral module `N/🪪️vouch` (`🟦️.ts`, `🧬️schema/🔣️.json`, tests): `openVouchV1({client,directory,consumer,key})`. With a cursor sealed for the same `key` (the receipt sha256) it asks `client.since(cursor)` and `skippable(paths)` returns the workspace-relative paths no change touched; without one (first run, other key, restarted service = new epoch, gap, overflow, no service, switch off) it only takes the start cursor (`mode:"cursor"`), so the SECOND look of the same verification is answered by the journal (`settled(paths)` = fresh fence, cumulative rows) instead of a second walk; any refusal gives empty sets = today's complete walks. `seal()` stores the latest cursor only after both passes held (atomic write, a newer cursor of the same epoch is never replaced by an older one, record in the journal's private runtime directory, never in the checkout). Paths in excluded regions (`.git .cache node_modules target …`), outside the workspace, the root itself and below symbolic links are never vouched.
- `verifyGenerationReceiptV1` (neutral) gets one optional port `vouched(paths)`; it is consulted for the input sources + manifest texts and for the frontier rows and those rows are skipped; evidence rows decoded once are reused by the second pass (checked against the sha256 of the bytes every time, dropped after 30 s: `decodedRowsRetainMilliseconds` in the schema).
- `verifyJavascriptDependencyGenerationReceiptV1` (storage glue) takes `options.vouch(receiptSha256)`; `ensureBuildEnvironmentV1` creates ONE `dependencyVouchV1` per look, passes it to both passes, marks the profile `dependencies.vouch` with `journal|cursor|walk[:reason]|off` and seals after the recheck matched. New file `B/🏡️resident/📜️journal/🪪️vouch/🟦️.ts` (`openJournalVouchV1`) is the only wiring to the real journal. `SEMIO_REPO_CHANGES=off` restores today's behaviour exactly.
Laws (all green on a mirror of the live tree, live tree untouched, guard clean):
| law | result |
|---|---|
| vouch decision == independent Python model (`🪪️vouch/🧪️tests/🐍️.py`, shares no code) on 9 fixed + 400 seeded random vectors (case folding, exclusion, links, structural vs modified directory rows) | pass |
| soundness against the file system: 40 seeds of random create/modify/delete/rename/chmod/touch/mkdir on a real tree; everything the vouch proves has identical stat identity + listing afterwards | pass (proves a non-trivial share, never everything) |
| lifecycle with a fake journal: cursor mode, journal mode, wrong key, restarted epoch, gap, no service, torn/foreign/failing, cancellation, monotone seal | 7/7 |
| generation receipt contract (existing 13 laws incl. the Python keying oracle, recheck-equivalence) + NEW: for all 40 fixture cases a stat-based vouch (proves exactly the unchanged paths) answers as the complete verification (class, path, detail, sha256), in the recheck too, a vouch that lies is caught by at least one case, and rows are skipped | 14/14 |
| REAL journal: for each fixture case a real file-watcher service (FSEvents) on the case's workspace; seal, apply the case's mutations, reopen (must be `journal` mode), verify vouched == verify complete, recheck settled == recheck complete; skipped rows > 0 | pass (70 s; skipped without `SEMIO_REPO_WATCH_BINARY`, `verify.sh` sets it from the compiled watcher) |
Measured on the LIVE receipt (`C/verify2-g-tools/bench-verify.ts`: 79,005 frontier rows, 5,605 inputs, 327 texts, 2,988 provider rows, real ports, real journal service started from the staged code on the live workspace):
| (CPU is the robust number: the host load was 25-43) | complete pass 1 | complete pass 2 | vouched pass 1 | vouched pass 2 |
|---|---|---|---|---|
| CPU | 1.02-1.45 s | 1.03-1.62 s | 0.33-0.47 s | 0.31-0.48 s |
| wall at load ~25 | 3.07 s | 2.95 s | 0.57 s | 0.54 s |
| wall at load ~42 | 6.8 s | 8.5 s | 3.0-3.2 s | 3.9-5.7 s |
First run after a journal start (`cursor` mode): pass 1 complete (0.92 s CPU), pass 2 answered by the journal (0.62 s CPU, 80,216 paths settled) = the old "double verify" already halves in the cold case. Expected on the hot course no-op (to be proven after the apply): `dependencies.verify` + `recheck` 9-14 s -> about 1-2 s at load 33, ~0.7 s CPU on a quiet host.

## 4. Not done / decided against (with the numbers)
- Source admission (cut 2): on a memo HIT the capture costs 0.6-0.7 s (load 33), the 3-6.5 s the bench class "admission" shows is the executor boot behind it (6.2-11.7 s to `leases.open` at load 32-42), which is not a stat question. The memo already vouches by content; a journal could only save its directory/absence proofs (watch-g's conclusion, confirmed). On a MISS (a peer edited any of the 1,456 closure files, i.e. almost always during this wave) the capture costs 18 s at load 42; the cure is a SEEDED recapture (per-file reuse of unchanged files' digest + import map, the journal naming the changed files) in `⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts`, a 29 KB security-relevant file; not attempted blind. In the steady state of a product developer (closure untouched) it is a hit.
- Graph skip (cut 3): see section 5; measured, designed, not implemented (needs engine plumbing in graphlat-g's area and cannot be proven on the live tree while peers keep the generation stale: my read-only replay of the fact index finds stale rows in every kind).
- A real-run proof of `vouch` needs gate-g's apply (rule: library edits only staged). Until then the numbers above are the bench on the live receipt with the staged code (simulated end to end, measured per tier).

## 5. Graph phase: what a journal can and cannot skip (measured, read-only)
Hot graph phase 8.5-27 s = child boot (the same executor boot, ~6-8 s) + plan: frontier stat pass + fact verification + assembly + plan file (graphlat-g: three plan passes per refresh, 20-30 s each under load).
- Frontier (`membershipByStat` in `generationHostV1`, `B/🕸️graph/🟦️.ts`): 79,292 directories + 27,041 members = 106k `lstat`; vouch-able with the same primitive (consumer `graph:frontier`, key = frontier digest) by handing `frontierHoldsV1`/`advanceFrontierV1` a `stat` that answers the recorded stamp for vouched paths; seal when `frontierHoldsV1` held.
- Facts: the newest fact index holds 535,723 rows: readdir 126k, exists 119k, analysis 70k, closure 45k (outside), lstat 43k, resolve 28k (outside), fileExists 26k, directoryExists 21k, file 20k, field 11k (outside). A replay of the whole index with the real `observationsHold`, 128-row chunks, `symlinks=true`, load 36: 100 s wall / 28 s CPU (readdir 11.2 s CPU, analysis 8.2, exists 4.1, resolve 1.4, lstat 0.65, file 1.0).
- Journal-safe kinds (truth is a pure function of the path's own state or of its direct entries): `file`, `lstat`, `readdir`, `readdirNames` (~47 % of that CPU; readdir/lstat are already free inside directories the frontier proved stable). NOT safe: `analysis` (the fact carries `nestedObservations`, re-checked by `nestedObservationsHoldV1`), `exists`/`fileExists`/`directoryExists` (follow symbolic links: a change of the target raises no event at the link), `resolve`/`closure`/`field`/`typescript*` (paths outside the workspace or in excluded regions). A sound whole-graph skip therefore needs (a) the symlink set of the fact paths recorded at seal time (one `lstat` per path, cold path only) or a "no event anywhere" criterion, (b) the vouch key = `manifest.factIndex` (available to the host via `previous.manifest` in `membership(previous)`), (c) a seal hook after `plan()` returned `clean` with no stale chunk. Honest estimate: frontier + safe kinds save ~1-3 s CPU of the graph phase; the child boot and `analysis`/`exists` stay. The brief's "no re-entry into graph refresh at all" additionally needs a run-level memo {request, generation pin, pinned snapshot digest, cursor} validated against the union of the fact paths; that is the next design step and belongs with graphlat-g's `plan-once` once it has landed.

## 6. Target status (hot course no-op, bench2-g q4: 60-83 s; target < 10 s)
| phase | before (measured) | with my batches (measured tier / simulated end to end) | other owner |
|---|---|---|---|
| door capture (hit) | 0.6-0.7 s | unchanged | misses: seeded recapture (open) |
| executor boot | 6-12 s | unchanged | exec/boot lanes |
| dependency verify + recheck | 9-14 s (load 33), 15.3 s wall (load 42) | ~1-2 s (CPU 2.1 s -> 0.8 s, wall 15.3 s -> 6.9-8.9 s at load 42) | - |
| graph | 8.5-27 s | unchanged (design in 5) | graphlat-g |
| tool-setup (planner replan) | 32-48 s | toolsetup-g `probes` | toolsetup-g |
The < 10 s no-op is NOT reached by these cuts: after `probes` + `vouch` the floor is two executor boots (door -> `repository:command` -> graph child, 6-12 s each under load) + the graph plan; getting under 10 s needs the run-level fast path (skip the graph child when the journal vouches the fact paths) and one executor boot instead of two.

## 7. Status (newest first)
- 20:30 batches `journal-fix` (6 files) and `vouch` (10 files) staged, selftest/tsc/laws green, announced to gate-g; bench services of mine killed by pid; mirror `v2g` kept until the batches are applied.
- 20:10 REAL-journal law over all fixture cases green; `journal-fix` extended with `directory-gone` + bounded stop after 4 bench services ignored SIGTERM.
- 19:57 live-receipt bench: CPU 2.2 s -> 0.65 s for the two passes; skippable() rewritten as set lookups.
- 19:46 service-death root cause found (memory cap), plan-reuse leak reproduced and attributed.
- 19:25 STARTED: read brief, toolsetup-g, watch-g, noop2-g results; profile of `repo dependencies ensure`.
