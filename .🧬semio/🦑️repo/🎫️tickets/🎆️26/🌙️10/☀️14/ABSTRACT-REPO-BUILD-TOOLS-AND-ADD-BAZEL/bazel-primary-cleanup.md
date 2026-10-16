# Bazel Primary And Cleanup Settlement

Actual Refusals54795 terminal1: expected `Continuous task exited while required by selected tasks`, received `Bazel action termination acknowledgement deadline exhausted`; 0pass/1fail/6assertions/90.20seconds in current-bazel-refusals-owner.log. The second row reached real consumer cancellation; its expected primary was replaced during cleanup before in-test stopped-PID/late-file assertions ran. This is semantic RED, not a pass.

The strict five-second acknowledgement requirement is retained. Server connection close removes the connection but does not authenticate a closed receipt or clear the active record. A process may be dead while its explicit termination receipt is missing; no arbitrary disconnect is treated as successful acknowledgement.

`withBazelTaskCleanupV1` always awaits cleanup, preserves an arbitrary primary (including undefined), and still throws cleanup refusal after an otherwise successful body. Error primaries retain a bounded array of secondary failures and a bounded diagnostic is emitted even for unattachable values. Both selected-task settlement and received-action cancellation use it. The control.stop phase still precedes awaited allSettled of every service/build; its throw cannot skip that drain. This preserves the prior try/finally drain ordering rather than claiming it was previously absent.

Low found one additional acquisition gap: runtime.json write after control/server creation was outside protected body. It is now the first awaited operation inside protected cleanup, so writer failure also stops the acquired server and listener. No policy/deadline/termination timeout change, ignored acknowledgement, or assertion weakening was added.

Actual helper missing-port RED30234 is separate from the runtime RED. Source90822 GREEN:11pass/9skip/328assertions/1.53seconds, bazel-cleanup-source-green.log. Language-neutral completion rows compare actual async function identity/ordering against SQLite; successful-body acknowledgement failure stays fatal, and primary undefined survives. This Source result preceded the tiny writer-acquisition correction; a combined current Source gate follows Root's concurrent snapshot fix.

Postfailure bounded read-only evidence: generated/bazel-refusals-after-failure-processes.json identifies own fixture bz-refusals-f29e5509 consumer21853. ps returned1/absent and the late completion file was absent. This was after process terminal and does not establish in-test drain timing or every worker/socket. Actual Refusals rerun is still required, preserving PID and late-file assertions.

## Current Captured Runtime Refusals

Fresh permanent-owner REFUSALS87708 completed terminal 0: 1 passed / 29 filtered / 11 assertions / 76.17 seconds (body 75.862 seconds), `🗑️generated/current-bazel-refusals-captured.log`. This run uses the current captured planner/action implementation under genuine Original120. Both early-service-exit and required-service-exit vectors passed their expected primary checks. Actual consumer PID absence and no late completion-file checks passed inside the test. Accepted command promises settled before terminal.

The second vector still emitted `Cleanup refusal retained after primary: Error: Bazel action termination acknowledgement deadline exhausted`. The strict acknowledgement was not silently treated as success; it remains a secondary diagnostic after the original required-service error, while consumer shutdown and no-late-write are separately proven. This result does not claim every action delivered an authenticated closed acknowledgement. The actual cancellation/parallel-sibling phase is a separate next runtime.

## Cancellation Fixture Startup and Settlement

Initial current cancellation60326 completed terminal 1 after 15.31 seconds with no consumer startup: its independent 15-second test timer expired while captured implementation planning was still running. The fixture then unconditionally acknowledged an additional signal it had never aborted, masking the startup failure with `Actual stopped derived Script signal scope required`. This is fixture setup/cleanup failure, not proof that user cancellation reached the backend.

The test-only correction removes that extra startup clock and polls with the same actual Original checks/continuation. It refuses promptly if accepted work settles before consumer startup. Unconditional primary-safe cleanup aborts the existing additional signal, awaits the accepted work through allSettled, then acknowledges that exact issued signal scope. The original signal and existing consumer PID/no-late-file assertions are unchanged. No production guard, factory, policy, deadline or runtime grant changed. Low reviewed the narrow correction clean.

Full Source65190 completed terminal 0: 20 passed / 9 runtime cases skipped / 791 assertions / 13.71 seconds (`🗑️generated/current-bazel-cancellation-source.log`). This includes independent phase/Original AST laws and the current full captured backend census; it does not assert cancellation runtime success. Runtime retry77030 is recorded separately once terminal.


Runtime cancellation retry77030 completed terminal 0: 1 passed / 29 filtered / 14 assertions / 47.59 seconds (body 47.369 seconds), `🗑️generated/current-bazel-cancellation-drained.log`. Actual user cancellation reached a started consumer; the parent Original signal remained un-aborted, its consumer PID became absent, and no late file appeared. The separate parallel-command failure vector also proved consumer shutdown/no late write. The fixed startup fixture and all accepted work settled before terminal. As with refusals, explicit acknowledgement failure remains a secondary diagnostic after an actual cancellation primary; no authenticated-receipt success is inferred from the process checks.
