# Wave T — Typed-Operation Fault Routing (2026-09-09)

TypeScript-only wave. Routes typed-operation result pages to the host call that owns their operation so
an unrelated `readLocalInteraction` / `refreshUi` can never fail because a retained action's job faulted.

## The defect (as measured 2026-09-09 09:00, puzzle 3d React target)

`consumeTypedOperationEffects` threw `typed-operation failed: …` for ANY lane-11 (Fault) page it saw, and
`settlePluginTurn` / `settleAcknowledgedPluginTurns` drain every effect the actor emits during a
continuation loop regardless of which host call dispatched the operation. At boot the shell issues
`readLocalInteraction`, `refreshUi` and `handleAction(setActiveExample)` back to back; the retained
`setActiveExample` job's cancel fault page landed in `readLocalInteraction`'s settle loop
(`local interaction observation failed typed-operation failed: typed-operation cancelled before its next
publication unit`) and, through `runQueuedTurn`'s `turnOutcomes.push({ instanceId, error })` →
`AppChannelClient.pumpOutcomes` (`🧰️framework/🛍️products/💻️os/🟦️.ts:3168`), into the action's own promise too.

## Ownership signal — read off the wire, not inferred

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`:

- `TypedOperationResultPage::renderer_exchange_bytes` (line 12050) lays the token out as
  `receiver u32 | operation u64 | generation u64 | sequence u32 | attempt u8 | lane u8 | len u32`, so byte
  offset 20 of the page body is the operation's own result sequence.
- `MountedTypedCommandFullOperation` mounts every operation at `result_sequence: 0`
  (lines 20121, 20963) and bumps it once per **acknowledged** page in `acknowledge_result_page`
  (line 15114). A page carrying `sequence === 0` is therefore that operation's **first reveal**, and the
  host call observing it is the call that dispatched it.
- Checked and rejected as ownership carriers: `CommandIngressStatus::CommandComplete(cursor)`
  (`📡️spr/🧵️channel/🦀️.rs:1019`) carries only a page cursor, no operation id;
  `typed-operation-pending-output` is a TS-side synthesized tag (`🟦️.tsx`), not a host marker.
- `OperationId` is a process-monotonic `AtomicU64` (`🧰️framework/🔨️modules/⏱️trace/🦀️.rs:699-705`); the
  implementation deliberately does **not** rely on that ordering — only on `sequence === 0`.

## Changes

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`

- `:437` `typedOperationResult` return type gains `sequence: number`; `:453` decodes it
  (`view.getUint32(20, true)`), the exact offset `renderer_exchange_bytes` writes.
- `:463-475` new fault codes / capacity: `TYPED_OPERATION_UNATTRIBUTED_FAULT =
  "interactive-job.unattributed-result-fault"`, `TYPED_OPERATION_PARK_EVICTION_FAULT =
  "interactive-job.parked-result-capacity"`, `TYPED_OPERATION_PARK_CAPACITY = 32`. Both are real fault
  codes, `[DEBUG]`-free.
- `:477-480` `reportTypedOperationFault` — the shell fault path (`console.error(\`${code}: ${message}\`)`)
  for a page no host call may be rejected with.
- `:482-491` `class TypedOperationCall` — one host call's ownership scope (`observe` / `fault` /
  `takeFault` / `close`).
- `:493-548` `class TypedOperationRouter` — per-instance owner ledger:
  - `observe(call, operation, sequence)` claims an unclaimed operation for a **live** call only when
    `sequence === 0`; returns whether `call` owns it.
  - `fault(call, operation, sequence, message)` returns `true` (⇒ the observer throws) only when the
    observer owns the operation. Otherwise: a **live** foreign owner gets the fault parked; no owner (or a
    closed one) sends the fault to the shell fault path once, under
    `interactive-job.unattributed-result-fault`.
  - Parking is bounded at `TYPED_OPERATION_PARK_CAPACITY`; on overflow the **oldest** entry is evicted and
    surfaced under `interactive-job.parked-result-capacity` — never silently dropped.
  - `close(call)` drops the call from `live`, releases the operations it owned, and surfaces any parked
    fault that outlived its owner.
- `:550-570` `typedOperationRoutersByActor` (one router per instance, keyed by actor id — 1:1, the same
  keying `retainedWindowByActor` uses) and `withTypedOperationCall(actorId, label, body)`, which opens the
  scope, runs the body, and rejects with **this call's** parked fault (`typed-operation failed: …`) only.
- `:571` `consumeTypedOperationEffects(effects, call?)`; `:590-595` lane 11 routes through
  `call.fault(...)` instead of throwing unconditionally, and every non-fault page calls `call?.observe(...)`
  so claims are established from the first reveal. Lanes 7/9/10 and the
  `TYPED_OPERATION_TERMINAL_SEEN` / `PENDING_OUTPUT` / terminal-output rules are untouched.
- `:1101` `teardownPluginActor` drops the actor's router (reached from `releaseInstanceMaps`).
- `:1171` `settlePluginTurn(..., call?)` and `:1213` `settleAcknowledgedPluginTurns(..., call?)` thread the
  scope into both `consumeTypedOperationEffects` sites (`:1207`, `:1225`). Omitting the scope keeps the
  previous "owns every page it observes" behaviour, which is what the existing in-source suite drives.
- Host-call sites now each open exactly one scope, opened **before** the `serializeCommandIngressForActor`
  gate so a queued call's scope is live while a peer call drains:
  - `:1633` `runQueuedTurn` → `command#<instance>` (`:1669` passes it to `settleAcknowledgedPluginTurns`).
  - `:1720-1725` `createApp`'s open + receipt-ack settles → one `open#<instance>` scope.
  - `:1806` `refreshUi` → `refresh-ui#<instance>` (`:1817` passes it).
  - `:1840` `captureExtensionCompletion.complete` → `extension-completion#<instance>`.
  - `:1880` `bindDocumentPort`'s `settle` → `document-port#<instance>`.
- Every page is still acknowledged exactly once: acknowledgement is produced by
  `typedOperationAcknowledgements(result)` inside `settlePluginTurn`'s `acknowledge`, which is independent
  of routing — a parked or unattributed fault is ACKed exactly like an owned one (the guest blocks on it).
- `PLUGIN_UI_CONTINUATION_LIMIT` budget, the `[DEBUG] thunk start/done/failed` (`:896/899/901`),
  `[DEBUG] command ingress …` (`:1664/1666`) and `[DEBUG] settle …` (`:1189`) traces are all left in place.

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧫️fixtures/📬️typed-operation-routing.json` (new)

Language-agnostic routing fixture: receiver, owned/foreign operation ids, first/fault sequences, fault
text, park capacity, overflow count, and both fault codes.

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🔣️.json`

`:1945-2000` new `$defs.PluginRuntimeTypedOperationRoutingV1` (const-pinned schema/codes, typed
operations, bounds) — the fixture is validated against it with Ajv, the third-party oracle the sibling
`PluginRuntimeTypedOperationSettlementV1` test already uses.

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`

- `:2` registry destructuring extended with `TYPED_OPERATION_PARK_CAPACITY`,
  `TYPED_OPERATION_PARK_EVICTION_FAULT`, `TYPED_OPERATION_UNATTRIBUTED_FAULT`, `TypedOperationCall`,
  `TypedOperationRouter`, `withTypedOperationCall` (mirrored at the runtime file's `registerTests1` call).
- `:1628` *"routes a foreign typed-operation fault page to its own call and never to the observing call"* —
  Ajv-validates the fixture and pins the runtime constants to it; opens an `owner` scope that claims
  operation B from a `sequence === 0` page; nested inside it an `observer` scope runs the **real**
  `settlePluginTurn` over a lane-11 page for B and **resolves** (`settled.effects === []`, observer not
  rejected); the exact ACK envelope reaches the fake shard client
  (`toEqual(typedOperationAcknowledgements(faulted))`); the `owner` call then rejects with B's fault text.
- `:1668` *"bounds parked typed-operation faults at a fixed capacity and surfaces the evicted oldest"* —
  `capacity + overflow` foreign faults parked against one owner: exactly `overflow` evictions reported
  under `interactive-job.parked-result-capacity`, the owner drains exactly `capacity` faults starting at
  the first non-evicted one, and a fault for an operation nobody ever claimed is reported once under
  `interactive-job.unattributed-result-fault`.

## Commands run + tails

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react
SEMIO_TEST_LEVEL=long bun x vitest run "PluginRuntime"
```

```
 RUN  v4.1.10 …/📦️packages/🟦️typescript/🎯️targets/⚛️react
 Test Files  1 passed (1)
      Tests  76 passed (76)
   Start at  09:25:46
   Duration  3.26s (transform 1.34s, setup 0ms, import 1.41s, tests 771ms, environment 763ms)
```

(74 before this wave; +2 new tests.) Both new tests confirmed to actually execute:

```
SEMIO_TEST_LEVEL=long bun x vitest run "PluginRuntime" -t "typed-operation" --reporter=verbose
 Test Files  1 passed (1)
      Tests  2 passed | 74 skipped (76)
```

```
bun x tsc --noEmit -p tsconfig.json     # …/🎯️targets/⚛️react/tsconfig.json
```

The project's `include` pulls the whole repo in, so it reports a large pre-existing error set. Filtered to
the files this wave touched:

- `🧱️elements/🔌️PluginRuntime/🟦️.tsx` — 1 error, pre-existing and untouched:
  `(2700,3365): error TS2339: Property 'dir' does not exist on type 'ImportMeta'.` (the `registerTests1`
  call's `import.meta.dir`).
- `🧪️tests/🔌️plugin-runtime/🟦️.tsx` — 137 errors, all pre-existing (`WireVariant`, `ShardEventEnvelope`,
  `WireTurnResult`, … are never imported into this in-source suite). Count before the wave: 137. The two
  new tests add **zero** new errors (a first draft added 2 of the same pre-existing class; the two type
  annotations were dropped so the count is unchanged).
- `🧬️schema/🔣️.json` — JSON, not type-checked; validated by the Ajv compile in the new test.

## Not verified

- **No browser run.** The measured boot defect was not re-measured in the puzzle 3d React target — the
  brief forbids starting servers and rebuilding wasm this wave. The fix is proven only at the in-source
  suite level (including the real `settlePluginTurn` drain path).
- **No wasm/Rust build.** Only `🔌️plugin/🦀️.rs` *reads* informed the wire layout; no Rust was compiled.
- **Late-fault production path.** In the measured trace the owning `handleAction` call had already
  returned when the cancel page arrived, so production will take the *unattributed* branch
  (`console.error("interactive-job.unattributed-result-fault: …")`) rather than the *parked* branch. Both
  branches are covered by tests, but which one the live boot takes has not been observed.
- **Nothing else in the repo type-checks clean** under this tsconfig; only the delta for the touched files
  was measured.
