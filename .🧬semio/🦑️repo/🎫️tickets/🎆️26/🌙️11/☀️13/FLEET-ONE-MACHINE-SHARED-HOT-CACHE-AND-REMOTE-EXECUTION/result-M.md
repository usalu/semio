# Package M: a graph-needing `repo` command on the fleet tree

Measured on `/Users/Shared/semio`, 2026-11-14 00:30 to 01:10, load 8 to 13 on 10 cores, 32 GB. I could not start a graph command of my own: a peer's cold `repo graph` (pid 60403, started 00:19, output in the session scratchpad `G/graph.err`, not mine, not killed) held the graph partition and 14 GB for the whole window. All graph numbers below are read from that live run (its stderr, `repo capacity status`, `ps`, `sample`), not from a run of mine.

## 1. Where the time goes (phase table)

| Phase | Seconds | Source |
|---|---|---|
| `door.capacity` (waiting for 5707 MB behind another graph job and cargo builds of other agents) | 335 | `graph.err` ("still working after N s; last phase door.capacity") |
| source admission, implementation source, source evidence | about 20 | graph.err markers |
| `dependencies ensure` before any graph work (stale receipt, full re-walk of 66k entries) | 36 to 43 per command | `SEMIO_REPO_PROFILE=1 repo dependencies ensure`: verify 15 s + witness/capture 17 s |
| remote exchange of the generation | 0 (HTTP 401 at that time, "building the first one") | graph.err |
| commands / playground / cargo targets collection | about 25 | graph.err |
| inference of 4775 projects, cold | 1.3 to 1.5 s per project, 0.65 to 0.78 projects/s with ONE worker; 1167 of 4775 after 31 min; the rest would need about 90 min | graph.err counters, `ps` sampling |
| worker boot (every 10 to 11 s a new `graph-farm-worker` process is spawned: RSS 200 MB, 450 MB after the module load, over 1024 MB after 2 to 3 projects, then recycled because `rssMegabytes > megabytes`) | about 4 to 5 s of every 11 s life | 15 "worker 0 restarted after 10xx MB" lines, `ps` every 1.5 s |

Findings behind the table:
1. Cold inference is CPU-bound on one core: the parent runs at 100 % of one core with 8 projects "running" (single JS thread), a single farm worker beside it, because the farm's `exec` requests (1024 MB each, 9 wanted, 1 granted: "the host cannot carry more right now") wait in the FIFO behind cargo builds of the other agents of this session ("behind semio-framework-repo-identity ... 1075 MB"). On an idle machine the pool would be up to 9 workers. This is contention, not a code defect, and it is exactly what this session's fleet does to a cold graph.
2. The admission books 5707 MB for the graph job, `repo capacity status` observes 14 090 MB for the same process (pid 60403, 9.6 GB resident, 15.6 GB footprint, peak 23 GB). Everything else on the 32 GB machine is starved by the difference (664 MB spare above the 4096 MB floor). Why the parent holds 14 GB at 1000 of 4775 projects is NOT found: no heap profile was possible without a run of my own (`SEMIO_REPO_CPU_PROFILE='{"version":1,"directory":"<abs dir>"}'` gives a `--cpu-prof` of the next graph run; `SEMIO_REPO_PROFILE=1` prints the phase table). Next agent: run those two on the first graph run that can start.
3. The dominating per-command cost found and fixed (section 2) was not in inference at all.

## 2. Landed fix 1: the dependency receipt never verified (36 to 43 s per command, now 7.7 s)

Cause (read in code, reproduced, fixed, measured). Commit `914d23addd29` ("Stamp dependency symlinks with a fixed ctime so a later attribute bump cannot fail the store fence") made `javascriptDependencyStampV1` (`📦️dependencies/💾️storage/🟦️.ts:275`) record `0` as change time for symbolic links. The receipt module kept comparing the real change time: `generationStampV1` (`🧾️generation/🟦️.ts`). The writer verifies each receipt it has just captured before it publishes it: the first link row (`node_modules/@semio-tech/course`) failed with `provider-changed ... identity`, the outcome was `stale`, and no `*.receipt.json` was ever written (none exists in `.cache/b/d/m/generation`). Every `repo` task or graph command therefore walked the whole dependency generation again (66 747 entries, `dependencies.verify` 15 s + `dependencies.witness`/capture 17 s) and printed `Store dependency generation receipt stale` each time. This is the "[DATA] ... receipt stale" line in the first lines of every run on this machine tonight.

Fix: `generationStampV1` uses `0` for the change time of a symbolic link, the same formula as the walk. Test (`🧾️generation/🧪️tests/🟦️.ts`, 16 pass, 1 skip): the stamp of a link equals the walk's `javascriptDependencyStampV1`, survives an `lchown` change-time bump, equals an independent Python `os.lstat` rendering; regular files keep the real change time. The red law test `stamp and anchor equal the provider walk's own formulas` (red since the commit) is updated to the current storage text.

Real, same machine, `repo dependencies ensure`:

| Run | Wall |
|---|---|
| before, run 1 | 36.3 s |
| before, run 2 | 43.5 s |
| after, run 1 (captures and publishes the receipt) | 43.5 s (the publishing run) |
| after, run 2 | 9.4 s |
| after, run 3 | 7.7 s (boot 2.2 s, verify 3.2 s, recheck 1.6 s) |

Every graph/task command saves about 30 s from now on, on every machine that has the commit (each rebuilds its receipt once). No migration: no receipt could have existed.

## 3. Landed fix 2: parser-free rows first (part of F's hand-over)

`🕸️graph/🕰️host/🟦️.ts`: `splitHoldRowsV1`/`PARSER_FREE_HOLD_KINDS_V1`; `holdsByStat` now checks rows of kind `file exists fileExists directoryExists lstat readdir readdirNames` (and, as before, `digest` and extract-`field` rows) before `analysesPrepared()`; only the rest (`analysis`, `resolve`, `stamp`, `registry`, ...) waits for the producer import. Semantics are unchanged (the two groups have disjoint row identities, so the consistency check is the same). Tests: new `🕸️graph/🧪️tests/🪶️hold-order` 3 pass (split equals an independent Python partition; source order law); neighbours green: upgrade-before-custody 3, carried-closures 3, prefetch-booking 2, generation-protocol 47. NOT verified through a real graph load (see 6).

## 4. Not landed: lazy dependency install for a graph read

Decided against landing it tonight, with reasons, because a wrong version breaks every `repo` graph command of eight machines:
- After an install the whole command must restart (`dependencyReadmissionV1`, the source closure includes the installed packages), so a lazy install is "try without, restart with" and needs every path that can reach a parser (`analysesPrepared`, `deriveAgain`, `implementationRows`, `prime`, `discoverPart`) to fail with a typed "parser material required" instead of a collected inference error; a cold build without parsers could record wrong rows. A safe version is read-only: allowed only when a current generation exists (local or restored) and every non-parser row held; any `analysis` row that is stale throws the restart.
- It cannot be tested here: the live tree has its dependencies installed, and a tree without them cannot boot `repo` (F's finding), so the lazy path has no world to run in.
- Value today is nil: `show` still runs Nx, which needs the install; only `graph` could benefit, and only once a fleet graph exists (nobody has published one: `<store>/v1/fleet/` holds `cache.json` and `fetch` only). The fleet cache itself is configured and authorised (`repo remote-cache status`: endpoint 6094, read-write, upload on).
- Part (a), landed, is the prerequisite.

## 5. Incident: `⚡️caching/🚀️bootstrap/📜️script.ts`

Current file (sha256 `95cc805a4013`, 673 lines, written 23:52:17), checked against the landings of A, C, F, G, H, I, L in `landings.md`:
- Referenced-but-undefined names: a TypeScript program over the file (`noResolve`, only "Cannot find name" diagnostics) reports only `process` and `Buffer` (no type libraries loaded), no other undefined identifier. The same check on the two files I edited is clean.
- Hooks present: A `resolveRemoteCacheV1` (import line 20, use line 254); G Nx single-flight front (`routeNxThroughFlightV1`, `stopFlight`, lines 448-460), `slot-settle` (`settleNxSlotsV1`, line 460, called in the outer `finally` line 628 before the front stops), `nxRemoteCacheEnvironmentV1` (455); F fleet graph step (`beginNxFleetV1` line 545, `finish` lines 603 and 612, release in `finally`). C's slot hook is G's settle (C's result says so). H, I and L recorded no edit of this file (their changes are in `🛰️fleet-fetch`, `🦀️cargo/🗃️unit-cache/📜️script.ts`, `🛰️fleet`).
- Verdict: no lost hunk found. Not excluded: a hunk that only an unrecorded agent added between 23:16 and 23:52.

## 6. Closure cap (2048)

Package J split the runtime kernel (2061 to 1960 files, cap 2048, early-warning test). I did not touch the limit. Remember when adding code: the runtime kernel has about 88 files of headroom; this package added no file to any kernel closure (one test directory only). `⚙️operations/🟦️.ts` still drags the Nx script (about 100 files) into the kernel for one catalogue lookup (J's note).

## 7. The three times (target: b under 30 s, c under 5 s) NOT measured

(a) cold local, (b) fleet copy present, (c) warm: none measured, because the graph partition was held by the peer's cold run (it would finish in roughly 90 more minutes and would also give the warm graph for (c)); starting a second graph command would only queue behind it (5707 MB each), and killing a process I did not start is forbidden. What exists instead: the per-command overhead before the graph work shrank from 36 to 43 s to 7.7 s (section 2), and a hit/restore costs per F: Nx graph about 2.5 s, portable generation restore 11 s + row verification (unmeasured). Targets (b) and (c) cannot be reached yet because (1) no machine has published a graph, (2) the warm path still pays about 7.7 s in `dependencies ensure` alone (c is above 5 s unless that is also made lazy or cached below its 3.2 s verify + 1.6 s recheck + 2.2 s boot), (3) the 472k-row verification after a restore is unmeasured.

## 8. Next steps for whoever continues (in order)

1. When the peer's graph run ends (or is stopped by its owner), run once with `SEMIO_REPO_PROFILE=1` and `SEMIO_REPO_CPU_PROFILE='{"version":1,"directory":"<abs>"}'` for the warm time (c) and the parent's 14 GB: `repo show projects`; then `repo graph` reset through the supported command and a second run for (b) after one machine has published (check `<store>/v1/fleet/` for the graph entry).
2. Farm: `recycleReasonV1` recycles a worker whenever `rss > megabytes` (1024) although a freshly booted worker already sits at 450 MB; make the cap relative to the post-boot baseline (`slot.baseline` exists) to cut the 4 to 5 s boot per 2 to 3 projects. Unmeasured estimate: up to 30 % less cold time with one worker.
3. Admission: the graph job reserves 5707 MB and uses 14 GB; either learn/book the true peak or give the parent a `BUN_JSC_forceRAMSize` as the workers have (the `🧠️memory` module declares `childHeapMegabytes` for graph children; I could not verify the parent gets it: `ps eww` shows no environment).
4. Seed: a published root-neutral generation (F's `publishGenerationV1`; the owner's checkout has a warm one, read-only attach of `.cache/b/g/fixed.dmg` as F did) would let a machine skip the 90-minute cold build.

## 9. Unverified

Any real graph command through the host change; (a), (b), (c); the cause of the parent's 14 GB; recycle estimate; Windows/Linux; the receipt fix on other machines (same code, expected to behave alike, not run there).

---

# 2026-11-14 02:30: cold inference, the farm, the parent, publishing (coordinator follow-up)

The peer's run (pid 60403) was stopped by its owner at 1168 of 4775 (SIGTERM, 00:58), so the graph partition was free; I ran `repo graph` six times myself (logs and monitors in the session scratchpad `M/g1..g8.err`, `mon.py`). All runs were stopped by me with SIGINT; none completed. The machine was never quiet: load 8 to 13 at 01:00, 18 to 37 at 02:20 (other agents' cargo and test runs).

## Causes found and fixed (measured where the machine allowed)

| # | Defect | Evidence | Fix | Result |
|---|---|---|---|---|
| 1 | Only one worker ever ran | `requestBuildFarmCapacityV1` is elastic and asked ONCE ("admitted exec with 1 of 9 job(s)", the grant never grew); in the old run `exec` was `1 job` the whole 30 min | `project-farm/🟨️.mjs`: ask again for the missing workers until the plan is met; `close` cancels the open ask; test: second ask is `preferredJobs - granted` | live: 3 workers at first grant, then 8 workers within 3 minutes (`ps`, `repo capacity status`: "waiting exec wants 6 job(s)") |
| 2 | Admission blocked a graph read behind other jobs' bookings | `graph: waiting ... the cores are fully booked by our own running jobs` while load was 7 and 12 GB spare: three `repo cargo` jobs book 8+10+1 jobs although they use little CPU; the graph waited 330 s (old run) and 180+ s (mine) | `host-capacity/📏️decision`: a `graph`/`graph-plan` request of 1 job counts the real load, not the ledger | after the change `door.capacity` 19 ms (profile row) instead of 180 to 335 s; a compile of the same size still waits (test) |
| 3 | Workers died after every job | `graph: worker N restarted after 1070..1137 MB` every 10 to 11 s; the cap was the absolute budget (1024 MB) and a worker is already 450 MB after its modules, 1.1 GB after the first job; boot costs 4 to 5 s | `recycleReasonV1`: ceiling = max(budget, 1.5 x first-reply size); python oracle and fixtures updated; worker calls `Bun.gc(true)` before it measures | restarts: 53 in 1596 projects with 3 workers (old code, g7: one per 2 jobs per worker) against 38 in 402 projects with 8 workers (g8): about one per 10 projects |
| 4 | Parent at 14 to 16 GB while admission books 5.7 GB | `repo capacity status`: reserved 5707 MB, observed 14090 MB; parent resident 4 GB at the first project, +20 MB per project until a plateau of 14.8 GB at about 330 projects: no leak, the JavaScriptCore heap grows to about half of the RAM because the parent (unlike slice workers and the resident host) gets no `BUN_JSC_forceRAMSize`; the door runs in the same process, so the variable cannot be set after launch | `project-nodes/🟨️.mjs`: every 16 projects, if resident > the grant's parent share (min 2 GB) collect (`createMemoryPortsV1().collect`), next threshold max(share, 1.5 x resident after) | parent at 330 projects: 15.0 GB (g7) against 10.7 GB (g8), 402 projects 8.8 GB; bounded but not at the booked 5.7 GB (the live set after the first project is 4 GB) |
| 5 | A dependency command failed on the first root symlink | `Internal error: ENOTDIR: lstat '/Users/Shared/semio/CLAUDE.md/package.json'` (the harness created `CLAUDE.md -> AGENTS.md` at 00:58) in every `repo graph` and task command, found by my third run | `🗂️workspaces/🟦️bun/🟨️.cjs`: `ENOTDIR` is missing | test (99 pass), run went through |
| 6 | The finished graph could not be published or restored | `graph: the remote cache did not exchange the generation (Content store answered 401 ...)` in every run | `remote-exchange/🟦️.ts` used the environment token only; now `resolveRemoteCacheV1` + `remoteCacheFetchV1` (fleet identity, signed) | probe against the live cache: plain 401, signed 404 (authorised miss); exchange tests 6 pass |

## Rates (projects per second of the inference phase)

- Before (peer's run, one worker, load 9 to 13): 1167 projects in about 1410 s = 0.83 on average, 0.5 to 0.65 once the 8 "running" items were rust owners; worker life 11 s of which 4 to 5 s boot.
- After (g8, 8 workers, load 18 to 37 from others): 402 projects in 11 minutes including the 60 s start = 0.6 to 0.9 per second, with the farm at 4 to 8 workers. Within the noise of the load the rate did NOT improve; the structure did (8 workers instead of 1, a worker per 10 projects instead of per 1 to 2, a start in 60 s instead of 6 to 7 minutes, parent 9 to 10 GB instead of 15 GB).
- The target (a cold graph under 10 minutes) is NOT reached and cannot be with these fixes: a profile of a worker (`sample`, 5 s) shows it neither waiting nor parsing: about 60 percent of the on-CPU samples are `openat` (1758) and `getdirentries64` (536), JavaScript the rest. Per project that is about 3 to 5 CPU-seconds, mostly opening and listing files (every rust and typescript owner reads its files and directories), so 4775 projects are 4 to 6 core-hours; on 10 idle cores 25 to 35 minutes, on this machine's real load far more.

## What would get the 10 minutes (not done)

1. Do not read what the replica already knows: key the per-file observations (`file`, `digest`, `analysis`) by the sha256 of the replica manifest (package F's `🪪️graph-state` reads all 106k hashes in 0.63 s) and keep analyses in a content-addressed store shared through the fleet cache, so a cold owner needs no `openat` for an unchanged file. This is the only change of the order of magnitude, and it is a design change of the observation layer (`👁️observation`), not a patch.
2. The fleet seed: one completed graph (about 90 minutes at 0.8 per second, less with item 1) published once; every other machine restores it (11 s + verification). The exchange is now authorised (item 6 above); it publishes when a build completes.
3. A parent heap bound at launch (`BUN_JSC_forceRAMSize` in the native launcher) instead of the periodic collection; the door runs in the launcher's process, so only the launcher can set it.

## (5) The peer's run and the fleet publication

Pid 60403 did not finish: it was SIGTERMed by its owner at 1168/4775, so nothing was published. No graph run completed on this machine; `<store>/v1/fleet/` holds no graph entry. With the fix of item 6 the first completed graph publishes under the F state key (`publishGenerationV1`, no per-machine counter); not run end to end.

## (6) The three times

Not measured: (a) cold local is now known to be hours (above); (b) and (c) need a completed graph, and none could be completed while the other agents kept the machine at load 18 to 37 and the disk at 97 percent (`/` fluctuated 23 to 42 GiB free around the 97 percent write limit: two of my starts were refused with "Partition / has only 23 GiB free ... Free space with `repo cache cas gc`"; the CAS holds 0.6 GiB, so the space belongs to others and a gc cannot help). The per-command overhead before the graph work is 7.7 s after the receipt fix (section 2 above) plus the source admission of about 30 s, so (c) cannot be under 5 s until the resident graph host or an in-process cache answers.

## Unverified

A graph completed end to end with these changes; the publish to the fleet; the effect of the parent collection on rate (it bounds memory, rate unchanged); the follow-up asks on an idle machine (here they were granted over 3 minutes); Windows and Linux.

---

# 2026-11-14 05:00: per-project records through the fleet, one cold graph to completion

## What was built (landing 4 in `landings.md`)

- The seam is the existing project record: `observeInferenceComputationV1` already writes, per project, a record = key digest + every observation row the inference made (`file`/`digest`/`analysis`/`readdir`/`lstat`... with their facts) + the result. Those rows ARE the opened-path closure (item 2: no system tracing and no new read hook is needed; the layer that does the reads records them). A record is valid when the digest equals and every row holds against the tree (`current`/`sharedObservationsHoldV1`: sha256 comparison when stamps differ).
- Sharing = moving that record. `👁️observation`: `exportProjectRecordV1` / `importProjectRecordV1` (expanded rows, no block files) and a port. `🪪️graph-state/📦️project-records` (schema first, port, 4 tests): state = sha256(root-neutral key text, root-neutral authority rows, platform, project) with **one slot per project** (package C supersession drops only that project's older records); document = the portable record with the root replaced by the token (package F's `neutralizeRootV1`/`expandRootV1`); lookup order local record -> fleet entry (signed fetch) -> claim and compute -> publish. Claims, long-poll wait, takeover and timeouts are package F's `decideGraphEntryV1`/`publishGraphEntryV1`, so eight machines cold at once split the projects. A local hit that was never published is offered (marker file beside the record), so even an interrupted run leaves its finished projects in the fleet.
- Law (item 2): a seeded record is only a candidate. The seam looks it up again against THIS tree (`if(gate?.seeded)reused=await lookup()`), and a tampered/unusable import ends in compute. Test: source law for that order + 40 random edits of a file inside the closure (always refuted) and outside it (never refuted), 20 each.
- Port tests (real cache server, three machines at three roots): record taken at another root has that root's paths and none of the producer's; second machine asks while the first computes -> it waits and takes the stored record (`waited` 1); a third takes it from the cache; dead cache / throwing import -> compute locally; an offer is stored once and remembered.

## Seed from `/Users/ueli/Documents/semio` (item 3): skipped

Attached its graph image read-only (`hdiutil attach -readonly`, detached again; nothing written, no `repo` run there). (1) Its per-project records are gone (`inference/contexts/*/np` is empty, 0 files); only the generation documents of Nov 12 remain. (2) Those documents are verified by the implementation digest of the graph code, which changed many times since (including by this ticket), so a restore would re-infer every owner. (3) The tree differs by two days of edits of eight machines. Coverage by the seed: 0 of 4775 projects.

## The cold graph (item 4)

Preconditions checked: no other graph command, no dependency install, free disk 46 GiB at the start of the final run (23 to 47 GiB over the night; two earlier starts were refused by the 97 percent write limit, `repo cache cas gc` frees nothing: the CAS is 0.6 GiB).
- Runs g1..g11 (log in the session scratchpad `M/`); defects found and fixed on the way, each with a re-run: admission blocked by other jobs' core bookings (section above), a dependency command failing on `CLAUDE.md -> AGENTS.md` (ENOTDIR, twice: workspace walk and receipt check), the generation exchange answering 401, workers recycled after every job (budget 1024 against 1100 used; now 1536 and recycle at the budget), the parent heap (collected every 16 projects; 4 to 7 GB instead of 15 to 20 GB at 2000+ projects).
- Final run g11 (the first one that finished): **53 minutes wall (03:07 to 04:00)**, 4775 of 4775 projects, 8 farm workers, 44 worker restarts in 4775 projects (was one per 11 s with one worker), rate **2.0 to 2.2 projects/s** from project 1800 on (was 0.5 to 0.8). About 1600 of the 4775 were already local records of the earlier interrupted runs; the other 3200 were computed in this run: 3200 / 2400 s of inference = about 1.3 per second overall. End: "generation 0 (5100 projects) uploaded to the remote cache, 5104 documents (47 MB) in 15 s".
- Summary line of the run: **projects from the fleet 2 taken, 3534 stored, 141 offered, 3714 computed here** (computed counts the claims taken, including those computed by farm workers); from the seed 0.
- Peak of the parent: 21 GB at the end of stage 1 (expansion), 4 to 7 GB during inference.

## The three times

- (a) cold local: **53 min** (1600 projects of the 4775 were local records of earlier partial runs; a really empty store is the sum, about 75 min at the same rates).
- (b) fleet hit, local generation and local project records deleted by hand (the supported `repo reset` fails with `Unknown closure reference publication: directory-stamps.json`, found tonight, not fixed): `generation 32 (5099 projects) restored from the remote cache, 5103 documents (48 MB) downloaded in 15 s`, then only 48 stale owners were left; they were re-inferred (`1 taken, 38 stored, 48 computed here`); the run did NOT finish in the 20 minutes I gave it, because three further rounds of the same 48 to 49 owners were stale (see below), so the time to first answer is **above 20 minutes, restore itself 15 s, verification of the restored generation under 3 minutes**. Per-project seeding without a generation (records deleted, generation kept away): 221 projects in 3 minutes (1.2 per second, the parent's single thread: import, brotli, row verification), i.e. about 60 minutes for 4775, not 30 s: whole-generation restore is the path for a cold machine, the per-project entries are the path for a machine that is behind.
- (c) warm: **not obtainable on this tree**. Every warm run (w1 22 minutes cancelled, w2, h2) sat in the same loop: 48 to 49 owners (`semio-s-artifact-norm-*`, `@semio-tech/framework-rs`...) are "stale: the source analysis of ⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts changed" because the rust crates' inference closure contains that script, which peers B/E edit every few minutes, and further `…/🤖️generated/…ts was added` files arrive from other machines; each settle round re-infers and meets the next edit. Not a defect of the new code; a consequence of a live tree whose inference closure includes a hot file. Remedy for the owner: keep tool scripts of the build system out of the rust crates' inference closure (the closure comes from `native-manifests` reading `📜️script.ts` of the unit-cache) or accept a delta round per edit.

## Defects found and not fixed

1. `repo reset`: `Internal error: Unknown closure reference publication: directory-stamps.json` (after an interrupted graph run).
2. Warm graph never settles while the unit-cache script is edited (above).
3. Seeding rate of one parent thread: 1 to 1.5 projects/s; the next step is to seed in the farm workers (they are idle during a fleet-hit) or to import with the manifest instead of reading every opened file.
4. The parent's heap bound still depends on the launcher (`BUN_JSC_forceRAMSize`); the periodic collection brought it from 15 to 20 GB down to 4 to 7 GB during inference, not at the booked 4 to 5 GB.

## Unverified

Eight machines cold at once (claims tested with three ports against a real server, not with eight machines); the effect on Linux/Windows; the second machine's real run against these 3714 entries (the same `repo graph` on another machine); the fleet-hit completion time (above 20 minutes, not finished).

---

# 2026-11-14 06:00: why the fleet-hit tail is long, what was fixed, what was not

Measured on the live tree with a timestamping pipe (`M/ts.py`, logs `t1..t3.err`), `repo graph`, fleet generation present, load 7 to 20, 68 to 77 GiB free. None of the targets (fleet hit under 60 s, warm under 10 s) is reached. What the 48-project tail really is:

## Timeline of one fleet-hit run (t1, seconds from start)

| s | what |
|---|---|
| 0 to 11 | boot, admission (4.1 GB, immediate: package R's rule did not make me wait) |
| 11 to 93 | FIRST verification pass of the local generation 35: membership 2 s, classify 5 s, facts 18 s, **input closures 56 s** |
| 93 | the pass finds the local state stale, the builder attempt restores generation 36 from the fleet (15 s) |
| 93 to 226 | SECOND full verification pass of the restored generation (133 s): a restored generation has no change-journal cursor ("No verified change journal cursor is recorded ...; verifying the tree once") |
| 226 | `refreshing 2093 projects`, `33700 stale facts (analysis 32972)`: `✏️s/bun.lock` and `♻️mit-bestand/bun.lock` changed, which stales the analysis facts of 2093 projects, 49 of them need real inference |
| 230 to 505+ | 48 to 49 projects inferred, but the two heaviest owners take 50 and 60+ s each (`framework-os-kernel`, `framework-os-dev`, 3600 rows) and run late; the next settle round starts when a peer edits again |
| then | generator producers stage (minutes) |

So the tail is not 48 projects at 2 per second. It is two verification passes of 80 to 130 s, a refresh of 1900 to 2100 projects' facts whenever a shared input changes (bun.lock, a Cargo.toml of the norm crates, and the files I edited myself: the process module `host-capacity` and `📜️script.ts` of the unit cache are in the inference closure of 48 to 1900 projects), two heavy owners of about a minute, and further rounds because the replicated tree keeps changing. Package R's reservation (4.09 GB at admission) did not queue the run.

## Fixed and landed (landing 5)

- One verification pass instead of two when a peer is ahead: `exchange.ahead` (the restore's own decision, one short request) lets the shared attempt return before it verifies; the builder attempt restores first. Tests: remote-exchange 7 pass (new: ahead true before restore, false after, false for a dead cache, order law), generations 30 pass.
- `repo reset`: the first cause (`directory-stamps.json` unknown to the closure proof) fixed. A second cause appears behind it and is NOT fixed: `Undeclared compiler-captured physical Source read refused: ...🧬️schema/🗑️reset.json` (`graphDeclarationV1` in `📜️contract` reads it, but the reset kernel's captured closure (1966 files) does not declare it; the declarations come from the closure compiler's trace, owner of that: package J / `📥️source/🔏️closure`). No test added for reset (the failure is in the capture, not in the role map).
- Tried and reverted: derive concurrency 8 -> 32 made the closure verification slower (61 s -> 99 s): it is CPU-bound in one thread.

## Not done, with the reason

1. Batch lookup route, verification in workers, bulk install (item 2): not started; the measured floor of a fleet-hit is the verification (about 80 s per pass), not the seeding; seeding one by one stays 1 to 1.5 projects/s on the parent's thread.
2. Second scratch checkout proof (item 4): not done. A checkout of this repository is 273k files and cannot boot `repo` without an installed dependency generation (package F's finding), so the second machine's "taken vs computed" ratio is unproven; what is proven: a project record published by one port is taken by another port at another root (4 tests) and the cold run published 3534 + 141 records.
3. Warm under 10 s: the verification alone is 80 s; it needs the change-journal cursor of the replica daemon (so a verification is O(changed)) for restored generations and for a generation installed by `restore`. That is the next step and belongs to the generation store, not to the exchange.
4. Publication under churn: machines publish whenever a build completes (`publishRemoteGenerationV1` after every build, `ifAbsent` for reads). The tail redoes only the stale owners (1 of 1 verified: the restore of generation 36 re-inferred 49, not 5100); the cost is the verification passes and the heavy owners, listed above.

## Three times (this tree, with the moving edits)

- cold local: 53 min (earlier section).
- fleet hit: restore 15 s; verification 80 to 130 s per pass; first answer after one pass plus the stale tail: about 150 to 250 s when the tree is quiet enough to settle, otherwise it does not settle (rounds of 5 min); the 60 s target is not met.
- warm (nothing changed): verification only, 80 s (the journal cursor is missing after every restore); target 10 s not met.

## Can a 16 GB worker run the graph step from the fleet within package R's rule?

No. In the fleet-hit path the parent alone reports 11.9 GB resident during the verification (graph-phase rows `rssMegabytes` 11883 to 11888) and 21 GB at the end of stage 1 (expansion), before any worker (1.5 GB each) starts; R's admission books 4.1 GB for it. A 16 GB machine therefore cannot hold the parent plus one worker without swapping. The parent's footprint is the blocker (garbage plus the 700k-fact index), not the workers; bounding it needs `BUN_JSC_forceRAMSize` at launch and a streaming verification of the fact index.
