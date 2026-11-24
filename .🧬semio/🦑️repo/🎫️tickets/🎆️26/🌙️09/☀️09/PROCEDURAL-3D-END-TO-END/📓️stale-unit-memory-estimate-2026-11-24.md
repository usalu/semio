# Stale Unit Memory Estimate (stdio_semio held back at 6351 MB)

## Root cause
The admission broker (`native-build/🎟️admission/🟦️.ts`) forecasts a unit's peak from `units.jsonl`
history keyed by `package@crate|target|profile|crateTypes`. The key does not change when the crate's
sources change, so after the 18 member stores moved out of `semio_s_artifact_stdio_semio`, the old
~4.9 GB p95 peak (x headroom = 6351 MB) was still applied to a crate that now needs ~133 MB.

The idle-broker escape (`availableAdmissionV1`, reason `waited-out`) needs `runningUnits === 0`
plus the 180 s wait, so while any other unit of the run is compiling the held unit stays held.
The rule exists and is correct; it only fires on an idle broker. The wrong forecast was the bug.

## Fix
- Observations record `lines` (Rust source lines of the crate directory at observation time).
- `forecastHistoryV1(history, lines)` keeps only rows whose `lines` are within
  `forecastSizeToleranceBasisPoints` (25 %) of the current size; rows without `lines` are stale.
- With no usable rows the existing size-based cold prior (`coldBase + coldPerKloc * kloc`) applies.
- Schema-first: `Policy.forecastSizeToleranceBasisPoints`, `ObservationV1.lines`.
- Test: `🧪️tests/🟦️.ts` "forecast history keeps only rows recorded for a source of comparable size"
  with vectors `forecasts` in `🧫️fixtures/🔣️.json` and an independent log-ratio oracle.

## Running hub-15
The broker loads `units.jsonl` once at start and keeps it in memory, and there is no tool command to
drop one unit's record. Editing the file would not change the running broker and hand-editing the
shared cache is not allowed, so hub-15 is not unblockable without a restart; it picks the fix up on restart.

## Test status
- `repo test-file` on the admission test file waited 562 s for the shared bun resource held by the running hub build and was refused ("Original Script deadline refused"); no test result.
- Run directly with the registered bun binary: the new forecast test, the Ajv schema agreement, keys and quantile tests pass (4 pass, 0 fail).
- The store/broker tests need a repo script invocation and fail standalone with "Original parent Script process invocation required"; they still need a `repo test-file` run once the bun resource is free.
