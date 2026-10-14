# Final Backend Review

Read-only review of shared task selection, dependency bootstrap, and the Bazel backend. No tests were executed during this review; the existing validation reports were read.

## Finding

- **P2 — A service can bypass an explicit readiness requirement by exiting successfully.** In `library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts:227`, `completion.then(accept, refuse)` resolves readiness when the service exits zero, even when `readyWhen` contains markers that were never emitted. For a continuous dependency with `readyWhen: "ready"` whose command exits zero without printing `ready`, the consumer is therefore admitted at line 229 although its prerequisite never became ready. The failure race at line 230 only observes rejected completions and cannot catch this case. Preserve finite continuous-root behavior if intended, but reject successful completion before required readiness markers are observed.

No other significant, demonstrable defects were identified within the timeboxed scope. Native Windows/Linux behavior was not executed.


## Resolution

The Bazel owner corrected premature successful service completion and added actual process regressions. A service exiting before its required marker refuses readiness and never starts its consumer. A service exiting after readiness cancels the active dependent consumer. The complete real Bazel suite passes seven tests and 55 assertions, including the existing cache/nested-task/output/service laws; see bazel-validation.md.


## Delayed Process Liveness Follow-Up

A later witness check found that stopping the Bazel client did not terminate a running consumer action: its delayed completion file appeared after the closure had already failed. The earlier immediate failure assertion was insufficient to establish process termination. This is being corrected by the backend owner with a delayed witness regression before final qualification.


## Resolved Delayed Process and Cold Setup Qualification

The final backend suite passed **8 tests / 82 assertions in 17.30 seconds** under unreachable HTTP, HTTPS and ALL proxy settings with empty NO_PROXY. Built-in local TCP control preserves the original invocation and requires worker join/start plus explicit post-command drain acknowledgement. Actual child PID and delayed completion witnesses establish termination after service failure, original user cancellation and a failed parallel sibling. The failing sibling waits for the slow child's startup witness, making the running-child case deterministic. Independent source review found no remaining control/deadline/drain blocker.

A new random-selection real repo_task executes offline from the shared pinned module cache/lock, with new-workspace downloads disabled. Native Bazel autoload-empty alone was proved insufficient because embedded bazel_tools unconditionally requires registry module metadata. Setup now owns that bounded seeding stage; uncached offline executable/module setup is refused before network. Server idle timeout is schema-owned at 60 seconds. Earlier immediate cancellation and ambient-proxy API assumptions were insufficient; the final checks supersede those claims.
