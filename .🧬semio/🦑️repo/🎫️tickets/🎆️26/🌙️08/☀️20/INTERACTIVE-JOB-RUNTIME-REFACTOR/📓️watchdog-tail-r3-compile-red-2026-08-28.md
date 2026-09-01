# Watchdog Tail R3 Missing-API Native RED

Actual canonical trace target exits1 with four E0599 missing `Watchdog::admission_checkpoint` diagnostics. **Zero of the three tests executed.** The original17 selected input capture is preserved [here](./📓️watchdog-tail-r3-selected-inputs-2026-08-28.md). No production trace change preceded this RED. Parent approved the exact consuming same-guard implementation after reviewing this result; no UI/WGPU mount is authorized by it.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-trace-rs:test --skip-nx-cache --args='exhaustive --lib watchdog_tail_ --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-watchdog-tail-native-r3-2026-08-28.md'
```

## Complete Captured Output

```text

> nx run @semio-tech/framework-trace-rs:test --args=exhaustive --lib watchdog_tail_ --no-fail-fast -- --nocapture

> bun ./📜️script.ts test exhaustive --lib watchdog_tail_ --no-fail-fast -- --nocapture

error[E0599]: no method named `admission_checkpoint` found for struct `component::Watchdog` in the current scope
error[E0599]: no method named `admission_checkpoint` found for struct `component::Watchdog` in the current scope
error[E0599]: no method named `admission_checkpoint` found for struct `component::Watchdog` in the current scope
error[E0599]: no method named `admission_checkpoint` found for struct `component::Watchdog` in the current scope
error: could not compile `semio-framework-trace` (lib test) due to 4 previous errorsWarning: command "bun ./📜️script.ts test exhaustive --lib watchdog_tail_ --no-fail-fast -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/framework-trace-rs failed

Failed tasks:

- @semio-tech/framework-trace-rs:test

Hint: run the command with --verbose for more details.


```

