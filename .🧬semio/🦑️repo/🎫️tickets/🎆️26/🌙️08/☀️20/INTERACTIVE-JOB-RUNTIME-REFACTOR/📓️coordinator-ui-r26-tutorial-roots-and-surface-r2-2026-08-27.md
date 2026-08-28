# Paged UI R26, Tutorial Roots, And Managed Surface R2

## Independent Strict R27 Follow-Up

The actual subsequent canonical strict run returns exactly seven tutorial diagnostics, exit 1. The four facade tuple-slot diagnostics are clear. This is a focused type-join repair, not full-project typecheck success.

```text

> nx run @semio-tech/framework-renderer-react:typecheck

> bun ./📜️script.ts typecheck

../../../../../../../../../../♻️mit-bestand/🧺️demonstrator/🟦️brand.ts(151,5): error TS2741: Property 'interactionSelection' is missing in type '{ focusedWindowId: string; activeUtilityByWindowId: {}; activePanelTabByGroup: {}; expandedTreeIds: never[]; commandPanelOpen: false; }' but required in type 'TutorialUiSnapshot'.
🧪️index.test.ts(6922,43): error TS2345: Argument of type '{ activeUtilityByWindowId: {}; activePanelTabByGroup: {}; expandedTreeIds: never[]; commandPanelOpen: false; }' is not assignable to parameter of type 'TutorialUiSnapshot'.
  Property 'interactionSelection' is missing in type '{ activeUtilityByWindowId: {}; activePanelTabByGroup: {}; expandedTreeIds: never[]; commandPanelOpen: false; }' but required in type 'TutorialUiSnapshot'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2102,5): error TS2353: Object literal may only specify known properties, and 'selectionJson' does not exist in type 'TutorialUiSnapshot'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2102,39): error TS2339: Property 'selectionJson' does not exist on type 'PluginViewState'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2153,41): error TS2339: Property 'selectionJson' does not exist on type 'TutorialUiSnapshot'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2153,76): error TS2339: Property 'selectionJson' does not exist on type 'PluginViewState'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2197,141): error TS2339: Property 'selectionJson' does not exist on type '{ readonly kind: "selection"; readonly domainId: string; readonly granularity: string; readonly ids: readonly string[]; }'.
Warning: command "bun ./📜️script.ts typecheck" exited with non-zero status code


 NX   Running target typecheck for project @semio-tech/framework-renderer-react failed

Failed tasks:

- @semio-tech/framework-renderer-react:typecheck

Hint: run the command with --verbose for more details.


exit_code=1
```


## Reviewed Native Gates

The coordinator read the exact native outputs:

| Gate | Actual result |
| --- | --- |
| Full paged UI contract R26 | 126 passed, zero skipped, 4.726 s |
| Tutorial immutable root R2 | 2 passed, 220 filtered, 0.01 s |
| Tutorial sparse three-map update R4 | 3 passed, 222 filtered, 0.02 s |

UI R23 first rejected the obsolete three-word owner-size expectation; R24 failed command argument parsing and ran no native tests; R25 reached 66 passes and one failure (58 not run) at the obsolete 6,320-byte UiPatchOp expectation. The actual new 64-bit payload is **6,416 bytes** (96 bytes larger), and first-patch backing is **34,088 bytes**, split into separately admitted directory and payload allocations. R26's refreshed structural fixture retains the logical capacity and runtime budgets. The test runner's historical flaky-task notice is not evidence of an automatic retry or a timing certificate.

The coordinator read the common paged-list implementation. Fixed-fanout metadata and leaf allocation are separate; physical allocation, placement and released backing are distinct counters. Four new laws include zero capacity, zero-sized elements, empty-tail release/reuse with a live 512-item prefix, order/oracle, exact oversized-element refusal and 32 binding-sized values. Runtime field clone/comparison/admission adoption and complete resident accounting remain open; cold full-reserve/Clone/try_push APIs are not interactive proof. Earlier wasm32 R20 applies to the pre-generic-list source, not this newer representation.

The tutorial root and update source now use shared ordered roots and preserve exact shared inputs. Candidate visibility waits for all three maps and cancellation explicitly retires partial ownership. Store, restore, query and tutorial consumers remain to be mounted; canonical sealing, topology authority and complete/sparse restore semantics stay required. ManuallyDrop during unwind alone is not recoverable ownership; the mounted command owner must keep cursors structurally outside an unwinding execution closure.

## Independent Managed Surface R2

The coordinator's canonical focused test passed **8 / 8**, 589 skipped, 597 discovered, 11.17 s, exit 0, start 17:55:05. This includes the actual source-replacement/unmount scene facade and the subscription/Surface patch runtime-mint regressions. The subscription reproducer had actually corrupted live list ownership before the guard; the new admission check rejects before reading/mutating supplied state. No full597 pass or live interpreter/WGPU cutover is claimed.

Strict R26 remained RED with the seven tutorial joins plus four new tuple-slot errors in the facade. The executor repaired the slot to explicit `0 | 1 | undefined`, narrowed before closure capture; independent strict R27 is running at this checkpoint. No cast is accepted as an ownership repair.

## Independent Focused Output

```text
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedSurface'

> nx run @semio-tech/framework-renderer-react:test-long --args=--run -t OwnedSurface

> bun ./📜️script.ts test long --run -t OwnedSurface

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
      Tests  8 passed | 589 skipped (597)
   Start at  17:55:05
   Duration  11.17s (transform 18.22s, setup 0ms, import 24.07s, tests 3.77s, environment 8.99s)




 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react



exit_code=0
```
