# Course journey-and-tabs hang (round 4)

## Symptom
`vitest run journey-and-tabs` printed nothing and ignored the 30 s test timeout, stalling right after `proctor.outage = true`.

## Diagnosis
- The worker ran at 40-75% CPU with growing RSS, so the event loop was never reached again (timeouts could not fire).
- Inspector `Debugger.pause` samples (SIGUSR1 plus a CDP script) always landed in `processRootScheduleInMicrotask -> renderRootSync`: whole-app sync re-renders driven from microtasks. A CPU profile showed the whole tree (RunScreen, navigation, OverviewCard) re-rendering, not one hot component.
- Root cause is a promise-only livelock in the in-flight session change, not the CardStack or CSS work (`useCardInterlock` only writes style attributes; its MutationObserver watches childList and the test ResizeObserver is a no-op, so it was ruled out):
  1. `record-answer` answers 503 -> `ProctorClient` marks `unreachable`.
  2. The session listener calls the new `revive()` -> `probe()` (`GET /instance`) succeeds -> marks `reachable`.
  3. `reachable` after non-throttled -> `outbox.wake()` retries the answer at once (no backoff) -> 503 -> `unreachable` -> step 2 again.
  With a transport that answers without latency every hop is a microtask, so the loop never yields; in production the same loop would hammer the proctor with a command plus a probe per round trip and flicker the connection banner.

## Fix (production)
`ProctorLink.revive` (`🎯️targets/⚛️react/🔨️modules/🧭️session/📡️proctor-link/🟦️.ts`) now waits `timing.minMs` (abortable `pause`) before probing, so a proctor that answers probes but refuses commands is asked at most once per minimum backoff and the loop yields to the event loop.

## Test config
`vitest-course.config.ts` now excludes `*-kit` helper folders, which the include glob had matched as empty "suites" (4 spurious failures).

## Results
- `journey-and-tabs`: 14 passed in 32 s (was a hang).
- Whole course suite: 47 files, 1135 tests, all passed (51 s).
- Temporary output in `🗑️generated` (probe scripts lived in the scratchpad, no [DEBUG] logs were added).
