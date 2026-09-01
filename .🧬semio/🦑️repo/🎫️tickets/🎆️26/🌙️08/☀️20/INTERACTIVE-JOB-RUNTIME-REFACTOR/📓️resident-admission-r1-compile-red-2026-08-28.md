# Resident Admission R1 Missing-API Compile RED

Actual canonical noargs native target exits1 with one E0432: unresolved `ResidentGrant`, `ResidentLedgerRoot`, `ResidentPartition`, `ResidentStep`, `ResidentStepKind`. Five warnings; **zero tests executed** (neither existing4 nor new5). This is a compiler RED, not five executed behavioral failures. Resident source hold released immediately at terminal; no implementation changed by this lane.

[20 selected domain/Cargo inputs](./📓️resident-admission-r1-selected-inputs-2026-08-28.md), not an atomic complete dependency closure. Unchanged sole retained target/jobs2/budgets; exhaustive selected through environment because the canonical target forbids arguments.

```sh
set -o pipefail
SEMIO_TEST_LEVEL=exhaustive SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/value-resident-rs:test --skip-nx-cache 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-resident-admission-r1-2026-08-28.md'
```

## Complete Captured Tool Output

```text

> nx run @semio-tech/value-resident-rs:test

> bun ./📜️script.ts test

error[E0432]: unresolved imports `super::ResidentGrant`, `super::ResidentLedgerRoot`, `super::ResidentPartition`, `super::ResidentStep`, `super::ResidentStepKind`
error: could not compile `semio-framework-value-resident` (lib test) due to 1 previous error; 5 warnings emittedWarning: command "bun ./📜️script.ts test" exited with non-zero status code


 NX   Running target test for project @semio-tech/value-resident-rs failed

Failed tasks:

- @semio-tech/value-resident-rs:test

Hint: run the command with --verbose for more details.


```

