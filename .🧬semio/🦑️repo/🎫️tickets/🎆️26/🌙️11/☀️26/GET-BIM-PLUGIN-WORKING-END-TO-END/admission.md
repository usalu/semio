# Admission: four catalog-entities crates refused as "beyond host"

## Root cause
`unit-cache/📜️script.ts` `beyondHost()` forecast every expensive unit with `localFinalPeakMegabytesV1(samples, serial)`. That function is the forecast of a linked wasm final: unless the managed serial back end runs, it extrapolates the one-thread measurement to the fair share of codegen threads (4) plus `extraMegabytesPerThread` per thread. A plain library unit always runs at one declared thread (`offloadDeclaredThreadsV1(false)`), but it got the final's price: entities-01 measured 3335 MB (`{"1":3335}`) became 4871 MB, above the 16g class `finalMegabytes` 4096, so `unitBeyondHostV1` said "beyond host". The gate then queued it for a bigger machine, the fleet answered with `worker-policy` (not retryable in `awaitBeyondHostV1`), and the unit was refused. Not an ICE record, not available-vs-total memory (holdable is 10240 MB), not RUSTC_ICE (that text is just the worker's refusal detail).

## Fix
- `🚀️offload/🚦️admission/🟦️.ts`: `localFinalPeakMegabytesV1` becomes `localUnitPeakMegabytesV1(samples, {linked, serial})`; a non-linked unit is forecast at one thread.
- `🚀️offload/🟦️.ts`: re-export renamed.
- `unit-cache/📜️script.ts`: passes `linked: unit.crateTypes.includes("cdylib")`.
- Tests: `🧪️tests/🏋️unit-admission/🐍️.py` (generator, now with `linked` rows), regenerated `🔣️.json`, `🟦️.ts` (rows use `linked`, new regression test for the 3335 MB case).

## Verification
- `repo test-file`: unit-admission 16 pass, native-build beyond-host 2 pass, offload tests 41 pass, offload beyond-host 6 pass, gate 4 pass; python `--check` ok.
- Build result: see below.

## Second issue: "RUSTC_ICE is not allowed" on forwarded units
`🗺️paths/🟦️.ts` forwards every requester variable matching `CARRIED_PREFIXES` (`CARGO_`, `RUSTC_`, `DEP_`), so the unit cache's `RUSTC_ICE` (an ICE-dump path, never a compile input) reached the worker, whose action policy (`🧱️actions`) denies the `RUSTC` prefix. Fix (schema-first, client side): `RUSTC_ICE` added to `policy.environment.deniedNames` in `🚀️offload/🧬️schema/🔣️.json`, so it is dropped from the action environment; the worker allow-list stays strict. Test: `🚀️offload/🧪️tests/🟦️.ts` plan now carries `RUSTC_ICE` and asserts it is absent (41 pass).

## Build verification
`repo cargo --store=shared --configuration=prototype -- check -p <ifc2x3 entities-01,-03,-04, ifc4 entities-04> -j 2` in the bim model workspace: all four checked, `Finished prototype profile in 10m 43s`, zero "needs more memory" lines (log `🗑️generated/admission-b.log`). The slow wall time is the host being shared (warden suspends units under memory pressure), not admission.

Note: the first verification run printed `Finished` (10m 43s) and then received SIGTERM from `repository:command` during teardown (shell exit 144, "unit history write failed"). A re-run to confirm a clean exit code was still compiling after 34 min (log `/private/tmp/claude-501/admc.log`, upstream crates rebuilt after the framework edits; the host is shared). Treat the four-crate result as `Finished` but with an unconfirmed clean exit code.
