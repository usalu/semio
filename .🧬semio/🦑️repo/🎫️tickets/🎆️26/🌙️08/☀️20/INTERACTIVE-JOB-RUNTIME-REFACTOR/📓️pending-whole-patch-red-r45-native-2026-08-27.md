# Whole Patch Owner R45 Native RED

Canonical UI contract selector `retained_pending_patch_` exited 1 before tests. Four diagnostics reference the missing new owned `UiPendingPatch` API; one fixture include used one parent directory too many. Correcting that include preserves the existing authoritative eighteen-component fixture. No native test passed in this run. Strict schema and Node Buffer oracle completed 47 assertions.

Actual output:

```text

> nx run @semio-tech/ui-contract-rs:test --args=--lib retained_pending_patch_ -- --nocapture

> bun ./📜️script.ts test --lib retained_pending_patch_ -- --nocapture

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] fixed-list-page-oracle checks=47
error: couldn't read `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/📋️patch/📨️pending/📄️whole/../../../../🌳️typed/🧪️components.json`: No such file or directory (os error 2)
error[E0425]: cannot find type `UiPendingPatch` in this scope
error[E0425]: cannot find type `UiPendingPatch` in this scope
error[E0425]: cannot find type `UiPendingPatch` in this scope
error[E0433]: cannot find type `UiPendingPatch` in this scope
error: could not compile `semio-framework-ui-contract` (lib test) due to 5 previous errors; 61 warnings emittedWarning: command "bun ./📜️script.ts test --lib retained_pending_patch_ -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-contract-rs failed

Failed tasks:

- @semio-tech/ui-contract-rs:test

Hint: run the command with --verbose for more details.
```
