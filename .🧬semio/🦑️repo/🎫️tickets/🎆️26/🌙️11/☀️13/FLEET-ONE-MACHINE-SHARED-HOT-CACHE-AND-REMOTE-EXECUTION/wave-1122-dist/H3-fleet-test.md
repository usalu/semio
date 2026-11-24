# H3 Fleet test attribution

File: `LIB/🏗️build/🌐️remote-execution/🧪️tests/🌐️fleet/🟦️.ts` (10 tests).

## Runs
| run | setup | result |
|---|---|---|
| 1 | `taskpolicy -b`, host load 40-70 | 1 pass / 9 fail, 1124 s |
| 2 | no taskpolicy (coordinator withdrew the rule), scaled deadlines | 9 pass / 1 fail, 33 s |
| 3 | + fixture fix | 10 pass / 0 fail, 46 s |

## Per test
| # | test | run 1 | cause | action |
|---|---|---|---|---|
| 1 | machine table fields | fail 54 s (registration deadline) | (a) H2 AND (b): run 2 exposed `freeMegabytes:60000` mismatch; H2 made it the spare above the 4096 MB floor (60000-4096=55904) | fixture updated to 55904 |
| 2 | critical pressure gets nothing | fail 470 s, registration | (b) load/ taskpolicy -b (disk reads ~60x slower; 11 bun boots) | passes in run 2/3 |
| 3 | worker killed mid-action | status 14 (fell back) | (b) lease 2 s expired under starvation | passes in run 2/3 |
| 4 | cancel kills tree | fail 49 s, registration | (b) | passes |
| 5 | offload=if-idle | fail 80 s, registration | (b) | passes |
| 6 | pin | fail 59 s, registration | (b) | passes |
| 7 | toolchain build | worker "" after 143 s | (b) | passes |
| 8 | worker admission refusal | 40.8 s vs 4 s assert | (b) | passes; assert now load-scaled |
| 9 | rescheduled when worker leaves | "first start" timeout 158 s | (b) | passes |
| 10 | two platforms | passed in run 1 | - | passes |

The H2 admission change is inert for these fixtures: test actions declare no `peakMegabytes`, in-process workers have no `admit`, and the host view of the entry workers reports 60000 MB available (poll asks 256 MB).

## Changes
- `🧪️tests/🌐️fleet/🟦️.ts`: eventuallyV1 wrapped with `stretchedBudgetMillisecondsV1`; 3000/4000 ms asserts scaled; `freeMegabytes` 55904.
- No product change.

## Live sanity (repo fleet status --json, twice, 17:17-17:18)
- No machine `critical`. Warn: g1-m01 (both reads), g1-m10 (first), g1-m13 (second); every warn machine reported headroom false / worker refusing. Others flipped to refusing under load (m07 m08 m11 m13), consistent with the gate.
- Per-machine swap is not in the JSON; the coordinator's swap stayed 2591 MB (flat). No machine seen with headroom true in warn/critical.
