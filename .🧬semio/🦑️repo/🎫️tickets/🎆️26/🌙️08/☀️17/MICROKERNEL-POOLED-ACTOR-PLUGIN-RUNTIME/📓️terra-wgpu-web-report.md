# 📓️ terra-wgpu-web — port wgpu-web renderer off the retired plugin ABI

Executor: `terra-wgpu-web`. Scope: `🧊️wgpu/🟦️typescript/**`, `🧊️wgpu/📦️index.ts` under
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/`
(TypeScript-only path — no Rust edited, no Rust build run).

## Starting state — the brief was already half-landed by a peer

The brief described `PluginWorkerClient` as still live in `🟦️boot.ts` (~line 49). On inspection it was
already gone: commit `d16fc1017c` (2026-08-19 15:51:04, message names it "Wgpu-web shard boot routes
through plugin-bridge worker handoff") had already ported the whole client off the old ABI. `boot.ts`
and `📦️index.ts` both import cleanly from the new `🐚️plugin-bridge.ts` (`loadPluginModule`/
`pluginHandleForBridge`), which itself already drives `ActivationRegistry` + `ShardClient`
(`🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`, `🧵️shard-runtime.ts`, `🖼️wire-turn.ts`) exactly
as `PluginRuntime/🟦️component.tsx` does. `git status` shows no local modifications to any of these
files before this session touched them — this was a genuinely prior, committed packet
(`wgpu-web-shard` per its own header docs), not stale ticket-note drift.

`PluginWorkerClient` grep-confirmed absent from first-party TS anywhere in the wgpu path scope, live
code or otherwise — the one surviving hit is a doc-comment in `🐚️plugin-bridge.ts`'s own header
noting it as retired.

## Real defect found and fixed — `plugin-bridge.ts` was built against a stale `PluginWasmHandle` shape

`PluginRuntime/🟦️component.tsx` (the reference this packet was told to copy) was itself edited
**after** `wgpu-web-shard` landed — `git log --date=iso` shows its last touch at
`cb9bcce7a4` / 2026-08-20 00:52:09, nine hours after the wgpu-web-shard commit. In that window
`PluginWasmHandle` flipped from a synchronous request/response `exchange(instanceId, frames) ->
Promise<frames>` method to a fire-and-forget `enqueue(instanceId, events): void` +
multicast `outcomes: AsyncIterable<TurnOutcome>` pair (`🎠️kernel/🟦️component.ts:221-234`,
`📌️important.md`'s "Replace, never wrap" list). `AppChannelClient`'s constructor
(`💻️os/🟦️component.ts:1863`, `AppChannelHandle = Pick<PluginWasmHandle, "enqueue" | "outcomes">`) now
only accepts that shape.

`🐚️plugin-bridge.ts` still built a `Pick<KernelPluginWasmHandle, "exchange">` object and passed it to
`new AppChannelClient(...)` — `exchange` no longer exists on `PluginWasmHandle` at all, so this was a
real, verified type error, not noise:

```
plugin-bridge.ts(246,54): error TS2344: Type '"exchange"' does not satisfy the constraint 'keyof PluginWasmHandle'.
plugin-bridge.ts(278,62): error TS2345: Argument of type 'Pick<PluginWasmHandle, "exchange">' is not
  assignable to parameter of type 'AppChannelHandle'.
  Type 'Pick<PluginWasmHandle, "exchange">' is missing the following properties from type
  'AppChannelHandle': enqueue, outcomes
```

**Fix** (in `🐚️plugin-bridge.ts`, region `🔖️WgpuPluginHandle`): replaced the `exchangeHandle` object
with `channelHandle: Pick<KernelPluginWasmHandle, "enqueue" | "outcomes">`, built from
`createTurnOutcomeBroadcast<TurnOutcome>()` (imported from `@semio-tech/framework`, already
re-exported by that package's `🟦️glue.ts`) plus a `runQueuedTurn` fire-and-forget helper — the same
frame-demux logic the old `exchange` body had (`shellFrameBytes` split into shell replies vs. leftover
effects, `applyRetainedWindowPatches` on any `uiPatches`), now pushing a `TurnOutcome` onto the
broadcast instead of returning a promise. `createApp` passes `channelHandle` to `AppChannelClient`;
`destroyApp` now calls `channelByInstance.get(instanceId)?.dispose()` before dropping the map entry
(ends that instance's outcome subscription, matching `AppChannelClient.dispose`'s own contract); the
handle-wide `dispose()` disposes every remaining channel and calls `turnOutcomes.complete()`. This
mirrors `PluginRuntime/🟦️component.tsx`'s own `handle`/`turnOutcomes`/`runQueuedTurn`/`adaptPluginHandle`
construction line for line — no new client invented, no `PluginRuntime` code touched (out of this
packet's lease).

Header doc comment updated to record the `enqueue`/`outcomes` requirement and the reason
`exchangeHandle` existed and had to go, so the next reader doesn't rediscover this the hard way.

## Acceptance

**grep-confirm — `PluginWorkerClient` gone from first-party TS in scope**: only hit is the
retired-history doc-comment in `🐚️plugin-bridge.ts`'s header; zero live references. `exchangeHandle` /
`.exchange(` / `exchange:` also grep-confirmed zero in scope after the fix.

**TypeScript suite** (`🧪️index.test.ts`, vitest, run directly — the package's own `nx test` target
runs `cargo test` first per `📜️script.ts`'s `TestScript`, which hit an unrelated, currently-red
`semio-framework-os-kernel` from a live peer edit at `🏪️store/🔄️sync/🦀️component.rs:1338`
(`E0308: expected HybridLogicalTimestamp, found future`) — out of this packet's Rust-free scope, so
vitest was invoked standalone instead):

```
$ cd 🎯️targets/🧊️wgpu && bunx vitest run --config 🧪️vitest.config.ts
 Test Files  1 passed (1)
      Tests  4 passed (4)
```

Both before and after the fix — the suite's fakes never exercised the broken `exchange` type path, so
it could not have caught this on its own; the fix was found by type-checking, not by this suite.

**Type-check** (scoped, since the repo's single root `tsconfig.json` covers the whole workspace and a
literal `tsc --noEmit -p tsconfig.json` run hit a genuine unrelated parse error in a live peer's file,
`🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts:18` — "Unterminated string literal" — blocking the
whole program; not this packet's file, not touched). Instead ran `tsc --noEmit` directly against the
four wgpu-scope entry files with the root config's compiler options reproduced on the CLI:

- **Before fix**: 5 errors inside `🧊️wgpu/**` (the `exchange`/`AppChannelHandle` mismatch above) + 90
  pre-existing errors in transitively-imported files entirely outside scope (`🖥️platform`,
  `🖱️ui/🎨️styling`, `🖼️assets`, `🛂️manifest`, `💻️os/🟦️component.ts`, `🔌️plugin/📇️registry/🟦️catalog.ts`
  — `TS5097`/`TS2304`/`TS2339`/`TS2300`/etc., all pre-dating this session).
- **After fix**: **0 errors inside `🧊️wgpu/**`**. The same 90 out-of-scope errors remain, byte-identical
  (diffed the two runs directly — the only line removed is the `exchange`/`AppChannelHandle` one).
  These 90 are not this packet's to fix (outside `path_scope`, pre-existing, unrelated files) and are
  not new damage from this change.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🐚️plugin-bridge.ts`
  — replaced the stale `exchange`-shaped handle with `enqueue`/`outcomes`, updated header doc.

No other file in scope needed a change: `🟦️boot.ts` and `📦️index.ts` already import only live symbols.

## Honest residue (not this packet's to fix, reported per binding rules)

- `semio-framework-os-kernel` is currently red from a live peer edit
  (`🏪️store/🔄️sync/🦀️component.rs:1338`, `E0308` on `HybridLogicalTimestamp`) — blocks this package's
  own `nx test`/`nx run …:test` target (which runs `cargo test` before vitest). Not Rust, not in scope,
  not touched.
- Repo-wide `tsc --noEmit -p tsconfig.json` currently aborts on a parse error in
  `🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts:18` (unterminated string literal) — a live peer edit,
  outside this packet's scope, not touched. Blocks any full-workspace type-check until fixed elsewhere.
- The 90 pre-existing out-of-scope type errors listed above are unrelated to this packet and were not
  introduced by it.
