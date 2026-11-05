# Flood Denial Evidence

2026-11-05. Authorized focused TS tests; no Native/server. Cursor production unchanged in this follow-on.

## Existing Public API Checks

Added two public behavioral tests and ran registered light Bun focused flood file:
7pass,0fail,867expect calls,exit0.

Invalid maximumWork values0,-1,65537,0.5,NaN are refused before/after real progress; the next funded advance receipt matches an uninterrupted reference cursor exactly, as does final mask. This tests state atomicity without reading private fields.

After actual flood pixel progress (completed>pixelcount), a different borrowed RGBA view is refused; result stays unpublished and next valid advance equals uninterrupted reference. Original workspace remains available for paidclose. close(0) and close(15) on4x4 workspace release0 and remain unclosed (15 is below smallest16byte backing); full96byte grant releases exact count*6 and terminates. These are logical release/accounting tests, not physical JavaScript GC proof.

Existing Uniform mutation and cancellation parity tests remain green; independent Graphlib region oracle/Ajv fixture validation still runs in same suite.

## Explicit Parity Limit

TS public API currently has maximumWork and close(maximumReleaseBytes), no demands/copy/capacity grant arguments. No capacity/copy-axis test or Rust physical grant parity is claimed. Root instructed not to invent a TS grant API in isolation; Runtime owns shared intentional quote/grant design. No behavioral red observed in this follow-on, because existing public behavior already satisfied tests. No speculative production fix made.


## Approved Grant Parity Follow-On (2026-11-05)

Root approved Runtime's schema-first contract in flood-parity.md and the managed-memory logical metadata distinction. TS cursor and focused tests now use required advance(source, maximumWork, grant, cancelled=false), demands(maximumWork), closeDemands(maximumCopyBytes), and close(grant). No old production overload exists. Outcome errors carry kind/message/progress; denial returns a successful zero-work/zero-receipt outcome before source binding/allocation. Source identity is ArrayBuffer identity + byteOffset + byteLength; same-range new views are accepted, copies/different offset/detached extent refused. Uniform RGBA identity uses scalar snapshots. One retained backing is retired per admitted close turn.

Actual test-first evidence: new public funded identity/demand regression on preceding source failed with TypeError cursor.demands is not a function; registered light Bun exited1 (7pass/1fail,867 assertions). After implementation and migrated callers, final focused command exited0: 10pass/0fail,5,674 assertions. This is genuine protocol red/green; earlier peer-fixed cancellation/scalar regressions were directly green and are not retroactively claimed red.

Command: repo tools exec --light bun -- test './🧰️framework/🔨️modules/🔲️pixels/✍️editing/🪣️flood/🧪️tests/🟦️.ts'

Coverage: independent Graphlib reachability + neutral geometry fixture; prefix result refusal; actual pixel cancellation and completed result revocation; uniform mutation; all quoted denial axes (items/copy/capacity/depth where positive) across allocation/init/flood, uninterrupted next-turn equality; close items/copy/release/depth denial then exact cumulative backing release; stale borrowed source retention; positive physical same-range view and negative copied/shifted/detached sources. Strict Ajv validates actual grant/quote/outcome data against Runtime's protocol schema, including every denial and funded phase. Receipts are checked against quotes. Test-only convenience helpers unwrap outcomes to preserve prior assertions; production has no compatibility adapter.

Additional registered fixture-check.ts run exited0: four neutral schema pairs valid, zero TS syntax errors/missing Rust fixture includes, eight include references resolved. No Native/browser/7.5ms or JS heap claim.

Important managed-runtime limit: typed-array byteLength is the retained backing receipt, while 512 metadata bytes are authored logical currency, not measured VM heap. Creating typed arrays intrinsically zero-initializes backing; allocation may perform O(N) runtime work despite the one allocation-attempt work unit. close releases cursor references for GC and cannot prove synchronous physical memory reclamation. Runtime/root were explicitly notified. Rust allocator/paid-drop evidence remains separate and unexecuted here.
