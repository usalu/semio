# Result H5: fleet monitoring readers and metrics pipeline (wave 1127)

## 1. Why `storage`, `workers`, `fleet` timed out (root causes, measured at load 50-80)
- `storage`: `partitionStateV1` called `nativeImageVolumeUsedBytesSyncV1` per mounted partition = `spawnSync diskutil info -plist` (1.3 s each under load) x 8 partitions, one after the other, on the event loop: 10-24 s, and while it ran no deadline timer of any reader could fire.
- `fleet` (remote execution facts): `remoteExecutionSidecarWatchV1("agent").closure().digest` hashed and import-scanned the whole sidecar closure (1032 files) synchronously on every call, twice (create + closure): 3-5 s of blocked loop. The network part (scheduler, cache `/status`, key samples) is only 0.6 s.
- `workers`: tree comparison + hostnames are cheap (0.3-1 s); it failed because the blocked loop starved its deadline/IO.
- `generation: null` is truthful: `.cache/b/g/m/generations/current.json` has `manifest:null, reason:"closure-missing"` (whole graph seal pending, lane H9). Not changed.

## 2. Fixes (all landed; locks released)
- `native-build/💾️storage/🍎️image/⚖️availability`: new `nativeImageVolumeUsedBytesV1` (async `execFile`, cancellable) next to the sync one; shim re-export.
- `build/🧹️cache-gc/🏃️runner`: `mountedPartitionUsageAsyncV1`; status path asks all partitions at once (non-deep only), each with a 3 s probe: a hung volume tool gives `partial` for that partition instead of failing the reader. Deep mode unchanged (sequential).
- `sidecar/🧮️closure` + `🛰️sidecar`: optional persisted closure memory (`SidecarClosureMemoryV1`, path+stamp -> sha256+imports); `status` keeps it in `.cache/f/fleet-sidecar-closure.json` (atomic), uses `.baseline` (no second build). Existing callers unchanged.
- New reader `rates` (source id `rates`) yields one machine at a time and yields to the loop between machines; keeps what it has when the deadline comes.
- Measured `repo fleet status --json`: 10.3 s with 3 sources failed -> 3.0-4.1 s with all 13 sources ok (storage 2.1-3.8 s, fleet 0.3-1.7 s warm, workers 2-3 s at load 50-60). `fleetfacts` alone 3.3 s -> 0.27 s warm.

## 3. Windowed rates (coordinator note)
- `metrics/🪟️windows`: `metricWindowTotalsV1` sums counter rings (ring value = per-sample growth) over trailing windows; share-weighted for the one bucket a window starts in.
- Fleet status: `rates` fact/section/text (15 min / 1 h / 24 h) for 11 counters (units compiled/restored/stored/skipped, local cache hit/miss, offload out/fallen back, refused, warden cancelled), per origin and fleet total; policy `rates` in the schema; en+de texts.
- Finding `worker-warden-intervened` now fires only when the warden cancelled something in the last 15 min (live: m03 m07 m10 m14); counters since start alone are history. Live: 3861 units compiled / 13098 in 1 h / 400361 in 24 h etc.
- Dashboard Fleet window: its cache/offload/units figures are already totals over the selected zoom range (store `📊️fleet` `total(...)` over the range), so it is windowed; no change, not run interactively.
- H2's baseline was folded in round 2 (section 7).

## 4. Metrics pipeline, live evidence (00:19-00:28)
- 15 of 16 origins publish t0..t4 + both fleet sources (`fleet/metrics/remote-cache`, `remote-execution`): t0/t1 age 0-7 s, t2 age <= 31 s, t3/t4 refreshed every 10 min / hourly; g1-m09 has no metrics dir (unprovisioned); g1-m05 (Windows) stale 150-250 s and 41 metrics stale (Windows lane H7).
- Retention bounded: every origin has identical fixed sizes (t0 773 KB, t1 927 KB, t2 3705 KB, t3 2594 KB, t4 5557 KB); rows: t2 ~1208 (20 h), t3 120, t4 20.
- Rollup consistency: fold of the finer tier equals the coarser bucket (count, min, max, sum) for all 30 checked bucket pairs (t0->t1, t1->t2, 15 origins), 0 mismatches.
- Completeness: per origin 56-58 of 66 supported metrics present. Missing everywhere on clients: `cache.remote.*` (ac/cas/nx; only the cache-server origin g1-server has them: origin-scope metrics that only one host feeds) and `graph.read.facts` (only fed during a graph slice read). Fleet scope is fed: `remote-execution` ring has `exec.offload.in`/`exec.queue.*`, `remote-cache` ring has `cache.remote.entries/bytes` (each lacks the other's by design); audit MA3's "fleet-scope never fed" is closed.
- `._t*.ring` AppleDouble stubs: reappear after each service restart on 8 origins (4 KB each), the publisher sweeps them every 10 min; M7 says they cannot be prevented from user space. Harmless; left.

## 5. Tests
- cache-gc `📁️directory` 4 pass (+1 new async/hung-tool test); sidecar `🧮️closure` 7 pass (+1 memory test); fleet-status 32 pass (+1 rates test, oracle `🐍️.py` extended: rates section + windowed warden rule + random rates facts, fixture case `warden-reported-alarm`); metrics 15 pass (+1 windows test).
- `repo typecheck` clean on fleet-status/metrics files; first typecheck (cache-gc, sidecar, status, availability) reported 3 errors only in the importer `native-build/💾️storage/🍎️image/♻️reclaim/🧪️tests/🟦️.ts` (spawnSync overloads, untouched by me). dangling-imports 0.

## 6. Not verified / open
- Edits to the three remote-execution files (closure, sidecar, status) were made before I took a lock while H4 held `landing.lock.remote-execution`; anchored read-modify-write, no conflict seen.
- `repo dashboard` not run (rule); dashboard windowing is by code reading only. Linux/Windows runs of the new code not checked.
- JSON of `repo fleet status --json` is still ~61 KB (workers rows 31 KB, findings 16 KB), not reduced.
- Other findings now visible live: `worker-host-name-duplicate` on 6 origins, `worker-memory-tight` m08.

## 7. Round 2 (coordinator decision: per-reason metrics, client-side remote cache, one rate source)
- 26 new counter metrics (schema-first: catalogue + collector schema + metrics `Sources`): `cache.skip.<reason>` for 12 named unit-cache skip reasons + `other`, `exec.decline.<reason>` for 11 named offload decline reasons + `other` (other = every unlisted reason, so each counted unit lands in exactly one id; test), `cache.remote.refused`. Metric ids may now contain `-` (id pattern in the metrics schema). Collectors `cache-skip-reasons`, `offload-reasons`; the ring id digest changed, so rings restarted (accepted, greenfield). Live 01:00: 14 of 15 origins already publish the new ids (m05 Windows still old, H7), e.g. m11 15 min: `cache.remote.ac.hit` 1087, `exec.decline.cheap` 32.
- Client side `cache.remote.*` on every origin: `remoteCacheFetchV1` (the one fetch behind every cache client) now counts every answer where it is seen (ac/cas/nx x hit, miss, put, conflict, refused) into `.cache/f/remote-cache-client/` (new `🌐️remote-cache/🛰️fleet/🧮️client-counters`, classification of `/ac|cas/<digest>` and `/v1/cache/<hash>`, status 401/403/429 = refused); the product metric source reads those instead of the server ledger (the server host counts its own client like any other). Live: every client origin has `cache.remote.ac.hit` points.
- ONE windowed source: deleted `readSkipGrowthV1`, `.cache/f/fleet-skip-baseline.json` (file removed from this Mac too), `cacheSkipGrewAgoSeconds`, policy `skipGrowthSeconds`, the cumulative `fleet.skips` text line. `unit-cache-denied`/`unit-cache-read-only` findings fire from the 15 min window of `cache.skip.upload-expired-denied` / `cache.skip.read-only` (message shows the window, en+de); oracle `🐍️.py`, fixture and tests updated. The `rates` section now lists 36 counters (rows with all-zero windows are hidden in the text). Live: 15 min / 60 min / 24 h rows for skips, declines and remote cache; 24 h only fills from the ring restart.
- Dashboard: drill-down groups `skips` ("Units not shared"/"Nicht geteilte Einheiten") and `declines` ("Offload declined"/"Auslagerung abgelehnt") plus all nx/cas/refused cache ids in `🎛️dashboard/🧬️schema/📊️fleet` (charts are per-minute rates over the zoom window, i.e. windowed by range). Headless test `a_drill_down_charts_the_rate_of_every_skip_and_decline_reason_in_english_and_german` (terminal crate, unit fleet_window) feeds a drill-down snapshot with those groups and asserts headings, en+de labels and `/min` lines. Result of the cargo run: see section 8.
- Tests: metrics 15, collector 29, fleet-status 32, sidecar product metrics 8, sidecar 7, remote-cache fleet 16 (client counter test), cache/sidecar earlier ones unchanged; `repo typecheck` clean on all touched files; dangling-imports 0.

## 8. Dashboard test run: NOT executed (cargo gate fault, not caused by this lane as far as I can tell)
- `repo cargo ... -- test -p semio-framework-repo-dashboard-terminal a_drill_down` (from `🧰️framework`, and from the crate folder) and `... check` of the dashboard fleet crate all stop before compiling with `Native Cargo owner preparation failed: Cargo workspace has no current source-bound members: 🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/📦️packages/🦀️rust/Cargo.toml` (`🗂️workspaces/🦀️cargo/🟦️.ts:180`). The metrics crate itself checks fine on its own (`check --manifest-path`, 4 s), so the fault is in how the owner of this path dependency is planned (every crate that depends on it fails). Needs the cargo-gate owner (H9 / build lane); logs in the session scratch only.
- So the new Rust test (`a_drill_down_charts_the_rate_of_every_skip_and_decline_reason_in_english_and_german`) is written but neither compiled nor run. Checked instead: the dashboard `Groups` const parses and every id of it is in the catalogue (python, same law the crate tests); data path proven live by the store reader (section 7 numbers); en/de labels exist for all 26 new ids (collector/metrics catalogue test).
- Also not verified: Windows (m05 still on the old ring ids), Linux m04 only via its published rings.
