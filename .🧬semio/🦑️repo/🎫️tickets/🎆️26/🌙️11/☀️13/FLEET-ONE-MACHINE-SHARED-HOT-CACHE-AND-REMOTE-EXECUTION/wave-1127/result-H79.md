# H79: live proof of the needs-live-proof items of audit-A7-open.md

Read-only lane, no landing. Snapshot 2026-11-27 12:19-12:25 CET: `repo fleet status --json` (143 KB, 13/13 sources ok, 4 s), MCP `remote_cache_status` / `remote_execution_status`, `S/origins/*/ready.json`, `ready-timings.json`, `ready-log-*`, `partitions.json`, `ready-wedge.json`. Scratch: `scratchpad/H79/fs.json`.
Code reality: the stage-input digest per worker shows only m01/m02 (and briefly m01) ran the 12:00-slot code (`14695823b92a` from 12:04, `79c735e01d81` 12:07); all other workers still run `a579a1413f46` (10:00 slot) and refresh in the background (ready.json `refreshing`). So everything landed 10:00 or later is mostly NOT-YET-OBSERVABLE; the live windows of `fleet.rates` (15 min / 1 h / 24 h) are the best evidence for the earlier landings.

## Live baseline (evidence lines used below)
- E1 workers: ready 8 (m02 m07 m08 m11 m12 m13 m14 m15), working 6 (m01 m03 m06 m10 workspace; m04 m05 tools), absent m09 + g1-server (fleet status `workers.counts`).
- E2 `rates.fleet` 24 h: cache.skip.spool-full 0, upload-failed 0, upload-expired 0, upload-expired-down 1, upload-entry-gone 163, key-incomplete 0 (cumulative m01 counters: 27.9k / 14.7k / 611 / 7.9k / 18.0k / 7.3k).
- E3 `rates.fleet` 24 h: exec.decline pin-failed 0, unmapped-path 85, infrastructure 90, no-idle-machine 80, queue-timeout 12, incomplete-inputs 6, worker-policy 4 (cumulative 676 / 3308 / 1672 / 183 / 176 / 165 / 235); offload.out 70 / 828 / 5063 (15 min / 1 h / 24 h), fellback 0 / 15 / 126; destinations by scheduler `offloadedIn`: m11 2516, m12 2050, m13 1877, m15 371, m07 313, m01 435.
- E4 remote cache server (started 10:36 CET, 42 restarts in ledger): origins ok 16, 77 208 entries; divergences per hour 93 (04:00 CET) -> 9 (10:00) -> 4 (11:00) -> 1 (12:00); current-run count 7, all `unit`.
- E5 remote-execution scheduler (started 12:13 CET): 15 workers registered, 15 distinct pins (m01..m08, m10..m15, g1-server), all advertise `semio-checkout-test=1`; queued 0 executing 0 completed 20 failed 5 cached 3.

## Table

| id | verdict | evidence / what would prove it |
|---|---|---|
| H39-1 pruning custody | NOT-YET-OBSERVABLE | no `custody` failure in any worker log since 05:00 (only `Validating current owner custody (0 directories...)` lines); absence is weak proof. Needs a prune during a concurrent build on m12 and a Windows conditional-filter run. |
| H1-1 m03 bundle retry | FAILED (superseded) | m03 workspace stage last ended 12:15 `stage-failed` after 4038 s (digest a579); the bundle is no longer the failure, a new one is (see F1 below). |
| H1-2 bundle guard e2e | NOT-YET | no worker log shows the guard text; needs a run on code >= 10:00 slot that hits a refused authorizer bundle. |
| H2-1 unit-cache registrations / H2-2 denial cause | PROVEN (current) | no `unit-cache-denied`/`read-only` finding in `findings` (28 findings, none), `cache.skip.upload-expired-denied` 0 and `read-only` 0 in 24 h; m11/m12 ready attempts 0. Historic cause stays unproven. |
| H3-1 uploader / cache server | PROVEN | E2: spool-full 0 in 24 h, upload-failed 0; E4 server up since 10:36, `fleet.origins.ok 16`, cas put 22 710 per 15 min. |
| H4-1 offload gate delta | PROVEN | E3: 24 h windows replace the cumulative counters; pin-failed 0, unmapped-path 85 vs 3308, no-idle-machine 80. |
| H5-1 fleet status | PROVEN (sources) / FAILED (payload) | 13/13 sources ok, m04 and m05 present; but JSON is now 143 KB (not 61 KB): `workers.rows` 29.7 KB, `rates.origins` 27.6 KB, `fleet.machines` 10.7 KB, findings 8.4 KB. See F2. m05 tree verdict `lagging` (24 files), m11/m15/m01 `diverged`. |
| H5-2 payload | FAILED | 143 555 bytes, see F2. |
| H6-2 build peaks (threads=1) | NOT-YET | needs same-unit peak before/after; `peakMegabytes` in timings is per stage, not per unit. Event: a cold `repo cargo` of the same crate set on two workers with old and new digest. |
| H7-1 Windows m05 | PARTIAL | m05 passed `dependencies` and `native` (native ok 06:52) and is now at `tools`; its `Directory partition capacity exceeded` line is only in `ready-log-dependencies-failed.txt` (old). Current: tools `stage-timeout` 5124 s at 12:12, `compiler-helpers could not be prepared (Original resource loan deadline exhausted)`. Source-served hang not shown. `ready-wedge.json` fired at 12:10 (silent 1532 s, 51 missed beats) and m05 launcher restarts the supervisor: m05 is still not Ready. |
| H8-1 dependency scopes / H8-2 | NOT-YET | no dependency stage ran on code >= H8 round 3 except m07 `dependencies ok 9 s` (10:25 digest d2e4b0ffcd22) and m12 `ok 4 s`; Install through the merged receipt and the m04 inode census not observed. m04 `Stage tools needs 150000 inodes free ... only 124305` (disk-collecting) shows the inode budget is live. |
| H10-1 ready suite stall | NOT-YET | m04 tools waits `native-fixed-image .cache/b/t (exclusive, 1787 s)` behind process 1607791 shared queued 2890 s; no `door.capacity` restart line. |
| H13-1 host gate m08 | PROVEN (gate) | m08 ready, attempts 1, warden `worker-memory-tight` guard level only (available 3928 MB, swap 1685 MB), no stage kill. |
| H14-1 cache server restart policy | PARTIAL: deferral PROVEN, restart NOT-YET | server runs since 10:36 CET although the 11:00 landing (H68) changed its code: restart deferred 1 h 46 min while `claims.active` 43-47 (oldest 1433 s). Max deferral 3 h => expected restart by 13:36; check `startedAt` then. |
| H15-1 / H49-1 divergences | PROVEN (decline) | E4: 93 -> 1 per hour; current-run count 7 of class `unit`. Remaining kinds in the 11:00 hour: `dep-info+rlib+rmeta+stream` 2 and `stream` 2, e.g. `libwasm_encoder.rlib` m07 vs m10. 40 stream-only pairs still not classified (see H68). |
| H16-1 remote cache probe | PARTIAL | probes nx-self-hosted 404/8 ms, bazel-http 404/15 ms reachable+authorized (read-only probe, 404 is the expected miss). Remote-down cold probe not run (no live experiments rule). |
| H17-1 / H32-1 / H11-* / H12-1 / H20-2 tests | NOT-YET | Rust and TS test files, not fleet state; need `repo test-file` runs by their owners; no worker log covers them. |
| H18-1 PATH runtime inputs | NOT-YET | scheduler property `semio-command-node` differs per worker: macOS hash `3200fbd9...`, Linux m04 `d1de76d8...`, g1-server `883fe1b8...` (server hash differs from the other 9 macOS workers). Evidence that node is NOT identical across workers; it is not yet shown to change a key. See F3. |
| H19-1 broker wedge | NOT-YET | no `broker` wedge line; the only watchdog artefacts are `ready-wedge.json` (supervisor, H65). Needs an observed silent running job. |
| H20-1 duplicate registration | PROVEN | E5: 15 workers, 15 distinct `semio-pin`, no duplicates after the 12:13 scheduler start. (m08 and g1-server register under different host names `semio-g1-m08.local` / `.fritz.box`: the duplicate-hostname finding, user item U-2.) |
| H22-1 fseventsd | NOT-YET (USER) | needs sudo. |
| H23-1 / H23-2 disk guard | PARTIAL | the guard fires live (m02: `.cache/b/r/m/u/sealed has 673 MB free, floor 771 MB`; m15: `.cache/b/r/m has 1294 MB free, floor 1311 MB`, both at ~12:10-12:15) and stages restart; the grandchild test hang is not live evidence. See F4. |
| H25-1 recheck after 10 s | NOT-YET | no admission wait line in logs. |
| H26-1 overflow-message fix | PARTIAL | m01 (12:08) and m06 (12:19) re-ran the stage with the new texts and fail on other causes (F1, F5); m12 attempts 0 ready. No overflow text in any current log. |
| H27-1 / H27-2 / H35-1 / H48-2 compiler-state image | NOT-YET | no `compiler-state` or nested-image line in any worker log; needs a stage that compiles a compiler-state producer (workspace on code >= 05:00 slot). |
| H28-1 ticket generated sweep | NOT-YET | m04 has not completed any stage since the landing (tools failed x4 in 1.3 h). |
| H29-1 / H44-1 / H50-1 test-file and suite offload | NOT-YET | all 15 workers advertise `semio-checkout-test=1` (registration PROVEN) but scheduler shows `completed 20, executing 0`, no `ran on <origin>` line anywhere. Event: `repo test-file` under host verdict `wait` or `SEMIO_TEST_FILE_OFFLOAD=always` on a coordinator file. Windows m05 and Linux m04 `headroom:false`/stage not Ready. |
| H30-1 `repo inputs` | NOT-YET | no worker log. |
| H31-1 cargo preparation | NOT-YET | m03 12:15 built serde_core through the shared lane (log lines `[units] miss ... differs in env:CARGO_ENCODED_RUSTFLAGS` on 4 logged crates). See F6. |
| H33-1 Nx notice / wait | PARTIAL | m03 12:15 Nx run ended `failed with exit status 130` after `Could not capture Nx descendants for cancellation` (m15 12:09). The Nx-vs-host notice lines (`parked`) appear in no log. |
| H36-1 go-plan | NOT-YET | m12/m14/m15 ready (attempts 0/0/5) but Go lease test not rerun. |
| H37-1 cross-machine Nx hit | PARTIAL | `rates.fleet` `cache.remote.nx.hit` 1 / 1 / 19 (15 min / 1 h / 24 h) vs miss 10 / 18 / 434, put 5 / 18 / 184: cross-machine Nx hits exist but are rare; m03's 12:15 run showed `Cache: 0/8 hit`. No promotion-skip line (H37 visible skip) in any log. |
| H38-1 / H43-1 / H43-2 / H56-* graph reads | NOT-YET (graph) | m07 12:00 `Waiting for the repo graph service (12828 ms); it is in source.served` then SIGTERM; m10 attempts 35 `graph read handed over to current code; asking again` x3 at 08:43. Owned by seal window. |
| H40-1 offload to idle machine | PROVEN | E3: offload.out 828 in 1 h, destinations m11/m12/m13 (idle Ready); `lost 0`, `silent 0`, `deadline 20` cumulative. Deadline/blob bound after sidecar hand-over not separately shown. |
| H45-1 lease order | NOT-YET | |
| H46-1 build retry | NOT-YET | |
| H47-1 / H66-1 r image no ENOSPC | FAILED (partial) | no ENOSPC line, but the guard stopped stages on r images at ~12:10 (F4); partitions.json shows `r` used above capacity in sparse bytes on m01 116 GB of 96, m03 80 of 54, m06 80 of 67 (sparse accounting, not free space). One-hour proof not met. |
| H51-1 m05 replica / log mirror | PROVEN (mirror) | `g1-m05/ready-log-tools.txt` updates every minute (12:21) with tool lines and no heartbeat noise; replica lag verdict `lagging` (24 files). |
| H53-1 / H57-1 m04 tools | FAILED | m04 tools: 4 failures within 1.3 h (903, 616, 1918 s) + stall; last error `ResourceLoanDeadlineExhausted` waiting for `native-fixed-image .cache/b/t` behind a shared holder queued 2890 s, and `Stage tools needs 150000 inodes free, 124305 free`. Interactive priority did not unblock it. See F7. |
| H54-1 capabilities-differ | NOT-YET | peer-caused. |
| H55-1 unit-cache guards | PARTIAL | restored.fleet 17 621 / 24 h (m01 117 per 15 min), compiled 158 690, skip.diverged 239 / 24 h, path-dependent 1369; which guard fired per worker is not in rates. |
| H58-1 replica daemon Windows | NOT-YET | |
| H59-1 always-run | NOT-YET | |
| H61-1 bundle rebuild retry | NOT-YET | no `rebuilt` line; digest a579 ran on 10:00 slot code after H61 landing, no refusal text since. |
| H62-1 failure tail 200 lines | PROVEN | all `ready-log-*-failed.txt` have 200 lines plus the ~33-line disk facts block (max 233: m03); no file above 233. |
| H63-1 / H64-1 deleter of image | NOT-YET | |
| H65-1 watchdog in production | PROVEN (fires) / FAILED (m05 still not Ready) | `ready-wedge.json`: m05 12:10 (silent 1532 s, 51 missed beats, launcher restarts), m01 12:17 (silent 90 s). m05 supervisor restart then `the stage process 39192 is gone: it left no exit file`. |
| H67-2 / H70-1 SIGTERM sender | PROVEN (new code) | m02 12:15 (digest 14695823b92a): `Stopped by this readiness supervisor (process 94133) at 11:15:04Z: cancel. The reason was recorded in the stage record before the signal was sent.` Historic 11 SIGTERMs stay unprovable. m15 shows `Process N received SIGTERM from owned execution repository:command (killing process 31058)` (sender named, H75). Note `stopBy` is not in `ready-timings.json` (only in the stage record), see F8. |
| H69-1 rust remap | NOT-YET | |
| H68-1 stream divergences | NOT-YET | one `stream` divergence at 12:01 (m07 vs m10, key bbc92115...) on workers still at the pre-landing code; recheck after workers refresh. |
| H71-1 staged input reader (m04) | PROVEN | 0 occurrences of `Bounded relative file read` in any m04 log; m04 now fails on other causes (F7). |
| H72-1 / H77-1 | NOT-YET | |
| H24-1 / H42-1 | NOT-YET | UI / test, no fleet signal. |
| H21-2 / H21-3 graph heap | NOT-YET | |
| (A7 not-done rows) | unchanged | not live-checkable; the ones touched by fresh evidence: H9-3/H78-1 (m01 still `Bounded closure reference DATA required`, F1), H56-3 (m07/m10 graph reads above), H7-2, H33-2. |

## Follow-ups: see `findings-H79.md` (F1-F8)
