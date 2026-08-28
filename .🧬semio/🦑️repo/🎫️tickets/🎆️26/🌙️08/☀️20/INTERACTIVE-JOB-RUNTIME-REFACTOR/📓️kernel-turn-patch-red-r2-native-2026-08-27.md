# Common Kernel Turn Patch R2 Actual RED

Correct owner route: `@semio-tech/framework-rs:test-wire-retirement-native --args='--lib ui_turn_patch_owner_ -- --nocapture'`.

The actual common kernel compiled and executed the Drop contention law. **0 passed, 1 failed, 244 skipped; one selected test was not run due to fail-fast**, 0.164 seconds. This is the requested semantic RED for Drop; normal-close contention is queued as a separate exact selector before production correction. R1 was a wrong OS-kernel target and is not prerequisite evidence.

Actual output:

```text

> nx run @semio-tech/framework-rs:test-wire-retirement-native --args=--lib ui_turn_patch_owner_ -- --nocapture

> bun ./📜️script.ts test-wire-retirement-native --lib ui_turn_patch_owner_ -- --nocapture

warning: ignoring --test-threads because --no-capture is specified
────────────
 Nextest run ID 0e7adca8-853d-4dca-8990-c809ce451417 with nextest profile: fundamental
    Starting 2 tests across 1 binary (244 tests skipped)
       START [         ] (1/2) semio-framework manifest::kernel::ui_turn_patch_tests::ui_turn_patch_owner_drop_hands_back_without_waiting_for_arena

running 1 test

thread 'manifest::kernel::ui_turn_patch_tests::ui_turn_patch_owner_drop_hands_back_without_waiting_for_arena' (5996815) panicked at 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️component.rs:1575:9:
assertion `left == right` failed
  left: true
 right: false
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<bool, bool>
   4: semio_framework::manifest::kernel::ui_turn_patch_tests::ui_turn_patch_owner_drop_hands_back_without_waiting_for_arena
   5: semio_framework::manifest::kernel::ui_turn_patch_tests::ui_turn_patch_owner_drop_hands_back_without_waiting_for_arena::{closure#0}
   6: <semio_framework::manifest::kernel::ui_turn_patch_tests::ui_turn_patch_owner_drop_hands_back_without_waiting_for_arena::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test manifest::kernel::ui_turn_patch_tests::ui_turn_patch_owner_drop_hands_back_without_waiting_for_arena ... FAILED

failures:

failures:
    manifest::kernel::ui_turn_patch_tests::ui_turn_patch_owner_drop_hands_back_without_waiting_for_arena

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 245 filtered out; finished in 0.15s

        FAIL [   0.162s] (1/2) semio-framework manifest::kernel::ui_turn_patch_tests::ui_turn_patch_owner_drop_hands_back_without_waiting_for_arena
  Cancelling due to test failure: 
────────────
     Summary [   0.164s] 1/2 tests run: 0 passed, 1 failed, 244 skipped
        FAIL [   0.162s] (1/2) semio-framework manifest::kernel::ui_turn_patch_tests::ui_turn_patch_owner_drop_hands_back_without_waiting_for_arena
warning: 1/2 tests were not run due to test failure (run with --no-fail-fast to run all tests, or run with --max-fail)
error: test run failed
Warning: command "bun ./📜️script.ts test-wire-retirement-native --lib ui_turn_patch_owner_ -- --nocapture" exited with non-zero status code


 NX   Running target test-wire-retirement-native for project @semio-tech/framework-rs failed

Failed tasks:

- @semio-tech/framework-rs:test-wire-retirement-native

Hint: run the command with --verbose for more details.
```
