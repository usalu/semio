STATUS: NO PANE PROVEN END TO END. act18 exited 130 (SIGINT, 14:35) not green; W3 window open (14:37), B5 batch W-b5-1 rc 1 (22:0x), B5P/B6 markers absent; rtpanes s9 running: aggregator FAIL, verfolgen FAIL (stale lane), 6 others not verified; act20 not started (no act20.status at 02:15); W3 B6 done; no browser run (refreshed 02:15)

# 1102 Audit Matrix (aud-matrix)

Read-only, built only from `C/1102-result-*.md`, `C/1102-audit-*.md`, `C/1102-window-plan.md` and `C/🗑️generated/1102/*.{rc,txt,status}`. Cell format `<log> rc <n> (HH:MM)`; `running` = log growing, no rc; `queued` = slot file only; `stale` = pre-dates the current window or run (for example rtpanes rc files before s9). Log paths are under `C/🗑️generated/1102/`.

Pane to lane: generator=gen (+flowext node graph), koordinator=cad (+cadi18n), aggregator=puzzle, energie=energy, aussuchen=hubplug (sourcing), bearbeiten=hubplug (process3d), verfolgen=gis, statik=fem. Cross-cutting: rtpanes, runtime, testkit, reacthost, testport, stdio, capext, describe, actinfra, e2eready, policy, phase, stale, draw2d.

## Matrix

| Pane (lane) | wasm/component (act18 / capext) | native tests | smoke (rtpanes s9) | pending patches | blocking faults | browser (e2eready) |
|---|---|---|---|---|---|---|
| generator (gen) | act18 exited 130 before a verdict; capext sourcing/cad wasm rc 0 only | `gen-t6` rc 1 (17:38, stale): compile/test failure, not re-read this refresh | `rtpanes-generator` rc 1 (17:40, stale, pre-s9); s9 not yet reached | 0 in gen dir | not classified this refresh | not run |
| koordinator (cad) | `capwasm-cad-aec-building-w2` rc 0 (13:50) | `cad-t8` rc 1 (20:38), `cad-t7` rc 1 (19:12) | `rtpanes-koordinator` rc 1 (17:50, stale, pre-s9) | cadi18n 37 files (22 diffs) written, applied status per W batches | not classified this refresh | not run |
| aggregator (puzzle) | not in capext | `puzzle-p5` rc 1 (17:08, stale) | `rtpanes-aggregator` rc 1 (18:15): 0 passed / 1 failed, s9 at 21:25 reports FAIL | 0 | aggregator smoke fails (test line in `rtpanes-aggregator.txt`) | not run |
| energie (energy) | none | `etest-18` rc 1 (20:58, "ok 0 passed": no tests ran); `etest-17` rc 1 (18:43); `etest-16` rc 1 (16:37) | `rtpanes-energie.txt` rc 1 (14:08, stale): 0 passed / 2 failed; not re-run in s9 | bridge patch applied earlier | no test ran in etest-18; cache write fault in etest-12 (14:42) | not run |
| aussuchen (hubplug, sourcing) | `captest-sourcing-beams/slabs/windows-t6` rc 0 (19:50-20:45); `capwasm-sourcing-windows-w1` rc 0 (05:06) | `hubplug-s7` rc 1 (19:15), `hubplug-s6` rc 1 (17:53) | `rtpanes-aussuchen` rc 1 (17:41, stale, pre-s9) | 0 in hubplug dir | `hubplug-s7` failures not parsed this refresh (s6/s7 rc 1 only); earlier s2 had 4 failing tests in `sourcing-curation --lib` | not run |
| bearbeiten (hubplug, process3d) | `capwasm-process-concrete/metal/robotic/wood` rc 0 (04:49-04:59) | same lane as aussuchen | `rtpanes-bearbeiten` rc 1 (17:42, stale, pre-s9) | 0 | not separately known | not run |
| verfolgen (gis) | not in capext | `gis-t12` rc 1 (20:59): 82 passed / 2 failed | `rtpanes-verfolgen` rc 1 (19:08): FAIL, s9 reports Stale lane | 0 | 2 gis failures in t12; Stale lane blocks the smoke | not run |
| statik (fem) | not in capext | `fem-t11` rc 1 (17:40); `fem-t10` rc 1 (17:11); `fem-t6` rc 1 (14:05, 18 passed / 1 failed) | `rtpanes-statik` rc 1 (17:40, stale, pre-s9) | 0 | 1 failing fem test (`actual_owner_receives_a_ten_thousand_node_fixture_with_bounded_identity_work`, t6) | not run |

Note: the `rtpanes-*.rc` files for generator, koordinator, aussuchen, bearbeiten and statik are from 17:40-17:50, before the s9 run; treat as stale until s9 writes new ones.

## Cross-cutting

| Lane | Status | Latest log (rc, time) | Pending patches | Blocking faults |
|---|---|---|---|---|
| rtpanes (smokes) | s9 RUNNING (21:25), 6 panes FIFO | `rtpanes-verfolgen` rc 1 (19:08); `rtpanes-aggregator` rc 1 (18:15); `rtpanes-seq-s7` done (19:08) | 2 (one DO NOT APPLY) | aggregator and verfolgen FAIL; the other six not verified in s9 |
| runtime laws | RED, advancing: stall fixed (continuation patch and job close Complete to Pending applied in W2) | `runtime-r15` rc 1 (21:11): 2 passed / 2 failed at terminal fault-result-page heap accounting; `runtime-r16.txt` running, no rc | 1 | `editor-action-producer` 0/1; `editor-task-resume` 2/4 |
| stale (new lane) | ROOT CAUSE FOUND by code reading; patch written, not applied, not compiled; law test written, not run | `1102-result-stale.md` (21:35) | 1 | rtpanes verfolgen fails on Stale |
| draw2d (new lane) | DONE except load-sensitivity: 931 pass / 0 fail across 84 files at original budgets; ts2d boolean suites 58/0; tsc strict clean | `draw2d-suite9.txt`, `draw2d-tsc7.txt` (21:38, latest) | n/a | none recorded; B11 landed (`win-B11-done`: skipped, draw plugin still calls BooleanJob.advanceWork per e2eready) |
| W3 window | OPEN (`window-open` present, note 14:37) | W-b1 to W-b10, W-tsn-1 rc 0 (18:46), W-flowext-w2 rc 0 (20:53), W-runtime-w1 rc 0; `W-b5-1` rc 1 (22:0x, sqlite snapshot compile error in `pack::snapshot::ArtifactSqliteSnapshot` import), `W-b5-2` rc 1 (22:34), `W-b5-3` rc 1 (22:5x), `W-b5-4` rc 0 (23:00); `win-B5-done` present (23:00): 11 patches applied, wasm green for 15 crates, own notes say NOT VERIFIED for the rest | see window plan | B5P: `W-b5p-1` rc 1, `W-b5p-2` rc 1, `W-b5p-3` rc 1 (23:27), `W-b5p-4` rc 1 (23:44); `win-B5P-done` present (00:01); `win-B6-done` present (01:04, note empty); act20 not started as of 01:12 |
| actinfra | task 3 finished in act18 cancel | `actinfra-wcomp.rc` (see log) | 2 | cache put refused at 1 GiB cap earlier |
| e2eready | READY, dry-load 8/8 | no serve marker; browser run not started | 0 | browser actions unverified |
| policy | 57 prepared, NOT compiled; audit FAIL | none new | 115 files, 57 diffs | 12 blocking audit findings (from `1102-audit-patchset.md`) |
| testport | RED | `testport-W-b4c` rc 0 (18:42), `testport-W-b4b` rc 1 (18:29) | 1 | compile errors in earlier batches (stale) |
| reacthost | RED | `reacthost-5` rc 1 (21:09); `reacthost-4` rc 0 (19:46) | 0 | `reacthost-5` fault (not classified) |
| stdio | RED | `stdio-t2-json` rc 1 (14:40) | 1 | json workspace native tests |
| capext | wasm --lib GREEN (11/11) | `captest-*-t6` rc 0 for sourcing beams/slabs/windows (19:50-20:45) | 0 | none |
| describe | wasm GREEN | `describe-1` rc 0 (04:11) | 1 | native tests not rerun |
| flowext | wasm GREEN incl. flow-primitive w2 rc 0 (14:18) | `flowext-run-w2` done (14:18); `flowtest-host5` rc 1 (21:03) | 1 | `flowtest-host5` fault (not classified) |
| uii18n | bun tests 4/4 GREEN (02:28) | `uii18n-bun` rc 0 | 6 | tsc, vitest, browser, Rust patches unverified |
| cadi18n | patch set written, not applied | none | 37 files, 22 pending | not compiled |
| winplan | PLAN CURRENT 13:58 | `winplan-seq.txt` 103 OK / 0 FAIL on scratch copy | 103 diffs | apply order: uii18n before cadi18n |

## act18

- Exit 130 at 14:35:50 ("SIGINT received: ... leases released"), last progress `[32 / 77]` at `semio-s-plugin-sourcing-slabs-component`, 4066 s. Failed actions: none. Not green, and cancelled rather than failed.
- `act-green` absent. `window-open` present since 14:37, so the freeze lifted for the W3 window.
- act20: `C/🗑️generated/1102/act20.status` does not exist yet.

## patches queued

| Lane | Dir files | Pending diffs (winplan) | Verdict |
|---|---|---|---|
| policy | 115 | 57 | audit FAIL, do not apply |
| cadi18n | 37 | 22 | written, not compiled |
| uii18n | 6 | 6 | written, not compiled |
| stale | 1 | 1 | written, not applied |
| actinfra | 2 | 1 | seq OK |
| rtpanes | 2 | 0 (1 DO NOT APPLY) | |
| phase | 4 | not in winplan | |
| flowext | 1 | 0 | script dry-run only |
| testkit | 1 | 1 | not applied |
| testport | 1 | 0 | script on scratch copy OK |
| runtime | 1 | 0 | |
| describe | 1 | 0 | |
| stdio | 1 | 0 | dry-run plans 17 edits |
| cad, capext, energy, gis, hubplug, puzzle, reacthost, serveprep, draw2d | 0 | 0 | |
