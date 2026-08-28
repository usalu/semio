# Coordinator Full React Verification — R16

## Actual Results

Independent full renderer test-long: **553PASS across5files**, exit0, start16:51:53,32.80seconds total. This includes the4 OwnedScene cases and the demonstrator peer's7 extension-completion tests added to the previous542-test boundary. The root inspected the complete scene parser and tests and started this run after the derived deep-case bound was coherent.

Independent strict typecheck R21 remains **RED with exactly7 tutorial/interactionSelection diagnostics**. No Scene, ownership-mint, derived-bound or extension test narrowing errors appear. The test fixture location moved toline6776; the semantic seven joins remain brand snapshot, test snapshot and five old selectionJson accesses in ShellHelpers.

The deep Scene case now asserts monotonic consumedBytes/completedRecords and a per-record no-progress bound derived from dual AVL allocation/reference/reservation/retirement work. No production grant or8ms ceiling changed. Detailed transition-to-constant explanation is being added by the lane; a test-loop upper bound is not a latency certificate.

## Remaining Runtime Scope

Scene preparation is not yet mounted into the live Interpreter/UiNodeView. Typed scene validation/projection, nested host JSON/pack preparation, native Scene fixture parity, all-app acceptance and hard latency remain independent obligations. The7 extension tests verify explicit failure/completion behavior, not canonical fresh extension actor evaluation. The7 tutorial joins are still assigned to the real local-interaction/atomic restore implementation.

## Commands And Complete Output

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache
```

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

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)


 Test Files  5 passed (5)
      Tests  553 passed (553)
   Start at  16:51:53
   Duration  32.80s (transform 14.39s, setup 0ms, import 20.79s, tests 33.22s, environment 3.69s)




 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react
```

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache
```

```text
> nx run @semio-tech/framework-renderer-react:typecheck

> bun ./📜️script.ts typecheck

../../../../../../../../../../♻️mit-bestand/🧺️demonstrator/🟦️brand.ts(151,5): error TS2741: Property 'interactionSelection' is missing in type '{ focusedWindowId: string; activeUtilityByWindowId: {}; activePanelTabByGroup: {}; expandedTreeIds: never[]; commandPanelOpen: false; }' but required in type 'TutorialUiSnapshot'.
🧪️index.test.ts(6776,43): error TS2345: Argument of type '{ activeUtilityByWindowId: {}; activePanelTabByGroup: {}; expandedTreeIds: never[]; commandPanelOpen: false; }' is not assignable to parameter of type 'TutorialUiSnapshot'.
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
```

