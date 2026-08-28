# Actor Activation And UI Wasm Compilation Checkpoint

## Independent Actor R7

The coordinator actually ran the complete actor target after the demonstrator activation-lease release: **74 passed in four files, 3.70 s, exit 0**, starting 17:45:59 on 2026-08-27.

Read-only source review confirms the operation lease captures the activation object, shard object and worker object; checks route and generation ownership at dispatch and after settlement; and keeps operation revocation distinct from old-root close ownership. This is JavaScript actor/transport proof. It does not prove fresh Wasm execution, bounded extension evaluation, payload paging, host UI retirement, or complete fatal-stream cleanup.

## Reviewed Native Output

The coordinator read `🧪️member-ui-wasm-width-r20-2026-08-27.txt`: the canonical UI `check-wasm` target completed all three compile passes (55.32 / 5.44 / 3.69 s) and Nx reported success. Warnings remain; this is compilation, not a warning-denial or Wasm execution gate.

The next actual native binding-clone test is RED: zero passed / one failed / 89 skipped in 0.197 s. Its DEBUG line records **allocated=66304, initialized=66304, Yield bytes=0**. This directly demonstrates why the former field clone cannot count as bounded work. The paged-list and retained clone repair remains in implementation. Exact log: `🧪️surface-binding-clone-red-r14-native-2026-08-27.txt`.

## Extension Execution Boundary

The coordinator confirmed the peer's stricter source finding: `host/effects::run_router_effect_job` currently checks cancellation and otherwise returns the explicit unmounted-pump fault. An imported function name is not mounted execution. The demonstrator owns canonical request/completion authority and its retained execution integration; no sync fallback or second ABI is authorized by these partial host tests.

## Independent Actor Output

```text
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-actor:test --skip-nx-cache

> nx run @semio-tech/framework-actor:test

> bun ./📜️script.ts test

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)


 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript

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


 Test Files  4 passed (4)
      Tests  74 passed (74)
   Start at  17:45:59
   Duration  3.70s (transform 2.75s, setup 0ms, import 574ms, tests 3.57s, environment 1ms)




 NX   Successfully ran target test for project @semio-tech/framework-actor



exit_code=0
```

