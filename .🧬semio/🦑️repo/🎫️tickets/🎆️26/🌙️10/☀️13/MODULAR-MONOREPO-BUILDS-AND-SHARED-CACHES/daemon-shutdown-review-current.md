# Daemon Shutdown Review — Current Source Check

Root inspected the authored service controller, the actual Nx server patch and the currently installed SDK request shutdown handlers. This review is read-only, not a complete daemon runtime proof.

The service keeps tools/leases/Rust/SDK scopes through its terminal promise. `resourcesStopped` waits physical scope release, while terminal requires the actual server close plus every registered worker and request to settle. Shutdown logs/cache cleanup/plugin cleanup/analytics precede `completeNxDaemonShutdownV1` in the authored patch, so those literal operations are not intentionally placed after storage release.

A potential awaited shutdown inside an ordinary request loan was raised for review because it would cycle: shutdown completion waits terminal, terminal waits request-close, request finally waits its shutdown body. Actual `handle-force-shutdown.js` and `handle-request-shutdown.js` both defer `handleServerProcessTermination` with setTimeout and return their response, so that direct cycle was **not demonstrated in either current route**. Storage independently confirmed this and is adding an actual Node request-settlement law; no broad work-loan bypass was justified or added. The fatal-handler route now signals shutdown without awaiting terminal and throws to settle its own loan.

The new Source controller authenticates the exact finite startup parent through a fresh physical witness endpoint before issuing its explicit cache-only daemon service policy. The service policy is separate from subsequent request envelopes. It acquires all root scopes before its IPC acquisition ACK, and binds the canonical repository separately from the watched workspace. Worker reader tokens bind root identity and actual same-process thread identity.

The new installed SDK and actual Native worker readiness/retirement proof remain pending. Source inspection and the focused Node service test cannot qualify the full deployed daemon by themselves. Historical foreign daemons remain untouched.
