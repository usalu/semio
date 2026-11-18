# Rustc wrapper chain: one convention

## Root cause
Two conventions collided. `unitCacheEnvironmentV1` (launch) sets the `/bin/sh` shim as `RUSTC_WRAPPER` and leaves `RUSTC` the real compiler. `compilerWrapperEnvironmentV1` (native-build admission) set `RUSTC_WRAPPER=bun` and `RUSTC=<sealed script.ts>` (read from `SEMIO_UNIT_CACHE_SCRIPT`, written by the compiler store). Any environment that carried the driver-chain `RUSTC` (the sealed 0o444 script) into a launch-convention lane (`compilerLaneEnvironmentV1` takes `compiler: environment.RUSTC`) probed `shim <script> -vV`, which exec'd the non-executable script: EACCES.

## Fix (greenfield, no compat)
- `unitCacheWrapperV1(cacheRoot)` (launch) replaces `sealedUnitCacheScriptV1`: answers the shim path (bilingual notice and undefined on failure).
- Compiler store exports `SEMIO_UNIT_CACHE_WRAPPER` (shim) instead of `SEMIO_UNIT_CACHE_SCRIPT`.
- `compilerWrapperEnvironmentV1` returns `RUSTC_WRAPPER=<shim>`, never `RUSTC`; `SEMIO_UNIT_CACHE_INNER=[semio-build-scheduler, RUSTC ?? "rustc"]`, so a miss still reaches the memory-admitting scheduler.
- Tests: admission replay test asserts wrapper = shim and no `RUSTC`; seal test asserts the wrapper is the sealed-digest shim.

## Not changed
Older unit-cache integration tests (`tests/🟦️.ts`, churn, freshness, incremental, complete) still build their own `RUSTC_WRAPPER=bun, RUSTC=script` harness env by hand; they are self-contained harnesses, not product code.

## Validation
Admission replay + seal + lane tests: 14 pass. Probe (`rustc-wrapper-chain-probe.ts`): shim `rustc -vV` with the driver chain env exits 0 and prints the real rustc version.
