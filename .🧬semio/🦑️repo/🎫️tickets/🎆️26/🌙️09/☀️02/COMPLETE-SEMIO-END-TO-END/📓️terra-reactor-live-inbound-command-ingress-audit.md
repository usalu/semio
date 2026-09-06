# Native Reactor Live-Inbound and Command-Ingress Audit

## Current boundary

`GuestLifecycleCell` correctly distinguishes `Opening`, `Captured`, `Live`, `Accepted`,
`Closing`, `Retired`, and `Released` in
[`🚪️lifetime/🦀️.rs:47-169`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs:47).
`native_close_key` already means *the exact currently acknowledged native allocation is Live*
([`🔄️turn/🦀️.rs:18-21`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:18)).
It is used for the final render reservation, but not for several earlier app-mutating paths.

The consequence is not merely a stale UI update: `handle_intent_frame`, command ingress, and a
task resume can reach typed Store mutation while an instance owns a Captured or Closing receipt.
The current close participant also has no terminal observation of the two retained
`COMMAND_INGRESS` owners.

## Inbound event census

| Inbound source | Current reducer path | Live/key requirement |
| --- | --- | --- |
| `UiIntent` | decodes and queues at [`🔄️turn/🦀️.rs:158-170`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:158), then calls `plugin_dispatch_intents` at [`:536-565`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:536). | **P0.** Require the exact current `NativeCloseKey` before queueing and again immediately before dispatch. Require the event instance and decoded `intent.surface` prefix to name that same key. Otherwise an `InstanceOpen + UiIntent` turn dispatches during Captured. |
| Command page argument | initial admission checks only `cell.is_live()` at [`:384-403`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:384). | Capture the returned `NativeCloseKey` in the retained owner, not only `cursor.instance`; every later page/dispatch must compare that exact key. |
| Retained command/presence owner | two unkeyed static slots at [`:1-14`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:1) are resumed at [`:263-381`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:263). `ReservedPresence`, `PendingPresencePage`, and `Presence` call app methods before any live check. | **P0.** Exact-key ownership and a durable close step are required; the one-turn `close_instances` condition cannot protect later `Accepted`/`Closing` turns or numeric reuse. |
| `Completed` / `HttpChunk` | resolve or append a request at [`:188-201`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:188). | No immediate app call, but they can feed a resume in the same close turn. Capture the key in request/task ownership or, at minimum, discard a no-longer-live resume before app invocation. |
| resolved task / restored task | `drain_task_resumes` directly calls `plugin_resume_task` at [`:809-839`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:809). | **P0.** A `Command` or `Emit` resume mutates the artifact. Each `PendingResume` must carry its captured key and the drain must terminally discard/fault it on an exact-key mismatch. Numeric instance comparison is insufficient after reopen. |
| typed-operation continuation | `plugin_continue_typed_operations` selects any runtime app with a runnable operation ([`🔌️plugin/🦀️.rs:31786-31796`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31786)), and the reducer routes it without a live check ([`🔄️turn/🦀️.rs:567-570`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:567)). | **P0.** Give continuation selection a live-key predicate or return the captured instance key and validate it before advancing/publishing. Closing output must enter its owned close path, never publish. |
| `Message::Shell` renderer ACK | only checks numeric `token.receiver == instance` before `plugin_acknowledge_typed_operation_result` ([`🔄️turn/🦀️.rs:232-237`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:232)). | **P0 for reuse.** The 25-byte `TypedOperationResultToken` contains no lifecycle (`🔌️plugin/🦀️.rs:13191-13197`). Bind an emitted renderer page to a fixed exact-lifetime ACK lease and require it on input; a current-live lookup alone cannot distinguish an old shell ACK after same-id reopen. |
| `SurfaceVisible` | dirty surface, then `plugin_render` is guarded, but `plugin_take_presence` is still called after failed/non-live render ([`🔄️turn/🦀️.rs:589-631`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:589)). | Guard the dirty record with its `NativeCloseKey`, and require that same key for both render and presence drain. `PatchTracker::defer` currently retains surface text without a key ([`🩹️patches/🦀️.rs:330-343`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs:330)); make deferred work key-bound to prevent old visible work rendering a reused instance. |
| `PatchAck` / `PatchRejected` | both carry an `ActorUiPatchReceipt` in kernel, yet the reducer discards it and matches only surface/revision ([`🔄️turn/🦀️.rs:179-186`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:179)). | **P0.** Verify receipt lifetime equals the current exact Live cell and sequence equals the issued patch receipt before changing `PENDING_PATCHES` or `PATCHES`; a stale rejection currently can alter a newer same-text surface. |
| `JobProgress` / `JobCompleted` | uses `JOB_RENDER_BINDINGS`; close activation explicitly closes those bindings ([`🚪️lifetime/🦀️.rs:225`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs:225)). | No direct app call once the binding closes. Keep its existing key-bound close path; add a regression so a progress/completion after close cannot enqueue a render. |
| `Timer` | removes a raw global timer id; production wake is otherwise absent ([`🔄️turn/🦀️.rs:240-246`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:240)). | No current production app mutation. It remains a stale-id integrity gap if timer IDs can be reused; defer lifecycle wire expansion unless Timer starts dispatching an app callback. |
| `SurfaceHidden`, `SurfaceResized`, `Wake`, `Request`, activation/capability/quota events, and non-Shell `Message` | ignored in this reducer. | No live guard needed until a handler is added. |

## Minimal command-ingress terminal design

Do not make a second close protocol. Extend the existing `ReactorCloseState` participant because
`NativeLifetimeOwner::terminal_is_empty` already requires its one exact
`reactor_close_complete(key)` receipt ([`🚪️lifetime/🦀️.rs:230-236`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs:230)).

1. Replace the enum payload in each of the two `COMMAND_INGRESS` slots with
   `CommandIngressOwner { key: NativeCloseKey, state: CommandIngressState }`. Capture `key` from
   `native_close_key` before the first page or presence reservation. Do not reconstruct it from a
   cursor instance.
2. Add a `command_ingress_complete` phase/cursor to `ReactorCloseState`, driven from
   `step_reactor_close` before `state.complete = true`. It scans at most one matching slot per
   opportunity. A different key remains untouched; an unkeyed owner is an invariant fault, not
   permission to drop it.
3. On exact close: convert `Generic` to its existing `PluginCommandIngress::Closing` state and
   bound-close `GenericAssembly`; cancel the reserved admission token before dropping a
   `ReservedPresence` page. A `Presence`/admitted presence page must be detached from the static
   slot and then retired by the already-owned application close, never resumed through
   `plugin_exchange` while Closing. The static owner becomes terminal only after its bounded page
   tails are empty.
4. While the owner is retained normally, every app call (`plugin_exchange`,
   `plugin_admit_reserved_presence`, `plugin_push_reserved_presence_page`) first compares
   `native_close_key(runtime, cursor.instance)` to its captured key. Mismatch routes only its
   closing step; it must not call the app or emit an app-produced patch/effect.
5. `ReactorCloseState` may report complete only after its request/resume/task/timer/metadata work
   **and** both matching ingress slots are terminal. That makes the current lifecycle's final
   `Retired` receipt a real proof that neither retained command owner can later address the
   released or reused app.

This is smaller than giving `NativeLifetimeOwner` a fourth independent participant and preserves
the existing all-or-nothing close ordering. It also fixes the present mismatch: the aggregate
admission corpus names `COMMAND_INGRESS` as a required descendant, while native
`terminal_is_empty` currently has no such check.

## Native laws using the real bundle factory

Place these beside `plugin_builder_contract_tests`' existing `TestRuntimeApps` /
`__semio_plugin_bundle` factory in `🔌️plugin/🦀️.rs`; call the ungated production
`reactor::poll_kernel` directly and complete the Captured/Accepted/Retired ACK tail.

1. `InstanceOpen + UiIntent` in one turn yields only Captured and no Store mutation/Emit; the
   same intent after the exact Captured ACK changes the test snapshot once. An intent whose
   surface prefix names another instance faults without changing either app.
2. A live command that leaves a two-page `GenericAssembly`, followed by close, requires bounded
   ingress close steps. `Retired` is withheld until its fixed page tail is empty; after a complete
   close/ACK and same numeric reopen, the old pages never call the new app.
3. A reserved and an admitted presence ingress each close without calling the Closing app; their
   cancel/page owners empty before `Retired`, while an unrelated second ingress slot remains
   intact.
4. Queue a real command/emit task resume, then issue close in the same turn. Its app mutation,
   frames, render, and effects are absent; it is terminally discarded/faulted and cannot run after
   reopening the same numeric instance.
5. A runnable typed operation is suppressed/retired after close rather than advanced by
   `plugin_continue_typed_operations`.
6. A dirty surface for Captured/Closing never calls `plugin_render` or `plugin_take_presence`; a
   deferred old-key surface remains inert after same-id reopen.
7. A stale PatchAck and PatchRejected carrying the old `ActorUiPatchReceipt` cannot acknowledge or
   retire a current patch. The exact current receipt succeeds.
8. A renderer ACK from the old lifetime is refused after numeric reuse; its token/lease cannot
   advance a current typed operation. A current exact ACK advances once.

No build or runtime test was run for this audit.
