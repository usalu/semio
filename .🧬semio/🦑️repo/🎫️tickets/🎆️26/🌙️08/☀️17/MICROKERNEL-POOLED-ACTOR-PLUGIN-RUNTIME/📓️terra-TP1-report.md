# terra-TP1-async-glue — report

## delivered

All five utilities added to the single owned file `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`, each in its own `//#region`/`//#endregion`, docstrings starting with a unique emoji (English first, German second sentence where the docstring describes user/dev-facing semantics):

1. **`createBoundedMailbox<T>`** (`//#region 📬️BoundedMailbox`) — TS twin of Rust `Mailbox` in `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`'s `📬️Mailbox` region (read first, semantics mirrored exactly: 4-lane array `["interactive","userVisible","background","maintenance"]` matching `Lane::ALL`/`priority_rank`, latest-wins coalescing scan within the incoming lane preserving queue position, bounded-ring eviction searching from lowest-priority lane down to (but excluding) the incoming lane, hard `rejected` when nothing lower exists). Exports `Lane`, `CoalesceKey`, `Backpressure` (`accept | coalesced | dropped(lane) | rejected`), `MailboxEnvelope<T>`, `BoundedMailbox<T>`, `createBoundedMailbox`.
   - Deliberately did **not** import the pre-existing ts-rs-generated mirror at `🧰️framework/🔨️modules/🎭️actor/🤖️generated/🟦️actor.ts` (registrar-only `🤖️generated/**`, and its `Backpressure` codegen is malformed for the tuple variant — `{ "kind": "dropped" } & Lane` — not a real intersection with a string-literal type). Built a fresh, correct, self-contained TS twin instead, scoped only to my owned file.
2. **`retryWithJitteredBackoff`** (`//#region 🔁️RetryWithJitteredBackoff`) — full-jitter backoff (`minMs + random()*(cap-minMs)`, `cap = min(maxMs, minMs*2^attempt)`), loops until success or `AbortSignal` abort (including already-aborted), every timer/listener cleaned up via a local `abortableDelay` helper on every exit path.
3. **`latestWins`** (`//#region 🥇️LatestWins`) — single-flight + at-most-one trailing follow-up; concurrent callers during an in-flight run all share the one queued follow-up's promise.
4. **`fetchWithTimeout`** (`//#region ⏱️FetchWithTimeout`) — composes an internal `AbortController` (timeout) with the caller's `AbortSignal`; returns `FetchTimeoutResponse`, a locally-declared structural interface (`ok`/`status`/`statusText`/`headers.get`/`json`/`text`) — no ambient `Response` type leaks through the exported signature; timer + external-signal listener cleaned up in a `finally`.
5. **`waitForEvent<T>`** (`//#region 🔔️WaitForEvent`) — resolves on first delivered value, rejects on abort (including pre-aborted), unsubscribes on both exit paths.

Tests added inline in the existing `if (import.meta.vitest)` block (this package's test convention — `includeSource: ["🟦️glue.ts"]`), one `describe`/region per utility: `🔖️BoundedMailboxTests`, `🔖️RetryWithJitteredBackoffTests`, `🔖️LatestWinsTests`, `🔖️FetchWithTimeoutTests`, `🔖️WaitForEventTests`. Added `vi` to the existing `const { describe, expect, it } = import.meta.vitest;` destructure (now includes `vi`) since fake timers / spies / `vi.stubGlobal` are needed for the backoff and fetch tests.

No other files touched.

## commands + exit codes

Test command (verbatim, run from repo root — later packets can reuse this):
```
cd "/Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript" && bun ./📜️script.ts test
```
This is the `bun ./📜️script.ts test` router → `nx:run-commands` `test` target in `📋️project.json`, equivalent to `bun nx run @semio-tech/framework:test`. It runs vitest (`🧪️vitest.config.ts`, `environment: "node"`, `includeSource: ["🟦️glue.ts"]`) against the whole file's inline test suite (there is no separate `*.test.ts`).

Final run output:
```
 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript

 Test Files  2 passed (2)
      Tests  182 passed (182)
   Start at  18:41:43
   Duration  428ms (transform 463ms, setup 0ms, import 541ms, tests 139ms, environment 0ms)

EXIT_CODE=0
```
(The harness reports the single `🟦️glue.ts` suite twice — "2 passed" test files / 182 = 91×2 — this doubled-reporting is pre-existing behavior of this package's vitest setup, unrelated to this change; see baseline below.)

Also ran a whole-repo type-check to confirm the new code introduces zero type errors:
```
bun x tsc --noEmit -p tsconfig.json
```
Exit code 1, but **zero** of the 19 reported errors reference `🟦️glue.ts` — all 19 are pre-existing syntax errors in unrelated files (`✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/🟦️component.ts`, two `stdio` plugin schema files, and `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️extension.ts`) — plausibly other live sessions' in-progress edits per the "concurrent cargo workspace churn" pattern, not anything this packet touched.

## baseline vs after

Could not use `git stash`/checkout to snapshot the pre-change file (forbidden — live peers + auto-commit). Baseline established by inspection instead: the diff to `🟦️glue.ts` is purely additive (5 new regions + 5 new test `describe` blocks + the `vi` destructure addition); no existing line was changed or removed. First full run (before fixing one bug of my own, see below) already showed **all 76 pre-existing unique tests passing** (152/152 counting the doubled report) — only my own new `latestWins` test failed, and only because of a bug in my own new code (see below), never a pre-existing failure. After the fix: 91 unique tests (76 pre-existing + 15 new) × 2 (doubled report) = 182/182 passing, exit 0.

Bug found and fixed during my own test run (not a pre-existing baseline issue): `latestWins`'s `launch()` initially wrapped `run()` in `Promise.resolve().then(run)`, deferring the call to a microtask — this broke the "single-flight" synchronous-launch assumption (my own test asserted `calls === 1` immediately after the first `trigger()` call, and observed `0`). Fixed by calling `run()` synchronously inside a `try`/`catch` (`Promise.resolve(run())`, catching a synchronous throw into `Promise.reject(error)`), which is also a strictly better implementation (matches Rust's own eager-dispatch mailbox semantics style) than the deferred version.

## lease-requests

none

## honest gaps

- `createBoundedMailbox` intentionally omits `deadline_ms`/`cancel_of`/`seq` (present on Rust's `Envelope`) — the ticket packet only asked for lane priority, coalescing, and bounded-ring backpressure; `earliest_deadline()`-style preemption wasn't in scope and isn't wired here. A later packet that needs deadline-aware scheduling will need to extend `MailboxEnvelope<T>`.
- `retryWithJitteredBackoff` has no `maxRetries`/attempt cap by design (the packet spec only lists `minMs`/`maxMs`/`signal`) — it retries indefinitely until success or abort. If a later packet needs a bounded retry count, that's an additive option, not present yet.
- `fetchWithTimeout`'s `FetchTimeoutResponse` only surfaces `ok`/`status`/`statusText`/`headers.get`/`json`/`text` — no `body`/`arrayBuffer`/`blob`/`redirected`/`url`. Kept intentionally minimal (structural, no external type import); callers needing more will need the interface widened later, non-breaking since it's additive.
- Did not wire `🎭️actor`'s ts-rs-generated `Lane`/`CoalesceKey`/`Backpressure` mirror into `glue.ts`'s exports (that module isn't currently re-exported from `glue.ts` at all) — out of scope for this packet and a bigger call than mine to make; flagging for whichever later packet actually needs the real actor-module wire types alongside this TS-side mailbox twin, since both now separately define very similar `Lane`/`CoalesceKey`/`Backpressure` names (different module, same shape) — worth a look at name-collision risk when that module eventually does get exported through `glue.ts`.

## coordinator follow-up

Both findings addressed. Nothing else touched.

### Finding 1 — Lane casing fixed by importing the real wire type instead of redeclaring

Confirmed by re-reading `🎭️actor/🤖️generated/🟦️actor.ts` (now present, `Lane = "Interactive" | "UserVisible" | "Background" | "Maintenance"`, `CoalesceKey = string`, and yes — `Backpressure = { "kind": "dropped" } & Lane`, unusable, exactly as flagged). Rather than hand-fixing my own `Lane`'s casing (which would just recreate the same "two independently-typed near-duplicates" risk one indirection later), the new mailbox file now does `import type { Lane, CoalesceKey } from "../../🤖️generated/🟦️actor.ts";` and re-exports both unchanged — so it can never drift from the wire again. Used the repo's dominant `*.ts`-extension import-specifier convention for generated mirrors (`🛂️manifest/🟦️component.ts`, `🔺️mesh/🟦️component.ts`, `🎠️kernel/🟦️component.ts`, `🖼️assets/…/🟦️component.ts` all do this); `🎭️actor/🟦️component.ts`'s own `"./🤖️generated/🟦️actor.js"` is the sole outlier in the codebase and I did not touch it (outside the two files I was told to touch). `Backpressure` stays locally declared (`{kind:"dropped"; lane: Lane}` etc.) with a docstring on the import block explaining exactly why it isn't taken from the mirror. Mailbox test fixtures now use the PascalCase wire values (`"Interactive"`/`"Background"`/`"Maintenance"`) throughout.

### Finding 2 — mailbox relocated

- Moved `Backpressure`/`MailboxEnvelope<T>`/`BoundedMailbox<T>`/`createBoundedMailbox` (plus `Lane`/`CoalesceKey` re-exports and all 4 mailbox tests) out of `🟦️glue.ts` into new file `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📬️mailbox.ts`, beside `🧵️shard-client.ts`.
- `🟦️glue.ts` now carries only the four generic utilities (`retryWithJitteredBackoff`, `latestWins`, `fetchWithTimeout`, `waitForEvent`) plus their tests — the `📬️BoundedMailbox` region and `🔖️BoundedMailboxTests` block are gone from it entirely; grepped afterward to confirm zero leftover references to `createBoundedMailbox`/`BoundedMailbox`/`MailboxEnvelope`/`Lane`/`CoalesceKey`/`Backpressure` remain in `glue.ts`.
- **Vitest config did need editing to pick up the new sibling file** (confirming the coordinator's suspicion): `🎭️actor/📦️packages/🟦️typescript/🧪️vitest.config.ts`'s `test.include`, `test.coverage.include`, and `test.includeSource` each listed only `["🧵️shard-client.ts"]` — a new sibling `.ts` file is not auto-discovered by this per-package vitest setup (it's not a glob, it's an explicit array of filenames). Added `"📬️mailbox.ts"` to all three arrays. Left `resolve.alias`/`package.json`'s `exports` (both point only at `🧵️shard-client.ts`, the package's public entrypoint) untouched — not asked for, and widening the package's public surface wasn't part of this follow-up's scope.

### commands + exit codes (verbatim)

```
cd "/Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript" && bun ./📜️script.ts test
```
```
 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript

 Test Files  2 passed (2)
      Tests  174 passed (174)
   Start at  18:50:44
   Duration  385ms (transform 417ms, setup 0ms, import 496ms, tests 108ms, environment 0ms)

EXIT_CODE_GLUE=0
```
(174 = 87 unique × 2, this package's pre-existing doubled-report quirk; 87 = the original 91 minus the 4 mailbox tests that moved out.)

```
cd "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript" && bun ./📜️script.ts test
```
```
 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript

 Test Files  4 passed (4)
      Tests  38 passed (38)
   Start at  18:50:45
   Duration  235ms (transform 122ms, setup 0ms, import 127ms, tests 83ms, environment 0ms)

EXIT_CODE_ACTOR=0
```
Ran again with `bun x vitest run --config 🧪️vitest.config.ts --reporter=verbose` to positively confirm the 4 new mailbox tests actually executed here (not silently skipped by a path/glob mismatch) — all 4 `createBoundedMailbox > …` cases appear by name in the verbose log (doubled, ×2, same harness quirk as the base package), alongside `🧵️shard-client.ts`'s pre-existing 15 (×2 = 30). 8 + 30 = 38, matches. One pre-existing `[DEBUG] shard 0 worker error` stderr line appears — that's `🧵️shard-client.ts`'s own "ShardClient worker crash" test intentionally triggering and logging a simulated error; unrelated to this change.

Also re-ran the whole-repo `bun x tsc --noEmit -p tsconfig.json` after both edits: byte-identical error output to the pre-follow-up run (still exactly the same 19 pre-existing, unrelated errors in `🔱️trinity`/`stdio` plugins and the vscode extension; none in `glue.ts`, `📬️mailbox.ts`, or either `🧪️vitest.config.ts`).

### lease-requests

none — both touched files (`📬️mailbox.ts` new file, `🎭️actor/📦️packages/🟦️typescript/🧪️vitest.config.ts`) were named explicitly by the coordinator's follow-up instructions.

### honest gaps (follow-up)

- Did not add `📬️mailbox.ts` to `🎭️actor/📦️packages/🟦️typescript/package.json`'s `exports` or the vitest `resolve.alias`, since the coordinator's instructions specified only the file move + test discovery, not widening the package's public entrypoint. T-P4's `turn-scheduler.ts` (same package) can reach it via a plain relative import (`./📬️mailbox.ts`) without that; if a later packet outside this package needs to import the mailbox directly through `@semio-tech/framework-actor`, that's a one-line addition someone should make deliberately, not as a side effect of this move.
