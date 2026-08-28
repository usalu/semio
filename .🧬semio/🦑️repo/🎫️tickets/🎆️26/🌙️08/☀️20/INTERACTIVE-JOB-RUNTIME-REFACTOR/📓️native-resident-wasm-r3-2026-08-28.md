# Native Resident Wasm R3

Actual existing `check-wasm` route completed wasm32-wasip2 then wasm32-unknown-unknown, both exit 0; compile durations 0.79s and 0.51s. This is compilation of the scalar resident vocabulary only, not guest execution, allocation of a composition permit, or Plugin integration. Source unchanged from R2. Output copied directly from untruncated retained tool result chunks.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/value-resident-rs:check-wasm --skip-nx-cache 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-native-resident-wasm-r3-2026-08-28.md'
```

```text

> nx run @semio-tech/value-resident-rs:check-wasm

> bun ./📜️script.ts check-wasm

    Checking semio-framework-value-resident v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 0.79s
    Checking semio-framework-value-resident v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 0.51s



 NX   Successfully ran target check-wasm for project @semio-tech/value-resident-rs



```

