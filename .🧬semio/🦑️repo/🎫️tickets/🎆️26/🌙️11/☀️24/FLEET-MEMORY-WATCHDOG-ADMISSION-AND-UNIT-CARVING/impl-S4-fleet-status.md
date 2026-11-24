# S4: per-worker memory and warden in `repo fleet status`

B = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚢️fleet-status`.

## Reader choice
Ended with the shared reader: `readHostWardenMirrorV1(path)` (from `…/🛡️warden/🔁️loop/🟦️.ts`, it appeared while I worked) validates each `memory.json`. The size bound (`readers.maximumMemoryRecordBytes` 262144, by `stat`) is mine. The coordinator reads `readHostWardenStateV1(hostCapacityDirectoryV1(env), now)`.

## Changes (only the fleet-status folder)
- `B/🧬️schema/🔣️.json`:
  - Policy: `findings.workerMemoryStaleSeconds` 60, `workerSwapWarnBasisPoints` 2500, `wardenOverheadBasisPoints` 500, `readers.maximumMemoryRecordBytes`. `PolicySchema` pinned alike.
  - Source id `memory`.
  - New definitions `WorkerMemory`, `MemoryFact {local, origins{origin: WorkerMemory|null}}`, `WorkerReportRow` (= `WorkerRow` + required `memory`, used by `WorkersSection.rows`). `Facts.memory`. Optional `Report.memory.warden` (the local warden).
  - `WorkerMemory` fields: the requested ones, plus `guardMegabytes` and `dangerMegabytes` (the tight finding names them). `lastAlarm.text` is `{en,de}`. An `absent` record has every number null.
  - Messages (EN+DE): `workers.row` extended (`; memory …`, before the refusal), `workers.memory*`, `workers.level.*`, `workers.mode.*`, `workers.warden.{missing,absent,stale}`, `memory.warden`, `finding.worker-memory-tight`, `-warden-intervened`, `-warden-absent`, `-warden-observing`, `-swap-high`, `-warden-overhead`.
- `B/🟦️.ts`: types `FleetWorkerMemoryV1`, `FleetMemoryFactV1`; source `memory`; `workersSection` joins by origin (`memory` null without a record); `memory.warden`; findings produced before `source-failed`, same sort and truncation (30); text columns on the workers row and a `Warden <origin>:` line for the local warden.
- `B/📥️readers/🟦️.ts`: `workerMemoryOfV1` (mirror to fact; the last alarm is the newest by `atMs`), `readFleetMemoryV1` (lists `<store>/v1/origins/*`, missing, oversized, half-written or invalid file gives null, never throws, sync and small so inside the reader budget), registered as `memory` in `FLEET_READERS_V1`. The local record replaces the store record of `SEMIO_FLEET_ORIGIN` (default hostname).
- `B/🧪️tests/🟦️.ts`: 5 new tests (calm; absent record, missing file, stale and offline worker; guard; danger with cancel alarm in EN and DE; observe, swap, overhead and a local warden without worker row; reader on a temp store with fresh/absent/invalid/half/oversized/missing; newest alarm).
- `B/🧪️tests/🐍️.py` (independent oracle): the same findings, the join and `memory.warden`, plus a seeded `memory` generator. The existing coverage test (every `finding.*` text is produced by a seeded case) and the 450 oracle comparisons cover the new ids.

## Semantics
- Memory findings only for a fresh (age <= 60 s), non-absent record. `worker-warden-absent` only for liveness `working|ready|idle` and only when the memory source answered. Reasons: no file, absent record, record older than 60 s.
- `worker-warden-intervened`: `suspended+cancelled > 0`, alert if `cancelled > 0`. The counters are cumulative since the warden started.
- `worker-swap-high`: swap*10000 > total*2500. `worker-warden-overhead`: `cpuBasisPoints` > 500.

## Tests
`repo test-file B/🧪️tests/🟦️.ts`: `23 pass, 0 fail, 2503 expect() calls` (before: 18 pass).

## Live (this machine, 2026-11-24)
`repo fleet status --json`: source `memory` ok (2.2 s with the others under load). Workers already mirror: 12 of 14 rows have a record, all `mode observe`, `level calm`, counters 0/0/0; `g1-m09` and `g1-server` have `memory: null` (no `memory.json`, not alive, so no absent finding). Example row `g1-m05`: available 31840 of 65447 MB, swap/compressed null (Windows), overhead cpu 15.3 bp, tick p95 30 ms, 59 MB. Local `memory.warden` is null (no local warden state on this machine).

Text EN: `g1-m01 macos: working, stage workspace (5/5), attempt 9, record 3 s, machine 1 s; memory free 7331 MB, swap 1526 MB, compressed 1291 MB, calm (observe), interventions s/r/c 0/0/0`.
Text DE: `g1-m01 macos: arbeitet, Stufe workspace (5/5), Versuch 9, Eintrag 2 s, Maschine 0 s; Speicher frei 5171 MB, Swap 1526 MB, komprimiert 3082 MB, ruhig (beobachtend), Eingriffe a/f/b 0/0/0`. A worker without a record: `g1-m09 -: no record, …; memory -`.
Language is chosen by `--lang=en|de` (or `SEMIO_LANGUAGE`).

New findings in the live output: none visible. `worker-warden-observing` (info) is produced for 12 workers but the existing cap of 30 findings (sorted alert, warn, info) is filled by 28 `unit-key-machine-dependent` warnings, 1 `source-failed` (storage reader timed out at 5 s under load) and 1 `unit-cache-read-only`. No tight, intervened, absent, swap or overhead condition holds right now.

## Not done
- Python test file `🐍️.py` is run only through the TS test (as before). Other test folders of this product (`🧪️tests/🛡️guard`) were not run.
- `worker-warden-observing` is info and hidden by the 30-finding cap whenever many warnings exist (existing rule kept). No swap-out-rate or compressor finding (only shown in JSON and the `Warden` line).
- Local warden shows only when `readHostWardenStateV1` has a fresh state on the machine running the command; the coordinator has no Ready stage.
