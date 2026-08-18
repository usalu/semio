# 📓️ terra-web-directory report

Owned path: `🧰️framework/🛍️products/💻️os/🟦️component.ts` — `DirectoryClient` region (inside `🔖️HubBinding`) and the backbone-envelope IO region (new `🌐️BackboneEnvelopeIo` sub-region inside `🔖️Backbone`). No other file touched.

## delivered

1. **Finding 1 (`getJson`/`postJson` no signal/timeout)** — every `DirectoryClient` REST method (`mintSession`, `me`, `spaces`, `space`, `command`, `events`, plus the private `getJson`/`postJson`) now takes an optional `DirectoryRequestOptions { signal? }` and routes through `fetchWithTimeout` with a fixed `DIRECTORY_HTTP_TIMEOUT_MS = 10_000`. A hung server now rejects at the timeout instead of hanging; the rejection carries no `.status`, which is exactly the shape the existing boot-path catch in `ShellHost/🟦️component.tsx` (`client.me()`/`client.mintSession()` inside the identity-bootstrap effect) and `directoryRejectionStatus` in `🟦️backbone-worker.ts` already treat as "hub unreachable → stay offline" / "queue and retry". Neither of those files was touched — the fix is entirely in making the promise settle at all.
2. **Finding 2 (`stream` reconnect has no jitter)** — rewritten onto `retryWithJitteredBackoff`, mirroring `🟦️backbone-worker.ts`'s already-landed `connectHubOnce`/`connectHub` idiom exactly (one WS attempt per `fn()` call; resolves only on caller-initiated `close()`, rejects on every other close/error/construct-throw so the retry loop keeps going with jitter). `lastSeq` resume semantics (always dialing with the highest `seq`/`headSeq` actually observed, never the caller's original `since`) are untouched — same variable, same update sites, same `wsUrl()` logic.
3. **Finding 3 (`readBackboneEnvelope`/`writeBackboneEnvelope` no signal/retry)** — both now take an optional trailing `signal: AbortSignal`. `readBackboneEnvelope` retries transport-level failures with `retryWithJitteredBackoff`, bounded by an overall `BACKBONE_ENVELOPE_RETRY_WINDOW_MS = 15_000` (so a permanently-dead backbone eventually rejects instead of retrying forever). `writeBackboneEnvelope` gets the timeout/cancellation but is deliberately **not** retried — see `## retry-safety-reasoning`.

## findings confirmed vs not reproduced

All three findings reproduced exactly as described against the live file before editing:
- `getJson`/`postJson` (pre-edit lines 3957–3966) called bare `fetch` with no signal/timeout.
- `stream`'s reconnect (pre-edit lines 4027–4031) used `setTimeout(connect, reconnectDelayMs); reconnectDelayMs = Math.min(reconnectDelayMs*2, MAX)` — deterministic doubling, no jitter.
- `readBackboneEnvelope`/`writeBackboneEnvelope` (pre-edit lines 107–142) called bare `fetch` with no signal and no retry of any kind.

No finding failed to reproduce.

## line ranges edited

All in `🧰️framework/🛍️products/💻️os/🟦️component.ts` (4483 lines total after edits):

- **15–16** (Header imports): extended the existing `@semio-tech/framework` import lines to bring in `fetchWithTimeout`, `retryWithJitteredBackoff` (values) and `FetchTimeoutResponse` (type). This is a one-line-each addition to an import statement already present, not a new import; flagged here for visibility even though it sits just above the `🔖️Backbone` region rather than inside it — see honest gaps.
- **105–318** (`🔖️Backbone` region): new `🌐️BackboneEnvelopeIo` sub-region (lines 107–313) replacing the old bare-`fetch` `readBackboneEnvelope`/`writeBackboneEnvelope` with the timeout+retry versions, their helper types (`BackboneFetchResponse`, `BackboneEnvelopeResponseError`), `readBackboneEnvelopeOnce`, and a new inline `if (import.meta.vitest)` test block (6 tests).
- **4069–4482** (`🔖️HubBinding` region): added `DIRECTORY_HTTP_TIMEOUT_MS` + `DirectoryRequestOptions` (after `DirectoryHttpError`, before the `DirectoryClient` docstring); rewrote `getJson`/`postJson`/`mintSession`/`me`/`spaces`/`space`/`command`/`events`/`stream` inside `DirectoryClient`; rewrote the `DirectoryClient.stream` inline test describe block (replaced the exponential-backoff test with a jittered-bounds version, added a `close()` stops reconnecting test, added a new `DirectoryClient http (getJson/postJson timeout + abort)` describe block with 3 tests).

Nothing else in the file was touched. `🎠️kernel/🟦️component.ts`'s `🔖️IoRouter` region was never opened.

## retry-safety-reasoning

- **`readBackboneEnvelope` — retried.** A read has no side effect on the server; re-issuing it after a transport-level failure (the request never definitively reached or was answered — `fetch`/`fetchWithTimeout` itself threw: DNS, connection refused, or our own timeout) can never duplicate an effect. Only genuinely transient failures are retried: a definitive server response (any status, including non-404 errors) is distinguished via a local `BackboneEnvelopeResponseError` marker class and immediately aborts the retry controller so it is never retried — retrying a real "500" or "403" would just repeat the same answer while burning the retry window. 404 is treated as a real, final "nothing written yet" and returns `null` without ever entering the retry path.
- **`writeBackboneEnvelope` — NOT retried, deliberately.** The call always PUTs the caller's complete current bundle, which makes a byte-identical retry *look* idempotent — but this function has no visibility into the server's actual write semantics (a pure last-write-wins slot vs. one that appends a history/audit entry per accepted write), and "the request timed out" gives no way to distinguish "never arrived" from "arrived and was applied, only the response never came back." A blind retry risks silently double-applying a write whose effect this client cannot observe well enough to rule that out. Per the packet brief's own instruction ("if a write cannot be safely retried, do not retry it… getting this wrong is worse than leaving it alone"), I chose not to retry it. It still gets the timeout/`signal` plumbing so a hung write is at least bounded and cancellable, and the docstring records exactly what would need to be true (a provably idempotent server-side replace) before a future packet adds retry here.
- **`DirectoryClient.mintSession`/`.command`** (both go through `postJson`) are **not** retried by `retryWithJitteredBackoff` either, for the same reason as the write above — `mintSession` mints a session (a side effect), `command` submits a directory command (already has its own definitive-vs-transient split one layer up in `🟦️backbone-worker.ts`'s `directoryRejectionStatus`/queue-and-retry, which this packet did not touch). They only gained the timeout/signal, not blind retry.
- **`DirectoryClient.stream`'s reconnect** is retried by design (that's finding 2) — but note this is retrying a *connection attempt*, not a write; every reconnect resumes from `lastSeq`, so there's no duplicate-application risk, only the already-existing gap/duplicate-free resume contract, which is unchanged.

## commands + exit codes

Full package suite (both files in this package, `../../🟦️component.ts` and `../../🟦️backbone-worker.ts`):

```
$ cd 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript && bun 📜️script.ts test
Test Files  4 failed (4)
     Tests  6 failed | 352 passed (358)
    Errors  2 errors
```
Exit code: 1 (non-zero — see baseline-vs-after below for attribution).

Isolated to just `../../🟦️component.ts` (my owned file), filtered by passing `component` as the file-match arg:

```
$ cd 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript && bun 📜️script.ts test component
Test Files  2 failed (2)
     Tests  2 failed | 322 passed (324)
```
Exit code: 1 (both failures are the pre-existing, unrelated `plan_workflow`/wasm-module test — see below).

## baseline vs after

**Baseline, captured BEFORE any edit** (full package, `bun 📜️script.ts test`):
```
Test Files  2 failed | 2 passed (4)
     Tests  2 failed | 322 passed (324)
```
Both failures were the same single test (`matches the Rust plan_workflow across shared fixtures decoded via wasm`, reported twice by the dual reporter), failing with `Error: Cannot find module '…rust/pkg/semio_framework_os.js'` — an un-built wasm artifact, nothing to do with this packet, present before I touched anything. `backbone-worker.ts` had **zero** failures at baseline.

**After my edits, component.ts only** (`bun 📜️script.ts test component`):
```
Test Files  2 failed (2)
     Tests  2 failed | 322 passed (324)
```
The dual reporter doubles every count, so the real per-file numbers are: baseline **152 tests** (component.ts), after **162 tests** — exactly **+10**, matching my additions one-for-one (swapped the old exponential-backoff stream test for a jittered-bounds version: ±0 net; added a `close()`-stops-reconnecting stream test: +1; added 3 `DirectoryClient` http timeout/abort tests: +3; added 6 backbone-envelope-io tests: +6 → 0+1+3+6=10). The failure count is unchanged (still exactly 1 real failure, the pre-existing wasm one) both before and after. All 10 new tests pass.

**After my edits, full package** (`bun 📜️script.ts test`): 4 additional failures appear, all inside `../../🟦️backbone-worker.ts`, all **not present** in my `component`-filtered run and **not caused by anything I touched** (I made zero edits to `🟦️backbone-worker.ts`). Attribution, not absorption: I ran the full-package suite twice in a row (19:22:33 and again ~2 min later) and the failing tests' reported source line numbers **changed between the two runs** (e.g. `sendWireFrame`'s caller moved from line 570/588 to line 324/336), and `🟦️backbone-worker.ts`'s on-disk mtime (19:22:33) fell exactly between my two full-package test runs (19:21:59 and 19:24:03) — i.e. a peer session is live-editing that file right now, exactly as the packet brief warned ("a peer packet is editing `🟦️backbone-worker.ts` in this same package concurrently"). This is not my regression.

## lease-requests

None. The only edit outside the two nominally-owned regions was extending two already-existing import lines at the top of the file (lines 15–16) to add `fetchWithTimeout`/`retryWithJitteredBackoff`/`FetchTimeoutResponse` to the existing `@semio-tech/framework` import — a minimal, additive, one-token-list change required for both owned regions to compile, not a new region-widening edit. No new import statements were added, and no other file was touched.

## honest gaps

- `DirectoryClient.mintSession` and `.command` (via `postJson`) got timeout/signal but not retry — see reasoning above; this is a deliberate scope decision, not an oversight, but it means finding 1's "every call accepts a signal and carries a timeout" is satisfied for all methods, while "continues offline on timeout" is only directly exercised (by test) for `me()`, the one actually on the identity/boot path per the ShellHost code I read.
- `DirectoryClient.stream`'s new implementation no longer resets its backoff delay to `HUB_RECONNECT_MIN_MS` on a successful `open` the way the old code explicitly did — `retryWithJitteredBackoff`'s internal attempt counter only grows for the life of one `stream()` call (matching the already-landed `connectHub`/`connectHubOnce` sibling in `🟦️backbone-worker.ts`, which has the same property). This means a directory stream that reconnects many times over a long session will asymptote to `HUB_RECONNECT_MAX_MS` (30s) between attempts and stay there, even after long healthy stretches, rather than snapping back to fast reconnects. I judged this an acceptable, safe trade-off (it can never hammer the server) rather than reinventing extra reset bookkeeping, and it matches the existing accepted pattern elsewhere in this file — but it is a real, intentional behavior change worth a future packet's attention if reconnect latency after long sessions turns out to matter.
- I did not run a full `tsc --noEmit` for this package (not part of the stated acceptance command, and the brief names 19 pre-existing repo-wide `tsc` errors elsewhere as already routed); vitest's transform did compile/execute my code paths successfully across all new and existing tests, which is the acceptance criterion actually specified.
- No real sleeps anywhere in the new/changed tests — all use `vi.useFakeTimers()`/`vi.advanceTimersByTimeAsync`/`vi.spyOn(Math.random)`, per the instruction.

## coordinator follow-up

Coordinator flagged the "no reset after sustained health" gap from the previous pass as a real regression, not an acceptable inherited property, citing CLAUDE.md's "support short connection-shortages and not freeze the app". Fixed in `🧰️framework/🛍️products/💻️os/🟦️component.ts`, still entirely within `DirectoryClient`/`🔖️HubBinding` (no other file touched):

**What changed.**
- New exported constant `HUB_HEALTHY_RESET_MS = HUB_RECONNECT_MAX_MS` (30s) — deliberately equal to the max backoff cap, with a docstring explaining why: surviving open for at least one full worst-case backoff cycle is long enough that a genuinely flapping server (accept-then-immediately-drop, whose cycle time is inherently far shorter) can never cross it by accident, so the reset can only ever fire for an actually-stable connection.
- `connectOnce` now arms a `HUB_HEALTHY_RESET_MS` timer on `ws.onopen` and, on close, resolves (instead of always rejecting) if either the stream was manually closed **or** that timer had already fired — i.e. "this connection proved itself healthy before it dropped". The timer is unconditionally cleared on every close path, so nothing is ever left pending.
- `stream()` no longer makes one long-lived `retryWithJitteredBackoff` call for its whole life. A new `runCycles()` loop calls it once per "cycle"; a cycle only ends (successfully) via the two resolve paths above. When a cycle ends because of the health-reset path (not manual close), `runCycles` starts a **fresh** `retryWithJitteredBackoff` call — a fresh internal attempt counter — for the next cycle.
- That fresh cycle is built "primed": its `fn` synthetically rejects once, immediately, before ever touching the network. This makes `retryWithJitteredBackoff` apply its own attempt-1 jittered delay (range `[HUB_RECONNECT_MIN_MS, 2·HUB_RECONNECT_MIN_MS]`) before the real redial, instead of dialing instantly. Reasoning for this specific choice, since the brief asked which of "implement the reset ourselves" vs. "lease-request a glue.ts change" I picked and why: `retryWithJitteredBackoff`'s attempt counter is a local variable fully closed over inside that one call in `🟦️glue.ts` — there is no parameter or return value that exposes or resets it, and it is a shared, already-verified (174/174) primitive also used by `🟦️backbone-worker.ts`'s `connectHub`, not something to reshape for one caller's need. Restarting the call from scratch is the intended way to get a fresh counter; the one gap that leaves is "the very first attempt of a fresh call is undelayed", and the synthetic-rejection prime closes exactly that gap by reusing the primitive's own jitter math for the pause rather than hand-rolling a second formula — which is what the brief asked me to avoid doing.
- Which of the two required behaviours (open-only vs. sustained-health) did I implement: **sustained health only**, by design — the health flag is set by a timer armed on open and is only ever read at close time, so a socket that opens and drops before `HUB_HEALTHY_RESET_MS` elapses never sets it and always takes the reject/escalate path. Resetting on open alone would defeat the backoff against exactly the accept-then-immediately-drop failure mode it exists to guard against; test (b) below proves this doesn't happen.

**Tests added** (all fake-timers, no real sleeps, inside the existing `DirectoryClient.stream` describe block):
- **(a)** a connection opened, held open for exactly `HUB_HEALTHY_RESET_MS`, then dropped — the next reconnect lands at exactly `HUB_RECONNECT_MIN_MS` (random pinned to 0, so the jittered range's lower bound), not at whatever an un-reset counter would have reached.
- **(b)** three rapid open-then-instant-close cycles (each held open for ~0ms, far under the threshold) — the backoff keeps escalating through caps `2·MIN → 4·MIN → 8·MIN` (random pinned to 1, the upper bound) with no reset ever observed; a bug that reset on open alone would show a reconnect long before each cap and this test would fail.
- **(c)** `close()` called partway through the health-timer's countdown (timer armed, not yet fired) — no further socket is ever created, and `vi.getTimerCount() === 0` afterward, proving neither the health timer nor any backoff timer is left pending.

**Command + exit code**, re-run after this change:
```
$ cd 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript && bun 📜️script.ts test component
 ❯ |@semio-tech/framework-os| ../../🟦️component.ts (165 tests | 1 failed)
     × matches the Rust plan_workflow across shared fixtures decoded via wasm
 Test Files  2 failed (2)
      Tests  2 failed | 328 passed (330)
EXIT_CODE=1
```
(Run without a pipe this time, per the coordinator's note — exit code 1 is entirely attributable to the single pre-existing wasm-module failure; no `tail`/pipe involved.) 162 → 165 tests (+3, exactly the three new ones); the same single pre-existing failure, nothing else changed, all three new tests pass.

Not touching `🟦️backbone-worker.ts`'s `connectHub`/`connectHubOnce`, per instruction — that fix is routed to the packet owning that file.
