# Linked Session Engine Verification

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-os-dev:test --skip-nx-cache --args='-t linkedSessionEngines'`

Exit code: 0. Independent coordinator execution of the composition metadata/source tests. This does not build the absent Puzzle Wasm package or certify a browser launch.

```text

> nx run @semio-tech/framework-os-dev:test --args=-t linkedSessionEngines

> bun ./📜️script.ts test -t linkedSessionEngines

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)


 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)


 Test Files  1 passed (1)
      Tests  2 passed | 50 skipped (52)
   Start at  11:40:53
   Duration  2.40s (transform 1.07s, setup 0ms, import 1.75s, tests 109ms, environment 430ms)




 NX   Successfully ran target test for project @semio-tech/framework-os-dev



```

