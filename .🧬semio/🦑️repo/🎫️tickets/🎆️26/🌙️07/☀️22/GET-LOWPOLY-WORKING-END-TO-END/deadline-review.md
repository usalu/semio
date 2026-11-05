# Deadline Patch Review

Read-only source review on 2026-11-05. No infrastructure edits, duplicate tests, builds, or native runs by this reviewer. Current workspace reads work. Runtime reports the corrected admission suite passes 10 tests / 58 assertions; that result is runtime-owned.

## Final Contract Interpretation

Serialized process envelopes are DATA. They describe numeric policy, capabilities and epoch; they contain no emitter identity and cannot distinguish an ordinary scope's wire from the byte-identical original birth envelope. The authoritative cancellation/liveness token is the CURRENT privately issued control, with its actual parent/followed restrictions.

Earlier review advice to attach revocable source-scope Sets to byte-identical envelope data was overly restrictive. It allowed a completed ordinary scopeA to poison the common root wire and prevent legitimate scopeB from performing the next Cargo capture. Unioning all same-byte emitters made this worse. That proposal is superseded by the positive sequential-scope requirement and the current DATA revision.

If a caller needs a narrower scope's cancellation authority, it must use that actual control or explicit private refinement/follows; a serialized wire alone cannot transport that identity. A still-live broader current control may use narrower deadline DATA after its former emitter ends. Numeric expiry still narrows the handoff and cannot be renewed.

## Current Source Matched

Owner: `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`.

- Root invocation captures its own admission-wait baseline; earlier independent waits do not fund it.
- Fresh finite/inherited child controls derive epoch from live parent remaining, then capture current wait baseline. The old expired raw parent epoch is no longer reused.
- Ordinary requested0 stays unbounded; finite requests beneath ordinary authority start finite clocks; inherited requested0 beneath finite authority stays bounded through actual parent traversal.
- Cached projection records retain wall epoch, monotonic initial remaining/start, and wait baseline; only new admission wait is credited, and active elapsed time still consumes finite maximum.
- Previously narrowed cached policy/deadline is considered before the ordinary return, so omitted wire does not restore unbounded output.
- Equal wire data retains its original numeric baseline without adding emitter-lifetime revocation.
- Known numeric wire provenance receives once-only admission credit. Unknown valid same-owner wire narrows by raw remaining and policy; malformed/foreign-owner/raw-expired unknown data gets no private wait credit. Known numeric expired data still refuses.
- Current owner, actual ancestors/follows/progress policies and cached bounds intersect through minimum. Integer emitted epoch is limited by both live remaining and numeric policy ceiling.
- Both finite and ordinary paths now materialize environment first, then recheck the current issued control. A getter that cancels that current control cannot return a stale handoff.

The DATA revision closes the ordinary sequential-capture regression while keeping current control cancellation dominant. No remaining source-level revival in those inspected paths was found.

## Type Cleanup Confirmed

Review caught a stale required sources member in the projectedChildDeadlines TypeScript record after the DATA revision removed that field from stored values. Runtime corrected it. A final source reread confirms the record now contains only deadline and maximumMilliseconds, matching its stored value. No TypeScript compiler run by this reviewer is claimed.

## Regression Coverage Matched

Current focused tests use Ajv for neutral schema and SQLite for independent arithmetic. They cover wait beyond raw epoch, open/closed waits, repeated wait without duplicate credit, fresh inherited/finite controls, active elapsed time, repeated no-wait projection, independent later root invocation, ordinary finite cache preservation, issuer retirement as DATA, numeric expiry, backwards wall-clock projection, reprojected DATA with independently live current authority, deterministic equal finite/ordinary wire cases, retired versus live siblings, current cancellation during final environment materialization, and legitimate sequential ordinary scopes using original/new root envelopes.

The suite now proves essential positive behavior as well as refusal. Current source checks preserve actual-current-control liveness rather than conservatively rejecting unrelated work because an indistinguishable prior emitter ended.

No Lowpoly native compilation/test or live browser pass follows from these process-level checks. Canonical native retry and live activation remain required.
