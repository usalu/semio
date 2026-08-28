# Runtime Published Phase R28 Native RED

The existing exact published-owner selector executed and failed: **0 passed, 1 failed, 95 skipped**, 0.030 seconds. Its four physical calls still retained the typed metadata owner, while the earlier fixture assumed a whole surface disappeared in one call.

The correction preserves the four logical ownership transitions and their exact final completion values, but explicitly labels the first owner as metadata and projects transitions across the actual typed steps. New grant tests separately verify exact semantic byte accounting at 1/64/4096 and contended proof retention. No larger grant or early metadata release is introduced.

Actual output:

```text

> nx run @semio-tech/ui-runtime-rs:test --args=--lib instance_lifetime_published_patch_close_retains_exact_handback_until_terminal -- --nocapture

> bun ./📜️script.ts test --lib instance_lifetime_published_patch_close_retains_exact_handback_until_terminal -- --nocapture

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] surface-ownership-oracle checks=23
warning: ignoring --test-threads because --no-capture is specified
────────────
 Nextest run ID 1cb5df9b-34be-4a96-a1ff-ca3e2c0cd282 with nextest profile: fundamental
    Starting 1 test across 1 binary (95 tests skipped)
       START [         ] (1/1) semio-framework-ui-runtime reconcile::tests::instance_lifetime_published_patch_close_retains_exact_handback_until_terminal

running 1 test

thread 'reconcile::tests::instance_lifetime_published_patch_close_retains_exact_handback_until_terminal' (5939176) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:3623:9:
assertion `left == right` failed
  left: Array [Array [String("surface"), String("credit"), String("handback")], Array [String("surface"), String("credit"), String("handback")], Array [String("surface"), String("credit"), String("handback")], Array [String("surface"), String("credit"), String("handback")]]
 right: Array [Array [String("credit"), String("handback")], Array [String("handback")], Array [], Array []]
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<serde_json::value::Value, serde_json::value::Value>
   4: semio_framework_ui_runtime::reconcile::tests::instance_lifetime_published_patch_close_retains_exact_handback_until_terminal
   5: semio_framework_ui_runtime::reconcile::tests::instance_lifetime_published_patch_close_retains_exact_handback_until_terminal::{closure#0}
   6: <semio_framework_ui_runtime::reconcile::tests::instance_lifetime_published_patch_close_retains_exact_handback_until_terminal::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test reconcile::tests::instance_lifetime_published_patch_close_retains_exact_handback_until_terminal ... FAILED

failures:

failures:
    reconcile::tests::instance_lifetime_published_patch_close_retains_exact_handback_until_terminal

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 95 filtered out; finished in 0.02s

        FAIL [   0.029s] (1/1) semio-framework-ui-runtime reconcile::tests::instance_lifetime_published_patch_close_retains_exact_handback_until_terminal
  Cancelling due to test failure: 
────────────
     Summary [   0.030s] 1 test run: 0 passed, 1 failed, 95 skipped
        FAIL [   0.029s] (1/1) semio-framework-ui-runtime reconcile::tests::instance_lifetime_published_patch_close_retains_exact_handback_until_terminal
error: test run failed
Warning: command "bun ./📜️script.ts test --lib instance_lifetime_published_patch_close_retains_exact_handback_until_terminal -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-runtime-rs failed

Failed tasks:

- @semio-tech/ui-runtime-rs:test

Hint: run the command with --verbose for more details.
```
