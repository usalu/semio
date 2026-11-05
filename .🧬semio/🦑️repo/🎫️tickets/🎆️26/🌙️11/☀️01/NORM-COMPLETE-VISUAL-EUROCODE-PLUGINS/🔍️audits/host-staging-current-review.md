# Current Host Staging Review

Read-only review of framework ShellHost staging session.ts and session/execution/lifecycle tests, with actual handle58985 first-leaf gate live. No JS edits or test execution.

Source: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📝️staging/session.ts.

Concrete retirement gap: refresh awaits source.query at66 without tracking its promise in close. close95 aborts the query, retires only the currently present leases99, then finishes/removes the host run101/118. A query resolving a lease after that point adds it66 and attempts stale release67. If that release rejects, catch75 suppresses it because the run is already closed. The lease remains in the detached run Set, while host.close no longer reaches that run. The existing pending-query cancellation test returns a promise that never resolves; lifecycle held-release tests cover leases already received, not this late-resolution plus failed-retirement path. The source interface permits query settlement after cancellation. The close promise also does not await late successful retirement.

Cancellation result boundary: execution cancellation aborts the supplied signal93 and waits for settlement. If source.execute fulfills despite an aborted signal (e.g. its operation already committed), execute87 reports executed; only rejection88 reports cancelled. Current cancellation test deliberately throws after observing abort. This is a precise result-contract boundary, not asserted data loss: successful native commit should not be mislabeled cancelled.

Concurrent open is guarded by opening identity116–118; cancellation invalidates pending openings121. Current tests cover latest intent, cancel during cleanup, retirement retry, and synchronous executing-notification cancellation.

Default production type remains OwnedUiSurfaceLease in both StagedActionSource8 and StagedActionHost107; test-only generic lease adaptation does not change that default. The lease export itself is a structural interface; runtime authenticity is instead issued by private ReadLease authority, WeakMap target certificates, and installed stamp checks in PluginRuntime/handle-ui-intake/lease.ts. No forged production issuer found in this bounded review.

## Review After Root Retention Repair

Current session.ts adds host #runs112 and query count36/65/78. Host close126 visits retained runs rather than only current presentation. A cancelled run with outstanding query cannot satisfy terminal93; late failed release preserves its lease94 and cannot remove the run. Subsequent host.close invokes close again because finished plus nonempty lease set does not take early return97. Already-finished retry skips session/control teardown101 and cannot call finish twice104. Successful late release and query-finally retirement callbacks eventually remove the retained run only after queries0, leases0 and closingnull. No new retirement gap found in this bounded source review. This is source evidence, not a runtime pass.

Reentrant cancellation review: refresh increments query count before publishing65/67, then checks closed/aborted before entering source.query; finally decrements even that early return78. execute installs its operation promise/controller83 before publishing executing84 and checks abort86 before source dispatch. Host installs run and adds ownership122 before initial publish, so initial-notification cancellation reaches the actual run. close installs #closing before publish106, allowing reentrant close to reuse the same promise. Opening identity120–121 still invalidates concurrent cancelled replacements. Default original lease type unchanged8/110. No concrete new issue identified.
