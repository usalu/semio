# Nx Watcher Startup Recovery — 2026-10-28

## Observed Failure

The original release gate sent REQUEST_PROJECT_GRAPH at 16:53:53.317 UTC. Its daemon reported initial native watch registration failure at 16:53:59.538 UTC: OS file watch limit reached. No project-graph request handler started afterward. Later native notifications only logged that a previous watcher error existed.

The actual failure is inside the SDK server's async listen callback. Output watcher registration rejects; its catch awaits handleWorkspaceChanges(err, []) but never rejects the enclosing startServer promise. The original service never reaches readiness. A connected caller has already sent its graph request, but withNxDaemonRequestV1 remains queued in readiness.admit(). The socket stays open indefinitely.

## Change

The authored pinned Nx SDK patch now rejects that enclosing startup promise after reporting the watcher error. Existing service ownership then drains the failed startup, refuses pending readiness requests, and closes the failed daemon connection. Existing client project-graph fallback disables only the current invocation. No persistent daemon-disabled marker, blanket default, peer-daemon mutation, kernel change, or installed vendor-cache edit is added.

## Regression

The language-neutral caller-environment fixture and schema now include the exact initial native watcher message and a bounded startup result. The repository regression extracts the actual immutable SDK listen callback through TypeScript, executes it against a failing native watcher, and uses the real owned daemon readiness gate with a request already waiting.

A retained read-only input, daemon-recovery.mjs, reproduces the behavior without starting a daemon or constructing an Original invocation. Against the original installed SDK it fails as expected: startupRejected=false, pendingRejected=false, reported=1. Against the authored patch applied only to an in-memory pristine SDK image it passes: startupRejected=true, pendingRejected=true, reported=1. AJV accepts the authored fixture/schema, and the owned strict patch validator accepts all hunk counts and cumulative coordinates.

The existing actual SDK invocation-local fallback test gains the exact initial OS watch-limit message; its next invocation must remain daemon-enabled.

## Validation Limits

The patched SDK must still be materialized through its normal owner before the full immutable SDK caller-environment suite can report a managed green result. The focused in-memory source regression does not claim a completed live daemon restart or full release gate.

The unrelated course test session 82275 remains alive. No duplicate test invocation was started for this daemon repair.

The retained probe also invokes the independent standard-library Python patch oracle through the registered repo tool interface. It reconstructs the pristine server image in memory and confirms the oracle produces byte-for-byte the same revised server image as the owned patch engine. That comparison passes.

Managed follow-up: dedicated normal target `repo run repo:test-caller-environment`, session 32120, has been started with ticket-local redirected logs. It is waiting for normal capacity admission; no test-runner bypass was used.

## Latest Managed Suite Status (2026-10-28T18:20:22.261Z)

Session 32120 passed capacity admission and now advances through graph closure verification (433 of 2,059 at the latest sample). It has not yet reached the actual SDK-backed caller-environment test. No result is claimed for that suite until terminal completion.

## Deliberate Capacity Release

At the parent's explicit request, session 32120 was cancelled through its own handle after closure verification while it waited for the shared graph compute slot. It terminated with status 130; owned PID 222451 disappeared and its captured child 252045 became a zombie with no RSS. The retained log confirms SIGINT and lease release. This releases its approximately 2 GB admission under measured host memory/swap pressure so actual backend compilation can proceed. The same managed caller-environment regression remains required and is deferred until backend compilation progresses; it is not reported as passing. Course session 82275 was untouched.
