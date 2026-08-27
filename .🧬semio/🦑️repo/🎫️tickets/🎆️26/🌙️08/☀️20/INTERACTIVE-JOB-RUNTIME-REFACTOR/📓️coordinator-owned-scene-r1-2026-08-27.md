# Coordinator Owned Scene Verification — R1

## Actual Gate

The coordinator independently ran the canonical renderer target at16:39:49 on2026-08-27:

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedScene --silent=false'
```

Result: exit0; **4 passed,542 skipped,546 discovered**, five files;4.04seconds total. This is a fresh execution, not a source-only or peer-reported result.

## Source And Test Review

Read the complete retained Scene module and all four test bodies. The parser handles the native Scene tags0–13 into a flat numeric-index arena with explicit linked frames. Text/bytes remain spans of the exact captured component pages; text reads process128 input bytes, byte reads copy at most256 bytes. Scene document/reader/retirement construction requires a private runtime mint capability, not merely TypeScript private syntax. Producer cancellation retains and closes generator state, frames, index readers/edits, queued index owners and source ownership.

Actual tests cover19 semantic and16 hostile vectors with strict Ajv, Immer, Node Buffer and the existing Scene decoder where its semantics agree. Raw JSON preserves the literal prototype-key fixture. Long Unicode, a real32768-byte packet,4096 nested options, duplicate-key byte comparison, source-capture lifetime and cancellation prefixes are exercised. The Node/recursive materialization helpers are test oracles, not production adapters.

## Limits Of This Result

The deep-case loop in this exact R1 still uses1024 iterations per record. It is not a measured8ms certificate or a source-derived progress bound. The lane is replacing it with an AVL/source-derived bound plus monotonic progress/no-stall assertions before live adoption; this R1 remains honest historical execution evidence. The strict lane rerun reports seven tutorial diagnostics only; root has not rerun that new strict boundary yet.

No live Interpreter/UiNodeView cutover, typed scene schema projection, native fixture parity, maximum-record timing, aggregate instance/transport close or all-app end-to-end completion is inferred. Old whole-scene conversion remains an explicit pending integration seam.

## Full Captured Output

```text
> nx run @semio-tech/framework-renderer-react:test-long --args=--run -t OwnedScene --silent=false

> bun ./📜️script.ts test long --run -t OwnedScene --silent=false

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
      Tests  4 passed | 542 skipped (546)
   Start at  16:39:49
   Duration  4.04s (transform 6.67s, setup 0ms, import 9.31s, tests 2.21s, environment 2.52s)




 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react
```

