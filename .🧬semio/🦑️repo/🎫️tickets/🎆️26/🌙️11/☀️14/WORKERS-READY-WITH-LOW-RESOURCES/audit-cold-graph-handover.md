# Cold Graph Discovery Handover

The recorded duplicate structural claims are documented in [the live audit](audit-dual-structural-claims-0640.md). The displaced loader stayed in cold discovery after the waiting loader filed its stale-code request. Cold discovery can cover the entire workspace without completing an incremental wave, so the existing wave-boundary handover check never ran while inference was active. The publication fence still rejected expired custody.

## Change

- Cold discovery polls its held claim’s request file at the existing one-second custody cadence. One lookup can be in flight at a time. This path skips implementation identity and claim roster scans. Owner progress keeps its existing reporting cadence.
- A recorded stale-code or stalled request sends the existing `SEMIO_STALE_IMPLEMENTATION` error through discovery’s cancellation signal. The host passes it into the existing owner wave alongside the issued lease and invocation signals. Inference stops admitting additional owners, drains pending work, and closes its farm and part row log. The loader waits for discovery and any pending request read before relinquishing custody.
- The caller’s original cancellation reason takes precedence at the boundary, including when it arrives after handover while discovery is draining. A discovery result that arrives after cancellation is disposed before release. Incremental wave journaling, custody admission, and the publication fence retain their existing behavior.

## Verification

- Before the implementation, both recorded-ask cases failed because cold discovery received no cancellation signal, and the parent cancellation test surfaced a generic inference abort. A follow-up red run reproduced lost parent cancellation when it arrived after the handover signal.
- The final focused run through `repo tools exec --light bun -- test` passed 22 tests across five files. The new JSON scenarios used real file-backed custody and an injected clock. SQLite independently derived the progress fields and projected the lifecycle from recorded events. The successor acquired custody before the fake grace period elapsed, and the old loader never sealed a generation. The run also covered the custody kernel, expired publication fence, incremental handover, journal recovery, and wave progress.
- The modified runtime sources transpiled and the scoped whitespace check passed. The new schema and fixture stay within the prescribed path limit. The full workspace build and a live host reload were outside this verification while the readiness run needed host capacity.
