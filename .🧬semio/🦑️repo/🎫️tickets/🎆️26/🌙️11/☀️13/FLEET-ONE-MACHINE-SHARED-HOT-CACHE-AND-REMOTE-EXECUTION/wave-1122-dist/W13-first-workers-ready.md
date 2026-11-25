# W13: first macOS workers Ready (2026-11-24, 20:18 to 21:35)

`B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`, `R` = `B🌐️remote-execution/🛰️sidecar/🚦️ready/`. Evidence is read-only: `/Users/Shared/semio-store/v1/origins/g1-*/{ready.json,ready-timings.json,slice-memory.json,memory.json,ready-log-workspace*.txt}`, sampled every 30 s by `W13/poll.py` (20:19 onwards, raw samples in `T/🗑️generated/W13/poll.jsonl`, views `W13/view.py`, `W13/now.py`, `W13/attempts.py`), plus `repo remote-execution status`. `repo fleet status` timed out its workers reader (load 60 to 90). Lock `lock.worker-ready` taken 20:18, released at the end. No sub-agent, nothing restarted by hand, no scratch build, no image touched, nothing written in `~/.sinc/play`.

## Result
- **g1-m11 is Ready since 21:26:01** (workspace run 20:23:55 + 3,725 s, outcome `ok`). Ready count 1 -> 2 of 11 (m02, m11). It stayed Ready through other lanes' landings after 21:26 (wgpu renderer sources) and at 21:30 (pid 843, `refreshing` false).
- It serves remote actions: the scheduler (started 19:47) counts `offloadedIn` 10 for g1-m11 (headroom true, sandbox full), m02 8.
- No other worker finished the workspace stage in the window; what each waits for is in section 4.

## 1. Timeline of the read on the workers (item 1)

The g1-m11 run that reached Ready (slice worker pid 72866; times from the 30 s samples):
| elapsed | phase |
|---|---|
| 0 to 56 s | waiting for g1-m13 to seal this selection (exchange wait; m13 went offline, see 4) |
| 56 to 210 s | file inventory (62,573 observations, 611 owners admitted) |
| 210 to 414 s | owner round 1 before the first take outcome (report silent) |
| 414 to 1,555 s | take: `taken` 0 -> 59 (957 s) -> 248 (1,313 s) -> 440 (1,555 s) -> 604 (1,587 s) -> 608 (1,677 s); report silent up to 1,080 s at a time but the progress line moved, so the 15-min silent-read rule did not fire |
| 1,677 to 1,794 s | file inventory again + `checking that the observed source facts still hold (round 1)` |
| 1,826 to 1,918 s | owner round 2 (73 s; it re-took the few owners whose paths moved, nothing re-verified in full) + facts hold (round 2) |
| 1,948 to 1,978 s | seal and publish the sealed graph to the fleet |
| 1,978 to 3,725 s | build of the selected targets (`norm-din4108` and 8 tasks): `unit:compile` 932 s for 221 units, `unit:publish` 2,123 s, `unit:lookup-fleet-miss` 226 (the fleet cache had none of these units) |

Owners of m11's pass (609): 462 stored locally, **141 fetched from the fleet** (`fleet.seeded`), 5 inferred here (`computed` 5, `reinferred` 2), 1 published, `refused` 0, `stale` 0. So for (b): the owners not stored on a worker ARE on the fleet and are fetched in the same pass by the fetch-first adopt loop; only 5 of 609 were inferred locally. Inference is already coordinated per selection at the graph level: a worker waits for the one that claims the seal (`waiting for g1-XX to seal this selection`, bound 1,500 s) and installs its sealed graph; today the claimants froze or went offline (m13), so the waiters timed out and read themselves.

All attempts seen (worker, slice pid, window, max `taken` report/CLI, longest report silence, end): `W13/attempts.py` over the poll file. Summary: 27 attempts; 26 ended `stalled` (SIGTERM) or were still running at 21:30; every one had report silences of 10 to 30 min; the furthest besides m11 were m12 567 of 609, m01 561, m10 544, m06 352.

## 2. What the silence is: a stack sample names it (item 2, landed)
The W12 premise (presence/first-grant filesystem work, ~15 calls per name, blocks the loop) does not hold on the current code:
- In every frozen report the presence body's own filesystem work is small: m01 1,815 checks in 785 ms, m11 1,453 in 160 ms, later 9,932 in 467 ms. The presence `continuation` (one `setImmediate` yield) waited 42 to 66 s: the loop was busy with something else.
- Freezes also happen before any presence session (m06/m07/m08/m12 `reading the owners (round 1)` at 3 to 15 s, m10/m12 at `taken 0`, presence timings all 0).

So I landed a sampler instead of a speculative rewrite: `R🌲️process-tree/🔬️sample/🟦️.ts` (schema `🔬️sample/🧬️schema/🔣️.json`): once per supervisor and attempt, when the stage reads and its own slice report is 180 s silent and the report's pid is alive, the supervisor runs `sample <pid> 3` and mirrors the main thread's own time by frame and the end of its heaviest path as `ready-log-workspace-sample.txt` in the worker's origin (EN + DE). The stage log is not touched (no fake progress). First version required the pid to be a member of the stage tree: no sample came (the slice worker is not in the roster), second edit 20:44 checks it is alive.

Samples (20:48 to 21:01, ten workers/attempts), main thread own time:
| worker | silent | `stat` | `getdirentries64` | `open`/`read` | other |
|---|---|---|---|---|---|
| m01 | 445 s | 1,982 of 2,247 (88 %) | 202 | 54 | |
| m03 | 225 s | 2,055 of 2,244 (92 %) | 10 | 81 | |
| m06 | 675 s | 1,975 of 2,031 (97 %) | 8 | 45 | |
| m08 | 778 s | 665 of 1,981 (34 %) | 46 | 255 | JIT 163, `lstat` 61, `pread` 46 |
| m11 | 1,029 s | 2,065 of 2,220 (93 %) | 93 | 54 | |
| m10, m12, m07, m01 (2nd) | 183 to 192 s | 85 to 95 % | | | |

The frames under `stat` are `bun+0xc3b35c -> 0x114f260 -> 0x1563e88 -> stat`, which on the coordinator with the pinned runtime (`…/runtimes/bun/90987a3a…/bun`, 1.4.2) is exactly `fs.statSync` (`existsSync` is `access`, `lstatSync` is `lstat`; probe scripts in the scratchpad, mapping reproduced). So the read's main thread spends its silent minutes in synchronous `statSync` + `readdirSync` + `readFileSync` from JS: the observed-filesystem layer of inference and the synchronous row verification (`B👁️observation/🟨️.mjs`: `current()`/`fileFactHolds`, "synchronous probes beat the four-thread pool here", `observedFs.readFileSync/statSync`, `typedListingV1`), not the presence check (which uses `lstat` with bigint and was < 1 s per report). The JIT frames above cannot be named by `sample`, so which caller (owner authority closure walk `projectAssemblyAuthorityV1`, the 5 local inferences, or stored-row verification) is the bulk is NOT proven; the look phase sums point at authority (m11 `authority` 67 s at 440 owners -> 1,406 s at 601) but that sum includes yield waits.

## 3. What I did NOT land and why (item 2 of the brief)
- Async/batched presence: not done. The measured presence filesystem work is < 1 % of the silent time; making it async would not move a single report.
- Fetch of missing owners in one batch + claimed inference: already happening (m11: 141 fetched in-pass, 5 inferred; seal exchange already elects one sealer per selection).
- A pass-wide validation round or moving `statSync` work off the main thread: not landed. It needs the JS caller (section 2) and touches the read closure; at 21:00 to 21:30 m11, m10, m12 and m01 were at 532 to 608 of 609, and a landing in the read closure restarts every in-flight read (W12). The rule of this lane (no second burst while a worker is close to finishing) held me back. The silent-read stop (15 min) stays as it was.

## 4. Per worker at 21:30: what each still waits for
| worker | state | blocker, with evidence |
|---|---|---|
| m02 | Ready | - |
| m11 | **Ready 21:26** | - |
| m10 | take 544 of 609, report silent 1,866 s, CLI line still moves | main thread in `statSync` (sample 20:49: 1,781 of 2,022); will reach the seal if not stopped |
| m12 | restarted 21:21 (previous attempt stalled at 567 of 609 after 1,496 s silence), round 1 again | same `statSync` silence; the stop came at 567 |
| m01 | take 561 of 609, report silent 1,392 s | same (sample 20:57: 2,168 of 2,308 `stat`) |
| m06 | take 352 of 609, silent 609 s | same (97 % `stat`) |
| m08 | take 92, silent 754 s | same, plus more JIT/`read` (record reads) |
| m07 | 5 attempts stopped in round 1 at 4 to 51 s, now round 1 again | same (sample 21:01: 2,198 of 2,371 `stat`); never reached the take today |
| m03 | file inventory, 1,600 s into the attempt | slow file inventory (W12's lstat walk); swap 6.9 GB (fleet status warning) |
| m13 | origin silent since 20:34 | last memory record: swap 9.7 GB used, swap-out 204 MB/s, swap volume 1.8 GB free, pressure warn; still registered at the scheduler (lastSeen 1 s), so the machine runs but its readiness mirror stopped; it was the sealer m11/m01 waited for at 20:24 |
| m04 (linux), m05 (windows) | dependencies stage | not in scope (macOS only) |

## 5. Tests (`repo test-file`, one at a time; pass/fail)
- `R🌲️process-tree/🔬️sample/🧪️tests/🟦️.ts` 6/0 (new): due rule; frame names; summary of a crafted and of a real `sample` report equal a CPython model (`🐍️.py`); EN/DE lines; bounds of the sampler call; a live bun child blocked in `lstatSync` is named by the real sampler.
- `R🧪️tests/💓️watchdog/🟦️.ts` 10/0, `R🧪️tests/🟦️.ts` 58/0.
- The two red tests W12 found, cause first, fixed (test-only):
  - `B🧠️memory/📋️slice-report/🧪️tests/🪜️read-steps` 7/1 -> 8/0. Cause: since 01324745f80 (11-22) `readyStepWatchV1` deliberately treats a step within its bound as moving (quiet 0); the case fed in-bound lines and expected quiet to grow. Now it feeds lines past the bound, where the quiet clock applies.
  - `B🧠️memory/📋️slice-report/🧪️tests/🟦️.ts` 12/2 -> 14/0. Causes: (1) the real-process worker stopped 200 ms after start while its asynchronous `ps`/footprint measurement (by design never awaited by a report) took longer at load 70; it now waits up to 20 s for the measurement. (2) the probe kids idled after one fill and macOS compressed them to 18.2 MB below the 20 MB sanity bound; they now keep touching their 24 MB.

## 6. Landings
`T/landings.md`, "W13" (20:35:57 and 20:44:40 supervisor sampler; 20:50 to 20:56 two test files). No read-closure module was edited, so no worker lost a read to this lane (sidecars restarted and adopted their stages at 20:38 and 20:48).

## Open (next lane)
1. Name the JS caller of the `statSync` storm: the samples prove the syscall, not the function. Cheapest proof: count `statSync`/`readdirSync` per caller inside `B👁️observation/🟨️.mjs` (`world` is the single binding, so one wrapper there sees every call) and publish the top callers in the slice report; then either verify rows once per take pass (`withObservationPassRoundV1` around the lanes, as the settle pass already does) or route `current()` through the async pool (`observationsHoldWithinRoundV1` already exists) so the loop turns.
2. Finished reads leave their last relayed slice report at `interval` (m11: 20:55:56, never `end`), because the relay only answers the parent's request; it is harmless now (the stage phase is `executing`) but the silent rule would misread a long quiet build phase.
3. Ready-slot relay bound 65,536 B vs generic report bound 98,304 B: a report between them is silently not relayed (not observed today; reports are 3 to 7 kB).
4. The build step of the workspace stage compiled 221 units locally on m11 with 226 fleet cache misses; the hot cache did not serve this selection yet.
5. m13: swap volume nearly full (1.8 GB free); someone with machine access must free it.
