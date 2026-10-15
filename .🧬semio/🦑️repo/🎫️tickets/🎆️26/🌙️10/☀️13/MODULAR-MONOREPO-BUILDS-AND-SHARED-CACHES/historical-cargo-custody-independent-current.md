# Historical Cargo Custody Independent Audit

Read-only source audit on 2026-10-15. No runtime, process probe, image opening, deletion, or test execution.

## Concrete Finding

`library/⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts:35` directly returns `retireInactiveCargoStoreOwnedV1(options)` for compact reservations. Unlike the historical branch, this does not register the accepted retirement with the enclosing writer's pending-operation set. A caller inside `withCompilerWriterCustodyV1` can start this compact retirement without awaiting it and return. The owned helper passes its synchronous writer check before awaiting reservation recheck (line 38), while the enclosing writer can observe no registered pending operation and release its lease. Retirement can then proceed after writer custody, and potentially the owning image lifetime, ends. Register compact retirement through the actual nested writer operation before its first await; retain the actual allocation owner too. Add a detached compact-retirement test that demonstrates owner return waits and escaped follow-up admission refuses.

## Historical Path Assessment

The canonical caller in `budget/🟦️.ts:17` explicitly acquires compiler writer custody inside authenticated image maintenance. Execution line 35 then nests writer custody, registering the retirement in the enclosing writer's pending set, and registers it against the narrowed issued resource control. The historical issuer waits for that narrowed control to drain before setting its private scope non-live and releasing image EX. This ordering covers accepted detached retirement, including callback failure, by source inspection.

The global compiler-name inventory is diagnostic only after custody authentication. The destructive path retains actual profile locks, complete global `lsof -nP -Fpn`, rooted descriptor checks across build and target roots, recensus comparisons, and deletion-byte bounds. The private historical WeakMap requires exact workspace, invocation, control, roots, lease directory, preview mode, and nonqueued request. Copied/foreign/escaped claims are refused.

## Correction and Limits

An initial message incorrectly suggested that execution's writer precondition blocked canonical historical preview. Following the actual caller showed its outer writer wrapper; that conclusion was promptly retracted. The test helper also currently contains that wrapper. No canonical precondition defect is reported.

The new tests exercise ten exact-boundary variants and an invalid-policy detached lower-call failure. Source inspection supports those assertions, but none were executed here. Their detached historical failure is a pre-effect failure, so it does not independently prove draining during actual scan or deletion. The immutable SDK derivative implementation is unrelated to this Rust review.
