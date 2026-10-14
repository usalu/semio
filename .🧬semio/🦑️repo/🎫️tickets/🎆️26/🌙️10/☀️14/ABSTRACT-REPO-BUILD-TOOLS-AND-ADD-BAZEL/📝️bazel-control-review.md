# Bazel Cancellation Review

Read-only independent source review on 2026-10-14 of `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts`, its tests/fixture, and framework process envelope/execution/termination helpers. No tests were executed by this audit. The Bazel owner was replacing HTTP control with direct TCP while this review ran; findings were sent directly to that owner before final qualification.

## Deadline Preservation

The runtime environment is created with `scriptInvocationEnvironment(invocation,...)`, which uses the invocation's current remaining time to derive a deadline epoch. It does not use the policy maximum as a fresh budget. Action processes call `receiveScriptProcessInvocation`; that rejects expired envelopes and derives remaining time from both original epoch and local monotonic elapsed time. `withBazelCancellation` retains the original policy/capabilities and delegates `remainingMilliseconds` to the received invocation while combining abort signals. `executeTask` obtains command timeout from `scriptInvocationBudget(invocation,0)`. No deadline reset was found on these paths.

This conclusion is source inspection, not a new subprocess deadline test. Sources: framework `🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`:6–63 and `🏃️process/🧭️routing/📥️invocation/🟦️.ts`:51–58.

## Drain Design

The TCP parent control binds loopback with a fresh token, closes admission before broadcasting cancellation, and keeps each joined action active until it sends an explicit closed message. An unexpected socket close does not count as a successful child drain. Parent stop has a finite acknowledgement wait, then aborts owned Bazel clients and closes sockets/server. The outer finally awaits all service/build client completion promises before releasing the workspace lease.

Action control sends its closed acknowledgement after the run callback settles; direct owned commands use `runOwnedCommand`, which terminates an owned process tree on abort and awaits the spawned child's close before settling. Framework termination handles descendant-created POSIX groups and Windows taskkill trees.

## Findings Sent to Owner

1. **Parallel commands could acknowledge before siblings stop.** At review time, `executeTask` used `Promise.all(operations.map(run))`. A first command failure rejects immediately without cancelling or awaiting still-running siblings. The surrounding action control then enters its finally and sends closed, which is earlier than complete command-tree drain. Requested a local abort controller, sibling cancellation on first failure, and an all-settled drain before rethrow/acknowledgement.
2. **Pre-join handshake did not observe original cancellation.** In the first TCP revision, `withBazelCancellation` awaited joined before checking the combined invocation. Original deadline/abort during connect or join could leave the action waiting, and finally could wait for termination acknowledgement without an admitted id. Requested cancellation/deadline handling during initial connect/join, with socket closure on pre-join refusal.
3. **Late action admission and runtime reuse require closure ordering.** Root marks control closed before cancel broadcast, and an unadmitted late join is refused. Workspace/runtime reuse remains safe only after active action drain and all Bazel client closes. The existing per-namespace workspace lease/finally enforces that intended order; review does not claim Bazel server late scheduling was independently exercised.

The owner reported earlier HTTP-version integration evidence (7 tests/65 assertions including child PID checks and an observation window exceeding consumer delay), followed by Bun proxy isolation failure prompting TCP replacement. Those results are owner-reported and do not qualify the final TCP implementation.

## Revised Source Review

The owner landed fixes and this reviewer reread the current helpers:

- Parallel commands now combine the original abort signal with a task-local controller. On first rejection they abort that controller, await every pending runOwnedCommand via allSettled, then rethrow the original error. The surrounding action cannot send its closed receipt before this drain.
- TCP admission now has join and start phases. Only start makes an action active; original abort and remaining-time timer reject the initial handshake, and cancellation before an identity closes the unadmitted socket. The run callback is entered only after started and checkScriptInvocation.
- Action receipt wait is bounded at 5000 ms; root also has a bounded drain wait. Root refuses new starts after closing admission and unexpected socket loss never counts as a closed receipt.
- No additional blocking issue was found in this source review. The owner is adding a parallel failure/slow sibling witness and qualifying the latest revision. This review does not report those not-yet-observed test results as passing.

No production or test source was edited by this reviewer.


