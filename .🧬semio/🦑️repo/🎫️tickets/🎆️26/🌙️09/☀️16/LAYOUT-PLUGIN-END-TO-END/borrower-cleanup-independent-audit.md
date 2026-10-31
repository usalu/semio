# Independent Borrower Terminal Cleanup Audit

Read-only source audit, 2026-10-29. No tests, builds, preview retries, effects, deadlines, caps, process cancellation or cache modifications were performed. Rust owns the implementation. This report audits the pre-fix close chain observed during this review, not an unprovided proposed patch.

## Actual Failure and Cause

The owning lane reports preview65226 expired while queued behind shared69943, before plan or reclamation. The current defining borrower source explains the reported close-time failure:

1. Repository partition close drains accepted partition work and closes the image before a privately owned lease image (`repo library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts`, `closePartition`).
2. Persistent Native image close stops borrower admissions, drains retained cleanup readers and maintenance, then invokes `closeNativeImageBorrowerV1` (`process native-build storage/🧹️cleanup/🎛️command/🟦️.ts:128–129,166`).
3. Borrower close drains its accepted operations then calls its original `scope.verify` (`📥️borrower/🟦️.ts:22,28,32`).
4. The SH verification closure calls `requireResourceLeaseReadCustodyV1` with the acquisition leaseControl. That guard begins with `checkResourceLeaseControl(control)` (`process/🔒️leases/🟦️.ts:225–226`). An expired ordinary body control therefore refuses even when the exact kernel SH remains held and all work has drained.

The ordinary body refusing after its deadline is correct. Reusing that body authorization as the only terminal release authorization is the lifecycle mismatch.

## Safe Authority Boundary

Terminal release should authenticate **the same privately issued original still-held lease**, its exact directory/resource/path/mode/control association and physical lock identity, while granting only close/drain/release. It must not admit any new body operation, mapping mutation, source capture, read, writer promotion or reopened clock.

The existing accepted-cleanup path demonstrates the intended distinct authority:

- `acceptNativeImageBorrowerCleanupV1` captures a private exact borrower/lease loan while the borrower is open and its body is live (`borrower:17–18`).
- `acceptResourceLeaseCleanupV1` checks both live controls and the exact accepted body/enrollment; capture binds the privately issued held lease and physical database/directory identities (`leases:118–121`).
- `requireAcceptedResourceLeaseRetainedCleanupV1` checks the same unspent original held SH/EX, same enrollment/binding and cleanup reserve, using physical identity rather than the expired ordinary clock (`leases:115,125–126`).
- `closeNativeImageBorrowerCleanupV1` checks the exact captured loan, synchronously closes births, runs under its accepted cleanup operation, drains pending work, rechecks cleanup custody and descriptor closure, then releases the same lease (`borrower:33–34`).

A cleanup loan cannot safely be captured after the ordinary deadline: its genuine preacceptance is part of its provenance. If the ordinary image close uses a distinct terminal-held-custody verifier instead, that verifier must be a closed lifecycle-only interface over the original private lease issuance and physical pin; the general read/write custody guards must remain time-bound.

## Drain and Continuity Invariants

- Stop top-level births synchronously before any await; reject close from an accepted borrower frame (`borrower:27–31`).
- Accepted nested operations may finish their already admitted descendant work; join the entire pending set before releasing the image guard (`borrower:25–26,32`).
- Retained cleanup readers must settle and drain before borrower release. The persistent receiver explicitly waits their release and pending sets (`command:129`).
- Keep the same guard through drain. Do not release/reacquire a substitute SH, lose the old database pin, reconstruct authority from serialized facts, or swap runtime Source/receiver identity.
- Require artifact descriptor closure before kernel release. A descriptor closure failure must retain unresolved custody, not merely report a failure after releasing it.
- Preserve actual body failures, including undefined errors, without treating cancellation/deadline as successful preview. Terminal cleanup success does not mean the preview ran or reclaimed bytes.
- The fixed backend has the analogous terminal chain (`storage/🧱️fixed/🟦️.ts:91,105–106`); a shared borrower-layer solution must preserve both SH and real maintenance EX identity.

## Concrete Hazards to Avoid

The current ordinary `closeBorrower` catches verification errors, appends them to failures, and still calls `scope.lease.release()` (`borrower:32`). Thus clock expiry is currently surfaced as a terminal failure even after release; a changed verifier must not turn a genuine custody/identity failure into unconditional release. The accepted-cleanup close instead retains unresolved custody when its verification fails (`34`).

The persistent receiver currently marks itself closed and clears its guard reference after catching borrower-close failure (`command:166`), and fixed close likewise retires receiver state after collecting a close error (`fixed:105`). If an implementation changes borrower close to retain custody on a failed terminal identity check, these callers must not erase the only unresolved owner/reference. This is a concrete integration hazard for the proposed class of fixes, not a claim that the final Rust patch has that bug.

Do not remove `checkResourceLeaseControl` from the general read custody guard, accept a copied lease, manufacture a new control/deadline, widen cleanup reserve, skip physical checks, or directly call release at the repository wrapper. Those changes would cross the body/terminal boundary rather than repair it.

## Verification Boundary

No final patch was independently reviewed or executed here. Canonical semantic and actual lifecycle tests remain required for Rust's final source. The prior preview has no plan/reclaim result. Compiler capacity remains unresolved according to the owning lane; no maintenance rollout or retry is released by this audit.


## Independent Proposed Design Review

2026-10-29. Coordinated directly with Rust owner. At review time no production changes had been made; this is an independent assessment of the stated proposal, not final-source or runtime acceptance.

Rust confirms ordinary normal-reader lifetime has no prebirth PhysicalCleanup enrollment. Therefore reuse the original accepted cleanup loan wherever one already exists, but do not manufacture an enrollment or capture a new loan after cancellation/expiry to repair ordinary close. The proposed leases-owned private terminal-release token is a justified separate lifecycle interface for that ordinary lifetime if captured while the actual issued lease and its acquisition control are live.

The proposed token captures the actual ResourceLease object and its exact private IssuedResourceLease record, acquisition control/signal identity, mode/resource/path and directory/database dev+ino. Both object identities matter: queued/combined wrappers can share an Issued record (`leases/🟦️.ts:402,503`), so accepting another wrapper merely because it names the same resource would weaken the original port association. Capture must validate the actual held transaction and normal file invariants before issuance. Release must require that same still-held private record and unchanged physical identities; no serialized identity or caller-provided reconstructed facts can substitute.

This token should expose only terminal release. It must not yield a ResourceLeaseControl, reader/writer proof, funding grant, arbitrary callback operation, mapping mutation or new work port. It needs no renewed body deadline: terminal authentication proves the original pin still exists, while ordinary read and borrower operation verification continue to call the unchanged time-bound controls. It must not reset deadline, mint another source or authorize a replacement kernel acquisition. Existing preaccepted cleanup paths must keep their exact original loan; cleanup failure cannot silently fall back to terminal token authority.

Borrower close must synchronously stop births, join all already accepted descendants, require artifact descriptor closure, then release the same original guard. Descriptor/identity/release failure must keep the borrower scope and the concrete Mac/fixed receiver and guard unresolved. The current kernel release sets phase=closing before database.close and marks closed only afterward (`leases/🟦️.ts:257`); a real close failure must retain that truthful unresolved phase. Never reset it to held, clear the sole guard reference, mark receiver closed, or treat earlier body cancellation as actual successful preview/reclamation. Successful release plus an earlier body failure must still report the body failure separately.

The current pre-fix ordinary close and caller hazards above remain applicable until the promised borrower and both Mac/fixed receiver changes are authored. Rust acknowledges those integration changes are required. No concrete flaw was found in the stated narrow-token design under these constraints. Final schema/API/implementation and hostile lifecycle cases remain to be independently checked after publication; no native/preview retry is released by this design review.


## Prepatch Protocol And Authored Tests

Source-only recheck while real runtime RED54108 waits for capacity. TerminalRelease schema explicitly declares actual-live-issued-lease capture, same-wrapper-and-issued-record custody, release-only effect, no bodyControl/renewal/newAdmission and retain-unresolved failure. This matches the proposed narrow boundary; schema text alone does not enforce production behavior.

Authored genuine expired/cancelled test acquires real SH lease, accepts actual borrower, opens independent SQLite connection and proves BEGIN EXCLUSIVE refusal before expiry/cancellation. It waits past the actual control remaining time or aborts its real signal, then asserts ordinary borrower operation refuses before calling close. Thus current expired close should fail at its real body verifier rather than only an absent new API. The later nativeImageBorrowerReleasedV1 call occurs after awaited close; it cannot mask that expected real prepatch failure.

Drain test retains an accepted operation behind a promise gate, starts terminal close, verifies new top-level admission refuses and independent EX remains blocked, then lets that already accepted operation admit its nested descendant and finish before close/EX succeeds. This exercises actual reader drain and original kernel pin, not a synthetic boolean substitute. Tests use canonical declareBuildTestEnvelopeV1/withBuildTestInvocationV1 and createIssuedResourceControlV1; no inline fabricated Script Original is present.

Concrete coverage limitation sent to Rust: neutral copied-token/foreign-pin rows currently run only an independent SQLite boolean model. They do not call actual terminal token APIs. Likewise current authored tests do not force a real release/physical-identity failure and assert Mac/fixed receiver guard retention. Those negative protocol/receiver behaviors remain unverified until final hostile source/tests are published; the model must not be reported as actual production refusal proof. No runtime result was produced by reviewer.


## Final Authored Source Review

2026-10-29. Final defining leases, borrower, both platform callers, schema and five authored tests reread after Rust publication. No runtime/check/preview executed by reviewer. Earlier runtime54108 was still queued when production changed, so it cannot establish a prepatch runtime RED; retain its eventual result as post-fix acceptance only.

`leases/🟦️.ts:108-117` now privately issues one token per actual Issued record while the genuine acquisition control is live. Its scope binds actual wrapper object, originalLeases record, control/signal association, mode/resource, captured release function and bigint directory/file dev/ino/mode plus file size. The private Issued record supplies original immutable path/resource/directory. Terminal identity requires held phase, unchanged wrapper release identity, no-follow directory/file observations, regular single-link file and exact pins. Release calls only the captured original wrapper and confirms private Issued.phase=closed before token.released=true. It exposes no body control or operation callback; general reader/writer/borrower body verification remains time-bound and unchanged. No late capture, renewed clock or new read/work authorization found.

`borrower/🟦️.ts` captures terminal token during actual borrower acceptance after body verification. Ordinary close synchronously stops births, drains pending accepted work, requires descriptors closed and invokes terminal release. On any descriptor/token/release error it retains unresolved scope with live=true and retainedBorrowers membership. Only confirmed success sets live=false, removes custody and then reports any prior canonical body failure. nativeImageBorrowerReleasedV1 is an observation over that private state, not an admission port.

Existing acceptNativeImageBorrowerCleanupV1/closeNativeImageBorrowerCleanupV1 and cleanup continuation remain on exact preaccepted PhysicalCleanup loan/enrollment. They do not fall back to terminal token when cleanup fails, and token release does not mint a cleanup enrollment. Their actual captured phase control/loan association is preserved.

Mac `command/🟦️.ts:128,166,174` and fixed `fixed/🟦️.ts:91,105-106,113` now consult nativeImageBorrowerReleasedV1 before clearing/retiring normal, maintenance and failed-reader receivers. True release failures leave stage=unresolved and private owner retained; failed-reader paths raw-release only when no actual borrower owner exists. Success accompanied by an old body failure can still retire the already released receiver and report that failure. No unconditional raw-release fallback or false receiver closure found in these inspected branches.

Actual hostile tests now call production APIs: copied opaque token refuses; duplicate acceptance refuses; a hardlink makes physical pin validation fail while independent SQLite EX remains blocked; restoring that alias permits exact token close and idempotent repeat. Separate wrapper-release failure is installed on the actual privately issued lease before acceptance, then real borrower close rejects with that identity, released=false, same closing promise, blocked independent EX and stopped new births. The unresolved real guard is deliberately retained until test-process exit; no emergency raw release masks failure. Earlier neutral copied-token/pin model limitation is therefore closed at borrower/token layer in authored tests. Full Mac/fixed filesystem fault-runtime coverage remains explicit pending; source control-flow review is not that runtime proof.

No additional actionable defect found in this final scoped source review. Genuine post-fix test acceptance and real maintenance/preview outcomes remain required under root ownership. This review does not authorize rollout or claim compiler storage recovery.


Final narrow follow-up reread: terminal acceptance now also explicitly rejects non-function lease.release before token capture. This adds an honest capture guard without broadening authorization. No new actionable defect identified. Rust reports sources frozen for normal GREEN45981 and focused canonical tsc66928; neither outcome is inferred here.


## Actual Full-Suite Failure Boundary

Independently read generated/borrower-terminal-green.log after root reports45981 EXIT1. Actual runner reports3pass/3fail/23expectations across6reported cases, including an unnamed consequence. Only neutral, expiry/cancellation and accepted-descendant drain laws passed. Hardlink hostile law reached final combined db.exec(begin exclusive;rollback) after copied-token/duplicate-capture/hardlink refusal and exact token release calls, then SQLite reported cannot rollback - no transaction is active. The later release-failure law did not run its body: shared Original admission was already closed by this earlier failure. Do not describe this result as five laws passing or real release-failure retention verified.

The log does not establish why combined BEGIN/ROLLBACK lacks an active transaction, nor prove a fixture-only cause. Earlier combined calls passed in other cases, so a blanket driver explanation is unsupported. Source repair to split BEGIN EXCLUSIVE, explicitly require db.inTransaction=true, then ROLLBACK and require false is a stronger independent lock observer; it must retain all existing copied-token/hardlink/held-lock checks and produce a genuine full-suite rerun. No exception waiver, swallowed transaction error or reduced hostile assertion is justified. Until that result, distinguish supported production-source review from unresolved kernel-observer/runtime behavior. Reviewer executed no reproduction or driver probe.


Observer repair source reread: the same originally blocked connection now must BEGIN EXCLUSIVE, report inTransaction=true, ROLLBACK and report false after token close; only afterward a fresh independent connection repeats those checks. Copied-token, duplicate acceptance, hardlink refusal and blocked-lock assertions remain. Reopening cannot mask failure of the original observer. This is an honest stronger assertion change; cause of45981 remains unresolved until actual fresh full23380 result. No production authority change or reviewer execution.


Rust reports canonical focused strict66928 TERMINAL EXIT0. Independently read generated/borrower-terminal-strict.log: normal capacity admission and no compiler diagnostics; exit status provenance is owner terminal result, not inferred from empty diagnostic output. This is typing evidence only. Full strengthened observer23380 remains pending; expected coverage sanity is all5named laws and32assertions, not prior partial3pass or fresh-only30assertion variant.


## Final Actual Five-Law Evidence

Independently read generated/borrower-terminal-green-3.log and current restructured test source after owner60637 terminal EXIT0: all5named laws passed,0failed,35expectations. This supersedes the earlier incomplete32assertion plan; current source honestly separates clean release from namespace attack. The actual failed-wrapper-release law now executes and passes, including released=false, same retained closing promise, independent blocked EX and stopped new births.

Clean token phase keeps the same independent blocked observer open before and through actual release, requires wrapper callcount0 before/1 after idempotent release, then that same connection must BEGIN EXCLUSIVE, inTransaction=true, ROLLBACK and false. No observer close precedes this positive kernel release proof.

The raw sqliteNamespaceControl helper contains only Database/filesystem calls, with no lease/borrower product calls. Actual log records SQLITE_BUSY errno5/activefalse while its raw SH is held, and SQLITE_IOERR_VNODE errno6922/activefalse after raw guard close following link/unlink. This independently demonstrates that the namespace attack itself can poison this Bun/SQLite observer even without product authority code. It supports restructuring the fixture, not a claim that arbitrary product failure is harmless.

Separate hostile phase now keeps the alias and real issued guard unresolved: actual captured terminal release refuses pin mismatch, captured wrapper attempts remain0, and existing private held-lease custody is still recognized. It retains actual held token/observer/alias until process exit; it makes no clean post-tamper EX or general kernel-continuity claim. This is a precise production refusal/retention assertion, complementary to clean same-observer kernel proof. No authority check was waived or production code broadened.

Focused production strict66928 and root14strict61699 passed as reported/read above; final test-helper strict78490 remains pending since helper source changed after66928. Full Mac/fixed filesystem fault-runtime and Windows kernel execution remain unclaimed. No reviewer runtime/driver probe was executed.


Final78490 owner terminal EXIT0 confirmed as reported; independently read generated/borrower-terminal-strict-final.log with normal admission and no diagnostics. This final five-root canonical strict run includes current restructured helper, closing earlier pending test-source typing limit. Combined actual scope:60637 five laws/35assertions plus78490 strict, prior root14strict61699. Runtime log identities/counts were read directly; strict exit-status provenance remains owner terminal tool. No full platform/Windows/general kernel or compiler recovery claim follows. Audit scope complete, no reviewer effects.
