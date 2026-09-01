# Transaction Command Close Six-Law Native RED R2

## Outcome

Actual canonical exhaustive/no-fail-fast execution: **2 passed, 4 failed, 524 skipped**, six executed in 0.219s; Nx exit 1, no abort. Source holds released immediately after terminal output. No production repair performed by this lane.

Actual native `size_of::<TxnCommand>()` is 1 byte. Both short and zero byte rows therefore used zero bytes; they are not distinct positive short grants.

| Exact selector suffix | Result | Actual first step | Expected/diagnostic |
| --- | --- | --- | --- |
| exact_grant_retains_external_completion | FAIL | Pending, 1 item, 0 bytes | released bytes 0 != 1, test line 100 |
| exact_grant_retains_pending_completion | FAIL | Pending, 1 item, 0 bytes | released bytes 0 != 1, test line 100 |
| requires_begin_close | PASS | Blocked, 0 items, 0 bytes | unchanged before close |
| short_bytes_preserves_owners | FAIL | Pending, 1 item, 0 bytes | Pending != Blocked, test line 98 |
| zero_bytes_preserves_owners | FAIL | Pending, 1 item, 0 bytes | Pending != Blocked, test line 98 |
| zero_items_preserves_owners | PASS | Blocked, 0 items, 0 bytes | unchanged with zero item grant |

All names have prefix `component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_`. The native list selected exactly these six. Tests cover command value-layout storage and completion-owner separation, not allocator overhead, completion allocation/final-owner retirement, callback timing, or full Plugin correctness.

## Source and Execution

Selected-input capture: [867 selected inputs](./📓️txn-command-close-r1-selected-inputs-2026-08-28.md). This is a selected nested capture, not an atomic complete dependency closure.

Compilation and actual binary selection: [full no-run output](./📓️txn-command-close-inventory-r1-full-output-2026-08-28.md), [six-name list](./📓️txn-command-close-native-selection-r1-2026-08-28.md).

Unchanged retained target, jobs 2, existing budgets and exhaustive profile; no stack, test-thread, timeout or quota workaround.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache --args='exhaustive txn_command_close_ --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-txn-command-close-six-r2-2026-08-28.md'
```

Raw tee path: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-txn-command-close-six-r2-2026-08-28.md`.

## Complete Captured Tool Output

The following is the complete untruncated output retained directly from the command result, including all failures and terminal footer.

```text

> nx run @semio-tech/framework-plugin:test --args=exhaustive txn_command_close_ --no-fail-fast -- --nocapture

> bun 📜️script.ts test exhaustive txn_command_close_ --no-fail-fast -- --nocapture

[0m[33mWarning[0m[2m:[0m [1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.[0m
[0m      [2mat [0m[0m[1m[3mwarnOnDeactivatedColors[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m33[0m[2m:[33m24[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mgetColorDepth[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m42[0m[2m:[33m39[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mshouldColorize[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m14[0m[2m:[33m109[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrefresh[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m18[0m[2m:[33m31[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:util/colors[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m24[0m[2m:[33m16[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:assert/assertion_error[0m[2m ([0m[0m[36minternal:assert/assertion_error[0m[2m:[0m[33m2[0m[2m:[33m187[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mloadAssertionError[0m[2m ([0m[0m[36mnode:assert[0m[2m:[0m[33m28[0m[2m:[33m96[0m[2m)[0m

[DEBUG] plugin-runner-oracle cases=6
────────────
[32;1m Nextest run[0m ID [1m0a6acb1a-a956-402e-b955-c71be3a043d8[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m6[0m tests across [1m1[0m binary ([1m524[0m tests [33;1mskipped[0m)
[32;1m       START[0m [         ] (1/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_exact_grant_retains_external_completion[0m

running 1 test
[DEBUG] txn-command-close id=exact-external-completion commandBytes=1 grantItems=1 grantBytes=1 step=pending releasedItems=1 releasedBytes=0

thread 'component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_external_completion' (9337831) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧪️tests/🧪️command-close/🦀️.rs:100:5:
assertion `left == right` failed: exact-external-completion
  left: 0
 right: 1
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<usize, usize>
   4: semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::check
   5: semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_external_completion
   6: semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_external_completion::{closure#0}
   7: <semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_external_completion::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_external_completion ... FAILED

failures:

failures:
    component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_external_completion

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 529 filtered out; finished in 0.05s

[31;1m        FAIL[0m [   0.074s] (1/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_exact_grant_retains_external_completion[0m
[32;1m       START[0m [         ] (2/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_exact_grant_retains_pending_completion[0m

running 1 test
[DEBUG] txn-command-close id=exact-pending-completion commandBytes=1 grantItems=1 grantBytes=1 step=pending releasedItems=1 releasedBytes=0

thread 'component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_pending_completion' (9337834) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧪️tests/🧪️command-close/🦀️.rs:100:5:
assertion `left == right` failed: exact-pending-completion
  left: 0
 right: 1
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<usize, usize>
   4: semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::check
   5: semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_pending_completion
   6: semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_pending_completion::{closure#0}
   7: <semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_pending_completion::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_pending_completion ... FAILED

failures:

failures:
    component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_exact_grant_retains_pending_completion

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 529 filtered out; finished in 0.02s

[31;1m        FAIL[0m [   0.030s] (2/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_exact_grant_retains_pending_completion[0m
[32;1m       START[0m [         ] (3/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_requires_begin_close[0m

running 1 test
[DEBUG] txn-command-close id=before-begin-close commandBytes=1 grantItems=1 grantBytes=1 step=blocked releasedItems=0 releasedBytes=0
test component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_requires_begin_close ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 529 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] (3/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_requires_begin_close[0m
[32;1m       START[0m [         ] (4/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_short_bytes_preserves_owners[0m

running 1 test
[DEBUG] txn-command-close id=short-bytes commandBytes=1 grantItems=1 grantBytes=0 step=pending releasedItems=1 releasedBytes=0

thread 'component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_short_bytes_preserves_owners' (9337843) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧪️tests/🧪️command-close/🦀️.rs:98:5:
assertion `left == right` failed: short-bytes
  left: "pending"
 right: "blocked"
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<&str, alloc::string::String>
   4: semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::check
   5: semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_short_bytes_preserves_owners
   6: semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_short_bytes_preserves_owners::{closure#0}
   7: <semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_short_bytes_preserves_owners::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_short_bytes_preserves_owners ... FAILED

failures:

failures:
    component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_short_bytes_preserves_owners

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 529 filtered out; finished in 0.02s

[31;1m        FAIL[0m [   0.030s] (4/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_short_bytes_preserves_owners[0m
[32;1m       START[0m [         ] (5/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_zero_bytes_preserves_owners[0m

running 1 test
[DEBUG] txn-command-close id=zero-bytes commandBytes=1 grantItems=1 grantBytes=0 step=pending releasedItems=1 releasedBytes=0

thread 'component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_zero_bytes_preserves_owners' (9337847) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧪️tests/🧪️command-close/🦀️.rs:98:5:
assertion `left == right` failed: zero-bytes
  left: "pending"
 right: "blocked"
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<&str, alloc::string::String>
   4: semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::check
   5: semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_zero_bytes_preserves_owners
   6: semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_zero_bytes_preserves_owners::{closure#0}
   7: <semio_framework_plugin::component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_zero_bytes_preserves_owners::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_zero_bytes_preserves_owners ... FAILED

failures:

failures:
    component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_zero_bytes_preserves_owners

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 529 filtered out; finished in 0.03s

[31;1m        FAIL[0m [   0.042s] (5/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_zero_bytes_preserves_owners[0m
[32;1m       START[0m [         ] (6/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_zero_items_preserves_owners[0m

running 1 test
[DEBUG] txn-command-close id=zero-items commandBytes=1 grantItems=0 grantBytes=1 step=blocked releasedItems=0 releasedBytes=0
test component::app::mutation_fixture::transaction::command_close_tests::txn_command_close_zero_items_preserves_owners ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 529 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.021s] (6/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_zero_items_preserves_owners[0m
────────────
[31;1m     Summary[0m [   0.219s] [1m6[0m tests run: [1m2[0m [32;1mpassed[0m, [1m4[0m [31;1mfailed[0m, [1m524[0m [33;1mskipped[0m
[31;1m        FAIL[0m [   0.074s] (1/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_exact_grant_retains_external_completion[0m
[31;1m        FAIL[0m [   0.030s] (2/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_exact_grant_retains_pending_completion[0m
[31;1m        FAIL[0m [   0.030s] (4/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_short_bytes_preserves_owners[0m
[31;1m        FAIL[0m [   0.042s] (5/6) [35;1msemio-framework-plugin[0m [36mcomponent::app::mutation_fixture::transaction::command_close_tests[0m[36m::[0m[34;1mtxn_command_close_zero_bytes_preserves_owners[0m
[31;1merror[0m: test run failed
Warning: command "bun 📜️script.ts test exhaustive txn_command_close_ --no-fail-fast -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/framework-plugin failed

Failed tasks:

- @semio-tech/framework-plugin:test

Hint: run the command with --verbose for more details.


```

