# Independent Full React R17 and Strict R22 — 2026-08-27

## Actual Results

Full React R17 executed576tests:566passed and10failed, five files,108.04seconds, exit1. All ten failures concern the demonstrator lane's staged descriptor admission tests. The implementation still fabricated empty manifests for missing/network/HTML/malformed descriptors or accepted incorrect ownership/missing app rosters at this source boundary. These are real behavioral failures, not scene-parser failures.

Strict R22 exited1 with eight diagnostics: seven existing tutorial interactionSelection joins and one SceneBinding fixture draft.children inference error. The renderer lane corrected the latter by constructing the schema-owned children field without a cast. No discovery diagnostic appeared in this exact run.

The demonstrator subsequently reported coherent descriptor repair and16focused passes. Independent full R18 and strict R23 are running; their results are not inferred here.

## Full React Command and Output

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

 ❯ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx (51 tests | 2 failed) 1568ms
     × propagates a network failure without manufacturing an empty descriptor 62ms
     × rejects the dev server's HTML SPA fallback without a parse warning 1ms
 ❯ |@semio-tech/framework-renderer-react| 🧪️index.test.ts (390 tests | 8 failed) 21243ms
     × refuses 'missing' before starting the actor runtime 664ms
     × refuses 'SPA fallback' before starting the actor runtime 411ms
     × refuses 'malformed JSON' before starting the actor runtime 24ms
     × refuses 'null envelope' before starting the actor runtime 7ms
     × refuses 'missing manifest' before starting the actor runtime 11ms
     × refuses 'different owner' before starting the actor runtime 6ms
     × refuses 'missing app roster' before starting the actor runtime 26ms
     × fetchDescriptorManifest refuses a missing descriptor and surfaces a published one 1ms
stderr | 🧪️index.test.ts
THREE.WARNING: Multiple instances of Three.js being imported.


⎯⎯⎯⎯⎯⎯ Failed Tests 10 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  |@semio-tech/framework-renderer-react| 🧪️index.test.ts > descriptor load admission > refuses 'missing' before starting the actor runtime
 FAIL  |@semio-tech/framework-renderer-react| 🧪️index.test.ts > descriptor load admission > refuses 'SPA fallback' before starting the actor runtime
 FAIL  |@semio-tech/framework-renderer-react| 🧪️index.test.ts > descriptor load admission > refuses 'malformed JSON' before starting the actor runtime
 FAIL  |@semio-tech/framework-renderer-react| 🧪️index.test.ts > descriptor load admission > refuses 'null envelope' before starting the actor runtime
 FAIL  |@semio-tech/framework-renderer-react| 🧪️index.test.ts > descriptor load admission > refuses 'missing manifest' before starting the actor runtime
AssertionError: promise resolved "{ manifest: { …(11) }, …(1) }" instead of rejecting

- Expected
+ Received

- Error {
-   "message": "rejected promise",
+ {
+   "manifest": {
+     "apps": [],
+     "artifactKinds": [],
+     "capabilities": [],
+     "commands": [],
+     "contributions": [],
+     "dependencies": [],
+     "examples": [],
+     "label": "fixture",
+     "pluginId": "fixture",
+     "topicContributions": [],
+     "version": "",
+   },
+   "runtime": undefined,
  }

 ❯ ð§ªï¸index.test.ts:143:158

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/10]⎯

 FAIL  |@semio-tech/framework-renderer-react| 🧪️index.test.ts > descriptor load admission > refuses 'different owner' before starting the actor runtime

AssertionError: promise resolved "{ manifest: { …(2) }, …(1) }" instead of rejecting

- Expected
+ Received

- Error {
-   "message": "rejected promise",
+ {
+   "manifest": {
+     "apps": [],
+     "pluginId": "other",
+   },
+   "runtime": undefined,
  }

 ❯ ð§ªï¸index.test.ts:143:158

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[2/10]⎯

 FAIL  |@semio-tech/framework-renderer-react| 🧪️index.test.ts > descriptor load admission > refuses 'missing app roster' before starting the actor runtime
AssertionError: promise resolved "{ …(2) }" instead of rejecting

- Expected
+ Received

- Error {
-   "message": "rejected promise",
+ {
+   "manifest": {
+     "pluginId": "fixture",
+   },
+   "runtime": undefined,
  }

 ❯ ð§ªï¸index.test.ts:143:158

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[3/10]⎯

 FAIL  |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > fetchDescriptorManifest refuses a missing descriptor and surfaces a published one
AssertionError: promise resolved "{ pluginId: 'mock', …(10) }" instead of rejecting

- Expected
+ Received

- Error {
-   "message": "rejected promise",
+ {
+   "apps": [],
+   "artifactKinds": [],
+   "capabilities": [],
+   "commands": [],
+   "contributions": [],
+   "dependencies": [],
+   "examples": [],
+   "label": "mock",
+   "pluginId": "mock",
+   "topicContributions": [],
+   "version": "",
  }

 ❯ ð§ªï¸index.test.ts:1664:84

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[4/10]⎯

 FAIL  |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > fetchDescriptorManifest AbortSignal > propagates a network failure without manufacturing an empty descriptor
AssertionError: promise resolved "{ pluginId: 'p', label: 'p', …(9) }" instead of rejecting

- Expected
+ Received

- Error {
-   "message": "rejected promise",
+ {
+   "apps": [],
+   "artifactKinds": [],
+   "capabilities": [],
+   "commands": [],
+   "contributions": [],
+   "dependencies": [],
+   "examples": [],
+   "label": "p",
+   "pluginId": "p",
+   "topicContributions": [],
+   "version": "",
  }

 ❯ ../../../../🧱️elements/PluginRuntime/ð¦ï¸component.tsx:3015:68

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[5/10]⎯

 FAIL  |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > fetchDescriptorManifest AbortSignal > rejects the dev server's HTML SPA fallback without a parse warning
AssertionError: promise resolved "{ pluginId: 'p', label: 'p', …(9) }" instead of rejecting

- Expected
+ Received

- Error {
-   "message": "rejected promise",
+ {
+   "apps": [],
+   "artifactKinds": [],
+   "capabilities": [],
+   "commands": [],
+   "contributions": [],
+   "dependencies": [],
+   "examples": [],
+   "label": "p",
+   "pluginId": "p",
+   "topicContributions": [],
+   "version": "",
  }

 ❯ ../../../../🧱️elements/PluginRuntime/ð¦ï¸component.tsx:3030:68

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[6/10]⎯

 Test Files  2 failed | 3 passed (5)
      Tests  10 failed | 566 passed (576)
   Start at  17:12:41
   Duration  108.04s (transform 51.67s, setup 0ms, import 68.54s, tests 121.88s, environment 5.23s)




 NX   Running target test-long for project @semio-tech/framework-renderer-react failed

Warning: command "bun ./📜️script.ts test long" exited with non-zero status codeFailed tasks:

- @semio-tech/framework-renderer-react:test-long

Hint: run the command with --verbose for more details.
exit_code=1
```

## Strict Command and Output

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache
```

```text
> nx run @semio-tech/framework-renderer-react:typecheck

> bun ./📜️script.ts typecheck

../../../../../../../../../../♻️mit-bestand/🧺️demonstrator/🟦️brand.ts(151,5): error TS2741: Property 'interactionSelection' is missing in type '{ focusedWindowId: string; activeUtilityByWindowId: {}; activePanelTabByGroup: {}; expandedTreeIds: never[]; commandPanelOpen: false; }' but required in type 'TutorialUiSnapshot'.
🧪️index.test.ts(6846,43): error TS2345: Argument of type '{ activeUtilityByWindowId: {}; activePanelTabByGroup: {}; expandedTreeIds: never[]; commandPanelOpen: false; }' is not assignable to parameter of type 'TutorialUiSnapshot'.
  Property 'interactionSelection' is missing in type '{ activeUtilityByWindowId: {}; activePanelTabByGroup: {}; expandedTreeIds: never[]; commandPanelOpen: false; }' but required in type 'TutorialUiSnapshot'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2102,5): error TS2353: Object literal may only specify known properties, and 'selectionJson' does not exist in type 'TutorialUiSnapshot'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2102,39): error TS2339: Property 'selectionJson' does not exist on type 'PluginViewState'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2153,41): error TS2339: Property 'selectionJson' does not exist on type 'TutorialUiSnapshot'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2153,76): error TS2339: Property 'selectionJson' does not exist on type 'PluginViewState'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2197,141): error TS2339: Property 'selectionJson' does not exist on type '{ readonly kind: "selection"; readonly domainId: string; readonly granularity: string; readonly ids: readonly string[]; }'.
../../../../🧱️elements/UiDocumentStore/🟦️component.tsx(1121,82): error TS2339: Property 'children' does not exist on type 'WritableDraft<{ id: number; key: string; component: { type: string; kind: string; docSchema: string; doc: { bytes: number[]; }; }; layout: { kind: string; width: string; height: string; }; style: {}; activity: string; accessibility: {}; }>'.



 NX   Running target typecheck for project @semio-tech/framework-renderer-react failed

Failed tasks:

- @semio-tech/framework-renderer-react:typecheck

Hint: run the command with --verbose for more details.

Warning: command "bun ./📜️script.ts typecheck" exited with non-zero status code
exit_code=1
```

