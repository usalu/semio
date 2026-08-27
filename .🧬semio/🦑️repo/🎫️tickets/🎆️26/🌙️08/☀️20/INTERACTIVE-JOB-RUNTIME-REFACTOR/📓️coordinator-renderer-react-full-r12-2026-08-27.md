# Independent Full React Renderer Execution R12

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run --reporter=verbose'`

Exit code: 0. All 521 tests in four files passed; 26.52 seconds Vitest total / 28.27 seconds aggregate parallel test execution, start 13:17:55. The four typed UI ownership tests are included. No stalled test was observed in this successful run. The preceding R11 quick run really exceeded its 30-second process deadline and remains recorded; changing the diagnostic test tier does not change production scheduler grants or interactivity limits.

This is the current independent renderer regression checkpoint, not live retained-renderer adoption, native route admission, a fresh-Wasm/browser result, or an 8 ms bound. The current strict typecheck still has the previously observed seven tutorial and two repository-discovery errors. The first verbose output chunk was tool-truncated; the retained capture below includes that explicit marker and the complete final result footer, rather than claiming an untruncated per-test transcript.

```text
Warning: truncated output (original token count: 8263)
Total output lines: 185


> nx run @semio-tech/framework-renderer-react:test-long --args=--run --reporter=verbose

> bun ./📜️script.ts test long --run --reporter=verbose

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

stderr | 🧪️index.test.ts
THREE.WARNING: Multiple instances of Three.js being imported.

stderr | ../../../../🧱️elements/Interpreter/🟦️component.tsx
THREE.WARNING: Multiple instances of Three.js being imported.

 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > command-ingress fault diagnostics > decodes the normalized scalar-wire fault payload instead of hiding the terminal cause 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > TypedWire > normalizes all native component variants and defaults with strict neutral and Immer oracles 819ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > instance-open retained UI lifecycle > refreshes window and panel surfaces from the language-agnostic ownership cases 771ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > instance-open retained UI lifecycle > acknowledges only patches that identify the exact retained surface 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > instance-open retained UI lifecycle > retains the first render patch so an unchanged surface-visible probe can reuse it 26ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > runs proposal -> prepare x2 -> commit, committing in reverse discovery order 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > undoGroup fans TransactionUndo out to every member of a completed transaction 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > rolls back a commit-failed transaction: undoes what already committed, rolls back the rest 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > reaches transaction.unknown-target when the initiator plugin has no registered handle 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > reaches transaction.unknown-target when InstanceDirectory has no entry for the foreign target 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > reaches transaction.unknown-mutation when the router has no entry for a foreign step 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > reaches transaction.contribution-not-permitted when a contributed mutation has no planner wired 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > plans and prepares a contributed mutation using the target's cached document pack 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > reaches transaction.cycle when the same (artifact, mutation, payload) step repeats 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > reaches transaction.depth-exceeded when a foreign-step chain runs past MAX_TRANSACTION_DEPTH 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > passes a member's TransactionPrepared.rejection code straight through — instance-busy, generation-mismatch, and the member-rejected default all reachable 5ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️el…3263 tokens truncated…component/key-value-list — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/input — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/state-activity-waiting — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/icon-select — accept 59ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/state-with-menu — accept 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/state-transition-introducing — accept 5ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/select — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/container — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/tree — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/state-activity-finished — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/state-disabled — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/surface — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/ring — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/toggle — accept 15ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/text — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/state-activity-loading — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/extension — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/image — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/state-transition-celebrating — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/slider — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 📐️layout/stack — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 📐️layout/nesting — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 📐️layout/grid — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 📐️layout/scroll — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 📐️layout/leaf — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 📐️layout/absolute — accept 27ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 📐️layout/overlay — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🖥️composite/surface-embedded — accept 21ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🖥️composite/form-with-validation — accept 44ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🖥️composite/dialog — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🖥️composite/tree-nested-sections — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🖥️composite/toolbar — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > ♿️accessibility/live-region — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > ♿️accessibility/labelled — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > ♿️accessibility/described — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > ♿️accessibility/decorative-image — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > ♿️accessibility/shortcut — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🩹️patch/set-activity — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🩹️patch/set-accessibility — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🩹️patch/set-component — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🩹️patch/set-layout — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🩹️patch/reorder-children — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🩹️patch/set-children — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🩹️patch/set-menu — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🩹️patch/set-bindings — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🩹️patch/remove-subtree — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🩹️patch/set-style — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🩹️patch/upsert — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🩹️patch/set-root — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🚫️rejection/quota-patch-ops — reject 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🚫️rejection/quota-patch-bytes — reject 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🚫️rejection/cycle — reject 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🚫️rejection/dangling-child — reject 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🚫️rejection/quota-children — reject 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🚫️rejection/quota-text-bytes — reject 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🚫️rejection/stale-base-revision — reject 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🚫️rejection/duplicate-sibling-key — reject 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🚫️rejection/quota-depth — reject 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🚫️rejection/quota-nodes — reject 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > tutorial document wire contract > keeps native document-track names and bidirectional event order 119ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > deduplicates module loads and retries an exact failed 'network' attempt 7ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > deduplicates module loads and retries an exact failed 'initialization' attempt 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > keeps identical Board controller/surface keys isolated across mounted shell scopes 428ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > keeps a remounted peer and gesture registered after the old attachment rejects 418ms
stderr | 🧪️index.test.ts > app-owned surface session factories > retires the exact mounted Board session once after 'ready' cancellation
The current testing environment is not configured to support act(...)

stderr | 🧪️index.test.ts > app-owned surface session factories > retires the exact mounted Board session once after 'ready' cancellation
The current testing environment is not configured to support act(...)
The current testing environment is not configured to support act(...)

 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > retires the exact mounted Board session once after 'constructing' cancellation 67ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > retires the exact mounted Board session once after 'attaching' cancellation 22ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > retires the exact mounted Board session once after 'ready' cancellation 6ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > joins exact plugin and app ownership while keeping instance scopes distinct 13ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > plugin session ownership > lets only the configured primary plugin establish a missing session 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > plugin session ownership > configures only the active aggregate outside studio mode 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework sync utilities > renders the real SyncAttachCard popover as a dismissible nonmodal dialog 168ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework sync utilities > builds three sync backbone toggles 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework sync utilities > has no active toggle when detached 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework sync utilities > groups File, Folder, and Remote under a single Sync category collection 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > live measure dispatch > serializes document updates and skips stale slider values 52ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > live measure dispatch > coalesces straight slider movement but preserves a down-up reversal 113ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > whole-subtree-retained-delete at one item/256 bytes matches the reference rejection order and Immer value oracle 3760ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > live measure dispatch > caps queued direction reversals so a jittery drag cannot grow the queue unbounded 114ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > reveal cutoff store > notifies only same-group subscribers and reflects the latest set value 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > reveal cutoff store > isRevealCutoffHidden hides instances at or past the live cutoff, and never instances without a revealIndex 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > reveal cutoff store > isRevealCutoffHidden treats a JSON null revealIndex as untagged, even at the boot cutoff of 0 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > reveal cutoff store > committed reveal cutoff reconciliation ignores same-value identity churn so a live fill drag is not reset by fillBuildTick 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > in-flight skipping interval > drops overlapping ticks instead of queueing them behind a slow run 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > coalescing action dispatcher > dedupes unchanged values and keeps at most one in-flight dispatch 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > shows terminal boot content instead of an infinite loading canvas 9ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > starts every panel anchor at the same 300px width 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > toggles the overlays slice via a direct value without touching unrelated slices 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > starts, advances, and dismisses an introduction via SET_INTRODUCTION_STEP without touching unrelated slices 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > auto-starts each introduction launch once and keeps a skipped replay-on-load introduction dismissed 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > COMPLETE_INTRODUCTION_INTERACTION appends and dedupes indices; SET_INTRODUCTION_STEP resets them 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > opens, replaces, and closes a dialog via SET_DIALOG without touching unrelated slices 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > toggles the layout slice via an updater function 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > toggles a middle anchor via SET_PANEL_VISIBLE the same way as a corner 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > rewrites window icons via SET_WINDOW_ICON for extras and base kinds 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > rewrites window titles via SET_WINDOW_TITLE for extras and base kinds 8ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > resets the dock override, every anchor's active path/visible/size, drill-down memory, and tree expansion via RESET_DOCK 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > HYDRATE_DOCK_UI restores a persisted size for the top-middle anchor 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > updates the uiPrefs slice and leaves the sync slice referentially unchanged 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > action-panel slice: fold/expand/stage/reset/active-utility update only their own keys and preserve identity on no-operations 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > actionPane slice: SET_ACTIVE_TOOL updates only activeToolId and preserves identity on no-operations (mode-scoped, not per-window) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > commandPanel slice: expand/collapse and stage/reset update only their own keys and preserve identity on no-operations (category active/fold state now lives in layout.panels['bottom-middle'], not this slice) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > command palette category is the bottom-middle anchor's own SET_PANEL_PATH; the UI's category-switch handler additionally dispatches SET_COMMAND_EXPANDED:null (reproducing the old single-action collapse-on-switch behavior across two actions) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > tutorial slice: SET_TUTORIAL starts a tutorial, resets rate/deviated, and clears an active introduction (mutual exclusivity) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > tutorial slice: SET_INTRODUCTION_STEP (non-null) clears an active tutorial (mutual exclusivity, reverse direction) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > tutorial slice: play/pause resets deviated only when transitioning to playing; rate/muted/captions/recording/deviated update independently 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > APPLY_TUTORIAL_UI_SNAPSHOT atomically restores layout/panels/tree/utility/tool/dialog/search across their owning slices 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > UPSERT_LOADED_PLUGIN inserts a new pluginId and replaces an existing one in place (order preserved) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > REMOVE_LOADED_PLUGIN drops only the matching pluginId 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > SET_PLUGIN_STATUS tracks per-pluginId status independent of loadedPlugins membership 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > Shell peer interaction (generic, app-agnostic) > regroups per-peer PresenceInteraction domains into a per-domain roster, keyed by clientId 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > Shell peer interaction (generic, app-agnostic) > is app-agnostic: two different apps sharing one domain id merge into the same entry 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > Shell peer interaction (generic, app-agnostic) > peerIdsSelecting/peerIdsHovering find which peers have a given target id 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > Shell peer interaction (generic, app-agnostic) > is defensive about an absent interaction field (older heartbeat, or wave 2a's wire field not yet landed) and an unknown domain 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ui identity preservation (puzzle 2d perf) > preserveJsonIdentity reuses the previous reference for structurally-equal values 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ui identity preservation (puzzle 2d perf) > preserveJsonIdentity returns the new reference when content actually differs 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ui identity preservation (puzzle 2d perf) > preserveJsonIdentity treats nested arrays/objects structurally, not just top-level fields 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ui identity preservation (puzzle 2d perf) > preserveJsonIdentity treats undefined previous as always-changed 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ui identity preservation (puzzle 2d perf) > mergeRecordPreservingIdentity reuses the whole previous record when every key is unchanged 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ui identity preservation (puzzle 2d perf) > mergeRecordPreservingIdentity reuses per-key references, replacing only the changed key 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ui identity preservation (puzzle 2d perf) > mergeRecordPreservingIdentity treats a key being added or removed as a change 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > batched ui refresh request/response (puzzle 2d perf round 3) > buildActiveUtilityByWindowId omits null utilities for batched refresh 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > batched ui refresh request/response (puzzle 2d perf round 3) > buildUiRefreshRequest forwards per-window utility map on viewState without a focused-window singular leak 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > batched ui refresh request/response (puzzle 2d perf round 3) > buildActiveUtilityByWindowId makes a just-activated transform visible to refresh before the next React render 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > batched ui refresh request/response (puzzle 2d perf round 3) > buildUiRefreshRequest for a full scope requests every window/panel/engagements/measures/labels section (utility bars are now registry-derived, not a plugin section) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > batched ui refresh request/response (puzzle 2d perf round 3) > buildUiRefreshRequest for none returns null 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > batched ui refresh request/response (puzzle 2d perf round 3) > buildUiRefreshRequest for a partial scope requests only the listed window/panel bodies and flags 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > batched ui refresh request/response (puzzle 2d perf round 3) > buildUiRefreshRequest returns null for a partial scope that matches nothing in this app 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > batched ui refresh request/response (puzzle 2d perf round 3) > buildUiRefreshRequest attaches the cached hash for a section that was already fetched once 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > batched ui refresh request/response (puzzle 2d perf round 3) > buildUiRefreshRequest gives two instances of the same window kind distinct keys and independent cached hashes 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > batched ui refresh request/response (puzzle 2d perf round 3) > applyUiRefreshResponseToCache writes changed sections and ignores hash-only (unchanged) ones 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > batched ui refresh request/response (puzzle 2d perf round 3) > buildUiRefreshRequest for a full scope also requests the mode-level tools section (keyed by tool id, not a window) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > batched ui refresh request/response (puzzle 2d perf round 3) > buildUiRefreshRequest for a partial scope requests tools only when the scope's `tools` flag is set 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > batched ui refresh request/response (puzzle 2d perf round 3) > applyUiRefreshResponseToCache caches the tools section same as measures/engagements/labels 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > adaptPluginHandle's own refreshUi is an honest empty result — window-body refresh now lives in loadPluginModule's ActivationRegistry/ShardClient turn loop, which a bare no-command handle has no access to 2ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > fetchDescriptorManifest falls back to an honest empty manifest when no 🔣️descriptor.json is reachable, and surfaces a real one when it is 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > resolves the descriptor before initializing the shard runtime 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > applyUiPatchToRetained > applies the semantic upsert and set-root operations emitted by the actor WIT boundary 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > applyUiPatchToRetained > applies incremental semantic field updates with a matching base revision 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > applyUiPatchToRetained > keeps the previous body when a semantic patch has a stale base revision 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > applyUiPatchToRetained > keeps the previous body when a patch violates the retained document graph 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > parses a typed InvocationResponse, including requestedEffects, from a plugin handle-action response 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > falls back to an empty InvocationResponse for malformed handle-action JSON 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > serializePerActor queues concurrent turns for the same actor one at a time, never overlapping 52ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > serializePerActor keys independently per actor — different actors run concurrently, not queued behind each other 16ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > serializePerActor keeps queuing subsequent turns after an earlier one rejects 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > declarative forms parity > shows the mixed-values placeholder on a non-uniform numberStepper 6ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > declarative forms parity > renders selectable builder cards with selection ring 4ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > keeps graph slider keyboard changes exact and disabled controls inert 42ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > serializes complete multi-turn command ingress sequences for one actor 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework external slots > degrades a resolvable external slot to 'unavailable' until the binary-channel render path lands 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > declarative forms parity > renders a group node as a labeled section nesting its child controls (Origin > X/Y/Z steppers) 11ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > declarative forms parity > applies separate presence fixtures without replacing the retained document 97ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > adaptPluginHandle.handleAction round-trips an action's output/uiScope/historyPatch from AppFrame::Invocation; requestedEffects is honestly empty for a bare command-only handle 3ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework external slots > renders external slot fallback text when unresolved 63ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > declarative forms parity > resolves stack gap/padding through the closed SpaceToken scale as inline CSS, and keeps separators off raw border-border 2ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > declarative forms parity > renders image nodes from url sources 3ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders node graph host from workflow scene json 15ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > validates the strict language-neutral graph parameter contract for all three consumers 85ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > adaptPluginHandle exposes setMergePolicy/resolveConflict/readConflicts and sends the real AppCommand wire frames — ticket 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS lane K2: the merge-policy Settings control and Conflicts panel Accept/Discard used to call `plugin.handle.setMergePolicy?.(…)` against a handle that never had the method, so the optional call silently no-opped — this asserts the method genuinely exists AND that calling it round-trips through the real `AppCommand`/`AppFrame` codecs, not an internal spy 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > declarative forms parity > passes number bounds and file accept to inputs 31ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > adaptPluginHandle.applyMutations decodes an unsolicited MergeReport/Conflicts reply and it reaches ShellState — ticket 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS lane L1 gap 1: a peer's ApplyEnvelopes ingest batches MergeReport/Conflicts frames alongside it (contract freeze §C6/§C9 'pushed unsolicited after every ingest'), but `applyMutations` used to only look for an Error frame and silently drop everything else — this asserts the guest's real roster survives the decode AND that dispatching it through `shellReducer`'s SET_CONFLICTS (what ShellHost's `applyRemoteMerge` does) lands a remote-origin quarantined conflict in both `selectOpenConflicts` and `selectQuarantinedConflicts`, exactly the panel/badge lane K2 wired 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > declarative forms parity > renders declarative text with appearance-aware foreground 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > declarative forms parity > disables gated wizard buttons 51ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders editable node graph host with find items 3ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > declarative forms parity > dag overlay label fills resolve to Canvas2D-safe hex for appearance 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer types > matches native action-semantics defaults without claiming migrated interactivity 9ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > declarative forms parity > renders field description, required marker and inline error 3ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > uses the live session camera for node graph wheel viewport actions 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer types > keeps window tabs concise while retaining the app fallback 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > declarative forms parity > renders slider unit readout 13ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer types > survives an app whose manifest declares no breadcrumb at all 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > declarative forms parity > renders numberStepper as a single-border Stepper control, not hand-rolled double-bordered buttons 36ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > encodes node graph selection and hover with framework interaction actions 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer types > flattens a recursive panelTabs tree to its leaves, depth-first 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer types > accepts component scene nodes 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > encodes node graph scenes as pack bytes for wasm sync 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer types > accepts graph-timeline component scene nodes 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > decodes graph pick channels from the native handle grammar with strict schema parity 51ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > owned declarative controls > renders and dispatches a panel input through the Interpreter export 15ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > owned declarative controls > dispatches a declarative select through the owned listbox 72ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > validates strict language-neutral graph slider labels and rejects unnamed rows 9ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > names graph slider overlays from exact localized captions and keeps scoped ids stable 21ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > backedge-cycle at one item/256 bytes matches the reference rejection order and Immer value oracle 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > missing-child at one item/256 bytes matches the reference rejection order and Immer value oracle 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > long-unicode-sibling-key at one item/256 bytes matches the reference rejection order and Immer value oracle 5ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > unchecked-dangling-snapshot at one item/256 bytes matches the reference rejection order and Immer value oracle 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > deep-chain at one item/256 bytes matches the reference rejection order and Immer value oracle 201ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > dispatches graph parameter keyboard and drag events from the mounted FlowGraphCanvasHost without fixture reads or end commits 702ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > parses slider overlay state json for flow graph hosts 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders graph slider overlays as track-only controls without a nested value readout 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > scales graph slider overlay chrome with canvas zoom so the knob matches other elements 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders canvas 2d host with infinite canvas session 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders canvas 2d host with draw gradient/blend/overlay/meta scene records 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders puzzle 2d board host shell 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > uses the live puzzle 2d board camera for wheel persistence actions 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > coalesces puzzle 2d board events: drops transients, keeps the latest camera, coalesces nodeMove per id 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > coalesces puzzle 2d board events: drops nodeMove rows once a nodeDragEnd follows 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > flushes puzzle 2d board events immediately for select/brushPlace/edge/delete rows, not for camera/nodeMove alone 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > collects live mirror mutations: coalesces nodeMove to the latest per id, ignores unrelated rows 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > collects live mirror mutations: nodeDragEnd.moves produce final positions 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > collects live mirror mutations: preselect sets the live highlight, select/preselectCancel commit selection and clear it 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > peer registry: registers/unregisters, excludes own surfaceId and other controllerIds 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > peer gesture ownership: begin/end tracks the owning surfaceId; a pane never defers against its own gesture 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > pushes live mirror mutations into peer sessions, skipping the source pane 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > notifies peers when a gesture ends, skipping the source pane, passing whether it flushed 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > maps context menu specs onto UI items with icons, colors, hover, and select handlers 9ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > maps suggestion-style specs without a color swatch field 33ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > enriches context menu shortcuts from app keybindings 6ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > formats keybinding chords for menu shortcut labels 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > numbers suggestion menu rows with digit shortcuts for the first nine candidates 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > enriches context menu shortcuts from app keybindings via mapContextMenuSpecs 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > maps tiled-map interaction snapshots onto the current wasm sync ABI 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > parses a catalogue drag payload and builds a drop-preview JSON 5ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > rejects a catalogue drag payload without a kindId 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > parses a puzzle 3d catalogue drag payload and snaps drop origins to the grid 8ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > raycasts the Z=0 ground under orthographic top and perspective cameras 2ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > shares the world catalogue drop preview across all registered hosts 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > shares live world selection previews across sibling panes without allowing an idle pane to clear the active gesture 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > pushes fixture-drop previews to every board2d peer on the same controller 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > inverts the canonical screen-to-world transform for a fixture drop 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > puzzle2dWorldToScreen is the exact inverse of puzzle2dScreenToWorld 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > maps a world-centered node inside the viewport with canonical camera math 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > canvas-2d surface colors follow the light theme canvas token instead of a hardcoded dark fill 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders world 3d empty state without mounting r3f canvas 16ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > marks every non-empty world-3d host with the bottom-right orbit view gizmo 6ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > admits only the fixed localized fill diagnostic schema 30ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders the Fill progress fill label with visible and ARIA parity 9ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders the Füllfortschritt fill label with visible and ARIA parity 2ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > keeps world-3d orbit camera seed local per viewport once detached 2ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > buildWorldCameraDispatchArgs carries position/target/zoom/up but never a projection field 4ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > worldCameraSetCameraDispatchArgs nests the camera pose under a `camera` key, never flat alongside windowId 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > worldCameraPoseApproxEqual matches exact poses and float-noise, rejects a genuinely different pose 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > shouldReattachWorldViewportCamera suppresses a self-echo of the last dispatched camera but not a genuinely different pose 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > preserves projectionSpec.view from gizmo snaps instead of clobbering to top 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > accepts extended world 3d scene fields 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > parses GIS 3D terrain style JSON, defaulting missing fields, and rejects a missing tileUrlTemplate 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > blocks instance picking for fill, brush, and volume brush engagements but not move 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > resolves vortex pointer-down to select in brush or vertex mode and click-or-drag otherwise 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > scopes suggestion menu ownership to the opening world window so sibling panes stay interactive 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > revisions vortex materials when selection or hover state changes 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > revisions world mesh materials when style kind changes so deselection clears selected paint 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > uses the world surface selection mode instead of a stale shared invertive mode 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > resolves mesh style by priority: disabled > celebrated > selected > highlighted > hovered > neutral 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > celebrateWorldInstances stamps ids and cancel clears them so paint prefers celebrated over selected 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > maps edge hover to line paint so coplanar edges stay distinct from face hover fill 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > treats centerline meshes without shaded triangles as curve-only instances 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > derives marquee bounds from edge samples when positions are empty 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders the new group selection as active and only objects leaving the old selection as highlighted 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > builds addBrushObject args from a parsed brush preview, or null when there is nothing to place 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > defaults sourceVortexIndex to 0 when the brush preview omits it 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > resolves brush/suggestion ghost mesh URLs even when the kind is not yet among scene meshes 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > resolves the right-click context menu target by priority: vortex, then object, then reference 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > titles context menus from the specific hit before falling back to the surface 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > covers every target domain emitted by current surface pickers 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders text editor host 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders text editor host with hover/newline/rename scene fields 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > multiSpanReplace renames every occurrence and remaps spans 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > lineRangeAt finds the line containing an offset 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders table host with ui-react table 2ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > stamps a row's own id onto the rendered row's data-row-id attribute 4ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > dispatches a row action button's own ActionDescriptor, unmodified, on click 63ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders vcs history host with an ancestor graph fork 3ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > synchronizes raster selection and hover through the current native interaction API 58ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders paint-2d host canvas surface from document sync scene 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders paint-2d navigator host with the composite viewport overlay channel 3ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > renders paint-2d host empty fallback without a scene 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework renderer hosts > interprets virtual file system component scenes 31ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > dag marquee overlay > computes a rect overlay with numeric bounds for the rectangle method 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > dag marquee overlay > computes a rect overlay from the rust tuple-array wire format 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > dag marquee overlay > computes a lasso overlay carrying the raw points for the lasso method 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > dag marquee overlay > computes a lasso overlay from the rust tuple-array wire format 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > dag marquee overlay > infers lasso from a non-rectangular path when method is omitted 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > dag marquee overlay > infers rectangle from four axis-aligned corner points when method is omitted 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > dag marquee overlay > returns null for fewer than two points 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > dag marquee overlay > returns null for malformed point entries 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > dag marquee overlay > renders a rect overlay from a computeDagMarqueeOverlay rect result without crashing 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > dag marquee overlay > renders a polygon overlay from a computeDagMarqueeOverlay lasso result without crashing 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ink canvas host > renders the semio example composite scene with rich text, table, and math fallback 23ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ink canvas host > shows the grid pattern in composite mode but not in navigator mode 3ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ink canvas host > resizes with a minimum size and scales ink points when a group is resized 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ink canvas host > splits an ink stroke into fragments when erasing its middle point 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ink canvas host > round-trips bold and link marks between paragraphs and html 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ink canvas host > round-trips a clipboard payload of note blocks 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ink canvas host > computes ink block bounds from its local points 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ink canvas host > applies the canonical wheel-zoom camera formula symmetrically for screen<->world conversion 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > spawned window chrome > builds spawned engagement and measures chrome from program contributions 4ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > partitionWindowMeasures > unwraps a tagged group's children into utilityOptions only when its utility is active 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > partitionWindowMeasures > drops a tagged group from both buckets when a different or no utility is active 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > partitionWindowMeasures > keeps untagged groups and non-group measures in general, unaffected by the active utility 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > partitionWindowMeasures > wires a utility-scoped group into spawnedWindowChromeForKind's utilityOptions slot only when its utility is active 2ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > partitionWindowMeasures > parses the real ui_wgpu camelCase wire JSON and unwraps a utility-scoped fill group into flat utilityOptions (snake_case divergence regression guard) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > utility ribbon > sorts utility nodes by order 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > utility ribbon > recurses into a collection level only when the path names one of its collections 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > utility ribbon > ignores a path entry that no longer names an enabled collection at that level 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > utility ribbon > emits a picker segment alongside loose leaves at the same depth 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > utility ribbon > reconciles an active path by truncating at the first stale entry instead of substituting a default 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > utility ribbon > buckets top-level utility nodes into ordered category collections (uncategorized nodes default to tools now that the Actions category is gone) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > utility ribbon > drops separator-only category buckets so an empty group never appears as a picker option 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > utility ribbon > reuses a category's single already-meaningful collection instead of re-wrapping it, avoiding a duplicate-looking picker level 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > utility ribbon > still wraps a category with multiple top-level nodes in a synthetic collection 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > utility ribbon > scopes grouping to the given categories only 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > utility ribbon > deduplicates utility nodes by id across window utility lists for a single shared footer entry 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > utility ribbon > renders utility ribbon with picker and batched toggles 3ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > utility ribbon > stacks the window utility bar ribbon upward, showing only the base picker row until a group is activated 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > utility ribbon > renders UtilityTree with a custom id for per-window namespacing 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > utility ribbon > renders utilityOptions as an extra ribbon row when direction is up 17ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ui search/find (fuse re-export from @semio-tech/ui-react) > UISearch renders all items and fuzzy-filters them through the owned ranker 35ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > does not render a flow frame before its surface is ready 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > selects the flow engine for scenes with engine flow capabilities 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > ui search/find (fuse re-export from @semio-tech/ui-react) > UIFind renders and fuzzy-filters items registered on its context through the owned ranker 32ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > renders presence peers from the scene payload 3ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > parses a catalogue app drag payload, ignoring extra keys 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > rejects catalogue app drag payloads missing pluginId/appId, and garbage 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > window action panel — staging and single dispatch (P1/P2) > stages both args locally, dispatches nothing until Execute, then fires exactly one merged descriptor and keeps staged values 66ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > builds a ghost neuron descriptor, preferring label over appId 0ms
stderr | 🧪️index.test.ts > s workflow flow routing > isolates render faults in ShellFaultBoundary
Error: boom
    at FaultyChild (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/ð§ªï¸index.test.ts:5124:7)
    at react_stack_bottom_frame (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:25904:19)
    at renderWithHooks (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:7662:21)
    at updateFunctionComponent (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:10166:18)
    at runWithFiberInDEV (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:874:12)
    at performUnitOfWork (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:17641:21)
    at workLoopSync (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:17469:40)
    at renderRootSync (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:17450:10)
    at performWorkOnRoot (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:16583:34)
    at performWorkOnRootViaSchedulerTask (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:18957:6)
    at flushActQueue (/Users/ueli/Documents/semio/node_modules/react/cjs/react.development.js:590:33)
    at /Users/ueli/Documents/semio/node_modules/react/cjs/react.development.js:884:9
    at /Users/ueli/Documents/semio/node_modules/@testing-library/react/dist/act-compat.js:46:24
    at renderRoot (/Users/ueli/Documents/semio/node_modules/@testing-library/react/dist/pure.js:189:17)
    at render (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/ð§ªï¸render.ts:35:18)
    at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/ð§ªï¸index.test.ts:5126:27
    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:1903:25
    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2326:19
    at Promise (native)
    at runWithCancel (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2323:9)
    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2305:19
    at Promise (native)
    at runWithTimeout (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2272:9)
    at run (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:1150:19)
    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2955:12
    at callAroundHooks (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2663:8)
    at callAroundEachHooks (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2799:7)
    at processTicksAndRejections (native) {
  [message]: 'boom',
  [stack]: 'Error: boom\n' +
    '    at FaultyChild (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/ð\x9F§ªï¸\x8Findex.test.ts:5124:7)\n' +
    '    at react_stack_bottom_frame (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:25904:19)\n' +
    '    at renderWithHooks (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:7662:21)\n' +
    '    at updateFunctionComponent (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:10166:18)\n' +
    '    at runWithFiberInDEV (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:874:12)\n' +
    '    at performUnitOfWork (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:17641:21)\n' +
    '    at workLoopSync (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:17469:40)\n' +
    '    at renderRootSync (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:17450:10)\n' +
    '    at performWorkOnRoot (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:16583:34)\n' +
    '    at performWorkOnRootViaSchedulerTask (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:18957:6)\n' +
    '    at flushActQueue (/Users/ueli/Documents/semio/node_modules/react/cjs/react.development.js:590:33)\n' +
    '    at /Users/ueli/Documents/semio/node_modules/react/cjs/react.development.js:884:9\n' +
    '    at /Users/ueli/Documents/semio/node_modules/@testing-library/react/dist/act-compat.js:46:24\n' +
    '    at renderRoot (/Users/ueli/Documents/semio/node_modules/@testing-library/react/dist/pure.js:189:17)\n' +
    '    at render (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/ð\x9F§ªï¸\x8Frender.ts:35:18)\n' +
    '    at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/ð\x9F§ªï¸\x8Findex.test.ts:5126:27\n' +
    '    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:1903:25\n' +
    '    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2326:19\n' +
    '    at Promise (native)\n' +
    '    at runWithCancel (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2323:9)\n' +
    '    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2305:19\n' +
    '    at Promise (native)\n' +
    '    at runWithTimeout (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2272:9)\n' +
    '    at run (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:1150:19)\n' +
    '    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2955:12\n' +
    '    at callAroundHooks (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2663:8)\n' +
    '    at callAroundEachHooks (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2799:7)\n' +
    '    at processTicksAndRejections (native)',
  [line]: 0,
  [column]: 0
}

The above error occurred in the <FaultyChild> component.

React will try to recreate this component tree from scratch using the error boundary you provided, ShellFaultBoundary.

[DEBUG] shell fault test Error: boom
    at FaultyChild (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/ð§ªï¸index.test.ts:5124:7)
    at react_stack_bottom_frame (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:25904:19)
    at renderWithHooks (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:7662:21)
    at updateFunctionComponent (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:10166:18)
    at runWithFiberInDEV (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:874:12)
    at performUnitOfWork (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:17641:21)
    at workLoopSync (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:17469:40)
    at renderRootSync (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:17450:10)
    at performWorkOnRoot (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:16583:34)
    at performWorkOnRootViaSchedulerTask (/Users/ueli/Documents/semio/node_modules/react-dom/cjs/react-dom-client.development.js:18957:6)
    at flushActQueue (/Users/ueli/Documents/semio/node_modules/react/cjs/react.development.js:590:33)
    at /Users/ueli/Documents/semio/node_modules/react/cjs/react.development.js:884:9
    at /Users/ueli/Documents/semio/node_modules/@testing-library/react/dist/act-compat.js:46:24
    at renderRoot (/Users/ueli/Documents/semio/node_modules/@testing-library/react/dist/pure.js:189:17)
    at render (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/ð§ªï¸render.ts:35:18)
    at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/ð§ªï¸index.test.ts:5126:27
    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:1903:25
    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2326:19
    at Promise (native)
    at runWithCancel (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2323:9)
    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2305:19
    at Promise (native)
    at runWithTimeout (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2272:9)
    at run (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:1150:19)
    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2955:12
    at callAroundHooks (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2663:8)
    at callAroundEachHooks (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2799:7)
    at processTicksAndRejections (native) 
    at FaultyChild (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts:4489:22)
    at ShellFaultBoundary (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🟦️component.tsx:549:10)

 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > builds addWidget descriptors from catalogue items 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > ranks catalogue suggestions by exact/prefix match with neurons first 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > returns every catalogue item for empty query without a 20-item cap 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > enables overflow scrolling only when spotlight suggestions are expanded 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > hosts a semantic tree document inside a panel leaf 27ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > bridges semantic intent scope, version, args, and input into the plugin action channel 2ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > resolves a fixture widget id to its workflow instance id, independent of selection state 5ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > parses studio and studio+instance shell paths, and rejects non-studio routes 2ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > classifies shell routes into landing, space, and notFound 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > shellActorId mints user:{userId}#{sessionId} once identity resolves, else client-{sessionId} 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > canonicalSurfaceId formats <kind>@<standard>/<subset>#<role> 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > reloadRetainsActiveApp accepts extension-only programs and rejects a dropped active app 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > directoryCommandFromAction maps all 7 frozen os.directory.* ids, share-link sugaring to create-invite 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > AutoCheckinScheduler (§C5 auto check-in) > 3 edits then idle ⇒ exactly one commitCheckpoint 7ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > AutoCheckinScheduler (§C5 auto check-in) > ≥ 200 uncommitted edits ⇒ checkpoint without waiting for idle 2ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > AutoCheckinScheduler (§C5 auto check-in) > notify(0) (a landed checkpoint) clears the pending latch for a fresh idle window later 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > AutoCheckinScheduler (§C5 auto check-in) > cancel() stops a pending idle timer (unmount/session-switch) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > sync status pill (§C5 status pill, ArtifactSyncStatus → persisted|pending(n)|remote(...)) > persisted: a live remote with nothing pending 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > sync status pill (§C5 status pill, ArtifactSyncStatus → persisted|pending(n)|remote(...)) > pending(n): a live remote with unacked mutations 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > sync status pill (§C5 status pill, ArtifactSyncStatus → persisted|pending(n)|remote(...)) > remote(connecting|backoff|detached): a non-live remote takes priority over a pending count 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > sync status pill (§C5 status pill, ArtifactSyncStatus → persisted|pending(n)|remote(...)) > no status observed yet reads as remote(detached) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > canCheckIn is true only for an editor role — viewer gets no affordance and no auto timer 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > isolates render faults in ShellFaultBoundary 15ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > s workflow flow routing > folds spawned focus into viewState so a subsequent host-effect session write keeps activeSpawnedId 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > window action panel — staging and single dispatch (P1/P2) > gates Execute on required args, but a default-satisfied required arg counts without staging 193ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > window action panel — staging and single dispatch (P1/P2) > Reset restores defaults while keeping the form expanded 30ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > window action panel — staging and single dispatch (P1/P2) > a zero-arg action row fires immediately with no args object 6ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > window action panel — staging and single dispatch (P1/P2) > renders every row disabled when an active utility gates actions 6ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > window action panel — staging and single dispatch (P1/P2) > groups actions into category sections like the command panel 16ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > palette redirect and keybinding rule (P3/P4) > only arg-carrying actions redirect to a staged form (P3 decision) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > palette redirect and keybinding rule (P3/P4) > keybinding intent: arg-less fires, arg-action opens unless already expanded and valid then executes (P4) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > registry-derived utilities and activation (P5) > resolveUtilities scopes to the window kind's refs, falling back to all app utilities when unset 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > registry-derived utilities and activation (P5) > derives grouped utility nodes with the active utility pressed and a setActiveUtility onChange tagged by window 4ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > registry-derived utilities and activation (P5) > deriveUtilityNodes twin marks exactly the active utility pressed 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > registry-derived utilities and activation (P5) > deriveUtilityNodes hoists a single-child group to a top-level toggle 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > registry-derived utilities and activation (P5) > resolveUtilityActivation toggles: click activates, re-click or empty deactivates 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > registry-derived utilities and activation (P5) > findPressedUtilityLeafId walks nested collections 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > registry-derived utilities and activation (P5) > isWorldTransformGumballMode requires an explicit move/rotate/scale/transform mode 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > registry-derived utilities and activation (P5) > worldGumballConfigForProjection intersects transform mode with planar window projections 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > registry-derived utilities and activation (P5) > gumballTransformDeltaBetweenPoses emits incremental translate/rotate/scale args 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > registry-derived utilities and activation (P5) > gumballLivePreviewDeltaBetweenPoses applies local start→current deltas for instant mid-drag preview 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > registry-derived utilities and activation (P5) > resolveWindowActions preserves every definition owned by the window 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > registry-derived utilities and activation (P5) > panelTabDefinitionToNode maps the framework-injected History panel tab through its rendered body 10ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveCommands / commandCategories (footer command panel registry) > aggregates os + program + app-scope + active-mode's mode-scope commands, excluding other modes' mode-scope commands 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveCommands / commandCategories (footer command panel registry) > switching the active mode swaps which mode-scope commands resolve 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveCommands / commandCategories (footer command panel registry) > resolves only os commands with no session (null program manifest / app) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveCommands / commandCategories (footer command panel registry) > owner-qualifies identical local command ids into collision-free UI keys 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveCommands / commandCategories (footer command panel registry) > commandCategories orders and dedupes categories by first appearance 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveModeTools / buildToolTabs (footer tool panel registry) > resolves the active mode's tools in declared order 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveModeTools / buildToolTabs (footer tool panel registry) > tools are opt-in per mode — no orphan fallback for a mode that declares none 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveModeTools / buildToolTabs (footer tool panel registry) > resolves nothing for an app/mode that doesn't exist 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveModeTools / buildToolTabs (footer tool panel registry) > buildToolTabs builds one leaf per resolved tool, whose lazily-resolved tree reflects the current active tool and its measures 7ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveModeTools / buildToolTabs (footer tool panel registry) > buildToolTabs' activation toggle dispatches setActiveTool with this tool's id 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveModeTools / buildToolTabs (footer tool panel registry) > toolIdFromPanelTabId extracts the mode tool id from a tool leaf tab id 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > Introduce App command > is available only for apps with an introduction 3ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > Introduce App command > starts the introduction at its first step 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > Play/Record Tutorial commands > os.playTutorial appears only when at least one tutorial is declared, offering each as a Select option 3ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > Play/Record Tutorial commands > os.recordTutorial appears only when the recorder is available (dev/studio), independent of declared tutorials 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > Play/Record Tutorial commands > os-scope Play/Record Tutorial commands are NOT handled by dispatchOsCommand (routed earlier, through the shell's own startTutorialRef/toggleTutorialRecordingRef bridge) 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell option locks (SEMIO_LOCKED_*) > resolves valid locale/appearance and falls back with a warning on invalid values while staying locked 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell option locks (SEMIO_LOCKED_*) > accepts any non-empty terminology id verbatim (app-declared ids can't be validated at boot) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell option locks (SEMIO_LOCKED_*) > returns an empty object for undefined locks 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell option locks (SEMIO_LOCKED_*) > initialShellState applies locked values over stored/default prefs 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell option locks (SEMIO_LOCKED_*) > mergeShellLockSources keeps brand locks locked and lets a defined env lock win per key 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell option locks (SEMIO_LOCKED_*) > resolveShellDefaults prefers env defaults over brand defaults and initialShellState seeds without locking 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell option locks (SEMIO_LOCKED_*) > resolveBootExampleId seeds the first registered example when nothing is active or defaulted 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell option locks (SEMIO_LOCKED_*) > shouldReplayIntroductionOnLoad opts a brand into replaying its tour after every window refresh 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell option locks (SEMIO_LOCKED_*) > isEphemeralShellBrand skips durable shell state so a refresh boots from brand defaults only 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell option locks (SEMIO_LOCKED_*) > registers all six Entwerfen mit Bestand demonstrator shell brands 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell option locks (SEMIO_LOCKED_*) > ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND introduction is app-specific only after the general landing tour was split out 4ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell option locks (SEMIO_LOCKED_*) > mit-bestand/demonstrator footer credits render the funding/partner logos, links, and locale text 18ms
stdout | 🧪️index.test.ts > TutorialRecorder LocalizedLabel synthesis > resolves exact language-neutral label cells without a default locale
🌐 i18next is made possible by our own product, Locize — consider powering your project with managed localization (AI, CDN, integrations): https://locize.com 💙

stderr | 🧪️index.test.ts > host effect dispatch (D2 DispatchAction, D3 RequestFileOpen.multiple, D5 RequestMediaFrames) > runRequestMediaFrames (D5): Tier 2 failure (video element throws mid-seek) ⇒ dispatches fallbackAction exactly once with raw bytes as a data URL, no frame/done calls
[os-shell] requestMediaFrames: decode failed, falling back to raw bytes Error: decode failed
    at set (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/ð§ªï¸index.test.ts:6203:9)
    at runTier2VideoFrames (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/ð¦ï¸component.tsx:889:5)
    at runRequestMediaFrames (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/ð¦ï¸component.tsx:963:13)
    at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/ð§ªï¸index.test.ts:6206:11
    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:1903:25
    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2326:19
    at Promise (native)
    at runWithCancel (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2323:9)
    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2305:19
    at Promise (native)
    at runWithTimeout (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2272:9)
    at run (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:1150:19)
    at /Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2955:12
    at callAroundHooks (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2663:8)
    at callAroundEachHooks (/Users/ueli/Documents/semio/node_modules/@vitest/runner/dist/chunk-artifact.js:2799:7)
    at processTicksAndRejections (native)

 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell option locks (SEMIO_LOCKED_*) > buildOsCommands omits only the commands for locked prefs 4ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > TutorialRecorder LocalizedLabel synthesis > TutorialRecorder synthesizes LocalizedLabel for addChapter and build titles 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell option locks (SEMIO_LOCKED_*) > dispatchOsCommand is a no-operation for a locked pref even if invoked directly 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > TutorialRecorder LocalizedLabel synthesis > FrameworkOsShell portal layer is unconstrained by z-tutorial so portaled elements sit above elevated windows 166ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > buildCommandCategoryTree / buildCommandCategoryTabs (command palette as a real bottom-middle Panel) > a zero-arg command row fires onExecute directly on click; only one command-list section is present when nothing is expanded 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > buildCommandCategoryTree / buildCommandCategoryTabs (command palette as a real bottom-middle Panel) > auto-expands a singleton arg-carrying category into a flat form with section actions and no disclosure list 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > buildCommandCategoryTree / buildCommandCategoryTabs (command palette as a real bottom-middle Panel) > an arg-carrying command row toggles expansion instead of executing, and a synthetic arg-form section only appears while expanded 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > buildCommandCategoryTree / buildCommandCategoryTabs (command palette as a real bottom-middle Panel) > Execute is disabled until the required arg is staged, and calling it passes the effective (staged) args; Reset dispatches onResetArgs 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > buildCommandCategoryTree / buildCommandCategoryTabs (command palette as a real bottom-middle Panel) > buildCommandCategoryTabs builds one namespaced PanelTabLeaf per category, whose lazily-resolved tree only contains that category's commands 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > host effect dispatch (D2 DispatchAction, D3 RequestFileOpen.multiple, D5 RequestMediaFrames) > encodes recursive host-effect actions as fully scoped JSON at the runtime boundary 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > host effect dispatch (D2 DispatchAction, D3 RequestFileOpen.multiple, D5 RequestMediaFrames) > scheduleDispatchAction (D2): fires dispatchOne with action/args only after delayMs elapses 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > host effect dispatch (D2 DispatchAction, D3 RequestFileOpen.multiple, D5 RequestMediaFrames) > scheduleDispatchAction (D2): delayMs 0 still defers to a scheduled tick, not a synchronous call 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > host effect dispatch (D2 DispatchAction, D3 RequestFileOpen.multiple, D5 RequestMediaFrames) > dispatchOpenedFiles (D3): single-file (multiple=false) makes exactly one call with {payload, name} and no index/total 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > host effect dispatch (D2 DispatchAction, D3 RequestFileOpen.multiple, D5 RequestMediaFrames) > dispatchOpenedFiles (D3): multiple=true dispatches once per file, in order, each extended with {index, total} 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > host effect dispatch (D2 DispatchAction, D3 RequestFileOpen.multiple, D5 RequestMediaFrames) > sampleMediaFrameTimestampsMs (D5): steps by sampleStride/fpsHint seconds, capped at maxFrames 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > host effect dispatch (D2 DispatchAction, D3 RequestFileOpen.multiple, D5 RequestMediaFrames) > runTier2VideoFrames (D5): dispatches frameAction once per sampled timestamp, in order, then doneAction exactly once 2ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > host effect dispatch (D2 DispatchAction, D3 RequestFileOpen.multiple, D5 RequestMediaFrames) > runRequestMediaFrames (D5): Tier 2 failure (video element throws mid-seek) ⇒ dispatches fallbackAction exactly once with raw bytes as a data URL, no frame/done calls 19ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > host effect dispatch (D2 DispatchAction, D3 RequestFileOpen.multiple, D5 RequestMediaFrames) > runRequestMediaFrames (D5): payload bytes in hand ⇒ Tier 2 seek-capture runs, ending in doneAction (no picker needed) 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > Display Windows tab — projection drag templates > shows window kind icons on section headers and kind rows 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > Display Windows tab — projection drag templates > nests the full Parallel/Perspective projection taxonomy under a world-3d window kind 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > Display Windows tab — projection drag templates > keeps a flat single drag entry for non-world-3d window kinds 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > Display Windows tab — projection drag templates > pre-reverses every level so the bottom-anchored (direction="up") Tree's own sibling-reversal renders Parallel children top-to-bottom 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > Display Windows tab — projection drag templates > each projection leaf's drag payload decodes back to its WorldProjectionSpec 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > createFrameworkSettingsPanelTab > exposes one Settings toggle whose children are General, Theme, and Hotkeys tabs 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > createFrameworkMarketplacePanelTab > shows an unavailable placeholder when no host is mounted yet 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > createFrameworkMarketplacePanelTab > groups plugins into one section per source, sorted by pluginId within a source 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > createFrameworkMarketplacePanelTab > integrates extensions as children of their owning plugin 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > createFrameworkMarketplacePanelTab > marks installing/reloading rows as loading, and every status is reflected in the row label 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > createFrameworkMarketplacePanelTab > routes install/uninstall/reload clicks for one row back through the host without touching others 4ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > createFrameworkMarketplacePanelTab > disables uninstall for the host/primary plugin and the active session's plugin (canUninstall: false) 7ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > introductionTargetsWindow > matches both the window kind and every open instance of that kind 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > introductionTargetsWindow > matches action-rail segments against the kind and its instances 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > windowMeasureTreeContainsId > finds nested measure ids used as introduction targets 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > renderWindowMeasuresTree > puts toggle icons before labels and uses checkboxes instead of icon toggles 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveFrameworkLayoutSeed — multi-pane default layouts > does not infer focus when an app has no explicit layout 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveFrameworkLayoutSeed — multi-pane default layouts > hydrates Top (1/3) + Perspective (2/3) instances and projection templates 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveFrameworkLayoutSeed — multi-pane default layouts > treats instance-id panes as extras so the host fetches bodies keyed by instance id, not only by kind 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveFrameworkLayoutSeed — multi-pane default layouts > re-derives window titles from localized windowKind labels on locale/terminology switch 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > resolveFrameworkLayoutSeed — multi-pane default layouts > re-derives titles for extra window instances based on their windowKindId 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > classifyWindowLayoutChange > returns null when the layout is identical (deep-equal, not just same reference) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > classifyWindowLayoutChange > returns null for a pure active-window-flag change (skeleton and sizes both unchanged) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > classifyWindowLayoutChange > returns 'resize' when only pane sizes differ 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > classifyWindowLayoutChange > returns 'rearrange' when window ids/nesting structure differ (drag-to-new-position, split, close) 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > classifyWindowLayoutChange > returns null when both are null 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > noteShellCommand > buildNoteShellCommandAction builds a noteShellCommand action descriptor targeting the given controller, carrying detail only when provided 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > noteShellCommand > is excluded from tutorial recording, alongside world-navigation/introduction/tutorial-control action ids 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > TutorialRecorder LocalizedLabel synthesis > synthesizeLocalizedLabel broadcasts a string across all 4 cells (native/reuse x en/de) 4ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > TutorialRecorder LocalizedLabel synthesis > resolves exact language-neutral label cells without a default locale 74ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > stale-before-work at one item/256 bytes matches the reference rejection order and Immer value oracle 6ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > one-leaf-large-table at one item/4096 bytes matches the reference rejection order and Immer value oracle 1761ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > whole-subtree-retained-delete at one item/4096 bytes matches the reference rejection order and Immer value oracle 1876ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > backedge-cycle at one item/4096 bytes matches the reference rejection order and Immer value oracle 7ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > missing-child at one item/4096 bytes matches the reference rejection order and Immer value oracle 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > long-unicode-sibling-key at one item/4096 bytes matches the reference rejection order and Immer value oracle 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > unchecked-dangling-snapshot at one item/4096 bytes matches the reference rejection order and Immer value oracle 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > deep-chain at one item/4096 bytes matches the reference rejection order and Immer value oracle 346ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > stale-before-work at one item/4096 bytes matches the reference rejection order and Immer value oracle 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > cancels every semantic phase without publishing or invalidating an old captured reader 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI atomic publication > mints an acknowledgement only after exact root and revision publication 3ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI atomic publication > rejects rebound owners and stale concurrent candidates without emitting ACK 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI atomic publication > rejects a different surface and cancellation of a ready candidate 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore transactions > applies every op kind and advances the revision 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore transactions > rejects a stale baseRevision and leaves state reference-identical 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore transactions > rejects a cycle and leaves state unchanged 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore transactions > rejects an unknown node target 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore transactions > rejects an oversized patch by op count 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore transactions > removes a whole orphaned subtree 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore per-node subscription granularity > notifies only the changed node's listeners, not siblings 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore per-node subscription granularity > does not notify any node listener on a rejected patch 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > emitIntent > carries the store's current revision and a monotonic per-surface seq 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > emitIntent > returns undefined when the node has no binding for that trigger 0ms

 Test Files  4 passed (4)
      Tests  521 passed (521)
   Start at  13:17:55
   Duration  26.52s (transform 33.53s, setup 0ms, import 44.78s, tests 28.27s, environment 2.99s)




 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react



```

