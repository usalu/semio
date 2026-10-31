# Presence Handshake Unit Timeout Audit

Actual19159 full presence file:6tests,3passed/3failed; exactly deadline,parent-cancelled,already-cancelled hang-server rows each hit5000ms. Open/refused rows and new swipe-policy law passed. No WS production edits were made by the swipe feature.

waitForPresenceSocket has its own finite timer, parent abort listener and immediate already-aborted check, removing listeners/timer on settlement and closing failed socket. presenceUpgradeStatus uses a real HTTP request with AbortSignal.any(parent,finite deadline), rejecting with parent reason or authored timeout. These Source paths do not by themselves establish which await hung.

Two unbounded fixture boundaries need phase attribution: oracle Promise settles only open/unexpected-response/error (not close), and finally awaits server.close callback after destroying only currently tracked connections. Every run(false) constructs a globalWebSocket even when controller is already aborted, then calls close while connecting. A late accepted connection after the cleanup snapshot could leave server.close waiting; alternatively oracle completion may be missing. This is a concrete lifecycle hypothesis, not an observed cause. Existing log lacks phase markers, so instrument runfalse/runtrue/upgrade/connection-count/close before repairing. No deadline increase, host-pressure assumption or assertion removal warranted.

## Actual41412 phase evidence and genuine oracle route

Actual phase log places all three failures in ORACLE before upgrade/teardown; own bounded wait already completed. Runtime node compatibility string26.3.0 coexists with actual Bun1.4.2. Therefore earlier final-close race hypothesis is not this observed timeout and is withdrawn as causal attribution.

Current tool registry vitest locator selects package script vitest.mjs via bun, while repositoryVitestPolicy and authored Course config describe node runtime/node-only tests. Bare ws can resolve metadata for8.22.0 yet execute Bun built-in compatibility: primary [Bun documentation](https://bun.sh/blog/bun-v1.4.4) explicitly describes import ws as native-backed built-in implementation. Do not treat resolved package path as executed-class provenance. Actual registered Node read locates installed ws8.22.0 index.js→lib/websocket.js; its genuine JS maps handshakeTimeout to HTTP request timeout and retains caller signal in request options. No timeout option should be dropped or imitated by our implementation.

Supported registered Node ingress can run the complete same presence file with existing config, without altered deadlines:

```sh
SEMIO_LANGUAGE=en repo tools exec node -- node_modules/vitest/vitest.mjs run --configLoader native --config 🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎚️config/🟦️.ts 🎓️teaching/🏛️architecture/📚️course/🧪️tests/🪢️presence/🟦️.ts
```

Qualified script lookup is dirname(require.resolve(vitest/package.json))/vitest.mjs; the vitest/vitest.mjs package subpath is not exported. Existing canonical aliases/config remain; no ambient executable, runtime fallback or schema guard bypass. Another clean scope is a registered Node oracle child while retaining first-party Bun comparison, if that is the intended multi-runtime law. Neither route was executed by auditor; any registry-wide runtime correction requires its own coherent existing owner proof.

Capacity correction: the proposed Vitest execution uses normal governed registered Node, never --light. --light is appropriate here only for bounded read-only audit scripts. The earlier proposed light Vitest spelling was incorrect and was not executed. Root owns one actual Node check after removing temporary DEBUG phase instrumentation; no Bun ordinary-suite GREEN or global registry runtime change is claimed.

## Actual genuine Node7809 positive

Parent authoritative exit0 independently matches presence-handshake-node-owner.log: normal capacity-admitted registered Node, same native Course config and complete presence file,1file/6tests passed6.37s, no skipped summary or DEBUG phase output. Five original handshake rows plus new swipe-policy law ran; first-party finite deadline/cancellation comparisons agree with genuine third-party ws on Node. This resolves the actual Node-owned file proof, not a claim that the Bun bare-ws compatibility suite is green or that registry runtime was globally changed.
