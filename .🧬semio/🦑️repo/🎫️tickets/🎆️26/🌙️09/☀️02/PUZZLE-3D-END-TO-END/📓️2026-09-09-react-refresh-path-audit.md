# React Plugin Refresh Path Audit — puzzle3d Silent Stall

Read-only audit. No source edited, no build/dev server run.

## Ranked stall causes (most likely first)

### 1. HIGH confidence — per-actor `command-ingress` queue wedged by the boot-time `refreshUi` call, never releasing

`refreshUi` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1583-1614`) wraps its ENTIRE body in:

```
const result = await serializeCommandIngressForActor(actorId, async () => { ... settlePluginTurn(...) ... });
```

`serializeCommandIngressForActor` (line 801-803) is `serializePerActor("command-ingress:"+actorId, run)` (line 791-796), which enqueues onto `getThunkScheduler()` (line 770-784) — a strict FIFO `TurnScheduler` keyed per actor. Its own doc (line 786-791) is explicit: **`run` never starts for a given `actorId` before the previous `run` for that SAME id has settled**. The `run()` closure is the ONLY code path that reaches `submitTurn` → `slot.worker.postMessage` (confirmed real `Worker.postMessage` calls at `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1417,1426,2133` — not a MessageChannel/port bypass, so the coordinator's `Worker.prototype.postMessage` hook is trustworthy).

This exactly explains the observed **zero Worker messages** when the coordinator called `handle.refreshUi(1, request)` directly: that call was silently queued behind the SAME actor's (`puzzle#1`) boot-time `refreshUi` call, whose own promise never settled. Since `serializePerActor` is a plain FIFO with no timeout (only a 256-entry capacity backpressure error, which never fires here), a stuck first call means every later call for that instance queues forever with no error, no log, and no shard traffic — matching "no console fault is logged."

This also explains why `ShellHost`'s `uiRefreshCacheRef` stays empty and `Mode` gets `windows: []`: `applyUiRefreshResponseToCache` (`🛠️ShellHelpers/🟦️.tsx`, called from `🏛️ShellHost/🟦️.tsx:3944`) only runs after `const response = await program.refreshUi(...)` (line 3908) resolves — it never has.

**Minimal fix**: give `serializePerActor`/`serializeCommandIngressForActor` (`🔌️PluginRuntime/🟦️.tsx:791,801`) a bounded timeout per queued `run`, or at minimum surface a `[DEBUG]` log when a `command-ingress:<actorId>` slot has been held longer than e.g. the shard heartbeat window, so a wedged actor is diagnosable instead of silent. Longer term: the real defect is #2 below — fixing that removes the wedge at its source.

### 2. MEDIUM confidence — where the boot call itself stops making progress without further shard traffic

The boot call reached the shard once (the "turn with 5 surface-visible events" the coordinator observed) via `submitTurn(actorId, events, {lane:"UserVisible", activation})` (line 1598-1602) inside `settlePluginTurn` (line 1019-1058). After that reply, `settlePluginTurn` only submits ANOTHER turn (`submitPluginTurn`, line 1031) while `acknowledgements.length > 0 || hasWork()` (line 1030); `hasWork()` requires `wireTurnStatusTag(...) === "more-work"` (line 1028). Since no further shard turn was observed, either:
- the last turn's status was not `"more-work"` and `acceptUiPatches` (line 1204) produced no acknowledgements — in which case `settlePluginTurn` would return NORMALLY (not hang) — so this is the less likely branch; or
- (more likely) `acceptUiPatches`'s per-patch `intake.advance(uiGrant)` loop (lines 1219-1226) — a HOST-SIDE, wasm-resource-backed (`OwnedUiPatchIntake`) state machine that does **not** call the shard on every step (only line 1227's ack does) — never reaches a recognized `"ready"`/`"blocked"`/`"rejected"` outcome. All three are the only handled arms; there is no `else`/default branch, so an unrecognized/stuck native status silently keeps looping (yielding only every 8th step via `yieldUi`, line 1193) up to `PLUGIN_UI_CONTINUATION_LIMIT = 4096` (line 1002, reused as the budget at lines 1197, 1220, 1230) before it would eventually throw `plugin-ui.intake-budget-exhausted`. If each step's wasm/component-model marshaling is nontrivially expensive (plausible on a loaded machine), thousands of iterations can exceed the coordinator's ~40s patience well before that budget is spent, and the whole time the actor's `command-ingress` slot in cause #1 stays occupied.

This lines up with the version-skew premise: the guest was built at 05:36 from sources that PREDATE the peer's 06:05 window-context-ownership change. Per `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/window-context-ownership.md`:
> "The WIT record and kernel event now require a separate render body key and packed host `ViewModel`... Each plugin app instance now retains bounded ephemeral surface contexts."

An old guest not yet carrying these per-surface ephemeral contexts is a strong candidate for producing UI-patch/native-resource state that the host's NEW `acceptUiPatches`/`OwnedUiPatchIntake` bookkeeping never recognizes as terminal — i.e., a genuine host/guest ABI skew, not a logic bug in either side alone.

**Minimal fix**: add a bounded, explicit "unrecognized intake status" branch (an `else { throw ... }`, or at minimum a `[DEBUG]` log with the raw status) after the three handled `current.kind` arms at lines 1224/1232/1233/1252 so a version-skewed guest fails loudly and immediately instead of possibly free-wheeling toward the 4096 budget. Confirming the exact stuck arm requires a live guest+host pair with a debugger or added `[DEBUG]` tracing — not possible from static reading alone, and not attempted here per the read-only/no-build constraint.

### 3. RULED OUT — `readWindowConfigPacks` / window-config lane read

Grepped every production call site of `readWindowConfigPacks(` and `.readWindowConfigs(` across `🧰️framework/🛍️products/💻️os`. The ONLY call site is a test mock: `🧪️tests/🔌️plugin-runtime/🟦️.tsx:1089`. Neither `🏛️ShellHost/🟦️.tsx` nor `🛠️ShellHelpers/🟦️.tsx` nor `🐚️Shell/🟦️.tsx` ever calls it. The method exists on `PluginWasmHandle` (`🔌️PluginRuntime/🟦️.tsx:153-155`, wired at `1936-1937` to `AppChannelClient.readWindowConfigs()`/`loadWindowConfig()` in `🧰️framework/🛍️products/💻️os/🟦️.ts:3380-3405`) but nothing in the boot/refresh path invokes it today. It cannot be the cause of the currently observed hang.

### 4. RULED OUT for this hang, but a real separate gap — `@semio-tech/store-worker` dynamic import / `rustHost = null`

`🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:168-192` is the CROSS-TAB collaboration/backbone worker (`MessageChannel`/`attachLocalBrokerPort`, line 278-279, 671), a wholly separate `Worker` from the plugin-turn shard worker (`SHARD_WORKER_URL`, `📮️shard-client`). `refreshUi`'s only touch on a document/backbone port is `documentBindings.get(instanceId)?.port` used solely to route `send-message`/`backbone`-targeted effects (`🔌️PluginRuntime/🟦️.tsx:1406-1413`); a bare `surface-visible` refresh with no document binding never reaches that code, so this is off the critical path for the puzzle3d hang. Separately confirmed real: no `package.json`/vite alias anywhere in `🧰️framework/🛍️products/💻️os` resolves `@semio-tech/store-worker` outside `backbone-worker.ts` itself — the module specifier genuinely is opaque to Vite (hence its "cannot be analyzed" warning), and if the package isn't installed, `ensureRustHost()`'s `await import(...)` (line 183) rejects, leaving `rustHostPromise` (line 192) permanently rejected and `rustHost` stuck `null`. Worth a follow-up ticket for the backbone/collaboration feature, but not this bug.

## 1. `refreshUi` call chain, step by step

1. **Shell → handle**: `🏛️ShellHost/🟦️.tsx:3832` `refreshUi` (React `useCallback`) reads `loadedPluginsRef.current` (NOT the render-closure `loadedPlugins`, per the fix documented at lines 3840-3853) to find `program = entry.handle` for the session's `pluginId`.
2. **Request build**: `buildUiRefreshRequest(scope, windowInstances, panelTabLeaves, viewState, cache)` (line 3906) returns `undefined` if there is nothing dirty; when it returns a request, line 3908 calls `await program.refreshUi(nextSession.instanceId, request)`.
3. **Handle → PluginWasmHandle**: `program` is the object `loadPluginModule` (`🔌️PluginRuntime/🟦️.tsx:1168`) returned: `{ ...richHandle, refreshUi, captureExtensionCompletion, bindDocumentPort }` (line 1714) — `richHandle` comes from `adaptPluginHandle` (line 1575/1866), whose OWN `refreshUi` is an honest stub `async () => ({})` (line 1904, doc explains channel v12 retired the old `AppCommand::RefreshUi` wire message) — **immediately overridden** by the real `refreshUi` closure defined at line 1583.
4. **Real `refreshUi`** (line 1583-1614): `uiRefreshSurfaceEvents(instanceId, request)` (line 1584, defined at 1095-1114) builds one `{kind:"surface-visible", payload:{surface, bodyKey, viewState}}` `ShardEventEnvelope` per window/panel target THAT HAS a `bodyKey` (line 1097: `if (!target.bodyKey) return [];`, line 1105 same for panels). **If `events.length === 0` it returns `{}` immediately** (line 1585) — this is exactly the "request with no bodyKey resolved immediately with `{}`" the coordinator observed; no channel/queue is touched.
5. With events present: `requireActorId` → `shardClient.captureActorActivation(actorId)` → `serializeCommandIngressForActor(actorId, ...)` (line 1595, see cause #1 above) → `settlePluginTurn(actorId, await submitTurn(actorId, events, {lane:"UserVisible", activation}), "UserVisible", missingSurfaceIds, (turn) => acceptUiPatches(instanceId, turn), true, activation)` (line 1596-1608).
6. **`submitTurn`** (line 1388-1391) calls `registry.touch(actorId)` then `submitPluginTurn` (line 923-949), which enqueues onto `getPluginTurnScheduler()` (line 860-887, mailbox capacity `PLUGIN_TURN_MAILBOX_CAPACITY = 32`, line 854) — its `runTurn` calls `payload.activation.turn(events, budget, commandPage)` → `ShardClient`/`CapturedShardActivation.turn` (`📮️shard-client/🟦️.ts:1911-1913`) → `slot.worker.postMessage(...)` (`📮️shard-client/🟦️.ts:1417/1426/2133`) — this is the ONE real dispatch to the shard `Worker`.
7. **Guest**: shard worker's Rust host runs the actor's `render_with_request_context`/turn-processing, replies via `worker.onmessage` → `ShardClient.handleMessage` (`📮️shard-client/🟦️.ts:1268-1300`), which correlates by `message.requestId` against `this.pending` — **note**: a reply whose `requestId` isn't found (or slot mismatch) is silently dropped at line 1291 (`if (!entry || ...) return;`) with no log — a second, lower-probability silent-stall mode worth flagging.
8. **`settlePluginTurn`** (line 1019-1058) loops `acknowledge()`/`hasWork()` (see cause #2) until quiescent or `PLUGIN_UI_CONTINUATION_LIMIT` (4096) is hit, then returns the merged `uiPatches`/`effects`/`status`.
9. Back in `refreshUi` (line 1611-1613): `requireActorId` + `activation.assertActive()`, then `ownedUiRefreshResponse(instanceId, actorId, request, routeDocumentEffects(result.effects, documentPort))` (line 1288-1312) walks `uiSurfaceByInstance` and calls `projectOwnedUiSurface` (line 1256-1287) to build the `BuiltNode` tree per requested window/panel key from the RETAINED native surface — this is the "UI patches → UiDocumentStore intake" step; it reads already-accepted/retained surfaces, it does not itself talk to the shard.
10. **Shell**: `ShellHost/🟦️.tsx:3908` resolves; `resolveExternalSlots` runs on changed bodies (lines 3921-3942); `applyUiRefreshResponseToCache(cache, {...})` (line 3944, defined in `🛠️ShellHelpers/🟦️.tsx`) populates `uiRefreshCacheRef`; any `response.requestedEffects` are applied via `applyHostEffects` (line 3946).

Every await above is named; the ONLY one with no timeout, no shard round-trip, and full responsibility for the observed symptom is step 5's `serializeCommandIngressForActor` combined with step 8's `settlePluginTurn`/`acceptUiPatches` never quiescing.

## 3. Window-config / window-transient lanes — host requirement on boot/refresh

Confirmed by exhaustive grep (§3 above): the React host currently issues **zero** `readWindowConfigPacks`/`loadWindowConfigPack` calls in the boot or refresh path. The `AppChannelClient` methods exist (`🧰️framework/🛍️products/💻️os/🟦️.ts:3380-3405`) and the `PluginWasmHandle` interface carries them (`🔌️PluginRuntime/🟦️.tsx:153-155`), but nothing calls them today outside tests. Consequently: **an old guest without a `WindowConfigOwner` declaration is never asked to answer a window-config read by the current React path, and cannot be the cause of THIS observed hang.** If/when the host starts calling `readWindowConfigPacks` during boot (per the ticket's stated direction), the same per-actor FIFO queue (cause #1's `serializeCommandIngressForActor`, or the general `submitPluginTurn`/`enqueuePluginTurn` per-actor mailbox at `PluginTurnScheduler`, line 860-899) would apply: a guest that never replies would wedge the SAME actor queue this bug already demonstrates is unprotected by any timeout — worth building the timeout fix from cause #1 BEFORE wiring up that new call, not after.

## 4. Boot example order (Nakagin before Concrete Forest)

- **Origin of the order**: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs:37`:
  ```rust
  EXAMPLES.get_or_init(|| vec![crate::examples::puzzle3d::nakagin_capsule_tower::SOURCE.clone(), crate::examples::puzzle3d::concrete_forest::SOURCE.clone()]).as_slice()
  ```
  This literal `vec![...]` order — NOT the Rust module declaration order in the parent file (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🦀️.rs:1654-1667`, which actually declares `concrete_forest` before `nakagin_capsule_tower`) — is what becomes `manifest.examples`.
- **`resolveBootExampleId`** (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx:289-297`): keeps a still-valid active/default id, else `defaultsExampleId` if valid, else `exampleOptions[0]?.id` (line 296) — i.e., the FIRST entry in `manifest.examples`, which is Nakagin.
- **The actual default document** is concrete-forest: `default_fixture()` (`.../✏️editor/🦀️.rs:280-282`) returns `CONCRETE_FOREST_EXAMPLE_FIXTURE.clone()`, and `PUZZLE3D_EXAMPLE_CONCRETE_FOREST = "concrete-forest"` / `PUZZLE3D_EXAMPLE_NAKAGIN = "nakagin-capsule-tower"` (`.../✏️editor/🦀️.rs:63-64`).
- **Minimal fix**: swap the two elements in the `vec![...]` at `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs:37` so `concrete_forest` is listed first, matching `default_fixture()`. (Note puzzle2d has the identical Nakagin-first ordering at `.../◻️2d/.../🦀️.rs:38` and puzzle5d at `.../🖐️5d/.../🦀️.rs:38` — same bug pattern, not in scope here but worth a follow-up.)
- **`defaults.exampleId` knob**: YES, settable today, but only via an env var, not per-playground-variant TOML. `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts:39-41`:
  ```ts
  const defaults = { exampleId: import.meta.env.VITE_SEMIO_DEFAULT_EXAMPLE || undefined };
  ```
  flows into `resolveShellDefaults(brand, defaults)` (`🐚️Shell/🟦️.tsx:280-282`) → `resolveBootExampleId(..., defaultsExampleId)` (line 292/295). The puzzle plugin's `[[package.metadata.semio.playground]]` entries (`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:36-40` for the `puzzle3d` variant, port 6013 — matches the coordinator's dev server) carry only `variant`/`app`/`aliases`/`ports`/`engines`; there is no `exampleId`/default field in that schema. So today the only knob is `VITE_SEMIO_DEFAULT_EXAMPLE=concrete-forest` set on the dev process env (not set for this session, hence the fallback to `exampleOptions[0]`).

## 5. `[DEBUG] cooperative-maintenance` flood

- **Location**: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27587` inside `pump_runtime_live_cooperative_turn` (line 27568-27600), itself gated `#[cfg(target_arch = "wasm32")]` (line 27567) — it only exists in the wasm guest build, never native.
- **Owner / date**: `git blame` → commit `49e59877caf`, Ueli Saluz, 2026-09-07 06:49:48 +0200 (this file's most recent touch to those lines; the underlying mechanism was introduced 2026-08-27 per `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/component-wasm-cooperative-maintenance-liveness.md`, which documents this as an intentionally TEMPORARY diagnostic: *"Temporary `[DEBUG] cooperative-maintenance` output is limited to thirteen power-of-two observations through 4096 helper calls per cell."*).
- **Gating**: NOT feature/env-gated beyond the `wasm32` target cfg. It IS self-limiting: line 27580 `if turn <= 4096 && turn.is_power_of_two()` — this fires at turn = 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096 (13 times total per plugin instance cell, ever, since `maintenance_probe_turns` only increments), NOT an unbounded flood. If the coordinator is seeing many more than 13 lines per instance, it is because MULTIPLE instances (multiple cells) are each independently emitting their own bounded 13-line run — the per-cell counter (`cell.maintenance_probe_turns`, line 25882/25900) is per-instance, not global.
- **Recommendation**: safe to leave (it is finite and was designed as a temporary liveness diagnostic per the ticket doc above), but since it is unconditional `eprintln!` with no `cfg!(debug_assertions)` guard, it WILL also print in release wasm builds — if the coordinator wants it silenced now that the underlying liveness gap is believed fixed, delete lines 27580-27598 (the whole `if turn <= 4096 && ...` block) or add a build-time env gate; there is no existing flag to flip.

## Files referenced

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` (refreshUi, settlePluginTurn, acceptUiPatches, serializeCommandIngressForActor/serializePerActor, adaptPluginHandle, loadPluginModule)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` (React refreshUi callback, loadedPluginsRef)
- `🧰️framework/🛍️products/💻️os/🟦️.ts` (AppChannelClient, PluginWasmHandle's readWindowConfigs/loadWindowConfig)
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` (kernel PluginWasmHandle type)
- `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:1198-1206` (5-function WIT ABI doc)
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts` (ShardClient.turn/handleMessage/postMessage)
- `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts` (ruled-out backbone worker)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx:274-297` (resolveBootExampleId, FrameworkOsDefaults)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts:39-41` (VITE_SEMIO_DEFAULT_EXAMPLE)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs:35-38` (examples() vec order)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:63-64,276-286` (default_fixture, example id constants)
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:36-40` (puzzle3d playground entry)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27567-27600` (cooperative-maintenance debug)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/window-context-ownership.md` (peer's WIT/surface-context change)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/component-wasm-cooperative-maintenance-liveness.md` (cooperative-maintenance origin)
