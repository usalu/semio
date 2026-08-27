# Independent Execution Record

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/repo-lib:test --args='-t "active ticket clean protection"' --skip-nx-cache`

Exit code: 0. Captured directly from the coordinator's execution, not inferred from an executor report. This is a scoped test, not end-to-end application or latency proof.

```text

> nx run @semio-tech/repo-lib:test --args=-t "active ticket clean protection"

> bun ./📜️script.ts test -t "active ticket clean protection"

bun test v1.3.14 (0d9b296a)

🧪️index.test.ts:
Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

✓ active ticket clean protection > projects synthetic candidates with strict manifest and third-party oracle parity [39.41ms]

 1 pass
 292 filtered out
 0 fail
 58 expect() calls
Ran 1 test across 1 file. [499.00ms]



 NX   Successfully ran target test for project @semio-tech/repo-lib



```

