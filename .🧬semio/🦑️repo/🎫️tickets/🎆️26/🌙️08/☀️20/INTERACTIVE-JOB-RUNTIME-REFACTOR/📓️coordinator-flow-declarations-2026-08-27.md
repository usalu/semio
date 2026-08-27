# Independent Source Verification

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run semio-framework-os-flow-core:declarations --skip-nx-cache`

Exit code: 0. Directly captured coordinator execution; no native/Wasm or all-app runtime credit follows from this source check.

```text

> nx run semio-framework-os-flow-core:declarations

> bun ./📜️script.ts declarations

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] Flow browser declarations: 108 schema methods, runtime prototype and TypeScript parser parity; 3 hostile fixtures rejected



 NX   Successfully ran target declarations for project semio-framework-os-flow-core



 NX   Nx detected a flaky task

  semio-framework-os-flow-core:declarations

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks


```

