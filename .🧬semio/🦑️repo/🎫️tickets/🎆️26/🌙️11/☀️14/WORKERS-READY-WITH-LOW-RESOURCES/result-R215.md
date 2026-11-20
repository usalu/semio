# R215: does the fleet's form-3 publication get adopted? (living report)

Started 2026-11-18 18:02 UTC. Sampler: scratchpad `R215/s.py` (R211 sampler plus reasons, pools, fleet cache entry count); raw samples `scratchpad/R215/samples.txt`. Scripts of the landing: `FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R215/` (`land.sh`, `land.tsv`). No download, no slice-lab run, no pkill, no sub-agent.

## 1. Pool restart on form 3 (checked 18:02 UTC)

Every pool role restarted by itself right after the 18:00 landing and the 18:01:24 interrupt: `startedMs` of `g1-m01`, `g1-m02`, `g1-m03`, `g1-m04`, `g1-m07`, `g1-m08`, `g1-m10`, `g1-server` between 18:01:31 and 18:01:40, counters reset to 0 and climbing (the old counters 57 to 84 were gone). There is no form marker in `pool.json`; the proof is the reset counters plus new `failures` code `leaf-missing` (a code that exists only in form 3). Not restarted: `g1-m05` (started 17:27, `pool.json` not updated for 9 minutes at 18:02, updated again later: it keeps running on its own cadence, its children do show `leaf-missing`, so it runs form 3 code) and `g1-m06` (`pool.json` of 2026-11-16, stopped for two days, nothing in its `pool.json` newer; no `service-failure*.txt`). `g1-m09` has no pool file at all. g1-m01 and g1-m02 pools are running now (not stopped): their `service-failure*.txt` are old (m02 `d9b6a3be` 08:43 UTC "Slice worker exited with code 124", others of 2026-11-17/18 00:27 "Build source authority keeps changing during inference"); nothing in the last 9 hours.

## 2. The sampled read (read of 18:01:34 UTC, runner 18668, no interrupt used; 90 minutes, 18:02 to 19:32)

Sampler `scratchpad/R215/s.py` (every 2 to 3 minutes; raw `samples.txt`, pool series `pools.txt`). The read reached the owner phase at 18:03 with 235 of 4785 records already present. "stale owners" = distinct owners with a `does not hold here` line (the log's own stale counter shows 1 to 4 because a stale owner that is inferred afterwards counts as inferred); "incomplete" = lines `leaf pieces are not in the fleet cache` (R214's typed case, 17 owners, 99 pieces at the end; the ledger shows no counter for it).

| UTC | done of 4785 | present | fetched | inferred here | running | stale owners | pools claimed / published (sum, since 18:01:35) | fleet cache `ac` entries | load |
|---|---|---|---|---|---|---|---|---|---|
| 18:05 | 18 | 18 | 0 | 0 | 15 | - | 252 / 77 | 47148 | 21 |
| 18:15 | 77 | 48 | 6 | 23 | 20 | 59 | 2031 / 1173 | 48963 | 20 |
| 18:27 | 98 | 50 | 15 | 33 | 21 | - | 2465 / 1437 | 48420 | 24 |
| 18:38 | 122 | 62 | 16 | 44 | 27 | - | 2720 / 1604 | 48474 | 23 |
| 18:50 | 159 | 63 | 18 | 78 | 24 | 75 | 3184 / 2002 | 48872 | 16 |
| 19:01 | 245 | 64 | 18 | 163 | 25 | 107 | 3612 / 2393 | 49308 | 18 |
| 19:12 | 335 | 64 | 18 | 253 | 25 | 111 | 3545 / 2396 | 49507 | 17 |
| 19:24 | 438 | 64 | 18 | 356 | 25 | 115 | 3679 / 2518 | 50643 | 15 |
| 19:32 | 522 | 65 | 22 | 435 | 26 | 130 | 3828 / 2646 | - | 17 |

Owners per minute (10-minute windows): 18:15 about 5, 18:27 2.6, 18:38 2.1, 18:50 3.3, 19:01 7.5, 19:12 8.2, 19:24 8.5, 19:32 11.4; average from 18:12 to 19:32: 5.8. Projection: 4263 owners are left at 19:32; at the last window (11.4 per minute, cheap owners first, so it will fall) 6.2 hours, at the average 12 hours. NOT before about 01:30 UTC and probably much later; the read did not finish in this window (task 4: nothing to record, no sealed generation, `repo show projects` not run).

The take pass (the 4658 look jobs) ended at about 19:26: only then the ledger says `4324 wanted from the fleet`.

## 3. The number: adoption per 100 taken

Taken = fetched 22 + stale owners 130 + incomplete owners 17 = 169. Adopted 22: **13 per 100 taken** (9 per 100 at 19:01: 18 of 125 + 0; it was 100 per 100 in the first 5 samples, which is a sample of 6). Where the 147 refused ones come from:

| reason | owners | what it is |
|---|---|---|
| `file` row of `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/📦️packages/🦀️rust/Cargo.toml` | about 105 (105 lines, flat since 18:44) | a REAL edit of the tree at 18:01:24 UTC (size 2025 to 1897, sha `0d11..` to `bfe0..`); the hub-*-rs family names it as an input. Judged against the tree here, not a judgment of context. |
| `file` row of `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/📦️packages/🦀️rust/Cargo.toml` | 40 (11 at 18:44, 40 after the second edit at 18:48:44 UTC) | a second real edit |
| `file` over `🏃️process/🔒️leases/📈️standing/🟦️.ts` | 2 | a third edit |
| directory row `mounted-owner/.../rust` | 1 | |
| `leaf pieces are not in the fleet cache` (incomplete, typed) | 17 owners, 99 pieces | entries whose leaves nobody has (a producer without the leaf upload, or a blob the cache evicted); the reader infers and republishes them |
| `analysis`, `closure`, `exists`, `whole` rows | 0 | none in the whole read |

Real edits: three paths in 90 minutes, two of them made 105 and 40 owners stale: about 100 owners per hour are stale by real edits at this rate. At 5 to 11 owners per minute that is 20 to 30 percent of the rate and does NOT by itself prevent convergence, but a record adopted early can go stale later (not counted: this read does not look again at a decided owner).

## 4. Why the read adopts so few although the fleet publishes (measured)

- The fleet publishes: 2646 records published by the pools in 90 minutes (30 per minute in the first 35 minutes, 19 per minute afterwards), 1094 distinct owners listed as stored in the fleet by pool files born after the restart (the hot list of the reader), 4785 owners in total.
- The read decided 522 owners. Of the 439 records it wrote locally since 18:01:35, 259 (59 percent) are for owners that the fleet lists as stored: the reader infers locally what the fleet already holds. Only 22 were fetched.
- `pool-published-<wave>.txt` lists only owners that a claim found ALREADY stored (`stored` of `pullOwnerPoolV1`); an owner a pool published itself is not listed (checked: lines in the post-restart files equal the `stored` counter of `pool.json`, 1326 of m03 vs 1326, never the `published` counter 387). Defect of the listing, not landed (a one-line `stored` call after a successful publish in `🌐️owner-pool/🟦️.ts` `execute`; it adds up to 2000 names to the list, but the list already holds 1094 names of which the read fetches 22, so it does not explain the gap).
- The look is the slow step: `[owners] owner looks: 4785 owners in 4658 farm jobs of up to 6 owners for 9 children`; heavy (native, release) owners are looked at one at a time after all light ones (`takeSplitV1`, `takeFetchFirstV1`), a look of a native owner computes the assembly authority (minutes under load: `os-hub` was parked 2335 s "waiting for a farm child to look at the owner: assembly authority"); looks are admitted by the same memory gate as inferences (`takeAdmittedV1`; `waiting for memory headroom (3655 of 7697 MB reserved)`); the farm showed `a job runs alone and the others wait` for a verify. The read has 8 farm children, 5 of them busy at 60 to 80 percent of a core, the machine load 15 to 24 on 10 cores. NOT measured: the outcome counts of the poll's looks (miss, stale, fault) and the seconds per look; no counter exists for them (the ledger has no `looks`, `miss`, `fault`). Recommendation for the next landing: add `looks/found/miss/stale/fault/incomplete` to the poll and to the ledger line; then decide between (a) pools publishing `name digest` lines so the reader fetches by key without a look (R204 section 2 (b)), (b) a reserved farm child and memory share for adopt looks, (c) skipping a local inference when the fleet lists the owner (59 percent duplicate work).
- Judging cost: not measured as seconds per record; the look children judged 7 to 319 records each, stale owners grew 107 (19:01), 111 (19:12), 115 (19:24), 130 (19:32): about one judged-and-refused record per minute, the same order as the fetches, so the poll takes roughly 1 to 2 records per minute out of a list of 1094 names.

## 5. The fix of this window (landing 18:30:01 to 18:30:35, see landings.md "R215 landing")

Top reason on the PRODUCER side, measured from `pool.json` failures at 18:30: `leaf-missing` on 511 of 738 claimed (g1-m01), 194 of 241 (g1-m07), 95 of 187 (g1-server), 53 of 205 (g1-m05); g1-m02 2, g1-m08 6, g1-m03 22, g1-m10 24. Cause found in `🌐️owner-pool/🔬️inference/🟨️.mjs`: the child infers into its own staging directory (`<staging>/build-input-closures`, `build-lock-closures`) and removes it after the publication, but R214 gave its fleet port the store of the machine as leaf source, so every owner whose leaves the machine did not already hold (all the new ones on a machine with a young store) was refused as `leaf-missing` and never reached the fleet; the machines whose store already held most leaves (m02, m08, m03, m10) hid it. Fix: `stagedLeafPortV1` (leaves first from the staging directory, then from the store) and `ownerPoolFleetPortV1` (used by the driver). Laws: leaves tests 10 (2 new), new `owner-pool/🧪️tests/🍃️staged-leaves` 2; red when the staging lookup is removed (2 and 1 fail), control: the bare port is refused `leaf-missing`. Effect measured: the pools load the driver at every child start, so the new code reached each pool as its children were recycled (lifetime 30 minutes or memory): `leaf-missing` of g1-m01 stood at 511 from 18:36 (0 new in 56 minutes, 900 new in the 28 minutes before), g1-m07 194, g1-server 95, all frozen; the same pools published 1059 records between 18:36 and 19:32. Pools did NOT restart for this landing (all `startedMs` 18:01:31 to 18:01:40): the driver is not a declared constant, new children carry it. No read interrupt used.

## 6. Pools (task 3)

Restart on form 3 (task 1): see section 1; all running pools restarted within 9 seconds of each other at 18:01:31 to 18:01:40 (the interrupt of the old read was at 18:01:24; the landing at 18:00:50), counters reset, so the roles do pick up the form bump; the mechanism (declared constant watch) was not proven here, the observation is the restart itself. g1-m05 restarted again at 19:08 on its own.

| origin | claimed | published | stored | failed | children/target | owners per minute | state at 19:32 |
|---|---|---|---|---|---|---|---|
| g1-m01 | 1042 | 481 | 296 | 514 | 4/0 | 3.4 | memory pressure, swapping 127 MB/s out (16 GB) |
| g1-m02 | 470 | 439 | 175 | 6 | 5/4 | 0 | no idle core (0 of 10) |
| g1-m03 | 580 | 493 | 1360 | 29 | 7/9 | 7.6 | starting children, 178 s pacing |
| g1-m04 | 10 | 8 | 11 | 0 | 1/0 | 0.2 | Linux, memory pressure, swapping |
| g1-m05 | 11 | 4 | 9 | 0 | 6/16 | 1 | restarted 19:08 |
| g1-m06 | 2 | 2 | 1210 | 0 | 1/1 | 0.2 | `pool.json` of 2026-11-16 10:41, no process since, no `service-failure*.txt`: nothing restarts it |
| g1-m07 | 420 | 170 | 645 | 207 | 1/1 | 0.2 | free memory 503 MB, next child needs 870 MB (16 GB) |
| g1-m08 | 457 | 418 | 1068 | 10 | 8/10 | 3.6 | |
| g1-m10 | 570 | 473 | 1033 | 28 | 6/10 | 1 | |
| g1-server | 272 | 164 | 21 | 96 | 5/5 | 1 | no idle core (the read and the pools share this machine) |

The pools are limited by their own machines (memory floor of 2048 MB, cores, pacing between child starts, swap) and no longer by `leaf-missing`; g1-m09 has no pool file at all. g1-m01 and g1-m02 were NOT stopped at 18:02: both restarted with the others (their `service-failure*.txt` are old: 08:43 UTC "Slice worker exited with code 124", 00:27 UTC "Build source authority keeps changing during inference").

## 7. Not verified

- Outcomes and latency of the poll's looks (no counter); the cause of the gap in section 4 is a ranked hypothesis, not a proof.
- Whether the leaf uploads of the pools reach the fleet cache blob store at the expected rate (published counters rose, the blob store was not walked; a sample of 300 leaves of this machine found none under `v1/blobs/<2>/<digest>` or `v1/main/ac`, so the blob layout of the leaves is unchecked).
- The code of the 18:30 landing in the pools of g1-m05 and g1-m06 (m06 does not run).
- No whole read finished; nothing sealed; `repo show projects` not run.
