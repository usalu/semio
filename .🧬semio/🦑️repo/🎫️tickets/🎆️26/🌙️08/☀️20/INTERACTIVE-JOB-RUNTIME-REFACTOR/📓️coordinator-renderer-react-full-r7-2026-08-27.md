# Independent Renderer Execution R7

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache`

Exit code: 1. Four suites failed import resolution and **zero tests executed**. This run overlapped the executor's announced OwnedWire TDD work: its new test imported the nonexistent `@semio-tech/os` facade instead of `@semio-tech/framework-os`. The executor was already repairing that join. This is neither a behavioral RED test nor evidence that the previous production renderer regressed. The earlier 506-test run remains scoped historical evidence; no current full-green claim is made until a coherent rerun. No fresh Wasm/browser, final owned-wire integration or hard-latency proof is inferred.

```text

> nx run @semio-tech/framework-renderer-react:test-long

> bun ./📜️script.ts test long

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

 ❯ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx (0 test)
 ❯ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx (0 test)
stderr | ../../../../🧱️elements/Interpreter/🟦️component.tsx
THREE.WARNING: Multiple instances of Three.js being imported.

 ❯ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx (0 test)
stderr | 🧪️index.test.ts
THREE.WARNING: Multiple instances of Three.js being imported.

 ❯ |@semio-tech/framework-renderer-react| 🧪️index.test.ts (0 test)

⎯⎯⎯⎯⎯⎯ Failed Suites 4 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  |@semio-tech/framework-renderer-react| 🧪️index.test.ts [ 🧪️index.test.ts ]
 FAIL  |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx [ ../../../../🧱️elements/Interpreter/🟦️component.tsx ]
 FAIL  |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx [ ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx ]
 FAIL  |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx [ ../../../../🧱️elements/PluginRuntime/🟦️component.tsx ]
Error: Failed to resolve import "@semio-tech/os" from "../../../../🧱️elements/UiDocumentStore/🟦️component.tsx". Does the file exist?
  Plugin: vite:import-analysis
  File: /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx:566:60
  481 |    const { default: wireFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wir...
  482 |    const { default: wireSchema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire...
  483 |    const { encodePackValue, decodePackValue } = await import("@semio-tech/os");
      |                                                              ^
  484 |    describe("OwnedWire", () => {
  485 |      it("validates strict neutral native bounds and canonical Rust byte vectors", () => {
 ❯ _formatLog ../../../../../../../../../../node_modules/vite/dist/node/chunks/config.js:29079:46
 ❯ error ../../../../../../../../../../node_modules/vite/dist/node/chunks/config.js:29076:13
 ❯ ../../../../../../../../../../node_modules/vite/dist/node/chunks/config.js:27257:37
 ❯ transform ../../../../../../../../../../node_modules/vite/dist/node/chunks/config.js:27225:17

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/4]⎯


 Test Files  4 failed (4)
      Tests  no tests
   Start at  12:21:28
   Duration  2.38s (transform 3.12s, setup 0ms, import 0ms, tests 0ms, environment 1.23s)

Warning: command "bun ./📜️script.ts test long" exited with non-zero status code


 NX   Running target test-long for project @semio-tech/framework-renderer-react failed

Failed tasks:

- @semio-tech/framework-renderer-react:test-long

Hint: run the command with --verbose for more details.


```
