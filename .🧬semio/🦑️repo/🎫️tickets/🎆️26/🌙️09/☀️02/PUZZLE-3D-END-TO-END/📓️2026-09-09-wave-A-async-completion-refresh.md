# Wave W-A — Async typed-operation completion → shell refresh (2026-09-09)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Repo MCP down for the whole wave; bookkeeping on disk.

## 0. State the wave was picked up in

The previous W-A agent was killed at ~17:50. `git diff HEAD --` over the three files the
hand-over named showed **none of its edits survived**:

- `🧰️framework/🛍️products/💻️os/🟦️.ts` — the +87 uncommitted lines are a **peer's**
  `directory-administration-*` worker-wire work (`parseDirectoryAdministrationWorkerRequestV1`,
  `SpaceArtifactCreationCatalogRefreshRequiredV1`, `workerWireCatalogGenerationIdV1`), not W-A's.
- `🔌️PluginRuntime/🟦️.tsx`, `🏛️ShellHost/🟦️.tsx` — clean against HEAD.
- No `OperationCompleted` / `subscribeOperationCompletions` / `onOperationCompleted` symbol
  existed anywhere in the tree.

W-A was therefore re-implemented from scratch. Peers' uncommitted work was left untouched.

## 1. The defect, re-derived

`setActiveExample` → `dispatch_typed_command_inner` mounts a retained typed operation and returns
a "started" `InvocationResult` on the FIRST reactor turn. The operation then runs to its terminal
result over hundreds of continuation turns. That terminal publication reached the host as

```rust
AppFrame::Invocation { in_reply_to: 0, ui_scope, .. }   // 🔌️plugin/🦀️.rs advance_typed_operation_output
```

`AppChannelClient.pumpOutcomes` (`💻️os/🟦️.ts`) resolves a frame only when its `in_reply_to`
matches a pending waiter, and seq 0 matches none — so the completion was **silently dropped**.
Nothing else carried the operation's outcome: outliner/inspection kept the old document and the
History panel listed no edit.

Second, independent defect: `HistoryPatch` crossed the wasm boundary through `decodePackValue`
without `packValueToExactJson`, so `entry.seq`/`patch.cursor` arrived as `{kind, value: bigint}`
pack integer carriers and the History rows rendered
`framework.history.entry.[object Object]` (`🏛️ShellHost/🟦️.tsx:7544`).

## 2. Changes (file:line, current tree)

### 2.1 Protocol — a first-class, correlated completion frame

New `AppFrame` variant, tag **25**, unsolicited and correlated by operation id (never `in_reply_to`).

| file | line | change |
|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs` | 1935 | `AppFrame::OperationCompleted { operation: u64, revision: u64, ui_scope: Vec<u8>, history_patch: Vec<u8> }` + docstring |
| same | 2672 | `encode_app_frame` arm (tag 25, varint/varint/bytes/bytes) |
| same | 2760 | `decode_app_frame` arm |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs` | 384 | `frame_in_reply_to`'s **exhaustive** match gets `AppFrame::OperationCompleted { .. } => None` (that match deliberately has no wildcard) |

TS twin:

| file | line | change |
|---|---|---|
| `🧰️framework/🛍️products/💻️os/🟦️.ts` | 2379 | `AppFrameValue` variant |
| same | 2381-2389 | `export type OperationCompletionV1` (decoded, exact-JSON-projected shape) |
| same | 2467 | `APP_FRAME_TAGS.OperationCompleted: 25` |
| same | 2912-2917 | `encodeAppFrame` branch |
| same | 3044-3050 | `decodeAppFrame` case |

Shared cross-language golden vector (new file):
`🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/🏁️app-frame-operation-completed.json`
→ `"OperationCompleted": "19070501010102"` for
`{ operation: 7, revision: 5, ui_scope: [1], history_patch: [2] }`.
Read by BOTH the Rust unit test and the TS suite.

### 2.2 Rust host — publishing the completion exactly once

| file | line | change |
|---|---|---|
| `🔌️plugin/🦀️.rs` | 12681-12697 | `pub struct TypedOperationCompletion { operation, revision, ui_scope, history_patch }` + private `TypedOperationCompletionWitness { operation, ui_scope }` |
| same | 11231-11232 | `PluginApp::take_typed_operation_completion(&mut self) -> Result<Option<TypedOperationCompletion>, Fault>` |
| same | 17658 / 18341 | `typed_completion_outbox: ArtifactFixedQueue<TypedOperationCompletionWitness>` field + construction at `TYPED_OPERATION_HOST_OUTBOX_SLOTS` (64) |
| same | 22214 | the witness is queued at the exact instant the `TypedOperationResultLane::Terminal` page is minted, capturing `emit.ui_scope.clone()`; a saturated receiver is a `plugin_sdk_fault`, never a silent drop |
| same | 24168-24173 | impl: pops the witness, resolves `history_patch(false)` **only when `history_dirty_sequences` is non-empty**, and stamps `store.content_revision_now()[..8]` as `revision` |
| same | 22873 / 22927 | bounded `close_step` unit + `retained_fields_terminal_is_empty` witness for the new queue |
| same | 24027 / 24037 | `has_pending_typed_operations` / `has_runnable_typed_operations` count the queue, so the actor stays runnable until the completion is published |
| same | 30617-30624 | `advance_typed_operation_output` emits `AppFrame::OperationCompleted` **before** the legacy seq-0 `Invocation` UI-scope frame |

Notes:
- the seq-0 `AppFrame::Invocation` UI-scope frame is intentionally left in place — it is a
  separate, already-existing carrier (`take_typed_operation_ui_scope`) and this wave does not own
  its removal; the completion is additive, not a compat layer.
- `history_patch(false)` drains `history_dirty_sequences`, which is exactly the delta that used
  to be stranded: `finish_recorded` never fired for a mounted operation because the invocation had
  already returned.

### 2.3 Host routing — `AppChannelClient` → handle → `ShellHost`

| file | line | change |
|---|---|---|
| `💻️os/🟦️.ts` | 3235 | `completionListeners: Set<(c: OperationCompletionV1) => void>` |
| same | 3298 | `pumpOutcomes` pulls `OperationCompleted` out of `ordinary` **before** correlation (same treatment as `LocalInteractionQuery`) — it must never be handed to a waiter, since `appChannelFrameBelongsTo` returns `true` for any frame without an `in_reply_to` |
| same | 3318-3320 | `onOperationCompleted(listener): () => void` |
| same | 3325-3339 | `publishOperationCompletion` — decodes `ui_scope`/`history_patch` through `decodePackWire` (i.e. `packValueToExactJson ∘ decodePackValue`), fans out, and never lets one throwing subscriber starve its siblings or the pump |
| same | 3345 | `dispose()` clears the subscriber set |

| file | line | change |
|---|---|---|
| `🔌️PluginRuntime/🟦️.tsx` | 850-857 | `export type PluginOperationCompletion { instanceId, operation, revision, uiScope, historyPatch, requestedEffects }` |
| same | 211 | `PluginWasmHandle.subscribeOperationCompletions(instanceId, listener) => () => void` |
| same | 2355-2371 | `adaptPluginHandle` implementation — wraps `AppChannelClient.onOperationCompleted`, drains `pendingCompletionEffects` for that instance and projects them through `wireEffectToFriendly` |
| same | 864 | new module-level `pendingCompletionEffects: Map<number, WireVariant[]>` |
| same | 1650 | released with the instance in `releaseInstanceMaps` |
| `📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` | 854-855 | re-exports `PluginOperationCompletion` |
| `🏛️ShellHost/🟦️.tsx` | 4776-4797 | `useEffect` per session: subscribes, then `applyHistoryPatch(completion.historyPatch)` + `applyHostEffects(completion.requestedEffects, session, resolveUiDirtyScope(completion.uiScope), captureEffectOwner(session, null))`; guarded so a destroyed instance logs instead of throwing during commit |

**Why a second effect map** (`pendingCompletionEffects`, not `pendingTurnEffects`): a completion
frame can ride the very same `TurnOutcome` that resolves a command, and `publishOperationCompletion`
runs synchronously inside the pump — i.e. strictly BEFORE `performInvocation`'s `await` continuation
drains `pendingTurnEffects`. A shared map would let the completion subscriber steal an in-flight
invocation's own `requestedEffects`. Two owners, two carriers; `pendingTurnEffects` keeps its exact
existing contract.

### 2.4 Drain polling

| file | line | change |
|---|---|---|
| `🔌️PluginRuntime/🟦️.tsx` | 866-880 | `PLUGIN_OPERATION_EFFECT_CAPACITY = 4096`, `PLUGIN_OPERATION_DRAIN_BUDGET = 4096`, `PLUGIN_OPERATION_WAKE_MAX_MS = 1000` |
| same | 882-903 | `drainTypedOperationTurns(budget, live, settle, yieldTurn)` — the bounded poll loop, extracted to module level with injected `live`/`settle` so its stop conditions are assertable without a shard worker. Returns `{ polls, stopped: "idle" \| "closed" \| "budget", nextWake }`. |
| same | 1766-1801 | `drainTypedOperations(instanceId)` inside `loadPluginModule` — one `UserVisible` continuation settle per poll, submitted through the SAME `serializeCommandIngressForActor(actorId, …)` every other caller uses (so a real command always wins the actor), routing effects exactly like `runQueuedTurn` does, one `yieldPluginUiContinuation()` macrotask between polls, per-instance re-entrancy guard, `nextWake` re-arm clamped to 1 s |
| same | 1749 | `runQueuedTurn` arms the drain when its settle ends `more-work` |

`nextWake` has no settled host-side unit anywhere in this file (nothing else consumes it), so it is
honoured only as "wake again, no later than this" and clamped — documented at the constant.

### 2.5 History carrier bug

| file | line | change |
|---|---|---|
| `🔌️PluginRuntime/🟦️.tsx` | 2136 | `invocationFromFrames`: `decodePackValue(...)` → `decodePackWire(..., "$.historyPatch")` for `AppFrame::Invocation.history_patch` |
| same | 2240 | `readHistory`: same projection for `AppFrame::HistorySnapshot.history_patch` |
| `💻️os/🟦️.ts` | 3331-3332 | the completion frame's own `ui_scope`/`history_patch` are projected the same way |

`entry.seq` is now a `number`, so `framework.history.entry.${entry.seq}`
(`🏛️ShellHost/🟦️.tsx:7544`) renders `framework.history.entry.41`, and
`revertToCommand { entrySeq: entry.seq }` (`🏛️ShellHost/🟦️.tsx:7551`) round-trips a JSON number
rather than a carrier object. **The `revertToCommand` round-trip was verified only at the projection
level (a vitest assertion on the projected `seq`), not by dispatching the action in a browser.**

## 3. Tests added

Rust (`📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs`):
- `:390` `app_frame_operation_completed_round_trips` (incl. `operation: u64::MAX`, empty payloads)
- `:396` `app_frame_operation_completed_matches_shared_cross_language_json_vector`
- `:680` / `:755` the variant + its golden hex joins the one-entry-per-variant fixture corpus

Rust (`🔌️plugin/🧪️tests/⏳️completion/🦀️.rs`, driving a REAL mounted typed operation to Terminal):
- `test_restart_publish_and_close` now also drains `take_typed_operation_completion()` per turn,
  asserts the completion's `ui_scope` is `Full` and its operation id is non-zero, and asserts the
  count against the new fixture key `restartAuthority.operationCompletions: 1`
  (`🔌️plugin/🧫️fixtures/⏳️completion/🔣️.json`) — i.e. **exactly one** completion per terminal
  operation.

TypeScript (`💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts`, `AppChannelClient` describe):
- "delivers an unsolicited typed-operation completion to its subscriber exactly once, never to a
  pending command waiter" — the fake handle returns `OperationCompleted` **plus** the `Invocation`
  reply in one outcome; asserts the waiter gets 1 frame (`Invocation` only), the subscriber gets
  exactly 1 completion, `cursor`/`seq` project to `7`/`41`,
  `framework.history.entry.41`, and that the returned unsubscribe stops delivery.
- "matches the shared cross-language typed-operation completion fixture vector, byte-for-byte".

TypeScript (`🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`):
- `typed-operation completion drain`: stops at the first idle settle (3 polls); never exceeds its
  budget; stops the moment `live()` goes false without settling again; surfaces the last
  `nextWake` for re-arming.
- `typed-operation completion delivery`: through a real `adaptPluginHandle`, one completion reaches
  its subscriber once, carrying `pendingCompletionEffects` and leaving `pendingTurnEffects` intact.
- Both `fakeHandle` literals gained `subscribeOperationCompletions` (the field is required).

## 4. Commands run + tails

All Rust foreground, `RUSTC_WRAPPER=""`, `RUST_MIN_STACK=134217728`,
`CARGO_TARGET_DIR=…/scratchpad/target-p3d` (seeded marker appeared 18:04).

```
$ cargo check -p semio-framework-plugin
    Checking semio-framework-plugin v0.1.0 (…/🔌️plugin/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 1m 42s
```

```
$ cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
    Checking semio-s-artifact-puzzle-3d v0.1.0 (…/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 31.45s
```

```
$ cargo test -p semio-framework-os-kernel operation_completed
test os_spr::channel::tests::app_frame_operation_completed_round_trips ... ok
test os_spr::channel::tests::app_frame_operation_completed_matches_shared_cross_language_json_vector ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1070 filtered out

$ cargo test -p semio-framework-os-kernel fixture_corpus
test os_spr::channel::tests::app_frame_fixture_corpus_matches_golden_hex_and_round_trips ... ok
test os_spr::channel::tests::app_command_fixture_corpus_matches_golden_hex_and_round_trips ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 1068 filtered out

$ cargo test -p semio-framework-os-kernel app_frame
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 1043 filtered out
```

```
$ cargo check -p semio-framework-os-run
error[E0277]: the trait bound `store::Media: serde::Serialize` is not satisfied
    --> …/🏃️run/📦️packages/🦀️rust/../../🚀️bin.rs:50:49
error[E0277]: the trait bound `store::Media: serde::Deserialize<'de>` is not satisfied
    --> …/🏃️run/📦️packages/🦀️rust/../../🚀️bin.rs:43:9
error: could not compile `semio-framework-os-run` (bin "semio-framework-os-run") due to 2 previous errors
```
Both errors are pre-existing `store::Media` serde fallout in `🚀️bin.rs`, untouched by this wave.
The **lib** (which is where `frame_in_reply_to` lives) compiled clean — i.e. the new exhaustive-match
arm is correct; had it been missing this would have been an `E0004` in `🏃️run/🦀️.rs`.

```
$ SEMIO_TEST_LEVEL=long bun x vitest run "PluginRuntime"       # …/🎯️targets/⚛️react
 Test Files  1 passed (1)
      Tests  81 passed (81)          # baseline was 76; +5 new
```

```
$ SEMIO_TEST_LEVEL=long bun x vitest run                        # …/💻️os/📦️packages/🟦️typescript
 Test Files  5 passed (5)
      Tests  343 passed (343)        # baseline was 341; +2 new
```

```
$ SEMIO_TEST_LEVEL=long bun x vitest run                        # …/🎯️targets/⚛️react (whole target)
 FAIL  |@semio-tech/framework-renderer-react| ../../../../🧪️tests/🧩️package-integration/🟦️.ts
ReferenceError: self is not defined
 ❯ ../../../../🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/…plugin-bridge.ts:159:1
 Test Files  1 failed | 18 passed (19)
      Tests  712 passed (712)
```
The one failing suite is a pre-existing node-environment failure in the **wgpu** plugin bridge
(`self is not defined` at import time). No file in this wave is on that import chain.

```
$ bun ./📜️script.ts typecheck                                   # …/🎯️targets/⚛️react
1016 diagnostics, all pre-existing (top offenders: 📜️script.ts 190, 🧪️owned-locale-detector-retirement 152,
🧪️docklayoutstore 84, 🔬️interactivity-p1q-r4 59, 🧪️backbone-envelope-io 44 — implicit-any / untyped-call noise,
plus a peer's duplicate `DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES` in 💻️os/🟦️.ts).
$ grep -cE "🔌️PluginRuntime|🏛️ShellHost|🔌️plugin-runtime" tc-react.txt   → 0
$ grep -iE "OperationComplet|subscribeOperation|drainTypedOperation|pendingCompletionEffects|PLUGIN_OPERATION" tc-react.txt → (none)
```
Zero typecheck diagnostics in any file this wave edited, and none naming any symbol it introduced.

## 5. NOT verified

1. **`cargo test -p semio-framework-plugin` cannot run.** The lib-test build is blocked by a peer's
   in-progress, uncommitted (`M ` staged) 208-line addition to
   `🔌️plugin/🧪️tests/🧩️composition/🦀️.rs`. Measured twice during the wave: first 22 errors
   (E0405/E0422/E0425/E0433 — `super::ArtifactStoreInitializationAuthority`,
   `super::ActiveArtifactStoreReplacementState`, … not found), then 12 (E0616 —
   `window_transient_store`, `store_replacement_jobs`, `child_content_generation` private). **Every
   one of them is in that peer file**; none in `⏳️completion/🦀️.rs` or `🔌️plugin/🦀️.rs`.
   Because the second run got past name resolution and emitted 56 warnings, the crate WAS
   type-checked and the new `⏳️completion` test code produced no diagnostics — but it has never been
   **executed**. Re-run `cargo test -p semio-framework-plugin checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume -- --nocapture`
   once the peer's composition test compiles; the `[DEBUG] restart typed-operation completion …`
   line it prints also reveals whether that fixture's operation carries a history delta (the test
   only asserts the completion count, not `history_patch.is_some()`).
2. **No browser run.** The wave changed no wasm and rebuilt no plugin (per brief: no wasm rebuild,
   no servers). The end-to-end claim "selecting Nakagin now refreshes outliner/inspection/History"
   is **not** measured — only the unit-level carriers are.
3. **The drain poller was never observed firing.** Re-reading `settlePluginTurn`, a settle either
   drains to a non-`more-work` status or throws at `PLUGIN_UI_CONTINUATION_LIMIT`; it does not
   return `more-work`. So in the measured Nakagin path the operation already reached its terminal
   result *inside* the command's own settle, and the real fix is §2.1-§2.3 (the frame was produced
   but dropped), not §2.4. The drain is a correct, bounded safety net for a turn that ends
   `more-work` outside a settle — its stop conditions are unit-tested, its live arming is not.
4. `revertToCommand { entrySeq }` round-trip — asserted at the projection level only (see §2.5).
5. `AppFrame::OperationCompleted` is not exercised by the native `🌉️ProgramBridge/🎯️targets/🧊️wgpu`
   TS bridge; that bridge has its own `pendingTurnEffects` and was left alone (out of wave scope).

## 6. Untouched, as instructed

`⚛️reactor/🔄️turn/🦀️.rs` (its `route_app_frame` already has an `other =>` arm, so the new frame
flows to the shell with no edit), `🕹️interaction/**`, `🧵️job`, the coordinator's `[DEBUG]` traces,
and W-T's `TypedOperationRouter` (wiring only — `withTypedOperationCall("operation-drain#…")` opens
a scope through the existing API, nothing inside the router changed).
