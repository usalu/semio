# Independent Source Verification

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-rs:test-wire-retirement-source --skip-nx-cache`

Exit code: 0. Directly captured coordinator execution; no native/Wasm or all-app runtime credit follows from this source check.

```text

> nx run @semio-tech/framework-rs:test-wire-retirement-source

> bun ./📜️script.ts test-wire-retirement-source

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] raw wire retirement source: 5 ownership cases, 4 hostile fixtures; native grant/terminal behavior is separate



 NX   Successfully ran target test-wire-retirement-source for project @semio-tech/framework-rs



```

