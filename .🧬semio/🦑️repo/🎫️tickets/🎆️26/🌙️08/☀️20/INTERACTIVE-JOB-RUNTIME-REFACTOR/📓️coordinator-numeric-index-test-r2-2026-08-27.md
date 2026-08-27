# Numeric Index Independent Execution R2

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/value-numeric-index:test --skip-nx-cache`

Exit code: 0. Independent persistent-index verification including the negative-zero parity changes. Full renderer ownership, notification and native/browser timing remain separate gates.

```text

> nx run @semio-tech/value-numeric-index:test

> bun ./📜️script.ts test

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] Numeric-index laws=12 lifecycle=37 ordinals=2 stress=3072 invalidIds=5 oracle=Immer+Map grants=256,4096 strictTS=0



 NX   Successfully ran target test for project @semio-tech/value-numeric-index



```

