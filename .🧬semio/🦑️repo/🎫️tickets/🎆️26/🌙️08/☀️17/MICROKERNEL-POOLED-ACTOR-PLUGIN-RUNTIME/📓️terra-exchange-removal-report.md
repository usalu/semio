# 📓️ terra-exchange-removal — report

## TL;DR

The `exchange` seam in the three path_scope files (`🎠️kernel/🟦️component.ts`'s `PluginWasmHandle`,
`💻️os/🟦️component.ts`'s `AppChannelHandle`/`AppChannelClient`, `PluginRuntime/🟦️component.tsx`'s
`adaptPluginHandle`/`loadPluginModule`) was **already fully removed before this packet started** —
landed in commit `cb9bcce7a4` (2026-08-20 00:52, both `🎠️kernel` and `💻️os` `🟦️component.ts`) as part
of channel v12. All three already use `enqueue(instanceId, events): void` +
`outcomes: AsyncIterable<TurnOutcome>` exactly as this packet's brief specified, with a real
`createTurnOutcomeBroadcast` multicast primitive backing `outcomes`.

What this packet actually found and fixed: a **downstream consumer test file**,
`💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
(the react-renderer package, which imports and exercises `PluginRuntime`'s `adaptPluginHandle`), still
built its test fakes against the deleted `exchange(instanceId, frames) -> Promise<frames>` shape,
cast past the type checker with `as unknown as Parameters<typeof adaptPluginHandle>[1]`. This was not
just stale documentation — it was **causing real runtime test failures**: `AppChannelClient`'s
constructor reads `handle.outcomes[Symbol.asyncIterator]()` eagerly, and the fake handles had no
`outcomes` property, throwing `TypeError: undefined is not an object (evaluating
'handle.outcomes[Symbol.asyncIterator]')` on every `adaptPluginHandle(...).createApp(...)` call. Fixed
by replacing the 5 `exchange:` fake properties with an `enqueue`/`outcomes` pair built on the same
`createTurnOutcomeBroadcast` primitive production code uses.

**The `APP_CHANNEL_VERSION` 12→13 bump specified in the task brief was deliberately NOT performed** —
see "On the version bump" below for why.

## What was verified already-done (no edit needed)

- `🎠️kernel/🟦️component.ts` (~line 104-234): `PluginWasmHandle` type has `manifest`/`createApp`/
  `destroyApp`/`enqueue`/`outcomes`/`dispose`. No `exchange` member. `TurnOutcome` type and
  `createTurnOutcomeBroadcast` (a real multicast-subscriber primitive, own inline vitest suite) both
  present and exported.
- `💻️os/🟦️component.ts` (~line 1786-2141): `AppChannelHandle = Pick<PluginWasmHandle, "enqueue" |
  "outcomes">`. `AppChannelClient` correlates replies FIFO against the handle-wide `outcomes` stream via
  its own `pumpOutcomes` background loop (one subscription per client instance, unsubscribed on
  `dispose()`). `APP_CHANNEL_VERSION = 12`, matching `protocol_channel::CHANNEL_VERSION` (Rust,
  `💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs:24`) and the shared fixture
  `💻️os/🧫️fixtures/📡️channel/channel-version.json` (`channelVersion: 12`) — all three still agree.
- `PluginRuntime/🟦️component.tsx` (~line 647-950): `loadPluginModule` builds a `KernelPluginWasmHandle`
  whose `enqueue`/`outcomes` are backed by its own `createTurnOutcomeBroadcast<TurnOutcome>()`
  (`turnOutcomes`, ~line 673); `adaptPluginHandle` wraps that handle in `AppChannelClient` per instance.
  No `exchange` member anywhere in the file (confirmed with `rg -a` — this file legitimately contains a
  `\0`-delimited map-key literal at one site, which makes plain `grep`/`rg` misclassify it as a binary
  file and silently report zero matches; **always pass `-a`/`--text` on this file**).
- `🧊️wgpu/🟦️typescript/🐚️plugin-bridge.ts`: confirmed **out of scope** per the brief (owned by
  `terra-wgpu-web`) and confirmed **live** — `git status` shows it modified/staged mid-session; a first
  grep early in this packet caught it still using `exchangeHandle`/`exchange:`, a later grep (after
  that peer's own edits landed) showed it already flipped to the same `enqueue`/`outcomes` shape,
  independently, by that packet. Not touched here either way.

## What was fixed

**File**: `💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
(not literally in the brief's named path_scope, but it is the test suite exercising the named
`PluginRuntime` consumer, its fakes were the last first-party `exchange` **members** left anywhere in
the repo, and fixing it was necessary to get an honest pass/fail count for the consumer this packet
owns — see "Scope reasoning" below).

- Added `createTurnOutcomeBroadcast`, `type TurnOutcome` to the existing `@semio-tech/framework` import
  (both are re-exported from `🎠️kernel/🟦️component.ts` via `export *` in `🧰️framework/📦️packages/
  🟦️typescript/🟦️glue.ts:13` — no new package dependency).
  ​- 
- Added a local helper `exchangeStyleChannel(respond)` inside the `describe("framework plugin
  runtime", …)` block that reconstructs `exchange`'s old synchronous request/reply shape on top of the
  two new primitives (`enqueue` calls `respond`, then pushes the result — or a thrown/rejected error —
  onto a `createTurnOutcomeBroadcast` whose `.stream` backs `outcomes`). This is purely a **test
  convenience**; nothing in production code has a synchronous responder to call this way.
- Replaced all 5 `exchange:` fake properties (4 inside `describe("framework plugin runtime")` via the
  helper, 1 standalone in `describe("framework external slots")` — that one is never actually invoked at
  runtime so got a minimal inline `enqueue`/`outcomes` stub instead of the helper, to avoid a
  cross-describe-block scoping issue) with `enqueue`/`outcomes`.
- Updated the header/inline comments in the same region that described the deleted `exchange` ABI as
  still current (they now describe the real `enqueue`/`outcomes` split, still naming `exchange` only in
  the historical/migration sense — those residual mentions are intentional prose, not members, and are
  what the acceptance grep below still turns up).
- Left `applyUiPatchToRetained`'s own `UiNode`-shaped tests (same `describe` block, different sub-block)
  **untouched** — their current failures are `snapshot.nodes` / `store.getState` errors from a live,
  uncommitted peer migration (`UiNode` → `UiSnapshot`/`UiNodeRecord`, ticket-unrelated,
  `PluginRuntime/🟦️component.tsx` and `UiDocumentStore/🟦️component.tsx` both show unstaged edits from
  that peer as of this report) — not this packet's concern, and fixing them would mean guessing at
  someone else's still-moving design.

Diff: 1 file, +55/−21 (`git diff --stat`).

## Scope reasoning: why the react-renderer test file, and why not further

The brief's `path_scope` names three files. This packet edited a fourth (the react-renderer test file)
because:
1. It is squarely "first-party TypeScript" implementing the `exchange` request/batched-reply seam the
   task is titled after — 5 live `exchange:` object members, not documentation.
2. It was causing **real, currently-failing tests** (`TypeError: undefined is not an object (evaluating
   'handle.outcomes[Symbol.asyncIterator]')`) directly inside the PluginRuntime consumer's own test
   coverage — leaving it broken would mean reporting a false "suite passes" for the very consumer this
   packet is responsible for.
3. It is `git`-clean (no live peer editing it) — confirmed via `git log`/`git status` before touching it.
4. The brief's own "Do NOT edit" list names exactly two exclusions (`🧊️wgpu/**`, the bridge generator);
   this file is neither.

Two other files still mention `exchange` in prose and were **deliberately left alone**, both because
they are outside path_scope AND currently owned by a live peer (touching either risks a merge collision
with in-flight work, which the ticket's rules treat as worse than a stale comment):
- `🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts:1235-1242` — a dangling/orphaned doc comment (no
  declaration follows it before `//#endregion`) claiming the WIT ABI is "`manifest`/`instantiate-app`/
  `exchange`" — stale, should say `enqueue`/`outcomes`. `git status` shows this file **staged**
  (uncommitted) with an unrelated in-flight change (`ActionId`/`Trigger` → `UiActionId`/`UiTrigger`
  rename, the same UI-contract migration mentioned above). Flagging for that peer/the coordinator rather
  than editing.
- `PluginRuntime/🟦️component.tsx` and `ShellHost/🟦️component.tsx` (~4 lines) — historical comments
  ("`PluginRuntime`'s `PluginWasmHandle` wraps the raw `exchange` ABI…") that are accurate as *history*
  but read as present-tense. `PluginRuntime/🟦️component.tsx` itself has unstaged live-peer edits right
  now (the same UI-contract migration); `ShellHost` was not touched because it sits well outside the
  named path_scope and none of these are live members. Low-value, low-risk residue — noted here rather
  than risking a collision.

## On the version bump (deliberately not done)

The brief specifies: *"bump `APP_CHANNEL_VERSION` 12 → 13 and update its cross-language pin test."*
This was **not performed**, for a concrete reason: there is no wire-format change left to justify it.

- `protocol_channel::CHANNEL_VERSION` (Rust, `💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs:24`) is
  **12**, unchanged.
- The shared pin fixture (`💻️os/🧫️fixtures/📡️channel/channel-version.json`) is **12**, unchanged.
- `APP_CHANNEL_VERSION` (TS) is **12**, unchanged, and its own docstring already states: *"Channel v12
  retired the `hello()`/`refreshUi()`/`attachBackbone()`/`detachBackbone()`/`drain()` surface... UI
  updates are a `UiPatch` push, and guests wake on events/timers/`next-wake`"* — i.e. the
  `enqueue`+`outcomes` shape this brief describes as the *target design* is already documented as
  channel v12's own payload, landed by an earlier packet.

Bumping only the TS constant to 13 with no matching Rust change would immediately fail the pin test
(Rust=12 vs fixture=12 vs TS=13) for no wire-format reason, and editing the Rust side is both outside
this packet's named path_scope and outside its explicit "No Rust build contention" instruction. Given
the seam is already fully removed, there is nothing left to bump a version *for*. Treating this as a
"stop and report" item per the ticket's own escalation rule rather than performing a bump that would
only break a currently-passing test.

## Acceptance evidence

**1. Grep-confirm no `\bexchange\b` member remains in first-party TS** — repo-wide, `.ts`/`.tsx`,
excluding `node_modules`/`dist`/`🤖️generated`/the ticket tree:

```
rg -a -n -i '\bexchange\b' . -g '*.ts' -g '*.tsx' -g '!node_modules' -g '!*/dist/*' \
   -g '!*/🤖️generated/*' -g '!.🧬semio/**'
```

Zero live members. Every remaining hit is prose in one of the four files discussed above (2 lines in
`🛂️manifest`, 3 in this packet's own updated test-file comments, 3 in `🧊️wgpu`'s bridge — peer-owned,
now also comment-only — 4 in `ShellHost`). **Note for whoever re-runs this**: plain `grep`/`rg` without
`-a`/`--text` silently report zero matches on `PluginRuntime/🟦️component.tsx` (misclassified as binary
because of one legitimate `\0` map-key-separator literal at byte offset ~40235, confirmed with
`python3` — not corruption) — always pass `-a`.

**2. `APP_CHANNEL_VERSION` pin test** — passes, at its current value 12 (see "On the version bump"):

```
cd 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript && bunx vitest run --config 🧪️vitest.config.ts \
   -t "pins APP_CHANNEL_VERSION"
→ Test Files  1 passed | 2 skipped (3)
  Tests  1 passed | 206 skipped (207)
```

**3. Real pass/fail counts, TS suites covering the three path_scope files** (all run directly via
`bunx vitest run`, bypassing the `nx` task queue — see "Environment note" below for why):

| package | command dir | result | notes |
|---|---|---|---|
| `@semio-tech/framework` (kernel) | `🧰️framework/📦️packages/🟦️typescript` | **87 passed / 0 failed** | unchanged — matches this ticket's own recorded baseline exactly; file not edited this packet |
| `@semio-tech/framework-os` | `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript` | **206 passed / 1 failed** (207 total) | unchanged — matches this ticket's own recorded baseline exactly (`💻️os/…/🟦️typescript 206 / 1`); the 1 failure is the pre-existing, already-routed `matches the Rust plan_workflow … decoded via wasm` (wasm pkg build broken by a `RUSTFLAGS`/`getrandom_backend` conflict, per `📌️important.md`'s own note under the W5 baseline table) — not related to this packet |
| `@semio-tech/framework-renderer-react` | `…/PluginRuntime`'s consumer package, `…/🎯️targets/⚛️react` | **409 passed / 28 failed** (437 total) | see below |

React-renderer detail: **before** this packet's fix, 4 of the 34-then-failing tests threw
`TypeError: undefined is not an object (evaluating 'handle.outcomes[Symbol.asyncIterator]')` — the
`exchange`-fixture bug. **After** the fix, that error class is gone (confirmed absent across 4 separate
runs, both via `nx` and directly via `vitest`, `grep`-checked each time). The suite's total failure
count is **flaky independent of this packet's change** — three back-to-back `nx`-driven runs before any
further edits returned 34, then 91, then 29 failures, with the swing entirely inside one describe
block (`Interpreter/🟦️component.tsx`'s "conformance corpus", a shared-fixture-load race — textbook
rule-11 "passes-alone-fails-in-suite means shared global state", not attributed to this packet). Two
direct `vitest` runs (bypassing `nx`) after the fix landed were mutually consistent at **28
failed / 409 passed**, itemized and all attributable to causes outside this packet:
- ~15: `TypeError: … reading 'nodes'` / `store.getState is not a function` / `uiTreeNodeToTreePanelConfig
  is not a function` — the same live, uncommitted `UiNode` → `UiSnapshot`/`UiNodeRecord` peer migration
  named above (`PluginRuntime/🟦️component.tsx` and `UiDocumentStore/🟦️component.tsx` both show unstaged
  edits mid-session).
- ~5: content/copy drift unrelated to any channel work (`"Artifact"` vs `"Document"` label, asset path
  `…/logo/` vs `…/logos/…`, an `os.resetDock` command-descriptor shape change).
- ~5: DOM/testing-library setup issues (`Invalid Chai property: toHaveTextContent`, `Unable to fire a
  "change" event`, a null DOM ref) — pre-existing test-harness gaps, not wire-protocol related.
- `Errors 2`: an unrelated unhandled `postMessage` rejection inside `🟦️backbone-worker.ts`, present
  identically before and after this packet's edit.

None of the 28 residual failures reference `exchange`, `enqueue`, `outcomes`, or `AppChannelClient`.

## Environment note (not a finding, for the next packet's context)

Mid-packet, `bun nx run @semio-tech/framework-os:test` sat with **zero output for 10+ minutes**; `ps aux`
showed the identical `nx run @semio-tech/framework-os:test` command line running as **260-300+ separate
processes simultaneously** (climbing over the observation window), system load average ~19, ~800% CPU,
~83% memory — consistent with this being a heavily-multi-session moment on this ticket, not a bug in
this packet's own work. Running `bunx vitest run --config 🧪️vitest.config.ts` directly inside the
package directory (bypassing the `nx` task queue entirely) got a clean answer in ~1s. Recorded here in
case another packet hits the same wall — this is a viable, honest way to get real numbers when `nx`
itself is the bottleneck, not a shortcut around the acceptance requirement.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
  (edited — the only file this packet modified)

## Files inspected, not touched (with reason)

- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` — already correct, no `exchange`
- `🧰️framework/🛍️products/💻️os/🟦️component.ts` — already correct, no `exchange`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx` —
  already correct, no `exchange`; also currently has unrelated live-peer uncommitted edits (UI-contract
  migration), so left alone beyond confirming it
- `🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts` — stale `exchange` doc comment (prose only), but
  file is staged/uncommitted by a live peer right now; flagged above, not edited
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx` —
  stale `exchange` doc comments (prose only), outside named path_scope; flagged above, not edited
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🐚️plugin-bridge.ts` —
  explicitly out of scope (`terra-wgpu-web`); observed mid-migration by that packet during this session,
  now also clean
