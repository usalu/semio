# Result E55 (workspace-stalls), 15:25 CET

Landings: `T/landings.md` sections "Wave 1122 E55" (two). Locks remote-execution and graph-state taken and released. Scratch: `/private/tmp/claude-501/-Users-ueli-Documents-semio/3a99932f-dbc8-44a3-a665-189b4162729c/scratchpad/E55/` (probes `snap.sh`, `sm.sh`), kept inputs in `T/🧰️scripts/E55/`.

## 1. Current cause per worker (15:21, read from `S/origins/<o>/ready.json`, `ready-log-workspace*.txt`, `slice-memory.json`)
The workspace stage is the selected read of the 604-owner worker slice. None of the five is stuck on a fault now; all five are slow, and the slowness has two measurable parts: owner lookups skip the fleet (`unavailable`) and the slice process answers its own requests very late.

| worker | attempt, read clock | owners decided | fleet lookups | what it is doing |
|---|---|---|---|---|
| m12 | 12, 1916 s | 450 of 604 (152 present, 297 fetched, 1 here) | 485 looks, 0 unavailable | fastest: 0 to 450 in 17 min because the fleet answers. Earlier attempts died at the closure proof (E46/E53 fixes; the refusal now names the directory), none has reached that step again yet |
| m01 | 4, 5085 s | 221 (158 fetched, 63 inferred here) | 460 looks, 241 unavailable, 240 metadata calls = 3652 s (15 s each) | slow but moving; step `reading the owners` is past its 3600 s bound (not stopped: it moves) |
| m08 | 10, 2076 s | 117 (72 fetched, 45 here) | 866 looks, **782 unavailable**, 85 metadata calls | breaker open nearly all the time, so owners are inferred cold; `@semio-tech/framework-actor-rs` is a heavy `never` owner waiting for the fleet |
| m10 | 6, 2166 s | 37 (33 fetched) | 194 looks, 115 unavailable; 8 metadata calls in flight with 0 ms completed at the first sample | heavy `never` owner `@semio-tech/norm-din4108-rs`; earlier attempt ended at the hard cap after 3.9 h with the counter stuck at pass 5, 10 of 604 (the heavy wait: fixed by E46 to 30 min) |
| m06 | 2, 3283 s | 247 of 600 completed owner checks | n/a | in the step `checking the file inventory of the tree (217768 distinct source observations)`, the post-read facts check of all observations, about 30 min per round on a 16 GB Mac; moving |

What m11 did differently (Ready since 11:14, `ready-timings.json`): it read the slice over many short attempts while the tree was quiet, inferred its heavy owners itself (peak 5.7 GB), then ran the real task of the stage (`nx build` of norm-din4108-rs, 4 min 23 s) and published the nx graph to the fleet. The other workers have never got past the read in the same attempt; m01's attempt of 11:47 did reach the build and died there with `MutationLeaf source authority failed: Operation not permitted (os error 1)` (not investigated; a macOS permission on the cargo build path is the suspect).

## 2. Root cause found and fixed in code: one slow lookup closes the fleet tier
`🪪️graph-state/🛰️exchange/📡️client/🟦️.ts`: every request has a 1500 ms deadline (`AbortSignal.timeout`), and ANY timeout writes the shared `down` marker, which makes `usable()` false for 15 s for every process of the machine (stage, pool puller). `fetchGraphEntryV1` then answers `unavailable` without asking, and the owner is inferred locally. A slice process that runs hours of single-thread JS work stalls its own event loop, so a timer fires for an answer that already arrived and the whole fleet tier is declared down; the next lookups skip the fleet. Data: m08 782 of 866 lookups `unavailable` with only 26 started metadata calls; m01 241 of 460; m12 (loop responsive) 0 and it is 3x faster. The server is fine: 45 AC lookups from the coordinator through the real client took p50 2 ms, p90 4 ms, max 8 ms.
Fix (landed 15:19): `attempt()` reports `late` (how long after its deadline the timeout fired). `getResult` asks once more with `breaker.confirmMilliseconds` (3000) when the timeout fired `breaker.lateMilliseconds` (750) or more late, and writes the marker only if that fails. A silent server and a refused connection still mark at once, so a dead fleet costs exactly what it cost before. A first variant that repeated every timeout failed the existing fleet-machines bound (49 s vs 45 s) and was replaced.
Proof: new test `📡️client/🧪️tests/🚦️confirm` (fixture grid of 8 outcome sequences, independent Python oracle, a late timeout produced by blocking the loop for 2.65 s) failed before the fix and passes (1/0); client 14/0, fleet-machines 6/0, lookup-fault 2/0, retry 8/0, project-records 7/0, take 9/0, whole-gate machines 6/0, two-platforms 10/0, shared-intake 4/0, leaves 10/0, graph-state main 14/0.

## 3. E53 hardening reviewed and applied (15:10)
Sound: the old rule stopped a reading stage that showed progress once it was 30 min into the phase and a declared semantics source changed; now `readyCodeStopV1` stops only for a holder wait or a stage that has been quiet `codeQuietMilliseconds` (5 min), so a progressing stage is never stopped for new code and the code applies to the next attempt. Four old stale-stage tests asserted the stop of a moving stage; they now use a quiet stage (`codeQuietMilliseconds` 900 in the test policy). Tests: code-stop 4/0 (Python-oracle grid, moving read not stopped, quiet read stopped, restart in the middle adopts the same pid), process-tree 16/0, stale-stage 14/0, ready main 56/0. watchdog adoption: 1 pass, 8 fail (`first.stallAt` is 258 ms after the log time read before the first detach: the child writes its first log line late under load 24-28); this is in adoption timing, not in the code stop, and I did not re-run a baseline because that needs a revert in the replicated tree.

## 4. Live evidence
Landed 15:04 (ready) and 15:19 (graph-state), in the replica log of the coordinator at 15:04:26 and 15:15:19 / 15:18:58-15:19:03. See the last section for the post-rollout samples.

## NOT verified
- That any of the five reaches Ready: all are mid-read; workers restart their slice process only after the code has been quiet, and other lanes land every few minutes.
- That the fleet breaker fix raises the fleet hit rate on the workers (needs a new slice process there, see samples).
- The watchdog adoption failures (baseline), the m01 cargo `Operation not permitted`.
- The 15-20 s average metadata gate time on m01 means the slice process loop is blocked by synchronous work (observations, verification); not fixed, the fix above only stops it from closing the tier.

## What others must know
- E54: the fleet owner records are reusable across machines (m12 fetched 297 in 17 min); a sealed generation or selected head would remove the read for workers altogether, and E54's finding (seal dies of OOM after the fragments) is the real unblocker.
- `📋️build-owners.json` changed 27 times since midnight (all machines); the selected scope key is the hash of selection and registry, so each edit starts a fresh `selections/<scope>` store and fleet head key (finding in `findings-E55.md`).

## Last samples (15:23)
```
15:23:02 g1-m01 running False workspace att 4 adopted | Reading the project graph slice (5155 s): taken 221 of 604.
15:23:02 g1-m06 running False workspace att 2 adopted | Reading the project graph slice (3344 s): checking the file inventory of the tree (2
15:23:02 g1-m08 running False workspace att 10 adopted | Reading the project graph slice (2131 s): taken 117 of 604.
15:23:02 g1-m10 running False workspace att 6 adopted | Reading the project graph slice (2230 s): taken 37 of 604.
15:23:02 g1-m11 ready True None att 0 None | 
15:23:02 g1-m12 running False workspace att 12 adopted | Reading the project graph slice (1974 s): taken 450 of 604.
15:23:03 g1-m01 pid 54152 up 4468 age 693 ledger {'present': 0, 'fetched': 158, 'here': 63, 'stale': 0, 'done': 221} looks 460 unavail 241 transport 1 meta 240 3652
15:23:03 g1-m08 pid 54315 up 1325 age 817 ledger {'present': 0, 'fetched': 72, 'here': 45, 'stale': 0, 'done': 117} looks 866 unavail 782 transport 2 meta 85 208
15:23:03 g1-m10 pid 50279 up 1765 age 474 ledger {'present': 0, 'fetched': 33, 'here': 4, 'stale': 15, 'done': 37} looks 194 unavail 115 transport 3 meta 83 843
15:23:03 g1-m12 pid 63689 up 1672 age 312 ledger {'present': 152, 'fetched': 297, 'here': 1, 'stale': 0, 'done': 450} looks 485 unavail 0 transport 0 meta 490 1148
```
The slice-memory samples are written every 4-13 min and a worker restarts its slice process only after the code was quiet, so the first sample of a process that runs the new client was not yet in `S/` when I stopped; no worker reached Ready during this lane. Next lane: compare `faults.unavailable` / `looks.count` of a slice-memory sample whose `uptimeSeconds` starts after 15:20 (m08 was 782 of 866 before).
