# Renderer R10 Quick Diagnostic

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache --args='--reporter=verbose'`

Exit code: 1. Uses the existing unmodified quick profile and verbose reporter after R8 timed out and R9 rejected duplicate timeout arguments. No production watchdog or test budget was enlarged. This diagnostic is not a replacement for the full canonical long gate.

```text

> nx run @semio-tech/framework-renderer-react:test-quick --args=--reporter=verbose

> bun ./📜️script.ts test quick --reporter=verbose

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

 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > command-ingress fault diagnostics > decodes the normalized scalar-wire fault payload instead of hiding the terminal cause 3ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > instance-open retained UI lifecycle > refreshes window and panel surfaces from the language-agnostic ownership cases 10ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > instance-open retained UI lifecycle > acknowledges only patches that identify the exact retained surface 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > instance-open retained UI lifecycle > retains the first render patch so an unchanged surface-visible probe can reuse it 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > runs proposal -> prepare x2 -> commit, committing in reverse discovery order 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > undoGroup fans TransactionUndo out to every member of a completed transaction 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > rolls back a commit-failed transaction: undoes what already committed, rolls back the rest 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > reaches transaction.unknown-target when the initiator plugin has no registered handle 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > reaches transaction.unknown-target when InstanceDirectory has no entry for the foreign target 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > reaches transaction.unknown-mutation when the router has no entry for a foreign step 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > reaches transaction.contribution-not-permitted when a contributed mutation has no planner wired 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > plans and prepares a contributed mutation using the target's cached document pack 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > reaches transaction.cycle when the same (artifact, mutation, payload) step repeats 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > reaches transaction.depth-exceeded when a foreign-step chain runs past MAX_TRANSACTION_DEPTH 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime TransactionCoordinator > passes a member's TransactionPrepared.rejection code straight through — instance-busy, generation-mismatch, and the member-rejected default all reachable 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime documentPack/transaction wire adapter > adaptPluginHandle's documentPack/transactionPrepare/transactionCommit/transactionRollback/transactionUndo/transactionRedo frame through AppChannelClient 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime documentPack/transaction wire adapter > documentPack() reflects the cache after loadAppDocumentPack() — the adapter reads the SAME live channel it just loaded through 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > continues admitted operations after surfaces are retained and ACKs each exact result 7ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > does not replay already acknowledged ingress publications during settlement 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > validates fixed result page authority and preserves document and download effects 3ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > keeps the command reply when publication supplies only an unsolicited UI scope 3ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > drains an actor's more-work turns until the reconciled UI patch is publishable 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > acknowledges each retained surface before requesting the next bounded publication 5ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > rejects an idle actor that did not publish a requested surface instead of retaining a loading placeholder 5ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > does not chase background work during instance-open before a UI surface is requested 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > drains until every missing requested surface has published its first patch 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > OwnedWire > validates strict neutral native bounds and canonical Rust byte vectors 35ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > OwnedWire > rejects malformed canonical framing and values without publishing a partial root 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > OwnedWire > owns large nested immutable values and preserves __proto__ as data 104ms
stderr | ../../../../🧱️elements/Interpreter/🟦️component.tsx
THREE.WARNING: Multiple instances of Three.js being imported.

stderr | 🧪️index.test.ts
THREE.WARNING: Multiple instances of Three.js being imported.

 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > allows a large retained surface to reconcile beyond the former continuation ceiling 477ms
stderr | ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime shard-loss wiring (real restore, not just a console.error) > handlePluginShardLost delegates to ActivationRegistry.handleShardLost for EXACTLY the affected actorIds
[DEBUG] PluginRuntime: shard 2 lost, restoring actors: plugin-a#1, plugin-b#7

 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > yields the browser event loop while a retained surface needs several continuation batches 3ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > does not poll for an unchanged refresh when every requested surface is already retained 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > dispatches a queued Interactive-lane turn before an already-queued UserVisible-lane turn for the SAME actor, regardless of arrival order 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > collapses a 200-call coalescing burst to a single dispatched turn, resolving EVERY waiter (not just the last) with the winning result 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > teardown rejects queued turn waiters before disposing the actor transport 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > surfaces Rejected once an actor's mailbox is genuinely full of distinct turns, instead of growing without bound 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue) > never dispatches a second turn for an actor before the first settles, even while a DIFFERENT actor's turns run concurrently 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime shard-loss wiring (real restore, not just a console.error) > handlePluginShardLost delegates to ActivationRegistry.handleShardLost for EXACTLY the affected actorIds 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > PluginRuntime shard-loss wiring (real restore, not just a console.error) > buildShardClientOptions wires onShardLost to handlePluginShardLost (not a bare console.error) and sizes shardCount via poolConcurrency 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > fetchDescriptorManifest AbortSignal > propagates an aborted signal's fetch rejection instead of silently falling back to an empty manifest 38ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > fetchDescriptorManifest AbortSignal > still falls back to an empty manifest on a genuine (non-abort) fetch failure — the existing E1-describe/W3 gap, unchanged 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > fetchDescriptorManifest AbortSignal > treats the dev server's HTML SPA fallback as an absent descriptor without a parse warning 20ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > loadPluginModulesInDependencyOrder — level-parallel boot > runs independent siblings in parallel within a level, holding a dependent until its dependency's WHOLE level finishes 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > loadPluginModulesInDependencyOrder — level-parallel boot > bounds within-level concurrency to the given limit — a third independent sibling waits for a free slot 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > loadPluginModulesInDependencyOrder — level-parallel boot > cascades a runtime load failure to skip dependents, while unrelated siblings still load — a distinct PluginLoadFailure, not a PluginGraphError 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > loadPluginModulesInDependencyOrder — level-parallel boot > defaults its concurrency bound to poolConcurrency() when the caller doesn't override it 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/PluginRuntime/🟦️component.tsx > loadPluginModulesInDependencyOrder — level-parallel boot > aborts cleanly: aborting mid-boot stops starting new loads without throwing, while an already-started load still settles normally 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > OwnedWire > cancels at every decoding phase and rejects shared or aliased input ownership 24ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > OwnedWire > links admitted scalar and collection bounds to the owning native schema 7ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > strictly validates its language-neutral lifecycle and UTF8 fixture 10ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > one-leaf-large-table at one item/256 bytes matches the reference rejection order and Immer value oracle 1980ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > whole-subtree-retained-delete at one item/256 bytes matches the reference rejection order and Immer value oracle 1025ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > backedge-cycle at one item/256 bytes matches the reference rejection order and Immer value oracle 4ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > missing-child at one item/256 bytes matches the reference rejection order and Immer value oracle 8ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > long-unicode-sibling-key at one item/256 bytes matches the reference rejection order and Immer value oracle 31ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > unchecked-dangling-snapshot at one item/256 bytes matches the reference rejection order and Immer value oracle 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > deep-chain at one item/256 bytes matches the reference rejection order and Immer value oracle 171ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > unknown component placeholder > renders a visible placeholder and never nothing for an unregistered component type 206ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > per-node render granularity (React level) > re-renders only the component whose own record changed 7ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > loads all 62 corpus fixtures 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/button — accept 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/number-stepper — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/separator — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/key-value-list — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/input — accept 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/state-activity-waiting — accept 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/icon-select — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/state-with-menu — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/state-transition-introducing — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/select — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/container — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/tree — accept 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/state-activity-finished — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/state-disabled — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/surface — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/ring — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/toggle — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/text — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/state-activity-loading — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/extension — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/image — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/state-transition-celebrating — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🧩️component/slider — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 📐️layout/stack — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 📐️layout/nesting — accept 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 📐️layout/grid — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 📐️layout/scroll — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 📐️layout/leaf — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 📐️layout/absolute — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 📐️layout/overlay — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🖥️composite/surface-embedded — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🖥️composite/form-with-validation — accept 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/Interpreter/🟦️component.tsx > conformance corpus > 🖥️composite/dialog — accept 3ms
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
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > tutorial document wire contract > keeps native document-track names and bidirectional event order 30ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > deduplicates module loads and retries an exact failed 'network' attempt 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > deduplicates module loads and retries an exact failed 'initialization' attempt 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > stale-before-work at one item/256 bytes matches the reference rejection order and Immer value oracle 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > one-leaf-large-table at one item/4096 bytes matches the reference rejection order and Immer value oracle 1395ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > keeps identical Board controller/surface keys isolated across mounted shell scopes 85ms
stderr | 🧪️index.test.ts > app-owned surface session factories > retires the exact mounted Board session once after 'ready' cancellation
The current testing environment is not configured to support act(...)

stderr | 🧪️index.test.ts > app-owned surface session factories > retires the exact mounted Board session once after 'ready' cancellation
The current testing environment is not configured to support act(...)
The current testing environment is not configured to support act(...)

 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > keeps a remounted peer and gesture registered after the old attachment rejects 118ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > retires the exact mounted Board session once after 'constructing' cancellation 2ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > retires the exact mounted Board session once after 'attaching' cancellation 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > retires the exact mounted Board session once after 'ready' cancellation 15ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > app-owned surface session factories > joins exact plugin and app ownership while keeping instance scopes distinct 15ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > plugin session ownership > lets only the configured primary plugin establish a missing session 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > plugin session ownership > configures only the active aggregate outside studio mode 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework sync utilities > renders the real SyncAttachCard popover as a dismissible nonmodal dialog 49ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework sync utilities > builds three sync backbone toggles 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework sync utilities > has no active toggle when detached 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework sync utilities > groups File, Folder, and Remote under a single Sync category collection 2ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > live measure dispatch > serializes document updates and skips stale slider values 53ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > live measure dispatch > coalesces straight slider movement but preserves a down-up reversal 102ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > live measure dispatch > caps queued direction reversals so a jittery drag cannot grow the queue unbounded 106ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > reveal cutoff store > notifies only same-group subscribers and reflects the latest set value 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > reveal cutoff store > isRevealCutoffHidden hides instances at or past the live cutoff, and never instances without a revealIndex 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > reveal cutoff store > isRevealCutoffHidden treats a JSON null revealIndex as untagged, even at the boot cutoff of 0 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > reveal cutoff store > committed reveal cutoff reconciliation ignores same-value identity churn so a live fill drag is not reset by fillBuildTick 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > in-flight skipping interval > drops overlapping ticks instead of queueing them behind a slow run 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > coalescing action dispatcher > dedupes unchanged values and keeps at most one in-flight dispatch 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > shows terminal boot content instead of an infinite loading canvas 11ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > starts every panel anchor at the same 300px width 2ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > toggles the overlays slice via a direct value without touching unrelated slices 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > starts, advances, and dismisses an introduction via SET_INTRODUCTION_STEP without touching unrelated slices 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > auto-starts each introduction launch once and keeps a skipped replay-on-load introduction dismissed 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > COMPLETE_INTRODUCTION_INTERACTION appends and dedupes indices; SET_INTRODUCTION_STEP resets them 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > opens, replaces, and closes a dialog via SET_DIALOG without touching unrelated slices 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > toggles the layout slice via an updater function 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > toggles a middle anchor via SET_PANEL_VISIBLE the same way as a corner 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > rewrites window icons via SET_WINDOW_ICON for extras and base kinds 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > shell store reducer > rewrites window titles via SET_WINDOW_TITLE for extras and base kinds 0ms
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
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > serializePerActor queues concurrent turns for the same actor one at a time, never overlapping 18ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > serializePerActor keys independently per actor — different actors run concurrently, not queued behind each other 10ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > serializePerActor keeps queuing subsequent turns after an earlier one rejects 1ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > serializes complete multi-turn command ingress sequences for one actor 0ms
 ✓ |@semio-tech/framework-renderer-react| 🧪️index.test.ts > framework plugin runtime > adaptPluginHandle.handleAction round-trips an action's output/uiScope/historyPatch from AppFrame::Invocation; requestedEffects is honestly empty for a bare command-only handle 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > whole-subtree-retained-delete at one item/4096 bytes matches the reference rejection order and Immer value oracle 895ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > backedge-cycle at one item/4096 bytes matches the reference rejection order and Immer value oracle 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > missing-child at one item/4096 bytes matches the reference rejection order and Immer value oracle 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > long-unicode-sibling-key at one item/4096 bytes matches the reference rejection order and Immer value oracle 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > unchecked-dangling-snapshot at one item/4096 bytes matches the reference rejection order and Immer value oracle 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > deep-chain at one item/4096 bytes matches the reference rejection order and Immer value oracle 150ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > stale-before-work at one item/4096 bytes matches the reference rejection order and Immer value oracle 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI patch preparation > cancels every semantic phase without publishing or invalidating an old captured reader 4ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI atomic publication > mints an acknowledgement only after exact root and revision publication 2ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI atomic publication > rejects rebound owners and stale concurrent candidates without emitting ACK 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > Retained UI atomic publication > rejects a different surface and cancellation of a ready candidate 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore transactions > applies every op kind and advances the revision 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore transactions > rejects a stale baseRevision and leaves state reference-identical 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore transactions > rejects a cycle and leaves state unchanged 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore transactions > rejects an unknown node target 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore transactions > rejects an oversized patch by op count 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore transactions > removes a whole orphaned subtree 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore per-node subscription granularity > notifies only the changed node's listeners, not siblings 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > UiDocumentStore per-node subscription granularity > does not notify any node listener on a rejected patch 0ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > emitIntent > carries the store's current revision and a monotonic per-surface seq 1ms
 ✓ |@semio-tech/framework-renderer-react| ../../../../🧱️elements/UiDocumentStore/🟦️component.tsx > emitIntent > returns undefined when the node has no binding for that trigger 0ms
[budget] /Users/ueli/.bun/bin/bun /Users/ueli/Documents/semio/node_modules/vitest/vitest.mjs run --config 🧪️vitest.config.ts --passWithNoTests --testTimeout 30000 --hookTimeout 30000 --teardownTimeout 30000 --reporter=verbose exceeded 30000ms — killed. Trim it, or assign it to a higher level (quick/long/exhaustive).
Warning: command "bun ./📜️script.ts test quick --reporter=verbose" exited with non-zero status code


 NX   Running target test-quick for project @semio-tech/framework-renderer-react failed

Failed tasks:

- @semio-tech/framework-renderer-react:test-quick

Hint: run the command with --verbose for more details.


```

