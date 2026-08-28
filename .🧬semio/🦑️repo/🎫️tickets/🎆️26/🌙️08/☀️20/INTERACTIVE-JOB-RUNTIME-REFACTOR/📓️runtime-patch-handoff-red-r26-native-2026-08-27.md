# Runtime Patch Handoff R26 Native RED

Canonical `@semio-tech/ui-runtime-rs:test --args='--lib retained_patch_handoff_ -- --nocapture'` exited 1 before tests, with twelve missing new in-place handoff and exact-grant close API diagnostics. The language-neutral schema/Node Buffer oracle completed 23 assertions. Neither new native law ran.

Actual captured output:

```text

> nx run @semio-tech/ui-runtime-rs:test --args=--lib retained_patch_handoff_ -- --nocapture

> bun ./📜️script.ts test --lib retained_patch_handoff_ -- --nocapture

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] surface-ownership-oracle checks=23
error[E0599]: no method named `close_step_with_grant` found for mutable reference `&mut reconcile::SurfaceReconcileReadyPatch` in the current scope
error[E0599]: no method named `publish_into` found for struct `reconcile::SurfaceReconcileReadyPatch` in the current scope
error[E0599]: no method named `terminal_is_empty` found for struct `reconcile::SurfaceReconcileReadyPatch` in the current scope
error[E0599]: no method named `terminal_is_empty` found for mutable reference `&mut reconcile::SurfaceReconcileReadyPatch` in the current scope
error[E0599]: no method named `publish_into` found for struct `reconcile::SurfaceReconcileReadyPatch` in the current scope
error[E0599]: no method named `close_step_with_grant` found for mutable reference `&mut reconcile::SurfaceReconcilePublishedPatch` in the current scope
error[E0599]: no method named `terminal_is_empty` found for struct `reconcile::SurfaceReconcileReadyPatch` in the current scope
error[E0599]: no method named `publish_into` found for struct `reconcile::SurfaceReconcileReadyPatch` in the current scope
error[E0599]: no associated function or constant named `acknowledge_into` found for struct `reconcile::SurfaceReconcilePublishedPatch` in the current scope
error[E0599]: no associated function or constant named `acknowledge_into` found for struct `reconcile::SurfaceReconcilePublishedPatch` in the current scope
error[E0599]: no associated function or constant named `acknowledge_into` found for struct `reconcile::SurfaceReconcilePublishedPatch` in the current scope
error[E0599]: no method named `publish_into` found for struct `reconcile::SurfaceReconcileReadyPatch` in the current scope
error: could not compile `semio-framework-ui-runtime` (lib test) due to 12 previous errors; 8 warnings emittedWarning: command "bun ./📜️script.ts test --lib retained_patch_handoff_ -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/ui-runtime-rs failed

Failed tasks:

- @semio-tech/ui-runtime-rs:test

Hint: run the command with --verbose for more details.
```
