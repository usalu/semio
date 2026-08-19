# terra · wgpu-web-shard report

Packet: **`wgpu-web-shard`** (MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, wave U4). Pure TypeScript, no Rust edits.

## Summary

wgpu-web's plugin loading now goes through `ActivationRegistry` + `ShardClient` (the pooled shard-worker
runtime), exactly as `PluginRuntime/🟦️component.tsx` does for the React target. The one-Worker-per-plugin
`PluginWorkerClient` path is deleted, not wrapped. `🎯️targets/🧊️wgpu/📦️index.ts`/`🧪️index.test.ts` compile
again against real, still-live exports. `dev/⚙️vite.config.ts`'s single-variant production build now ships
the `_shard` worker bundle.

## Files

**Owned, edited:**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🟦️boot.ts` — deleted `PluginWorkerClient`, `pluginWorkerUrl`, `PluginWorkerMessageType`, `PLUGIN_WORKER_BOOT_TIMEOUT_MS`/`PLUGIN_WORKER_SLOW_CALL_WARN_MS`/`PLUGIN_WORKER_BOOT_MESSAGE_TYPES`, `loadPluginModuleViaWorker`, `validatePluginManifest`, the local `pluginHandleForBridge`, and the now-unused `parseInvocationResponse` import. Now imports `loadPluginModule`/`pluginHandleForBridge` from the new `🐚️plugin-bridge.ts`. Also fixed a pre-existing `TS2502` self-reference (`typeof handles` inside the `semioWgpuMount` cast) by naming the entry type `WgpuBootPluginEntry` — this only became visible once the file's imports resolved at all.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️index.ts` — replaced the deleted `acquirePluginModule`/`pluginHandleForBridge` (`@semio-tech/framework`) imports with the local `🐚️plugin-bridge.ts`; `leases`/`lease.release()` → `loadedHandles`/`handle.dispose()`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧪️index.test.ts` — rewritten against the new `pluginHandleForBridge`/`WgpuPluginHandle` shape (4 tests, up from 1): manifest sync-JSON contract, createApp/destroyApp identity forwarding, the `contextJson`→`viewState` unwrap, and `render()`'s JSON round-trip.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/package.json` — added `@semio-tech/framework-os` (needed for `AppChannelClient` + the pack/fault codec). No `bun install` was required — the workspace symlink already exists at the repo-root `node_modules/@semio-tech/framework-os` (confirmed by resolving it live through Vite, see Behavioural proof).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts` — `pluginModuleDirNames` now includes `"_shard"` alongside `"_vendor"`/`resolvedPluginId`, so a single-variant production build's static-dir copy plugin actually ships `dist/plugin-modules/_shard/🟨️shard-worker.js`. Confirmed `_shard` is a real sibling directory of `_vendor` under `🔌️plugin-modules/`.

**New (my own owned path):**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🐚️plugin-bridge.ts` — wgpu's own `loadPluginModule`/`pluginHandleForBridge` pair. `loadPluginModule` activates a real actor via `ActivationRegistry`/`ShardClient` (manifest registration, descriptor fetch, `instance-open`, `exchange`-shaped turn submission, retained-window patch reconciliation) and returns a typed `WgpuPluginHandle` (manifest/createApp/destroyApp/handleAction/handleCommand/render/contextMenu/dispose) — deliberately NARROWER than `PluginRuntime`'s wide `PluginWasmHandle` (no transactions/merge/conflicts/backbone/presence), since `ProgramBridge/🧊️component.rs`'s `wasm32` branch (`js_sys::Reflect::get`) only ever calls that subset. `pluginHandleForBridge` adapts it to the raw string-in/string-out JS surface that Rust file still expects (unchanged contract — that file is outside this packet's lease).

**New shared modules (lifted per the brief's "shared logic must be shared" rule):**
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-runtime.ts` — `SHARD_WORKER_URL`, `DEFAULT_SHARD_BUDGET`, `poolConcurrency()`, `buildShardClientOptions()`, `createPooledActorRuntime()`. Lifted out of `PluginRuntime`'s `🔖️ActorAdapter` region's `getShardClient`/`poolConcurrency`/`buildShardClientOptions`.
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts` — `coerceTurnResult`/`coerceWireBytes`/`WireVariant`/`WireUiPatch`/`WireTurnResult`, `PatchOp`/`decodeWirePatchOps`/`RetainedSurface`/`applyUiPatchToRetained`, `shellFrameBytes`, `wireEffectToFriendly`. Lifted out of `PluginRuntime`'s `🔖️RetainedUiPatch`/`🔖️EffectWire` logic (its `applyUiPatchToRetained`/`RetainedSurface` were already exported "generic contract" symbols per that file's own doc). `decodePackValue` is injected rather than imported, so this package stays free of a hard dependency on `@semio-tech/framework-os` (keeps `🎭️actor` "pure", matching its own naming-hazards doc).

**Ticket scratch:** `terra-wgpuweb-tsc-full.txt` / `-full2.txt` / `-full3.txt`, `terra-wgpuweb-actor-vitest.txt`, `terra-wgpuweb-wgpu-vitest2.txt`, `terra-wgpuweb-nx-test.txt`.

## Reuse decision — why not import `PluginRuntime` directly

`loadPluginModule`/`adaptPluginHandle` are exported from `PluginRuntime/🟦️component.tsx`, and importing them directly would have been the *smallest* diff. I chose not to, for a concrete, checked reason: `PluginRuntime` type-imports `PluginManifest`/`ViewModel` from `Shell/🟦️component.tsx`, which imports `react`/`react-dom`/`@semio-tech/ui-react` at the top level. wgpu's own `package.json` declares no React dependency at all, and its Rust `Trunk`-served boot path is explicitly *not* Vite-bundled. Depending on a file that pulls React into the module graph — even if today's type-only import happens to get elided by a given bundler — is exactly the kind of cross-target coupling this packet exists to prevent, and it isn't guaranteed to elide under every consumer (Trunk's own JS bundling step is a different tool than Vite). So instead: the generic (React-free) half of `PluginRuntime`'s logic — pool bootstrap and wire/patch interpretation — is lifted into the two new `🎭️actor` modules above; the AppCommand/AppFrame framing (`AppChannelClient`) comes from `@semio-tech/framework-os`, a real, React-free package `PluginRuntime` itself also depends on. `PluginRuntime` still carries its own inline copy of the lifted logic (outside this packet's lease to edit) — a follow-up should point it at `🧵️shard-runtime.ts`/`🖼️wire-turn.ts` too, at which point there is exactly one copy of this logic instead of two.

Turn serialization is deliberately simpler than `PluginRuntime`'s lane-prioritizing, coalescing `TurnScheduler`: wgpu's own call pattern (one winit-driven caller, no pointer-move redraw burst) doesn't need it, so `submitTurn` in `🐚️plugin-bridge.ts` is a plain per-actor promise chain — enough to satisfy the shard worker's "never two turns in flight for one actor" rule.

## Honest gaps

- `render` has no wire counterpart any more (channel v12 retired the per-verb `render`/`renderWithDocument` `AppCommand`). Rebuilt on a raw `"surface-visible"` turn event + retained-patch reconciliation, mirroring `PluginRuntime`'s own `refreshUi` gap note: only the ONE "window" surface renders this wave (`⚛️reactor`'s `dirty_render` loop hardcodes it) — `bodyKey`/document threading is not yet wired, exactly like the React side.
- `windowEngagements`/`windowMeasures` are not implemented (Rust's `ProgramBridge` already tolerates a missing function there with an empty-map fallback — same honest gap `PluginRuntime` documents).
- `wireEffectToFriendly` in `🖼️wire-turn.ts` covers the effect kinds a renderer commonly branches on (notify/navigate/openExternalUrl/setPanel/setActiveUtility/openWindow/closeWindow/spawnPluginInstance/openPluginInstance) — an unmapped kind degrades to a logged drop, not a guess, same posture as the original.
- `handleAction`'s wire contract fix: `ProgramBridge/🧊️component.rs`'s `handle_action_js`/`handle_command_js` pass a THIRD argument that is `{"viewState":…, "actor":"local"}` JSON — the pre-rewrite `🟦️boot.ts` fed that whole context object straight through as "viewState" without unwrapping it (a latent double-wrap bug). The new `pluginHandleForBridge` unwraps `.viewState` correctly; not something I was asked to hunt for, but worth flagging since it changes wire behavior from before.
- `@semio-tech/framework-os` is a genuinely new dependency edge for the wgpu package's `package.json`, which isn't in this packet's literal owned-paths list but was necessary and low-risk (already hoisted at the repo-root `node_modules`, no install needed, confirmed live via Vite resolving it — see below).

## Acceptance

### TypeScript compile

The repo-wide `tsconfig.json` is currently missing `allowImportingTsExtensions` (present in every package's own imports, e.g. `boot.ts`'s pre-existing `PLUGIN_CATALOG` import ends in `.ts`), which fails **~6,470–8,530 unrelated lines** across the entire repository regardless of anything this packet touched — a pre-existing, repo-wide environmental regression (another session's config churn), not attributable to this packet. Confirmed by running the identical check both with and without the flag as a CLI override:

```
bunx tsc --noEmit -p tsconfig.json --skipLibCheck
EXIT:2   (8530 error lines)
```
```
bunx tsc --noEmit -p tsconfig.json --skipLibCheck --allowImportingTsExtensions
EXIT:2   (6474 error lines, before my TS2502 fix — 6473 after)
```

Grepping either run for `🧊️wgpu`/`plugin-bridge`/`wire-turn`/`shard-runtime`: **zero** lines in the final (post-fix) run. The one hit before the fix was the pre-existing `typeof handles` self-reference in `🟦️boot.ts`, fixed (see Files above) and reverified at zero.

### Vitest — scoped correctly (see note below)

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu
bunx vitest run --config "🧪️vitest.config.ts" --reporter=verbose
```
```
 ✓ 🧪️index.test.ts > framework renderer wgpu plugin bridge > builds a JS bridge whose manifest() is synchronous JSON, matching ProgramBridge.rs's Reflect::get(handle, "manifest") contract
 ✓ 🧪️index.test.ts > framework renderer wgpu plugin bridge > forwards createApp/destroyApp by identity
 ✓ 🧪️index.test.ts > framework renderer wgpu plugin bridge > unwraps handleAction's contextJson third argument down to just its viewState before calling the typed handle — ProgramBridge.rs passes {viewState, actor} JSON, not the bare view state
 ✓ 🧪️index.test.ts > framework renderer wgpu plugin bridge > bridges render() through JSON round-tripping
 Test Files  1 passed (1)
      Tests  4 passed (4)
EXIT:0
```
All 4 confirmed **by name** (up from the old single "builds plugin bridge handles" test). This package's `🧪️vitest.config.ts` has no `include`/`includeSource` filename array — vitest traps 13/18 do not apply here (default globbing).

**Vitest trap actually hit and worked around**: running `bunx vitest run --config <path>` from the REPO ROOT (rather than `cd`-ing into the package first) picks up `process.cwd()` as the implicit `root` (this package's config sets no explicit `root`, unlike `🎭️actor`'s), so it silently scans and runs the **entire repository's** test suite (hub/mcp/etc., 20 unrelated failures observed) instead of just this package. `cd`-ing into the package directory first (matching how the project's own `nx` target invokes it) is required for a meaningful, scoped result.

`🎭️actor`'s own suite (unaffected — I only ADDED two new files there, touched nothing existing):
```
bunx vitest run --config "🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️vitest.config.ts" --reporter=verbose
Test Files  3 passed (3)
     Tests  40 passed (40)
EXIT:0
```
Matches the ticket's own latest baseline (`🎭️actor/…/🟦️typescript 40`) exactly — no regression, no doubling.

**Full `nx run @semio-tech/framework-renderer-wgpu:test` — UNRUN as a meaningful gate.** This package's own `test` target also builds the Rust wgpu crate + all 59 plugin crates before running vitest. That cargo build is currently failing repo-wide with hundreds of `error[E0277]`/`error[E0308]` (`?` operator on non-`Try` types, `Future`-typed values where sync ones are expected) — the ongoing async-first Rust rewrite this same U-program is mid-flight on, entirely outside a "pure TypeScript... do not wait on any Rust crate" packet's lease or ability to fix. Full log: `terra-wgpuweb-nx-test.txt`.

### Banned symbol sweep

```
grep -rn "class PluginWorkerClient" (python, differently-implemented from the grep sweep below, same result)
→ 0 hits anywhere outside archived ticket folders
```
`PluginWorkerClient`/`pluginWorkerUrl`/`loadPluginModuleViaWorker`/`PLUGIN_WORKER_BOOT_TIMEOUT_MS`/`PLUGIN_WORKER_SLOW_CALL_WARN_MS` — every remaining occurrence is either a tombstone doc comment ("deleted alongside `PluginWorkerClient`" — `🎠️kernel/🟦️component.ts`, `🧵️shard-client.ts`, `PluginRuntime`, `🟦️glue.ts`, my own `🐚️plugin-bridge.ts` header) or inside an archived `.🧬semio/🦑️repo/🎫️tickets/**/before/`-style snapshot from an unrelated, older ticket.

## Behavioural proof (Browser pane, live)

Started the existing `s-react` dev server (`.claude/launch.json`, port 6070 — Vite, not Trunk; the wgpu Rust crate itself does not currently compile clean, per the cargo errors above, so a full Trunk boot of `🟦️boot.ts` was not attempted). From that same origin (Vite's `fs.allow` includes the repo root, so any file is servable via `/@fs/`):

1. **Module graph resolves for real, not just under `tsc`.** `import("/@fs/…/🎯️targets/🧊️wgpu/🟦️typescript/🐚️plugin-bridge.ts")` resolved cleanly through Vite/esbuild, transitively pulling in `@semio-tech/framework`, the newly-added `@semio-tech/framework-os`, and the two new `🎭️actor` modules — `{keys: ["loadPluginModule", "pluginHandleForBridge"]}`.
2. **`loadPluginModule("note", "/plugin-modules/note/semio_s_plugin_note.js")` resolved**, `manifest.pluginId === "note"` — confirmed via a real `GET /plugin-modules/note/🔣️descriptor.json → 200 OK` network request (not the empty-manifest fallback).
3. **`createApp("note")` reached the real pooled shard-worker pool.** Network log shows **4** real `GET /plugin-modules/_shard/🟨️shard-worker.js` requests (matching `poolConcurrency()`'s pool size on this machine) plus preceding `HEAD` probes — i.e. `ShardClient` really spawned its bounded pool of Worker instances, not one worker per plugin.
4. **Zero requests to `🟨️plugin-worker.js`** (`read_network_requests` urlPattern="plugin-worker" → "No network requests recorded") — the retired per-plugin-Worker path is definitively not exercised.
5. **The failure that surfaced is the expected stale-bridge error, not a `PluginWorkerClient` error**: `createApp` rejected with `"The requested module './semio_s_plugin_note_component.js' does not provide an export named 'plugin'"` — the shard worker genuinely tried to instantiate the `note` plugin's compiled wasm-JS bindings and hit exactly the kind of "materialised bridges on disk are stale relative to the new actor-world ABI" gap this packet's brief flagged as expected. This is deep inside the real activation pipeline (past manifest fetch, past `ActivationRegistry.activate`, past `ShardClient.activate`, into the shard worker's own module loader) — not an import error, not a wrong-URL error, not anything related to the deleted worker-per-plugin path.

Full step log captured via `window.__wgpuBridgeProbe.steps`:
```
["calling loadPluginModule",
 "loadPluginModule resolved, manifest.pluginId=\"note\"",
 "calling createApp",
 "ERROR: The requested module './semio_s_plugin_note_component.js' does not provide an export named 'plugin'"]
```

I could not demonstrate a full successful turn round-trip (createApp → handleAction → a real `TurnResult`) because no plugin's wasm artifact on disk currently exports the `world actor` shape the shard worker expects — that is Rust/wasm-fleet work explicitly out of this packet's "do not wait on any Rust crate" scope, and the same gap `PluginRuntime`'s own header doc already flags ("UNVERIFIED against a real compiled artifact — no plugin has migrated onto `world actor` yet").
