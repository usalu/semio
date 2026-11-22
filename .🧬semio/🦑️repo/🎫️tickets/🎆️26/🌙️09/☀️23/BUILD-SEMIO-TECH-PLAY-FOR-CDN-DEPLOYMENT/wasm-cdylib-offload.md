# Wasm cdylib offload

Root `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution` (abbreviated `RE/`).

## What changed
- Schema `RE/🚀️offload/🧬️schema/🔣️.json`: new `policy.wasm` (targets `wasm32-unknown-unknown`, `wasm32-wasip2`; crate type `cdylib`; link-argument patterns `^--max-memory=[0-9]{1,12}$` and `^-zstack-size=[0-9]{1,10}$`, the exact wasm-ld flags of `.cargo/config.toml`; `actionSeconds` 3600; `peakMegabytes` 6144) and property `semio-peak-mb`.
- `RE/🚀️offload/📐️policy/🟦️.ts`: shared decisions for requester and worker (`offloadCrateTypeRefusalV1`, `offloadWasmLinkArgumentV1`, `offloadWasmTargetV1`, `offloadPeakMegabytesV1`, `offloadActionSecondsV1`, crate type and target readers).
- Worker filter `RE/🧱️actions/🟦️.ts` (applied on both sides): `cdylib` only with a wasm target; the two numeric link arguments only with a wasm target; `-Z no-parallel-backend` allowed. Everything else stays refused (`-C linker`, other link args, dylib, staticlib, emscripten).
- Requester prefilter in `unit-cache/📜️script.ts` uses the same function (was a flat crate-type list).
- `RE/🚀️offload/📜️action/🟦️.ts`: a cdylib action runs with `-Z threads=1` and `-Z no-parallel-backend` (as the local scheduler does, so a worker's peak matches the prediction); the macOS `-oso_prefix` link arg is no longer sent for non-Apple targets (it would have broken wasm-ld); the action timeout is the wasm one.
- Placement sized by predicted peak, using existing facts: control property `semio-peak-mb` (stripped from platform matching like `semio-offload`), `PlacementOperationV1.peakMegabytes`, `PlacementWorkerV1.committedMegabytes` (peaks the scheduler already placed and still runs, since the next load report may not show them). `placementHasHeadroomV1` requires `report.freeMegabytes - committed >= peak`; a machine with no report never gets a sized action. The 6144 MB claim is the broker cold bound, above the measured 4.7 GiB peak. Unsized units are unchanged. A decline of a sized unit does not write the 3 s shared decline memo (it would block small units).
- Time: the fleet policy `🏗️build/🔣️policy.json` `maximumActionSeconds` 1800 to 3600 (components run up to 31 min and would otherwise time out remotely and rerun locally); client budget derives from the wasm `actionSeconds`.
- Avoided: `🎟️admission/` (other agent).

## Proof
- `RE/🚀️offload/🧪️tests/🕸️wasm-cdylib/🟦️.ts` + neutral vectors `🔣️.json` (11 requester cases, 17 worker cases, release link args). Fixture: library `d` plus cdylib `t` (module, `include_str!`, panic with `file!()`), release flags (opt-level s, thin LTO, 16 CGUs, strip, max-memory, stack size, `--remap-path-scope=all`). For each of wasm32-unknown-unknown and wasm32-wasip2: local compile at two checkout paths (`-Z threads=0`) and the worker path (planner, argv, worker validator, fresh input roots at two further paths, serial back end) give identical bytes and identical sha256 and size, no checkout path inside, and one action digest for both requesters. Passes.
- Placement: new test (small, committed, silent, big machine) and the 400-scenario Python oracle extended with peak and committed memory; 2000-seed properties extended (never a sized action to a machine without enough memory). Passes.
- Actions suite 10/10, wasm-cdylib 8/8, placement 8/8. Full remote-execution and unit-cache test sweep: see end of file.
- Typecheck: no new error in touched files (existing errors in these folders are unrelated: `resumed`, `size_bytes`, memo types, path flavour).

## Not verified
- Real fleet run (item 4): the fleet answers (11 darwin-arm64 workers, toolchain `312cc5b2932fbb31`, all with the wasm targets, 16 GB minis) but its running workers and scheduler run the old code, which refuses cdylib, ignores peaks and caps actions at 1800 s. Restarting fleet services was not asked and was not done, so no real offloaded cdylib and no digest comparison against the fleet. After workers and scheduler restart, run a small wasm cdylib with `SEMIO_OFFLOAD=always`.
- Peak is the 6144 MB policy bound, not per-crate history (avoids depending on the admission module being changed by another agent); a history-derived value can be passed as `peakMegabytes` to `offloadRustcUnitV1`.
- Output identity is shown for a small fixture; a real 100+ MiB component under thin LTO with threads=1 and serial back end is expected identical (same flags as the local scheduler) but not run.
- The running release attempt loads none of this until its next start.

## Sweep result
Remote-execution and unit-cache test files (sidecar and wire excluded): 90 pass, 11 fail. The failures are in seal, gc units, scratch (9 cases), freshness, retention-io, count, real-Bazel and worker-helper files; none touches offload, policy, placement or wasm, and all offload, scheduler, action and placement files pass. They were not compared against a run without these changes (no git stash allowed), so "unrelated" is by inspection of the failing areas, not proven.
