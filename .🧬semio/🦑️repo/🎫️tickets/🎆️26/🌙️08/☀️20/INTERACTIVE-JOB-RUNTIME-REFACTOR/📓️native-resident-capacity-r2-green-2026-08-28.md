# Native Resident Capacity R2 GREEN

Actual canonical exhaustive run: four tests executed, four passed, zero skipped, 0.013s; Nx exit 0. These validate only the scalar schema-backed capacity/resource vocabulary and the constructor's zero-heap law, not a mounted ledger/composition permit or live Registry opening. The unchanged test source hash is `8be2b653a03573f2afa6722d7c7880e5ab98466fc58f18b08fc0cab04aefff2e`; implementation `c5810696d9ce7326e0bd3d09abab2b3bde12ad8f138951923f47065f16d24264`.

This output is copied directly from retained tool result chunks, not a later filesystem read. No output chunk was truncated.

```sh
set -o pipefail
SEMIO_COVERAGE=0 SEMIO_TEST_LEVEL=exhaustive NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/value-resident-rs:test --skip-nx-cache 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-native-resident-capacity-green-r2-2026-08-28.md'
```

```text

> nx run @semio-tech/value-resident-rs:test

> bun ./📜️script.ts test

────────────
[32;1m Nextest run[0m ID [1ma6776497-6fd5-47b9-8c2a-4eef408300f3[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m4[0m tests across [1m1[0m binary
────────────
[32;1m     Summary[0m [   0.013s] [1m4[0m tests run: [1m4[0m [32;1mpassed[0m, [1m0[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-3KnBP9[0m



 NX   Successfully ran target test for project @semio-tech/value-resident-rs



```

