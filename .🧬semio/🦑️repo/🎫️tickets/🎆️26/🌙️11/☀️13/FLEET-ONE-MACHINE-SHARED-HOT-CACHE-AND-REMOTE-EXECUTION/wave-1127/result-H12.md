# Wave 1127 H12: test reds (incremental)

| item | before | after | change |
|---|---|---|---|
| T3 goal-gate | fails before first test (Ajv draft-07) | 28/0 | already fixed by a peer (Ajv class chosen by `$schema`); verified |
| T10 broker | 12/1 (sourceKlocV1 5 vs 2) | 13/0 | `admission/🟦️.ts` `sourceKlocV1`: a build script with an unresolvable source closure falls back to the directory estimate, no longer to the entry's module closure |
| typecheck profile test | 2 errors (`lto` string\|boolean) | 0 | `offload/🪪️profile/🧪️tests`: typed `profileSettingsV1` maps fixture settings to the real parameter type; 5/0 |
| typecheck disk test | 2 errors (`collect` possibly undefined) | 0 | `host-capacity/💽️disk/🧪️tests`: `collectorOfV1` requires the collector; 10/0 |
| T14 generations load | 113/1 | 114/0 | progress counters already green; failing was the bilingual-texts test: `generations/🧬️schema/🔣️.json` Messages const lacked the `healing` text (en+de) that `🔌️load/🟨️.mjs` declares; added |
| reclaim test typecheck | 3 errors | 0 | `native-build/💾️storage/🍎️image/♻️reclaim/🧪️tests`: typed `outputs` record, `!python` guard (testPythonV1 may be null), `[...calls]`; 16/0 + 42/0 |
| T11 operations | 14/2 | 16/0 | scoping fixture lacked `--reclaim` for `cache gc` (product declares it); real-CLI help test (3 cold CLI boots, ~17-40 s each under load) now uses `stretchedBudgetMillisecondsV1(120_000)` |
| T16 resident host | fails over 64 KiB pin | 39/0 | already green on current tree, no change |
| T7 artifacts | 4/2 (`ResourceLoanDeadlineExhausted` at 30 s under load) | 5/0 | `⚡️caching/📦️artifacts/🧪️tests`: three literal 30000 ms admission controls now `stretchedBudgetMillisecondsV1(30000)` |
| T20 step-overdue | 21/1 (`executing phase is never stopped`, exit null at 28 s) | 22/0 | test policy `SLOW` gets `livenessCpuMilliseconds:10`: a busy-loop shell under load 60+ spent < 500 ms CPU per look, so the liveness rule read it as silent and stopped it at the 12 s window; the test is about the rule, not the CPU threshold |
| T15 sidecar signed-scheduler | 6/1 | 7/0 | already green, no change |
| H3 uploader `names the endpoint but not the access` | 12/1 | 13/0 | test wrote a stale facts copy against the closed schema: now `candidates`, `selectedExecution:null`, `publisher.origin`, no `host`, no `cache.endpoints` (`unit-cache/🛰️fleet/📤️uploader/🧪️tests`) |
| H3 buildscript fleet-run | 1/1 (`uploadUnitV1` returned "retry") | 2/0 | the fake fleet client lacked `storeBlob`/`storeResult` (uploader moved to verdict API); fake now answers `"stored"/"refused"/"diverged"` |
| T6 extracted-function ceiling | 12/1 (new `new Function(` in `budget/…/🎟️reservation/🧪️tests/📈️regrow`) | 13/0 | regrow test migrated to `loadProductFunctionsSyncV1` (only the 5 injected ports replaced; real fs/crypto/schema now resolve against the module); regrow 4/0 |
| admission test typecheck (growth, available) | 3 errors | 0 | typed `wardenLevelOfV1`, `expect<string>` / `expect<unknown>`; available 4/0, growth 8/0 |
