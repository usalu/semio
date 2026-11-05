# Control DAG Extra Public Coverage

Root authorized permanent test-only coverage for the two previously source-reviewed obligations. No production process source edits, Native/server runs, process termination, environment spoofing, synthetic authority, or timeout widening.

## Files And Contract

Permanent owner:
`🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/⏳️admission/{extra.ts,extra.json,extra.schema.json}`.

Existing admission/🟦️.ts now imports ./extra.ts alongside projection/dag/freshness, preserving all other imports. Coordinated ownership with Verify before edits; Verify owns remaining() and existing DAG/freshness files.

Strict neutral schema fixes the legal issued owners, actual finite following bound1500ms, parent60s, renewable idle80ms/maximum5000ms, real expiry wait120ms and expected expiry kind. Graphlib independently confirms the small cross-root parent/following graph has three reachable nodes.

## Four Directly Green Public Cases

1. Two genuine overlapping withScriptProcessEnvelope calls are born outside each other's async context, with different owner names/capabilities and distinct actual issued invocation objects. Finite first invocation remains live while second child follows its control. Child remaining and projected wire deadline stay within the first's1500ms bound. Public additional-signal cancellation on that followed control then refuses both remaining() and projection. First invocation itself settles normally, proving borrowed cancellation does not cancel the independent root.

2. Healthy renewable control has normal remaining≤80ms idle window, while scriptResourceProcessEnvironmentV1 yields maximum5000ms and deadline remaining>160ms. This observes maximumOnly excluding healthy renewable idle in the real public projection.

3. After a real120ms synchronous elapsed interval, projection refuses actual80ms idle expiry with public error name ProgressDeadlineExpired and reason idle. No clock is mocked or authority renewed. Projection's ordinary entry authentication can reject expiry before its maximumOnly subpass; the test proves public behavior, not a private-stage hook.

4. Following a successful renewable projection, public additional-signal cancellation is rechecked on the next projection and the exact error propagates. No memo survives across calls.

These cases validate existing correct behavior after the settled fix. **No new red/TDD implementation claim** is made for this feature coverage.

## Actual Execution

`repo tools exec --light bun -- test './🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/⏳️admission/extra.ts'`

Actual exit0:4pass,0fail,22assertions.

Subsequent combined dag.ts/freshness.ts/extra.ts run observed9pass1fail61assertions. Verify concurrently added a new sixth preexisting-suite case, “fresh cutoff evaluation preserves an open authenticated admission wait”; that case expected authorized=true but observed false with Original resource loan deadline exhausted. All four extra cases and five prior DAG cases still passed. Root and Verify were immediately notified; Verify owns that separate actual red/correction. This report does not present the combined source as frozen/green while that peer correction is pending.

Root owns old admission and actual-child regressions. No Native slowdown causation or old-image replacement outcome is inferred from these tests.
