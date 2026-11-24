# L1 Lane pool trim removed the restored release lanes

## 1. Where "2 lanes, 18240 MiB" comes from (snapshot `/Users/ueli/.sinc/play`, same as live for the pool except the restart-hold edit)
- Message `trimmed` (`🧬️schema/🔣️.json` Messages) printed by `acquirePooledLaneV1` (`🧵️lane/🏊️pool/🟦️.ts`) right after the new lane is entered. `{count}` is `laneIdsV1(root).length` AFTER the trim: it includes the reserved default lane `d` and the lane just entered, so "2 lanes" is not the pool size and not the victim count. The log shows `removed 1 idle lane(s)` (one lane, not two; `play .../pinned/site-build.log:9`).
- Byte bound: `poolQuotaBytesV1` = `floor(min(capacity, machine store quota) * gc.laneShare)`, `laneShare` = 0.234375 (schema, fixed constant), capacity = image minus one compile workspace = 76 GiB: 76 GiB x 0.234375 = 17.8 GiB = 18240 MiB exactly. Lane count bound: `pool.maximumLanes` 16 (fixed), live 32.
- Victim order (`planPoolTrimV1`): all idle lanes except product lanes retained within 12 h (`q…`) and held lanes, least recently used first by the `mtime` of `lane.json`, ties by id, until count <= 16 and recorded bytes <= quota. Recorded bytes = `lane.size` minus incremental. A lane of 17 GiB alone nearly fills the quota, the restored set of 31 GiB cannot fit it, so the oldest lanes are victims.
- Why restored lanes were victims: (1) the bound was a share of the image with no relation to what a release build needs (component lane 17 GiB + host + tool lanes = 31 GiB > 17.8 GiB); (2) recency was only the file `mtime` of `lane.json` (a restore/copy resets it or drops it: a lane without `lane.json` reads as `lastUsedMs = 0`, the oldest possible); (3) non-product lanes (the wasip2 component lane comes through `withRepositoryComponentLaneV1`, a pooled `p…` lane) were never retained; (4) the starting build's own lanes were not known to the trim, only the lane it just entered was protected by its claim; (5) the collector (`planGcV1`) applies the same share and a 24 h idle age to every unretained lane, so it would remove them next. Live now: `r/m/u/l` holds `pecaddf q044384 q27b12a q8750af` (1.5 GiB in total, all re-created after 08:33).
- Live vs snapshot: live additionally has the restart hold (`holdLaneV1`, `held`); snapshot lacks it. `restartHoldMilliseconds` was added to the schema by a peer while I worked.

## 2. Fix (schema first)
- Schema `pool.recentMilliseconds` (12 h) and `pool.flavourPattern`; messages `poolCannotHold` EN/DE.
- Lane record `lane.json` carries `usedMs` (written at entry and at release) and `flavour`; `laneUsedMsV1` = max(usedMs, mtime), directory mtime when no record: the stamp is file content, so it survives a restore or copy that resets file times. Pool and collector (`listLanesV1`) use it.
- Flavour = the compiler build mode, taken from the shared build directory name (`laneFlavourV1`, one directory per mode) so the launcher path (`repo cargo`, component drivers) and the compiler store path agree; the lanes of a build declare themselves by being used with that flavour.
- Bound: `poolNeedBytesV1` = largest sum of recorded bytes over the lanes of one flavour; the trim bound is `max(share bound, need)`. The share itself is what the declared reserves leave (test: laneShare + unitShare + admission + incremental + (1 - highWater) <= 1).
- `planPoolTrimV1(lanes, bounds, {requested, pressure})`: never a claimed or requested lane; a recent lane of a declared flavour only under free-space pressure (image used above the high-water mark, measured with `measureVolumeV1` through the new `volume` option); LRU otherwise. The starting build requests its entered lane and every recent lane of its flavour.
- If the need exceeds what the image holds below high water the trim reports `poolCannotHold` (EN/DE) and removes nothing; the build proceeds. The collector exempts recent flavoured lanes from idle age and lane share (`retained`), pressure alone may remove them after all others (existing contract).
- Fixed on the way: `trimmed` message `count` is now the pool size (reserved lane excluded).
- Progress and cancellation: unchanged (the trim is synchronous inside the claimed catalog turn; acquisition keeps its AbortSignal).

## 3. Tests (`repo test-file`, one at a time, all pass)
New `🌡️flavour-need` 5/0 with a Python oracle over a shared corpus: restored 17+8+6 GiB release lanes on an 80 GiB image with the 17.8 GiB share bound and reset file times, release starting: only the stale lane is trimmed, none of the three; pressure: the unrequested LRU lane goes; need over image: `poolCannotHold`, nothing removed; use stamp survives mtime reset; reserves invariant. Regression: pool-model 6, restart-hold 5, product-retention 4, pool-claims 7, sessions 1, product 8 (the test now ages the use stamp too), lane 2, compiler lane view 3, gc 14, retention-io 10, unit-cache main 28.

## 4. Snapshot patch
`L1-patched/` (README, `apply.sh <root>` with pre-image sha256 checks, `list.tsv`, 8 patches generated from the snapshot's contents, `make.py`). Verified on a scratch copy: all patches apply, results equal live for the files without peer edits, TS parses. Not applied to the play tree.

## 5. Open
- Lanes restored from before the patch have no `usedMs`/`flavour`; the first start after applying can still trim them once (README).
- The launcher derives the flavour from the shared build directory name (`p`/`r`); a renamed layout would need `flavourPattern` kept in step.
