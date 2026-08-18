# 📓️ terra — H1-react report

Packet: **H1-react** — put the React renderer on the actor kernel.

Status: **done** for the packet's four scope items. Real gaps flagged honestly below, not hidden.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`

**Not touched** (checked, confirmed clean): `WasmSessionLoader/🟦️component.tsx` (grepped for every removed ABI symbol — zero hits, this file deals with unrelated direct wasm-bindgen sessions), `🧑️‍💻️dev/🟦️component.ts` (a thin boot entry that only imports `bootFrameworkOs` from `Shell/🟦️component.tsx`, not mine — nothing to change).

## Ownership-boundary note (resolved before editing)

`📌️important.md`'s registrar-only file list names `ShellHost/🟦️component.tsx`. This packet's own brief explicitly lists it under "Owned writable paths" and item 2 of scope is entirely about this file, and `📓️status.md`'s "A3-kernel-types" entry already ties H1-react to finishing `ShellHost`'s `invokeExtension` branch by name. Treated the packet brief as authoritative for this packet (a direct, current instruction from the dispatching session) over the general standing rule, which predates this packet's dispatch. Flagging this explicitly rather than silently resolving it.

## Item 1 — PluginRuntime: ActivationRegistry + ShardClient, no more one-Worker-per-plugin

`loadPluginModule` no longer calls the deleted `acquirePluginModule`/`PluginModuleLease` (H2's "must not exist" list). It now:

- Registers the plugin's manifest with a shared, lazily-constructed `ActivationRegistry` (backed by one shared `ShardClient` — `min(hardwareConcurrency-1,4)` shards, workers created from `/plugin-modules/_shard/🟨️shard-worker.js`).
- Reads a build-time manifest via a new `fetchDescriptorManifest(pluginId, moduleUrl)` — fetches `🔣️descriptor.json` sibling to the module, falls back to an **honest empty manifest** (matching `ProgramBridge/🧊️component.rs`'s native `read_descriptor_manifest`, H3-wgpu-native) since only `🗒️note` has a real committed descriptor as of this ticket (`📓️status.md`'s E2 entry).
- On `createApp`: mints a globally-unique instance id (mirrors native `KernelClient`'s single global `next_instance_id` — a per-plugin counter would let two plugins both mint instance `1` and cross-contaminate the module-level effect buffer), activates a real actor via `ActivationRegistry.activate`, and submits an `instance-open` event as the first turn.
- `exchange(instanceId, frames)` (the raw kernel `PluginWasmHandle` shape `AppChannelClient` still expects) wraps each frame as an `app-command` event, submits one `ShardClient.turn`, and demuxes `TurnResult.effects` for `Effect::SendMessage{target: Shell{instance}}` entries (exactly what `⚛️reactor/🦀️component.rs`'s `route_app_frame` wraps every non-`UiPatch` `AppFrame` reply in — mirrors `📦️glue.rs`'s native `apply_turn_result`, H3-wgpu-native) back into the `AppFrame` bytes `AppChannelClient` decodes. Non-frame effects are stashed per-instance (`pendingTurnEffects`) and drained by `handleAction`/`handleCommand`.
- `destroyApp`/`dispose` call `ShardClient.dispose` per actor — no separate module-URL lease pool to evict anymore.
- **Concurrency safety**: `🟨️shard-worker.js` *rejects* (does not queue) a second in-flight turn for the same actor. The deleted `withSerializedPluginWasmHandle` used to queue concurrent `exchange()` calls transparently; added `serializePerActor` (one promise chain per actor id) as its direct replacement, and every `submitTurn` call (createApp/exchange/refreshUi/completeExtensionInvoke) funnels through it. Tested directly (3 new tests: same-actor calls never overlap, different actors run concurrently, a rejected turn doesn't stall the queue).

`adaptPluginHandle`'s signature changed from `(pluginId, lease: PluginModuleLease)` to `(pluginId, lease: {handle: KernelPluginWasmHandle; release})` (the deleted type replaced with its own inline shape) — every command/transaction/merge method it already had (`handleAction`, `transactionPrepare/Commit/Rollback/Undo/Redo`, `setMergePolicy`, `resolveConflict`, `readConflicts`, `applyMutations`, `documentPack`) is **unchanged**, since they only ever spoke `AppCommand`/`AppFrame` bytes through the one `exchange` seam and don't care what backs it now.

Also fixed, found by grep before editing: `type HostEffect` import (A3's global rename to `Effect` missed this file — the coordinator's own status.md entry already flagged one other miss in `⚛️react/📦️index.tsx`; this was a second, separate one, now fixed).

## Item 2 — ShellHost: UiPatch → retained tree, scene reads never await a plugin, invokeExtension finished

**Retained-tree UiPatch mechanism** (`applyUiPatchToRetained`, exported, independently unit-tested): decodes a `TurnResult.uiPatches[i].ops` entry, applies a root `PatchOp::Replace` (the *only* shape `⚛️reactor/🩹️patches/🦀️component.rs`'s `PatchTracker` emits this wave — its own doc says so plainly), and treats anything else — or a stale `baseRevision` on a non-full-replace patch — as an honest desync that keeps the previously retained body rather than applying an unverified partial walk. Mirrors `📦️glue.rs`'s native `KernelThreadState.retained` (H3-wgpu-native) field-for-field in intent.

This is wired into `PluginRuntime.loadPluginModule`'s own `refreshUi` override: a window-body request submits `Event::SurfaceVisible` and reads back **that same turn's** `TurnResult.uiPatches` (or the retained cache on an unchanged body — the guest's `PatchTracker` emits nothing when nothing changed) — never a separate blocking round trip, and `ShellHost` never touches wasm directly, so **the UI thread never awaits a plugin**, satisfying the packet's literal ask. `ShellHost`'s ~15 `program.refreshUi(...)` call sites needed **zero changes** — the public `PluginWasmHandle.refreshUi` contract is preserved exactly.

**Honest, reported-not-hidden limitation**: `⚛️reactor/🦀️component.rs`'s `dirty_render` loop hardcodes `plugin_render(instance, "window", "{}")` regardless of which surface key `SurfaceVisible` names, and `kernel_ui_patch_to_wit` hardcodes `surface.surface = 0` — meaning only ONE "window" surface can be distinguished per instance this wave. Panel/engagements/measures/tools/labels sections have no wire path at all yet (same honest gap `ProgramBridge/🧊️component.rs`'s native `window_engagements`/`window_measures` stubs already report) — `performRefreshUi`'s replacement returns an honest empty result for those, not a guess.

**invokeExtension** — the loud placeholder A3 left is gone. Added `PluginWasmHandle.completeExtensionInvoke(instanceId, req, outcome)`, implemented in `loadPluginModule` by submitting `Event::Completed{req, outcome}` on the *originating* actor's own turn queue (resumes the guest SDK's parked `RequestRegistry` future, design-abi.md §2). `ShellHost`'s `applyHostEffects` `invokeExtension` branch now calls it on both success (`ok` outcome, pack-encoded output) and failure (`fault` outcome) instead of `console.error`-and-drop.

**Real regression found and fixed, unrelated to the above but blocking compilation**: `ShellHost` imported and called `evictPluginModule` (H2 deleted the function outright, not just its own unused import in `glue.ts`) at 3 call sites (reload/uninstall-plugin/uninstall-extension). Removed all 3 calls — `current.handle.dispose()`, already called immediately before each, now fully owns freeing every actor that handle ever activated (`ShardClient.dispose` per actor); there is no separate shared-module resource left to evict in the new design. `pluginModuleUrlByIdRef` (the bookkeeping map that fed `evictPluginModule`) is left in place, write-only now, for a future consumer, with an updated doc comment — not removed, to keep this a minimal, targeted fix.

## Item 3 — react/index.tsx + index.test.ts: A4's leases

- `index.tsx`: dropped the `type SectionProbe` import (unused elsewhere in the file, confirmed by grep). Also found and fixed the same class of miss as PluginRuntime's `HostEffect`: `acquirePluginModule`/`evictPluginModule`/`type PluginModuleLease` were *also* imported here (separately from the barrel re-export), unused elsewhere, removed.
- `index.test.ts`: rewrote the two tests A4 flagged plus three more it couldn't have known about (discovered via targeted `tsc`, all upstream of A4's own report):
  1. **"preserves batched UI refreshes..."** (`RefreshUi`/`SectionProbe`/`UiSection`) → rewritten as an assertion that a bare `adaptPluginHandle` (no actor context) returns an honest empty `refreshUi` result, since there is no `AppCommand` left to send for it.
  2. **"adaptPluginHandle.handleAction round-trips ... with effects ..."** (`encodeAppFrame({Effects:...})`) → the `Effects` frame construction is dropped (that variant doesn't exist); `output`/`uiScope`/`historyPatch` coverage from the *same* `AppFrame::Invocation` frame is kept verbatim (still real, unchanged wire coverage); `requestedEffects` is asserted `[]` with a comment explaining effects now arrive via `TurnResult.effects`, which a bare exchange-only fake has nothing to populate.
  3. **"loads plugin modules through framework-core, refcounted"** (`acquirePluginModule`, deleted) → replaced with a real test of `fetchDescriptorManifest`'s honest-empty-vs-real-descriptor fallback (mocks `fetch`).
  4. **"serializes concurrent program wasm handle calls"** (`withSerializedPluginWasmHandle`, deleted alongside `PluginWorkerClient`) → replaced with 3 direct tests of `serializePerActor`, its real replacement.
  5. **"detects jco payload-shaped plugin instance busy errors"** (`isPluginInstanceBusyError`/`pluginErrorText`, deleted) → removed with a `🪦️` tombstone comment: `serializePerActor` makes the race that used to produce a "busy" error structurally impossible at this layer, so there is no error shape left to detect — prevention replaced detection, not a coverage drop.
  - Also fixed one real pre-existing bug in the test I was rewriting: the `Invocation` frame literal in test 2 was missing the (already-required) `messages` field.

New `applyUiPatchToRetained`/`fetchDescriptorManifest`/`serializePerActor` are exported from `PluginRuntime.tsx` and re-exported from `react/📦️index.tsx` purely for this direct testability — no other consumer.

## Acceptance — commands run, real output, exit codes

**Typecheck** — no dedicated `typecheck` target exists for either package (same finding every prior packet in this ticket already recorded). Ran `node_modules/.bin/tsc --noEmit` against each of the 4 touched files individually with the repo's own `tsconfig.json` compiler options reproduced on the command line (bare `tsc` can't take `--project` + file args together). Triaged every error by file:

- `PluginRuntime/🟦️component.tsx`: **0 errors** in this file itself (only the same `TS5097` `.ts`-extension noise every bundler-resolved import in this repo produces under bare tsc, already catalogued by A3/H2's reports).
- `ShellHost/🟦️component.tsx`: **0 errors** attributable to this packet. ~80 remaining errors are pre-existing and unrelated — `TutorialUiSnapshot`/`TutorialTracks`/`TutorialDocumentEventKind` (a different in-flight ticket's tutorial-recording work), `WindowKindDefinition`/`SpawnedAppEntry.breadcrumb`/`ArtifactPresencePeer.cursor`/`PluginViewState.presencePeersJson` (hub-spaces/presence work), `ExternalSlotResolverContext` (a real but pre-existing two-different-`PluginWasmHandle`-types-same-name collision between kernel's and PluginRuntime's own — existed before this packet touched anything, `PluginRuntime`'s `PluginWasmHandle` never had a raw `exchange` method). None reference `AppFrame`/`AppCommand`/`SectionProbe`/`ShardClient`/`ActivationRegistry`/`UiPatch`. Full output: `📓️terra-H1-tsc-shellhost.txt`.
  - **Real regression found and fixed** during this triage: `evictPluginModule` (deleted by H2) — see item 2 above.
- `⚛️react/📦️index.tsx`: **0 errors** attributable to this packet after fixing the `acquirePluginModule`/`evictPluginModule`/`PluginModuleLease` imports. Remaining errors are entirely in a *different*, transitively-pulled-in file (`🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` — Tutorial/translation-schema/`@xyflow`/`three` type mismatches, an unrelated package) plus a couple of pre-existing unrelated ones in this file itself (`SelectionMergeMode` naming, `TutorialDocumentEvent*`, `ThreeEvent` from `three`).
- `⚛️react/🧪️index.test.ts`: **0 errors** referencing anything this packet touched (`SectionProbe`/`AppFrame`/`AppCommand`/`PluginModuleLease`/`acquirePluginModule`/`refreshUi`/`adaptPluginHandle`/`applyUiPatchToRetained`/`fetchDescriptorManifest`/`serializePerActor`/`messages` field — all clean). ~80 remaining errors are pre-existing test-fixture/type mismatches unrelated to the ABI flip (`ActionDefinition` missing `iconId`/`semantics`, `IconName` literals, `ModeWindowDescriptor`, etc.) — none in the `describe("framework plugin runtime", ...)` block.

**`bun ./📜️script.ts test quick` — `@semio-tech/framework-renderer-react`** (the react target — the real acceptance gate for this packet, not `framework-os`'s, since that package's vitest config only runs `💻️os/🟦️component.ts`'s own inline tests and never touches these files):

```
Test Files  1 failed (1)
     Tests  15 failed | 321 passed (336)
   Errors  1 error
EXIT: 1
```

**15 failures, all pre-existing and unrelated** — verified by name (`renders selectable builder cards with selection ring`, `interprets virtual file system component scenes`, `isolates render faults in ShellFaultBoundary`, window-action-panel staging tests, `commandCategories orders...`, mit-bestand footer/introduction tests, `buildCommandCategoryTabs` tests, `FrameworkOsShell portal layer...`) — none touch plugin runtime, channel wire types, or anything this packet edited; root causes match the pre-existing `Tutorial*`/`ActionDefinition`/CSS-class/`toHaveTextContent`-matcher gaps already found via `tsc` triage above. The 1 unhandled error (`backbone-worker.ts`'s `postMessage` under jsdom) is likewise pre-existing and unrelated.

**Isolated re-run of just this packet's own tests** (`-t "framework plugin runtime"`, the describe block containing every `adaptPluginHandle`/`applyUiPatchToRetained`/`serializePerActor`/`fetchDescriptorManifest` test):

```
Test Files  1 passed (1)
     Tests  14 passed | 322 skipped (336)
EXIT: 0
```

Full raw output: `📓️terra-H1-vitest-final.txt`.

**`bun ./📜️script.ts test quick` — `@semio-tech/framework-os`** (unaffected by this packet — nothing here touches `💻️os/🟦️component.ts`/`🟦️backbone-worker.ts` — run to confirm zero regression against the ticket's recorded baseline):

```
Test Files  2 failed | 2 passed (4)
     Tests  2 failed | 322 passed (324)
EXIT: 1
```

**Exactly the same 2 pre-existing failures** the ticket's baseline names (`@semio-tech/framework-os workflow > matches the Rust plan_workflow across shared fixtures decoded via wasm`, twice — missing built wasm artifact `semio_framework_os.js`, unrelated to this packet). Total test count grew from the ticket's recorded 318 to 324 (other packets landing tests concurrently, e.g. E1/E2/A5) — the failure count and identity are unchanged. **No regression.**

## Known gaps (flagged, not hidden)

1. **`wireEffectToFriendly`'s WIT-effect→friendly-`Effect` conversion covers only the effect kinds this renderer's own `applyHostEffects` branches on today** (~14 of ~40 WIT `effect` variants: `notify`, `navigate`, `open-external-url`, `set-panel`, `clipboard-write`, `replay-shell-command`, `set-active-utility`, `set-active-tool`, `open-window`, `close-window`, `dispatch-action`, `open-dialog`, `invoke-extension`, `spawn-plugin-instance`, `open-plugin-instance`, `request-sync`). Anything else degrades to an honest `[DEBUG]`-logged drop, not a guess. Widening this is mechanical once a real compiled plugin exists to verify field names against.
2. **The exact jco binding shape crossing the wasm boundary (`ShardClient.turn()`'s resolved value, and the `ShardEventEnvelope` shape `🟨️shard-worker.js` passes into `reactor.poll`) is UNVERIFIED against a real compiled artifact** — the same gap H2's and H3's own reports already flag, still open: no plugin has migrated onto `world actor` yet (W3 hasn't started). This packet's `WireVariant{tag,val}`/camelCase-field assumption is the same convention every other packet touching this boundary already committed to, documented once at the top of `PluginRuntime.tsx`'s `🔖️ActorAdapter` region rather than hedged with speculative fallback branches. Swapping field names is mechanical if the real shape differs.
3. **Panel/engagements/measures/tools/labels window-refresh sections have no wire path** (see item 2 above) — an upstream limitation of `⚛️reactor/🦀️component.rs`'s current `dirty_render` loop (A2's territory), not invented or worked around here.
4. **`Event::Completed`'s `outcome` payload for `completeExtensionInvoke`** is pack-encoded JSON (`encodePackValue`) on the assumption the guest SDK's `RequestRegistry` future resolves to a JSON-shaped value for an `invoke-extension` completion — reasonable by analogy with every other `pack`-typed field this file already handles, but genuinely unverified end-to-end (same class of gap as #2 — no real extension consumer exists yet either).

## Ownership / process compliance

No git-modifying command run. No `.cargo/config.toml`/registrar file touched. No scratch file outside `.txt`/`.md` in the ticket folder. All new `console.*` calls carry the `[DEBUG]` prefix. Grepped the touched files for every symbol on `📌️important.md`'s "must not exist" list plus H2's own list — all remaining mentions are doc comments explaining what was deleted, zero real usages. Flagged one out-of-scope, pre-existing, unrelated bug via `spawn_task` (a dangling `tailwind.config.ts` import in `@semio-tech/ui-styling` that briefly blocked this package's whole test suite from loading at all — resolved by the time of the final acceptance run, either by that task or a concurrent session; not edited by me either way, outside `📺️renderer/**`).
