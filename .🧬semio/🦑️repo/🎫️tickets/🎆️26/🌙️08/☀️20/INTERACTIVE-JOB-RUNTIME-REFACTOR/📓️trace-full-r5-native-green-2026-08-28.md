# Full Trace R5 Native GREEN

Actual unfiltered canonical `--lib` exhaustive/no-fail-fast regression: **23 passed, 0 failed, 0 skipped**,0.312s,Nx exit0. The actual roster below was extracted from the23 executed successful test result lines, not assumed from the previous20+3 source count. One expected should-panic UI-thread test emitted its intended panic/backtrace and passed. No failure/abort.

Fresh [18 selected trace/Cargo inputs](./📓️trace-full-r5-selected-inputs-2026-08-28.md), not a complete atomic dependency closure. Same retained target/jobs2/budgets. This completes the full current trace suite only; no UI/WGPU queue publication or whole callback certification is inferred.

## Actual Executed Roster

1. `component::microsecond_clock_tests::microsecond_clock_installation_is_exact_and_repeatable` — PASS
2. `component::microsecond_clock_tests::microsecond_watchdog_boundary_is_strictly_below_eight_ms` — PASS
3. `component::microsecond_telemetry_contention_tests::microsecond_telemetry_contention_event_returns_exact_event_without_waiting` — PASS
4. `component::microsecond_telemetry_contention_tests::microsecond_telemetry_contention_timer_site_does_not_wait` — PASS
5. `component::microsecond_telemetry_contention_tests::microsecond_telemetry_contention_watchdog_site_preserves_fault_without_waiting` — PASS
6. `component::microsecond_telemetry_contention_tests::microsecond_telemetry_contention_watchdog_violation_preserves_fault_without_waiting` — PASS
7. `component::microsecond_telemetry_contention_tests::microsecond_telemetry_exact_verdict_survives_saturation_and_invalid_clock` — PASS
8. `component::tests::assert_ui_thread_panics_off_ui_thread` — PASS
9. `component::tests::cancellation_latency_measures_requested_to_observed` — PASS
10. `component::tests::clock_is_monotonically_non_decreasing` — PASS
11. `component::tests::counters_snapshot_reflects_updates` — PASS
12. `component::tests::io_boundary_thread_registers_distinct_from_worker_and_ui` — PASS
13. `component::tests::latency_helpers_are_none_before_their_events_land` — PASS
14. `component::tests::percentile_ring_empty_reads_as_zero` — PASS
15. `component::tests::percentile_ring_orders_samples_correctly` — PASS
16. `component::tests::percentile_ring_wraps_past_capacity_keeping_newest` — PASS
17. `component::tests::thread_role_registers_and_asserts` — PASS
18. `component::tests::trace_follows_one_operation_start_to_preview_to_commit` — PASS
19. `component::tests::watchdog_reports_contract_violation_on_overrun` — PASS
20. `component::tests::watchdog_stays_silent_under_ceiling` — PASS
21. `component::watchdog_tail_tests::watchdog_tail_a_previous_success_cannot_change_a_later_same_operation_result` — PASS
22. `component::watchdog_tail_tests::watchdog_tail_held_optional_telemetry_keeps_terminal_fault_without_waiting` — PASS
23. `component::watchdog_tail_tests::watchdog_tail_uses_the_original_guard_for_admission_and_terminal` — PASS

## Invocation

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-trace-rs:test --skip-nx-cache --args='exhaustive --lib --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-trace-full-r5-2026-08-28.md'
```

## Complete Captured Output

```text

> nx run @semio-tech/framework-trace-rs:test --args=exhaustive --lib --no-fail-fast -- --nocapture

> bun ./📜️script.ts test exhaustive --lib --no-fail-fast -- --nocapture

────────────
[32;1m Nextest run[0m ID [1mb3864c4d-eb7d-4e5a-8854-12c426f0134f[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m23[0m tests across [1m1[0m binary
[32;1m       START[0m [         ] ( 1/23) [35;1msemio-framework-trace[0m [36mcomponent::microsecond_clock_tests[0m[36m::[0m[34;1mmicrosecond_clock_installation_is_exact_and_repeatable[0m

running 1 test
[DEBUG] monotonic clock installation current=null requested="browser" accepted=true retained="browser"
[DEBUG] monotonic clock installation current="browser" requested="browser" accepted=true retained="browser"
[DEBUG] monotonic clock installation current="browser" requested="foreign" accepted=false retained="browser"
[DEBUG] monotonic clock installation current="foreign" requested="browser" accepted=false retained="foreign"
test component::microsecond_clock_tests::microsecond_clock_installation_is_exact_and_repeatable ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 1/23) [35;1msemio-framework-trace[0m [36mcomponent::microsecond_clock_tests[0m[36m::[0m[34;1mmicrosecond_clock_installation_is_exact_and_repeatable[0m
[32;1m       START[0m [         ] ( 2/23) [35;1msemio-framework-trace[0m [36mcomponent::microsecond_clock_tests[0m[36m::[0m[34;1mmicrosecond_watchdog_boundary_is_strictly_below_eight_ms[0m

running 1 test
[DEBUG] exact watchdog boundary elapsed_us=7999 violated=false
[DEBUG] exact watchdog boundary elapsed_us=8000 violated=true
[DEBUG] exact watchdog boundary elapsed_us=8001 violated=true
test component::microsecond_clock_tests::microsecond_watchdog_boundary_is_strictly_below_eight_ms ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 2/23) [35;1msemio-framework-trace[0m [36mcomponent::microsecond_clock_tests[0m[36m::[0m[34;1mmicrosecond_watchdog_boundary_is_strictly_below_eight_ms[0m
[32;1m       START[0m [         ] ( 3/23) [35;1msemio-framework-trace[0m [36mcomponent::microsecond_telemetry_contention_tests[0m[36m::[0m[34;1mmicrosecond_telemetry_contention_event_returns_exact_event_without_waiting[0m

running 1 test
[DEBUG] telemetry contention event-ring-held returns_while_held=true exact_event=true
test component::microsecond_telemetry_contention_tests::microsecond_telemetry_contention_event_returns_exact_event_without_waiting ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 3/23) [35;1msemio-framework-trace[0m [36mcomponent::microsecond_telemetry_contention_tests[0m[36m::[0m[34;1mmicrosecond_telemetry_contention_event_returns_exact_event_without_waiting[0m
[32;1m       START[0m [         ] ( 4/23) [35;1msemio-framework-trace[0m [36mcomponent::microsecond_telemetry_contention_tests[0m[36m::[0m[34;1mmicrosecond_telemetry_contention_timer_site_does_not_wait[0m

running 1 test
[DEBUG] telemetry contention timer-site-held returns_while_held=true
test component::microsecond_telemetry_contention_tests::microsecond_telemetry_contention_timer_site_does_not_wait ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 4/23) [35;1msemio-framework-trace[0m [36mcomponent::microsecond_telemetry_contention_tests[0m[36m::[0m[34;1mmicrosecond_telemetry_contention_timer_site_does_not_wait[0m
[32;1m       START[0m [         ] ( 5/23) [35;1msemio-framework-trace[0m [36mcomponent::microsecond_telemetry_contention_tests[0m[36m::[0m[34;1mmicrosecond_telemetry_contention_watchdog_site_preserves_fault_without_waiting[0m

running 1 test
[DEBUG] telemetry contention watchdog-site-held returns_while_held=true exact_violation_retained=true
test component::microsecond_telemetry_contention_tests::microsecond_telemetry_contention_watchdog_site_preserves_fault_without_waiting ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.02s

[32;1m        PASS[0m [   0.024s] ( 5/23) [35;1msemio-framework-trace[0m [36mcomponent::microsecond_telemetry_contention_tests[0m[36m::[0m[34;1mmicrosecond_telemetry_contention_watchdog_site_preserves_fault_without_waiting[0m
[32;1m       START[0m [         ] ( 6/23) [35;1msemio-framework-trace[0m [36mcomponent::microsecond_telemetry_contention_tests[0m[36m::[0m[34;1mmicrosecond_telemetry_contention_watchdog_violation_preserves_fault_without_waiting[0m

running 1 test
[DEBUG] telemetry contention watchdog-violation-held returns_while_held=true exact_violation_retained=true
test component::microsecond_telemetry_contention_tests::microsecond_telemetry_contention_watchdog_violation_preserves_fault_without_waiting ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.024s] ( 6/23) [35;1msemio-framework-trace[0m [36mcomponent::microsecond_telemetry_contention_tests[0m[36m::[0m[34;1mmicrosecond_telemetry_contention_watchdog_violation_preserves_fault_without_waiting[0m
[32;1m       START[0m [         ] ( 7/23) [35;1msemio-framework-trace[0m [36mcomponent::microsecond_telemetry_contention_tests[0m[36m::[0m[34;1mmicrosecond_telemetry_exact_verdict_survives_saturation_and_invalid_clock[0m

running 1 test
[DEBUG] exact callback verdict survives full/contended telemetry and rejects backward/missing clocks
test component::microsecond_telemetry_contention_tests::microsecond_telemetry_exact_verdict_survives_saturation_and_invalid_clock ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 7/23) [35;1msemio-framework-trace[0m [36mcomponent::microsecond_telemetry_contention_tests[0m[36m::[0m[34;1mmicrosecond_telemetry_exact_verdict_survives_saturation_and_invalid_clock[0m
[32;1m       START[0m [         ] ( 8/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1massert_ui_thread_panics_off_ui_thread[0m

running 1 test

thread 'component::tests::assert_ui_thread_panics_off_ui_thread' (9504626) panicked at 🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/../../🦀️component.rs:513:5:
assert_ui_thread: called from Worker(3), not the UI thread
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_trace::component::assert_ui_thread
   3: semio_framework_trace::component::tests::assert_ui_thread_panics_off_ui_thread
   4: semio_framework_trace::component::tests::assert_ui_thread_panics_off_ui_thread::{closure#0}
   5: <semio_framework_trace::component::tests::assert_ui_thread_panics_off_ui_thread::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test component::tests::assert_ui_thread_panics_off_ui_thread - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.020s] ( 8/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1massert_ui_thread_panics_off_ui_thread[0m
[32;1m       START[0m [         ] ( 9/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mcancellation_latency_measures_requested_to_observed[0m

running 1 test
test component::tests::cancellation_latency_measures_requested_to_observed ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 9/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mcancellation_latency_measures_requested_to_observed[0m
[32;1m       START[0m [         ] (10/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mclock_is_monotonically_non_decreasing[0m

running 1 test
test component::tests::clock_is_monotonically_non_decreasing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] (10/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mclock_is_monotonically_non_decreasing[0m
[32;1m       START[0m [         ] (11/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mcounters_snapshot_reflects_updates[0m

running 1 test
test component::tests::counters_snapshot_reflects_updates ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] (11/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mcounters_snapshot_reflects_updates[0m
[32;1m       START[0m [         ] (12/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mio_boundary_thread_registers_distinct_from_worker_and_ui[0m

running 1 test
test component::tests::io_boundary_thread_registers_distinct_from_worker_and_ui ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.009s] (12/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mio_boundary_thread_registers_distinct_from_worker_and_ui[0m
[32;1m       START[0m [         ] (13/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mlatency_helpers_are_none_before_their_events_land[0m

running 1 test
test component::tests::latency_helpers_are_none_before_their_events_land ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.009s] (13/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mlatency_helpers_are_none_before_their_events_land[0m
[32;1m       START[0m [         ] (14/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mpercentile_ring_empty_reads_as_zero[0m

running 1 test
test component::tests::percentile_ring_empty_reads_as_zero ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.009s] (14/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mpercentile_ring_empty_reads_as_zero[0m
[32;1m       START[0m [         ] (15/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mpercentile_ring_orders_samples_correctly[0m

running 1 test
test component::tests::percentile_ring_orders_samples_correctly ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.009s] (15/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mpercentile_ring_orders_samples_correctly[0m
[32;1m       START[0m [         ] (16/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mpercentile_ring_wraps_past_capacity_keeping_newest[0m

running 1 test
test component::tests::percentile_ring_wraps_past_capacity_keeping_newest ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.009s] (16/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mpercentile_ring_wraps_past_capacity_keeping_newest[0m
[32;1m       START[0m [         ] (17/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mthread_role_registers_and_asserts[0m

running 1 test
test component::tests::thread_role_registers_and_asserts ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.009s] (17/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mthread_role_registers_and_asserts[0m
[32;1m       START[0m [         ] (18/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mtrace_follows_one_operation_start_to_preview_to_commit[0m

running 1 test
test component::tests::trace_follows_one_operation_start_to_preview_to_commit ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.021s] (18/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mtrace_follows_one_operation_start_to_preview_to_commit[0m
[32;1m       START[0m [         ] (19/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mwatchdog_reports_contract_violation_on_overrun[0m

running 1 test
test component::tests::watchdog_reports_contract_violation_on_overrun ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.03s

[32;1m        PASS[0m [   0.044s] (19/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mwatchdog_reports_contract_violation_on_overrun[0m
[32;1m       START[0m [         ] (20/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mwatchdog_stays_silent_under_ceiling[0m

running 1 test
test component::tests::watchdog_stays_silent_under_ceiling ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (20/23) [35;1msemio-framework-trace[0m [36mcomponent::tests[0m[36m::[0m[34;1mwatchdog_stays_silent_under_ceiling[0m
[32;1m       START[0m [         ] (21/23) [35;1msemio-framework-trace[0m [36mcomponent::watchdog_tail_tests[0m[36m::[0m[34;1mwatchdog_tail_a_previous_success_cannot_change_a_later_same_operation_result[0m

running 1 test
test component::watchdog_tail_tests::watchdog_tail_a_previous_success_cannot_change_a_later_same_operation_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.009s] (21/23) [35;1msemio-framework-trace[0m [36mcomponent::watchdog_tail_tests[0m[36m::[0m[34;1mwatchdog_tail_a_previous_success_cannot_change_a_later_same_operation_result[0m
[32;1m       START[0m [         ] (22/23) [35;1msemio-framework-trace[0m [36mcomponent::watchdog_tail_tests[0m[36m::[0m[34;1mwatchdog_tail_held_optional_telemetry_keeps_terminal_fault_without_waiting[0m

running 1 test
test component::watchdog_tail_tests::watchdog_tail_held_optional_telemetry_keeps_terminal_fault_without_waiting ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] (22/23) [35;1msemio-framework-trace[0m [36mcomponent::watchdog_tail_tests[0m[36m::[0m[34;1mwatchdog_tail_held_optional_telemetry_keeps_terminal_fault_without_waiting[0m
[32;1m       START[0m [         ] (23/23) [35;1msemio-framework-trace[0m [36mcomponent::watchdog_tail_tests[0m[36m::[0m[34;1mwatchdog_tail_uses_the_original_guard_for_admission_and_terminal[0m

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

[32;1m        PASS[0m [   0.010s] (23/23) [35;1msemio-framework-trace[0m [36mcomponent::watchdog_tail_tests[0m[36m::[0m[34;1mwatchdog_tail_uses_the_original_guard_for_admission_and_terminal[0m
────────────
[32;1m     Summary[0m [   0.312s] [1m23[0m tests run: [1m23[0m [32;1mpassed[0m, [1m0[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-KZfkb8[0m



 NX   Successfully ran target test for project @semio-tech/framework-trace-rs



```

