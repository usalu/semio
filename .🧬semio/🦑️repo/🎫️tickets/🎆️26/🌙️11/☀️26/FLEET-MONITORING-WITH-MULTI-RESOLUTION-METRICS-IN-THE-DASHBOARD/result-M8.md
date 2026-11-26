# Result M8: n/a cells, live headless render, writer lock by identity, data path overhead

Paths: `D` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`, `M` = `🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics`, `S` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar`. Kept scripts: `🧰️scripts/M8/` (`cargo.sh` plain-toolchain scratch cargo, `patch-lock.py`, `extract-renders.py`). `repo dashboard` was never run; everything is render-to-text.

## 1. n/a cells (D/📊️fleet, D/🖥️terminal/📊️fleet, schema `D/🧬️schema/📊️fleet`)
- **Wire, schema first.** `Row.unsupported` (overview metric ids the origin's platform cannot measure, from `metric_supported(id, ready.platform)`) and `Series.unsupported` (`const true`, no points). A merely empty series is still left out (that stays "no data yet"); an unsupported one is always sent so the window can say why. The reader does not even read an unsupported overview id.
- **Window.** Overview: a cell whose metric is unsupported shows `⊘ n/a` (en), `⊘ n. v.` (de), `~ n/a` on an ASCII terminal; it is a different word and symbol from `no data` (nothing yet), `stale` (word, triangle glyph, dimmed row) and `missing`. Drill-down: `Load average (1 min)  ⊘ n/a · not measured on windows` / `auf windows nicht gemessen` instead of a chart. Labels `fleet_na`, `fleet_detail_unsupported` in the compile-checked en/de label macro. Today's overview ids are measurable on every platform, so the overview n/a path is proven with a synthetic row; the drill-down n/a shows live for the Windows origin g1-m05 (host group: load average and swap used).
- Tests: fleet crate `cargo test` 18/0 (new: Windows origin from the Mac fixture rings gives exactly `host.cpu.load`, `host.swap.used` as unsupported with no points, a Mac none, the flag travels only when set; overview invariant), terminal laws `fleet_window` 13/0 (new: n/a vs no data vs stale in en, de and ASCII; drill-down n/a line in en and de), `repo test-file 📊️fleet/🧪️tests/🟦️.ts` 6/0 (new: **Ajv** accepts `unsupported:true`, refuses `false`, and the TypeScript `metricSupportedOnV1` yields the same Windows set for the host group).
- Cosmetic findings from the live render, fixed: label column 26 -> 34 cells (the `(fleet)` marker and "Remote executions running" were cut off), beat column 8 -> 10 (`24min 36s` was cut). Text snapshots regenerated (`FLEET_TEXT_WRITE=1`).

## 2. Live headless render
`live_store_render` (ignored test in `D/🖥️terminal/🧪️tests/🔬️unit/📊️fleet`, not part of the suite): resolves the store root from the configuration (`SEMIO_FLEET_STORE_ROOT`, else `storeRoot` of `<workspace>/.cache/f/machine.json`, here `/Users/Shared/semio-store`), reads the real `v1/origins/*` and `v1/fleet/metrics/*` through the daemon's `read_snapshot`, feeds the real Fleet window of a 200 x 60 terminal and prints its rows. Run: `FLEET_LIVE_OUT=<dir> 🧰️scripts/M8/cargo.sh test --manifest-path <laws Cargo.toml> live_store_render -- --ignored --nocapture` (`FLEET_LIVE_RANGES=1,2,3`, `FLEET_LIVE_ORIGIN`, `FLEET_LIVE_LANGUAGE`).

Checked by eye on the output below (06:00 CET, 5 min, 1 h and 24 h, every range key):
- all 16 origin folders appear as rows (15 with files, g1-m09 which has only `fleet-auth.json` is `! missing` with `no data` in every cell and no invented numbers); live origins carry CPU, memory, disk gauges with word and threshold ticks, trend sparklines, units/min, cache ratios, offload and the machine-name warning; origins whose readiness file is older than 2 beats (g1-m08, g1-m15, g1-server, later g1-m05, g1-m12) are `stale`, dimmed, sorted before the healthy ones and show their last beat age; g1-m15 shows `no data` for CPU and disk because it published nothing in the window;
- the totals row `Σ fleet` counts `n/16 ready` and averages live origins, and the facts line shows the fleet-scope metrics from `fleet/metrics/*`: `queue 0 running / 0 queued · offloaded in 0 · remote cache 50.0k entries, 71.8MiB`;
- the 24 h overview has data only in the last two columns for the Macs (history is hours old), older columns are dots (gap), by design;
- drill-downs show the tier ruler with a `┬` at every tier boundary and its age labels (`12min 47s … now`, `1h 53min … 6min 20s … now`, `2h … 1h 9min … 13min 30s … now`), the six groups, two-row band charts with now/min/max, and the Windows origin shows the two `⊘ n/a` lines. A coarse bucket that straddles the range start makes the oldest tier label older than the range (1 h range shows `1h 53min`); that is the merged series by design (M4), not a bug.
- Not nice but real data: many Macs report `dup 5` (machine-name warning), Windows g1-m05 has holes in its CPU trend (sparse published rows).


## 3. Writer lock by identity (M/🛰️collector, S/📈️metrics, schema `M/🛰️collector/🧬️schema`)
- `collector.lock` is now a record `{version:1, pid, startMs, role, checkout}` (schema `Lock`, `lockRolePattern`): identity is pid plus start time (`🏃️process/🔖️identity`; a reused pid is no holder), `role` is `origin` or `fleet:<source>`, `checkout` the checkout the collector runs for. Written whole under a temporary name and linked into place (a reader never sees half a record; falls back to exclusive create where hard links are missing); an unreadable record younger than 2 s is respected instead of robbed.
- **Only a real collector of this checkout may hold the machine-wide ring directory** (`metricsLockRefusalV1`): refused are a claim whose checkout lies in the OS temp directory, any claim under a test runner (`NODE_ENV=test`, which `bun test` sets), a machine role without a checkout and an unknown role. A private ring directory (anything outside `<host-capacity>/metrics`) is free for tests, so tests always use a private lock path. The collector reports a refusal once, en and de.
- A collector that finds its lock taken at a slow step stops writing (closes its rings) instead of writing next to the new holder; it takes the lock again when the other is gone. A live holder of an older code (record without role) is never robbed: two writers would corrupt the ring cursors, so that case cannot be fixed from new code; the old test members end by themselves and the real service takes over the moment they die (reported once as `foreign` with the holder's role, pid and checkout). Nothing can retroactively stop an old binary from taking an absent lock; after this landing no new member can.
- `metricsScratchPathV1` moved into the collector module (one definition, re-exported by `S/📈️metrics`); `startFleetMetricsV1` and `startFleetScopeMetricsV1` pass the checkout.
- Tests: collector 29/0 (new: identity record validated by **Ajv** against the schema, role-bound release, legacy holder untouched and reused-pid holder replaced, refusal matrix for scratch / test runner / unknown role / missing checkout / private directory, a lost lock stops the writes), product metrics 8/0, metrics 14/0. `repo typecheck` of both folders passes (it also made me fix three pre-existing type errors in the M7 tests: `volumes()` null, `label` on a tier, a tuple type).

## 4. Overhead of the dashboard data path (16 origin folders, 15 with rings, real store)
Measured with `live_overhead` (ignored test in `D/📊️fleet/🧪️tests/🔬️unit`): per range one overview read (16 columns) and one drill-down read (120 columns); CPU is process CPU over many reads (`ps`), wire bytes are the JSON of the snapshot the daemon sends. **The host was at load average 45-116 during all measurements**, so every CPU figure is inflated by contention and cache misses; the order of magnitude is what counts.
- Before: 5 min overview 34 ms CPU per read at 1 s refresh = 3.4 % of one core, 26.7 KB per refresh; 1 h overview 76 ms every 2 s = 3.8 %; 24 h 96 ms every 5 s = 1.9 %. Phase split: the raw ring reader is about 80 % of it (it reads whole rows, 71 metrics x 36 bytes, of 300 + 360 + ... rows per origin: decoding only the 10 wanted metrics instead of all saves little, reading dominates).
- Changes: (1) the daemon remembers the row of each origin with the modification stamp (mtime, size) of the 8 files behind it (`Reader`, `read_snapshot_with`, schema `rememberMs` 30 s): an unchanged origin costs 8 `stat` calls, a row older than 30 s is read again whatever the stamp says (NFS attribute caching), freshness is judged again at every read; (2) refresh time of the 1 h range 2 s -> 10 s and of 24 h 5 s -> 15 s (schema data; 5 min stays 1 s, 7 d 30 s, 90 d 60 s). Only the one range the focused Fleet window shows is ever read, and nothing at all while no Fleet window is open.
- After (steady state, daemon-side):

| range | request | origins | wire bytes | CPU ms/refresh, nothing remembered | CPU ms/refresh, remembered (steady) | wall ms/refresh (steady) | refresh | daemon CPU % of one core (steady) | wire bytes/s |
|---|---|---|---|---|---|---|---|---|---|
| 5m | overview | 16 | 27381 | 33.0 | 15.0 | 26.1 | 1 s | 1.50 | 27381 |
| 5m | drill-down | 1 | 4504 | 2.0 | 5.0 | 1.6 | 1 s | 0.50 | 4504 |
| 1h | overview | 16 | 29164 | 70.0 | 78.3 | 149.3 | 10 s | 0.78 | 2916 |
| 1h | drill-down | 1 | 79232 | 12.0 | 10.0 | 11.1 | 10 s | 0.10 | 7923 |
| 24h | overview | 16 | 9138 | 94.0 | 100.0 | 235.0 | 15 s | 0.67 | 609 |
| 24h | drill-down | 1 | 108234 | 19.0 | 13.3 | 32.9 | 15 s | 0.09 | 7216 |
| 7d | overview | 16 | 7781 | 95.0 | 93.3 | 137.8 | 30 s | 0.31 | 259 |
| 7d | drill-down | 1 | 107790 | 18.0 | 15.0 | 17.2 | 30 s | 0.05 | 3593 |
| 90d | overview | 16 | 7795 | 132.0 | 123.3 | 170.8 | 60 s | 0.21 | 130 |
| 90d | drill-down | 1 | 107818 | 20.0 | 21.7 | 21.6 | 60 s | 0.04 | 1797 |

phase raw reader 5 min, 10 overview ids: wall 79.8 ms, CPU 26.0 ms per refresh (15 origins)
phase raw reader 5 min, all ids: wall 152.4 ms, CPU 39.5 ms per refresh (15 origins)
phase raw reader 1 h, 10 overview ids: wall 312.8 ms, CPU 65.5 ms per refresh (15 origins)
- Wire: the daemon sends only when the content changed (5 min always does) or as a 5 s heartbeat; worst case 27 KB/s for the 5 min overview, 4.5 KB for its drill-down, 2.9 KB/s at 1 h, under 1 KB/s from 24 h up; drill-downs of 1 h or more are about 80-108 KB per refresh every 10-60 s. Point rows are `[start, width, min, max, mean, last]` as JSON floats; a leaner overview point (mean only) would halve the 5 min figure, not done.
- Remaining cost: the first read after open or after a range change reads everything (about 30-130 ms CPU on the loaded host), the 5 min view re-reads about 3 of 16 origins per refresh (publication every 5 s per origin, staggered), 1 h and coarser re-read all (their newest column legitimately changes every publication). Worst steady figure is about 1.5 % of one core at 5 min, under 1 % elsewhere, only while the window is open.
- Not done: an incremental reader that keeps closed rows instead of re-reading whole rows (would cut the 1 h to 90 d reads by an order of magnitude); a leaner overview point.

## Proof summary
Fleet crate 18/0, terminal laws `fleet_window` 13/0 (the whole laws crate has 14 failing tests in docking and source-editor files that this lane did not touch: `docking_window_handle_tooltips` expects a `⠿` handle glyph, the `sources` tests need "caller-owned source editor artifacts"), daemon 63/0, `repo test-file` fleet ts 6/0, collector 29/0, metrics 14/0, product metrics 8/0. `repo typecheck` exit 0 for the TS paths touched. dangling-imports exit 0 for metrics, sidecar metrics, dashboard fleet and terminal.

## Live renders (read-only, 200 x 60 terminal, wide lines cut at their right end only by the 120-cell chart budget)

### Overview, range key 1, language en (read 50.2ms)

```
Fleet · range 5 min · sort problems first · columns all
1-5 range · ←/→ zoom · s sort · c columns · Enter details · Esc back
Origin     Readiness                  Beat       CPU            Memory         Disk           CPU trend        Units/min           Hit L/R     Offload in/out Name
Σ fleet    4/16 ready                            ██░░░╎╎ 29% ok ████▎╎╎ 60% ok ████▌ critical ▄▄▃▃▄▄▄▄▄▄▄▃▃▃▃▃ ▃▃▅▃▂▃▃▃▆ 827/min   - / -       0/0            ▲ 6
Fleet facts: queue 0 running / 0 queued · offloaded in 0 · remote cache 50.0k entries, 71.8MiB
g1-m09     ! missing                  -          no data        no data        no data        ················ ········· -         - / -       -              -
g1-m05     ◌ stale · backoff depende… 4min 42s   █████ critical ████▏╎╎ 59% ok ████▉ critical ·█··█··········· ▄▄▄······ 0/min     - / -       0/0            ok
g1-m12     ◌ stale · backoff workspa… 1min 7s    █▊░░░╎╎ 25% ok ███▎░╎╎ 47% ok █████▍╎ 76% ok ▃▂▂▂▄▃▃▃▃▂▂▂▃▃▃▃ ▄▄▄▄▄▄▄▄▄ 0/min     - / -       0/0            ▲ dup 5
g1-m15     ◌ stale · running workspa… 31min 47s  no data        █████▍╎ 76% ok no data        ················ ········· -         - / -       -              ok
g1-server  ◌ stale                    -          █████ critical ████▋╎╎ 66% ok ████▊ critical ████████████████ ▄▄▄▄▄▄▄▄▄ 0/min     30% / 0%    0/0            ok
g1-m02     running workspace 5/5      6.5s       ████░╎╎ 57% ok ████▉╎╎ 70% ok ████▎ 84% warn ▇▇▇▆▇█▇▇▆▇█▇▆▆▆▅ ▃▃▄▃▂▃▃▃▆ 716/min   - / -       0/0            ok
g1-m04     waiting dependencies 3/5   19s        █▋░░░╎╎ 22% ok ██▊░░╎╎ 39% ok ████▉ critical ▅▅▄▄▅▄▆▅▄▃▃▃▄▃▂▂ ▄▄▄▄▄▄▄▄▄ 0/min     - / -       0/0            ok
g1-m06     backoff workspace 5/5      57s        █▌░░░╎╎ 21% ok ████▌╎╎ 64% ok ████▋ critical ▂▃▂▂▂▂▂▃▃▂▂▂▂▂▂▂ ▄▄▄▄▄▄▄▄▄ 0/min     - / -       0/0            ok
g1-m07     running workspace 5/5      5.9s       █░░░░╎╎ 14% ok ███▊░╎╎ 54% ok ████▏ 84% warn ▃▂▃▂▂▂▂▂▃▃▃▂▂▂▂▂ ▄▄▄▄▄▄▄▄▄ 0/min     - / -       0/0            ok
g1-m08     backoff workspace 5/5      59s        █▋░░░╎╎ 22% ok ████▊╎╎ 68% ok ████▋ critical ▃▃▂▃▃▃▃▄▄▅▆▄▅▄▄▃ ▄▄▄▄▄▄▄▄▄ 0/min     - / -       0/0            ▲ dup 5
g1-m10     running workspace 5/5      5.0s       ████▎╎╎ 61% ok █████╎╎ 72% ok ████▋ critical ▃▃▄▅▆▅▄▄▃▃▃▃▃▃▅▅ ▆▃▇▃▅▅▄▃▁ 111/min   - / -       0/0            ▲ dup 5
g1-m13     running workspace 5/5      3.3s       ▌░░░╎╎ 9.0% ok ████▏╎╎ 58% ok ████▎ 85% warn ▄▄▄▄▄▃▃▃▄▃·▂▂▂▂▂ ▄▄▄▄▄▄▄▄▄ 0/min     - / -       0/0            ▲ dup 5
g1-m01     ready workspace 5/5        2.9s       ▋░░░╎╎ 9.7% ok ███▊░╎╎ 53% ok ████▋ critical ▂▂▂▂▂▂▂▃▂▂▂▂▃▃▃▂ ▄▄▄▄▄▄▄▄▄ 0/min     - / -       0/0            ok
g1-m03     ready                      1min 21s   ██▋░░╎╎ 37% ok ████░╎╎ 56% ok ████▋ critical ▄▄▄▃▄▄▄▄▄▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄▄▄ 0/min     - / -       0/0            ▲ dup 5
g1-m11     ready                      1min 20s   █▉░░░╎╎ 26% ok ████▊╎╎ 68% ok ████▊ critical ▃▄▃▃▃▃▃▃▄▄▄▃▃▃▃▃ ▄▄▄▄▄▄▄▄▄ 0/min     - / -       0/0            ok
g1-m14     ready workspace 5/5        5.6s       ██▋░░╎╎ 37% ok ████▏╎╎ 59% ok ████▋ critical ▆▅▄▄▄▅▄▅▆▅▄▄▃▄▄▅ ▄▄▄▄▄▄▄▄▄ 0/min     - / -       0/0            ▲ dup 5
```

### Overview, range key 2, language en (read 334.5ms)

```
Fleet · range 1 h · sort problems first · columns all
1-5 range · ←/→ zoom · s sort · c columns · Enter details · Esc back
Origin     Readiness                  Beat       CPU            Memory         Disk           CPU trend        Units/min           Hit L/R     Offload in/out Name
Σ fleet    4/16 ready                            ██░░░╎╎ 29% ok ████▏╎╎ 59% ok ████▌ critical ▅▅▅▅▅▅▅▅▄▄▄▄▄▄▄▃ ▂▂▁▂▁▁▁▁▅ 78.1/min  0% / -      0/0            ▲ 6
Fleet facts: queue 0 running / 0 queued · offloaded in 0 · remote cache 50.0k entries, 71.8MiB
g1-m09     ! missing                  -          no data        no data        no data        ················ ········· -         - / -       -              -
g1-m05     ◌ stale · backoff depende… 4min 43s   █████ critical ████▏╎╎ 59% ok ████▉ critical ███·████▇█▇▆▇███ ▄▄▄▄▄▄▄▄▄ 0/min     - / -       0/0            ok
g1-m08     ◌ stale · backoff workspa… 1min       █▋░░░╎╎ 22% ok ████▊╎╎ 68% ok ████▋ critical ▅▄▅▅▆▅▅▅▄▄▅▄▄▄▃▄ ▁▃▃▁▁▁▁▁▁ 0.74/min  - / -       0/0            ▲ dup 5
g1-m12     ◌ stale · backoff workspa… 1min 7s    █▊░░░╎╎ 25% ok ███▎░╎╎ 47% ok █████▍╎ 76% ok ▅▅▅▅▅▅▄▅▆▅▄▅▅▆▃▃ ▄▄▄▄▄▄▄▄▄ 0/min     - / -       0/0            ▲ dup 5
g1-m15     ◌ stale · running workspa… 31min 48s  ████▉ critical █████▍╎ 77% ok ████▌╎╎ 64% ok ███████▇········ ▂▅▆▁▁···· 11.8/min  - / -       0/0            ok
g1-server  ◌ stale                    -          █████ critical ████▋╎╎ 66% ok ████▊ critical ████████████████ ▁▁▅▁▁▁▁▁▁ 925/min   32% / 0%    0/6527         ok
g1-m02     running workspace 5/5      1.0s       ████░╎╎ 57% ok ████▉╎╎ 70% ok ████▎ 84% warn ▆▆▆▆▅▅▆▅▆▅▅▆▆▆▆▇ ▁▁▁▁▁▁▁▂▅ 15.2/min  - / -       0/0            ok
g1-m04     waiting dependencies 3/5   20s        █▋░░░╎╎ 22% ok ██▊░░╎╎ 39% ok ████▉ critical ▆▆▆▆▇▇▇▇▆▆▆▄▄▃▄▄ ▄▄▄▄▄▄▄▄▄ 0/min     - / -       0/0            ok
g1-m06     backoff workspace 5/5      58s        █▌░░░╎╎ 21% ok ████▌╎╎ 64% ok ████▋ critical ▆▆▅▅▅▅▄▅▃▃▃▃▂▃▃▂ ▁▅▃▄▅▂▄▄▁ 0.28/min  - / -       0/0            ok
g1-m07     running workspace 5/5      1.0s       █░░░░╎╎ 14% ok ███▊░╎╎ 54% ok ████▏ 84% warn ▄▄▃▃▃▃▃▄▃▂▂▃▃▄▃▂ ▆▃▁▁▁▁▁▁▁ 5.46/min  0% / -      0/0            ok
g1-m10     running workspace 5/5      5.8s       ████▎╎╎ 61% ok █████╎╎ 72% ok ████▋ critical ▅▅▆▄▅▄▅▅▅▄▄▃▃▄▄▄ ▂▂▂▂▂▁▂▁▅ 13.5/min  - / -       0/0            ▲ dup 5
g1-m13     running workspace 5/5      4.1s       ▌░░░╎╎ 9.0% ok ████▏╎╎ 58% ok ████▎ 85% warn ▃▅▄▃▂▂▄▅▂▃▂▂▂▂▄▃ ▄▄▄▄▄▄▄▄▄ 0/min     - / -       0/0            ▲ dup 5
g1-m01     ready workspace 5/5        3.6s       ▋░░░╎╎ 9.7% ok ███▊░╎╎ 53% ok ████▋ critical ▇▇▅▇▅▅▅▅▃▃▂▃▂▂▃▂ ▆▆▃▇▄▁▁▁▁ 40.3/min  0% / -      0/0            ok
g1-m03     ready                      1min 22s   ██▋░░╎╎ 37% ok ████░╎╎ 56% ok ████▋ critical ▇▅▄▅▅▅▅▅▄▄▄▄▄▃▄▄ ▅▁▁▁▁▁▁▁▁ 3.23/min  - / -       0/0            ▲ dup 5
g1-m11     ready                      1min 21s   █▉░░░╎╎ 26% ok ████▊╎╎ 68% ok ████▊ critical ▄▄▃▃▄▃▃▄▄▄▃▃▃▄▃▃ ▄▄▄▄▄▄▄▄▄ 0/min     - / -       0/0            ok
g1-m14     ready workspace 5/5        868ms      ██▋░░╎╎ 37% ok ████▏╎╎ 59% ok ████▋ critical ▇▇▆▆▆▆▆▆▆▆▅▆▆▅▅▄ ▁▁▁▁▁▁▁▂▄ 0.07/min  0% / -      0/0            ▲ dup 5
```

### Overview, range key 3, language en (read 524.6ms)

```
Fleet · range 24 h · sort problems first · columns all
1-5 range · ←/→ zoom · s sort · c columns · Enter details · Esc back
Origin     Readiness                  Beat       CPU            Memory         Disk           CPU trend        Units/min           Hit L/R     Offload in/out Name
Σ fleet    4/16 ready                            ██▏░░╎╎ 30% ok ████▏╎╎ 59% ok ████▌ critical ··············▅▄ ·······█▄ 84.3/min  0% / -      0/0            ▲ 6
Fleet facts: queue 0 running / 0 queued · offloaded in 0 · remote cache 50.0k entries, 71.8MiB
g1-m09     ! missing                  -          no data        no data        no data        ················ ········· -         - / -       -              -
g1-m05     ◌ stale · backoff depende… 4min 44s   █████ critical ████▏╎╎ 59% ok ████▉ critical ··············▅▇ ·······▄▄ 0/min     - / -       0/0            ok
g1-m06     ◌ stale · backoff workspa… 1min       █▌░░░╎╎ 21% ok ████▌╎╎ 64% ok ████▋ critical ··············▄▃ ·······█▄ 0.55/min  - / -       0/0            ok
g1-m08     ◌ stale · backoff workspa… 1min 2s    █▋░░░╎╎ 22% ok ████▊╎╎ 68% ok ████▋ critical ··············▅▄ ·······█▄ 1.32/min  - / -       0/0            ▲ dup 5
g1-m12     ◌ stale · backoff workspa… 1min 9s    ██▏░░╎╎ 31% ok ███▋░╎╎ 51% ok █████▍╎ 76% ok ··············▅▄ ·······▄▄ 0/min     - / -       0/0            ▲ dup 5
g1-m15     ◌ stale · running workspa… 31min 50s  ████▉ critical █████▍╎ 77% ok ████▌╎╎ 64% ok ··············█· ·······▄▄ 3.42/min  - / -       0/0            ok
g1-server  ◌ stale                    -          █████ critical ████▋╎╎ 66% ok ████▊ critical ··············██ ·······█▄ 1.47k/min 35% / 0%    0/32635        ok
g1-m02     running workspace 5/5      2.8s       ████░╎╎ 57% ok ████▉╎╎ 70% ok ████▎ 84% warn ··············▇▆ ·······▁▅ 7.21/min  - / -       0/0            ok
g1-m04     waiting dependencies 3/5   22s        █▋░░░╎╎ 22% ok ██▊░░╎╎ 39% ok ████▉ critical ··············▆▅ ·······▄▄ 0/min     - / -       0/0            ok
g1-m07     running workspace 5/5      2.9s       █░░░░╎╎ 14% ok ███▊░╎╎ 54% ok ████▏ 84% warn ··············▄▃ ·······█▄ 12.9/min  0% / -      0/0            ok
g1-m10     running workspace 5/5      2.0s       ████▎╎╎ 61% ok █████╎╎ 72% ok ████▋ critical ··············▅▄ ·······▁▅ 10.8/min  - / -       0/0            ▲ dup 5
g1-m13     running workspace 5/5      5.9s       ▌░░░╎╎ 9.0% ok ████▏╎╎ 58% ok ████▎ 85% warn ··············▄▃ ·······▄▄ 0/min     - / -       0/0            ▲ dup 5
g1-m01     ready workspace 5/5        5.5s       ▋░░░╎╎ 9.7% ok ███▊░╎╎ 53% ok ████▋ critical ··············▆▃ ·······█▄ 36.6/min  0% / -      0/0            ok
g1-m03     ready                      1min 24s   ██▌░░╎╎ 35% ok ████░╎╎ 57% ok ████▋ critical ··············▆▄ ·······█▄ 7.25/min  - / -       0/0            ▲ dup 5
g1-m11     ready                      1min 23s   █▉░░░╎╎ 26% ok ████▊╎╎ 68% ok ████▊ critical ··············▄▄ ·······█▄ 9.50/min  - / -       0/0            ok
g1-m14     ready workspace 5/5        2.7s       ██▋░░╎╎ 37% ok ████▏╎╎ 59% ok ████▋ critical ··············▆▅ ·······▁▅ 0.04/min  0% / -      0/0            ▲ dup 5
```

### Drill-down g1-m01, range key 1, language en (read 4.9ms) (first lines)

```
g1-m01 · 5 min · ready workspace 5/5
1-5 range · ←/→ zoom · s sort · c columns · Enter details · Esc back
                                   ┬──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┬
                                   4min 59s                                                                                                             now
▾ Host
CPU busy share                                                                                       ▃                                   ░         ▃        now 9.7%  min 3.7%  max 77%
                                   ▃▃▃▃▃▃▃▄▅▅▅▅▅▆▄▆▂▂▂▂▂▂▂▂▂▂▂▃▄▂▂▂▂▂▂▂▂▃▃▂▂▂▂▂▂▂▂▂▂▃▂▂▂▂▂▂▂▂▂▂▂▄▃▃▆▃█▃▄▄▄▅▆▇▃▃▄▅▃▃▃▃▃▃▄▃▃▃▄▅▄▄▅▅▃▅▆▅▇▆▇▆█▅▄▄▆▄▃▄▃▄█▄▆▅▆▄▂▂
Load average (1 min)               ███▇▆▆▅▅▅▅▅▄▄█████▆▆▅▄▄▄▄▃▂▂▂▂▂▂▂▁▁▂▂▂▁▁▁▁                                                                               now 1.85  min 1.85  max 4.91
                                   █████████████████████████████████████████████████▇▇▇▆▆▆▆▆▆▆▅▄▃▃▃▃▃▄▄▄▄▄▃▃▄▄▄▄▄▄▄▃▃▃▃▃▂▂▃▄▄▄▄▄▄▄▃▃▄▄▆▆▆▇▇▆▆▆▆▆▅▅▅▅▅▄▄▄▃▂▁
Memory available                   ▅▅▆▆▇▇▇▇▅▂▂▁▂▂▁▁▃▃▃▃▂▃▂▂▂▆▆▆▆▆▆▆▆▆▆▆▅▅▅▇███████████████████▇▇▆▆▆▆▆▆▆▅▃▂▂▂▂▂▁▁▁▁▁▁▁▁▁                                 ███ now 7.53GiB  min 4.19GiB  max 7.5…
                                   █████████████████████████████████████████████████████████████████████████████████████████▇▇▇▇▇▇▇▅▆▆▅▄▄▃▃▅▆▅▅▆▆▇▇▇▇▆▄▄███
Swap used                                                                                                                                                   now 1.94GiB  min 1.94GiB  max 1.9…
                                   ████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
Disk free                          ▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▂▂▂▂▂▂▂▂▂                                                      ████████ now 18.8GiB  min 18.4GiB  max 18.…
                                   ████████████████████████████████████████████████████████████████████████▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▁▁▁▁▁█████████████▇▇▇▇▇████████
Repo tool processes                         ▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃                                                                       ▃▃▃▃▃▃▃▁  ▁▃▃▃▃▃▃▃▆█████ now 9  min 6  max 9
                                   ▁▁▁▁▁▁▁▁▆████████████████▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆████████▆▆██████████████
▾ Build
Compilation tasks running                  ████████████████████████████████████████████████████████████████████████████████████████████████████████████████ now 1  min 0  max 1
                                   ▁▁▁▁▁▁▁▁████████████████████████████████████████████████████████████████████████████████████████████████████████████████
Tasks waiting in the queue                                                                                                                                  now 0  min 0  max 0
                                   ████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
Units compiled                                                                                                                                              now 0/min  min 0  max 0
                                   ████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
Units restored from the local cac…                                                                                                                          now 0/min  min 0  max 0
                                   ████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
Units restored from the fleet cac…                                                                                                                          now 0/min  min 0  max 0
                                   ████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
Units stored                                                                                                                                                now 0/min  min 0  max 0
                                   ████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
Units skipped                                                      ████    ▁▁▁▁▁                           ▆▆▆▆▅▅▅▅              ▂▂▂▂                       now 0/min  min 0  max 2.46k
                                   ▁▁▁▁▁▁▁▁▁▆▆▆▆▆▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████▅▅▅▅█████▅▅▅▅▅▅▅▅▅▂▂▂▂▁▁▁▁▁▂▂▂▂▁▁▁▁▁████████▁▁▁▁▁▁▁▁▁▁▁▁▁▁████▁▁▁▁▁▁▁▁▁▂▂▂▂▂▁▁▁▁▁▁▁▁
▾ Cache
Local cache hits                                                                                                                                            now 0/min  min 0  max 0
  …
```

### Drill-down g1-m05, range key 2, language en (read 6.5ms) (first lines)

```
g1-m05 · 1 h · ◌ stale · backoff dependencies 3/5
1-5 range · ←/→ zoom · s sort · c columns · Enter details · Esc back
                                   ┬┬─┬─────────────────────────────────────────────────────────────────────────────────────────────────────────────┬┬────┬
                                   1h 57min 50s                                                                                                   10min 50s
▾ Host
CPU busy share                     █████████████████▇▇▇▇████████████████████████████████████████▄▄███▅▅▇▇████████▇▇▇▇▇▅▅▇▇▇█████▇▇▇▇░░░░░▆▆▇███████████████ now 100%  min 8.2%  max 100%
                                   ███████████████████████████████████████████████████████████████████████████████████████████████████▄████████████████████
Load average (1 min)               ⊘ n/a · not measured on windows
Memory available                   ▃▃▃▃▃▄▄▄▄▃▃▃▂▂▃▄▄▃▃▃▄▄▁▁   ▁▁▁▁▁▁▁▁▁▁▁▁▁▂▂  ▁▂▂▂▂▃▂▂░░   ░░▂▄▄▃▃▃▄▄▂▂▁░░  ▁▁▁▂▂ ▂▂▄▄▃▃▃▃▃▂▃▃▂▂▃▃▃▆▆▇▆▆▆▆▄▃▃▃▃▄▄▄▄▄▂▂▂▃▃▂ now 26.0GiB  min 12.8GiB  max 33.…
                                   ████████████████████████▇███████████████████████████▆▆▃████████████████████████▄████████████████████████████████████████
Swap used                          ⊘ n/a · not measured on windows
Disk free                          ▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▁▁▁▁                                                                         ▃▃▃▃▃▃▃▃▃▃▃▂▂▅▅██▇▇▇ now 19.3GiB  min 0B  max 21.5GiB
                                   █████████████████████████████████████▇▇▇▆▆▅▅▅▅▅▅▄▄▄▄▄▄▄▄▄▄▃▃▃▃▃▃▃▃▃▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████████████████████
Repo tool processes                                                                                                                                         now 0  min 0  max 0
                                   ████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
▾ Build
Compilation tasks running                                                                                                                                   now 0  min 0  max 0
                                   ████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
Tasks waiting in the queue                                                                                                                                  now 0  min 0  max 0
                                   ████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
Units compiled                                                                                                                                              now 0/min  min 0  max 0
                                   ████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
Units restored from the local cac…                                                                                                                          now 0/min  min 0  max 0
                                   ████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
  …
```

### Drill-down g1-m01, range key 3, language en (read 204.7ms) (first lines)

```
g1-m01 · 24 h · ready workspace 5/5
1-5 range · ←/→ zoom · s sort · c columns · Enter details · Esc back
                                   ┬┬────────────────────────────────────┬─┬────────────────────────────────────────────────────────────┬─┬───────────────┬
                                   2h 12min                              1h 14min                                                       18min 20s       now
▾ Host
CPU busy share                     ▆▆▇▇▇▇▇▇▇▇▆▆▆▅▄▄▇▇▇▇▁▁▃▃▄▄▄▃▅▅▇▆▆▆▆▆▆▇▇▆▆▆▆▆▅▆▆▄▃▃▁▄▄▆▆▆▅▂▂▁░░▁▁▂░░▁▂▂▂▁▁▁░░░░░░░░░░░░░░░░░░░░░░░░░░  ░░       ░░    ░░░ now 9.7%  min 3.2%  max 100%
                                   ██████████████████████████████████████████████████████████████████████████▇▇▅▅▅▅▄▄▄▄▄▄▄▄▅▅▄▅▅▃▃▃▄▅▅▃▄▄▇▇▅▂▂▂▂▂▂▄▄▄▃▃▅▅▅▅
Load average (1 min)               ▁▁▂▂▂▆▆▇▅▅▄▁▁▂░░▂▂▂▂░░     ░  ▂▂▂▁░░▁▄▄▁▁▁▁▁▁▂▂░                                                                         now 1.85  min 1.85  max 32.0
                                   ██████████████▇▇████▆▆▅▅▇▆▆▇▆▆██████████████████▇▇▆▇▇▇▇▇▇▅▅▅▄▄▅▅▆▅▅▆▅▅▅▅▅▅▄▄▃▃▃▂▂▂▂▂▂▂▂▂▂▂▃▃▃▂▁▁▂▂▂▂▂▂▃▃▂▂▂▂▂▂▂▁▁▁▁▁▁▂▂▁
Memory available                   ░░                 ░▁▁░░░░░░░░          ░░                 ░░░     ░   ░░░░░▁▂▂▁▂▂▂▂▂▃▃▄▁▁▅▃▃▂▂▂▁▁▁▄░░░░▁▂▂▃▄▄▄▃▃▁░░   ▁ now 7.53GiB  min 2.64GiB  max 9.1…
                                   ▅▅▂▂▄▄▄▃▃▃▅▅▅▄▆▆▄▃▃▅████▇▆▆█▆▆▃▄▄▄▅▅▅▃▃▃▅▅▄▄▅▄▄▄▄▄▃▅▅▅▄▄▃▄▄▇▇▇▆▆▇██▇▇▇▇▇▇█▇▇██████████████████████████▇▇████████████▇▆▆█
Swap used                          ░░▁▁░   ░░▃▃▃▃▃▃▄▅▅▁                ░▂▂▂▆▆▆▆▆▅▅▅▁▁                                                                       now 1.94GiB  min 1.85GiB  max 3.7…
                                   ████████████████████▆▆▆▆▅▅▅▅▅▅▅▆▆▅████████████████▅▃▃▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▁▁▁▁▁▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂
Disk free                          ▂▂  ░  ░░░▁▁▂▁▁▂▂▂▁░░  ▁▂▂▂▂▂▂▄▄▅▅▅▄▄▄▃▃▃▃░          ░▇▇▅▂▂     ▃▄▄▄▄▄▄▄▃▁▁ ░░▁░░░░ ▂▂▂▂▂▂▂▂▂▂▂▂▂▃▃▃▂▂▃▃▃▃▃▃▃▃▃▃▃▄▄▄▃▃▃▄ now 18.8GiB  min 11.6GiB  max 21.…
                                   █████▇▇██████████████████████████████████████▇▅▅▃▂▂▂▂▄█████▆▃▃▃▃███████████▇▇▇███▆▆▅████████████████████████████████████
Repo tool processes                ░░░░▃▄▄▄▃▃░▁▁░░░▂▄▄▁           ░░░▃▃▁░░░░░░░░░░░░░░▁▁▂▄▄▂░░            ░░░                                               now 9  min 1  max 18
                                   ████████████████████▅▅▄▄▅▅▅▄▄▄▄▆▆▇███▇▇▇██████████▇██████▆▆▂▃▃▄▄▄▄▄▅▆▆▆▇▇▆▅▅▃▃▃▃▃▃▃▃▂▂▂▃▃▃▄▄▄▃▃▃▃▂▂▄▄▄▆▆▇▇▇▆▆▆▆▆▆▆▆▆▇▇▇█
▾ Build
Compilation tasks running          ░░░░▂▅▅▄▃▃░░░   ▁▂▂░           ░░░░░░  ░░░░░▁▁▁░░░░▂▂▃▅▅▄░░                                                              now 1  min 0  max 16
                                   ▇▇█████████▇▇▆▆▆████▂▂▂▂▃▂▂▂▂▂▃▆▆▇██▇▆▆▇█████████████████▇▇▃▂▂▃▃▄▄▄▅▆▆▆▅▅▄▅▅▃▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂
Tasks waiting in the queue         ░░                                      ░░    ░░   ░░░░░▃▄▄▄▅▅▂▂    ░░░░░▄▄█▇▇░                                          now 0  min 0  max 4
                                   ▁▁▁▁▁▁▁▁▂▂▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▃▁▁▁▁▁▁▁▁▂▂▄▄▄▅▅▅▃▃▂▁▁██▅▅▅████████▃▂▂▁▆▆▇▇▇█████▇▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁
Units compiled                                                     ░░           ░       ░▁▁              ░░░░░░                                             now 0/min  min 0  max 324
                                   ▂▂▃▃▅▄▄▄▃▃▃▃▄▁▁▂▂▂▂▁▁▁▁▁▁▁▁▁▁▁▁▁██▅▄▄▄▂▂▂▂▄▄▄▇▅▅▄▃▃▁▁▄██▆▄▄▂▄▄▂▂▁▁▁▅▅▅▃▃███▅▅▅▄▄▄▃▃▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁
  …
```

### Drill-down g1-m05, range key 2, language de (read 8.8ms) (first lines)

```
g1-m05 · 1 Std. · ◌ veraltet · backoff dependencies 3/5
1-5 Zeitraum · ←/→ Zoom · s Sortierung · c Spalten · Eingabe Details · Esc zurück
                                   ┬┬─┬─────────────────────────────────────────────────────────────────────────────────────────────────────────────┬┬────┬
                                   1Std                                                                                                           10Min 50s
▾ Rechner
CPU-Auslastung                     █████████████████▇▇▇▇████████████████████████████████████████▄▄███▅▅▇▇████████▇▇▇▇▇▅▅▇▇▇█████▇▇▇▇░░░░░▆▆▇███████████████ jetzt 100%  min 8,2%  max 100%
                                   ███████████████████████████████████████████████████████████████████████████████████████████████████▄████████████████████
Last im Mittel (1 Min.)            ⊘ n. v. · auf windows nicht gemessen
Speicher verfügbar                 ▃▃▃▃▃▄▄▄▄▃▃▃▂▂▃▄▄▃▃▃▄▄▁▁   ▁▁▁▁▁▁▁▁▁▁▁▁▁▂▂  ▁▂▂▂▂▃▂▂░░   ░░▂▄▄▃▃▃▄▄▂▂▁░░  ▁▁▁▂▂ ▂▂▄▄▃▃▃▃▃▂▃▃▂▂▃▃▃▆▆▇▆▆▆▆▄▃▃▃▃▄▄▄▄▄▂▂▂▃▃▂ jetzt 26,0GiB  min 12,8GiB  max 3…
                                   ████████████████████████▇███████████████████████████▆▆▃████████████████████████▄████████████████████████████████████████
Swap belegt                        ⊘ n. v. · auf windows nicht gemessen
Festplatte frei                    ▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▁▁▁▁                                                                         ▃▃▃▃▃▃▃▃▃▃▃▂▂▅▅██▇▇▇ jetzt 19,3GiB  min 0B  max 21,5GiB
                                   █████████████████████████████████████▇▇▇▆▆▅▅▅▅▅▅▄▄▄▄▄▄▄▄▄▄▃▃▃▃▃▃▃▃▃▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████████████████████
Repo-Werkzeugprozesse                                                                                                                                       jetzt 0  min 0  max 0
                                   ████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
▾ Bauen
  …
```

