# F25 result: labelled pre-inventory window, inventory attempt/sub-phase, boot phase on host-wait

## Landed (details and hashes: T/landings.md "Wave 1125 F25")
1. Every step between worker start and inventory prints a bilingual progress line the supervisor's `stepLinePatterns` already read: `worker-boot` (module imports; the ticker now starts before them), `slice-prepare` (backend, compiler runtime, native recheck), `registry-read`, `graph-state` (graph state lease wait + store recheck + staging cleanup), `module-load`.
2. The inventory line reads "checking the file inventory of the tree (attempt N): <sub-phase> (counters)" / "Prüfe das Dateiverzeichnis des Baums (Versuch N): <Teilphase> ..." with N counted per process, so a restart at 0/1 is a new attempt and a changed line (progress for the stall detector).
3. Client `host-wait` names the boot phase from the service log: "Waiting for the repo graph service (N ms); it is in source.capture (432 files)." (EN+DE, plain text while the service has said none).

## Measured (this Mac, load 29-41, `repo run @semio-tech/ui-styling-tokens:generate:prototype --dry-run`; trace `W/trace-F25.txt`)
- Before: service wait 22 s, then lines "(2 s)" .. "(68 s)" with NO label (66 s), inventory at 70 s.
- After (run 1, service restarted by the landing): unlabelled 3-53 s (that was the cold worker module load, before the worker-boot patch), `graph-state` 55-65 s, inventory 65 s. After (run 2, final code): unlabelled only 0-6 s; `graph-state` 6-96 s (lease wait behind peers' graph tests, now visible as such), `module-load` 96-111 s, inventory attempt 1 with sub-phase "checking the memo against the tree" then "scanning the owners" at 111-124 s.
- So the silent window was: worker module load (tens of seconds cold), the graph-state lease queue (up to 90 s with peers at load 40) and module-load (15 s). Nothing was a redundant scan; the lease wait is contention, not work.
- Boot of the graph service (task 3): the service log shows boot is `build:source-admission` -> `source.capture` (walks the build's own source closure, about 11 ms per file under load, 1728 files at 20 s) -> `source.served` at ~50 s. A service whose source memo hits goes to `source.served` in 5 s (logs host-97791de433a8, host-a3575468e1e7); a miss walks. The memo implementation digest covers the authorizer files, so every edit there (and any partial change) pays the walk once per machine. No cheap safe fix found; NOT landed.

## Not verified
- Real worker with the new worker.ts on a 16 GB worker; the `host-wait-phase` text in a live cold boot (unit-tested with a stub log, the live service was already up in both runs).
- `step-overdue` test (4 failures, reasoned pre-existing: inventory "seconds since progress" semantics and a 16384-byte tailBytes fixture; new steps pass its law). `dangling-imports.py` exit 0 for all my folders except test-string hits (`slice-worker-scope` string, F19's `wiring` test).
- Last landing was made while F27 held the graph lock; I touched only memory and resident then.
## Other lanes
- F19/F23: `load/🟦️.ts` gained `sliceReadWaitStepV1` import, `registry-read`/`graph-state`/`module-load` wrappers and `{round:++inventoryAttempts}`; tests that extract `loadBuildGraphUnpinnedV1` by source (`candidate`, `adopt-registry`) now bind both step functions.
