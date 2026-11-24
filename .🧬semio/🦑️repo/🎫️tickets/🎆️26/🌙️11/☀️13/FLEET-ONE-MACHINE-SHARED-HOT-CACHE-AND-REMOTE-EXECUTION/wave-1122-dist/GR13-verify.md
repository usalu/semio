# GR13: red test files of GR12 attributed, graph-read effect measured (2026-11-23, 18:35 to 18:50 Mac clock)

`I` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/`. Lock `lock.graph-closure` taken and released. Landing in `T/landings.md`.

## 1. Red test files (each run once with `repo test-file`)

| file | result | cause | verdict |
|---|---|---|---|
| `I/🧭️script-inputs/🧪️tests/closure-answer-refresh.test.ts` | 5 pass, 1 fail | regex expects `await withClosurePassV1(() => observeInferenceSnapshotV1(...` in `I/📦️project-nodes/🟨️.mjs`; the source reads `observeOwnerAttemptsV1(round => withClosurePassV1(...` | pre-existing |
| `I/🧭️script-inputs/🧪️tests/🫧️take-release/🟦️.ts` | 1 pass, 2 fail | (a) source-text test slices from `const look=job=>{` which no longer exists; (b) real-process memory test, 1,069 MB against a 224 MB bound | pre-existing |
| `I/🧪️tests/🔒️lint/🟦️.ts` | 1 pass, 1 fail | module-level Maps | 2 of 8 caused by this wave (fixed), 6 pre-existing |

Evidence:
- closure-answer-refresh: `git log -S'observeOwnerAttemptsV1(round => withClosurePassV1'` first appears in commit 01324745f80 (2026-11-22 13:42), and the same line is in `HEAD:` of `project-nodes/🟨️.mjs` (line 360). The test file was last changed 11-15 (efff7f986f5). The wave's diff of `project-nodes/🟨️.mjs` (GR7 lookahead, GR12 `{reask:true}`) does not touch that statement. The assertion has failed since 11-22.
- take-release (a): `git log -S'const look=job=>{'` shows the string was removed by 5db034ffe53 (2026-11-20 17:40); HEAD already has `const look=(job,lane="pass",signal=waveSignal)=>`. (b): `releaseOwnerStateV1`, `lookReleaseDueV1`, `releaseAfterTakeLookV1`, `managedBytesV1` are byte-identical in HEAD and the working tree (md5 of the extracted functions f56464531b9f...), and `lookReleaseGrowthBytes` is 134217728 in both. The release became growth-driven in 5db034ffe53 (11-20); the test (last changed 3cde99d423d, 11-18) was written for the count-based release and was never updated. Not caused by GR3/GR9/GR12.
- lint: the failing run listed 8 names. At HEAD, six exist already: `readContractsV1` (01324745f80, 11-22), `stagesV1`, `running`, `preflights`, `verdictDirsMadeV1`, `closureFleetLanesV1` (all 3cde99d423d, 11-18). Two are NOT in HEAD and are from this wave: `aheadMemoV1` (GR7, `project-nodes/🟨️.mjs`) and `importRefusalsV1` (GR9, `script-inputs/🟨️.mjs`). GR12's earlier added Map was already replaced by a bounded cache.

Fixes landed (product, not assertions; the lint contract says long-lived state uses `createBoundedCacheV1`):
- `aheadMemoV1` is `createBoundedCacheV1({name:"inference:ahead-jobs", maxEntries/maxBytes: lookaheadMemoMaximum})`; the manual `size>=max -> clear()` is gone (the cache evicts the oldest instead of dropping all, which is also better for the lookahead memo); import added.
- `importRefusalsV1` is `createBoundedCacheV1({name:"inference:import-refusals", 4096})` (`IMPORT_REFUSALS_MAXIMUM_V1` and the manual eviction loop removed); `.d.mts` type updated; the test uses `.stats().entries` instead of `.size` (the bounded cache has no `size`).
- After: `🚫️import-refused` 2 pass 0 fail; lint now lists only the 6 pre-existing names (1 pass, 1 fail); `project-nodes/🟨️.mjs` imports (62 exports).
- Not touched (pre-existing, outside this wave): the three stale assertions above and the 6 Maps. Each needs its own decision (update the regex/slice anchor and the memory bound for the growth-driven release; bound the 6 Maps).

## 2. Graph-read effect

Which read: the read of 16:53 (started with the old journal code) died of `Out of memory (reference 534390f9)` at 18:15 after "all project results are in, storing 459 project fragments" (heap 3.0 -> 12 GB), exactly the GR8 prediction: `read.1795454144134.log`. Its final tally: wave 1 `132 of 132 done: 65 present, 31 fetched from the fleet, 36 inferred here` (GR7 effect: 31 fetched versus 0 before; the OOM is the journal bug fixed by GR8, loaded only by the next read). No head or generation was published.

The first read that started after that: runner 16216 / read 16220, started 18:25:08 (`read-record.json`), log `.cache/f/graph-server/read.log`. Whether it loaded the GR12 module cannot be proven: GR12's test file is dated 18:26 and its probe 18:29, i.e. it started 1 to 4 minutes before GR12 finished. `repo fleet status` says "code current" for it. It loaded GR7, GR8 (journal), GR9 and, very likely, GR3. So the numbers below are for the read after GR7/GR8/GR9, GR12 unconfirmed.

State at 18:49 (24 min in; the owner phase is wave 1 of 5,222 owners, not done; I did not start or cancel anything):
- Wave 1: 88 of 132 done (71 present, 1 fetched, 16 inferred here, 8 running, 4 stale in the fleet). Slower than the previous read's early wave (fetched 31 by the end there, 1 here so far): the fleet publishes the owners of the current wave only after the coordinator's wave starts, and the fleet is busy with the workspace stage.
- `adoption:` line at 65 of 132 (the only one printed so far): `80 owners listed as stored in the fleet, 93 looks, 55 adopted (1 fetched, 54 present), 14 stale (closure 4), 13 missed, 8 faulted (unavailable 8); 14 owners inferred here although the fleet lists them`. Same measure as before (adopted / (adopted + stale)): 55/69 = 80 % against 98/140 = 70 %; stale closure 4 against 24; 14 listed-but-inferred against 18 (partial, one wave only, not comparable in absolute numbers: 80 listed against 87).
- Closure re-asks (GR12): 0 `closure question asked again` lines and 0 `closure-entry-stale/absent` lines. The stale looks of this read are `kind closure` rows with `["generated"]` or `["some", [..._carg...]]` scopes (owners `architect-plugin`, `norm-energy-plugin`, `norm-eurocode-structural-plugin`, `norm-eurocode-plugin`: recorded `4f53cda18c2b`, fact `[]`): the machine-local generated-state rows of the GR8 finding, not closure ENTRIES, so GR12 would not act on them even when loaded. No evidence yet for GR12 either way; re-asks seconds: 0 s.
- Owners per minute fleet-wide (GR8 `rate.py`, 4.0 min, 12 pullers alive): 25 published, 39 claimed = 6.25 owners per minute published (GR7 after: 5.7, before 2.4). Sampled `pool.json`: 34 slots, 7 children, running 1 to 10 (30 s samples 5, 6, 8, 9, 8, 6, 6, 10 running, children 12 to 26 of 22 to 31 slots, i.e. about 25 to 45 percent busy slots), `ownersPerMinute` sum 1.4 in the one-shot sample; four pullers `pressure` or `nothing-fits` (m10, m11, m13, m12), `g1-m04` `pulling` with target 8 but 0 claimed. Fleet load 40 to 50 on the coordinator. No tier 1 unit seen yet (the wave has 132 owners, below one wave of cold work).
- Memory of the heavy child (pid 17235, the owner-phase process of the read): 2.4 GB at 18:39 (grant 3,072 MB), 3.0 to 3.6 GB RSS 18:40 to 18:47 (3,137 / 3,378 / 3,208 / 3,145 / 3,598 / 3,513 / 3,565 / 3,225 MB). Heap census 1.9 to 2.3 GB, `the process held 2.3 to 2.5 GB of its grant of 3,072 MB`. Over the grant, but stable, no OOM, no stall; the previous read's blow-up came after wave 1 (storing fragments), which this read has not reached. The GR8 journal-memory fix is therefore not yet proven live.
- Stalls: none; one early `shared closures N/49 ... parked: inferring in this process` phase (133 to 223 s with 0 rows, wave start), then wave 1 progressed 28 -> 88 owners in 10 minutes (about 6 per minute).
- Head/generation: not published yet (read still in wave 1 of the first 5,222-owner pass; `status null`, `endedMs null`).

## 3. What to check on the next completed wave
- `all project results are in, storing N project fragments` followed by `indexing` (no OOM at about 5,000 s).
- Final `adoption:` line of wave 1 and any `closure question asked again` line (proof that GR12 is loaded and acting).
- Tier 1 units in `pool.json` once a wave has more than 8 open owners.
- Worker stage `dependencies` of m04 (disk 5.1 of 6.1 GB free) and m05 (attempt 31) still stuck: unchanged from GR8.
