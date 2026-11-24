# GR7: fleet-wide owner inference (2026-11-23, 16:35 to about 17:10 Mac clock)

`B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`, `G` = `B/🕸️graph/`, `POOL` = `G/📥️inference/📦️project-nodes/🌐️owner-pool/`. Lock `lock.graph-serve` taken and released. Landing in `T/landings.md`.

## Verdict
1. The mechanism the task asks for already exists and runs: the **owner pool** (`POOL`). A read publishes its open owners to the fleet cache, every machine pulls them, claims each by the digest of its record state (heartbeat, lapse returns the owner to the pool), infers in a memory-bounded child, publishes the record, and the reading machine takes it as "fetched from the fleet". Admission is per owner and by measured spare memory (`ownerPoolSizingV1`: floor, held memory of the machine's own work, measured child peak = p75 of the last 32 children + 15 %), not by `serveTotalMegabytes`. All 13 machines run a puller (live `pool.json`, 12 alive pulling or idle). The 24 GB threshold only gates the whole-generation refresh and the wanted-board whole read on the coordinator; it does not gate owner serving. So I did not add a second mechanism and did not touch `serveTotalMegabytes` (a whole read still needs a big machine; owner serving does not).
2. Why the fleet nevertheless infers 2 to 3 owners per minute: the pool only ever sees the owners of ONE wave (43 units, 112 KB), because the whole read works through the stale owners in waves of 48 (growing to 960 only after waves that infer nothing). 13 machines have 70 to 84 slots, 9 to 10 children run (live, 16:42 and 16:53), most pullers are `idle` or `starting` with nothing to claim: the fleet starves for lack of visible work, the read is bound by the slowest cold cargo owner of each wave.
3. Landed: **lookahead + tiers + wave priority** (all schema-declared):
   - A wave that is cold (at least `announceMinimumOwners` = 8 open owners) also tells the pool the next `ownerAhead` = 480 stale owners of the read in the read's order (priority owners first, because `orderOwnersV1` already puts the wanted-board and steering priority first) as **tier 1** units next to its own **tier 0** units. A puller never claims a tier 1 owner while a tier 0 owner is open for it, so what the read waits for, and the selection/priority owners, go first and the whole-graph remainder last.
   - Wave priority: the head carries `priority` (0 = read a command waits for, i.e. selection or interactive read; 1 = background whole-graph read of the graph server, `SEMIO_GRAPH_BACKGROUND=1`); a machine starts its next child on the open wave with the lowest priority, then fewest children, then newest (`ownerPoolWaveOrderV1`). A worker's selection of the Ready `workspace` stage (607 owners, all tier 0 of its own wave) therefore outranks the coordinator's whole-graph wave on every machine.
   - Ahead jobs of crates need `cargoTargets`; they are built once per process (memo keyed by root, command revision, manifest size and time), at most 20 s per wave (`lookaheadBuildMilliseconds`), memo bounded at 4,096 (`lookaheadMemoMaximum`); a failure drops that owner only.
   - Dedupe: `ownerPoolLookaheadV1` (wave first, then ahead, taken and duplicates removed). An owner that is claimed by another machine is skipped by the existing claims (held, retried after its expected time); a dead server's claim lapses with its TTL (`claimTtlMilliseconds` 60 s, heartbeat) and the owner returns to the pool.
4. Item 3 and 4 of the task already hold and are unchanged: the coordinator's own read keeps inferring locally with its farm and takes what the fleet claimed or stored (`takeClaimedV1`, "fetched from the fleet"); records are content-keyed over the digest head (registry digest, command revision, `late`) and the owner JSON, a taken record is verified row by row against local digests (GR2) and a mismatch is read again.

## What changed (file, what)
| file | change |
|---|---|
| `POOL🟦️.ts` | unit `tier`; `pullOwnerPoolV1.pick` walks tier by tier (heavy-first and random-cursor walk inside a tier, one cursor per tier, gallop re-places all); `OwnerPoolHeadV1.priority`; `ownerPoolWaveOrderV1` |
| `POOL🧬️schema/🔣️.json` | `Unit.tier`, `Head.priority`, Policy `lookaheadBuildMilliseconds`, `lookaheadMemoMaximum` |
| `POOL🏃️runner/🟨️.mjs`, `POOL📣️publisher/🟦️.ts` | next child by `ownerPoolWaveOrderV1`; publisher passes `priority` into the head |
| `POOL🔭️lookahead/🟦️.ts` (new) | `ownerPoolLookaheadV1` |
| `G📥️inference/📦️project-nodes/🟨️.mjs` | `aheadRoots`, `buildAheadV1` (before `announce`, only for cold waves), units = wave tier 0 + ahead tier 1, publisher priority |
| `G🕰️generations/🟨️.mjs`, `.d.mts`, `🧬️schema/🔣️.json` | `spec.ahead` = next `ownerAhead` (480, EN+DE descriptions) names after the wave |
| `G🕰️host/🟦️.ts`, `G🧭️discovery/🟦️.ts` | roots of `spec.ahead` reach the inference as `aheadRoots` (`partOwnerScopeV1`); never as `ownerRoots` |
Old pullers and children ignore `tier`/`priority` (no compat layer needed): they still see all units; new children pick up the code when they are launched (children live at most 30 min).

## Tests (all `repo test-file`, one at a time)
- New `POOL🧪️tests/🧭️fleet-schedule` (7 pass): Python oracle (`🐍️oracle.py`) computes per machine the slots by spare memory and the expected owner set; TS must equal it and then runs the real `pullOwnerPoolV1` on 1 to 6 machines with a TTL claim store: every owner published exactly once, tier 1 never started while a tier 0 owner is unclaimed, machines without spare serve nothing, a machine killed with owners in flight leaves them to the others after the lapse (each published once by another machine), plus wave order and single-machine tier order.
- New `POOL🔭️lookahead/🧪️tests` (3), `G🕰️generations/🧪️tests/🔭️ahead` (2: every wave names the owners of the waves that follow, in order, last wave none), `G🧭️discovery/🧪️tests/🔭️owner-scope` (1).
- New integration `B🪪️graph-state/📦️project-records/🧪️tests/🤝️pool-serve` (1 pass): real cache server, two serving machines (two store roots, 2 slots each) pull 4 selection + 8 lookahead owners through the real fleet port (claim, publish); a third machine reads all 12 from the fleet with 0 local inferences; the 12 inferences are split between the two servers with no owner twice.
- Existing and green after the change: owner-pool main 32/0, spread 4/0, early-wave 7/0, turn 3/0, lanes 2/0, heavy 5/0; adaptive-waves 24/0; project-nodes take 11/0, pool-admission 4/0, selected-fleet 2/0, away 1/0, known-unit 2/0, read-parent 4/0.

## Live (no restarts by me)
- Before (16:42, old code everywhere): 13 pullers alive, 48 children, 9 running of 70 slots, 2.4 owners/min fleet-wide, all on one wave of 43 units (`before-pool.txt`, `pool-sample.py` in `GR7/`). Per-owner measured child price in `pool.json`: 462 to 1,592 MB (not 4 to 5 GB), grant 4,915 MB on 16 GB machines; m02, m03, m08 refuse with "nothing-fits" because 2,457 MB are held for the machine's own work (H1/H2 area), m13 under pressure.
- 16:53 the coordinator's whole read (5,060 s old) died with "Out of memory" (heap 11.3 GB against a 3,072 MB grant) and the graph server started a new read at 16:53:46, which loads the new generation/host/discovery code at start and project-nodes lazily. Its numbers (ahead publication, owners per minute) are under "After" below.

## After
### Measured by GR8 (2026-11-23 17:08 to 17:16, the read that started 16:53:46 with the GR7 code; `GR8/pool-sample.py`, `GR8/rate.py`)
| number | before (16:42) | after (17:08 to 17:15) |
|---|---|---|
| owners per minute, fleet-wide (records published by pullers, summed positive deltas of 13 `pool.json`, 4 min) | 2.4 | 5.7 (the pools' own `ownersPerMinute` summed: 3.8 to 6.2) |
| pullers alive | 13 | 13 (m05 Windows: 16 slots, 0 children, see GR8 item 3) |
| slots / children / running | 70 / 48 / 9 | 42 to 67 slots (humbler hosts under H2 headroom), 16 to 36 children, 7 to 17 owners running = 13 to 28 % of the slots busy |
| fetched vs inferred (coordinator, wave 1 of the read: 132 dirty owners) | 22 fetched / 14 inferred (earlier read) | 0 fetched from the fleet, 19 inferred here, 62 present at 81 of 132 (17:15) |
| waves in the pool | 1 wave of 43 units | 4 waves of 71 to 87 units, ALL tier 0, no tier 1 unit |
Finding 1: the lookahead has nothing to name in this read. The whole read has only 132 dirty owners (one wave, `inferring 132 projects in this process`, `total = 132`); the 5,093 `stale` projects are edge-only owners that need an edge refresh and a fragment, not an inference, so `rest` is empty and `spec.ahead` is not set. The 480-owner lookahead helps a read with more than one wave of cold owners (a workspace selection of 607 owners after a clean checkout), not this one.
Finding 2 (the limiter now): the fleet infers the cold cargo-plugin owners (norm-*, architect, block ...) and the coordinator does not take them: `adoption: 73 owners listed as stored in the fleet ... 64 adopted (0 fetched, 64 present), 10 stale (closure 16), 8 owners inferred here although the fleet lists them`. The first row that does not hold is a `closure` row (`["some", [...cargo/native-build dirs]]`, `["generated"]` with `recorded X, now 9067b12990aa`): the closure facts of those owners depend on state of the machine that inferred them (which generated or build directories exist), so the record of a worker does not hold in the coordinator's tree. Fleet work for the 10 slowest owners is therefore wasted for this read; not changed here (a closure-row design question, not a point fix).
Extrapolation at 5.7 owners/min and all owners cold: the 607-owner `workspace` selection about 107 min (was 253 min at 2.4/min), the whole graph of 5,222 owners about 15 h (was 36 h). These are upper bounds: a read infers only its stale owners (132 here, about 25 to 50 min with the 15 to 35 min cold cargo tail), the rest is fragments, patch and seal (item 1 of GR8: that step died of OOM at 16:48 and is fixed).

## Open items (honest)
- m05 (Windows, 64 GB, 16 slots) claimed and failed 515 owners with `unshareable` ("the record names the root in a spelling that cannot be shared"): the biggest machine contributes nothing; root cause is the root rewrite on Windows spellings (`neutralizeRootV1`), not fixed here.
- The whole-read OOM at 11 GB (16:53) is a separate heavy-child growth, not touched.
- Pool children are only as free as the machine's own work leaves them (H2): stage-claim holds of 2,457 MB make m02, m03, m08 serve nothing.
- `serveTotalMegabytes` unchanged by decision (see verdict 1); the wanted board stays the priority channel to the coordinator's whole read.
- Not measured: manifest size with 480 ahead units (estimate 2.6 KB median per unit, about 1.3 MB; the cache accepts 3 MB sessions today); the effect on whole-read duration is only visible over several waves.
