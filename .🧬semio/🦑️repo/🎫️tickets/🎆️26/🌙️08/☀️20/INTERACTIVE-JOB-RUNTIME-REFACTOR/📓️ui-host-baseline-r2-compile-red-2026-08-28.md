# UI Host Baseline R2 Compile RED

Canonical native baseline selection terminated before executing any law: soleE0432 unresolved import `super::browser::linear_memory_test_import`,3warnings,Nx1. **Zero of the five baseline tests executed.** This is not semantic baselineRED5. The runner emitted the summarized diagnostic below; no unseen full compiler rendering is reconstructed.

## Exact Cause and Test-Only Join

window.rs:1908 is an unconditional native cfg(test) browser-port law, but its existing import seam/functions at1376–1387 were cfg(wasm32,test). The actual adapter method bodies also selected the native Closed fallback during tests. Gating the test away would remove its semantics, so the narrow correction exposes the existing import seam under cfg(test), executes the existing Wasm adapter bodies under cfg(wasm32 OR test), and retains native Closed fallbacks under cfg(non-wasm AND non-test). No method body, fixture expectation, native non-test behavior, Wasm non-test import, queue production or input baseline changed. Seven cfg attributes only.

The unchanged five input-admission laws rerun next. The existing browser envelope law itself still needs actual execution before crediting that native adapter behavior. Source capture follows before rerun; originalR1 selected window SHA608176… preserves the pre-repair boundary.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/ui-host-rs:test --skip-nx-cache --args='exhaustive --lib input_admission_ -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-ui-host-baseline-five-r2-2026-08-28.md'
```

[Raw R2](./🧪️member-ui-host-baseline-five-r2-2026-08-28.md). Full actual tool output:

```text

> nx run @semio-tech/ui-host-rs:test --args=exhaustive --lib input_admission_ -- --nocapture

> bun ./📜️script.ts test exhaustive --lib input_admission_ -- --nocapture

error[E0432]: unresolved import `super::browser::linear_memory_test_import`
error: could not compile `semio-framework-ui-host` (lib test) due to 1 previous error; 3 warnings emittedWarning: command "bun ./📜️script.ts test exhaustive --lib input_admission_ -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-host-rs failed

Failed tasks:

- @semio-tech/ui-host-rs:test

Hint: run the command with --verbose for more details.


```

