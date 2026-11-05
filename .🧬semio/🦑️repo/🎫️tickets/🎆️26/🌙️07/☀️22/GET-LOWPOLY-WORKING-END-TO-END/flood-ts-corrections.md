# Flood TS Corrections

2026-11-05. Authorized ownership: TS flood cursor and focused TS tests only. Runtime owns Rust. No Native/server.

## Change

Replaced retained target tuple metadata with four numeric channel fields plus numeric packed Uniform identity and sourceBound flag. Only borrowed RGBA Uint8Array reference is retained; Uniform caller tuple is not owned or retained. Initial color captured once, without hidden target typedarray/tuple ownership. Per-step work, workspace buffers and logical paidclose accounting remain intact.

Runtime had already corrected done/cancel parity and Uniform alias identity before ownership handoff. Those peer fixes were preserved, not reverted to manufacture red.

## Public Regression Coverage

Added tests for:
- mutating same caller Uniform tuple refuses while original workspace stays alive, unpublished, and zero-grant close releases nothing;
- completed cursor receiving cancellation revokes done/result, then explicit paidclose releases exact workspace;
- fresh by-value Uniform tuples and borrowed RGBA produce identical region masks and per-step receipts at budget1, release exact6bytes/pixel.

Existing neutral region corpus still compares independent Graphlib reachability and validates JSON via Ajv. No fixture changes.

## Actual Checks

repo tools exec --light bun -- test ./🧰️framework/🔨️modules/🔲️pixels/✍️editing/🪣️flood/🧪️tests/🟦️.ts:
exit0,5pass,0fail,740expect calls.

Retained fixture-check.ts:
exit0;4strict schema pairs valid,0TypeScript syntax diagnostics,8Rust include_str references resolve.

Public behavior regressions initially passed because peer correction landed first; no observed public red is claimed. An interim private-layout introspection assertion failed but was removed per root direction because it mirrored representation. It is not retained or claimed as meaningful TDD evidence. Scalar metadata replacement is an ownership refactor supported by public output/work/cancel/close regression and source inspection.

## Ownership

Runtime informed TS cursor/tests frozen and owns further Rust allocation capacity/failure proof. No Rust/neutral fixture/production Lowpoly edits made by this task.


## Approved Grant Parity Follow-On (2026-11-05)

Root approved Runtime's schema-first contract in flood-parity.md and the managed-memory logical metadata distinction. TS cursor and focused tests now use required advance(source, maximumWork, grant, cancelled=false), demands(maximumWork), closeDemands(maximumCopyBytes), and close(grant). No old production overload exists. Outcome errors carry kind/message/progress; denial returns a successful zero-work/zero-receipt outcome before source binding/allocation. Source identity is ArrayBuffer identity + byteOffset + byteLength; same-range new views are accepted, copies/different offset/detached extent refused. Uniform RGBA identity uses scalar snapshots. One retained backing is retired per admitted close turn.

Actual test-first evidence: new public funded identity/demand regression on preceding source failed with TypeError cursor.demands is not a function; registered light Bun exited1 (7pass/1fail,867 assertions). After implementation and migrated callers, final focused command exited0: 10pass/0fail,5,674 assertions. This is genuine protocol red/green; earlier peer-fixed cancellation/scalar regressions were directly green and are not retroactively claimed red.

Command: repo tools exec --light bun -- test './🧰️framework/🔨️modules/🔲️pixels/✍️editing/🪣️flood/🧪️tests/🟦️.ts'

Coverage: independent Graphlib reachability + neutral geometry fixture; prefix result refusal; actual pixel cancellation and completed result revocation; uniform mutation; all quoted denial axes (items/copy/capacity/depth where positive) across allocation/init/flood, uninterrupted next-turn equality; close items/copy/release/depth denial then exact cumulative backing release; stale borrowed source retention; positive physical same-range view and negative copied/shifted/detached sources. Strict Ajv validates actual grant/quote/outcome data against Runtime's protocol schema, including every denial and funded phase. Receipts are checked against quotes. Test-only convenience helpers unwrap outcomes to preserve prior assertions; production has no compatibility adapter.

Additional registered fixture-check.ts run exited0: four neutral schema pairs valid, zero TS syntax errors/missing Rust fixture includes, eight include references resolved. No Native/browser/7.5ms or JS heap claim.

Important managed-runtime limit: typed-array byteLength is the retained backing receipt, while 512 metadata bytes are authored logical currency, not measured VM heap. Creating typed arrays intrinsically zero-initializes backing; allocation may perform O(N) runtime work despite the one allocation-attempt work unit. close releases cursor references for GC and cannot prove synchronous physical memory reclamation. Runtime/root were explicitly notified. Rust allocator/paid-drop evidence remains separate and unexecuted here.
