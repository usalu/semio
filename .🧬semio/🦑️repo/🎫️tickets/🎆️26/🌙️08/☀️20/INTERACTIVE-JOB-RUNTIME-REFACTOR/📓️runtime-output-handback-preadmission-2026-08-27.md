# Runtime Output Handback Preadmission

## R75 Actual Ready Revalidation GREEN

After R74, Ready is checked only after the existing generation/cancellation/fuel/deadline guards. The authored live Plugin transfer turn invokes that revalidation before any root transfer. Actual runtime gate: 4 passed, 115 skipped, exit 0, 0.146 seconds. Four deadline/cancellation vectors belong to one test; no hard callback timing is claimed. Plugin caller compile remains pending R6 fixture repairs.

```text

> nx run @semio-tech/ui-runtime-rs:test --args=exhaustive --lib surface_output_admission_ -- --nocapture

> bun ./📜️script.ts test exhaustive --lib surface_output_admission_ -- --nocapture

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] surface-ownership-oracle checks=40
────────────
 Nextest run ID fbde6007-79d2-4709-a970-4c3e00394a33 with nextest profile: exhaustive
    Starting 4 tests across 1 binary (115 tests skipped)
       START [         ] (1/4) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind

running 1 test

thread 'reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind' (7579124) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../📤️output/🧪️component.rs:161:9:
[DEBUG] actual ready transfer callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0}
   3: <semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0}>, ()>
   8: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind
   9: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0}
  10: <semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] ready-transfer bytes=8776 shell-preserved=true refused-payload-preserved=true unwind-targets-retained=true
test reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.028s] (1/4) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind
       START [         ] (2/4) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer

running 1 test
[DEBUG] ready-revalidation actual=["fault", "fault", "pending", "pending"] exact-source-preserved=true
test reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.026s] (2/4) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer
       START [         ] (3/4) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free

running 1 test
[DEBUG] handback-admission one-free-accepted=false producer-invoked=false
test reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.00s

        PASS [   0.022s] (3/4) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free
       START [         ] (4/4) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback

running 1 test
[DEBUG] handback-admission post-seal-transfer=true late-slot-acquisition=false
test reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        PASS [   0.061s] (4/4) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback
────────────
     Summary [   0.146s] 4 tests run: 4 passed, 115 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-sqnQgr



 NX   Successfully ran target test for project @semio-tech/ui-runtime-rs



```

## R74 Actual Ready Revalidation RED

One native test executes four neutral vectors after actual job seal. Current `drive_one` returned Ready in all four cases, rather than fault/fault/pending/pending for cancellation, stale generation, zero fuel and equal deadline. Exact patch source remained retained and was closed before comparison. Exit 1: 0 passed, 1 failed, 118 skipped, 0.030 seconds. The shared fixture/script independently validates the u64 deadline cases with Node Buffer.

```text

> nx run @semio-tech/ui-runtime-rs:test --args=exhaustive --lib surface_output_admission_ready_rechecks -- --nocapture

> bun ./📜️script.ts test exhaustive --lib surface_output_admission_ready_rechecks -- --nocapture

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] surface-ownership-oracle checks=40
────────────
 Nextest run ID 243ceb70-467b-4576-a3ac-e9ce92f86496 with nextest profile: exhaustive
    Starting 1 test across 1 binary (118 tests skipped)
       START [         ] (1/1) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer

running 1 test

thread 'reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer' (7570357) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../📤️output/🧪️component.rs:200:5:
assertion `left == right` failed
  left: ["ready", "ready", "ready", "ready"]
 right: ["fault", "fault", "pending", "pending"]
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<alloc::vec::Vec<&str>, alloc::vec::Vec<&str>>
   4: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer
   5: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer::{closure#0}
   6: <semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer ... FAILED

failures:

failures:
    reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.01s

        FAIL [   0.028s] (1/1) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer
  Cancelling due to test failure: 
────────────
     Summary [   0.030s] 1 test run: 0 passed, 1 failed, 118 skipped
        FAIL [   0.028s] (1/1) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer
error: test run failed
Warning: command "bun ./📜️script.ts test exhaustive --lib surface_output_admission_ready_rechecks -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-runtime-rs failed

Failed tasks:

- @semio-tech/ui-runtime-rs:test

Hint: run the command with --verbose for more details.


```

## R73 Actual Structural Transfer GREEN

Canonical runtime three-test run: exit 0, 3 passed, 115 skipped, 0.108 seconds. The actual transfer law preserves the exact source under a too-small grant and occupied current receiver; an injected panic after actual transfer leaves both receivers outside the callback and the original job shell at its original allocation. The fixed transfer debit is reported below.

Production `SurfaceReconcileJob::take_ready` is removed; its cfg(test) helper delegates the same granted in-place API. The sole authored live Plugin caller now drives jobs in the retained SurfaceSlot, separates child completion from the transfer turn, preflights a Ready receiver and terminal-shell slot, and transfers with the unchanged 32KiB grant minus receiver/shell metadata. The empty shell remains owned for close. This Plugin caller has not yet compiled after this change; R6 was earlier. Producer-branch take/tuple ownership, original lifetime metadata, live shared-output preadmission and per-tracker backing remain open.

```text

> nx run @semio-tech/ui-runtime-rs:test --args=exhaustive --lib surface_output_admission_ -- --nocapture

> bun ./📜️script.ts test exhaustive --lib surface_output_admission_ -- --nocapture

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] surface-ownership-oracle checks=40
────────────
 Nextest run ID b8cc27b3-0e3a-4f2a-8baa-c09936b21896 with nextest profile: exhaustive
    Starting 3 tests across 1 binary (115 tests skipped)
       START [         ] (1/3) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind

running 1 test

thread 'reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind' (7550964) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../📤️output/🧪️component.rs:161:9:
[DEBUG] actual ready transfer callback unwind
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0}
   3: <semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0}>, ()>
   8: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind
   9: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0}
  10: <semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[DEBUG] ready-transfer bytes=8776 shell-preserved=true refused-payload-preserved=true unwind-targets-retained=true
test reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 117 filtered out; finished in 0.03s

        PASS [   0.052s] (1/3) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind
       START [         ] (2/3) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free

running 1 test
[DEBUG] handback-admission one-free-accepted=false producer-invoked=false
test reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 117 filtered out; finished in 0.00s

        PASS [   0.030s] (2/3) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free
       START [         ] (3/3) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback

running 1 test
[DEBUG] handback-admission post-seal-transfer=true late-slot-acquisition=false
test reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 117 filtered out; finished in 0.00s

        PASS [   0.023s] (3/3) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback
────────────
     Summary [   0.108s] 3 tests run: 3 passed, 115 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-3WaLUk



 NX   Successfully ran target test for project @semio-tech/ui-runtime-rs



```

## R72 Structural Transfer API RED

The new source/target ownership test keeps the job and both receivers outside an actual unwind callback, verifies exact payload identity under insufficient grant/occupied receiver, and requires the original shell to survive successful transfer. Actual compile RED: five E0599 diagnostics for the absent `required_ready_transfer_bytes`/`take_ready_into` methods, no tests executed. The production library remains the coherent R71 boundary while the approved Plugin R6 inventory runs; the API and authored live caller must cut over together afterward.

```text

> nx run @semio-tech/ui-runtime-rs:test --args=exhaustive --lib surface_output_admission_inplace_transfer -- --nocapture

> bun ./📜️script.ts test exhaustive --lib surface_output_admission_inplace_transfer -- --nocapture

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] surface-ownership-oracle checks=40
error[E0599]: no associated function or constant named `required_ready_transfer_bytes` found for struct `reconcile::SurfaceReconcileJob` in the current scope
error[E0599]: no method named `take_ready_into` found for struct `reconcile::SurfaceReconcileJob` in the current scope
error[E0599]: no method named `take_ready_into` found for struct `reconcile::SurfaceReconcileJob` in the current scope
error[E0599]: no method named `take_ready_into` found for struct `reconcile::SurfaceReconcileJob` in the current scope
error[E0599]: no method named `take_ready_into` found for struct `reconcile::SurfaceReconcileJob` in the current scope
error: could not compile `semio-framework-ui-runtime` (lib test) due to 5 previous errors; 10 warnings emittedWarning: command "bun ./📜️script.ts test exhaustive --lib surface_output_admission_inplace_transfer -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-runtime-rs failed

Failed tasks:

- @semio-tech/ui-runtime-rs:test

Hint: run the command with --verbose for more details.


```

## R71 Actual Semantic GREEN

After both actual REDs, constructors now retain the second exact handback in the original reservation and transfer it through accepted, rejected and cancellation owners. `take_ready` no longer reserves a slot after seal. Actual canonical two-test run: exit 0, 2 passed, 115 skipped, 0.031 seconds. This is preadmission evidence, not yet structural in-place publication or nonblocking Drop/registry proof.

```text

> nx run @semio-tech/ui-runtime-rs:test --args=exhaustive --lib surface_output_admission_ -- --nocapture

> bun ./📜️script.ts test exhaustive --lib surface_output_admission_ -- --nocapture

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] surface-ownership-oracle checks=40
────────────
 Nextest run ID 25ba2188-049c-457d-8f27-d27dfe38b262 with nextest profile: exhaustive
    Starting 2 tests across 1 binary (115 tests skipped)
       START [         ] (1/2) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free

running 1 test
[DEBUG] handback-admission one-free-accepted=false producer-invoked=false
test reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 116 filtered out; finished in 0.00s

        PASS [   0.016s] (1/2) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free
       START [         ] (2/2) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback

running 1 test
[DEBUG] handback-admission post-seal-transfer=true late-slot-acquisition=false
test reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 116 filtered out; finished in 0.00s

        PASS [   0.014s] (2/2) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback
────────────
     Summary [   0.031s] 2 tests run: 2 passed, 115 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-v9gscM



 NX   Successfully ran target test for project @semio-tech/ui-runtime-rs



```

## R70 Actual Post-Seal Semantic RED

Canonical exhaustive filter `surface_output_admission_transfers_after_seal`, exit 1: zero passed, one failed, 116 skipped, 0.103 seconds. With a real sealed job and every remaining handback occupied, the existing transfer refused its already-admitted result (false instead of true). Exact owners were drained before the assertion. Initial Nx graph wait resolved without intervention.

```text
Waiting for graph construction in another process to complete

> nx run @semio-tech/ui-runtime-rs:test --args=exhaustive --lib surface_output_admission_transfers_after_seal -- --nocapture

> bun ./📜️script.ts test exhaustive --lib surface_output_admission_transfers_after_seal -- --nocapture

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] surface-ownership-oracle checks=40
────────────
 Nextest run ID 7d1231c4-c354-4bb2-8877-ad5921567592 with nextest profile: exhaustive
    Starting 1 test across 1 binary (116 tests skipped)
       START [         ] (1/1) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback

running 1 test

thread 'reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback' (7445720) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../📤️output/🧪️component.rs:132:5:
assertion `left == right` failed
  left: false
 right: true
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<bool, bool>
   4: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback
   5: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback::{closure#0}
   6: <semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback ... FAILED

failures:

failures:
    reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 116 filtered out; finished in 0.07s

        FAIL [   0.098s] (1/1) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback
  Cancelling due to test failure: 
────────────
     Summary [   0.103s] 1 test run: 0 passed, 1 failed, 116 skipped
        FAIL [   0.098s] (1/1) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_transfers_after_seal_with_no_unreserved_handback
error: test run failed
Warning: command "bun ./📜️script.ts test exhaustive --lib surface_output_admission_transfers_after_seal -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-runtime-rs failed

Failed tasks:

- @semio-tech/ui-runtime-rs:test

Hint: run the command with --verbose for more details.


```

The live runtime currently admits only one handback in `SurfaceReconcileReservation::try_new`, and `SurfaceReconcileJob::take_ready` acquires the second after candidate seal. Two exact reservations must exist before producer invocation. A saturated registry after seal must not block transfer of the already-admitted pair.

The permanent output fixture/schema now specifies the unchanged 384-slot registry, two handbacks per producer, refusal when only one slot remains, and successful transfer after later saturation. The existing canonical runtime script validates strict Ajv schema and independent Node Buffer occupancy/retained-view expectations. This does not yet prove registry contention, allocation work, or the live Plugin output-pool join.

## R69 Actual Semantic RED

Canonical exhaustive runtime filter `surface_output_admission_refuses_before_producer`: exit 1, zero passed, one failed, 116 skipped, 0.023 seconds. The current constructor incorrectly returned an admitted reservation with only one slot free. The test releases all exact owners before asserting, preventing a secondary cleanup failure. No production implementation changed before this RED.

```text

> nx run @semio-tech/ui-runtime-rs:test --args=exhaustive --lib surface_output_admission_refuses_before_producer -- --nocapture

> bun ./📜️script.ts test exhaustive --lib surface_output_admission_refuses_before_producer -- --nocapture

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] surface-ownership-oracle checks=40
────────────
 Nextest run ID 3c710312-d420-40cf-a553-a1e7154fbf20 with nextest profile: exhaustive
    Starting 1 test across 1 binary (116 tests skipped)
       START [         ] (1/1) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free

running 1 test

thread 'reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free' (7427637) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../📤️output/🧪️component.rs:107:5:
assertion `left == right` failed
  left: true
 right: false
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<bool, bool>
   4: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free
   5: semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free::{closure#0}
   6: <semio_framework_ui_runtime::reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free ... FAILED

failures:

failures:
    reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 116 filtered out; finished in 0.01s

        FAIL [   0.021s] (1/1) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free
  Cancelling due to test failure: 
────────────
     Summary [   0.023s] 1 test run: 0 passed, 1 failed, 116 skipped
        FAIL [   0.021s] (1/1) semio-framework-ui-runtime reconcile::tests::output_pool_tests::surface_output_admission_refuses_before_producer_when_only_one_handback_is_free
error: test run failed
Warning: command "bun ./📜️script.ts test exhaustive --lib surface_output_admission_refuses_before_producer -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-runtime-rs failed

Failed tasks:

- @semio-tech/ui-runtime-rs:test

Hint: run the command with --verbose for more details.


```
