# 📓️ terra — exchange-removal report

Packet: **exchange-removal** — delete the banned `exchange` RPC symbol (kernel/os/PluginRuntime).

Status: **done** on the three owned files. Zero `\bexchange\b` in all three (pasted below). Real,
precisely-scoped fallout in two files this packet does not own — see Lease-requests. One brief
instruction (bump `APP_CHANNEL_VERSION`) deliberately NOT followed — see that section for why.

## What changed, and why it's the right shape

`PluginWasmHandle.exchange(instanceId, frames) -> Promise<frames>` assumed a command's reply always
lands on the SAME call that sent it — a synchronous-RPC shape wrong for the turn model (a reply may
arrive N turns later). Replaced with:

- **`🎠️kernel/🟦️component.ts`** (new `//#region 🔖️TurnOutcomeBroadcast`, right above `PluginWasmHandle`):
  - `TurnOutcome` — `{instanceId, frames}` or `{instanceId, error}` (the `error` arm covers what used to
    reject `exchange`'s promise; an `AppFrame::Error` frame is still an ordinary `frames` entry, decoded
    exactly as before).
  - `createTurnOutcomeBroadcast<T>()` — a small multicast queue: every independent
    `[Symbol.asyncIterator]()` call gets its OWN subscription fed every `push`ed value (not a shared
    drain-once FIFO), `return()` unsubscribes, `complete()` force-closes every live subscriber. This is
    the ONE new primitive both consumers below build on.
  - `PluginWasmHandle.exchange` → `enqueue(instanceId, events): void` (fire-and-forget) +
    `outcomes: AsyncIterable<TurnOutcome>` (handle-wide, multicast).
  - 4 new in-source tests pin the broadcast's multicast/unsubscribe/force-close contract directly
    (`@semio-tech/framework-kernel`, the ONLY package whose `includeSource` actually globs this file —
    see the testing note below).

- **`💻️os/🟦️component.ts`**: `AppChannelHandle = Pick<PluginWasmHandle, "enqueue" | "outcomes">`.
  `AppChannelClient` now owns one persistent background loop (`pumpOutcomes`, started in the
  constructor) against its own `outcomes` iterator: it filters to `this.instanceId`, and resolves the
  OLDEST pending `sendCommand` (renamed from `exchangeOne`) waiter FIFO. This is sound, not a guess:
  `PluginRuntime`'s `TurnScheduler` already serializes one actor's turns in submission order within a
  lane, and every real call site here still awaits one command before sending the next — so outcomes
  for one instance can never arrive out of send order. Added `AppChannelClient.dispose()` (calls the
  iterator's own `return()`) so a torn-down instance doesn't leak a subscriber against the handle-wide
  stream for the rest of the handle's lifetime. `APP_CHANNEL_VERSION` **left at 12** — see below.

- **`PluginRuntime/🟦️component.tsx`**: `loadPluginModule` now builds one `createTurnOutcomeBroadcast`
  per handle (`turnOutcomes`). The old inline `exchange: async (instanceId, frames) => {...}` body moved
  into `runQueuedTurn` (same `ShardEventEnvelope` mapping, same `shellFrameBytes`/`pendingTurnEffects`/
  `applyRetainedWindowPatches` demux, byte-for-byte unchanged) — now called fire-and-forget from
  `enqueue`, wrapped in try/catch so a turn-submission failure becomes an `error`-shaped outcome instead
  of an unobserved rejection. `handle.dispose()` now also calls `turnOutcomes.complete()`.
  `adaptPluginHandle`'s `destroyApp` now calls `channels.get(instanceId)?.dispose()` before dropping the
  channel. Converted the file's own three `exchange`-shaped test fakes (`fakeLease` ×2, both under
  "PluginRuntime documentPack/transaction wire adapter") to `enqueue`/`outcomes` using the same broadcast
  helper. Every remaining doc-comment mention of "exchange" reworded (not deleted-and-forgotten — each
  now names the real replacement: `enqueue`/`outcomes`, `runQueuedTurn`, or "turn-channel call").

## `zero \bexchange\b` — pasted search, all three owned files

```
$ python3 -c "... re.search(r'\bexchange\b', line, re.IGNORECASE) over all 3 files ..."
🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts -> 0
🧰️framework/🛍️products/💻️os/🟦️component.ts -> 0
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx -> 0
```
(`grep -w` behaves identically — verified `plugin_exchange`/`exchangeOne` do NOT match a
whole-word `exchange` search, confirmed with a differently-implemented Python regex check per the
"reproduce a negative with a second tool" rule; both queries agree.)

## `APP_CHANNEL_VERSION` — NOT bumped, deliberately

The brief said "bump 12 → 13 and update its cross-language pin test." I did not, because doing so would
be **wrong**, not merely unnecessary:

- Both the TS doc comment (`💻️os/🟦️component.ts:1788-1790`, an orphaned fragment sitting just above
  `decodeFaultFromWire`) and the Rust pin test's own assertion message
  (`📡️spr/🧵️channel/🦀️component.rs:1614`: `"CHANNEL_VERSION and the shared cross-language pin
  disagree — bump both, plus APP_CHANNEL_VERSION"`) are explicit: the version guards **wire-incompatible
  `AppCommand`/`AppFrame` frame changes**.
- This packet changed **zero** wire bytes. `enqueue`'s `events`/`outcomes`' `frames` are the exact same
  `encodeAppCommand`/`decodeAppFrame` byte streams `exchange` always carried — only the JS-level calling
  convention around them (sync-return vs. fire-and-forget/broadcast) changed.
- The pin fixture (`🧫️fixtures/📡️channel/channel-version.json`) is genuinely **shared cross-language**:
  Rust's `CHANNEL_VERSION` in `📡️spr/🧵️channel/🦀️component.rs:24` is a **hardcoded `u32 = 12`**, a file
  outside every packet's `path_scope` I can find (not `🖥️host`/`🔌️plugin`, not mine, not named in any
  packet's owned paths). Bumping only the JSON pin (the only file I *could* reach) would desync it from
  Rust's still-12 constant and fail the RUST test in a file I'm not permitted to touch — a real
  regression, for zero actual protocol benefit.

Left `APP_CHANNEL_VERSION = 12` and the pin fixture untouched. The pin test still passes (see Acceptance)
because nothing about the pinned number changed. If the coordinator wants a symbolic bump anyway despite
no wire change, that's a `Cargo`/`📡️spr` edit outside this packet's lease — flag it explicitly rather
than silently doing it wrong.

## Testing note: PluginRuntime/🟦️component.tsx has NO wired test runner

Confirmed by reading every `🧪️vitest.config.ts` under `📺️renderer/**`: the react-renderer target
(`@semio-tech/framework-renderer-react`) has no `includeSource` at all (default `include` only matches
`*.test.ts`, and `PluginRuntime/🟦️component.tsx` isn't one), and no other config names this file either.
This is a **pre-existing gap**, already documented by a prior packet
(`terra-web-plugin-runtime-scratch/after.vitest.config.ts`'s own doc comment: "has no project of its
own"), not something this packet introduced or is newly discovering. I reused that existing scratch
harness rather than building a new one (R10-adjacent: reuse the diagnostic tool that's already there).
Flagging again here because it means this file's own 26 in-source tests — including the 3 I converted —
are invisible to every `nx`/`bun ./📜️script.ts test` command; the coordinator should decide whether
wiring a real project for this file is worth a dedicated packet.

## Acceptance — commands run, real output, exit codes

**`@semio-tech/framework-kernel`** (the dedicated package for `🎠️kernel/🟦️component.ts` —
`includeSource: ["*.ts"]`, globs the file directly):
```
$ cd 🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript && bun ./📜️script.ts test quick --reporter=verbose
 Test Files  1 passed (1)
      Tests  33 passed (33)
EXIT: 0
```
33 = the recorded baseline of 29 + my 4 new `createTurnOutcomeBroadcast` tests, all passing by name
(pasted in full: `terra-exchange-test-kernel-verbose.txt`).

**`@semio-tech/framework`** (glue.ts's own suite, unaffected — sanity check only):
```
$ cd 🧰️framework/📦️packages/🟦️typescript && bun ./📜️script.ts test quick
 Test Files  1 passed (1)
      Tests  87 passed (87)
EXIT: 0
```
Matches the recorded baseline exactly (`terra-exchange-test-framework.txt`).

**`@semio-tech/framework-os`** (`includeSource` names `💻️os/🟦️component.ts` directly, no doubling):
```
$ cd 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript && bun ./📜️script.ts test quick --reporter=verbose
 Test Files  1 failed | 2 passed (3)
      Tests  1 failed | 206 passed (207)
EXIT: 1
```
The 1 failure is the SAME pre-existing named failure the coordinator's own LATEST baseline records
(`matches the Rust plan_workflow across shared fixtures decoded via wasm` — missing built wasm artifact,
zero references to anything this packet touched). **206 passed, matching baseline exactly, zero
regression.** All 8 `AppChannelClient` tests (including `pins APP_CHANNEL_VERSION against the shared
cross-language channel version`) pass by name (pasted: `terra-exchange-test-os-verbose.txt`).

**`PluginRuntime/🟦️component.tsx`'s own in-source suite** (via the pre-existing scratch harness — see
testing note above; run from repo root):
```
$ bunx vitest run --config ".../terra-web-plugin-runtime-scratch/after.vitest.config.ts" --reporter=verbose
 Test Files  1 passed (1)
      Tests  26 passed (26)
EXIT: 0
```
All 26 pass by name, including both converted `fakeLease` tests under "PluginRuntime documentPack/
transaction wire adapter" (pasted: `terra-exchange-test-pluginruntime-scratch.txt`).

**`@semio-tech/framework-renderer-react`** (`🧪️index.test.ts` — NOT owned by this packet; see
Lease-requests for why 4 of its failures are mine to disclose, not mine to fix):
```
$ cd .../📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react && bun ./📜️script.ts test quick --reporter=verbose
 Test Files  1 failed (1)
      Tests  15 failed | 321 passed (336)
EXIT: 1
```
**11 of the 15 are the SAME 11 pre-existing failures `📌️important.md`'s own LATEST baseline already
records** ("react-renderer 325/336, 11 = exact subset of a 15-name baseline") — named: declarative forms
parity (selection ring), framework renderer hosts (virtual file system scenes), s workflow flow routing
(ShellFaultBoundary), window action panel ×3 (staging/gates/reset), registry-derived utilities (P5),
resolveCommands/commandCategories, shell option locks ×2 (ENTWERFEN_MIT_BESTAND/footer credits),
buildCommandCategoryTabs — none reference `PluginRuntime`, `AppChannelClient`, `exchange`, `enqueue`, or
`outcomes` anywhere in their name or failure output. **The other 4 are new, and are exactly and only**
`adaptPluginHandle`-via-`exchange`-shaped-fake tests — see Lease-requests below; every one fails with the
identical `TypeError: undefined is not an object (evaluating 'handle.outcomes[Symbol.asyncIterator]')`
inside `new AppChannelClient`, i.e. the foreign file's own fake handle, not anything in my owned files.
Full output: `terra-exchange-test-react-renderer.txt`.

**`tsc --noEmit`** against the 3 owned files (bare tsc, same noise caveat A3/H2's reports already
established — no dedicated typecheck target, `--all-targets`-equivalent doesn't exist for this repo's TS):
```
EXIT: 2, 1205 errors total
```
Only **1** of the 1205 mentions any symbol this packet touched (`enqueue`/`outcomes`/`TurnOutcome`/
`sendCommand`/`AppChannelClient`/`createTurnOutcomeBroadcast`) — and it is a **pre-existing** mismatch in
`ShellHost/🟦️component.tsx:2442` (registrar-adjacent, not owned) between kernel's raw `PluginWasmHandle`
and `PluginRuntime`'s OWN, differently-shaped, SAME-NAMED `PluginWasmHandle` (the "rich" adapted handle —
`handleAction`/`refreshUi`/etc., not `manifest`/`createApp`/`enqueue`); that assignment was already
structurally broken before this packet (it was missing `manifest`/`createApp`/`destroyApp`/`exchange`
too) — this packet only changed which 2 property names the diagnostic lists as missing. Not a new
regression. Full output: `terra-exchange-tsc-owned-files.txt`.

## Lease-requests

Both are real compile/runtime breaks caused directly by this packet's mandated rename
(`exchange` → `enqueue`/`outcomes`), in files outside this packet's owned paths. Neither was edited.

```lease-request
file: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts
reason: not owned (📺️renderer/** target package, not PluginRuntime/**). 4 tests construct a raw `exchange: async (...) => {...}` fake and pass it to `adaptPluginHandle` — all 4 now throw `TypeError: undefined is not an object (evaluating 'handle.outcomes[Symbol.asyncIterator]')` inside `new AppChannelClient`, since the fake no longer satisfies `KernelPluginWasmHandle`. Exact fix (same pattern already applied in `PluginRuntime/🟦️component.tsx`'s own two converted `fakeLease` tests and `💻️os/🟦️component.ts`'s `fakeHandle`): import `createTurnOutcomeBroadcast`/`TurnOutcome` from `@semio-tech/framework` (or the direct kernel path this file already imports other kernel types from), build one broadcast per fake handle, replace `exchange: async (id, frames) => {...; return outFrames;}` with `enqueue: (id, events) => {...; broadcast.push({instanceId: id, frames: outFrames});}` + `outcomes: broadcast.stream`. Four sites: line 1089 (`refreshUi` honest-empty test — the fake THROWS inside exchange/enqueue on purpose, so just rename the key), lines 1246/1273/1325 (three fakes that decode/reply for real — each needs the broadcast wiring). A 5th `exchange: async () => []` at line 1461 does NOT need touching — it feeds `resolveExternalSlots`, which never calls the handle at all (confirmed by reading `kernel/component.ts`'s `resolveExternalSlots`), so it's inert either way and that test already passes.
```

```lease-request
file: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🐚️plugin-bridge.ts
reason: not owned (📺️renderer/** wgpu target, not PluginRuntime/**). `loadPluginModule`'s `exchangeHandle: Pick<KernelPluginWasmHandle, "exchange">` (line 246) is now a TYPE ERROR — pasted: `error TS2344: Type '"exchange"' does not satisfy the constraint 'keyof PluginWasmHandle'` (line 246) and `error TS2345: ... Pick<PluginWasmHandle, "exchange"> ... missing ... enqueue, outcomes` (line 278, the `new AppChannelClient(exchangeHandle, ...)` call at line 278). No vitest config covers this file (no `.test.ts` in this directory), so this is a build/typecheck-only break, not a test regression — but real, and it will surface the first time this target actually builds. Fix: mirror `PluginRuntime/🟦️component.tsx`'s own `runQueuedTurn`/`turnOutcomes` conversion exactly — this file's `exchangeHandle` body (lines 246-265) is a near-verbatim copy of the old `PluginRuntime` `exchange` implementation this packet already converted, same `ShardEventEnvelope` mapping, same `shellFrameBytes`/`pendingTurnEffects`/`applyRetainedWindowPatches` demux. Full tsc output: `terra-exchange-tsc-plugin-bridge.txt`.
```

## Ownership / process compliance

No git-modifying command was run. No registrar file touched. `APP_CHANNEL_VERSION`/the shared
`channel-version.json` pin deliberately left unchanged (see above), not silently skipped. All scratch
verification output (`tsc`/vitest raw output) is `.txt` inside this ticket folder:
`terra-exchange-test-framework.txt`, `terra-exchange-test-framework-verbose.txt`,
`terra-exchange-test-kernel-verbose.txt`, `terra-exchange-test-os.txt`,
`terra-exchange-test-os-verbose.txt`, `terra-exchange-test-pluginruntime-scratch.txt`,
`terra-exchange-test-react-renderer.txt`, `terra-exchange-tsc-owned-files.txt`,
`terra-exchange-tsc-plugin-bridge.txt`. No `[DEBUG]`-prefixed line was added to source (existing ones
left untouched). Reused the pre-existing `terra-web-plugin-runtime-scratch/after.vitest.config.ts`
rather than building a new scratch harness. Did not touch `ShellHost/🟦️component.tsx` (registrar-
adjacent per `📌️important.md`, confirmed still untouched — its one tsc mention above is read-only
diagnosis, not an edit).
