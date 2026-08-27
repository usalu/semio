# Independent Full React Renderer Execution R11

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache --args='--run'`

Exit code: 1. The canonical quick process watchdog killed the run after 30,000 ms; no result summary was emitted. No current 521-test pass is claimed. The executor's earlier 517-test pass at 13:09 and four targeted typed tests remain scoped historical evidence. A verbose canonical long diagnostic is next; this changes only the diagnostic harness tier, not a production runtime grant or watchdog.

```text

> nx run @semio-tech/framework-renderer-react:test-quick --args=--run

> bun ./📜️script.ts test quick --run

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)


 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[budget] /Users/ueli/.bun/bin/bun /Users/ueli/Documents/semio/node_modules/vitest/vitest.mjs run --config 🧪️vitest.config.ts --passWithNoTests --testTimeout 30000 --hookTimeout 30000 --teardownTimeout 30000 --run exceeded 30000ms — killed. Trim it, or assign it to a higher level (quick/long/exhaustive).
Warning: command "bun ./📜️script.ts test quick --run" exited with non-zero status code


 NX   Running target test-quick for project @semio-tech/framework-renderer-react failed

Failed tasks:

- @semio-tech/framework-renderer-react:test-quick

Hint: run the command with --verbose for more details.


```

