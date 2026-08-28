# Coordinator Indexed JSON Document R1

## Executed Gate

Actual exit0: **1 passed, 629 skipped, 630 total**, five files (one passed, four skipped), start20:26:25, duration71.49s. This selected test covers exact token/span/reader ownership, independent JSON.parse/Buffer parity, strict Ajv fixture validation, depth4096, largeUnicode, cancellation and private mint refusal. It does not certify live UI projection, numeric/string conversion, or 8ms timing.

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedSceneJsonDocument'
```

## Stable Captured Files

The following three files were identical before and after execution. The separately added string test was skipped by the exact selector.

```text
40dc0e4edac13dd858c450eece091f3e61a5ad4a06c655744cfe093fc4af8e73  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🧾️value/🟦️component.ts
0ff0e4666b2bc332cf60b2a032309f2917208d9e3578813f8b4fce8f083b2f4d  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🟦️component.ts
08843fac0da69eabef2b7fdbbe31974ab8f277ed1b3f243e6c1dc31fe5ea43b3  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
```

Fixture and fixture schema were read fully after the run; they were not included in this run's prehash claim. Their current hashes were1cc5127e6fa63f4c888a1fe1ec885187a8d4cdaa0dafd6ed6716a88e4d2c54f7 ande6003694ff4b8e0b1c80e866854f8fe6f57476e8f8d9917e89d8c221314cf634.

## Review Follow-Up

The selected gate did not exercise a child terminal step consuming the whole4096-byte allowance. In the current document cursor's `#drain`, the wrapper unlinks an owner then adds32 bookkeeping bytes, so a4096-byte terminal child can be rejected as4128 only after unlink. Routed to the UI owner for a real near-grant RED and admitted wrapper retirement; no production edit by coordinator. Existing successful gate is not represented as covering that new review case.

## Captured Completion

ANSI control sequences only are stripped below; the captured output is otherwise preserved.

```text

> nx run @semio-tech/framework-renderer-react:test-long --args=--run -t OwnedSceneJsonDocument

> bun ./📜️script.ts test long --run -t OwnedSceneJsonDocument

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

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)


 Test Files  1 passed | 4 skipped (5)
      Tests  1 passed | 629 skipped (630)
   Start at  20:26:25
   Duration  71.49s (transform 9.30s, setup 0ms, import 13.23s, tests 68.32s, environment 6.82s)




 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react


```

