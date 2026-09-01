# Browser Envelope R4 Native GREEN

Actual unchanged existing `browser_linear_memory_exact_envelope_retry_and_preflight_laws`: **1 passed, 67 skipped**, 0.015s; Nx exit0. This executes the envelope/retry/preflight law through the seven test-only cfg joins. Native production behavior and Wasm production bodies were not changed; no browser process or full UI-host/WGPU claim. The five original input-admission semantic REDs remain unchanged.

Selected capture: [R4 inputs](./📓️ui-host-browser-envelope-selected-inputs-r4-2026-08-28.md). Window SHA256 `02cdfea796b72c5ff244068b095280dc9c1fcee70f019e04918c65912002549f`.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/ui-host-rs:test --skip-nx-cache --args='exhaustive --lib browser_linear_memory_exact_envelope_retry_and_preflight_laws -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-ui-host-browser-envelope-r4-2026-08-28.md'
```

## Complete Captured Tool Output

```text

> nx run @semio-tech/ui-host-rs:test --args=exhaustive --lib browser_linear_memory_exact_envelope_retry_and_preflight_laws -- --nocapture

> bun ./📜️script.ts test exhaustive --lib browser_linear_memory_exact_envelope_retry_and_preflight_laws -- --nocapture

────────────
[32;1m Nextest run[0m ID [1m574ffd8b-d4da-49da-8add-23eaea4f42e8[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m1[0m test across [1m1[0m binary ([1m67[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] (1/1) [35;1msemio-framework-ui-host[0m [36mwindow::tests[0m[36m::[0m[34;1mbrowser_linear_memory_exact_envelope_retry_and_preflight_laws[0m

running 1 test
test window::tests::browser_linear_memory_exact_envelope_retry_and_preflight_laws ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 67 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] (1/1) [35;1msemio-framework-ui-host[0m [36mwindow::tests[0m[36m::[0m[34;1mbrowser_linear_memory_exact_envelope_retry_and_preflight_laws[0m
────────────
[32;1m     Summary[0m [   0.015s] [1m1[0m test run: [1m1[0m [32;1mpassed[0m, [1m67[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-ajIq9L[0m



 NX   Successfully ran target test for project @semio-tech/ui-host-rs



```

