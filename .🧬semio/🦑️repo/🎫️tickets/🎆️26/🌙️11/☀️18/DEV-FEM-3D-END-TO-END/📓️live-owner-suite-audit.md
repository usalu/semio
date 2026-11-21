# Live Owner Suite Failure Audit

Read-only comparison of the saved `🗑️generated/live-owner-suite.txt` with current source. No suite or build was run. The captured file reports 19 passed, 1 skipped, 13 failed. The live mounted scheduling fixture passed. The test file has since changed (for example, the first failure moved from line 25 to line 33), so the capture does not establish current suite status or chronological preexistence relative to the changes under review.

Twelve failures stopped on literal source text or source-slice assertions after their fixture/SQLite checks passed. None is a behavioral execution of the reactor cleanup path:

- Mounted worker paid-turn forwarding and descriptor borrowing; mounted callers' inline outcome: exact worker expressions, flags or bounded slice boundaries.
- Publication continuation identity and tool-run receipt handoffs: exact function-body call strings and output assignments.
- Child-root maintenance and lifecycle grant: exact signatures and expected code spelling. Current child-root retirement accepts a `RetainedAllocationEffect` instead of the captured literal `grant: RetainedCloneGrant`.
- Media/typed handbacks, envelope/initializer handbacks, envelope return pumps, replacement close, and envelope maintenance entrance: exact signatures, calls or slice boundaries. Current `drive_retained_store` accepts `RetainedAllocationEffect`; current envelope ladder methods include a `StepContext` as well as a grant. These changes invalidate the captured source-text expectations; they do not by themselves establish equivalent behavior.

The thirteenth, actor capture, fails earlier with `law.sqlVector` undefined even after its fixture passes the referenced schema. This is a real test/fixture contract mismatch in the captured run, not evidence that actor capture or reactor maintenance failed at runtime.

The log therefore contains no demonstrated active cleanup regression. It also cannot prove the affected ownership, grant and handback invariants remain correct. Current behavioral tests or an updated language-neutral contract must exercise those invariants; merely replacing source strings would not validate them. The fresh-browser maintenance failure documented in activation verification was separate runtime evidence, and the recorded live-owner fixture and focused SQLite truth tables passed after the scheduler changes.

## Current run

`repo tools exec bun -- test ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🟦️.ts` completed with 19 passed, 1 skipped and 16 failed across 36 tests. Both live-cleanup scheduling cases passed. The new failures since the saved run are again source-text checks; the capture is `🗑️generated/live-owner-suite-current.txt` until ticket cleanup. The actor test still throws on `law.sqlVector.originalGrant`: the fixture defines `nativeGrant`, `cases`, and `copyPolicy`, but no `sqlVector`, while its schema accepts that fixture. Native actor tests and the native oracle separately check the actual copy and heap behavior; the stale SQL fixture reference needs repair.

## Fixture and publication assertion repair

The actor-capture case now reads the schema-validated UTF-8 samples and native copy grant. The SQLite oracle measures the ASCII, German and maximum-length samples as BLOBs and compares the copy debit against TextEncoder. The source assertion checks that capture yields before document, app configuration, window configuration and draft publication. It accepts the current paged document publication operation while preserving the original capture owner check.

The focused Bun run passed the actor case and both live maintenance cases, with 28 assertions across three tests. The broader current baseline reported 15 other source-text assertions failing.
