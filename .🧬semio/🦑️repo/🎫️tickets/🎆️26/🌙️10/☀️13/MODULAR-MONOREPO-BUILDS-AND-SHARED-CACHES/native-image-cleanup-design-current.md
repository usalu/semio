# Accepted Native Image Cleanup Design

2026-10-15. The actual macOS cancellation regression obtained genuine RED and its exact fixture settled. The sixteen schedules now execute against a concrete first-party immutable state/funding module and independent SQLite traces; the latest contract checkpoint is 28 tests, 60 expectations, all GREEN after recorded REDs. See `native-image-cleanup-current.md` for actual qualification and its limits. This subtask has not edited provider, process issuer, leases, shared compiler, or captured SDK Source. Real canonical enrollment and Native wiring remain coordinated with Root/Storage; no further native image operation has run.

## Scope And API Decision

Replace the unrestricted temporary `provisionNativeImageStorageV1` lifecycle with a scoped temporary-image owner. The scope receives an actual existing issued operation, opens its real exclusive physical owner before creation, supplies the admitted body with a shorter nonrenewable body phase, drains accepted work, and settles the private cleanup continuation before returning. Migrate every direct temporary caller together; keep no compatibility wrapper that can return a cleanup obligation outside the funded scope.

The public callback receives the ordinary image facts/port plus its existing issued body phase. It receives no cleanup control, platform process port, mutable ownership object, cleanup deadline setter, or command constructor. All accepted work using that temporary image must enroll in the same scope. An escaped callback promise or a compiler launched with the enclosing whole-operation control would defeat the body cutoff and must be refused by the actual issued resource/image boundary. Existing opaque issuance patterns provide the ancestry/identity check; structural `ResourceLeaseControl` shape alone does not establish funding.

Persistent fresh creation uses a private preparation transaction in the same provider, rather than borrowing the temporary public lifecycle and forgetting to close it. Existing persistent reopen and reader close continue to use the durable persistent owner. This is one physical ownership implementation with explicit temporary settlement and persistent-transfer exits, not two independently maintained attach implementations.

## Private Funding And Absolute Bound

Funding belongs to the real existing operation issuer, not the image command runner. Before acquiring/create/attach effects, the issuer verifies actual issued ancestry and a finite, live whole-operation deadline. It accepts one cleanup enrollment bound to the exact image-owner scope and registers that enrollment in its pending/drain set. An ordinary unlimited control cannot silently become an unlimited expensive image owner: it must have an explicitly authored finite enclosing operation before enrollment.

Let the issuer's existing monotonic whole-operation deadline be `D`, the already authored maximum terminal cleanup quantum be `R = 10000 ms`, and the current time be `t`. Refuse admission before physical effects if `D - t <= R`. The body cutoff is `B = D - R`; all image-dependent body work receives that existing child's shortened phase and cannot admit further work at or after `B`. The issuer supplies this bound from its private accepted record, rather than trusting a mutable caller function or computing new authority from environment after cancellation.

The cleanup scope records `D`, `R`, and an unforgeable enrollment identity before its first physical effect. Its single transition to closing seals `C = min(D, closingTime + R)` once. Every later observation/deactivation/verification must satisfy that same `C`; concurrent/repeated close joins the same promise and cannot reset it. `C` never extends the already accepted absolute whole-operation deadline. Starting the already reserved quantum once is consumption of the original enrollment; it is not permission to add ten seconds after an expired Original. If closing starts at/after `D`, admit no new command and retain/report unresolved ownership through the accepted receiver described below.

Body cancellation closes body admission immediately and begins drainage. It does not revoke cleanup of physical effects already accepted by the same operation. Cleanup's own cancellation is a separate, private terminal decision accepted with the enrollment; it stops its owned cleanup child and permits only joining, evidence preservation, and accepted unresolved transfer. Neither body nor cleanup cancellation creates a new `ScriptInvocation`, a new environment-derived root, or a public `ResourceLeaseControl` with a replacement signal.

The cleanup runner accepts only its unforgeable enrolled image token and a closed action enum. Its macOS command array is internal and fixed: bounded system image information, or deactivation of the exact accepted mapping. It accepts no caller command string, argument list, cwd, environment, path override, or additional signal. System process/termination/output primitives stay behind first-party interfaces. Reusing a generic body capture function with a publicly reachable freshly fabricated control is not this design.

The actual accepted activation child must settle before any deactivation/reconciliation can claim completion. Its termination/join time consumes the reserved terminal allowance. No detach timeout permits abandoning a still-live child and releasing its physical guard. A late child is unresolved custody, not a successful bounded cleanup.

## Actual Physical Acceptance And Reconciliation

Acquire and retain the genuine exclusive lease for the exact normalized image-owner resource before creating the fresh directory. Record the created host directory and underlying mount-directory identities before mounting, then record the actual regular backing's device/inode/size/link facts. Keep the actual guard handle privately; neither a path string nor a durable receipt is the guard.

Before invoking attach, record the accepted activation attempt in the owner. Successful attach captures complete bounded structured tool output and reconciles it with complete system inventory, accepted backing identity, mount-point identity, and actual mounted device/filesystem facts. A failed or canceled attach is first joined, then reconciled by the same private cleanup enrollment. Do not derive inactive state from rejected `await`, an unset local flag, an absent receipt, or a successful child exit.

On macOS the adapter must identify the accepted image path and actual kernel device/mount association; only that association may be deactivated. Prefer the captured device identity to an unchecked mountpoint pathname, and recheck the accepted directory/backing/mapping immediately before deactivation. Cooperative repository ownership is enforced by the real exclusive guard. If observation finds replacement, ambiguity, a changed device, missing/overflowed inventory, or other unknown material, refuse deactivation/deletion and retain unresolved custody. Do not claim an atomic image-identity detach guarantee from a pathname-based system CLI alone.

After the actual deactivation child closes, require complete inventory absence for the accepted mapping and actual underlying host mount-directory identity. Recheck the accepted backing and every owned descendant. Remove only known original files and known empty original directories; do not use recursive deletion of the caller path as a fallback. Unknown descendants or an active mapping cause unresolved settlement. Linux loop/mount and Windows VHD/disk/partition identifiers remain their own adapter evidence; the neutral protocol never assumes macOS detach syntax.

## Explicit Persistent Handoff

The persistent preparation owner retains EX through backing creation, activation, physical observation, receipt materialization, and acceptance by the durable persistent owner. A receipt write alone is not an accepted handoff. The acceptance point must be explicit inside the owner transaction and observable by the enclosing cleanup scope before EX is released.

Before acceptance, any create/attach/observe/receipt error belongs to the original preparation owner and triggers its exact physical reconciliation/cleanup. After acceptance, body/reader-admission failure leaves a durable persistent artifact owned by that accepted receiver; it does not run temporary deactivation. Shared reader close directly releases its actual held guard even after cancellation. Persistent reader close is never treated as physical retirement.

Publish the handed-off state only once. The original temporary/preparation scope must refuse further body/cleanup operations after that point. An acknowledgment failure before acceptance preserves the original exclusive custody and primary handoff failure. An error after acknowledged transfer is reported without making both owners independently eligible to deactivate the same image.

## Custody Through Failure

The scope stops new body admissions before draining every accepted body promise and progress publication. All actual command children are joined. A stalled progress callback cannot be wished away by `Promise.race`; until its accepted work settles, retain the owner/guard and expose unresolved progress custody. Cleanup must not invoke a canceled body callback as though it were fresh cleanup authority. Accepted cleanup progress uses its own originally enrolled bounded notification path, and any notification refusal remains visible while physical settlement continues where possible.

Release the actual guard only after physical settlement, accepted persistent transfer, or an explicit accepted unresolved-ownership transfer. The unresolved receiver must take the real retained guard, exact physical evidence, outstanding child/publication joins, and failure set; it cannot be a JSON record with no live receiver. Its acknowledgment must happen before the originating issuer removes the cleanup enrollment from drainage. No new body/command authority accompanies that transfer.

An implementation that cannot provide that real unresolved receiver must keep the originating scope unresolved and report its undischarged custody. It must not return a fictional successful close, drop the guard, silently swallow cleanup failure, or invent an unrelated maintenance Original. Designing a durable cross-process recovery receiver is a separate operation contract; this initial repair must not pretend that text evidence alone solves it. This receiver boundary needs an executable acceptance/refusal fixture before a repaired provider can be called complete.

Retain failure objects in ordered first-party records: primary body/handoff failure; accepted body/drain/progress failures; physical cleanup/verification failures; transfer/guard-release failures. If exactly one failure exists, preserve it. If more than one exists, return an `AggregateError` containing every actual failure, without replacing an earlier cause from `finally`. Preserve thrown `undefined` through an explicit failed flag, and deduplicate only identical error objects already recorded through the same accepted promise. Both concurrent close callers observe the same settlement promise/failure set.

## Additional Fixture Source

Two new unreferenced design-input files are staged under the storage owner:

- `🧹️cleanup/🧬️schema/📥️continuation.json` defines a closed language-neutral schedule/expectation corpus. Serialized data cannot grant a cleanup capability.
- `🧹️cleanup/🧪️tests/🧫️fixtures/📥️continuation.json` supplies sixteen additional schedules: concurrent and repeated close; attach rejection after actual activation; failed persistent publication; accepted persistent handoff; reserved body cutoff; cleanup expiry and child/handoff joins; rejected and stalled progress publication; replacement/unknown physical evidence; combined primary/cleanup and primary/guard failures; copied grant refusal; expired enrollment refusal; and a successful tool result with a still-active mapping.

These two files now drive the registered contract route's transition tests. A first-party immutable state/funding implementation in `🧹️cleanup/📋️state/🟦️.ts` agrees with the independent SQLite trace for all sixteen schedules. Additional executed edge cases cover immediate cancellation admission refusal, pending accepted work, capped terminal deadlines, complete actual failure aggregation including thrown `undefined`, and refusal of funding/backing rebinding. These are concrete neutral state/funding results; actual private issuer custody and macOS side-effect/guard/contention qualification remain outstanding. Data projection success does not by itself prove those runtime paths.

## Coordinated Repair Order

1. Root declares the current Job cohort settled and opens the defining-Source repair window.
2. The executable neutral transition/funding fixtures and independent SQLite trace outputs have completed RED → GREEN. Add actual private enrollment/physical-owner fixtures when Storage's canonical seam is authored; no structural registration fallback or separate receiver is permitted.
3. Implement issuer enrollment/funding and private image ownership transitions, then migrate all temporary callers and the persistent preparation handoff together. Keep cleanup capability out of package exports.
4. Run contract/type and targeted owner fixtures, obtaining actual console/debug evidence. Run the existing genuine native cancellation regression to GREEN and add actual failure-after-side-effect/identity/custody tests.
5. Run targeted persistent close/handoff regressions, publish the resulting coherent SDK through its normal accepted path, and qualify any dependent native cohort only against that new defining-Source checkpoint.
