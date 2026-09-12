# Preview-Eval Cancellation, End To End (2026-09-12)

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "preview eval cancellation". Repo MCP was down for
the whole run (`repo (-32602): invalid initialize params`); ticket bookkeeping is on disk and no
ticket was opened/closed/reopened. Outputs under `🗑️generated/cancellation/`.

Supersedes `📓️extension-invoke-door-2026-09-12.md` §9 item 2 ("a parked request has no host-side
cancel route yet") and extends `📓️tessellation-jobs-2026-09-09.md` §1.3 (the three retirements).

---

## 1. `cancelPreviewEval` end to end, as it was

### 1.1 The declared hops

| # | hop | file:line | what it actually did |
|---|---|---|---|
| 1 | command declared | `✏️editor/🦀️.rs:92` (`app_commands!`) | `"cancelPreviewEval" as "cancel-preview-eval"` |
| 2 | retained tool id | `✏️editor/🦀️.rs:295` | listed in `GENERATION3D_RETAINED_TOOL_IDS` |
| 3 | publication contract | `✏️editor/🦀️.rs:756` | `HostOnly` — no store lane |
| 4 | bounded proof | `✏️editor/🦀️.rs:800` | `bounded_first_step_tool_proofs!` |
| 5 | parsed | `✏️editor/🦀️.rs:1619` | `CancelPreviewEval {}` — **no window address at all** |
| 6 | catalogued | `✏️editor/🦀️.rs:1858` | `Cancel Preview Computation` / `Vorschauberechnung abbrechen`, `ActionKind::View` |
| 7 | interactive job | `✏️editor/🦀️.rs:1939` | `InteractiveJobClassification::Migrated` |
| 8 | guest handler | `✏️editor/🎮️commands/🛑️cancel-preview-eval/🦀️.rs:20` | `session.cancel_preview_evaluation()`, `Emit::default()` |
| 9 | session | `🌊️flow/🖥️host/🦀️.rs:3166` (pre-change) | empties `pending_tessellate_by_hash` / `tessellate_handle_by_hash` / `tessellate_chunks_by_hash`, stamps non-complete progress rows `Cancelled`, `tick_scheduled = false` |
| 10 | kernel | `🌊️flow/📐️brep-geometry/🦀️.rs:583` | `cancel_all_tessellations()` over the **process-global** `tessellation_jobs()` registry |
| 11 | status projection | `✏️editor/🦀️.rs:2299` (pre-change) | `cancellable: phase.is_cancellable()`, `cancelAction: "cancelPreviewEval"` |

### 1.2 Which window kinds declare it

**None.** `✏️editor/🦀️.rs:1965-1985` lists `window_kind_action_refs` for the four owning windows, and
`cancelPreviewEval` is not among them — so it falls into the deliberately-unowned bucket that
`build_definition` (`🔌️plugin/🦀️.rs:5334-5338`) copies onto *every* window. It was therefore
dispatchable everywhere and offered nowhere.

### 1.3 Was it in the preview's UI?

No. `🌐️World3dHost/🟦️.tsx` read exactly one field off `World3dScene.statusJson`:

```ts
return (JSON.parse(scene?.statusJson ?? "{}") as { computing?: boolean; label?: string }).computing === true;
```

and rendered a spinner plus `ui.common.loading`. `phase`, `phaseLabel`, `progress` and — critically —
`cancellable`/`cancelAction` were **published by the guest and read by nobody**. There was no button,
no keyboard route, no localized phase text. `📓️gap-inventory-2026-09-10.md:130` recorded this as
"exposed via `cancelAction` … consumed by status chrome"; no such chrome existed.

### 1.4 What it did NOT reach

**(a) The extension actor's own kernel jobs.** This is the defect that made the whole gesture
cosmetic. `FlowEvalSession::cancel_preview_evaluation` calls `crate::brep_geometry::cancel_all_tessellations()`
— the **generation3d guest's own linked copy** of `semio-framework-os-flow`. The retained
`TessellationJob`s live in `flow-extension-brep`'s registry, inside a *different wasm component
instance* with its own process globals (the host resolves them as two separate loaded programs:
`dispatchInvokeExtensionEffect`, `🏛️ShellHost/🟦️.tsx:1751`). From the app's side that registry is
always empty, so `cancel_all_tessellations()` retired nothing, every time.

The door onto them already existed and was **dead code**: `tessellateCancel`
(`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs:1965`), landed with the tessellation-jobs lane, per-handle
or whole-registry. Repo-wide grep found **one** occurrence — its own definition. Nothing has ever
invoked it.

**(b) The host-side in-flight request.** `driveInboundRequest` honours an `AbortSignal`
(`🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts:296`) and `PluginWasmHandle.invoke` forwards
`context.signal` into it (`🔌️PluginRuntime/🟦️.tsx:2632-2656`), but `runCapturedExtensionEffect`
passed `{ originInstanceId: instanceId }` only (`🏛️ShellHost/🟦️.tsx:1692`, pre-change). No caller in
the repo ever constructed a signal for an extension request.

> 🚪️ **Why a guest-emitted cancel cannot fix (b).** Every extension invocation of one app instance is
> serialized under `serializePerActor(\`${requester.pluginId}:${requester.instanceId}\`)`
> (`🏛️ShellHost/🟦️.tsx:1748`, pre-change). A `tessellateCancel` the guest emits therefore queues
> **behind** the very request it means to stop. Only the host, outside that queue, can abort a call
> already handed to the door. That is why the abort lives in the shell and is triggered by the
> surface, not by the guest.

**(c) The latch.** `cancel_preview_evaluation` set `tick_scheduled = false` and never touched
`window_tick_latches` (`🖥️host/🦀️.rs:2575`). A window whose latch still carried `in_flight > 0`
refused the next `arm_window_tick`; a window with **no** latch row read as "never ticked", which
`window_tick_owed` answers `true` for — so the host refresh poll re-armed the chain the user had just
stopped.

**(d) A `cancelled` status in the common case.** The projection stamped `Cancelled` only onto
*existing* progress rows. A cancel raised during the `evaluate` round trips — which is the whole slow
half of a boolean preview — has no row to stamp, so the surface published `phase: "idle"` and the
gesture vanished without a trace.

**(e) The chunk cursor.** `cancel_preview_evaluation` dropped `tessellate_chunks_by_hash` (the
half-received base64 body) but kept `PreviewTessellateProgress::next_chunk`. `next_tessellate_chunk`
reads that cursor (`🖥️host/🦀️.rs:3139`), so the **next** evaluation of the same handle asked the
kernel for chunk *n* and concatenated it onto an empty buffer — a silently truncated `pack` body.
That is precisely "a stale mesh from the cancelled run leaking in", and it was reachable from any
cancel of a multi-chunk transfer.

---

## 2. What now happens

### 2.1 Session (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`)

| line | change |
|---|---|
| `:2651` | new `preview_cancelled: bool` on `FlowEvalSessionState` — ephemeral local-only, never serialized |
| `:2999` | `begin_window_tick` clears it: a tick that BEGINS is work resuming |
| `:3188-3208` | `preview_tessellate_status` now reports `Transferring` for a kernel-complete job whose body is still crossing (the enum declared the phase; nothing ever set it) |
| `:3241` | `cancel_preview_evaluation(window_id)` — takes the addressed window, resets every latch to `FlowEvalWindowTickLatch::default()`, **creates** the addressed window's latch if absent, zeroes `next_chunk`/`chunks`, sets `preview_cancelled` |
| `:3276` | `preview_cancel_invocation_request_json` — the `tessellateCancel` body (whole-registry: `nodeHash` is a one-way digest of `(handle, tolerance)`, so the tolerance half of the extension's job key is not recoverable guest-side) |
| `:3283` | `preview_cancelled()` reader |
| `:3292` | `extensions_in_flight()` — session-wide sum over the per-window latches |
| `:3105` | `invalidate_for_flow_extension_registry` clears the banner too |

### 2.2 Chain (`🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs`)

- `:101` `CancelPreviewEval { window_id, window_kind_id }` — the payload moved here from the editor and
  now carries the window address, so the cancel's own round trip returns to the window it stopped.
- `:85` new `FlowTessellateCancelResolve { window_id, window_kind_id, output_json, ok }`.
- `:607` / `:615` `cancel_preview_eval` / `cancel_preview_eval_for(…, address)` — local retirement,
  then **one** `ExtensionInvocation(brep → "tessellateCancel" → "flowTessellateCancelResolve")`.
  An unaddressable kernel emits nothing.
- `:627` `resolve_tessellate_cancel` — folds the acknowledgement and arms **nothing**.

### 2.3 Why a dedicated response action

`ExtensionInvocation::response_action` must name an app-owned command. Reusing `flowTessellateResolve`
would have run a fold whose law is "one budgeted tessellate step" and whose `settle_window_extension`
+ `needs_another_round_trip` path can re-arm — restarting the chain the user just stopped. The new
command is registered at `✏️editor/🦀️.rs:93, :303, :838, :853, :1723, :1959, :2041` and
`🧊️generation3d/🦀️.rs:715`.

### 2.4 Status (`✏️editor/🦀️.rs:2395-2420`)

```rust
let phase = if address.is_err() { Faulted } else if session.preview_cancelled() { Cancelled } else { status.phase };
let work_in_flight = status.is_cancellable() || session.extensions_in_flight() > 0 || session.pending();
let cancellable = work_in_flight && !session.preview_cancelled() && address.is_ok();
```

`cancellable` is now true whenever *any* work a cancel would stop is outstanding, including the
`evaluate` round trips that admit no tessellation at all. Progress counters are untouched by the
cancel, so `phase: "cancelled"` ships with frozen `unitsDone/unitsTotal/facesDone/facesTotal` and the
en+de `phaseLabel` pair (`Cancelled` / `Abgebrochen`).

### 2.5 Host door (TypeScript)

| file:line | change |
|---|---|
| `🛠️ShellHelpers/🟦️.tsx:2765` | `beginCancellableExtensionRequest(actorKey)` → `{ signal, finish }`, registered in a per-requesting-actor set |
| `:2785` | `abortExtensionRequestsForActor(actorKey, reason)` → count aborted |
| `:2795` | `inFlightExtensionRequestCount(actorKey)` |
| `:2802` / `:2816` | `declareSurfaceCancelAction(actionId)` / `isDeclaredSurfaceCancelAction` — reference-counted, so two preview windows may offer the same verb |
| `🏛️ShellHost/🟦️.tsx:1681,1704` | `runCapturedExtensionEffect` takes the requester actor key, registers a controller and passes `{ originInstanceId, signal }` into `invoke` |
| `:1764` | `dispatchInvokeExtensionEffect` threads the same key it already serializes under |
| `:6019` | `onAction` aborts that actor's in-flight extension requests when the dispatched action is one a **mounted surface declared** — then forwards the gesture normally |

The shell never learns `cancelPreviewEval` from code: it learns it from the surface's own status
contract, which keeps the funnel domain-neutral.

### 2.6 Surface contract + UI

- `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts:322` `World3dComputeStatusV1` — the declared shape of
  `World3dScene.statusJson` (`computing`, `phase`, `phaseLabel{en,de}`, units/faces/inFlight/ratio,
  `cancellable`, `cancelAction`).
- `:375` `world3dComputeStatusV1(json)` — the one total parser. Malformed JSON or a hostile type
  degrades to a neutral status; `cancellable` is honoured only alongside a non-empty `cancelAction`
  (a button with nothing to dispatch is worse than no button).
- `🌐️World3dHost/🟦️.tsx:3425` `WorldComputeStatusPane` — spinner, producer-localized phase text,
  `unitsDone/unitsTotal (n%)`, and a real `<button data-slot="world-compute-cancel">` labelled from
  `ui.common.cancel` (en+de, no default language). A `<button>`, so it is tab-reachable and answers
  Enter/Space without extra wiring.
- `:4841` declares the action id to the shell for exactly as long as the status says `cancellable`.
- `:6410` mounts the pane in the existing overlay rail.

---

## 3. Tests — all run in the foreground, results verbatim

### 3.1 Rust law (fixture-driven)

Fixture: `🪆️subsets/✳️any/🧫️fixtures/🛑️preview-cancel.json` — 5 rows, each a sequence
(`admit`/`step`/`chunk`/`parkExtension`/`cancel`/`lateAnswer`) with the expected status **before** and
**after**, the expected invocations, the chunk cursor and whether a later gesture resumes.

```
$ cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib cancel
running 4 tests
test standards::…::cancelled_and_stale_aba_initializers_retire_to_terminal_empty ... ok
test editor::generation3d::commands::flow_tessellate_cancel_resolve::tests::the_cancel_acknowledgement_arms_nothing ... ok
test editor::generation3d::commands::cancel_preview_eval::tests::the_cancel_gesture_obeys_its_fixture_end_to_end ... ok
test editor::generation3d::commands::cancel_preview_eval::tests::an_unaddressable_kernel_cancels_locally_and_emits_nothing ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 371 filtered out; finished in 0.05s
```

The law drives the real `FlowEvalSession` and the real status projection, and after every row asserts
no tick is armed, none is owed, the chunk cursor is 0, a later gesture arms exactly one, and the first
tick that begins retires the banner.

### 3.2 Whole crate

```
$ RUST_MIN_STACK=67108864 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib
test result: FAILED. 368 passed; 7 failed
```

Of the 7: **5 are the known red rows** already recorded in `📓️status.md:662`
(`add_generation_records_an_undoable_generation_operation`,
`generation_preview_is_one_app_transient_shared_by_two_generation_windows`,
`two_instances_converge_disjoint_widget_moves`, `undo_redo_round_trips_flow_graph_edits`,
`vcs_artifact_app_…_fail_closed` / `publication.contended`). The 6th,
`retained_route_dispositions_are_exact_and_exhaustive`, was **mine** (28 → 29 routes) and is fixed —
counts bumped at `✏️editor/🧪️tests/🔬️unit/🦀️.rs:296-299`. The 7th,
`a_later_set_contributions_re_arms_the_chain_the_gate_stopped`, is the known flaky
process-global-registry row and passes in isolation:

```
$ … --lib -- retained_route_dispositions_are_exact_and_exhaustive a_later_set_contributions_re_arms_the_chain_the_gate_stopped
test result: ok. 2 passed; 0 failed
```

> ⚠️ Without `RUST_MIN_STACK`, `a_second_refresh_arms_no_second_tick_for_the_same_preview_window`
> **overflows its stack and SIGABRTs the whole test binary** in a debug build. Pre-existing (it
> overflows in isolation too, and passes with a 64 MB stack), but it aborts the process, so every
> later test in the binary is silently unmeasured. Worth its own row.

### 3.3 TypeScript twin (third-party independent implementation)

`✏️editor/🎮️commands/🛑️cancel-preview-eval/🧪️tests/🔬️unit/contract.ts` re-implements the session model
from the fixture's own prose (pending table, ledger, latch, banner) and replays the identical rows,
then feeds each row's published status through the **real** `world3dComputeStatusV1` parser to assert
what the surface would render. Runner: `🔍️preview-cancel-contract.ts`.

```
$ bun ".🧬semio/…/PROCEDURAL-3D-END-TO-END/🔍️preview-cancel-contract.ts"
generation3d preview-cancel rows=5 capability=tessellateCancel response=flowTessellateCancelResolve action=cancelPreviewEval
OK generation3d preview-cancel TS twin
```

### 3.4 React engine vitest — the door's signal

Two new cases in `🧑‍🎨engine/🧪️tests/📥️inbound-request/🟦️.ts`, driven through the **real**
`runInvokeExtensionEffect` over a real `driveInboundRequest` door onto a guest that parks forever
(one real macrotask per turn — a synchronous `more-work` loop burns the whole budget inside one
microtask flush and no cancellation could ever be observed between turns):

```
$ SEMIO_TEST_LEVEL=full bunx vitest run --config vitest.config.ts "🧪️tests/📥️inbound-request"
 Test Files  1 passed (1)
      Tests  18 passed (18)
```

The cancel case asserts, in order: the shell hands `invoke` an `AbortSignal` (`toBeInstanceOf(AbortSignal)`,
not aborted), the requester's in-flight count is 1, `abortExtensionRequestsForActor` returns 1, the
signal becomes aborted, the completion carries a **fault** decoding to `extension.request-cancelled`,
and the count returns to 0. The second case pins the reference-counted surface declaration.

`🧪️tests/🖱️world3d-interaction` (the real jsdom mount of `World3dHost`): **14 passed (14)**.

### 3.5 Typecheck

`bunx tsc -p tsconfig.json --noEmit` on the react engine target: zero diagnostics on any line I
touched. (The repo-wide config reports many pre-existing errors elsewhere, including four in
`World3dHost` about `brushPreviewJson`/`pickEnabled` that predate this lane.)

---

## 4. Runtime on 6018

`🐍️cancel-preview-probe.mjs` (ticket root) boots the playground, samples every **distinct** status
frame at 50 ms — the affordance is a frame, not a steady state — and clicks the surface's own cancel
button the moment it appears. Outputs in `🗑️generated/cancellation/cancel-1` and `cancel-5`.

### 4.1 The headline measurement (`cancel-5`, `Sphere Cut With Torus`)

Four distinct frames in 91 s:

| t | phase | cancellable | computing | inFlight | pane rendered | cancel button |
|---|---|---|---|---|---|---|
| 0.6 s | — | — | — | — | no | no |
| 3.2 s | `faulted` | false | — | 0 | no | no |
| 23.8 s | `idle` | false | — | 0 | no | no |
| 39.5 s | `idle` | **false** | **true** | 0 | **yes** | **no** |

Status JSON at 90.9 s, mid-evaluation (the eval body shows `brep_prim3d_sphere_3` / `brep_prim3d_torus_4`
already resolved and the boolean still running):

```json
{"computing":true,"phase":"idle","phaseLabel":{"en":"Idle","de":"Bereit"},
 "progress":{"unitsDone":0,"unitsTotal":0,"facesDone":0,"facesTotal":0,"inFlight":0,"ratio":1.0},
 "cancellable":false,"cancelAction":"cancelPreviewEval",
 "debug":{"evalLen":507,"meshesLen":2,"instancesLen":2,…}}
```

Two things are proven here:

1. **The host-side UI change is live and correct.** The pane renders from 39.5 s
   (`{"phase":"idle","cancellable":false,"ratio":null,"text":"Idle"}`) — `World3dHost` is reading the
   full status contract off the served TS. It shows no button because the **served wasm** still uses
   the old ledger-only predicate and publishes `cancellable: false`.
2. **The old predicate is exactly as blind as §1.4(d) says.** Across the entire 51-second evaluation
   the guest never once said the work was cancellable. `cancellable` was `false` in every sampled
   frame. This is the hole `✏️editor/🦀️.rs:2413` closes, and it needs a **restage of the procedural
   plugin** — which this lane was forbidden to build, so the cancellable-during-evaluate behaviour
   stops at native proof (§3.1/§3.3).

### 4.2 The other finding: a single guest turn is uninterruptible at the door

`cancel-1` ran the same example and the shard watchdog killed it:

```
[DEBUG] shard 0 terminated by the host watchdog: the worker was silent for 16049 ms;
outstanding: turn flow-extension-brep#request started 16112 ms ago; turn procedural#1 started 16049 ms ago.
```

and every answered request in both runs reports `turns: 1`:

```
[DEBUG] extension request answered {pluginId: flow-extension-brep, capability: evaluate, req: 1, turns: 1, bytes: 123}
[DEBUG] extension request answered {pluginId: flow-extension-brep, capability: evaluate, req: 2, turns: 1, bytes: 123}
```

`driveInboundRequest` reads its signal **at turn boundaries only** — deliberately, so a turn already
handed to the worker is never half-abandoned. `brep.bool.cut` spends 16+ s inside ONE turn, so the
abort has nothing to land between. The abort therefore retires a *parked* (multi-turn) request and
the *remaining* requests of a chain; it cannot interrupt a single long synchronous kernel call. The
real fix for that is to make `evaluate` budgeted and resumable the way `tessellate` already is
(`📓️tessellation-jobs-2026-09-09.md` §1.2) — see §5.

### 4.3 Two runs lost to a peer's edit

`cancel-2`/`cancel-3`/`cancel-4` produced a blank page. The cause was not this lane:

```
pageerror SyntaxError: The requested module '/@fs/…/🎭️actor/🧵️shard-runtime/🟦️.ts'
does not provide an export named 'shardWorkerUrl'
```

`🧵️shard-runtime/🟦️.ts` (mtime 07:28, mid-session) exports it again now, and the boot recovered. The
server on 6018 answered every probe in < 30 ms throughout and was never wedged.

---

## 5. What is still owed

1. **Restage the procedural plugin.** The guest half (`cancellable` during `evaluate`,
   `phase: "cancelled"`, the `tessellateCancel` invocation, the new `flowTessellateCancelResolve`
   command, the chunk-cursor reset, the latch quiescence) is proven natively and by the TS twin but
   is not in the served wasm. The plugin descriptor (`🌀️procedural/🔣️.json`, `🛂️.descriptor.semio`)
   also needs regenerating for the new action id.
2. **Budgeted `evaluate`.** §4.2 is the remaining hard limit: one 16 s kernel call in one turn is
   uninterruptible and trips the 16 s shard watchdog. `tessellate` already steps with a unit budget;
   `evaluate` does not. Until it does, cancellation of the boolean examples can only take effect
   between round trips, and the watchdog beats the user to it.
3. **The viewer surface.** `👁️viewer` runs the same `🧵️preview-eval` chain but declares no cancel
   command of its own; only `✏️editor` does. Its preview publishes a status with no `cancelAction`,
   so the pane renders progress and no button — correct by the contract, but the viewer should
   probably own the verb too.
4. **The `Transferring` phase** is now reachable for the first time (`🖥️host/🦀️.rs:3204`). Nothing
   downstream special-cases it beyond the generic label; a surface that wants a distinct transfer
   indicator can now have one.
5. **`RUST_MIN_STACK`** — §3.2's stack overflow aborts the generation3d test binary in default debug
   runs, masking every test after it. Independent of this lane, but it makes the crate's gate
   unreliable.

---

## 6. Files

Created:
- `🪆️subsets/✳️any/🧫️fixtures/🛑️preview-cancel.json`
- `✏️editor/🎮️commands/🧯️flow-tessellate-cancel-resolve/🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/🛑️cancel-preview-eval/🧪️tests/🔬️unit/contract.ts`
- `<ticket>/🐍️cancel-preview-probe.mjs`, `<ticket>/🔍️preview-cancel-contract.ts`, this report

Changed:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`
- `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📥️inbound-request/🟦️.ts`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs`
- `✏️s/…/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs`
- `✏️s/…/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/…/✏️editor/🎮️commands/🛑️cancel-preview-eval/🦀️.rs` + its unit test
- `✏️s/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
