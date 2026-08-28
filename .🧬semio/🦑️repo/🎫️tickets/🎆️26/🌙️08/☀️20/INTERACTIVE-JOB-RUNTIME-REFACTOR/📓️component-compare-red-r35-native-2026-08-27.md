# Component Compare R35 Native RED

Actual canonical exact-filter compilation with schema/tests mounted before the API implementation. Captured tail:

```text

> nx run @semio-tech/ui-contract-rs:test --args=--lib retained_component_compare_ -- --nocapture

> bun ./📜️script.ts test --lib retained_component_compare_ -- --nocapture

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] fixed-list-page-oracle checks=25
error[E0425]: cannot find type `UiComponentCompare` in this scope
error[E0433]: cannot find type `UiComponentCompare` in this scope
error[E0433]: cannot find type `UiComponentCompare` in this scope
error[E0433]: cannot find type `UiComponentCompare` in this scope
error: could not compile `semio-framework-ui-contract` (lib test) due to 4 previous errors; 23 warnings emittedWarning: command "bun ./📜️script.ts test --lib retained_component_compare_ -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-contract-rs failed

Failed tasks:

- @semio-tech/ui-contract-rs:test

Hint: run the command with --verbose for more details.


```
