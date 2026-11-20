# Coordinator Pressure Census Audit — 2026-11-19

Scope: read-only inspection of existing local code and saved logs; no new graph read or worker access. Findings refer to the current source, while saved logs may be from earlier code.

## Code paths

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🦶️footprint/🟨️.mjs` implements three distinct callers of `heapStats`. The sampler at lines 83–108 keeps its own `quietUntil`: each measured census duration C is followed by C × 19 quiet time at the declared 5% duty. Starved samples during that interval use the last counts. This state is private to the sampler; it does not govern the other two paths.

The gate at lines 130–159, 173–179 performs a census following a reported settlement when its pre-settlement footprint reaches the 60% soft threshold. Admissions generally check a 30 s minimum between settlements; if still at the 85% hard threshold after the first settlement, `admit` may immediately perform another with relievers. The watcher checks every 10 s and also observes the minimum. The census follows collection and is awaited. The current code labels it with the refreshed post-collection footprint. `settleWaveMemoryV1`, lines 309–318, separately awaits a census after a published wave whenever reporting is enabled and the refreshed footprint is positive. Neither census is charged to the sampler’s duty budget. The gate’s minimum is a settlement interval, not a measured census-duty bound; an explicit `settle` and the immediate hard-pressure second settlement also exist.

## Existing measurements

- `🗑️generated/server-read-0333.log` contains seven English census lines, all immediately following pressure-relief messages; no published-wave census appears. Nearby periodic elapsed lines place the second, fourth, fifth and seventh around 930, 1010, 1070 and 1550 s, respectively. The first, third and sixth have no precise timestamp in the adjacent lines. This establishes repeated pressure counts, not exact intervals or census durations. The log predates the corrected footprint label: e.g. 8663 MB before collection, 8541 MB after, and census labeled 8663 MB. Other saved `server-read-{0931,1006,1133,1205,1308}.log` files contain no English census lines; absence is limited to those saved excerpts and their active language.
- `result-R57.md` reports real unswapped heap-count timing of 0.55–0.73 s at 6 million live objects and 1.5 s at 36 million. Its seven-owner harness measured sampler counts 33 calls / 9.3 s before, 14 / 6.5 s after duty limiting; wall time was 132.8 s before and 144.1 s after. It explicitly reports no measured end-to-end improvement, a small unswapped harness heap, and no live-process profile attributing the stalls to census. Those measurements are not timings of the current gate or post-wave calls.
- `audit-observation-collection-memory.md` documents a later coordinator relief event from 9295 to 9094 MiB and the former pre-collection label defect; the active read predates the later fix. It does not report census elapsed time.

## Assessment

The extra paths can perform full heap walks independently of the sampler, and pressure logging has occurred repeatedly in a real read. Existing evidence does not quantify the time spent in those gate or post-wave calls, their share of coordinator wall time, or the current rate across a completed representative read. The observed seven calls and older unswapped timings alone do not establish that changing their scheduling improves end-to-end performance. The sampler duty guarantee should not be described as a shared bound on all censuses. No implementation change or performance claim is supported by these records alone.
