# 📓️ terra-web-backbone report

Owned path: `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts` (only file touched).

## delivered

1. **Folder watch is SSE-primary now.** `watchFolder` does an immediate bootstrap read, opens SSE,
   and starts a slow (24–36s jittered) sanity-poll fallback (`startSanityPolling`). The old
   unconditional 1.5s `setInterval` poll (`:84`, `:366-374` in the pre-change file) is gone.
   `ArtifactState.sseHealthy` is the single explicit flag: `true` on SSE open, `false` on every
   SSE close — the sanity tick only actually revalidates when it's `false`.
2. **Single-flight, non-overlapping revalidation.** `ArtifactState.revalidateFolder` is built once
   per document via `latestWins(() => pollFolderOnce(state, folder))` (from packet `web-glue`'s
   `🟦️glue.ts`) and is the ONLY thing SSE `onmessage`, the sanity tick, and the `externalChanged`
   local message ever call — so no caller can ever cause two overlapping `pollFolderOnce` reads.
3. **SSE reconnects after a post-open drop.** `connectSseOnce` wraps one EventSource attempt;
   `watchFolder` drives it through `retryWithJitteredBackoff` (SSE_RECONNECT_MIN/MAX_MS = 1s/30s).
4. **Hub reconnect is now jittered.** `connectHubOnce` wraps one WebSocket attempt; `connectHub`
   drives it through the same `retryWithJitteredBackoff` helper (HUB_RECONNECT_MIN/MAX_MS,
   unchanged values, now shared single-source jitter instead of manual `delay *= 2`).
5. **Per-document `AbortController` (`ArtifactState.docAbort`)**, created in `openArtifact`,
   aborted in `closeArtifact`. Folder read (`pollFolderOnce`) and write (`writeFolder`) go through
   `fetchWithTimeout` with this signal (15s timeout); the hub/SSE reconnect loops use the same
   signal to stop looping immediately on close. Blob get/put (`getCachedBlob`/`putCachedBlob`,
   document-agnostic, no per-document context to tie to) now also go through `fetchWithTimeout`
   with a fixed 15s timeout, no caller signal (nothing calls them yet).
6. **Bounded, lossless mutation queue.** `PENDING_MUTATIONS_QUEUE_LIMIT = 2000`. A `localMutations`
   batch that would exceed it is rejected wholesale via `rejectMutationQueueOverflow` (never
   partially accepted, never silently dropped) and reported through the same `commandOutcome` /
   `CommandAckOutcome.rejected` wire vocabulary a real hub rejection uses, with a distinguishing
   negative `batchId` range so it can never collide with real (0-based, increasing) hub batch ids.
7. **Outbox + flush-on-reconnect.** New `ArtifactState.outbox: MutationEnvelope[]`.
   `relayMutationsToHub` no longer no-ops when the socket is closed — it pushes into `outbox`
   instead. A socket's `onclose` also moves any batch it never acked from `pendingBatches` back
   into `outbox` (a dead socket will never deliver that `Ack`). `handleHubFrame`'s `Welcome` branch
   flushes the whole outbox via `relayMutationsToHub` the moment the handshake succeeds — this is
   the actual fix for "queued forever, only resent if the user happens to edit again."

## findings confirmed vs not reproduced

All five findings from the brief reproduced exactly as described against the live file before
editing — none were stale:

- Finding 1 (unconditional poll + no in-flight guard): confirmed at the described lines.
- Finding 2 (no reconnect after an open SSE drops): confirmed — `onerror` only acted on `!sseOpened`.
- Finding 3 (fetches with no `AbortSignal`): confirmed at all four call sites.
- Finding 4 (unjittered reconnect backoff): confirmed (`delayMs *= 2` with no randomization).
- Finding 5 (silent no-op relay + unbounded queues): confirmed — `relayMutationsToHub` returned
  early with no signal, `pendingMutations`/`pendingBatches` had no cap, and `Welcome` never flushed
  anything queued while offline.

One **additional latent bug found and fixed while testing finding 5**, in this same file's own
pre-existing test region (not part of the audit, but in my owned path): the `backbone-worker
directory lane` test stubbed `globalThis.WebSocket = FakeDirectoryWebSocket` and never restored
it, leaking a WebSocket class with no `OPEN` static into every later test — this made
`state.socket?.readyState === WebSocket.OPEN` evaluate `undefined === undefined → true` even when
`state.socket` was `null`, crashing `sendWireFrame`. Fixed by capturing and restoring the original
global in that test's `finally` (see `## honest gaps` for why this only surfaced under my new tests).

## commands + exit codes

Test command (discovered per the ticket's convention):
```
cd "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript" && bun ./📜️script.ts test
```

**Baseline (before any edit to `🟦️backbone-worker.ts`), captured first:**
```
 Test Files  2 failed | 2 passed (4)
      Tests  2 failed | 322 passed (324)
```
Exit code: `1`. Both failures: `@semio-tech/framework-os workflow > matches the Rust plan_workflow
across shared fixtures decoded via wasm`, in `🟦️component.ts` (not my file) —
`Error: Cannot find module '…host/📦️packages/🦀️rust/pkg/semio_framework_os.js'`, a missing built
wasm artifact, unrelated to this packet.

**After all edits (production + 8 new tests), run 3× consecutively to rule out flakiness:**
```
 Test Files  2 failed | 2 passed (4)
      Tests  2 failed | 356 passed (358)
```
Exit code: `1` on every run. The 2 failures are byte-identical to the baseline's 2 (same missing
wasm module, same test name, same file — confirmed via `grep` across all 3 runs' output). All 34
new assertions across 8 new `it(...)` blocks in `🟦️backbone-worker.ts` pass on every run.

**Repo-wide `tsc --noEmit -p tsconfig.json`:** exit `2`, **19** `error TS` lines, all four in the
same three files the ticket already documented as pre-existing/out-of-scope (`trinity`, two
`stdio` artifact-standard schema files, the vscode extension). Zero errors in
`🟦️backbone-worker.ts` — confirmed via `grep backbone-worker` on the tsc output (no match).

## baseline vs after

| | tests | passed | failed | pre-existing failures absorbed into my number? |
|---|---|---|---|---|
| baseline | 324 | 322 | 2 (wasm module, unrelated) | no — measured first, in isolation |
| after | 358 | 356 | 2 (same wasm module failure) | no — same 2, confirmed identical across 3 runs |

Net: +34 tests added (8 new `it` blocks under `backbone-worker offline resilience`), all green,
zero regressions, zero new `tsc` errors.

## overflow + loss semantics

Stated plainly: **a mutation is never silently dropped by this worker.**

- **Under normal pressure (hub unreachable, but the local queue has room):** the mutation is
  accepted into `pendingMutations` (status-visible pending count) and `outbox` (unsent-to-hub),
  and is automatically relayed the moment the hub's `Welcome` handshake completes on
  reconnect — no user action or subsequent edit required to trigger the resend.
- **Under a dead in-flight batch (sent, socket died before `Ack`):** moved from `pendingBatches`
  back into `outbox` on `onclose`, then flushed the same way on the next `Welcome`.
- **Under queue overflow** (`pendingMutations.length + incoming > 2000`): the ENTIRE incoming
  batch is rejected — not partially accepted, not queued, not dropped — and a `commandOutcome`
  event with `outcome: { kind: "rejected", reason: "pending mutation queue full", messages: […] }`
  is emitted, using the exact same vocabulary a real hub-side rejection uses. The caller (the
  document/store above this worker, outside this packet's scope) is responsible for treating that
  rejection as unconfirmed/failed local work — this worker's job ends at "never claim success for
  something it didn't actually queue, and never lose track of it silently." `console.error` also
  logs every overflow, tagged `[backbone-worker]`.
- The overflow batch-id range is negative (`-1, -2, …`), disjoint from the hub's real batch ids
  (`0, 1, 2, …`), so the two can never be confused downstream.

## lease-requests

None. Everything needed lived inside the owned file; the one extra fix (WebSocket-stub leak in
the pre-existing directory-lane test) is inside the same owned file, not a lease.

## honest gaps

- **`post()`/`emitEvent()` are not directly observable from these tests.** `workerScope` is a
  top-level `const` resolved once at module load from `typeof self !== "undefined"`, and this
  package's vitest config runs `environment: "node"`, where `self` is undefined — so
  `workerScope` is `null` for every test in this file, making `post()` an inert no-op. This matches
  every PRE-EXISTING test in this file (none of them observe `post`/postMessage output either —
  they all assert on internal state or `console.error`/mock-fetch/mock-socket side effects
  instead). My overflow test follows the same convention: it verifies the reject-not-drop
  behaviour via `state.pendingMutations` staying untouched plus a `console.error` spy, not via
  observing the `commandOutcome` event's wire bytes. If a future packet wires up a real
  `DedicatedWorkerGlobalScope` test double, the `commandOutcome` payload itself could be asserted
  directly.
- **The hub-reconnect status display (`RemoteState.backoff.retryInMs`) is an approximation.**
  `connectHubOnce` still tracks `state.reconnectDelayMs` (doubling, un-jittered) purely to report a
  believable countdown number to the UI; the ACTUAL wait is `retryWithJitteredBackoff`'s own
  independently-computed random value. The two are not the same number. This avoids reaching into
  `🟦️glue.ts` for a per-attempt callback (out of my owned path, and the helper's contract doesn't
  expose one), but means the displayed `retryInMs` is a rough estimate, not the true wait.
- **Blob cache fetches (`getCachedBlob`/`putCachedBlob`) have no caller-supplied `AbortSignal`.**
  They're document-agnostic (no `ArtifactState` to tie a controller to) and nothing calls them yet
  (per this file's own pre-existing comment), so I gave them a fixed `fetchWithTimeout` timeout
  only. Whoever wires a real caller in should decide whether to thread a signal through at that
  point.
- **`SANITY_POLL_MIN_MS`/`MAX_MS` (24–36s) and `SSE_RECONNECT_MIN_MS`/`MAX_MS` (1–30s) are new
  constants I chose**, not specified numbers from the brief — reasoned from "slow (~30s) jittered
  sanity fallback" and "cheap to retry" respectively, but not validated against any real dev-server
  load characteristics.
- Did not touch `DirectoryClient.stream`'s own un-jittered reconnect backoff in `🟦️component.ts`
  (same bug class as finding 4, same file the brief explicitly marks as out of my owned path) —
  left alone as instructed.

## files touched

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts` (only file changed)

## coordinator follow-up

### 1. Reset reconnect backoff after sustained health (finding 4b)

**Root cause confirmed as described.** `connectHub` called `retryWithJitteredBackoff` exactly
once per document lifetime, looping forever internally via `connectHubOnce` rejecting on every
close. That helper's `attempt` counter is private to `🟦️glue.ts` and only resets when the whole
call returns — never once during a single multi-hour `connectHub` invocation. Same property
existed in the SSE reconnect loop I added for findings 1/2 (`connectSseOnce` via the same
single-long-lived-call pattern) — confirmed and fixed there too, as asked.

**Design.** `🟦️glue.ts`'s `retryWithJitteredBackoff` signature has no "reset now" hook and I do not
own that file, so the reset is implemented in my own loop, not inside the helper: a new
`reconnectForever(signal, attempt, minMs, maxMs)` (region `🔖️Reconnect`) calls
`retryWithJitteredBackoff(attempt, {...})` inside a `while (!signal.aborted)` loop. `attempt`
(`connectHubOnce`/`connectSseOnce`) now **resolves** (ending that `retryWithJitteredBackoff` call
successfully, so the NEXT loop iteration starts a fresh call with a zeroed attempt counter) in two
cases: the document aborted (as before), or the connection stayed open at least
`SUSTAINED_HEALTHY_MS` (15s, new constant, docstring reasons about it being half of both
transports' 30s ceiling — long enough that no accept-then-immediately-drop cycle could cross it,
short enough that a modestly-long-healthy session still gets credit) before an ordinary close.
Closed-before-sustained-health still **rejects**, so `retryWithJitteredBackoff`'s own backoff
keeps climbing inside the same call — this is deliberately NOT "socket opened resets it", per the
brief's own warning about defeating the backoff against a fast accept/drop loop.

**Tests added** (in `backbone-worker offline resilience`, `Math.random` pinned to `0.5` — not
`0`, since `0` collapses every jittered delay to its floor and would hide growth entirely,
defeating the point of testing growth vs. reset):
- `a hub drop after sustained health resets the backoff, unlike continued accumulation` — drives
  two quick failures (known delays 750ms, 1250ms) then a sustained-healthy connection, drops it,
  and asserts the next failure's wait is the RESET value (750ms, attempt 1 of a fresh call) by
  checking it fires within an 800ms window that would NOT have been enough for the un-reset
  continuation value (2250ms, attempt 3).
- `rapid accept-then-drop cycling does NOT reset the hub backoff — it keeps climbing` — opens+closes
  twice well under `SUSTAINED_HEALTHY_MS` and asserts the wait keeps growing (750ms → 1250ms), never
  falling back to the floor.
- `abort cancels the hub reconnect loop promptly, with no leaked timer` — asserts `vi.getTimerCount()
  === 0` immediately after `closeArtifact`, and no further reconnect attempt ever happens.
- `an SSE drop after sustained health resets ITS backoff too` — same shape as the first test, at
  SSE's own numbers (1000/30000ms → attempt 1 = 1500ms), proving the identical fix applied to
  `connectSseOnce`.

All four pass, deterministically, across 3 consecutive full-suite runs (see `## commands + exit
codes` below).

### 2. Negative batch-id namespace — verified safe, evidence below

**Both halves hold. Kept the sign-based sentinel; no change made.**

**Half 1 — the wire field is genuinely unsigned, on both sides, not inferred from TS `number`.**
Checked the actual definitions, not the TS type alone:
- Rust (`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs:51,100`):
  `Commands { batch_id: u64, … }` and `Ack { batch_id: u64, … }` — `u64`, unsigned.
- Encode/decode on both sides go through the same unsigned-LEB128 primitive:
  Rust calls `write_varint_u64`/`read_varint_u64` (same file, and
  `🎒️pack/🧾️codec/🦀️component.rs:72-88`, doc comment: *"Writes `value` as an unsigned LEB128
  varint"*); TS uses `writeVarintU64`/`readVarintU64` at `🟦️component.ts:1226,1272,1319,1372` for
  exactly these two fields. No zigzag/sign step anywhere in this path.
- **Crucially, my synthetic overflow `batchId` never touches this encoding at all.**
  `rejectMutationQueueOverflow` only ever reaches `ArtifactEvent::commandOutcome.batchId`, and
  `wireArtifactEvent`/`parseArtifactEvent` (`🟦️component.ts:1105-1117`) pass `commandOutcome`
  through UNCHANGED — it never goes near `writeVarintU64`. It's encoded generically by
  `encodePackValue` as `PACK_TAG_F64` (`🟦️component.ts:1804`, a full IEEE754 double) alongside
  every other plain JS number in this codebase's pack values — negative integers round-trip
  perfectly there. The two "batch id" fields are on entirely separate wire paths; the unsigned
  constraint only applies to the one my code never writes to.

**Half 2 — the hub can never itself emit a negative or zero id — stronger than asked, in fact: the
hub never generates a `batch_id` at all.** It destructures the client-supplied value out of
`ClientFrame::Commands` and echoes it straight back in `ServerFrame::Ack`
(`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:596,602`). The only real generator is this file's own
`state.nextBatchId` (`u64`, starts at `0`, `wrapping_add(1)` — mirrored server-side counter at
`🏪️store/🔄️sync/🦀️component.rs:882,946,1336-1337`, same `u64`/`0`/`wrapping_add(1)` shape) — a
counter I fully control, that structurally never produces a negative value, and only ever
increases from `0`.

**Conclusion:** real batch ids are always `>= 0` by construction on every path (client generates,
hub only echoes, wire is unsigned); my synthetic overflow ids are always `< 0`
(`nextLocalOverflowBatchId` starts at `-1`, decrements). The two ranges are disjoint by
construction, not by convention, and never cross the same encoding path. Kept as-is.

### commands + exit codes (follow-up)

Same command as before, run 3× after the follow-up changes:
```
cd "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript" && bun ./📜️script.ts test
```
All 3 runs: `Test Files  2 failed | 2 passed (4)` / `Tests  2 failed | 370 passed (372)`, exit `1`.
The 2 failures are the same pre-existing wasm-module test as every prior run in this report.
`tsc --noEmit -p tsconfig.json` (repo root): exit `2`, 19 `error TS` lines, same 4 pre-existing
files, zero in `🟦️backbone-worker.ts` (confirmed via `grep`).

### honest gaps (follow-up)

- The SSE-side sustained-health fix has one dedicated test (the reset case); I did not duplicate
  the "rapid cycling doesn't reset" and "abort, no leak" tests for SSE specifically — the
  mechanism is byte-for-byte the same code shape as the hub path (`reconnectForever` is the exact
  same function for both), so I judged the hub's three tests plus one SSE confirmation sufficient
  rather than doubling every case, but a reviewer wanting full symmetry would reasonably ask for
  the other two SSE variants as well.
- `Math.random` pinned at a single fixed value (`0.5`) makes every jittered delay in a test exactly
  computable, but does not exercise the full `[minMs, cap]` random range — this proves the reset
  mechanism works at one point in the distribution, not that jitter itself is correctly bounded
  (that property was already covered by the pre-existing, unrelated `retryWithJitteredBackoff`
  tests in `🟦️glue.ts`, not re-verified here).
