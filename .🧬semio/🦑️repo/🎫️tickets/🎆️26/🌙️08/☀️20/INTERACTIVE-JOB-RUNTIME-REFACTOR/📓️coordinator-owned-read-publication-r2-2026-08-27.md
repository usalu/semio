# Independent Read Publication R2 and Live Scene Boundary

The coordinator ran the canonical Read filter after the staged-publication and opaque-subscription source join. Actual result: **5 passed**, 532 skipped, 537 discovered in five files, exit0; 15.08 seconds total, 1.13 seconds test time, start15:55:44. The complete lease/epoch implementation was read: one private commit state exposes staged snapshots together, foreign tokens cannot stage, cancelled captures keep capacity occupied until retained retirement, and close includes a partially retired cancelled second slot.

This does not certify the concrete surface owner or live interpreter. Their test-first implementation continues separately.

## Live Scene Consumer Finding

The coordinator inspected `Interpreter/🟦️component.tsx` at `surfacePropsToComponentSceneNode`. Live Surface rendering still creates a whole Uint8Array and calls `decodeScenePackValue` synchronously in React render before handing the recursive scene to one of fourteen host elements. The new owned SurfaceByteView exposes `byteAt`, not the old numeric-array interface. Converting it back into a whole array would not complete the interactivity refactor.

The required live join is a retained, incremental scene decode/prepared projection with exact captured byte ownership, per-host consumption and bounded old-scene retirement. React snapshot acknowledgement alone must not release a scene still used by an asynchronous host/worker callback. This requirement was sent to the renderer executor after the concrete surface owner; the old comment declaring fourteen hosts outside a past packet is not an exemption from the master task. No production source was changed by this scout.

## Command

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t Read'
```

## Complete Output

```text

> nx run @semio-tech/framework-renderer-react:test-long --args=--run -t Read

> bun ./📜️script.ts test long --run -t Read

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
      Tests  5 passed | 532 skipped (537)
   Start at  15:55:44
   Duration  15.08s (transform 21.37s, setup 0ms, import 30.10s, tests 1.13s, environment 9.29s)




 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react



```

