# Deadline Contract Audit

Read-only source/schema/test audit, 2026-11-05. No infrastructure edits, build, test, or reproduction process was run. The coordinator reports native67153 exited before tests after about 4434 seconds, immediately after compiler capacity admission; this audit independently explains source-level failure paths, but does not authenticate that run's entire log.

## Contract Found In Current Owners

The process envelope schema (`🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧬️schema/🔣️.json`) binds finite policy to an integer epoch deadline, and ordinary maximum=0 to null. Its DeadlineTimer definition says finite remaining uses the minimum of wall and monotonic clocks. These are finite, privately issued authority bounds; they are not permission to reset a parent's budget at each child.

The current watchdog owner (`.../🏃️process/🐕️watchdog/🟦️.ts:103-112`) explicitly says admission waits are excluded because deadlines start when work is admitted. It counts overlapping waits once, includes open waits, and defines credited remaining as raw remaining plus nonnegative wait accumulated since that deadline began.

The admission regression (`.../🏃️process/🧪️tests/⏳️admission/🟦️.ts`) checks this arithmetic against Python; checks overlapping/open waits; and checks a 600 ms control survives a 450 ms admission wait plus 300 ms work, then expires after further work. Thus the finite maximum is a nonrenewable maximum of elapsed work after declared admission-wait exclusion, not unconditional wall time. Progress renews an idle window; ordinary CPU/output progress does not reset the maximum. The budget owner also supplies explicit wait credit to progress maximum calculations.

The child projection regression (`.../🏃️process/📤️child/🧪️tests/🟦️.ts`) uses Ajv for envelope validation and SQLite for maximum narrowing, tests a real spawned child, requires repeated projections not to extend the epoch when no wait intervenes, and rejects copied, retired, expired, or cancelled controls before reading environment getters. Its corpus covers finite 20s parent requests of 0/5s/40s/2s. It does not cover admission waiting across projection or before finite child creation.

## Concrete Inconsistencies

Current process owner path: `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`.

1. `remaining()` at line259 computes live credited authority, but `scriptResourceProcessEnvironmentV1()` at line333 first minimizes raw original/carried/control/cached epochs and only then clamps with now+available. An expired raw epoch wins even when live credited remaining is positive. It can throw Original child phase deadline exhausted after a legitimate long queue wait.

2. `controlFor()` at line279 constructs a new finite child epoch as min(raw parent epoch, now+request). Its new waitedAtStart is the current ledger value. `remaining()` subtracts that new baseline from wait credit for the child epoch, although that epoch may be inherited from the older parent clock. A fresh finite child after a long wait can therefore inherit an already-expired epoch and receive none of the historical parent credit. Fixing environment projection alone leaves this path.

3. `withScriptProcessEnvelope()` at line235 snapshots monotonic start and initial finite remaining, but both its timeout and control remaining add the entire process-global admission ledger without an invocation-start baseline. A later independent invocation in the same process can receive wait credit earned before its birth. Nested invocation reuse is separately guarded to preserve the actual parent, so it must continue to reuse the parent's baseline rather than create fresh authority.

4. `projectedChildDeadlines` at line331 stores only a number. It cannot distinguish wait already materialized into a previous child epoch from newly accumulated admission wait. Adding all historic wait again would double-credit; retaining that raw number unchanged loses subsequent valid credit.

5. Supplied carried envelopes are reparsed at the current raw wall time and ignored on any parsing failure. A prior authentic narrow projection that is raw-expired solely because of an excluded wait can be discarded, including its narrower policy. Arbitrary caller-controlled wire data must not gain private wait credit, but ignoring authentic narrower carried authority is not a valid repair either. Preserve provenance and the narrower finite bound; do not revive invalid/unowned envelopes by assuming they waited.

## Authority Invariants For A Focused Repair

Use a deadline's own captured wait baseline. Preserve the baseline when inheriting its epoch; when creating a genuinely fresh finite clock, derive its effective ceiling from current live parent remaining and snapshot the current wait ledger. A finite child consumes its own requested work budget and every parent/followed/progress restriction. Requested0 inherits the parent's authority; ordinary unbounded parent plus requested0 remains null. A finite request under ordinary authority starts a finite clock now, not at a historic finite epoch.

Wait exclusion must occur once per bound, then bounds combine with min. Do not sum credited remaining across ancestors. The maximum numeric policy must never widen, cancellation must remain dominant, and wall-clock rollback must never outrun the monotonic work clock.

A projected epoch may advance only by newly authenticated admission-wait credit. Repeated projection without new waits still cannot extend it. The existing no-wait monotonic test remains valid; raw epoch monotonicity across an actual excluded wait is not the authority invariant. The correct invariant is no additional active-time budget, allowing only the new measured wait to move the epoch.

A carried narrower maximum/deadline must stay narrower after correction. Track its actual issuance baseline/provenance rather than blindly crediting any same-owner JSON. Emit only integer representable finite epochs, bounded by both remaining live authority and the narrowed policy ceiling. Perform issuance/cancellation checks before inspecting caller environment and again before handoff.

## Missing Neutral Regression Cases

- Finite control created before wait: project before a wait, during an open wait, and after closure; admit a real child each time; apply the new wait once.
- Fresh finite child created after the parent's raw epoch passed during admission; it must retain positive effective parent authority without gaining a fresh parent budget.
- Ordinary parent with inherited requested0 and with finite request; finite parent with inherited requested0; narrower followed control.
- Two projections separated by two waits, including overlap; a third projection without new wait must not extend authority.
- Independent invocation after an earlier invocation waited; historical credit must not transfer. Nested reuse must retain the original clock.
- Authentic narrower carried projection plus broader current owner; preserve narrow policy and deadline. Unowned/corrupt/expired wire must not be revived.
- Wall clock advances/rolls back while monotonic time advances; real work still exhausts the finite maximum.
- Abort, retirement, or copied structural control before and during a wait/projection; cancellation and private issuance refusal must dominate.

No recommendation here changes Lowpoly behavior or bypasses the shared preparation/Source fence. The blocker belongs to the common process authority handoff, and needs owning protocol tests plus the canonical native retry before any Lowpoly native/browser pass can be claimed.
