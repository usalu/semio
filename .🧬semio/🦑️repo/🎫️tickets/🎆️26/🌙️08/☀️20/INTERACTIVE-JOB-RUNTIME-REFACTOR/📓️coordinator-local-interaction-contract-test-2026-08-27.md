# Local Interaction Contract Verification

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-replication-rs:test-local-interaction-source --skip-nx-cache`

Exit code: 0. Independent coordinator execution of schema/cold semantic parity tests. The live query, retained restore producer, publication and tutorial integration have not been certified by this test.

```text

> nx run @semio-tech/framework-replication-rs:test-local-interaction-source

> bun ./📜️script.ts test-local-interaction-source

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] Local-interaction source cases=11 hostileRejections=9 oracle=immer semanticKeyBytes=6650 nativeRuntimeClaims=0



 NX   Successfully ran target test-local-interaction-source for project @semio-tech/framework-replication-rs



```

