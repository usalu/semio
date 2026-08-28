# Document Component Comparison R43 Native RED

Canonical command: `bun x nx run @semio-tech/ui-contract-rs:test --skip-nx-cache --args='--lib retained_document_component_compare_ -- --nocapture'`, using the existing shared target and retained artifact directory.

Exit 1 before tests: twelve missing new comparison API diagnostics and one fixture-only attempt to serialize `UiNodeTable` instead of its actual ordered entries. The latter is corrected to the real serde-backed entries without changing wire expectations. Three new tests did not execute. Strict schema/Node Buffer source oracle completed 43 assertions.

Actual captured output:

```text

> nx run @semio-tech/ui-contract-rs:test --args=--lib retained_document_component_compare_ -- --nocapture

> bun ./📜️script.ts test --lib retained_document_component_compare_ -- --nocapture

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] fixed-list-page-oracle checks=43
error[E0425]: cannot find type `UiDocumentComponentCompare` in this scope
error[E0425]: cannot find type `UiDocumentComponentCompare` in this scope
error[E0425]: cannot find type `UiDocumentComponentCompare` in this scope
error[E0433]: cannot find type `UiDocumentComponentCompare` in this scope
error[E0433]: cannot find type `UiDocumentCompareError` in this scope
error[E0433]: cannot find type `UiDocumentCompareError` in this scope
error[E0433]: cannot find type `UiDocumentCompareError` in this scope
error[E0433]: cannot find type `UiDocumentComponentCompare` in this scope
error[E0433]: cannot find type `UiDocumentComponentCompare` in this scope
error[E0277]: the trait bound `UiNodeTable: serde::Serialize` is not satisfied
error[E0433]: cannot find type `UiDocumentComponentCompare` in this scope
error[E0433]: cannot find type `UiDocumentComponentCompare` in this scope
error[E0433]: cannot find type `UiDocumentCompareError` in this scope
error: could not compile `semio-framework-ui-contract` (lib test) due to 13 previous errors; 60 warnings emittedWarning: command "bun ./📜️script.ts test --lib retained_document_component_compare_ -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-contract-rs failed

Failed tasks:

- @semio-tech/ui-contract-rs:test

Hint: run the command with --verbose for more details.
```
