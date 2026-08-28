# Native Resident Capacity R1 Compile RED

The canonical released `@semio-tech/value-resident-rs:test` target was run without arguments, with the existing exhaustive level via environment, coverage disabled, and jobs=2 in the retained target. Exit 1: one E0432 unresolved import diagnostic for `ResidentCapacity`, `ResidentFault`, `ResidentResources`, `RESIDENT_MAXIMUM_COUNT`. Four test bodies were mounted but none executed. This is the requested missing-API compile RED, not four behavioral failures. The resident-only source hold was released for Dag to implement the schema-backed vocabulary. No Plugin/UI dependency adoption or composition permit is claimed.

## Command

```sh
set -o pipefail
SEMIO_COVERAGE=0 SEMIO_TEST_LEVEL=exhaustive NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/value-resident-rs:test --skip-nx-cache 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-native-resident-capacity-red-r1-2026-08-28.txt'
```

## Actual Output

```text

> nx run @semio-tech/value-resident-rs:test

> bun ./📜️script.ts test

error[E0432]: unresolved imports `super::ResidentCapacity`, `super::ResidentFault`, `super::ResidentResources`, `super::RESIDENT_MAXIMUM_COUNT`
error: could not compile `semio-framework-value-resident` (lib test) due to 1 previous error; 1 warning emittedWarning: command "bun ./📜️script.ts test" exited with non-zero status code


 NX   Running target test for project @semio-tech/value-resident-rs failed

Failed tasks:

- @semio-tech/value-resident-rs:test

Hint: run the command with --verbose for more details.


```

