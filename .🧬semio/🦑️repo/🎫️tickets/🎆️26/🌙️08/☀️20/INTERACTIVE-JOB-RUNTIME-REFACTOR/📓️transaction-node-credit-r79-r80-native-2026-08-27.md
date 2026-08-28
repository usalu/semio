# Transaction Independent Node Credit R79–R80

Follow-up source review of the cfg(test)-only R78 cutover found that Job opportunities charged zero nodes/bytes to the outer transaction. The existing all-zero-credit test could pass while an independent zero-node ceiling was ignored. This is a real test-oracle regression, not a production runtime claim.

Schema-first `runtime/🔄️transaction` fixture has two cases with the original 262144 item and 64MiB byte ceilings unchanged: a one-node surface under zero node credit must fault without a patch; one node credit admits exactly one patch. The permanent runtime script validates strict Ajv schema and independently compares the Node Buffer little-endian node count.

R79 actual semantic RED: 0 passed, 1 failed, 119 skipped, 0.150s, exit 1; zero-node case produced one patch instead of zero. The test cleans all owners through the test-only paired wrappers on failure.

The correction exposes current cursor/final Job census only under cfg(test), and charges checked monotonic node/byte deltas to the transaction on each child turn. This is the child's retained census, not a claim of exhaustive physical traversal/copy work or callback timing. No production API is added. R80 actual GREEN: one Rust test/two vectors passed, 119 skipped, 0.038s, exit 0. Original tests and limits remain unchanged.

## Actual R79 Output

```text
> nx run @semio-tech/ui-runtime-rs:test --args=exhaustive --lib transaction_canonical_job_preserves_independent_node_credit -- --nocapture

> bun ./📜️script.ts test exhaustive --lib transaction_canonical_job_preserves_independent_node_credit -- --nocapture

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
 Nextest run ID 328fe992-4b42-49d4-977e-2b81498dfd6c with nextest profile: exhaustive
    Starting 1 test across 1 binary (119 tests skipped)
       START [         ] (1/1) semio-framework-ui-runtime transaction::tests::transaction_canonical_job_preserves_independent_node_credit

running 1 test

thread 'transaction::tests::transaction_canonical_job_preserves_independent_node_credit' (7764412) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️transaction.rs:1231:13:
assertion `left == right` failed
  left: 1
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<usize, usize>
   4: semio_framework_ui_runtime::transaction::tests::transaction_canonical_job_preserves_independent_node_credit
   5: semio_framework_ui_runtime::transaction::tests::transaction_canonical_job_preserves_independent_node_credit::{closure#0}
   6: <semio_framework_ui_runtime::transaction::tests::transaction_canonical_job_preserves_independent_node_credit::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test transaction::tests::transaction_canonical_job_preserves_independent_node_credit ... FAILED

failures:

failures:
    transaction::tests::transaction_canonical_job_preserves_independent_node_credit

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 119 filtered out; finished in 0.01s

        FAIL [   0.140s] (1/1) semio-framework-ui-runtime transaction::tests::transaction_canonical_job_preserves_independent_node_credit
  Cancelling due to test failure: 
────────────
     Summary [   0.150s] 1 test run: 0 passed, 1 failed, 119 skipped
        FAIL [   0.140s] (1/1) semio-framework-ui-runtime transaction::tests::transaction_canonical_job_preserves_independent_node_credit
error: test run failed
Warning: command "bun ./📜️script.ts test exhaustive --lib transaction_canonical_job_preserves_independent_node_credit -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-runtime-rs failed

Failed tasks:

- @semio-tech/ui-runtime-rs:test

Hint: run the command with --verbose for more details.
```

## Actual R80 Output

```text
> nx run @semio-tech/ui-runtime-rs:test --args=exhaustive --lib transaction_canonical_job_preserves_independent_node_credit -- --nocapture

> bun ./📜️script.ts test exhaustive --lib transaction_canonical_job_preserves_independent_node_credit -- --nocapture

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
 Nextest run ID d06b3959-1f81-45cc-bf8f-7e9861631359 with nextest profile: exhaustive
    Starting 1 test across 1 binary (119 tests skipped)
       START [         ] (1/1) semio-framework-ui-runtime transaction::tests::transaction_canonical_job_preserves_independent_node_credit

running 1 test
test transaction::tests::transaction_canonical_job_preserves_independent_node_credit ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 119 filtered out; finished in 0.00s

        PASS [   0.036s] (1/1) semio-framework-ui-runtime transaction::tests::transaction_canonical_job_preserves_independent_node_credit
────────────
     Summary [   0.038s] 1 test run: 1 passed, 119 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-MrZ4vI



 NX   Successfully ran target test for project @semio-tech/ui-runtime-rs
```

