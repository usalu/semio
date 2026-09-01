# Input Root R6 Missing-API Native Compile RED

Actual canonical target exits1 before tests:31 diagnostics consisting of missing private `input_root` module, missing `EventQueue::try_admit_root_with`, and missing `EventQueue.root`;11 warnings. **Zero of the five new tests executed.** Original queue production and original baseline5 are unchanged. This is not native root/concurrency/grant proof.

[28 selected inputs](./📓️ui-host-input-root-r6-selected-inputs-2026-08-28.md), not a full atomic dependency closure. [Concrete intended seam and scope](./📓️input-root-native-red-packet-2026-08-28.md). No production implementation mounted; no compiler remains active.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/ui-host-rs:test --skip-nx-cache --args='exhaustive --lib input_root_native_ -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-ui-host-input-root-r6-2026-08-28.md'
```

## Complete Captured Output

```text

> nx run @semio-tech/ui-host-rs:test --args=exhaustive --lib input_root_native_ -- --nocapture

> bun ./📜️script.ts test exhaustive --lib input_root_native_ -- --nocapture

error[E0432]: unresolved import `super::input_root`
error[E0599]: no method named `try_admit_root_with` found for struct `enqueue::EventQueue` in the current scope
error[E0599]: no method named `try_admit_root_with` found for struct `enqueue::EventQueue` in the current scope
error[E0599]: no method named `try_admit_root_with` found for struct `enqueue::EventQueue` in the current scope
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0599]: no method named `try_admit_root_with` found for struct `enqueue::EventQueue` in the current scope
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0599]: no method named `try_admit_root_with` found for struct `enqueue::EventQueue` in the current scope
error[E0599]: no method named `try_admit_root_with` found for struct `enqueue::EventQueue` in the current scope
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0599]: no method named `try_admit_root_with` found for struct `enqueue::EventQueue` in the current scope
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0599]: no method named `try_admit_root_with` found for struct `enqueue::EventQueue` in the current scope
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0599]: no method named `try_admit_root_with` found for struct `enqueue::EventQueue` in the current scope
error[E0609]: no field `root` on type `enqueue::EventQueue`
error[E0599]: no method named `try_admit_root_with` found for struct `enqueue::EventQueue` in the current scope
error[E0609]: no field `root` on type `enqueue::EventQueue`
error: could not compile `semio-framework-ui-host` (lib test) due to 31 previous errors; 11 warnings emittedWarning: command "bun ./📜️script.ts test exhaustive --lib input_root_native_ -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-host-rs failed

Failed tasks:

- @semio-tech/ui-host-rs:test

Hint: run the command with --verbose for more details.


```

