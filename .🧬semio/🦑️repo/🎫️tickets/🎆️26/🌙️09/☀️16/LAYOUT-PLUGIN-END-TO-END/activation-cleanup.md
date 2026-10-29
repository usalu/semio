# Activation Cleanup Ordering

2026-10-28. The original activation refusal is retained in generated/activation.log (7,234 lines). No cache image, journal, receipt, mount, partition, lease, or peer process was modified in this packet.

## Exact Refusal and Source Attribution

Activation log lines7202-7215 state:

~~~text
First failure: Accepted physical body must stop before cleanup
error: Accepted physical body must stop before cleanup
at checkAcceptedPhysicalCleanupV1 (.../routing/invocation/process/🟦️.ts:1059:11)
at authorize (.../native-build/storage/🟦️.ts:177:5)
at settleAbsentNativeStorageChildV1 (.../native-build/storage/🟦️.ts:180:3)
at <anonymous> (.../native-build/storage/platform/🟦️.ts:171:17)
at async <anonymous> (.../routing/invocation/process/🟦️.ts:970:21)
~~~

The later Original process admission closed refusal is consequential. The cleanup guard correctly refused settlement before the accepted body's cleanup phase and finite cleanup window were established.

Fresh checked-out sources already contain the repaired ordering in retireNativePlatformFundedChildStorageV1: it captures cleanup custody and performs any physical retirement inside the ordinary body, awaits beginAcceptedPhysicalCleanupV1, then chooses successful, absent, or retained-reader settlement. settleAbsentNativeStorageChildV1 still calls the original checkAcceptedPhysicalCleanupV1, and its actual journal-retirement path still requires exact captured reservation custody, absence rechecks, parent identity, retained journal writer and synchronization before releasing the charge. Those gates were not weakened or edited.

Source timing and identities prove that the failed activation must not be treated as execution of the present platform source:

- Current platform SHA-256: d7e92cef1eb02785f2ff4aecce6763642f206f67a9c431cd0bc91afbcd4df7c6.
- Platform mtime:1793217322.1152153; activation log final mtime:1793218266.5809913.
- Earlier source memo57e54531fcc20786e71f.json (dependency command, mtime1793215973.7691808) captured platform SHA-256 b5d3379291b1efededdf56cb8d45aa4657983ba0e4568afb42512e7bc218ac5a.
- Later source memo39b11b90e467a391dbbd.json (runtime command, mtime1793217435.583618) captures the current d7e92c platform hash.
- The invocation source SHA-256 is3dba52df22882f30f4ad4018c5ce09e6b88b0e6eba2b591a489be759e1cc0e64 in both observations.
- Lightweight Bun transpilation for stack attribution places current platform line171 at if(!absent), and its awaited begin at line183. The failed log instead places settleAbsent at171. Current invocation generated line970 is still the accepted body callback, matching the stack.

The long-running operation used captured implementation bytes, while later commands observed a changed platform source. The exact older captured body was not rewritten or reconstructed; no claim depends on guessing its entire text. The recorded gate failure plus differing platform identity/line attribution establish premature absent settlement in the failed captured execution. Retrying the declared activation with a fresh current capture is the required operational step after regression validation.

## Regression Boundary

The new native-child-cleanup owned suite extracts the actual current retireNativePlatformFundedChildStorageV1 function with the existing TypeScript parser/transpiler and executes that orchestration under explicit test-owned ports. Its process envelope, accepted physical enrollment, pending body operation, begin/drain and cleanup-phase check are the authentic implementation. No fake token is passed to the real private gate, and test ports confer no real platform/cache custody.

Fresh bounded fixture directories independently witness a namespace already absent, a physical child retired before settlement, capture failure retaining charge, original physical failure surviving a later settlement refusal, and a namespace that reappears and therefore cannot be discharged as absent. The pending body child must actually stop before either successful settlement branch can observe the namespace. The real gate must refuse during capture. Neutral JSON fixtures are schema-checked with Ajv; SQLite independently checks held-charge outcomes and event ordering. A structural TypeScript AST law additionally requires both settlement branches to occur after the awaited begin.

This is an executable platform-control ordering regression, not an integration claim that real native cache backing files were mounted or retired. Existing real storage/journal enforcement remains untouched.

## Owned Route and Status

Declared target: repo run @semio-tech/framework-process:test-native-child-cleanup --output=stream.

The first attempted direct owner script door correctly refused Original parent Script process invocation required. That refusal is retained in generated/child-cleanup-script-door-refusal.log; no manual invocation envelope was synthesized. The declared target is running with its complete output retained in generated/child-cleanup-test.log. Final pass/fail will be appended after completion. No duplicate activation was launched.

## Files

- Process project target declaration.
- Process owned-suite table declaration.
- native-build/storage/platform/tests/🟦️.ts, its neutral fixture and schema.
- Ticket run-child-cleanup.py and this report.

No new production coordinator or indirection was added because the present source already had the necessary begin-before-settlement repair.

## Custody Queue Observation

The owned regression has not reached its test process yet. Read-only repo leases status reports process69374 waiting for shared mapping9d349bf4f2ee76d326b2ebe63ebb888383c7e49a28632d6331331654a7ac7600 behind exclusive holder10518/328. Holder10518 also has a queued shared request against the same mapping while retaining its exclusive lease, in foreign sessionf9862ee2-2e3c-43fc-9b16-d905a75045d7, started by9424. The status tool reported zero detected deadlock cycles. The observed exclusive hold/shared wait had already exceeded35minutes; this is custody evidence, not permission to infer a stale owner or release it. The full current status is retained in generated/child-cleanup-leases.log. No process was stopped, no lease/receipt was removed or edited, and no cache/mount workaround was attempted. The root task was informed that actual regression dispatch awaits owner-side resolution.

## Declared Independent Backend Attempt

The root task authorized the declared Bazel backend after independent custody-blocker.md established that its tools/dependencies/products stores are independent from the peer-held Nx mapping. Only this packet own queued Nx session51521 was cancelled through its existing controller. The immediate observation still showed draining rows; a later read-only status shows no PID69374 rows and the peer10518 exclusive mapping/self-shared wait remains unchanged. Both observations are retained in generated/child-cleanup-leases-after-cancel.log and generated/child-cleanup-leases-after-cancel-settled.log. No peer received a signal and no physical custody file was changed.

The current attempt is the exact declared command repo --tool=bazel run @semio-tech/framework-process:test-native-child-cleanup --output=stream. Its complete output is generated/child-cleanup-bazel.log, exec session77167. It is presently in ordinary machine-capacity admission. This is a real independently owned backend selection, not a change to the blocked Nx resource or the original source/process authority. Final actual test result remains pending.

## Actual Declared Regression Result

The independent declared backend command completed successfully with exit0:

~~~text
repo --tool=bazel run @semio-tech/framework-process:test-native-child-cleanup --output=stream
(pass) neutral current platform stops the accepted physical body before child settlement [123.90ms]
(pass) actual platform awaits body stop before either namespace settlement branch [0.85ms]
2 pass,0 fail,77 expect() calls;2tests across1file;462ms test runtime.
~~~

Bazel invocation ce311c44-b904-4453-ae3c-d0457ef134f6 analyzed5packages/11targets, ran2actions, and completed in4.762seconds after normal outer capacity/source preparation. Target was //:t_24d8bce3b700d0a0. The owned action and final snapshot restoration both completed. Full1142-line output is retained in generated/child-cleanup-bazel.log. The suite declares strict typechecking; this report makes no separate cross-platform compiler-run claim.

The real canonical cleanup guard refused body-phase use during the fixture and accepted settlement only after its pending ordinary child stopped. All5neutral cases passed, including actual test-owned namespace absence, physical child deletion before successful settlement, failed capture holding charge, exact first-error preservation after a later refusal, and a reappeared namespace retaining its charge. Ajv validated the portable fixture and SQLite independently verified charge/order output.

The root task has been informed that a fresh declared Bazel activation can now proceed. This packet launches no activation itself and claims no browser/live component success. No production gate, native image, mount, cache receipt, journal, intent, or peer process was changed.
