# GR14: the `workspace` selection head (2026-11-23, 20:06 to about 21:00 Mac clock)

`X` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🤝️selected-exchange/`. Lock `lock.graph-read` taken and released. Scripts and logs of this lane: `GR14/` (`probe-head.ts`, `probe-publish.ts`, `dry.err`, `dry2.err`).

## 1. What publishes the head of a selection
- The head of a scope is published by the process that READS that selection and finishes: `selectedGenerationV1` (`X/🟦️.ts`) publishes after `input.read` returns a sealed result. Scope = `buildGraphScopeKeyV1(selection)`; the Ready stage `workspace` is `repo run-many --projects=@semio-tech/norm-din4108,@semio-tech/norm-din4108-rs,repo-mcp --targets=build` (sidecar `🚦️ready/🧬️schema/🔣️.json:192`), and `repo run-many` with the same projects and targets plus `--dry-run` loads the SAME scope (proved: the coordinator's dry run first printed "waiting for g1-m08 to seal this selection", i.e. the claim key of the workers; selection 605 owners, closure round of 3,600). So the cheapest correct producer is exactly `repo run-many --projects=... --targets=build --dry-run` on the coordinator: the graph read of the selection, no task execution.
- Nobody had finished one because of TWO defects, both found in this lane:
  1. Single-claimant wait. The first machine to claim the head key reads, all others wait up to 25 min for it (`waitMaximumMilliseconds`) while a claim holder's lease is renewed. The claim went to 16 GB workers whose stage is stopped by the "nothing new for 10 minutes" stall rule and restarted (attempts 5 to 16 in `ready.json`), and the waiters (including the coordinator) re-queued behind the next weak claimant. A strong machine waited for the weakest.
  2. The sealed generation of the selection was never storable: the `admission` document was 317.6 MiB, above `documentMaximumBytes` (256 MiB, `🪪️graph-state/🧬️schema/🔣️.json:33`), publish returned `limit-document` and the exchange discarded the reason silently (no line, `published: false` in `slice-memory.json` `sources.exchange`). Every owner entry repeated the observation rows it consumed (tool closure rows of about 1,900 files each); 636 owners x shared rows. So even a finished read could not publish a head. This is why the fleet never had one.

## 2. Worker state traced (20:07 to 20:10)
`ready.json` of g1-m06: stage `workspace` (5/5), attempt 11, `stalled` ("showed nothing new for 10 minutes ... started again"), tail "Reading the project graph slice (965 s): reading the owners (round 1) 329 of 3600", log shows the number frozen for 970 s. m01/m03/m08: file-inventory phase or round 1; m02, m07, m10, m12, m13: "checking the file inventory of the tree (217,000 distinct source observations)" for 5,500 to 9,400 s. All were inferring or verifying on their own because no head existed. Fetched versus inferred per worker cannot be read from the worker logs (the stage log keeps only progress lines; the `[selected-exchange]` line and `slice-memory.json` `sources.exchange` appear only after the read ends).

## 3. What was changed (landing in `landings.md`, "Wave 1122-dist GR14")
- `X/🟦️.ts`: (a) a machine with at least `readBesideMinimumTotalMegabytes` (24,576 MB, schema policy) does not wait for a renewing claim holder; it reads beside it (`reading-beside` text) and publishes the head, workers (waiters) take it when it is stored. (b) the admission document is version 2: one shared table of distinct neutral rows plus per-owner index lists (317.6 MiB -> 99.2 MiB on the real selection); the reader rebuilds every owner's rows (`localizeRowsV1` once per distinct row). (c) a failed publish is told (`not-published` with the reason), no longer silent.
- `X/🔌️attempt/🟦️.ts`: passes `os.totalmem()`; `X/🧬️schema/🔣️.json`: policy `readBesideMinimumTotalMegabytes`, texts `reading-beside`, `not-published` (EN, DE).
- Tests (one at a time, `repo test-file`): exchange `🧪️tests/🟦️.ts` 21 pass 0 fail (new: big machine reads beside a renewing holder at once and a small one waits; admission rows stored once and every owner's rows come back), attempt 3/0, wiring 3/0, idle-claim 3/0, real-server 3/0.

## 4. Timeline
| time | event |
|---|---|
| 20:06 | `repo fleet status`: Generation none, workers 0 ready / 11 working; coordinator whole read at owners 92 of 5,222 |
| 20:09 | started `repo run-many ... --dry-run` (foreground-observed log `GR14/dry.err`): "waiting for g1-m08 to seal this selection" 15 s .. 100 s |
| 20:11 | claim of m08 lapsed (stall restart), the coordinator became the reader: file inventory, "taken 0 of 605" at 20:14, 590 of 605 at 20:16, owners checked 20:18, sealed graph written (selected/2f245baf...: graph 30 MB, authority 110 MB, admission 123 MB) at 20:18; the whole read was not disturbed (graph memory 3.0 to 3.4 GB of its grant, host 13 GB free, swap 2.1 to 2.4 GB constant) |
| 20:18 | publish returned `limit-document` silently; probe `GR14/probe-head.ts`: head miss |
| 20:2x | reproduced the publish with `GR14/probe-publish.ts`: documents graph 28.6, owners 95.1, admission 317.6 MiB, reason `limit-document` (rows 357,418, admission entries 636) |
| 20:30 | admission document v2 landed; `probe-publish.ts` stored a head of 620 owners in 12.3 s (documents 28.6 / 95.1 / 99.2 MiB); `probe-head.ts` lookup 1.6 s, head complete (graph 25.9 MB) |
| 20:37 | m08 restarted by its stall rule, took the head: "607 owners admitted" in its progress line (memo seeded from the head), 129,152 verified memo rows after 239 s |
| 20:42 to 20:57 | m03 (607 admitted, 636 owner checks), m02/m07 (607 admitted), m06, m08: all in the round-1 / "taken N of 605" phase after the head; none finished yet |

## 5. State at the end (see section 6 for the last observation)
- The head of the Ready `workspace` selection exists on the fleet (published by this lane through the real `publishSelectedGenerationV1`, revision 1, darwin-arm64, scope `2f245baf...`).
- Workers adopt its admission facts (607 owners admitted without inference) but still take the 605 owner RECORDS one by one ("taken 7 / 17 / 34 of 605" after 15 to 20 minutes) and read round 1 of 3,600 owners: the head replaces the observation of the tree, not the per-owner records, and a worker's tree differs from the coordinator's (the `view.txt` of a worker lists replica deletions), so rows that do not hold are read again.

## 6. Open items
- `Workers: 0 ready` and `Generation: none published` were unchanged at the last look; the head is necessary, not sufficient: the per-owner take on a 16 GB worker (records stale on `generated` closure rows, GR8/GR12; take budgets under memory pressure: pool.json of m03 says "next child needs 713 MB but 0 MB free above the 2048 MB floor") is the remaining time.
- No `[selected-exchange]` line is visible on any machine while the read runs (stderr of the graph child is not relayed to the stage log); the outcome shows only in `slice-memory.json` after the read. A line in the stage log would remove the guessing.
- The whole read of the coordinator publishes no generation head yet (owners 92 of 5,222 at 20:06); the selection head does not depend on it.

## 7. Last observation (21:00)
Workers m08 (taken 18 of 605), m03 (17), m06 (34) are in the per-owner take after adopting the head's admission facts (607 owners admitted); m02, m07, m10, m12, m13 are still in the file-inventory phase of processes started before the head existed (they restart by themselves on the stall rule and will look the head up at start). `Workers: 0 ready` unchanged; no worker has left `workspace` yet. Next lane: why the per-owner take (`taken N of 605`) advances 1 to 3 owners per minute on a 16 GB worker although the head and the fleet records exist (records rejected on machine-local `generated` closure rows, take memory floor "0 MB free above 2048 MB").
