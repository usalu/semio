# MA3 Audit: Live Metrics Pipeline Across The Fleet

Read-only. Taken 2026-11-26 02:56-02:57Z (03:56 CET). Store: `/Users/Shared/semio-store/v1/`. Reader: `readSeriesV1` (TS, `🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🟦️.ts`). Scripts kept: `🧰️scripts/MA3/🔎️read-fleet-metrics.ts` (full inventory, 5 min and 1 h windows, full-history metric census) and `🧰️scripts/MA3/🔎️timeline.ts` (tier runs of one metric). Run with `/Users/ueli/.bun/bin/bun` (1.3.14). No output file was kept; no state was changed.

## 1. Inventory of published rings

| origin | platform (per brief) | `metrics/` | t0..t4 (size, mtime UTC) |
|---|---|---|---|
| g1-m01, m02, m03, m06, m07, m08, m10, m11, m12, m13, m14, m15 | macOS | present | t0 733 KB, t1 880 KB, t2 3.5 MB, t3 2.4 MB, t4 5.3 MB; t0/t1/t2 mtime 02:56 |
| g1-m05 | Windows | present | same sizes, t0..t2 mtime 02:56, t3 02:51, t4 02:23 |
| g1-m04 | Linux | **absent** | does not publish |
| g1-m09 | (not provisioned) | absent | does not publish |
| g1-server | (not stated) | absent | does not publish |

- Fleet scope: `/Users/Shared/semio-store/v1/fleet/metrics/` does not exist. The five fleet-scope metrics (`exec.queue.*`, `exec.offload.in`, `cache.remote.entries`, `cache.remote.bytes`) have no published source at all.
- `._t0.ring`..`._t4.ring` (4 KB AppleDouble resource-fork stubs) sit next to the real rings on g1-m01, m02, m03, m06, m07, m08, m10, m12, m14, m15. Readers ignore them, but they are copy artefacts from a non-APFS copy path and should be removed by the publisher or a sweep.
- Sizes match the M2/M6 layout (69 metrics, t0 300 buckets).

## 2. Windowed reads (readSeriesV1, 9 focus ids, 5 min and 1 h ending now)

Values are the mean over the window (samples-weighted), min and max of the point minima and maxima; memory and disk in GB (10^9 bytes).

| origin | cpu busy 5 min (mean / min) | mem avail 5 min (mean) | disk free 5 min (mean) | self cpu 1 h (mean / max, %) | self RSS 1 h (mean / max, MB) |
|---|---|---|---|---|---|
| g1-m01 | 73.9 / 28.2 | 5.51 | 20.8 | 0.77 / 12.4 | 53 / 111 |
| g1-m02 | 93.8 / 22.4 | 3.40 | 6.5 | 1.09 / 14.9 | 51 / 120 |
| g1-m03 | 87.7 / 60.2 | 5.63 | 26.5 | 0.90 / 13.0 | 71 / 127 |
| g1-m05 (Windows) | 43.8 / 9.8 | 32.7 (of 68.6 total) | 13.1 (of 842 total) | 1.05 / 26.0 | 75 / 108 |
| g1-m06 | 73.1 / 9.2 | 4.80 | 18.8 | 0.75 / 15.0 | 57 / 112 |
| g1-m07 | 42.0 / 17.4 | 5.91 | 32.9 | 0.52 / 7.3 | 82 / 122 |
| g1-m08 | 52.8 / 23.1 | 4.80 | 14.5 | 0.89 / 9.0 | 49 / 122 |
| g1-m10 | 47.2 / 16.9 | 5.96 | 19.9 | 0.65 / 9.4 | 58 / 121 |
| g1-m11 | 46.5 / 26.2 | 6.64 | 30.2 | 0.65 / 10.3 | 69 / 127 |
| g1-m12 | 62.3 / 13.5 | 5.93 | 50.5 | 0.80 / 10.7 | 66 / 121 |
| g1-m13 | 34.7 / 9.4 | 5.62 | 23.5 | 0.60 / 7.8 | 68 / 123 |
| g1-m14 | 84.0 / 20.6 | 5.68 | 20.6 | 0.84 / 12.2 | 82 / 127 |
| g1-m15 | 99.97 / 97.8 | 5.03 | 46.3 | 1.21 / 55.8 | 49 / 117 |

Counters over 5 min: `build.units.compiled` has 30-31 points per origin (10 s cadence, see below), `cache.local.hit` and `cache.remote.ac.hit` have **no points on any origin**.

Coverage and gaps:
- Slow collectors (disk, units, queue, every 10 s) are sampled at 10 s, so each 1 s point covers about 10 % of the window. Their low `coveredPct` (about 10 %) is by design, not a gap. The dashboard must not draw these as gaps.
- Fast metrics (cpu, memory) cover 91-98 % of the last 5 min, with 2-36 gaps of 1-5 s (restart and tick jitter). m15 has 36 gaps over 5 min.
- 1 h windows cover only about 60 % (m01 59.8 %, m15 74 %) because the rings are about 35 min old. Tier 1 starts at 02:19-02:22 UTC on the Mac origins (m01, m15 checked), with a few t2/t3 rows before that. The largest gap (up to 1411 s) is the part of the hour before the collector's first row. This is a history-age effect, not data loss. Expect the 1 h coverage to recover over the next hour.
- Within tier 1 there are short holes (for example 02:19:00-02:19:40 and 02:33:30-02:34:00 on m01; 02:36-02:51 continuous). They line up with service restarts.

Plausibility:
- `host.cpu.busy`: all origins stay within 0-100 (max 100, min 7.4-97.8). OK.
- `host.memory.available` is within 0 and `host.memory.total` on every origin (Mac total 17.18 GB, Windows 68.6 GB; max available 10.2 GB on Macs, 35.3 GB on m05). OK.
- `host.disk.free` is within 0 and `host.disk.total` (Mac 245 GB, m05 842 GB). OK.
- `build.units.compiled` values are non-negative integers. OK.

## 3. Platform view and collector overhead

- Publication: all 13 origins with rings publish all 5 tiers (macOS 12, Windows 1). Linux g1-m04 does not publish, and neither do g1-server or g1-m09.
- Field coverage: a Mac origin fills 51 of 69 metrics. The Windows origin fills 36. Windows lacks `host.cpu.load`, `host.memory.compressed`, `host.swap.used`, `host.swap.rate`, `host.files.open`, `host.images.attached`, `warden.headroom`, `process.repo.cpu`, and all `graph.*` pool and read metrics. This matches the M6 notes, except the `graph.*` ones, which are not listed as Windows gaps.
- Overhead against the budget (< 0.5 % of one core, < 30 MB): the `metrics.self.*` values are taken from `process.cpuUsage()` and `process.memoryUsage.rss()` of the whole readiness service process (`🛰️collector/🔌️host/🟦️.ts` line 50), not the collector alone. The 1 h mean is 0.52-1.21 % of one core on every origin (all above 0.5 %), and RSS is 49-82 MB mean (above 30 MB; the bare runtime alone is about 31 MB per M3). So the published self metrics **do not show compliance with the budget**. M6's 0.206 % was a collector-only measurement. A collector-only self metric is needed before the budget can be claimed from rings. There is no `metrics.self.rss` id; `metrics.self.memory` is the RSS.
- Plausibility of self metrics: no negative values; spikes to 26-56 % CPU appear on m05 and m15 (likely the start and first publication).

## 4. Metrics empty everywhere (never fed)

18 of 69 metrics have no points on any origin in the full history:
- Local cache: `cache.local.hit`, `cache.local.miss`, `cache.local.bytes`. M5 reports these as landed, but no ring of any origin holds them (the feed is not observed, cause not checked here).
- Remote cache: `cache.remote.ac.hit/miss/put/conflict`, `cache.remote.cas.hit/miss/put`, `cache.remote.nx.hit/miss/put`, `cache.remote.entries`, `cache.remote.bytes`. There is no cache-server ring (`fleet/metrics` absent, `g1-server` unpublished).
- Execution: `exec.offload.in`, `exec.queue.running`, `exec.queue.queued`. These are fleet-scope and their ring is absent.

## 5. Findings for the coordinator

1. Only 13 of 16 origin dirs publish, and the Linux origin g1-m04 does not. Fix or confirm the Linux publication path.
2. Local-cache counters (`cache.local.*`) and every remote-cache and fleet-scope metric are never fed. Dashboard columns for them will be empty.
3. The overhead budget cannot be read from the rings: `metrics.self.*` covers the whole process and exceeds the budget on all 13 origins. Add a collector-only self metric or subtract the baseline.
4. The 1 h view is about 60 % populated until the history is about one hour old. Expected, not a bug.
5. Remove the `._t*.ring` AppleDouble stubs from the store.

Limits: one snapshot; the Windows and Linux runs were not checked live; no Python cross-check of the values was run in this lane.
