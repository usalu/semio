# Watchdog Tail R4 Native GREEN

## Actual Scope

**3 passed, 20 skipped**,0.033s,Nx exit0. All eight clock vectors executed, including7999 success,8000 equality fault,8001 fault, terminal missing/backward readings and sticky intermediate missing/backward faults. The exact site sample is observed before the terminal clock. Real held site/violation mutexes did not prevent terminal fault return. A prior successful same-operation/generation diagnostic does not change the later guard's8000us fault.

This is trace guard/telemetry evidence only. It does not publish a WGPU event/metrics candidate, prove full outer callback timing, fund resident storage, or authorize a normal Accepted result after a committed tail fault. The existing20 trace tests were not run in this filter. The unchanged original five UI-host queue REDs and new root missing-API RED remain open.

## Implementation Boundary

New domain source `trace/⏱clock/🏁tail/🦀.rs` supplies `Watchdog::admission_checkpoint(self) -> WatchdogAdmission`. The wrapper privately owns the same original guard and observation, exposes immutable diagnostic .verdict(), and is consumed by .finish_after_telemetry(). Optional interim report runs before the final reading. Missing/backward intermediate authority stays faulted. No fresh timer, caller-supplied verdict, public constructor/replacement, or global authority lookup is added.

Old Watchdog fields/layout, start and finish bodies remain unchanged. Its previous pure verdict calculation is extracted into verdict_at(), and report() uses that same result before unchanged telemetry. The private wrapper disarms the inner Drop before its final operations, avoiding extra hidden end-clock/report work after the terminal reading.

Parent authorized this exact implementation after actual R3 four-missing-method RED. No UI/WGPU/Plugin caller adoption was mounted. Trace source/sole compiler released immediately after terminal.

[R3 original17 selected inputs](./📓️watchdog-tail-r3-selected-inputs-2026-08-28.md), [R4 current18 selected inputs](./📓️watchdog-tail-r4-selected-inputs-2026-08-28.md); selected captures, not complete atomic dependency closures. Scoped diffcheck passed. Same retained target/jobs2/exhaustive/no-fail-fast/budgets.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-trace-rs:test --skip-nx-cache --args='exhaustive --lib watchdog_tail_ --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-watchdog-tail-native-r4-2026-08-28.md'
```

## Complete Captured Tool Output

```text

> nx run @semio-tech/framework-trace-rs:test --args=exhaustive --lib watchdog_tail_ --no-fail-fast -- --nocapture

> bun ./📜️script.ts test exhaustive --lib watchdog_tail_ --no-fail-fast -- --nocapture

────────────
[32;1m Nextest run[0m ID [1m2e331861-8187-4e85-b210-01f71eb8d4e1[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m3[0m tests across [1m1[0m binary ([1m20[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] (1/3) [35;1msemio-framework-trace[0m [36mcomponent::watchdog_tail_tests[0m[36m::[0m[34;1mwatchdog_tail_a_previous_success_cannot_change_a_later_same_operation_result[0m

running 1 test
test component::watchdog_tail_tests::watchdog_tail_a_previous_success_cannot_change_a_later_same_operation_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] (1/3) [35;1msemio-framework-trace[0m [36mcomponent::watchdog_tail_tests[0m[36m::[0m[34;1mwatchdog_tail_a_previous_success_cannot_change_a_later_same_operation_result[0m
[32;1m       START[0m [         ] (2/3) [35;1msemio-framework-trace[0m [36mcomponent::watchdog_tail_tests[0m[36m::[0m[34;1mwatchdog_tail_held_optional_telemetry_keeps_terminal_fault_without_waiting[0m

running 1 test
test component::watchdog_tail_tests::watchdog_tail_held_optional_telemetry_keeps_terminal_fault_without_waiting ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (2/3) [35;1msemio-framework-trace[0m [36mcomponent::watchdog_tail_tests[0m[36m::[0m[34;1mwatchdog_tail_held_optional_telemetry_keeps_terminal_fault_without_waiting[0m
[32;1m       START[0m [         ] (3/3) [35;1msemio-framework-trace[0m [36mcomponent::watchdog_tail_tests[0m[36m::[0m[34;1mwatchdog_tail_uses_the_original_guard_for_admission_and_terminal[0m

running 1 test
[DEBUG] watchdog-tail case="accepted-window" admissionFault=false terminalFault=false elapsed=Some(7999)
[DEBUG] watchdog-tail case="publication-tail-equality" admissionFault=false terminalFault=true elapsed=Some(8000)
[DEBUG] watchdog-tail case="telemetry-tail-overrun" admissionFault=false terminalFault=true elapsed=Some(8001)
[DEBUG] watchdog-tail case="admission-equality-refusal" admissionFault=true terminalFault=true elapsed=Some(8001)
[DEBUG] watchdog-tail case="terminal-clock-missing" admissionFault=false terminalFault=true elapsed=None
[DEBUG] watchdog-tail case="terminal-clock-backward" admissionFault=false terminalFault=true elapsed=None
[DEBUG] watchdog-tail case="interim-clock-backward" admissionFault=false terminalFault=true elapsed=None
[DEBUG] watchdog-tail case="admission-clock-missing-stays-faulted" admissionFault=true terminalFault=true elapsed=None
test component::watchdog_tail_tests::watchdog_tail_uses_the_original_guard_for_admission_and_terminal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.009s] (3/3) [35;1msemio-framework-trace[0m [36mcomponent::watchdog_tail_tests[0m[36m::[0m[34;1mwatchdog_tail_uses_the_original_guard_for_admission_and_terminal[0m
────────────
[32;1m     Summary[0m [   0.033s] [1m3[0m tests run: [1m3[0m [32;1mpassed[0m, [1m20[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-vqawl9[0m



 NX   Successfully ran target test for project @semio-tech/framework-trace-rs



```

